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

struct type_13 {
    member: array<vec4<f32>>,
}

struct type_15 {
    member: vec3<f32>,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: u32,
    member_4: vec3<f32>,
    member_5: u32,
    member_6: vec3<f32>,
    member_7: u32,
}

struct type_18 {
    member: array<type_15>,
}

struct type_20 {
    member: u32,
    member_1: u32,
    member_2: u32,
    member_3: u32,
    member_4: vec4<f32>,
    member_5: vec4<f32>,
    member_6: vec4<f32>,
}

struct type_23 {
    member: array<type_20>,
}

struct type_25 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_28 {
    member: array<type_25>,
}

struct type_32 {
    member: array<f32>,
}

struct type_34 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
    member_4: vec4<u32>,
    member_5: vec4<f32>,
}

struct type_36 {
    member: array<type_34>,
}

struct type_39 {
    member: array<u32>,
}

struct type_41 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
}

struct type_43 {
    member: array<type_41>,
}

struct type_48 {
    member: u32,
    member_1: u32,
}

struct type_52 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_55 {
    member: u32,
    member_1: type_52,
}

struct type_56 {
    member: f32,
    member_1: f32,
}

struct type_57 {
    member: u32,
    member_1: type_56,
}

struct type_58 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_59 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_60 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_69 {
    member: type_34,
    member_1: type_41,
    member_2: type_41,
}

struct type_72 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

struct type_74 {
    member: u32,
    member_1: vec4<f32>,
}

struct type_79 {
    member: array<atomic<u32>>,
}

var<private> global: vec3<u32>;
@group(0) @binding(0) 
var<uniform> global_1: type_9;
@group(0) @binding(1) 
var<storage> global_2: type_13;
@group(0) @binding(2) 
var<storage> global_3: type_18;
@group(0) @binding(3) 
var<storage> global_4: type_23;
@group(0) @binding(4) 
var<storage> global_5: type_28;
@group(0) @binding(5) 
var<storage> global_6: type_32;
@group(0) @binding(6) 
var<storage, read_write> global_7: type_36;
@group(0) @binding(7) 
var<storage, read_write> global_8: type_79;
@group(0) @binding(8) 
var<storage, read_write> global_9: type_43;
@group(0) @binding(1) 
var<storage> global_10: type_36;
@group(0) @binding(2) 
var<storage, read_write> global_11: type_13;

fn function_() {
    var phi_487_: type_48;
    var phi_33400_: bool;
    var phi_507_: vec3<f32>;
    var phi_510_: u32;
    var phi_21665_: f32;
    var phi_21677_: type_56;
    var phi_21688_: type_57;
    var phi_21689_: type_57;
    var phi_21693_: type_57;
    var phi_21694_: type_57;
    var phi_21695_: bool;
    var phi_21699_: type_57;
    var phi_21586_: type_57;
    var phi_21737_: bool;
    var phi_21817_: f32;
    var phi_21829_: type_56;
    var phi_21840_: type_57;
    var phi_21841_: type_57;
    var phi_21845_: type_57;
    var phi_21846_: type_57;
    var phi_21847_: bool;
    var phi_21851_: type_57;
    var phi_21785_: f32;
    var phi_21889_: bool;
    var phi_21614_: f32;
    var phi_21616_: f32;
    var phi_21625_: type_57;
    var phi_21629_: type_57;
    var phi_21633_: type_57;
    var phi_21904_: bool;
    var phi_9510_: vec3<f32>;
    var phi_9513_: vec3<f32>;
    var phi_9515_: vec3<f32>;
    var phi_9517_: vec3<f32>;
    var phi_9519_: u32;
    var phi_9603_: u32;
    var phi_9618_: type_56;
    var phi_9619_: type_56;
    var phi_9620_: type_56;
    var phi_22104_: bool;
    var phi_22231_: bool;
    var phi_22246_: bool;
    var phi_9948_: vec3<f32>;
    var phi_10021_: vec3<f32>;
    var phi_10022_: vec3<f32>;
    var phi_10023_: bool;
    var phi_10028_: vec3<f32>;
    var phi_10029_: bool;
    var phi_22370_: bool;
    var phi_22385_: bool;
    var phi_10320_: vec3<f32>;
    var phi_9511_: vec3<f32>;
    var phi_9514_: vec3<f32>;
    var phi_9516_: vec3<f32>;
    var phi_9518_: vec3<f32>;
    var phi_9520_: u32;
    var phi_33383_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_33867_: bool;
    var phi_10377_: type_60;
    var phi_33866_: bool;
    var phi_10379_: type_60;
    var phi_10380_: bool;
    var phi_10385_: type_60;
    var phi_886_: vec3<f32>;
    var phi_888_: vec3<f32>;
    var phi_889_: bool;
    var phi_33475_: bool;
    var phi_899_: vec3<f32>;
    var phi_902_: i32;
    var phi_904_: u32;
    var phi_22654_: bool;
    var phi_1326_: u32;
    var phi_1356_: u32;
    var phi_1327_: u32;
    var phi_1357_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1375_: u32;
    var phi_2680_: type_55;
    var phi_2688_: type_52;
    var phi_2690_: type_52;
    var phi_2691_: bool;
    var phi_2692_: type_52;
    var phi_2693_: bool;
    var phi_2694_: bool;
    var phi_22765_: f32;
    var phi_22777_: type_56;
    var phi_22788_: type_57;
    var phi_22789_: type_57;
    var phi_22793_: type_57;
    var phi_22794_: type_57;
    var phi_22795_: bool;
    var phi_22799_: type_57;
    var phi_22686_: type_57;
    var phi_22837_: bool;
    var phi_22917_: f32;
    var phi_22929_: type_56;
    var phi_22940_: type_57;
    var phi_22941_: type_57;
    var phi_22945_: type_57;
    var phi_22946_: type_57;
    var phi_22947_: bool;
    var phi_22951_: type_57;
    var phi_22885_: f32;
    var phi_22989_: bool;
    var phi_22714_: f32;
    var phi_22716_: f32;
    var phi_22725_: type_57;
    var phi_22729_: type_57;
    var phi_22733_: type_57;
    var phi_2712_: vec3<f32>;
    var phi_2715_: vec3<f32>;
    var phi_2717_: vec3<f32>;
    var phi_2719_: vec3<f32>;
    var phi_2721_: u32;
    var phi_2805_: u32;
    var phi_2820_: type_56;
    var phi_2821_: type_56;
    var phi_2822_: type_56;
    var phi_23188_: bool;
    var phi_23315_: bool;
    var phi_23330_: bool;
    var phi_3150_: vec3<f32>;
    var phi_23452_: f32;
    var phi_23464_: type_56;
    var phi_23475_: type_57;
    var phi_23476_: type_57;
    var phi_23480_: type_57;
    var phi_23481_: type_57;
    var phi_23482_: bool;
    var phi_23486_: type_57;
    var phi_23420_: f32;
    var phi_23562_: f32;
    var phi_23574_: type_56;
    var phi_23585_: type_57;
    var phi_23586_: type_57;
    var phi_23590_: type_57;
    var phi_23591_: type_57;
    var phi_23592_: bool;
    var phi_23596_: type_57;
    var phi_23530_: f32;
    var phi_23357_: vec3<f32>;
    var phi_23358_: u32;
    var phi_23728_: bool;
    var phi_23391_: vec3<f32>;
    var phi_23392_: u32;
    var local_11: vec3<f32>;
    var phi_23399_: type_58;
    var phi_23403_: type_58;
    var phi_3209_: vec3<f32>;
    var phi_3210_: vec3<f32>;
    var phi_2713_: vec3<f32>;
    var phi_2716_: vec3<f32>;
    var phi_2718_: vec3<f32>;
    var phi_2720_: vec3<f32>;
    var phi_2722_: u32;
    var phi_33443_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_33537_: bool;
    var phi_3230_: type_52;
    var phi_33536_: bool;
    var phi_3231_: type_52;
    var phi_33535_: bool;
    var phi_3254_: vec3<f32>;
    var phi_3255_: i32;
    var phi_3256_: u32;
    var phi_33534_: bool;
    var phi_900_: vec3<f32>;
    var phi_903_: i32;
    var phi_905_: u32;
    var phi_3257_: bool;
    var phi_33533_: bool;
    var phi_3591_: vec3<f32>;
    var phi_3594_: u32;
    var phi_23856_: f32;
    var phi_23871_: f32;
    var phi_23886_: f32;
    var phi_3665_: i32;
    var phi_3695_: i32;
    var phi_3703_: vec3<f32>;
    var phi_3706_: i32;
    var phi_3729_: vec3<f32>;
    var phi_3732_: i32;
    var phi_3730_: vec3<f32>;
    var phi_3733_: i32;
    var phi_3704_: vec3<f32>;
    var phi_3707_: i32;
    var phi_3931_: vec3<f32>;
    var phi_3932_: vec3<f32>;
    var phi_3933_: vec3<f32>;
    var phi_3592_: vec3<f32>;
    var phi_3595_: u32;
    var phi_33538_: bool;
    var local_15: vec3<f32>;
    var phi_33586_: bool;
    var phi_3944_: f32;
    var phi_3968_: f32;
    var phi_3970_: u32;
    var phi_3973_: f32;
    var phi_4076_: f32;
    var phi_4077_: f32;
    var phi_4093_: f32;
    var phi_3971_: u32;
    var phi_3974_: f32;
    var phi_4096_: u32;
    var phi_4099_: f32;
    var phi_4207_: f32;
    var phi_4097_: u32;
    var phi_4100_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_24002_: f32;
    var phi_24014_: type_56;
    var phi_24025_: type_57;
    var phi_24026_: type_57;
    var phi_24030_: type_57;
    var phi_24031_: type_57;
    var phi_24032_: bool;
    var phi_24036_: type_57;
    var phi_23970_: f32;
    var phi_4608_: bool;
    var phi_24162_: f32;
    var phi_24174_: type_56;
    var phi_24185_: type_57;
    var phi_24186_: type_57;
    var phi_24190_: type_57;
    var phi_24191_: type_57;
    var phi_24192_: bool;
    var phi_24196_: type_57;
    var phi_24083_: type_57;
    var phi_24234_: bool;
    var phi_24314_: f32;
    var phi_24326_: type_56;
    var phi_24337_: type_57;
    var phi_24338_: type_57;
    var phi_24342_: type_57;
    var phi_24343_: type_57;
    var phi_24344_: bool;
    var phi_24348_: type_57;
    var phi_24282_: f32;
    var phi_24386_: bool;
    var phi_24111_: f32;
    var phi_24113_: f32;
    var phi_24122_: type_57;
    var phi_24126_: type_57;
    var phi_24130_: type_57;
    var phi_4622_: vec3<f32>;
    var phi_4625_: u32;
    var phi_4709_: u32;
    var phi_4724_: type_56;
    var phi_4725_: type_56;
    var phi_4726_: type_56;
    var phi_24585_: bool;
    var phi_4623_: vec3<f32>;
    var phi_4626_: u32;
    var phi_33565_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_33632_: bool;
    var phi_4751_: vec3<f32>;
    var phi_33631_: bool;
    var phi_4753_: vec3<f32>;
    var phi_4755_: bool;
    var phi_33627_: bool;
    var phi_4766_: vec3<f32>;
    var phi_4767_: bool;
    var phi_33626_: bool;
    var phi_4769_: vec3<f32>;
    var phi_4770_: bool;
    var phi_4771_: bool;
    var phi_33625_: bool;
    var phi_4775_: vec3<f32>;
    var phi_4776_: vec3<f32>;
    var phi_4777_: bool;
    var phi_4778_: vec3<f32>;
    var phi_4779_: bool;
    var phi_5099_: vec3<f32>;
    var phi_5102_: u32;
    var phi_24656_: f32;
    var phi_24671_: f32;
    var phi_24686_: f32;
    var phi_5173_: i32;
    var phi_5203_: i32;
    var phi_5211_: vec3<f32>;
    var phi_5214_: i32;
    var phi_5237_: vec3<f32>;
    var phi_5240_: i32;
    var phi_5311_: vec3<f32>;
    var phi_5314_: u32;
    var phi_5434_: vec3<f32>;
    var phi_5312_: vec3<f32>;
    var phi_5315_: u32;
    var phi_5238_: vec3<f32>;
    var phi_5241_: i32;
    var phi_5212_: vec3<f32>;
    var phi_5215_: i32;
    var phi_5439_: vec3<f32>;
    var phi_5440_: vec3<f32>;
    var phi_5441_: vec3<f32>;
    var phi_5100_: vec3<f32>;
    var phi_5103_: u32;
    var phi_33620_: bool;
    var phi_33662_: bool;
    var phi_5461_: vec3<f32>;
    var phi_5674_: u32;
    var phi_5675_: bool;
    var phi_5678_: u32;
    var phi_5679_: bool;
    var phi_5680_: bool;
    var phi_5690_: f32;
    var phi_5693_: f32;
    var phi_5695_: u32;
    var phi_5697_: vec3<f32>;
    var phi_5699_: vec3<f32>;
    var phi_5701_: f32;
    var phi_5703_: f32;
    var phi_5741_: u32;
    var phi_5756_: vec3<f32>;
    var phi_5776_: vec3<f32>;
    var phi_24701_: vec3<f32>;
    var phi_5777_: u32;
    var phi_5778_: vec3<f32>;
    var phi_5779_: u32;
    var phi_5780_: vec3<f32>;
    var phi_5781_: u32;
    var phi_5783_: vec3<f32>;
    var phi_5918_: u32;
    var phi_5921_: f32;
    var phi_5923_: f32;
    var phi_5925_: f32;
    var phi_5927_: vec3<f32>;
    var phi_24756_: f32;
    var phi_24762_: f32;
    var phi_24790_: f32;
    var phi_24796_: f32;
    var phi_24824_: f32;
    var phi_24830_: f32;
    var phi_24858_: f32;
    var phi_24864_: f32;
    var phi_24892_: f32;
    var phi_24898_: f32;
    var phi_24926_: f32;
    var phi_24932_: f32;
    var phi_24960_: f32;
    var phi_24966_: f32;
    var phi_24994_: f32;
    var phi_25000_: f32;
    var phi_5919_: u32;
    var phi_5922_: f32;
    var phi_5924_: f32;
    var phi_5926_: f32;
    var phi_5928_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6344_: f32;
    var phi_6396_: vec3<f32>;
    var phi_6397_: f32;
    var phi_6398_: vec3<f32>;
    var phi_6399_: f32;
    var phi_6400_: vec3<f32>;
    var phi_6401_: f32;
    var phi_6403_: vec3<f32>;
    var phi_6404_: f32;
    var phi_6405_: f32;
    var phi_6406_: vec3<f32>;
    var phi_6407_: vec3<f32>;
    var phi_6408_: f32;
    var phi_6409_: f32;
    var phi_6410_: vec3<f32>;
    var phi_6411_: vec3<f32>;
    var phi_6412_: f32;
    var phi_6413_: f32;
    var phi_6414_: vec3<f32>;
    var phi_6415_: vec3<f32>;
    var phi_6416_: f32;
    var phi_6417_: f32;
    var phi_6418_: vec3<f32>;
    var phi_6419_: vec3<f32>;
    var phi_6421_: f32;
    var phi_6422_: f32;
    var phi_6423_: vec3<f32>;
    var phi_6424_: vec3<f32>;
    var phi_6425_: f32;
    var phi_6426_: f32;
    var phi_6427_: f32;
    var phi_6428_: f32;
    var phi_6429_: vec3<f32>;
    var phi_6430_: vec3<f32>;
    var phi_6431_: f32;
    var phi_6432_: f32;
    var phi_5691_: f32;
    var phi_5694_: f32;
    var phi_5696_: u32;
    var phi_5698_: vec3<f32>;
    var phi_5700_: vec3<f32>;
    var phi_5702_: f32;
    var phi_5704_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_25076_: bool;
    var local_29: f32;
    var phi_6464_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6543_: f32;
    var phi_6544_: vec3<f32>;
    var phi_6545_: f32;
    var phi_6547_: vec3<f32>;
    var phi_6550_: u32;
    var phi_6998_: f32;
    var phi_7178_: u32;
    var phi_7181_: f32;
    var phi_7183_: f32;
    var phi_7185_: f32;
    var phi_7187_: vec3<f32>;
    var phi_25165_: f32;
    var phi_25171_: f32;
    var phi_25199_: f32;
    var phi_25205_: f32;
    var phi_25233_: f32;
    var phi_25239_: f32;
    var phi_25267_: f32;
    var phi_25273_: f32;
    var phi_25301_: f32;
    var phi_25307_: f32;
    var phi_25335_: f32;
    var phi_25341_: f32;
    var phi_25369_: f32;
    var phi_25375_: f32;
    var phi_25403_: f32;
    var phi_25409_: f32;
    var phi_7179_: u32;
    var phi_7182_: f32;
    var phi_7184_: f32;
    var phi_7186_: f32;
    var phi_7188_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7601_: f32;
    var phi_25455_: bool;
    var phi_25470_: bool;
    var phi_7694_: vec3<f32>;
    var phi_7695_: vec3<f32>;
    var phi_7696_: vec3<f32>;
    var phi_7697_: vec3<f32>;
    var phi_6548_: vec3<f32>;
    var phi_6551_: u32;
    var phi_7716_: vec3<f32>;
    var phi_7719_: u32;
    var phi_7721_: f32;
    var phi_8022_: vec3<f32>;
    var phi_8023_: u32;
    var phi_8024_: f32;
    var phi_8025_: vec3<f32>;
    var phi_8042_: vec3<f32>;
    var phi_8044_: u32;
    var phi_8047_: vec3<f32>;
    var phi_8049_: f32;
    var phi_8051_: f32;
    var phi_8053_: f32;
    var phi_8073_: bool;
    var phi_8089_: bool;
    var phi_8091_: bool;
    var phi_8093_: bool;
    var phi_8094_: bool;
    var phi_8095_: bool;
    var phi_8096_: bool;
    var phi_8097_: bool;
    var phi_8141_: vec3<f32>;
    var phi_25489_: vec3<f32>;
    var phi_8142_: u32;
    var phi_8143_: vec3<f32>;
    var phi_8144_: u32;
    var phi_8145_: vec3<f32>;
    var phi_8280_: u32;
    var phi_8283_: f32;
    var phi_8285_: f32;
    var phi_8287_: f32;
    var phi_8289_: vec3<f32>;
    var phi_25544_: f32;
    var phi_25550_: f32;
    var phi_25578_: f32;
    var phi_25584_: f32;
    var phi_25612_: f32;
    var phi_25618_: f32;
    var phi_25646_: f32;
    var phi_25652_: f32;
    var phi_25680_: f32;
    var phi_25686_: f32;
    var phi_25714_: f32;
    var phi_25720_: f32;
    var phi_25748_: f32;
    var phi_25754_: f32;
    var phi_25782_: f32;
    var phi_25788_: f32;
    var phi_8281_: u32;
    var phi_8284_: f32;
    var phi_8286_: f32;
    var phi_8288_: f32;
    var phi_8290_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8703_: f32;
    var phi_8729_: vec3<f32>;
    var phi_8730_: f32;
    var phi_8731_: f32;
    var phi_8732_: vec3<f32>;
    var phi_8733_: f32;
    var phi_8734_: f32;
    var phi_8735_: vec3<f32>;
    var phi_8736_: f32;
    var phi_8737_: f32;
    var phi_8738_: vec3<f32>;
    var phi_8739_: f32;
    var phi_8740_: f32;
    var phi_8742_: vec3<f32>;
    var phi_8743_: f32;
    var phi_8744_: f32;
    var phi_8745_: f32;
    var phi_8045_: u32;
    var phi_8048_: vec3<f32>;
    var phi_8050_: f32;
    var phi_8052_: f32;
    var phi_8054_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9035_: type_59;
    var local_42: f32;
    var phi_9036_: type_59;
    var local_43: f32;
    var local_44: f32;
    var phi_9037_: type_59;
    var phi_9038_: type_59;
    var phi_7717_: vec3<f32>;
    var phi_7720_: u32;
    var phi_7722_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9067_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_33870_: bool;
    var phi_9114_: vec3<f32>;
    var phi_33869_: bool;
    var phi_9115_: vec3<f32>;
    var phi_33868_: bool;
    var phi_9116_: vec3<f32>;
    var phi_33864_: bool;
    var phi_10389_: vec3<f32>;
    var phi_33863_: bool;
    var phi_10390_: vec3<f32>;
    var phi_33862_: bool;
    var phi_508_: vec3<f32>;
    var phi_511_: u32;
    var phi_33861_: bool;
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
            let _e276 = global;
            let _e286 = global_1.member.member_7[2u];
            let _e290 = (_e276.x + (_e276.y * 4194240u));
            let _e295 = atomicLoad((&global_8.member[((_e286 * 4u) + 3u)]));
            if (_e290 >= _e295) {
                phi_487_ = type_48(0u, type_48().member_1);
            } else {
                let _e300 = global_1.member.member[0u];
                let _e305 = global_1.member.member[1u];
                let _e311 = atomicLoad((&global_8.member[((8u + ((_e286 * _e300) * _e305)) + _e290)]));
                phi_487_ = type_48(1u, _e311);
            }
            let _e317 = phi_487_;
            if (bitcast<i32>(_e317.member) != 0i) {
                phi_33400_ = false;
                phi_507_ = vec3<f32>(0f, 0f, 0f);
                phi_510_ = 0u;
                loop {
                    let _e323 = phi_33400_;
                    let _e325 = phi_507_;
                    let _e327 = phi_510_;
                    local_55 = _e325;
                    local_56 = _e325;
                    local_57 = _e325;
                    let _e328 = (_e327 < 2u);
                    if _e328 {
                        let _e330 = ((2u * _e317.member_1) + _e327);
                        if (_e330 < arrayLength((&global_9.member))) {
                        } else {
                            phi_33861_ = true;
                            break;
                        }
                        let _e334 = global_9.member[_e330];
                        if ((((_e334.member_2.x * _e334.member_2.x) + (_e334.member_2.y * _e334.member_2.y)) + (_e334.member_2.z * _e334.member_2.z)) == 0f) {
                            phi_33863_ = _e323;
                            phi_10390_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e334.member.w < 0f) {
                                let _e1289 = global_1.member.member_8[0u];
                                let _e1292 = global_1.member.member_8[3u];
                                if (_e1289 == 4294967295u) {
                                    let _e1294 = (_e1292 == 4294967295u);
                                    if _e1294 {
                                        let _e1297 = global_1.member.member_3[1u];
                                        let _e1300 = global_1.member.member_3[2u];
                                        let _e1303 = ((atan2(_e334.member_1.z, _e334.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1308 = (((_e1303 - floor(_e1303)) * f32(_e1297)) - 0.5f);
                                        let _e1310 = select(_e334.member_1.y, -1f, (_e334.member_1.y < -1f));
                                        let _e1317 = (((acos(select(_e1310, 1f, (_e1310 > 1f))) * 0.31830987f) * f32(_e1300)) - 0.5f);
                                        let _e1318 = (_e1300 - 1u);
                                        let _e1319 = f32(_e1318);
                                        if (0f <= _e1319) {
                                        } else {
                                            phi_33861_ = true;
                                            break;
                                        }
                                        let _e1322 = select(_e1317, 0f, (_e1317 < 0f));
                                        let _e1324 = select(_e1322, _e1319, (_e1322 > _e1319));
                                        let _e1325 = floor(_e1308);
                                        let _e1326 = floor(_e1324);
                                        let _e1327 = (_e1308 - _e1325);
                                        let _e1328 = (_e1324 - _e1326);
                                        let _e1339 = (_e1297 == 0u);
                                        if _e1339 {
                                            phi_33861_ = true;
                                            break;
                                        }
                                        let _e1340 = (bitcast<u32>((select(0i, select(select(i32(_e1325), i32(-2147483648), (_e1325 < -2147483600f)), 2147483647i, (_e1325 > 2147483500f)), (_e1325 == _e1325)) + bitcast<i32>(_e1297))) % _e1297);
                                        let _e1345 = select(select(u32(_e1326), 0u, (_e1326 < 0f)), 4294967295u, (_e1326 > 4294967000f));
                                        let _e1346 = (_e1345 + 1u);
                                        let _e1351 = global_1.member.member_2[2u];
                                        let _e1352 = (_e1345 * _e1297);
                                        if _e1339 {
                                            phi_33861_ = true;
                                            break;
                                        }
                                        let _e1353 = (_e1340 % _e1297);
                                        let _e1356 = (_e1351 + (3u * (_e1352 + _e1353)));
                                        let _e1358 = global_6.member[_e1356];
                                        let _e1361 = global_6.member[(_e1356 + 1u)];
                                        let _e1364 = global_6.member[(_e1356 + 2u)];
                                        if _e1339 {
                                            phi_33861_ = true;
                                            break;
                                        }
                                        let _e1367 = ((_e1340 + 1u) % _e1297);
                                        let _e1370 = (_e1351 + (3u * (_e1352 + _e1367)));
                                        let _e1372 = global_6.member[_e1370];
                                        let _e1375 = global_6.member[(_e1370 + 1u)];
                                        let _e1378 = global_6.member[(_e1370 + 2u)];
                                        let _e1386 = (vec3<f32>(_e1358, _e1361, _e1364) + vec3<f32>(((_e1372 - _e1358) * _e1327), ((_e1375 - _e1361) * _e1327), ((_e1378 - _e1364) * _e1327)));
                                        let _e1387 = (select(_e1346, _e1318, (_e1318 < _e1346)) * _e1297);
                                        if _e1339 {
                                            phi_33861_ = true;
                                            break;
                                        }
                                        let _e1390 = (_e1351 + (3u * (_e1387 + _e1353)));
                                        let _e1392 = global_6.member[_e1390];
                                        let _e1395 = global_6.member[(_e1390 + 1u)];
                                        let _e1398 = global_6.member[(_e1390 + 2u)];
                                        if _e1339 {
                                            phi_33861_ = true;
                                            break;
                                        }
                                        let _e1402 = (_e1351 + (3u * (_e1387 + _e1367)));
                                        let _e1404 = global_6.member[_e1402];
                                        let _e1407 = global_6.member[(_e1402 + 1u)];
                                        let _e1410 = global_6.member[(_e1402 + 2u)];
                                        let _e1418 = (vec3<f32>(_e1392, _e1395, _e1398) + vec3<f32>(((_e1404 - _e1392) * _e1327), ((_e1407 - _e1395) * _e1327), ((_e1410 - _e1398) * _e1327)));
                                        phi_886_ = (_e1386 + vec3<f32>(((_e1418.x - _e1386.x) * _e1328), ((_e1418.y - _e1386.y) * _e1328), ((_e1418.z - _e1386.z) * _e1328)));
                                    } else {
                                        phi_886_ = vec3<f32>();
                                    }
                                    let _e1434 = phi_886_;
                                    phi_888_ = _e1434;
                                    phi_889_ = select(true, false, _e1294);
                                } else {
                                    phi_888_ = vec3<f32>();
                                    phi_889_ = true;
                                }
                                let _e1437 = phi_888_;
                                let _e1439 = phi_889_;
                                if _e1439 {
                                    let _e1440 = (_e1292 != 4294967295u);
                                    if (_e1289 != 4294967295u) {
                                        phi_33475_ = _e323;
                                        phi_899_ = vec3<f32>(0f, 0f, 0f);
                                        phi_902_ = 0i;
                                        phi_904_ = _e1289;
                                        loop {
                                            let _e1443 = phi_33475_;
                                            let _e1445 = phi_899_;
                                            let _e1447 = phi_902_;
                                            let _e1449 = phi_904_;
                                            local_17 = _e1445;
                                            local_18 = _e1445;
                                            local_19 = _e1445;
                                            local_61 = _e1445;
                                            local_62 = _e1445;
                                            if (_e1449 != 4294967295u) {
                                                let _e1451 = (_e1447 < 2i);
                                                if _e1451 {
                                                    let _e1453 = global_6.member[_e1449];
                                                    let _e1456 = global_6.member[(_e1449 + 1u)];
                                                    let _e1459 = global_6.member[(_e1449 + 2u)];
                                                    let _e1462 = global_6.member[(_e1449 + 3u)];
                                                    let _e1465 = global_6.member[(_e1449 + 4u)];
                                                    let _e1468 = global_6.member[(_e1449 + 5u)];
                                                    let _e1471 = global_6.member[(_e1449 + 6u)];
                                                    let _e1474 = global_6.member[(_e1449 + 7u)];
                                                    let _e1477 = global_6.member[(_e1449 + 8u)];
                                                    let _e1480 = global_6.member[(_e1449 + 9u)];
                                                    let _e1483 = global_6.member[(_e1449 + 10u)];
                                                    let _e1486 = global_6.member[(_e1449 + 11u)];
                                                    let _e1489 = global_6.member[(_e1449 + 12u)];
                                                    let _e1492 = global_6.member[(_e1449 + 13u)];
                                                    let _e1495 = global_6.member[(_e1449 + 14u)];
                                                    let _e1498 = global_6.member[(_e1449 + 15u)];
                                                    let _e1502 = global_6.member[(_e1449 + 16u)];
                                                    let _e1505 = global_6.member[(_e1449 + 17u)];
                                                    let _e1508 = global_6.member[(_e1449 + 18u)];
                                                    let _e1511 = global_6.member[(_e1449 + 19u)];
                                                    let _e1514 = global_6.member[(_e1449 + 20u)];
                                                    let _e1517 = global_6.member[(_e1449 + 30u)];
                                                    let _e1518 = bitcast<u32>(_e1517);
                                                    let _e1521 = global_6.member[(_e1449 + 31u)];
                                                    let _e1522 = bitcast<u32>(_e1521);
                                                    let _e1525 = global_6.member[(_e1449 + 32u)];
                                                    let _e1527 = (_e334.member_1.y < 0f);
                                                    if _e1527 {
                                                        phi_2692_ = type_52();
                                                        phi_2693_ = true;
                                                        phi_2694_ = false;
                                                    } else {
                                                        let _e1528 = (_e1522 == 4294967295u);
                                                        if _e1528 {
                                                            phi_2690_ = type_52();
                                                            phi_2691_ = false;
                                                        } else {
                                                            let _e1542 = (_e334.member.x - _e1453);
                                                            let _e1543 = (_e334.member.y - _e1456);
                                                            let _e1544 = (_e334.member.z - _e1459);
                                                            let _e1552 = ((((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e1453) + (_e334.member.y * _e1456)) + (_e334.member.z * _e1459)))) + _e1468) / (sqrt((((_e1542 * _e1542) + (_e1543 * _e1543)) + (_e1544 * _e1544))) + _e1462));
                                                            let _e1553 = (_e1449 + 21u);
                                                            let _e1556 = global_6.member[(_e1449 + 28u)];
                                                            if (_e1552 > _e1556) {
                                                                phi_2680_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                let _e1559 = global_6.member[_e1553];
                                                                if (_e1552 != _e1552) {
                                                                    phi_22654_ = true;
                                                                } else {
                                                                    phi_22654_ = (_e1559 >= _e1552);
                                                                }
                                                                let _e1563 = phi_22654_;
                                                                let _e1564 = select(_e1552, _e1559, _e1563);
                                                                phi_1326_ = 0u;
                                                                loop {
                                                                    let _e1566 = phi_1326_;
                                                                    local_8 = _e1566;
                                                                    local_9 = _e1566;
                                                                    local_10 = _e1566;
                                                                    if (_e1566 < 8u) {
                                                                        let _e1570 = global_6.member[(_e1553 + _e1566)];
                                                                        let _e1571 = (_e1570 <= _e1564);
                                                                        if _e1571 {
                                                                            phi_1356_ = (_e1566 + 1u);
                                                                        } else {
                                                                            phi_1356_ = u32();
                                                                        }
                                                                        let _e1574 = phi_1356_;
                                                                        phi_1327_ = _e1574;
                                                                        phi_1357_ = _e1571;
                                                                    } else {
                                                                        phi_1327_ = u32();
                                                                        phi_1357_ = false;
                                                                    }
                                                                    let _e1576 = phi_1327_;
                                                                    let _e1578 = phi_1357_;
                                                                    phi_1326_ = _e1576;
                                                                    if !(_e1578) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1581 = local_8;
                                                                if (_e1581 < 1u) {
                                                                    phi_1375_ = 1u;
                                                                } else {
                                                                    let _e1584 = local_9;
                                                                    let _e1587 = local_10;
                                                                    phi_1375_ = select(_e1587, 7u, (_e1584 > 7u));
                                                                }
                                                                let _e1590 = phi_1375_;
                                                                let _e1591 = (_e1590 - 1u);
                                                                let _e1594 = global_6.member[(_e1553 + _e1591)];
                                                                let _e1597 = global_6.member[(_e1553 + _e1590)];
                                                                let _e1600 = ((_e1564 - _e1594) / (_e1597 - _e1594));
                                                                let _e1601 = select(_e334.member_1.y, 0f, _e1527);
                                                                let _e1607 = (sqrt((asin(select(_e1601, 1f, (_e1601 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1609 = (atan2(_e334.member_1.z, _e334.member_1.x) * 0.15915494f);
                                                                let _e1612 = ((_e1609 - floor(_e1609)) * 256f);
                                                                let _e1613 = floor(_e1612);
                                                                let _e1619 = (select(select(u32(_e1613), 0u, (_e1613 < 0f)), 4294967295u, (_e1613 > 4294967000f)) % 256u);
                                                                let _e1620 = floor(_e1607);
                                                                let _e1625 = select(select(u32(_e1620), 0u, (_e1620 < 0f)), 4294967295u, (_e1620 > 4294967000f));
                                                                let _e1627 = select(_e1625, 126u, (126u < _e1625));
                                                                let _e1629 = ((_e1619 + 1u) % 256u);
                                                                let _e1631 = (_e1612 - _e1613);
                                                                let _e1633 = (_e1607 - f32(_e1627));
                                                                let _e1635 = (_e1522 + (_e1591 * 294912u));
                                                                let _e1636 = (_e1627 * 256u);
                                                                let _e1638 = (9u * (_e1636 + _e1619));
                                                                let _e1639 = (_e1635 + _e1638);
                                                                let _e1641 = (9u * (_e1636 + _e1629));
                                                                let _e1642 = (_e1635 + _e1641);
                                                                let _e1643 = ((_e1627 + 1u) * 256u);
                                                                let _e1645 = (9u * (_e1643 + _e1619));
                                                                let _e1646 = (_e1635 + _e1645);
                                                                let _e1648 = (9u * (_e1643 + _e1629));
                                                                let _e1649 = (_e1635 + _e1648);
                                                                let _e1650 = (1f - _e1631);
                                                                let _e1651 = (1f - _e1633);
                                                                let _e1652 = (_e1650 * _e1651);
                                                                let _e1653 = (_e1631 * _e1651);
                                                                let _e1654 = (_e1650 * _e1633);
                                                                let _e1655 = (_e1631 * _e1633);
                                                                let _e1657 = global_6.member[_e1639];
                                                                let _e1660 = global_6.member[(_e1639 + 1u)];
                                                                let _e1663 = global_6.member[(_e1639 + 2u)];
                                                                let _e1668 = global_6.member[_e1642];
                                                                let _e1671 = global_6.member[(_e1642 + 1u)];
                                                                let _e1674 = global_6.member[(_e1642 + 2u)];
                                                                let _e1682 = global_6.member[_e1646];
                                                                let _e1685 = global_6.member[(_e1646 + 1u)];
                                                                let _e1688 = global_6.member[(_e1646 + 2u)];
                                                                let _e1696 = global_6.member[_e1649];
                                                                let _e1699 = global_6.member[(_e1649 + 1u)];
                                                                let _e1702 = global_6.member[(_e1649 + 2u)];
                                                                let _e1711 = global_6.member[(_e1639 + 3u)];
                                                                let _e1714 = global_6.member[(_e1639 + 4u)];
                                                                let _e1717 = global_6.member[(_e1639 + 5u)];
                                                                let _e1723 = global_6.member[(_e1642 + 3u)];
                                                                let _e1726 = global_6.member[(_e1642 + 4u)];
                                                                let _e1729 = global_6.member[(_e1642 + 5u)];
                                                                let _e1738 = global_6.member[(_e1646 + 3u)];
                                                                let _e1741 = global_6.member[(_e1646 + 4u)];
                                                                let _e1744 = global_6.member[(_e1646 + 5u)];
                                                                let _e1753 = global_6.member[(_e1649 + 3u)];
                                                                let _e1756 = global_6.member[(_e1649 + 4u)];
                                                                let _e1759 = global_6.member[(_e1649 + 5u)];
                                                                let _e1768 = global_6.member[(_e1639 + 6u)];
                                                                let _e1771 = global_6.member[(_e1639 + 7u)];
                                                                let _e1774 = global_6.member[(_e1639 + 8u)];
                                                                let _e1780 = global_6.member[(_e1642 + 6u)];
                                                                let _e1783 = global_6.member[(_e1642 + 7u)];
                                                                let _e1786 = global_6.member[(_e1642 + 8u)];
                                                                let _e1795 = global_6.member[(_e1646 + 6u)];
                                                                let _e1798 = global_6.member[(_e1646 + 7u)];
                                                                let _e1801 = global_6.member[(_e1646 + 8u)];
                                                                let _e1810 = global_6.member[(_e1649 + 6u)];
                                                                let _e1813 = global_6.member[(_e1649 + 7u)];
                                                                let _e1816 = global_6.member[(_e1649 + 8u)];
                                                                let _e1824 = (_e1522 + (_e1590 * 294912u));
                                                                let _e1825 = (_e1824 + _e1638);
                                                                let _e1826 = (_e1824 + _e1641);
                                                                let _e1827 = (_e1824 + _e1645);
                                                                let _e1828 = (_e1824 + _e1648);
                                                                let _e1830 = global_6.member[_e1825];
                                                                let _e1833 = global_6.member[(_e1825 + 1u)];
                                                                let _e1836 = global_6.member[(_e1825 + 2u)];
                                                                let _e1841 = global_6.member[_e1826];
                                                                let _e1844 = global_6.member[(_e1826 + 1u)];
                                                                let _e1847 = global_6.member[(_e1826 + 2u)];
                                                                let _e1855 = global_6.member[_e1827];
                                                                let _e1858 = global_6.member[(_e1827 + 1u)];
                                                                let _e1861 = global_6.member[(_e1827 + 2u)];
                                                                let _e1869 = global_6.member[_e1828];
                                                                let _e1872 = global_6.member[(_e1828 + 1u)];
                                                                let _e1875 = global_6.member[(_e1828 + 2u)];
                                                                let _e1884 = global_6.member[(_e1825 + 3u)];
                                                                let _e1887 = global_6.member[(_e1825 + 4u)];
                                                                let _e1890 = global_6.member[(_e1825 + 5u)];
                                                                let _e1896 = global_6.member[(_e1826 + 3u)];
                                                                let _e1899 = global_6.member[(_e1826 + 4u)];
                                                                let _e1902 = global_6.member[(_e1826 + 5u)];
                                                                let _e1911 = global_6.member[(_e1827 + 3u)];
                                                                let _e1914 = global_6.member[(_e1827 + 4u)];
                                                                let _e1917 = global_6.member[(_e1827 + 5u)];
                                                                let _e1926 = global_6.member[(_e1828 + 3u)];
                                                                let _e1929 = global_6.member[(_e1828 + 4u)];
                                                                let _e1932 = global_6.member[(_e1828 + 5u)];
                                                                let _e1941 = global_6.member[(_e1825 + 6u)];
                                                                let _e1944 = global_6.member[(_e1825 + 7u)];
                                                                let _e1947 = global_6.member[(_e1825 + 8u)];
                                                                let _e1953 = global_6.member[(_e1826 + 6u)];
                                                                let _e1956 = global_6.member[(_e1826 + 7u)];
                                                                let _e1959 = global_6.member[(_e1826 + 8u)];
                                                                let _e1968 = global_6.member[(_e1827 + 6u)];
                                                                let _e1971 = global_6.member[(_e1827 + 7u)];
                                                                let _e1974 = global_6.member[(_e1827 + 8u)];
                                                                let _e1983 = global_6.member[(_e1828 + 6u)];
                                                                let _e1986 = global_6.member[(_e1828 + 7u)];
                                                                let _e1989 = global_6.member[(_e1828 + 8u)];
                                                                let _e1996 = (1f - _e1600);
                                                                phi_2680_ = type_55(1u, type_52(vec3<f32>(((((((_e1657 * _e1652) + (_e1668 * _e1653)) + (_e1682 * _e1654)) + (_e1696 * _e1655)) * _e1996) + (((((_e1830 * _e1652) + (_e1841 * _e1653)) + (_e1855 * _e1654)) + (_e1869 * _e1655)) * _e1600)), ((((((_e1660 * _e1652) + (_e1671 * _e1653)) + (_e1685 * _e1654)) + (_e1699 * _e1655)) * _e1996) + (((((_e1833 * _e1652) + (_e1844 * _e1653)) + (_e1858 * _e1654)) + (_e1872 * _e1655)) * _e1600)), ((((((_e1663 * _e1652) + (_e1674 * _e1653)) + (_e1688 * _e1654)) + (_e1702 * _e1655)) * _e1996) + (((((_e1836 * _e1652) + (_e1847 * _e1653)) + (_e1861 * _e1654)) + (_e1875 * _e1655)) * _e1600))), vec3<f32>(((((((_e1711 * _e1652) + (_e1723 * _e1653)) + (_e1738 * _e1654)) + (_e1753 * _e1655)) * _e1996) + (((((_e1884 * _e1652) + (_e1896 * _e1653)) + (_e1911 * _e1654)) + (_e1926 * _e1655)) * _e1600)), ((((((_e1714 * _e1652) + (_e1726 * _e1653)) + (_e1741 * _e1654)) + (_e1756 * _e1655)) * _e1996) + (((((_e1887 * _e1652) + (_e1899 * _e1653)) + (_e1914 * _e1654)) + (_e1929 * _e1655)) * _e1600)), ((((((_e1717 * _e1652) + (_e1729 * _e1653)) + (_e1744 * _e1654)) + (_e1759 * _e1655)) * _e1996) + (((((_e1890 * _e1652) + (_e1902 * _e1653)) + (_e1917 * _e1654)) + (_e1932 * _e1655)) * _e1600))), vec3<f32>(((((((_e1768 * _e1652) + (_e1780 * _e1653)) + (_e1795 * _e1654)) + (_e1810 * _e1655)) * _e1996) + (((((_e1941 * _e1652) + (_e1953 * _e1653)) + (_e1968 * _e1654)) + (_e1983 * _e1655)) * _e1600)), ((((((_e1771 * _e1652) + (_e1783 * _e1653)) + (_e1798 * _e1654)) + (_e1813 * _e1655)) * _e1996) + (((((_e1944 * _e1652) + (_e1956 * _e1653)) + (_e1971 * _e1654)) + (_e1986 * _e1655)) * _e1600)), ((((((_e1774 * _e1652) + (_e1786 * _e1653)) + (_e1801 * _e1654)) + (_e1816 * _e1655)) * _e1996) + (((((_e1947 * _e1652) + (_e1959 * _e1653)) + (_e1974 * _e1654)) + (_e1989 * _e1655)) * _e1600)))));
                                                            }
                                                            let _e2033 = phi_2680_;
                                                            let _e2036 = (bitcast<i32>(_e2033.member) != 0i);
                                                            if _e2036 {
                                                                phi_2688_ = _e2033.member_1;
                                                            } else {
                                                                phi_2688_ = type_52();
                                                            }
                                                            let _e2039 = phi_2688_;
                                                            phi_2690_ = _e2039;
                                                            phi_2691_ = select(true, false, _e2036);
                                                        }
                                                        let _e2042 = phi_2690_;
                                                        let _e2044 = phi_2691_;
                                                        phi_2692_ = _e2042;
                                                        phi_2693_ = _e1528;
                                                        phi_2694_ = _e2044;
                                                    }
                                                    let _e2046 = phi_2692_;
                                                    let _e2048 = phi_2693_;
                                                    let _e2050 = phi_2694_;
                                                    if select(_e2050, true, _e2048) {
                                                        let _e2059 = ((((_e334.member.x - _e1453) * _e334.member_1.x) + ((_e334.member.y - _e1456) * _e334.member_1.y)) + ((_e334.member.z - _e1459) * _e334.member_1.z));
                                                        let _e2071 = ((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e1453) + (_e334.member.y * _e1456)) + (_e334.member.z * _e1459))));
                                                        let _e2072 = (_e2071 + _e1471);
                                                        let _e2073 = (_e2059 * _e2059);
                                                        let _e2074 = (_e2073 - _e2072);
                                                        if (_e2074 < 0f) {
                                                            phi_22793_ = type_57(0u, type_57().member_1);
                                                            phi_22794_ = type_57();
                                                            phi_22795_ = true;
                                                        } else {
                                                            if (_e2059 < 0f) {
                                                                phi_22765_ = -(sqrt(_e2074));
                                                            } else {
                                                                phi_22765_ = sqrt(_e2074);
                                                            }
                                                            let _e2081 = phi_22765_;
                                                            let _e2083 = -((_e2059 + _e2081));
                                                            let _e2084 = (_e2083 == 0f);
                                                            if _e2084 {
                                                                phi_22788_ = type_57(1u, type_56(0f, 0f));
                                                                phi_22789_ = type_57();
                                                            } else {
                                                                let _e2085 = (_e2072 / _e2083);
                                                                if (_e2083 < _e2085) {
                                                                    phi_22777_ = type_56(_e2083, _e2085);
                                                                } else {
                                                                    phi_22777_ = type_56(_e2085, _e2083);
                                                                }
                                                                let _e2090 = phi_22777_;
                                                                phi_22788_ = type_57();
                                                                phi_22789_ = type_57(1u, _e2090);
                                                            }
                                                            let _e2093 = phi_22788_;
                                                            let _e2095 = phi_22789_;
                                                            phi_22793_ = _e2093;
                                                            phi_22794_ = _e2095;
                                                            phi_22795_ = _e2084;
                                                        }
                                                        let _e2100 = phi_22793_;
                                                        let _e2102 = phi_22794_;
                                                        let _e2104 = phi_22795_;
                                                        if _e2104 {
                                                            phi_22799_ = _e2100;
                                                        } else {
                                                            phi_22799_ = _e2102;
                                                        }
                                                        let _e2106 = phi_22799_;
                                                        if (bitcast<i32>(_e2106.member) != 0i) {
                                                            phi_22686_ = type_57(0u, _e2106.member_1);
                                                        } else {
                                                            phi_22686_ = type_57(1u, type_57().member_1);
                                                        }
                                                        let _e2116 = phi_22686_;
                                                        if (bitcast<i32>(_e2116.member) != 0i) {
                                                            phi_22733_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e2116.member_1.member_1 <= 0f) {
                                                                phi_22729_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e2116.member_1.member != _e2116.member_1.member) {
                                                                    phi_22837_ = true;
                                                                } else {
                                                                    phi_22837_ = (0f >= _e2116.member_1.member);
                                                                }
                                                                let _e2128 = phi_22837_;
                                                                let _e2129 = select(_e2116.member_1.member, 0f, _e2128);
                                                                let _e2130 = (_e2071 + _e1468);
                                                                if (_e2130 > 0f) {
                                                                    let _e2132 = (_e2073 - _e2130);
                                                                    if (_e2132 < 0f) {
                                                                        phi_22945_ = type_57(0u, type_57().member_1);
                                                                        phi_22946_ = type_57();
                                                                        phi_22947_ = true;
                                                                    } else {
                                                                        if (_e2059 < 0f) {
                                                                            phi_22917_ = -(sqrt(_e2132));
                                                                        } else {
                                                                            phi_22917_ = sqrt(_e2132);
                                                                        }
                                                                        let _e2139 = phi_22917_;
                                                                        let _e2141 = -((_e2059 + _e2139));
                                                                        let _e2142 = (_e2141 == 0f);
                                                                        if _e2142 {
                                                                            phi_22940_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_22941_ = type_57();
                                                                        } else {
                                                                            let _e2143 = (_e2130 / _e2141);
                                                                            if (_e2141 < _e2143) {
                                                                                phi_22929_ = type_56(_e2141, _e2143);
                                                                            } else {
                                                                                phi_22929_ = type_56(_e2143, _e2141);
                                                                            }
                                                                            let _e2148 = phi_22929_;
                                                                            phi_22940_ = type_57();
                                                                            phi_22941_ = type_57(1u, _e2148);
                                                                        }
                                                                        let _e2151 = phi_22940_;
                                                                        let _e2153 = phi_22941_;
                                                                        phi_22945_ = _e2151;
                                                                        phi_22946_ = _e2153;
                                                                        phi_22947_ = _e2142;
                                                                    }
                                                                    let _e2158 = phi_22945_;
                                                                    let _e2160 = phi_22946_;
                                                                    let _e2162 = phi_22947_;
                                                                    if _e2162 {
                                                                        phi_22951_ = _e2158;
                                                                    } else {
                                                                        phi_22951_ = _e2160;
                                                                    }
                                                                    let _e2164 = phi_22951_;
                                                                    if (bitcast<i32>(_e2164.member) != 0i) {
                                                                        phi_22885_ = select(_e2164.member_1.member_1, _e2164.member_1.member, (_e2164.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22885_ = -1f;
                                                                    }
                                                                    let _e2175 = phi_22885_;
                                                                    if (_e2175 >= 0f) {
                                                                        if (_e2116.member_1.member_1 != _e2116.member_1.member_1) {
                                                                            phi_22989_ = true;
                                                                        } else {
                                                                            phi_22989_ = (_e2175 <= _e2116.member_1.member_1);
                                                                        }
                                                                        let _e2180 = phi_22989_;
                                                                        phi_22714_ = select(_e2116.member_1.member_1, _e2175, _e2180);
                                                                    } else {
                                                                        phi_22714_ = _e2116.member_1.member_1;
                                                                    }
                                                                    let _e2183 = phi_22714_;
                                                                    phi_22716_ = _e2183;
                                                                } else {
                                                                    phi_22716_ = _e2116.member_1.member_1;
                                                                }
                                                                let _e2185 = phi_22716_;
                                                                if (_e2185 <= _e2129) {
                                                                    phi_22725_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    phi_22725_ = type_57(1u, type_56(_e2129, _e2185));
                                                                }
                                                                let _e2193 = phi_22725_;
                                                                phi_22729_ = _e2193;
                                                            }
                                                            let _e2198 = phi_22729_;
                                                            phi_22733_ = _e2198;
                                                        }
                                                        let _e2203 = phi_22733_;
                                                        if (bitcast<i32>(_e2203.member) != 0i) {
                                                            phi_2712_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2715_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2717_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2719_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2721_ = 0u;
                                                            loop {
                                                                let _e2212 = phi_2712_;
                                                                let _e2214 = phi_2715_;
                                                                let _e2216 = phi_2717_;
                                                                let _e2218 = phi_2719_;
                                                                let _e2220 = phi_2721_;
                                                                local_12 = _e2218;
                                                                local_13 = _e2216;
                                                                local_14 = _e2214;
                                                                let _e2221 = (_e2220 < 32u);
                                                                if _e2221 {
                                                                    let _e2222 = -(_e2059);
                                                                    if (_e2203.member_1.member <= _e2203.member_1.member_1) {
                                                                    } else {
                                                                        phi_33443_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2225 = select(_e2222, _e2203.member_1.member, (_e2222 < _e2203.member_1.member));
                                                                    let _e2227 = select(_e2225, _e2203.member_1.member_1, (_e2225 > _e2203.member_1.member_1));
                                                                    let _e2228 = (_e2227 - _e2203.member_1.member);
                                                                    let _e2229 = (_e2203.member_1.member_1 - _e2227);
                                                                    let _e2230 = (_e2203.member_1.member_1 - _e2203.member_1.member);
                                                                    let _e2231 = (0.000000001f * _e2230);
                                                                    if (_e2228 <= _e2231) {
                                                                        let _e2306 = (f32(_e2220) * 0.03125f);
                                                                        let _e2309 = (f32((_e2220 + 1u)) * 0.03125f);
                                                                        let _e2311 = ((_e2306 + _e2309) * 0.5f);
                                                                        phi_2822_ = type_56((_e2203.member_1.member + ((_e2230 * _e2311) * _e2311)), (_e2230 * ((_e2309 * _e2309) - (_e2306 * _e2306))));
                                                                    } else {
                                                                        if (_e2229 <= _e2231) {
                                                                            let _e2289 = (f32((31u - _e2220)) * 0.03125f);
                                                                            let _e2292 = (f32((32u - _e2220)) * 0.03125f);
                                                                            let _e2294 = ((_e2289 + _e2292) * 0.5f);
                                                                            phi_2821_ = type_56((_e2203.member_1.member_1 - ((_e2230 * _e2294) * _e2294)), (_e2230 * ((_e2292 * _e2292) - (_e2289 * _e2289))));
                                                                        } else {
                                                                            let _e2237 = round(((32f * _e2228) / (_e2228 + _e2229)));
                                                                            let _e2242 = select(select(u32(_e2237), 0u, (_e2237 < 0f)), 4294967295u, (_e2237 > 4294967000f));
                                                                            if (_e2242 < 1u) {
                                                                                phi_2805_ = 1u;
                                                                            } else {
                                                                                phi_2805_ = select(_e2242, 31u, (_e2242 > 31u));
                                                                            }
                                                                            let _e2247 = phi_2805_;
                                                                            if (_e2220 < _e2247) {
                                                                                let _e2268 = ((_e2247 - 1u) - _e2220);
                                                                                let _e2270 = f32(_e2247);
                                                                                let _e2271 = (f32(_e2268) / _e2270);
                                                                                let _e2274 = (f32((_e2268 + 1u)) / _e2270);
                                                                                let _e2276 = ((_e2271 + _e2274) * 0.5f);
                                                                                phi_2820_ = type_56((_e2227 - ((_e2228 * _e2276) * _e2276)), (_e2228 * ((_e2274 * _e2274) - (_e2271 * _e2271))));
                                                                            } else {
                                                                                let _e2250 = (_e2220 - _e2247);
                                                                                let _e2252 = f32((32u - _e2247));
                                                                                let _e2253 = (f32(_e2250) / _e2252);
                                                                                let _e2256 = (f32((_e2250 + 1u)) / _e2252);
                                                                                let _e2258 = ((_e2253 + _e2256) * 0.5f);
                                                                                phi_2820_ = type_56((_e2227 + ((_e2229 * _e2258) * _e2258)), (_e2229 * ((_e2256 * _e2256) - (_e2253 * _e2253))));
                                                                            }
                                                                            let _e2286 = phi_2820_;
                                                                            phi_2821_ = _e2286;
                                                                        }
                                                                        let _e2304 = phi_2821_;
                                                                        phi_2822_ = _e2304;
                                                                    }
                                                                    let _e2321 = phi_2822_;
                                                                    let _e2327 = (_e334.member.x + (_e334.member_1.x * _e2321.member));
                                                                    let _e2328 = (_e334.member.y + (_e334.member_1.y * _e2321.member));
                                                                    let _e2329 = (_e334.member.z + (_e334.member_1.z * _e2321.member));
                                                                    let _e2342 = ((((_e2327 * _e2327) + (_e2328 * _e2328)) + (_e2329 * _e2329)) - (2f * (((_e2327 * _e1453) + (_e2328 * _e1456)) + (_e2329 * _e1459))));
                                                                    let _e2343 = (_e2342 + _e1468);
                                                                    let _e2344 = (_e2327 - _e1453);
                                                                    let _e2345 = (_e2328 - _e1456);
                                                                    let _e2346 = (_e2329 - _e1459);
                                                                    let _e2352 = sqrt((((_e2344 * _e2344) + (_e2345 * _e2345)) + (_e2346 * _e2346)));
                                                                    let _e2354 = (_e2343 / (_e2352 + _e1462));
                                                                    let _e2358 = (1f - (abs((_e2354 - 25000f)) * 0.00006666667f));
                                                                    if (_e2358 != _e2358) {
                                                                        phi_23188_ = true;
                                                                    } else {
                                                                        phi_23188_ = (0f >= _e2358);
                                                                    }
                                                                    let _e2362 = phi_23188_;
                                                                    let _e2364 = -(_e2354);
                                                                    let _e2367 = (exp((_e2364 / _e1483)) * _e2321.member_1);
                                                                    let _e2370 = (exp((_e2364 / _e1486)) * _e2321.member_1);
                                                                    let _e2372 = (_e2367 * 0.5f);
                                                                    let _e2373 = (_e2370 * 0.5f);
                                                                    let _e2374 = ((select(_e2358, 0f, _e2362) * _e2321.member_1) * 0.5f);
                                                                    let _e2376 = (_e2212.x + _e2372);
                                                                    let _e2378 = (_e2212.y + _e2373);
                                                                    let _e2380 = (_e2212.z + _e2374);
                                                                    let _e2385 = (_e1502 * 1.1f);
                                                                    let _e2386 = (_e2385 * _e2378);
                                                                    let _e2405 = (_e1502 * _e2370);
                                                                    if (_e1518 == 4294967295u) {
                                                                        phi_3150_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2413 = (1f / _e2352);
                                                                        let _e2421 = ((((_e2344 * _e2413) * _e1474) + ((_e2345 * _e2413) * _e1477)) + ((_e2346 * _e2413) * _e1480));
                                                                        if (_e2354 != _e2354) {
                                                                            phi_23315_ = true;
                                                                        } else {
                                                                            phi_23315_ = (0f >= _e2354);
                                                                        }
                                                                        let _e2425 = phi_23315_;
                                                                        let _e2429 = sqrt((select(_e2354, 0f, _e2425) / (_e1465 - _e1462)));
                                                                        if (_e2429 != _e2429) {
                                                                            phi_23330_ = true;
                                                                        } else {
                                                                            phi_23330_ = (1f <= _e2429);
                                                                        }
                                                                        let _e2433 = phi_23330_;
                                                                        let _e2435 = (select(_e2429, 1f, _e2433) * 31f);
                                                                        let _e2437 = select(_e2421, -1f, (_e2421 < -1f));
                                                                        let _e2441 = ((select(_e2437, 1f, (_e2437 > 1f)) + 1f) * 15.5f);
                                                                        let _e2446 = select(select(u32(_e2435), 0u, (_e2435 < 0f)), 4294967295u, (_e2435 > 4294967000f));
                                                                        let _e2448 = select(_e2446, 30u, (30u < _e2446));
                                                                        let _e2453 = select(select(u32(_e2441), 0u, (_e2441 < 0f)), 4294967295u, (_e2441 > 4294967000f));
                                                                        let _e2455 = select(_e2453, 30u, (30u < _e2453));
                                                                        let _e2457 = (_e2435 - f32(_e2448));
                                                                        let _e2459 = (_e2441 - f32(_e2455));
                                                                        let _e2463 = (_e1518 + (3u * ((_e2448 * 32u) + _e2455)));
                                                                        let _e2468 = (_e1518 + (3u * (((_e2448 + 1u) * 32u) + _e2455)));
                                                                        let _e2470 = global_6.member[_e2463];
                                                                        let _e2473 = global_6.member[(_e2463 + 1u)];
                                                                        let _e2476 = global_6.member[(_e2463 + 2u)];
                                                                        let _e2477 = (1f - _e2459);
                                                                        let _e2483 = global_6.member[(_e2463 + 3u)];
                                                                        let _e2486 = global_6.member[(_e2463 + 4u)];
                                                                        let _e2489 = global_6.member[(_e2463 + 5u)];
                                                                        let _e2496 = (1f - _e2457);
                                                                        let _e2501 = global_6.member[_e2468];
                                                                        let _e2504 = global_6.member[(_e2468 + 1u)];
                                                                        let _e2507 = global_6.member[(_e2468 + 2u)];
                                                                        let _e2513 = global_6.member[(_e2468 + 3u)];
                                                                        let _e2516 = global_6.member[(_e2468 + 4u)];
                                                                        let _e2519 = global_6.member[(_e2468 + 5u)];
                                                                        phi_3150_ = vec3<f32>(((((_e2470 * _e2477) + (_e2483 * _e2459)) * _e2496) + (((_e2501 * _e2477) + (_e2513 * _e2459)) * _e2457)), ((((_e2473 * _e2477) + (_e2486 * _e2459)) * _e2496) + (((_e2504 * _e2477) + (_e2516 * _e2459)) * _e2457)), ((((_e2476 * _e2477) + (_e2489 * _e2459)) * _e2496) + (((_e2507 * _e2477) + (_e2519 * _e2459)) * _e2457)));
                                                                    }
                                                                    let _e2534 = phi_3150_;
                                                                    let _e2552 = (((_e2344 * _e1474) + (_e2345 * _e1477)) + (_e2346 * _e1480));
                                                                    let _e2553 = (_e2552 * _e2552);
                                                                    let _e2554 = (_e2553 - _e2343);
                                                                    if (_e2554 < 0f) {
                                                                        phi_23480_ = type_57(0u, type_57().member_1);
                                                                        phi_23481_ = type_57();
                                                                        phi_23482_ = true;
                                                                    } else {
                                                                        if (_e2552 < 0f) {
                                                                            phi_23452_ = -(sqrt(_e2554));
                                                                        } else {
                                                                            phi_23452_ = sqrt(_e2554);
                                                                        }
                                                                        let _e2561 = phi_23452_;
                                                                        let _e2563 = -((_e2552 + _e2561));
                                                                        let _e2564 = (_e2563 == 0f);
                                                                        if _e2564 {
                                                                            phi_23475_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23476_ = type_57();
                                                                        } else {
                                                                            let _e2565 = (_e2343 / _e2563);
                                                                            if (_e2563 < _e2565) {
                                                                                phi_23464_ = type_56(_e2563, _e2565);
                                                                            } else {
                                                                                phi_23464_ = type_56(_e2565, _e2563);
                                                                            }
                                                                            let _e2570 = phi_23464_;
                                                                            phi_23475_ = type_57();
                                                                            phi_23476_ = type_57(1u, _e2570);
                                                                        }
                                                                        let _e2573 = phi_23475_;
                                                                        let _e2575 = phi_23476_;
                                                                        phi_23480_ = _e2573;
                                                                        phi_23481_ = _e2575;
                                                                        phi_23482_ = _e2564;
                                                                    }
                                                                    let _e2580 = phi_23480_;
                                                                    let _e2582 = phi_23481_;
                                                                    let _e2584 = phi_23482_;
                                                                    if _e2584 {
                                                                        phi_23486_ = _e2580;
                                                                    } else {
                                                                        phi_23486_ = _e2582;
                                                                    }
                                                                    let _e2586 = phi_23486_;
                                                                    if (bitcast<i32>(_e2586.member) != 0i) {
                                                                        phi_23420_ = select(_e2586.member_1.member_1, _e2586.member_1.member, (_e2586.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23420_ = -1f;
                                                                    }
                                                                    let _e2597 = phi_23420_;
                                                                    if (_e2597 >= 0f) {
                                                                        phi_23403_ = type_58(0u, type_58().member_1);
                                                                    } else {
                                                                        let _e2599 = (_e2342 + _e1471);
                                                                        let _e2600 = (_e2553 - _e2599);
                                                                        if (_e2600 < 0f) {
                                                                            phi_23590_ = type_57(0u, type_57().member_1);
                                                                            phi_23591_ = type_57();
                                                                            phi_23592_ = true;
                                                                        } else {
                                                                            if (_e2552 < 0f) {
                                                                                phi_23562_ = -(sqrt(_e2600));
                                                                            } else {
                                                                                phi_23562_ = sqrt(_e2600);
                                                                            }
                                                                            let _e2607 = phi_23562_;
                                                                            let _e2609 = -((_e2552 + _e2607));
                                                                            let _e2610 = (_e2609 == 0f);
                                                                            if _e2610 {
                                                                                phi_23585_ = type_57(1u, type_56(0f, 0f));
                                                                                phi_23586_ = type_57();
                                                                            } else {
                                                                                let _e2611 = (_e2599 / _e2609);
                                                                                if (_e2609 < _e2611) {
                                                                                    phi_23574_ = type_56(_e2609, _e2611);
                                                                                } else {
                                                                                    phi_23574_ = type_56(_e2611, _e2609);
                                                                                }
                                                                                let _e2616 = phi_23574_;
                                                                                phi_23585_ = type_57();
                                                                                phi_23586_ = type_57(1u, _e2616);
                                                                            }
                                                                            let _e2619 = phi_23585_;
                                                                            let _e2621 = phi_23586_;
                                                                            phi_23590_ = _e2619;
                                                                            phi_23591_ = _e2621;
                                                                            phi_23592_ = _e2610;
                                                                        }
                                                                        let _e2626 = phi_23590_;
                                                                        let _e2628 = phi_23591_;
                                                                        let _e2630 = phi_23592_;
                                                                        if _e2630 {
                                                                            phi_23596_ = _e2626;
                                                                        } else {
                                                                            phi_23596_ = _e2628;
                                                                        }
                                                                        let _e2632 = phi_23596_;
                                                                        if (bitcast<i32>(_e2632.member) != 0i) {
                                                                            phi_23530_ = select(_e2632.member_1.member_1, _e2632.member_1.member, (_e2632.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_23530_ = -1f;
                                                                        }
                                                                        let _e2643 = phi_23530_;
                                                                        if (_e2643 < 0f) {
                                                                            phi_23399_ = type_58(0u, type_58().member_1);
                                                                        } else {
                                                                            phi_23357_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_23358_ = 0u;
                                                                            loop {
                                                                                let _e2646 = phi_23357_;
                                                                                let _e2648 = phi_23358_;
                                                                                local_11 = _e2646;
                                                                                let _e2649 = (_e2648 < 16u);
                                                                                if _e2649 {
                                                                                    let _e2651 = (f32(_e2648) * 0.0625f);
                                                                                    let _e2652 = (_e2648 + 1u);
                                                                                    let _e2654 = (f32(_e2652) * 0.0625f);
                                                                                    let _e2656 = ((_e2651 + _e2654) * 0.5f);
                                                                                    let _e2658 = ((_e2643 * _e2656) * _e2656);
                                                                                    let _e2662 = (_e2643 * ((_e2654 * _e2654) - (_e2651 * _e2651)));
                                                                                    let _e2667 = (vec3<f32>(_e2327, _e2328, _e2329) + vec3<f32>((_e1474 * _e2658), (_e1477 * _e2658), (_e1480 * _e2658)));
                                                                                    let _e2684 = (_e2667.x - _e1453);
                                                                                    let _e2685 = (_e2667.y - _e1456);
                                                                                    let _e2686 = (_e2667.z - _e1459);
                                                                                    let _e2694 = ((((((_e2667.x * _e2667.x) + (_e2667.y * _e2667.y)) + (_e2667.z * _e2667.z)) - (2f * (((_e2667.x * _e1453) + (_e2667.y * _e1456)) + (_e2667.z * _e1459)))) + _e1468) / (sqrt((((_e2684 * _e2684) + (_e2685 * _e2685)) + (_e2686 * _e2686))) + _e1462));
                                                                                    let _e2698 = (1f - (abs((_e2694 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2698 != _e2698) {
                                                                                        phi_23728_ = true;
                                                                                    } else {
                                                                                        phi_23728_ = (0f >= _e2698);
                                                                                    }
                                                                                    let _e2702 = phi_23728_;
                                                                                    let _e2704 = -(_e2694);
                                                                                    phi_23391_ = (_e2646 + vec3<f32>((exp((_e2704 / _e1483)) * _e2662), (exp((_e2704 / _e1486)) * _e2662), (select(_e2698, 0f, _e2702) * _e2662)));
                                                                                    phi_23392_ = _e2652;
                                                                                } else {
                                                                                    phi_23391_ = vec3<f32>();
                                                                                    phi_23392_ = u32();
                                                                                }
                                                                                let _e2715 = phi_23391_;
                                                                                let _e2717 = phi_23392_;
                                                                                phi_23357_ = _e2715;
                                                                                phi_23358_ = _e2717;
                                                                                if !(_e2649) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2720 = local_11;
                                                                            phi_23399_ = type_58(1u, _e2720);
                                                                        }
                                                                        let _e2726 = phi_23399_;
                                                                        phi_23403_ = _e2726;
                                                                    }
                                                                    let _e2731 = phi_23403_;
                                                                    if (bitcast<i32>(_e2731.member) != 0i) {
                                                                        let _e2736 = (vec3<f32>(_e2376, _e2378, _e2380) + _e2731.member_1);
                                                                        let _e2742 = (_e2385 * _e2736.y);
                                                                        let _e2754 = exp(-((((_e1492 * _e2736.x) + _e2742) + (_e1508 * _e2736.z))));
                                                                        let _e2756 = exp(-((((_e1495 * _e2736.x) + _e2742) + (_e1511 * _e2736.z))));
                                                                        let _e2758 = exp(-((((_e1498 * _e2736.x) + _e2742) + (_e1514 * _e2736.z))));
                                                                        phi_3209_ = vec3<f32>((_e2216.x + (_e2754 * _e2370)), (_e2216.y + (_e2756 * _e2370)), (_e2216.z + (_e2758 * _e2370)));
                                                                        phi_3210_ = vec3<f32>((_e2218.x + (_e2754 * _e2367)), (_e2218.y + (_e2756 * _e2367)), (_e2218.z + (_e2758 * _e2367)));
                                                                    } else {
                                                                        phi_3209_ = _e2216;
                                                                        phi_3210_ = _e2218;
                                                                    }
                                                                    let _e2780 = phi_3209_;
                                                                    let _e2782 = phi_3210_;
                                                                    phi_2713_ = vec3<f32>((_e2376 + _e2372), (_e2378 + _e2373), (_e2380 + _e2374));
                                                                    phi_2716_ = vec3<f32>((_e2214.x + ((exp(-((((_e1492 * _e2376) + _e2386) + (_e1508 * _e2380)))) * ((_e1492 * _e2367) + _e2405)) * _e2534.x)), (_e2214.y + ((exp(-((((_e1495 * _e2376) + _e2386) + (_e1511 * _e2380)))) * ((_e1495 * _e2367) + _e2405)) * _e2534.y)), (_e2214.z + ((exp(-((((_e1498 * _e2376) + _e2386) + (_e1514 * _e2380)))) * ((_e1498 * _e2367) + _e2405)) * _e2534.z)));
                                                                    phi_2718_ = _e2780;
                                                                    phi_2720_ = _e2782;
                                                                    phi_2722_ = (_e2220 + 1u);
                                                                } else {
                                                                    phi_2713_ = vec3<f32>();
                                                                    phi_2716_ = vec3<f32>();
                                                                    phi_2718_ = vec3<f32>();
                                                                    phi_2720_ = vec3<f32>();
                                                                    phi_2722_ = u32();
                                                                }
                                                                let _e2789 = phi_2713_;
                                                                let _e2791 = phi_2716_;
                                                                let _e2793 = phi_2718_;
                                                                let _e2795 = phi_2720_;
                                                                let _e2797 = phi_2722_;
                                                                phi_2712_ = _e2789;
                                                                phi_2715_ = _e2791;
                                                                phi_2717_ = _e2793;
                                                                phi_2719_ = _e2795;
                                                                phi_2721_ = _e2797;
                                                                phi_33443_ = _e1443;
                                                                if !(_e2221) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2800 = phi_33443_;
                                                            phi_33533_ = _e2800;
                                                            if _e2800 {
                                                                break;
                                                            }
                                                            let _e2802 = local_12;
                                                            let _e2805 = local_13;
                                                            let _e2808 = local_14;
                                                            phi_33537_ = _e2800;
                                                            phi_3230_ = type_52((_e2802 * vec3<f32>(_e1492, _e1495, _e1498)), (_e2805 * _e1502), _e2808);
                                                        } else {
                                                            phi_33537_ = _e1443;
                                                            phi_3230_ = type_52(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2811 = phi_33537_;
                                                        let _e2813 = phi_3230_;
                                                        phi_33536_ = _e2811;
                                                        phi_3231_ = _e2813;
                                                    } else {
                                                        phi_33536_ = _e1443;
                                                        phi_3231_ = _e2046;
                                                    }
                                                    let _e2815 = phi_33536_;
                                                    let _e2817 = phi_3231_;
                                                    let _e2825 = (((_e334.member_1.x * _e1474) + (_e334.member_1.y * _e1477)) + (_e334.member_1.z * _e1480));
                                                    let _e2827 = (1f + (_e2825 * _e2825));
                                                    let _e2828 = (0.059683103f * _e2827);
                                                    let _e2835 = (_e1505 * _e1505);
                                                    let _e2846 = ((0.119366206f * ((1f - _e2835) * _e2827)) / ((2f + _e2835) * pow(((1f + _e2835) - ((2f * _e1505) * _e2825)), 1.5f)));
                                                    phi_33535_ = _e2815;
                                                    phi_3254_ = vec3<f32>((_e1445.x + (((((_e2817.member.x * _e2828) + (_e2817.member_1.x * _e2846)) + _e2817.member_2.x) * _e1489) * 0.38317f)), (_e1445.y + (((((_e2817.member.y * _e2828) + (_e2817.member_1.y * _e2846)) + _e2817.member_2.y) * _e1489) * 0.38317f)), (_e1445.z + (((((_e2817.member.z * _e2828) + (_e2817.member_1.z * _e2846)) + _e2817.member_2.z) * _e1489) * 0.38317f)));
                                                    phi_3255_ = (_e1447 + 1i);
                                                    phi_3256_ = bitcast<u32>(_e1525);
                                                } else {
                                                    phi_33535_ = _e1443;
                                                    phi_3254_ = vec3<f32>();
                                                    phi_3255_ = i32();
                                                    phi_3256_ = u32();
                                                }
                                                let _e2877 = phi_33535_;
                                                let _e2879 = phi_3254_;
                                                let _e2881 = phi_3255_;
                                                let _e2883 = phi_3256_;
                                                phi_33534_ = _e2877;
                                                phi_900_ = _e2879;
                                                phi_903_ = _e2881;
                                                phi_905_ = _e2883;
                                                phi_3257_ = _e1451;
                                            } else {
                                                phi_33534_ = _e1443;
                                                phi_900_ = vec3<f32>();
                                                phi_903_ = i32();
                                                phi_905_ = u32();
                                                phi_3257_ = false;
                                            }
                                            let _e2885 = phi_33534_;
                                            let _e2887 = phi_900_;
                                            let _e2889 = phi_903_;
                                            let _e2891 = phi_905_;
                                            let _e2893 = phi_3257_;
                                            phi_33475_ = _e2885;
                                            phi_899_ = _e2887;
                                            phi_902_ = _e2889;
                                            phi_904_ = _e2891;
                                            phi_33533_ = _e2885;
                                            if !(_e2893) {
                                                break;
                                            }
                                        }
                                        let _e2896 = phi_33533_;
                                        phi_33861_ = _e2896;
                                        if _e2896 {
                                            break;
                                        }
                                        if _e1440 {
                                            let _e2899 = global_6.member[(_e1292 + 10u)];
                                            let _e2902 = global_6.member[(_e1292 + 21u)];
                                            let _e2903 = bitcast<u32>(_e2902);
                                            let _e2906 = global_6.member[(_e1292 + 22u)];
                                            let _e2907 = bitcast<u32>(_e2906);
                                            let _e2910 = global_6.member[(_e1292 + 23u)];
                                            let _e2914 = global_6.member[(_e1292 + 24u)];
                                            let _e2918 = global_6.member[(_e1292 + 25u)];
                                            let _e2922 = global_6.member[(_e1292 + 26u)];
                                            let _e2925 = global_6.member[_e1292];
                                            let _e2928 = global_6.member[(_e1292 + 1u)];
                                            let _e2931 = global_6.member[(_e1292 + 2u)];
                                            let _e2934 = global_6.member[(_e1292 + 3u)];
                                            let _e2937 = global_6.member[(_e1292 + 4u)];
                                            let _e2940 = global_6.member[(_e1292 + 5u)];
                                            let _e2943 = global_6.member[(_e1292 + 6u)];
                                            let _e2946 = global_6.member[(_e1292 + 7u)];
                                            let _e2949 = global_6.member[(_e1292 + 8u)];
                                            let _e2954 = (((_e334.member_1.x * _e2934) + (_e334.member_1.y * _e2937)) + (_e334.member_1.z * _e2940));
                                            let _e2956 = select(_e2954, -1f, (_e2954 < -1f));
                                            let _e2959 = asin(select(_e2956, 1f, (_e2956 > 1f)));
                                            let _e2970 = atan2((((_e334.member_1.x * _e2943) + (_e334.member_1.y * _e2946)) + (_e334.member_1.z * _e2949)), (((_e334.member_1.x * _e2925) + (_e334.member_1.y * _e2928)) + (_e334.member_1.z * _e2931)));
                                            if (_e2903 != 4294967295u) {
                                                let _e2973 = global_6.member[_e2903];
                                                let _e2976 = global_6.member[(_e2903 + 1u)];
                                                let _e2977 = bitcast<u32>(_e2976);
                                                let _e2980 = global_6.member[(_e2903 + 4u)];
                                                phi_3591_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3594_ = 0u;
                                                loop {
                                                    let _e2986 = phi_3591_;
                                                    let _e2988 = phi_3594_;
                                                    local_15 = _e2986;
                                                    let _e2989 = (_e2988 < 6u);
                                                    if _e2989 {
                                                        let _e2990 = (_e2988 / 2u);
                                                        if (_e2990 == 0u) {
                                                            phi_23856_ = _e334.member_1.x;
                                                        } else {
                                                            phi_23856_ = select(_e334.member_1.z, _e334.member_1.y, (_e2990 == 1u));
                                                        }
                                                        let _e2998 = phi_23856_;
                                                        let _e2999 = (_e2998 * select(-1f, 1f, ((_e2988 % 2u) == 0u)));
                                                        if (_e2999 > 0f) {
                                                            let _e3003 = ((_e2990 + 1u) % 3u);
                                                            if (_e3003 == 0u) {
                                                                phi_23871_ = _e334.member_1.x;
                                                            } else {
                                                                phi_23871_ = select(_e334.member_1.z, _e334.member_1.y, (_e3003 == 1u));
                                                            }
                                                            let _e3008 = phi_23871_;
                                                            let _e3009 = (_e3008 / _e2999);
                                                            let _e3011 = ((_e2990 + 2u) % 3u);
                                                            if (_e3011 == 0u) {
                                                                phi_23886_ = _e334.member_1.x;
                                                            } else {
                                                                phi_23886_ = select(_e334.member_1.z, _e334.member_1.y, (_e3011 == 1u));
                                                            }
                                                            let _e3016 = phi_23886_;
                                                            let _e3017 = (_e3016 / _e2999);
                                                            let _e3019 = (1f + (_e2973 * 12f));
                                                            if (abs(_e3009) <= _e3019) {
                                                                if (abs(_e3017) <= _e3019) {
                                                                    let _e3025 = f32(_e2977);
                                                                    let _e3027 = floor((((_e3009 + 1f) * 0.5f) * _e3025));
                                                                    let _e3034 = select(0i, select(select(i32(_e3027), i32(-2147483648), (_e3027 < -2147483600f)), 2147483647i, (_e3027 > 2147483500f)), (_e3027 == _e3027));
                                                                    let _e3036 = (bitcast<i32>(_e2977) - 1i);
                                                                    let _e3037 = (0i <= _e3036);
                                                                    if _e3037 {
                                                                    } else {
                                                                        phi_33538_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3034 < 0i) {
                                                                        phi_3665_ = 0i;
                                                                    } else {
                                                                        phi_3665_ = select(_e3034, _e3036, (_e3034 > _e3036));
                                                                    }
                                                                    let _e3042 = phi_3665_;
                                                                    let _e3046 = floor((((_e3017 + 1f) * 0.5f) * _e3025));
                                                                    let _e3053 = select(0i, select(select(i32(_e3046), i32(-2147483648), (_e3046 < -2147483600f)), 2147483647i, (_e3046 > 2147483500f)), (_e3046 == _e3046));
                                                                    if _e3037 {
                                                                    } else {
                                                                        phi_33538_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3053 < 0i) {
                                                                        phi_3695_ = 0i;
                                                                    } else {
                                                                        phi_3695_ = select(_e3053, _e3036, (_e3053 > _e3036));
                                                                    }
                                                                    let _e3058 = phi_3695_;
                                                                    let _e3059 = (_e3042 - 1i);
                                                                    phi_3703_ = _e2986;
                                                                    phi_3706_ = select(0i, _e3059, (0i < _e3059));
                                                                    loop {
                                                                        let _e3063 = phi_3703_;
                                                                        let _e3065 = phi_3706_;
                                                                        let _e3066 = (_e3042 + 1i);
                                                                        local_59 = _e3063;
                                                                        let _e3069 = (_e3065 <= select(_e3066, _e3036, (_e3036 < _e3066)));
                                                                        if _e3069 {
                                                                            let _e3070 = (_e3058 - 1i);
                                                                            phi_3729_ = _e3063;
                                                                            phi_3732_ = select(0i, _e3070, (0i < _e3070));
                                                                            loop {
                                                                                let _e3074 = phi_3729_;
                                                                                let _e3076 = phi_3732_;
                                                                                let _e3077 = (_e3058 + 1i);
                                                                                local_58 = _e3074;
                                                                                let _e3080 = (_e3076 <= select(_e3077, _e3036, (_e3036 < _e3077)));
                                                                                if _e3080 {
                                                                                    let _e3089 = global_6.member[(bitcast<u32>(_e2980) + ((((_e2988 * _e2977) + bitcast<u32>(_e3065)) * _e2977) + bitcast<u32>(_e3076)))];
                                                                                    phi_3730_ = vec3<f32>((_e3074.x + _e3089), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3733_ = (_e3076 + 1i);
                                                                                } else {
                                                                                    phi_3730_ = vec3<f32>();
                                                                                    phi_3733_ = i32();
                                                                                }
                                                                                let _e3098 = phi_3730_;
                                                                                let _e3100 = phi_3733_;
                                                                                phi_3729_ = _e3098;
                                                                                phi_3732_ = _e3100;
                                                                                if !(_e3080) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6935 = local_58;
                                                                            phi_3704_ = _e6935;
                                                                            phi_3707_ = (_e3065 + 1i);
                                                                        } else {
                                                                            phi_3704_ = vec3<f32>();
                                                                            phi_3707_ = i32();
                                                                        }
                                                                        let _e3104 = phi_3704_;
                                                                        let _e3106 = phi_3707_;
                                                                        phi_3703_ = _e3104;
                                                                        phi_3706_ = _e3106;
                                                                        if !(_e3069) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6939 = local_59;
                                                                    phi_3931_ = _e6939;
                                                                } else {
                                                                    phi_3931_ = _e2986;
                                                                }
                                                                let _e3109 = phi_3931_;
                                                                phi_3932_ = _e3109;
                                                            } else {
                                                                phi_3932_ = _e2986;
                                                            }
                                                            let _e3111 = phi_3932_;
                                                            phi_3933_ = _e3111;
                                                        } else {
                                                            phi_3933_ = _e2986;
                                                        }
                                                        let _e3113 = phi_3933_;
                                                        phi_3592_ = _e3113;
                                                        phi_3595_ = (_e2988 + 1u);
                                                    } else {
                                                        phi_3592_ = vec3<f32>();
                                                        phi_3595_ = u32();
                                                    }
                                                    let _e3116 = phi_3592_;
                                                    let _e3118 = phi_3595_;
                                                    phi_3591_ = _e3116;
                                                    phi_3594_ = _e3118;
                                                    phi_33538_ = _e2896;
                                                    if !(_e2989) {
                                                        break;
                                                    }
                                                }
                                                let _e3121 = phi_33538_;
                                                phi_33861_ = _e3121;
                                                if _e3121 {
                                                    break;
                                                }
                                                let _e3123 = local_15;
                                                phi_33586_ = _e3121;
                                                phi_3944_ = (_e3123.x * (1f / ((6.2831855f * _e2973) * _e2973)));
                                            } else {
                                                phi_33586_ = _e2896;
                                                phi_3944_ = 0f;
                                            }
                                            let _e3127 = phi_33586_;
                                            let _e3129 = phi_3944_;
                                            if (_e2907 != 4294967295u) {
                                                let _e3146 = (_e2959 * 1.3333334f);
                                                let _e3153 = global_6.member[_e2907];
                                                phi_3968_ = (_e3129 + ((_e3153 * ((exp((-(abs(_e2959)) / (0.06f + (abs(_e2970) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2970 * -0.71428573f) * (_e2970 * 0.71428573f)))))) + (1.3f * exp((((_e2970 * _e2970) + (_e3146 * _e3146)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3968_ = _e3129;
                                            }
                                            let _e3159 = phi_3968_;
                                            phi_3970_ = 0u;
                                            phi_3973_ = _e3159;
                                            loop {
                                                let _e3161 = phi_3970_;
                                                let _e3163 = phi_3973_;
                                                local_60 = _e3163;
                                                let _e3164 = (_e3161 < bitcast<u32>(_e2914));
                                                if _e3164 {
                                                    let _e3166 = (bitcast<u32>(_e2910) + (_e3161 * 18u));
                                                    let _e3169 = global_6.member[(_e3166 + 1u)];
                                                    let _e3172 = global_6.member[(_e3166 + 2u)];
                                                    let _e3175 = global_6.member[(_e3166 + 3u)];
                                                    let _e3183 = global_6.member[(_e3166 + 5u)];
                                                    if ((((_e334.member_1.x * _e3169) + (_e334.member_1.y * _e3172)) + (_e334.member_1.z * _e3175)) >= _e3183) {
                                                        let _e3186 = global_6.member[_e3166];
                                                        let _e3191 = select(select(u32(_e3186), 0u, (_e3186 < 0f)), 4294967295u, (_e3186 > 4294967000f));
                                                        if (_e3191 == 0u) {
                                                            phi_4077_ = 1.8f;
                                                        } else {
                                                            if (_e3191 == 1u) {
                                                                phi_4076_ = 1f;
                                                            } else {
                                                                phi_4076_ = select(0f, 4.7f, (_e3191 == 2u));
                                                            }
                                                            let _e3197 = phi_4076_;
                                                            phi_4077_ = _e3197;
                                                        }
                                                        let _e3199 = phi_4077_;
                                                        let _e3202 = global_6.member[(_e3166 + 6u)];
                                                        phi_4093_ = (_e3163 + (_e3202 * _e3199));
                                                    } else {
                                                        phi_4093_ = _e3163;
                                                    }
                                                    let _e3206 = phi_4093_;
                                                    phi_3971_ = (_e3161 + 1u);
                                                    phi_3974_ = _e3206;
                                                } else {
                                                    phi_3971_ = u32();
                                                    phi_3974_ = f32();
                                                }
                                                let _e3209 = phi_3971_;
                                                let _e3211 = phi_3974_;
                                                phi_3970_ = _e3209;
                                                phi_3973_ = _e3211;
                                                if !(_e3164) {
                                                    break;
                                                }
                                            }
                                            phi_4096_ = 0u;
                                            let _e6959 = local_60;
                                            phi_4099_ = _e6959;
                                            loop {
                                                let _e3214 = phi_4096_;
                                                let _e3216 = phi_4099_;
                                                local_16 = _e3216;
                                                let _e3217 = (_e3214 < bitcast<u32>(_e2922));
                                                if _e3217 {
                                                    let _e3219 = (bitcast<u32>(_e2918) + (_e3214 * 29u));
                                                    let _e3222 = global_6.member[(_e3219 + 1u)];
                                                    let _e3225 = global_6.member[(_e3219 + 2u)];
                                                    let _e3228 = global_6.member[(_e3219 + 3u)];
                                                    let _e3236 = global_6.member[(_e3219 + 5u)];
                                                    if ((((_e334.member_1.x * _e3222) + (_e334.member_1.y * _e3225)) + (_e334.member_1.z * _e3228)) >= _e3236) {
                                                        let _e3239 = global_6.member[_e3219];
                                                        let _e3243 = global_6.member[(_e3219 + 6u)];
                                                        phi_4207_ = (_e3216 + (_e3243 * select(1f, 3.3808f, (_e3239 < 0.5f))));
                                                    } else {
                                                        phi_4207_ = _e3216;
                                                    }
                                                    let _e3248 = phi_4207_;
                                                    phi_4097_ = (_e3214 + 1u);
                                                    phi_4100_ = _e3248;
                                                } else {
                                                    phi_4097_ = u32();
                                                    phi_4100_ = f32();
                                                }
                                                let _e3251 = phi_4097_;
                                                let _e3253 = phi_4100_;
                                                phi_4096_ = _e3251;
                                                phi_4099_ = _e3253;
                                                if !(_e3217) {
                                                    break;
                                                }
                                            }
                                            let _e3257 = global_6.member[(_e1292 + 9u)];
                                            let _e3259 = local_16;
                                            let _e3262 = local_17;
                                            let _e3266 = local_18;
                                            let _e3271 = local_19;
                                            let _e3276 = ((_e3259 * _e3257) <= (_e2899 * (((0.2126f * _e3262.x) + (0.7152f * _e3266.y)) + (0.0722f * _e3271.z))));
                                            if _e3276 {
                                                phi_33627_ = _e3127;
                                                phi_4766_ = vec3<f32>();
                                                phi_4767_ = bool();
                                            } else {
                                                let _e3278 = global_6.member[_e1289];
                                                let _e3281 = global_6.member[(_e1289 + 1u)];
                                                let _e3284 = global_6.member[(_e1289 + 2u)];
                                                let _e3287 = global_6.member[(_e1289 + 3u)];
                                                let _e3290 = global_6.member[(_e1289 + 5u)];
                                                let _e3293 = global_6.member[(_e1289 + 6u)];
                                                let _e3296 = global_6.member[(_e1289 + 10u)];
                                                let _e3299 = global_6.member[(_e1289 + 11u)];
                                                let _e3302 = global_6.member[(_e1289 + 13u)];
                                                let _e3305 = global_6.member[(_e1289 + 14u)];
                                                let _e3308 = global_6.member[(_e1289 + 15u)];
                                                let _e3311 = global_6.member[(_e1289 + 16u)];
                                                let _e3314 = global_6.member[(_e1289 + 18u)];
                                                let _e3317 = global_6.member[(_e1289 + 19u)];
                                                let _e3320 = global_6.member[(_e1289 + 20u)];
                                                let _e3332 = ((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e3278) + (_e334.member.y * _e3281)) + (_e334.member.z * _e3284))));
                                                let _e3333 = (_e3332 + _e3290);
                                                let _e3334 = (_e3333 > 0f);
                                                if _e3334 {
                                                    let _e3342 = ((((_e334.member.x - _e3278) * _e334.member_1.x) + ((_e334.member.y - _e3281) * _e334.member_1.y)) + ((_e334.member.z - _e3284) * _e334.member_1.z));
                                                    let _e3344 = ((_e3342 * _e3342) - _e3333);
                                                    if (_e3344 < 0f) {
                                                        phi_24030_ = type_57(0u, type_57().member_1);
                                                        phi_24031_ = type_57();
                                                        phi_24032_ = true;
                                                    } else {
                                                        if (_e3342 < 0f) {
                                                            phi_24002_ = -(sqrt(_e3344));
                                                        } else {
                                                            phi_24002_ = sqrt(_e3344);
                                                        }
                                                        let _e3351 = phi_24002_;
                                                        let _e3353 = -((_e3342 + _e3351));
                                                        let _e3354 = (_e3353 == 0f);
                                                        if _e3354 {
                                                            phi_24025_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24026_ = type_57();
                                                        } else {
                                                            let _e3355 = (_e3333 / _e3353);
                                                            if (_e3353 < _e3355) {
                                                                phi_24014_ = type_56(_e3353, _e3355);
                                                            } else {
                                                                phi_24014_ = type_56(_e3355, _e3353);
                                                            }
                                                            let _e3360 = phi_24014_;
                                                            phi_24025_ = type_57();
                                                            phi_24026_ = type_57(1u, _e3360);
                                                        }
                                                        let _e3363 = phi_24025_;
                                                        let _e3365 = phi_24026_;
                                                        phi_24030_ = _e3363;
                                                        phi_24031_ = _e3365;
                                                        phi_24032_ = _e3354;
                                                    }
                                                    let _e3370 = phi_24030_;
                                                    let _e3372 = phi_24031_;
                                                    let _e3374 = phi_24032_;
                                                    if _e3374 {
                                                        phi_24036_ = _e3370;
                                                    } else {
                                                        phi_24036_ = _e3372;
                                                    }
                                                    let _e3376 = phi_24036_;
                                                    if (bitcast<i32>(_e3376.member) != 0i) {
                                                        phi_23970_ = select(_e3376.member_1.member_1, _e3376.member_1.member, (_e3376.member_1.member >= 0f));
                                                    } else {
                                                        phi_23970_ = -1f;
                                                    }
                                                    let _e3387 = phi_23970_;
                                                    phi_4608_ = select(true, false, (_e3387 >= 0f));
                                                } else {
                                                    phi_4608_ = true;
                                                }
                                                let _e3391 = phi_4608_;
                                                if _e3391 {
                                                    let _e3399 = ((((_e334.member.x - _e3278) * _e334.member_1.x) + ((_e334.member.y - _e3281) * _e334.member_1.y)) + ((_e334.member.z - _e3284) * _e334.member_1.z));
                                                    let _e3400 = (_e3332 + _e3293);
                                                    let _e3401 = (_e3399 * _e3399);
                                                    let _e3402 = (_e3401 - _e3400);
                                                    if (_e3402 < 0f) {
                                                        phi_24190_ = type_57(0u, type_57().member_1);
                                                        phi_24191_ = type_57();
                                                        phi_24192_ = true;
                                                    } else {
                                                        if (_e3399 < 0f) {
                                                            phi_24162_ = -(sqrt(_e3402));
                                                        } else {
                                                            phi_24162_ = sqrt(_e3402);
                                                        }
                                                        let _e3409 = phi_24162_;
                                                        let _e3411 = -((_e3399 + _e3409));
                                                        let _e3412 = (_e3411 == 0f);
                                                        if _e3412 {
                                                            phi_24185_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24186_ = type_57();
                                                        } else {
                                                            let _e3413 = (_e3400 / _e3411);
                                                            if (_e3411 < _e3413) {
                                                                phi_24174_ = type_56(_e3411, _e3413);
                                                            } else {
                                                                phi_24174_ = type_56(_e3413, _e3411);
                                                            }
                                                            let _e3418 = phi_24174_;
                                                            phi_24185_ = type_57();
                                                            phi_24186_ = type_57(1u, _e3418);
                                                        }
                                                        let _e3421 = phi_24185_;
                                                        let _e3423 = phi_24186_;
                                                        phi_24190_ = _e3421;
                                                        phi_24191_ = _e3423;
                                                        phi_24192_ = _e3412;
                                                    }
                                                    let _e3428 = phi_24190_;
                                                    let _e3430 = phi_24191_;
                                                    let _e3432 = phi_24192_;
                                                    if _e3432 {
                                                        phi_24196_ = _e3428;
                                                    } else {
                                                        phi_24196_ = _e3430;
                                                    }
                                                    let _e3434 = phi_24196_;
                                                    if (bitcast<i32>(_e3434.member) != 0i) {
                                                        phi_24083_ = type_57(0u, _e3434.member_1);
                                                    } else {
                                                        phi_24083_ = type_57(1u, type_57().member_1);
                                                    }
                                                    let _e3444 = phi_24083_;
                                                    if (bitcast<i32>(_e3444.member) != 0i) {
                                                        phi_24130_ = type_57(0u, type_57().member_1);
                                                    } else {
                                                        if (_e3444.member_1.member_1 <= 0f) {
                                                            phi_24126_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e3444.member_1.member != _e3444.member_1.member) {
                                                                phi_24234_ = true;
                                                            } else {
                                                                phi_24234_ = (0f >= _e3444.member_1.member);
                                                            }
                                                            let _e3456 = phi_24234_;
                                                            let _e3457 = select(_e3444.member_1.member, 0f, _e3456);
                                                            if _e3334 {
                                                                let _e3458 = (_e3401 - _e3333);
                                                                if (_e3458 < 0f) {
                                                                    phi_24342_ = type_57(0u, type_57().member_1);
                                                                    phi_24343_ = type_57();
                                                                    phi_24344_ = true;
                                                                } else {
                                                                    if (_e3399 < 0f) {
                                                                        phi_24314_ = -(sqrt(_e3458));
                                                                    } else {
                                                                        phi_24314_ = sqrt(_e3458);
                                                                    }
                                                                    let _e3465 = phi_24314_;
                                                                    let _e3467 = -((_e3399 + _e3465));
                                                                    let _e3468 = (_e3467 == 0f);
                                                                    if _e3468 {
                                                                        phi_24337_ = type_57(1u, type_56(0f, 0f));
                                                                        phi_24338_ = type_57();
                                                                    } else {
                                                                        let _e3469 = (_e3333 / _e3467);
                                                                        if (_e3467 < _e3469) {
                                                                            phi_24326_ = type_56(_e3467, _e3469);
                                                                        } else {
                                                                            phi_24326_ = type_56(_e3469, _e3467);
                                                                        }
                                                                        let _e3474 = phi_24326_;
                                                                        phi_24337_ = type_57();
                                                                        phi_24338_ = type_57(1u, _e3474);
                                                                    }
                                                                    let _e3477 = phi_24337_;
                                                                    let _e3479 = phi_24338_;
                                                                    phi_24342_ = _e3477;
                                                                    phi_24343_ = _e3479;
                                                                    phi_24344_ = _e3468;
                                                                }
                                                                let _e3484 = phi_24342_;
                                                                let _e3486 = phi_24343_;
                                                                let _e3488 = phi_24344_;
                                                                if _e3488 {
                                                                    phi_24348_ = _e3484;
                                                                } else {
                                                                    phi_24348_ = _e3486;
                                                                }
                                                                let _e3490 = phi_24348_;
                                                                if (bitcast<i32>(_e3490.member) != 0i) {
                                                                    phi_24282_ = select(_e3490.member_1.member_1, _e3490.member_1.member, (_e3490.member_1.member >= 0f));
                                                                } else {
                                                                    phi_24282_ = -1f;
                                                                }
                                                                let _e3501 = phi_24282_;
                                                                if (_e3501 >= 0f) {
                                                                    if (_e3444.member_1.member_1 != _e3444.member_1.member_1) {
                                                                        phi_24386_ = true;
                                                                    } else {
                                                                        phi_24386_ = (_e3501 <= _e3444.member_1.member_1);
                                                                    }
                                                                    let _e3506 = phi_24386_;
                                                                    phi_24111_ = select(_e3444.member_1.member_1, _e3501, _e3506);
                                                                } else {
                                                                    phi_24111_ = _e3444.member_1.member_1;
                                                                }
                                                                let _e3509 = phi_24111_;
                                                                phi_24113_ = _e3509;
                                                            } else {
                                                                phi_24113_ = _e3444.member_1.member_1;
                                                            }
                                                            let _e3511 = phi_24113_;
                                                            if (_e3511 <= _e3457) {
                                                                phi_24122_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                phi_24122_ = type_57(1u, type_56(_e3457, _e3511));
                                                            }
                                                            let _e3519 = phi_24122_;
                                                            phi_24126_ = _e3519;
                                                        }
                                                        let _e3524 = phi_24126_;
                                                        phi_24130_ = _e3524;
                                                    }
                                                    let _e3529 = phi_24130_;
                                                    let _e3532 = (bitcast<i32>(_e3529.member) != 0i);
                                                    if _e3532 {
                                                        phi_4622_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4625_ = 0u;
                                                        loop {
                                                            let _e3538 = phi_4622_;
                                                            let _e3540 = phi_4625_;
                                                            local_20 = _e3538;
                                                            local_21 = _e3538;
                                                            local_22 = _e3538;
                                                            let _e3541 = (_e3540 < 32u);
                                                            if _e3541 {
                                                                let _e3542 = -(_e3399);
                                                                if (_e3529.member_1.member <= _e3529.member_1.member_1) {
                                                                } else {
                                                                    phi_33565_ = true;
                                                                    break;
                                                                }
                                                                let _e3545 = select(_e3542, _e3529.member_1.member, (_e3542 < _e3529.member_1.member));
                                                                let _e3547 = select(_e3545, _e3529.member_1.member_1, (_e3545 > _e3529.member_1.member_1));
                                                                let _e3548 = (_e3547 - _e3529.member_1.member);
                                                                let _e3549 = (_e3529.member_1.member_1 - _e3547);
                                                                let _e3550 = (_e3529.member_1.member_1 - _e3529.member_1.member);
                                                                let _e3551 = (0.000000001f * _e3550);
                                                                if (_e3548 <= _e3551) {
                                                                    let _e3626 = (f32(_e3540) * 0.03125f);
                                                                    let _e3629 = (f32((_e3540 + 1u)) * 0.03125f);
                                                                    let _e3631 = ((_e3626 + _e3629) * 0.5f);
                                                                    phi_4726_ = type_56((_e3529.member_1.member + ((_e3550 * _e3631) * _e3631)), (_e3550 * ((_e3629 * _e3629) - (_e3626 * _e3626))));
                                                                } else {
                                                                    if (_e3549 <= _e3551) {
                                                                        let _e3609 = (f32((31u - _e3540)) * 0.03125f);
                                                                        let _e3612 = (f32((32u - _e3540)) * 0.03125f);
                                                                        let _e3614 = ((_e3609 + _e3612) * 0.5f);
                                                                        phi_4725_ = type_56((_e3529.member_1.member_1 - ((_e3550 * _e3614) * _e3614)), (_e3550 * ((_e3612 * _e3612) - (_e3609 * _e3609))));
                                                                    } else {
                                                                        let _e3557 = round(((32f * _e3548) / (_e3548 + _e3549)));
                                                                        let _e3562 = select(select(u32(_e3557), 0u, (_e3557 < 0f)), 4294967295u, (_e3557 > 4294967000f));
                                                                        if (_e3562 < 1u) {
                                                                            phi_4709_ = 1u;
                                                                        } else {
                                                                            phi_4709_ = select(_e3562, 31u, (_e3562 > 31u));
                                                                        }
                                                                        let _e3567 = phi_4709_;
                                                                        if (_e3540 < _e3567) {
                                                                            let _e3588 = ((_e3567 - 1u) - _e3540);
                                                                            let _e3590 = f32(_e3567);
                                                                            let _e3591 = (f32(_e3588) / _e3590);
                                                                            let _e3594 = (f32((_e3588 + 1u)) / _e3590);
                                                                            let _e3596 = ((_e3591 + _e3594) * 0.5f);
                                                                            phi_4724_ = type_56((_e3547 - ((_e3548 * _e3596) * _e3596)), (_e3548 * ((_e3594 * _e3594) - (_e3591 * _e3591))));
                                                                        } else {
                                                                            let _e3570 = (_e3540 - _e3567);
                                                                            let _e3572 = f32((32u - _e3567));
                                                                            let _e3573 = (f32(_e3570) / _e3572);
                                                                            let _e3576 = (f32((_e3570 + 1u)) / _e3572);
                                                                            let _e3578 = ((_e3573 + _e3576) * 0.5f);
                                                                            phi_4724_ = type_56((_e3547 + ((_e3549 * _e3578) * _e3578)), (_e3549 * ((_e3576 * _e3576) - (_e3573 * _e3573))));
                                                                        }
                                                                        let _e3606 = phi_4724_;
                                                                        phi_4725_ = _e3606;
                                                                    }
                                                                    let _e3624 = phi_4725_;
                                                                    phi_4726_ = _e3624;
                                                                }
                                                                let _e3641 = phi_4726_;
                                                                let _e3647 = (_e334.member.x + (_e334.member_1.x * _e3641.member));
                                                                let _e3648 = (_e334.member.y + (_e334.member_1.y * _e3641.member));
                                                                let _e3649 = (_e334.member.z + (_e334.member_1.z * _e3641.member));
                                                                let _e3663 = (_e3647 - _e3278);
                                                                let _e3664 = (_e3648 - _e3281);
                                                                let _e3665 = (_e3649 - _e3284);
                                                                let _e3673 = ((((((_e3647 * _e3647) + (_e3648 * _e3648)) + (_e3649 * _e3649)) - (2f * (((_e3647 * _e3278) + (_e3648 * _e3281)) + (_e3649 * _e3284)))) + _e3290) / (sqrt((((_e3663 * _e3663) + (_e3664 * _e3664)) + (_e3665 * _e3665))) + _e3287));
                                                                let _e3677 = (1f - (abs((_e3673 - 25000f)) * 0.00006666667f));
                                                                if (_e3677 != _e3677) {
                                                                    phi_24585_ = true;
                                                                } else {
                                                                    phi_24585_ = (0f >= _e3677);
                                                                }
                                                                let _e3681 = phi_24585_;
                                                                let _e3683 = -(_e3673);
                                                                phi_4623_ = (_e3538 + vec3<f32>((exp((_e3683 / _e3296)) * _e3641.member_1), (exp((_e3683 / _e3299)) * _e3641.member_1), (select(_e3677, 0f, _e3681) * _e3641.member_1)));
                                                                phi_4626_ = (_e3540 + 1u);
                                                            } else {
                                                                phi_4623_ = vec3<f32>();
                                                                phi_4626_ = u32();
                                                            }
                                                            let _e3695 = phi_4623_;
                                                            let _e3697 = phi_4626_;
                                                            phi_4622_ = _e3695;
                                                            phi_4625_ = _e3697;
                                                            phi_33565_ = _e3127;
                                                            if !(_e3541) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3700 = phi_33565_;
                                                        phi_33861_ = _e3700;
                                                        if _e3700 {
                                                            break;
                                                        }
                                                        let _e3702 = local_20;
                                                        let _e3709 = local_21;
                                                        let _e3711 = ((_e3311 * 1.1f) * _e3709.y);
                                                        let _e3716 = local_22;
                                                        phi_33632_ = _e3700;
                                                        phi_4751_ = vec3<f32>(exp(-((((_e3302 * _e3702.x) + _e3711) + (_e3314 * _e3716.z)))), exp(-((((_e3305 * _e3702.x) + _e3711) + (_e3317 * _e3716.z)))), exp(-((((_e3308 * _e3702.x) + _e3711) + (_e3320 * _e3716.z)))));
                                                    } else {
                                                        phi_33632_ = _e3127;
                                                        phi_4751_ = vec3<f32>();
                                                    }
                                                    let _e3732 = phi_33632_;
                                                    let _e3734 = phi_4751_;
                                                    phi_33631_ = _e3732;
                                                    phi_4753_ = _e3734;
                                                    phi_4755_ = select(true, false, _e3532);
                                                } else {
                                                    phi_33631_ = _e3127;
                                                    phi_4753_ = vec3<f32>();
                                                    phi_4755_ = true;
                                                }
                                                let _e3737 = phi_33631_;
                                                let _e3739 = phi_4753_;
                                                let _e3741 = phi_4755_;
                                                let _e3745 = select(_e3739, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3391)), vec3(_e3741));
                                                let _e3749 = select(_e3745.y, _e3745.x, (_e3745.x > _e3745.y));
                                                phi_33627_ = _e3737;
                                                phi_4766_ = _e3745;
                                                phi_4767_ = (select(_e3745.z, _e3749, (_e3749 > _e3745.z)) > 0f);
                                            }
                                            let _e3755 = phi_33627_;
                                            let _e3757 = phi_4766_;
                                            let _e3759 = phi_4767_;
                                            phi_33626_ = _e3755;
                                            phi_4769_ = _e3757;
                                            phi_4770_ = _e3759;
                                            phi_4771_ = select(true, false, _e3276);
                                        } else {
                                            phi_33626_ = _e2896;
                                            phi_4769_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4770_ = _e1440;
                                            phi_4771_ = true;
                                        }
                                        let _e3762 = phi_33626_;
                                        let _e3764 = phi_4769_;
                                        let _e3766 = phi_4770_;
                                        let _e3768 = phi_4771_;
                                        phi_33625_ = _e3762;
                                        let _e7030 = local_61;
                                        phi_4775_ = _e7030;
                                        phi_4776_ = _e3764;
                                        phi_4777_ = _e3766;
                                        let _e7035 = local_62;
                                        phi_4778_ = _e7035;
                                        phi_4779_ = _e3768;
                                    } else {
                                        phi_33625_ = _e323;
                                        phi_4775_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4776_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4777_ = _e1440;
                                        phi_4778_ = vec3<f32>();
                                        phi_4779_ = true;
                                    }
                                    let _e3770 = phi_33625_;
                                    let _e3772 = phi_4775_;
                                    let _e3774 = phi_4776_;
                                    let _e3776 = phi_4777_;
                                    let _e3778 = phi_4778_;
                                    let _e3780 = phi_4779_;
                                    if _e3780 {
                                        if _e3776 {
                                            let _e3781 = (_e1292 + 20u);
                                            let _e3784 = global_6.member[(_e1292 + 21u)];
                                            let _e3785 = bitcast<u32>(_e3784);
                                            let _e3788 = global_6.member[(_e1292 + 22u)];
                                            let _e3789 = bitcast<u32>(_e3788);
                                            let _e3792 = global_6.member[(_e1292 + 23u)];
                                            let _e3796 = global_6.member[(_e1292 + 24u)];
                                            let _e3800 = global_6.member[(_e1292 + 25u)];
                                            let _e3804 = global_6.member[(_e1292 + 26u)];
                                            let _e3807 = global_6.member[_e1292];
                                            let _e3810 = global_6.member[(_e1292 + 1u)];
                                            let _e3813 = global_6.member[(_e1292 + 2u)];
                                            let _e3816 = global_6.member[(_e1292 + 3u)];
                                            let _e3819 = global_6.member[(_e1292 + 4u)];
                                            let _e3822 = global_6.member[(_e1292 + 5u)];
                                            let _e3825 = global_6.member[(_e1292 + 6u)];
                                            let _e3828 = global_6.member[(_e1292 + 7u)];
                                            let _e3831 = global_6.member[(_e1292 + 8u)];
                                            let _e3836 = (((_e334.member_1.x * _e3816) + (_e334.member_1.y * _e3819)) + (_e334.member_1.z * _e3822));
                                            let _e3838 = select(_e3836, -1f, (_e3836 < -1f));
                                            let _e3841 = asin(select(_e3838, 1f, (_e3838 > 1f)));
                                            let _e3852 = atan2((((_e334.member_1.x * _e3825) + (_e334.member_1.y * _e3828)) + (_e334.member_1.z * _e3831)), (((_e334.member_1.x * _e3807) + (_e334.member_1.y * _e3810)) + (_e334.member_1.z * _e3813)));
                                            if (_e3785 != 4294967295u) {
                                                let _e3855 = global_6.member[_e3785];
                                                let _e3858 = global_6.member[(_e3785 + 1u)];
                                                let _e3859 = bitcast<u32>(_e3858);
                                                let _e3862 = global_6.member[(_e3785 + 2u)];
                                                let _e3866 = global_6.member[(_e3785 + 3u)];
                                                let _e3868 = (4f * _e3855);
                                                phi_5099_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5102_ = 0u;
                                                loop {
                                                    let _e3873 = phi_5099_;
                                                    let _e3875 = phi_5102_;
                                                    local_66 = _e3873;
                                                    let _e3876 = (_e3875 < 6u);
                                                    if _e3876 {
                                                        let _e3877 = (_e3875 / 2u);
                                                        if (_e3877 == 0u) {
                                                            phi_24656_ = _e334.member_1.x;
                                                        } else {
                                                            phi_24656_ = select(_e334.member_1.z, _e334.member_1.y, (_e3877 == 1u));
                                                        }
                                                        let _e3885 = phi_24656_;
                                                        let _e3886 = (_e3885 * select(-1f, 1f, ((_e3875 % 2u) == 0u)));
                                                        if (_e3886 > 0f) {
                                                            let _e3890 = ((_e3877 + 1u) % 3u);
                                                            if (_e3890 == 0u) {
                                                                phi_24671_ = _e334.member_1.x;
                                                            } else {
                                                                phi_24671_ = select(_e334.member_1.z, _e334.member_1.y, (_e3890 == 1u));
                                                            }
                                                            let _e3895 = phi_24671_;
                                                            let _e3896 = (_e3895 / _e3886);
                                                            let _e3898 = ((_e3877 + 2u) % 3u);
                                                            if (_e3898 == 0u) {
                                                                phi_24686_ = _e334.member_1.x;
                                                            } else {
                                                                phi_24686_ = select(_e334.member_1.z, _e334.member_1.y, (_e3898 == 1u));
                                                            }
                                                            let _e3903 = phi_24686_;
                                                            let _e3904 = (_e3903 / _e3886);
                                                            let _e3906 = (1f + (_e3855 * 12f));
                                                            if (abs(_e3896) <= _e3906) {
                                                                if (abs(_e3904) <= _e3906) {
                                                                    let _e3912 = f32(_e3859);
                                                                    let _e3914 = floor((((_e3896 + 1f) * 0.5f) * _e3912));
                                                                    let _e3921 = select(0i, select(select(i32(_e3914), i32(-2147483648), (_e3914 < -2147483600f)), 2147483647i, (_e3914 > 2147483500f)), (_e3914 == _e3914));
                                                                    let _e3923 = (bitcast<i32>(_e3859) - 1i);
                                                                    let _e3924 = (0i <= _e3923);
                                                                    if _e3924 {
                                                                    } else {
                                                                        phi_33620_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3921 < 0i) {
                                                                        phi_5173_ = 0i;
                                                                    } else {
                                                                        phi_5173_ = select(_e3921, _e3923, (_e3921 > _e3923));
                                                                    }
                                                                    let _e3929 = phi_5173_;
                                                                    let _e3933 = floor((((_e3904 + 1f) * 0.5f) * _e3912));
                                                                    let _e3940 = select(0i, select(select(i32(_e3933), i32(-2147483648), (_e3933 < -2147483600f)), 2147483647i, (_e3933 > 2147483500f)), (_e3933 == _e3933));
                                                                    if _e3924 {
                                                                    } else {
                                                                        phi_33620_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3940 < 0i) {
                                                                        phi_5203_ = 0i;
                                                                    } else {
                                                                        phi_5203_ = select(_e3940, _e3923, (_e3940 > _e3923));
                                                                    }
                                                                    let _e3945 = phi_5203_;
                                                                    let _e3946 = (_e3929 - 1i);
                                                                    phi_5211_ = _e3873;
                                                                    phi_5214_ = select(0i, _e3946, (0i < _e3946));
                                                                    loop {
                                                                        let _e3950 = phi_5211_;
                                                                        let _e3952 = phi_5214_;
                                                                        let _e3953 = (_e3929 + 1i);
                                                                        local_65 = _e3950;
                                                                        let _e3956 = (_e3952 <= select(_e3953, _e3923, (_e3923 < _e3953)));
                                                                        if _e3956 {
                                                                            let _e3957 = (_e3945 - 1i);
                                                                            phi_5237_ = _e3950;
                                                                            phi_5240_ = select(0i, _e3957, (0i < _e3957));
                                                                            loop {
                                                                                let _e3961 = phi_5237_;
                                                                                let _e3963 = phi_5240_;
                                                                                let _e3964 = (_e3945 + 1i);
                                                                                local_64 = _e3961;
                                                                                let _e3967 = (_e3963 <= select(_e3964, _e3923, (_e3923 < _e3964)));
                                                                                if _e3967 {
                                                                                    let _e3974 = (bitcast<u32>(_e3862) + ((((_e3875 * _e3859) + bitcast<u32>(_e3952)) * _e3859) + bitcast<u32>(_e3963)));
                                                                                    let _e3976 = global_6.member[_e3974];
                                                                                    let _e3980 = global_6.member[(_e3974 + 1u)];
                                                                                    phi_5311_ = _e3961;
                                                                                    phi_5314_ = bitcast<u32>(_e3976);
                                                                                    loop {
                                                                                        let _e3983 = phi_5311_;
                                                                                        let _e3985 = phi_5314_;
                                                                                        local_63 = _e3983;
                                                                                        let _e3986 = (_e3985 < bitcast<u32>(_e3980));
                                                                                        if _e3986 {
                                                                                            let _e3988 = (bitcast<u32>(_e3866) + (6u * _e3985));
                                                                                            let _e3990 = global_6.member[_e3988];
                                                                                            let _e3993 = global_6.member[(_e3988 + 1u)];
                                                                                            let _e3996 = global_6.member[(_e3988 + 2u)];
                                                                                            let _e3999 = global_6.member[(_e3988 + 3u)];
                                                                                            let _e4002 = global_6.member[(_e3988 + 4u)];
                                                                                            let _e4005 = global_6.member[(_e3988 + 5u)];
                                                                                            let _e4006 = (_e334.member_1.x - _e3990);
                                                                                            let _e4007 = (_e334.member_1.y - _e3993);
                                                                                            let _e4008 = (_e334.member_1.z - _e3996);
                                                                                            let _e4013 = (((_e4006 * _e4006) + (_e4007 * _e4007)) + (_e4008 * _e4008));
                                                                                            if (_e4013 < (_e3868 * _e3868)) {
                                                                                                let _e4021 = ((1f / ((6.2831855f * _e3855) * _e3855)) * exp((-(_e4013) / ((2f * _e3855) * _e3855))));
                                                                                                phi_5434_ = vec3<f32>((_e3983.x + (_e3999 * _e4021)), (_e3983.y + (_e4002 * _e4021)), (_e3983.z + (_e4005 * _e4021)));
                                                                                            } else {
                                                                                                phi_5434_ = _e3983;
                                                                                            }
                                                                                            let _e4033 = phi_5434_;
                                                                                            phi_5312_ = _e4033;
                                                                                            phi_5315_ = (_e3985 + 1u);
                                                                                        } else {
                                                                                            phi_5312_ = vec3<f32>();
                                                                                            phi_5315_ = u32();
                                                                                        }
                                                                                        let _e4036 = phi_5312_;
                                                                                        let _e4038 = phi_5315_;
                                                                                        phi_5311_ = _e4036;
                                                                                        phi_5314_ = _e4038;
                                                                                        if !(_e3986) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7055 = local_63;
                                                                                    phi_5238_ = _e7055;
                                                                                    phi_5241_ = (_e3963 + 1i);
                                                                                } else {
                                                                                    phi_5238_ = vec3<f32>();
                                                                                    phi_5241_ = i32();
                                                                                }
                                                                                let _e4042 = phi_5238_;
                                                                                let _e4044 = phi_5241_;
                                                                                phi_5237_ = _e4042;
                                                                                phi_5240_ = _e4044;
                                                                                if !(_e3967) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7059 = local_64;
                                                                            phi_5212_ = _e7059;
                                                                            phi_5215_ = (_e3952 + 1i);
                                                                        } else {
                                                                            phi_5212_ = vec3<f32>();
                                                                            phi_5215_ = i32();
                                                                        }
                                                                        let _e4048 = phi_5212_;
                                                                        let _e4050 = phi_5215_;
                                                                        phi_5211_ = _e4048;
                                                                        phi_5214_ = _e4050;
                                                                        if !(_e3956) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7063 = local_65;
                                                                    phi_5439_ = _e7063;
                                                                } else {
                                                                    phi_5439_ = _e3873;
                                                                }
                                                                let _e4053 = phi_5439_;
                                                                phi_5440_ = _e4053;
                                                            } else {
                                                                phi_5440_ = _e3873;
                                                            }
                                                            let _e4055 = phi_5440_;
                                                            phi_5441_ = _e4055;
                                                        } else {
                                                            phi_5441_ = _e3873;
                                                        }
                                                        let _e4057 = phi_5441_;
                                                        phi_5100_ = _e4057;
                                                        phi_5103_ = (_e3875 + 1u);
                                                    } else {
                                                        phi_5100_ = vec3<f32>();
                                                        phi_5103_ = u32();
                                                    }
                                                    let _e4060 = phi_5100_;
                                                    let _e4062 = phi_5103_;
                                                    phi_5099_ = _e4060;
                                                    phi_5102_ = _e4062;
                                                    phi_33620_ = _e3770;
                                                    if !(_e3876) {
                                                        break;
                                                    }
                                                }
                                                let _e4065 = phi_33620_;
                                                phi_33861_ = _e4065;
                                                if _e4065 {
                                                    break;
                                                }
                                                phi_33662_ = _e4065;
                                                let _e7072 = local_66;
                                                phi_5461_ = _e7072;
                                            } else {
                                                phi_33662_ = _e3770;
                                                phi_5461_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4067 = phi_33662_;
                                            let _e4069 = phi_5461_;
                                            if (_e3789 != 4294967295u) {
                                                let _e4072 = global_6.member[_e3789];
                                                let _e4075 = global_6.member[(_e3789 + 1u)];
                                                let _e4078 = global_6.member[(_e3789 + 2u)];
                                                let _e4081 = global_6.member[(_e3789 + 3u)];
                                                let _e4084 = global_6.member[(_e3789 + 4u)];
                                                let _e4087 = global_6.member[(_e3789 + 5u)];
                                                let _e4090 = global_6.member[(_e3789 + 6u)];
                                                let _e4093 = global_6.member[(_e3789 + 7u)];
                                                let _e4096 = global_6.member[(_e3789 + 8u)];
                                                let _e4099 = global_6.member[(_e3789 + 9u)];
                                                let _e4102 = global_6.member[(_e3789 + 10u)];
                                                let _e4106 = global_6.member[(_e3789 + 11u)];
                                                let _e4109 = global_6.member[(_e3789 + 12u)];
                                                let _e4112 = global_6.member[(_e3789 + 13u)];
                                                let _e4114 = (_e4072 > 0f);
                                                let _e4115 = (_e4075 > 0f);
                                                if _e4115 {
                                                    let _e4118 = (abs(_e3841) <= 0.8f);
                                                    let _e4119 = select(2u, 0u, _e4114);
                                                    phi_5674_ = _e4119;
                                                    phi_5675_ = _e4118;
                                                    phi_5678_ = _e4119;
                                                    phi_5679_ = _e4118;
                                                    phi_5680_ = select(true, false, _e4118);
                                                } else {
                                                    phi_5674_ = u32();
                                                    phi_5675_ = bool();
                                                    phi_5678_ = select(2u, 0u, _e4114);
                                                    phi_5679_ = false;
                                                    phi_5680_ = true;
                                                }
                                                let _e4122 = phi_5674_;
                                                let _e4124 = phi_5675_;
                                                let _e4126 = phi_5678_;
                                                let _e4128 = phi_5679_;
                                                let _e4130 = phi_5680_;
                                                let _e4132 = select(select(f32(), 0f, _e4115), 0f, _e4130);
                                                phi_5690_ = _e4132;
                                                phi_5693_ = _e4132;
                                                phi_5695_ = select(_e4122, _e4126, _e4130);
                                                phi_5697_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5699_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5701_ = 0f;
                                                phi_5703_ = 1f;
                                                loop {
                                                    let _e4137 = phi_5690_;
                                                    let _e4139 = phi_5693_;
                                                    let _e4141 = phi_5695_;
                                                    let _e4143 = phi_5697_;
                                                    let _e4145 = phi_5699_;
                                                    let _e4147 = phi_5701_;
                                                    let _e4149 = phi_5703_;
                                                    local_26 = _e4149;
                                                    local_27 = _e4149;
                                                    local_28 = _e4149;
                                                    local_29 = _e4149;
                                                    local_30 = _e4147;
                                                    local_31 = _e4139;
                                                    local_32 = _e4137;
                                                    let _e4150 = (_e4141 < select(11u, 2u, _e4130));
                                                    if _e4150 {
                                                        let _e4151 = (_e4141 < 2u);
                                                        if _e4151 {
                                                            let _e4207 = select(23f, 4f, (_e4141 == 0u));
                                                            phi_5781_ = 4u;
                                                            phi_5783_ = vec3<f32>((_e334.member_1.x * _e4207), (_e334.member_1.y * _e4207), (_e334.member_1.z * _e4207));
                                                        } else {
                                                            if (_e4141 == 2u) {
                                                                phi_5779_ = 3u;
                                                                phi_5780_ = vec3<f32>(((_e334.member_1.x * 1.5f) + 5f), ((_e334.member_1.y * 1.5f) + 5f), ((_e334.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4153 = (_e4141 >= 7u);
                                                                if _e4153 {
                                                                    phi_5741_ = (_e4141 - 7u);
                                                                } else {
                                                                    phi_5741_ = (_e4141 - 3u);
                                                                }
                                                                let _e4157 = phi_5741_;
                                                                if _e4153 {
                                                                    phi_5756_ = vec3<f32>(((_e334.member_1.x * 11f) + -3f), ((_e334.member_1.y * 11f) + 1f), ((_e334.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5756_ = vec3<f32>((_e334.member_1.x * 7f), (_e334.member_1.y * 7f), (_e334.member_1.z * 7f));
                                                                }
                                                                let _e4170 = phi_5756_;
                                                                if (_e4157 < 3u) {
                                                                    if (_e4157 == 0u) {
                                                                        phi_24701_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_24701_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4157 == 1u)));
                                                                    }
                                                                    let _e4189 = phi_24701_;
                                                                    phi_5777_ = 4u;
                                                                    phi_5778_ = (_e4170 + _e4189);
                                                                } else {
                                                                    if _e4153 {
                                                                        phi_5776_ = (_e4170 + _e4143);
                                                                    } else {
                                                                        phi_5776_ = (_e4170 + vec3<f32>((_e4145.x * 1.1f), (_e4145.y * 1.1f), (_e4145.z * 1.1f)));
                                                                    }
                                                                    let _e4182 = phi_5776_;
                                                                    phi_5777_ = select(7u, 5u, _e4153);
                                                                    phi_5778_ = _e4182;
                                                                }
                                                                let _e4192 = phi_5777_;
                                                                let _e4194 = phi_5778_;
                                                                phi_5779_ = _e4192;
                                                                phi_5780_ = _e4194;
                                                            }
                                                            let _e4203 = phi_5779_;
                                                            let _e4205 = phi_5780_;
                                                            phi_5781_ = _e4203;
                                                            phi_5783_ = _e4205;
                                                        }
                                                        let _e4213 = phi_5781_;
                                                        let _e4215 = phi_5783_;
                                                        let _e4217 = select(vec3<f32>(_e4106, _e4109, _e4112), vec3<f32>(_e4096, _e4099, _e4102), vec3(_e4151));
                                                        let _e4219 = global_6.member[_e3781];
                                                        let _e4220 = bitcast<u32>(_e4219);
                                                        let _e4223 = global_6.member[(_e1292 + 11u)];
                                                        let _e4226 = global_6.member[(_e1292 + 12u)];
                                                        let _e4229 = global_6.member[(_e1292 + 13u)];
                                                        let _e4232 = global_6.member[(_e1292 + 14u)];
                                                        let _e4235 = global_6.member[(_e1292 + 15u)];
                                                        let _e4238 = global_6.member[(_e1292 + 16u)];
                                                        let _e4241 = global_6.member[(_e1292 + 17u)];
                                                        let _e4244 = global_6.member[(_e1292 + 18u)];
                                                        let _e4247 = global_6.member[(_e1292 + 19u)];
                                                        phi_5918_ = 0u;
                                                        phi_5921_ = 0f;
                                                        phi_5923_ = 1f;
                                                        phi_5925_ = 0f;
                                                        phi_5927_ = _e4215;
                                                        loop {
                                                            let _e4249 = phi_5918_;
                                                            let _e4251 = phi_5921_;
                                                            let _e4253 = phi_5923_;
                                                            let _e4255 = phi_5925_;
                                                            let _e4257 = phi_5927_;
                                                            local_23 = _e4251;
                                                            local_24 = _e4255;
                                                            local_25 = _e4251;
                                                            let _e4258 = (_e4249 < _e4213);
                                                            if _e4258 {
                                                                let _e4261 = (_e4257.x + _e4217.x);
                                                                let _e4264 = (_e4257.y + _e4217.y);
                                                                let _e4267 = (_e4257.z + _e4217.z);
                                                                let _e4268 = floor(_e4261);
                                                                let _e4269 = floor(_e4264);
                                                                let _e4270 = floor(_e4267);
                                                                let _e4279 = bitcast<u32>((select(0i, select(select(i32(_e4268), i32(-2147483648), (_e4268 < -2147483600f)), 2147483647i, (_e4268 > 2147483500f)), (_e4268 == _e4268)) & 255i));
                                                                let _e4288 = bitcast<u32>((select(0i, select(select(i32(_e4269), i32(-2147483648), (_e4269 < -2147483600f)), 2147483647i, (_e4269 > 2147483500f)), (_e4269 == _e4269)) & 255i));
                                                                let _e4297 = bitcast<u32>((select(0i, select(select(i32(_e4270), i32(-2147483648), (_e4270 < -2147483600f)), 2147483647i, (_e4270 > 2147483500f)), (_e4270 == _e4270)) & 255i));
                                                                let _e4298 = (_e4261 - _e4268);
                                                                let _e4299 = (_e4264 - _e4269);
                                                                let _e4300 = (_e4267 - _e4270);
                                                                let _e4307 = (((_e4298 * _e4298) * _e4298) * ((_e4298 * ((_e4298 * 6f) - 15f)) + 10f));
                                                                let _e4314 = (((_e4299 * _e4299) * _e4299) * ((_e4299 * ((_e4299 * 6f) - 15f)) + 10f));
                                                                let _e4324 = global_6.member[(_e4220 + _e4279)];
                                                                let _e4330 = (select(select(u32(_e4324), 0u, (_e4324 < 0f)), 4294967295u, (_e4324 > 4294967000f)) + _e4288);
                                                                let _e4333 = global_6.member[(_e4220 + _e4330)];
                                                                let _e4339 = (select(select(u32(_e4333), 0u, (_e4333 < 0f)), 4294967295u, (_e4333 > 4294967000f)) + _e4297);
                                                                let _e4343 = global_6.member[(_e4220 + (_e4330 + 1u))];
                                                                let _e4349 = (select(select(u32(_e4343), 0u, (_e4343 < 0f)), 4294967295u, (_e4343 > 4294967000f)) + _e4297);
                                                                let _e4353 = global_6.member[(_e4220 + (_e4279 + 1u))];
                                                                let _e4359 = (select(select(u32(_e4353), 0u, (_e4353 < 0f)), 4294967295u, (_e4353 > 4294967000f)) + _e4288);
                                                                let _e4362 = global_6.member[(_e4220 + _e4359)];
                                                                let _e4368 = (select(select(u32(_e4362), 0u, (_e4362 < 0f)), 4294967295u, (_e4362 > 4294967000f)) + _e4297);
                                                                let _e4372 = global_6.member[(_e4220 + (_e4359 + 1u))];
                                                                let _e4378 = (select(select(u32(_e4372), 0u, (_e4372 < 0f)), 4294967295u, (_e4372 > 4294967000f)) + _e4297);
                                                                let _e4381 = global_6.member[(_e4220 + _e4339)];
                                                                let _e4387 = (select(select(u32(_e4381), 0u, (_e4381 < 0f)), 4294967295u, (_e4381 > 4294967000f)) % 12u);
                                                                let _e4390 = select(-1f, 1f, ((_e4387 & 1u) == 0u));
                                                                let _e4393 = select(-1f, 1f, ((_e4387 & 2u) == 0u));
                                                                if (_e4387 < 4u) {
                                                                    phi_24762_ = ((_e4390 * _e4298) + (_e4393 * _e4299));
                                                                } else {
                                                                    if (_e4387 < 8u) {
                                                                        phi_24756_ = ((_e4390 * _e4298) + (_e4393 * _e4300));
                                                                    } else {
                                                                        phi_24756_ = ((_e4390 * _e4299) + (_e4393 * _e4300));
                                                                    }
                                                                    let _e4403 = phi_24756_;
                                                                    phi_24762_ = _e4403;
                                                                }
                                                                let _e4408 = phi_24762_;
                                                                let _e4411 = global_6.member[(_e4220 + _e4368)];
                                                                let _e4417 = (_e4298 - 1f);
                                                                let _e4418 = (select(select(u32(_e4411), 0u, (_e4411 < 0f)), 4294967295u, (_e4411 > 4294967000f)) % 12u);
                                                                let _e4421 = select(-1f, 1f, ((_e4418 & 1u) == 0u));
                                                                let _e4424 = select(-1f, 1f, ((_e4418 & 2u) == 0u));
                                                                if (_e4418 < 4u) {
                                                                    phi_24796_ = ((_e4421 * _e4417) + (_e4424 * _e4299));
                                                                } else {
                                                                    if (_e4418 < 8u) {
                                                                        phi_24790_ = ((_e4421 * _e4417) + (_e4424 * _e4300));
                                                                    } else {
                                                                        phi_24790_ = ((_e4421 * _e4299) + (_e4424 * _e4300));
                                                                    }
                                                                    let _e4434 = phi_24790_;
                                                                    phi_24796_ = _e4434;
                                                                }
                                                                let _e4439 = phi_24796_;
                                                                let _e4442 = global_6.member[(_e4220 + _e4349)];
                                                                let _e4448 = (_e4299 - 1f);
                                                                let _e4449 = (select(select(u32(_e4442), 0u, (_e4442 < 0f)), 4294967295u, (_e4442 > 4294967000f)) % 12u);
                                                                let _e4452 = select(-1f, 1f, ((_e4449 & 1u) == 0u));
                                                                let _e4455 = select(-1f, 1f, ((_e4449 & 2u) == 0u));
                                                                if (_e4449 < 4u) {
                                                                    phi_24830_ = ((_e4452 * _e4298) + (_e4455 * _e4448));
                                                                } else {
                                                                    if (_e4449 < 8u) {
                                                                        phi_24824_ = ((_e4452 * _e4298) + (_e4455 * _e4300));
                                                                    } else {
                                                                        phi_24824_ = ((_e4452 * _e4448) + (_e4455 * _e4300));
                                                                    }
                                                                    let _e4465 = phi_24824_;
                                                                    phi_24830_ = _e4465;
                                                                }
                                                                let _e4470 = phi_24830_;
                                                                let _e4473 = global_6.member[(_e4220 + _e4378)];
                                                                let _e4479 = (select(select(u32(_e4473), 0u, (_e4473 < 0f)), 4294967295u, (_e4473 > 4294967000f)) % 12u);
                                                                let _e4482 = select(-1f, 1f, ((_e4479 & 1u) == 0u));
                                                                let _e4485 = select(-1f, 1f, ((_e4479 & 2u) == 0u));
                                                                if (_e4479 < 4u) {
                                                                    phi_24864_ = ((_e4482 * _e4417) + (_e4485 * _e4448));
                                                                } else {
                                                                    if (_e4479 < 8u) {
                                                                        phi_24858_ = ((_e4482 * _e4417) + (_e4485 * _e4300));
                                                                    } else {
                                                                        phi_24858_ = ((_e4482 * _e4448) + (_e4485 * _e4300));
                                                                    }
                                                                    let _e4495 = phi_24858_;
                                                                    phi_24864_ = _e4495;
                                                                }
                                                                let _e4500 = phi_24864_;
                                                                let _e4504 = global_6.member[(_e4220 + (_e4339 + 1u))];
                                                                let _e4510 = (_e4300 - 1f);
                                                                let _e4511 = (select(select(u32(_e4504), 0u, (_e4504 < 0f)), 4294967295u, (_e4504 > 4294967000f)) % 12u);
                                                                let _e4514 = select(-1f, 1f, ((_e4511 & 1u) == 0u));
                                                                let _e4517 = select(-1f, 1f, ((_e4511 & 2u) == 0u));
                                                                if (_e4511 < 4u) {
                                                                    phi_24898_ = ((_e4514 * _e4298) + (_e4517 * _e4299));
                                                                } else {
                                                                    if (_e4511 < 8u) {
                                                                        phi_24892_ = ((_e4514 * _e4298) + (_e4517 * _e4510));
                                                                    } else {
                                                                        phi_24892_ = ((_e4514 * _e4299) + (_e4517 * _e4510));
                                                                    }
                                                                    let _e4527 = phi_24892_;
                                                                    phi_24898_ = _e4527;
                                                                }
                                                                let _e4532 = phi_24898_;
                                                                let _e4536 = global_6.member[(_e4220 + (_e4368 + 1u))];
                                                                let _e4542 = (select(select(u32(_e4536), 0u, (_e4536 < 0f)), 4294967295u, (_e4536 > 4294967000f)) % 12u);
                                                                let _e4545 = select(-1f, 1f, ((_e4542 & 1u) == 0u));
                                                                let _e4548 = select(-1f, 1f, ((_e4542 & 2u) == 0u));
                                                                if (_e4542 < 4u) {
                                                                    phi_24932_ = ((_e4545 * _e4417) + (_e4548 * _e4299));
                                                                } else {
                                                                    if (_e4542 < 8u) {
                                                                        phi_24926_ = ((_e4545 * _e4417) + (_e4548 * _e4510));
                                                                    } else {
                                                                        phi_24926_ = ((_e4545 * _e4299) + (_e4548 * _e4510));
                                                                    }
                                                                    let _e4558 = phi_24926_;
                                                                    phi_24932_ = _e4558;
                                                                }
                                                                let _e4563 = phi_24932_;
                                                                let _e4567 = global_6.member[(_e4220 + (_e4349 + 1u))];
                                                                let _e4573 = (select(select(u32(_e4567), 0u, (_e4567 < 0f)), 4294967295u, (_e4567 > 4294967000f)) % 12u);
                                                                let _e4576 = select(-1f, 1f, ((_e4573 & 1u) == 0u));
                                                                let _e4579 = select(-1f, 1f, ((_e4573 & 2u) == 0u));
                                                                if (_e4573 < 4u) {
                                                                    phi_24966_ = ((_e4576 * _e4298) + (_e4579 * _e4448));
                                                                } else {
                                                                    if (_e4573 < 8u) {
                                                                        phi_24960_ = ((_e4576 * _e4298) + (_e4579 * _e4510));
                                                                    } else {
                                                                        phi_24960_ = ((_e4576 * _e4448) + (_e4579 * _e4510));
                                                                    }
                                                                    let _e4589 = phi_24960_;
                                                                    phi_24966_ = _e4589;
                                                                }
                                                                let _e4594 = phi_24966_;
                                                                let _e4598 = global_6.member[(_e4220 + (_e4378 + 1u))];
                                                                let _e4604 = (select(select(u32(_e4598), 0u, (_e4598 < 0f)), 4294967295u, (_e4598 > 4294967000f)) % 12u);
                                                                let _e4607 = select(-1f, 1f, ((_e4604 & 1u) == 0u));
                                                                let _e4610 = select(-1f, 1f, ((_e4604 & 2u) == 0u));
                                                                if (_e4604 < 4u) {
                                                                    phi_25000_ = ((_e4607 * _e4417) + (_e4610 * _e4448));
                                                                } else {
                                                                    if (_e4604 < 8u) {
                                                                        phi_24994_ = ((_e4607 * _e4417) + (_e4610 * _e4510));
                                                                    } else {
                                                                        phi_24994_ = ((_e4607 * _e4448) + (_e4610 * _e4510));
                                                                    }
                                                                    let _e4620 = phi_24994_;
                                                                    phi_25000_ = _e4620;
                                                                }
                                                                let _e4625 = phi_25000_;
                                                                let _e4628 = (_e4408 + (_e4307 * (_e4439 - _e4408)));
                                                                let _e4634 = (_e4532 + (_e4307 * (_e4563 - _e4532)));
                                                                let _e4640 = (_e4628 + (_e4314 * ((_e4470 + (_e4307 * (_e4500 - _e4470))) - _e4628)));
                                                                phi_5919_ = (_e4249 + 1u);
                                                                phi_5922_ = (_e4251 + _e4253);
                                                                phi_5924_ = (_e4253 * 0.5f);
                                                                phi_5926_ = (_e4255 + ((_e4640 + ((((_e4300 * _e4300) * _e4300) * ((_e4300 * ((_e4300 * 6f) - 15f)) + 10f)) * ((_e4634 + (_e4314 * ((_e4594 + (_e4307 * (_e4625 - _e4594))) - _e4634))) - _e4640))) * _e4253));
                                                                phi_5928_ = vec3<f32>(((((_e4223 * _e4257.x) + (_e4226 * _e4257.y)) + (_e4229 * _e4257.z)) * 2.03f), ((((_e4232 * _e4257.x) + (_e4235 * _e4257.y)) + (_e4238 * _e4257.z)) * 2.03f), ((((_e4241 * _e4257.x) + (_e4244 * _e4257.y)) + (_e4247 * _e4257.z)) * 2.03f));
                                                            } else {
                                                                phi_5919_ = u32();
                                                                phi_5922_ = f32();
                                                                phi_5924_ = f32();
                                                                phi_5926_ = f32();
                                                                phi_5928_ = vec3<f32>();
                                                            }
                                                            let _e4672 = phi_5919_;
                                                            let _e4674 = phi_5922_;
                                                            let _e4676 = phi_5924_;
                                                            let _e4678 = phi_5926_;
                                                            let _e4680 = phi_5928_;
                                                            phi_5918_ = _e4672;
                                                            phi_5921_ = _e4674;
                                                            phi_5923_ = _e4676;
                                                            phi_5925_ = _e4678;
                                                            phi_5927_ = _e4680;
                                                            if !(_e4258) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4683 = local_23;
                                                        if (_e4683 > 0f) {
                                                            let _e4686 = local_24;
                                                            let _e4688 = local_25;
                                                            phi_6344_ = (_e4686 / _e4688);
                                                        } else {
                                                            phi_6344_ = 0f;
                                                        }
                                                        let _e4691 = phi_6344_;
                                                        if (_e4141 == 0u) {
                                                            phi_6427_ = _e4137;
                                                            phi_6428_ = _e4139;
                                                            phi_6429_ = _e4143;
                                                            phi_6430_ = _e4145;
                                                            phi_6431_ = _e4147;
                                                            phi_6432_ = (_e4149 + (1.5f * _e4691));
                                                        } else {
                                                            if (_e4141 == 1u) {
                                                                phi_6421_ = _e4137;
                                                                phi_6422_ = _e4139;
                                                                phi_6423_ = _e4143;
                                                                phi_6424_ = _e4145;
                                                                phi_6425_ = _e4147;
                                                                phi_6426_ = (_e4149 + (0.8f * _e4691));
                                                            } else {
                                                                let _e4694 = (_e4141 == 2u);
                                                                if _e4694 {
                                                                    phi_6416_ = _e4137;
                                                                    phi_6417_ = _e4139;
                                                                    phi_6418_ = _e4143;
                                                                    phi_6419_ = _e4145;
                                                                } else {
                                                                    if (_e4141 == 3u) {
                                                                        phi_6412_ = _e4137;
                                                                        phi_6413_ = _e4139;
                                                                        phi_6414_ = _e4143;
                                                                        phi_6415_ = vec3<f32>(_e4691, _e4145.y, _e4145.z);
                                                                    } else {
                                                                        if (_e4141 == 4u) {
                                                                            phi_6408_ = _e4137;
                                                                            phi_6409_ = _e4139;
                                                                            phi_6410_ = _e4143;
                                                                            phi_6411_ = vec3<f32>(_e4145.x, _e4691, _e4145.z);
                                                                        } else {
                                                                            if (_e4141 == 5u) {
                                                                                phi_6404_ = _e4137;
                                                                                phi_6405_ = _e4139;
                                                                                phi_6406_ = _e4143;
                                                                                phi_6407_ = vec3<f32>(_e4145.x, _e4145.y, _e4691);
                                                                            } else {
                                                                                let _e4698 = (_e4141 == 6u);
                                                                                if _e4698 {
                                                                                    phi_6401_ = _e4137;
                                                                                    phi_6403_ = _e4143;
                                                                                } else {
                                                                                    if (_e4141 == 7u) {
                                                                                        phi_6399_ = _e4137;
                                                                                        phi_6400_ = vec3<f32>(_e4691, _e4143.y, _e4143.z);
                                                                                    } else {
                                                                                        if (_e4141 == 8u) {
                                                                                            phi_6397_ = _e4137;
                                                                                            phi_6398_ = vec3<f32>(_e4143.x, _e4691, _e4143.z);
                                                                                        } else {
                                                                                            let _e4701 = (_e4141 == 9u);
                                                                                            if _e4701 {
                                                                                                phi_6396_ = vec3<f32>(_e4143.x, _e4143.y, _e4691);
                                                                                            } else {
                                                                                                phi_6396_ = _e4143;
                                                                                            }
                                                                                            let _e4707 = phi_6396_;
                                                                                            phi_6397_ = select(_e4691, _e4137, _e4701);
                                                                                            phi_6398_ = _e4707;
                                                                                        }
                                                                                        let _e4714 = phi_6397_;
                                                                                        let _e4716 = phi_6398_;
                                                                                        phi_6399_ = _e4714;
                                                                                        phi_6400_ = _e4716;
                                                                                    }
                                                                                    let _e4722 = phi_6399_;
                                                                                    let _e4724 = phi_6400_;
                                                                                    phi_6401_ = _e4722;
                                                                                    phi_6403_ = _e4724;
                                                                                }
                                                                                let _e4726 = phi_6401_;
                                                                                let _e4728 = phi_6403_;
                                                                                phi_6404_ = _e4726;
                                                                                phi_6405_ = select(_e4139, _e4691, _e4698);
                                                                                phi_6406_ = _e4728;
                                                                                phi_6407_ = _e4145;
                                                                            }
                                                                            let _e4735 = phi_6404_;
                                                                            let _e4737 = phi_6405_;
                                                                            let _e4739 = phi_6406_;
                                                                            let _e4741 = phi_6407_;
                                                                            phi_6408_ = _e4735;
                                                                            phi_6409_ = _e4737;
                                                                            phi_6410_ = _e4739;
                                                                            phi_6411_ = _e4741;
                                                                        }
                                                                        let _e4747 = phi_6408_;
                                                                        let _e4749 = phi_6409_;
                                                                        let _e4751 = phi_6410_;
                                                                        let _e4753 = phi_6411_;
                                                                        phi_6412_ = _e4747;
                                                                        phi_6413_ = _e4749;
                                                                        phi_6414_ = _e4751;
                                                                        phi_6415_ = _e4753;
                                                                    }
                                                                    let _e4759 = phi_6412_;
                                                                    let _e4761 = phi_6413_;
                                                                    let _e4763 = phi_6414_;
                                                                    let _e4765 = phi_6415_;
                                                                    phi_6416_ = _e4759;
                                                                    phi_6417_ = _e4761;
                                                                    phi_6418_ = _e4763;
                                                                    phi_6419_ = _e4765;
                                                                }
                                                                let _e4767 = phi_6416_;
                                                                let _e4769 = phi_6417_;
                                                                let _e4771 = phi_6418_;
                                                                let _e4773 = phi_6419_;
                                                                phi_6421_ = _e4767;
                                                                phi_6422_ = _e4769;
                                                                phi_6423_ = _e4771;
                                                                phi_6424_ = _e4773;
                                                                phi_6425_ = select(_e4147, _e4691, _e4694);
                                                                phi_6426_ = _e4149;
                                                            }
                                                            let _e4778 = phi_6421_;
                                                            let _e4780 = phi_6422_;
                                                            let _e4782 = phi_6423_;
                                                            let _e4784 = phi_6424_;
                                                            let _e4786 = phi_6425_;
                                                            let _e4788 = phi_6426_;
                                                            phi_6427_ = _e4778;
                                                            phi_6428_ = _e4780;
                                                            phi_6429_ = _e4782;
                                                            phi_6430_ = _e4784;
                                                            phi_6431_ = _e4786;
                                                            phi_6432_ = _e4788;
                                                        }
                                                        let _e4792 = phi_6427_;
                                                        let _e4794 = phi_6428_;
                                                        let _e4796 = phi_6429_;
                                                        let _e4798 = phi_6430_;
                                                        let _e4800 = phi_6431_;
                                                        let _e4802 = phi_6432_;
                                                        phi_5691_ = _e4792;
                                                        phi_5694_ = _e4794;
                                                        phi_5696_ = (_e4141 + 1u);
                                                        phi_5698_ = _e4796;
                                                        phi_5700_ = _e4798;
                                                        phi_5702_ = _e4800;
                                                        phi_5704_ = _e4802;
                                                    } else {
                                                        phi_5691_ = f32();
                                                        phi_5694_ = f32();
                                                        phi_5696_ = u32();
                                                        phi_5698_ = vec3<f32>();
                                                        phi_5700_ = vec3<f32>();
                                                        phi_5702_ = f32();
                                                        phi_5704_ = f32();
                                                    }
                                                    let _e4805 = phi_5691_;
                                                    let _e4807 = phi_5694_;
                                                    let _e4809 = phi_5696_;
                                                    let _e4811 = phi_5698_;
                                                    let _e4813 = phi_5700_;
                                                    let _e4815 = phi_5702_;
                                                    let _e4817 = phi_5704_;
                                                    phi_5690_ = _e4805;
                                                    phi_5693_ = _e4807;
                                                    phi_5695_ = _e4809;
                                                    phi_5697_ = _e4811;
                                                    phi_5699_ = _e4813;
                                                    phi_5701_ = _e4815;
                                                    phi_5703_ = _e4817;
                                                    if !(_e4150) {
                                                        break;
                                                    }
                                                }
                                                if _e4114 {
                                                    let _e4832 = (exp((-(abs(_e3841)) / (0.06f + (abs(_e3852) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3852 * -0.71428573f) * (_e3852 * 0.71428573f))))));
                                                    let _e4834 = (_e3841 * 1.3333334f);
                                                    let _e4839 = (1.3f * exp((((_e3852 * _e3852) + (_e4834 * _e4834)) * -12.499999f)));
                                                    let _e4850 = local_26;
                                                    let _e4852 = local_27;
                                                    if (_e4850 != _e4852) {
                                                        phi_25076_ = true;
                                                    } else {
                                                        let _e4855 = local_28;
                                                        phi_25076_ = (0.1f >= _e4855);
                                                    }
                                                    let _e4858 = phi_25076_;
                                                    let _e4860 = local_29;
                                                    let _e4863 = (pow(select(_e4860, 0.1f, _e4858), 1.3f) * _e4072);
                                                    phi_6464_ = vec3<f32>((_e4069.x + (((_e4078 * _e4832) + (_e4087 * _e4839)) * _e4863)), (_e4069.y + (((_e4081 * _e4832) + (_e4090 * _e4839)) * _e4863)), (_e4069.z + (((_e4084 * _e4832) + (_e4093 * _e4839)) * _e4863)));
                                                } else {
                                                    phi_6464_ = _e4069;
                                                }
                                                let _e4875 = phi_6464_;
                                                if select(_e4124, _e4128, _e4130) {
                                                    let _e4877 = local_30;
                                                    let _e4883 = ((_e3841 - (0.025f * _e4877)) / (0.045f + (abs(_e3852) * 0.0095492955f)));
                                                    let _e4888 = local_31;
                                                    let _e4889 = (_e4888 * 3.3333333f);
                                                    let _e4891 = select(_e4889, 0f, (_e4889 < 0f));
                                                    let _e4893 = select(_e4891, 1f, (_e4891 > 1f));
                                                    let _e4909 = local_32;
                                                    let _e4911 = ((_e4909 - 0.25f) * 5.0000005f);
                                                    let _e4913 = select(_e4911, 0f, (_e4911 < 0f));
                                                    let _e4915 = select(_e4913, 1f, (_e4913 > 1f));
                                                    phi_6543_ = (_e4075 * (((exp((-(_e4883) * _e4883)) * (0.15f + (3f * ((_e4893 * _e4893) * (3f - (2f * _e4893)))))) * (0.5f + (0.9f * exp(((_e3852 * -0.6666667f) * (_e3852 * 0.6666667f)))))) + (1.2f * (exp(((_e3841 * -3.3333333f) * (_e3841 * 3.3333333f))) * ((_e4915 * _e4915) * (3f - (2f * _e4915)))))));
                                                } else {
                                                    phi_6543_ = 0f;
                                                }
                                                let _e4929 = phi_6543_;
                                                phi_6544_ = _e4875;
                                                phi_6545_ = _e4929;
                                            } else {
                                                phi_6544_ = _e4069;
                                                phi_6545_ = 0f;
                                            }
                                            let _e4931 = phi_6544_;
                                            let _e4933 = phi_6545_;
                                            phi_6547_ = _e4931;
                                            phi_6550_ = 0u;
                                            loop {
                                                let _e4935 = phi_6547_;
                                                let _e4937 = phi_6550_;
                                                local_49 = _e4935;
                                                local_50 = _e4935;
                                                local_51 = _e4935;
                                                let _e4938 = (_e4937 < bitcast<u32>(_e3804));
                                                if _e4938 {
                                                    let _e4940 = (bitcast<u32>(_e3800) + (_e4937 * 29u));
                                                    let _e4942 = global_6.member[_e4940];
                                                    let _e4943 = (_e4942 > 0.5f);
                                                    let _e4946 = global_6.member[(_e4940 + 1u)];
                                                    let _e4949 = global_6.member[(_e4940 + 2u)];
                                                    let _e4952 = global_6.member[(_e4940 + 3u)];
                                                    let _e4955 = global_6.member[(_e4940 + 4u)];
                                                    let _e4958 = global_6.member[(_e4940 + 5u)];
                                                    let _e4961 = global_6.member[(_e4940 + 6u)];
                                                    let _e4966 = (((_e334.member_1.x * _e4946) + (_e334.member_1.y * _e4949)) + (_e334.member_1.z * _e4952));
                                                    let _e4967 = (_e4966 < _e4958);
                                                    if _e4967 {
                                                        phi_7696_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7697_ = vec3<f32>();
                                                    } else {
                                                        let _e4970 = global_6.member[(_e4940 + 7u)];
                                                        let _e4973 = global_6.member[(_e4940 + 8u)];
                                                        let _e4976 = global_6.member[(_e4940 + 9u)];
                                                        let _e4979 = global_6.member[(_e4940 + 10u)];
                                                        let _e4982 = global_6.member[(_e4940 + 11u)];
                                                        let _e4985 = global_6.member[(_e4940 + 12u)];
                                                        let _e4988 = global_6.member[(_e4940 + 13u)];
                                                        let _e4991 = global_6.member[(_e4940 + 14u)];
                                                        let _e4994 = global_6.member[(_e4940 + 15u)];
                                                        let _e4997 = global_6.member[(_e4940 + 16u)];
                                                        let _e5000 = global_6.member[(_e4940 + 17u)];
                                                        let _e5003 = global_6.member[(_e4940 + 18u)];
                                                        let _e5006 = global_6.member[(_e4940 + 19u)];
                                                        let _e5009 = global_6.member[(_e4940 + 20u)];
                                                        let _e5012 = global_6.member[(_e4940 + 21u)];
                                                        let _e5015 = global_6.member[(_e4940 + 22u)];
                                                        let _e5018 = global_6.member[(_e4940 + 23u)];
                                                        let _e5021 = global_6.member[(_e4940 + 24u)];
                                                        let _e5024 = global_6.member[(_e4940 + 25u)];
                                                        let _e5027 = global_6.member[(_e4940 + 26u)];
                                                        let _e5030 = global_6.member[(_e4940 + 27u)];
                                                        let _e5033 = global_6.member[(_e4940 + 28u)];
                                                        let _e5040 = (((_e334.member_1.x / _e4966) - _e4946) / _e4955);
                                                        let _e5041 = (((_e334.member_1.y / _e4966) - _e4949) / _e4955);
                                                        let _e5042 = (((_e334.member_1.z / _e4966) - _e4952) / _e4955);
                                                        if _e4943 {
                                                            let _e5647 = (((_e5040 * _e4997) + (_e5041 * _e5000)) + (_e5042 * _e5003));
                                                            let _e5653 = ((((_e5040 * _e5006) + (_e5041 * _e5009)) + (_e5042 * _e5012)) / _e5015);
                                                            phi_7694_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4961 * exp((-2.5f * pow((sqrt(((_e5647 * _e5647) + (_e5653 * _e5653))) * 3.3333333f), 0.7f)))));
                                                            phi_7695_ = vec3<f32>();
                                                        } else {
                                                            let _e5047 = (((_e4970 * _e4946) + (_e4973 * _e4949)) + (_e4976 * _e4952));
                                                            if (abs(_e5047) < 0.12f) {
                                                                phi_6998_ = select(0.12f, -0.12f, (_e5047 < 0f));
                                                            } else {
                                                                phi_6998_ = _e5047;
                                                            }
                                                            let _e5053 = phi_6998_;
                                                            let _e5059 = ((((_e4970 * _e5040) + (_e4973 * _e5041)) + (_e4976 * _e5042)) / _e5053);
                                                            let _e5063 = (_e5040 - (_e4946 * _e5059));
                                                            let _e5064 = (_e5041 - (_e4949 * _e5059));
                                                            let _e5065 = (_e5042 - (_e4952 * _e5059));
                                                            let _e5070 = (((_e5063 * _e4979) + (_e5064 * _e4982)) + (_e5065 * _e4985));
                                                            let _e5075 = (((_e5063 * _e4988) + (_e5064 * _e4991)) + (_e5065 * _e4994));
                                                            let _e5079 = sqrt(((_e5070 * _e5070) + (_e5075 * _e5075)));
                                                            let _e5091 = abs(_e5053);
                                                            let _e5093 = (pow((0.5f + (0.5f * cos(((_e5018 * (atan2(_e5075, _e5070) - (log((_e5079 + 0.03f)) / _e5021))) + _e5024)))), 3f) * sqrt(_e5091));
                                                            let _e5098 = global_6.member[_e3781];
                                                            let _e5099 = bitcast<u32>(_e5098);
                                                            let _e5102 = global_6.member[(_e1292 + 11u)];
                                                            let _e5105 = global_6.member[(_e1292 + 12u)];
                                                            let _e5108 = global_6.member[(_e1292 + 13u)];
                                                            let _e5111 = global_6.member[(_e1292 + 14u)];
                                                            let _e5114 = global_6.member[(_e1292 + 15u)];
                                                            let _e5117 = global_6.member[(_e1292 + 16u)];
                                                            let _e5120 = global_6.member[(_e1292 + 17u)];
                                                            let _e5123 = global_6.member[(_e1292 + 18u)];
                                                            let _e5126 = global_6.member[(_e1292 + 19u)];
                                                            phi_7178_ = 0u;
                                                            phi_7181_ = 0f;
                                                            phi_7183_ = 1f;
                                                            phi_7185_ = 0f;
                                                            phi_7187_ = vec3<f32>((_e5070 * 9f), (_e5075 * 9f), 0f);
                                                            loop {
                                                                let _e5128 = phi_7178_;
                                                                let _e5130 = phi_7181_;
                                                                let _e5132 = phi_7183_;
                                                                let _e5134 = phi_7185_;
                                                                let _e5136 = phi_7187_;
                                                                local_33 = _e5130;
                                                                local_34 = _e5134;
                                                                local_35 = _e5130;
                                                                let _e5137 = (_e5128 < 4u);
                                                                if _e5137 {
                                                                    let _e5139 = (_e5136.x + _e5027);
                                                                    let _e5141 = (_e5136.y + _e5030);
                                                                    let _e5143 = (_e5136.z + _e5033);
                                                                    let _e5144 = floor(_e5139);
                                                                    let _e5145 = floor(_e5141);
                                                                    let _e5146 = floor(_e5143);
                                                                    let _e5155 = bitcast<u32>((select(0i, select(select(i32(_e5144), i32(-2147483648), (_e5144 < -2147483600f)), 2147483647i, (_e5144 > 2147483500f)), (_e5144 == _e5144)) & 255i));
                                                                    let _e5164 = bitcast<u32>((select(0i, select(select(i32(_e5145), i32(-2147483648), (_e5145 < -2147483600f)), 2147483647i, (_e5145 > 2147483500f)), (_e5145 == _e5145)) & 255i));
                                                                    let _e5173 = bitcast<u32>((select(0i, select(select(i32(_e5146), i32(-2147483648), (_e5146 < -2147483600f)), 2147483647i, (_e5146 > 2147483500f)), (_e5146 == _e5146)) & 255i));
                                                                    let _e5174 = (_e5139 - _e5144);
                                                                    let _e5175 = (_e5141 - _e5145);
                                                                    let _e5176 = (_e5143 - _e5146);
                                                                    let _e5183 = (((_e5174 * _e5174) * _e5174) * ((_e5174 * ((_e5174 * 6f) - 15f)) + 10f));
                                                                    let _e5190 = (((_e5175 * _e5175) * _e5175) * ((_e5175 * ((_e5175 * 6f) - 15f)) + 10f));
                                                                    let _e5200 = global_6.member[(_e5099 + _e5155)];
                                                                    let _e5206 = (select(select(u32(_e5200), 0u, (_e5200 < 0f)), 4294967295u, (_e5200 > 4294967000f)) + _e5164);
                                                                    let _e5209 = global_6.member[(_e5099 + _e5206)];
                                                                    let _e5215 = (select(select(u32(_e5209), 0u, (_e5209 < 0f)), 4294967295u, (_e5209 > 4294967000f)) + _e5173);
                                                                    let _e5219 = global_6.member[(_e5099 + (_e5206 + 1u))];
                                                                    let _e5225 = (select(select(u32(_e5219), 0u, (_e5219 < 0f)), 4294967295u, (_e5219 > 4294967000f)) + _e5173);
                                                                    let _e5229 = global_6.member[(_e5099 + (_e5155 + 1u))];
                                                                    let _e5235 = (select(select(u32(_e5229), 0u, (_e5229 < 0f)), 4294967295u, (_e5229 > 4294967000f)) + _e5164);
                                                                    let _e5238 = global_6.member[(_e5099 + _e5235)];
                                                                    let _e5244 = (select(select(u32(_e5238), 0u, (_e5238 < 0f)), 4294967295u, (_e5238 > 4294967000f)) + _e5173);
                                                                    let _e5248 = global_6.member[(_e5099 + (_e5235 + 1u))];
                                                                    let _e5254 = (select(select(u32(_e5248), 0u, (_e5248 < 0f)), 4294967295u, (_e5248 > 4294967000f)) + _e5173);
                                                                    let _e5257 = global_6.member[(_e5099 + _e5215)];
                                                                    let _e5263 = (select(select(u32(_e5257), 0u, (_e5257 < 0f)), 4294967295u, (_e5257 > 4294967000f)) % 12u);
                                                                    let _e5266 = select(-1f, 1f, ((_e5263 & 1u) == 0u));
                                                                    let _e5269 = select(-1f, 1f, ((_e5263 & 2u) == 0u));
                                                                    if (_e5263 < 4u) {
                                                                        phi_25171_ = ((_e5266 * _e5174) + (_e5269 * _e5175));
                                                                    } else {
                                                                        if (_e5263 < 8u) {
                                                                            phi_25165_ = ((_e5266 * _e5174) + (_e5269 * _e5176));
                                                                        } else {
                                                                            phi_25165_ = ((_e5266 * _e5175) + (_e5269 * _e5176));
                                                                        }
                                                                        let _e5279 = phi_25165_;
                                                                        phi_25171_ = _e5279;
                                                                    }
                                                                    let _e5284 = phi_25171_;
                                                                    let _e5287 = global_6.member[(_e5099 + _e5244)];
                                                                    let _e5293 = (_e5174 - 1f);
                                                                    let _e5294 = (select(select(u32(_e5287), 0u, (_e5287 < 0f)), 4294967295u, (_e5287 > 4294967000f)) % 12u);
                                                                    let _e5297 = select(-1f, 1f, ((_e5294 & 1u) == 0u));
                                                                    let _e5300 = select(-1f, 1f, ((_e5294 & 2u) == 0u));
                                                                    if (_e5294 < 4u) {
                                                                        phi_25205_ = ((_e5297 * _e5293) + (_e5300 * _e5175));
                                                                    } else {
                                                                        if (_e5294 < 8u) {
                                                                            phi_25199_ = ((_e5297 * _e5293) + (_e5300 * _e5176));
                                                                        } else {
                                                                            phi_25199_ = ((_e5297 * _e5175) + (_e5300 * _e5176));
                                                                        }
                                                                        let _e5310 = phi_25199_;
                                                                        phi_25205_ = _e5310;
                                                                    }
                                                                    let _e5315 = phi_25205_;
                                                                    let _e5318 = global_6.member[(_e5099 + _e5225)];
                                                                    let _e5324 = (_e5175 - 1f);
                                                                    let _e5325 = (select(select(u32(_e5318), 0u, (_e5318 < 0f)), 4294967295u, (_e5318 > 4294967000f)) % 12u);
                                                                    let _e5328 = select(-1f, 1f, ((_e5325 & 1u) == 0u));
                                                                    let _e5331 = select(-1f, 1f, ((_e5325 & 2u) == 0u));
                                                                    if (_e5325 < 4u) {
                                                                        phi_25239_ = ((_e5328 * _e5174) + (_e5331 * _e5324));
                                                                    } else {
                                                                        if (_e5325 < 8u) {
                                                                            phi_25233_ = ((_e5328 * _e5174) + (_e5331 * _e5176));
                                                                        } else {
                                                                            phi_25233_ = ((_e5328 * _e5324) + (_e5331 * _e5176));
                                                                        }
                                                                        let _e5341 = phi_25233_;
                                                                        phi_25239_ = _e5341;
                                                                    }
                                                                    let _e5346 = phi_25239_;
                                                                    let _e5349 = global_6.member[(_e5099 + _e5254)];
                                                                    let _e5355 = (select(select(u32(_e5349), 0u, (_e5349 < 0f)), 4294967295u, (_e5349 > 4294967000f)) % 12u);
                                                                    let _e5358 = select(-1f, 1f, ((_e5355 & 1u) == 0u));
                                                                    let _e5361 = select(-1f, 1f, ((_e5355 & 2u) == 0u));
                                                                    if (_e5355 < 4u) {
                                                                        phi_25273_ = ((_e5358 * _e5293) + (_e5361 * _e5324));
                                                                    } else {
                                                                        if (_e5355 < 8u) {
                                                                            phi_25267_ = ((_e5358 * _e5293) + (_e5361 * _e5176));
                                                                        } else {
                                                                            phi_25267_ = ((_e5358 * _e5324) + (_e5361 * _e5176));
                                                                        }
                                                                        let _e5371 = phi_25267_;
                                                                        phi_25273_ = _e5371;
                                                                    }
                                                                    let _e5376 = phi_25273_;
                                                                    let _e5380 = global_6.member[(_e5099 + (_e5215 + 1u))];
                                                                    let _e5386 = (_e5176 - 1f);
                                                                    let _e5387 = (select(select(u32(_e5380), 0u, (_e5380 < 0f)), 4294967295u, (_e5380 > 4294967000f)) % 12u);
                                                                    let _e5390 = select(-1f, 1f, ((_e5387 & 1u) == 0u));
                                                                    let _e5393 = select(-1f, 1f, ((_e5387 & 2u) == 0u));
                                                                    if (_e5387 < 4u) {
                                                                        phi_25307_ = ((_e5390 * _e5174) + (_e5393 * _e5175));
                                                                    } else {
                                                                        if (_e5387 < 8u) {
                                                                            phi_25301_ = ((_e5390 * _e5174) + (_e5393 * _e5386));
                                                                        } else {
                                                                            phi_25301_ = ((_e5390 * _e5175) + (_e5393 * _e5386));
                                                                        }
                                                                        let _e5403 = phi_25301_;
                                                                        phi_25307_ = _e5403;
                                                                    }
                                                                    let _e5408 = phi_25307_;
                                                                    let _e5412 = global_6.member[(_e5099 + (_e5244 + 1u))];
                                                                    let _e5418 = (select(select(u32(_e5412), 0u, (_e5412 < 0f)), 4294967295u, (_e5412 > 4294967000f)) % 12u);
                                                                    let _e5421 = select(-1f, 1f, ((_e5418 & 1u) == 0u));
                                                                    let _e5424 = select(-1f, 1f, ((_e5418 & 2u) == 0u));
                                                                    if (_e5418 < 4u) {
                                                                        phi_25341_ = ((_e5421 * _e5293) + (_e5424 * _e5175));
                                                                    } else {
                                                                        if (_e5418 < 8u) {
                                                                            phi_25335_ = ((_e5421 * _e5293) + (_e5424 * _e5386));
                                                                        } else {
                                                                            phi_25335_ = ((_e5421 * _e5175) + (_e5424 * _e5386));
                                                                        }
                                                                        let _e5434 = phi_25335_;
                                                                        phi_25341_ = _e5434;
                                                                    }
                                                                    let _e5439 = phi_25341_;
                                                                    let _e5443 = global_6.member[(_e5099 + (_e5225 + 1u))];
                                                                    let _e5449 = (select(select(u32(_e5443), 0u, (_e5443 < 0f)), 4294967295u, (_e5443 > 4294967000f)) % 12u);
                                                                    let _e5452 = select(-1f, 1f, ((_e5449 & 1u) == 0u));
                                                                    let _e5455 = select(-1f, 1f, ((_e5449 & 2u) == 0u));
                                                                    if (_e5449 < 4u) {
                                                                        phi_25375_ = ((_e5452 * _e5174) + (_e5455 * _e5324));
                                                                    } else {
                                                                        if (_e5449 < 8u) {
                                                                            phi_25369_ = ((_e5452 * _e5174) + (_e5455 * _e5386));
                                                                        } else {
                                                                            phi_25369_ = ((_e5452 * _e5324) + (_e5455 * _e5386));
                                                                        }
                                                                        let _e5465 = phi_25369_;
                                                                        phi_25375_ = _e5465;
                                                                    }
                                                                    let _e5470 = phi_25375_;
                                                                    let _e5474 = global_6.member[(_e5099 + (_e5254 + 1u))];
                                                                    let _e5480 = (select(select(u32(_e5474), 0u, (_e5474 < 0f)), 4294967295u, (_e5474 > 4294967000f)) % 12u);
                                                                    let _e5483 = select(-1f, 1f, ((_e5480 & 1u) == 0u));
                                                                    let _e5486 = select(-1f, 1f, ((_e5480 & 2u) == 0u));
                                                                    if (_e5480 < 4u) {
                                                                        phi_25409_ = ((_e5483 * _e5293) + (_e5486 * _e5324));
                                                                    } else {
                                                                        if (_e5480 < 8u) {
                                                                            phi_25403_ = ((_e5483 * _e5293) + (_e5486 * _e5386));
                                                                        } else {
                                                                            phi_25403_ = ((_e5483 * _e5324) + (_e5486 * _e5386));
                                                                        }
                                                                        let _e5496 = phi_25403_;
                                                                        phi_25409_ = _e5496;
                                                                    }
                                                                    let _e5501 = phi_25409_;
                                                                    let _e5504 = (_e5284 + (_e5183 * (_e5315 - _e5284)));
                                                                    let _e5510 = (_e5408 + (_e5183 * (_e5439 - _e5408)));
                                                                    let _e5516 = (_e5504 + (_e5190 * ((_e5346 + (_e5183 * (_e5376 - _e5346))) - _e5504)));
                                                                    phi_7179_ = (_e5128 + 1u);
                                                                    phi_7182_ = (_e5130 + _e5132);
                                                                    phi_7184_ = (_e5132 * 0.5f);
                                                                    phi_7186_ = (_e5134 + ((_e5516 + ((((_e5176 * _e5176) * _e5176) * ((_e5176 * ((_e5176 * 6f) - 15f)) + 10f)) * ((_e5510 + (_e5190 * ((_e5470 + (_e5183 * (_e5501 - _e5470))) - _e5510))) - _e5516))) * _e5132));
                                                                    phi_7188_ = vec3<f32>(((((_e5102 * _e5136.x) + (_e5105 * _e5136.y)) + (_e5108 * _e5136.z)) * 2.03f), ((((_e5111 * _e5136.x) + (_e5114 * _e5136.y)) + (_e5117 * _e5136.z)) * 2.03f), ((((_e5120 * _e5136.x) + (_e5123 * _e5136.y)) + (_e5126 * _e5136.z)) * 2.03f));
                                                                } else {
                                                                    phi_7179_ = u32();
                                                                    phi_7182_ = f32();
                                                                    phi_7184_ = f32();
                                                                    phi_7186_ = f32();
                                                                    phi_7188_ = vec3<f32>();
                                                                }
                                                                let _e5548 = phi_7179_;
                                                                let _e5550 = phi_7182_;
                                                                let _e5552 = phi_7184_;
                                                                let _e5554 = phi_7186_;
                                                                let _e5556 = phi_7188_;
                                                                phi_7178_ = _e5548;
                                                                phi_7181_ = _e5550;
                                                                phi_7183_ = _e5552;
                                                                phi_7185_ = _e5554;
                                                                phi_7187_ = _e5556;
                                                                if !(_e5137) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5559 = local_33;
                                                            if (_e5559 > 0f) {
                                                                let _e5562 = local_34;
                                                                let _e5564 = local_35;
                                                                phi_7601_ = (_e5562 / _e5564);
                                                            } else {
                                                                phi_7601_ = 0f;
                                                            }
                                                            let _e5567 = phi_7601_;
                                                            let _e5569 = (1f + (1.2f * _e5567));
                                                            if (_e5569 != _e5569) {
                                                                phi_25455_ = true;
                                                            } else {
                                                                phi_25455_ = (0f >= _e5569);
                                                            }
                                                            let _e5573 = phi_25455_;
                                                            let _e5578 = ((_e5079 - 1.15f) * -2.8571432f);
                                                            let _e5580 = select(_e5578, 0f, (_e5578 < 0f));
                                                            let _e5582 = select(_e5580, 1f, (_e5580 > 1f));
                                                            let _e5588 = (1f / _e5091);
                                                            if (_e5588 != _e5588) {
                                                                phi_25470_ = true;
                                                            } else {
                                                                phi_25470_ = (4f <= _e5588);
                                                            }
                                                            let _e5592 = phi_25470_;
                                                            let _e5595 = (((exp((_e5079 * -3.5714285f)) * ((_e5582 * _e5582) * (3f - (2f * _e5582)))) * select(_e5588, 4f, _e5592)) * 0.12f);
                                                            let _e5601 = sqrt((((_e5040 * _e5040) + (_e5041 * _e5041)) + (_e5042 * _e5042)));
                                                            let _e5608 = (((_e5040 * _e5006) + (_e5041 * _e5009)) + (_e5042 * _e5012));
                                                            let _e5609 = (1f - _e5091);
                                                            let _e5625 = ((_e5595 * (0.6f + (0.4f * (1f - _e5093)))) + exp((_e5601 * -16.666668f)));
                                                            let _e5631 = (((_e5595 * _e5093) * 1.8f) * select(_e5569, 0f, _e5573));
                                                            let _e5638 = (_e4961 * (1f - ((((0.75f * _e5609) * _e5609) * exp(((_e5608 * -28.571428f) * (_e5608 * 28.571428f)))) * exp((_e5601 * -1.4285715f)))));
                                                            phi_7694_ = vec3<f32>();
                                                            phi_7695_ = vec3<f32>((((1.1633643f * _e5625) + (0.86689025f * _e5631)) * _e5638), (((0.97722596f * _e5625) + (1.0155001f * _e5631)) * _e5638), (((0.74455315f * _e5625) + (1.2384146f * _e5631)) * _e5638));
                                                        }
                                                        let _e5665 = phi_7694_;
                                                        let _e5667 = phi_7695_;
                                                        phi_7696_ = _e5665;
                                                        phi_7697_ = _e5667;
                                                    }
                                                    let _e5669 = phi_7696_;
                                                    let _e5671 = phi_7697_;
                                                    phi_6548_ = (_e4935 + select(_e5671, _e5669, vec3(select(_e4943, true, _e4967))));
                                                    phi_6551_ = (_e4937 + 1u);
                                                } else {
                                                    phi_6548_ = vec3<f32>();
                                                    phi_6551_ = u32();
                                                }
                                                let _e5678 = phi_6548_;
                                                let _e5680 = phi_6551_;
                                                phi_6547_ = _e5678;
                                                phi_6550_ = _e5680;
                                                if !(_e4938) {
                                                    break;
                                                }
                                            }
                                            phi_33861_ = _e4067;
                                            if _e4067 {
                                                break;
                                            }
                                            phi_7716_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7719_ = 0u;
                                            phi_7721_ = _e4933;
                                            loop {
                                                let _e5683 = phi_7716_;
                                                let _e5685 = phi_7719_;
                                                let _e5687 = phi_7721_;
                                                local_45 = _e5687;
                                                local_46 = _e5687;
                                                local_47 = _e5687;
                                                local_48 = _e5687;
                                                local_52 = _e5683;
                                                local_53 = _e5683;
                                                local_54 = _e5683;
                                                let _e5688 = (_e5685 < bitcast<u32>(_e3796));
                                                if _e5688 {
                                                    let _e5690 = (bitcast<u32>(_e3792) + (_e5685 * 18u));
                                                    let _e5692 = global_6.member[_e5690];
                                                    let _e5697 = select(select(u32(_e5692), 0u, (_e5692 < 0f)), 4294967295u, (_e5692 > 4294967000f));
                                                    let _e5700 = global_6.member[(_e5690 + 1u)];
                                                    let _e5703 = global_6.member[(_e5690 + 2u)];
                                                    let _e5706 = global_6.member[(_e5690 + 3u)];
                                                    let _e5709 = global_6.member[(_e5690 + 4u)];
                                                    let _e5712 = global_6.member[(_e5690 + 5u)];
                                                    let _e5715 = global_6.member[(_e5690 + 6u)];
                                                    let _e5718 = global_6.member[(_e5690 + 7u)];
                                                    let _e5721 = global_6.member[(_e5690 + 8u)];
                                                    let _e5724 = global_6.member[(_e5690 + 9u)];
                                                    let _e5727 = global_6.member[(_e5690 + 10u)];
                                                    let _e5730 = global_6.member[(_e5690 + 11u)];
                                                    let _e5733 = global_6.member[(_e5690 + 12u)];
                                                    let _e5736 = global_6.member[(_e5690 + 13u)];
                                                    let _e5739 = global_6.member[(_e5690 + 14u)];
                                                    let _e5742 = global_6.member[(_e5690 + 15u)];
                                                    let _e5745 = global_6.member[(_e5690 + 16u)];
                                                    let _e5748 = global_6.member[(_e5690 + 17u)];
                                                    let _e5753 = (((_e334.member_1.x * _e5700) + (_e334.member_1.y * _e5703)) + (_e334.member_1.z * _e5706));
                                                    if (_e5753 < _e5712) {
                                                        phi_9038_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5758 = ((_e334.member_1.x / _e5753) - _e5700);
                                                        let _e5759 = ((_e334.member_1.y / _e5753) - _e5703);
                                                        let _e5760 = ((_e334.member_1.z / _e5753) - _e5706);
                                                        let _e5767 = ((((_e5758 * _e5721) + (_e5759 * _e5724)) + (_e5760 * _e5727)) / (_e5709 * _e5739));
                                                        let _e5773 = ((((_e5758 * _e5730) + (_e5759 * _e5733)) + (_e5760 * _e5736)) / _e5709);
                                                        let _e5777 = sqrt(((_e5767 * _e5767) + (_e5773 * _e5773)));
                                                        let _e5780 = vec3<f32>((_e5767 * 2.2f), (_e5773 * 2.2f), 0f);
                                                        let _e5781 = (_e5697 == 0u);
                                                        if _e5781 {
                                                            phi_8023_ = 6u;
                                                            phi_8024_ = 1.2f;
                                                            phi_8025_ = _e5780;
                                                        } else {
                                                            let _e5782 = (_e5697 == 1u);
                                                            if _e5782 {
                                                                phi_8022_ = vec3<f32>((_e5767 * 1.54f), (_e5773 * 1.54f), 0f);
                                                            } else {
                                                                phi_8022_ = _e5780;
                                                            }
                                                            let _e5787 = phi_8022_;
                                                            phi_8023_ = select(6u, 5u, _e5782);
                                                            phi_8024_ = select(1f, 0.8f, _e5782);
                                                            phi_8025_ = _e5787;
                                                        }
                                                        let _e5791 = phi_8023_;
                                                        let _e5793 = phi_8024_;
                                                        let _e5795 = phi_8025_;
                                                        if _e5781 {
                                                            phi_8042_ = vec3<f32>(((_e5767 * 3.7400002f) + -7f), ((_e5773 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8042_ = vec3<f32>((_e5767 * 3.3000002f), (_e5773 * 3.3000002f), 0f);
                                                        }
                                                        let _e5805 = phi_8042_;
                                                        phi_8044_ = 0u;
                                                        phi_8047_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8049_ = 0f;
                                                        phi_8051_ = 0f;
                                                        phi_8053_ = 0f;
                                                        loop {
                                                            let _e5808 = phi_8044_;
                                                            let _e5810 = phi_8047_;
                                                            let _e5812 = phi_8049_;
                                                            let _e5814 = phi_8051_;
                                                            let _e5816 = phi_8053_;
                                                            local_39 = _e5816;
                                                            local_40 = _e5814;
                                                            local_41 = _e5812;
                                                            local_42 = _e5814;
                                                            local_43 = _e5814;
                                                            local_44 = _e5812;
                                                            let _e5817 = (_e5808 < 6u);
                                                            if _e5817 {
                                                                let _e5818 = (_e5808 == 0u);
                                                                if _e5818 {
                                                                    phi_8095_ = true;
                                                                    phi_8096_ = false;
                                                                    phi_8097_ = false;
                                                                } else {
                                                                    if (_e5808 <= 4u) {
                                                                        phi_8073_ = (_e5697 == 2u);
                                                                    } else {
                                                                        phi_8073_ = true;
                                                                    }
                                                                    let _e5822 = phi_8073_;
                                                                    if _e5822 {
                                                                        let _e5823 = (_e5808 == 5u);
                                                                        if _e5823 {
                                                                            if _e5781 {
                                                                                phi_8089_ = false;
                                                                            } else {
                                                                                phi_8089_ = (_e5697 == 2u);
                                                                            }
                                                                            let _e5826 = phi_8089_;
                                                                            phi_8091_ = _e5826;
                                                                        } else {
                                                                            phi_8091_ = false;
                                                                        }
                                                                        let _e5828 = phi_8091_;
                                                                        phi_8093_ = select(false, _e5781, _e5823);
                                                                        phi_8094_ = _e5828;
                                                                    } else {
                                                                        phi_8093_ = false;
                                                                        phi_8094_ = false;
                                                                    }
                                                                    let _e5831 = phi_8093_;
                                                                    let _e5833 = phi_8094_;
                                                                    phi_8095_ = select(true, false, _e5822);
                                                                    phi_8096_ = _e5831;
                                                                    phi_8097_ = _e5833;
                                                                }
                                                                let _e5836 = phi_8095_;
                                                                let _e5838 = phi_8096_;
                                                                let _e5840 = phi_8097_;
                                                                if select(select(_e5840, false, _e5836), true, select(_e5838, true, _e5836)) {
                                                                    if _e5818 {
                                                                        phi_8144_ = 3u;
                                                                        phi_8145_ = vec3<f32>(((_e5767 * 1.7600001f) + 3f), ((_e5773 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5808 < 4u) {
                                                                            let _e5857 = (_e5808 - 1u);
                                                                            if (_e5857 == 0u) {
                                                                                phi_25489_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_25489_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5857 == 1u)));
                                                                            }
                                                                            let _e5863 = phi_25489_;
                                                                            phi_8142_ = 4u;
                                                                            phi_8143_ = (_e5795 + _e5863);
                                                                        } else {
                                                                            let _e5845 = (_e5808 == 4u);
                                                                            if _e5845 {
                                                                                phi_8141_ = (_e5795 + vec3<f32>((_e5810.x * _e5793), (_e5810.y * _e5793), (_e5810.z * _e5793)));
                                                                            } else {
                                                                                phi_8141_ = _e5805;
                                                                            }
                                                                            let _e5855 = phi_8141_;
                                                                            phi_8142_ = select(select(4u, 5u, _e5781), _e5791, _e5845);
                                                                            phi_8143_ = _e5855;
                                                                        }
                                                                        let _e5866 = phi_8142_;
                                                                        let _e5868 = phi_8143_;
                                                                        phi_8144_ = _e5866;
                                                                        phi_8145_ = _e5868;
                                                                    }
                                                                    let _e5875 = phi_8144_;
                                                                    let _e5877 = phi_8145_;
                                                                    let _e5879 = global_6.member[_e3781];
                                                                    let _e5880 = bitcast<u32>(_e5879);
                                                                    let _e5883 = global_6.member[(_e1292 + 11u)];
                                                                    let _e5886 = global_6.member[(_e1292 + 12u)];
                                                                    let _e5889 = global_6.member[(_e1292 + 13u)];
                                                                    let _e5892 = global_6.member[(_e1292 + 14u)];
                                                                    let _e5895 = global_6.member[(_e1292 + 15u)];
                                                                    let _e5898 = global_6.member[(_e1292 + 16u)];
                                                                    let _e5901 = global_6.member[(_e1292 + 17u)];
                                                                    let _e5904 = global_6.member[(_e1292 + 18u)];
                                                                    let _e5907 = global_6.member[(_e1292 + 19u)];
                                                                    phi_8280_ = 0u;
                                                                    phi_8283_ = 0f;
                                                                    phi_8285_ = 1f;
                                                                    phi_8287_ = 0f;
                                                                    phi_8289_ = _e5877;
                                                                    loop {
                                                                        let _e5909 = phi_8280_;
                                                                        let _e5911 = phi_8283_;
                                                                        let _e5913 = phi_8285_;
                                                                        let _e5915 = phi_8287_;
                                                                        let _e5917 = phi_8289_;
                                                                        local_36 = _e5911;
                                                                        local_37 = _e5915;
                                                                        local_38 = _e5911;
                                                                        let _e5918 = (_e5909 < _e5875);
                                                                        if _e5918 {
                                                                            let _e5920 = (_e5917.x + _e5742);
                                                                            let _e5922 = (_e5917.y + _e5745);
                                                                            let _e5924 = (_e5917.z + _e5748);
                                                                            let _e5925 = floor(_e5920);
                                                                            let _e5926 = floor(_e5922);
                                                                            let _e5927 = floor(_e5924);
                                                                            let _e5936 = bitcast<u32>((select(0i, select(select(i32(_e5925), i32(-2147483648), (_e5925 < -2147483600f)), 2147483647i, (_e5925 > 2147483500f)), (_e5925 == _e5925)) & 255i));
                                                                            let _e5945 = bitcast<u32>((select(0i, select(select(i32(_e5926), i32(-2147483648), (_e5926 < -2147483600f)), 2147483647i, (_e5926 > 2147483500f)), (_e5926 == _e5926)) & 255i));
                                                                            let _e5954 = bitcast<u32>((select(0i, select(select(i32(_e5927), i32(-2147483648), (_e5927 < -2147483600f)), 2147483647i, (_e5927 > 2147483500f)), (_e5927 == _e5927)) & 255i));
                                                                            let _e5955 = (_e5920 - _e5925);
                                                                            let _e5956 = (_e5922 - _e5926);
                                                                            let _e5957 = (_e5924 - _e5927);
                                                                            let _e5964 = (((_e5955 * _e5955) * _e5955) * ((_e5955 * ((_e5955 * 6f) - 15f)) + 10f));
                                                                            let _e5971 = (((_e5956 * _e5956) * _e5956) * ((_e5956 * ((_e5956 * 6f) - 15f)) + 10f));
                                                                            let _e5981 = global_6.member[(_e5880 + _e5936)];
                                                                            let _e5987 = (select(select(u32(_e5981), 0u, (_e5981 < 0f)), 4294967295u, (_e5981 > 4294967000f)) + _e5945);
                                                                            let _e5990 = global_6.member[(_e5880 + _e5987)];
                                                                            let _e5996 = (select(select(u32(_e5990), 0u, (_e5990 < 0f)), 4294967295u, (_e5990 > 4294967000f)) + _e5954);
                                                                            let _e6000 = global_6.member[(_e5880 + (_e5987 + 1u))];
                                                                            let _e6006 = (select(select(u32(_e6000), 0u, (_e6000 < 0f)), 4294967295u, (_e6000 > 4294967000f)) + _e5954);
                                                                            let _e6010 = global_6.member[(_e5880 + (_e5936 + 1u))];
                                                                            let _e6016 = (select(select(u32(_e6010), 0u, (_e6010 < 0f)), 4294967295u, (_e6010 > 4294967000f)) + _e5945);
                                                                            let _e6019 = global_6.member[(_e5880 + _e6016)];
                                                                            let _e6025 = (select(select(u32(_e6019), 0u, (_e6019 < 0f)), 4294967295u, (_e6019 > 4294967000f)) + _e5954);
                                                                            let _e6029 = global_6.member[(_e5880 + (_e6016 + 1u))];
                                                                            let _e6035 = (select(select(u32(_e6029), 0u, (_e6029 < 0f)), 4294967295u, (_e6029 > 4294967000f)) + _e5954);
                                                                            let _e6038 = global_6.member[(_e5880 + _e5996)];
                                                                            let _e6044 = (select(select(u32(_e6038), 0u, (_e6038 < 0f)), 4294967295u, (_e6038 > 4294967000f)) % 12u);
                                                                            let _e6047 = select(-1f, 1f, ((_e6044 & 1u) == 0u));
                                                                            let _e6050 = select(-1f, 1f, ((_e6044 & 2u) == 0u));
                                                                            if (_e6044 < 4u) {
                                                                                phi_25550_ = ((_e6047 * _e5955) + (_e6050 * _e5956));
                                                                            } else {
                                                                                if (_e6044 < 8u) {
                                                                                    phi_25544_ = ((_e6047 * _e5955) + (_e6050 * _e5957));
                                                                                } else {
                                                                                    phi_25544_ = ((_e6047 * _e5956) + (_e6050 * _e5957));
                                                                                }
                                                                                let _e6060 = phi_25544_;
                                                                                phi_25550_ = _e6060;
                                                                            }
                                                                            let _e6065 = phi_25550_;
                                                                            let _e6068 = global_6.member[(_e5880 + _e6025)];
                                                                            let _e6074 = (_e5955 - 1f);
                                                                            let _e6075 = (select(select(u32(_e6068), 0u, (_e6068 < 0f)), 4294967295u, (_e6068 > 4294967000f)) % 12u);
                                                                            let _e6078 = select(-1f, 1f, ((_e6075 & 1u) == 0u));
                                                                            let _e6081 = select(-1f, 1f, ((_e6075 & 2u) == 0u));
                                                                            if (_e6075 < 4u) {
                                                                                phi_25584_ = ((_e6078 * _e6074) + (_e6081 * _e5956));
                                                                            } else {
                                                                                if (_e6075 < 8u) {
                                                                                    phi_25578_ = ((_e6078 * _e6074) + (_e6081 * _e5957));
                                                                                } else {
                                                                                    phi_25578_ = ((_e6078 * _e5956) + (_e6081 * _e5957));
                                                                                }
                                                                                let _e6091 = phi_25578_;
                                                                                phi_25584_ = _e6091;
                                                                            }
                                                                            let _e6096 = phi_25584_;
                                                                            let _e6099 = global_6.member[(_e5880 + _e6006)];
                                                                            let _e6105 = (_e5956 - 1f);
                                                                            let _e6106 = (select(select(u32(_e6099), 0u, (_e6099 < 0f)), 4294967295u, (_e6099 > 4294967000f)) % 12u);
                                                                            let _e6109 = select(-1f, 1f, ((_e6106 & 1u) == 0u));
                                                                            let _e6112 = select(-1f, 1f, ((_e6106 & 2u) == 0u));
                                                                            if (_e6106 < 4u) {
                                                                                phi_25618_ = ((_e6109 * _e5955) + (_e6112 * _e6105));
                                                                            } else {
                                                                                if (_e6106 < 8u) {
                                                                                    phi_25612_ = ((_e6109 * _e5955) + (_e6112 * _e5957));
                                                                                } else {
                                                                                    phi_25612_ = ((_e6109 * _e6105) + (_e6112 * _e5957));
                                                                                }
                                                                                let _e6122 = phi_25612_;
                                                                                phi_25618_ = _e6122;
                                                                            }
                                                                            let _e6127 = phi_25618_;
                                                                            let _e6130 = global_6.member[(_e5880 + _e6035)];
                                                                            let _e6136 = (select(select(u32(_e6130), 0u, (_e6130 < 0f)), 4294967295u, (_e6130 > 4294967000f)) % 12u);
                                                                            let _e6139 = select(-1f, 1f, ((_e6136 & 1u) == 0u));
                                                                            let _e6142 = select(-1f, 1f, ((_e6136 & 2u) == 0u));
                                                                            if (_e6136 < 4u) {
                                                                                phi_25652_ = ((_e6139 * _e6074) + (_e6142 * _e6105));
                                                                            } else {
                                                                                if (_e6136 < 8u) {
                                                                                    phi_25646_ = ((_e6139 * _e6074) + (_e6142 * _e5957));
                                                                                } else {
                                                                                    phi_25646_ = ((_e6139 * _e6105) + (_e6142 * _e5957));
                                                                                }
                                                                                let _e6152 = phi_25646_;
                                                                                phi_25652_ = _e6152;
                                                                            }
                                                                            let _e6157 = phi_25652_;
                                                                            let _e6161 = global_6.member[(_e5880 + (_e5996 + 1u))];
                                                                            let _e6167 = (_e5957 - 1f);
                                                                            let _e6168 = (select(select(u32(_e6161), 0u, (_e6161 < 0f)), 4294967295u, (_e6161 > 4294967000f)) % 12u);
                                                                            let _e6171 = select(-1f, 1f, ((_e6168 & 1u) == 0u));
                                                                            let _e6174 = select(-1f, 1f, ((_e6168 & 2u) == 0u));
                                                                            if (_e6168 < 4u) {
                                                                                phi_25686_ = ((_e6171 * _e5955) + (_e6174 * _e5956));
                                                                            } else {
                                                                                if (_e6168 < 8u) {
                                                                                    phi_25680_ = ((_e6171 * _e5955) + (_e6174 * _e6167));
                                                                                } else {
                                                                                    phi_25680_ = ((_e6171 * _e5956) + (_e6174 * _e6167));
                                                                                }
                                                                                let _e6184 = phi_25680_;
                                                                                phi_25686_ = _e6184;
                                                                            }
                                                                            let _e6189 = phi_25686_;
                                                                            let _e6193 = global_6.member[(_e5880 + (_e6025 + 1u))];
                                                                            let _e6199 = (select(select(u32(_e6193), 0u, (_e6193 < 0f)), 4294967295u, (_e6193 > 4294967000f)) % 12u);
                                                                            let _e6202 = select(-1f, 1f, ((_e6199 & 1u) == 0u));
                                                                            let _e6205 = select(-1f, 1f, ((_e6199 & 2u) == 0u));
                                                                            if (_e6199 < 4u) {
                                                                                phi_25720_ = ((_e6202 * _e6074) + (_e6205 * _e5956));
                                                                            } else {
                                                                                if (_e6199 < 8u) {
                                                                                    phi_25714_ = ((_e6202 * _e6074) + (_e6205 * _e6167));
                                                                                } else {
                                                                                    phi_25714_ = ((_e6202 * _e5956) + (_e6205 * _e6167));
                                                                                }
                                                                                let _e6215 = phi_25714_;
                                                                                phi_25720_ = _e6215;
                                                                            }
                                                                            let _e6220 = phi_25720_;
                                                                            let _e6224 = global_6.member[(_e5880 + (_e6006 + 1u))];
                                                                            let _e6230 = (select(select(u32(_e6224), 0u, (_e6224 < 0f)), 4294967295u, (_e6224 > 4294967000f)) % 12u);
                                                                            let _e6233 = select(-1f, 1f, ((_e6230 & 1u) == 0u));
                                                                            let _e6236 = select(-1f, 1f, ((_e6230 & 2u) == 0u));
                                                                            if (_e6230 < 4u) {
                                                                                phi_25754_ = ((_e6233 * _e5955) + (_e6236 * _e6105));
                                                                            } else {
                                                                                if (_e6230 < 8u) {
                                                                                    phi_25748_ = ((_e6233 * _e5955) + (_e6236 * _e6167));
                                                                                } else {
                                                                                    phi_25748_ = ((_e6233 * _e6105) + (_e6236 * _e6167));
                                                                                }
                                                                                let _e6246 = phi_25748_;
                                                                                phi_25754_ = _e6246;
                                                                            }
                                                                            let _e6251 = phi_25754_;
                                                                            let _e6255 = global_6.member[(_e5880 + (_e6035 + 1u))];
                                                                            let _e6261 = (select(select(u32(_e6255), 0u, (_e6255 < 0f)), 4294967295u, (_e6255 > 4294967000f)) % 12u);
                                                                            let _e6264 = select(-1f, 1f, ((_e6261 & 1u) == 0u));
                                                                            let _e6267 = select(-1f, 1f, ((_e6261 & 2u) == 0u));
                                                                            if (_e6261 < 4u) {
                                                                                phi_25788_ = ((_e6264 * _e6074) + (_e6267 * _e6105));
                                                                            } else {
                                                                                if (_e6261 < 8u) {
                                                                                    phi_25782_ = ((_e6264 * _e6074) + (_e6267 * _e6167));
                                                                                } else {
                                                                                    phi_25782_ = ((_e6264 * _e6105) + (_e6267 * _e6167));
                                                                                }
                                                                                let _e6277 = phi_25782_;
                                                                                phi_25788_ = _e6277;
                                                                            }
                                                                            let _e6282 = phi_25788_;
                                                                            let _e6285 = (_e6065 + (_e5964 * (_e6096 - _e6065)));
                                                                            let _e6291 = (_e6189 + (_e5964 * (_e6220 - _e6189)));
                                                                            let _e6297 = (_e6285 + (_e5971 * ((_e6127 + (_e5964 * (_e6157 - _e6127))) - _e6285)));
                                                                            phi_8281_ = (_e5909 + 1u);
                                                                            phi_8284_ = (_e5911 + _e5913);
                                                                            phi_8286_ = (_e5913 * 0.5f);
                                                                            phi_8288_ = (_e5915 + ((_e6297 + ((((_e5957 * _e5957) * _e5957) * ((_e5957 * ((_e5957 * 6f) - 15f)) + 10f)) * ((_e6291 + (_e5971 * ((_e6251 + (_e5964 * (_e6282 - _e6251))) - _e6291))) - _e6297))) * _e5913));
                                                                            phi_8290_ = vec3<f32>(((((_e5883 * _e5917.x) + (_e5886 * _e5917.y)) + (_e5889 * _e5917.z)) * 2.03f), ((((_e5892 * _e5917.x) + (_e5895 * _e5917.y)) + (_e5898 * _e5917.z)) * 2.03f), ((((_e5901 * _e5917.x) + (_e5904 * _e5917.y)) + (_e5907 * _e5917.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8281_ = u32();
                                                                            phi_8284_ = f32();
                                                                            phi_8286_ = f32();
                                                                            phi_8288_ = f32();
                                                                            phi_8290_ = vec3<f32>();
                                                                        }
                                                                        let _e6329 = phi_8281_;
                                                                        let _e6331 = phi_8284_;
                                                                        let _e6333 = phi_8286_;
                                                                        let _e6335 = phi_8288_;
                                                                        let _e6337 = phi_8290_;
                                                                        phi_8280_ = _e6329;
                                                                        phi_8283_ = _e6331;
                                                                        phi_8285_ = _e6333;
                                                                        phi_8287_ = _e6335;
                                                                        phi_8289_ = _e6337;
                                                                        if !(_e5918) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6340 = local_36;
                                                                    if (_e6340 > 0f) {
                                                                        let _e6343 = local_37;
                                                                        let _e6345 = local_38;
                                                                        phi_8703_ = (_e6343 / _e6345);
                                                                    } else {
                                                                        phi_8703_ = 0f;
                                                                    }
                                                                    let _e6348 = phi_8703_;
                                                                    if _e5818 {
                                                                        phi_8738_ = _e5810;
                                                                        phi_8739_ = _e5812;
                                                                        phi_8740_ = _e5814;
                                                                    } else {
                                                                        if (_e5808 == 1u) {
                                                                            phi_8735_ = vec3<f32>(_e6348, _e5810.y, _e5810.z);
                                                                            phi_8736_ = _e5812;
                                                                            phi_8737_ = _e5814;
                                                                        } else {
                                                                            if (_e5808 == 2u) {
                                                                                phi_8732_ = vec3<f32>(_e5810.x, _e6348, _e5810.z);
                                                                                phi_8733_ = _e5812;
                                                                                phi_8734_ = _e5814;
                                                                            } else {
                                                                                if (_e5808 == 3u) {
                                                                                    phi_8729_ = vec3<f32>(_e5810.x, _e5810.y, _e6348);
                                                                                    phi_8730_ = _e5812;
                                                                                    phi_8731_ = _e5814;
                                                                                } else {
                                                                                    let _e6352 = (_e5808 == 4u);
                                                                                    phi_8729_ = _e5810;
                                                                                    phi_8730_ = select(_e6348, _e5812, _e6352);
                                                                                    phi_8731_ = select(_e5814, _e6348, _e6352);
                                                                                }
                                                                                let _e6360 = phi_8729_;
                                                                                let _e6362 = phi_8730_;
                                                                                let _e6364 = phi_8731_;
                                                                                phi_8732_ = _e6360;
                                                                                phi_8733_ = _e6362;
                                                                                phi_8734_ = _e6364;
                                                                            }
                                                                            let _e6370 = phi_8732_;
                                                                            let _e6372 = phi_8733_;
                                                                            let _e6374 = phi_8734_;
                                                                            phi_8735_ = _e6370;
                                                                            phi_8736_ = _e6372;
                                                                            phi_8737_ = _e6374;
                                                                        }
                                                                        let _e6380 = phi_8735_;
                                                                        let _e6382 = phi_8736_;
                                                                        let _e6384 = phi_8737_;
                                                                        phi_8738_ = _e6380;
                                                                        phi_8739_ = _e6382;
                                                                        phi_8740_ = _e6384;
                                                                    }
                                                                    let _e6386 = phi_8738_;
                                                                    let _e6388 = phi_8739_;
                                                                    let _e6390 = phi_8740_;
                                                                    phi_8742_ = _e6386;
                                                                    phi_8743_ = _e6388;
                                                                    phi_8744_ = _e6390;
                                                                    phi_8745_ = select(_e5816, _e6348, _e5818);
                                                                } else {
                                                                    phi_8742_ = _e5810;
                                                                    phi_8743_ = _e5812;
                                                                    phi_8744_ = _e5814;
                                                                    phi_8745_ = _e5816;
                                                                }
                                                                let _e6393 = phi_8742_;
                                                                let _e6395 = phi_8743_;
                                                                let _e6397 = phi_8744_;
                                                                let _e6399 = phi_8745_;
                                                                phi_8045_ = (_e5808 + 1u);
                                                                phi_8048_ = _e6393;
                                                                phi_8050_ = _e6395;
                                                                phi_8052_ = _e6397;
                                                                phi_8054_ = _e6399;
                                                            } else {
                                                                phi_8045_ = u32();
                                                                phi_8048_ = vec3<f32>();
                                                                phi_8050_ = f32();
                                                                phi_8052_ = f32();
                                                                phi_8054_ = f32();
                                                            }
                                                            let _e6402 = phi_8045_;
                                                            let _e6404 = phi_8048_;
                                                            let _e6406 = phi_8050_;
                                                            let _e6408 = phi_8052_;
                                                            let _e6410 = phi_8054_;
                                                            phi_8044_ = _e6402;
                                                            phi_8047_ = _e6404;
                                                            phi_8049_ = _e6406;
                                                            phi_8051_ = _e6408;
                                                            phi_8053_ = _e6410;
                                                            if !(_e5817) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6413 = local_39;
                                                        let _e6416 = (_e5777 * (1f + (0.45f * _e6413)));
                                                        if _e5781 {
                                                            let _e6519 = local_43;
                                                            let _e6521 = ((_e6519 - -0.25f) * 1.4285715f);
                                                            let _e6523 = select(_e6521, 0f, (_e6521 < 0f));
                                                            let _e6525 = select(_e6523, 1f, (_e6523 > 1f));
                                                            let _e6529 = ((_e6525 * _e6525) * (3f - (2f * _e6525)));
                                                            let _e6531 = ((_e6416 - 1.05f) * -1.1764706f);
                                                            let _e6533 = select(_e6531, 0f, (_e6531 < 0f));
                                                            let _e6535 = select(_e6533, 1f, (_e6533 > 1f));
                                                            let _e6539 = ((_e6535 * _e6535) * (3f - (2f * _e6535)));
                                                            let _e6540 = (_e6529 * _e6539);
                                                            let _e6542 = ((_e6416 - 0.65f) * -1.5384616f);
                                                            let _e6544 = select(_e6542, 0f, (_e6542 < 0f));
                                                            let _e6546 = select(_e6544, 1f, (_e6544 > 1f));
                                                            let _e6551 = (((_e6546 * _e6546) * (3f - (2f * _e6546))) * _e6529);
                                                            let _e6553 = local_44;
                                                            let _e6555 = ((_e6553 - 0.1f) * 2.857143f);
                                                            let _e6557 = select(_e6555, 0f, (_e6555 < 0f));
                                                            let _e6559 = select(_e6557, 1f, (_e6557 > 1f));
                                                            let _e6564 = (((_e6559 * _e6559) * (3f - (2f * _e6559))) * _e6539);
                                                            let _e6569 = ((0.8f * _e6551) * _e6551);
                                                            let _e6578 = (_e5715 * exp((-2f * _e6564)));
                                                            phi_9037_ = type_59(vec3<f32>((((3.1815925f * _e6540) + (0.14634217f * _e6569)) * _e6578), (((0.38179109f * _e6540) + (1.2439084f * _e6569)) * _e6578), (((0.69995034f * _e6540) + (1.0975662f * _e6569)) * _e6578)), (_e5718 * _e6564));
                                                        } else {
                                                            if (_e5697 == 1u) {
                                                                let _e6489 = local_42;
                                                                let _e6491 = ((_e6489 - -0.3f) * 1.25f);
                                                                let _e6493 = select(_e6491, 0f, (_e6491 < 0f));
                                                                let _e6495 = select(_e6493, 1f, (_e6493 > 1f));
                                                                let _e6501 = ((_e6416 - 1f) * -1f);
                                                                let _e6503 = select(_e6501, 0f, (_e6501 < 0f));
                                                                let _e6505 = select(_e6503, 1f, (_e6503 > 1f));
                                                                let _e6511 = (((_e6495 * _e6495) * (3f - (2f * _e6495))) * pow(((_e6505 * _e6505) * (3f - (2f * _e6505))), 1.5f));
                                                                phi_9036_ = type_59((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5715 * _e6511)), (_e5718 * _e6511));
                                                            } else {
                                                                if (_e5697 == 2u) {
                                                                    let _e6445 = local_41;
                                                                    let _e6448 = (_e5777 * (1f + (0.15f * _e6445)));
                                                                    let _e6449 = (_e6448 - 0.65f);
                                                                    let _e6453 = exp(((_e6449 * -5.5555553f) * (_e6449 * 5.5555553f)));
                                                                    let _e6455 = ((_e6448 - 0.75f) * -1.8181818f);
                                                                    let _e6457 = select(_e6455, 0f, (_e6455 < 0f));
                                                                    let _e6459 = select(_e6457, 1f, (_e6457 > 1f));
                                                                    let _e6463 = ((_e6459 * _e6459) * (3f - (2f * _e6459)));
                                                                    let _e6468 = (exp(((_e5777 * -25f) * (_e5777 * 25f))) * 3f);
                                                                    phi_9035_ = type_59(vec3<f32>(((((3.1815925f * _e6453) + (_e6463 * 0.10243952f)) + _e6468) * _e5715), ((((0.38179109f * _e6453) + (_e6463 * 0.8707359f)) + _e6468) * _e5715), ((((0.69995034f * _e6453) + (_e6463 * 0.76829636f)) + _e6468) * _e5715)), 0f);
                                                                } else {
                                                                    let _e6420 = local_40;
                                                                    let _e6422 = ((_e6420 - -0.2f) * 1.6666666f);
                                                                    let _e6424 = select(_e6422, 0f, (_e6422 < 0f));
                                                                    let _e6426 = select(_e6424, 1f, (_e6424 > 1f));
                                                                    let _e6432 = ((_e6416 - 1f) * -1.4285715f);
                                                                    let _e6434 = select(_e6432, 0f, (_e6432 < 0f));
                                                                    let _e6436 = select(_e6434, 1f, (_e6434 > 1f));
                                                                    phi_9035_ = type_59(vec3<f32>(0f, 0f, 0f), (_e5718 * (((_e6426 * _e6426) * (3f - (2f * _e6426))) * ((_e6436 * _e6436) * (3f - (2f * _e6436))))));
                                                                }
                                                                let _e6487 = phi_9035_;
                                                                phi_9036_ = _e6487;
                                                            }
                                                            let _e6517 = phi_9036_;
                                                            phi_9037_ = _e6517;
                                                        }
                                                        let _e6586 = phi_9037_;
                                                        phi_9038_ = _e6586;
                                                    }
                                                    let _e6588 = phi_9038_;
                                                    phi_7717_ = (_e5683 + _e6588.member);
                                                    phi_7720_ = (_e5685 + 1u);
                                                    phi_7722_ = (_e5687 + _e6588.member_1);
                                                } else {
                                                    phi_7717_ = vec3<f32>();
                                                    phi_7720_ = u32();
                                                    phi_7722_ = f32();
                                                }
                                                let _e6595 = phi_7717_;
                                                let _e6597 = phi_7720_;
                                                let _e6599 = phi_7722_;
                                                phi_7716_ = _e6595;
                                                phi_7719_ = _e6597;
                                                phi_7721_ = _e6599;
                                                if !(_e5688) {
                                                    break;
                                                }
                                            }
                                            phi_33861_ = _e4067;
                                            if _e4067 {
                                                break;
                                            }
                                            let _e6602 = local_45;
                                            if (_e6602 <= 0f) {
                                                phi_9067_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6605 = local_46;
                                                let _e6608 = local_47;
                                                let _e6613 = local_48;
                                                phi_9067_ = vec3<f32>(exp((_e6608 * -0.81f)), exp(-(_e6605)), exp((_e6613 * -1.25f)));
                                            }
                                            let _e6618 = phi_9067_;
                                            let _e6620 = local_49;
                                            let _e6625 = local_50;
                                            let _e6630 = local_51;
                                            let _e6635 = local_52;
                                            let _e6639 = local_53;
                                            let _e6643 = local_54;
                                            let _e6648 = global_6.member[(_e1292 + 9u)];
                                            phi_33870_ = _e4067;
                                            phi_9114_ = vec3<f32>((_e3772.x + ((((_e6620.x * _e6618.x) + _e6635.x) * _e6648) * _e3774.x)), (_e3772.y + ((((_e6625.y * _e6618.y) + _e6639.y) * _e6648) * _e3774.y)), (_e3772.z + ((((_e6630.z * _e6618.z) + _e6643.z) * _e6648) * _e3774.z)));
                                        } else {
                                            phi_33870_ = _e3770;
                                            phi_9114_ = _e3772;
                                        }
                                        let _e6666 = phi_33870_;
                                        let _e6668 = phi_9114_;
                                        phi_33869_ = _e6666;
                                        phi_9115_ = _e6668;
                                    } else {
                                        phi_33869_ = _e3770;
                                        phi_9115_ = _e3778;
                                    }
                                    let _e6670 = phi_33869_;
                                    let _e6672 = phi_9115_;
                                    phi_33868_ = _e6670;
                                    phi_9116_ = _e6672;
                                } else {
                                    phi_33868_ = _e323;
                                    phi_9116_ = _e1437;
                                }
                                let _e6674 = phi_33868_;
                                let _e6676 = phi_9116_;
                                phi_33864_ = _e6674;
                                phi_10389_ = (vec3<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z) * _e6676);
                            } else {
                                let _e350 = global_1.member.member_8[1u];
                                let _e360 = global_6.member[_e350];
                                let _e363 = global_6.member[(_e350 + 1u)];
                                let _e366 = global_6.member[(_e350 + 2u)];
                                let _e369 = global_6.member[(_e350 + 3u)];
                                let _e372 = global_6.member[(_e350 + 4u)];
                                let _e375 = global_6.member[(_e350 + 5u)];
                                let _e378 = global_6.member[(_e350 + 6u)];
                                let _e381 = global_6.member[(_e350 + 7u)];
                                let _e384 = global_6.member[(_e350 + 8u)];
                                let _e387 = global_6.member[(_e350 + 9u)];
                                let _e390 = global_6.member[(_e350 + 10u)];
                                let _e393 = global_6.member[(_e350 + 11u)];
                                let _e396 = global_6.member[(_e350 + 12u)];
                                let _e399 = global_6.member[(_e350 + 13u)];
                                let _e402 = global_6.member[(_e350 + 14u)];
                                let _e405 = global_6.member[(_e350 + 15u)];
                                let _e409 = global_6.member[(_e350 + 16u)];
                                let _e412 = global_6.member[(_e350 + 17u)];
                                let _e415 = global_6.member[(_e350 + 18u)];
                                let _e418 = global_6.member[(_e350 + 19u)];
                                let _e421 = global_6.member[(_e350 + 20u)];
                                let _e424 = global_6.member[(_e350 + 29u)];
                                let _e425 = bitcast<u32>(_e424);
                                let _e428 = global_6.member[(_e350 + 30u)];
                                let _e429 = bitcast<u32>(_e428);
                                let _e437 = ((((_e334.member.x - _e360) * _e334.member_1.x) + ((_e334.member.y - _e363) * _e334.member_1.y)) + ((_e334.member.z - _e366) * _e334.member_1.z));
                                let _e449 = ((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e360) + (_e334.member.y * _e363)) + (_e334.member.z * _e366))));
                                let _e450 = (_e449 + _e378);
                                let _e451 = (_e437 * _e437);
                                let _e452 = (_e451 - _e450);
                                if (_e452 < 0f) {
                                    phi_21693_ = type_57(0u, type_57().member_1);
                                    phi_21694_ = type_57();
                                    phi_21695_ = true;
                                } else {
                                    if (_e437 < 0f) {
                                        phi_21665_ = -(sqrt(_e452));
                                    } else {
                                        phi_21665_ = sqrt(_e452);
                                    }
                                    let _e459 = phi_21665_;
                                    let _e461 = -((_e437 + _e459));
                                    let _e462 = (_e461 == 0f);
                                    if _e462 {
                                        phi_21688_ = type_57(1u, type_56(0f, 0f));
                                        phi_21689_ = type_57();
                                    } else {
                                        let _e463 = (_e450 / _e461);
                                        if (_e461 < _e463) {
                                            phi_21677_ = type_56(_e461, _e463);
                                        } else {
                                            phi_21677_ = type_56(_e463, _e461);
                                        }
                                        let _e468 = phi_21677_;
                                        phi_21688_ = type_57();
                                        phi_21689_ = type_57(1u, _e468);
                                    }
                                    let _e471 = phi_21688_;
                                    let _e473 = phi_21689_;
                                    phi_21693_ = _e471;
                                    phi_21694_ = _e473;
                                    phi_21695_ = _e462;
                                }
                                let _e478 = phi_21693_;
                                let _e480 = phi_21694_;
                                let _e482 = phi_21695_;
                                if _e482 {
                                    phi_21699_ = _e478;
                                } else {
                                    phi_21699_ = _e480;
                                }
                                let _e484 = phi_21699_;
                                if (bitcast<i32>(_e484.member) != 0i) {
                                    phi_21586_ = type_57(0u, _e484.member_1);
                                } else {
                                    phi_21586_ = type_57(1u, type_57().member_1);
                                }
                                let _e494 = phi_21586_;
                                if (bitcast<i32>(_e494.member) != 0i) {
                                    phi_21633_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e494.member_1.member_1 <= 0f) {
                                        phi_21629_ = type_57(0u, type_57().member_1);
                                    } else {
                                        if (_e494.member_1.member != _e494.member_1.member) {
                                            phi_21737_ = true;
                                        } else {
                                            phi_21737_ = (0f >= _e494.member_1.member);
                                        }
                                        let _e506 = phi_21737_;
                                        let _e507 = select(_e494.member_1.member, 0f, _e506);
                                        let _e508 = (_e449 + _e375);
                                        if (_e508 > 0f) {
                                            let _e510 = (_e451 - _e508);
                                            if (_e510 < 0f) {
                                                phi_21845_ = type_57(0u, type_57().member_1);
                                                phi_21846_ = type_57();
                                                phi_21847_ = true;
                                            } else {
                                                if (_e437 < 0f) {
                                                    phi_21817_ = -(sqrt(_e510));
                                                } else {
                                                    phi_21817_ = sqrt(_e510);
                                                }
                                                let _e517 = phi_21817_;
                                                let _e519 = -((_e437 + _e517));
                                                let _e520 = (_e519 == 0f);
                                                if _e520 {
                                                    phi_21840_ = type_57(1u, type_56(0f, 0f));
                                                    phi_21841_ = type_57();
                                                } else {
                                                    let _e521 = (_e508 / _e519);
                                                    if (_e519 < _e521) {
                                                        phi_21829_ = type_56(_e519, _e521);
                                                    } else {
                                                        phi_21829_ = type_56(_e521, _e519);
                                                    }
                                                    let _e526 = phi_21829_;
                                                    phi_21840_ = type_57();
                                                    phi_21841_ = type_57(1u, _e526);
                                                }
                                                let _e529 = phi_21840_;
                                                let _e531 = phi_21841_;
                                                phi_21845_ = _e529;
                                                phi_21846_ = _e531;
                                                phi_21847_ = _e520;
                                            }
                                            let _e536 = phi_21845_;
                                            let _e538 = phi_21846_;
                                            let _e540 = phi_21847_;
                                            if _e540 {
                                                phi_21851_ = _e536;
                                            } else {
                                                phi_21851_ = _e538;
                                            }
                                            let _e542 = phi_21851_;
                                            if (bitcast<i32>(_e542.member) != 0i) {
                                                phi_21785_ = select(_e542.member_1.member_1, _e542.member_1.member, (_e542.member_1.member >= 0f));
                                            } else {
                                                phi_21785_ = -1f;
                                            }
                                            let _e553 = phi_21785_;
                                            if (_e553 >= 0f) {
                                                if (_e494.member_1.member_1 != _e494.member_1.member_1) {
                                                    phi_21889_ = true;
                                                } else {
                                                    phi_21889_ = (_e553 <= _e494.member_1.member_1);
                                                }
                                                let _e558 = phi_21889_;
                                                phi_21614_ = select(_e494.member_1.member_1, _e553, _e558);
                                            } else {
                                                phi_21614_ = _e494.member_1.member_1;
                                            }
                                            let _e561 = phi_21614_;
                                            phi_21616_ = _e561;
                                        } else {
                                            phi_21616_ = _e494.member_1.member_1;
                                        }
                                        let _e563 = phi_21616_;
                                        if (_e563 <= _e507) {
                                            phi_21625_ = type_57(0u, type_57().member_1);
                                        } else {
                                            phi_21625_ = type_57(1u, type_56(_e507, _e563));
                                        }
                                        let _e571 = phi_21625_;
                                        phi_21629_ = _e571;
                                    }
                                    let _e576 = phi_21629_;
                                    phi_21633_ = _e576;
                                }
                                let _e581 = phi_21633_;
                                if (bitcast<i32>(_e581.member) != 0i) {
                                    let _e587 = (_e581.member_1.member < _e334.member.w);
                                    if _e587 {
                                        if (_e581.member_1.member_1 != _e581.member_1.member_1) {
                                            phi_21904_ = true;
                                        } else {
                                            phi_21904_ = (_e334.member.w <= _e581.member_1.member_1);
                                        }
                                        let _e593 = phi_21904_;
                                        let _e594 = select(_e581.member_1.member_1, _e334.member.w, _e593);
                                        phi_9510_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9513_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9515_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9517_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9519_ = 0u;
                                        loop {
                                            let _e596 = phi_9510_;
                                            let _e598 = phi_9513_;
                                            let _e600 = phi_9515_;
                                            let _e602 = phi_9517_;
                                            let _e604 = phi_9519_;
                                            local = _e600;
                                            local_1 = _e598;
                                            local_2 = _e596;
                                            local_3 = _e596;
                                            local_4 = _e596;
                                            local_5 = _e602;
                                            local_6 = _e602;
                                            local_7 = _e602;
                                            let _e605 = (_e604 < 32u);
                                            if _e605 {
                                                let _e606 = -(_e437);
                                                if (_e581.member_1.member <= _e594) {
                                                } else {
                                                    phi_33383_ = true;
                                                    break;
                                                }
                                                let _e609 = select(_e606, _e581.member_1.member, (_e606 < _e581.member_1.member));
                                                let _e611 = select(_e609, _e594, (_e609 > _e594));
                                                let _e612 = (_e611 - _e581.member_1.member);
                                                let _e613 = (_e594 - _e611);
                                                let _e614 = (_e594 - _e581.member_1.member);
                                                let _e615 = (0.000000001f * _e614);
                                                if (_e612 <= _e615) {
                                                    let _e690 = (f32(_e604) * 0.03125f);
                                                    let _e693 = (f32((_e604 + 1u)) * 0.03125f);
                                                    let _e695 = ((_e690 + _e693) * 0.5f);
                                                    phi_9620_ = type_56((_e581.member_1.member + ((_e614 * _e695) * _e695)), (_e614 * ((_e693 * _e693) - (_e690 * _e690))));
                                                } else {
                                                    if (_e613 <= _e615) {
                                                        let _e673 = (f32((31u - _e604)) * 0.03125f);
                                                        let _e676 = (f32((32u - _e604)) * 0.03125f);
                                                        let _e678 = ((_e673 + _e676) * 0.5f);
                                                        phi_9619_ = type_56((_e594 - ((_e614 * _e678) * _e678)), (_e614 * ((_e676 * _e676) - (_e673 * _e673))));
                                                    } else {
                                                        let _e621 = round(((32f * _e612) / (_e612 + _e613)));
                                                        let _e626 = select(select(u32(_e621), 0u, (_e621 < 0f)), 4294967295u, (_e621 > 4294967000f));
                                                        if (_e626 < 1u) {
                                                            phi_9603_ = 1u;
                                                        } else {
                                                            phi_9603_ = select(_e626, 31u, (_e626 > 31u));
                                                        }
                                                        let _e631 = phi_9603_;
                                                        if (_e604 < _e631) {
                                                            let _e652 = ((_e631 - 1u) - _e604);
                                                            let _e654 = f32(_e631);
                                                            let _e655 = (f32(_e652) / _e654);
                                                            let _e658 = (f32((_e652 + 1u)) / _e654);
                                                            let _e660 = ((_e655 + _e658) * 0.5f);
                                                            phi_9618_ = type_56((_e611 - ((_e612 * _e660) * _e660)), (_e612 * ((_e658 * _e658) - (_e655 * _e655))));
                                                        } else {
                                                            let _e634 = (_e604 - _e631);
                                                            let _e636 = f32((32u - _e631));
                                                            let _e637 = (f32(_e634) / _e636);
                                                            let _e640 = (f32((_e634 + 1u)) / _e636);
                                                            let _e642 = ((_e637 + _e640) * 0.5f);
                                                            phi_9618_ = type_56((_e611 + ((_e613 * _e642) * _e642)), (_e613 * ((_e640 * _e640) - (_e637 * _e637))));
                                                        }
                                                        let _e670 = phi_9618_;
                                                        phi_9619_ = _e670;
                                                    }
                                                    let _e688 = phi_9619_;
                                                    phi_9620_ = _e688;
                                                }
                                                let _e705 = phi_9620_;
                                                let _e711 = (_e334.member.x + (_e334.member_1.x * _e705.member));
                                                let _e712 = (_e334.member.y + (_e334.member_1.y * _e705.member));
                                                let _e713 = (_e334.member.z + (_e334.member_1.z * _e705.member));
                                                let _e726 = ((((_e711 * _e711) + (_e712 * _e712)) + (_e713 * _e713)) - (2f * (((_e711 * _e360) + (_e712 * _e363)) + (_e713 * _e366))));
                                                let _e728 = (_e711 - _e360);
                                                let _e729 = (_e712 - _e363);
                                                let _e730 = (_e713 - _e366);
                                                let _e736 = sqrt((((_e728 * _e728) + (_e729 * _e729)) + (_e730 * _e730)));
                                                let _e738 = ((_e726 + _e375) / (_e736 + _e369));
                                                let _e742 = (1f - (abs((_e738 - 25000f)) * 0.00006666667f));
                                                if (_e742 != _e742) {
                                                    phi_22104_ = true;
                                                } else {
                                                    phi_22104_ = (0f >= _e742);
                                                }
                                                let _e746 = phi_22104_;
                                                let _e748 = -(_e738);
                                                let _e751 = (exp((_e748 / _e390)) * _e705.member_1);
                                                let _e754 = (exp((_e748 / _e393)) * _e705.member_1);
                                                let _e756 = (_e751 * 0.5f);
                                                let _e757 = (_e754 * 0.5f);
                                                let _e758 = ((select(_e742, 0f, _e746) * _e705.member_1) * 0.5f);
                                                let _e760 = (_e602.x + _e756);
                                                let _e762 = (_e602.y + _e757);
                                                let _e764 = (_e602.z + _e758);
                                                let _e769 = (_e409 * 1.1f);
                                                let _e770 = (_e769 * _e762);
                                                let _e789 = (_e409 * _e754);
                                                if (_e429 == 4294967295u) {
                                                    phi_9948_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e797 = (1f / _e736);
                                                    let _e805 = ((((_e728 * _e797) * _e381) + ((_e729 * _e797) * _e384)) + ((_e730 * _e797) * _e387));
                                                    if (_e738 != _e738) {
                                                        phi_22231_ = true;
                                                    } else {
                                                        phi_22231_ = (0f >= _e738);
                                                    }
                                                    let _e809 = phi_22231_;
                                                    let _e813 = sqrt((select(_e738, 0f, _e809) / (_e372 - _e369)));
                                                    if (_e813 != _e813) {
                                                        phi_22246_ = true;
                                                    } else {
                                                        phi_22246_ = (1f <= _e813);
                                                    }
                                                    let _e817 = phi_22246_;
                                                    let _e819 = (select(_e813, 1f, _e817) * 31f);
                                                    let _e821 = select(_e805, -1f, (_e805 < -1f));
                                                    let _e825 = ((select(_e821, 1f, (_e821 > 1f)) + 1f) * 15.5f);
                                                    let _e830 = select(select(u32(_e819), 0u, (_e819 < 0f)), 4294967295u, (_e819 > 4294967000f));
                                                    let _e832 = select(_e830, 30u, (30u < _e830));
                                                    let _e837 = select(select(u32(_e825), 0u, (_e825 < 0f)), 4294967295u, (_e825 > 4294967000f));
                                                    let _e839 = select(_e837, 30u, (30u < _e837));
                                                    let _e841 = (_e819 - f32(_e832));
                                                    let _e843 = (_e825 - f32(_e839));
                                                    let _e847 = (_e429 + (3u * ((_e832 * 32u) + _e839)));
                                                    let _e852 = (_e429 + (3u * (((_e832 + 1u) * 32u) + _e839)));
                                                    let _e854 = global_6.member[_e847];
                                                    let _e857 = global_6.member[(_e847 + 1u)];
                                                    let _e860 = global_6.member[(_e847 + 2u)];
                                                    let _e861 = (1f - _e843);
                                                    let _e867 = global_6.member[(_e847 + 3u)];
                                                    let _e870 = global_6.member[(_e847 + 4u)];
                                                    let _e873 = global_6.member[(_e847 + 5u)];
                                                    let _e880 = (1f - _e841);
                                                    let _e885 = global_6.member[_e852];
                                                    let _e888 = global_6.member[(_e852 + 1u)];
                                                    let _e891 = global_6.member[(_e852 + 2u)];
                                                    let _e897 = global_6.member[(_e852 + 3u)];
                                                    let _e900 = global_6.member[(_e852 + 4u)];
                                                    let _e903 = global_6.member[(_e852 + 5u)];
                                                    phi_9948_ = vec3<f32>(((((_e854 * _e861) + (_e867 * _e843)) * _e880) + (((_e885 * _e861) + (_e897 * _e843)) * _e841)), ((((_e857 * _e861) + (_e870 * _e843)) * _e880) + (((_e888 * _e861) + (_e900 * _e843)) * _e841)), ((((_e860 * _e861) + (_e873 * _e843)) * _e880) + (((_e891 * _e861) + (_e903 * _e843)) * _e841)));
                                                }
                                                let _e918 = phi_9948_;
                                                let _e932 = (_e726 + _e378);
                                                if (_e932 > 0f) {
                                                    let _e938 = (((_e728 * _e381) + (_e729 * _e384)) + (_e730 * _e387));
                                                    let _e940 = ((_e938 * _e938) - _e932);
                                                    if (_e938 >= 0f) {
                                                        phi_10022_ = vec3<f32>();
                                                        phi_10023_ = true;
                                                    } else {
                                                        let _e942 = (_e940 <= 0f);
                                                        if _e942 {
                                                            phi_10021_ = vec3<f32>();
                                                        } else {
                                                            let _e945 = (-(_e938) - sqrt(_e940));
                                                            phi_10021_ = vec3<f32>((_e711 + (_e381 * _e945)), (_e712 + (_e384 * _e945)), (_e713 + (_e387 * _e945)));
                                                        }
                                                        let _e954 = phi_10021_;
                                                        phi_10022_ = _e954;
                                                        phi_10023_ = _e942;
                                                    }
                                                    let _e956 = phi_10022_;
                                                    let _e958 = phi_10023_;
                                                    phi_10028_ = _e956;
                                                    phi_10029_ = select(true, false, _e958);
                                                } else {
                                                    phi_10028_ = vec3<f32>(_e711, _e712, _e713);
                                                    phi_10029_ = true;
                                                }
                                                let _e961 = phi_10028_;
                                                let _e963 = phi_10029_;
                                                if _e963 {
                                                    let _e965 = (_e961.x - _e360);
                                                    let _e967 = (_e961.y - _e363);
                                                    let _e969 = (_e961.z - _e366);
                                                    let _e975 = sqrt((((_e965 * _e965) + (_e967 * _e967)) + (_e969 * _e969)));
                                                    let _e976 = (1f / _e975);
                                                    let _e994 = ((((((_e961.x * _e961.x) + (_e961.y * _e961.y)) + (_e961.z * _e961.z)) - (2f * (((_e961.x * _e360) + (_e961.y * _e363)) + (_e961.z * _e366)))) + _e375) / (_e975 + _e369));
                                                    if (_e994 != _e994) {
                                                        phi_22370_ = true;
                                                    } else {
                                                        phi_22370_ = (0f >= _e994);
                                                    }
                                                    let _e998 = phi_22370_;
                                                    let _e1002 = sqrt((select(_e994, 0f, _e998) / (_e372 - _e369)));
                                                    if (_e1002 != _e1002) {
                                                        phi_22385_ = true;
                                                    } else {
                                                        phi_22385_ = (1f <= _e1002);
                                                    }
                                                    let _e1006 = phi_22385_;
                                                    let _e1008 = (select(_e1002, 1f, _e1006) * 63f);
                                                    let _e1013 = ((((_e965 * _e976) * _e381) + ((_e967 * _e976) * _e384)) + ((_e969 * _e976) * _e387));
                                                    let _e1015 = select(_e1013, -1f, (_e1013 < -1f));
                                                    let _e1019 = ((select(_e1015, 1f, (_e1015 > 1f)) + 1f) * 127.5f);
                                                    let _e1024 = select(select(u32(_e1008), 0u, (_e1008 < 0f)), 4294967295u, (_e1008 > 4294967000f));
                                                    let _e1026 = select(_e1024, 62u, (62u < _e1024));
                                                    let _e1031 = select(select(u32(_e1019), 0u, (_e1019 < 0f)), 4294967295u, (_e1019 > 4294967000f));
                                                    let _e1033 = select(_e1031, 254u, (254u < _e1031));
                                                    let _e1035 = (_e1008 - f32(_e1026));
                                                    let _e1037 = (_e1019 - f32(_e1033));
                                                    let _e1041 = (_e425 + (3u * ((_e1026 * 256u) + _e1033)));
                                                    let _e1046 = (_e425 + (3u * (((_e1026 + 1u) * 256u) + _e1033)));
                                                    let _e1048 = global_6.member[_e1041];
                                                    let _e1051 = global_6.member[(_e1041 + 1u)];
                                                    let _e1054 = global_6.member[(_e1041 + 2u)];
                                                    let _e1055 = (1f - _e1037);
                                                    let _e1061 = global_6.member[(_e1041 + 3u)];
                                                    let _e1064 = global_6.member[(_e1041 + 4u)];
                                                    let _e1067 = global_6.member[(_e1041 + 5u)];
                                                    let _e1074 = (1f - _e1035);
                                                    let _e1079 = global_6.member[_e1046];
                                                    let _e1082 = global_6.member[(_e1046 + 1u)];
                                                    let _e1085 = global_6.member[(_e1046 + 2u)];
                                                    let _e1091 = global_6.member[(_e1046 + 3u)];
                                                    let _e1094 = global_6.member[(_e1046 + 4u)];
                                                    let _e1097 = global_6.member[(_e1046 + 5u)];
                                                    phi_10320_ = vec3<f32>(((((_e1048 * _e1055) + (_e1061 * _e1037)) * _e1074) + (((_e1079 * _e1055) + (_e1091 * _e1037)) * _e1035)), ((((_e1051 * _e1055) + (_e1064 * _e1037)) * _e1074) + (((_e1082 * _e1055) + (_e1094 * _e1037)) * _e1035)), ((((_e1054 * _e1055) + (_e1067 * _e1037)) * _e1074) + (((_e1085 * _e1055) + (_e1097 * _e1037)) * _e1035)));
                                                } else {
                                                    phi_10320_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1112 = phi_10320_;
                                                let _e1113 = (vec3<f32>(_e760, _e762, _e764) + _e1112);
                                                let _e1119 = (_e769 * _e1113.y);
                                                let _e1131 = exp(-((((_e399 * _e1113.x) + _e1119) + (_e415 * _e1113.z))));
                                                let _e1133 = exp(-((((_e402 * _e1113.x) + _e1119) + (_e418 * _e1113.z))));
                                                let _e1135 = exp(-((((_e405 * _e1113.x) + _e1119) + (_e421 * _e1113.z))));
                                                phi_9511_ = vec3<f32>((_e596.x + ((exp(-((((_e399 * _e760) + _e770) + (_e415 * _e764)))) * ((_e399 * _e751) + _e789)) * _e918.x)), (_e596.y + ((exp(-((((_e402 * _e760) + _e770) + (_e418 * _e764)))) * ((_e402 * _e751) + _e789)) * _e918.y)), (_e596.z + ((exp(-((((_e405 * _e760) + _e770) + (_e421 * _e764)))) * ((_e405 * _e751) + _e789)) * _e918.z)));
                                                phi_9514_ = vec3<f32>((_e598.x + (_e1131 * _e754)), (_e598.y + (_e1133 * _e754)), (_e598.z + (_e1135 * _e754)));
                                                phi_9516_ = vec3<f32>((_e600.x + (_e1131 * _e751)), (_e600.y + (_e1133 * _e751)), (_e600.z + (_e1135 * _e751)));
                                                phi_9518_ = vec3<f32>((_e760 + _e756), (_e762 + _e757), (_e764 + _e758));
                                                phi_9520_ = (_e604 + 1u);
                                            } else {
                                                phi_9511_ = vec3<f32>();
                                                phi_9514_ = vec3<f32>();
                                                phi_9516_ = vec3<f32>();
                                                phi_9518_ = vec3<f32>();
                                                phi_9520_ = u32();
                                            }
                                            let _e1162 = phi_9511_;
                                            let _e1164 = phi_9514_;
                                            let _e1166 = phi_9516_;
                                            let _e1168 = phi_9518_;
                                            let _e1170 = phi_9520_;
                                            phi_9510_ = _e1162;
                                            phi_9513_ = _e1164;
                                            phi_9515_ = _e1166;
                                            phi_9517_ = _e1168;
                                            phi_9519_ = _e1170;
                                            phi_33383_ = _e323;
                                            if !(_e605) {
                                                break;
                                            }
                                        }
                                        let _e1173 = phi_33383_;
                                        phi_33861_ = _e1173;
                                        if _e1173 {
                                            break;
                                        }
                                        let _e1175 = local;
                                        let _e1176 = (_e1175 * vec3<f32>(_e399, _e402, _e405));
                                        let _e1178 = local_1;
                                        let _e1179 = (_e1178 * _e409);
                                        let _e1184 = (((_e334.member_1.x * _e381) + (_e334.member_1.y * _e384)) + (_e334.member_1.z * _e387));
                                        let _e1186 = (1f + (_e1184 * _e1184));
                                        let _e1187 = (0.059683103f * _e1186);
                                        let _e1194 = (_e412 * _e412);
                                        let _e1205 = ((0.119366206f * ((1f - _e1194) * _e1186)) / ((2f + _e1194) * pow(((1f + _e1194) - ((2f * _e412) * _e1184)), 1.5f)));
                                        let _e1216 = local_2;
                                        let _e1220 = local_3;
                                        let _e1224 = local_4;
                                        let _e1233 = local_5;
                                        let _e1240 = local_6;
                                        let _e1242 = ((_e409 * 1.1f) * _e1240.y);
                                        let _e1247 = local_7;
                                        phi_33867_ = _e1173;
                                        phi_10377_ = type_60(vec3<f32>(exp(-((((_e399 * _e1233.x) + _e1242) + (_e415 * _e1247.z)))), exp(-((((_e402 * _e1233.x) + _e1242) + (_e418 * _e1247.z)))), exp(-((((_e405 * _e1233.x) + _e1242) + (_e421 * _e1247.z))))), (vec3<f32>(((((_e1176.x * _e1187) + (_e1179.x * _e1205)) + _e1216.x) * _e396), ((((_e1176.y * _e1187) + (_e1179.y * _e1205)) + _e1220.y) * _e396), ((((_e1176.z * _e1187) + (_e1179.z * _e1205)) + _e1224.z) * _e396)) * 0.38317f));
                                    } else {
                                        phi_33867_ = _e323;
                                        phi_10377_ = type_60();
                                    }
                                    let _e1264 = phi_33867_;
                                    let _e1266 = phi_10377_;
                                    phi_33866_ = _e1264;
                                    phi_10379_ = _e1266;
                                    phi_10380_ = select(true, false, _e587);
                                } else {
                                    phi_33866_ = _e323;
                                    phi_10379_ = type_60();
                                    phi_10380_ = true;
                                }
                                let _e1269 = phi_33866_;
                                let _e1271 = phi_10379_;
                                let _e1273 = phi_10380_;
                                if _e1273 {
                                    phi_10385_ = type_60(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10385_ = _e1271;
                                }
                                let _e1275 = phi_10385_;
                                phi_33864_ = _e1269;
                                phi_10389_ = (vec3<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z) * _e1275.member_1);
                            }
                            let _e6680 = phi_33864_;
                            let _e6682 = phi_10389_;
                            phi_33863_ = _e6680;
                            phi_10390_ = _e6682;
                        }
                        let _e6684 = phi_33863_;
                        let _e6686 = phi_10390_;
                        phi_33862_ = _e6684;
                        phi_508_ = (_e325 + _e6686);
                        phi_511_ = (_e327 + 1u);
                    } else {
                        phi_33862_ = _e323;
                        phi_508_ = vec3<f32>();
                        phi_511_ = u32();
                    }
                    let _e6690 = phi_33862_;
                    let _e6692 = phi_508_;
                    let _e6694 = phi_511_;
                    phi_33400_ = _e6690;
                    phi_507_ = _e6692;
                    phi_510_ = _e6694;
                    phi_33861_ = _e6690;
                    if !(_e328) {
                        break;
                    }
                }
                let _e6697 = phi_33861_;
                if _e6697 {
                    break;
                }
                let _e6699 = local_55;
                let _e6703 = local_56;
                let _e6708 = local_57;
                if ((((_e6699.x * _e6699.x) + (_e6703.y * _e6703.y)) + (_e6708.z * _e6708.z)) > 0f) {
                    if (_e317.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6718 = global_7.member[_e317.member_1].member_3[0u];
                    global_7.member[_e317.member_1].member_3[0u] = (_e6718 + _e6699.x);
                    let _e6724 = global_7.member[_e317.member_1].member_3[1u];
                    global_7.member[_e317.member_1].member_3[1u] = (_e6724 + _e6703.y);
                    let _e6730 = global_7.member[_e317.member_1].member_3[2u];
                    global_7.member[_e317.member_1].member_3[2u] = (_e6730 + _e6708.z);
                    let _e6736 = global_7.member[_e317.member_1].member_3[3u];
                    global_7.member[_e317.member_1].member_3[3u] = _e6736;
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
    var phi_11190_: type_48;
    var phi_25983_: f32;
    var phi_25995_: type_56;
    var phi_26006_: type_57;
    var phi_26007_: type_57;
    var phi_26011_: type_57;
    var phi_26012_: type_57;
    var phi_26013_: bool;
    var phi_26017_: type_57;
    var phi_25904_: type_57;
    var phi_26055_: bool;
    var phi_26135_: f32;
    var phi_26147_: type_56;
    var phi_26158_: type_57;
    var phi_26159_: type_57;
    var phi_26163_: type_57;
    var phi_26164_: type_57;
    var phi_26165_: bool;
    var phi_26169_: type_57;
    var phi_26103_: f32;
    var phi_26207_: bool;
    var phi_25932_: f32;
    var phi_25934_: f32;
    var phi_25943_: type_57;
    var phi_25947_: type_57;
    var phi_25951_: type_57;
    var phi_26222_: bool;
    var phi_12709_: vec3<f32>;
    var phi_12712_: u32;
    var phi_12796_: u32;
    var phi_12811_: type_56;
    var phi_12812_: type_56;
    var phi_12813_: type_56;
    var phi_26421_: bool;
    var phi_12710_: vec3<f32>;
    var phi_12713_: u32;
    var phi_33872_: bool;
    var local_69: vec3<f32>;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var phi_36012_: bool;
    var phi_12838_: vec3<f32>;
    var phi_36011_: bool;
    var phi_12840_: vec3<f32>;
    var phi_12841_: bool;
    var phi_36009_: bool;
    var phi_12848_: vec3<f32>;
    var phi_12849_: type_41;
    var phi_36008_: bool;
    var phi_12850_: vec3<f32>;
    var phi_12851_: type_41;
    var phi_13130_: type_60;
    var phi_13097_: vec3<f32>;
    var phi_13098_: vec3<f32>;
    var phi_13131_: type_60;
    var phi_13254_: i32;
    var phi_13257_: u32;
    var phi_13311_: bool;
    var phi_13317_: bool;
    var phi_13324_: u32;
    var phi_13326_: i32;
    var phi_13327_: u32;
    var phi_13255_: i32;
    var phi_13258_: u32;
    var phi_13329_: bool;
    var local_72: u32;
    var phi_13863_: vec3<f32>;
    var phi_13877_: vec3<f32>;
    var phi_26488_: bool;
    var phi_13910_: f32;
    var phi_14018_: vec3<f32>;
    var phi_14024_: f32;
    var phi_26561_: bool;
    var phi_26550_: f32;
    var phi_14105_: vec3<f32>;
    var phi_14106_: vec3<f32>;
    var phi_14107_: vec3<f32>;
    var phi_34006_: u32;
    var phi_14109_: vec3<f32>;
    var phi_14110_: vec3<f32>;
    var phi_14111_: vec3<f32>;
    var phi_14112_: bool;
    var phi_14127_: vec3<f32>;
    var phi_14128_: vec3<f32>;
    var phi_34004_: u32;
    var phi_14130_: vec3<f32>;
    var phi_14131_: vec3<f32>;
    var phi_14132_: vec3<f32>;
    var phi_34003_: u32;
    var phi_14133_: vec3<f32>;
    var phi_14134_: vec3<f32>;
    var phi_14135_: vec3<f32>;
    var phi_14136_: bool;
    var phi_34002_: u32;
    var phi_14137_: vec3<f32>;
    var phi_14138_: vec3<f32>;
    var phi_14139_: vec3<f32>;
    var phi_14140_: bool;
    var phi_14141_: bool;
    var phi_26634_: bool;
    var phi_26796_: bool;
    var phi_26811_: bool;
    var phi_26826_: bool;
    var phi_26853_: bool;
    var phi_26868_: bool;
    var phi_26883_: bool;
    var phi_26910_: bool;
    var phi_26925_: bool;
    var phi_26940_: bool;
    var phi_14320_: f32;
    var phi_14328_: f32;
    var phi_14336_: f32;
    var phi_27137_: bool;
    var phi_27126_: f32;
    var phi_14443_: vec3<f32>;
    var phi_27165_: bool;
    var phi_14444_: vec3<f32>;
    var phi_14445_: vec3<f32>;
    var phi_14446_: vec3<f32>;
    var phi_34390_: vec3<f32>;
    var phi_34313_: vec3<f32>;
    var phi_34234_: u32;
    var phi_34152_: f32;
    var phi_14448_: vec3<f32>;
    var phi_14449_: vec3<f32>;
    var phi_14450_: bool;
    var phi_27180_: bool;
    var phi_27231_: bool;
    var phi_27220_: f32;
    var phi_34361_: vec3<f32>;
    var phi_13983_: vec3<f32>;
    var phi_34713_: vec3<f32>;
    var phi_34507_: f32;
    var phi_34360_: vec3<f32>;
    var phi_34048_: u32;
    var phi_13985_: vec3<f32>;
    var phi_13986_: bool;
    var phi_34659_: vec3<f32>;
    var phi_34467_: f32;
    var phi_34388_: vec3<f32>;
    var phi_34311_: vec3<f32>;
    var phi_34232_: u32;
    var phi_34150_: f32;
    var phi_34073_: u32;
    var phi_33999_: u32;
    var phi_14458_: vec3<f32>;
    var phi_14459_: vec3<f32>;
    var phi_14460_: vec3<f32>;
    var phi_14461_: bool;
    var phi_27297_: bool;
    var phi_14530_: vec3<f32>;
    var phi_34056_: u32;
    var phi_14532_: vec3<f32>;
    var phi_14533_: bool;
    var phi_34055_: u32;
    var phi_14534_: vec3<f32>;
    var phi_14535_: bool;
    var phi_34053_: u32;
    var phi_14541_: vec3<f32>;
    var phi_14542_: bool;
    var phi_14575_: type_69;
    var phi_34058_: u32;
    var phi_14577_: vec3<f32>;
    var phi_14578_: type_69;
    var phi_14579_: bool;
    var phi_14585_: type_69;
    var phi_34062_: u32;
    var phi_14586_: type_69;
    var phi_34656_: vec3<f32>;
    var phi_34464_: f32;
    var phi_34385_: vec3<f32>;
    var phi_34308_: vec3<f32>;
    var phi_34229_: u32;
    var phi_34147_: f32;
    var phi_34070_: u32;
    var phi_34061_: u32;
    var phi_14588_: type_69;
    var phi_14589_: bool;
    var phi_35985_: bool;
    var phi_34060_: u32;
    var phi_14600_: vec3<f32>;
    var phi_14603_: type_41;
    var phi_14605_: u32;
    var phi_14607_: u32;
    var phi_17771_: u32;
    var phi_17774_: u32;
    var phi_17772_: u32;
    var phi_17775_: u32;
    var local_73: u32;
    var phi_17812_: u32;
    var phi_17815_: u32;
    var phi_17813_: u32;
    var phi_17816_: u32;
    var local_74: u32;
    var local_75: u32;
    var phi_17892_: f32;
    var local_76: u32;
    var local_77: u32;
    var phi_17941_: f32;
    var phi_18097_: f32;
    var phi_18104_: vec3<f32>;
    var phi_36405_: u32;
    var phi_18106_: f32;
    var phi_18107_: vec3<f32>;
    var phi_18108_: vec3<f32>;
    var phi_16769_: u32;
    var phi_16772_: u32;
    var phi_16770_: u32;
    var phi_16773_: u32;
    var local_78: u32;
    var local_79: u32;
    var phi_17542_: f32;
    var phi_17545_: f32;
    var phi_27570_: bool;
    var phi_27551_: f32;
    var phi_17546_: f32;
    var phi_27585_: bool;
    var phi_27608_: bool;
    var phi_27623_: bool;
    var phi_17055_: f32;
    var phi_27638_: bool;
    var phi_27653_: bool;
    var phi_27668_: bool;
    var phi_27733_: bool;
    var phi_27821_: bool;
    var phi_27802_: f32;
    var phi_17120_: type_72;
    var phi_17548_: type_72;
    var phi_17724_: bool;
    var phi_17733_: vec3<f32>;
    var phi_17735_: vec3<f32>;
    var phi_17736_: bool;
    var phi_17741_: f32;
    var phi_17742_: vec3<f32>;
    var phi_36413_: u32;
    var phi_17743_: f32;
    var phi_17744_: vec3<f32>;
    var phi_17745_: vec3<f32>;
    var phi_17746_: f32;
    var phi_36412_: u32;
    var phi_17747_: f32;
    var phi_17748_: vec3<f32>;
    var phi_17749_: vec3<f32>;
    var phi_17750_: f32;
    var phi_36404_: u32;
    var phi_18109_: f32;
    var phi_18110_: vec3<f32>;
    var phi_18111_: vec3<f32>;
    var phi_18112_: f32;
    var phi_27836_: bool;
    var phi_27946_: f32;
    var phi_27974_: f32;
    var phi_28002_: f32;
    var phi_28136_: bool;
    var phi_28151_: bool;
    var phi_28166_: bool;
    var phi_28193_: bool;
    var phi_28208_: bool;
    var phi_28223_: bool;
    var phi_28250_: bool;
    var phi_28265_: bool;
    var phi_28280_: bool;
    var phi_14941_: f32;
    var phi_14949_: f32;
    var phi_14957_: f32;
    var phi_28477_: bool;
    var phi_28466_: f32;
    var phi_15064_: vec3<f32>;
    var phi_28505_: bool;
    var phi_15065_: vec3<f32>;
    var phi_15077_: type_59;
    var phi_15078_: type_59;
    var phi_15079_: bool;
    var phi_15084_: type_59;
    var phi_28542_: bool;
    var phi_28572_: bool;
    var phi_28561_: f32;
    var phi_28611_: vec3<f32>;
    var phi_15112_: type_59;
    var phi_15113_: type_59;
    var phi_28633_: vec3<f32>;
    var phi_15114_: type_59;
    var phi_15115_: type_59;
    var phi_15116_: bool;
    var phi_15120_: type_59;
    var phi_28665_: bool;
    var phi_28736_: bool;
    var phi_15940_: f32;
    var phi_28751_: bool;
    var phi_28766_: bool;
    var phi_16413_: vec3<f32>;
    var phi_16414_: vec3<f32>;
    var phi_16415_: bool;
    var phi_16420_: vec3<f32>;
    var phi_16421_: bool;
    var phi_28890_: bool;
    var phi_28905_: bool;
    var phi_16712_: vec3<f32>;
    var phi_16714_: vec3<f32>;
    var phi_16741_: vec3<f32>;
    var phi_16742_: vec3<f32>;
    var phi_15558_: vec3<f32>;
    var phi_15559_: vec3<f32>;
    var phi_15560_: bool;
    var phi_15565_: vec3<f32>;
    var phi_15566_: bool;
    var phi_29067_: bool;
    var phi_29082_: bool;
    var phi_15857_: vec3<f32>;
    var phi_15859_: vec3<f32>;
    var phi_36447_: u32;
    var phi_16743_: f32;
    var phi_16744_: vec3<f32>;
    var phi_16745_: vec3<f32>;
    var phi_36446_: u32;
    var phi_16746_: f32;
    var phi_16747_: vec3<f32>;
    var phi_16748_: vec3<f32>;
    var phi_36403_: u32;
    var phi_18113_: f32;
    var phi_18114_: vec3<f32>;
    var phi_18115_: vec3<f32>;
    var phi_18116_: f32;
    var phi_14703_: f32;
    var phi_14704_: f32;
    var phi_36402_: u32;
    var phi_18117_: f32;
    var phi_18118_: vec3<f32>;
    var phi_18119_: vec3<f32>;
    var phi_18120_: f32;
    var phi_29135_: bool;
    var phi_29245_: f32;
    var phi_29273_: f32;
    var phi_29301_: f32;
    var phi_29435_: bool;
    var phi_29450_: bool;
    var phi_29465_: bool;
    var phi_29492_: bool;
    var phi_29507_: bool;
    var phi_29522_: bool;
    var phi_29549_: bool;
    var phi_29564_: bool;
    var phi_29579_: bool;
    var phi_18343_: f32;
    var phi_18351_: f32;
    var phi_18359_: f32;
    var phi_29776_: bool;
    var phi_29765_: f32;
    var phi_18466_: vec3<f32>;
    var phi_29804_: bool;
    var phi_18467_: vec3<f32>;
    var phi_18479_: type_59;
    var phi_18480_: type_59;
    var phi_18481_: bool;
    var phi_18486_: type_59;
    var phi_29841_: bool;
    var phi_29871_: bool;
    var phi_29860_: f32;
    var phi_29910_: vec3<f32>;
    var phi_18514_: type_59;
    var phi_18515_: type_59;
    var phi_29932_: vec3<f32>;
    var phi_18516_: type_59;
    var phi_18517_: type_59;
    var phi_18518_: bool;
    var phi_18522_: type_59;
    var phi_29946_: f32;
    var phi_29949_: f32;
    var phi_18531_: f32;
    var phi_18564_: u32;
    var phi_18567_: u32;
    var phi_18569_: u32;
    var phi_18571_: u32;
    var phi_18573_: u32;
    var phi_18575_: vec4<f32>;
    var phi_18577_: f32;
    var phi_18964_: u32;
    var phi_18966_: u32;
    var phi_18967_: u32;
    var phi_18969_: bool;
    var phi_18970_: bool;
    var phi_18971_: u32;
    var phi_18972_: u32;
    var phi_18973_: u32;
    var phi_18974_: bool;
    var phi_18975_: bool;
    var phi_18976_: bool;
    var phi_30047_: bool;
    var phi_30077_: bool;
    var phi_30024_: type_56;
    var phi_30027_: type_56;
    var phi_30028_: bool;
    var phi_30033_: type_56;
    var phi_30134_: bool;
    var phi_30149_: bool;
    var phi_30164_: bool;
    var phi_30179_: bool;
    var phi_30111_: type_56;
    var phi_30114_: type_56;
    var phi_30115_: bool;
    var phi_30120_: type_56;
    var phi_30221_: bool;
    var phi_30236_: bool;
    var phi_30251_: bool;
    var phi_30266_: bool;
    var phi_30198_: type_56;
    var phi_30201_: type_56;
    var phi_30202_: bool;
    var phi_30207_: type_56;
    var phi_30002_: bool;
    var phi_30005_: bool;
    var phi_19005_: f32;
    var phi_30368_: bool;
    var phi_30398_: bool;
    var phi_30345_: type_56;
    var phi_30348_: type_56;
    var phi_30349_: bool;
    var phi_30354_: type_56;
    var phi_30455_: bool;
    var phi_30470_: bool;
    var phi_30485_: bool;
    var phi_30500_: bool;
    var phi_30432_: type_56;
    var phi_30435_: type_56;
    var phi_30436_: bool;
    var phi_30441_: type_56;
    var phi_30542_: bool;
    var phi_30557_: bool;
    var phi_30572_: bool;
    var phi_30587_: bool;
    var phi_30519_: type_56;
    var phi_30522_: type_56;
    var phi_30523_: bool;
    var phi_30528_: type_56;
    var phi_30323_: bool;
    var phi_30326_: bool;
    var phi_19014_: f32;
    var phi_19025_: u32;
    var phi_19036_: u32;
    var phi_19075_: u32;
    var phi_19076_: u32;
    var phi_19087_: u32;
    var phi_19088_: u32;
    var phi_19089_: u32;
    var phi_19090_: u32;
    var phi_19092_: u32;
    var phi_19093_: u32;
    var phi_19095_: u32;
    var phi_19096_: u32;
    var phi_19097_: bool;
    var phi_19098_: bool;
    var phi_18890_: vec4<f32>;
    var phi_18891_: vec4<f32>;
    var phi_18892_: bool;
    var phi_18896_: vec4<f32>;
    var phi_18897_: bool;
    var phi_18802_: bool;
    var phi_18839_: bool;
    var phi_18845_: f32;
    var phi_18855_: vec4<f32>;
    var phi_18856_: vec4<f32>;
    var phi_18857_: bool;
    var phi_18861_: vec4<f32>;
    var phi_18862_: bool;
    var phi_18866_: vec4<f32>;
    var phi_18867_: bool;
    var phi_18898_: vec4<f32>;
    var phi_18899_: bool;
    var phi_18732_: vec4<f32>;
    var phi_18733_: vec4<f32>;
    var phi_18734_: bool;
    var phi_18738_: vec4<f32>;
    var phi_18739_: bool;
    var phi_18740_: vec4<f32>;
    var phi_18741_: bool;
    var phi_18742_: bool;
    var phi_18750_: vec4<f32>;
    var phi_18751_: bool;
    var phi_18752_: vec4<f32>;
    var phi_18753_: bool;
    var phi_18754_: bool;
    var phi_18762_: vec4<f32>;
    var phi_18763_: bool;
    var phi_18908_: vec4<f32>;
    var phi_18909_: bool;
    var phi_18930_: f32;
    var phi_18936_: u32;
    var phi_18565_: u32;
    var phi_18568_: u32;
    var phi_18570_: u32;
    var phi_18572_: u32;
    var phi_18574_: u32;
    var phi_18576_: vec4<f32>;
    var phi_18578_: f32;
    var phi_19109_: vec4<f32>;
    var phi_19110_: bool;
    var phi_19111_: bool;
    var phi_35913_: bool;
    var phi_21555_: bool;
    var phi_21554_: vec4<f32>;
    var local_80: u32;
    var local_81: vec4<f32>;
    var phi_19116_: type_74;
    var local_82: u32;
    var phi_19122_: type_74;
    var phi_19174_: vec3<f32>;
    var phi_19175_: type_41;
    var phi_19176_: vec3<f32>;
    var phi_19177_: type_41;
    var phi_36355_: bool;
    var phi_19178_: vec3<f32>;
    var phi_19179_: type_41;
    var phi_19180_: u32;
    var phi_36354_: bool;
    var phi_19181_: vec3<f32>;
    var phi_19182_: type_41;
    var phi_19183_: u32;
    var phi_36640_: u32;
    var phi_36353_: bool;
    var phi_14601_: vec3<f32>;
    var phi_14604_: type_41;
    var phi_14606_: u32;
    var phi_14608_: u32;
    var phi_36360_: u32;
    var phi_36352_: bool;
    var phi_30738_: bool;
    var phi_30768_: bool;
    var phi_30757_: f32;
    var phi_30688_: bool;
    var phi_30824_: vec3<f32>;
    var phi_30980_: bool;
    var phi_30995_: bool;
    var phi_31010_: bool;
    var phi_30919_: vec3<f32>;
    var phi_30922_: vec3<f32>;
    var phi_30923_: bool;
    var phi_30693_: vec3<f32>;
    var phi_30697_: vec3<f32>;
    var phi_30698_: bool;
    var phi_31110_: bool;
    var phi_31084_: vec3<f32>;
    var phi_30705_: vec3<f32>;
    var phi_31125_: bool;
    var phi_31235_: f32;
    var phi_31263_: f32;
    var phi_31291_: f32;
    var phi_31425_: bool;
    var phi_31440_: bool;
    var phi_31455_: bool;
    var phi_31482_: bool;
    var phi_31497_: bool;
    var phi_31512_: bool;
    var phi_31539_: bool;
    var phi_31554_: bool;
    var phi_31569_: bool;
    var phi_19402_: f32;
    var phi_19410_: f32;
    var phi_19418_: f32;
    var phi_31766_: bool;
    var phi_31755_: f32;
    var phi_19525_: vec3<f32>;
    var phi_31794_: bool;
    var phi_19526_: vec3<f32>;
    var phi_19538_: type_59;
    var phi_19539_: type_59;
    var phi_19540_: bool;
    var phi_19545_: type_59;
    var phi_31831_: bool;
    var phi_31861_: bool;
    var phi_31850_: f32;
    var phi_31900_: vec3<f32>;
    var phi_19573_: type_59;
    var phi_19574_: type_59;
    var phi_31922_: vec3<f32>;
    var phi_19575_: type_59;
    var phi_19576_: type_59;
    var phi_19577_: bool;
    var phi_19581_: type_59;
    var phi_31947_: bool;
    var phi_19657_: vec3<f32>;
    var phi_38194_: u32;
    var phi_19659_: vec3<f32>;
    var phi_19660_: bool;
    var phi_38193_: u32;
    var phi_19661_: vec3<f32>;
    var phi_19662_: bool;
    var phi_38191_: u32;
    var phi_19668_: vec3<f32>;
    var phi_19669_: bool;
    var local_83: u32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: type_41;
    var phi_19698_: vec3<f32>;
    var local_88: u32;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: type_41;
    var phi_19714_: type_69;
    var phi_19715_: type_69;
    var phi_38198_: u32;
    var phi_19717_: type_69;
    var phi_19718_: type_69;
    var phi_19719_: bool;
    var phi_38197_: u32;
    var phi_19720_: type_69;
    var phi_19721_: type_69;
    var phi_19722_: bool;
    var phi_19723_: bool;
    var local_93: u32;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: vec3<f32>;
    var local_97: type_41;
    var phi_19729_: type_69;
    var local_98: u32;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: type_41;
    var phi_19734_: type_69;
    var phi_19735_: type_69;
    var phi_19736_: bool;
    var phi_19740_: type_69;
    var phi_19741_: type_69;
    var phi_19742_: bool;
    var phi_13388_: bool;
    var phi_13397_: vec3<f32>;
    var phi_13399_: vec3<f32>;
    var phi_13400_: bool;
    var phi_13818_: f32;
    var phi_13821_: f32;
    var phi_32168_: bool;
    var phi_32149_: f32;
    var phi_13822_: f32;
    var phi_32186_: f32;
    var phi_32189_: f32;
    var phi_13825_: f32;
    var phi_13826_: f32;
    var phi_13827_: f32;
    var phi_19746_: type_69;
    var phi_19747_: type_69;
    var phi_19748_: bool;
    var phi_11449_: f32;
    var phi_32236_: f32;
    var phi_32239_: f32;
    var phi_11451_: f32;
    var phi_11452_: f32;
    var phi_32265_: bool;
    var phi_11503_: f32;
    var phi_32283_: f32;
    var phi_32286_: f32;
    var phi_11505_: f32;
    var phi_32297_: bool;
    var phi_32312_: bool;
    var phi_11974_: vec3<f32>;
    var phi_11975_: vec3<f32>;
    var phi_11976_: bool;
    var phi_11981_: vec3<f32>;
    var phi_11982_: bool;
    var phi_32436_: bool;
    var phi_32451_: bool;
    var phi_12273_: vec3<f32>;
    var phi_12275_: vec3<f32>;
    var phi_12302_: vec3<f32>;
    var phi_12316_: vec3<f32>;
    var phi_19752_: type_69;
    var phi_19753_: type_69;
    var phi_19754_: bool;
    var phi_19758_: type_69;

    switch bitcast<i32>(0u) {
        default: {
            let _e278 = global;
            let _e288 = arrayLength((&global_9.member));
            let _e292 = global_1.member.member_7[2u];
            let _e296 = (_e278.x + (_e278.y * 4194240u));
            let _e301 = atomicLoad((&global_8.member[((_e292 * 4u) + 3u)]));
            if (_e296 >= _e301) {
                phi_11190_ = type_48(0u, type_48().member_1);
            } else {
                let _e306 = global_1.member.member[0u];
                let _e311 = global_1.member.member[1u];
                let _e317 = atomicLoad((&global_8.member[((8u + ((_e292 * _e306) * _e311)) + _e296)]));
                phi_11190_ = type_48(1u, _e317);
            }
            let _e323 = phi_11190_;
            if (bitcast<i32>(_e323.member) != 0i) {
                let _e328 = (_e323.member_1 < arrayLength((&global_7.member)));
                if _e328 {
                } else {
                    break;
                }
                let _e331 = global_7.member[_e323.member_1];
                let _e345 = vec3<f32>(_e331.member_1.x, _e331.member_1.y, _e331.member_1.z);
                let _e350 = vec3<f32>(_e331.member_2.x, _e331.member_2.y, _e331.member_2.z);
                let _e356 = select(select(u32(_e331.member_2.w), 0u, (_e331.member_2.w < 0f)), 4294967295u, (_e331.member_2.w > 4294967000f));
                let _e360 = vec3<f32>(_e331.member_3.x, _e331.member_3.y, _e331.member_3.z);
                let _e370 = ((_e331.member_4.y & 2u) != 0u);
                let _e373 = global_1.member.member[3u];
                let _e376 = global_1.member.member_3[3u];
                let _e379 = global_1.member.member_1[2u];
                let _e382 = (select(select(u32(_e331.member_3.w), 0u, (_e331.member_3.w < 0f)), 4294967295u, (_e331.member_3.w > 4294967000f)) + 1u);
                if (_e331.member_4.z == 4294967295u) {
                    if _e370 {
                        phi_11452_ = 1f;
                    } else {
                        if ((_e376 & 2u) == 0u) {
                            phi_11451_ = 1f;
                        } else {
                            let _e6552 = global_1.member.member_3[1u];
                            let _e6555 = global_1.member.member_3[2u];
                            let _e6558 = ((atan2(_e331.member_1.z, _e331.member_1.x) * 0.15915494f) + 0.5f);
                            let _e6562 = select(_e331.member_1.y, -1f, (_e331.member_1.y < -1f));
                            let _e6565 = acos(select(_e6562, 1f, (_e6562 > 1f)));
                            let _e6567 = f32(_e6552);
                            let _e6568 = ((_e6558 - floor(_e6558)) * _e6567);
                            let _e6573 = select(select(u32(_e6568), 0u, (_e6568 < 0f)), 4294967295u, (_e6568 > 4294967000f));
                            let _e6574 = (_e6552 - 1u);
                            let _e6577 = f32(_e6555);
                            let _e6578 = ((_e6565 * 0.31830987f) * _e6577);
                            let _e6583 = select(select(u32(_e6578), 0u, (_e6578 < 0f)), 4294967295u, (_e6578 > 4294967000f));
                            let _e6584 = (_e6555 - 1u);
                            let _e6586 = select(_e6583, _e6584, (_e6584 < _e6583));
                            let _e6587 = sin(_e6565);
                            if (_e6587 <= 0f) {
                                phi_11449_ = 0f;
                            } else {
                                let _e6591 = global_1.member.member_2[3u];
                                let _e6594 = global_1.member.member_3[0u];
                                let _e6598 = (_e6591 + _e6586);
                                let _e6601 = global_6.member[(_e6598 + 1u)];
                                let _e6603 = global_6.member[_e6598];
                                let _e6606 = ((_e6594 + (_e6586 * (_e6552 + 1u))) + select(_e6573, _e6574, (_e6574 < _e6573)));
                                let _e6609 = global_6.member[(_e6606 + 1u)];
                                let _e6611 = global_6.member[_e6606];
                                phi_11449_ = ((((_e6601 - _e6603) * _e6577) * ((_e6609 - _e6611) * _e6567)) / (19.73921f * _e6587));
                            }
                            let _e6618 = phi_11449_;
                            if (_e331.member_1.w >= 300000000000000000000000000000000000000f) {
                                phi_32239_ = 1f;
                            } else {
                                let _e6620 = (_e331.member_1.w * _e331.member_1.w);
                                let _e6622 = (_e6620 + (_e6618 * _e6618));
                                if (_e6622 == 0f) {
                                    phi_32236_ = 0f;
                                } else {
                                    phi_32236_ = (_e6620 / _e6622);
                                }
                                let _e6626 = phi_32236_;
                                phi_32239_ = _e6626;
                            }
                            let _e6628 = phi_32239_;
                            phi_11451_ = _e6628;
                        }
                        let _e6630 = phi_11451_;
                        phi_11452_ = _e6630;
                    }
                    let _e6632 = phi_11452_;
                    if ((_e376 & 4u) == 0u) {
                        phi_12316_ = _e360;
                    } else {
                        if _e370 {
                            phi_11505_ = 1f;
                        } else {
                            let _e6643 = global_1.member.member_5;
                            let _e6647 = (_e331.member_1.x - _e6643.x);
                            let _e6648 = (_e331.member_1.y - _e6643.y);
                            let _e6649 = (_e331.member_1.z - _e6643.z);
                            let _e6656 = (sqrt((((_e6647 * _e6647) + (_e6648 * _e6648)) + (_e6649 * _e6649))) * 0.5f);
                            if (_e6656 != _e6656) {
                                phi_32265_ = true;
                            } else {
                                phi_32265_ = (1f <= _e6656);
                            }
                            let _e6660 = phi_32265_;
                            let _e6666 = global_1.member.member_6[3u];
                            if ((2f * asin(select(_e6656, 1f, _e6660))) >= _e6666) {
                                phi_11503_ = 0f;
                            } else {
                                let _e6669 = sin((_e6666 * 0.5f));
                                phi_11503_ = (1f / ((12.566371f * _e6669) * _e6669));
                            }
                            let _e6674 = phi_11503_;
                            if (_e331.member_1.w >= 300000000000000000000000000000000000000f) {
                                phi_32286_ = 1f;
                            } else {
                                let _e6676 = (_e331.member_1.w * _e331.member_1.w);
                                let _e6678 = (_e6676 + (_e6674 * _e6674));
                                if (_e6678 == 0f) {
                                    phi_32283_ = 0f;
                                } else {
                                    phi_32283_ = (_e6676 / _e6678);
                                }
                                let _e6682 = phi_32283_;
                                phi_32286_ = _e6682;
                            }
                            let _e6684 = phi_32286_;
                            phi_11505_ = _e6684;
                        }
                        let _e6686 = phi_11505_;
                        let _e6688 = global_1.member.member_5;
                        let _e6692 = (_e331.member_1.x - _e6688.x);
                        let _e6693 = (_e331.member_1.y - _e6688.y);
                        let _e6694 = (_e331.member_1.z - _e6688.z);
                        let _e6701 = (sqrt((((_e6692 * _e6692) + (_e6693 * _e6693)) + (_e6694 * _e6694))) * 0.5f);
                        if (_e6701 != _e6701) {
                            phi_32297_ = true;
                        } else {
                            phi_32297_ = (1f <= _e6701);
                        }
                        let _e6705 = phi_32297_;
                        let _e6708 = (2f * asin(select(_e6701, 1f, _e6705)));
                        let _e6712 = global_1.member.member_6[3u];
                        if (_e6708 >= _e6712) {
                            phi_12302_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e6716 = (1f - pow((_e6708 / _e6712), 2f));
                            if (_e6716 != _e6716) {
                                phi_32312_ = true;
                            } else {
                                phi_32312_ = (0f >= _e6716);
                            }
                            let _e6720 = phi_32312_;
                            let _e6723 = global_1.member.member_6;
                            let _e6729 = global_1.member.member_8[2u];
                            if (_e6729 == 4294967295u) {
                                phi_12275_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e6732 = global_6.member[_e6729];
                                let _e6735 = global_6.member[(_e6729 + 1u)];
                                let _e6738 = global_6.member[(_e6729 + 2u)];
                                let _e6741 = global_6.member[(_e6729 + 3u)];
                                let _e6744 = global_6.member[(_e6729 + 4u)];
                                let _e6747 = global_6.member[(_e6729 + 5u)];
                                let _e6750 = global_6.member[(_e6729 + 6u)];
                                let _e6753 = global_6.member[(_e6729 + 7u)];
                                let _e6756 = global_6.member[(_e6729 + 8u)];
                                let _e6759 = global_6.member[(_e6729 + 9u)];
                                let _e6762 = global_6.member[(_e6729 + 13u)];
                                let _e6765 = global_6.member[(_e6729 + 14u)];
                                let _e6768 = global_6.member[(_e6729 + 15u)];
                                let _e6771 = global_6.member[(_e6729 + 16u)];
                                let _e6774 = global_6.member[(_e6729 + 18u)];
                                let _e6777 = global_6.member[(_e6729 + 19u)];
                                let _e6780 = global_6.member[(_e6729 + 20u)];
                                let _e6783 = global_6.member[(_e6729 + 29u)];
                                let _e6784 = bitcast<u32>(_e6783);
                                let _e6797 = (((((_e331.member.x * _e331.member.x) + (_e331.member.y * _e331.member.y)) + (_e331.member.z * _e331.member.z)) - (2f * (((_e331.member.x * _e6732) + (_e331.member.y * _e6735)) + (_e331.member.z * _e6738)))) + _e6750);
                                if (_e6797 > 0f) {
                                    let _e6806 = ((((_e331.member.x - _e6732) * _e6753) + ((_e331.member.y - _e6735) * _e6756)) + ((_e331.member.z - _e6738) * _e6759));
                                    let _e6808 = ((_e6806 * _e6806) - _e6797);
                                    if (_e6806 >= 0f) {
                                        phi_11975_ = vec3<f32>();
                                        phi_11976_ = true;
                                    } else {
                                        let _e6810 = (_e6808 <= 0f);
                                        if _e6810 {
                                            phi_11974_ = vec3<f32>();
                                        } else {
                                            let _e6813 = (-(_e6806) - sqrt(_e6808));
                                            phi_11974_ = vec3<f32>((_e331.member.x + (_e6753 * _e6813)), (_e331.member.y + (_e6756 * _e6813)), (_e331.member.z + (_e6759 * _e6813)));
                                        }
                                        let _e6822 = phi_11974_;
                                        phi_11975_ = _e6822;
                                        phi_11976_ = _e6810;
                                    }
                                    let _e6824 = phi_11975_;
                                    let _e6826 = phi_11976_;
                                    phi_11981_ = _e6824;
                                    phi_11982_ = select(true, false, _e6826);
                                } else {
                                    phi_11981_ = vec3<f32>(_e331.member.x, _e331.member.y, _e331.member.z);
                                    phi_11982_ = true;
                                }
                                let _e6829 = phi_11981_;
                                let _e6831 = phi_11982_;
                                if _e6831 {
                                    let _e6833 = (_e6829.x - _e6732);
                                    let _e6835 = (_e6829.y - _e6735);
                                    let _e6837 = (_e6829.z - _e6738);
                                    let _e6843 = sqrt((((_e6833 * _e6833) + (_e6835 * _e6835)) + (_e6837 * _e6837)));
                                    let _e6844 = (1f / _e6843);
                                    let _e6862 = ((((((_e6829.x * _e6829.x) + (_e6829.y * _e6829.y)) + (_e6829.z * _e6829.z)) - (2f * (((_e6829.x * _e6732) + (_e6829.y * _e6735)) + (_e6829.z * _e6738)))) + _e6747) / (_e6843 + _e6741));
                                    if (_e6862 != _e6862) {
                                        phi_32436_ = true;
                                    } else {
                                        phi_32436_ = (0f >= _e6862);
                                    }
                                    let _e6866 = phi_32436_;
                                    let _e6870 = sqrt((select(_e6862, 0f, _e6866) / (_e6744 - _e6741)));
                                    if (_e6870 != _e6870) {
                                        phi_32451_ = true;
                                    } else {
                                        phi_32451_ = (1f <= _e6870);
                                    }
                                    let _e6874 = phi_32451_;
                                    let _e6876 = (select(_e6870, 1f, _e6874) * 63f);
                                    let _e6881 = ((((_e6833 * _e6844) * _e6753) + ((_e6835 * _e6844) * _e6756)) + ((_e6837 * _e6844) * _e6759));
                                    let _e6883 = select(_e6881, -1f, (_e6881 < -1f));
                                    let _e6887 = ((select(_e6883, 1f, (_e6883 > 1f)) + 1f) * 127.5f);
                                    let _e6892 = select(select(u32(_e6876), 0u, (_e6876 < 0f)), 4294967295u, (_e6876 > 4294967000f));
                                    let _e6894 = select(_e6892, 62u, (62u < _e6892));
                                    let _e6899 = select(select(u32(_e6887), 0u, (_e6887 < 0f)), 4294967295u, (_e6887 > 4294967000f));
                                    let _e6901 = select(_e6899, 254u, (254u < _e6899));
                                    let _e6903 = (_e6876 - f32(_e6894));
                                    let _e6905 = (_e6887 - f32(_e6901));
                                    let _e6909 = (_e6784 + (3u * ((_e6894 * 256u) + _e6901)));
                                    let _e6914 = (_e6784 + (3u * (((_e6894 + 1u) * 256u) + _e6901)));
                                    let _e6916 = global_6.member[_e6909];
                                    let _e6919 = global_6.member[(_e6909 + 1u)];
                                    let _e6922 = global_6.member[(_e6909 + 2u)];
                                    let _e6923 = (1f - _e6905);
                                    let _e6929 = global_6.member[(_e6909 + 3u)];
                                    let _e6932 = global_6.member[(_e6909 + 4u)];
                                    let _e6935 = global_6.member[(_e6909 + 5u)];
                                    let _e6942 = (1f - _e6903);
                                    let _e6947 = global_6.member[_e6914];
                                    let _e6950 = global_6.member[(_e6914 + 1u)];
                                    let _e6953 = global_6.member[(_e6914 + 2u)];
                                    let _e6959 = global_6.member[(_e6914 + 3u)];
                                    let _e6962 = global_6.member[(_e6914 + 4u)];
                                    let _e6965 = global_6.member[(_e6914 + 5u)];
                                    phi_12273_ = vec3<f32>(((((_e6916 * _e6923) + (_e6929 * _e6905)) * _e6942) + (((_e6947 * _e6923) + (_e6959 * _e6905)) * _e6903)), ((((_e6919 * _e6923) + (_e6932 * _e6905)) * _e6942) + (((_e6950 * _e6923) + (_e6962 * _e6905)) * _e6903)), ((((_e6922 * _e6923) + (_e6935 * _e6905)) * _e6942) + (((_e6953 * _e6923) + (_e6965 * _e6905)) * _e6903)));
                                } else {
                                    phi_12273_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e6980 = phi_12273_;
                                let _e6987 = ((_e6771 * 1.1f) * _e6980.y);
                                phi_12275_ = vec3<f32>(exp(-((((_e6762 * _e6980.x) + _e6987) + (_e6774 * _e6980.z)))), exp(-((((_e6765 * _e6980.x) + _e6987) + (_e6777 * _e6980.z)))), exp(-((((_e6768 * _e6980.x) + _e6987) + (_e6780 * _e6980.z)))));
                            }
                            let _e7006 = phi_12275_;
                            let _e7018 = sin((_e6712 * 0.5f));
                            let _e7021 = (((1f - (0.6f * (1f - sqrt(select(_e6716, 0f, _e6720))))) * 1.25f) / ((12.566371f * _e7018) * _e7018));
                            phi_12302_ = vec3<f32>(((_e6723.x * _e7006.x) * _e7021), ((_e6723.y * _e7006.y) * _e7021), ((_e6723.z * _e7006.z) * _e7021));
                        }
                        let _e7027 = phi_12302_;
                        phi_12316_ = vec3<f32>((_e331.member_3.x + ((_e331.member_2.x * _e7027.x) * _e6686)), (_e331.member_3.y + ((_e331.member_2.y * _e7027.y) * _e6686)), (_e331.member_3.z + ((_e331.member_2.z * _e7027.z) * _e6686)));
                    }
                    let _e7042 = phi_12316_;
                    phi_19752_ = type_69(type_34(_e331.member, _e331.member_1, vec4<f32>(_e331.member_2.x, _e331.member_2.y, _e331.member_2.z, f32(_e356)), vec4<f32>(_e7042.x, _e7042.y, _e7042.z, f32(_e382)), vec4<u32>(_e331.member_4.x, (_e331.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), type_41(vec4<f32>(_e331.member.x, _e331.member.y, _e331.member.z, -1f), vec4<f32>(_e331.member_1.x, _e331.member_1.y, _e331.member_1.z, 0f), vec4<f32>((_e331.member_2.x * _e6632), (_e331.member_2.y * _e6632), (_e331.member_2.z * _e6632), 0f)), type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_19753_ = type_69();
                    phi_19754_ = true;
                } else {
                    if ((_e376 & 8u) == 0u) {
                        phi_36008_ = false;
                        phi_12850_ = _e350;
                        phi_12851_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    } else {
                        if (_e331.member_5.x > 100f) {
                            let _e393 = global_1.member.member_8[1u];
                            let _e395 = global_6.member[_e393];
                            let _e398 = global_6.member[(_e393 + 1u)];
                            let _e401 = global_6.member[(_e393 + 2u)];
                            let _e404 = global_6.member[(_e393 + 3u)];
                            let _e407 = global_6.member[(_e393 + 5u)];
                            let _e410 = global_6.member[(_e393 + 6u)];
                            let _e413 = global_6.member[(_e393 + 10u)];
                            let _e416 = global_6.member[(_e393 + 11u)];
                            let _e419 = global_6.member[(_e393 + 13u)];
                            let _e422 = global_6.member[(_e393 + 14u)];
                            let _e425 = global_6.member[(_e393 + 15u)];
                            let _e428 = global_6.member[(_e393 + 16u)];
                            let _e431 = global_6.member[(_e393 + 18u)];
                            let _e434 = global_6.member[(_e393 + 19u)];
                            let _e437 = global_6.member[(_e393 + 20u)];
                            let _e445 = ((((_e331.member.x - _e395) * _e331.member_1.x) + ((_e331.member.y - _e398) * _e331.member_1.y)) + ((_e331.member.z - _e401) * _e331.member_1.z));
                            let _e457 = ((((_e331.member.x * _e331.member.x) + (_e331.member.y * _e331.member.y)) + (_e331.member.z * _e331.member.z)) - (2f * (((_e331.member.x * _e395) + (_e331.member.y * _e398)) + (_e331.member.z * _e401))));
                            let _e458 = (_e457 + _e410);
                            let _e459 = (_e445 * _e445);
                            let _e460 = (_e459 - _e458);
                            if (_e460 < 0f) {
                                phi_26011_ = type_57(0u, type_57().member_1);
                                phi_26012_ = type_57();
                                phi_26013_ = true;
                            } else {
                                if (_e445 < 0f) {
                                    phi_25983_ = -(sqrt(_e460));
                                } else {
                                    phi_25983_ = sqrt(_e460);
                                }
                                let _e467 = phi_25983_;
                                let _e469 = -((_e445 + _e467));
                                let _e470 = (_e469 == 0f);
                                if _e470 {
                                    phi_26006_ = type_57(1u, type_56(0f, 0f));
                                    phi_26007_ = type_57();
                                } else {
                                    let _e471 = (_e458 / _e469);
                                    if (_e469 < _e471) {
                                        phi_25995_ = type_56(_e469, _e471);
                                    } else {
                                        phi_25995_ = type_56(_e471, _e469);
                                    }
                                    let _e476 = phi_25995_;
                                    phi_26006_ = type_57();
                                    phi_26007_ = type_57(1u, _e476);
                                }
                                let _e479 = phi_26006_;
                                let _e481 = phi_26007_;
                                phi_26011_ = _e479;
                                phi_26012_ = _e481;
                                phi_26013_ = _e470;
                            }
                            let _e486 = phi_26011_;
                            let _e488 = phi_26012_;
                            let _e490 = phi_26013_;
                            if _e490 {
                                phi_26017_ = _e486;
                            } else {
                                phi_26017_ = _e488;
                            }
                            let _e492 = phi_26017_;
                            if (bitcast<i32>(_e492.member) != 0i) {
                                phi_25904_ = type_57(0u, _e492.member_1);
                            } else {
                                phi_25904_ = type_57(1u, type_57().member_1);
                            }
                            let _e502 = phi_25904_;
                            if (bitcast<i32>(_e502.member) != 0i) {
                                phi_25951_ = type_57(0u, type_57().member_1);
                            } else {
                                if (_e502.member_1.member_1 <= 0f) {
                                    phi_25947_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e502.member_1.member != _e502.member_1.member) {
                                        phi_26055_ = true;
                                    } else {
                                        phi_26055_ = (0f >= _e502.member_1.member);
                                    }
                                    let _e514 = phi_26055_;
                                    let _e515 = select(_e502.member_1.member, 0f, _e514);
                                    let _e516 = (_e457 + _e407);
                                    if (_e516 > 0f) {
                                        let _e518 = (_e459 - _e516);
                                        if (_e518 < 0f) {
                                            phi_26163_ = type_57(0u, type_57().member_1);
                                            phi_26164_ = type_57();
                                            phi_26165_ = true;
                                        } else {
                                            if (_e445 < 0f) {
                                                phi_26135_ = -(sqrt(_e518));
                                            } else {
                                                phi_26135_ = sqrt(_e518);
                                            }
                                            let _e525 = phi_26135_;
                                            let _e527 = -((_e445 + _e525));
                                            let _e528 = (_e527 == 0f);
                                            if _e528 {
                                                phi_26158_ = type_57(1u, type_56(0f, 0f));
                                                phi_26159_ = type_57();
                                            } else {
                                                let _e529 = (_e516 / _e527);
                                                if (_e527 < _e529) {
                                                    phi_26147_ = type_56(_e527, _e529);
                                                } else {
                                                    phi_26147_ = type_56(_e529, _e527);
                                                }
                                                let _e534 = phi_26147_;
                                                phi_26158_ = type_57();
                                                phi_26159_ = type_57(1u, _e534);
                                            }
                                            let _e537 = phi_26158_;
                                            let _e539 = phi_26159_;
                                            phi_26163_ = _e537;
                                            phi_26164_ = _e539;
                                            phi_26165_ = _e528;
                                        }
                                        let _e544 = phi_26163_;
                                        let _e546 = phi_26164_;
                                        let _e548 = phi_26165_;
                                        if _e548 {
                                            phi_26169_ = _e544;
                                        } else {
                                            phi_26169_ = _e546;
                                        }
                                        let _e550 = phi_26169_;
                                        if (bitcast<i32>(_e550.member) != 0i) {
                                            phi_26103_ = select(_e550.member_1.member_1, _e550.member_1.member, (_e550.member_1.member >= 0f));
                                        } else {
                                            phi_26103_ = -1f;
                                        }
                                        let _e561 = phi_26103_;
                                        if (_e561 >= 0f) {
                                            if (_e502.member_1.member_1 != _e502.member_1.member_1) {
                                                phi_26207_ = true;
                                            } else {
                                                phi_26207_ = (_e561 <= _e502.member_1.member_1);
                                            }
                                            let _e566 = phi_26207_;
                                            phi_25932_ = select(_e502.member_1.member_1, _e561, _e566);
                                        } else {
                                            phi_25932_ = _e502.member_1.member_1;
                                        }
                                        let _e569 = phi_25932_;
                                        phi_25934_ = _e569;
                                    } else {
                                        phi_25934_ = _e502.member_1.member_1;
                                    }
                                    let _e571 = phi_25934_;
                                    if (_e571 <= _e515) {
                                        phi_25943_ = type_57(0u, type_57().member_1);
                                    } else {
                                        phi_25943_ = type_57(1u, type_56(_e515, _e571));
                                    }
                                    let _e579 = phi_25943_;
                                    phi_25947_ = _e579;
                                }
                                let _e584 = phi_25947_;
                                phi_25951_ = _e584;
                            }
                            let _e589 = phi_25951_;
                            if (bitcast<i32>(_e589.member) != 0i) {
                                let _e595 = (_e589.member_1.member < _e331.member_5.x);
                                if _e595 {
                                    if (_e589.member_1.member_1 != _e589.member_1.member_1) {
                                        phi_26222_ = true;
                                    } else {
                                        phi_26222_ = (_e331.member_5.x <= _e589.member_1.member_1);
                                    }
                                    let _e601 = phi_26222_;
                                    let _e602 = select(_e589.member_1.member_1, _e331.member_5.x, _e601);
                                    phi_12709_ = vec3<f32>(0f, 0f, 0f);
                                    phi_12712_ = 0u;
                                    loop {
                                        let _e604 = phi_12709_;
                                        let _e606 = phi_12712_;
                                        local_69 = _e604;
                                        local_70 = _e604;
                                        local_71 = _e604;
                                        let _e607 = (_e606 < 32u);
                                        if _e607 {
                                            let _e608 = -(_e445);
                                            if (_e589.member_1.member <= _e602) {
                                            } else {
                                                phi_33872_ = true;
                                                break;
                                            }
                                            let _e611 = select(_e608, _e589.member_1.member, (_e608 < _e589.member_1.member));
                                            let _e613 = select(_e611, _e602, (_e611 > _e602));
                                            let _e614 = (_e613 - _e589.member_1.member);
                                            let _e615 = (_e602 - _e613);
                                            let _e616 = (_e602 - _e589.member_1.member);
                                            let _e617 = (0.000000001f * _e616);
                                            if (_e614 <= _e617) {
                                                let _e692 = (f32(_e606) * 0.03125f);
                                                let _e695 = (f32((_e606 + 1u)) * 0.03125f);
                                                let _e697 = ((_e692 + _e695) * 0.5f);
                                                phi_12813_ = type_56((_e589.member_1.member + ((_e616 * _e697) * _e697)), (_e616 * ((_e695 * _e695) - (_e692 * _e692))));
                                            } else {
                                                if (_e615 <= _e617) {
                                                    let _e675 = (f32((31u - _e606)) * 0.03125f);
                                                    let _e678 = (f32((32u - _e606)) * 0.03125f);
                                                    let _e680 = ((_e675 + _e678) * 0.5f);
                                                    phi_12812_ = type_56((_e602 - ((_e616 * _e680) * _e680)), (_e616 * ((_e678 * _e678) - (_e675 * _e675))));
                                                } else {
                                                    let _e623 = round(((32f * _e614) / (_e614 + _e615)));
                                                    let _e628 = select(select(u32(_e623), 0u, (_e623 < 0f)), 4294967295u, (_e623 > 4294967000f));
                                                    if (_e628 < 1u) {
                                                        phi_12796_ = 1u;
                                                    } else {
                                                        phi_12796_ = select(_e628, 31u, (_e628 > 31u));
                                                    }
                                                    let _e633 = phi_12796_;
                                                    if (_e606 < _e633) {
                                                        let _e654 = ((_e633 - 1u) - _e606);
                                                        let _e656 = f32(_e633);
                                                        let _e657 = (f32(_e654) / _e656);
                                                        let _e660 = (f32((_e654 + 1u)) / _e656);
                                                        let _e662 = ((_e657 + _e660) * 0.5f);
                                                        phi_12811_ = type_56((_e613 - ((_e614 * _e662) * _e662)), (_e614 * ((_e660 * _e660) - (_e657 * _e657))));
                                                    } else {
                                                        let _e636 = (_e606 - _e633);
                                                        let _e638 = f32((32u - _e633));
                                                        let _e639 = (f32(_e636) / _e638);
                                                        let _e642 = (f32((_e636 + 1u)) / _e638);
                                                        let _e644 = ((_e639 + _e642) * 0.5f);
                                                        phi_12811_ = type_56((_e613 + ((_e615 * _e644) * _e644)), (_e615 * ((_e642 * _e642) - (_e639 * _e639))));
                                                    }
                                                    let _e672 = phi_12811_;
                                                    phi_12812_ = _e672;
                                                }
                                                let _e690 = phi_12812_;
                                                phi_12813_ = _e690;
                                            }
                                            let _e707 = phi_12813_;
                                            let _e713 = (_e331.member.x + (_e331.member_1.x * _e707.member));
                                            let _e714 = (_e331.member.y + (_e331.member_1.y * _e707.member));
                                            let _e715 = (_e331.member.z + (_e331.member_1.z * _e707.member));
                                            let _e729 = (_e713 - _e395);
                                            let _e730 = (_e714 - _e398);
                                            let _e731 = (_e715 - _e401);
                                            let _e739 = ((((((_e713 * _e713) + (_e714 * _e714)) + (_e715 * _e715)) - (2f * (((_e713 * _e395) + (_e714 * _e398)) + (_e715 * _e401)))) + _e407) / (sqrt((((_e729 * _e729) + (_e730 * _e730)) + (_e731 * _e731))) + _e404));
                                            let _e743 = (1f - (abs((_e739 - 25000f)) * 0.00006666667f));
                                            if (_e743 != _e743) {
                                                phi_26421_ = true;
                                            } else {
                                                phi_26421_ = (0f >= _e743);
                                            }
                                            let _e747 = phi_26421_;
                                            let _e749 = -(_e739);
                                            phi_12710_ = (_e604 + vec3<f32>((exp((_e749 / _e413)) * _e707.member_1), (exp((_e749 / _e416)) * _e707.member_1), (select(_e743, 0f, _e747) * _e707.member_1)));
                                            phi_12713_ = (_e606 + 1u);
                                        } else {
                                            phi_12710_ = vec3<f32>();
                                            phi_12713_ = u32();
                                        }
                                        let _e761 = phi_12710_;
                                        let _e763 = phi_12713_;
                                        phi_12709_ = _e761;
                                        phi_12712_ = _e763;
                                        phi_33872_ = false;
                                        if !(_e607) {
                                            break;
                                        }
                                    }
                                    let _e766 = phi_33872_;
                                    if _e766 {
                                        break;
                                    }
                                    let _e768 = local_69;
                                    let _e775 = local_70;
                                    let _e777 = ((_e428 * 1.1f) * _e775.y);
                                    let _e782 = local_71;
                                    phi_36012_ = _e766;
                                    phi_12838_ = vec3<f32>(exp(-((((_e419 * _e768.x) + _e777) + (_e431 * _e782.z)))), exp(-((((_e422 * _e768.x) + _e777) + (_e434 * _e782.z)))), exp(-((((_e425 * _e768.x) + _e777) + (_e437 * _e782.z)))));
                                } else {
                                    phi_36012_ = false;
                                    phi_12838_ = vec3<f32>();
                                }
                                let _e798 = phi_36012_;
                                let _e800 = phi_12838_;
                                phi_36011_ = _e798;
                                phi_12840_ = _e800;
                                phi_12841_ = select(true, false, _e595);
                            } else {
                                phi_36011_ = false;
                                phi_12840_ = vec3<f32>();
                                phi_12841_ = true;
                            }
                            let _e803 = phi_36011_;
                            let _e805 = phi_12840_;
                            let _e807 = phi_12841_;
                            phi_36009_ = _e803;
                            phi_12848_ = (_e350 * select(_e805, vec3<f32>(1f, 1f, 1f), vec3(_e807)));
                            phi_12849_ = type_41(vec4<f32>(_e331.member.x, _e331.member.y, _e331.member.z, _e331.member_5.x), vec4<f32>(_e331.member_1.x, _e331.member_1.y, _e331.member_1.z, 0f), vec4<f32>(_e331.member_2.x, _e331.member_2.y, _e331.member_2.z, 0f));
                        } else {
                            phi_36009_ = false;
                            phi_12848_ = _e350;
                            phi_12849_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e812 = phi_36009_;
                        let _e814 = phi_12848_;
                        let _e816 = phi_12849_;
                        phi_36008_ = _e812;
                        phi_12850_ = _e814;
                        phi_12851_ = _e816;
                    }
                    let _e818 = phi_36008_;
                    let _e820 = phi_12850_;
                    let _e822 = phi_12851_;
                    let _e826 = (_e331.member.x + (_e331.member_1.x * _e331.member_5.x));
                    let _e827 = (_e331.member.y + (_e331.member_1.y * _e331.member_5.x));
                    let _e828 = (_e331.member.z + (_e331.member_1.z * _e331.member_5.x));
                    let _e831 = global_1.member.member_1[0u];
                    let _e834 = global_1.member.member_1[1u];
                    let _e835 = (_e831 + _e834);
                    let _e836 = (3u * _e331.member_4.z);
                    let _e838 = global_2.member[_e836];
                    let _e841 = global_2.member[(_e836 + 1u)];
                    let _e847 = select(select(u32(_e841.w), 0u, (_e841.w < 0f)), 4294967295u, (_e841.w > 4294967000f));
                    let _e848 = (3u * _e835);
                    let _e850 = (_e848 + (5u * _e331.member_4.z));
                    let _e852 = global_2.member[_e850];
                    let _e855 = global_2.member[(_e850 + 1u)];
                    let _e858 = global_2.member[(_e850 + 2u)];
                    let _e870 = global_2.member[(_e850 + 3u)];
                    let _e873 = global_2.member[(_e850 + 4u)];
                    let _e880 = (_e847 == 0u);
                    if _e880 {
                        let _e903 = (_e852.x - _e838.x);
                        let _e904 = (_e852.y - _e838.y);
                        let _e905 = (_e852.z - _e838.z);
                        let _e906 = (_e855.x - _e838.x);
                        let _e907 = (_e855.y - _e838.y);
                        let _e908 = (_e855.z - _e838.z);
                        let _e911 = ((_e904 * _e908) - (_e907 * _e905));
                        let _e914 = ((_e905 * _e906) - (_e908 * _e903));
                        let _e917 = ((_e903 * _e907) - (_e906 * _e904));
                        let _e924 = (1f / sqrt((((_e911 * _e911) + (_e914 * _e914)) + (_e917 * _e917))));
                        let _e928 = vec3<f32>((_e911 * _e924), (_e914 * _e924), (_e917 * _e924));
                        if (select(select(u32(_e858.w), 0u, (_e858.w < 0f)), 4294967295u, (_e858.w > 4294967000f)) == 1u) {
                            let _e933 = ((1f - _e331.member_5.y) - _e331.member_5.z);
                            let _e952 = (((_e858.x * _e933) + (_e870.x * _e331.member_5.y)) + (_e873.x * _e331.member_5.z));
                            let _e953 = (((_e858.y * _e933) + (_e870.y * _e331.member_5.y)) + (_e873.y * _e331.member_5.z));
                            let _e954 = (((_e858.z * _e933) + (_e870.z * _e331.member_5.y)) + (_e873.z * _e331.member_5.z));
                            let _e959 = (((_e952 * _e952) + (_e953 * _e953)) + (_e954 * _e954));
                            if (_e959 > 0f) {
                                let _e962 = (1f / sqrt(_e959));
                                phi_13097_ = vec3<f32>((_e952 * _e962), (_e953 * _e962), (_e954 * _e962));
                            } else {
                                phi_13097_ = _e928;
                            }
                            let _e968 = phi_13097_;
                            phi_13098_ = _e968;
                        } else {
                            phi_13098_ = _e928;
                        }
                        let _e970 = phi_13098_;
                        phi_13131_ = type_60(_e928, _e970);
                    } else {
                        if (_e847 == 1u) {
                            let _e896 = vec3<f32>(((_e826 - _e838.x) / _e838.w), ((_e827 - _e838.y) / _e838.w), ((_e828 - _e838.z) / _e838.w));
                            phi_13130_ = type_60(_e896, _e896);
                        } else {
                            let _e884 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e331.member_1.y > 0f)));
                            phi_13130_ = type_60(_e884, _e884);
                        }
                        let _e899 = phi_13130_;
                        phi_13131_ = _e899;
                    }
                    let _e973 = phi_13131_;
                    let _e987 = select(select(u32(_e855.w), 0u, (_e855.w < 0f)), 4294967295u, (_e855.w > 4294967000f));
                    phi_13254_ = 0i;
                    phi_13257_ = select(select(u32(_e852.w), 0u, (_e852.w < 0f)), 4294967295u, (_e852.w > 4294967000f));
                    loop {
                        let _e989 = phi_13254_;
                        let _e991 = phi_13257_;
                        local_72 = _e991;
                        if (_e989 < 4i) {
                            let _e994 = global_4.member[_e991];
                            let _e996 = (_e994.member != 6u);
                            if _e996 {
                                phi_13326_ = i32();
                                phi_13327_ = u32();
                            } else {
                                let _e1000 = (abs(_e826) / _e994.member_6.z);
                                let _e1011 = (abs(_e828) / _e994.member_6.w);
                                if ((select(select(u32(_e1000), 0u, (_e1000 < 0f)), 4294967295u, (_e1000 > 4294967000f)) % 2u) == 0u) {
                                    phi_13311_ = (_e826 > 0f);
                                } else {
                                    phi_13311_ = (_e826 <= 0f);
                                }
                                let _e1022 = phi_13311_;
                                if ((select(select(u32(_e1011), 0u, (_e1011 < 0f)), 4294967295u, (_e1011 > 4294967000f)) % 2u) == 0u) {
                                    phi_13317_ = (_e828 > 0f);
                                } else {
                                    phi_13317_ = (_e828 <= 0f);
                                }
                                let _e1026 = phi_13317_;
                                if (_e1022 != _e1026) {
                                    phi_13324_ = _e994.member_1;
                                } else {
                                    phi_13324_ = _e994.member_2;
                                }
                                let _e1031 = phi_13324_;
                                phi_13326_ = (_e989 + 1i);
                                phi_13327_ = _e1031;
                            }
                            let _e1034 = phi_13326_;
                            let _e1036 = phi_13327_;
                            phi_13255_ = _e1034;
                            phi_13258_ = _e1036;
                            phi_13329_ = select(true, false, _e996);
                        } else {
                            phi_13255_ = i32();
                            phi_13258_ = u32();
                            phi_13329_ = false;
                        }
                        let _e1039 = phi_13255_;
                        let _e1041 = phi_13258_;
                        let _e1043 = phi_13329_;
                        phi_13254_ = _e1039;
                        phi_13257_ = _e1041;
                        if !(_e1043) {
                            break;
                        }
                    }
                    let _e1046 = local_72;
                    let _e1048 = global_4.member[_e1046];
                    let _e1050 = (_e1048.member == 4u);
                    if _e1050 {
                        if _e1050 {
                            if (_e1048.member_1 == 1u) {
                                phi_13388_ = true;
                            } else {
                                phi_13388_ = ((((-(_e331.member_1.x) * _e973.member_1.x) + (-(_e331.member_1.y) * _e973.member_1.y)) + (-(_e331.member_1.z) * _e973.member_1.z)) > 0f);
                            }
                            let _e6366 = phi_13388_;
                            if _e6366 {
                                phi_13397_ = vec3<f32>(_e1048.member_4.x, _e1048.member_4.y, _e1048.member_4.z);
                            } else {
                                phi_13397_ = vec3<f32>();
                            }
                            let _e6373 = phi_13397_;
                            phi_13399_ = _e6373;
                            phi_13400_ = select(true, false, _e6366);
                        } else {
                            phi_13399_ = vec3<f32>();
                            phi_13400_ = true;
                        }
                        let _e6376 = phi_13399_;
                        let _e6378 = phi_13400_;
                        let _e6380 = select(_e6376, vec3<f32>(0f, 0f, 0f), vec3(_e6378));
                        if _e370 {
                            phi_13827_ = 1f;
                        } else {
                            if ((_e376 & 1u) == 0u) {
                                phi_13826_ = 1f;
                            } else {
                                if (_e987 > 0u) {
                                    let _e6387 = global_1.member.member_2[1u];
                                    let _e6388 = (_e6387 + (_e987 - 1u));
                                    let _e6391 = global_6.member[(_e6388 + 1u)];
                                    let _e6393 = global_6.member[_e6388];
                                    if (_e847 == 1u) {
                                        let _e6454 = (_e838.x - _e331.member.x);
                                        let _e6455 = (_e838.y - _e331.member.y);
                                        let _e6456 = (_e838.z - _e331.member.z);
                                        let _e6461 = (((_e6454 * _e6454) + (_e6455 * _e6455)) + (_e6456 * _e6456));
                                        let _e6462 = (_e838.w * _e838.w);
                                        if (_e6461 <= (_e6462 * 1.000001f)) {
                                            let _e6476 = (_e826 - _e331.member.x);
                                            let _e6477 = (_e827 - _e331.member.y);
                                            let _e6478 = (_e828 - _e331.member.z);
                                            let _e6483 = (((_e6476 * _e6476) + (_e6477 * _e6477)) + (_e6478 * _e6478));
                                            phi_32149_ = (_e6483 / (((abs((((_e973.member.x * _e6476) + (_e973.member.y * _e6477)) + (_e973.member.z * _e6478))) / sqrt(_e6483)) * 12.566371f) * _e6462));
                                        } else {
                                            let _e6465 = (_e6462 / _e6461);
                                            if (_e6465 != _e6465) {
                                                phi_32168_ = true;
                                            } else {
                                                phi_32168_ = (1f <= _e6465);
                                            }
                                            let _e6469 = phi_32168_;
                                            let _e6470 = select(_e6465, 1f, _e6469);
                                            phi_32149_ = (0.15915494f / (_e6470 / (1f + sqrt((1f - _e6470)))));
                                        }
                                        let _e6499 = phi_32149_;
                                        phi_13822_ = _e6499;
                                    } else {
                                        let _e6396 = (_e826 - _e331.member.x);
                                        let _e6397 = (_e827 - _e331.member.y);
                                        let _e6398 = (_e828 - _e331.member.z);
                                        let _e6403 = (((_e6396 * _e6396) + (_e6397 * _e6397)) + (_e6398 * _e6398));
                                        let _e6414 = (abs((((_e973.member.x * _e6396) + (_e973.member.y * _e6397)) + (_e973.member.z * _e6398))) / sqrt(_e6403));
                                        if (_e6414 <= 0f) {
                                            phi_13821_ = 0f;
                                        } else {
                                            if _e880 {
                                                let _e6422 = (_e852.x - _e838.x);
                                                let _e6423 = (_e852.y - _e838.y);
                                                let _e6424 = (_e852.z - _e838.z);
                                                let _e6425 = (_e855.x - _e838.x);
                                                let _e6426 = (_e855.y - _e838.y);
                                                let _e6427 = (_e855.z - _e838.z);
                                                let _e6430 = ((_e6423 * _e6427) - (_e6426 * _e6424));
                                                let _e6433 = ((_e6424 * _e6425) - (_e6427 * _e6422));
                                                let _e6436 = ((_e6422 * _e6426) - (_e6425 * _e6423));
                                                phi_13818_ = (sqrt((((_e6430 * _e6430) + (_e6433 * _e6433)) + (_e6436 * _e6436))) * 0.5f);
                                            } else {
                                                phi_13818_ = ((12.566371f * _e838.w) * _e838.w);
                                            }
                                            let _e6445 = phi_13818_;
                                            phi_13821_ = (_e6403 / (_e6414 * _e6445));
                                        }
                                        let _e6449 = phi_13821_;
                                        phi_13822_ = _e6449;
                                    }
                                    let _e6501 = phi_13822_;
                                    let _e6502 = ((_e6391 - _e6393) * _e6501);
                                    if (_e331.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_32189_ = 1f;
                                    } else {
                                        let _e6504 = (_e331.member_1.w * _e331.member_1.w);
                                        let _e6506 = (_e6504 + (_e6502 * _e6502));
                                        if (_e6506 == 0f) {
                                            phi_32186_ = 0f;
                                        } else {
                                            phi_32186_ = (_e6504 / _e6506);
                                        }
                                        let _e6510 = phi_32186_;
                                        phi_32189_ = _e6510;
                                    }
                                    let _e6512 = phi_32189_;
                                    phi_13825_ = _e6512;
                                } else {
                                    phi_13825_ = 1f;
                                }
                                let _e6514 = phi_13825_;
                                phi_13826_ = _e6514;
                            }
                            let _e6516 = phi_13826_;
                            phi_13827_ = _e6516;
                        }
                        let _e6518 = phi_13827_;
                        phi_19746_ = type_69(type_34(_e331.member, _e331.member_1, vec4<f32>(_e820.x, _e820.y, _e820.z, f32(_e356)), vec4<f32>((_e331.member_3.x + ((_e820.x * _e6380.x) * _e6518)), (_e331.member_3.y + ((_e820.y * _e6380.y) * _e6518)), (_e331.member_3.z + ((_e820.z * _e6380.z) * _e6518)), f32(_e382)), vec4<u32>(_e331.member_4.x, (_e331.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19747_ = type_69();
                        phi_19748_ = true;
                    } else {
                        let _e1051 = -(_e331.member_1.x);
                        let _e1052 = -(_e331.member_1.y);
                        let _e1053 = -(_e331.member_1.z);
                        if ((((_e973.member.x * _e1051) + (_e973.member.y * _e1052)) + (_e973.member.z * _e1053)) < 0f) {
                            phi_13863_ = -(_e973.member);
                        } else {
                            phi_13863_ = _e973.member;
                        }
                        let _e1065 = phi_13863_;
                        if ((((_e973.member_1.x * _e1051) + (_e973.member_1.y * _e1052)) + (_e973.member_1.z * _e1053)) < 0f) {
                            phi_13877_ = -(_e973.member_1);
                        } else {
                            phi_13877_ = _e973.member_1;
                        }
                        let _e1077 = phi_13877_;
                        let _e1080 = global_1.member.member_4[2u];
                        let _e1081 = abs(_e826);
                        let _e1082 = abs(_e827);
                        let _e1083 = abs(_e828);
                        let _e1085 = select(_e1082, _e1081, (_e1081 > _e1082));
                        let _e1089 = (0.00002f * (1f + select(_e1083, _e1085, (_e1085 > _e1083))));
                        if (_e1080 != _e1080) {
                            phi_26488_ = true;
                        } else {
                            phi_26488_ = (_e1089 >= _e1080);
                        }
                        let _e1093 = phi_26488_;
                        let _e1095 = (select(_e1080, _e1089, _e1093) * 2f);
                        let _e1102 = (_e826 + (_e1065.x * _e1095));
                        let _e1103 = (_e827 + (_e1065.y * _e1095));
                        let _e1104 = (_e828 + (_e1065.z * _e1095));
                        let _e1105 = vec3<f32>(_e1102, _e1103, _e1104);
                        if ((_e331.member_4.y & 4u) == 0u) {
                            phi_13910_ = 0f;
                        } else {
                            let _e1110 = global_1.member.member_4[0u];
                            phi_13910_ = _e1110;
                        }
                        let _e1112 = phi_13910_;
                        let _e1115 = select(-1f, 1f, (_e1077.z >= 0f));
                        let _e1117 = (-1f / (_e1115 + _e1077.z));
                        let _e1121 = ((_e1077.x * _e1077.y) * _e1117);
                        let _e1125 = (1f + (((_e1115 * _e1077.x) * _e1077.x) * _e1117));
                        let _e1126 = (_e1115 * _e1121);
                        let _e1128 = (-(_e1115) * _e1077.x);
                        let _e1131 = (_e1115 + ((_e1077.y * _e1077.y) * _e1117));
                        let _e1132 = -(_e1077.y);
                        if (_e1048.member == 0u) {
                            phi_34656_ = vec3<f32>(0f, 0f, 0f);
                            phi_34464_ = 1.5f;
                            phi_34385_ = vec3<f32>(0f, 0f, 0f);
                            phi_34308_ = vec3<f32>(_e1048.member_4.x, _e1048.member_4.y, _e1048.member_4.z);
                            phi_34229_ = 0u;
                            phi_34147_ = 1f;
                            phi_34070_ = 0u;
                            phi_34061_ = _e331.member_4.x;
                            phi_14588_ = type_69();
                            phi_14589_ = true;
                        } else {
                            if (_e1048.member == 1u) {
                                let _e1681 = (_e1048.member_6.y * _e1048.member_6.y);
                                if (_e1681 != _e1681) {
                                    phi_27180_ = true;
                                } else {
                                    phi_27180_ = (_e1112 >= _e1681);
                                }
                                let _e1685 = phi_27180_;
                                let _e1686 = select(_e1681, _e1112, _e1685);
                                let _e1687 = (_e1686 >= 0.001f);
                                if _e1687 {
                                    phi_34713_ = vec3<f32>(_e1048.member_4.x, _e1048.member_4.y, _e1048.member_4.z);
                                    phi_34507_ = _e1048.member_6.x;
                                    phi_34360_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34048_ = _e331.member_4.x;
                                    phi_13985_ = vec3<f32>();
                                    phi_13986_ = true;
                                } else {
                                    let _e1689 = ((_e331.member_4.x * 747796405u) + 2891336453u);
                                    let _e1697 = (((_e1689 >> bitcast<u32>((((_e1689 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1689) * 277803737u);
                                    let _e1700 = ((_e1697 >> bitcast<u32>(22i)) ^ _e1697);
                                    let _e1710 = abs((((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z)));
                                    let _e1713 = (1f / _e1048.member_6.x);
                                    let _e1717 = ((_e1713 * _e1713) * (1f - (_e1710 * _e1710)));
                                    if (_e1717 >= 1f) {
                                        phi_27220_ = 1f;
                                    } else {
                                        let _e1720 = sqrt((1f - _e1717));
                                        if (_e1710 != _e1710) {
                                            phi_27231_ = true;
                                        } else {
                                            phi_27231_ = (_e1720 <= _e1710);
                                        }
                                        let _e1724 = phi_27231_;
                                        let _e1729 = pow(((1f - _e1048.member_6.x) / (1f + _e1048.member_6.x)), 2f);
                                        phi_27220_ = (_e1729 + ((1f - _e1729) * pow((1f - select(_e1710, _e1720, _e1724)), 5f)));
                                    }
                                    let _e1736 = phi_27220_;
                                    let _e1737 = ((f32((_e1700 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1736);
                                    if _e1737 {
                                        let _e1750 = (((_e1077.x * _e331.member_1.x) + (_e1077.y * _e331.member_1.y)) + (_e1077.z * _e331.member_1.z));
                                        phi_34361_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13983_ = (_e345 - vec3<f32>(((_e1077.x * 2f) * _e1750), ((_e1077.y * 2f) * _e1750), ((_e1077.z * 2f) * _e1750)));
                                    } else {
                                        phi_34361_ = vec3<f32>(_e1048.member_4.x, _e1048.member_4.y, _e1048.member_4.z);
                                        phi_13983_ = vec3<f32>();
                                    }
                                    let _e1757 = phi_34361_;
                                    let _e1759 = phi_13983_;
                                    phi_34713_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34507_ = 1.5f;
                                    phi_34360_ = _e1757;
                                    phi_34048_ = _e1700;
                                    phi_13985_ = _e1759;
                                    phi_13986_ = select(true, false, _e1737);
                                }
                                let _e1769 = phi_34713_;
                                let _e1771 = phi_34507_;
                                let _e1773 = phi_34360_;
                                let _e1775 = phi_34048_;
                                let _e1777 = phi_13985_;
                                let _e1779 = phi_13986_;
                                phi_34659_ = _e1769;
                                phi_34467_ = _e1771;
                                phi_34388_ = vec3<f32>(0f, 0f, 0f);
                                phi_34311_ = _e1773;
                                phi_34232_ = select(0u, 1u, _e1687);
                                phi_34150_ = select(1f, _e1686, _e1687);
                                phi_34073_ = select(0u, 2u, _e1687);
                                phi_33999_ = _e1775;
                                phi_14458_ = _e1105;
                                phi_14459_ = _e1777;
                                phi_14460_ = vec3<f32>(1f, 1f, 1f);
                                phi_14461_ = _e1779;
                            } else {
                                if (_e1048.member == 3u) {
                                    phi_34002_ = _e331.member_4.x;
                                    phi_14137_ = vec3<f32>();
                                    phi_14138_ = vec3<f32>();
                                    phi_14139_ = vec3<f32>();
                                    phi_14140_ = true;
                                    phi_14141_ = false;
                                } else {
                                    let _e1136 = (_e1048.member == 5u);
                                    if _e1136 {
                                        phi_34003_ = _e331.member_4.x;
                                        phi_14133_ = vec3<f32>();
                                        phi_14134_ = vec3<f32>();
                                        phi_14135_ = vec3<f32>();
                                        phi_14136_ = false;
                                    } else {
                                        let _e1137 = (_e1048.member == 2u);
                                        if _e1137 {
                                            let _e1142 = (((_e331.member_1.x * _e973.member_1.x) + (_e331.member_1.y * _e973.member_1.y)) + (_e331.member_1.z * _e973.member_1.z));
                                            let _e1143 = (_e1142 > 0f);
                                            if _e1143 {
                                                phi_14018_ = -(_e973.member_1);
                                            } else {
                                                phi_14018_ = _e973.member_1;
                                            }
                                            let _e1146 = phi_14018_;
                                            if _e1143 {
                                                phi_14024_ = _e1048.member_6.x;
                                            } else {
                                                phi_14024_ = (1f / _e1048.member_6.x);
                                            }
                                            let _e1151 = phi_14024_;
                                            let _e1152 = abs(_e1142);
                                            let _e1153 = (_e1151 * _e1151);
                                            let _e1156 = (_e1153 * (1f - (_e1152 * _e1152)));
                                            if (_e1156 >= 1f) {
                                                phi_26550_ = 1f;
                                            } else {
                                                let _e1159 = sqrt((1f - _e1156));
                                                if (_e1152 != _e1152) {
                                                    phi_26561_ = true;
                                                } else {
                                                    phi_26561_ = (_e1159 <= _e1152);
                                                }
                                                let _e1163 = phi_26561_;
                                                let _e1168 = pow(((1f - _e1048.member_6.x) / (1f + _e1048.member_6.x)), 2f);
                                                phi_26550_ = (_e1168 + ((1f - _e1168) * pow((1f - select(_e1152, _e1159, _e1163)), 5f)));
                                            }
                                            let _e1175 = phi_26550_;
                                            let _e1183 = (((_e331.member_1.x * _e1146.x) + (_e331.member_1.y * _e1146.y)) + (_e331.member_1.z * _e1146.z));
                                            let _e1187 = (1f - (_e1153 * (1f - (_e1183 * _e1183))));
                                            if (_e1187 > 0f) {
                                                let _e1190 = ((_e331.member_4.x * 747796405u) + 2891336453u);
                                                let _e1198 = (((_e1190 >> bitcast<u32>((((_e1190 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1190) * 277803737u);
                                                let _e1201 = ((_e1198 >> bitcast<u32>(22i)) ^ _e1198);
                                                let _e1206 = ((f32((_e1201 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1175);
                                                if _e1206 {
                                                    let _e1216 = sqrt(_e1187);
                                                    let _e1220 = (((_e331.member_1.x - (_e1146.x * _e1183)) * _e1151) - (_e1146.x * _e1216));
                                                    let _e1221 = (((_e331.member_1.y - (_e1146.y * _e1183)) * _e1151) - (_e1146.y * _e1216));
                                                    let _e1222 = (((_e331.member_1.z - (_e1146.z * _e1183)) * _e1151) - (_e1146.z * _e1216));
                                                    let _e1229 = (1f / sqrt((((_e1220 * _e1220) + (_e1221 * _e1221)) + (_e1222 * _e1222))));
                                                    phi_14105_ = vec3<f32>((_e826 - (_e1146.x * _e1095)), (_e827 - (_e1146.y * _e1095)), (_e828 - (_e1146.z * _e1095)));
                                                    phi_14106_ = vec3<f32>((_e1220 * _e1229), (_e1221 * _e1229), (_e1222 * _e1229));
                                                    phi_14107_ = vec3<f32>(_e1048.member_4.x, _e1048.member_4.y, _e1048.member_4.z);
                                                } else {
                                                    phi_14105_ = vec3<f32>();
                                                    phi_14106_ = vec3<f32>();
                                                    phi_14107_ = vec3<f32>();
                                                }
                                                let _e1247 = phi_14105_;
                                                let _e1249 = phi_14106_;
                                                let _e1251 = phi_14107_;
                                                phi_34006_ = _e1201;
                                                phi_14109_ = _e1247;
                                                phi_14110_ = _e1249;
                                                phi_14111_ = _e1251;
                                                phi_14112_ = select(true, false, _e1206);
                                            } else {
                                                phi_34006_ = _e331.member_4.x;
                                                phi_14109_ = vec3<f32>();
                                                phi_14110_ = vec3<f32>();
                                                phi_14111_ = vec3<f32>();
                                                phi_14112_ = true;
                                            }
                                            let _e1254 = phi_34006_;
                                            let _e1256 = phi_14109_;
                                            let _e1258 = phi_14110_;
                                            let _e1260 = phi_14111_;
                                            let _e1262 = phi_14112_;
                                            if _e1262 {
                                                let _e1270 = (((_e973.member_1.x * _e331.member_1.x) + (_e973.member_1.y * _e331.member_1.y)) + (_e973.member_1.z * _e331.member_1.z));
                                                phi_14127_ = vec3<f32>((_e826 + (_e1146.x * _e1095)), (_e827 + (_e1146.y * _e1095)), (_e828 + (_e1146.z * _e1095)));
                                                phi_14128_ = (_e345 - vec3<f32>(((_e973.member_1.x * 2f) * _e1270), ((_e973.member_1.y * 2f) * _e1270), ((_e973.member_1.z * 2f) * _e1270)));
                                            } else {
                                                phi_14127_ = _e1256;
                                                phi_14128_ = _e1258;
                                            }
                                            let _e1284 = phi_14127_;
                                            let _e1286 = phi_14128_;
                                            phi_34004_ = _e1254;
                                            phi_14130_ = _e1284;
                                            phi_14131_ = _e1286;
                                            phi_14132_ = select(_e1260, vec3<f32>(1f, 1f, 1f), vec3(_e1262));
                                        } else {
                                            phi_34004_ = _e331.member_4.x;
                                            phi_14130_ = vec3<f32>();
                                            phi_14131_ = vec3<f32>();
                                            phi_14132_ = vec3<f32>();
                                        }
                                        let _e1290 = phi_34004_;
                                        let _e1292 = phi_14130_;
                                        let _e1294 = phi_14131_;
                                        let _e1296 = phi_14132_;
                                        phi_34003_ = _e1290;
                                        phi_14133_ = _e1292;
                                        phi_14134_ = _e1294;
                                        phi_14135_ = _e1296;
                                        phi_14136_ = _e1137;
                                    }
                                    let _e1298 = phi_34003_;
                                    let _e1300 = phi_14133_;
                                    let _e1302 = phi_14134_;
                                    let _e1304 = phi_14135_;
                                    let _e1306 = phi_14136_;
                                    phi_34002_ = _e1298;
                                    phi_14137_ = _e1300;
                                    phi_14138_ = _e1302;
                                    phi_14139_ = _e1304;
                                    phi_14140_ = _e1136;
                                    phi_14141_ = _e1306;
                                }
                                let _e1308 = phi_34002_;
                                let _e1310 = phi_14137_;
                                let _e1312 = phi_14138_;
                                let _e1314 = phi_14139_;
                                let _e1316 = phi_14140_;
                                let _e1318 = phi_14141_;
                                if _e1316 {
                                    let _e1321 = (_e1048.member_6.y * _e1048.member_6.y);
                                    if (_e1321 != _e1321) {
                                        phi_26634_ = true;
                                    } else {
                                        phi_26634_ = (_e1112 >= _e1321);
                                    }
                                    let _e1325 = phi_26634_;
                                    let _e1326 = select(_e1321, _e1112, _e1325);
                                    let _e1338 = select(2u, 0u, (_e1048.member == 5u));
                                    let _e1339 = (_e1326 < 0.001f);
                                    if _e1339 {
                                        let _e1347 = (((_e1077.x * _e331.member_1.x) + (_e1077.y * _e331.member_1.y)) + (_e1077.z * _e331.member_1.z));
                                        let _e1358 = abs((((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z)));
                                        if (_e1338 == 0u) {
                                            let _e1640 = (1f - _e1358);
                                            if (_e1640 != _e1640) {
                                                phi_27165_ = true;
                                            } else {
                                                phi_27165_ = (0f >= _e1640);
                                            }
                                            let _e1644 = phi_27165_;
                                            let _e1646 = pow(select(_e1640, 0f, _e1644), 5f);
                                            let _e1647 = (1f - _e1646);
                                            phi_14444_ = vec3<f32>(((_e1048.member_4.x * _e1647) + _e1646), ((_e1048.member_4.y * _e1647) + _e1646), ((_e1048.member_4.z * _e1647) + _e1646));
                                        } else {
                                            if (_e1338 == 1u) {
                                                let _e1622 = (0.44444448f * (1f - (_e1358 * _e1358)));
                                                if (_e1622 >= 1f) {
                                                    phi_27126_ = 1f;
                                                } else {
                                                    let _e1625 = sqrt((1f - _e1622));
                                                    if (_e1358 != _e1358) {
                                                        phi_27137_ = true;
                                                    } else {
                                                        phi_27137_ = (_e1625 <= _e1358);
                                                    }
                                                    let _e1629 = phi_27137_;
                                                    phi_27126_ = (0.040000003f + (0.96f * pow((1f - select(_e1358, _e1625, _e1629)), 5f)));
                                                }
                                                let _e1636 = phi_27126_;
                                                phi_14443_ = vec3(_e1636);
                                            } else {
                                                let _e1362 = select(_e1358, 0f, (_e1358 < 0f));
                                                let _e1364 = select(_e1362, 1f, (_e1362 > 1f));
                                                let _e1366 = (1f - (_e1364 * _e1364));
                                                let _e1373 = ((_e1048.member_4.x * _e1048.member_4.x) - (_e1048.member_5.x * _e1048.member_5.x));
                                                let _e1374 = ((_e1048.member_4.y * _e1048.member_4.y) - (_e1048.member_5.y * _e1048.member_5.y));
                                                let _e1375 = ((_e1048.member_4.z * _e1048.member_4.z) - (_e1048.member_5.z * _e1048.member_5.z));
                                                let _e1377 = (_e1048.member_4.x * (_e1048.member_5.x + _e1048.member_5.x));
                                                let _e1379 = (_e1048.member_4.y * (_e1048.member_5.y + _e1048.member_5.y));
                                                let _e1381 = (_e1048.member_4.z * (_e1048.member_5.z + _e1048.member_5.z));
                                                let _e1388 = ((_e1373 * _e1373) + (_e1377 * _e1377));
                                                let _e1389 = ((_e1374 * _e1374) + (_e1379 * _e1379));
                                                let _e1390 = ((_e1375 * _e1375) + (_e1381 * _e1381));
                                                let _e1395 = ((_e1366 * _e1373) / _e1388);
                                                let _e1396 = ((_e1366 * _e1374) / _e1389);
                                                let _e1397 = ((_e1366 * _e1375) / _e1390);
                                                let _e1398 = (1f - _e1395);
                                                let _e1399 = (1f - _e1396);
                                                let _e1400 = (1f - _e1397);
                                                let _e1402 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1395, _e1396, _e1397));
                                                let _e1403 = (_e1366 * _e1377);
                                                let _e1404 = (_e1366 * _e1379);
                                                let _e1405 = (_e1366 * _e1381);
                                                let _e1410 = (vec3<f32>(_e1403, _e1404, _e1405) / vec3<f32>(_e1388, _e1389, _e1390));
                                                let _e1423 = ((_e1402.x * _e1402.x) + (_e1410.x * _e1410.x));
                                                let _e1424 = ((_e1402.y * _e1402.y) + (_e1410.y * _e1410.y));
                                                let _e1425 = ((_e1402.z * _e1402.z) + (_e1410.z * _e1410.z));
                                                if (_e1423 != _e1423) {
                                                    phi_26796_ = true;
                                                } else {
                                                    phi_26796_ = (0f >= _e1423);
                                                }
                                                let _e1429 = phi_26796_;
                                                let _e1431 = sqrt(select(_e1423, 0f, _e1429));
                                                if (_e1424 != _e1424) {
                                                    phi_26811_ = true;
                                                } else {
                                                    phi_26811_ = (0f >= _e1424);
                                                }
                                                let _e1435 = phi_26811_;
                                                let _e1437 = sqrt(select(_e1424, 0f, _e1435));
                                                if (_e1425 != _e1425) {
                                                    phi_26826_ = true;
                                                } else {
                                                    phi_26826_ = (0f >= _e1425);
                                                }
                                                let _e1441 = phi_26826_;
                                                let _e1443 = sqrt(select(_e1425, 0f, _e1441));
                                                let _e1447 = ((_e1431 + _e1398) * 0.5f);
                                                let _e1448 = ((_e1437 + _e1399) * 0.5f);
                                                let _e1449 = ((_e1443 + _e1400) * 0.5f);
                                                if (_e1447 != _e1447) {
                                                    phi_26853_ = true;
                                                } else {
                                                    phi_26853_ = (0f >= _e1447);
                                                }
                                                let _e1453 = phi_26853_;
                                                let _e1455 = sqrt(select(_e1447, 0f, _e1453));
                                                if (_e1448 != _e1448) {
                                                    phi_26868_ = true;
                                                } else {
                                                    phi_26868_ = (0f >= _e1448);
                                                }
                                                let _e1459 = phi_26868_;
                                                let _e1461 = sqrt(select(_e1448, 0f, _e1459));
                                                if (_e1449 != _e1449) {
                                                    phi_26883_ = true;
                                                } else {
                                                    phi_26883_ = (0f >= _e1449);
                                                }
                                                let _e1465 = phi_26883_;
                                                let _e1467 = sqrt(select(_e1449, 0f, _e1465));
                                                let _e1468 = vec3<f32>(_e1455, _e1461, _e1467);
                                                let _e1472 = ((_e1431 - _e1398) * 0.5f);
                                                let _e1473 = ((_e1437 - _e1399) * 0.5f);
                                                let _e1474 = ((_e1443 - _e1400) * 0.5f);
                                                if (_e1472 != _e1472) {
                                                    phi_26910_ = true;
                                                } else {
                                                    phi_26910_ = (0f >= _e1472);
                                                }
                                                let _e1478 = phi_26910_;
                                                let _e1480 = sqrt(select(_e1472, 0f, _e1478));
                                                if (_e1473 != _e1473) {
                                                    phi_26925_ = true;
                                                } else {
                                                    phi_26925_ = (0f >= _e1473);
                                                }
                                                let _e1484 = phi_26925_;
                                                let _e1486 = sqrt(select(_e1473, 0f, _e1484));
                                                if (_e1474 != _e1474) {
                                                    phi_26940_ = true;
                                                } else {
                                                    phi_26940_ = (0f >= _e1474);
                                                }
                                                let _e1490 = phi_26940_;
                                                let _e1492 = sqrt(select(_e1474, 0f, _e1490));
                                                if ((_e1403 / _e1388) < 0f) {
                                                    phi_14320_ = -(_e1480);
                                                } else {
                                                    phi_14320_ = _e1480;
                                                }
                                                let _e1496 = phi_14320_;
                                                if ((_e1404 / _e1389) < 0f) {
                                                    phi_14328_ = -(_e1486);
                                                } else {
                                                    phi_14328_ = _e1486;
                                                }
                                                let _e1500 = phi_14328_;
                                                if ((_e1405 / _e1390) < 0f) {
                                                    phi_14336_ = -(_e1492);
                                                } else {
                                                    phi_14336_ = _e1492;
                                                }
                                                let _e1504 = phi_14336_;
                                                let _e1508 = (_e1048.member_5.x * _e1364);
                                                let _e1509 = (_e1048.member_5.y * _e1364);
                                                let _e1510 = (_e1048.member_5.z * _e1364);
                                                let _e1520 = vec3<f32>(((_e1048.member_4.x * _e1455) - (_e1048.member_5.x * _e1496)), ((_e1048.member_4.y * _e1461) - (_e1048.member_5.y * _e1500)), ((_e1048.member_4.z * _e1467) - (_e1048.member_5.z * _e1504)));
                                                let _e1527 = ((_e1048.member_4.x * _e1496) + (_e1048.member_5.x * _e1455));
                                                let _e1528 = ((_e1048.member_4.y * _e1500) + (_e1048.member_5.y * _e1461));
                                                let _e1529 = ((_e1048.member_4.z * _e1504) + (_e1048.member_5.z * _e1467));
                                                let _e1531 = vec3<f32>((_e1048.member_4.x * _e1364), (_e1048.member_4.y * _e1364), (_e1048.member_4.z * _e1364));
                                                let _e1532 = (_e1531 - _e1468);
                                                let _e1533 = (_e1508 - _e1496);
                                                let _e1534 = (_e1509 - _e1500);
                                                let _e1535 = (_e1510 - _e1504);
                                                let _e1548 = (_e1531 + _e1468);
                                                let _e1549 = (_e1508 + _e1496);
                                                let _e1550 = (_e1509 + _e1500);
                                                let _e1551 = (_e1510 + _e1504);
                                                let _e1561 = ((_e1548.x * _e1548.x) + (_e1549 * _e1549));
                                                let _e1562 = ((_e1548.y * _e1548.y) + (_e1550 * _e1550));
                                                let _e1563 = ((_e1548.z * _e1548.z) + (_e1551 * _e1551));
                                                let _e1573 = vec3(_e1364);
                                                let _e1574 = (_e1573 - _e1520);
                                                let _e1575 = -(vec3<f32>(_e1527, _e1528, _e1529));
                                                let _e1591 = (_e1573 + _e1520);
                                                let _e1601 = ((_e1591.x * _e1591.x) + (_e1527 * _e1527));
                                                let _e1602 = ((_e1591.y * _e1591.y) + (_e1528 * _e1528));
                                                let _e1603 = ((_e1591.z * _e1591.z) + (_e1529 * _e1529));
                                                phi_14443_ = vec3<f32>((((((_e1532.x * _e1532.x) + (_e1533 * _e1533)) / select(0.000000000000000000000000000001f, _e1561, (_e1561 > 0.000000000000000000000000000001f))) + (((_e1574.x * _e1574.x) + (_e1575.x * _e1575.x)) / select(0.000000000000000000000000000001f, _e1601, (_e1601 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1532.y * _e1532.y) + (_e1534 * _e1534)) / select(0.000000000000000000000000000001f, _e1562, (_e1562 > 0.000000000000000000000000000001f))) + (((_e1574.y * _e1574.y) + (_e1575.y * _e1575.y)) / select(0.000000000000000000000000000001f, _e1602, (_e1602 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1532.z * _e1532.z) + (_e1535 * _e1535)) / select(0.000000000000000000000000000001f, _e1563, (_e1563 > 0.000000000000000000000000000001f))) + (((_e1574.z * _e1574.z) + (_e1575.z * _e1575.z)) / select(0.000000000000000000000000000001f, _e1603, (_e1603 > 0.000000000000000000000000000001f)))) * 0.5f));
                                            }
                                            let _e1639 = phi_14443_;
                                            phi_14444_ = _e1639;
                                        }
                                        let _e1656 = phi_14444_;
                                        phi_14445_ = (_e345 - vec3<f32>(((_e1077.x * 2f) * _e1347), ((_e1077.y * 2f) * _e1347), ((_e1077.z * 2f) * _e1347)));
                                        phi_14446_ = _e1656;
                                    } else {
                                        phi_14445_ = vec3<f32>();
                                        phi_14446_ = vec3<f32>();
                                    }
                                    let _e1658 = phi_14445_;
                                    let _e1660 = phi_14446_;
                                    phi_34390_ = vec3<f32>(_e1048.member_5.x, _e1048.member_5.y, _e1048.member_5.z);
                                    phi_34313_ = vec3<f32>(_e1048.member_4.x, _e1048.member_4.y, _e1048.member_4.z);
                                    phi_34234_ = _e1338;
                                    phi_34152_ = _e1326;
                                    phi_14448_ = _e1658;
                                    phi_14449_ = _e1660;
                                    phi_14450_ = _e1339;
                                } else {
                                    phi_34390_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34313_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34234_ = 0u;
                                    phi_34152_ = 1f;
                                    phi_14448_ = _e1312;
                                    phi_14449_ = _e1314;
                                    phi_14450_ = _e1318;
                                }
                                let _e1662 = phi_34390_;
                                let _e1664 = phi_34313_;
                                let _e1666 = phi_34234_;
                                let _e1668 = phi_34152_;
                                let _e1670 = phi_14448_;
                                let _e1672 = phi_14449_;
                                let _e1674 = phi_14450_;
                                phi_34659_ = vec3<f32>(0f, 0f, 0f);
                                phi_34467_ = 1.5f;
                                phi_34388_ = _e1662;
                                phi_34311_ = _e1664;
                                phi_34232_ = _e1666;
                                phi_34150_ = _e1668;
                                phi_34073_ = select(0u, 1u, _e1316);
                                phi_33999_ = _e1308;
                                phi_14458_ = select(_e1310, _e1105, vec3(_e1316));
                                phi_14459_ = _e1670;
                                phi_14460_ = _e1672;
                                phi_14461_ = select(true, false, _e1674);
                            }
                            let _e1784 = phi_34659_;
                            let _e1786 = phi_34467_;
                            let _e1788 = phi_34388_;
                            let _e1790 = phi_34311_;
                            let _e1792 = phi_34232_;
                            let _e1794 = phi_34150_;
                            let _e1796 = phi_34073_;
                            let _e1798 = phi_33999_;
                            let _e1800 = phi_14458_;
                            let _e1802 = phi_14459_;
                            let _e1804 = phi_14460_;
                            let _e1806 = phi_14461_;
                            let _e1807 = select(true, false, _e1806);
                            if _e1807 {
                                if (_e356 >= _e373) {
                                    phi_34058_ = _e1798;
                                    phi_14577_ = _e820;
                                    phi_14578_ = type_69();
                                    phi_14579_ = true;
                                } else {
                                    let _e1820 = global_1.member.member_4[1u];
                                    if (sqrt((((_e1804.x * _e1804.x) + (_e1804.y * _e1804.y)) + (_e1804.z * _e1804.z))) <= _e1820) {
                                        phi_34053_ = _e1798;
                                        phi_14541_ = _e820;
                                        phi_14542_ = false;
                                    } else {
                                        let _e1824 = global_1.member.member_7[1u];
                                        if (_e1824 == 1u) {
                                            let _e1829 = global_1.member.member_4[3u];
                                            if (f32(_e356) >= _e1829) {
                                                let _e1832 = select(_e1804.y, _e1804.x, (_e1804.x > _e1804.y));
                                                let _e1834 = select(_e1804.z, _e1832, (_e1832 > _e1804.z));
                                                if (_e1834 != _e1834) {
                                                    phi_27297_ = true;
                                                } else {
                                                    phi_27297_ = (1f <= _e1834);
                                                }
                                                let _e1838 = phi_27297_;
                                                let _e1839 = select(_e1834, 1f, _e1838);
                                                let _e1841 = ((_e1798 * 747796405u) + 2891336453u);
                                                let _e1849 = (((_e1841 >> bitcast<u32>((((_e1841 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1841) * 277803737u);
                                                let _e1852 = ((_e1849 >> bitcast<u32>(22i)) ^ _e1849);
                                                let _e1857 = ((f32((_e1852 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1839);
                                                if _e1857 {
                                                    phi_14530_ = vec3<f32>();
                                                } else {
                                                    phi_14530_ = vec3<f32>((_e820.x / _e1839), (_e820.y / _e1839), (_e820.z / _e1839));
                                                }
                                                let _e1866 = phi_14530_;
                                                phi_34056_ = _e1852;
                                                phi_14532_ = _e1866;
                                                phi_14533_ = select(true, false, _e1857);
                                            } else {
                                                phi_34056_ = _e1798;
                                                phi_14532_ = _e820;
                                                phi_14533_ = true;
                                            }
                                            let _e1869 = phi_34056_;
                                            let _e1871 = phi_14532_;
                                            let _e1873 = phi_14533_;
                                            phi_34055_ = _e1869;
                                            phi_14534_ = _e1871;
                                            phi_14535_ = _e1873;
                                        } else {
                                            phi_34055_ = _e1798;
                                            phi_14534_ = _e820;
                                            phi_14535_ = true;
                                        }
                                        let _e1875 = phi_34055_;
                                        let _e1877 = phi_14534_;
                                        let _e1879 = phi_14535_;
                                        phi_34053_ = _e1875;
                                        phi_14541_ = select(_e820, _e1877, vec3(_e1879));
                                        phi_14542_ = select(false, true, _e1879);
                                    }
                                    let _e1884 = phi_34053_;
                                    let _e1886 = phi_14541_;
                                    let _e1888 = phi_14542_;
                                    if _e1888 {
                                        let _e1889 = (_e1886 * _e1804);
                                        let _e1900 = (_e1802 * (1f / sqrt((((_e1802.x * _e1802.x) + (_e1802.y * _e1802.y)) + (_e1802.z * _e1802.z)))));
                                        phi_14575_ = type_69(type_34(vec4<f32>(_e1800.x, _e1800.y, _e1800.z, _e331.member.w), vec4<f32>(_e1900.x, _e1900.y, _e1900.z, _e331.member_1.w), vec4<f32>(_e1889.x, _e1889.y, _e1889.z, f32((_e356 + 1u))), vec4<f32>(_e331.member_3.x, _e331.member_3.y, _e331.member_3.z, f32(_e382)), vec4<u32>(_e1884, (_e331.member_4.y | 2u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14575_ = type_69();
                                    }
                                    let _e1923 = phi_14575_;
                                    phi_34058_ = _e1884;
                                    phi_14577_ = _e1886;
                                    phi_14578_ = _e1923;
                                    phi_14579_ = select(true, false, _e1888);
                                }
                                let _e1926 = phi_34058_;
                                let _e1928 = phi_14577_;
                                let _e1930 = phi_14578_;
                                let _e1932 = phi_14579_;
                                if _e1932 {
                                    phi_14585_ = type_69(type_34(_e331.member, _e331.member_1, vec4<f32>(_e1928.x, _e1928.y, _e1928.z, f32(_e356)), vec4<f32>(_e331.member_3.x, _e331.member_3.y, _e331.member_3.z, f32(_e382)), vec4<u32>(_e1926, (_e331.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                } else {
                                    phi_14585_ = _e1930;
                                }
                                let _e1945 = phi_14585_;
                                phi_34062_ = _e1926;
                                phi_14586_ = _e1945;
                            } else {
                                phi_34062_ = _e1798;
                                phi_14586_ = type_69();
                            }
                            let _e1947 = phi_34062_;
                            let _e1949 = phi_14586_;
                            phi_34656_ = _e1784;
                            phi_34464_ = _e1786;
                            phi_34385_ = _e1788;
                            phi_34308_ = _e1790;
                            phi_34229_ = _e1792;
                            phi_34147_ = _e1794;
                            phi_34070_ = _e1796;
                            phi_34061_ = _e1947;
                            phi_14588_ = _e1949;
                            phi_14589_ = select(true, false, _e1807);
                        }
                        let _e1957 = phi_34656_;
                        let _e1959 = phi_34464_;
                        let _e1961 = phi_34385_;
                        let _e1963 = phi_34308_;
                        let _e1965 = phi_34229_;
                        let _e1967 = phi_34147_;
                        let _e1969 = phi_34070_;
                        let _e1971 = phi_34061_;
                        let _e1973 = phi_14588_;
                        let _e1975 = phi_14589_;
                        if _e1975 {
                            let _e1977 = ((_e331.member_4.y | 4u) & 4294967293u);
                            let _e1980 = global_1.member.member_1[3u];
                            phi_35985_ = _e818;
                            phi_34060_ = _e1971;
                            phi_14600_ = _e360;
                            phi_14603_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_14605_ = 0u;
                            phi_14607_ = _e382;
                            loop {
                                let _e1982 = phi_35985_;
                                let _e1984 = phi_34060_;
                                let _e1986 = phi_14600_;
                                let _e1988 = phi_14603_;
                                let _e1990 = phi_14605_;
                                let _e1992 = phi_14607_;
                                local_83 = _e1992;
                                local_84 = _e1986;
                                local_85 = _e1986;
                                local_86 = _e1986;
                                local_87 = _e1988;
                                local_88 = _e1992;
                                local_89 = _e1986;
                                local_90 = _e1986;
                                local_91 = _e1986;
                                local_92 = _e1988;
                                local_93 = _e1992;
                                local_94 = _e1986;
                                local_95 = _e1986;
                                local_96 = _e1986;
                                local_97 = _e1988;
                                local_98 = _e1992;
                                local_99 = _e1986;
                                local_100 = _e1986;
                                local_101 = _e1986;
                                local_102 = _e1988;
                                let _e1994 = (_e1990 < (_e1980 + 3u));
                                if _e1994 {
                                    if (_e1990 < _e1980) {
                                        let _e3991 = global_5.member[_e1990];
                                        let _e3996 = (_e3991.member.x - _e1102);
                                        let _e3997 = (_e3991.member.y - _e1103);
                                        let _e3998 = (_e3991.member.z - _e1104);
                                        let _e4003 = (((_e3996 * _e3996) + (_e3997 * _e3997)) + (_e3998 * _e3998));
                                        let _e4004 = sqrt(_e4003);
                                        let _e4005 = (_e3996 / _e4004);
                                        let _e4006 = (_e3997 / _e4004);
                                        let _e4007 = (_e3998 / _e4004);
                                        if (_e3991.member.w < 0.5f) {
                                            phi_14704_ = 1f;
                                        } else {
                                            let _e4031 = (((_e3991.member_2.x * -(_e4005)) + (_e3991.member_2.y * -(_e4006))) + (_e3991.member_2.z * -(_e4007)));
                                            if (_e3991.member_3.x == _e3991.member_2.w) {
                                                phi_14703_ = select(1f, 0f, (_e4031 < _e3991.member_3.x));
                                            } else {
                                                let _e4035 = ((_e4031 - _e3991.member_3.x) / (_e3991.member_2.w - _e3991.member_3.x));
                                                let _e4037 = select(_e4035, 0f, (_e4035 < 0f));
                                                let _e4039 = select(_e4037, 1f, (_e4037 > 1f));
                                                phi_14703_ = ((_e4039 * _e4039) * (3f - (2f * _e4039)));
                                            }
                                            let _e4047 = phi_14703_;
                                            phi_14704_ = _e4047;
                                        }
                                        let _e4049 = phi_14704_;
                                        phi_36402_ = _e1984;
                                        phi_18117_ = 0f;
                                        phi_18118_ = vec3<f32>(_e4005, _e4006, _e4007);
                                        phi_18119_ = vec3<f32>(((_e3991.member_1.x * _e4049) / _e4003), ((_e3991.member_1.y * _e4049) / _e4003), ((_e3991.member_1.z * _e4049) / _e4003));
                                        phi_18120_ = _e4004;
                                    } else {
                                        if (_e1990 == _e1980) {
                                            if ((_e376 & 4u) == 0u) {
                                                phi_36446_ = _e1984;
                                                phi_16746_ = 0f;
                                                phi_16747_ = vec3<f32>(0f, 0f, 0f);
                                                phi_16748_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                let _e2743 = global_1.member.member_5;
                                                let _e2751 = global_1.member.member_6[3u];
                                                let _e2753 = sin((_e2751 * 0.5f));
                                                let _e2755 = ((12.566371f * _e2753) * _e2753);
                                                let _e2760 = (((_e2743.x * _e1077.x) + (_e2743.y * _e1077.y)) + (_e2743.z * _e1077.z));
                                                if (_e2760 != _e2760) {
                                                    phi_27836_ = true;
                                                } else {
                                                    phi_27836_ = (0f >= _e2760);
                                                }
                                                let _e2764 = phi_27836_;
                                                let _e2766 = (select(_e2760, 0f, _e2764) * 0.31830987f);
                                                if (_e1969 == 0u) {
                                                    if (_e2760 <= 0f) {
                                                        phi_28633_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_28633_ = (_e1963 * (_e2760 * 0.31830987f));
                                                    }
                                                    let _e3242 = phi_28633_;
                                                    phi_15114_ = type_59(_e3242, _e2766);
                                                    phi_15115_ = type_59();
                                                    phi_15116_ = true;
                                                } else {
                                                    let _e2772 = (((_e1051 * _e1125) + (_e1052 * _e1126)) + (_e1053 * _e1128));
                                                    let _e2777 = (((_e1051 * _e1121) + (_e1052 * _e1131)) + (_e1053 * _e1132));
                                                    let _e2782 = (((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z));
                                                    let _e2787 = (((_e2743.x * _e1125) + (_e2743.y * _e1126)) + (_e2743.z * _e1128));
                                                    let _e2792 = (((_e2743.x * _e1121) + (_e2743.y * _e1131)) + (_e2743.z * _e1132));
                                                    if (_e2782 <= 0f) {
                                                        phi_15078_ = type_59();
                                                        phi_15079_ = true;
                                                    } else {
                                                        let _e2794 = (_e2760 <= 0f);
                                                        if _e2794 {
                                                            phi_15077_ = type_59();
                                                        } else {
                                                            let _e2795 = (_e2772 + _e2787);
                                                            let _e2796 = (_e2777 + _e2792);
                                                            let _e2797 = (_e2782 + _e2760);
                                                            let _e2804 = (1f / sqrt((((_e2795 * _e2795) + (_e2796 * _e2796)) + (_e2797 * _e2797))));
                                                            let _e2807 = (_e2797 * _e2804);
                                                            if (_e2807 <= 0f) {
                                                                phi_27946_ = 0f;
                                                            } else {
                                                                let _e2809 = (_e1967 * _e1967);
                                                                let _e2813 = ((((_e2809 - 1f) * _e2807) * _e2807) + 1f);
                                                                phi_27946_ = (_e2809 / ((3.1415927f * _e2813) * _e2813));
                                                            }
                                                            let _e2818 = phi_27946_;
                                                            let _e2819 = (_e2782 * _e2782);
                                                            if (_e2819 >= 1f) {
                                                                phi_27974_ = 0f;
                                                            } else {
                                                                phi_27974_ = ((sqrt((1f + (((_e1967 * _e1967) * ((_e2772 * _e2772) + (_e2777 * _e2777))) / _e2819))) - 1f) * 0.5f);
                                                            }
                                                            let _e2832 = phi_27974_;
                                                            let _e2833 = (1f + _e2832);
                                                            let _e2834 = (_e2760 * _e2760);
                                                            if (_e2834 >= 1f) {
                                                                phi_28002_ = 0f;
                                                            } else {
                                                                phi_28002_ = ((sqrt((1f + (((_e1967 * _e1967) * ((_e2787 * _e2787) + (_e2792 * _e2792))) / _e2834))) - 1f) * 0.5f);
                                                            }
                                                            let _e2847 = phi_28002_;
                                                            let _e2854 = (((_e2772 * (_e2795 * _e2804)) + (_e2777 * (_e2796 * _e2804))) + (_e2782 * _e2807));
                                                            if (_e1965 == 0u) {
                                                                let _e3149 = (1f - _e2854);
                                                                if (_e3149 != _e3149) {
                                                                    phi_28505_ = true;
                                                                } else {
                                                                    phi_28505_ = (0f >= _e3149);
                                                                }
                                                                let _e3153 = phi_28505_;
                                                                let _e3155 = pow(select(_e3149, 0f, _e3153), 5f);
                                                                let _e3156 = (1f - _e3155);
                                                                phi_15065_ = vec3<f32>(((_e1963.x * _e3156) + _e3155), ((_e1963.y * _e3156) + _e3155), ((_e1963.z * _e3156) + _e3155));
                                                            } else {
                                                                if (_e1965 == 1u) {
                                                                    let _e3122 = (1f / _e1959);
                                                                    let _e3126 = ((_e3122 * _e3122) * (1f - (_e2854 * _e2854)));
                                                                    if (_e3126 >= 1f) {
                                                                        phi_28466_ = 1f;
                                                                    } else {
                                                                        let _e3129 = sqrt((1f - _e3126));
                                                                        if (_e2854 != _e2854) {
                                                                            phi_28477_ = true;
                                                                        } else {
                                                                            phi_28477_ = (_e3129 <= _e2854);
                                                                        }
                                                                        let _e3133 = phi_28477_;
                                                                        let _e3138 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                                                        phi_28466_ = (_e3138 + ((1f - _e3138) * pow((1f - select(_e2854, _e3129, _e3133)), 5f)));
                                                                    }
                                                                    let _e3145 = phi_28466_;
                                                                    phi_15064_ = vec3(_e3145);
                                                                } else {
                                                                    let _e2858 = select(_e2854, 0f, (_e2854 < 0f));
                                                                    let _e2860 = select(_e2858, 1f, (_e2858 > 1f));
                                                                    let _e2862 = (1f - (_e2860 * _e2860));
                                                                    let _e2875 = ((_e1963.x * _e1963.x) - (_e1961.x * _e1961.x));
                                                                    let _e2876 = ((_e1963.y * _e1963.y) - (_e1961.y * _e1961.y));
                                                                    let _e2877 = ((_e1963.z * _e1963.z) - (_e1961.z * _e1961.z));
                                                                    let _e2879 = (_e1963.x * (_e1961.x + _e1961.x));
                                                                    let _e2881 = (_e1963.y * (_e1961.y + _e1961.y));
                                                                    let _e2883 = (_e1963.z * (_e1961.z + _e1961.z));
                                                                    let _e2890 = ((_e2875 * _e2875) + (_e2879 * _e2879));
                                                                    let _e2891 = ((_e2876 * _e2876) + (_e2881 * _e2881));
                                                                    let _e2892 = ((_e2877 * _e2877) + (_e2883 * _e2883));
                                                                    let _e2897 = ((_e2862 * _e2875) / _e2890);
                                                                    let _e2898 = ((_e2862 * _e2876) / _e2891);
                                                                    let _e2899 = ((_e2862 * _e2877) / _e2892);
                                                                    let _e2900 = (1f - _e2897);
                                                                    let _e2901 = (1f - _e2898);
                                                                    let _e2902 = (1f - _e2899);
                                                                    let _e2904 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2897, _e2898, _e2899));
                                                                    let _e2905 = (_e2862 * _e2879);
                                                                    let _e2906 = (_e2862 * _e2881);
                                                                    let _e2907 = (_e2862 * _e2883);
                                                                    let _e2912 = (vec3<f32>(_e2905, _e2906, _e2907) / vec3<f32>(_e2890, _e2891, _e2892));
                                                                    let _e2925 = ((_e2904.x * _e2904.x) + (_e2912.x * _e2912.x));
                                                                    let _e2926 = ((_e2904.y * _e2904.y) + (_e2912.y * _e2912.y));
                                                                    let _e2927 = ((_e2904.z * _e2904.z) + (_e2912.z * _e2912.z));
                                                                    if (_e2925 != _e2925) {
                                                                        phi_28136_ = true;
                                                                    } else {
                                                                        phi_28136_ = (0f >= _e2925);
                                                                    }
                                                                    let _e2931 = phi_28136_;
                                                                    let _e2933 = sqrt(select(_e2925, 0f, _e2931));
                                                                    if (_e2926 != _e2926) {
                                                                        phi_28151_ = true;
                                                                    } else {
                                                                        phi_28151_ = (0f >= _e2926);
                                                                    }
                                                                    let _e2937 = phi_28151_;
                                                                    let _e2939 = sqrt(select(_e2926, 0f, _e2937));
                                                                    if (_e2927 != _e2927) {
                                                                        phi_28166_ = true;
                                                                    } else {
                                                                        phi_28166_ = (0f >= _e2927);
                                                                    }
                                                                    let _e2943 = phi_28166_;
                                                                    let _e2945 = sqrt(select(_e2927, 0f, _e2943));
                                                                    let _e2949 = ((_e2933 + _e2900) * 0.5f);
                                                                    let _e2950 = ((_e2939 + _e2901) * 0.5f);
                                                                    let _e2951 = ((_e2945 + _e2902) * 0.5f);
                                                                    if (_e2949 != _e2949) {
                                                                        phi_28193_ = true;
                                                                    } else {
                                                                        phi_28193_ = (0f >= _e2949);
                                                                    }
                                                                    let _e2955 = phi_28193_;
                                                                    let _e2957 = sqrt(select(_e2949, 0f, _e2955));
                                                                    if (_e2950 != _e2950) {
                                                                        phi_28208_ = true;
                                                                    } else {
                                                                        phi_28208_ = (0f >= _e2950);
                                                                    }
                                                                    let _e2961 = phi_28208_;
                                                                    let _e2963 = sqrt(select(_e2950, 0f, _e2961));
                                                                    if (_e2951 != _e2951) {
                                                                        phi_28223_ = true;
                                                                    } else {
                                                                        phi_28223_ = (0f >= _e2951);
                                                                    }
                                                                    let _e2967 = phi_28223_;
                                                                    let _e2969 = sqrt(select(_e2951, 0f, _e2967));
                                                                    let _e2970 = vec3<f32>(_e2957, _e2963, _e2969);
                                                                    let _e2974 = ((_e2933 - _e2900) * 0.5f);
                                                                    let _e2975 = ((_e2939 - _e2901) * 0.5f);
                                                                    let _e2976 = ((_e2945 - _e2902) * 0.5f);
                                                                    if (_e2974 != _e2974) {
                                                                        phi_28250_ = true;
                                                                    } else {
                                                                        phi_28250_ = (0f >= _e2974);
                                                                    }
                                                                    let _e2980 = phi_28250_;
                                                                    let _e2982 = sqrt(select(_e2974, 0f, _e2980));
                                                                    if (_e2975 != _e2975) {
                                                                        phi_28265_ = true;
                                                                    } else {
                                                                        phi_28265_ = (0f >= _e2975);
                                                                    }
                                                                    let _e2986 = phi_28265_;
                                                                    let _e2988 = sqrt(select(_e2975, 0f, _e2986));
                                                                    if (_e2976 != _e2976) {
                                                                        phi_28280_ = true;
                                                                    } else {
                                                                        phi_28280_ = (0f >= _e2976);
                                                                    }
                                                                    let _e2992 = phi_28280_;
                                                                    let _e2994 = sqrt(select(_e2976, 0f, _e2992));
                                                                    if ((_e2905 / _e2890) < 0f) {
                                                                        phi_14941_ = -(_e2982);
                                                                    } else {
                                                                        phi_14941_ = _e2982;
                                                                    }
                                                                    let _e2998 = phi_14941_;
                                                                    if ((_e2906 / _e2891) < 0f) {
                                                                        phi_14949_ = -(_e2988);
                                                                    } else {
                                                                        phi_14949_ = _e2988;
                                                                    }
                                                                    let _e3002 = phi_14949_;
                                                                    if ((_e2907 / _e2892) < 0f) {
                                                                        phi_14957_ = -(_e2994);
                                                                    } else {
                                                                        phi_14957_ = _e2994;
                                                                    }
                                                                    let _e3006 = phi_14957_;
                                                                    let _e3010 = (_e1961.x * _e2860);
                                                                    let _e3011 = (_e1961.y * _e2860);
                                                                    let _e3012 = (_e1961.z * _e2860);
                                                                    let _e3022 = vec3<f32>(((_e1963.x * _e2957) - (_e1961.x * _e2998)), ((_e1963.y * _e2963) - (_e1961.y * _e3002)), ((_e1963.z * _e2969) - (_e1961.z * _e3006)));
                                                                    let _e3029 = ((_e1963.x * _e2998) + (_e1961.x * _e2957));
                                                                    let _e3030 = ((_e1963.y * _e3002) + (_e1961.y * _e2963));
                                                                    let _e3031 = ((_e1963.z * _e3006) + (_e1961.z * _e2969));
                                                                    let _e3033 = vec3<f32>((_e1963.x * _e2860), (_e1963.y * _e2860), (_e1963.z * _e2860));
                                                                    let _e3034 = (_e3033 - _e2970);
                                                                    let _e3035 = (_e3010 - _e2998);
                                                                    let _e3036 = (_e3011 - _e3002);
                                                                    let _e3037 = (_e3012 - _e3006);
                                                                    let _e3050 = (_e3033 + _e2970);
                                                                    let _e3051 = (_e3010 + _e2998);
                                                                    let _e3052 = (_e3011 + _e3002);
                                                                    let _e3053 = (_e3012 + _e3006);
                                                                    let _e3063 = ((_e3050.x * _e3050.x) + (_e3051 * _e3051));
                                                                    let _e3064 = ((_e3050.y * _e3050.y) + (_e3052 * _e3052));
                                                                    let _e3065 = ((_e3050.z * _e3050.z) + (_e3053 * _e3053));
                                                                    let _e3075 = vec3(_e2860);
                                                                    let _e3076 = (_e3075 - _e3022);
                                                                    let _e3077 = -(vec3<f32>(_e3029, _e3030, _e3031));
                                                                    let _e3093 = (_e3075 + _e3022);
                                                                    let _e3103 = ((_e3093.x * _e3093.x) + (_e3029 * _e3029));
                                                                    let _e3104 = ((_e3093.y * _e3093.y) + (_e3030 * _e3030));
                                                                    let _e3105 = ((_e3093.z * _e3093.z) + (_e3031 * _e3031));
                                                                    phi_15064_ = vec3<f32>((((((_e3034.x * _e3034.x) + (_e3035 * _e3035)) / select(0.000000000000000000000000000001f, _e3063, (_e3063 > 0.000000000000000000000000000001f))) + (((_e3076.x * _e3076.x) + (_e3077.x * _e3077.x)) / select(0.000000000000000000000000000001f, _e3103, (_e3103 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3034.y * _e3034.y) + (_e3036 * _e3036)) / select(0.000000000000000000000000000001f, _e3064, (_e3064 > 0.000000000000000000000000000001f))) + (((_e3076.y * _e3076.y) + (_e3077.y * _e3077.y)) / select(0.000000000000000000000000000001f, _e3104, (_e3104 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3034.z * _e3034.z) + (_e3037 * _e3037)) / select(0.000000000000000000000000000001f, _e3065, (_e3065 > 0.000000000000000000000000000001f))) + (((_e3076.z * _e3076.z) + (_e3077.z * _e3077.z)) / select(0.000000000000000000000000000001f, _e3105, (_e3105 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e3148 = phi_15064_;
                                                                phi_15065_ = _e3148;
                                                            }
                                                            let _e3168 = phi_15065_;
                                                            let _e3170 = (4f * _e2782);
                                                            phi_15077_ = type_59((_e3168 * (((1f / (_e2833 + _e2847)) * _e2818) / _e3170)), ((_e2818 / _e2833) / _e3170));
                                                        }
                                                        let _e3177 = phi_15077_;
                                                        phi_15078_ = _e3177;
                                                        phi_15079_ = _e2794;
                                                    }
                                                    let _e3179 = phi_15078_;
                                                    let _e3181 = phi_15079_;
                                                    if _e3181 {
                                                        phi_15084_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_15084_ = _e3179;
                                                    }
                                                    let _e3183 = phi_15084_;
                                                    let _e3186 = (_e1969 == 1u);
                                                    if _e3186 {
                                                        phi_15112_ = _e3183;
                                                        phi_15113_ = type_59();
                                                    } else {
                                                        if (_e2782 != _e2782) {
                                                            phi_28542_ = true;
                                                        } else {
                                                            phi_28542_ = (0f >= _e2782);
                                                        }
                                                        let _e3190 = phi_28542_;
                                                        let _e3191 = select(_e2782, 0f, _e3190);
                                                        let _e3192 = (1f / _e1959);
                                                        let _e3196 = ((_e3192 * _e3192) * (1f - (_e3191 * _e3191)));
                                                        if (_e3196 >= 1f) {
                                                            phi_28561_ = 1f;
                                                        } else {
                                                            let _e3199 = sqrt((1f - _e3196));
                                                            if (_e3191 != _e3191) {
                                                                phi_28572_ = true;
                                                            } else {
                                                                phi_28572_ = (_e3199 <= _e3191);
                                                            }
                                                            let _e3203 = phi_28572_;
                                                            let _e3208 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                                            phi_28561_ = (_e3208 + ((1f - _e3208) * pow((1f - select(_e3191, _e3199, _e3203)), 5f)));
                                                        }
                                                        let _e3215 = phi_28561_;
                                                        if (_e2760 <= 0f) {
                                                            phi_28611_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28611_ = (_e1957 * (_e2760 * 0.31830987f));
                                                        }
                                                        let _e3220 = phi_28611_;
                                                        let _e3221 = (1f - _e3215);
                                                        phi_15112_ = type_59();
                                                        phi_15113_ = type_59((_e3183.member + vec3<f32>((_e3220.x * _e3221), (_e3220.y * _e3221), (_e3220.z * _e3221))), ((_e3215 * _e3183.member_1) + (_e3221 * _e2766)));
                                                    }
                                                    let _e3235 = phi_15112_;
                                                    let _e3237 = phi_15113_;
                                                    phi_15114_ = _e3235;
                                                    phi_15115_ = _e3237;
                                                    phi_15116_ = _e3186;
                                                }
                                                let _e3245 = phi_15114_;
                                                let _e3247 = phi_15115_;
                                                let _e3249 = phi_15116_;
                                                if _e3249 {
                                                    phi_15120_ = _e3245;
                                                } else {
                                                    phi_15120_ = _e3247;
                                                }
                                                let _e3251 = phi_15120_;
                                                if ((_e3251.member_1 * _e2755) < 0.001f) {
                                                    let _e3678 = global_1.member.member_6;
                                                    let _e3684 = global_1.member.member_8[2u];
                                                    if (_e3684 == 4294967295u) {
                                                        phi_15859_ = vec3<f32>(1f, 1f, 1f);
                                                    } else {
                                                        let _e3687 = global_6.member[_e3684];
                                                        let _e3690 = global_6.member[(_e3684 + 1u)];
                                                        let _e3693 = global_6.member[(_e3684 + 2u)];
                                                        let _e3696 = global_6.member[(_e3684 + 3u)];
                                                        let _e3699 = global_6.member[(_e3684 + 4u)];
                                                        let _e3702 = global_6.member[(_e3684 + 5u)];
                                                        let _e3705 = global_6.member[(_e3684 + 6u)];
                                                        let _e3708 = global_6.member[(_e3684 + 7u)];
                                                        let _e3711 = global_6.member[(_e3684 + 8u)];
                                                        let _e3714 = global_6.member[(_e3684 + 9u)];
                                                        let _e3717 = global_6.member[(_e3684 + 13u)];
                                                        let _e3720 = global_6.member[(_e3684 + 14u)];
                                                        let _e3723 = global_6.member[(_e3684 + 15u)];
                                                        let _e3726 = global_6.member[(_e3684 + 16u)];
                                                        let _e3729 = global_6.member[(_e3684 + 18u)];
                                                        let _e3732 = global_6.member[(_e3684 + 19u)];
                                                        let _e3735 = global_6.member[(_e3684 + 20u)];
                                                        let _e3738 = global_6.member[(_e3684 + 29u)];
                                                        let _e3739 = bitcast<u32>(_e3738);
                                                        let _e3752 = (((((_e1102 * _e1102) + (_e1103 * _e1103)) + (_e1104 * _e1104)) - (2f * (((_e1102 * _e3687) + (_e1103 * _e3690)) + (_e1104 * _e3693)))) + _e3705);
                                                        if (_e3752 > 0f) {
                                                            let _e3761 = ((((_e1102 - _e3687) * _e3708) + ((_e1103 - _e3690) * _e3711)) + ((_e1104 - _e3693) * _e3714));
                                                            let _e3763 = ((_e3761 * _e3761) - _e3752);
                                                            if (_e3761 >= 0f) {
                                                                phi_15559_ = vec3<f32>();
                                                                phi_15560_ = true;
                                                            } else {
                                                                let _e3765 = (_e3763 <= 0f);
                                                                if _e3765 {
                                                                    phi_15558_ = vec3<f32>();
                                                                } else {
                                                                    let _e3768 = (-(_e3761) - sqrt(_e3763));
                                                                    phi_15558_ = vec3<f32>((_e1102 + (_e3708 * _e3768)), (_e1103 + (_e3711 * _e3768)), (_e1104 + (_e3714 * _e3768)));
                                                                }
                                                                let _e3777 = phi_15558_;
                                                                phi_15559_ = _e3777;
                                                                phi_15560_ = _e3765;
                                                            }
                                                            let _e3779 = phi_15559_;
                                                            let _e3781 = phi_15560_;
                                                            phi_15565_ = _e3779;
                                                            phi_15566_ = select(true, false, _e3781);
                                                        } else {
                                                            phi_15565_ = _e1105;
                                                            phi_15566_ = true;
                                                        }
                                                        let _e3784 = phi_15565_;
                                                        let _e3786 = phi_15566_;
                                                        if _e3786 {
                                                            let _e3788 = (_e3784.x - _e3687);
                                                            let _e3790 = (_e3784.y - _e3690);
                                                            let _e3792 = (_e3784.z - _e3693);
                                                            let _e3798 = sqrt((((_e3788 * _e3788) + (_e3790 * _e3790)) + (_e3792 * _e3792)));
                                                            let _e3799 = (1f / _e3798);
                                                            let _e3817 = ((((((_e3784.x * _e3784.x) + (_e3784.y * _e3784.y)) + (_e3784.z * _e3784.z)) - (2f * (((_e3784.x * _e3687) + (_e3784.y * _e3690)) + (_e3784.z * _e3693)))) + _e3702) / (_e3798 + _e3696));
                                                            if (_e3817 != _e3817) {
                                                                phi_29067_ = true;
                                                            } else {
                                                                phi_29067_ = (0f >= _e3817);
                                                            }
                                                            let _e3821 = phi_29067_;
                                                            let _e3825 = sqrt((select(_e3817, 0f, _e3821) / (_e3699 - _e3696)));
                                                            if (_e3825 != _e3825) {
                                                                phi_29082_ = true;
                                                            } else {
                                                                phi_29082_ = (1f <= _e3825);
                                                            }
                                                            let _e3829 = phi_29082_;
                                                            let _e3831 = (select(_e3825, 1f, _e3829) * 63f);
                                                            let _e3836 = ((((_e3788 * _e3799) * _e3708) + ((_e3790 * _e3799) * _e3711)) + ((_e3792 * _e3799) * _e3714));
                                                            let _e3838 = select(_e3836, -1f, (_e3836 < -1f));
                                                            let _e3842 = ((select(_e3838, 1f, (_e3838 > 1f)) + 1f) * 127.5f);
                                                            let _e3847 = select(select(u32(_e3831), 0u, (_e3831 < 0f)), 4294967295u, (_e3831 > 4294967000f));
                                                            let _e3849 = select(_e3847, 62u, (62u < _e3847));
                                                            let _e3854 = select(select(u32(_e3842), 0u, (_e3842 < 0f)), 4294967295u, (_e3842 > 4294967000f));
                                                            let _e3856 = select(_e3854, 254u, (254u < _e3854));
                                                            let _e3858 = (_e3831 - f32(_e3849));
                                                            let _e3860 = (_e3842 - f32(_e3856));
                                                            let _e3864 = (_e3739 + (3u * ((_e3849 * 256u) + _e3856)));
                                                            let _e3869 = (_e3739 + (3u * (((_e3849 + 1u) * 256u) + _e3856)));
                                                            let _e3871 = global_6.member[_e3864];
                                                            let _e3874 = global_6.member[(_e3864 + 1u)];
                                                            let _e3877 = global_6.member[(_e3864 + 2u)];
                                                            let _e3878 = (1f - _e3860);
                                                            let _e3884 = global_6.member[(_e3864 + 3u)];
                                                            let _e3887 = global_6.member[(_e3864 + 4u)];
                                                            let _e3890 = global_6.member[(_e3864 + 5u)];
                                                            let _e3897 = (1f - _e3858);
                                                            let _e3902 = global_6.member[_e3869];
                                                            let _e3905 = global_6.member[(_e3869 + 1u)];
                                                            let _e3908 = global_6.member[(_e3869 + 2u)];
                                                            let _e3914 = global_6.member[(_e3869 + 3u)];
                                                            let _e3917 = global_6.member[(_e3869 + 4u)];
                                                            let _e3920 = global_6.member[(_e3869 + 5u)];
                                                            phi_15857_ = vec3<f32>(((((_e3871 * _e3878) + (_e3884 * _e3860)) * _e3897) + (((_e3902 * _e3878) + (_e3914 * _e3860)) * _e3858)), ((((_e3874 * _e3878) + (_e3887 * _e3860)) * _e3897) + (((_e3905 * _e3878) + (_e3917 * _e3860)) * _e3858)), ((((_e3877 * _e3878) + (_e3890 * _e3860)) * _e3897) + (((_e3908 * _e3878) + (_e3920 * _e3860)) * _e3858)));
                                                        } else {
                                                            phi_15857_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3935 = phi_15857_;
                                                        let _e3942 = ((_e3726 * 1.1f) * _e3935.y);
                                                        phi_15859_ = vec3<f32>(exp(-((((_e3717 * _e3935.x) + _e3942) + (_e3729 * _e3935.z)))), exp(-((((_e3720 * _e3935.x) + _e3942) + (_e3732 * _e3935.z)))), exp(-((((_e3723 * _e3935.x) + _e3942) + (_e3735 * _e3935.z)))));
                                                    }
                                                    let _e3961 = phi_15859_;
                                                    phi_36447_ = _e1984;
                                                    phi_16743_ = 0f;
                                                    phi_16744_ = vec3<f32>(_e2743.x, _e2743.y, _e2743.z);
                                                    phi_16745_ = (vec3<f32>(_e3678.x, _e3678.y, _e3678.z) * _e3961);
                                                } else {
                                                    let _e3256 = ((_e1984 * 747796405u) + 2891336453u);
                                                    let _e3264 = (((_e3256 >> bitcast<u32>((((_e3256 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3256) * 277803737u);
                                                    let _e3267 = ((_e3264 >> bitcast<u32>(22i)) ^ _e3264);
                                                    let _e3273 = (((f32((_e3267 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2755) * 0.15915494f);
                                                    let _e3274 = (1f - _e3273);
                                                    let _e3276 = (_e3273 * (2f - _e3273));
                                                    if (_e3276 != _e3276) {
                                                        phi_28665_ = true;
                                                    } else {
                                                        phi_28665_ = (0f >= _e3276);
                                                    }
                                                    let _e3280 = phi_28665_;
                                                    let _e3282 = sqrt(select(_e3276, 0f, _e3280));
                                                    let _e3284 = ((_e3267 * 747796405u) + 2891336453u);
                                                    let _e3292 = (((_e3284 >> bitcast<u32>((((_e3284 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3284) * 277803737u);
                                                    let _e3295 = ((_e3292 >> bitcast<u32>(22i)) ^ _e3292);
                                                    let _e3299 = (f32((_e3295 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                    let _e3301 = select(-1f, 1f, (_e2743.z >= 0f));
                                                    let _e3303 = (-1f / (_e3301 + _e2743.z));
                                                    let _e3305 = ((_e2743.x * _e2743.y) * _e3303);
                                                    let _e3318 = (_e3282 * cos(_e3299));
                                                    let _e3323 = (_e3282 * sin(_e3299));
                                                    let _e3333 = ((((1f + (((_e3301 * _e2743.x) * _e2743.x) * _e3303)) * _e3318) + (_e3305 * _e3323)) + (_e2743.x * _e3274));
                                                    let _e3334 = ((((_e3301 * _e3305) * _e3318) + ((_e3301 + ((_e2743.y * _e2743.y) * _e3303)) * _e3323)) + (_e2743.y * _e3274));
                                                    let _e3335 = ((((-(_e3301) * _e2743.x) * _e3318) + (-(_e2743.y) * _e3323)) + (_e2743.z * _e3274));
                                                    let _e3337 = (_e3333 - _e2743.x);
                                                    let _e3338 = (_e3334 - _e2743.y);
                                                    let _e3339 = (_e3335 - _e2743.z);
                                                    let _e3346 = (sqrt((((_e3337 * _e3337) + (_e3338 * _e3338)) + (_e3339 * _e3339))) * 0.5f);
                                                    let _e3347 = (_e3346 != _e3346);
                                                    if _e3347 {
                                                        phi_28736_ = true;
                                                    } else {
                                                        phi_28736_ = (1f <= _e3346);
                                                    }
                                                    let _e3350 = phi_28736_;
                                                    if ((2f * asin(select(_e3346, 1f, _e3350))) >= _e2751) {
                                                        phi_15940_ = 0f;
                                                    } else {
                                                        phi_15940_ = (1f / _e2755);
                                                    }
                                                    let _e3357 = phi_15940_;
                                                    if (_e3357 > 0f) {
                                                        if _e3347 {
                                                            phi_28751_ = true;
                                                        } else {
                                                            phi_28751_ = (1f <= _e3346);
                                                        }
                                                        let _e3361 = phi_28751_;
                                                        let _e3364 = (2f * asin(select(_e3346, 1f, _e3361)));
                                                        if (_e3364 >= _e2751) {
                                                            phi_16741_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e3368 = (1f - pow((_e3364 / _e2751), 2f));
                                                            if (_e3368 != _e3368) {
                                                                phi_28766_ = true;
                                                            } else {
                                                                phi_28766_ = (0f >= _e3368);
                                                            }
                                                            let _e3372 = phi_28766_;
                                                            let _e3375 = global_1.member.member_6;
                                                            let _e3381 = global_1.member.member_8[2u];
                                                            if (_e3381 == 4294967295u) {
                                                                phi_16714_ = vec3<f32>(1f, 1f, 1f);
                                                            } else {
                                                                let _e3384 = global_6.member[_e3381];
                                                                let _e3387 = global_6.member[(_e3381 + 1u)];
                                                                let _e3390 = global_6.member[(_e3381 + 2u)];
                                                                let _e3393 = global_6.member[(_e3381 + 3u)];
                                                                let _e3396 = global_6.member[(_e3381 + 4u)];
                                                                let _e3399 = global_6.member[(_e3381 + 5u)];
                                                                let _e3402 = global_6.member[(_e3381 + 6u)];
                                                                let _e3405 = global_6.member[(_e3381 + 7u)];
                                                                let _e3408 = global_6.member[(_e3381 + 8u)];
                                                                let _e3411 = global_6.member[(_e3381 + 9u)];
                                                                let _e3414 = global_6.member[(_e3381 + 13u)];
                                                                let _e3417 = global_6.member[(_e3381 + 14u)];
                                                                let _e3420 = global_6.member[(_e3381 + 15u)];
                                                                let _e3423 = global_6.member[(_e3381 + 16u)];
                                                                let _e3426 = global_6.member[(_e3381 + 18u)];
                                                                let _e3429 = global_6.member[(_e3381 + 19u)];
                                                                let _e3432 = global_6.member[(_e3381 + 20u)];
                                                                let _e3435 = global_6.member[(_e3381 + 29u)];
                                                                let _e3436 = bitcast<u32>(_e3435);
                                                                let _e3449 = (((((_e1102 * _e1102) + (_e1103 * _e1103)) + (_e1104 * _e1104)) - (2f * (((_e1102 * _e3384) + (_e1103 * _e3387)) + (_e1104 * _e3390)))) + _e3402);
                                                                if (_e3449 > 0f) {
                                                                    let _e3458 = ((((_e1102 - _e3384) * _e3405) + ((_e1103 - _e3387) * _e3408)) + ((_e1104 - _e3390) * _e3411));
                                                                    let _e3460 = ((_e3458 * _e3458) - _e3449);
                                                                    if (_e3458 >= 0f) {
                                                                        phi_16414_ = vec3<f32>();
                                                                        phi_16415_ = true;
                                                                    } else {
                                                                        let _e3462 = (_e3460 <= 0f);
                                                                        if _e3462 {
                                                                            phi_16413_ = vec3<f32>();
                                                                        } else {
                                                                            let _e3465 = (-(_e3458) - sqrt(_e3460));
                                                                            phi_16413_ = vec3<f32>((_e1102 + (_e3405 * _e3465)), (_e1103 + (_e3408 * _e3465)), (_e1104 + (_e3411 * _e3465)));
                                                                        }
                                                                        let _e3474 = phi_16413_;
                                                                        phi_16414_ = _e3474;
                                                                        phi_16415_ = _e3462;
                                                                    }
                                                                    let _e3476 = phi_16414_;
                                                                    let _e3478 = phi_16415_;
                                                                    phi_16420_ = _e3476;
                                                                    phi_16421_ = select(true, false, _e3478);
                                                                } else {
                                                                    phi_16420_ = _e1105;
                                                                    phi_16421_ = true;
                                                                }
                                                                let _e3481 = phi_16420_;
                                                                let _e3483 = phi_16421_;
                                                                if _e3483 {
                                                                    let _e3485 = (_e3481.x - _e3384);
                                                                    let _e3487 = (_e3481.y - _e3387);
                                                                    let _e3489 = (_e3481.z - _e3390);
                                                                    let _e3495 = sqrt((((_e3485 * _e3485) + (_e3487 * _e3487)) + (_e3489 * _e3489)));
                                                                    let _e3496 = (1f / _e3495);
                                                                    let _e3514 = ((((((_e3481.x * _e3481.x) + (_e3481.y * _e3481.y)) + (_e3481.z * _e3481.z)) - (2f * (((_e3481.x * _e3384) + (_e3481.y * _e3387)) + (_e3481.z * _e3390)))) + _e3399) / (_e3495 + _e3393));
                                                                    if (_e3514 != _e3514) {
                                                                        phi_28890_ = true;
                                                                    } else {
                                                                        phi_28890_ = (0f >= _e3514);
                                                                    }
                                                                    let _e3518 = phi_28890_;
                                                                    let _e3522 = sqrt((select(_e3514, 0f, _e3518) / (_e3396 - _e3393)));
                                                                    if (_e3522 != _e3522) {
                                                                        phi_28905_ = true;
                                                                    } else {
                                                                        phi_28905_ = (1f <= _e3522);
                                                                    }
                                                                    let _e3526 = phi_28905_;
                                                                    let _e3528 = (select(_e3522, 1f, _e3526) * 63f);
                                                                    let _e3533 = ((((_e3485 * _e3496) * _e3405) + ((_e3487 * _e3496) * _e3408)) + ((_e3489 * _e3496) * _e3411));
                                                                    let _e3535 = select(_e3533, -1f, (_e3533 < -1f));
                                                                    let _e3539 = ((select(_e3535, 1f, (_e3535 > 1f)) + 1f) * 127.5f);
                                                                    let _e3544 = select(select(u32(_e3528), 0u, (_e3528 < 0f)), 4294967295u, (_e3528 > 4294967000f));
                                                                    let _e3546 = select(_e3544, 62u, (62u < _e3544));
                                                                    let _e3551 = select(select(u32(_e3539), 0u, (_e3539 < 0f)), 4294967295u, (_e3539 > 4294967000f));
                                                                    let _e3553 = select(_e3551, 254u, (254u < _e3551));
                                                                    let _e3555 = (_e3528 - f32(_e3546));
                                                                    let _e3557 = (_e3539 - f32(_e3553));
                                                                    let _e3561 = (_e3436 + (3u * ((_e3546 * 256u) + _e3553)));
                                                                    let _e3566 = (_e3436 + (3u * (((_e3546 + 1u) * 256u) + _e3553)));
                                                                    let _e3568 = global_6.member[_e3561];
                                                                    let _e3571 = global_6.member[(_e3561 + 1u)];
                                                                    let _e3574 = global_6.member[(_e3561 + 2u)];
                                                                    let _e3575 = (1f - _e3557);
                                                                    let _e3581 = global_6.member[(_e3561 + 3u)];
                                                                    let _e3584 = global_6.member[(_e3561 + 4u)];
                                                                    let _e3587 = global_6.member[(_e3561 + 5u)];
                                                                    let _e3594 = (1f - _e3555);
                                                                    let _e3599 = global_6.member[_e3566];
                                                                    let _e3602 = global_6.member[(_e3566 + 1u)];
                                                                    let _e3605 = global_6.member[(_e3566 + 2u)];
                                                                    let _e3611 = global_6.member[(_e3566 + 3u)];
                                                                    let _e3614 = global_6.member[(_e3566 + 4u)];
                                                                    let _e3617 = global_6.member[(_e3566 + 5u)];
                                                                    phi_16712_ = vec3<f32>(((((_e3568 * _e3575) + (_e3581 * _e3557)) * _e3594) + (((_e3599 * _e3575) + (_e3611 * _e3557)) * _e3555)), ((((_e3571 * _e3575) + (_e3584 * _e3557)) * _e3594) + (((_e3602 * _e3575) + (_e3614 * _e3557)) * _e3555)), ((((_e3574 * _e3575) + (_e3587 * _e3557)) * _e3594) + (((_e3605 * _e3575) + (_e3617 * _e3557)) * _e3555)));
                                                                } else {
                                                                    phi_16712_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e3632 = phi_16712_;
                                                                let _e3639 = ((_e3423 * 1.1f) * _e3632.y);
                                                                phi_16714_ = vec3<f32>(exp(-((((_e3414 * _e3632.x) + _e3639) + (_e3426 * _e3632.z)))), exp(-((((_e3417 * _e3632.x) + _e3639) + (_e3429 * _e3632.z)))), exp(-((((_e3420 * _e3632.x) + _e3639) + (_e3432 * _e3632.z)))));
                                                            }
                                                            let _e3658 = phi_16714_;
                                                            let _e3669 = (((1f - (0.6f * (1f - sqrt(select(_e3368, 0f, _e3372))))) * 1.25f) / _e2755);
                                                            phi_16741_ = vec3<f32>(((_e3375.x * _e3658.x) * _e3669), ((_e3375.y * _e3658.y) * _e3669), ((_e3375.z * _e3658.z) * _e3669));
                                                        }
                                                        let _e3675 = phi_16741_;
                                                        phi_16742_ = _e3675;
                                                    } else {
                                                        phi_16742_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e3677 = phi_16742_;
                                                    phi_36447_ = _e3295;
                                                    phi_16743_ = _e3357;
                                                    phi_16744_ = vec3<f32>(_e3333, _e3334, _e3335);
                                                    phi_16745_ = _e3677;
                                                }
                                                let _e3965 = phi_36447_;
                                                let _e3967 = phi_16743_;
                                                let _e3969 = phi_16744_;
                                                let _e3971 = phi_16745_;
                                                phi_36446_ = _e3965;
                                                phi_16746_ = _e3967;
                                                phi_16747_ = _e3969;
                                                phi_16748_ = _e3971;
                                            }
                                            let _e3973 = phi_36446_;
                                            let _e3975 = phi_16746_;
                                            let _e3977 = phi_16747_;
                                            let _e3979 = phi_16748_;
                                            phi_36403_ = _e3973;
                                            phi_18113_ = _e3975;
                                            phi_18114_ = _e3977;
                                            phi_18115_ = _e3979;
                                            phi_18116_ = 1000000000000000000000000000000f;
                                        } else {
                                            if (_e1990 == (_e1980 + 1u)) {
                                                if ((_e376 & 1u) == 0u) {
                                                    phi_36412_ = _e1984;
                                                    phi_17747_ = 0f;
                                                    phi_17748_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17749_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17750_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e379 > 0u) {
                                                        let _e2221 = global_1.member.member_2[1u];
                                                        let _e2223 = ((_e1984 * 747796405u) + 2891336453u);
                                                        let _e2231 = (((_e2223 >> bitcast<u32>((((_e2223 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2223) * 277803737u);
                                                        let _e2234 = ((_e2231 >> bitcast<u32>(22i)) ^ _e2231);
                                                        phi_16769_ = 0u;
                                                        phi_16772_ = _e379;
                                                        loop {
                                                            let _e2240 = phi_16769_;
                                                            let _e2242 = phi_16772_;
                                                            local_78 = _e2240;
                                                            local_79 = _e2240;
                                                            let _e2244 = ((_e2240 + 1u) < _e2242);
                                                            if _e2244 {
                                                                let _e2246 = ((_e2240 + _e2242) / 2u);
                                                                let _e2249 = global_6.member[(_e2221 + _e2246)];
                                                                let _e2250 = (_e2249 <= (f32((_e2234 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                phi_16770_ = select(_e2240, _e2246, _e2250);
                                                                phi_16773_ = select(_e2246, _e2242, _e2250);
                                                            } else {
                                                                phi_16770_ = u32();
                                                                phi_16773_ = u32();
                                                            }
                                                            let _e2254 = phi_16770_;
                                                            let _e2256 = phi_16773_;
                                                            phi_16769_ = _e2254;
                                                            phi_16772_ = _e2256;
                                                            if !(_e2244) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2259 = local_78;
                                                        let _e2260 = (_e2221 + _e2259);
                                                        let _e2263 = global_6.member[(_e2260 + 1u)];
                                                        let _e2265 = global_6.member[_e2260];
                                                        let _e2269 = global_1.member.member_2[0u];
                                                        let _e2271 = local_79;
                                                        let _e2274 = global_6.member[(_e2269 + _e2271)];
                                                        let _e2275 = bitcast<u32>(_e2274);
                                                        let _e2277 = ((_e2234 * 747796405u) + 2891336453u);
                                                        let _e2285 = (((_e2277 >> bitcast<u32>((((_e2277 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2277) * 277803737u);
                                                        let _e2288 = ((_e2285 >> bitcast<u32>(22i)) ^ _e2285);
                                                        let _e2291 = f32((_e2288 >> bitcast<u32>(8i)));
                                                        let _e2292 = (_e2291 * 0.000000059604645f);
                                                        let _e2294 = ((_e2288 * 747796405u) + 2891336453u);
                                                        let _e2302 = (((_e2294 >> bitcast<u32>((((_e2294 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2294) * 277803737u);
                                                        let _e2305 = ((_e2302 >> bitcast<u32>(22i)) ^ _e2302);
                                                        let _e2308 = f32((_e2305 >> bitcast<u32>(8i)));
                                                        let _e2310 = (3u * _e2275);
                                                        let _e2312 = global_2.member[_e2310];
                                                        let _e2315 = global_2.member[(_e2310 + 1u)];
                                                        let _e2321 = select(select(u32(_e2315.w), 0u, (_e2315.w < 0f)), 4294967295u, (_e2315.w > 4294967000f));
                                                        let _e2323 = (_e848 + (5u * _e2275));
                                                        let _e2325 = global_2.member[_e2323];
                                                        let _e2328 = global_2.member[(_e2323 + 1u)];
                                                        let _e2335 = (_e2321 == 1u);
                                                        if _e2335 {
                                                            let _e2464 = (_e2312.x - _e1102);
                                                            let _e2465 = (_e2312.y - _e1103);
                                                            let _e2466 = (_e2312.z - _e1104);
                                                            let _e2471 = (((_e2464 * _e2464) + (_e2465 * _e2465)) + (_e2466 * _e2466));
                                                            let _e2472 = (_e2312.w * _e2312.w);
                                                            let _e2474 = (_e2471 <= (_e2472 * 1.000001f));
                                                            if _e2474 {
                                                                let _e2583 = (_e2291 * 0.00000037450704f);
                                                                let _e2585 = ((_e2308 * 0.00000011920929f) - 1f);
                                                                let _e2587 = (1f - (_e2585 * _e2585));
                                                                if (_e2587 != _e2587) {
                                                                    phi_27733_ = true;
                                                                } else {
                                                                    phi_27733_ = (0f >= _e2587);
                                                                }
                                                                let _e2591 = phi_27733_;
                                                                let _e2593 = sqrt(select(_e2587, 0f, _e2591));
                                                                let _e2595 = (_e2593 * cos(_e2583));
                                                                let _e2597 = (_e2593 * sin(_e2583));
                                                                let _e2602 = (_e2312.x + (_e2595 * _e2312.w));
                                                                let _e2603 = (_e2312.y + (_e2597 * _e2312.w));
                                                                let _e2604 = (_e2312.z + (_e2585 * _e2312.w));
                                                                if _e2474 {
                                                                    let _e2617 = (_e2602 - _e1102);
                                                                    let _e2618 = (_e2603 - _e1103);
                                                                    let _e2619 = (_e2604 - _e1104);
                                                                    let _e2624 = (((_e2617 * _e2617) + (_e2618 * _e2618)) + (_e2619 * _e2619));
                                                                    phi_27802_ = (_e2624 / (((abs((((_e2595 * _e2617) + (_e2597 * _e2618)) + (_e2585 * _e2619))) / sqrt(_e2624)) * 12.566371f) * _e2472));
                                                                } else {
                                                                    let _e2606 = (_e2472 / _e2471);
                                                                    if (_e2606 != _e2606) {
                                                                        phi_27821_ = true;
                                                                    } else {
                                                                        phi_27821_ = (1f <= _e2606);
                                                                    }
                                                                    let _e2610 = phi_27821_;
                                                                    let _e2611 = select(_e2606, 1f, _e2610);
                                                                    phi_27802_ = (0.15915494f / (_e2611 / (1f + sqrt((1f - _e2611)))));
                                                                }
                                                                let _e2637 = phi_27802_;
                                                                phi_17120_ = type_72(vec3<f32>(_e2602, _e2603, _e2604), vec3<f32>(_e2595, _e2597, _e2585), _e2637);
                                                            } else {
                                                                let _e2475 = (_e2472 / _e2471);
                                                                let _e2476 = (_e2475 != _e2475);
                                                                if _e2476 {
                                                                    phi_27585_ = true;
                                                                } else {
                                                                    phi_27585_ = (1f <= _e2475);
                                                                }
                                                                let _e2479 = phi_27585_;
                                                                let _e2480 = select(_e2475, 1f, _e2479);
                                                                if _e2476 {
                                                                    phi_27608_ = true;
                                                                } else {
                                                                    phi_27608_ = (1f <= _e2475);
                                                                }
                                                                let _e2484 = phi_27608_;
                                                                let _e2485 = select(_e2475, 1f, _e2484);
                                                                let _e2489 = (_e2485 / (1f + sqrt((1f - _e2485))));
                                                                if (_e2480 < 0.00068523f) {
                                                                    phi_17055_ = (_e2480 * _e2292);
                                                                } else {
                                                                    let _e2492 = (1f - (_e2292 * _e2489));
                                                                    let _e2494 = (1f - (_e2492 * _e2492));
                                                                    if (_e2494 != _e2494) {
                                                                        phi_27623_ = true;
                                                                    } else {
                                                                        phi_27623_ = (0f >= _e2494);
                                                                    }
                                                                    let _e2498 = phi_27623_;
                                                                    phi_17055_ = select(_e2494, 0f, _e2498);
                                                                }
                                                                let _e2502 = phi_17055_;
                                                                let _e2503 = (1f - _e2502);
                                                                if (_e2503 != _e2503) {
                                                                    phi_27638_ = true;
                                                                } else {
                                                                    phi_27638_ = (0f >= _e2503);
                                                                }
                                                                let _e2507 = phi_27638_;
                                                                let _e2512 = (1f - (_e2502 / _e2480));
                                                                if (_e2512 != _e2512) {
                                                                    phi_27653_ = true;
                                                                } else {
                                                                    phi_27653_ = (0f >= _e2512);
                                                                }
                                                                let _e2516 = phi_27653_;
                                                                let _e2520 = ((_e2502 / sqrt(_e2480)) + (sqrt(select(_e2503, 0f, _e2507)) * sqrt(select(_e2512, 0f, _e2516))));
                                                                let _e2522 = (1f - (_e2520 * _e2520));
                                                                if (_e2522 != _e2522) {
                                                                    phi_27668_ = true;
                                                                } else {
                                                                    phi_27668_ = (0f >= _e2522);
                                                                }
                                                                let _e2526 = phi_27668_;
                                                                let _e2528 = sqrt(select(_e2522, 0f, _e2526));
                                                                let _e2529 = (_e2308 * 0.00000037450704f);
                                                                let _e2530 = sqrt(_e2471);
                                                                let _e2531 = (_e2464 / _e2530);
                                                                let _e2532 = (_e2465 / _e2530);
                                                                let _e2533 = (_e2466 / _e2530);
                                                                let _e2535 = select(-1f, 1f, (_e2533 >= 0f));
                                                                let _e2537 = (-1f / (_e2535 + _e2533));
                                                                let _e2539 = ((_e2531 * _e2532) * _e2537);
                                                                let _e2552 = (_e2528 * cos(_e2529));
                                                                let _e2557 = (_e2528 * sin(_e2529));
                                                                let _e2570 = -(((((1f + (((_e2535 * _e2531) * _e2531) * _e2537)) * _e2552) + (_e2539 * _e2557)) + (_e2531 * _e2520)));
                                                                let _e2571 = -(((((_e2535 * _e2539) * _e2552) + ((_e2535 + ((_e2532 * _e2532) * _e2537)) * _e2557)) + (_e2532 * _e2520)));
                                                                let _e2572 = -(((((-(_e2535) * _e2531) * _e2552) + (-(_e2532) * _e2557)) + (_e2533 * _e2520)));
                                                                phi_17120_ = type_72(vec3<f32>((_e2312.x + (_e2570 * _e2312.w)), (_e2312.y + (_e2571 * _e2312.w)), (_e2312.z + (_e2572 * _e2312.w))), vec3<f32>(_e2570, _e2571, _e2572), (0.15915494f / _e2489));
                                                            }
                                                            let _e2640 = phi_17120_;
                                                            phi_17548_ = _e2640;
                                                        } else {
                                                            let _e2336 = sqrt(_e2292);
                                                            let _e2337 = (1f - _e2336);
                                                            let _e2338 = ((_e2308 * 0.000000059604645f) * _e2336);
                                                            let _e2351 = (_e2336 - _e2338);
                                                            let _e2355 = (((_e2312.x * _e2337) + (_e2325.x * _e2338)) + (_e2328.x * _e2351));
                                                            let _e2356 = (((_e2312.y * _e2337) + (_e2325.y * _e2338)) + (_e2328.y * _e2351));
                                                            let _e2357 = (((_e2312.z * _e2337) + (_e2325.z * _e2338)) + (_e2328.z * _e2351));
                                                            let _e2359 = (_e2325.x - _e2312.x);
                                                            let _e2360 = (_e2325.y - _e2312.y);
                                                            let _e2361 = (_e2325.z - _e2312.z);
                                                            let _e2362 = (_e2328.x - _e2312.x);
                                                            let _e2363 = (_e2328.y - _e2312.y);
                                                            let _e2364 = (_e2328.z - _e2312.z);
                                                            let _e2367 = ((_e2360 * _e2364) - (_e2363 * _e2361));
                                                            let _e2370 = ((_e2361 * _e2362) - (_e2364 * _e2359));
                                                            let _e2373 = ((_e2359 * _e2363) - (_e2362 * _e2360));
                                                            let _e2379 = sqrt((((_e2367 * _e2367) + (_e2370 * _e2370)) + (_e2373 * _e2373)));
                                                            let _e2380 = (1f / _e2379);
                                                            let _e2381 = (_e2367 * _e2380);
                                                            let _e2382 = (_e2370 * _e2380);
                                                            let _e2383 = (_e2373 * _e2380);
                                                            if _e2335 {
                                                                let _e2414 = (_e2312.x - _e1102);
                                                                let _e2415 = (_e2312.y - _e1103);
                                                                let _e2416 = (_e2312.z - _e1104);
                                                                let _e2421 = (((_e2414 * _e2414) + (_e2415 * _e2415)) + (_e2416 * _e2416));
                                                                let _e2422 = (_e2312.w * _e2312.w);
                                                                if (_e2421 <= (_e2422 * 1.000001f)) {
                                                                    let _e2436 = (_e2355 - _e1102);
                                                                    let _e2437 = (_e2356 - _e1103);
                                                                    let _e2438 = (_e2357 - _e1104);
                                                                    let _e2443 = (((_e2436 * _e2436) + (_e2437 * _e2437)) + (_e2438 * _e2438));
                                                                    phi_27551_ = (_e2443 / (((abs((((_e2381 * _e2436) + (_e2382 * _e2437)) + (_e2383 * _e2438))) / sqrt(_e2443)) * 12.566371f) * _e2422));
                                                                } else {
                                                                    let _e2425 = (_e2422 / _e2421);
                                                                    if (_e2425 != _e2425) {
                                                                        phi_27570_ = true;
                                                                    } else {
                                                                        phi_27570_ = (1f <= _e2425);
                                                                    }
                                                                    let _e2429 = phi_27570_;
                                                                    let _e2430 = select(_e2425, 1f, _e2429);
                                                                    phi_27551_ = (0.15915494f / (_e2430 / (1f + sqrt((1f - _e2430)))));
                                                                }
                                                                let _e2456 = phi_27551_;
                                                                phi_17546_ = _e2456;
                                                            } else {
                                                                let _e2385 = (_e2355 - _e1102);
                                                                let _e2386 = (_e2356 - _e1103);
                                                                let _e2387 = (_e2357 - _e1104);
                                                                let _e2392 = (((_e2385 * _e2385) + (_e2386 * _e2386)) + (_e2387 * _e2387));
                                                                let _e2400 = (abs((((_e2381 * _e2385) + (_e2382 * _e2386)) + (_e2383 * _e2387))) / sqrt(_e2392));
                                                                if (_e2400 <= 0f) {
                                                                    phi_17545_ = 0f;
                                                                } else {
                                                                    if (_e2321 == 0u) {
                                                                        phi_17542_ = (_e2379 * 0.5f);
                                                                    } else {
                                                                        phi_17542_ = ((12.566371f * _e2312.w) * _e2312.w);
                                                                    }
                                                                    let _e2408 = phi_17542_;
                                                                    phi_17545_ = (_e2392 / (_e2400 * _e2408));
                                                                }
                                                                let _e2412 = phi_17545_;
                                                                phi_17546_ = _e2412;
                                                            }
                                                            let _e2458 = phi_17546_;
                                                            phi_17548_ = type_72(vec3<f32>(_e2355, _e2356, _e2357), vec3<f32>(_e2381, _e2382, _e2383), _e2458);
                                                        }
                                                        let _e2642 = phi_17548_;
                                                        let _e2647 = (_e2642.member.x - _e1102);
                                                        let _e2649 = (_e2642.member.y - _e1103);
                                                        let _e2651 = (_e2642.member.z - _e1104);
                                                        let _e2657 = sqrt((((_e2647 * _e2647) + (_e2649 * _e2649)) + (_e2651 * _e2651)));
                                                        let _e2658 = (_e2647 / _e2657);
                                                        let _e2659 = (_e2649 / _e2657);
                                                        let _e2660 = (_e2651 / _e2657);
                                                        if (_e2642.member_2 > 0f) {
                                                            let _e2676 = global_4.member[select(select(u32(_e2325.w), 0u, (_e2325.w < 0f)), 4294967295u, (_e2325.w > 4294967000f))];
                                                            if (_e2676.member == 4u) {
                                                                if (_e2676.member_1 == 1u) {
                                                                    phi_17724_ = true;
                                                                } else {
                                                                    phi_17724_ = ((((-(_e2658) * _e2642.member_1.x) + (-(_e2659) * _e2642.member_1.y)) + (-(_e2660) * _e2642.member_1.z)) > 0f);
                                                                }
                                                                let _e2691 = phi_17724_;
                                                                if _e2691 {
                                                                    phi_17733_ = vec3<f32>(_e2676.member_4.x, _e2676.member_4.y, _e2676.member_4.z);
                                                                } else {
                                                                    phi_17733_ = vec3<f32>();
                                                                }
                                                                let _e2698 = phi_17733_;
                                                                phi_17735_ = _e2698;
                                                                phi_17736_ = select(true, false, _e2691);
                                                            } else {
                                                                phi_17735_ = vec3<f32>();
                                                                phi_17736_ = true;
                                                            }
                                                            let _e2701 = phi_17735_;
                                                            let _e2703 = phi_17736_;
                                                            phi_17741_ = ((_e2263 - _e2265) * _e2642.member_2);
                                                            phi_17742_ = select(_e2701, vec3<f32>(0f, 0f, 0f), vec3(_e2703));
                                                        } else {
                                                            phi_17741_ = 0f;
                                                            phi_17742_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2707 = phi_17741_;
                                                        let _e2709 = phi_17742_;
                                                        phi_36413_ = _e2305;
                                                        phi_17743_ = _e2707;
                                                        phi_17744_ = vec3<f32>(_e2658, _e2659, _e2660);
                                                        phi_17745_ = _e2709;
                                                        phi_17746_ = ((_e2657 * 0.9999f) - _e1095);
                                                    } else {
                                                        phi_36413_ = _e1984;
                                                        phi_17743_ = 0f;
                                                        phi_17744_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17745_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17746_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e2711 = phi_36413_;
                                                    let _e2713 = phi_17743_;
                                                    let _e2715 = phi_17744_;
                                                    let _e2717 = phi_17745_;
                                                    let _e2719 = phi_17746_;
                                                    phi_36412_ = _e2711;
                                                    phi_17747_ = _e2713;
                                                    phi_17748_ = _e2715;
                                                    phi_17749_ = _e2717;
                                                    phi_17750_ = _e2719;
                                                }
                                                let _e2721 = phi_36412_;
                                                let _e2723 = phi_17747_;
                                                let _e2725 = phi_17748_;
                                                let _e2727 = phi_17749_;
                                                let _e2729 = phi_17750_;
                                                phi_36404_ = _e2721;
                                                phi_18109_ = _e2723;
                                                phi_18110_ = _e2725;
                                                phi_18111_ = _e2727;
                                                phi_18112_ = _e2729;
                                            } else {
                                                if ((_e376 & 2u) == 0u) {
                                                    phi_36405_ = _e1984;
                                                    phi_18106_ = 0f;
                                                    phi_18107_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_18108_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e2002 = ((_e1984 * 747796405u) + 2891336453u);
                                                    let _e2010 = (((_e2002 >> bitcast<u32>((((_e2002 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2002) * 277803737u);
                                                    let _e2013 = ((_e2010 >> bitcast<u32>(22i)) ^ _e2010);
                                                    let _e2017 = (f32((_e2013 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                    let _e2019 = ((_e2013 * 747796405u) + 2891336453u);
                                                    let _e2027 = (((_e2019 >> bitcast<u32>((((_e2019 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2019) * 277803737u);
                                                    let _e2030 = ((_e2027 >> bitcast<u32>(22i)) ^ _e2027);
                                                    let _e2034 = (f32((_e2030 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                    let _e2037 = global_1.member.member_3[1u];
                                                    let _e2040 = global_1.member.member_3[2u];
                                                    let _e2043 = global_1.member.member_2[3u];
                                                    phi_17771_ = 0u;
                                                    phi_17774_ = _e2040;
                                                    loop {
                                                        let _e2045 = phi_17771_;
                                                        let _e2047 = phi_17774_;
                                                        local_73 = _e2045;
                                                        local_76 = _e2045;
                                                        local_77 = _e2045;
                                                        let _e2049 = ((_e2045 + 1u) < _e2047);
                                                        if _e2049 {
                                                            let _e2051 = ((_e2045 + _e2047) / 2u);
                                                            let _e2054 = global_6.member[(_e2043 + _e2051)];
                                                            let _e2055 = (_e2054 <= _e2017);
                                                            phi_17772_ = select(_e2045, _e2051, _e2055);
                                                            phi_17775_ = select(_e2051, _e2047, _e2055);
                                                        } else {
                                                            phi_17772_ = u32();
                                                            phi_17775_ = u32();
                                                        }
                                                        let _e2059 = phi_17772_;
                                                        let _e2061 = phi_17775_;
                                                        phi_17771_ = _e2059;
                                                        phi_17774_ = _e2061;
                                                        if !(_e2049) {
                                                            break;
                                                        }
                                                    }
                                                    let _e2065 = global_1.member.member_3[0u];
                                                    let _e2066 = (_e2037 + 1u);
                                                    let _e2068 = local_73;
                                                    let _e2070 = (_e2065 + (_e2068 * _e2066));
                                                    phi_17812_ = 0u;
                                                    phi_17815_ = _e2037;
                                                    loop {
                                                        let _e2072 = phi_17812_;
                                                        let _e2074 = phi_17815_;
                                                        local_74 = _e2072;
                                                        local_75 = _e2072;
                                                        let _e2076 = ((_e2072 + 1u) < _e2074);
                                                        if _e2076 {
                                                            let _e2078 = ((_e2072 + _e2074) / 2u);
                                                            let _e2081 = global_6.member[(_e2070 + _e2078)];
                                                            let _e2082 = (_e2081 <= _e2034);
                                                            phi_17813_ = select(_e2072, _e2078, _e2082);
                                                            phi_17816_ = select(_e2078, _e2074, _e2082);
                                                        } else {
                                                            phi_17813_ = u32();
                                                            phi_17816_ = u32();
                                                        }
                                                        let _e2086 = phi_17813_;
                                                        let _e2088 = phi_17816_;
                                                        phi_17812_ = _e2086;
                                                        phi_17815_ = _e2088;
                                                        if !(_e2076) {
                                                            break;
                                                        }
                                                    }
                                                    let _e2091 = local_74;
                                                    let _e2094 = local_75;
                                                    let _e2095 = (_e2070 + _e2094);
                                                    let _e2097 = global_6.member[_e2095];
                                                    let _e2100 = global_6.member[(_e2095 + 1u)];
                                                    if (_e2100 > _e2097) {
                                                        let _e2104 = ((_e2034 - _e2097) / (_e2100 - _e2097));
                                                        let _e2106 = select(_e2104, 0f, (_e2104 < 0f));
                                                        phi_17892_ = select(_e2106, 0.99999f, (_e2106 > 0.99999f));
                                                    } else {
                                                        phi_17892_ = 0.5f;
                                                    }
                                                    let _e2110 = phi_17892_;
                                                    let _e2112 = f32(_e2037);
                                                    let _e2115 = local_76;
                                                    let _e2118 = local_77;
                                                    let _e2119 = (_e2043 + _e2118);
                                                    let _e2121 = global_6.member[_e2119];
                                                    let _e2124 = global_6.member[(_e2119 + 1u)];
                                                    if (_e2124 > _e2121) {
                                                        let _e2128 = ((_e2017 - _e2121) / (_e2124 - _e2121));
                                                        let _e2130 = select(_e2128, 0f, (_e2128 < 0f));
                                                        phi_17941_ = select(_e2130, 0.99999f, (_e2130 > 0.99999f));
                                                    } else {
                                                        phi_17941_ = 0.5f;
                                                    }
                                                    let _e2134 = phi_17941_;
                                                    let _e2136 = f32(_e2040);
                                                    let _e2139 = ((((f32(_e2091) + _e2110) / _e2112) - 0.5f) * 6.2831855f);
                                                    let _e2140 = (((f32(_e2115) + _e2134) / _e2136) * 3.1415927f);
                                                    let _e2141 = sin(_e2140);
                                                    let _e2143 = (_e2141 * cos(_e2139));
                                                    let _e2144 = cos(_e2140);
                                                    let _e2146 = (_e2141 * sin(_e2139));
                                                    let _e2149 = ((atan2(_e2146, _e2143) * 0.15915494f) + 0.5f);
                                                    let _e2153 = select(_e2144, -1f, (_e2144 < -1f));
                                                    let _e2156 = acos(select(_e2153, 1f, (_e2153 > 1f)));
                                                    let _e2158 = ((_e2149 - floor(_e2149)) * _e2112);
                                                    let _e2163 = select(select(u32(_e2158), 0u, (_e2158 < 0f)), 4294967295u, (_e2158 > 4294967000f));
                                                    let _e2164 = (_e2037 - 1u);
                                                    let _e2167 = ((_e2156 * 0.31830987f) * _e2136);
                                                    let _e2172 = select(select(u32(_e2167), 0u, (_e2167 < 0f)), 4294967295u, (_e2167 > 4294967000f));
                                                    let _e2173 = (_e2040 - 1u);
                                                    let _e2175 = select(_e2172, _e2173, (_e2173 < _e2172));
                                                    let _e2176 = sin(_e2156);
                                                    if (_e2176 <= 0f) {
                                                        phi_18097_ = 0f;
                                                    } else {
                                                        let _e2180 = (_e2043 + _e2175);
                                                        let _e2183 = global_6.member[(_e2180 + 1u)];
                                                        let _e2185 = global_6.member[_e2180];
                                                        let _e2188 = ((_e2065 + (_e2175 * _e2066)) + select(_e2163, _e2164, (_e2164 < _e2163)));
                                                        let _e2191 = global_6.member[(_e2188 + 1u)];
                                                        let _e2193 = global_6.member[_e2188];
                                                        phi_18097_ = ((((_e2183 - _e2185) * _e2136) * ((_e2191 - _e2193) * _e2112)) / (19.73921f * _e2176));
                                                    }
                                                    let _e2200 = phi_18097_;
                                                    let _e2201 = (_e2200 > 0f);
                                                    if _e2201 {
                                                        phi_18104_ = vec3<f32>(_e2143, _e2144, _e2146);
                                                    } else {
                                                        phi_18104_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e2204 = phi_18104_;
                                                    phi_36405_ = _e2030;
                                                    phi_18106_ = select(0f, _e2200, _e2201);
                                                    phi_18107_ = _e2204;
                                                    phi_18108_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2201));
                                                }
                                                let _e2209 = phi_36405_;
                                                let _e2211 = phi_18106_;
                                                let _e2213 = phi_18107_;
                                                let _e2215 = phi_18108_;
                                                phi_36404_ = _e2209;
                                                phi_18109_ = _e2211;
                                                phi_18110_ = _e2213;
                                                phi_18111_ = _e2215;
                                                phi_18112_ = 1000000000000000000000000000000f;
                                            }
                                            let _e2731 = phi_36404_;
                                            let _e2733 = phi_18109_;
                                            let _e2735 = phi_18110_;
                                            let _e2737 = phi_18111_;
                                            let _e2739 = phi_18112_;
                                            phi_36403_ = _e2731;
                                            phi_18113_ = _e2733;
                                            phi_18114_ = _e2735;
                                            phi_18115_ = _e2737;
                                            phi_18116_ = _e2739;
                                        }
                                        let _e3981 = phi_36403_;
                                        let _e3983 = phi_18113_;
                                        let _e3985 = phi_18114_;
                                        let _e3987 = phi_18115_;
                                        let _e3989 = phi_18116_;
                                        phi_36402_ = _e3981;
                                        phi_18117_ = _e3983;
                                        phi_18118_ = _e3985;
                                        phi_18119_ = _e3987;
                                        phi_18120_ = _e3989;
                                    }
                                    let _e4058 = phi_36402_;
                                    let _e4060 = phi_18117_;
                                    let _e4062 = phi_18118_;
                                    let _e4064 = phi_18119_;
                                    let _e4066 = phi_18120_;
                                    if ((((_e4064.x * _e4064.x) + (_e4064.y * _e4064.y)) + (_e4064.z * _e4064.z)) > 0f) {
                                        let _e4083 = (((_e4062.x * _e1077.x) + (_e4062.y * _e1077.y)) + (_e4062.z * _e1077.z));
                                        if (_e4083 != _e4083) {
                                            phi_29135_ = true;
                                        } else {
                                            phi_29135_ = (0f >= _e4083);
                                        }
                                        let _e4087 = phi_29135_;
                                        let _e4089 = (select(_e4083, 0f, _e4087) * 0.31830987f);
                                        if (_e1969 == 0u) {
                                            if (_e4083 <= 0f) {
                                                phi_29932_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_29932_ = (_e1963 * (_e4083 * 0.31830987f));
                                            }
                                            let _e4565 = phi_29932_;
                                            phi_18516_ = type_59(_e4565, _e4089);
                                            phi_18517_ = type_59();
                                            phi_18518_ = true;
                                        } else {
                                            let _e4095 = (((_e1051 * _e1125) + (_e1052 * _e1126)) + (_e1053 * _e1128));
                                            let _e4100 = (((_e1051 * _e1121) + (_e1052 * _e1131)) + (_e1053 * _e1132));
                                            let _e4105 = (((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z));
                                            let _e4110 = (((_e4062.x * _e1125) + (_e4062.y * _e1126)) + (_e4062.z * _e1128));
                                            let _e4115 = (((_e4062.x * _e1121) + (_e4062.y * _e1131)) + (_e4062.z * _e1132));
                                            if (_e4105 <= 0f) {
                                                phi_18480_ = type_59();
                                                phi_18481_ = true;
                                            } else {
                                                let _e4117 = (_e4083 <= 0f);
                                                if _e4117 {
                                                    phi_18479_ = type_59();
                                                } else {
                                                    let _e4118 = (_e4095 + _e4110);
                                                    let _e4119 = (_e4100 + _e4115);
                                                    let _e4120 = (_e4105 + _e4083);
                                                    let _e4127 = (1f / sqrt((((_e4118 * _e4118) + (_e4119 * _e4119)) + (_e4120 * _e4120))));
                                                    let _e4130 = (_e4120 * _e4127);
                                                    if (_e4130 <= 0f) {
                                                        phi_29245_ = 0f;
                                                    } else {
                                                        let _e4132 = (_e1967 * _e1967);
                                                        let _e4136 = ((((_e4132 - 1f) * _e4130) * _e4130) + 1f);
                                                        phi_29245_ = (_e4132 / ((3.1415927f * _e4136) * _e4136));
                                                    }
                                                    let _e4141 = phi_29245_;
                                                    let _e4142 = (_e4105 * _e4105);
                                                    if (_e4142 >= 1f) {
                                                        phi_29273_ = 0f;
                                                    } else {
                                                        phi_29273_ = ((sqrt((1f + (((_e1967 * _e1967) * ((_e4095 * _e4095) + (_e4100 * _e4100))) / _e4142))) - 1f) * 0.5f);
                                                    }
                                                    let _e4155 = phi_29273_;
                                                    let _e4156 = (1f + _e4155);
                                                    let _e4157 = (_e4083 * _e4083);
                                                    if (_e4157 >= 1f) {
                                                        phi_29301_ = 0f;
                                                    } else {
                                                        phi_29301_ = ((sqrt((1f + (((_e1967 * _e1967) * ((_e4110 * _e4110) + (_e4115 * _e4115))) / _e4157))) - 1f) * 0.5f);
                                                    }
                                                    let _e4170 = phi_29301_;
                                                    let _e4177 = (((_e4095 * (_e4118 * _e4127)) + (_e4100 * (_e4119 * _e4127))) + (_e4105 * _e4130));
                                                    if (_e1965 == 0u) {
                                                        let _e4472 = (1f - _e4177);
                                                        if (_e4472 != _e4472) {
                                                            phi_29804_ = true;
                                                        } else {
                                                            phi_29804_ = (0f >= _e4472);
                                                        }
                                                        let _e4476 = phi_29804_;
                                                        let _e4478 = pow(select(_e4472, 0f, _e4476), 5f);
                                                        let _e4479 = (1f - _e4478);
                                                        phi_18467_ = vec3<f32>(((_e1963.x * _e4479) + _e4478), ((_e1963.y * _e4479) + _e4478), ((_e1963.z * _e4479) + _e4478));
                                                    } else {
                                                        if (_e1965 == 1u) {
                                                            let _e4445 = (1f / _e1959);
                                                            let _e4449 = ((_e4445 * _e4445) * (1f - (_e4177 * _e4177)));
                                                            if (_e4449 >= 1f) {
                                                                phi_29765_ = 1f;
                                                            } else {
                                                                let _e4452 = sqrt((1f - _e4449));
                                                                if (_e4177 != _e4177) {
                                                                    phi_29776_ = true;
                                                                } else {
                                                                    phi_29776_ = (_e4452 <= _e4177);
                                                                }
                                                                let _e4456 = phi_29776_;
                                                                let _e4461 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                                                phi_29765_ = (_e4461 + ((1f - _e4461) * pow((1f - select(_e4177, _e4452, _e4456)), 5f)));
                                                            }
                                                            let _e4468 = phi_29765_;
                                                            phi_18466_ = vec3(_e4468);
                                                        } else {
                                                            let _e4181 = select(_e4177, 0f, (_e4177 < 0f));
                                                            let _e4183 = select(_e4181, 1f, (_e4181 > 1f));
                                                            let _e4185 = (1f - (_e4183 * _e4183));
                                                            let _e4198 = ((_e1963.x * _e1963.x) - (_e1961.x * _e1961.x));
                                                            let _e4199 = ((_e1963.y * _e1963.y) - (_e1961.y * _e1961.y));
                                                            let _e4200 = ((_e1963.z * _e1963.z) - (_e1961.z * _e1961.z));
                                                            let _e4202 = (_e1963.x * (_e1961.x + _e1961.x));
                                                            let _e4204 = (_e1963.y * (_e1961.y + _e1961.y));
                                                            let _e4206 = (_e1963.z * (_e1961.z + _e1961.z));
                                                            let _e4213 = ((_e4198 * _e4198) + (_e4202 * _e4202));
                                                            let _e4214 = ((_e4199 * _e4199) + (_e4204 * _e4204));
                                                            let _e4215 = ((_e4200 * _e4200) + (_e4206 * _e4206));
                                                            let _e4220 = ((_e4185 * _e4198) / _e4213);
                                                            let _e4221 = ((_e4185 * _e4199) / _e4214);
                                                            let _e4222 = ((_e4185 * _e4200) / _e4215);
                                                            let _e4223 = (1f - _e4220);
                                                            let _e4224 = (1f - _e4221);
                                                            let _e4225 = (1f - _e4222);
                                                            let _e4227 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4220, _e4221, _e4222));
                                                            let _e4228 = (_e4185 * _e4202);
                                                            let _e4229 = (_e4185 * _e4204);
                                                            let _e4230 = (_e4185 * _e4206);
                                                            let _e4235 = (vec3<f32>(_e4228, _e4229, _e4230) / vec3<f32>(_e4213, _e4214, _e4215));
                                                            let _e4248 = ((_e4227.x * _e4227.x) + (_e4235.x * _e4235.x));
                                                            let _e4249 = ((_e4227.y * _e4227.y) + (_e4235.y * _e4235.y));
                                                            let _e4250 = ((_e4227.z * _e4227.z) + (_e4235.z * _e4235.z));
                                                            if (_e4248 != _e4248) {
                                                                phi_29435_ = true;
                                                            } else {
                                                                phi_29435_ = (0f >= _e4248);
                                                            }
                                                            let _e4254 = phi_29435_;
                                                            let _e4256 = sqrt(select(_e4248, 0f, _e4254));
                                                            if (_e4249 != _e4249) {
                                                                phi_29450_ = true;
                                                            } else {
                                                                phi_29450_ = (0f >= _e4249);
                                                            }
                                                            let _e4260 = phi_29450_;
                                                            let _e4262 = sqrt(select(_e4249, 0f, _e4260));
                                                            if (_e4250 != _e4250) {
                                                                phi_29465_ = true;
                                                            } else {
                                                                phi_29465_ = (0f >= _e4250);
                                                            }
                                                            let _e4266 = phi_29465_;
                                                            let _e4268 = sqrt(select(_e4250, 0f, _e4266));
                                                            let _e4272 = ((_e4256 + _e4223) * 0.5f);
                                                            let _e4273 = ((_e4262 + _e4224) * 0.5f);
                                                            let _e4274 = ((_e4268 + _e4225) * 0.5f);
                                                            if (_e4272 != _e4272) {
                                                                phi_29492_ = true;
                                                            } else {
                                                                phi_29492_ = (0f >= _e4272);
                                                            }
                                                            let _e4278 = phi_29492_;
                                                            let _e4280 = sqrt(select(_e4272, 0f, _e4278));
                                                            if (_e4273 != _e4273) {
                                                                phi_29507_ = true;
                                                            } else {
                                                                phi_29507_ = (0f >= _e4273);
                                                            }
                                                            let _e4284 = phi_29507_;
                                                            let _e4286 = sqrt(select(_e4273, 0f, _e4284));
                                                            if (_e4274 != _e4274) {
                                                                phi_29522_ = true;
                                                            } else {
                                                                phi_29522_ = (0f >= _e4274);
                                                            }
                                                            let _e4290 = phi_29522_;
                                                            let _e4292 = sqrt(select(_e4274, 0f, _e4290));
                                                            let _e4293 = vec3<f32>(_e4280, _e4286, _e4292);
                                                            let _e4297 = ((_e4256 - _e4223) * 0.5f);
                                                            let _e4298 = ((_e4262 - _e4224) * 0.5f);
                                                            let _e4299 = ((_e4268 - _e4225) * 0.5f);
                                                            if (_e4297 != _e4297) {
                                                                phi_29549_ = true;
                                                            } else {
                                                                phi_29549_ = (0f >= _e4297);
                                                            }
                                                            let _e4303 = phi_29549_;
                                                            let _e4305 = sqrt(select(_e4297, 0f, _e4303));
                                                            if (_e4298 != _e4298) {
                                                                phi_29564_ = true;
                                                            } else {
                                                                phi_29564_ = (0f >= _e4298);
                                                            }
                                                            let _e4309 = phi_29564_;
                                                            let _e4311 = sqrt(select(_e4298, 0f, _e4309));
                                                            if (_e4299 != _e4299) {
                                                                phi_29579_ = true;
                                                            } else {
                                                                phi_29579_ = (0f >= _e4299);
                                                            }
                                                            let _e4315 = phi_29579_;
                                                            let _e4317 = sqrt(select(_e4299, 0f, _e4315));
                                                            if ((_e4228 / _e4213) < 0f) {
                                                                phi_18343_ = -(_e4305);
                                                            } else {
                                                                phi_18343_ = _e4305;
                                                            }
                                                            let _e4321 = phi_18343_;
                                                            if ((_e4229 / _e4214) < 0f) {
                                                                phi_18351_ = -(_e4311);
                                                            } else {
                                                                phi_18351_ = _e4311;
                                                            }
                                                            let _e4325 = phi_18351_;
                                                            if ((_e4230 / _e4215) < 0f) {
                                                                phi_18359_ = -(_e4317);
                                                            } else {
                                                                phi_18359_ = _e4317;
                                                            }
                                                            let _e4329 = phi_18359_;
                                                            let _e4333 = (_e1961.x * _e4183);
                                                            let _e4334 = (_e1961.y * _e4183);
                                                            let _e4335 = (_e1961.z * _e4183);
                                                            let _e4345 = vec3<f32>(((_e1963.x * _e4280) - (_e1961.x * _e4321)), ((_e1963.y * _e4286) - (_e1961.y * _e4325)), ((_e1963.z * _e4292) - (_e1961.z * _e4329)));
                                                            let _e4352 = ((_e1963.x * _e4321) + (_e1961.x * _e4280));
                                                            let _e4353 = ((_e1963.y * _e4325) + (_e1961.y * _e4286));
                                                            let _e4354 = ((_e1963.z * _e4329) + (_e1961.z * _e4292));
                                                            let _e4356 = vec3<f32>((_e1963.x * _e4183), (_e1963.y * _e4183), (_e1963.z * _e4183));
                                                            let _e4357 = (_e4356 - _e4293);
                                                            let _e4358 = (_e4333 - _e4321);
                                                            let _e4359 = (_e4334 - _e4325);
                                                            let _e4360 = (_e4335 - _e4329);
                                                            let _e4373 = (_e4356 + _e4293);
                                                            let _e4374 = (_e4333 + _e4321);
                                                            let _e4375 = (_e4334 + _e4325);
                                                            let _e4376 = (_e4335 + _e4329);
                                                            let _e4386 = ((_e4373.x * _e4373.x) + (_e4374 * _e4374));
                                                            let _e4387 = ((_e4373.y * _e4373.y) + (_e4375 * _e4375));
                                                            let _e4388 = ((_e4373.z * _e4373.z) + (_e4376 * _e4376));
                                                            let _e4398 = vec3(_e4183);
                                                            let _e4399 = (_e4398 - _e4345);
                                                            let _e4400 = -(vec3<f32>(_e4352, _e4353, _e4354));
                                                            let _e4416 = (_e4398 + _e4345);
                                                            let _e4426 = ((_e4416.x * _e4416.x) + (_e4352 * _e4352));
                                                            let _e4427 = ((_e4416.y * _e4416.y) + (_e4353 * _e4353));
                                                            let _e4428 = ((_e4416.z * _e4416.z) + (_e4354 * _e4354));
                                                            phi_18466_ = vec3<f32>((((((_e4357.x * _e4357.x) + (_e4358 * _e4358)) / select(0.000000000000000000000000000001f, _e4386, (_e4386 > 0.000000000000000000000000000001f))) + (((_e4399.x * _e4399.x) + (_e4400.x * _e4400.x)) / select(0.000000000000000000000000000001f, _e4426, (_e4426 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4357.y * _e4357.y) + (_e4359 * _e4359)) / select(0.000000000000000000000000000001f, _e4387, (_e4387 > 0.000000000000000000000000000001f))) + (((_e4399.y * _e4399.y) + (_e4400.y * _e4400.y)) / select(0.000000000000000000000000000001f, _e4427, (_e4427 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4357.z * _e4357.z) + (_e4360 * _e4360)) / select(0.000000000000000000000000000001f, _e4388, (_e4388 > 0.000000000000000000000000000001f))) + (((_e4399.z * _e4399.z) + (_e4400.z * _e4400.z)) / select(0.000000000000000000000000000001f, _e4428, (_e4428 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e4471 = phi_18466_;
                                                        phi_18467_ = _e4471;
                                                    }
                                                    let _e4491 = phi_18467_;
                                                    let _e4493 = (4f * _e4105);
                                                    phi_18479_ = type_59((_e4491 * (((1f / (_e4156 + _e4170)) * _e4141) / _e4493)), ((_e4141 / _e4156) / _e4493));
                                                }
                                                let _e4500 = phi_18479_;
                                                phi_18480_ = _e4500;
                                                phi_18481_ = _e4117;
                                            }
                                            let _e4502 = phi_18480_;
                                            let _e4504 = phi_18481_;
                                            if _e4504 {
                                                phi_18486_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_18486_ = _e4502;
                                            }
                                            let _e4506 = phi_18486_;
                                            let _e4509 = (_e1969 == 1u);
                                            if _e4509 {
                                                phi_18514_ = _e4506;
                                                phi_18515_ = type_59();
                                            } else {
                                                if (_e4105 != _e4105) {
                                                    phi_29841_ = true;
                                                } else {
                                                    phi_29841_ = (0f >= _e4105);
                                                }
                                                let _e4513 = phi_29841_;
                                                let _e4514 = select(_e4105, 0f, _e4513);
                                                let _e4515 = (1f / _e1959);
                                                let _e4519 = ((_e4515 * _e4515) * (1f - (_e4514 * _e4514)));
                                                if (_e4519 >= 1f) {
                                                    phi_29860_ = 1f;
                                                } else {
                                                    let _e4522 = sqrt((1f - _e4519));
                                                    if (_e4514 != _e4514) {
                                                        phi_29871_ = true;
                                                    } else {
                                                        phi_29871_ = (_e4522 <= _e4514);
                                                    }
                                                    let _e4526 = phi_29871_;
                                                    let _e4531 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                                    phi_29860_ = (_e4531 + ((1f - _e4531) * pow((1f - select(_e4514, _e4522, _e4526)), 5f)));
                                                }
                                                let _e4538 = phi_29860_;
                                                if (_e4083 <= 0f) {
                                                    phi_29910_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_29910_ = (_e1957 * (_e4083 * 0.31830987f));
                                                }
                                                let _e4543 = phi_29910_;
                                                let _e4544 = (1f - _e4538);
                                                phi_18514_ = type_59();
                                                phi_18515_ = type_59((_e4506.member + vec3<f32>((_e4543.x * _e4544), (_e4543.y * _e4544), (_e4543.z * _e4544))), ((_e4538 * _e4506.member_1) + (_e4544 * _e4089)));
                                            }
                                            let _e4558 = phi_18514_;
                                            let _e4560 = phi_18515_;
                                            phi_18516_ = _e4558;
                                            phi_18517_ = _e4560;
                                            phi_18518_ = _e4509;
                                        }
                                        let _e4568 = phi_18516_;
                                        let _e4570 = phi_18517_;
                                        let _e4572 = phi_18518_;
                                        if _e4572 {
                                            phi_18522_ = _e4568;
                                        } else {
                                            phi_18522_ = _e4570;
                                        }
                                        let _e4574 = phi_18522_;
                                        if (_e4060 > 0f) {
                                            if (_e4060 >= 300000000000000000000000000000000000000f) {
                                                phi_29949_ = 1f;
                                            } else {
                                                let _e4579 = (_e4060 * _e4060);
                                                let _e4581 = (_e4579 + (_e4574.member_1 * _e4574.member_1));
                                                if (_e4581 == 0f) {
                                                    phi_29946_ = 0f;
                                                } else {
                                                    phi_29946_ = (_e4579 / _e4581);
                                                }
                                                let _e4585 = phi_29946_;
                                                phi_29949_ = _e4585;
                                            }
                                            let _e4587 = phi_29949_;
                                            phi_18531_ = (_e4587 / _e4060);
                                        } else {
                                            phi_18531_ = 1f;
                                        }
                                        let _e4590 = phi_18531_;
                                        if ((((_e4574.member.x * _e4574.member.x) + (_e4574.member.y * _e4574.member.y)) + (_e4574.member.z * _e4574.member.z)) > 0f) {
                                            let _e4601 = (vec3<f32>(1f, 1f, 1f) / _e4062);
                                            local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                            local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                            phi_18564_ = 4294967295u;
                                            phi_18567_ = select(4294967295u, 0u, (_e831 > 0u));
                                            phi_18569_ = 0u;
                                            phi_18571_ = _e835;
                                            phi_18573_ = _e831;
                                            phi_18575_ = vec4<f32>(0f, 0f, 0f, 0f);
                                            phi_18577_ = _e4066;
                                            loop {
                                                let _e4605 = phi_18564_;
                                                let _e4607 = phi_18567_;
                                                let _e4609 = phi_18569_;
                                                let _e4611 = phi_18571_;
                                                let _e4613 = phi_18573_;
                                                let _e4615 = phi_18575_;
                                                let _e4617 = phi_18577_;
                                                local_80 = _e4605;
                                                local_81 = _e4615;
                                                local_82 = _e4613;
                                                if (_e4613 < _e4611) {
                                                    let _e4980 = (3u * _e4613);
                                                    let _e4982 = global_2.member[_e4980];
                                                    let _e4985 = global_2.member[(_e4980 + 1u)];
                                                    let _e4991 = select(select(u32(_e4985.w), 0u, (_e4985.w < 0f)), 4294967295u, (_e4985.w > 4294967000f));
                                                    if (_e4991 == 0u) {
                                                        let _e5085 = global_2.member[(_e4980 + 2u)];
                                                        let _e5094 = ((_e4062.y * _e5085.z) - (_e5085.y * _e4062.z));
                                                        let _e5097 = ((_e4062.z * _e5085.x) - (_e5085.z * _e4062.x));
                                                        let _e5100 = ((_e4062.x * _e5085.y) - (_e5085.x * _e4062.y));
                                                        let _e5105 = (((_e4985.x * _e5094) + (_e4985.y * _e5097)) + (_e4985.z * _e5100));
                                                        if (abs(_e5105) < 0.000000000001f) {
                                                            phi_18762_ = vec4<f32>();
                                                            phi_18763_ = true;
                                                        } else {
                                                            let _e5108 = (1f / _e5105);
                                                            let _e5112 = (_e1102 - _e4982.x);
                                                            let _e5113 = (_e1103 - _e4982.y);
                                                            let _e5114 = (_e1104 - _e4982.z);
                                                            let _e5120 = ((((_e5112 * _e5094) + (_e5113 * _e5097)) + (_e5114 * _e5100)) * _e5108);
                                                            if (_e5120 < 0f) {
                                                                phi_18752_ = vec4<f32>();
                                                                phi_18753_ = true;
                                                                phi_18754_ = false;
                                                            } else {
                                                                let _e5122 = (_e5120 > 1f);
                                                                if _e5122 {
                                                                    phi_18750_ = vec4<f32>();
                                                                    phi_18751_ = false;
                                                                } else {
                                                                    let _e5125 = ((_e5113 * _e4985.z) - (_e4985.y * _e5114));
                                                                    let _e5128 = ((_e5114 * _e4985.x) - (_e4985.z * _e5112));
                                                                    let _e5131 = ((_e5112 * _e4985.y) - (_e4985.x * _e5113));
                                                                    let _e5137 = ((((_e4062.x * _e5125) + (_e4062.y * _e5128)) + (_e4062.z * _e5131)) * _e5108);
                                                                    if (_e5137 < 0f) {
                                                                        phi_18740_ = vec4<f32>();
                                                                        phi_18741_ = true;
                                                                        phi_18742_ = false;
                                                                    } else {
                                                                        let _e5140 = ((_e5120 + _e5137) > 1f);
                                                                        if _e5140 {
                                                                            phi_18738_ = vec4<f32>();
                                                                            phi_18739_ = false;
                                                                        } else {
                                                                            let _e5146 = ((((_e5085.x * _e5125) + (_e5085.y * _e5128)) + (_e5085.z * _e5131)) * _e5108);
                                                                            if (_e5146 <= 0f) {
                                                                                phi_18733_ = vec4<f32>();
                                                                                phi_18734_ = true;
                                                                            } else {
                                                                                let _e5148 = (_e5146 >= _e4617);
                                                                                if _e5148 {
                                                                                    phi_18732_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_18732_ = vec4<f32>(_e5146, _e5120, _e5137, 1f);
                                                                                }
                                                                                let _e5151 = phi_18732_;
                                                                                phi_18733_ = _e5151;
                                                                                phi_18734_ = _e5148;
                                                                            }
                                                                            let _e5153 = phi_18733_;
                                                                            let _e5155 = phi_18734_;
                                                                            phi_18738_ = _e5153;
                                                                            phi_18739_ = _e5155;
                                                                        }
                                                                        let _e5157 = phi_18738_;
                                                                        let _e5159 = phi_18739_;
                                                                        phi_18740_ = _e5157;
                                                                        phi_18741_ = _e5140;
                                                                        phi_18742_ = _e5159;
                                                                    }
                                                                    let _e5161 = phi_18740_;
                                                                    let _e5163 = phi_18741_;
                                                                    let _e5165 = phi_18742_;
                                                                    phi_18750_ = _e5161;
                                                                    phi_18751_ = select(_e5165, true, _e5163);
                                                                }
                                                                let _e5168 = phi_18750_;
                                                                let _e5170 = phi_18751_;
                                                                phi_18752_ = _e5168;
                                                                phi_18753_ = _e5122;
                                                                phi_18754_ = _e5170;
                                                            }
                                                            let _e5172 = phi_18752_;
                                                            let _e5174 = phi_18753_;
                                                            let _e5176 = phi_18754_;
                                                            phi_18762_ = _e5172;
                                                            phi_18763_ = select(_e5176, true, _e5174);
                                                        }
                                                        let _e5179 = phi_18762_;
                                                        let _e5181 = phi_18763_;
                                                        phi_18908_ = _e5179;
                                                        phi_18909_ = _e5181;
                                                    } else {
                                                        if (_e4991 == 1u) {
                                                            let _e5015 = (_e1102 - _e4982.x);
                                                            let _e5016 = (_e1103 - _e4982.y);
                                                            let _e5017 = (_e1104 - _e4982.z);
                                                            let _e5022 = (((_e5015 * _e4062.x) + (_e5016 * _e4062.y)) + (_e5017 * _e4062.z));
                                                            if (_e5022 > 0f) {
                                                                phi_18802_ = select(true, false, ((((_e5015 * _e5015) + (_e5016 * _e5016)) + (_e5017 * _e5017)) > (_e4982.w * _e4982.w)));
                                                            } else {
                                                                phi_18802_ = true;
                                                            }
                                                            let _e5034 = phi_18802_;
                                                            if _e5034 {
                                                                let _e5038 = (_e5015 - (_e4062.x * _e5022));
                                                                let _e5039 = (_e5016 - (_e4062.y * _e5022));
                                                                let _e5040 = (_e5017 - (_e4062.z * _e5022));
                                                                let _e5048 = ((_e4982.w * _e4982.w) - (((_e5038 * _e5038) + (_e5039 * _e5039)) + (_e5040 * _e5040)));
                                                                if (_e5048 < 0f) {
                                                                    phi_18861_ = vec4<f32>();
                                                                    phi_18862_ = true;
                                                                } else {
                                                                    let _e5050 = sqrt(_e5048);
                                                                    let _e5051 = -(_e5022);
                                                                    let _e5052 = (_e5051 - _e5050);
                                                                    if (_e5052 <= 0f) {
                                                                        phi_18839_ = true;
                                                                    } else {
                                                                        phi_18839_ = (_e5052 >= _e4617);
                                                                    }
                                                                    let _e5056 = phi_18839_;
                                                                    if _e5056 {
                                                                        phi_18845_ = (_e5051 + _e5050);
                                                                    } else {
                                                                        phi_18845_ = _e5052;
                                                                    }
                                                                    let _e5059 = phi_18845_;
                                                                    if (_e5059 <= 0f) {
                                                                        phi_18856_ = vec4<f32>();
                                                                        phi_18857_ = true;
                                                                    } else {
                                                                        let _e5061 = (_e5059 >= _e4617);
                                                                        if _e5061 {
                                                                            phi_18855_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18855_ = vec4<f32>(_e5059, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5064 = phi_18855_;
                                                                        phi_18856_ = _e5064;
                                                                        phi_18857_ = _e5061;
                                                                    }
                                                                    let _e5066 = phi_18856_;
                                                                    let _e5068 = phi_18857_;
                                                                    phi_18861_ = _e5066;
                                                                    phi_18862_ = _e5068;
                                                                }
                                                                let _e5070 = phi_18861_;
                                                                let _e5072 = phi_18862_;
                                                                phi_18866_ = _e5070;
                                                                phi_18867_ = _e5072;
                                                            } else {
                                                                phi_18866_ = vec4<f32>();
                                                                phi_18867_ = true;
                                                            }
                                                            let _e5074 = phi_18866_;
                                                            let _e5076 = phi_18867_;
                                                            phi_18898_ = _e5074;
                                                            phi_18899_ = _e5076;
                                                        } else {
                                                            if (abs(_e4062.y) < 0.000000000001f) {
                                                                phi_18896_ = vec4<f32>();
                                                                phi_18897_ = true;
                                                            } else {
                                                                let _e4998 = ((_e4982.x - _e1103) / _e4062.y);
                                                                if (_e4998 <= 0f) {
                                                                    phi_18891_ = vec4<f32>();
                                                                    phi_18892_ = true;
                                                                } else {
                                                                    let _e5000 = (_e4998 >= _e4617);
                                                                    if _e5000 {
                                                                        phi_18890_ = vec4<f32>();
                                                                    } else {
                                                                        phi_18890_ = vec4<f32>(_e4998, 0f, 0f, 1f);
                                                                    }
                                                                    let _e5003 = phi_18890_;
                                                                    phi_18891_ = _e5003;
                                                                    phi_18892_ = _e5000;
                                                                }
                                                                let _e5005 = phi_18891_;
                                                                let _e5007 = phi_18892_;
                                                                phi_18896_ = _e5005;
                                                                phi_18897_ = _e5007;
                                                            }
                                                            let _e5009 = phi_18896_;
                                                            let _e5011 = phi_18897_;
                                                            phi_18898_ = _e5009;
                                                            phi_18899_ = _e5011;
                                                        }
                                                        let _e5078 = phi_18898_;
                                                        let _e5080 = phi_18899_;
                                                        phi_18908_ = _e5078;
                                                        phi_18909_ = select(true, false, select(true, false, _e5080));
                                                    }
                                                    let _e5183 = phi_18908_;
                                                    let _e5185 = phi_18909_;
                                                    let _e5188 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5183, vec4(select(true, false, _e5185)));
                                                    let _e5190 = (_e5188.w > 0f);
                                                    if _e5190 {
                                                        phi_18930_ = _e5188.x;
                                                    } else {
                                                        phi_18930_ = _e4617;
                                                    }
                                                    let _e5193 = phi_18930_;
                                                    let _e5197 = select(true, false, _e5190);
                                                    if _e5197 {
                                                        phi_18936_ = (_e4613 + 1u);
                                                    } else {
                                                        phi_18936_ = u32();
                                                    }
                                                    let _e5200 = phi_18936_;
                                                    phi_18565_ = select(_e4605, _e4613, _e5190);
                                                    phi_18568_ = _e4607;
                                                    phi_18570_ = _e4609;
                                                    phi_18572_ = _e4611;
                                                    phi_18574_ = _e5200;
                                                    phi_18576_ = select(_e4615, _e5188, vec4(_e5190));
                                                    phi_18578_ = _e5193;
                                                    phi_19109_ = _e5188;
                                                    phi_19110_ = _e5197;
                                                    phi_19111_ = false;
                                                } else {
                                                    if (_e4607 == 4294967295u) {
                                                        let _e4620 = (_e4609 == 0u);
                                                        if _e4620 {
                                                            phi_18966_ = u32();
                                                            phi_18967_ = u32();
                                                            phi_18969_ = false;
                                                            phi_18970_ = false;
                                                        } else {
                                                            let _e4621 = (_e4609 - 1u);
                                                            let _e4622 = (_e4621 < 64u);
                                                            if _e4622 {
                                                            } else {
                                                                phi_35913_ = true;
                                                                phi_21555_ = bool();
                                                                phi_21554_ = vec4<f32>();
                                                                break;
                                                            }
                                                            let _e4624 = local_68[_e4621];
                                                            let _e4625 = (_e4624 >= _e4617);
                                                            if _e4625 {
                                                                phi_18964_ = u32();
                                                            } else {
                                                                if _e4622 {
                                                                } else {
                                                                    phi_35913_ = true;
                                                                    phi_21555_ = bool();
                                                                    phi_21554_ = vec4<f32>();
                                                                    break;
                                                                }
                                                                let _e4627 = local_67[_e4621];
                                                                phi_18964_ = _e4627;
                                                            }
                                                            let _e4629 = phi_18964_;
                                                            phi_18966_ = _e4621;
                                                            phi_18967_ = _e4629;
                                                            phi_18969_ = select(true, false, _e4625);
                                                            phi_18970_ = _e4625;
                                                        }
                                                        let _e4632 = phi_18966_;
                                                        let _e4634 = phi_18967_;
                                                        let _e4636 = phi_18969_;
                                                        let _e4638 = phi_18970_;
                                                        phi_18971_ = _e4632;
                                                        phi_18972_ = _e4634;
                                                        phi_18973_ = _e4632;
                                                        phi_18974_ = _e4636;
                                                        phi_18975_ = _e4638;
                                                        phi_18976_ = _e4620;
                                                    } else {
                                                        phi_18971_ = u32();
                                                        phi_18972_ = _e4607;
                                                        phi_18973_ = _e4609;
                                                        phi_18974_ = true;
                                                        phi_18975_ = false;
                                                        phi_18976_ = false;
                                                    }
                                                    let _e4640 = phi_18971_;
                                                    let _e4642 = phi_18972_;
                                                    let _e4644 = phi_18973_;
                                                    let _e4646 = phi_18974_;
                                                    let _e4648 = phi_18975_;
                                                    let _e4650 = phi_18976_;
                                                    if _e4646 {
                                                        let _e4652 = ((_e4642 & 2147483648u) == 0u);
                                                        if _e4652 {
                                                            let _e4660 = global_3.member[_e4642];
                                                            if (_e4660.member_1 == 4294967295u) {
                                                                phi_19005_ = 300000000000000000000000000000000000000f;
                                                            } else {
                                                                let _e4672 = ((_e4660.member.x - _e1102) * _e4601.x);
                                                                let _e4674 = ((_e4660.member.y - _e1103) * _e4601.y);
                                                                let _e4676 = ((_e4660.member.z - _e1104) * _e4601.z);
                                                                let _e4683 = ((_e4660.member_2.x - _e1102) * _e4601.x);
                                                                let _e4684 = ((_e4660.member_2.y - _e1103) * _e4601.y);
                                                                let _e4685 = ((_e4660.member_2.z - _e1104) * _e4601.z);
                                                                if (_e4672 == _e4672) {
                                                                    let _e4687 = (_e4683 == _e4683);
                                                                    if _e4687 {
                                                                        let _e4688 = (_e4672 != _e4672);
                                                                        if _e4688 {
                                                                            phi_30047_ = true;
                                                                        } else {
                                                                            phi_30047_ = (_e4683 <= _e4672);
                                                                        }
                                                                        let _e4691 = phi_30047_;
                                                                        let _e4692 = select(_e4672, _e4683, _e4691);
                                                                        if _e4688 {
                                                                            phi_30077_ = true;
                                                                        } else {
                                                                            phi_30077_ = (_e4683 >= _e4672);
                                                                        }
                                                                        let _e4697 = phi_30077_;
                                                                        let _e4698 = select(_e4672, _e4683, _e4697);
                                                                        phi_30024_ = type_56(select(-300000000000000000000000000000000000000f, _e4692, (_e4692 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4698, (_e4698 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_30024_ = type_56();
                                                                    }
                                                                    let _e4703 = phi_30024_;
                                                                    phi_30027_ = _e4703;
                                                                    phi_30028_ = select(true, false, _e4687);
                                                                } else {
                                                                    phi_30027_ = type_56();
                                                                    phi_30028_ = true;
                                                                }
                                                                let _e4706 = phi_30027_;
                                                                let _e4708 = phi_30028_;
                                                                if _e4708 {
                                                                    phi_30033_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_30033_ = _e4706;
                                                                }
                                                                let _e4710 = phi_30033_;
                                                                if (_e4674 == _e4674) {
                                                                    let _e4714 = (_e4684 == _e4684);
                                                                    if _e4714 {
                                                                        let _e4715 = (_e4674 != _e4674);
                                                                        if _e4715 {
                                                                            phi_30134_ = true;
                                                                        } else {
                                                                            phi_30134_ = (_e4684 <= _e4674);
                                                                        }
                                                                        let _e4718 = phi_30134_;
                                                                        let _e4719 = select(_e4674, _e4684, _e4718);
                                                                        if (_e4710.member != _e4710.member) {
                                                                            phi_30149_ = true;
                                                                        } else {
                                                                            phi_30149_ = (_e4719 >= _e4710.member);
                                                                        }
                                                                        let _e4723 = phi_30149_;
                                                                        if _e4715 {
                                                                            phi_30164_ = true;
                                                                        } else {
                                                                            phi_30164_ = (_e4684 >= _e4674);
                                                                        }
                                                                        let _e4727 = phi_30164_;
                                                                        let _e4728 = select(_e4674, _e4684, _e4727);
                                                                        if (_e4710.member_1 != _e4710.member_1) {
                                                                            phi_30179_ = true;
                                                                        } else {
                                                                            phi_30179_ = (_e4728 <= _e4710.member_1);
                                                                        }
                                                                        let _e4732 = phi_30179_;
                                                                        phi_30111_ = type_56(select(_e4710.member, _e4719, _e4723), select(_e4710.member_1, _e4728, _e4732));
                                                                    } else {
                                                                        phi_30111_ = type_56();
                                                                    }
                                                                    let _e4736 = phi_30111_;
                                                                    phi_30114_ = _e4736;
                                                                    phi_30115_ = select(true, false, _e4714);
                                                                } else {
                                                                    phi_30114_ = type_56();
                                                                    phi_30115_ = true;
                                                                }
                                                                let _e4739 = phi_30114_;
                                                                let _e4741 = phi_30115_;
                                                                if _e4741 {
                                                                    phi_30120_ = _e4710;
                                                                } else {
                                                                    phi_30120_ = _e4739;
                                                                }
                                                                let _e4743 = phi_30120_;
                                                                if (_e4676 == _e4676) {
                                                                    let _e4747 = (_e4685 == _e4685);
                                                                    if _e4747 {
                                                                        let _e4748 = (_e4676 != _e4676);
                                                                        if _e4748 {
                                                                            phi_30221_ = true;
                                                                        } else {
                                                                            phi_30221_ = (_e4685 <= _e4676);
                                                                        }
                                                                        let _e4751 = phi_30221_;
                                                                        let _e4752 = select(_e4676, _e4685, _e4751);
                                                                        if (_e4743.member != _e4743.member) {
                                                                            phi_30236_ = true;
                                                                        } else {
                                                                            phi_30236_ = (_e4752 >= _e4743.member);
                                                                        }
                                                                        let _e4756 = phi_30236_;
                                                                        if _e4748 {
                                                                            phi_30251_ = true;
                                                                        } else {
                                                                            phi_30251_ = (_e4685 >= _e4676);
                                                                        }
                                                                        let _e4760 = phi_30251_;
                                                                        let _e4761 = select(_e4676, _e4685, _e4760);
                                                                        if (_e4743.member_1 != _e4743.member_1) {
                                                                            phi_30266_ = true;
                                                                        } else {
                                                                            phi_30266_ = (_e4761 <= _e4743.member_1);
                                                                        }
                                                                        let _e4765 = phi_30266_;
                                                                        phi_30198_ = type_56(select(_e4743.member, _e4752, _e4756), select(_e4743.member_1, _e4761, _e4765));
                                                                    } else {
                                                                        phi_30198_ = type_56();
                                                                    }
                                                                    let _e4769 = phi_30198_;
                                                                    phi_30201_ = _e4769;
                                                                    phi_30202_ = select(true, false, _e4747);
                                                                } else {
                                                                    phi_30201_ = type_56();
                                                                    phi_30202_ = true;
                                                                }
                                                                let _e4772 = phi_30201_;
                                                                let _e4774 = phi_30202_;
                                                                if _e4774 {
                                                                    phi_30207_ = _e4743;
                                                                } else {
                                                                    phi_30207_ = _e4772;
                                                                }
                                                                let _e4776 = phi_30207_;
                                                                if (_e4776.member_1 < 0f) {
                                                                    phi_30005_ = true;
                                                                } else {
                                                                    if (_e4776.member > _e4776.member_1) {
                                                                        phi_30002_ = true;
                                                                    } else {
                                                                        phi_30002_ = (_e4776.member > _e4617);
                                                                    }
                                                                    let _e4783 = phi_30002_;
                                                                    phi_30005_ = _e4783;
                                                                }
                                                                let _e4785 = phi_30005_;
                                                                phi_19005_ = select(_e4776.member, 300000000000000000000000000000000000000f, _e4785);
                                                            }
                                                            let _e4788 = phi_19005_;
                                                            if (_e4660.member_5 == 4294967295u) {
                                                                phi_19014_ = 300000000000000000000000000000000000000f;
                                                            } else {
                                                                let _e4800 = ((_e4660.member_4.x - _e1102) * _e4601.x);
                                                                let _e4802 = ((_e4660.member_4.y - _e1103) * _e4601.y);
                                                                let _e4804 = ((_e4660.member_4.z - _e1104) * _e4601.z);
                                                                let _e4811 = ((_e4660.member_6.x - _e1102) * _e4601.x);
                                                                let _e4812 = ((_e4660.member_6.y - _e1103) * _e4601.y);
                                                                let _e4813 = ((_e4660.member_6.z - _e1104) * _e4601.z);
                                                                if (_e4800 == _e4800) {
                                                                    let _e4815 = (_e4811 == _e4811);
                                                                    if _e4815 {
                                                                        let _e4816 = (_e4800 != _e4800);
                                                                        if _e4816 {
                                                                            phi_30368_ = true;
                                                                        } else {
                                                                            phi_30368_ = (_e4811 <= _e4800);
                                                                        }
                                                                        let _e4819 = phi_30368_;
                                                                        let _e4820 = select(_e4800, _e4811, _e4819);
                                                                        if _e4816 {
                                                                            phi_30398_ = true;
                                                                        } else {
                                                                            phi_30398_ = (_e4811 >= _e4800);
                                                                        }
                                                                        let _e4825 = phi_30398_;
                                                                        let _e4826 = select(_e4800, _e4811, _e4825);
                                                                        phi_30345_ = type_56(select(-300000000000000000000000000000000000000f, _e4820, (_e4820 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4826, (_e4826 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_30345_ = type_56();
                                                                    }
                                                                    let _e4831 = phi_30345_;
                                                                    phi_30348_ = _e4831;
                                                                    phi_30349_ = select(true, false, _e4815);
                                                                } else {
                                                                    phi_30348_ = type_56();
                                                                    phi_30349_ = true;
                                                                }
                                                                let _e4834 = phi_30348_;
                                                                let _e4836 = phi_30349_;
                                                                if _e4836 {
                                                                    phi_30354_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_30354_ = _e4834;
                                                                }
                                                                let _e4838 = phi_30354_;
                                                                if (_e4802 == _e4802) {
                                                                    let _e4842 = (_e4812 == _e4812);
                                                                    if _e4842 {
                                                                        let _e4843 = (_e4802 != _e4802);
                                                                        if _e4843 {
                                                                            phi_30455_ = true;
                                                                        } else {
                                                                            phi_30455_ = (_e4812 <= _e4802);
                                                                        }
                                                                        let _e4846 = phi_30455_;
                                                                        let _e4847 = select(_e4802, _e4812, _e4846);
                                                                        if (_e4838.member != _e4838.member) {
                                                                            phi_30470_ = true;
                                                                        } else {
                                                                            phi_30470_ = (_e4847 >= _e4838.member);
                                                                        }
                                                                        let _e4851 = phi_30470_;
                                                                        if _e4843 {
                                                                            phi_30485_ = true;
                                                                        } else {
                                                                            phi_30485_ = (_e4812 >= _e4802);
                                                                        }
                                                                        let _e4855 = phi_30485_;
                                                                        let _e4856 = select(_e4802, _e4812, _e4855);
                                                                        if (_e4838.member_1 != _e4838.member_1) {
                                                                            phi_30500_ = true;
                                                                        } else {
                                                                            phi_30500_ = (_e4856 <= _e4838.member_1);
                                                                        }
                                                                        let _e4860 = phi_30500_;
                                                                        phi_30432_ = type_56(select(_e4838.member, _e4847, _e4851), select(_e4838.member_1, _e4856, _e4860));
                                                                    } else {
                                                                        phi_30432_ = type_56();
                                                                    }
                                                                    let _e4864 = phi_30432_;
                                                                    phi_30435_ = _e4864;
                                                                    phi_30436_ = select(true, false, _e4842);
                                                                } else {
                                                                    phi_30435_ = type_56();
                                                                    phi_30436_ = true;
                                                                }
                                                                let _e4867 = phi_30435_;
                                                                let _e4869 = phi_30436_;
                                                                if _e4869 {
                                                                    phi_30441_ = _e4838;
                                                                } else {
                                                                    phi_30441_ = _e4867;
                                                                }
                                                                let _e4871 = phi_30441_;
                                                                if (_e4804 == _e4804) {
                                                                    let _e4875 = (_e4813 == _e4813);
                                                                    if _e4875 {
                                                                        let _e4876 = (_e4804 != _e4804);
                                                                        if _e4876 {
                                                                            phi_30542_ = true;
                                                                        } else {
                                                                            phi_30542_ = (_e4813 <= _e4804);
                                                                        }
                                                                        let _e4879 = phi_30542_;
                                                                        let _e4880 = select(_e4804, _e4813, _e4879);
                                                                        if (_e4871.member != _e4871.member) {
                                                                            phi_30557_ = true;
                                                                        } else {
                                                                            phi_30557_ = (_e4880 >= _e4871.member);
                                                                        }
                                                                        let _e4884 = phi_30557_;
                                                                        if _e4876 {
                                                                            phi_30572_ = true;
                                                                        } else {
                                                                            phi_30572_ = (_e4813 >= _e4804);
                                                                        }
                                                                        let _e4888 = phi_30572_;
                                                                        let _e4889 = select(_e4804, _e4813, _e4888);
                                                                        if (_e4871.member_1 != _e4871.member_1) {
                                                                            phi_30587_ = true;
                                                                        } else {
                                                                            phi_30587_ = (_e4889 <= _e4871.member_1);
                                                                        }
                                                                        let _e4893 = phi_30587_;
                                                                        phi_30519_ = type_56(select(_e4871.member, _e4880, _e4884), select(_e4871.member_1, _e4889, _e4893));
                                                                    } else {
                                                                        phi_30519_ = type_56();
                                                                    }
                                                                    let _e4897 = phi_30519_;
                                                                    phi_30522_ = _e4897;
                                                                    phi_30523_ = select(true, false, _e4875);
                                                                } else {
                                                                    phi_30522_ = type_56();
                                                                    phi_30523_ = true;
                                                                }
                                                                let _e4900 = phi_30522_;
                                                                let _e4902 = phi_30523_;
                                                                if _e4902 {
                                                                    phi_30528_ = _e4871;
                                                                } else {
                                                                    phi_30528_ = _e4900;
                                                                }
                                                                let _e4904 = phi_30528_;
                                                                if (_e4904.member_1 < 0f) {
                                                                    phi_30326_ = true;
                                                                } else {
                                                                    if (_e4904.member > _e4904.member_1) {
                                                                        phi_30323_ = true;
                                                                    } else {
                                                                        phi_30323_ = (_e4904.member > _e4617);
                                                                    }
                                                                    let _e4911 = phi_30323_;
                                                                    phi_30326_ = _e4911;
                                                                }
                                                                let _e4913 = phi_30326_;
                                                                phi_19014_ = select(_e4904.member, 300000000000000000000000000000000000000f, _e4913);
                                                            }
                                                            let _e4916 = phi_19014_;
                                                            if (_e4660.member_3 > 0u) {
                                                                phi_19025_ = ((2147483648u | ((_e4660.member_3 - 1u) << bitcast<u32>(28u))) | _e4660.member_1);
                                                            } else {
                                                                phi_19025_ = _e4660.member_1;
                                                            }
                                                            let _e4925 = phi_19025_;
                                                            if (_e4660.member_7 > 0u) {
                                                                phi_19036_ = ((2147483648u | ((_e4660.member_7 - 1u) << bitcast<u32>(28u))) | _e4660.member_5);
                                                            } else {
                                                                phi_19036_ = _e4660.member_5;
                                                            }
                                                            let _e4934 = phi_19036_;
                                                            let _e4935 = (_e4788 <= _e4916);
                                                            let _e4939 = select(_e4788, _e4916, _e4935);
                                                            if (_e4939 < 300000000000000000000000000000000000000f) {
                                                                let _e4941 = (_e4644 < 64u);
                                                                if _e4941 {
                                                                    if _e4941 {
                                                                    } else {
                                                                        phi_35913_ = true;
                                                                        phi_21555_ = bool();
                                                                        phi_21554_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    local_67[_e4644] = select(_e4925, _e4934, _e4935);
                                                                    if _e4941 {
                                                                    } else {
                                                                        phi_35913_ = true;
                                                                        phi_21555_ = bool();
                                                                        phi_21554_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    local_68[_e4644] = _e4939;
                                                                    phi_19075_ = (_e4644 + 1u);
                                                                } else {
                                                                    phi_19075_ = _e4644;
                                                                }
                                                                let _e4946 = phi_19075_;
                                                                phi_19076_ = _e4946;
                                                            } else {
                                                                phi_19076_ = _e4644;
                                                            }
                                                            let _e4948 = phi_19076_;
                                                            phi_19087_ = select(4294967295u, select(_e4934, _e4925, _e4935), (select(_e4916, _e4788, _e4935) < 300000000000000000000000000000000000000f));
                                                            phi_19088_ = _e4948;
                                                            phi_19089_ = u32();
                                                            phi_19090_ = u32();
                                                        } else {
                                                            let _e4653 = (_e4642 & 268435455u);
                                                            phi_19087_ = u32();
                                                            phi_19088_ = u32();
                                                            phi_19089_ = ((_e4653 + ((_e4642 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                            phi_19090_ = _e4653;
                                                        }
                                                        let _e4952 = phi_19087_;
                                                        let _e4954 = phi_19088_;
                                                        let _e4956 = phi_19089_;
                                                        let _e4958 = phi_19090_;
                                                        phi_19092_ = _e4952;
                                                        phi_19093_ = _e4954;
                                                        phi_19095_ = _e4956;
                                                        phi_19096_ = _e4958;
                                                        phi_19097_ = select(true, false, _e4652);
                                                        phi_19098_ = _e4652;
                                                    } else {
                                                        phi_19092_ = u32();
                                                        phi_19093_ = u32();
                                                        phi_19095_ = _e4611;
                                                        phi_19096_ = _e4613;
                                                        phi_19097_ = _e4648;
                                                        phi_19098_ = false;
                                                    }
                                                    let _e4961 = phi_19092_;
                                                    let _e4963 = phi_19093_;
                                                    let _e4965 = phi_19095_;
                                                    let _e4967 = phi_19096_;
                                                    let _e4969 = phi_19097_;
                                                    let _e4971 = phi_19098_;
                                                    phi_18565_ = _e4605;
                                                    phi_18568_ = select(_e4961, 4294967295u, _e4969);
                                                    phi_18570_ = select(_e4963, select(_e4640, _e4644, _e4646), _e4969);
                                                    phi_18572_ = select(_e4611, _e4965, _e4969);
                                                    phi_18574_ = select(_e4613, _e4967, _e4969);
                                                    phi_18576_ = _e4615;
                                                    phi_18578_ = _e4617;
                                                    phi_19109_ = vec4<f32>();
                                                    phi_19110_ = select(_e4971, true, _e4969);
                                                    phi_19111_ = select(select(_e4650, false, _e4646), false, _e4969);
                                                }
                                                let _e5202 = phi_18565_;
                                                let _e5204 = phi_18568_;
                                                let _e5206 = phi_18570_;
                                                let _e5208 = phi_18572_;
                                                let _e5210 = phi_18574_;
                                                let _e5212 = phi_18576_;
                                                let _e5214 = phi_18578_;
                                                let _e5216 = phi_19109_;
                                                let _e5218 = phi_19110_;
                                                let _e5220 = phi_19111_;
                                                phi_18564_ = _e5202;
                                                phi_18567_ = _e5204;
                                                phi_18569_ = _e5206;
                                                phi_18571_ = _e5208;
                                                phi_18573_ = _e5210;
                                                phi_18575_ = _e5212;
                                                phi_18577_ = _e5214;
                                                phi_35913_ = _e1982;
                                                phi_21555_ = _e5220;
                                                phi_21554_ = _e5216;
                                                if !(_e5218) {
                                                    break;
                                                }
                                            }
                                            let _e5223 = phi_35913_;
                                            let _e5225 = phi_21555_;
                                            let _e5227 = phi_21554_;
                                            phi_36360_ = _e4058;
                                            phi_36352_ = _e5223;
                                            if _e5223 {
                                                break;
                                            }
                                            if _e5225 {
                                                let _e5229 = local_80;
                                                let _e5231 = local_81;
                                                phi_19116_ = type_74(_e5229, _e5231);
                                            } else {
                                                phi_19116_ = type_74();
                                            }
                                            let _e5234 = phi_19116_;
                                            if select(true, false, _e5225) {
                                                let _e5237 = local_82;
                                                phi_19122_ = type_74(_e5237, _e5227);
                                            } else {
                                                phi_19122_ = _e5234;
                                            }
                                            let _e5240 = phi_19122_;
                                            if (_e5240.member != 4294967295u) {
                                                phi_19176_ = _e1986;
                                                phi_19177_ = _e1988;
                                            } else {
                                                if (_e1990 == (_e1980 + 2u)) {
                                                    phi_19174_ = _e1986;
                                                    phi_19175_ = type_41(vec4<f32>(_e1102, _e1103, _e1104, -1f), vec4<f32>(_e4062.x, _e4062.y, _e4062.z, 0f), vec4<f32>(((_e820.x * _e4574.member.x) * _e4590), ((_e820.y * _e4574.member.y) * _e4590), ((_e820.z * _e4574.member.z) * _e4590), 0f));
                                                } else {
                                                    phi_19174_ = vec3<f32>((_e1986.x + (((_e820.x * _e4574.member.x) * _e4064.x) * _e4590)), (_e1986.y + (((_e820.y * _e4574.member.y) * _e4064.y) * _e4590)), (_e1986.z + (((_e820.z * _e4574.member.z) * _e4064.z) * _e4590)));
                                                    phi_19175_ = _e1988;
                                                }
                                                let _e5278 = phi_19174_;
                                                let _e5280 = phi_19175_;
                                                phi_19176_ = _e5278;
                                                phi_19177_ = _e5280;
                                            }
                                            let _e5282 = phi_19176_;
                                            let _e5284 = phi_19177_;
                                            phi_36355_ = _e5223;
                                            phi_19178_ = _e5282;
                                            phi_19179_ = _e5284;
                                            phi_19180_ = (_e1992 + 1u);
                                        } else {
                                            phi_36355_ = _e1982;
                                            phi_19178_ = _e1986;
                                            phi_19179_ = _e1988;
                                            phi_19180_ = _e1992;
                                        }
                                        let _e5286 = phi_36355_;
                                        let _e5288 = phi_19178_;
                                        let _e5290 = phi_19179_;
                                        let _e5292 = phi_19180_;
                                        phi_36354_ = _e5286;
                                        phi_19181_ = _e5288;
                                        phi_19182_ = _e5290;
                                        phi_19183_ = _e5292;
                                    } else {
                                        phi_36354_ = _e1982;
                                        phi_19181_ = _e1986;
                                        phi_19182_ = _e1988;
                                        phi_19183_ = _e1992;
                                    }
                                    let _e5294 = phi_36354_;
                                    let _e5296 = phi_19181_;
                                    let _e5298 = phi_19182_;
                                    let _e5300 = phi_19183_;
                                    phi_36640_ = _e4058;
                                    phi_36353_ = _e5294;
                                    phi_14601_ = _e5296;
                                    phi_14604_ = _e5298;
                                    phi_14606_ = (_e1990 + 1u);
                                    phi_14608_ = _e5300;
                                } else {
                                    phi_36640_ = _e1984;
                                    phi_36353_ = _e1982;
                                    phi_14601_ = vec3<f32>();
                                    phi_14604_ = type_41();
                                    phi_14606_ = u32();
                                    phi_14608_ = u32();
                                }
                                let _e5303 = phi_36640_;
                                let _e5305 = phi_36353_;
                                let _e5307 = phi_14601_;
                                let _e5309 = phi_14604_;
                                let _e5311 = phi_14606_;
                                let _e5313 = phi_14608_;
                                phi_35985_ = _e5305;
                                phi_34060_ = _e5303;
                                phi_14600_ = _e5307;
                                phi_14603_ = _e5309;
                                phi_14605_ = _e5311;
                                phi_14607_ = _e5313;
                                phi_36360_ = _e5303;
                                phi_36352_ = _e5305;
                                if !(_e1994) {
                                    break;
                                }
                            }
                            let _e5316 = phi_36360_;
                            let _e5318 = phi_36352_;
                            if _e5318 {
                                break;
                            }
                            if (_e356 >= _e373) {
                                let _e6322 = local_98;
                                let _e6325 = local_99;
                                let _e6328 = local_100;
                                let _e6331 = local_101;
                                let _e6337 = local_102;
                                phi_19734_ = type_69(type_34(_e331.member, _e331.member_1, vec4<f32>(_e820.x, _e820.y, _e820.z, f32(_e356)), vec4<f32>(_e6325.x, _e6328.y, _e6331.z, f32(_e6322)), vec4<u32>(_e5316, (_e1977 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6337);
                                phi_19735_ = type_69();
                                phi_19736_ = true;
                            } else {
                                let _e5321 = ((_e5316 * 747796405u) + 2891336453u);
                                let _e5329 = (((_e5321 >> bitcast<u32>((((_e5321 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5321) * 277803737u);
                                let _e5332 = ((_e5329 >> bitcast<u32>(22i)) ^ _e5329);
                                let _e5335 = f32((_e5332 >> bitcast<u32>(8i)));
                                let _e5338 = ((_e5332 * 747796405u) + 2891336453u);
                                let _e5346 = (((_e5338 >> bitcast<u32>((((_e5338 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5338) * 277803737u);
                                let _e5349 = ((_e5346 >> bitcast<u32>(22i)) ^ _e5346);
                                let _e5352 = f32((_e5349 >> bitcast<u32>(8i)));
                                let _e5354 = ((_e5349 * 747796405u) + 2891336453u);
                                let _e5362 = (((_e5354 >> bitcast<u32>((((_e5354 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5354) * 277803737u);
                                let _e5365 = ((_e5362 >> bitcast<u32>(22i)) ^ _e5362);
                                let _e5370 = (_e1969 == 0u);
                                if _e5370 {
                                    phi_30697_ = vec3<f32>();
                                    phi_30698_ = true;
                                } else {
                                    if (_e1969 == 2u) {
                                        let _e5376 = (((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z));
                                        if (_e5376 != _e5376) {
                                            phi_30738_ = true;
                                        } else {
                                            phi_30738_ = (0f >= _e5376);
                                        }
                                        let _e5380 = phi_30738_;
                                        let _e5381 = select(_e5376, 0f, _e5380);
                                        let _e5382 = (1f / _e1959);
                                        let _e5386 = ((_e5382 * _e5382) * (1f - (_e5381 * _e5381)));
                                        if (_e5386 >= 1f) {
                                            phi_30757_ = 1f;
                                        } else {
                                            let _e5389 = sqrt((1f - _e5386));
                                            if (_e5381 != _e5381) {
                                                phi_30768_ = true;
                                            } else {
                                                phi_30768_ = (_e5389 <= _e5381);
                                            }
                                            let _e5393 = phi_30768_;
                                            let _e5398 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                            phi_30757_ = (_e5398 + ((1f - _e5398) * pow((1f - select(_e5381, _e5389, _e5393)), 5f)));
                                        }
                                        let _e5405 = phi_30757_;
                                        phi_30688_ = select(true, false, ((f32((_e5365 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5405));
                                    } else {
                                        phi_30688_ = true;
                                    }
                                    let _e5409 = phi_30688_;
                                    if _e5409 {
                                        let _e5414 = (((_e1051 * _e1125) + (_e1052 * _e1126)) + (_e1053 * _e1128));
                                        let _e5419 = (((_e1051 * _e1121) + (_e1052 * _e1131)) + (_e1053 * _e1132));
                                        let _e5424 = (((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z));
                                        if (_e5424 <= 0f) {
                                            phi_30922_ = vec3<f32>();
                                            phi_30923_ = true;
                                        } else {
                                            let _e5427 = (_e1967 * _e5414);
                                            let _e5428 = (_e1967 * _e5419);
                                            let _e5435 = (1f / sqrt((((_e5427 * _e5427) + (_e5428 * _e5428)) + (_e5424 * _e5424))));
                                            let _e5436 = (_e5427 * _e5435);
                                            let _e5437 = (_e5428 * _e5435);
                                            let _e5438 = (_e5424 * _e5435);
                                            let _e5441 = ((_e5436 * _e5436) + (_e5437 * _e5437));
                                            if (_e5441 > 0f) {
                                                let _e5444 = sqrt(_e5441);
                                                phi_30824_ = vec3<f32>((-(_e5437) / _e5444), (_e5436 / _e5444), 0f);
                                            } else {
                                                phi_30824_ = vec3<f32>(1f, 0f, 0f);
                                            }
                                            let _e5449 = phi_30824_;
                                            let _e5462 = sqrt((_e5335 * 0.000000059604645f));
                                            let _e5463 = (_e5352 * 0.00000037450704f);
                                            let _e5465 = (_e5462 * cos(_e5463));
                                            let _e5467 = (0.5f * (1f + _e5438));
                                            let _e5470 = (1f - (_e5465 * _e5465));
                                            if (_e5470 != _e5470) {
                                                phi_30980_ = true;
                                            } else {
                                                phi_30980_ = (0f >= _e5470);
                                            }
                                            let _e5474 = phi_30980_;
                                            let _e5481 = (((1f - _e5467) * sqrt(select(_e5470, 0f, _e5474))) + ((_e5467 * _e5462) * sin(_e5463)));
                                            let _e5492 = (_e5470 - (_e5481 * _e5481));
                                            if (_e5492 != _e5492) {
                                                phi_30995_ = true;
                                            } else {
                                                phi_30995_ = (0f >= _e5492);
                                            }
                                            let _e5496 = phi_30995_;
                                            let _e5498 = sqrt(select(_e5492, 0f, _e5496));
                                            let _e5504 = (((_e5449.z * _e5465) + (((_e5436 * _e5449.y) - (_e5449.x * _e5437)) * _e5481)) + (_e5438 * _e5498));
                                            let _e5505 = (_e1967 * (((_e5449.x * _e5465) + (((_e5437 * _e5449.z) - (_e5449.y * _e5438)) * _e5481)) + (_e5436 * _e5498)));
                                            let _e5506 = (_e1967 * (((_e5449.y * _e5465) + (((_e5438 * _e5449.x) - (_e5449.z * _e5436)) * _e5481)) + (_e5437 * _e5498)));
                                            if (_e5504 != _e5504) {
                                                phi_31010_ = true;
                                            } else {
                                                phi_31010_ = (0f >= _e5504);
                                            }
                                            let _e5510 = phi_31010_;
                                            let _e5511 = select(_e5504, 0f, _e5510);
                                            let _e5518 = (1f / sqrt((((_e5505 * _e5505) + (_e5506 * _e5506)) + (_e5511 * _e5511))));
                                            let _e5519 = (_e5505 * _e5518);
                                            let _e5520 = (_e5506 * _e5518);
                                            let _e5521 = (_e5511 * _e5518);
                                            let _e5527 = (2f * (((_e5414 * _e5519) + (_e5419 * _e5520)) + (_e5424 * _e5521)));
                                            let _e5530 = (_e5521 * _e5527);
                                            let _e5533 = (vec3<f32>((_e5519 * _e5527), (_e5520 * _e5527), _e5530) - vec3<f32>(_e5414, _e5419, _e5424));
                                            let _e5534 = ((_e5530 - _e5424) <= 0f);
                                            if _e5534 {
                                                phi_30919_ = vec3<f32>();
                                            } else {
                                                phi_30919_ = vec3<f32>((((_e1125 * _e5533.x) + (_e1121 * _e5533.y)) + (_e1077.x * _e5533.z)), (((_e1126 * _e5533.x) + (_e1131 * _e5533.y)) + (_e1077.y * _e5533.z)), (((_e1128 * _e5533.x) + (_e1132 * _e5533.y)) + (_e1077.z * _e5533.z)));
                                            }
                                            let _e5555 = phi_30919_;
                                            phi_30922_ = _e5555;
                                            phi_30923_ = _e5534;
                                        }
                                        let _e5557 = phi_30922_;
                                        let _e5559 = phi_30923_;
                                        phi_30693_ = select(_e5557, vec3<f32>(0f, 0f, 0f), vec3(_e5559));
                                    } else {
                                        phi_30693_ = vec3<f32>();
                                    }
                                    let _e5563 = phi_30693_;
                                    phi_30697_ = _e5563;
                                    phi_30698_ = select(true, false, _e5409);
                                }
                                let _e5566 = phi_30697_;
                                let _e5568 = phi_30698_;
                                if _e5568 {
                                    let _e5569 = (_e5335 * 0.00000037450704f);
                                    let _e5571 = ((_e5352 * 0.00000011920929f) - 1f);
                                    let _e5573 = (1f - (_e5571 * _e5571));
                                    if (_e5573 != _e5573) {
                                        phi_31110_ = true;
                                    } else {
                                        phi_31110_ = (0f >= _e5573);
                                    }
                                    let _e5577 = phi_31110_;
                                    let _e5579 = sqrt(select(_e5573, 0f, _e5577));
                                    let _e5584 = (_e1077.x + (_e5579 * cos(_e5569)));
                                    let _e5585 = (_e1077.y + (_e5579 * sin(_e5569)));
                                    let _e5586 = (_e1077.z + _e5571);
                                    let _e5592 = sqrt((((_e5584 * _e5584) + (_e5585 * _e5585)) + (_e5586 * _e5586)));
                                    if (_e5592 < 0.000001f) {
                                        phi_31084_ = _e1077;
                                    } else {
                                        let _e5594 = (1f / _e5592);
                                        phi_31084_ = vec3<f32>((_e5584 * _e5594), (_e5585 * _e5594), (_e5586 * _e5594));
                                    }
                                    let _e5600 = phi_31084_;
                                    phi_30705_ = _e5600;
                                } else {
                                    phi_30705_ = _e5566;
                                }
                                let _e5602 = phi_30705_;
                                let _e5610 = (((_e5602.x * _e1077.x) + (_e5602.y * _e1077.y)) + (_e5602.z * _e1077.z));
                                if (_e5610 != _e5610) {
                                    phi_31125_ = true;
                                } else {
                                    phi_31125_ = (0f >= _e5610);
                                }
                                let _e5614 = phi_31125_;
                                let _e5616 = (select(_e5610, 0f, _e5614) * 0.31830987f);
                                if _e5370 {
                                    if (_e5610 <= 0f) {
                                        phi_31922_ = vec3<f32>(0f, 0f, 0f);
                                    } else {
                                        phi_31922_ = (_e1963 * (_e5610 * 0.31830987f));
                                    }
                                    let _e6091 = phi_31922_;
                                    phi_19575_ = type_59(_e6091, _e5616);
                                    phi_19576_ = type_59();
                                    phi_19577_ = true;
                                } else {
                                    let _e5621 = (((_e1051 * _e1125) + (_e1052 * _e1126)) + (_e1053 * _e1128));
                                    let _e5626 = (((_e1051 * _e1121) + (_e1052 * _e1131)) + (_e1053 * _e1132));
                                    let _e5631 = (((_e1051 * _e1077.x) + (_e1052 * _e1077.y)) + (_e1053 * _e1077.z));
                                    let _e5636 = (((_e5602.x * _e1125) + (_e5602.y * _e1126)) + (_e5602.z * _e1128));
                                    let _e5641 = (((_e5602.x * _e1121) + (_e5602.y * _e1131)) + (_e5602.z * _e1132));
                                    if (_e5631 <= 0f) {
                                        phi_19539_ = type_59();
                                        phi_19540_ = true;
                                    } else {
                                        let _e5643 = (_e5610 <= 0f);
                                        if _e5643 {
                                            phi_19538_ = type_59();
                                        } else {
                                            let _e5644 = (_e5621 + _e5636);
                                            let _e5645 = (_e5626 + _e5641);
                                            let _e5646 = (_e5631 + _e5610);
                                            let _e5653 = (1f / sqrt((((_e5644 * _e5644) + (_e5645 * _e5645)) + (_e5646 * _e5646))));
                                            let _e5656 = (_e5646 * _e5653);
                                            if (_e5656 <= 0f) {
                                                phi_31235_ = 0f;
                                            } else {
                                                let _e5658 = (_e1967 * _e1967);
                                                let _e5662 = ((((_e5658 - 1f) * _e5656) * _e5656) + 1f);
                                                phi_31235_ = (_e5658 / ((3.1415927f * _e5662) * _e5662));
                                            }
                                            let _e5667 = phi_31235_;
                                            let _e5668 = (_e5631 * _e5631);
                                            if (_e5668 >= 1f) {
                                                phi_31263_ = 0f;
                                            } else {
                                                phi_31263_ = ((sqrt((1f + (((_e1967 * _e1967) * ((_e5621 * _e5621) + (_e5626 * _e5626))) / _e5668))) - 1f) * 0.5f);
                                            }
                                            let _e5681 = phi_31263_;
                                            let _e5682 = (1f + _e5681);
                                            let _e5683 = (_e5610 * _e5610);
                                            if (_e5683 >= 1f) {
                                                phi_31291_ = 0f;
                                            } else {
                                                phi_31291_ = ((sqrt((1f + (((_e1967 * _e1967) * ((_e5636 * _e5636) + (_e5641 * _e5641))) / _e5683))) - 1f) * 0.5f);
                                            }
                                            let _e5696 = phi_31291_;
                                            let _e5703 = (((_e5621 * (_e5644 * _e5653)) + (_e5626 * (_e5645 * _e5653))) + (_e5631 * _e5656));
                                            if (_e1965 == 0u) {
                                                let _e5998 = (1f - _e5703);
                                                if (_e5998 != _e5998) {
                                                    phi_31794_ = true;
                                                } else {
                                                    phi_31794_ = (0f >= _e5998);
                                                }
                                                let _e6002 = phi_31794_;
                                                let _e6004 = pow(select(_e5998, 0f, _e6002), 5f);
                                                let _e6005 = (1f - _e6004);
                                                phi_19526_ = vec3<f32>(((_e1963.x * _e6005) + _e6004), ((_e1963.y * _e6005) + _e6004), ((_e1963.z * _e6005) + _e6004));
                                            } else {
                                                if (_e1965 == 1u) {
                                                    let _e5971 = (1f / _e1959);
                                                    let _e5975 = ((_e5971 * _e5971) * (1f - (_e5703 * _e5703)));
                                                    if (_e5975 >= 1f) {
                                                        phi_31755_ = 1f;
                                                    } else {
                                                        let _e5978 = sqrt((1f - _e5975));
                                                        if (_e5703 != _e5703) {
                                                            phi_31766_ = true;
                                                        } else {
                                                            phi_31766_ = (_e5978 <= _e5703);
                                                        }
                                                        let _e5982 = phi_31766_;
                                                        let _e5987 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                                        phi_31755_ = (_e5987 + ((1f - _e5987) * pow((1f - select(_e5703, _e5978, _e5982)), 5f)));
                                                    }
                                                    let _e5994 = phi_31755_;
                                                    phi_19525_ = vec3(_e5994);
                                                } else {
                                                    let _e5707 = select(_e5703, 0f, (_e5703 < 0f));
                                                    let _e5709 = select(_e5707, 1f, (_e5707 > 1f));
                                                    let _e5711 = (1f - (_e5709 * _e5709));
                                                    let _e5724 = ((_e1963.x * _e1963.x) - (_e1961.x * _e1961.x));
                                                    let _e5725 = ((_e1963.y * _e1963.y) - (_e1961.y * _e1961.y));
                                                    let _e5726 = ((_e1963.z * _e1963.z) - (_e1961.z * _e1961.z));
                                                    let _e5728 = (_e1963.x * (_e1961.x + _e1961.x));
                                                    let _e5730 = (_e1963.y * (_e1961.y + _e1961.y));
                                                    let _e5732 = (_e1963.z * (_e1961.z + _e1961.z));
                                                    let _e5739 = ((_e5724 * _e5724) + (_e5728 * _e5728));
                                                    let _e5740 = ((_e5725 * _e5725) + (_e5730 * _e5730));
                                                    let _e5741 = ((_e5726 * _e5726) + (_e5732 * _e5732));
                                                    let _e5746 = ((_e5711 * _e5724) / _e5739);
                                                    let _e5747 = ((_e5711 * _e5725) / _e5740);
                                                    let _e5748 = ((_e5711 * _e5726) / _e5741);
                                                    let _e5749 = (1f - _e5746);
                                                    let _e5750 = (1f - _e5747);
                                                    let _e5751 = (1f - _e5748);
                                                    let _e5753 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5746, _e5747, _e5748));
                                                    let _e5754 = (_e5711 * _e5728);
                                                    let _e5755 = (_e5711 * _e5730);
                                                    let _e5756 = (_e5711 * _e5732);
                                                    let _e5761 = (vec3<f32>(_e5754, _e5755, _e5756) / vec3<f32>(_e5739, _e5740, _e5741));
                                                    let _e5774 = ((_e5753.x * _e5753.x) + (_e5761.x * _e5761.x));
                                                    let _e5775 = ((_e5753.y * _e5753.y) + (_e5761.y * _e5761.y));
                                                    let _e5776 = ((_e5753.z * _e5753.z) + (_e5761.z * _e5761.z));
                                                    if (_e5774 != _e5774) {
                                                        phi_31425_ = true;
                                                    } else {
                                                        phi_31425_ = (0f >= _e5774);
                                                    }
                                                    let _e5780 = phi_31425_;
                                                    let _e5782 = sqrt(select(_e5774, 0f, _e5780));
                                                    if (_e5775 != _e5775) {
                                                        phi_31440_ = true;
                                                    } else {
                                                        phi_31440_ = (0f >= _e5775);
                                                    }
                                                    let _e5786 = phi_31440_;
                                                    let _e5788 = sqrt(select(_e5775, 0f, _e5786));
                                                    if (_e5776 != _e5776) {
                                                        phi_31455_ = true;
                                                    } else {
                                                        phi_31455_ = (0f >= _e5776);
                                                    }
                                                    let _e5792 = phi_31455_;
                                                    let _e5794 = sqrt(select(_e5776, 0f, _e5792));
                                                    let _e5798 = ((_e5782 + _e5749) * 0.5f);
                                                    let _e5799 = ((_e5788 + _e5750) * 0.5f);
                                                    let _e5800 = ((_e5794 + _e5751) * 0.5f);
                                                    if (_e5798 != _e5798) {
                                                        phi_31482_ = true;
                                                    } else {
                                                        phi_31482_ = (0f >= _e5798);
                                                    }
                                                    let _e5804 = phi_31482_;
                                                    let _e5806 = sqrt(select(_e5798, 0f, _e5804));
                                                    if (_e5799 != _e5799) {
                                                        phi_31497_ = true;
                                                    } else {
                                                        phi_31497_ = (0f >= _e5799);
                                                    }
                                                    let _e5810 = phi_31497_;
                                                    let _e5812 = sqrt(select(_e5799, 0f, _e5810));
                                                    if (_e5800 != _e5800) {
                                                        phi_31512_ = true;
                                                    } else {
                                                        phi_31512_ = (0f >= _e5800);
                                                    }
                                                    let _e5816 = phi_31512_;
                                                    let _e5818 = sqrt(select(_e5800, 0f, _e5816));
                                                    let _e5819 = vec3<f32>(_e5806, _e5812, _e5818);
                                                    let _e5823 = ((_e5782 - _e5749) * 0.5f);
                                                    let _e5824 = ((_e5788 - _e5750) * 0.5f);
                                                    let _e5825 = ((_e5794 - _e5751) * 0.5f);
                                                    if (_e5823 != _e5823) {
                                                        phi_31539_ = true;
                                                    } else {
                                                        phi_31539_ = (0f >= _e5823);
                                                    }
                                                    let _e5829 = phi_31539_;
                                                    let _e5831 = sqrt(select(_e5823, 0f, _e5829));
                                                    if (_e5824 != _e5824) {
                                                        phi_31554_ = true;
                                                    } else {
                                                        phi_31554_ = (0f >= _e5824);
                                                    }
                                                    let _e5835 = phi_31554_;
                                                    let _e5837 = sqrt(select(_e5824, 0f, _e5835));
                                                    if (_e5825 != _e5825) {
                                                        phi_31569_ = true;
                                                    } else {
                                                        phi_31569_ = (0f >= _e5825);
                                                    }
                                                    let _e5841 = phi_31569_;
                                                    let _e5843 = sqrt(select(_e5825, 0f, _e5841));
                                                    if ((_e5754 / _e5739) < 0f) {
                                                        phi_19402_ = -(_e5831);
                                                    } else {
                                                        phi_19402_ = _e5831;
                                                    }
                                                    let _e5847 = phi_19402_;
                                                    if ((_e5755 / _e5740) < 0f) {
                                                        phi_19410_ = -(_e5837);
                                                    } else {
                                                        phi_19410_ = _e5837;
                                                    }
                                                    let _e5851 = phi_19410_;
                                                    if ((_e5756 / _e5741) < 0f) {
                                                        phi_19418_ = -(_e5843);
                                                    } else {
                                                        phi_19418_ = _e5843;
                                                    }
                                                    let _e5855 = phi_19418_;
                                                    let _e5859 = (_e1961.x * _e5709);
                                                    let _e5860 = (_e1961.y * _e5709);
                                                    let _e5861 = (_e1961.z * _e5709);
                                                    let _e5871 = vec3<f32>(((_e1963.x * _e5806) - (_e1961.x * _e5847)), ((_e1963.y * _e5812) - (_e1961.y * _e5851)), ((_e1963.z * _e5818) - (_e1961.z * _e5855)));
                                                    let _e5878 = ((_e1963.x * _e5847) + (_e1961.x * _e5806));
                                                    let _e5879 = ((_e1963.y * _e5851) + (_e1961.y * _e5812));
                                                    let _e5880 = ((_e1963.z * _e5855) + (_e1961.z * _e5818));
                                                    let _e5882 = vec3<f32>((_e1963.x * _e5709), (_e1963.y * _e5709), (_e1963.z * _e5709));
                                                    let _e5883 = (_e5882 - _e5819);
                                                    let _e5884 = (_e5859 - _e5847);
                                                    let _e5885 = (_e5860 - _e5851);
                                                    let _e5886 = (_e5861 - _e5855);
                                                    let _e5899 = (_e5882 + _e5819);
                                                    let _e5900 = (_e5859 + _e5847);
                                                    let _e5901 = (_e5860 + _e5851);
                                                    let _e5902 = (_e5861 + _e5855);
                                                    let _e5912 = ((_e5899.x * _e5899.x) + (_e5900 * _e5900));
                                                    let _e5913 = ((_e5899.y * _e5899.y) + (_e5901 * _e5901));
                                                    let _e5914 = ((_e5899.z * _e5899.z) + (_e5902 * _e5902));
                                                    let _e5924 = vec3(_e5709);
                                                    let _e5925 = (_e5924 - _e5871);
                                                    let _e5926 = -(vec3<f32>(_e5878, _e5879, _e5880));
                                                    let _e5942 = (_e5924 + _e5871);
                                                    let _e5952 = ((_e5942.x * _e5942.x) + (_e5878 * _e5878));
                                                    let _e5953 = ((_e5942.y * _e5942.y) + (_e5879 * _e5879));
                                                    let _e5954 = ((_e5942.z * _e5942.z) + (_e5880 * _e5880));
                                                    phi_19525_ = vec3<f32>((((((_e5883.x * _e5883.x) + (_e5884 * _e5884)) / select(0.000000000000000000000000000001f, _e5912, (_e5912 > 0.000000000000000000000000000001f))) + (((_e5925.x * _e5925.x) + (_e5926.x * _e5926.x)) / select(0.000000000000000000000000000001f, _e5952, (_e5952 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5883.y * _e5883.y) + (_e5885 * _e5885)) / select(0.000000000000000000000000000001f, _e5913, (_e5913 > 0.000000000000000000000000000001f))) + (((_e5925.y * _e5925.y) + (_e5926.y * _e5926.y)) / select(0.000000000000000000000000000001f, _e5953, (_e5953 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5883.z * _e5883.z) + (_e5886 * _e5886)) / select(0.000000000000000000000000000001f, _e5914, (_e5914 > 0.000000000000000000000000000001f))) + (((_e5925.z * _e5925.z) + (_e5926.z * _e5926.z)) / select(0.000000000000000000000000000001f, _e5954, (_e5954 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e5997 = phi_19525_;
                                                phi_19526_ = _e5997;
                                            }
                                            let _e6017 = phi_19526_;
                                            let _e6019 = (4f * _e5631);
                                            phi_19538_ = type_59((_e6017 * (((1f / (_e5682 + _e5696)) * _e5667) / _e6019)), ((_e5667 / _e5682) / _e6019));
                                        }
                                        let _e6026 = phi_19538_;
                                        phi_19539_ = _e6026;
                                        phi_19540_ = _e5643;
                                    }
                                    let _e6028 = phi_19539_;
                                    let _e6030 = phi_19540_;
                                    if _e6030 {
                                        phi_19545_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                    } else {
                                        phi_19545_ = _e6028;
                                    }
                                    let _e6032 = phi_19545_;
                                    let _e6035 = (_e1969 == 1u);
                                    if _e6035 {
                                        phi_19573_ = _e6032;
                                        phi_19574_ = type_59();
                                    } else {
                                        if (_e5631 != _e5631) {
                                            phi_31831_ = true;
                                        } else {
                                            phi_31831_ = (0f >= _e5631);
                                        }
                                        let _e6039 = phi_31831_;
                                        let _e6040 = select(_e5631, 0f, _e6039);
                                        let _e6041 = (1f / _e1959);
                                        let _e6045 = ((_e6041 * _e6041) * (1f - (_e6040 * _e6040)));
                                        if (_e6045 >= 1f) {
                                            phi_31850_ = 1f;
                                        } else {
                                            let _e6048 = sqrt((1f - _e6045));
                                            if (_e6040 != _e6040) {
                                                phi_31861_ = true;
                                            } else {
                                                phi_31861_ = (_e6048 <= _e6040);
                                            }
                                            let _e6052 = phi_31861_;
                                            let _e6057 = pow(((1f - _e1959) / (1f + _e1959)), 2f);
                                            phi_31850_ = (_e6057 + ((1f - _e6057) * pow((1f - select(_e6040, _e6048, _e6052)), 5f)));
                                        }
                                        let _e6064 = phi_31850_;
                                        if (_e5610 <= 0f) {
                                            phi_31900_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_31900_ = (_e1957 * (_e5610 * 0.31830987f));
                                        }
                                        let _e6069 = phi_31900_;
                                        let _e6070 = (1f - _e6064);
                                        phi_19573_ = type_59();
                                        phi_19574_ = type_59((_e6032.member + vec3<f32>((_e6069.x * _e6070), (_e6069.y * _e6070), (_e6069.z * _e6070))), ((_e6064 * _e6032.member_1) + (_e6070 * _e5616)));
                                    }
                                    let _e6084 = phi_19573_;
                                    let _e6086 = phi_19574_;
                                    phi_19575_ = _e6084;
                                    phi_19576_ = _e6086;
                                    phi_19577_ = _e6035;
                                }
                                let _e6094 = phi_19575_;
                                let _e6096 = phi_19576_;
                                let _e6098 = phi_19577_;
                                if _e6098 {
                                    phi_19581_ = _e6094;
                                } else {
                                    phi_19581_ = _e6096;
                                }
                                let _e6100 = phi_19581_;
                                if ((((_e5602.x * _e5602.x) + (_e5602.y * _e5602.y)) + (_e5602.z * _e5602.z)) == 0f) {
                                    phi_38197_ = _e5365;
                                    phi_19720_ = type_69();
                                    phi_19721_ = type_69();
                                    phi_19722_ = true;
                                    phi_19723_ = false;
                                } else {
                                    let _e6109 = (_e6100.member_1 <= 0f);
                                    if _e6109 {
                                        phi_38198_ = _e5365;
                                        phi_19717_ = type_69();
                                        phi_19718_ = type_69();
                                        phi_19719_ = false;
                                    } else {
                                        let _e6111 = (_e6100.member.x / _e6100.member_1);
                                        let _e6113 = (_e6100.member.y / _e6100.member_1);
                                        let _e6115 = (_e6100.member.z / _e6100.member_1);
                                        let _e6124 = global_1.member.member_4[1u];
                                        if (sqrt((((_e6111 * _e6111) + (_e6113 * _e6113)) + (_e6115 * _e6115))) <= _e6124) {
                                            phi_38191_ = _e5365;
                                            phi_19668_ = _e820;
                                            phi_19669_ = false;
                                        } else {
                                            let _e6128 = global_1.member.member_7[1u];
                                            if (_e6128 == 1u) {
                                                let _e6133 = global_1.member.member_4[3u];
                                                if (f32(_e356) >= _e6133) {
                                                    let _e6136 = select(_e6113, _e6111, (_e6111 > _e6113));
                                                    let _e6138 = select(_e6115, _e6136, (_e6136 > _e6115));
                                                    if (_e6138 != _e6138) {
                                                        phi_31947_ = true;
                                                    } else {
                                                        phi_31947_ = (1f <= _e6138);
                                                    }
                                                    let _e6142 = phi_31947_;
                                                    let _e6143 = select(_e6138, 1f, _e6142);
                                                    let _e6145 = ((_e5365 * 747796405u) + 2891336453u);
                                                    let _e6153 = (((_e6145 >> bitcast<u32>((((_e6145 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6145) * 277803737u);
                                                    let _e6156 = ((_e6153 >> bitcast<u32>(22i)) ^ _e6153);
                                                    let _e6161 = ((f32((_e6156 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6143);
                                                    if _e6161 {
                                                        phi_19657_ = vec3<f32>();
                                                    } else {
                                                        phi_19657_ = vec3<f32>((_e820.x / _e6143), (_e820.y / _e6143), (_e820.z / _e6143));
                                                    }
                                                    let _e6170 = phi_19657_;
                                                    phi_38194_ = _e6156;
                                                    phi_19659_ = _e6170;
                                                    phi_19660_ = select(true, false, _e6161);
                                                } else {
                                                    phi_38194_ = _e5365;
                                                    phi_19659_ = _e820;
                                                    phi_19660_ = true;
                                                }
                                                let _e6173 = phi_38194_;
                                                let _e6175 = phi_19659_;
                                                let _e6177 = phi_19660_;
                                                phi_38193_ = _e6173;
                                                phi_19661_ = _e6175;
                                                phi_19662_ = _e6177;
                                            } else {
                                                phi_38193_ = _e5365;
                                                phi_19661_ = _e820;
                                                phi_19662_ = true;
                                            }
                                            let _e6179 = phi_38193_;
                                            let _e6181 = phi_19661_;
                                            let _e6183 = phi_19662_;
                                            phi_38191_ = _e6179;
                                            phi_19668_ = select(_e820, _e6181, vec3(_e6183));
                                            phi_19669_ = select(false, true, _e6183);
                                        }
                                        let _e6188 = phi_38191_;
                                        let _e6190 = phi_19668_;
                                        let _e6192 = phi_19669_;
                                        if _e6192 {
                                            if ((((_e5602.x * _e1065.x) + (_e5602.y * _e1065.y)) + (_e5602.z * _e1065.z)) < 0f) {
                                                phi_19698_ = -(_e1065);
                                            } else {
                                                phi_19698_ = _e1065;
                                            }
                                            let _e6231 = phi_19698_;
                                            let _e6248 = local_88;
                                            let _e6251 = local_89;
                                            let _e6254 = local_90;
                                            let _e6257 = local_91;
                                            let _e6263 = local_92;
                                            phi_19714_ = type_69(type_34(vec4<f32>((_e826 + (_e6231.x * _e1095)), (_e827 + (_e6231.y * _e1095)), (_e828 + (_e6231.z * _e1095)), _e331.member.w), vec4<f32>(_e5602.x, _e5602.y, _e5602.z, _e6100.member_1), vec4<f32>((_e6190.x * _e6111), (_e6190.y * _e6113), (_e6190.z * _e6115), f32((_e356 + 1u))), vec4<f32>(_e6251.x, _e6254.y, _e6257.z, f32(_e6248)), vec4<u32>(_e6188, _e1977, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6263);
                                            phi_19715_ = type_69();
                                        } else {
                                            let _e6200 = local_83;
                                            let _e6203 = local_84;
                                            let _e6206 = local_85;
                                            let _e6209 = local_86;
                                            let _e6215 = local_87;
                                            phi_19714_ = type_69();
                                            phi_19715_ = type_69(type_34(_e331.member, _e331.member_1, vec4<f32>(_e6190.x, _e6190.y, _e6190.z, f32(_e356)), vec4<f32>(_e6203.x, _e6206.y, _e6209.z, f32(_e6200)), vec4<u32>(_e6188, (_e1977 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6215);
                                        }
                                        let _e6266 = phi_19714_;
                                        let _e6268 = phi_19715_;
                                        phi_38198_ = _e6188;
                                        phi_19717_ = _e6266;
                                        phi_19718_ = _e6268;
                                        phi_19719_ = select(true, false, _e6192);
                                    }
                                    let _e6271 = phi_38198_;
                                    let _e6273 = phi_19717_;
                                    let _e6275 = phi_19718_;
                                    let _e6277 = phi_19719_;
                                    phi_38197_ = _e6271;
                                    phi_19720_ = _e6273;
                                    phi_19721_ = _e6275;
                                    phi_19722_ = _e6109;
                                    phi_19723_ = _e6277;
                                }
                                let _e6279 = phi_38197_;
                                let _e6281 = phi_19720_;
                                let _e6283 = phi_19721_;
                                let _e6285 = phi_19722_;
                                let _e6287 = phi_19723_;
                                if _e6285 {
                                    let _e6295 = local_93;
                                    let _e6298 = local_94;
                                    let _e6301 = local_95;
                                    let _e6304 = local_96;
                                    let _e6310 = local_97;
                                    phi_19729_ = type_69(type_34(_e331.member, _e331.member_1, vec4<f32>(_e820.x, _e820.y, _e820.z, f32(_e356)), vec4<f32>(_e6298.x, _e6301.y, _e6304.z, f32(_e6295)), vec4<u32>(_e6279, (_e1977 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6310);
                                } else {
                                    phi_19729_ = _e6283;
                                }
                                let _e6313 = phi_19729_;
                                phi_19734_ = _e6313;
                                phi_19735_ = _e6281;
                                phi_19736_ = select(_e6287, true, _e6285);
                            }
                            let _e6340 = phi_19734_;
                            let _e6342 = phi_19735_;
                            let _e6344 = phi_19736_;
                            phi_19740_ = _e6340;
                            phi_19741_ = _e6342;
                            phi_19742_ = _e6344;
                        } else {
                            phi_19740_ = _e1973;
                            phi_19741_ = type_69();
                            phi_19742_ = true;
                        }
                        let _e6346 = phi_19740_;
                        let _e6348 = phi_19741_;
                        let _e6350 = phi_19742_;
                        phi_19746_ = _e6346;
                        phi_19747_ = _e6348;
                        phi_19748_ = _e6350;
                    }
                    let _e6543 = phi_19746_;
                    let _e6545 = phi_19747_;
                    let _e6547 = phi_19748_;
                    phi_19752_ = _e6543;
                    phi_19753_ = _e6545;
                    phi_19754_ = _e6547;
                }
                let _e7055 = phi_19752_;
                let _e7057 = phi_19753_;
                let _e7059 = phi_19754_;
                if _e7059 {
                    phi_19758_ = _e7055;
                } else {
                    phi_19758_ = _e7057;
                }
                let _e7061 = phi_19758_;
                if _e328 {
                } else {
                    break;
                }
                global_7.member[_e323.member_1] = _e7061.member;
                let _e7065 = (2u * _e323.member_1);
                if (_e7065 < _e288) {
                } else {
                    break;
                }
                global_9.member[_e7065] = _e7061.member_1;
                let _e7069 = (_e7065 + 1u);
                if (_e7069 < _e288) {
                } else {
                    break;
                }
                global_9.member[_e7069] = _e7061.member_2;
                if ((_e7061.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7077 = (1u - _e292);
                    let _e7078 = (_e7077 * 4u);
                    let _e7082 = atomicAdd((&global_8.member[(_e7078 + 3u)]), 1u);
                    let _e7086 = global_1.member.member[0u];
                    let _e7091 = global_1.member.member[1u];
                    atomicStore((&global_8.member[((8u + ((_e7077 * _e7086) * _e7091)) + _e7082)]), _e323.member_1);
                    if ((_e7082 % 64u) == 0u) {
                        let _e7099 = (_e7082 / 64u);
                        if (_e7099 < 65535u) {
                            let _e7103 = atomicAdd((&global_8.member[_e7078]), 1u);
                        }
                        if ((_e7099 % 65535u) == 0u) {
                            let _e7109 = atomicAdd((&global_8.member[(_e7078 + 1u)]), 1u);
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

fn function_2() {
    var local_103: array<u32, 64>;
    var local_104: array<f32, 64>;
    var phi_20799_: type_48;
    var phi_20852_: u32;
    var phi_20855_: u32;
    var phi_20857_: u32;
    var phi_20859_: u32;
    var phi_20861_: u32;
    var phi_20863_: vec4<f32>;
    var phi_20865_: f32;
    var phi_21235_: u32;
    var phi_21237_: u32;
    var phi_21238_: u32;
    var phi_21240_: bool;
    var phi_21241_: bool;
    var phi_21242_: u32;
    var phi_21243_: u32;
    var phi_21244_: u32;
    var phi_21245_: bool;
    var phi_21246_: bool;
    var phi_21247_: bool;
    var phi_32620_: bool;
    var phi_32650_: bool;
    var phi_32597_: type_56;
    var phi_32600_: type_56;
    var phi_32601_: bool;
    var phi_32606_: type_56;
    var phi_32707_: bool;
    var phi_32722_: bool;
    var phi_32737_: bool;
    var phi_32752_: bool;
    var phi_32684_: type_56;
    var phi_32687_: type_56;
    var phi_32688_: bool;
    var phi_32693_: type_56;
    var phi_32794_: bool;
    var phi_32809_: bool;
    var phi_32824_: bool;
    var phi_32839_: bool;
    var phi_32771_: type_56;
    var phi_32774_: type_56;
    var phi_32775_: bool;
    var phi_32780_: type_56;
    var phi_32575_: bool;
    var phi_32578_: bool;
    var phi_21276_: f32;
    var phi_32941_: bool;
    var phi_32971_: bool;
    var phi_32918_: type_56;
    var phi_32921_: type_56;
    var phi_32922_: bool;
    var phi_32927_: type_56;
    var phi_33028_: bool;
    var phi_33043_: bool;
    var phi_33058_: bool;
    var phi_33073_: bool;
    var phi_33005_: type_56;
    var phi_33008_: type_56;
    var phi_33009_: bool;
    var phi_33014_: type_56;
    var phi_33115_: bool;
    var phi_33130_: bool;
    var phi_33145_: bool;
    var phi_33160_: bool;
    var phi_33092_: type_56;
    var phi_33095_: type_56;
    var phi_33096_: bool;
    var phi_33101_: type_56;
    var phi_32896_: bool;
    var phi_32899_: bool;
    var phi_21285_: f32;
    var phi_21296_: u32;
    var phi_21307_: u32;
    var phi_21346_: u32;
    var phi_21347_: u32;
    var phi_21358_: u32;
    var phi_21359_: u32;
    var phi_21360_: u32;
    var phi_21361_: u32;
    var phi_21363_: u32;
    var phi_21364_: u32;
    var phi_21366_: u32;
    var phi_21367_: u32;
    var phi_21368_: bool;
    var phi_21369_: bool;
    var phi_21161_: vec4<f32>;
    var phi_21162_: vec4<f32>;
    var phi_21163_: bool;
    var phi_21167_: vec4<f32>;
    var phi_21168_: bool;
    var phi_21078_: bool;
    var phi_21112_: bool;
    var phi_21118_: f32;
    var phi_21128_: vec4<f32>;
    var phi_21129_: vec4<f32>;
    var phi_21130_: bool;
    var phi_21134_: vec4<f32>;
    var phi_21135_: bool;
    var phi_21139_: vec4<f32>;
    var phi_21140_: bool;
    var phi_21169_: vec4<f32>;
    var phi_21170_: bool;
    var phi_21011_: vec4<f32>;
    var phi_21012_: vec4<f32>;
    var phi_21013_: bool;
    var phi_21017_: vec4<f32>;
    var phi_21018_: bool;
    var phi_21019_: vec4<f32>;
    var phi_21020_: bool;
    var phi_21021_: bool;
    var phi_21029_: vec4<f32>;
    var phi_21030_: bool;
    var phi_21031_: vec4<f32>;
    var phi_21032_: bool;
    var phi_21033_: bool;
    var phi_21041_: vec4<f32>;
    var phi_21042_: bool;
    var phi_21179_: vec4<f32>;
    var phi_21180_: bool;
    var phi_21201_: f32;
    var phi_20853_: u32;
    var phi_20856_: u32;
    var phi_20858_: u32;
    var phi_20860_: u32;
    var phi_20862_: u32;
    var phi_20864_: vec4<f32>;
    var phi_20866_: f32;
    var phi_21380_: vec4<f32>;
    var phi_21381_: bool;
    var phi_21382_: bool;
    var phi_38250_: bool;
    var phi_21562_: bool;
    var phi_21561_: vec4<f32>;
    var local_105: u32;
    var local_106: vec4<f32>;
    var phi_21387_: type_74;
    var local_107: u32;
    var phi_21393_: type_74;

    switch bitcast<i32>(0u) {
        default: {
            let _e278 = global;
            let _e287 = global_1.member.member_7[2u];
            let _e291 = (_e278.x + (_e278.y * 4194240u));
            let _e296 = atomicLoad((&global_8.member[((_e287 * 4u) + 3u)]));
            if (_e291 >= _e296) {
                phi_20799_ = type_48(0u, type_48().member_1);
            } else {
                let _e301 = global_1.member.member[0u];
                let _e306 = global_1.member.member[1u];
                let _e312 = atomicLoad((&global_8.member[((8u + ((_e287 * _e301) * _e306)) + _e291)]));
                phi_20799_ = type_48(1u, _e312);
            }
            let _e318 = phi_20799_;
            if (bitcast<i32>(_e318.member) != 0i) {
                let _e323 = (_e318.member_1 < arrayLength((&global_7.member)));
                if _e323 {
                } else {
                    break;
                }
                let _e329 = global_7.member[_e318.member_1].member;
                let _e336 = global_7.member[_e318.member_1].member_1;
                let _e342 = global_1.member.member_1[0u];
                let _e345 = global_1.member.member_1[1u];
                let _e348 = (vec3<f32>(1f, 1f, 1f) / vec3<f32>(_e336.x, _e336.y, _e336.z));
                local_103 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                local_104 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                phi_20852_ = 4294967295u;
                phi_20855_ = select(4294967295u, 0u, (_e342 > 0u));
                phi_20857_ = 0u;
                phi_20859_ = (_e342 + _e345);
                phi_20861_ = _e342;
                phi_20863_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_20865_ = 300000000000000000000000000000000000000f;
                loop {
                    let _e352 = phi_20852_;
                    let _e354 = phi_20855_;
                    let _e356 = phi_20857_;
                    let _e358 = phi_20859_;
                    let _e360 = phi_20861_;
                    let _e362 = phi_20863_;
                    let _e364 = phi_20865_;
                    local_105 = _e352;
                    local_106 = _e362;
                    local_107 = _e360;
                    if (_e360 < _e358) {
                        let _e727 = (3u * _e360);
                        let _e729 = global_2.member[_e727];
                        let _e732 = global_2.member[(_e727 + 1u)];
                        let _e738 = select(select(u32(_e732.w), 0u, (_e732.w < 0f)), 4294967295u, (_e732.w > 4294967000f));
                        if (_e738 == 0u) {
                            let _e832 = global_2.member[(_e727 + 2u)];
                            let _e841 = ((_e336.y * _e832.z) - (_e832.y * _e336.z));
                            let _e844 = ((_e336.z * _e832.x) - (_e832.z * _e336.x));
                            let _e847 = ((_e336.x * _e832.y) - (_e832.x * _e336.y));
                            let _e852 = (((_e732.x * _e841) + (_e732.y * _e844)) + (_e732.z * _e847));
                            if (abs(_e852) < 0.000000000001f) {
                                phi_21041_ = vec4<f32>();
                                phi_21042_ = true;
                            } else {
                                let _e855 = (1f / _e852);
                                let _e859 = (_e329.x - _e729.x);
                                let _e860 = (_e329.y - _e729.y);
                                let _e861 = (_e329.z - _e729.z);
                                let _e867 = ((((_e859 * _e841) + (_e860 * _e844)) + (_e861 * _e847)) * _e855);
                                if (_e867 < 0f) {
                                    phi_21031_ = vec4<f32>();
                                    phi_21032_ = true;
                                    phi_21033_ = false;
                                } else {
                                    let _e869 = (_e867 > 1f);
                                    if _e869 {
                                        phi_21029_ = vec4<f32>();
                                        phi_21030_ = false;
                                    } else {
                                        let _e872 = ((_e860 * _e732.z) - (_e732.y * _e861));
                                        let _e875 = ((_e861 * _e732.x) - (_e732.z * _e859));
                                        let _e878 = ((_e859 * _e732.y) - (_e732.x * _e860));
                                        let _e884 = ((((_e336.x * _e872) + (_e336.y * _e875)) + (_e336.z * _e878)) * _e855);
                                        if (_e884 < 0f) {
                                            phi_21019_ = vec4<f32>();
                                            phi_21020_ = true;
                                            phi_21021_ = false;
                                        } else {
                                            let _e887 = ((_e867 + _e884) > 1f);
                                            if _e887 {
                                                phi_21017_ = vec4<f32>();
                                                phi_21018_ = false;
                                            } else {
                                                let _e893 = ((((_e832.x * _e872) + (_e832.y * _e875)) + (_e832.z * _e878)) * _e855);
                                                if (_e893 <= 0f) {
                                                    phi_21012_ = vec4<f32>();
                                                    phi_21013_ = true;
                                                } else {
                                                    let _e895 = (_e893 >= _e364);
                                                    if _e895 {
                                                        phi_21011_ = vec4<f32>();
                                                    } else {
                                                        phi_21011_ = vec4<f32>(_e893, _e867, _e884, 1f);
                                                    }
                                                    let _e898 = phi_21011_;
                                                    phi_21012_ = _e898;
                                                    phi_21013_ = _e895;
                                                }
                                                let _e900 = phi_21012_;
                                                let _e902 = phi_21013_;
                                                phi_21017_ = _e900;
                                                phi_21018_ = _e902;
                                            }
                                            let _e904 = phi_21017_;
                                            let _e906 = phi_21018_;
                                            phi_21019_ = _e904;
                                            phi_21020_ = _e887;
                                            phi_21021_ = _e906;
                                        }
                                        let _e908 = phi_21019_;
                                        let _e910 = phi_21020_;
                                        let _e912 = phi_21021_;
                                        phi_21029_ = _e908;
                                        phi_21030_ = select(_e912, true, _e910);
                                    }
                                    let _e915 = phi_21029_;
                                    let _e917 = phi_21030_;
                                    phi_21031_ = _e915;
                                    phi_21032_ = _e869;
                                    phi_21033_ = _e917;
                                }
                                let _e919 = phi_21031_;
                                let _e921 = phi_21032_;
                                let _e923 = phi_21033_;
                                phi_21041_ = _e919;
                                phi_21042_ = select(_e923, true, _e921);
                            }
                            let _e926 = phi_21041_;
                            let _e928 = phi_21042_;
                            phi_21179_ = _e926;
                            phi_21180_ = _e928;
                        } else {
                            if (_e738 == 1u) {
                                let _e762 = (_e329.x - _e729.x);
                                let _e763 = (_e329.y - _e729.y);
                                let _e764 = (_e329.z - _e729.z);
                                let _e769 = (((_e762 * _e336.x) + (_e763 * _e336.y)) + (_e764 * _e336.z));
                                if (_e769 > 0f) {
                                    phi_21078_ = select(true, false, ((((_e762 * _e762) + (_e763 * _e763)) + (_e764 * _e764)) > (_e729.w * _e729.w)));
                                } else {
                                    phi_21078_ = true;
                                }
                                let _e781 = phi_21078_;
                                if _e781 {
                                    let _e785 = (_e762 - (_e336.x * _e769));
                                    let _e786 = (_e763 - (_e336.y * _e769));
                                    let _e787 = (_e764 - (_e336.z * _e769));
                                    let _e795 = ((_e729.w * _e729.w) - (((_e785 * _e785) + (_e786 * _e786)) + (_e787 * _e787)));
                                    if (_e795 < 0f) {
                                        phi_21134_ = vec4<f32>();
                                        phi_21135_ = true;
                                    } else {
                                        let _e797 = sqrt(_e795);
                                        let _e798 = -(_e769);
                                        let _e799 = (_e798 - _e797);
                                        if (_e799 <= 0f) {
                                            phi_21112_ = true;
                                        } else {
                                            phi_21112_ = (_e799 >= _e364);
                                        }
                                        let _e803 = phi_21112_;
                                        if _e803 {
                                            phi_21118_ = (_e798 + _e797);
                                        } else {
                                            phi_21118_ = _e799;
                                        }
                                        let _e806 = phi_21118_;
                                        if (_e806 <= 0f) {
                                            phi_21129_ = vec4<f32>();
                                            phi_21130_ = true;
                                        } else {
                                            let _e808 = (_e806 >= _e364);
                                            if _e808 {
                                                phi_21128_ = vec4<f32>();
                                            } else {
                                                phi_21128_ = vec4<f32>(_e806, 0f, 0f, 1f);
                                            }
                                            let _e811 = phi_21128_;
                                            phi_21129_ = _e811;
                                            phi_21130_ = _e808;
                                        }
                                        let _e813 = phi_21129_;
                                        let _e815 = phi_21130_;
                                        phi_21134_ = _e813;
                                        phi_21135_ = _e815;
                                    }
                                    let _e817 = phi_21134_;
                                    let _e819 = phi_21135_;
                                    phi_21139_ = _e817;
                                    phi_21140_ = _e819;
                                } else {
                                    phi_21139_ = vec4<f32>();
                                    phi_21140_ = true;
                                }
                                let _e821 = phi_21139_;
                                let _e823 = phi_21140_;
                                phi_21169_ = _e821;
                                phi_21170_ = _e823;
                            } else {
                                if (abs(_e336.y) < 0.000000000001f) {
                                    phi_21167_ = vec4<f32>();
                                    phi_21168_ = true;
                                } else {
                                    let _e745 = ((_e729.x - _e329.y) / _e336.y);
                                    if (_e745 <= 0f) {
                                        phi_21162_ = vec4<f32>();
                                        phi_21163_ = true;
                                    } else {
                                        let _e747 = (_e745 >= _e364);
                                        if _e747 {
                                            phi_21161_ = vec4<f32>();
                                        } else {
                                            phi_21161_ = vec4<f32>(_e745, 0f, 0f, 1f);
                                        }
                                        let _e750 = phi_21161_;
                                        phi_21162_ = _e750;
                                        phi_21163_ = _e747;
                                    }
                                    let _e752 = phi_21162_;
                                    let _e754 = phi_21163_;
                                    phi_21167_ = _e752;
                                    phi_21168_ = _e754;
                                }
                                let _e756 = phi_21167_;
                                let _e758 = phi_21168_;
                                phi_21169_ = _e756;
                                phi_21170_ = _e758;
                            }
                            let _e825 = phi_21169_;
                            let _e827 = phi_21170_;
                            phi_21179_ = _e825;
                            phi_21180_ = select(true, false, select(true, false, _e827));
                        }
                        let _e930 = phi_21179_;
                        let _e932 = phi_21180_;
                        let _e935 = select(vec4<f32>(0f, 0f, 0f, 0f), _e930, vec4(select(true, false, _e932)));
                        let _e937 = (_e935.w > 0f);
                        if _e937 {
                            phi_21201_ = _e935.x;
                        } else {
                            phi_21201_ = _e364;
                        }
                        let _e940 = phi_21201_;
                        phi_20853_ = select(_e352, _e360, _e937);
                        phi_20856_ = _e354;
                        phi_20858_ = _e356;
                        phi_20860_ = _e358;
                        phi_20862_ = (_e360 + 1u);
                        phi_20864_ = select(_e362, _e935, vec4(_e937));
                        phi_20866_ = _e940;
                        phi_21380_ = _e935;
                        phi_21381_ = true;
                        phi_21382_ = false;
                    } else {
                        if (_e354 == 4294967295u) {
                            let _e367 = (_e356 == 0u);
                            if _e367 {
                                phi_21237_ = u32();
                                phi_21238_ = u32();
                                phi_21240_ = false;
                                phi_21241_ = false;
                            } else {
                                let _e368 = (_e356 - 1u);
                                let _e369 = (_e368 < 64u);
                                if _e369 {
                                } else {
                                    phi_38250_ = true;
                                    phi_21562_ = bool();
                                    phi_21561_ = vec4<f32>();
                                    break;
                                }
                                let _e371 = local_104[_e368];
                                let _e372 = (_e371 >= _e364);
                                if _e372 {
                                    phi_21235_ = u32();
                                } else {
                                    if _e369 {
                                    } else {
                                        phi_38250_ = true;
                                        phi_21562_ = bool();
                                        phi_21561_ = vec4<f32>();
                                        break;
                                    }
                                    let _e374 = local_103[_e368];
                                    phi_21235_ = _e374;
                                }
                                let _e376 = phi_21235_;
                                phi_21237_ = _e368;
                                phi_21238_ = _e376;
                                phi_21240_ = select(true, false, _e372);
                                phi_21241_ = _e372;
                            }
                            let _e379 = phi_21237_;
                            let _e381 = phi_21238_;
                            let _e383 = phi_21240_;
                            let _e385 = phi_21241_;
                            phi_21242_ = _e379;
                            phi_21243_ = _e381;
                            phi_21244_ = _e379;
                            phi_21245_ = _e383;
                            phi_21246_ = _e385;
                            phi_21247_ = _e367;
                        } else {
                            phi_21242_ = u32();
                            phi_21243_ = _e354;
                            phi_21244_ = _e356;
                            phi_21245_ = true;
                            phi_21246_ = false;
                            phi_21247_ = false;
                        }
                        let _e387 = phi_21242_;
                        let _e389 = phi_21243_;
                        let _e391 = phi_21244_;
                        let _e393 = phi_21245_;
                        let _e395 = phi_21246_;
                        let _e397 = phi_21247_;
                        if _e393 {
                            let _e399 = ((_e389 & 2147483648u) == 0u);
                            if _e399 {
                                let _e407 = global_3.member[_e389];
                                if (_e407.member_1 == 4294967295u) {
                                    phi_21276_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e419 = ((_e407.member.x - _e329.x) * _e348.x);
                                    let _e421 = ((_e407.member.y - _e329.y) * _e348.y);
                                    let _e423 = ((_e407.member.z - _e329.z) * _e348.z);
                                    let _e430 = ((_e407.member_2.x - _e329.x) * _e348.x);
                                    let _e431 = ((_e407.member_2.y - _e329.y) * _e348.y);
                                    let _e432 = ((_e407.member_2.z - _e329.z) * _e348.z);
                                    if (_e419 == _e419) {
                                        let _e434 = (_e430 == _e430);
                                        if _e434 {
                                            let _e435 = (_e419 != _e419);
                                            if _e435 {
                                                phi_32620_ = true;
                                            } else {
                                                phi_32620_ = (_e430 <= _e419);
                                            }
                                            let _e438 = phi_32620_;
                                            let _e439 = select(_e419, _e430, _e438);
                                            if _e435 {
                                                phi_32650_ = true;
                                            } else {
                                                phi_32650_ = (_e430 >= _e419);
                                            }
                                            let _e444 = phi_32650_;
                                            let _e445 = select(_e419, _e430, _e444);
                                            phi_32597_ = type_56(select(-300000000000000000000000000000000000000f, _e439, (_e439 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e445, (_e445 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_32597_ = type_56();
                                        }
                                        let _e450 = phi_32597_;
                                        phi_32600_ = _e450;
                                        phi_32601_ = select(true, false, _e434);
                                    } else {
                                        phi_32600_ = type_56();
                                        phi_32601_ = true;
                                    }
                                    let _e453 = phi_32600_;
                                    let _e455 = phi_32601_;
                                    if _e455 {
                                        phi_32606_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_32606_ = _e453;
                                    }
                                    let _e457 = phi_32606_;
                                    if (_e421 == _e421) {
                                        let _e461 = (_e431 == _e431);
                                        if _e461 {
                                            let _e462 = (_e421 != _e421);
                                            if _e462 {
                                                phi_32707_ = true;
                                            } else {
                                                phi_32707_ = (_e431 <= _e421);
                                            }
                                            let _e465 = phi_32707_;
                                            let _e466 = select(_e421, _e431, _e465);
                                            if (_e457.member != _e457.member) {
                                                phi_32722_ = true;
                                            } else {
                                                phi_32722_ = (_e466 >= _e457.member);
                                            }
                                            let _e470 = phi_32722_;
                                            if _e462 {
                                                phi_32737_ = true;
                                            } else {
                                                phi_32737_ = (_e431 >= _e421);
                                            }
                                            let _e474 = phi_32737_;
                                            let _e475 = select(_e421, _e431, _e474);
                                            if (_e457.member_1 != _e457.member_1) {
                                                phi_32752_ = true;
                                            } else {
                                                phi_32752_ = (_e475 <= _e457.member_1);
                                            }
                                            let _e479 = phi_32752_;
                                            phi_32684_ = type_56(select(_e457.member, _e466, _e470), select(_e457.member_1, _e475, _e479));
                                        } else {
                                            phi_32684_ = type_56();
                                        }
                                        let _e483 = phi_32684_;
                                        phi_32687_ = _e483;
                                        phi_32688_ = select(true, false, _e461);
                                    } else {
                                        phi_32687_ = type_56();
                                        phi_32688_ = true;
                                    }
                                    let _e486 = phi_32687_;
                                    let _e488 = phi_32688_;
                                    if _e488 {
                                        phi_32693_ = _e457;
                                    } else {
                                        phi_32693_ = _e486;
                                    }
                                    let _e490 = phi_32693_;
                                    if (_e423 == _e423) {
                                        let _e494 = (_e432 == _e432);
                                        if _e494 {
                                            let _e495 = (_e423 != _e423);
                                            if _e495 {
                                                phi_32794_ = true;
                                            } else {
                                                phi_32794_ = (_e432 <= _e423);
                                            }
                                            let _e498 = phi_32794_;
                                            let _e499 = select(_e423, _e432, _e498);
                                            if (_e490.member != _e490.member) {
                                                phi_32809_ = true;
                                            } else {
                                                phi_32809_ = (_e499 >= _e490.member);
                                            }
                                            let _e503 = phi_32809_;
                                            if _e495 {
                                                phi_32824_ = true;
                                            } else {
                                                phi_32824_ = (_e432 >= _e423);
                                            }
                                            let _e507 = phi_32824_;
                                            let _e508 = select(_e423, _e432, _e507);
                                            if (_e490.member_1 != _e490.member_1) {
                                                phi_32839_ = true;
                                            } else {
                                                phi_32839_ = (_e508 <= _e490.member_1);
                                            }
                                            let _e512 = phi_32839_;
                                            phi_32771_ = type_56(select(_e490.member, _e499, _e503), select(_e490.member_1, _e508, _e512));
                                        } else {
                                            phi_32771_ = type_56();
                                        }
                                        let _e516 = phi_32771_;
                                        phi_32774_ = _e516;
                                        phi_32775_ = select(true, false, _e494);
                                    } else {
                                        phi_32774_ = type_56();
                                        phi_32775_ = true;
                                    }
                                    let _e519 = phi_32774_;
                                    let _e521 = phi_32775_;
                                    if _e521 {
                                        phi_32780_ = _e490;
                                    } else {
                                        phi_32780_ = _e519;
                                    }
                                    let _e523 = phi_32780_;
                                    if (_e523.member_1 < 0f) {
                                        phi_32578_ = true;
                                    } else {
                                        if (_e523.member > _e523.member_1) {
                                            phi_32575_ = true;
                                        } else {
                                            phi_32575_ = (_e523.member > _e364);
                                        }
                                        let _e530 = phi_32575_;
                                        phi_32578_ = _e530;
                                    }
                                    let _e532 = phi_32578_;
                                    phi_21276_ = select(_e523.member, 300000000000000000000000000000000000000f, _e532);
                                }
                                let _e535 = phi_21276_;
                                if (_e407.member_5 == 4294967295u) {
                                    phi_21285_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e547 = ((_e407.member_4.x - _e329.x) * _e348.x);
                                    let _e549 = ((_e407.member_4.y - _e329.y) * _e348.y);
                                    let _e551 = ((_e407.member_4.z - _e329.z) * _e348.z);
                                    let _e558 = ((_e407.member_6.x - _e329.x) * _e348.x);
                                    let _e559 = ((_e407.member_6.y - _e329.y) * _e348.y);
                                    let _e560 = ((_e407.member_6.z - _e329.z) * _e348.z);
                                    if (_e547 == _e547) {
                                        let _e562 = (_e558 == _e558);
                                        if _e562 {
                                            let _e563 = (_e547 != _e547);
                                            if _e563 {
                                                phi_32941_ = true;
                                            } else {
                                                phi_32941_ = (_e558 <= _e547);
                                            }
                                            let _e566 = phi_32941_;
                                            let _e567 = select(_e547, _e558, _e566);
                                            if _e563 {
                                                phi_32971_ = true;
                                            } else {
                                                phi_32971_ = (_e558 >= _e547);
                                            }
                                            let _e572 = phi_32971_;
                                            let _e573 = select(_e547, _e558, _e572);
                                            phi_32918_ = type_56(select(-300000000000000000000000000000000000000f, _e567, (_e567 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e573, (_e573 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_32918_ = type_56();
                                        }
                                        let _e578 = phi_32918_;
                                        phi_32921_ = _e578;
                                        phi_32922_ = select(true, false, _e562);
                                    } else {
                                        phi_32921_ = type_56();
                                        phi_32922_ = true;
                                    }
                                    let _e581 = phi_32921_;
                                    let _e583 = phi_32922_;
                                    if _e583 {
                                        phi_32927_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_32927_ = _e581;
                                    }
                                    let _e585 = phi_32927_;
                                    if (_e549 == _e549) {
                                        let _e589 = (_e559 == _e559);
                                        if _e589 {
                                            let _e590 = (_e549 != _e549);
                                            if _e590 {
                                                phi_33028_ = true;
                                            } else {
                                                phi_33028_ = (_e559 <= _e549);
                                            }
                                            let _e593 = phi_33028_;
                                            let _e594 = select(_e549, _e559, _e593);
                                            if (_e585.member != _e585.member) {
                                                phi_33043_ = true;
                                            } else {
                                                phi_33043_ = (_e594 >= _e585.member);
                                            }
                                            let _e598 = phi_33043_;
                                            if _e590 {
                                                phi_33058_ = true;
                                            } else {
                                                phi_33058_ = (_e559 >= _e549);
                                            }
                                            let _e602 = phi_33058_;
                                            let _e603 = select(_e549, _e559, _e602);
                                            if (_e585.member_1 != _e585.member_1) {
                                                phi_33073_ = true;
                                            } else {
                                                phi_33073_ = (_e603 <= _e585.member_1);
                                            }
                                            let _e607 = phi_33073_;
                                            phi_33005_ = type_56(select(_e585.member, _e594, _e598), select(_e585.member_1, _e603, _e607));
                                        } else {
                                            phi_33005_ = type_56();
                                        }
                                        let _e611 = phi_33005_;
                                        phi_33008_ = _e611;
                                        phi_33009_ = select(true, false, _e589);
                                    } else {
                                        phi_33008_ = type_56();
                                        phi_33009_ = true;
                                    }
                                    let _e614 = phi_33008_;
                                    let _e616 = phi_33009_;
                                    if _e616 {
                                        phi_33014_ = _e585;
                                    } else {
                                        phi_33014_ = _e614;
                                    }
                                    let _e618 = phi_33014_;
                                    if (_e551 == _e551) {
                                        let _e622 = (_e560 == _e560);
                                        if _e622 {
                                            let _e623 = (_e551 != _e551);
                                            if _e623 {
                                                phi_33115_ = true;
                                            } else {
                                                phi_33115_ = (_e560 <= _e551);
                                            }
                                            let _e626 = phi_33115_;
                                            let _e627 = select(_e551, _e560, _e626);
                                            if (_e618.member != _e618.member) {
                                                phi_33130_ = true;
                                            } else {
                                                phi_33130_ = (_e627 >= _e618.member);
                                            }
                                            let _e631 = phi_33130_;
                                            if _e623 {
                                                phi_33145_ = true;
                                            } else {
                                                phi_33145_ = (_e560 >= _e551);
                                            }
                                            let _e635 = phi_33145_;
                                            let _e636 = select(_e551, _e560, _e635);
                                            if (_e618.member_1 != _e618.member_1) {
                                                phi_33160_ = true;
                                            } else {
                                                phi_33160_ = (_e636 <= _e618.member_1);
                                            }
                                            let _e640 = phi_33160_;
                                            phi_33092_ = type_56(select(_e618.member, _e627, _e631), select(_e618.member_1, _e636, _e640));
                                        } else {
                                            phi_33092_ = type_56();
                                        }
                                        let _e644 = phi_33092_;
                                        phi_33095_ = _e644;
                                        phi_33096_ = select(true, false, _e622);
                                    } else {
                                        phi_33095_ = type_56();
                                        phi_33096_ = true;
                                    }
                                    let _e647 = phi_33095_;
                                    let _e649 = phi_33096_;
                                    if _e649 {
                                        phi_33101_ = _e618;
                                    } else {
                                        phi_33101_ = _e647;
                                    }
                                    let _e651 = phi_33101_;
                                    if (_e651.member_1 < 0f) {
                                        phi_32899_ = true;
                                    } else {
                                        if (_e651.member > _e651.member_1) {
                                            phi_32896_ = true;
                                        } else {
                                            phi_32896_ = (_e651.member > _e364);
                                        }
                                        let _e658 = phi_32896_;
                                        phi_32899_ = _e658;
                                    }
                                    let _e660 = phi_32899_;
                                    phi_21285_ = select(_e651.member, 300000000000000000000000000000000000000f, _e660);
                                }
                                let _e663 = phi_21285_;
                                if (_e407.member_3 > 0u) {
                                    phi_21296_ = ((2147483648u | ((_e407.member_3 - 1u) << bitcast<u32>(28u))) | _e407.member_1);
                                } else {
                                    phi_21296_ = _e407.member_1;
                                }
                                let _e672 = phi_21296_;
                                if (_e407.member_7 > 0u) {
                                    phi_21307_ = ((2147483648u | ((_e407.member_7 - 1u) << bitcast<u32>(28u))) | _e407.member_5);
                                } else {
                                    phi_21307_ = _e407.member_5;
                                }
                                let _e681 = phi_21307_;
                                let _e682 = (_e535 <= _e663);
                                let _e686 = select(_e535, _e663, _e682);
                                if (_e686 < 300000000000000000000000000000000000000f) {
                                    let _e688 = (_e391 < 64u);
                                    if _e688 {
                                        if _e688 {
                                        } else {
                                            phi_38250_ = true;
                                            phi_21562_ = bool();
                                            phi_21561_ = vec4<f32>();
                                            break;
                                        }
                                        local_103[_e391] = select(_e672, _e681, _e682);
                                        if _e688 {
                                        } else {
                                            phi_38250_ = true;
                                            phi_21562_ = bool();
                                            phi_21561_ = vec4<f32>();
                                            break;
                                        }
                                        local_104[_e391] = _e686;
                                        phi_21346_ = (_e391 + 1u);
                                    } else {
                                        phi_21346_ = _e391;
                                    }
                                    let _e693 = phi_21346_;
                                    phi_21347_ = _e693;
                                } else {
                                    phi_21347_ = _e391;
                                }
                                let _e695 = phi_21347_;
                                phi_21358_ = select(4294967295u, select(_e681, _e672, _e682), (select(_e663, _e535, _e682) < 300000000000000000000000000000000000000f));
                                phi_21359_ = _e695;
                                phi_21360_ = u32();
                                phi_21361_ = u32();
                            } else {
                                let _e400 = (_e389 & 268435455u);
                                phi_21358_ = u32();
                                phi_21359_ = u32();
                                phi_21360_ = ((_e400 + ((_e389 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_21361_ = _e400;
                            }
                            let _e699 = phi_21358_;
                            let _e701 = phi_21359_;
                            let _e703 = phi_21360_;
                            let _e705 = phi_21361_;
                            phi_21363_ = _e699;
                            phi_21364_ = _e701;
                            phi_21366_ = _e703;
                            phi_21367_ = _e705;
                            phi_21368_ = select(true, false, _e399);
                            phi_21369_ = _e399;
                        } else {
                            phi_21363_ = u32();
                            phi_21364_ = u32();
                            phi_21366_ = _e358;
                            phi_21367_ = _e360;
                            phi_21368_ = _e395;
                            phi_21369_ = false;
                        }
                        let _e708 = phi_21363_;
                        let _e710 = phi_21364_;
                        let _e712 = phi_21366_;
                        let _e714 = phi_21367_;
                        let _e716 = phi_21368_;
                        let _e718 = phi_21369_;
                        phi_20853_ = _e352;
                        phi_20856_ = select(_e708, 4294967295u, _e716);
                        phi_20858_ = select(_e710, select(_e387, _e391, _e393), _e716);
                        phi_20860_ = select(_e358, _e712, _e716);
                        phi_20862_ = select(_e360, _e714, _e716);
                        phi_20864_ = _e362;
                        phi_20866_ = _e364;
                        phi_21380_ = vec4<f32>();
                        phi_21381_ = select(_e718, true, _e716);
                        phi_21382_ = select(select(_e397, false, _e393), false, _e716);
                    }
                    let _e946 = phi_20853_;
                    let _e948 = phi_20856_;
                    let _e950 = phi_20858_;
                    let _e952 = phi_20860_;
                    let _e954 = phi_20862_;
                    let _e956 = phi_20864_;
                    let _e958 = phi_20866_;
                    let _e960 = phi_21380_;
                    let _e962 = phi_21381_;
                    let _e964 = phi_21382_;
                    phi_20852_ = _e946;
                    phi_20855_ = _e948;
                    phi_20857_ = _e950;
                    phi_20859_ = _e952;
                    phi_20861_ = _e954;
                    phi_20863_ = _e956;
                    phi_20865_ = _e958;
                    phi_38250_ = false;
                    phi_21562_ = _e964;
                    phi_21561_ = _e960;
                    if !(_e962) {
                        break;
                    }
                }
                let _e967 = phi_38250_;
                let _e969 = phi_21562_;
                let _e971 = phi_21561_;
                if _e967 {
                    break;
                }
                if _e969 {
                    let _e973 = local_105;
                    let _e975 = local_106;
                    phi_21387_ = type_74(_e973, _e975);
                } else {
                    phi_21387_ = type_74();
                }
                let _e978 = phi_21387_;
                if select(true, false, _e969) {
                    let _e981 = local_107;
                    phi_21393_ = type_74(_e981, _e971);
                } else {
                    phi_21393_ = _e978;
                }
                let _e984 = phi_21393_;
                let _e987 = global_7.member[_e318.member_1];
                let _e999 = type_34(_e987.member, _e987.member_1, _e987.member_2, _e987.member_3, vec4<u32>(_e987.member_4.x, _e987.member_4.y, _e984.member, _e987.member_4.w), _e987.member_5);
                if _e323 {
                } else {
                    break;
                }
                global_7.member[_e318.member_1] = type_34(_e999.member, _e999.member_1, _e999.member_2, _e999.member_3, _e999.member_4, _e984.member_1);
            }
            break;
        }
    }
    return;
}

fn function_3() {
    var phi_21427_: type_48;
    var phi_21474_: bool;
    var phi_21475_: bool;
    var phi_21476_: bool;
    var phi_21492_: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e276 = global;
            let _e284 = (_e276.x + (_e276.y * 4194240u));
            let _e288 = global_1.member.member[0u];
            let _e292 = global_1.member.member[1u];
            if (_e284 >= (_e288 * _e292)) {
                phi_21427_ = type_48(0u, type_48().member_1);
            } else {
                phi_21427_ = type_48(1u, _e284);
            }
            let _e300 = phi_21427_;
            if (bitcast<i32>(_e300.member) != 0i) {
                if (_e300.member_1 < arrayLength((&global_10.member))) {
                } else {
                    break;
                }
                let _e310 = global_10.member[_e300.member_1].member_4[1u];
                if ((_e310 & 8u) == 0u) {
                    let _e316 = global_10.member[_e300.member_1].member_3;
                    let _e324 = global_10.member[_e300.member_1].member[3u];
                    let _e325 = (_e316.x * _e324);
                    let _e326 = (_e316.y * _e324);
                    let _e327 = (_e316.z * _e324);
                    if (_e325 == _e325) {
                        let _e330 = (_e326 == _e326);
                        if _e330 {
                            phi_21474_ = select(true, false, (_e327 == _e327));
                        } else {
                            phi_21474_ = false;
                        }
                        let _e334 = phi_21474_;
                        phi_21475_ = select(true, false, _e330);
                        phi_21476_ = _e334;
                    } else {
                        phi_21475_ = true;
                        phi_21476_ = false;
                    }
                    let _e337 = phi_21475_;
                    let _e339 = phi_21476_;
                    let _e342 = select(vec3<f32>(_e325, _e326, _e327), vec3<f32>(0f, 0f, 0f), vec3(select(_e339, true, _e337)));
                    let _e347 = global_10.member[_e300.member_1].member_3[3u];
                    phi_21492_ = vec4<f32>(_e342.x, _e342.y, _e342.z, _e347);
                } else {
                    phi_21492_ = vec4<f32>(0f, 0f, 0f, 0f);
                }
                let _e353 = phi_21492_;
                if (_e353.w > 0f) {
                    if (_e300.member_1 < arrayLength((&global_11.member))) {
                    } else {
                        break;
                    }
                    let _e361 = global_11.member[_e300.member_1][0u];
                    global_11.member[_e300.member_1][0u] = (_e361 + _e353.x);
                    let _e367 = global_11.member[_e300.member_1][1u];
                    global_11.member[_e300.member_1][1u] = (_e367 + _e353.y);
                    let _e373 = global_11.member[_e300.member_1][2u];
                    global_11.member[_e300.member_1][2u] = (_e373 + _e353.z);
                    let _e378 = global_11.member[_e300.member_1][3u];
                    global_11.member[_e300.member_1][3u] = (_e378 + _e353.w);
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

@compute @workgroup_size(64, 1, 1) 
fn intersect_paths(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}

@compute @workgroup_size(64, 1, 1) 
fn add_paths(@builtin(global_invocation_id) param_3: vec3<u32>) {
    global = param_3;
    function_3();
}
