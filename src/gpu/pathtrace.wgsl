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
var<storage, read_write> global_10: type_36;
@group(0) @binding(2) 
var<storage, read_write> global_11: type_79;
@group(0) @binding(3) 
var<storage, read_write> global_12: type_36;
@group(0) @binding(1) 
var<storage> global_13: type_36;
@group(0) @binding(2) 
var<storage, read_write> global_14: type_13;

fn function_() {
    var phi_497_: type_48;
    var phi_33746_: bool;
    var phi_517_: vec3<f32>;
    var phi_520_: u32;
    var phi_21980_: f32;
    var phi_21992_: type_56;
    var phi_22003_: type_57;
    var phi_22004_: type_57;
    var phi_22008_: type_57;
    var phi_22009_: type_57;
    var phi_22010_: bool;
    var phi_22014_: type_57;
    var phi_21901_: type_57;
    var phi_22052_: bool;
    var phi_22132_: f32;
    var phi_22144_: type_56;
    var phi_22155_: type_57;
    var phi_22156_: type_57;
    var phi_22160_: type_57;
    var phi_22161_: type_57;
    var phi_22162_: bool;
    var phi_22166_: type_57;
    var phi_22100_: f32;
    var phi_22204_: bool;
    var phi_21929_: f32;
    var phi_21931_: f32;
    var phi_21940_: type_57;
    var phi_21944_: type_57;
    var phi_21948_: type_57;
    var phi_22219_: bool;
    var phi_9520_: vec3<f32>;
    var phi_9523_: vec3<f32>;
    var phi_9525_: vec3<f32>;
    var phi_9527_: vec3<f32>;
    var phi_9529_: u32;
    var phi_9613_: u32;
    var phi_9628_: type_56;
    var phi_9629_: type_56;
    var phi_9630_: type_56;
    var phi_22419_: bool;
    var phi_22546_: bool;
    var phi_22561_: bool;
    var phi_9958_: vec3<f32>;
    var phi_10031_: vec3<f32>;
    var phi_10032_: vec3<f32>;
    var phi_10033_: bool;
    var phi_10038_: vec3<f32>;
    var phi_10039_: bool;
    var phi_22685_: bool;
    var phi_22700_: bool;
    var phi_10330_: vec3<f32>;
    var phi_9521_: vec3<f32>;
    var phi_9524_: vec3<f32>;
    var phi_9526_: vec3<f32>;
    var phi_9528_: vec3<f32>;
    var phi_9530_: u32;
    var phi_33729_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_34213_: bool;
    var phi_10387_: type_60;
    var phi_34212_: bool;
    var phi_10389_: type_60;
    var phi_10390_: bool;
    var phi_10395_: type_60;
    var phi_896_: vec3<f32>;
    var phi_898_: vec3<f32>;
    var phi_899_: bool;
    var phi_33821_: bool;
    var phi_909_: vec3<f32>;
    var phi_912_: i32;
    var phi_914_: u32;
    var phi_22969_: bool;
    var phi_1336_: u32;
    var phi_1366_: u32;
    var phi_1337_: u32;
    var phi_1367_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1385_: u32;
    var phi_2690_: type_55;
    var phi_2698_: type_52;
    var phi_2700_: type_52;
    var phi_2701_: bool;
    var phi_2702_: type_52;
    var phi_2703_: bool;
    var phi_2704_: bool;
    var phi_23080_: f32;
    var phi_23092_: type_56;
    var phi_23103_: type_57;
    var phi_23104_: type_57;
    var phi_23108_: type_57;
    var phi_23109_: type_57;
    var phi_23110_: bool;
    var phi_23114_: type_57;
    var phi_23001_: type_57;
    var phi_23152_: bool;
    var phi_23232_: f32;
    var phi_23244_: type_56;
    var phi_23255_: type_57;
    var phi_23256_: type_57;
    var phi_23260_: type_57;
    var phi_23261_: type_57;
    var phi_23262_: bool;
    var phi_23266_: type_57;
    var phi_23200_: f32;
    var phi_23304_: bool;
    var phi_23029_: f32;
    var phi_23031_: f32;
    var phi_23040_: type_57;
    var phi_23044_: type_57;
    var phi_23048_: type_57;
    var phi_2722_: vec3<f32>;
    var phi_2725_: vec3<f32>;
    var phi_2727_: vec3<f32>;
    var phi_2729_: vec3<f32>;
    var phi_2731_: u32;
    var phi_2815_: u32;
    var phi_2830_: type_56;
    var phi_2831_: type_56;
    var phi_2832_: type_56;
    var phi_23503_: bool;
    var phi_23630_: bool;
    var phi_23645_: bool;
    var phi_3160_: vec3<f32>;
    var phi_23767_: f32;
    var phi_23779_: type_56;
    var phi_23790_: type_57;
    var phi_23791_: type_57;
    var phi_23795_: type_57;
    var phi_23796_: type_57;
    var phi_23797_: bool;
    var phi_23801_: type_57;
    var phi_23735_: f32;
    var phi_23877_: f32;
    var phi_23889_: type_56;
    var phi_23900_: type_57;
    var phi_23901_: type_57;
    var phi_23905_: type_57;
    var phi_23906_: type_57;
    var phi_23907_: bool;
    var phi_23911_: type_57;
    var phi_23845_: f32;
    var phi_23672_: vec3<f32>;
    var phi_23673_: u32;
    var phi_24043_: bool;
    var phi_23706_: vec3<f32>;
    var phi_23707_: u32;
    var local_11: vec3<f32>;
    var phi_23714_: type_58;
    var phi_23718_: type_58;
    var phi_3219_: vec3<f32>;
    var phi_3220_: vec3<f32>;
    var phi_2723_: vec3<f32>;
    var phi_2726_: vec3<f32>;
    var phi_2728_: vec3<f32>;
    var phi_2730_: vec3<f32>;
    var phi_2732_: u32;
    var phi_33789_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_33883_: bool;
    var phi_3240_: type_52;
    var phi_33882_: bool;
    var phi_3241_: type_52;
    var phi_33881_: bool;
    var phi_3264_: vec3<f32>;
    var phi_3265_: i32;
    var phi_3266_: u32;
    var phi_33880_: bool;
    var phi_910_: vec3<f32>;
    var phi_913_: i32;
    var phi_915_: u32;
    var phi_3267_: bool;
    var phi_33879_: bool;
    var phi_3601_: vec3<f32>;
    var phi_3604_: u32;
    var phi_24171_: f32;
    var phi_24186_: f32;
    var phi_24201_: f32;
    var phi_3675_: i32;
    var phi_3705_: i32;
    var phi_3713_: vec3<f32>;
    var phi_3716_: i32;
    var phi_3739_: vec3<f32>;
    var phi_3742_: i32;
    var phi_3740_: vec3<f32>;
    var phi_3743_: i32;
    var phi_3714_: vec3<f32>;
    var phi_3717_: i32;
    var phi_3941_: vec3<f32>;
    var phi_3942_: vec3<f32>;
    var phi_3943_: vec3<f32>;
    var phi_3602_: vec3<f32>;
    var phi_3605_: u32;
    var phi_33884_: bool;
    var local_15: vec3<f32>;
    var phi_33932_: bool;
    var phi_3954_: f32;
    var phi_3978_: f32;
    var phi_3980_: u32;
    var phi_3983_: f32;
    var phi_4086_: f32;
    var phi_4087_: f32;
    var phi_4103_: f32;
    var phi_3981_: u32;
    var phi_3984_: f32;
    var phi_4106_: u32;
    var phi_4109_: f32;
    var phi_4217_: f32;
    var phi_4107_: u32;
    var phi_4110_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_24317_: f32;
    var phi_24329_: type_56;
    var phi_24340_: type_57;
    var phi_24341_: type_57;
    var phi_24345_: type_57;
    var phi_24346_: type_57;
    var phi_24347_: bool;
    var phi_24351_: type_57;
    var phi_24285_: f32;
    var phi_4618_: bool;
    var phi_24477_: f32;
    var phi_24489_: type_56;
    var phi_24500_: type_57;
    var phi_24501_: type_57;
    var phi_24505_: type_57;
    var phi_24506_: type_57;
    var phi_24507_: bool;
    var phi_24511_: type_57;
    var phi_24398_: type_57;
    var phi_24549_: bool;
    var phi_24629_: f32;
    var phi_24641_: type_56;
    var phi_24652_: type_57;
    var phi_24653_: type_57;
    var phi_24657_: type_57;
    var phi_24658_: type_57;
    var phi_24659_: bool;
    var phi_24663_: type_57;
    var phi_24597_: f32;
    var phi_24701_: bool;
    var phi_24426_: f32;
    var phi_24428_: f32;
    var phi_24437_: type_57;
    var phi_24441_: type_57;
    var phi_24445_: type_57;
    var phi_4632_: vec3<f32>;
    var phi_4635_: u32;
    var phi_4719_: u32;
    var phi_4734_: type_56;
    var phi_4735_: type_56;
    var phi_4736_: type_56;
    var phi_24900_: bool;
    var phi_4633_: vec3<f32>;
    var phi_4636_: u32;
    var phi_33911_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_33978_: bool;
    var phi_4761_: vec3<f32>;
    var phi_33977_: bool;
    var phi_4763_: vec3<f32>;
    var phi_4765_: bool;
    var phi_33973_: bool;
    var phi_4776_: vec3<f32>;
    var phi_4777_: bool;
    var phi_33972_: bool;
    var phi_4779_: vec3<f32>;
    var phi_4780_: bool;
    var phi_4781_: bool;
    var phi_33971_: bool;
    var phi_4785_: vec3<f32>;
    var phi_4786_: vec3<f32>;
    var phi_4787_: bool;
    var phi_4788_: vec3<f32>;
    var phi_4789_: bool;
    var phi_5109_: vec3<f32>;
    var phi_5112_: u32;
    var phi_24971_: f32;
    var phi_24986_: f32;
    var phi_25001_: f32;
    var phi_5183_: i32;
    var phi_5213_: i32;
    var phi_5221_: vec3<f32>;
    var phi_5224_: i32;
    var phi_5247_: vec3<f32>;
    var phi_5250_: i32;
    var phi_5321_: vec3<f32>;
    var phi_5324_: u32;
    var phi_5444_: vec3<f32>;
    var phi_5322_: vec3<f32>;
    var phi_5325_: u32;
    var phi_5248_: vec3<f32>;
    var phi_5251_: i32;
    var phi_5222_: vec3<f32>;
    var phi_5225_: i32;
    var phi_5449_: vec3<f32>;
    var phi_5450_: vec3<f32>;
    var phi_5451_: vec3<f32>;
    var phi_5110_: vec3<f32>;
    var phi_5113_: u32;
    var phi_33966_: bool;
    var phi_34008_: bool;
    var phi_5471_: vec3<f32>;
    var phi_5684_: u32;
    var phi_5685_: bool;
    var phi_5688_: u32;
    var phi_5689_: bool;
    var phi_5690_: bool;
    var phi_5700_: f32;
    var phi_5703_: f32;
    var phi_5705_: u32;
    var phi_5707_: vec3<f32>;
    var phi_5709_: vec3<f32>;
    var phi_5711_: f32;
    var phi_5713_: f32;
    var phi_5751_: u32;
    var phi_5766_: vec3<f32>;
    var phi_5786_: vec3<f32>;
    var phi_25016_: vec3<f32>;
    var phi_5787_: u32;
    var phi_5788_: vec3<f32>;
    var phi_5789_: u32;
    var phi_5790_: vec3<f32>;
    var phi_5791_: u32;
    var phi_5793_: vec3<f32>;
    var phi_5928_: u32;
    var phi_5931_: f32;
    var phi_5933_: f32;
    var phi_5935_: f32;
    var phi_5937_: vec3<f32>;
    var phi_25071_: f32;
    var phi_25077_: f32;
    var phi_25105_: f32;
    var phi_25111_: f32;
    var phi_25139_: f32;
    var phi_25145_: f32;
    var phi_25173_: f32;
    var phi_25179_: f32;
    var phi_25207_: f32;
    var phi_25213_: f32;
    var phi_25241_: f32;
    var phi_25247_: f32;
    var phi_25275_: f32;
    var phi_25281_: f32;
    var phi_25309_: f32;
    var phi_25315_: f32;
    var phi_5929_: u32;
    var phi_5932_: f32;
    var phi_5934_: f32;
    var phi_5936_: f32;
    var phi_5938_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6354_: f32;
    var phi_6406_: vec3<f32>;
    var phi_6407_: f32;
    var phi_6408_: vec3<f32>;
    var phi_6409_: f32;
    var phi_6410_: vec3<f32>;
    var phi_6411_: f32;
    var phi_6413_: vec3<f32>;
    var phi_6414_: f32;
    var phi_6415_: f32;
    var phi_6416_: vec3<f32>;
    var phi_6417_: vec3<f32>;
    var phi_6418_: f32;
    var phi_6419_: f32;
    var phi_6420_: vec3<f32>;
    var phi_6421_: vec3<f32>;
    var phi_6422_: f32;
    var phi_6423_: f32;
    var phi_6424_: vec3<f32>;
    var phi_6425_: vec3<f32>;
    var phi_6426_: f32;
    var phi_6427_: f32;
    var phi_6428_: vec3<f32>;
    var phi_6429_: vec3<f32>;
    var phi_6431_: f32;
    var phi_6432_: f32;
    var phi_6433_: vec3<f32>;
    var phi_6434_: vec3<f32>;
    var phi_6435_: f32;
    var phi_6436_: f32;
    var phi_6437_: f32;
    var phi_6438_: f32;
    var phi_6439_: vec3<f32>;
    var phi_6440_: vec3<f32>;
    var phi_6441_: f32;
    var phi_6442_: f32;
    var phi_5701_: f32;
    var phi_5704_: f32;
    var phi_5706_: u32;
    var phi_5708_: vec3<f32>;
    var phi_5710_: vec3<f32>;
    var phi_5712_: f32;
    var phi_5714_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_25391_: bool;
    var local_29: f32;
    var phi_6474_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6553_: f32;
    var phi_6554_: vec3<f32>;
    var phi_6555_: f32;
    var phi_6557_: vec3<f32>;
    var phi_6560_: u32;
    var phi_7008_: f32;
    var phi_7188_: u32;
    var phi_7191_: f32;
    var phi_7193_: f32;
    var phi_7195_: f32;
    var phi_7197_: vec3<f32>;
    var phi_25480_: f32;
    var phi_25486_: f32;
    var phi_25514_: f32;
    var phi_25520_: f32;
    var phi_25548_: f32;
    var phi_25554_: f32;
    var phi_25582_: f32;
    var phi_25588_: f32;
    var phi_25616_: f32;
    var phi_25622_: f32;
    var phi_25650_: f32;
    var phi_25656_: f32;
    var phi_25684_: f32;
    var phi_25690_: f32;
    var phi_25718_: f32;
    var phi_25724_: f32;
    var phi_7189_: u32;
    var phi_7192_: f32;
    var phi_7194_: f32;
    var phi_7196_: f32;
    var phi_7198_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7611_: f32;
    var phi_25770_: bool;
    var phi_25785_: bool;
    var phi_7704_: vec3<f32>;
    var phi_7705_: vec3<f32>;
    var phi_7706_: vec3<f32>;
    var phi_7707_: vec3<f32>;
    var phi_6558_: vec3<f32>;
    var phi_6561_: u32;
    var phi_7726_: vec3<f32>;
    var phi_7729_: u32;
    var phi_7731_: f32;
    var phi_8032_: vec3<f32>;
    var phi_8033_: u32;
    var phi_8034_: f32;
    var phi_8035_: vec3<f32>;
    var phi_8052_: vec3<f32>;
    var phi_8054_: u32;
    var phi_8057_: vec3<f32>;
    var phi_8059_: f32;
    var phi_8061_: f32;
    var phi_8063_: f32;
    var phi_8083_: bool;
    var phi_8099_: bool;
    var phi_8101_: bool;
    var phi_8103_: bool;
    var phi_8104_: bool;
    var phi_8105_: bool;
    var phi_8106_: bool;
    var phi_8107_: bool;
    var phi_8151_: vec3<f32>;
    var phi_25804_: vec3<f32>;
    var phi_8152_: u32;
    var phi_8153_: vec3<f32>;
    var phi_8154_: u32;
    var phi_8155_: vec3<f32>;
    var phi_8290_: u32;
    var phi_8293_: f32;
    var phi_8295_: f32;
    var phi_8297_: f32;
    var phi_8299_: vec3<f32>;
    var phi_25859_: f32;
    var phi_25865_: f32;
    var phi_25893_: f32;
    var phi_25899_: f32;
    var phi_25927_: f32;
    var phi_25933_: f32;
    var phi_25961_: f32;
    var phi_25967_: f32;
    var phi_25995_: f32;
    var phi_26001_: f32;
    var phi_26029_: f32;
    var phi_26035_: f32;
    var phi_26063_: f32;
    var phi_26069_: f32;
    var phi_26097_: f32;
    var phi_26103_: f32;
    var phi_8291_: u32;
    var phi_8294_: f32;
    var phi_8296_: f32;
    var phi_8298_: f32;
    var phi_8300_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8713_: f32;
    var phi_8739_: vec3<f32>;
    var phi_8740_: f32;
    var phi_8741_: f32;
    var phi_8742_: vec3<f32>;
    var phi_8743_: f32;
    var phi_8744_: f32;
    var phi_8745_: vec3<f32>;
    var phi_8746_: f32;
    var phi_8747_: f32;
    var phi_8748_: vec3<f32>;
    var phi_8749_: f32;
    var phi_8750_: f32;
    var phi_8752_: vec3<f32>;
    var phi_8753_: f32;
    var phi_8754_: f32;
    var phi_8755_: f32;
    var phi_8055_: u32;
    var phi_8058_: vec3<f32>;
    var phi_8060_: f32;
    var phi_8062_: f32;
    var phi_8064_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9045_: type_59;
    var local_42: f32;
    var phi_9046_: type_59;
    var local_43: f32;
    var local_44: f32;
    var phi_9047_: type_59;
    var phi_9048_: type_59;
    var phi_7727_: vec3<f32>;
    var phi_7730_: u32;
    var phi_7732_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9077_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_34216_: bool;
    var phi_9124_: vec3<f32>;
    var phi_34215_: bool;
    var phi_9125_: vec3<f32>;
    var phi_34214_: bool;
    var phi_9126_: vec3<f32>;
    var phi_34210_: bool;
    var phi_10399_: vec3<f32>;
    var phi_34209_: bool;
    var phi_10400_: vec3<f32>;
    var phi_34208_: bool;
    var phi_518_: vec3<f32>;
    var phi_521_: u32;
    var phi_34207_: bool;
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
            let _e280 = global;
            let _e290 = global_1.member.member_7[2u];
            let _e294 = (_e280.x + (_e280.y * 4194240u));
            let _e299 = atomicLoad((&global_8.member[((_e290 * 4u) + 3u)]));
            if (_e294 >= _e299) {
                phi_497_ = type_48(0u, type_48().member_1);
            } else {
                let _e304 = global_1.member.member[0u];
                let _e308 = global_1.member.member[1u];
                let _e313 = global_1.member.member_7[3u];
                let _e320 = atomicLoad((&global_8.member[((12u + (_e290 * ((_e304 * _e308) * _e313))) + _e294)]));
                phi_497_ = type_48(1u, _e320);
            }
            let _e326 = phi_497_;
            if (bitcast<i32>(_e326.member) != 0i) {
                phi_33746_ = false;
                phi_517_ = vec3<f32>(0f, 0f, 0f);
                phi_520_ = 0u;
                loop {
                    let _e332 = phi_33746_;
                    let _e334 = phi_517_;
                    let _e336 = phi_520_;
                    local_55 = _e334;
                    local_56 = _e334;
                    local_57 = _e334;
                    let _e337 = (_e336 < 2u);
                    if _e337 {
                        let _e339 = ((2u * _e326.member_1) + _e336);
                        if (_e339 < arrayLength((&global_9.member))) {
                        } else {
                            phi_34207_ = true;
                            break;
                        }
                        let _e343 = global_9.member[_e339];
                        if ((((_e343.member_2.x * _e343.member_2.x) + (_e343.member_2.y * _e343.member_2.y)) + (_e343.member_2.z * _e343.member_2.z)) == 0f) {
                            phi_34209_ = _e332;
                            phi_10400_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e343.member.w < 0f) {
                                let _e1298 = global_1.member.member_8[0u];
                                let _e1301 = global_1.member.member_8[3u];
                                if (_e1298 == 4294967295u) {
                                    let _e1303 = (_e1301 == 4294967295u);
                                    if _e1303 {
                                        let _e1306 = global_1.member.member_3[1u];
                                        let _e1309 = global_1.member.member_3[2u];
                                        let _e1312 = ((atan2(_e343.member_1.z, _e343.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1317 = (((_e1312 - floor(_e1312)) * f32(_e1306)) - 0.5f);
                                        let _e1319 = select(_e343.member_1.y, -1f, (_e343.member_1.y < -1f));
                                        let _e1326 = (((acos(select(_e1319, 1f, (_e1319 > 1f))) * 0.31830987f) * f32(_e1309)) - 0.5f);
                                        let _e1327 = (_e1309 - 1u);
                                        let _e1328 = f32(_e1327);
                                        if (0f <= _e1328) {
                                        } else {
                                            phi_34207_ = true;
                                            break;
                                        }
                                        let _e1331 = select(_e1326, 0f, (_e1326 < 0f));
                                        let _e1333 = select(_e1331, _e1328, (_e1331 > _e1328));
                                        let _e1334 = floor(_e1317);
                                        let _e1335 = floor(_e1333);
                                        let _e1336 = (_e1317 - _e1334);
                                        let _e1337 = (_e1333 - _e1335);
                                        let _e1348 = (_e1306 == 0u);
                                        if _e1348 {
                                            phi_34207_ = true;
                                            break;
                                        }
                                        let _e1349 = (bitcast<u32>((select(0i, select(select(i32(_e1334), i32(-2147483648), (_e1334 < -2147483600f)), 2147483647i, (_e1334 > 2147483500f)), (_e1334 == _e1334)) + bitcast<i32>(_e1306))) % _e1306);
                                        let _e1354 = select(select(u32(_e1335), 0u, (_e1335 < 0f)), 4294967295u, (_e1335 > 4294967000f));
                                        let _e1355 = (_e1354 + 1u);
                                        let _e1360 = global_1.member.member_2[2u];
                                        let _e1361 = (_e1354 * _e1306);
                                        if _e1348 {
                                            phi_34207_ = true;
                                            break;
                                        }
                                        let _e1362 = (_e1349 % _e1306);
                                        let _e1365 = (_e1360 + (3u * (_e1361 + _e1362)));
                                        let _e1367 = global_6.member[_e1365];
                                        let _e1370 = global_6.member[(_e1365 + 1u)];
                                        let _e1373 = global_6.member[(_e1365 + 2u)];
                                        if _e1348 {
                                            phi_34207_ = true;
                                            break;
                                        }
                                        let _e1376 = ((_e1349 + 1u) % _e1306);
                                        let _e1379 = (_e1360 + (3u * (_e1361 + _e1376)));
                                        let _e1381 = global_6.member[_e1379];
                                        let _e1384 = global_6.member[(_e1379 + 1u)];
                                        let _e1387 = global_6.member[(_e1379 + 2u)];
                                        let _e1395 = (vec3<f32>(_e1367, _e1370, _e1373) + vec3<f32>(((_e1381 - _e1367) * _e1336), ((_e1384 - _e1370) * _e1336), ((_e1387 - _e1373) * _e1336)));
                                        let _e1396 = (select(_e1355, _e1327, (_e1327 < _e1355)) * _e1306);
                                        if _e1348 {
                                            phi_34207_ = true;
                                            break;
                                        }
                                        let _e1399 = (_e1360 + (3u * (_e1396 + _e1362)));
                                        let _e1401 = global_6.member[_e1399];
                                        let _e1404 = global_6.member[(_e1399 + 1u)];
                                        let _e1407 = global_6.member[(_e1399 + 2u)];
                                        if _e1348 {
                                            phi_34207_ = true;
                                            break;
                                        }
                                        let _e1411 = (_e1360 + (3u * (_e1396 + _e1376)));
                                        let _e1413 = global_6.member[_e1411];
                                        let _e1416 = global_6.member[(_e1411 + 1u)];
                                        let _e1419 = global_6.member[(_e1411 + 2u)];
                                        let _e1427 = (vec3<f32>(_e1401, _e1404, _e1407) + vec3<f32>(((_e1413 - _e1401) * _e1336), ((_e1416 - _e1404) * _e1336), ((_e1419 - _e1407) * _e1336)));
                                        phi_896_ = (_e1395 + vec3<f32>(((_e1427.x - _e1395.x) * _e1337), ((_e1427.y - _e1395.y) * _e1337), ((_e1427.z - _e1395.z) * _e1337)));
                                    } else {
                                        phi_896_ = vec3<f32>();
                                    }
                                    let _e1443 = phi_896_;
                                    phi_898_ = _e1443;
                                    phi_899_ = select(true, false, _e1303);
                                } else {
                                    phi_898_ = vec3<f32>();
                                    phi_899_ = true;
                                }
                                let _e1446 = phi_898_;
                                let _e1448 = phi_899_;
                                if _e1448 {
                                    let _e1449 = (_e1301 != 4294967295u);
                                    if (_e1298 != 4294967295u) {
                                        phi_33821_ = _e332;
                                        phi_909_ = vec3<f32>(0f, 0f, 0f);
                                        phi_912_ = 0i;
                                        phi_914_ = _e1298;
                                        loop {
                                            let _e1452 = phi_33821_;
                                            let _e1454 = phi_909_;
                                            let _e1456 = phi_912_;
                                            let _e1458 = phi_914_;
                                            local_17 = _e1454;
                                            local_18 = _e1454;
                                            local_19 = _e1454;
                                            local_61 = _e1454;
                                            local_62 = _e1454;
                                            if (_e1458 != 4294967295u) {
                                                let _e1460 = (_e1456 < 2i);
                                                if _e1460 {
                                                    let _e1462 = global_6.member[_e1458];
                                                    let _e1465 = global_6.member[(_e1458 + 1u)];
                                                    let _e1468 = global_6.member[(_e1458 + 2u)];
                                                    let _e1471 = global_6.member[(_e1458 + 3u)];
                                                    let _e1474 = global_6.member[(_e1458 + 4u)];
                                                    let _e1477 = global_6.member[(_e1458 + 5u)];
                                                    let _e1480 = global_6.member[(_e1458 + 6u)];
                                                    let _e1483 = global_6.member[(_e1458 + 7u)];
                                                    let _e1486 = global_6.member[(_e1458 + 8u)];
                                                    let _e1489 = global_6.member[(_e1458 + 9u)];
                                                    let _e1492 = global_6.member[(_e1458 + 10u)];
                                                    let _e1495 = global_6.member[(_e1458 + 11u)];
                                                    let _e1498 = global_6.member[(_e1458 + 12u)];
                                                    let _e1501 = global_6.member[(_e1458 + 13u)];
                                                    let _e1504 = global_6.member[(_e1458 + 14u)];
                                                    let _e1507 = global_6.member[(_e1458 + 15u)];
                                                    let _e1511 = global_6.member[(_e1458 + 16u)];
                                                    let _e1514 = global_6.member[(_e1458 + 17u)];
                                                    let _e1517 = global_6.member[(_e1458 + 18u)];
                                                    let _e1520 = global_6.member[(_e1458 + 19u)];
                                                    let _e1523 = global_6.member[(_e1458 + 20u)];
                                                    let _e1526 = global_6.member[(_e1458 + 30u)];
                                                    let _e1527 = bitcast<u32>(_e1526);
                                                    let _e1530 = global_6.member[(_e1458 + 31u)];
                                                    let _e1531 = bitcast<u32>(_e1530);
                                                    let _e1534 = global_6.member[(_e1458 + 32u)];
                                                    let _e1536 = (_e343.member_1.y < 0f);
                                                    if _e1536 {
                                                        phi_2702_ = type_52();
                                                        phi_2703_ = true;
                                                        phi_2704_ = false;
                                                    } else {
                                                        let _e1537 = (_e1531 == 4294967295u);
                                                        if _e1537 {
                                                            phi_2700_ = type_52();
                                                            phi_2701_ = false;
                                                        } else {
                                                            let _e1551 = (_e343.member.x - _e1462);
                                                            let _e1552 = (_e343.member.y - _e1465);
                                                            let _e1553 = (_e343.member.z - _e1468);
                                                            let _e1561 = ((((((_e343.member.x * _e343.member.x) + (_e343.member.y * _e343.member.y)) + (_e343.member.z * _e343.member.z)) - (2f * (((_e343.member.x * _e1462) + (_e343.member.y * _e1465)) + (_e343.member.z * _e1468)))) + _e1477) / (sqrt((((_e1551 * _e1551) + (_e1552 * _e1552)) + (_e1553 * _e1553))) + _e1471));
                                                            let _e1562 = (_e1458 + 21u);
                                                            let _e1565 = global_6.member[(_e1458 + 28u)];
                                                            if (_e1561 > _e1565) {
                                                                phi_2690_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                let _e1568 = global_6.member[_e1562];
                                                                if (_e1561 != _e1561) {
                                                                    phi_22969_ = true;
                                                                } else {
                                                                    phi_22969_ = (_e1568 >= _e1561);
                                                                }
                                                                let _e1572 = phi_22969_;
                                                                let _e1573 = select(_e1561, _e1568, _e1572);
                                                                phi_1336_ = 0u;
                                                                loop {
                                                                    let _e1575 = phi_1336_;
                                                                    local_8 = _e1575;
                                                                    local_9 = _e1575;
                                                                    local_10 = _e1575;
                                                                    if (_e1575 < 8u) {
                                                                        let _e1579 = global_6.member[(_e1562 + _e1575)];
                                                                        let _e1580 = (_e1579 <= _e1573);
                                                                        if _e1580 {
                                                                            phi_1366_ = (_e1575 + 1u);
                                                                        } else {
                                                                            phi_1366_ = u32();
                                                                        }
                                                                        let _e1583 = phi_1366_;
                                                                        phi_1337_ = _e1583;
                                                                        phi_1367_ = _e1580;
                                                                    } else {
                                                                        phi_1337_ = u32();
                                                                        phi_1367_ = false;
                                                                    }
                                                                    let _e1585 = phi_1337_;
                                                                    let _e1587 = phi_1367_;
                                                                    phi_1336_ = _e1585;
                                                                    if !(_e1587) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1590 = local_8;
                                                                if (_e1590 < 1u) {
                                                                    phi_1385_ = 1u;
                                                                } else {
                                                                    let _e1593 = local_9;
                                                                    let _e1596 = local_10;
                                                                    phi_1385_ = select(_e1596, 7u, (_e1593 > 7u));
                                                                }
                                                                let _e1599 = phi_1385_;
                                                                let _e1600 = (_e1599 - 1u);
                                                                let _e1603 = global_6.member[(_e1562 + _e1600)];
                                                                let _e1606 = global_6.member[(_e1562 + _e1599)];
                                                                let _e1609 = ((_e1573 - _e1603) / (_e1606 - _e1603));
                                                                let _e1610 = select(_e343.member_1.y, 0f, _e1536);
                                                                let _e1616 = (sqrt((asin(select(_e1610, 1f, (_e1610 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1618 = (atan2(_e343.member_1.z, _e343.member_1.x) * 0.15915494f);
                                                                let _e1621 = ((_e1618 - floor(_e1618)) * 256f);
                                                                let _e1622 = floor(_e1621);
                                                                let _e1628 = (select(select(u32(_e1622), 0u, (_e1622 < 0f)), 4294967295u, (_e1622 > 4294967000f)) % 256u);
                                                                let _e1629 = floor(_e1616);
                                                                let _e1634 = select(select(u32(_e1629), 0u, (_e1629 < 0f)), 4294967295u, (_e1629 > 4294967000f));
                                                                let _e1636 = select(_e1634, 126u, (126u < _e1634));
                                                                let _e1638 = ((_e1628 + 1u) % 256u);
                                                                let _e1640 = (_e1621 - _e1622);
                                                                let _e1642 = (_e1616 - f32(_e1636));
                                                                let _e1644 = (_e1531 + (_e1600 * 294912u));
                                                                let _e1645 = (_e1636 * 256u);
                                                                let _e1647 = (9u * (_e1645 + _e1628));
                                                                let _e1648 = (_e1644 + _e1647);
                                                                let _e1650 = (9u * (_e1645 + _e1638));
                                                                let _e1651 = (_e1644 + _e1650);
                                                                let _e1652 = ((_e1636 + 1u) * 256u);
                                                                let _e1654 = (9u * (_e1652 + _e1628));
                                                                let _e1655 = (_e1644 + _e1654);
                                                                let _e1657 = (9u * (_e1652 + _e1638));
                                                                let _e1658 = (_e1644 + _e1657);
                                                                let _e1659 = (1f - _e1640);
                                                                let _e1660 = (1f - _e1642);
                                                                let _e1661 = (_e1659 * _e1660);
                                                                let _e1662 = (_e1640 * _e1660);
                                                                let _e1663 = (_e1659 * _e1642);
                                                                let _e1664 = (_e1640 * _e1642);
                                                                let _e1666 = global_6.member[_e1648];
                                                                let _e1669 = global_6.member[(_e1648 + 1u)];
                                                                let _e1672 = global_6.member[(_e1648 + 2u)];
                                                                let _e1677 = global_6.member[_e1651];
                                                                let _e1680 = global_6.member[(_e1651 + 1u)];
                                                                let _e1683 = global_6.member[(_e1651 + 2u)];
                                                                let _e1691 = global_6.member[_e1655];
                                                                let _e1694 = global_6.member[(_e1655 + 1u)];
                                                                let _e1697 = global_6.member[(_e1655 + 2u)];
                                                                let _e1705 = global_6.member[_e1658];
                                                                let _e1708 = global_6.member[(_e1658 + 1u)];
                                                                let _e1711 = global_6.member[(_e1658 + 2u)];
                                                                let _e1720 = global_6.member[(_e1648 + 3u)];
                                                                let _e1723 = global_6.member[(_e1648 + 4u)];
                                                                let _e1726 = global_6.member[(_e1648 + 5u)];
                                                                let _e1732 = global_6.member[(_e1651 + 3u)];
                                                                let _e1735 = global_6.member[(_e1651 + 4u)];
                                                                let _e1738 = global_6.member[(_e1651 + 5u)];
                                                                let _e1747 = global_6.member[(_e1655 + 3u)];
                                                                let _e1750 = global_6.member[(_e1655 + 4u)];
                                                                let _e1753 = global_6.member[(_e1655 + 5u)];
                                                                let _e1762 = global_6.member[(_e1658 + 3u)];
                                                                let _e1765 = global_6.member[(_e1658 + 4u)];
                                                                let _e1768 = global_6.member[(_e1658 + 5u)];
                                                                let _e1777 = global_6.member[(_e1648 + 6u)];
                                                                let _e1780 = global_6.member[(_e1648 + 7u)];
                                                                let _e1783 = global_6.member[(_e1648 + 8u)];
                                                                let _e1789 = global_6.member[(_e1651 + 6u)];
                                                                let _e1792 = global_6.member[(_e1651 + 7u)];
                                                                let _e1795 = global_6.member[(_e1651 + 8u)];
                                                                let _e1804 = global_6.member[(_e1655 + 6u)];
                                                                let _e1807 = global_6.member[(_e1655 + 7u)];
                                                                let _e1810 = global_6.member[(_e1655 + 8u)];
                                                                let _e1819 = global_6.member[(_e1658 + 6u)];
                                                                let _e1822 = global_6.member[(_e1658 + 7u)];
                                                                let _e1825 = global_6.member[(_e1658 + 8u)];
                                                                let _e1833 = (_e1531 + (_e1599 * 294912u));
                                                                let _e1834 = (_e1833 + _e1647);
                                                                let _e1835 = (_e1833 + _e1650);
                                                                let _e1836 = (_e1833 + _e1654);
                                                                let _e1837 = (_e1833 + _e1657);
                                                                let _e1839 = global_6.member[_e1834];
                                                                let _e1842 = global_6.member[(_e1834 + 1u)];
                                                                let _e1845 = global_6.member[(_e1834 + 2u)];
                                                                let _e1850 = global_6.member[_e1835];
                                                                let _e1853 = global_6.member[(_e1835 + 1u)];
                                                                let _e1856 = global_6.member[(_e1835 + 2u)];
                                                                let _e1864 = global_6.member[_e1836];
                                                                let _e1867 = global_6.member[(_e1836 + 1u)];
                                                                let _e1870 = global_6.member[(_e1836 + 2u)];
                                                                let _e1878 = global_6.member[_e1837];
                                                                let _e1881 = global_6.member[(_e1837 + 1u)];
                                                                let _e1884 = global_6.member[(_e1837 + 2u)];
                                                                let _e1893 = global_6.member[(_e1834 + 3u)];
                                                                let _e1896 = global_6.member[(_e1834 + 4u)];
                                                                let _e1899 = global_6.member[(_e1834 + 5u)];
                                                                let _e1905 = global_6.member[(_e1835 + 3u)];
                                                                let _e1908 = global_6.member[(_e1835 + 4u)];
                                                                let _e1911 = global_6.member[(_e1835 + 5u)];
                                                                let _e1920 = global_6.member[(_e1836 + 3u)];
                                                                let _e1923 = global_6.member[(_e1836 + 4u)];
                                                                let _e1926 = global_6.member[(_e1836 + 5u)];
                                                                let _e1935 = global_6.member[(_e1837 + 3u)];
                                                                let _e1938 = global_6.member[(_e1837 + 4u)];
                                                                let _e1941 = global_6.member[(_e1837 + 5u)];
                                                                let _e1950 = global_6.member[(_e1834 + 6u)];
                                                                let _e1953 = global_6.member[(_e1834 + 7u)];
                                                                let _e1956 = global_6.member[(_e1834 + 8u)];
                                                                let _e1962 = global_6.member[(_e1835 + 6u)];
                                                                let _e1965 = global_6.member[(_e1835 + 7u)];
                                                                let _e1968 = global_6.member[(_e1835 + 8u)];
                                                                let _e1977 = global_6.member[(_e1836 + 6u)];
                                                                let _e1980 = global_6.member[(_e1836 + 7u)];
                                                                let _e1983 = global_6.member[(_e1836 + 8u)];
                                                                let _e1992 = global_6.member[(_e1837 + 6u)];
                                                                let _e1995 = global_6.member[(_e1837 + 7u)];
                                                                let _e1998 = global_6.member[(_e1837 + 8u)];
                                                                let _e2005 = (1f - _e1609);
                                                                phi_2690_ = type_55(1u, type_52(vec3<f32>(((((((_e1666 * _e1661) + (_e1677 * _e1662)) + (_e1691 * _e1663)) + (_e1705 * _e1664)) * _e2005) + (((((_e1839 * _e1661) + (_e1850 * _e1662)) + (_e1864 * _e1663)) + (_e1878 * _e1664)) * _e1609)), ((((((_e1669 * _e1661) + (_e1680 * _e1662)) + (_e1694 * _e1663)) + (_e1708 * _e1664)) * _e2005) + (((((_e1842 * _e1661) + (_e1853 * _e1662)) + (_e1867 * _e1663)) + (_e1881 * _e1664)) * _e1609)), ((((((_e1672 * _e1661) + (_e1683 * _e1662)) + (_e1697 * _e1663)) + (_e1711 * _e1664)) * _e2005) + (((((_e1845 * _e1661) + (_e1856 * _e1662)) + (_e1870 * _e1663)) + (_e1884 * _e1664)) * _e1609))), vec3<f32>(((((((_e1720 * _e1661) + (_e1732 * _e1662)) + (_e1747 * _e1663)) + (_e1762 * _e1664)) * _e2005) + (((((_e1893 * _e1661) + (_e1905 * _e1662)) + (_e1920 * _e1663)) + (_e1935 * _e1664)) * _e1609)), ((((((_e1723 * _e1661) + (_e1735 * _e1662)) + (_e1750 * _e1663)) + (_e1765 * _e1664)) * _e2005) + (((((_e1896 * _e1661) + (_e1908 * _e1662)) + (_e1923 * _e1663)) + (_e1938 * _e1664)) * _e1609)), ((((((_e1726 * _e1661) + (_e1738 * _e1662)) + (_e1753 * _e1663)) + (_e1768 * _e1664)) * _e2005) + (((((_e1899 * _e1661) + (_e1911 * _e1662)) + (_e1926 * _e1663)) + (_e1941 * _e1664)) * _e1609))), vec3<f32>(((((((_e1777 * _e1661) + (_e1789 * _e1662)) + (_e1804 * _e1663)) + (_e1819 * _e1664)) * _e2005) + (((((_e1950 * _e1661) + (_e1962 * _e1662)) + (_e1977 * _e1663)) + (_e1992 * _e1664)) * _e1609)), ((((((_e1780 * _e1661) + (_e1792 * _e1662)) + (_e1807 * _e1663)) + (_e1822 * _e1664)) * _e2005) + (((((_e1953 * _e1661) + (_e1965 * _e1662)) + (_e1980 * _e1663)) + (_e1995 * _e1664)) * _e1609)), ((((((_e1783 * _e1661) + (_e1795 * _e1662)) + (_e1810 * _e1663)) + (_e1825 * _e1664)) * _e2005) + (((((_e1956 * _e1661) + (_e1968 * _e1662)) + (_e1983 * _e1663)) + (_e1998 * _e1664)) * _e1609)))));
                                                            }
                                                            let _e2042 = phi_2690_;
                                                            let _e2045 = (bitcast<i32>(_e2042.member) != 0i);
                                                            if _e2045 {
                                                                phi_2698_ = _e2042.member_1;
                                                            } else {
                                                                phi_2698_ = type_52();
                                                            }
                                                            let _e2048 = phi_2698_;
                                                            phi_2700_ = _e2048;
                                                            phi_2701_ = select(true, false, _e2045);
                                                        }
                                                        let _e2051 = phi_2700_;
                                                        let _e2053 = phi_2701_;
                                                        phi_2702_ = _e2051;
                                                        phi_2703_ = _e1537;
                                                        phi_2704_ = _e2053;
                                                    }
                                                    let _e2055 = phi_2702_;
                                                    let _e2057 = phi_2703_;
                                                    let _e2059 = phi_2704_;
                                                    if select(_e2059, true, _e2057) {
                                                        let _e2068 = ((((_e343.member.x - _e1462) * _e343.member_1.x) + ((_e343.member.y - _e1465) * _e343.member_1.y)) + ((_e343.member.z - _e1468) * _e343.member_1.z));
                                                        let _e2080 = ((((_e343.member.x * _e343.member.x) + (_e343.member.y * _e343.member.y)) + (_e343.member.z * _e343.member.z)) - (2f * (((_e343.member.x * _e1462) + (_e343.member.y * _e1465)) + (_e343.member.z * _e1468))));
                                                        let _e2081 = (_e2080 + _e1480);
                                                        let _e2082 = (_e2068 * _e2068);
                                                        let _e2083 = (_e2082 - _e2081);
                                                        if (_e2083 < 0f) {
                                                            phi_23108_ = type_57(0u, type_57().member_1);
                                                            phi_23109_ = type_57();
                                                            phi_23110_ = true;
                                                        } else {
                                                            if (_e2068 < 0f) {
                                                                phi_23080_ = -(sqrt(_e2083));
                                                            } else {
                                                                phi_23080_ = sqrt(_e2083);
                                                            }
                                                            let _e2090 = phi_23080_;
                                                            let _e2092 = -((_e2068 + _e2090));
                                                            let _e2093 = (_e2092 == 0f);
                                                            if _e2093 {
                                                                phi_23103_ = type_57(1u, type_56(0f, 0f));
                                                                phi_23104_ = type_57();
                                                            } else {
                                                                let _e2094 = (_e2081 / _e2092);
                                                                if (_e2092 < _e2094) {
                                                                    phi_23092_ = type_56(_e2092, _e2094);
                                                                } else {
                                                                    phi_23092_ = type_56(_e2094, _e2092);
                                                                }
                                                                let _e2099 = phi_23092_;
                                                                phi_23103_ = type_57();
                                                                phi_23104_ = type_57(1u, _e2099);
                                                            }
                                                            let _e2102 = phi_23103_;
                                                            let _e2104 = phi_23104_;
                                                            phi_23108_ = _e2102;
                                                            phi_23109_ = _e2104;
                                                            phi_23110_ = _e2093;
                                                        }
                                                        let _e2109 = phi_23108_;
                                                        let _e2111 = phi_23109_;
                                                        let _e2113 = phi_23110_;
                                                        if _e2113 {
                                                            phi_23114_ = _e2109;
                                                        } else {
                                                            phi_23114_ = _e2111;
                                                        }
                                                        let _e2115 = phi_23114_;
                                                        if (bitcast<i32>(_e2115.member) != 0i) {
                                                            phi_23001_ = type_57(0u, _e2115.member_1);
                                                        } else {
                                                            phi_23001_ = type_57(1u, type_57().member_1);
                                                        }
                                                        let _e2125 = phi_23001_;
                                                        if (bitcast<i32>(_e2125.member) != 0i) {
                                                            phi_23048_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e2125.member_1.member_1 <= 0f) {
                                                                phi_23044_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e2125.member_1.member != _e2125.member_1.member) {
                                                                    phi_23152_ = true;
                                                                } else {
                                                                    phi_23152_ = (0f >= _e2125.member_1.member);
                                                                }
                                                                let _e2137 = phi_23152_;
                                                                let _e2138 = select(_e2125.member_1.member, 0f, _e2137);
                                                                let _e2139 = (_e2080 + _e1477);
                                                                if (_e2139 > 0f) {
                                                                    let _e2141 = (_e2082 - _e2139);
                                                                    if (_e2141 < 0f) {
                                                                        phi_23260_ = type_57(0u, type_57().member_1);
                                                                        phi_23261_ = type_57();
                                                                        phi_23262_ = true;
                                                                    } else {
                                                                        if (_e2068 < 0f) {
                                                                            phi_23232_ = -(sqrt(_e2141));
                                                                        } else {
                                                                            phi_23232_ = sqrt(_e2141);
                                                                        }
                                                                        let _e2148 = phi_23232_;
                                                                        let _e2150 = -((_e2068 + _e2148));
                                                                        let _e2151 = (_e2150 == 0f);
                                                                        if _e2151 {
                                                                            phi_23255_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23256_ = type_57();
                                                                        } else {
                                                                            let _e2152 = (_e2139 / _e2150);
                                                                            if (_e2150 < _e2152) {
                                                                                phi_23244_ = type_56(_e2150, _e2152);
                                                                            } else {
                                                                                phi_23244_ = type_56(_e2152, _e2150);
                                                                            }
                                                                            let _e2157 = phi_23244_;
                                                                            phi_23255_ = type_57();
                                                                            phi_23256_ = type_57(1u, _e2157);
                                                                        }
                                                                        let _e2160 = phi_23255_;
                                                                        let _e2162 = phi_23256_;
                                                                        phi_23260_ = _e2160;
                                                                        phi_23261_ = _e2162;
                                                                        phi_23262_ = _e2151;
                                                                    }
                                                                    let _e2167 = phi_23260_;
                                                                    let _e2169 = phi_23261_;
                                                                    let _e2171 = phi_23262_;
                                                                    if _e2171 {
                                                                        phi_23266_ = _e2167;
                                                                    } else {
                                                                        phi_23266_ = _e2169;
                                                                    }
                                                                    let _e2173 = phi_23266_;
                                                                    if (bitcast<i32>(_e2173.member) != 0i) {
                                                                        phi_23200_ = select(_e2173.member_1.member_1, _e2173.member_1.member, (_e2173.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23200_ = -1f;
                                                                    }
                                                                    let _e2184 = phi_23200_;
                                                                    if (_e2184 >= 0f) {
                                                                        if (_e2125.member_1.member_1 != _e2125.member_1.member_1) {
                                                                            phi_23304_ = true;
                                                                        } else {
                                                                            phi_23304_ = (_e2184 <= _e2125.member_1.member_1);
                                                                        }
                                                                        let _e2189 = phi_23304_;
                                                                        phi_23029_ = select(_e2125.member_1.member_1, _e2184, _e2189);
                                                                    } else {
                                                                        phi_23029_ = _e2125.member_1.member_1;
                                                                    }
                                                                    let _e2192 = phi_23029_;
                                                                    phi_23031_ = _e2192;
                                                                } else {
                                                                    phi_23031_ = _e2125.member_1.member_1;
                                                                }
                                                                let _e2194 = phi_23031_;
                                                                if (_e2194 <= _e2138) {
                                                                    phi_23040_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    phi_23040_ = type_57(1u, type_56(_e2138, _e2194));
                                                                }
                                                                let _e2202 = phi_23040_;
                                                                phi_23044_ = _e2202;
                                                            }
                                                            let _e2207 = phi_23044_;
                                                            phi_23048_ = _e2207;
                                                        }
                                                        let _e2212 = phi_23048_;
                                                        if (bitcast<i32>(_e2212.member) != 0i) {
                                                            phi_2722_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2725_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2727_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2729_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2731_ = 0u;
                                                            loop {
                                                                let _e2221 = phi_2722_;
                                                                let _e2223 = phi_2725_;
                                                                let _e2225 = phi_2727_;
                                                                let _e2227 = phi_2729_;
                                                                let _e2229 = phi_2731_;
                                                                local_12 = _e2227;
                                                                local_13 = _e2225;
                                                                local_14 = _e2223;
                                                                let _e2230 = (_e2229 < 32u);
                                                                if _e2230 {
                                                                    let _e2231 = -(_e2068);
                                                                    if (_e2212.member_1.member <= _e2212.member_1.member_1) {
                                                                    } else {
                                                                        phi_33789_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2234 = select(_e2231, _e2212.member_1.member, (_e2231 < _e2212.member_1.member));
                                                                    let _e2236 = select(_e2234, _e2212.member_1.member_1, (_e2234 > _e2212.member_1.member_1));
                                                                    let _e2237 = (_e2236 - _e2212.member_1.member);
                                                                    let _e2238 = (_e2212.member_1.member_1 - _e2236);
                                                                    let _e2239 = (_e2212.member_1.member_1 - _e2212.member_1.member);
                                                                    let _e2240 = (0.000000001f * _e2239);
                                                                    if (_e2237 <= _e2240) {
                                                                        let _e2315 = (f32(_e2229) * 0.03125f);
                                                                        let _e2318 = (f32((_e2229 + 1u)) * 0.03125f);
                                                                        let _e2320 = ((_e2315 + _e2318) * 0.5f);
                                                                        phi_2832_ = type_56((_e2212.member_1.member + ((_e2239 * _e2320) * _e2320)), (_e2239 * ((_e2318 * _e2318) - (_e2315 * _e2315))));
                                                                    } else {
                                                                        if (_e2238 <= _e2240) {
                                                                            let _e2298 = (f32((31u - _e2229)) * 0.03125f);
                                                                            let _e2301 = (f32((32u - _e2229)) * 0.03125f);
                                                                            let _e2303 = ((_e2298 + _e2301) * 0.5f);
                                                                            phi_2831_ = type_56((_e2212.member_1.member_1 - ((_e2239 * _e2303) * _e2303)), (_e2239 * ((_e2301 * _e2301) - (_e2298 * _e2298))));
                                                                        } else {
                                                                            let _e2246 = round(((32f * _e2237) / (_e2237 + _e2238)));
                                                                            let _e2251 = select(select(u32(_e2246), 0u, (_e2246 < 0f)), 4294967295u, (_e2246 > 4294967000f));
                                                                            if (_e2251 < 1u) {
                                                                                phi_2815_ = 1u;
                                                                            } else {
                                                                                phi_2815_ = select(_e2251, 31u, (_e2251 > 31u));
                                                                            }
                                                                            let _e2256 = phi_2815_;
                                                                            if (_e2229 < _e2256) {
                                                                                let _e2277 = ((_e2256 - 1u) - _e2229);
                                                                                let _e2279 = f32(_e2256);
                                                                                let _e2280 = (f32(_e2277) / _e2279);
                                                                                let _e2283 = (f32((_e2277 + 1u)) / _e2279);
                                                                                let _e2285 = ((_e2280 + _e2283) * 0.5f);
                                                                                phi_2830_ = type_56((_e2236 - ((_e2237 * _e2285) * _e2285)), (_e2237 * ((_e2283 * _e2283) - (_e2280 * _e2280))));
                                                                            } else {
                                                                                let _e2259 = (_e2229 - _e2256);
                                                                                let _e2261 = f32((32u - _e2256));
                                                                                let _e2262 = (f32(_e2259) / _e2261);
                                                                                let _e2265 = (f32((_e2259 + 1u)) / _e2261);
                                                                                let _e2267 = ((_e2262 + _e2265) * 0.5f);
                                                                                phi_2830_ = type_56((_e2236 + ((_e2238 * _e2267) * _e2267)), (_e2238 * ((_e2265 * _e2265) - (_e2262 * _e2262))));
                                                                            }
                                                                            let _e2295 = phi_2830_;
                                                                            phi_2831_ = _e2295;
                                                                        }
                                                                        let _e2313 = phi_2831_;
                                                                        phi_2832_ = _e2313;
                                                                    }
                                                                    let _e2330 = phi_2832_;
                                                                    let _e2336 = (_e343.member.x + (_e343.member_1.x * _e2330.member));
                                                                    let _e2337 = (_e343.member.y + (_e343.member_1.y * _e2330.member));
                                                                    let _e2338 = (_e343.member.z + (_e343.member_1.z * _e2330.member));
                                                                    let _e2351 = ((((_e2336 * _e2336) + (_e2337 * _e2337)) + (_e2338 * _e2338)) - (2f * (((_e2336 * _e1462) + (_e2337 * _e1465)) + (_e2338 * _e1468))));
                                                                    let _e2352 = (_e2351 + _e1477);
                                                                    let _e2353 = (_e2336 - _e1462);
                                                                    let _e2354 = (_e2337 - _e1465);
                                                                    let _e2355 = (_e2338 - _e1468);
                                                                    let _e2361 = sqrt((((_e2353 * _e2353) + (_e2354 * _e2354)) + (_e2355 * _e2355)));
                                                                    let _e2363 = (_e2352 / (_e2361 + _e1471));
                                                                    let _e2367 = (1f - (abs((_e2363 - 25000f)) * 0.00006666667f));
                                                                    if (_e2367 != _e2367) {
                                                                        phi_23503_ = true;
                                                                    } else {
                                                                        phi_23503_ = (0f >= _e2367);
                                                                    }
                                                                    let _e2371 = phi_23503_;
                                                                    let _e2373 = -(_e2363);
                                                                    let _e2376 = (exp((_e2373 / _e1492)) * _e2330.member_1);
                                                                    let _e2379 = (exp((_e2373 / _e1495)) * _e2330.member_1);
                                                                    let _e2381 = (_e2376 * 0.5f);
                                                                    let _e2382 = (_e2379 * 0.5f);
                                                                    let _e2383 = ((select(_e2367, 0f, _e2371) * _e2330.member_1) * 0.5f);
                                                                    let _e2385 = (_e2221.x + _e2381);
                                                                    let _e2387 = (_e2221.y + _e2382);
                                                                    let _e2389 = (_e2221.z + _e2383);
                                                                    let _e2394 = (_e1511 * 1.1f);
                                                                    let _e2395 = (_e2394 * _e2387);
                                                                    let _e2414 = (_e1511 * _e2379);
                                                                    if (_e1527 == 4294967295u) {
                                                                        phi_3160_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2422 = (1f / _e2361);
                                                                        let _e2430 = ((((_e2353 * _e2422) * _e1483) + ((_e2354 * _e2422) * _e1486)) + ((_e2355 * _e2422) * _e1489));
                                                                        if (_e2363 != _e2363) {
                                                                            phi_23630_ = true;
                                                                        } else {
                                                                            phi_23630_ = (0f >= _e2363);
                                                                        }
                                                                        let _e2434 = phi_23630_;
                                                                        let _e2438 = sqrt((select(_e2363, 0f, _e2434) / (_e1474 - _e1471)));
                                                                        if (_e2438 != _e2438) {
                                                                            phi_23645_ = true;
                                                                        } else {
                                                                            phi_23645_ = (1f <= _e2438);
                                                                        }
                                                                        let _e2442 = phi_23645_;
                                                                        let _e2444 = (select(_e2438, 1f, _e2442) * 31f);
                                                                        let _e2446 = select(_e2430, -1f, (_e2430 < -1f));
                                                                        let _e2450 = ((select(_e2446, 1f, (_e2446 > 1f)) + 1f) * 15.5f);
                                                                        let _e2455 = select(select(u32(_e2444), 0u, (_e2444 < 0f)), 4294967295u, (_e2444 > 4294967000f));
                                                                        let _e2457 = select(_e2455, 30u, (30u < _e2455));
                                                                        let _e2462 = select(select(u32(_e2450), 0u, (_e2450 < 0f)), 4294967295u, (_e2450 > 4294967000f));
                                                                        let _e2464 = select(_e2462, 30u, (30u < _e2462));
                                                                        let _e2466 = (_e2444 - f32(_e2457));
                                                                        let _e2468 = (_e2450 - f32(_e2464));
                                                                        let _e2472 = (_e1527 + (3u * ((_e2457 * 32u) + _e2464)));
                                                                        let _e2477 = (_e1527 + (3u * (((_e2457 + 1u) * 32u) + _e2464)));
                                                                        let _e2479 = global_6.member[_e2472];
                                                                        let _e2482 = global_6.member[(_e2472 + 1u)];
                                                                        let _e2485 = global_6.member[(_e2472 + 2u)];
                                                                        let _e2486 = (1f - _e2468);
                                                                        let _e2492 = global_6.member[(_e2472 + 3u)];
                                                                        let _e2495 = global_6.member[(_e2472 + 4u)];
                                                                        let _e2498 = global_6.member[(_e2472 + 5u)];
                                                                        let _e2505 = (1f - _e2466);
                                                                        let _e2510 = global_6.member[_e2477];
                                                                        let _e2513 = global_6.member[(_e2477 + 1u)];
                                                                        let _e2516 = global_6.member[(_e2477 + 2u)];
                                                                        let _e2522 = global_6.member[(_e2477 + 3u)];
                                                                        let _e2525 = global_6.member[(_e2477 + 4u)];
                                                                        let _e2528 = global_6.member[(_e2477 + 5u)];
                                                                        phi_3160_ = vec3<f32>(((((_e2479 * _e2486) + (_e2492 * _e2468)) * _e2505) + (((_e2510 * _e2486) + (_e2522 * _e2468)) * _e2466)), ((((_e2482 * _e2486) + (_e2495 * _e2468)) * _e2505) + (((_e2513 * _e2486) + (_e2525 * _e2468)) * _e2466)), ((((_e2485 * _e2486) + (_e2498 * _e2468)) * _e2505) + (((_e2516 * _e2486) + (_e2528 * _e2468)) * _e2466)));
                                                                    }
                                                                    let _e2543 = phi_3160_;
                                                                    let _e2561 = (((_e2353 * _e1483) + (_e2354 * _e1486)) + (_e2355 * _e1489));
                                                                    let _e2562 = (_e2561 * _e2561);
                                                                    let _e2563 = (_e2562 - _e2352);
                                                                    if (_e2563 < 0f) {
                                                                        phi_23795_ = type_57(0u, type_57().member_1);
                                                                        phi_23796_ = type_57();
                                                                        phi_23797_ = true;
                                                                    } else {
                                                                        if (_e2561 < 0f) {
                                                                            phi_23767_ = -(sqrt(_e2563));
                                                                        } else {
                                                                            phi_23767_ = sqrt(_e2563);
                                                                        }
                                                                        let _e2570 = phi_23767_;
                                                                        let _e2572 = -((_e2561 + _e2570));
                                                                        let _e2573 = (_e2572 == 0f);
                                                                        if _e2573 {
                                                                            phi_23790_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23791_ = type_57();
                                                                        } else {
                                                                            let _e2574 = (_e2352 / _e2572);
                                                                            if (_e2572 < _e2574) {
                                                                                phi_23779_ = type_56(_e2572, _e2574);
                                                                            } else {
                                                                                phi_23779_ = type_56(_e2574, _e2572);
                                                                            }
                                                                            let _e2579 = phi_23779_;
                                                                            phi_23790_ = type_57();
                                                                            phi_23791_ = type_57(1u, _e2579);
                                                                        }
                                                                        let _e2582 = phi_23790_;
                                                                        let _e2584 = phi_23791_;
                                                                        phi_23795_ = _e2582;
                                                                        phi_23796_ = _e2584;
                                                                        phi_23797_ = _e2573;
                                                                    }
                                                                    let _e2589 = phi_23795_;
                                                                    let _e2591 = phi_23796_;
                                                                    let _e2593 = phi_23797_;
                                                                    if _e2593 {
                                                                        phi_23801_ = _e2589;
                                                                    } else {
                                                                        phi_23801_ = _e2591;
                                                                    }
                                                                    let _e2595 = phi_23801_;
                                                                    if (bitcast<i32>(_e2595.member) != 0i) {
                                                                        phi_23735_ = select(_e2595.member_1.member_1, _e2595.member_1.member, (_e2595.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23735_ = -1f;
                                                                    }
                                                                    let _e2606 = phi_23735_;
                                                                    if (_e2606 >= 0f) {
                                                                        phi_23718_ = type_58(0u, type_58().member_1);
                                                                    } else {
                                                                        let _e2608 = (_e2351 + _e1480);
                                                                        let _e2609 = (_e2562 - _e2608);
                                                                        if (_e2609 < 0f) {
                                                                            phi_23905_ = type_57(0u, type_57().member_1);
                                                                            phi_23906_ = type_57();
                                                                            phi_23907_ = true;
                                                                        } else {
                                                                            if (_e2561 < 0f) {
                                                                                phi_23877_ = -(sqrt(_e2609));
                                                                            } else {
                                                                                phi_23877_ = sqrt(_e2609);
                                                                            }
                                                                            let _e2616 = phi_23877_;
                                                                            let _e2618 = -((_e2561 + _e2616));
                                                                            let _e2619 = (_e2618 == 0f);
                                                                            if _e2619 {
                                                                                phi_23900_ = type_57(1u, type_56(0f, 0f));
                                                                                phi_23901_ = type_57();
                                                                            } else {
                                                                                let _e2620 = (_e2608 / _e2618);
                                                                                if (_e2618 < _e2620) {
                                                                                    phi_23889_ = type_56(_e2618, _e2620);
                                                                                } else {
                                                                                    phi_23889_ = type_56(_e2620, _e2618);
                                                                                }
                                                                                let _e2625 = phi_23889_;
                                                                                phi_23900_ = type_57();
                                                                                phi_23901_ = type_57(1u, _e2625);
                                                                            }
                                                                            let _e2628 = phi_23900_;
                                                                            let _e2630 = phi_23901_;
                                                                            phi_23905_ = _e2628;
                                                                            phi_23906_ = _e2630;
                                                                            phi_23907_ = _e2619;
                                                                        }
                                                                        let _e2635 = phi_23905_;
                                                                        let _e2637 = phi_23906_;
                                                                        let _e2639 = phi_23907_;
                                                                        if _e2639 {
                                                                            phi_23911_ = _e2635;
                                                                        } else {
                                                                            phi_23911_ = _e2637;
                                                                        }
                                                                        let _e2641 = phi_23911_;
                                                                        if (bitcast<i32>(_e2641.member) != 0i) {
                                                                            phi_23845_ = select(_e2641.member_1.member_1, _e2641.member_1.member, (_e2641.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_23845_ = -1f;
                                                                        }
                                                                        let _e2652 = phi_23845_;
                                                                        if (_e2652 < 0f) {
                                                                            phi_23714_ = type_58(0u, type_58().member_1);
                                                                        } else {
                                                                            phi_23672_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_23673_ = 0u;
                                                                            loop {
                                                                                let _e2655 = phi_23672_;
                                                                                let _e2657 = phi_23673_;
                                                                                local_11 = _e2655;
                                                                                let _e2658 = (_e2657 < 16u);
                                                                                if _e2658 {
                                                                                    let _e2660 = (f32(_e2657) * 0.0625f);
                                                                                    let _e2661 = (_e2657 + 1u);
                                                                                    let _e2663 = (f32(_e2661) * 0.0625f);
                                                                                    let _e2665 = ((_e2660 + _e2663) * 0.5f);
                                                                                    let _e2667 = ((_e2652 * _e2665) * _e2665);
                                                                                    let _e2671 = (_e2652 * ((_e2663 * _e2663) - (_e2660 * _e2660)));
                                                                                    let _e2676 = (vec3<f32>(_e2336, _e2337, _e2338) + vec3<f32>((_e1483 * _e2667), (_e1486 * _e2667), (_e1489 * _e2667)));
                                                                                    let _e2693 = (_e2676.x - _e1462);
                                                                                    let _e2694 = (_e2676.y - _e1465);
                                                                                    let _e2695 = (_e2676.z - _e1468);
                                                                                    let _e2703 = ((((((_e2676.x * _e2676.x) + (_e2676.y * _e2676.y)) + (_e2676.z * _e2676.z)) - (2f * (((_e2676.x * _e1462) + (_e2676.y * _e1465)) + (_e2676.z * _e1468)))) + _e1477) / (sqrt((((_e2693 * _e2693) + (_e2694 * _e2694)) + (_e2695 * _e2695))) + _e1471));
                                                                                    let _e2707 = (1f - (abs((_e2703 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2707 != _e2707) {
                                                                                        phi_24043_ = true;
                                                                                    } else {
                                                                                        phi_24043_ = (0f >= _e2707);
                                                                                    }
                                                                                    let _e2711 = phi_24043_;
                                                                                    let _e2713 = -(_e2703);
                                                                                    phi_23706_ = (_e2655 + vec3<f32>((exp((_e2713 / _e1492)) * _e2671), (exp((_e2713 / _e1495)) * _e2671), (select(_e2707, 0f, _e2711) * _e2671)));
                                                                                    phi_23707_ = _e2661;
                                                                                } else {
                                                                                    phi_23706_ = vec3<f32>();
                                                                                    phi_23707_ = u32();
                                                                                }
                                                                                let _e2724 = phi_23706_;
                                                                                let _e2726 = phi_23707_;
                                                                                phi_23672_ = _e2724;
                                                                                phi_23673_ = _e2726;
                                                                                if !(_e2658) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2729 = local_11;
                                                                            phi_23714_ = type_58(1u, _e2729);
                                                                        }
                                                                        let _e2735 = phi_23714_;
                                                                        phi_23718_ = _e2735;
                                                                    }
                                                                    let _e2740 = phi_23718_;
                                                                    if (bitcast<i32>(_e2740.member) != 0i) {
                                                                        let _e2745 = (vec3<f32>(_e2385, _e2387, _e2389) + _e2740.member_1);
                                                                        let _e2751 = (_e2394 * _e2745.y);
                                                                        let _e2763 = exp(-((((_e1501 * _e2745.x) + _e2751) + (_e1517 * _e2745.z))));
                                                                        let _e2765 = exp(-((((_e1504 * _e2745.x) + _e2751) + (_e1520 * _e2745.z))));
                                                                        let _e2767 = exp(-((((_e1507 * _e2745.x) + _e2751) + (_e1523 * _e2745.z))));
                                                                        phi_3219_ = vec3<f32>((_e2225.x + (_e2763 * _e2379)), (_e2225.y + (_e2765 * _e2379)), (_e2225.z + (_e2767 * _e2379)));
                                                                        phi_3220_ = vec3<f32>((_e2227.x + (_e2763 * _e2376)), (_e2227.y + (_e2765 * _e2376)), (_e2227.z + (_e2767 * _e2376)));
                                                                    } else {
                                                                        phi_3219_ = _e2225;
                                                                        phi_3220_ = _e2227;
                                                                    }
                                                                    let _e2789 = phi_3219_;
                                                                    let _e2791 = phi_3220_;
                                                                    phi_2723_ = vec3<f32>((_e2385 + _e2381), (_e2387 + _e2382), (_e2389 + _e2383));
                                                                    phi_2726_ = vec3<f32>((_e2223.x + ((exp(-((((_e1501 * _e2385) + _e2395) + (_e1517 * _e2389)))) * ((_e1501 * _e2376) + _e2414)) * _e2543.x)), (_e2223.y + ((exp(-((((_e1504 * _e2385) + _e2395) + (_e1520 * _e2389)))) * ((_e1504 * _e2376) + _e2414)) * _e2543.y)), (_e2223.z + ((exp(-((((_e1507 * _e2385) + _e2395) + (_e1523 * _e2389)))) * ((_e1507 * _e2376) + _e2414)) * _e2543.z)));
                                                                    phi_2728_ = _e2789;
                                                                    phi_2730_ = _e2791;
                                                                    phi_2732_ = (_e2229 + 1u);
                                                                } else {
                                                                    phi_2723_ = vec3<f32>();
                                                                    phi_2726_ = vec3<f32>();
                                                                    phi_2728_ = vec3<f32>();
                                                                    phi_2730_ = vec3<f32>();
                                                                    phi_2732_ = u32();
                                                                }
                                                                let _e2798 = phi_2723_;
                                                                let _e2800 = phi_2726_;
                                                                let _e2802 = phi_2728_;
                                                                let _e2804 = phi_2730_;
                                                                let _e2806 = phi_2732_;
                                                                phi_2722_ = _e2798;
                                                                phi_2725_ = _e2800;
                                                                phi_2727_ = _e2802;
                                                                phi_2729_ = _e2804;
                                                                phi_2731_ = _e2806;
                                                                phi_33789_ = _e1452;
                                                                if !(_e2230) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2809 = phi_33789_;
                                                            phi_33879_ = _e2809;
                                                            if _e2809 {
                                                                break;
                                                            }
                                                            let _e2811 = local_12;
                                                            let _e2814 = local_13;
                                                            let _e2817 = local_14;
                                                            phi_33883_ = _e2809;
                                                            phi_3240_ = type_52((_e2811 * vec3<f32>(_e1501, _e1504, _e1507)), (_e2814 * _e1511), _e2817);
                                                        } else {
                                                            phi_33883_ = _e1452;
                                                            phi_3240_ = type_52(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2820 = phi_33883_;
                                                        let _e2822 = phi_3240_;
                                                        phi_33882_ = _e2820;
                                                        phi_3241_ = _e2822;
                                                    } else {
                                                        phi_33882_ = _e1452;
                                                        phi_3241_ = _e2055;
                                                    }
                                                    let _e2824 = phi_33882_;
                                                    let _e2826 = phi_3241_;
                                                    let _e2834 = (((_e343.member_1.x * _e1483) + (_e343.member_1.y * _e1486)) + (_e343.member_1.z * _e1489));
                                                    let _e2836 = (1f + (_e2834 * _e2834));
                                                    let _e2837 = (0.059683103f * _e2836);
                                                    let _e2844 = (_e1514 * _e1514);
                                                    let _e2855 = ((0.119366206f * ((1f - _e2844) * _e2836)) / ((2f + _e2844) * pow(((1f + _e2844) - ((2f * _e1514) * _e2834)), 1.5f)));
                                                    phi_33881_ = _e2824;
                                                    phi_3264_ = vec3<f32>((_e1454.x + (((((_e2826.member.x * _e2837) + (_e2826.member_1.x * _e2855)) + _e2826.member_2.x) * _e1498) * 0.38317f)), (_e1454.y + (((((_e2826.member.y * _e2837) + (_e2826.member_1.y * _e2855)) + _e2826.member_2.y) * _e1498) * 0.38317f)), (_e1454.z + (((((_e2826.member.z * _e2837) + (_e2826.member_1.z * _e2855)) + _e2826.member_2.z) * _e1498) * 0.38317f)));
                                                    phi_3265_ = (_e1456 + 1i);
                                                    phi_3266_ = bitcast<u32>(_e1534);
                                                } else {
                                                    phi_33881_ = _e1452;
                                                    phi_3264_ = vec3<f32>();
                                                    phi_3265_ = i32();
                                                    phi_3266_ = u32();
                                                }
                                                let _e2886 = phi_33881_;
                                                let _e2888 = phi_3264_;
                                                let _e2890 = phi_3265_;
                                                let _e2892 = phi_3266_;
                                                phi_33880_ = _e2886;
                                                phi_910_ = _e2888;
                                                phi_913_ = _e2890;
                                                phi_915_ = _e2892;
                                                phi_3267_ = _e1460;
                                            } else {
                                                phi_33880_ = _e1452;
                                                phi_910_ = vec3<f32>();
                                                phi_913_ = i32();
                                                phi_915_ = u32();
                                                phi_3267_ = false;
                                            }
                                            let _e2894 = phi_33880_;
                                            let _e2896 = phi_910_;
                                            let _e2898 = phi_913_;
                                            let _e2900 = phi_915_;
                                            let _e2902 = phi_3267_;
                                            phi_33821_ = _e2894;
                                            phi_909_ = _e2896;
                                            phi_912_ = _e2898;
                                            phi_914_ = _e2900;
                                            phi_33879_ = _e2894;
                                            if !(_e2902) {
                                                break;
                                            }
                                        }
                                        let _e2905 = phi_33879_;
                                        phi_34207_ = _e2905;
                                        if _e2905 {
                                            break;
                                        }
                                        if _e1449 {
                                            let _e2908 = global_6.member[(_e1301 + 10u)];
                                            let _e2911 = global_6.member[(_e1301 + 21u)];
                                            let _e2912 = bitcast<u32>(_e2911);
                                            let _e2915 = global_6.member[(_e1301 + 22u)];
                                            let _e2916 = bitcast<u32>(_e2915);
                                            let _e2919 = global_6.member[(_e1301 + 23u)];
                                            let _e2923 = global_6.member[(_e1301 + 24u)];
                                            let _e2927 = global_6.member[(_e1301 + 25u)];
                                            let _e2931 = global_6.member[(_e1301 + 26u)];
                                            let _e2934 = global_6.member[_e1301];
                                            let _e2937 = global_6.member[(_e1301 + 1u)];
                                            let _e2940 = global_6.member[(_e1301 + 2u)];
                                            let _e2943 = global_6.member[(_e1301 + 3u)];
                                            let _e2946 = global_6.member[(_e1301 + 4u)];
                                            let _e2949 = global_6.member[(_e1301 + 5u)];
                                            let _e2952 = global_6.member[(_e1301 + 6u)];
                                            let _e2955 = global_6.member[(_e1301 + 7u)];
                                            let _e2958 = global_6.member[(_e1301 + 8u)];
                                            let _e2963 = (((_e343.member_1.x * _e2943) + (_e343.member_1.y * _e2946)) + (_e343.member_1.z * _e2949));
                                            let _e2965 = select(_e2963, -1f, (_e2963 < -1f));
                                            let _e2968 = asin(select(_e2965, 1f, (_e2965 > 1f)));
                                            let _e2979 = atan2((((_e343.member_1.x * _e2952) + (_e343.member_1.y * _e2955)) + (_e343.member_1.z * _e2958)), (((_e343.member_1.x * _e2934) + (_e343.member_1.y * _e2937)) + (_e343.member_1.z * _e2940)));
                                            if (_e2912 != 4294967295u) {
                                                let _e2982 = global_6.member[_e2912];
                                                let _e2985 = global_6.member[(_e2912 + 1u)];
                                                let _e2986 = bitcast<u32>(_e2985);
                                                let _e2989 = global_6.member[(_e2912 + 4u)];
                                                phi_3601_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3604_ = 0u;
                                                loop {
                                                    let _e2995 = phi_3601_;
                                                    let _e2997 = phi_3604_;
                                                    local_15 = _e2995;
                                                    let _e2998 = (_e2997 < 6u);
                                                    if _e2998 {
                                                        let _e2999 = (_e2997 / 2u);
                                                        if (_e2999 == 0u) {
                                                            phi_24171_ = _e343.member_1.x;
                                                        } else {
                                                            phi_24171_ = select(_e343.member_1.z, _e343.member_1.y, (_e2999 == 1u));
                                                        }
                                                        let _e3007 = phi_24171_;
                                                        let _e3008 = (_e3007 * select(-1f, 1f, ((_e2997 % 2u) == 0u)));
                                                        if (_e3008 > 0f) {
                                                            let _e3012 = ((_e2999 + 1u) % 3u);
                                                            if (_e3012 == 0u) {
                                                                phi_24186_ = _e343.member_1.x;
                                                            } else {
                                                                phi_24186_ = select(_e343.member_1.z, _e343.member_1.y, (_e3012 == 1u));
                                                            }
                                                            let _e3017 = phi_24186_;
                                                            let _e3018 = (_e3017 / _e3008);
                                                            let _e3020 = ((_e2999 + 2u) % 3u);
                                                            if (_e3020 == 0u) {
                                                                phi_24201_ = _e343.member_1.x;
                                                            } else {
                                                                phi_24201_ = select(_e343.member_1.z, _e343.member_1.y, (_e3020 == 1u));
                                                            }
                                                            let _e3025 = phi_24201_;
                                                            let _e3026 = (_e3025 / _e3008);
                                                            let _e3028 = (1f + (_e2982 * 12f));
                                                            if (abs(_e3018) <= _e3028) {
                                                                if (abs(_e3026) <= _e3028) {
                                                                    let _e3034 = f32(_e2986);
                                                                    let _e3036 = floor((((_e3018 + 1f) * 0.5f) * _e3034));
                                                                    let _e3043 = select(0i, select(select(i32(_e3036), i32(-2147483648), (_e3036 < -2147483600f)), 2147483647i, (_e3036 > 2147483500f)), (_e3036 == _e3036));
                                                                    let _e3045 = (bitcast<i32>(_e2986) - 1i);
                                                                    let _e3046 = (0i <= _e3045);
                                                                    if _e3046 {
                                                                    } else {
                                                                        phi_33884_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3043 < 0i) {
                                                                        phi_3675_ = 0i;
                                                                    } else {
                                                                        phi_3675_ = select(_e3043, _e3045, (_e3043 > _e3045));
                                                                    }
                                                                    let _e3051 = phi_3675_;
                                                                    let _e3055 = floor((((_e3026 + 1f) * 0.5f) * _e3034));
                                                                    let _e3062 = select(0i, select(select(i32(_e3055), i32(-2147483648), (_e3055 < -2147483600f)), 2147483647i, (_e3055 > 2147483500f)), (_e3055 == _e3055));
                                                                    if _e3046 {
                                                                    } else {
                                                                        phi_33884_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3062 < 0i) {
                                                                        phi_3705_ = 0i;
                                                                    } else {
                                                                        phi_3705_ = select(_e3062, _e3045, (_e3062 > _e3045));
                                                                    }
                                                                    let _e3067 = phi_3705_;
                                                                    let _e3068 = (_e3051 - 1i);
                                                                    phi_3713_ = _e2995;
                                                                    phi_3716_ = select(0i, _e3068, (0i < _e3068));
                                                                    loop {
                                                                        let _e3072 = phi_3713_;
                                                                        let _e3074 = phi_3716_;
                                                                        let _e3075 = (_e3051 + 1i);
                                                                        local_59 = _e3072;
                                                                        let _e3078 = (_e3074 <= select(_e3075, _e3045, (_e3045 < _e3075)));
                                                                        if _e3078 {
                                                                            let _e3079 = (_e3067 - 1i);
                                                                            phi_3739_ = _e3072;
                                                                            phi_3742_ = select(0i, _e3079, (0i < _e3079));
                                                                            loop {
                                                                                let _e3083 = phi_3739_;
                                                                                let _e3085 = phi_3742_;
                                                                                let _e3086 = (_e3067 + 1i);
                                                                                local_58 = _e3083;
                                                                                let _e3089 = (_e3085 <= select(_e3086, _e3045, (_e3045 < _e3086)));
                                                                                if _e3089 {
                                                                                    let _e3098 = global_6.member[(bitcast<u32>(_e2989) + ((((_e2997 * _e2986) + bitcast<u32>(_e3074)) * _e2986) + bitcast<u32>(_e3085)))];
                                                                                    phi_3740_ = vec3<f32>((_e3083.x + _e3098), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3743_ = (_e3085 + 1i);
                                                                                } else {
                                                                                    phi_3740_ = vec3<f32>();
                                                                                    phi_3743_ = i32();
                                                                                }
                                                                                let _e3107 = phi_3740_;
                                                                                let _e3109 = phi_3743_;
                                                                                phi_3739_ = _e3107;
                                                                                phi_3742_ = _e3109;
                                                                                if !(_e3089) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6944 = local_58;
                                                                            phi_3714_ = _e6944;
                                                                            phi_3717_ = (_e3074 + 1i);
                                                                        } else {
                                                                            phi_3714_ = vec3<f32>();
                                                                            phi_3717_ = i32();
                                                                        }
                                                                        let _e3113 = phi_3714_;
                                                                        let _e3115 = phi_3717_;
                                                                        phi_3713_ = _e3113;
                                                                        phi_3716_ = _e3115;
                                                                        if !(_e3078) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6948 = local_59;
                                                                    phi_3941_ = _e6948;
                                                                } else {
                                                                    phi_3941_ = _e2995;
                                                                }
                                                                let _e3118 = phi_3941_;
                                                                phi_3942_ = _e3118;
                                                            } else {
                                                                phi_3942_ = _e2995;
                                                            }
                                                            let _e3120 = phi_3942_;
                                                            phi_3943_ = _e3120;
                                                        } else {
                                                            phi_3943_ = _e2995;
                                                        }
                                                        let _e3122 = phi_3943_;
                                                        phi_3602_ = _e3122;
                                                        phi_3605_ = (_e2997 + 1u);
                                                    } else {
                                                        phi_3602_ = vec3<f32>();
                                                        phi_3605_ = u32();
                                                    }
                                                    let _e3125 = phi_3602_;
                                                    let _e3127 = phi_3605_;
                                                    phi_3601_ = _e3125;
                                                    phi_3604_ = _e3127;
                                                    phi_33884_ = _e2905;
                                                    if !(_e2998) {
                                                        break;
                                                    }
                                                }
                                                let _e3130 = phi_33884_;
                                                phi_34207_ = _e3130;
                                                if _e3130 {
                                                    break;
                                                }
                                                let _e3132 = local_15;
                                                phi_33932_ = _e3130;
                                                phi_3954_ = (_e3132.x * (1f / ((6.2831855f * _e2982) * _e2982)));
                                            } else {
                                                phi_33932_ = _e2905;
                                                phi_3954_ = 0f;
                                            }
                                            let _e3136 = phi_33932_;
                                            let _e3138 = phi_3954_;
                                            if (_e2916 != 4294967295u) {
                                                let _e3155 = (_e2968 * 1.3333334f);
                                                let _e3162 = global_6.member[_e2916];
                                                phi_3978_ = (_e3138 + ((_e3162 * ((exp((-(abs(_e2968)) / (0.06f + (abs(_e2979) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2979 * -0.71428573f) * (_e2979 * 0.71428573f)))))) + (1.3f * exp((((_e2979 * _e2979) + (_e3155 * _e3155)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3978_ = _e3138;
                                            }
                                            let _e3168 = phi_3978_;
                                            phi_3980_ = 0u;
                                            phi_3983_ = _e3168;
                                            loop {
                                                let _e3170 = phi_3980_;
                                                let _e3172 = phi_3983_;
                                                local_60 = _e3172;
                                                let _e3173 = (_e3170 < bitcast<u32>(_e2923));
                                                if _e3173 {
                                                    let _e3175 = (bitcast<u32>(_e2919) + (_e3170 * 18u));
                                                    let _e3178 = global_6.member[(_e3175 + 1u)];
                                                    let _e3181 = global_6.member[(_e3175 + 2u)];
                                                    let _e3184 = global_6.member[(_e3175 + 3u)];
                                                    let _e3192 = global_6.member[(_e3175 + 5u)];
                                                    if ((((_e343.member_1.x * _e3178) + (_e343.member_1.y * _e3181)) + (_e343.member_1.z * _e3184)) >= _e3192) {
                                                        let _e3195 = global_6.member[_e3175];
                                                        let _e3200 = select(select(u32(_e3195), 0u, (_e3195 < 0f)), 4294967295u, (_e3195 > 4294967000f));
                                                        if (_e3200 == 0u) {
                                                            phi_4087_ = 1.8f;
                                                        } else {
                                                            if (_e3200 == 1u) {
                                                                phi_4086_ = 1f;
                                                            } else {
                                                                phi_4086_ = select(0f, 4.7f, (_e3200 == 2u));
                                                            }
                                                            let _e3206 = phi_4086_;
                                                            phi_4087_ = _e3206;
                                                        }
                                                        let _e3208 = phi_4087_;
                                                        let _e3211 = global_6.member[(_e3175 + 6u)];
                                                        phi_4103_ = (_e3172 + (_e3211 * _e3208));
                                                    } else {
                                                        phi_4103_ = _e3172;
                                                    }
                                                    let _e3215 = phi_4103_;
                                                    phi_3981_ = (_e3170 + 1u);
                                                    phi_3984_ = _e3215;
                                                } else {
                                                    phi_3981_ = u32();
                                                    phi_3984_ = f32();
                                                }
                                                let _e3218 = phi_3981_;
                                                let _e3220 = phi_3984_;
                                                phi_3980_ = _e3218;
                                                phi_3983_ = _e3220;
                                                if !(_e3173) {
                                                    break;
                                                }
                                            }
                                            phi_4106_ = 0u;
                                            let _e6968 = local_60;
                                            phi_4109_ = _e6968;
                                            loop {
                                                let _e3223 = phi_4106_;
                                                let _e3225 = phi_4109_;
                                                local_16 = _e3225;
                                                let _e3226 = (_e3223 < bitcast<u32>(_e2931));
                                                if _e3226 {
                                                    let _e3228 = (bitcast<u32>(_e2927) + (_e3223 * 29u));
                                                    let _e3231 = global_6.member[(_e3228 + 1u)];
                                                    let _e3234 = global_6.member[(_e3228 + 2u)];
                                                    let _e3237 = global_6.member[(_e3228 + 3u)];
                                                    let _e3245 = global_6.member[(_e3228 + 5u)];
                                                    if ((((_e343.member_1.x * _e3231) + (_e343.member_1.y * _e3234)) + (_e343.member_1.z * _e3237)) >= _e3245) {
                                                        let _e3248 = global_6.member[_e3228];
                                                        let _e3252 = global_6.member[(_e3228 + 6u)];
                                                        phi_4217_ = (_e3225 + (_e3252 * select(1f, 3.3808f, (_e3248 < 0.5f))));
                                                    } else {
                                                        phi_4217_ = _e3225;
                                                    }
                                                    let _e3257 = phi_4217_;
                                                    phi_4107_ = (_e3223 + 1u);
                                                    phi_4110_ = _e3257;
                                                } else {
                                                    phi_4107_ = u32();
                                                    phi_4110_ = f32();
                                                }
                                                let _e3260 = phi_4107_;
                                                let _e3262 = phi_4110_;
                                                phi_4106_ = _e3260;
                                                phi_4109_ = _e3262;
                                                if !(_e3226) {
                                                    break;
                                                }
                                            }
                                            let _e3266 = global_6.member[(_e1301 + 9u)];
                                            let _e3268 = local_16;
                                            let _e3271 = local_17;
                                            let _e3275 = local_18;
                                            let _e3280 = local_19;
                                            let _e3285 = ((_e3268 * _e3266) <= (_e2908 * (((0.2126f * _e3271.x) + (0.7152f * _e3275.y)) + (0.0722f * _e3280.z))));
                                            if _e3285 {
                                                phi_33973_ = _e3136;
                                                phi_4776_ = vec3<f32>();
                                                phi_4777_ = bool();
                                            } else {
                                                let _e3287 = global_6.member[_e1298];
                                                let _e3290 = global_6.member[(_e1298 + 1u)];
                                                let _e3293 = global_6.member[(_e1298 + 2u)];
                                                let _e3296 = global_6.member[(_e1298 + 3u)];
                                                let _e3299 = global_6.member[(_e1298 + 5u)];
                                                let _e3302 = global_6.member[(_e1298 + 6u)];
                                                let _e3305 = global_6.member[(_e1298 + 10u)];
                                                let _e3308 = global_6.member[(_e1298 + 11u)];
                                                let _e3311 = global_6.member[(_e1298 + 13u)];
                                                let _e3314 = global_6.member[(_e1298 + 14u)];
                                                let _e3317 = global_6.member[(_e1298 + 15u)];
                                                let _e3320 = global_6.member[(_e1298 + 16u)];
                                                let _e3323 = global_6.member[(_e1298 + 18u)];
                                                let _e3326 = global_6.member[(_e1298 + 19u)];
                                                let _e3329 = global_6.member[(_e1298 + 20u)];
                                                let _e3341 = ((((_e343.member.x * _e343.member.x) + (_e343.member.y * _e343.member.y)) + (_e343.member.z * _e343.member.z)) - (2f * (((_e343.member.x * _e3287) + (_e343.member.y * _e3290)) + (_e343.member.z * _e3293))));
                                                let _e3342 = (_e3341 + _e3299);
                                                let _e3343 = (_e3342 > 0f);
                                                if _e3343 {
                                                    let _e3351 = ((((_e343.member.x - _e3287) * _e343.member_1.x) + ((_e343.member.y - _e3290) * _e343.member_1.y)) + ((_e343.member.z - _e3293) * _e343.member_1.z));
                                                    let _e3353 = ((_e3351 * _e3351) - _e3342);
                                                    if (_e3353 < 0f) {
                                                        phi_24345_ = type_57(0u, type_57().member_1);
                                                        phi_24346_ = type_57();
                                                        phi_24347_ = true;
                                                    } else {
                                                        if (_e3351 < 0f) {
                                                            phi_24317_ = -(sqrt(_e3353));
                                                        } else {
                                                            phi_24317_ = sqrt(_e3353);
                                                        }
                                                        let _e3360 = phi_24317_;
                                                        let _e3362 = -((_e3351 + _e3360));
                                                        let _e3363 = (_e3362 == 0f);
                                                        if _e3363 {
                                                            phi_24340_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24341_ = type_57();
                                                        } else {
                                                            let _e3364 = (_e3342 / _e3362);
                                                            if (_e3362 < _e3364) {
                                                                phi_24329_ = type_56(_e3362, _e3364);
                                                            } else {
                                                                phi_24329_ = type_56(_e3364, _e3362);
                                                            }
                                                            let _e3369 = phi_24329_;
                                                            phi_24340_ = type_57();
                                                            phi_24341_ = type_57(1u, _e3369);
                                                        }
                                                        let _e3372 = phi_24340_;
                                                        let _e3374 = phi_24341_;
                                                        phi_24345_ = _e3372;
                                                        phi_24346_ = _e3374;
                                                        phi_24347_ = _e3363;
                                                    }
                                                    let _e3379 = phi_24345_;
                                                    let _e3381 = phi_24346_;
                                                    let _e3383 = phi_24347_;
                                                    if _e3383 {
                                                        phi_24351_ = _e3379;
                                                    } else {
                                                        phi_24351_ = _e3381;
                                                    }
                                                    let _e3385 = phi_24351_;
                                                    if (bitcast<i32>(_e3385.member) != 0i) {
                                                        phi_24285_ = select(_e3385.member_1.member_1, _e3385.member_1.member, (_e3385.member_1.member >= 0f));
                                                    } else {
                                                        phi_24285_ = -1f;
                                                    }
                                                    let _e3396 = phi_24285_;
                                                    phi_4618_ = select(true, false, (_e3396 >= 0f));
                                                } else {
                                                    phi_4618_ = true;
                                                }
                                                let _e3400 = phi_4618_;
                                                if _e3400 {
                                                    let _e3408 = ((((_e343.member.x - _e3287) * _e343.member_1.x) + ((_e343.member.y - _e3290) * _e343.member_1.y)) + ((_e343.member.z - _e3293) * _e343.member_1.z));
                                                    let _e3409 = (_e3341 + _e3302);
                                                    let _e3410 = (_e3408 * _e3408);
                                                    let _e3411 = (_e3410 - _e3409);
                                                    if (_e3411 < 0f) {
                                                        phi_24505_ = type_57(0u, type_57().member_1);
                                                        phi_24506_ = type_57();
                                                        phi_24507_ = true;
                                                    } else {
                                                        if (_e3408 < 0f) {
                                                            phi_24477_ = -(sqrt(_e3411));
                                                        } else {
                                                            phi_24477_ = sqrt(_e3411);
                                                        }
                                                        let _e3418 = phi_24477_;
                                                        let _e3420 = -((_e3408 + _e3418));
                                                        let _e3421 = (_e3420 == 0f);
                                                        if _e3421 {
                                                            phi_24500_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24501_ = type_57();
                                                        } else {
                                                            let _e3422 = (_e3409 / _e3420);
                                                            if (_e3420 < _e3422) {
                                                                phi_24489_ = type_56(_e3420, _e3422);
                                                            } else {
                                                                phi_24489_ = type_56(_e3422, _e3420);
                                                            }
                                                            let _e3427 = phi_24489_;
                                                            phi_24500_ = type_57();
                                                            phi_24501_ = type_57(1u, _e3427);
                                                        }
                                                        let _e3430 = phi_24500_;
                                                        let _e3432 = phi_24501_;
                                                        phi_24505_ = _e3430;
                                                        phi_24506_ = _e3432;
                                                        phi_24507_ = _e3421;
                                                    }
                                                    let _e3437 = phi_24505_;
                                                    let _e3439 = phi_24506_;
                                                    let _e3441 = phi_24507_;
                                                    if _e3441 {
                                                        phi_24511_ = _e3437;
                                                    } else {
                                                        phi_24511_ = _e3439;
                                                    }
                                                    let _e3443 = phi_24511_;
                                                    if (bitcast<i32>(_e3443.member) != 0i) {
                                                        phi_24398_ = type_57(0u, _e3443.member_1);
                                                    } else {
                                                        phi_24398_ = type_57(1u, type_57().member_1);
                                                    }
                                                    let _e3453 = phi_24398_;
                                                    if (bitcast<i32>(_e3453.member) != 0i) {
                                                        phi_24445_ = type_57(0u, type_57().member_1);
                                                    } else {
                                                        if (_e3453.member_1.member_1 <= 0f) {
                                                            phi_24441_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e3453.member_1.member != _e3453.member_1.member) {
                                                                phi_24549_ = true;
                                                            } else {
                                                                phi_24549_ = (0f >= _e3453.member_1.member);
                                                            }
                                                            let _e3465 = phi_24549_;
                                                            let _e3466 = select(_e3453.member_1.member, 0f, _e3465);
                                                            if _e3343 {
                                                                let _e3467 = (_e3410 - _e3342);
                                                                if (_e3467 < 0f) {
                                                                    phi_24657_ = type_57(0u, type_57().member_1);
                                                                    phi_24658_ = type_57();
                                                                    phi_24659_ = true;
                                                                } else {
                                                                    if (_e3408 < 0f) {
                                                                        phi_24629_ = -(sqrt(_e3467));
                                                                    } else {
                                                                        phi_24629_ = sqrt(_e3467);
                                                                    }
                                                                    let _e3474 = phi_24629_;
                                                                    let _e3476 = -((_e3408 + _e3474));
                                                                    let _e3477 = (_e3476 == 0f);
                                                                    if _e3477 {
                                                                        phi_24652_ = type_57(1u, type_56(0f, 0f));
                                                                        phi_24653_ = type_57();
                                                                    } else {
                                                                        let _e3478 = (_e3342 / _e3476);
                                                                        if (_e3476 < _e3478) {
                                                                            phi_24641_ = type_56(_e3476, _e3478);
                                                                        } else {
                                                                            phi_24641_ = type_56(_e3478, _e3476);
                                                                        }
                                                                        let _e3483 = phi_24641_;
                                                                        phi_24652_ = type_57();
                                                                        phi_24653_ = type_57(1u, _e3483);
                                                                    }
                                                                    let _e3486 = phi_24652_;
                                                                    let _e3488 = phi_24653_;
                                                                    phi_24657_ = _e3486;
                                                                    phi_24658_ = _e3488;
                                                                    phi_24659_ = _e3477;
                                                                }
                                                                let _e3493 = phi_24657_;
                                                                let _e3495 = phi_24658_;
                                                                let _e3497 = phi_24659_;
                                                                if _e3497 {
                                                                    phi_24663_ = _e3493;
                                                                } else {
                                                                    phi_24663_ = _e3495;
                                                                }
                                                                let _e3499 = phi_24663_;
                                                                if (bitcast<i32>(_e3499.member) != 0i) {
                                                                    phi_24597_ = select(_e3499.member_1.member_1, _e3499.member_1.member, (_e3499.member_1.member >= 0f));
                                                                } else {
                                                                    phi_24597_ = -1f;
                                                                }
                                                                let _e3510 = phi_24597_;
                                                                if (_e3510 >= 0f) {
                                                                    if (_e3453.member_1.member_1 != _e3453.member_1.member_1) {
                                                                        phi_24701_ = true;
                                                                    } else {
                                                                        phi_24701_ = (_e3510 <= _e3453.member_1.member_1);
                                                                    }
                                                                    let _e3515 = phi_24701_;
                                                                    phi_24426_ = select(_e3453.member_1.member_1, _e3510, _e3515);
                                                                } else {
                                                                    phi_24426_ = _e3453.member_1.member_1;
                                                                }
                                                                let _e3518 = phi_24426_;
                                                                phi_24428_ = _e3518;
                                                            } else {
                                                                phi_24428_ = _e3453.member_1.member_1;
                                                            }
                                                            let _e3520 = phi_24428_;
                                                            if (_e3520 <= _e3466) {
                                                                phi_24437_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                phi_24437_ = type_57(1u, type_56(_e3466, _e3520));
                                                            }
                                                            let _e3528 = phi_24437_;
                                                            phi_24441_ = _e3528;
                                                        }
                                                        let _e3533 = phi_24441_;
                                                        phi_24445_ = _e3533;
                                                    }
                                                    let _e3538 = phi_24445_;
                                                    let _e3541 = (bitcast<i32>(_e3538.member) != 0i);
                                                    if _e3541 {
                                                        phi_4632_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4635_ = 0u;
                                                        loop {
                                                            let _e3547 = phi_4632_;
                                                            let _e3549 = phi_4635_;
                                                            local_20 = _e3547;
                                                            local_21 = _e3547;
                                                            local_22 = _e3547;
                                                            let _e3550 = (_e3549 < 32u);
                                                            if _e3550 {
                                                                let _e3551 = -(_e3408);
                                                                if (_e3538.member_1.member <= _e3538.member_1.member_1) {
                                                                } else {
                                                                    phi_33911_ = true;
                                                                    break;
                                                                }
                                                                let _e3554 = select(_e3551, _e3538.member_1.member, (_e3551 < _e3538.member_1.member));
                                                                let _e3556 = select(_e3554, _e3538.member_1.member_1, (_e3554 > _e3538.member_1.member_1));
                                                                let _e3557 = (_e3556 - _e3538.member_1.member);
                                                                let _e3558 = (_e3538.member_1.member_1 - _e3556);
                                                                let _e3559 = (_e3538.member_1.member_1 - _e3538.member_1.member);
                                                                let _e3560 = (0.000000001f * _e3559);
                                                                if (_e3557 <= _e3560) {
                                                                    let _e3635 = (f32(_e3549) * 0.03125f);
                                                                    let _e3638 = (f32((_e3549 + 1u)) * 0.03125f);
                                                                    let _e3640 = ((_e3635 + _e3638) * 0.5f);
                                                                    phi_4736_ = type_56((_e3538.member_1.member + ((_e3559 * _e3640) * _e3640)), (_e3559 * ((_e3638 * _e3638) - (_e3635 * _e3635))));
                                                                } else {
                                                                    if (_e3558 <= _e3560) {
                                                                        let _e3618 = (f32((31u - _e3549)) * 0.03125f);
                                                                        let _e3621 = (f32((32u - _e3549)) * 0.03125f);
                                                                        let _e3623 = ((_e3618 + _e3621) * 0.5f);
                                                                        phi_4735_ = type_56((_e3538.member_1.member_1 - ((_e3559 * _e3623) * _e3623)), (_e3559 * ((_e3621 * _e3621) - (_e3618 * _e3618))));
                                                                    } else {
                                                                        let _e3566 = round(((32f * _e3557) / (_e3557 + _e3558)));
                                                                        let _e3571 = select(select(u32(_e3566), 0u, (_e3566 < 0f)), 4294967295u, (_e3566 > 4294967000f));
                                                                        if (_e3571 < 1u) {
                                                                            phi_4719_ = 1u;
                                                                        } else {
                                                                            phi_4719_ = select(_e3571, 31u, (_e3571 > 31u));
                                                                        }
                                                                        let _e3576 = phi_4719_;
                                                                        if (_e3549 < _e3576) {
                                                                            let _e3597 = ((_e3576 - 1u) - _e3549);
                                                                            let _e3599 = f32(_e3576);
                                                                            let _e3600 = (f32(_e3597) / _e3599);
                                                                            let _e3603 = (f32((_e3597 + 1u)) / _e3599);
                                                                            let _e3605 = ((_e3600 + _e3603) * 0.5f);
                                                                            phi_4734_ = type_56((_e3556 - ((_e3557 * _e3605) * _e3605)), (_e3557 * ((_e3603 * _e3603) - (_e3600 * _e3600))));
                                                                        } else {
                                                                            let _e3579 = (_e3549 - _e3576);
                                                                            let _e3581 = f32((32u - _e3576));
                                                                            let _e3582 = (f32(_e3579) / _e3581);
                                                                            let _e3585 = (f32((_e3579 + 1u)) / _e3581);
                                                                            let _e3587 = ((_e3582 + _e3585) * 0.5f);
                                                                            phi_4734_ = type_56((_e3556 + ((_e3558 * _e3587) * _e3587)), (_e3558 * ((_e3585 * _e3585) - (_e3582 * _e3582))));
                                                                        }
                                                                        let _e3615 = phi_4734_;
                                                                        phi_4735_ = _e3615;
                                                                    }
                                                                    let _e3633 = phi_4735_;
                                                                    phi_4736_ = _e3633;
                                                                }
                                                                let _e3650 = phi_4736_;
                                                                let _e3656 = (_e343.member.x + (_e343.member_1.x * _e3650.member));
                                                                let _e3657 = (_e343.member.y + (_e343.member_1.y * _e3650.member));
                                                                let _e3658 = (_e343.member.z + (_e343.member_1.z * _e3650.member));
                                                                let _e3672 = (_e3656 - _e3287);
                                                                let _e3673 = (_e3657 - _e3290);
                                                                let _e3674 = (_e3658 - _e3293);
                                                                let _e3682 = ((((((_e3656 * _e3656) + (_e3657 * _e3657)) + (_e3658 * _e3658)) - (2f * (((_e3656 * _e3287) + (_e3657 * _e3290)) + (_e3658 * _e3293)))) + _e3299) / (sqrt((((_e3672 * _e3672) + (_e3673 * _e3673)) + (_e3674 * _e3674))) + _e3296));
                                                                let _e3686 = (1f - (abs((_e3682 - 25000f)) * 0.00006666667f));
                                                                if (_e3686 != _e3686) {
                                                                    phi_24900_ = true;
                                                                } else {
                                                                    phi_24900_ = (0f >= _e3686);
                                                                }
                                                                let _e3690 = phi_24900_;
                                                                let _e3692 = -(_e3682);
                                                                phi_4633_ = (_e3547 + vec3<f32>((exp((_e3692 / _e3305)) * _e3650.member_1), (exp((_e3692 / _e3308)) * _e3650.member_1), (select(_e3686, 0f, _e3690) * _e3650.member_1)));
                                                                phi_4636_ = (_e3549 + 1u);
                                                            } else {
                                                                phi_4633_ = vec3<f32>();
                                                                phi_4636_ = u32();
                                                            }
                                                            let _e3704 = phi_4633_;
                                                            let _e3706 = phi_4636_;
                                                            phi_4632_ = _e3704;
                                                            phi_4635_ = _e3706;
                                                            phi_33911_ = _e3136;
                                                            if !(_e3550) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3709 = phi_33911_;
                                                        phi_34207_ = _e3709;
                                                        if _e3709 {
                                                            break;
                                                        }
                                                        let _e3711 = local_20;
                                                        let _e3718 = local_21;
                                                        let _e3720 = ((_e3320 * 1.1f) * _e3718.y);
                                                        let _e3725 = local_22;
                                                        phi_33978_ = _e3709;
                                                        phi_4761_ = vec3<f32>(exp(-((((_e3311 * _e3711.x) + _e3720) + (_e3323 * _e3725.z)))), exp(-((((_e3314 * _e3711.x) + _e3720) + (_e3326 * _e3725.z)))), exp(-((((_e3317 * _e3711.x) + _e3720) + (_e3329 * _e3725.z)))));
                                                    } else {
                                                        phi_33978_ = _e3136;
                                                        phi_4761_ = vec3<f32>();
                                                    }
                                                    let _e3741 = phi_33978_;
                                                    let _e3743 = phi_4761_;
                                                    phi_33977_ = _e3741;
                                                    phi_4763_ = _e3743;
                                                    phi_4765_ = select(true, false, _e3541);
                                                } else {
                                                    phi_33977_ = _e3136;
                                                    phi_4763_ = vec3<f32>();
                                                    phi_4765_ = true;
                                                }
                                                let _e3746 = phi_33977_;
                                                let _e3748 = phi_4763_;
                                                let _e3750 = phi_4765_;
                                                let _e3754 = select(_e3748, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3400)), vec3(_e3750));
                                                let _e3758 = select(_e3754.y, _e3754.x, (_e3754.x > _e3754.y));
                                                phi_33973_ = _e3746;
                                                phi_4776_ = _e3754;
                                                phi_4777_ = (select(_e3754.z, _e3758, (_e3758 > _e3754.z)) > 0f);
                                            }
                                            let _e3764 = phi_33973_;
                                            let _e3766 = phi_4776_;
                                            let _e3768 = phi_4777_;
                                            phi_33972_ = _e3764;
                                            phi_4779_ = _e3766;
                                            phi_4780_ = _e3768;
                                            phi_4781_ = select(true, false, _e3285);
                                        } else {
                                            phi_33972_ = _e2905;
                                            phi_4779_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4780_ = _e1449;
                                            phi_4781_ = true;
                                        }
                                        let _e3771 = phi_33972_;
                                        let _e3773 = phi_4779_;
                                        let _e3775 = phi_4780_;
                                        let _e3777 = phi_4781_;
                                        phi_33971_ = _e3771;
                                        let _e7039 = local_61;
                                        phi_4785_ = _e7039;
                                        phi_4786_ = _e3773;
                                        phi_4787_ = _e3775;
                                        let _e7044 = local_62;
                                        phi_4788_ = _e7044;
                                        phi_4789_ = _e3777;
                                    } else {
                                        phi_33971_ = _e332;
                                        phi_4785_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4786_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4787_ = _e1449;
                                        phi_4788_ = vec3<f32>();
                                        phi_4789_ = true;
                                    }
                                    let _e3779 = phi_33971_;
                                    let _e3781 = phi_4785_;
                                    let _e3783 = phi_4786_;
                                    let _e3785 = phi_4787_;
                                    let _e3787 = phi_4788_;
                                    let _e3789 = phi_4789_;
                                    if _e3789 {
                                        if _e3785 {
                                            let _e3790 = (_e1301 + 20u);
                                            let _e3793 = global_6.member[(_e1301 + 21u)];
                                            let _e3794 = bitcast<u32>(_e3793);
                                            let _e3797 = global_6.member[(_e1301 + 22u)];
                                            let _e3798 = bitcast<u32>(_e3797);
                                            let _e3801 = global_6.member[(_e1301 + 23u)];
                                            let _e3805 = global_6.member[(_e1301 + 24u)];
                                            let _e3809 = global_6.member[(_e1301 + 25u)];
                                            let _e3813 = global_6.member[(_e1301 + 26u)];
                                            let _e3816 = global_6.member[_e1301];
                                            let _e3819 = global_6.member[(_e1301 + 1u)];
                                            let _e3822 = global_6.member[(_e1301 + 2u)];
                                            let _e3825 = global_6.member[(_e1301 + 3u)];
                                            let _e3828 = global_6.member[(_e1301 + 4u)];
                                            let _e3831 = global_6.member[(_e1301 + 5u)];
                                            let _e3834 = global_6.member[(_e1301 + 6u)];
                                            let _e3837 = global_6.member[(_e1301 + 7u)];
                                            let _e3840 = global_6.member[(_e1301 + 8u)];
                                            let _e3845 = (((_e343.member_1.x * _e3825) + (_e343.member_1.y * _e3828)) + (_e343.member_1.z * _e3831));
                                            let _e3847 = select(_e3845, -1f, (_e3845 < -1f));
                                            let _e3850 = asin(select(_e3847, 1f, (_e3847 > 1f)));
                                            let _e3861 = atan2((((_e343.member_1.x * _e3834) + (_e343.member_1.y * _e3837)) + (_e343.member_1.z * _e3840)), (((_e343.member_1.x * _e3816) + (_e343.member_1.y * _e3819)) + (_e343.member_1.z * _e3822)));
                                            if (_e3794 != 4294967295u) {
                                                let _e3864 = global_6.member[_e3794];
                                                let _e3867 = global_6.member[(_e3794 + 1u)];
                                                let _e3868 = bitcast<u32>(_e3867);
                                                let _e3871 = global_6.member[(_e3794 + 2u)];
                                                let _e3875 = global_6.member[(_e3794 + 3u)];
                                                let _e3877 = (4f * _e3864);
                                                phi_5109_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5112_ = 0u;
                                                loop {
                                                    let _e3882 = phi_5109_;
                                                    let _e3884 = phi_5112_;
                                                    local_66 = _e3882;
                                                    let _e3885 = (_e3884 < 6u);
                                                    if _e3885 {
                                                        let _e3886 = (_e3884 / 2u);
                                                        if (_e3886 == 0u) {
                                                            phi_24971_ = _e343.member_1.x;
                                                        } else {
                                                            phi_24971_ = select(_e343.member_1.z, _e343.member_1.y, (_e3886 == 1u));
                                                        }
                                                        let _e3894 = phi_24971_;
                                                        let _e3895 = (_e3894 * select(-1f, 1f, ((_e3884 % 2u) == 0u)));
                                                        if (_e3895 > 0f) {
                                                            let _e3899 = ((_e3886 + 1u) % 3u);
                                                            if (_e3899 == 0u) {
                                                                phi_24986_ = _e343.member_1.x;
                                                            } else {
                                                                phi_24986_ = select(_e343.member_1.z, _e343.member_1.y, (_e3899 == 1u));
                                                            }
                                                            let _e3904 = phi_24986_;
                                                            let _e3905 = (_e3904 / _e3895);
                                                            let _e3907 = ((_e3886 + 2u) % 3u);
                                                            if (_e3907 == 0u) {
                                                                phi_25001_ = _e343.member_1.x;
                                                            } else {
                                                                phi_25001_ = select(_e343.member_1.z, _e343.member_1.y, (_e3907 == 1u));
                                                            }
                                                            let _e3912 = phi_25001_;
                                                            let _e3913 = (_e3912 / _e3895);
                                                            let _e3915 = (1f + (_e3864 * 12f));
                                                            if (abs(_e3905) <= _e3915) {
                                                                if (abs(_e3913) <= _e3915) {
                                                                    let _e3921 = f32(_e3868);
                                                                    let _e3923 = floor((((_e3905 + 1f) * 0.5f) * _e3921));
                                                                    let _e3930 = select(0i, select(select(i32(_e3923), i32(-2147483648), (_e3923 < -2147483600f)), 2147483647i, (_e3923 > 2147483500f)), (_e3923 == _e3923));
                                                                    let _e3932 = (bitcast<i32>(_e3868) - 1i);
                                                                    let _e3933 = (0i <= _e3932);
                                                                    if _e3933 {
                                                                    } else {
                                                                        phi_33966_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3930 < 0i) {
                                                                        phi_5183_ = 0i;
                                                                    } else {
                                                                        phi_5183_ = select(_e3930, _e3932, (_e3930 > _e3932));
                                                                    }
                                                                    let _e3938 = phi_5183_;
                                                                    let _e3942 = floor((((_e3913 + 1f) * 0.5f) * _e3921));
                                                                    let _e3949 = select(0i, select(select(i32(_e3942), i32(-2147483648), (_e3942 < -2147483600f)), 2147483647i, (_e3942 > 2147483500f)), (_e3942 == _e3942));
                                                                    if _e3933 {
                                                                    } else {
                                                                        phi_33966_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3949 < 0i) {
                                                                        phi_5213_ = 0i;
                                                                    } else {
                                                                        phi_5213_ = select(_e3949, _e3932, (_e3949 > _e3932));
                                                                    }
                                                                    let _e3954 = phi_5213_;
                                                                    let _e3955 = (_e3938 - 1i);
                                                                    phi_5221_ = _e3882;
                                                                    phi_5224_ = select(0i, _e3955, (0i < _e3955));
                                                                    loop {
                                                                        let _e3959 = phi_5221_;
                                                                        let _e3961 = phi_5224_;
                                                                        let _e3962 = (_e3938 + 1i);
                                                                        local_65 = _e3959;
                                                                        let _e3965 = (_e3961 <= select(_e3962, _e3932, (_e3932 < _e3962)));
                                                                        if _e3965 {
                                                                            let _e3966 = (_e3954 - 1i);
                                                                            phi_5247_ = _e3959;
                                                                            phi_5250_ = select(0i, _e3966, (0i < _e3966));
                                                                            loop {
                                                                                let _e3970 = phi_5247_;
                                                                                let _e3972 = phi_5250_;
                                                                                let _e3973 = (_e3954 + 1i);
                                                                                local_64 = _e3970;
                                                                                let _e3976 = (_e3972 <= select(_e3973, _e3932, (_e3932 < _e3973)));
                                                                                if _e3976 {
                                                                                    let _e3983 = (bitcast<u32>(_e3871) + ((((_e3884 * _e3868) + bitcast<u32>(_e3961)) * _e3868) + bitcast<u32>(_e3972)));
                                                                                    let _e3985 = global_6.member[_e3983];
                                                                                    let _e3989 = global_6.member[(_e3983 + 1u)];
                                                                                    phi_5321_ = _e3970;
                                                                                    phi_5324_ = bitcast<u32>(_e3985);
                                                                                    loop {
                                                                                        let _e3992 = phi_5321_;
                                                                                        let _e3994 = phi_5324_;
                                                                                        local_63 = _e3992;
                                                                                        let _e3995 = (_e3994 < bitcast<u32>(_e3989));
                                                                                        if _e3995 {
                                                                                            let _e3997 = (bitcast<u32>(_e3875) + (6u * _e3994));
                                                                                            let _e3999 = global_6.member[_e3997];
                                                                                            let _e4002 = global_6.member[(_e3997 + 1u)];
                                                                                            let _e4005 = global_6.member[(_e3997 + 2u)];
                                                                                            let _e4008 = global_6.member[(_e3997 + 3u)];
                                                                                            let _e4011 = global_6.member[(_e3997 + 4u)];
                                                                                            let _e4014 = global_6.member[(_e3997 + 5u)];
                                                                                            let _e4015 = (_e343.member_1.x - _e3999);
                                                                                            let _e4016 = (_e343.member_1.y - _e4002);
                                                                                            let _e4017 = (_e343.member_1.z - _e4005);
                                                                                            let _e4022 = (((_e4015 * _e4015) + (_e4016 * _e4016)) + (_e4017 * _e4017));
                                                                                            if (_e4022 < (_e3877 * _e3877)) {
                                                                                                let _e4030 = ((1f / ((6.2831855f * _e3864) * _e3864)) * exp((-(_e4022) / ((2f * _e3864) * _e3864))));
                                                                                                phi_5444_ = vec3<f32>((_e3992.x + (_e4008 * _e4030)), (_e3992.y + (_e4011 * _e4030)), (_e3992.z + (_e4014 * _e4030)));
                                                                                            } else {
                                                                                                phi_5444_ = _e3992;
                                                                                            }
                                                                                            let _e4042 = phi_5444_;
                                                                                            phi_5322_ = _e4042;
                                                                                            phi_5325_ = (_e3994 + 1u);
                                                                                        } else {
                                                                                            phi_5322_ = vec3<f32>();
                                                                                            phi_5325_ = u32();
                                                                                        }
                                                                                        let _e4045 = phi_5322_;
                                                                                        let _e4047 = phi_5325_;
                                                                                        phi_5321_ = _e4045;
                                                                                        phi_5324_ = _e4047;
                                                                                        if !(_e3995) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7064 = local_63;
                                                                                    phi_5248_ = _e7064;
                                                                                    phi_5251_ = (_e3972 + 1i);
                                                                                } else {
                                                                                    phi_5248_ = vec3<f32>();
                                                                                    phi_5251_ = i32();
                                                                                }
                                                                                let _e4051 = phi_5248_;
                                                                                let _e4053 = phi_5251_;
                                                                                phi_5247_ = _e4051;
                                                                                phi_5250_ = _e4053;
                                                                                if !(_e3976) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7068 = local_64;
                                                                            phi_5222_ = _e7068;
                                                                            phi_5225_ = (_e3961 + 1i);
                                                                        } else {
                                                                            phi_5222_ = vec3<f32>();
                                                                            phi_5225_ = i32();
                                                                        }
                                                                        let _e4057 = phi_5222_;
                                                                        let _e4059 = phi_5225_;
                                                                        phi_5221_ = _e4057;
                                                                        phi_5224_ = _e4059;
                                                                        if !(_e3965) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7072 = local_65;
                                                                    phi_5449_ = _e7072;
                                                                } else {
                                                                    phi_5449_ = _e3882;
                                                                }
                                                                let _e4062 = phi_5449_;
                                                                phi_5450_ = _e4062;
                                                            } else {
                                                                phi_5450_ = _e3882;
                                                            }
                                                            let _e4064 = phi_5450_;
                                                            phi_5451_ = _e4064;
                                                        } else {
                                                            phi_5451_ = _e3882;
                                                        }
                                                        let _e4066 = phi_5451_;
                                                        phi_5110_ = _e4066;
                                                        phi_5113_ = (_e3884 + 1u);
                                                    } else {
                                                        phi_5110_ = vec3<f32>();
                                                        phi_5113_ = u32();
                                                    }
                                                    let _e4069 = phi_5110_;
                                                    let _e4071 = phi_5113_;
                                                    phi_5109_ = _e4069;
                                                    phi_5112_ = _e4071;
                                                    phi_33966_ = _e3779;
                                                    if !(_e3885) {
                                                        break;
                                                    }
                                                }
                                                let _e4074 = phi_33966_;
                                                phi_34207_ = _e4074;
                                                if _e4074 {
                                                    break;
                                                }
                                                phi_34008_ = _e4074;
                                                let _e7081 = local_66;
                                                phi_5471_ = _e7081;
                                            } else {
                                                phi_34008_ = _e3779;
                                                phi_5471_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4076 = phi_34008_;
                                            let _e4078 = phi_5471_;
                                            if (_e3798 != 4294967295u) {
                                                let _e4081 = global_6.member[_e3798];
                                                let _e4084 = global_6.member[(_e3798 + 1u)];
                                                let _e4087 = global_6.member[(_e3798 + 2u)];
                                                let _e4090 = global_6.member[(_e3798 + 3u)];
                                                let _e4093 = global_6.member[(_e3798 + 4u)];
                                                let _e4096 = global_6.member[(_e3798 + 5u)];
                                                let _e4099 = global_6.member[(_e3798 + 6u)];
                                                let _e4102 = global_6.member[(_e3798 + 7u)];
                                                let _e4105 = global_6.member[(_e3798 + 8u)];
                                                let _e4108 = global_6.member[(_e3798 + 9u)];
                                                let _e4111 = global_6.member[(_e3798 + 10u)];
                                                let _e4115 = global_6.member[(_e3798 + 11u)];
                                                let _e4118 = global_6.member[(_e3798 + 12u)];
                                                let _e4121 = global_6.member[(_e3798 + 13u)];
                                                let _e4123 = (_e4081 > 0f);
                                                let _e4124 = (_e4084 > 0f);
                                                if _e4124 {
                                                    let _e4127 = (abs(_e3850) <= 0.8f);
                                                    let _e4128 = select(2u, 0u, _e4123);
                                                    phi_5684_ = _e4128;
                                                    phi_5685_ = _e4127;
                                                    phi_5688_ = _e4128;
                                                    phi_5689_ = _e4127;
                                                    phi_5690_ = select(true, false, _e4127);
                                                } else {
                                                    phi_5684_ = u32();
                                                    phi_5685_ = bool();
                                                    phi_5688_ = select(2u, 0u, _e4123);
                                                    phi_5689_ = false;
                                                    phi_5690_ = true;
                                                }
                                                let _e4131 = phi_5684_;
                                                let _e4133 = phi_5685_;
                                                let _e4135 = phi_5688_;
                                                let _e4137 = phi_5689_;
                                                let _e4139 = phi_5690_;
                                                let _e4141 = select(select(f32(), 0f, _e4124), 0f, _e4139);
                                                phi_5700_ = _e4141;
                                                phi_5703_ = _e4141;
                                                phi_5705_ = select(_e4131, _e4135, _e4139);
                                                phi_5707_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5709_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5711_ = 0f;
                                                phi_5713_ = 1f;
                                                loop {
                                                    let _e4146 = phi_5700_;
                                                    let _e4148 = phi_5703_;
                                                    let _e4150 = phi_5705_;
                                                    let _e4152 = phi_5707_;
                                                    let _e4154 = phi_5709_;
                                                    let _e4156 = phi_5711_;
                                                    let _e4158 = phi_5713_;
                                                    local_26 = _e4158;
                                                    local_27 = _e4158;
                                                    local_28 = _e4158;
                                                    local_29 = _e4158;
                                                    local_30 = _e4156;
                                                    local_31 = _e4148;
                                                    local_32 = _e4146;
                                                    let _e4159 = (_e4150 < select(11u, 2u, _e4139));
                                                    if _e4159 {
                                                        let _e4160 = (_e4150 < 2u);
                                                        if _e4160 {
                                                            let _e4216 = select(23f, 4f, (_e4150 == 0u));
                                                            phi_5791_ = 4u;
                                                            phi_5793_ = vec3<f32>((_e343.member_1.x * _e4216), (_e343.member_1.y * _e4216), (_e343.member_1.z * _e4216));
                                                        } else {
                                                            if (_e4150 == 2u) {
                                                                phi_5789_ = 3u;
                                                                phi_5790_ = vec3<f32>(((_e343.member_1.x * 1.5f) + 5f), ((_e343.member_1.y * 1.5f) + 5f), ((_e343.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4162 = (_e4150 >= 7u);
                                                                if _e4162 {
                                                                    phi_5751_ = (_e4150 - 7u);
                                                                } else {
                                                                    phi_5751_ = (_e4150 - 3u);
                                                                }
                                                                let _e4166 = phi_5751_;
                                                                if _e4162 {
                                                                    phi_5766_ = vec3<f32>(((_e343.member_1.x * 11f) + -3f), ((_e343.member_1.y * 11f) + 1f), ((_e343.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5766_ = vec3<f32>((_e343.member_1.x * 7f), (_e343.member_1.y * 7f), (_e343.member_1.z * 7f));
                                                                }
                                                                let _e4179 = phi_5766_;
                                                                if (_e4166 < 3u) {
                                                                    if (_e4166 == 0u) {
                                                                        phi_25016_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_25016_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4166 == 1u)));
                                                                    }
                                                                    let _e4198 = phi_25016_;
                                                                    phi_5787_ = 4u;
                                                                    phi_5788_ = (_e4179 + _e4198);
                                                                } else {
                                                                    if _e4162 {
                                                                        phi_5786_ = (_e4179 + _e4152);
                                                                    } else {
                                                                        phi_5786_ = (_e4179 + vec3<f32>((_e4154.x * 1.1f), (_e4154.y * 1.1f), (_e4154.z * 1.1f)));
                                                                    }
                                                                    let _e4191 = phi_5786_;
                                                                    phi_5787_ = select(7u, 5u, _e4162);
                                                                    phi_5788_ = _e4191;
                                                                }
                                                                let _e4201 = phi_5787_;
                                                                let _e4203 = phi_5788_;
                                                                phi_5789_ = _e4201;
                                                                phi_5790_ = _e4203;
                                                            }
                                                            let _e4212 = phi_5789_;
                                                            let _e4214 = phi_5790_;
                                                            phi_5791_ = _e4212;
                                                            phi_5793_ = _e4214;
                                                        }
                                                        let _e4222 = phi_5791_;
                                                        let _e4224 = phi_5793_;
                                                        let _e4226 = select(vec3<f32>(_e4115, _e4118, _e4121), vec3<f32>(_e4105, _e4108, _e4111), vec3(_e4160));
                                                        let _e4228 = global_6.member[_e3790];
                                                        let _e4229 = bitcast<u32>(_e4228);
                                                        let _e4232 = global_6.member[(_e1301 + 11u)];
                                                        let _e4235 = global_6.member[(_e1301 + 12u)];
                                                        let _e4238 = global_6.member[(_e1301 + 13u)];
                                                        let _e4241 = global_6.member[(_e1301 + 14u)];
                                                        let _e4244 = global_6.member[(_e1301 + 15u)];
                                                        let _e4247 = global_6.member[(_e1301 + 16u)];
                                                        let _e4250 = global_6.member[(_e1301 + 17u)];
                                                        let _e4253 = global_6.member[(_e1301 + 18u)];
                                                        let _e4256 = global_6.member[(_e1301 + 19u)];
                                                        phi_5928_ = 0u;
                                                        phi_5931_ = 0f;
                                                        phi_5933_ = 1f;
                                                        phi_5935_ = 0f;
                                                        phi_5937_ = _e4224;
                                                        loop {
                                                            let _e4258 = phi_5928_;
                                                            let _e4260 = phi_5931_;
                                                            let _e4262 = phi_5933_;
                                                            let _e4264 = phi_5935_;
                                                            let _e4266 = phi_5937_;
                                                            local_23 = _e4260;
                                                            local_24 = _e4264;
                                                            local_25 = _e4260;
                                                            let _e4267 = (_e4258 < _e4222);
                                                            if _e4267 {
                                                                let _e4270 = (_e4266.x + _e4226.x);
                                                                let _e4273 = (_e4266.y + _e4226.y);
                                                                let _e4276 = (_e4266.z + _e4226.z);
                                                                let _e4277 = floor(_e4270);
                                                                let _e4278 = floor(_e4273);
                                                                let _e4279 = floor(_e4276);
                                                                let _e4288 = bitcast<u32>((select(0i, select(select(i32(_e4277), i32(-2147483648), (_e4277 < -2147483600f)), 2147483647i, (_e4277 > 2147483500f)), (_e4277 == _e4277)) & 255i));
                                                                let _e4297 = bitcast<u32>((select(0i, select(select(i32(_e4278), i32(-2147483648), (_e4278 < -2147483600f)), 2147483647i, (_e4278 > 2147483500f)), (_e4278 == _e4278)) & 255i));
                                                                let _e4306 = bitcast<u32>((select(0i, select(select(i32(_e4279), i32(-2147483648), (_e4279 < -2147483600f)), 2147483647i, (_e4279 > 2147483500f)), (_e4279 == _e4279)) & 255i));
                                                                let _e4307 = (_e4270 - _e4277);
                                                                let _e4308 = (_e4273 - _e4278);
                                                                let _e4309 = (_e4276 - _e4279);
                                                                let _e4316 = (((_e4307 * _e4307) * _e4307) * ((_e4307 * ((_e4307 * 6f) - 15f)) + 10f));
                                                                let _e4323 = (((_e4308 * _e4308) * _e4308) * ((_e4308 * ((_e4308 * 6f) - 15f)) + 10f));
                                                                let _e4333 = global_6.member[(_e4229 + _e4288)];
                                                                let _e4339 = (select(select(u32(_e4333), 0u, (_e4333 < 0f)), 4294967295u, (_e4333 > 4294967000f)) + _e4297);
                                                                let _e4342 = global_6.member[(_e4229 + _e4339)];
                                                                let _e4348 = (select(select(u32(_e4342), 0u, (_e4342 < 0f)), 4294967295u, (_e4342 > 4294967000f)) + _e4306);
                                                                let _e4352 = global_6.member[(_e4229 + (_e4339 + 1u))];
                                                                let _e4358 = (select(select(u32(_e4352), 0u, (_e4352 < 0f)), 4294967295u, (_e4352 > 4294967000f)) + _e4306);
                                                                let _e4362 = global_6.member[(_e4229 + (_e4288 + 1u))];
                                                                let _e4368 = (select(select(u32(_e4362), 0u, (_e4362 < 0f)), 4294967295u, (_e4362 > 4294967000f)) + _e4297);
                                                                let _e4371 = global_6.member[(_e4229 + _e4368)];
                                                                let _e4377 = (select(select(u32(_e4371), 0u, (_e4371 < 0f)), 4294967295u, (_e4371 > 4294967000f)) + _e4306);
                                                                let _e4381 = global_6.member[(_e4229 + (_e4368 + 1u))];
                                                                let _e4387 = (select(select(u32(_e4381), 0u, (_e4381 < 0f)), 4294967295u, (_e4381 > 4294967000f)) + _e4306);
                                                                let _e4390 = global_6.member[(_e4229 + _e4348)];
                                                                let _e4396 = (select(select(u32(_e4390), 0u, (_e4390 < 0f)), 4294967295u, (_e4390 > 4294967000f)) % 12u);
                                                                let _e4399 = select(-1f, 1f, ((_e4396 & 1u) == 0u));
                                                                let _e4402 = select(-1f, 1f, ((_e4396 & 2u) == 0u));
                                                                if (_e4396 < 4u) {
                                                                    phi_25077_ = ((_e4399 * _e4307) + (_e4402 * _e4308));
                                                                } else {
                                                                    if (_e4396 < 8u) {
                                                                        phi_25071_ = ((_e4399 * _e4307) + (_e4402 * _e4309));
                                                                    } else {
                                                                        phi_25071_ = ((_e4399 * _e4308) + (_e4402 * _e4309));
                                                                    }
                                                                    let _e4412 = phi_25071_;
                                                                    phi_25077_ = _e4412;
                                                                }
                                                                let _e4417 = phi_25077_;
                                                                let _e4420 = global_6.member[(_e4229 + _e4377)];
                                                                let _e4426 = (_e4307 - 1f);
                                                                let _e4427 = (select(select(u32(_e4420), 0u, (_e4420 < 0f)), 4294967295u, (_e4420 > 4294967000f)) % 12u);
                                                                let _e4430 = select(-1f, 1f, ((_e4427 & 1u) == 0u));
                                                                let _e4433 = select(-1f, 1f, ((_e4427 & 2u) == 0u));
                                                                if (_e4427 < 4u) {
                                                                    phi_25111_ = ((_e4430 * _e4426) + (_e4433 * _e4308));
                                                                } else {
                                                                    if (_e4427 < 8u) {
                                                                        phi_25105_ = ((_e4430 * _e4426) + (_e4433 * _e4309));
                                                                    } else {
                                                                        phi_25105_ = ((_e4430 * _e4308) + (_e4433 * _e4309));
                                                                    }
                                                                    let _e4443 = phi_25105_;
                                                                    phi_25111_ = _e4443;
                                                                }
                                                                let _e4448 = phi_25111_;
                                                                let _e4451 = global_6.member[(_e4229 + _e4358)];
                                                                let _e4457 = (_e4308 - 1f);
                                                                let _e4458 = (select(select(u32(_e4451), 0u, (_e4451 < 0f)), 4294967295u, (_e4451 > 4294967000f)) % 12u);
                                                                let _e4461 = select(-1f, 1f, ((_e4458 & 1u) == 0u));
                                                                let _e4464 = select(-1f, 1f, ((_e4458 & 2u) == 0u));
                                                                if (_e4458 < 4u) {
                                                                    phi_25145_ = ((_e4461 * _e4307) + (_e4464 * _e4457));
                                                                } else {
                                                                    if (_e4458 < 8u) {
                                                                        phi_25139_ = ((_e4461 * _e4307) + (_e4464 * _e4309));
                                                                    } else {
                                                                        phi_25139_ = ((_e4461 * _e4457) + (_e4464 * _e4309));
                                                                    }
                                                                    let _e4474 = phi_25139_;
                                                                    phi_25145_ = _e4474;
                                                                }
                                                                let _e4479 = phi_25145_;
                                                                let _e4482 = global_6.member[(_e4229 + _e4387)];
                                                                let _e4488 = (select(select(u32(_e4482), 0u, (_e4482 < 0f)), 4294967295u, (_e4482 > 4294967000f)) % 12u);
                                                                let _e4491 = select(-1f, 1f, ((_e4488 & 1u) == 0u));
                                                                let _e4494 = select(-1f, 1f, ((_e4488 & 2u) == 0u));
                                                                if (_e4488 < 4u) {
                                                                    phi_25179_ = ((_e4491 * _e4426) + (_e4494 * _e4457));
                                                                } else {
                                                                    if (_e4488 < 8u) {
                                                                        phi_25173_ = ((_e4491 * _e4426) + (_e4494 * _e4309));
                                                                    } else {
                                                                        phi_25173_ = ((_e4491 * _e4457) + (_e4494 * _e4309));
                                                                    }
                                                                    let _e4504 = phi_25173_;
                                                                    phi_25179_ = _e4504;
                                                                }
                                                                let _e4509 = phi_25179_;
                                                                let _e4513 = global_6.member[(_e4229 + (_e4348 + 1u))];
                                                                let _e4519 = (_e4309 - 1f);
                                                                let _e4520 = (select(select(u32(_e4513), 0u, (_e4513 < 0f)), 4294967295u, (_e4513 > 4294967000f)) % 12u);
                                                                let _e4523 = select(-1f, 1f, ((_e4520 & 1u) == 0u));
                                                                let _e4526 = select(-1f, 1f, ((_e4520 & 2u) == 0u));
                                                                if (_e4520 < 4u) {
                                                                    phi_25213_ = ((_e4523 * _e4307) + (_e4526 * _e4308));
                                                                } else {
                                                                    if (_e4520 < 8u) {
                                                                        phi_25207_ = ((_e4523 * _e4307) + (_e4526 * _e4519));
                                                                    } else {
                                                                        phi_25207_ = ((_e4523 * _e4308) + (_e4526 * _e4519));
                                                                    }
                                                                    let _e4536 = phi_25207_;
                                                                    phi_25213_ = _e4536;
                                                                }
                                                                let _e4541 = phi_25213_;
                                                                let _e4545 = global_6.member[(_e4229 + (_e4377 + 1u))];
                                                                let _e4551 = (select(select(u32(_e4545), 0u, (_e4545 < 0f)), 4294967295u, (_e4545 > 4294967000f)) % 12u);
                                                                let _e4554 = select(-1f, 1f, ((_e4551 & 1u) == 0u));
                                                                let _e4557 = select(-1f, 1f, ((_e4551 & 2u) == 0u));
                                                                if (_e4551 < 4u) {
                                                                    phi_25247_ = ((_e4554 * _e4426) + (_e4557 * _e4308));
                                                                } else {
                                                                    if (_e4551 < 8u) {
                                                                        phi_25241_ = ((_e4554 * _e4426) + (_e4557 * _e4519));
                                                                    } else {
                                                                        phi_25241_ = ((_e4554 * _e4308) + (_e4557 * _e4519));
                                                                    }
                                                                    let _e4567 = phi_25241_;
                                                                    phi_25247_ = _e4567;
                                                                }
                                                                let _e4572 = phi_25247_;
                                                                let _e4576 = global_6.member[(_e4229 + (_e4358 + 1u))];
                                                                let _e4582 = (select(select(u32(_e4576), 0u, (_e4576 < 0f)), 4294967295u, (_e4576 > 4294967000f)) % 12u);
                                                                let _e4585 = select(-1f, 1f, ((_e4582 & 1u) == 0u));
                                                                let _e4588 = select(-1f, 1f, ((_e4582 & 2u) == 0u));
                                                                if (_e4582 < 4u) {
                                                                    phi_25281_ = ((_e4585 * _e4307) + (_e4588 * _e4457));
                                                                } else {
                                                                    if (_e4582 < 8u) {
                                                                        phi_25275_ = ((_e4585 * _e4307) + (_e4588 * _e4519));
                                                                    } else {
                                                                        phi_25275_ = ((_e4585 * _e4457) + (_e4588 * _e4519));
                                                                    }
                                                                    let _e4598 = phi_25275_;
                                                                    phi_25281_ = _e4598;
                                                                }
                                                                let _e4603 = phi_25281_;
                                                                let _e4607 = global_6.member[(_e4229 + (_e4387 + 1u))];
                                                                let _e4613 = (select(select(u32(_e4607), 0u, (_e4607 < 0f)), 4294967295u, (_e4607 > 4294967000f)) % 12u);
                                                                let _e4616 = select(-1f, 1f, ((_e4613 & 1u) == 0u));
                                                                let _e4619 = select(-1f, 1f, ((_e4613 & 2u) == 0u));
                                                                if (_e4613 < 4u) {
                                                                    phi_25315_ = ((_e4616 * _e4426) + (_e4619 * _e4457));
                                                                } else {
                                                                    if (_e4613 < 8u) {
                                                                        phi_25309_ = ((_e4616 * _e4426) + (_e4619 * _e4519));
                                                                    } else {
                                                                        phi_25309_ = ((_e4616 * _e4457) + (_e4619 * _e4519));
                                                                    }
                                                                    let _e4629 = phi_25309_;
                                                                    phi_25315_ = _e4629;
                                                                }
                                                                let _e4634 = phi_25315_;
                                                                let _e4637 = (_e4417 + (_e4316 * (_e4448 - _e4417)));
                                                                let _e4643 = (_e4541 + (_e4316 * (_e4572 - _e4541)));
                                                                let _e4649 = (_e4637 + (_e4323 * ((_e4479 + (_e4316 * (_e4509 - _e4479))) - _e4637)));
                                                                phi_5929_ = (_e4258 + 1u);
                                                                phi_5932_ = (_e4260 + _e4262);
                                                                phi_5934_ = (_e4262 * 0.5f);
                                                                phi_5936_ = (_e4264 + ((_e4649 + ((((_e4309 * _e4309) * _e4309) * ((_e4309 * ((_e4309 * 6f) - 15f)) + 10f)) * ((_e4643 + (_e4323 * ((_e4603 + (_e4316 * (_e4634 - _e4603))) - _e4643))) - _e4649))) * _e4262));
                                                                phi_5938_ = vec3<f32>(((((_e4232 * _e4266.x) + (_e4235 * _e4266.y)) + (_e4238 * _e4266.z)) * 2.03f), ((((_e4241 * _e4266.x) + (_e4244 * _e4266.y)) + (_e4247 * _e4266.z)) * 2.03f), ((((_e4250 * _e4266.x) + (_e4253 * _e4266.y)) + (_e4256 * _e4266.z)) * 2.03f));
                                                            } else {
                                                                phi_5929_ = u32();
                                                                phi_5932_ = f32();
                                                                phi_5934_ = f32();
                                                                phi_5936_ = f32();
                                                                phi_5938_ = vec3<f32>();
                                                            }
                                                            let _e4681 = phi_5929_;
                                                            let _e4683 = phi_5932_;
                                                            let _e4685 = phi_5934_;
                                                            let _e4687 = phi_5936_;
                                                            let _e4689 = phi_5938_;
                                                            phi_5928_ = _e4681;
                                                            phi_5931_ = _e4683;
                                                            phi_5933_ = _e4685;
                                                            phi_5935_ = _e4687;
                                                            phi_5937_ = _e4689;
                                                            if !(_e4267) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4692 = local_23;
                                                        if (_e4692 > 0f) {
                                                            let _e4695 = local_24;
                                                            let _e4697 = local_25;
                                                            phi_6354_ = (_e4695 / _e4697);
                                                        } else {
                                                            phi_6354_ = 0f;
                                                        }
                                                        let _e4700 = phi_6354_;
                                                        if (_e4150 == 0u) {
                                                            phi_6437_ = _e4146;
                                                            phi_6438_ = _e4148;
                                                            phi_6439_ = _e4152;
                                                            phi_6440_ = _e4154;
                                                            phi_6441_ = _e4156;
                                                            phi_6442_ = (_e4158 + (1.5f * _e4700));
                                                        } else {
                                                            if (_e4150 == 1u) {
                                                                phi_6431_ = _e4146;
                                                                phi_6432_ = _e4148;
                                                                phi_6433_ = _e4152;
                                                                phi_6434_ = _e4154;
                                                                phi_6435_ = _e4156;
                                                                phi_6436_ = (_e4158 + (0.8f * _e4700));
                                                            } else {
                                                                let _e4703 = (_e4150 == 2u);
                                                                if _e4703 {
                                                                    phi_6426_ = _e4146;
                                                                    phi_6427_ = _e4148;
                                                                    phi_6428_ = _e4152;
                                                                    phi_6429_ = _e4154;
                                                                } else {
                                                                    if (_e4150 == 3u) {
                                                                        phi_6422_ = _e4146;
                                                                        phi_6423_ = _e4148;
                                                                        phi_6424_ = _e4152;
                                                                        phi_6425_ = vec3<f32>(_e4700, _e4154.y, _e4154.z);
                                                                    } else {
                                                                        if (_e4150 == 4u) {
                                                                            phi_6418_ = _e4146;
                                                                            phi_6419_ = _e4148;
                                                                            phi_6420_ = _e4152;
                                                                            phi_6421_ = vec3<f32>(_e4154.x, _e4700, _e4154.z);
                                                                        } else {
                                                                            if (_e4150 == 5u) {
                                                                                phi_6414_ = _e4146;
                                                                                phi_6415_ = _e4148;
                                                                                phi_6416_ = _e4152;
                                                                                phi_6417_ = vec3<f32>(_e4154.x, _e4154.y, _e4700);
                                                                            } else {
                                                                                let _e4707 = (_e4150 == 6u);
                                                                                if _e4707 {
                                                                                    phi_6411_ = _e4146;
                                                                                    phi_6413_ = _e4152;
                                                                                } else {
                                                                                    if (_e4150 == 7u) {
                                                                                        phi_6409_ = _e4146;
                                                                                        phi_6410_ = vec3<f32>(_e4700, _e4152.y, _e4152.z);
                                                                                    } else {
                                                                                        if (_e4150 == 8u) {
                                                                                            phi_6407_ = _e4146;
                                                                                            phi_6408_ = vec3<f32>(_e4152.x, _e4700, _e4152.z);
                                                                                        } else {
                                                                                            let _e4710 = (_e4150 == 9u);
                                                                                            if _e4710 {
                                                                                                phi_6406_ = vec3<f32>(_e4152.x, _e4152.y, _e4700);
                                                                                            } else {
                                                                                                phi_6406_ = _e4152;
                                                                                            }
                                                                                            let _e4716 = phi_6406_;
                                                                                            phi_6407_ = select(_e4700, _e4146, _e4710);
                                                                                            phi_6408_ = _e4716;
                                                                                        }
                                                                                        let _e4723 = phi_6407_;
                                                                                        let _e4725 = phi_6408_;
                                                                                        phi_6409_ = _e4723;
                                                                                        phi_6410_ = _e4725;
                                                                                    }
                                                                                    let _e4731 = phi_6409_;
                                                                                    let _e4733 = phi_6410_;
                                                                                    phi_6411_ = _e4731;
                                                                                    phi_6413_ = _e4733;
                                                                                }
                                                                                let _e4735 = phi_6411_;
                                                                                let _e4737 = phi_6413_;
                                                                                phi_6414_ = _e4735;
                                                                                phi_6415_ = select(_e4148, _e4700, _e4707);
                                                                                phi_6416_ = _e4737;
                                                                                phi_6417_ = _e4154;
                                                                            }
                                                                            let _e4744 = phi_6414_;
                                                                            let _e4746 = phi_6415_;
                                                                            let _e4748 = phi_6416_;
                                                                            let _e4750 = phi_6417_;
                                                                            phi_6418_ = _e4744;
                                                                            phi_6419_ = _e4746;
                                                                            phi_6420_ = _e4748;
                                                                            phi_6421_ = _e4750;
                                                                        }
                                                                        let _e4756 = phi_6418_;
                                                                        let _e4758 = phi_6419_;
                                                                        let _e4760 = phi_6420_;
                                                                        let _e4762 = phi_6421_;
                                                                        phi_6422_ = _e4756;
                                                                        phi_6423_ = _e4758;
                                                                        phi_6424_ = _e4760;
                                                                        phi_6425_ = _e4762;
                                                                    }
                                                                    let _e4768 = phi_6422_;
                                                                    let _e4770 = phi_6423_;
                                                                    let _e4772 = phi_6424_;
                                                                    let _e4774 = phi_6425_;
                                                                    phi_6426_ = _e4768;
                                                                    phi_6427_ = _e4770;
                                                                    phi_6428_ = _e4772;
                                                                    phi_6429_ = _e4774;
                                                                }
                                                                let _e4776 = phi_6426_;
                                                                let _e4778 = phi_6427_;
                                                                let _e4780 = phi_6428_;
                                                                let _e4782 = phi_6429_;
                                                                phi_6431_ = _e4776;
                                                                phi_6432_ = _e4778;
                                                                phi_6433_ = _e4780;
                                                                phi_6434_ = _e4782;
                                                                phi_6435_ = select(_e4156, _e4700, _e4703);
                                                                phi_6436_ = _e4158;
                                                            }
                                                            let _e4787 = phi_6431_;
                                                            let _e4789 = phi_6432_;
                                                            let _e4791 = phi_6433_;
                                                            let _e4793 = phi_6434_;
                                                            let _e4795 = phi_6435_;
                                                            let _e4797 = phi_6436_;
                                                            phi_6437_ = _e4787;
                                                            phi_6438_ = _e4789;
                                                            phi_6439_ = _e4791;
                                                            phi_6440_ = _e4793;
                                                            phi_6441_ = _e4795;
                                                            phi_6442_ = _e4797;
                                                        }
                                                        let _e4801 = phi_6437_;
                                                        let _e4803 = phi_6438_;
                                                        let _e4805 = phi_6439_;
                                                        let _e4807 = phi_6440_;
                                                        let _e4809 = phi_6441_;
                                                        let _e4811 = phi_6442_;
                                                        phi_5701_ = _e4801;
                                                        phi_5704_ = _e4803;
                                                        phi_5706_ = (_e4150 + 1u);
                                                        phi_5708_ = _e4805;
                                                        phi_5710_ = _e4807;
                                                        phi_5712_ = _e4809;
                                                        phi_5714_ = _e4811;
                                                    } else {
                                                        phi_5701_ = f32();
                                                        phi_5704_ = f32();
                                                        phi_5706_ = u32();
                                                        phi_5708_ = vec3<f32>();
                                                        phi_5710_ = vec3<f32>();
                                                        phi_5712_ = f32();
                                                        phi_5714_ = f32();
                                                    }
                                                    let _e4814 = phi_5701_;
                                                    let _e4816 = phi_5704_;
                                                    let _e4818 = phi_5706_;
                                                    let _e4820 = phi_5708_;
                                                    let _e4822 = phi_5710_;
                                                    let _e4824 = phi_5712_;
                                                    let _e4826 = phi_5714_;
                                                    phi_5700_ = _e4814;
                                                    phi_5703_ = _e4816;
                                                    phi_5705_ = _e4818;
                                                    phi_5707_ = _e4820;
                                                    phi_5709_ = _e4822;
                                                    phi_5711_ = _e4824;
                                                    phi_5713_ = _e4826;
                                                    if !(_e4159) {
                                                        break;
                                                    }
                                                }
                                                if _e4123 {
                                                    let _e4841 = (exp((-(abs(_e3850)) / (0.06f + (abs(_e3861) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3861 * -0.71428573f) * (_e3861 * 0.71428573f))))));
                                                    let _e4843 = (_e3850 * 1.3333334f);
                                                    let _e4848 = (1.3f * exp((((_e3861 * _e3861) + (_e4843 * _e4843)) * -12.499999f)));
                                                    let _e4859 = local_26;
                                                    let _e4861 = local_27;
                                                    if (_e4859 != _e4861) {
                                                        phi_25391_ = true;
                                                    } else {
                                                        let _e4864 = local_28;
                                                        phi_25391_ = (0.1f >= _e4864);
                                                    }
                                                    let _e4867 = phi_25391_;
                                                    let _e4869 = local_29;
                                                    let _e4872 = (pow(select(_e4869, 0.1f, _e4867), 1.3f) * _e4081);
                                                    phi_6474_ = vec3<f32>((_e4078.x + (((_e4087 * _e4841) + (_e4096 * _e4848)) * _e4872)), (_e4078.y + (((_e4090 * _e4841) + (_e4099 * _e4848)) * _e4872)), (_e4078.z + (((_e4093 * _e4841) + (_e4102 * _e4848)) * _e4872)));
                                                } else {
                                                    phi_6474_ = _e4078;
                                                }
                                                let _e4884 = phi_6474_;
                                                if select(_e4133, _e4137, _e4139) {
                                                    let _e4886 = local_30;
                                                    let _e4892 = ((_e3850 - (0.025f * _e4886)) / (0.045f + (abs(_e3861) * 0.0095492955f)));
                                                    let _e4897 = local_31;
                                                    let _e4898 = (_e4897 * 3.3333333f);
                                                    let _e4900 = select(_e4898, 0f, (_e4898 < 0f));
                                                    let _e4902 = select(_e4900, 1f, (_e4900 > 1f));
                                                    let _e4918 = local_32;
                                                    let _e4920 = ((_e4918 - 0.25f) * 5.0000005f);
                                                    let _e4922 = select(_e4920, 0f, (_e4920 < 0f));
                                                    let _e4924 = select(_e4922, 1f, (_e4922 > 1f));
                                                    phi_6553_ = (_e4084 * (((exp((-(_e4892) * _e4892)) * (0.15f + (3f * ((_e4902 * _e4902) * (3f - (2f * _e4902)))))) * (0.5f + (0.9f * exp(((_e3861 * -0.6666667f) * (_e3861 * 0.6666667f)))))) + (1.2f * (exp(((_e3850 * -3.3333333f) * (_e3850 * 3.3333333f))) * ((_e4924 * _e4924) * (3f - (2f * _e4924)))))));
                                                } else {
                                                    phi_6553_ = 0f;
                                                }
                                                let _e4938 = phi_6553_;
                                                phi_6554_ = _e4884;
                                                phi_6555_ = _e4938;
                                            } else {
                                                phi_6554_ = _e4078;
                                                phi_6555_ = 0f;
                                            }
                                            let _e4940 = phi_6554_;
                                            let _e4942 = phi_6555_;
                                            phi_6557_ = _e4940;
                                            phi_6560_ = 0u;
                                            loop {
                                                let _e4944 = phi_6557_;
                                                let _e4946 = phi_6560_;
                                                local_49 = _e4944;
                                                local_50 = _e4944;
                                                local_51 = _e4944;
                                                let _e4947 = (_e4946 < bitcast<u32>(_e3813));
                                                if _e4947 {
                                                    let _e4949 = (bitcast<u32>(_e3809) + (_e4946 * 29u));
                                                    let _e4951 = global_6.member[_e4949];
                                                    let _e4952 = (_e4951 > 0.5f);
                                                    let _e4955 = global_6.member[(_e4949 + 1u)];
                                                    let _e4958 = global_6.member[(_e4949 + 2u)];
                                                    let _e4961 = global_6.member[(_e4949 + 3u)];
                                                    let _e4964 = global_6.member[(_e4949 + 4u)];
                                                    let _e4967 = global_6.member[(_e4949 + 5u)];
                                                    let _e4970 = global_6.member[(_e4949 + 6u)];
                                                    let _e4975 = (((_e343.member_1.x * _e4955) + (_e343.member_1.y * _e4958)) + (_e343.member_1.z * _e4961));
                                                    let _e4976 = (_e4975 < _e4967);
                                                    if _e4976 {
                                                        phi_7706_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7707_ = vec3<f32>();
                                                    } else {
                                                        let _e4979 = global_6.member[(_e4949 + 7u)];
                                                        let _e4982 = global_6.member[(_e4949 + 8u)];
                                                        let _e4985 = global_6.member[(_e4949 + 9u)];
                                                        let _e4988 = global_6.member[(_e4949 + 10u)];
                                                        let _e4991 = global_6.member[(_e4949 + 11u)];
                                                        let _e4994 = global_6.member[(_e4949 + 12u)];
                                                        let _e4997 = global_6.member[(_e4949 + 13u)];
                                                        let _e5000 = global_6.member[(_e4949 + 14u)];
                                                        let _e5003 = global_6.member[(_e4949 + 15u)];
                                                        let _e5006 = global_6.member[(_e4949 + 16u)];
                                                        let _e5009 = global_6.member[(_e4949 + 17u)];
                                                        let _e5012 = global_6.member[(_e4949 + 18u)];
                                                        let _e5015 = global_6.member[(_e4949 + 19u)];
                                                        let _e5018 = global_6.member[(_e4949 + 20u)];
                                                        let _e5021 = global_6.member[(_e4949 + 21u)];
                                                        let _e5024 = global_6.member[(_e4949 + 22u)];
                                                        let _e5027 = global_6.member[(_e4949 + 23u)];
                                                        let _e5030 = global_6.member[(_e4949 + 24u)];
                                                        let _e5033 = global_6.member[(_e4949 + 25u)];
                                                        let _e5036 = global_6.member[(_e4949 + 26u)];
                                                        let _e5039 = global_6.member[(_e4949 + 27u)];
                                                        let _e5042 = global_6.member[(_e4949 + 28u)];
                                                        let _e5049 = (((_e343.member_1.x / _e4975) - _e4955) / _e4964);
                                                        let _e5050 = (((_e343.member_1.y / _e4975) - _e4958) / _e4964);
                                                        let _e5051 = (((_e343.member_1.z / _e4975) - _e4961) / _e4964);
                                                        if _e4952 {
                                                            let _e5656 = (((_e5049 * _e5006) + (_e5050 * _e5009)) + (_e5051 * _e5012));
                                                            let _e5662 = ((((_e5049 * _e5015) + (_e5050 * _e5018)) + (_e5051 * _e5021)) / _e5024);
                                                            phi_7704_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4970 * exp((-2.5f * pow((sqrt(((_e5656 * _e5656) + (_e5662 * _e5662))) * 3.3333333f), 0.7f)))));
                                                            phi_7705_ = vec3<f32>();
                                                        } else {
                                                            let _e5056 = (((_e4979 * _e4955) + (_e4982 * _e4958)) + (_e4985 * _e4961));
                                                            if (abs(_e5056) < 0.12f) {
                                                                phi_7008_ = select(0.12f, -0.12f, (_e5056 < 0f));
                                                            } else {
                                                                phi_7008_ = _e5056;
                                                            }
                                                            let _e5062 = phi_7008_;
                                                            let _e5068 = ((((_e4979 * _e5049) + (_e4982 * _e5050)) + (_e4985 * _e5051)) / _e5062);
                                                            let _e5072 = (_e5049 - (_e4955 * _e5068));
                                                            let _e5073 = (_e5050 - (_e4958 * _e5068));
                                                            let _e5074 = (_e5051 - (_e4961 * _e5068));
                                                            let _e5079 = (((_e5072 * _e4988) + (_e5073 * _e4991)) + (_e5074 * _e4994));
                                                            let _e5084 = (((_e5072 * _e4997) + (_e5073 * _e5000)) + (_e5074 * _e5003));
                                                            let _e5088 = sqrt(((_e5079 * _e5079) + (_e5084 * _e5084)));
                                                            let _e5100 = abs(_e5062);
                                                            let _e5102 = (pow((0.5f + (0.5f * cos(((_e5027 * (atan2(_e5084, _e5079) - (log((_e5088 + 0.03f)) / _e5030))) + _e5033)))), 3f) * sqrt(_e5100));
                                                            let _e5107 = global_6.member[_e3790];
                                                            let _e5108 = bitcast<u32>(_e5107);
                                                            let _e5111 = global_6.member[(_e1301 + 11u)];
                                                            let _e5114 = global_6.member[(_e1301 + 12u)];
                                                            let _e5117 = global_6.member[(_e1301 + 13u)];
                                                            let _e5120 = global_6.member[(_e1301 + 14u)];
                                                            let _e5123 = global_6.member[(_e1301 + 15u)];
                                                            let _e5126 = global_6.member[(_e1301 + 16u)];
                                                            let _e5129 = global_6.member[(_e1301 + 17u)];
                                                            let _e5132 = global_6.member[(_e1301 + 18u)];
                                                            let _e5135 = global_6.member[(_e1301 + 19u)];
                                                            phi_7188_ = 0u;
                                                            phi_7191_ = 0f;
                                                            phi_7193_ = 1f;
                                                            phi_7195_ = 0f;
                                                            phi_7197_ = vec3<f32>((_e5079 * 9f), (_e5084 * 9f), 0f);
                                                            loop {
                                                                let _e5137 = phi_7188_;
                                                                let _e5139 = phi_7191_;
                                                                let _e5141 = phi_7193_;
                                                                let _e5143 = phi_7195_;
                                                                let _e5145 = phi_7197_;
                                                                local_33 = _e5139;
                                                                local_34 = _e5143;
                                                                local_35 = _e5139;
                                                                let _e5146 = (_e5137 < 4u);
                                                                if _e5146 {
                                                                    let _e5148 = (_e5145.x + _e5036);
                                                                    let _e5150 = (_e5145.y + _e5039);
                                                                    let _e5152 = (_e5145.z + _e5042);
                                                                    let _e5153 = floor(_e5148);
                                                                    let _e5154 = floor(_e5150);
                                                                    let _e5155 = floor(_e5152);
                                                                    let _e5164 = bitcast<u32>((select(0i, select(select(i32(_e5153), i32(-2147483648), (_e5153 < -2147483600f)), 2147483647i, (_e5153 > 2147483500f)), (_e5153 == _e5153)) & 255i));
                                                                    let _e5173 = bitcast<u32>((select(0i, select(select(i32(_e5154), i32(-2147483648), (_e5154 < -2147483600f)), 2147483647i, (_e5154 > 2147483500f)), (_e5154 == _e5154)) & 255i));
                                                                    let _e5182 = bitcast<u32>((select(0i, select(select(i32(_e5155), i32(-2147483648), (_e5155 < -2147483600f)), 2147483647i, (_e5155 > 2147483500f)), (_e5155 == _e5155)) & 255i));
                                                                    let _e5183 = (_e5148 - _e5153);
                                                                    let _e5184 = (_e5150 - _e5154);
                                                                    let _e5185 = (_e5152 - _e5155);
                                                                    let _e5192 = (((_e5183 * _e5183) * _e5183) * ((_e5183 * ((_e5183 * 6f) - 15f)) + 10f));
                                                                    let _e5199 = (((_e5184 * _e5184) * _e5184) * ((_e5184 * ((_e5184 * 6f) - 15f)) + 10f));
                                                                    let _e5209 = global_6.member[(_e5108 + _e5164)];
                                                                    let _e5215 = (select(select(u32(_e5209), 0u, (_e5209 < 0f)), 4294967295u, (_e5209 > 4294967000f)) + _e5173);
                                                                    let _e5218 = global_6.member[(_e5108 + _e5215)];
                                                                    let _e5224 = (select(select(u32(_e5218), 0u, (_e5218 < 0f)), 4294967295u, (_e5218 > 4294967000f)) + _e5182);
                                                                    let _e5228 = global_6.member[(_e5108 + (_e5215 + 1u))];
                                                                    let _e5234 = (select(select(u32(_e5228), 0u, (_e5228 < 0f)), 4294967295u, (_e5228 > 4294967000f)) + _e5182);
                                                                    let _e5238 = global_6.member[(_e5108 + (_e5164 + 1u))];
                                                                    let _e5244 = (select(select(u32(_e5238), 0u, (_e5238 < 0f)), 4294967295u, (_e5238 > 4294967000f)) + _e5173);
                                                                    let _e5247 = global_6.member[(_e5108 + _e5244)];
                                                                    let _e5253 = (select(select(u32(_e5247), 0u, (_e5247 < 0f)), 4294967295u, (_e5247 > 4294967000f)) + _e5182);
                                                                    let _e5257 = global_6.member[(_e5108 + (_e5244 + 1u))];
                                                                    let _e5263 = (select(select(u32(_e5257), 0u, (_e5257 < 0f)), 4294967295u, (_e5257 > 4294967000f)) + _e5182);
                                                                    let _e5266 = global_6.member[(_e5108 + _e5224)];
                                                                    let _e5272 = (select(select(u32(_e5266), 0u, (_e5266 < 0f)), 4294967295u, (_e5266 > 4294967000f)) % 12u);
                                                                    let _e5275 = select(-1f, 1f, ((_e5272 & 1u) == 0u));
                                                                    let _e5278 = select(-1f, 1f, ((_e5272 & 2u) == 0u));
                                                                    if (_e5272 < 4u) {
                                                                        phi_25486_ = ((_e5275 * _e5183) + (_e5278 * _e5184));
                                                                    } else {
                                                                        if (_e5272 < 8u) {
                                                                            phi_25480_ = ((_e5275 * _e5183) + (_e5278 * _e5185));
                                                                        } else {
                                                                            phi_25480_ = ((_e5275 * _e5184) + (_e5278 * _e5185));
                                                                        }
                                                                        let _e5288 = phi_25480_;
                                                                        phi_25486_ = _e5288;
                                                                    }
                                                                    let _e5293 = phi_25486_;
                                                                    let _e5296 = global_6.member[(_e5108 + _e5253)];
                                                                    let _e5302 = (_e5183 - 1f);
                                                                    let _e5303 = (select(select(u32(_e5296), 0u, (_e5296 < 0f)), 4294967295u, (_e5296 > 4294967000f)) % 12u);
                                                                    let _e5306 = select(-1f, 1f, ((_e5303 & 1u) == 0u));
                                                                    let _e5309 = select(-1f, 1f, ((_e5303 & 2u) == 0u));
                                                                    if (_e5303 < 4u) {
                                                                        phi_25520_ = ((_e5306 * _e5302) + (_e5309 * _e5184));
                                                                    } else {
                                                                        if (_e5303 < 8u) {
                                                                            phi_25514_ = ((_e5306 * _e5302) + (_e5309 * _e5185));
                                                                        } else {
                                                                            phi_25514_ = ((_e5306 * _e5184) + (_e5309 * _e5185));
                                                                        }
                                                                        let _e5319 = phi_25514_;
                                                                        phi_25520_ = _e5319;
                                                                    }
                                                                    let _e5324 = phi_25520_;
                                                                    let _e5327 = global_6.member[(_e5108 + _e5234)];
                                                                    let _e5333 = (_e5184 - 1f);
                                                                    let _e5334 = (select(select(u32(_e5327), 0u, (_e5327 < 0f)), 4294967295u, (_e5327 > 4294967000f)) % 12u);
                                                                    let _e5337 = select(-1f, 1f, ((_e5334 & 1u) == 0u));
                                                                    let _e5340 = select(-1f, 1f, ((_e5334 & 2u) == 0u));
                                                                    if (_e5334 < 4u) {
                                                                        phi_25554_ = ((_e5337 * _e5183) + (_e5340 * _e5333));
                                                                    } else {
                                                                        if (_e5334 < 8u) {
                                                                            phi_25548_ = ((_e5337 * _e5183) + (_e5340 * _e5185));
                                                                        } else {
                                                                            phi_25548_ = ((_e5337 * _e5333) + (_e5340 * _e5185));
                                                                        }
                                                                        let _e5350 = phi_25548_;
                                                                        phi_25554_ = _e5350;
                                                                    }
                                                                    let _e5355 = phi_25554_;
                                                                    let _e5358 = global_6.member[(_e5108 + _e5263)];
                                                                    let _e5364 = (select(select(u32(_e5358), 0u, (_e5358 < 0f)), 4294967295u, (_e5358 > 4294967000f)) % 12u);
                                                                    let _e5367 = select(-1f, 1f, ((_e5364 & 1u) == 0u));
                                                                    let _e5370 = select(-1f, 1f, ((_e5364 & 2u) == 0u));
                                                                    if (_e5364 < 4u) {
                                                                        phi_25588_ = ((_e5367 * _e5302) + (_e5370 * _e5333));
                                                                    } else {
                                                                        if (_e5364 < 8u) {
                                                                            phi_25582_ = ((_e5367 * _e5302) + (_e5370 * _e5185));
                                                                        } else {
                                                                            phi_25582_ = ((_e5367 * _e5333) + (_e5370 * _e5185));
                                                                        }
                                                                        let _e5380 = phi_25582_;
                                                                        phi_25588_ = _e5380;
                                                                    }
                                                                    let _e5385 = phi_25588_;
                                                                    let _e5389 = global_6.member[(_e5108 + (_e5224 + 1u))];
                                                                    let _e5395 = (_e5185 - 1f);
                                                                    let _e5396 = (select(select(u32(_e5389), 0u, (_e5389 < 0f)), 4294967295u, (_e5389 > 4294967000f)) % 12u);
                                                                    let _e5399 = select(-1f, 1f, ((_e5396 & 1u) == 0u));
                                                                    let _e5402 = select(-1f, 1f, ((_e5396 & 2u) == 0u));
                                                                    if (_e5396 < 4u) {
                                                                        phi_25622_ = ((_e5399 * _e5183) + (_e5402 * _e5184));
                                                                    } else {
                                                                        if (_e5396 < 8u) {
                                                                            phi_25616_ = ((_e5399 * _e5183) + (_e5402 * _e5395));
                                                                        } else {
                                                                            phi_25616_ = ((_e5399 * _e5184) + (_e5402 * _e5395));
                                                                        }
                                                                        let _e5412 = phi_25616_;
                                                                        phi_25622_ = _e5412;
                                                                    }
                                                                    let _e5417 = phi_25622_;
                                                                    let _e5421 = global_6.member[(_e5108 + (_e5253 + 1u))];
                                                                    let _e5427 = (select(select(u32(_e5421), 0u, (_e5421 < 0f)), 4294967295u, (_e5421 > 4294967000f)) % 12u);
                                                                    let _e5430 = select(-1f, 1f, ((_e5427 & 1u) == 0u));
                                                                    let _e5433 = select(-1f, 1f, ((_e5427 & 2u) == 0u));
                                                                    if (_e5427 < 4u) {
                                                                        phi_25656_ = ((_e5430 * _e5302) + (_e5433 * _e5184));
                                                                    } else {
                                                                        if (_e5427 < 8u) {
                                                                            phi_25650_ = ((_e5430 * _e5302) + (_e5433 * _e5395));
                                                                        } else {
                                                                            phi_25650_ = ((_e5430 * _e5184) + (_e5433 * _e5395));
                                                                        }
                                                                        let _e5443 = phi_25650_;
                                                                        phi_25656_ = _e5443;
                                                                    }
                                                                    let _e5448 = phi_25656_;
                                                                    let _e5452 = global_6.member[(_e5108 + (_e5234 + 1u))];
                                                                    let _e5458 = (select(select(u32(_e5452), 0u, (_e5452 < 0f)), 4294967295u, (_e5452 > 4294967000f)) % 12u);
                                                                    let _e5461 = select(-1f, 1f, ((_e5458 & 1u) == 0u));
                                                                    let _e5464 = select(-1f, 1f, ((_e5458 & 2u) == 0u));
                                                                    if (_e5458 < 4u) {
                                                                        phi_25690_ = ((_e5461 * _e5183) + (_e5464 * _e5333));
                                                                    } else {
                                                                        if (_e5458 < 8u) {
                                                                            phi_25684_ = ((_e5461 * _e5183) + (_e5464 * _e5395));
                                                                        } else {
                                                                            phi_25684_ = ((_e5461 * _e5333) + (_e5464 * _e5395));
                                                                        }
                                                                        let _e5474 = phi_25684_;
                                                                        phi_25690_ = _e5474;
                                                                    }
                                                                    let _e5479 = phi_25690_;
                                                                    let _e5483 = global_6.member[(_e5108 + (_e5263 + 1u))];
                                                                    let _e5489 = (select(select(u32(_e5483), 0u, (_e5483 < 0f)), 4294967295u, (_e5483 > 4294967000f)) % 12u);
                                                                    let _e5492 = select(-1f, 1f, ((_e5489 & 1u) == 0u));
                                                                    let _e5495 = select(-1f, 1f, ((_e5489 & 2u) == 0u));
                                                                    if (_e5489 < 4u) {
                                                                        phi_25724_ = ((_e5492 * _e5302) + (_e5495 * _e5333));
                                                                    } else {
                                                                        if (_e5489 < 8u) {
                                                                            phi_25718_ = ((_e5492 * _e5302) + (_e5495 * _e5395));
                                                                        } else {
                                                                            phi_25718_ = ((_e5492 * _e5333) + (_e5495 * _e5395));
                                                                        }
                                                                        let _e5505 = phi_25718_;
                                                                        phi_25724_ = _e5505;
                                                                    }
                                                                    let _e5510 = phi_25724_;
                                                                    let _e5513 = (_e5293 + (_e5192 * (_e5324 - _e5293)));
                                                                    let _e5519 = (_e5417 + (_e5192 * (_e5448 - _e5417)));
                                                                    let _e5525 = (_e5513 + (_e5199 * ((_e5355 + (_e5192 * (_e5385 - _e5355))) - _e5513)));
                                                                    phi_7189_ = (_e5137 + 1u);
                                                                    phi_7192_ = (_e5139 + _e5141);
                                                                    phi_7194_ = (_e5141 * 0.5f);
                                                                    phi_7196_ = (_e5143 + ((_e5525 + ((((_e5185 * _e5185) * _e5185) * ((_e5185 * ((_e5185 * 6f) - 15f)) + 10f)) * ((_e5519 + (_e5199 * ((_e5479 + (_e5192 * (_e5510 - _e5479))) - _e5519))) - _e5525))) * _e5141));
                                                                    phi_7198_ = vec3<f32>(((((_e5111 * _e5145.x) + (_e5114 * _e5145.y)) + (_e5117 * _e5145.z)) * 2.03f), ((((_e5120 * _e5145.x) + (_e5123 * _e5145.y)) + (_e5126 * _e5145.z)) * 2.03f), ((((_e5129 * _e5145.x) + (_e5132 * _e5145.y)) + (_e5135 * _e5145.z)) * 2.03f));
                                                                } else {
                                                                    phi_7189_ = u32();
                                                                    phi_7192_ = f32();
                                                                    phi_7194_ = f32();
                                                                    phi_7196_ = f32();
                                                                    phi_7198_ = vec3<f32>();
                                                                }
                                                                let _e5557 = phi_7189_;
                                                                let _e5559 = phi_7192_;
                                                                let _e5561 = phi_7194_;
                                                                let _e5563 = phi_7196_;
                                                                let _e5565 = phi_7198_;
                                                                phi_7188_ = _e5557;
                                                                phi_7191_ = _e5559;
                                                                phi_7193_ = _e5561;
                                                                phi_7195_ = _e5563;
                                                                phi_7197_ = _e5565;
                                                                if !(_e5146) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5568 = local_33;
                                                            if (_e5568 > 0f) {
                                                                let _e5571 = local_34;
                                                                let _e5573 = local_35;
                                                                phi_7611_ = (_e5571 / _e5573);
                                                            } else {
                                                                phi_7611_ = 0f;
                                                            }
                                                            let _e5576 = phi_7611_;
                                                            let _e5578 = (1f + (1.2f * _e5576));
                                                            if (_e5578 != _e5578) {
                                                                phi_25770_ = true;
                                                            } else {
                                                                phi_25770_ = (0f >= _e5578);
                                                            }
                                                            let _e5582 = phi_25770_;
                                                            let _e5587 = ((_e5088 - 1.15f) * -2.8571432f);
                                                            let _e5589 = select(_e5587, 0f, (_e5587 < 0f));
                                                            let _e5591 = select(_e5589, 1f, (_e5589 > 1f));
                                                            let _e5597 = (1f / _e5100);
                                                            if (_e5597 != _e5597) {
                                                                phi_25785_ = true;
                                                            } else {
                                                                phi_25785_ = (4f <= _e5597);
                                                            }
                                                            let _e5601 = phi_25785_;
                                                            let _e5604 = (((exp((_e5088 * -3.5714285f)) * ((_e5591 * _e5591) * (3f - (2f * _e5591)))) * select(_e5597, 4f, _e5601)) * 0.12f);
                                                            let _e5610 = sqrt((((_e5049 * _e5049) + (_e5050 * _e5050)) + (_e5051 * _e5051)));
                                                            let _e5617 = (((_e5049 * _e5015) + (_e5050 * _e5018)) + (_e5051 * _e5021));
                                                            let _e5618 = (1f - _e5100);
                                                            let _e5634 = ((_e5604 * (0.6f + (0.4f * (1f - _e5102)))) + exp((_e5610 * -16.666668f)));
                                                            let _e5640 = (((_e5604 * _e5102) * 1.8f) * select(_e5578, 0f, _e5582));
                                                            let _e5647 = (_e4970 * (1f - ((((0.75f * _e5618) * _e5618) * exp(((_e5617 * -28.571428f) * (_e5617 * 28.571428f)))) * exp((_e5610 * -1.4285715f)))));
                                                            phi_7704_ = vec3<f32>();
                                                            phi_7705_ = vec3<f32>((((1.1633643f * _e5634) + (0.86689025f * _e5640)) * _e5647), (((0.97722596f * _e5634) + (1.0155001f * _e5640)) * _e5647), (((0.74455315f * _e5634) + (1.2384146f * _e5640)) * _e5647));
                                                        }
                                                        let _e5674 = phi_7704_;
                                                        let _e5676 = phi_7705_;
                                                        phi_7706_ = _e5674;
                                                        phi_7707_ = _e5676;
                                                    }
                                                    let _e5678 = phi_7706_;
                                                    let _e5680 = phi_7707_;
                                                    phi_6558_ = (_e4944 + select(_e5680, _e5678, vec3(select(_e4952, true, _e4976))));
                                                    phi_6561_ = (_e4946 + 1u);
                                                } else {
                                                    phi_6558_ = vec3<f32>();
                                                    phi_6561_ = u32();
                                                }
                                                let _e5687 = phi_6558_;
                                                let _e5689 = phi_6561_;
                                                phi_6557_ = _e5687;
                                                phi_6560_ = _e5689;
                                                if !(_e4947) {
                                                    break;
                                                }
                                            }
                                            phi_34207_ = _e4076;
                                            if _e4076 {
                                                break;
                                            }
                                            phi_7726_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7729_ = 0u;
                                            phi_7731_ = _e4942;
                                            loop {
                                                let _e5692 = phi_7726_;
                                                let _e5694 = phi_7729_;
                                                let _e5696 = phi_7731_;
                                                local_45 = _e5696;
                                                local_46 = _e5696;
                                                local_47 = _e5696;
                                                local_48 = _e5696;
                                                local_52 = _e5692;
                                                local_53 = _e5692;
                                                local_54 = _e5692;
                                                let _e5697 = (_e5694 < bitcast<u32>(_e3805));
                                                if _e5697 {
                                                    let _e5699 = (bitcast<u32>(_e3801) + (_e5694 * 18u));
                                                    let _e5701 = global_6.member[_e5699];
                                                    let _e5706 = select(select(u32(_e5701), 0u, (_e5701 < 0f)), 4294967295u, (_e5701 > 4294967000f));
                                                    let _e5709 = global_6.member[(_e5699 + 1u)];
                                                    let _e5712 = global_6.member[(_e5699 + 2u)];
                                                    let _e5715 = global_6.member[(_e5699 + 3u)];
                                                    let _e5718 = global_6.member[(_e5699 + 4u)];
                                                    let _e5721 = global_6.member[(_e5699 + 5u)];
                                                    let _e5724 = global_6.member[(_e5699 + 6u)];
                                                    let _e5727 = global_6.member[(_e5699 + 7u)];
                                                    let _e5730 = global_6.member[(_e5699 + 8u)];
                                                    let _e5733 = global_6.member[(_e5699 + 9u)];
                                                    let _e5736 = global_6.member[(_e5699 + 10u)];
                                                    let _e5739 = global_6.member[(_e5699 + 11u)];
                                                    let _e5742 = global_6.member[(_e5699 + 12u)];
                                                    let _e5745 = global_6.member[(_e5699 + 13u)];
                                                    let _e5748 = global_6.member[(_e5699 + 14u)];
                                                    let _e5751 = global_6.member[(_e5699 + 15u)];
                                                    let _e5754 = global_6.member[(_e5699 + 16u)];
                                                    let _e5757 = global_6.member[(_e5699 + 17u)];
                                                    let _e5762 = (((_e343.member_1.x * _e5709) + (_e343.member_1.y * _e5712)) + (_e343.member_1.z * _e5715));
                                                    if (_e5762 < _e5721) {
                                                        phi_9048_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5767 = ((_e343.member_1.x / _e5762) - _e5709);
                                                        let _e5768 = ((_e343.member_1.y / _e5762) - _e5712);
                                                        let _e5769 = ((_e343.member_1.z / _e5762) - _e5715);
                                                        let _e5776 = ((((_e5767 * _e5730) + (_e5768 * _e5733)) + (_e5769 * _e5736)) / (_e5718 * _e5748));
                                                        let _e5782 = ((((_e5767 * _e5739) + (_e5768 * _e5742)) + (_e5769 * _e5745)) / _e5718);
                                                        let _e5786 = sqrt(((_e5776 * _e5776) + (_e5782 * _e5782)));
                                                        let _e5789 = vec3<f32>((_e5776 * 2.2f), (_e5782 * 2.2f), 0f);
                                                        let _e5790 = (_e5706 == 0u);
                                                        if _e5790 {
                                                            phi_8033_ = 6u;
                                                            phi_8034_ = 1.2f;
                                                            phi_8035_ = _e5789;
                                                        } else {
                                                            let _e5791 = (_e5706 == 1u);
                                                            if _e5791 {
                                                                phi_8032_ = vec3<f32>((_e5776 * 1.54f), (_e5782 * 1.54f), 0f);
                                                            } else {
                                                                phi_8032_ = _e5789;
                                                            }
                                                            let _e5796 = phi_8032_;
                                                            phi_8033_ = select(6u, 5u, _e5791);
                                                            phi_8034_ = select(1f, 0.8f, _e5791);
                                                            phi_8035_ = _e5796;
                                                        }
                                                        let _e5800 = phi_8033_;
                                                        let _e5802 = phi_8034_;
                                                        let _e5804 = phi_8035_;
                                                        if _e5790 {
                                                            phi_8052_ = vec3<f32>(((_e5776 * 3.7400002f) + -7f), ((_e5782 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8052_ = vec3<f32>((_e5776 * 3.3000002f), (_e5782 * 3.3000002f), 0f);
                                                        }
                                                        let _e5814 = phi_8052_;
                                                        phi_8054_ = 0u;
                                                        phi_8057_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8059_ = 0f;
                                                        phi_8061_ = 0f;
                                                        phi_8063_ = 0f;
                                                        loop {
                                                            let _e5817 = phi_8054_;
                                                            let _e5819 = phi_8057_;
                                                            let _e5821 = phi_8059_;
                                                            let _e5823 = phi_8061_;
                                                            let _e5825 = phi_8063_;
                                                            local_39 = _e5825;
                                                            local_40 = _e5823;
                                                            local_41 = _e5821;
                                                            local_42 = _e5823;
                                                            local_43 = _e5823;
                                                            local_44 = _e5821;
                                                            let _e5826 = (_e5817 < 6u);
                                                            if _e5826 {
                                                                let _e5827 = (_e5817 == 0u);
                                                                if _e5827 {
                                                                    phi_8105_ = true;
                                                                    phi_8106_ = false;
                                                                    phi_8107_ = false;
                                                                } else {
                                                                    if (_e5817 <= 4u) {
                                                                        phi_8083_ = (_e5706 == 2u);
                                                                    } else {
                                                                        phi_8083_ = true;
                                                                    }
                                                                    let _e5831 = phi_8083_;
                                                                    if _e5831 {
                                                                        let _e5832 = (_e5817 == 5u);
                                                                        if _e5832 {
                                                                            if _e5790 {
                                                                                phi_8099_ = false;
                                                                            } else {
                                                                                phi_8099_ = (_e5706 == 2u);
                                                                            }
                                                                            let _e5835 = phi_8099_;
                                                                            phi_8101_ = _e5835;
                                                                        } else {
                                                                            phi_8101_ = false;
                                                                        }
                                                                        let _e5837 = phi_8101_;
                                                                        phi_8103_ = select(false, _e5790, _e5832);
                                                                        phi_8104_ = _e5837;
                                                                    } else {
                                                                        phi_8103_ = false;
                                                                        phi_8104_ = false;
                                                                    }
                                                                    let _e5840 = phi_8103_;
                                                                    let _e5842 = phi_8104_;
                                                                    phi_8105_ = select(true, false, _e5831);
                                                                    phi_8106_ = _e5840;
                                                                    phi_8107_ = _e5842;
                                                                }
                                                                let _e5845 = phi_8105_;
                                                                let _e5847 = phi_8106_;
                                                                let _e5849 = phi_8107_;
                                                                if select(select(_e5849, false, _e5845), true, select(_e5847, true, _e5845)) {
                                                                    if _e5827 {
                                                                        phi_8154_ = 3u;
                                                                        phi_8155_ = vec3<f32>(((_e5776 * 1.7600001f) + 3f), ((_e5782 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5817 < 4u) {
                                                                            let _e5866 = (_e5817 - 1u);
                                                                            if (_e5866 == 0u) {
                                                                                phi_25804_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_25804_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5866 == 1u)));
                                                                            }
                                                                            let _e5872 = phi_25804_;
                                                                            phi_8152_ = 4u;
                                                                            phi_8153_ = (_e5804 + _e5872);
                                                                        } else {
                                                                            let _e5854 = (_e5817 == 4u);
                                                                            if _e5854 {
                                                                                phi_8151_ = (_e5804 + vec3<f32>((_e5819.x * _e5802), (_e5819.y * _e5802), (_e5819.z * _e5802)));
                                                                            } else {
                                                                                phi_8151_ = _e5814;
                                                                            }
                                                                            let _e5864 = phi_8151_;
                                                                            phi_8152_ = select(select(4u, 5u, _e5790), _e5800, _e5854);
                                                                            phi_8153_ = _e5864;
                                                                        }
                                                                        let _e5875 = phi_8152_;
                                                                        let _e5877 = phi_8153_;
                                                                        phi_8154_ = _e5875;
                                                                        phi_8155_ = _e5877;
                                                                    }
                                                                    let _e5884 = phi_8154_;
                                                                    let _e5886 = phi_8155_;
                                                                    let _e5888 = global_6.member[_e3790];
                                                                    let _e5889 = bitcast<u32>(_e5888);
                                                                    let _e5892 = global_6.member[(_e1301 + 11u)];
                                                                    let _e5895 = global_6.member[(_e1301 + 12u)];
                                                                    let _e5898 = global_6.member[(_e1301 + 13u)];
                                                                    let _e5901 = global_6.member[(_e1301 + 14u)];
                                                                    let _e5904 = global_6.member[(_e1301 + 15u)];
                                                                    let _e5907 = global_6.member[(_e1301 + 16u)];
                                                                    let _e5910 = global_6.member[(_e1301 + 17u)];
                                                                    let _e5913 = global_6.member[(_e1301 + 18u)];
                                                                    let _e5916 = global_6.member[(_e1301 + 19u)];
                                                                    phi_8290_ = 0u;
                                                                    phi_8293_ = 0f;
                                                                    phi_8295_ = 1f;
                                                                    phi_8297_ = 0f;
                                                                    phi_8299_ = _e5886;
                                                                    loop {
                                                                        let _e5918 = phi_8290_;
                                                                        let _e5920 = phi_8293_;
                                                                        let _e5922 = phi_8295_;
                                                                        let _e5924 = phi_8297_;
                                                                        let _e5926 = phi_8299_;
                                                                        local_36 = _e5920;
                                                                        local_37 = _e5924;
                                                                        local_38 = _e5920;
                                                                        let _e5927 = (_e5918 < _e5884);
                                                                        if _e5927 {
                                                                            let _e5929 = (_e5926.x + _e5751);
                                                                            let _e5931 = (_e5926.y + _e5754);
                                                                            let _e5933 = (_e5926.z + _e5757);
                                                                            let _e5934 = floor(_e5929);
                                                                            let _e5935 = floor(_e5931);
                                                                            let _e5936 = floor(_e5933);
                                                                            let _e5945 = bitcast<u32>((select(0i, select(select(i32(_e5934), i32(-2147483648), (_e5934 < -2147483600f)), 2147483647i, (_e5934 > 2147483500f)), (_e5934 == _e5934)) & 255i));
                                                                            let _e5954 = bitcast<u32>((select(0i, select(select(i32(_e5935), i32(-2147483648), (_e5935 < -2147483600f)), 2147483647i, (_e5935 > 2147483500f)), (_e5935 == _e5935)) & 255i));
                                                                            let _e5963 = bitcast<u32>((select(0i, select(select(i32(_e5936), i32(-2147483648), (_e5936 < -2147483600f)), 2147483647i, (_e5936 > 2147483500f)), (_e5936 == _e5936)) & 255i));
                                                                            let _e5964 = (_e5929 - _e5934);
                                                                            let _e5965 = (_e5931 - _e5935);
                                                                            let _e5966 = (_e5933 - _e5936);
                                                                            let _e5973 = (((_e5964 * _e5964) * _e5964) * ((_e5964 * ((_e5964 * 6f) - 15f)) + 10f));
                                                                            let _e5980 = (((_e5965 * _e5965) * _e5965) * ((_e5965 * ((_e5965 * 6f) - 15f)) + 10f));
                                                                            let _e5990 = global_6.member[(_e5889 + _e5945)];
                                                                            let _e5996 = (select(select(u32(_e5990), 0u, (_e5990 < 0f)), 4294967295u, (_e5990 > 4294967000f)) + _e5954);
                                                                            let _e5999 = global_6.member[(_e5889 + _e5996)];
                                                                            let _e6005 = (select(select(u32(_e5999), 0u, (_e5999 < 0f)), 4294967295u, (_e5999 > 4294967000f)) + _e5963);
                                                                            let _e6009 = global_6.member[(_e5889 + (_e5996 + 1u))];
                                                                            let _e6015 = (select(select(u32(_e6009), 0u, (_e6009 < 0f)), 4294967295u, (_e6009 > 4294967000f)) + _e5963);
                                                                            let _e6019 = global_6.member[(_e5889 + (_e5945 + 1u))];
                                                                            let _e6025 = (select(select(u32(_e6019), 0u, (_e6019 < 0f)), 4294967295u, (_e6019 > 4294967000f)) + _e5954);
                                                                            let _e6028 = global_6.member[(_e5889 + _e6025)];
                                                                            let _e6034 = (select(select(u32(_e6028), 0u, (_e6028 < 0f)), 4294967295u, (_e6028 > 4294967000f)) + _e5963);
                                                                            let _e6038 = global_6.member[(_e5889 + (_e6025 + 1u))];
                                                                            let _e6044 = (select(select(u32(_e6038), 0u, (_e6038 < 0f)), 4294967295u, (_e6038 > 4294967000f)) + _e5963);
                                                                            let _e6047 = global_6.member[(_e5889 + _e6005)];
                                                                            let _e6053 = (select(select(u32(_e6047), 0u, (_e6047 < 0f)), 4294967295u, (_e6047 > 4294967000f)) % 12u);
                                                                            let _e6056 = select(-1f, 1f, ((_e6053 & 1u) == 0u));
                                                                            let _e6059 = select(-1f, 1f, ((_e6053 & 2u) == 0u));
                                                                            if (_e6053 < 4u) {
                                                                                phi_25865_ = ((_e6056 * _e5964) + (_e6059 * _e5965));
                                                                            } else {
                                                                                if (_e6053 < 8u) {
                                                                                    phi_25859_ = ((_e6056 * _e5964) + (_e6059 * _e5966));
                                                                                } else {
                                                                                    phi_25859_ = ((_e6056 * _e5965) + (_e6059 * _e5966));
                                                                                }
                                                                                let _e6069 = phi_25859_;
                                                                                phi_25865_ = _e6069;
                                                                            }
                                                                            let _e6074 = phi_25865_;
                                                                            let _e6077 = global_6.member[(_e5889 + _e6034)];
                                                                            let _e6083 = (_e5964 - 1f);
                                                                            let _e6084 = (select(select(u32(_e6077), 0u, (_e6077 < 0f)), 4294967295u, (_e6077 > 4294967000f)) % 12u);
                                                                            let _e6087 = select(-1f, 1f, ((_e6084 & 1u) == 0u));
                                                                            let _e6090 = select(-1f, 1f, ((_e6084 & 2u) == 0u));
                                                                            if (_e6084 < 4u) {
                                                                                phi_25899_ = ((_e6087 * _e6083) + (_e6090 * _e5965));
                                                                            } else {
                                                                                if (_e6084 < 8u) {
                                                                                    phi_25893_ = ((_e6087 * _e6083) + (_e6090 * _e5966));
                                                                                } else {
                                                                                    phi_25893_ = ((_e6087 * _e5965) + (_e6090 * _e5966));
                                                                                }
                                                                                let _e6100 = phi_25893_;
                                                                                phi_25899_ = _e6100;
                                                                            }
                                                                            let _e6105 = phi_25899_;
                                                                            let _e6108 = global_6.member[(_e5889 + _e6015)];
                                                                            let _e6114 = (_e5965 - 1f);
                                                                            let _e6115 = (select(select(u32(_e6108), 0u, (_e6108 < 0f)), 4294967295u, (_e6108 > 4294967000f)) % 12u);
                                                                            let _e6118 = select(-1f, 1f, ((_e6115 & 1u) == 0u));
                                                                            let _e6121 = select(-1f, 1f, ((_e6115 & 2u) == 0u));
                                                                            if (_e6115 < 4u) {
                                                                                phi_25933_ = ((_e6118 * _e5964) + (_e6121 * _e6114));
                                                                            } else {
                                                                                if (_e6115 < 8u) {
                                                                                    phi_25927_ = ((_e6118 * _e5964) + (_e6121 * _e5966));
                                                                                } else {
                                                                                    phi_25927_ = ((_e6118 * _e6114) + (_e6121 * _e5966));
                                                                                }
                                                                                let _e6131 = phi_25927_;
                                                                                phi_25933_ = _e6131;
                                                                            }
                                                                            let _e6136 = phi_25933_;
                                                                            let _e6139 = global_6.member[(_e5889 + _e6044)];
                                                                            let _e6145 = (select(select(u32(_e6139), 0u, (_e6139 < 0f)), 4294967295u, (_e6139 > 4294967000f)) % 12u);
                                                                            let _e6148 = select(-1f, 1f, ((_e6145 & 1u) == 0u));
                                                                            let _e6151 = select(-1f, 1f, ((_e6145 & 2u) == 0u));
                                                                            if (_e6145 < 4u) {
                                                                                phi_25967_ = ((_e6148 * _e6083) + (_e6151 * _e6114));
                                                                            } else {
                                                                                if (_e6145 < 8u) {
                                                                                    phi_25961_ = ((_e6148 * _e6083) + (_e6151 * _e5966));
                                                                                } else {
                                                                                    phi_25961_ = ((_e6148 * _e6114) + (_e6151 * _e5966));
                                                                                }
                                                                                let _e6161 = phi_25961_;
                                                                                phi_25967_ = _e6161;
                                                                            }
                                                                            let _e6166 = phi_25967_;
                                                                            let _e6170 = global_6.member[(_e5889 + (_e6005 + 1u))];
                                                                            let _e6176 = (_e5966 - 1f);
                                                                            let _e6177 = (select(select(u32(_e6170), 0u, (_e6170 < 0f)), 4294967295u, (_e6170 > 4294967000f)) % 12u);
                                                                            let _e6180 = select(-1f, 1f, ((_e6177 & 1u) == 0u));
                                                                            let _e6183 = select(-1f, 1f, ((_e6177 & 2u) == 0u));
                                                                            if (_e6177 < 4u) {
                                                                                phi_26001_ = ((_e6180 * _e5964) + (_e6183 * _e5965));
                                                                            } else {
                                                                                if (_e6177 < 8u) {
                                                                                    phi_25995_ = ((_e6180 * _e5964) + (_e6183 * _e6176));
                                                                                } else {
                                                                                    phi_25995_ = ((_e6180 * _e5965) + (_e6183 * _e6176));
                                                                                }
                                                                                let _e6193 = phi_25995_;
                                                                                phi_26001_ = _e6193;
                                                                            }
                                                                            let _e6198 = phi_26001_;
                                                                            let _e6202 = global_6.member[(_e5889 + (_e6034 + 1u))];
                                                                            let _e6208 = (select(select(u32(_e6202), 0u, (_e6202 < 0f)), 4294967295u, (_e6202 > 4294967000f)) % 12u);
                                                                            let _e6211 = select(-1f, 1f, ((_e6208 & 1u) == 0u));
                                                                            let _e6214 = select(-1f, 1f, ((_e6208 & 2u) == 0u));
                                                                            if (_e6208 < 4u) {
                                                                                phi_26035_ = ((_e6211 * _e6083) + (_e6214 * _e5965));
                                                                            } else {
                                                                                if (_e6208 < 8u) {
                                                                                    phi_26029_ = ((_e6211 * _e6083) + (_e6214 * _e6176));
                                                                                } else {
                                                                                    phi_26029_ = ((_e6211 * _e5965) + (_e6214 * _e6176));
                                                                                }
                                                                                let _e6224 = phi_26029_;
                                                                                phi_26035_ = _e6224;
                                                                            }
                                                                            let _e6229 = phi_26035_;
                                                                            let _e6233 = global_6.member[(_e5889 + (_e6015 + 1u))];
                                                                            let _e6239 = (select(select(u32(_e6233), 0u, (_e6233 < 0f)), 4294967295u, (_e6233 > 4294967000f)) % 12u);
                                                                            let _e6242 = select(-1f, 1f, ((_e6239 & 1u) == 0u));
                                                                            let _e6245 = select(-1f, 1f, ((_e6239 & 2u) == 0u));
                                                                            if (_e6239 < 4u) {
                                                                                phi_26069_ = ((_e6242 * _e5964) + (_e6245 * _e6114));
                                                                            } else {
                                                                                if (_e6239 < 8u) {
                                                                                    phi_26063_ = ((_e6242 * _e5964) + (_e6245 * _e6176));
                                                                                } else {
                                                                                    phi_26063_ = ((_e6242 * _e6114) + (_e6245 * _e6176));
                                                                                }
                                                                                let _e6255 = phi_26063_;
                                                                                phi_26069_ = _e6255;
                                                                            }
                                                                            let _e6260 = phi_26069_;
                                                                            let _e6264 = global_6.member[(_e5889 + (_e6044 + 1u))];
                                                                            let _e6270 = (select(select(u32(_e6264), 0u, (_e6264 < 0f)), 4294967295u, (_e6264 > 4294967000f)) % 12u);
                                                                            let _e6273 = select(-1f, 1f, ((_e6270 & 1u) == 0u));
                                                                            let _e6276 = select(-1f, 1f, ((_e6270 & 2u) == 0u));
                                                                            if (_e6270 < 4u) {
                                                                                phi_26103_ = ((_e6273 * _e6083) + (_e6276 * _e6114));
                                                                            } else {
                                                                                if (_e6270 < 8u) {
                                                                                    phi_26097_ = ((_e6273 * _e6083) + (_e6276 * _e6176));
                                                                                } else {
                                                                                    phi_26097_ = ((_e6273 * _e6114) + (_e6276 * _e6176));
                                                                                }
                                                                                let _e6286 = phi_26097_;
                                                                                phi_26103_ = _e6286;
                                                                            }
                                                                            let _e6291 = phi_26103_;
                                                                            let _e6294 = (_e6074 + (_e5973 * (_e6105 - _e6074)));
                                                                            let _e6300 = (_e6198 + (_e5973 * (_e6229 - _e6198)));
                                                                            let _e6306 = (_e6294 + (_e5980 * ((_e6136 + (_e5973 * (_e6166 - _e6136))) - _e6294)));
                                                                            phi_8291_ = (_e5918 + 1u);
                                                                            phi_8294_ = (_e5920 + _e5922);
                                                                            phi_8296_ = (_e5922 * 0.5f);
                                                                            phi_8298_ = (_e5924 + ((_e6306 + ((((_e5966 * _e5966) * _e5966) * ((_e5966 * ((_e5966 * 6f) - 15f)) + 10f)) * ((_e6300 + (_e5980 * ((_e6260 + (_e5973 * (_e6291 - _e6260))) - _e6300))) - _e6306))) * _e5922));
                                                                            phi_8300_ = vec3<f32>(((((_e5892 * _e5926.x) + (_e5895 * _e5926.y)) + (_e5898 * _e5926.z)) * 2.03f), ((((_e5901 * _e5926.x) + (_e5904 * _e5926.y)) + (_e5907 * _e5926.z)) * 2.03f), ((((_e5910 * _e5926.x) + (_e5913 * _e5926.y)) + (_e5916 * _e5926.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8291_ = u32();
                                                                            phi_8294_ = f32();
                                                                            phi_8296_ = f32();
                                                                            phi_8298_ = f32();
                                                                            phi_8300_ = vec3<f32>();
                                                                        }
                                                                        let _e6338 = phi_8291_;
                                                                        let _e6340 = phi_8294_;
                                                                        let _e6342 = phi_8296_;
                                                                        let _e6344 = phi_8298_;
                                                                        let _e6346 = phi_8300_;
                                                                        phi_8290_ = _e6338;
                                                                        phi_8293_ = _e6340;
                                                                        phi_8295_ = _e6342;
                                                                        phi_8297_ = _e6344;
                                                                        phi_8299_ = _e6346;
                                                                        if !(_e5927) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6349 = local_36;
                                                                    if (_e6349 > 0f) {
                                                                        let _e6352 = local_37;
                                                                        let _e6354 = local_38;
                                                                        phi_8713_ = (_e6352 / _e6354);
                                                                    } else {
                                                                        phi_8713_ = 0f;
                                                                    }
                                                                    let _e6357 = phi_8713_;
                                                                    if _e5827 {
                                                                        phi_8748_ = _e5819;
                                                                        phi_8749_ = _e5821;
                                                                        phi_8750_ = _e5823;
                                                                    } else {
                                                                        if (_e5817 == 1u) {
                                                                            phi_8745_ = vec3<f32>(_e6357, _e5819.y, _e5819.z);
                                                                            phi_8746_ = _e5821;
                                                                            phi_8747_ = _e5823;
                                                                        } else {
                                                                            if (_e5817 == 2u) {
                                                                                phi_8742_ = vec3<f32>(_e5819.x, _e6357, _e5819.z);
                                                                                phi_8743_ = _e5821;
                                                                                phi_8744_ = _e5823;
                                                                            } else {
                                                                                if (_e5817 == 3u) {
                                                                                    phi_8739_ = vec3<f32>(_e5819.x, _e5819.y, _e6357);
                                                                                    phi_8740_ = _e5821;
                                                                                    phi_8741_ = _e5823;
                                                                                } else {
                                                                                    let _e6361 = (_e5817 == 4u);
                                                                                    phi_8739_ = _e5819;
                                                                                    phi_8740_ = select(_e6357, _e5821, _e6361);
                                                                                    phi_8741_ = select(_e5823, _e6357, _e6361);
                                                                                }
                                                                                let _e6369 = phi_8739_;
                                                                                let _e6371 = phi_8740_;
                                                                                let _e6373 = phi_8741_;
                                                                                phi_8742_ = _e6369;
                                                                                phi_8743_ = _e6371;
                                                                                phi_8744_ = _e6373;
                                                                            }
                                                                            let _e6379 = phi_8742_;
                                                                            let _e6381 = phi_8743_;
                                                                            let _e6383 = phi_8744_;
                                                                            phi_8745_ = _e6379;
                                                                            phi_8746_ = _e6381;
                                                                            phi_8747_ = _e6383;
                                                                        }
                                                                        let _e6389 = phi_8745_;
                                                                        let _e6391 = phi_8746_;
                                                                        let _e6393 = phi_8747_;
                                                                        phi_8748_ = _e6389;
                                                                        phi_8749_ = _e6391;
                                                                        phi_8750_ = _e6393;
                                                                    }
                                                                    let _e6395 = phi_8748_;
                                                                    let _e6397 = phi_8749_;
                                                                    let _e6399 = phi_8750_;
                                                                    phi_8752_ = _e6395;
                                                                    phi_8753_ = _e6397;
                                                                    phi_8754_ = _e6399;
                                                                    phi_8755_ = select(_e5825, _e6357, _e5827);
                                                                } else {
                                                                    phi_8752_ = _e5819;
                                                                    phi_8753_ = _e5821;
                                                                    phi_8754_ = _e5823;
                                                                    phi_8755_ = _e5825;
                                                                }
                                                                let _e6402 = phi_8752_;
                                                                let _e6404 = phi_8753_;
                                                                let _e6406 = phi_8754_;
                                                                let _e6408 = phi_8755_;
                                                                phi_8055_ = (_e5817 + 1u);
                                                                phi_8058_ = _e6402;
                                                                phi_8060_ = _e6404;
                                                                phi_8062_ = _e6406;
                                                                phi_8064_ = _e6408;
                                                            } else {
                                                                phi_8055_ = u32();
                                                                phi_8058_ = vec3<f32>();
                                                                phi_8060_ = f32();
                                                                phi_8062_ = f32();
                                                                phi_8064_ = f32();
                                                            }
                                                            let _e6411 = phi_8055_;
                                                            let _e6413 = phi_8058_;
                                                            let _e6415 = phi_8060_;
                                                            let _e6417 = phi_8062_;
                                                            let _e6419 = phi_8064_;
                                                            phi_8054_ = _e6411;
                                                            phi_8057_ = _e6413;
                                                            phi_8059_ = _e6415;
                                                            phi_8061_ = _e6417;
                                                            phi_8063_ = _e6419;
                                                            if !(_e5826) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6422 = local_39;
                                                        let _e6425 = (_e5786 * (1f + (0.45f * _e6422)));
                                                        if _e5790 {
                                                            let _e6528 = local_43;
                                                            let _e6530 = ((_e6528 - -0.25f) * 1.4285715f);
                                                            let _e6532 = select(_e6530, 0f, (_e6530 < 0f));
                                                            let _e6534 = select(_e6532, 1f, (_e6532 > 1f));
                                                            let _e6538 = ((_e6534 * _e6534) * (3f - (2f * _e6534)));
                                                            let _e6540 = ((_e6425 - 1.05f) * -1.1764706f);
                                                            let _e6542 = select(_e6540, 0f, (_e6540 < 0f));
                                                            let _e6544 = select(_e6542, 1f, (_e6542 > 1f));
                                                            let _e6548 = ((_e6544 * _e6544) * (3f - (2f * _e6544)));
                                                            let _e6549 = (_e6538 * _e6548);
                                                            let _e6551 = ((_e6425 - 0.65f) * -1.5384616f);
                                                            let _e6553 = select(_e6551, 0f, (_e6551 < 0f));
                                                            let _e6555 = select(_e6553, 1f, (_e6553 > 1f));
                                                            let _e6560 = (((_e6555 * _e6555) * (3f - (2f * _e6555))) * _e6538);
                                                            let _e6562 = local_44;
                                                            let _e6564 = ((_e6562 - 0.1f) * 2.857143f);
                                                            let _e6566 = select(_e6564, 0f, (_e6564 < 0f));
                                                            let _e6568 = select(_e6566, 1f, (_e6566 > 1f));
                                                            let _e6573 = (((_e6568 * _e6568) * (3f - (2f * _e6568))) * _e6548);
                                                            let _e6578 = ((0.8f * _e6560) * _e6560);
                                                            let _e6587 = (_e5724 * exp((-2f * _e6573)));
                                                            phi_9047_ = type_59(vec3<f32>((((3.1815925f * _e6549) + (0.14634217f * _e6578)) * _e6587), (((0.38179109f * _e6549) + (1.2439084f * _e6578)) * _e6587), (((0.69995034f * _e6549) + (1.0975662f * _e6578)) * _e6587)), (_e5727 * _e6573));
                                                        } else {
                                                            if (_e5706 == 1u) {
                                                                let _e6498 = local_42;
                                                                let _e6500 = ((_e6498 - -0.3f) * 1.25f);
                                                                let _e6502 = select(_e6500, 0f, (_e6500 < 0f));
                                                                let _e6504 = select(_e6502, 1f, (_e6502 > 1f));
                                                                let _e6510 = ((_e6425 - 1f) * -1f);
                                                                let _e6512 = select(_e6510, 0f, (_e6510 < 0f));
                                                                let _e6514 = select(_e6512, 1f, (_e6512 > 1f));
                                                                let _e6520 = (((_e6504 * _e6504) * (3f - (2f * _e6504))) * pow(((_e6514 * _e6514) * (3f - (2f * _e6514))), 1.5f));
                                                                phi_9046_ = type_59((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5724 * _e6520)), (_e5727 * _e6520));
                                                            } else {
                                                                if (_e5706 == 2u) {
                                                                    let _e6454 = local_41;
                                                                    let _e6457 = (_e5786 * (1f + (0.15f * _e6454)));
                                                                    let _e6458 = (_e6457 - 0.65f);
                                                                    let _e6462 = exp(((_e6458 * -5.5555553f) * (_e6458 * 5.5555553f)));
                                                                    let _e6464 = ((_e6457 - 0.75f) * -1.8181818f);
                                                                    let _e6466 = select(_e6464, 0f, (_e6464 < 0f));
                                                                    let _e6468 = select(_e6466, 1f, (_e6466 > 1f));
                                                                    let _e6472 = ((_e6468 * _e6468) * (3f - (2f * _e6468)));
                                                                    let _e6477 = (exp(((_e5786 * -25f) * (_e5786 * 25f))) * 3f);
                                                                    phi_9045_ = type_59(vec3<f32>(((((3.1815925f * _e6462) + (_e6472 * 0.10243952f)) + _e6477) * _e5724), ((((0.38179109f * _e6462) + (_e6472 * 0.8707359f)) + _e6477) * _e5724), ((((0.69995034f * _e6462) + (_e6472 * 0.76829636f)) + _e6477) * _e5724)), 0f);
                                                                } else {
                                                                    let _e6429 = local_40;
                                                                    let _e6431 = ((_e6429 - -0.2f) * 1.6666666f);
                                                                    let _e6433 = select(_e6431, 0f, (_e6431 < 0f));
                                                                    let _e6435 = select(_e6433, 1f, (_e6433 > 1f));
                                                                    let _e6441 = ((_e6425 - 1f) * -1.4285715f);
                                                                    let _e6443 = select(_e6441, 0f, (_e6441 < 0f));
                                                                    let _e6445 = select(_e6443, 1f, (_e6443 > 1f));
                                                                    phi_9045_ = type_59(vec3<f32>(0f, 0f, 0f), (_e5727 * (((_e6435 * _e6435) * (3f - (2f * _e6435))) * ((_e6445 * _e6445) * (3f - (2f * _e6445))))));
                                                                }
                                                                let _e6496 = phi_9045_;
                                                                phi_9046_ = _e6496;
                                                            }
                                                            let _e6526 = phi_9046_;
                                                            phi_9047_ = _e6526;
                                                        }
                                                        let _e6595 = phi_9047_;
                                                        phi_9048_ = _e6595;
                                                    }
                                                    let _e6597 = phi_9048_;
                                                    phi_7727_ = (_e5692 + _e6597.member);
                                                    phi_7730_ = (_e5694 + 1u);
                                                    phi_7732_ = (_e5696 + _e6597.member_1);
                                                } else {
                                                    phi_7727_ = vec3<f32>();
                                                    phi_7730_ = u32();
                                                    phi_7732_ = f32();
                                                }
                                                let _e6604 = phi_7727_;
                                                let _e6606 = phi_7730_;
                                                let _e6608 = phi_7732_;
                                                phi_7726_ = _e6604;
                                                phi_7729_ = _e6606;
                                                phi_7731_ = _e6608;
                                                if !(_e5697) {
                                                    break;
                                                }
                                            }
                                            phi_34207_ = _e4076;
                                            if _e4076 {
                                                break;
                                            }
                                            let _e6611 = local_45;
                                            if (_e6611 <= 0f) {
                                                phi_9077_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6614 = local_46;
                                                let _e6617 = local_47;
                                                let _e6622 = local_48;
                                                phi_9077_ = vec3<f32>(exp((_e6617 * -0.81f)), exp(-(_e6614)), exp((_e6622 * -1.25f)));
                                            }
                                            let _e6627 = phi_9077_;
                                            let _e6629 = local_49;
                                            let _e6634 = local_50;
                                            let _e6639 = local_51;
                                            let _e6644 = local_52;
                                            let _e6648 = local_53;
                                            let _e6652 = local_54;
                                            let _e6657 = global_6.member[(_e1301 + 9u)];
                                            phi_34216_ = _e4076;
                                            phi_9124_ = vec3<f32>((_e3781.x + ((((_e6629.x * _e6627.x) + _e6644.x) * _e6657) * _e3783.x)), (_e3781.y + ((((_e6634.y * _e6627.y) + _e6648.y) * _e6657) * _e3783.y)), (_e3781.z + ((((_e6639.z * _e6627.z) + _e6652.z) * _e6657) * _e3783.z)));
                                        } else {
                                            phi_34216_ = _e3779;
                                            phi_9124_ = _e3781;
                                        }
                                        let _e6675 = phi_34216_;
                                        let _e6677 = phi_9124_;
                                        phi_34215_ = _e6675;
                                        phi_9125_ = _e6677;
                                    } else {
                                        phi_34215_ = _e3779;
                                        phi_9125_ = _e3787;
                                    }
                                    let _e6679 = phi_34215_;
                                    let _e6681 = phi_9125_;
                                    phi_34214_ = _e6679;
                                    phi_9126_ = _e6681;
                                } else {
                                    phi_34214_ = _e332;
                                    phi_9126_ = _e1446;
                                }
                                let _e6683 = phi_34214_;
                                let _e6685 = phi_9126_;
                                phi_34210_ = _e6683;
                                phi_10399_ = (vec3<f32>(_e343.member_2.x, _e343.member_2.y, _e343.member_2.z) * _e6685);
                            } else {
                                let _e359 = global_1.member.member_8[1u];
                                let _e369 = global_6.member[_e359];
                                let _e372 = global_6.member[(_e359 + 1u)];
                                let _e375 = global_6.member[(_e359 + 2u)];
                                let _e378 = global_6.member[(_e359 + 3u)];
                                let _e381 = global_6.member[(_e359 + 4u)];
                                let _e384 = global_6.member[(_e359 + 5u)];
                                let _e387 = global_6.member[(_e359 + 6u)];
                                let _e390 = global_6.member[(_e359 + 7u)];
                                let _e393 = global_6.member[(_e359 + 8u)];
                                let _e396 = global_6.member[(_e359 + 9u)];
                                let _e399 = global_6.member[(_e359 + 10u)];
                                let _e402 = global_6.member[(_e359 + 11u)];
                                let _e405 = global_6.member[(_e359 + 12u)];
                                let _e408 = global_6.member[(_e359 + 13u)];
                                let _e411 = global_6.member[(_e359 + 14u)];
                                let _e414 = global_6.member[(_e359 + 15u)];
                                let _e418 = global_6.member[(_e359 + 16u)];
                                let _e421 = global_6.member[(_e359 + 17u)];
                                let _e424 = global_6.member[(_e359 + 18u)];
                                let _e427 = global_6.member[(_e359 + 19u)];
                                let _e430 = global_6.member[(_e359 + 20u)];
                                let _e433 = global_6.member[(_e359 + 29u)];
                                let _e434 = bitcast<u32>(_e433);
                                let _e437 = global_6.member[(_e359 + 30u)];
                                let _e438 = bitcast<u32>(_e437);
                                let _e446 = ((((_e343.member.x - _e369) * _e343.member_1.x) + ((_e343.member.y - _e372) * _e343.member_1.y)) + ((_e343.member.z - _e375) * _e343.member_1.z));
                                let _e458 = ((((_e343.member.x * _e343.member.x) + (_e343.member.y * _e343.member.y)) + (_e343.member.z * _e343.member.z)) - (2f * (((_e343.member.x * _e369) + (_e343.member.y * _e372)) + (_e343.member.z * _e375))));
                                let _e459 = (_e458 + _e387);
                                let _e460 = (_e446 * _e446);
                                let _e461 = (_e460 - _e459);
                                if (_e461 < 0f) {
                                    phi_22008_ = type_57(0u, type_57().member_1);
                                    phi_22009_ = type_57();
                                    phi_22010_ = true;
                                } else {
                                    if (_e446 < 0f) {
                                        phi_21980_ = -(sqrt(_e461));
                                    } else {
                                        phi_21980_ = sqrt(_e461);
                                    }
                                    let _e468 = phi_21980_;
                                    let _e470 = -((_e446 + _e468));
                                    let _e471 = (_e470 == 0f);
                                    if _e471 {
                                        phi_22003_ = type_57(1u, type_56(0f, 0f));
                                        phi_22004_ = type_57();
                                    } else {
                                        let _e472 = (_e459 / _e470);
                                        if (_e470 < _e472) {
                                            phi_21992_ = type_56(_e470, _e472);
                                        } else {
                                            phi_21992_ = type_56(_e472, _e470);
                                        }
                                        let _e477 = phi_21992_;
                                        phi_22003_ = type_57();
                                        phi_22004_ = type_57(1u, _e477);
                                    }
                                    let _e480 = phi_22003_;
                                    let _e482 = phi_22004_;
                                    phi_22008_ = _e480;
                                    phi_22009_ = _e482;
                                    phi_22010_ = _e471;
                                }
                                let _e487 = phi_22008_;
                                let _e489 = phi_22009_;
                                let _e491 = phi_22010_;
                                if _e491 {
                                    phi_22014_ = _e487;
                                } else {
                                    phi_22014_ = _e489;
                                }
                                let _e493 = phi_22014_;
                                if (bitcast<i32>(_e493.member) != 0i) {
                                    phi_21901_ = type_57(0u, _e493.member_1);
                                } else {
                                    phi_21901_ = type_57(1u, type_57().member_1);
                                }
                                let _e503 = phi_21901_;
                                if (bitcast<i32>(_e503.member) != 0i) {
                                    phi_21948_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e503.member_1.member_1 <= 0f) {
                                        phi_21944_ = type_57(0u, type_57().member_1);
                                    } else {
                                        if (_e503.member_1.member != _e503.member_1.member) {
                                            phi_22052_ = true;
                                        } else {
                                            phi_22052_ = (0f >= _e503.member_1.member);
                                        }
                                        let _e515 = phi_22052_;
                                        let _e516 = select(_e503.member_1.member, 0f, _e515);
                                        let _e517 = (_e458 + _e384);
                                        if (_e517 > 0f) {
                                            let _e519 = (_e460 - _e517);
                                            if (_e519 < 0f) {
                                                phi_22160_ = type_57(0u, type_57().member_1);
                                                phi_22161_ = type_57();
                                                phi_22162_ = true;
                                            } else {
                                                if (_e446 < 0f) {
                                                    phi_22132_ = -(sqrt(_e519));
                                                } else {
                                                    phi_22132_ = sqrt(_e519);
                                                }
                                                let _e526 = phi_22132_;
                                                let _e528 = -((_e446 + _e526));
                                                let _e529 = (_e528 == 0f);
                                                if _e529 {
                                                    phi_22155_ = type_57(1u, type_56(0f, 0f));
                                                    phi_22156_ = type_57();
                                                } else {
                                                    let _e530 = (_e517 / _e528);
                                                    if (_e528 < _e530) {
                                                        phi_22144_ = type_56(_e528, _e530);
                                                    } else {
                                                        phi_22144_ = type_56(_e530, _e528);
                                                    }
                                                    let _e535 = phi_22144_;
                                                    phi_22155_ = type_57();
                                                    phi_22156_ = type_57(1u, _e535);
                                                }
                                                let _e538 = phi_22155_;
                                                let _e540 = phi_22156_;
                                                phi_22160_ = _e538;
                                                phi_22161_ = _e540;
                                                phi_22162_ = _e529;
                                            }
                                            let _e545 = phi_22160_;
                                            let _e547 = phi_22161_;
                                            let _e549 = phi_22162_;
                                            if _e549 {
                                                phi_22166_ = _e545;
                                            } else {
                                                phi_22166_ = _e547;
                                            }
                                            let _e551 = phi_22166_;
                                            if (bitcast<i32>(_e551.member) != 0i) {
                                                phi_22100_ = select(_e551.member_1.member_1, _e551.member_1.member, (_e551.member_1.member >= 0f));
                                            } else {
                                                phi_22100_ = -1f;
                                            }
                                            let _e562 = phi_22100_;
                                            if (_e562 >= 0f) {
                                                if (_e503.member_1.member_1 != _e503.member_1.member_1) {
                                                    phi_22204_ = true;
                                                } else {
                                                    phi_22204_ = (_e562 <= _e503.member_1.member_1);
                                                }
                                                let _e567 = phi_22204_;
                                                phi_21929_ = select(_e503.member_1.member_1, _e562, _e567);
                                            } else {
                                                phi_21929_ = _e503.member_1.member_1;
                                            }
                                            let _e570 = phi_21929_;
                                            phi_21931_ = _e570;
                                        } else {
                                            phi_21931_ = _e503.member_1.member_1;
                                        }
                                        let _e572 = phi_21931_;
                                        if (_e572 <= _e516) {
                                            phi_21940_ = type_57(0u, type_57().member_1);
                                        } else {
                                            phi_21940_ = type_57(1u, type_56(_e516, _e572));
                                        }
                                        let _e580 = phi_21940_;
                                        phi_21944_ = _e580;
                                    }
                                    let _e585 = phi_21944_;
                                    phi_21948_ = _e585;
                                }
                                let _e590 = phi_21948_;
                                if (bitcast<i32>(_e590.member) != 0i) {
                                    let _e596 = (_e590.member_1.member < _e343.member.w);
                                    if _e596 {
                                        if (_e590.member_1.member_1 != _e590.member_1.member_1) {
                                            phi_22219_ = true;
                                        } else {
                                            phi_22219_ = (_e343.member.w <= _e590.member_1.member_1);
                                        }
                                        let _e602 = phi_22219_;
                                        let _e603 = select(_e590.member_1.member_1, _e343.member.w, _e602);
                                        phi_9520_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9523_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9525_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9527_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9529_ = 0u;
                                        loop {
                                            let _e605 = phi_9520_;
                                            let _e607 = phi_9523_;
                                            let _e609 = phi_9525_;
                                            let _e611 = phi_9527_;
                                            let _e613 = phi_9529_;
                                            local = _e609;
                                            local_1 = _e607;
                                            local_2 = _e605;
                                            local_3 = _e605;
                                            local_4 = _e605;
                                            local_5 = _e611;
                                            local_6 = _e611;
                                            local_7 = _e611;
                                            let _e614 = (_e613 < 32u);
                                            if _e614 {
                                                let _e615 = -(_e446);
                                                if (_e590.member_1.member <= _e603) {
                                                } else {
                                                    phi_33729_ = true;
                                                    break;
                                                }
                                                let _e618 = select(_e615, _e590.member_1.member, (_e615 < _e590.member_1.member));
                                                let _e620 = select(_e618, _e603, (_e618 > _e603));
                                                let _e621 = (_e620 - _e590.member_1.member);
                                                let _e622 = (_e603 - _e620);
                                                let _e623 = (_e603 - _e590.member_1.member);
                                                let _e624 = (0.000000001f * _e623);
                                                if (_e621 <= _e624) {
                                                    let _e699 = (f32(_e613) * 0.03125f);
                                                    let _e702 = (f32((_e613 + 1u)) * 0.03125f);
                                                    let _e704 = ((_e699 + _e702) * 0.5f);
                                                    phi_9630_ = type_56((_e590.member_1.member + ((_e623 * _e704) * _e704)), (_e623 * ((_e702 * _e702) - (_e699 * _e699))));
                                                } else {
                                                    if (_e622 <= _e624) {
                                                        let _e682 = (f32((31u - _e613)) * 0.03125f);
                                                        let _e685 = (f32((32u - _e613)) * 0.03125f);
                                                        let _e687 = ((_e682 + _e685) * 0.5f);
                                                        phi_9629_ = type_56((_e603 - ((_e623 * _e687) * _e687)), (_e623 * ((_e685 * _e685) - (_e682 * _e682))));
                                                    } else {
                                                        let _e630 = round(((32f * _e621) / (_e621 + _e622)));
                                                        let _e635 = select(select(u32(_e630), 0u, (_e630 < 0f)), 4294967295u, (_e630 > 4294967000f));
                                                        if (_e635 < 1u) {
                                                            phi_9613_ = 1u;
                                                        } else {
                                                            phi_9613_ = select(_e635, 31u, (_e635 > 31u));
                                                        }
                                                        let _e640 = phi_9613_;
                                                        if (_e613 < _e640) {
                                                            let _e661 = ((_e640 - 1u) - _e613);
                                                            let _e663 = f32(_e640);
                                                            let _e664 = (f32(_e661) / _e663);
                                                            let _e667 = (f32((_e661 + 1u)) / _e663);
                                                            let _e669 = ((_e664 + _e667) * 0.5f);
                                                            phi_9628_ = type_56((_e620 - ((_e621 * _e669) * _e669)), (_e621 * ((_e667 * _e667) - (_e664 * _e664))));
                                                        } else {
                                                            let _e643 = (_e613 - _e640);
                                                            let _e645 = f32((32u - _e640));
                                                            let _e646 = (f32(_e643) / _e645);
                                                            let _e649 = (f32((_e643 + 1u)) / _e645);
                                                            let _e651 = ((_e646 + _e649) * 0.5f);
                                                            phi_9628_ = type_56((_e620 + ((_e622 * _e651) * _e651)), (_e622 * ((_e649 * _e649) - (_e646 * _e646))));
                                                        }
                                                        let _e679 = phi_9628_;
                                                        phi_9629_ = _e679;
                                                    }
                                                    let _e697 = phi_9629_;
                                                    phi_9630_ = _e697;
                                                }
                                                let _e714 = phi_9630_;
                                                let _e720 = (_e343.member.x + (_e343.member_1.x * _e714.member));
                                                let _e721 = (_e343.member.y + (_e343.member_1.y * _e714.member));
                                                let _e722 = (_e343.member.z + (_e343.member_1.z * _e714.member));
                                                let _e735 = ((((_e720 * _e720) + (_e721 * _e721)) + (_e722 * _e722)) - (2f * (((_e720 * _e369) + (_e721 * _e372)) + (_e722 * _e375))));
                                                let _e737 = (_e720 - _e369);
                                                let _e738 = (_e721 - _e372);
                                                let _e739 = (_e722 - _e375);
                                                let _e745 = sqrt((((_e737 * _e737) + (_e738 * _e738)) + (_e739 * _e739)));
                                                let _e747 = ((_e735 + _e384) / (_e745 + _e378));
                                                let _e751 = (1f - (abs((_e747 - 25000f)) * 0.00006666667f));
                                                if (_e751 != _e751) {
                                                    phi_22419_ = true;
                                                } else {
                                                    phi_22419_ = (0f >= _e751);
                                                }
                                                let _e755 = phi_22419_;
                                                let _e757 = -(_e747);
                                                let _e760 = (exp((_e757 / _e399)) * _e714.member_1);
                                                let _e763 = (exp((_e757 / _e402)) * _e714.member_1);
                                                let _e765 = (_e760 * 0.5f);
                                                let _e766 = (_e763 * 0.5f);
                                                let _e767 = ((select(_e751, 0f, _e755) * _e714.member_1) * 0.5f);
                                                let _e769 = (_e611.x + _e765);
                                                let _e771 = (_e611.y + _e766);
                                                let _e773 = (_e611.z + _e767);
                                                let _e778 = (_e418 * 1.1f);
                                                let _e779 = (_e778 * _e771);
                                                let _e798 = (_e418 * _e763);
                                                if (_e438 == 4294967295u) {
                                                    phi_9958_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e806 = (1f / _e745);
                                                    let _e814 = ((((_e737 * _e806) * _e390) + ((_e738 * _e806) * _e393)) + ((_e739 * _e806) * _e396));
                                                    if (_e747 != _e747) {
                                                        phi_22546_ = true;
                                                    } else {
                                                        phi_22546_ = (0f >= _e747);
                                                    }
                                                    let _e818 = phi_22546_;
                                                    let _e822 = sqrt((select(_e747, 0f, _e818) / (_e381 - _e378)));
                                                    if (_e822 != _e822) {
                                                        phi_22561_ = true;
                                                    } else {
                                                        phi_22561_ = (1f <= _e822);
                                                    }
                                                    let _e826 = phi_22561_;
                                                    let _e828 = (select(_e822, 1f, _e826) * 31f);
                                                    let _e830 = select(_e814, -1f, (_e814 < -1f));
                                                    let _e834 = ((select(_e830, 1f, (_e830 > 1f)) + 1f) * 15.5f);
                                                    let _e839 = select(select(u32(_e828), 0u, (_e828 < 0f)), 4294967295u, (_e828 > 4294967000f));
                                                    let _e841 = select(_e839, 30u, (30u < _e839));
                                                    let _e846 = select(select(u32(_e834), 0u, (_e834 < 0f)), 4294967295u, (_e834 > 4294967000f));
                                                    let _e848 = select(_e846, 30u, (30u < _e846));
                                                    let _e850 = (_e828 - f32(_e841));
                                                    let _e852 = (_e834 - f32(_e848));
                                                    let _e856 = (_e438 + (3u * ((_e841 * 32u) + _e848)));
                                                    let _e861 = (_e438 + (3u * (((_e841 + 1u) * 32u) + _e848)));
                                                    let _e863 = global_6.member[_e856];
                                                    let _e866 = global_6.member[(_e856 + 1u)];
                                                    let _e869 = global_6.member[(_e856 + 2u)];
                                                    let _e870 = (1f - _e852);
                                                    let _e876 = global_6.member[(_e856 + 3u)];
                                                    let _e879 = global_6.member[(_e856 + 4u)];
                                                    let _e882 = global_6.member[(_e856 + 5u)];
                                                    let _e889 = (1f - _e850);
                                                    let _e894 = global_6.member[_e861];
                                                    let _e897 = global_6.member[(_e861 + 1u)];
                                                    let _e900 = global_6.member[(_e861 + 2u)];
                                                    let _e906 = global_6.member[(_e861 + 3u)];
                                                    let _e909 = global_6.member[(_e861 + 4u)];
                                                    let _e912 = global_6.member[(_e861 + 5u)];
                                                    phi_9958_ = vec3<f32>(((((_e863 * _e870) + (_e876 * _e852)) * _e889) + (((_e894 * _e870) + (_e906 * _e852)) * _e850)), ((((_e866 * _e870) + (_e879 * _e852)) * _e889) + (((_e897 * _e870) + (_e909 * _e852)) * _e850)), ((((_e869 * _e870) + (_e882 * _e852)) * _e889) + (((_e900 * _e870) + (_e912 * _e852)) * _e850)));
                                                }
                                                let _e927 = phi_9958_;
                                                let _e941 = (_e735 + _e387);
                                                if (_e941 > 0f) {
                                                    let _e947 = (((_e737 * _e390) + (_e738 * _e393)) + (_e739 * _e396));
                                                    let _e949 = ((_e947 * _e947) - _e941);
                                                    if (_e947 >= 0f) {
                                                        phi_10032_ = vec3<f32>();
                                                        phi_10033_ = true;
                                                    } else {
                                                        let _e951 = (_e949 <= 0f);
                                                        if _e951 {
                                                            phi_10031_ = vec3<f32>();
                                                        } else {
                                                            let _e954 = (-(_e947) - sqrt(_e949));
                                                            phi_10031_ = vec3<f32>((_e720 + (_e390 * _e954)), (_e721 + (_e393 * _e954)), (_e722 + (_e396 * _e954)));
                                                        }
                                                        let _e963 = phi_10031_;
                                                        phi_10032_ = _e963;
                                                        phi_10033_ = _e951;
                                                    }
                                                    let _e965 = phi_10032_;
                                                    let _e967 = phi_10033_;
                                                    phi_10038_ = _e965;
                                                    phi_10039_ = select(true, false, _e967);
                                                } else {
                                                    phi_10038_ = vec3<f32>(_e720, _e721, _e722);
                                                    phi_10039_ = true;
                                                }
                                                let _e970 = phi_10038_;
                                                let _e972 = phi_10039_;
                                                if _e972 {
                                                    let _e974 = (_e970.x - _e369);
                                                    let _e976 = (_e970.y - _e372);
                                                    let _e978 = (_e970.z - _e375);
                                                    let _e984 = sqrt((((_e974 * _e974) + (_e976 * _e976)) + (_e978 * _e978)));
                                                    let _e985 = (1f / _e984);
                                                    let _e1003 = ((((((_e970.x * _e970.x) + (_e970.y * _e970.y)) + (_e970.z * _e970.z)) - (2f * (((_e970.x * _e369) + (_e970.y * _e372)) + (_e970.z * _e375)))) + _e384) / (_e984 + _e378));
                                                    if (_e1003 != _e1003) {
                                                        phi_22685_ = true;
                                                    } else {
                                                        phi_22685_ = (0f >= _e1003);
                                                    }
                                                    let _e1007 = phi_22685_;
                                                    let _e1011 = sqrt((select(_e1003, 0f, _e1007) / (_e381 - _e378)));
                                                    if (_e1011 != _e1011) {
                                                        phi_22700_ = true;
                                                    } else {
                                                        phi_22700_ = (1f <= _e1011);
                                                    }
                                                    let _e1015 = phi_22700_;
                                                    let _e1017 = (select(_e1011, 1f, _e1015) * 63f);
                                                    let _e1022 = ((((_e974 * _e985) * _e390) + ((_e976 * _e985) * _e393)) + ((_e978 * _e985) * _e396));
                                                    let _e1024 = select(_e1022, -1f, (_e1022 < -1f));
                                                    let _e1028 = ((select(_e1024, 1f, (_e1024 > 1f)) + 1f) * 127.5f);
                                                    let _e1033 = select(select(u32(_e1017), 0u, (_e1017 < 0f)), 4294967295u, (_e1017 > 4294967000f));
                                                    let _e1035 = select(_e1033, 62u, (62u < _e1033));
                                                    let _e1040 = select(select(u32(_e1028), 0u, (_e1028 < 0f)), 4294967295u, (_e1028 > 4294967000f));
                                                    let _e1042 = select(_e1040, 254u, (254u < _e1040));
                                                    let _e1044 = (_e1017 - f32(_e1035));
                                                    let _e1046 = (_e1028 - f32(_e1042));
                                                    let _e1050 = (_e434 + (3u * ((_e1035 * 256u) + _e1042)));
                                                    let _e1055 = (_e434 + (3u * (((_e1035 + 1u) * 256u) + _e1042)));
                                                    let _e1057 = global_6.member[_e1050];
                                                    let _e1060 = global_6.member[(_e1050 + 1u)];
                                                    let _e1063 = global_6.member[(_e1050 + 2u)];
                                                    let _e1064 = (1f - _e1046);
                                                    let _e1070 = global_6.member[(_e1050 + 3u)];
                                                    let _e1073 = global_6.member[(_e1050 + 4u)];
                                                    let _e1076 = global_6.member[(_e1050 + 5u)];
                                                    let _e1083 = (1f - _e1044);
                                                    let _e1088 = global_6.member[_e1055];
                                                    let _e1091 = global_6.member[(_e1055 + 1u)];
                                                    let _e1094 = global_6.member[(_e1055 + 2u)];
                                                    let _e1100 = global_6.member[(_e1055 + 3u)];
                                                    let _e1103 = global_6.member[(_e1055 + 4u)];
                                                    let _e1106 = global_6.member[(_e1055 + 5u)];
                                                    phi_10330_ = vec3<f32>(((((_e1057 * _e1064) + (_e1070 * _e1046)) * _e1083) + (((_e1088 * _e1064) + (_e1100 * _e1046)) * _e1044)), ((((_e1060 * _e1064) + (_e1073 * _e1046)) * _e1083) + (((_e1091 * _e1064) + (_e1103 * _e1046)) * _e1044)), ((((_e1063 * _e1064) + (_e1076 * _e1046)) * _e1083) + (((_e1094 * _e1064) + (_e1106 * _e1046)) * _e1044)));
                                                } else {
                                                    phi_10330_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1121 = phi_10330_;
                                                let _e1122 = (vec3<f32>(_e769, _e771, _e773) + _e1121);
                                                let _e1128 = (_e778 * _e1122.y);
                                                let _e1140 = exp(-((((_e408 * _e1122.x) + _e1128) + (_e424 * _e1122.z))));
                                                let _e1142 = exp(-((((_e411 * _e1122.x) + _e1128) + (_e427 * _e1122.z))));
                                                let _e1144 = exp(-((((_e414 * _e1122.x) + _e1128) + (_e430 * _e1122.z))));
                                                phi_9521_ = vec3<f32>((_e605.x + ((exp(-((((_e408 * _e769) + _e779) + (_e424 * _e773)))) * ((_e408 * _e760) + _e798)) * _e927.x)), (_e605.y + ((exp(-((((_e411 * _e769) + _e779) + (_e427 * _e773)))) * ((_e411 * _e760) + _e798)) * _e927.y)), (_e605.z + ((exp(-((((_e414 * _e769) + _e779) + (_e430 * _e773)))) * ((_e414 * _e760) + _e798)) * _e927.z)));
                                                phi_9524_ = vec3<f32>((_e607.x + (_e1140 * _e763)), (_e607.y + (_e1142 * _e763)), (_e607.z + (_e1144 * _e763)));
                                                phi_9526_ = vec3<f32>((_e609.x + (_e1140 * _e760)), (_e609.y + (_e1142 * _e760)), (_e609.z + (_e1144 * _e760)));
                                                phi_9528_ = vec3<f32>((_e769 + _e765), (_e771 + _e766), (_e773 + _e767));
                                                phi_9530_ = (_e613 + 1u);
                                            } else {
                                                phi_9521_ = vec3<f32>();
                                                phi_9524_ = vec3<f32>();
                                                phi_9526_ = vec3<f32>();
                                                phi_9528_ = vec3<f32>();
                                                phi_9530_ = u32();
                                            }
                                            let _e1171 = phi_9521_;
                                            let _e1173 = phi_9524_;
                                            let _e1175 = phi_9526_;
                                            let _e1177 = phi_9528_;
                                            let _e1179 = phi_9530_;
                                            phi_9520_ = _e1171;
                                            phi_9523_ = _e1173;
                                            phi_9525_ = _e1175;
                                            phi_9527_ = _e1177;
                                            phi_9529_ = _e1179;
                                            phi_33729_ = _e332;
                                            if !(_e614) {
                                                break;
                                            }
                                        }
                                        let _e1182 = phi_33729_;
                                        phi_34207_ = _e1182;
                                        if _e1182 {
                                            break;
                                        }
                                        let _e1184 = local;
                                        let _e1185 = (_e1184 * vec3<f32>(_e408, _e411, _e414));
                                        let _e1187 = local_1;
                                        let _e1188 = (_e1187 * _e418);
                                        let _e1193 = (((_e343.member_1.x * _e390) + (_e343.member_1.y * _e393)) + (_e343.member_1.z * _e396));
                                        let _e1195 = (1f + (_e1193 * _e1193));
                                        let _e1196 = (0.059683103f * _e1195);
                                        let _e1203 = (_e421 * _e421);
                                        let _e1214 = ((0.119366206f * ((1f - _e1203) * _e1195)) / ((2f + _e1203) * pow(((1f + _e1203) - ((2f * _e421) * _e1193)), 1.5f)));
                                        let _e1225 = local_2;
                                        let _e1229 = local_3;
                                        let _e1233 = local_4;
                                        let _e1242 = local_5;
                                        let _e1249 = local_6;
                                        let _e1251 = ((_e418 * 1.1f) * _e1249.y);
                                        let _e1256 = local_7;
                                        phi_34213_ = _e1182;
                                        phi_10387_ = type_60(vec3<f32>(exp(-((((_e408 * _e1242.x) + _e1251) + (_e424 * _e1256.z)))), exp(-((((_e411 * _e1242.x) + _e1251) + (_e427 * _e1256.z)))), exp(-((((_e414 * _e1242.x) + _e1251) + (_e430 * _e1256.z))))), (vec3<f32>(((((_e1185.x * _e1196) + (_e1188.x * _e1214)) + _e1225.x) * _e405), ((((_e1185.y * _e1196) + (_e1188.y * _e1214)) + _e1229.y) * _e405), ((((_e1185.z * _e1196) + (_e1188.z * _e1214)) + _e1233.z) * _e405)) * 0.38317f));
                                    } else {
                                        phi_34213_ = _e332;
                                        phi_10387_ = type_60();
                                    }
                                    let _e1273 = phi_34213_;
                                    let _e1275 = phi_10387_;
                                    phi_34212_ = _e1273;
                                    phi_10389_ = _e1275;
                                    phi_10390_ = select(true, false, _e596);
                                } else {
                                    phi_34212_ = _e332;
                                    phi_10389_ = type_60();
                                    phi_10390_ = true;
                                }
                                let _e1278 = phi_34212_;
                                let _e1280 = phi_10389_;
                                let _e1282 = phi_10390_;
                                if _e1282 {
                                    phi_10395_ = type_60(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10395_ = _e1280;
                                }
                                let _e1284 = phi_10395_;
                                phi_34210_ = _e1278;
                                phi_10399_ = (vec3<f32>(_e343.member_2.x, _e343.member_2.y, _e343.member_2.z) * _e1284.member_1);
                            }
                            let _e6689 = phi_34210_;
                            let _e6691 = phi_10399_;
                            phi_34209_ = _e6689;
                            phi_10400_ = _e6691;
                        }
                        let _e6693 = phi_34209_;
                        let _e6695 = phi_10400_;
                        phi_34208_ = _e6693;
                        phi_518_ = (_e334 + _e6695);
                        phi_521_ = (_e336 + 1u);
                    } else {
                        phi_34208_ = _e332;
                        phi_518_ = vec3<f32>();
                        phi_521_ = u32();
                    }
                    let _e6699 = phi_34208_;
                    let _e6701 = phi_518_;
                    let _e6703 = phi_521_;
                    phi_33746_ = _e6699;
                    phi_517_ = _e6701;
                    phi_520_ = _e6703;
                    phi_34207_ = _e6699;
                    if !(_e337) {
                        break;
                    }
                }
                let _e6706 = phi_34207_;
                if _e6706 {
                    break;
                }
                let _e6708 = local_55;
                let _e6712 = local_56;
                let _e6717 = local_57;
                if ((((_e6708.x * _e6708.x) + (_e6712.y * _e6712.y)) + (_e6717.z * _e6717.z)) > 0f) {
                    if (_e326.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6727 = global_7.member[_e326.member_1].member_3[0u];
                    global_7.member[_e326.member_1].member_3[0u] = (_e6727 + _e6708.x);
                    let _e6733 = global_7.member[_e326.member_1].member_3[1u];
                    global_7.member[_e326.member_1].member_3[1u] = (_e6733 + _e6712.y);
                    let _e6739 = global_7.member[_e326.member_1].member_3[2u];
                    global_7.member[_e326.member_1].member_3[2u] = (_e6739 + _e6717.z);
                    let _e6745 = global_7.member[_e326.member_1].member_3[3u];
                    global_7.member[_e326.member_1].member_3[3u] = _e6745;
                }
            }
            break;
        }
    }
    return;
}

fn function_1() {
    switch bitcast<i32>(0u) {
        default: {
            let _e280 = global;
            let _e288 = (_e280.x + (_e280.y * 4194240u));
            let _e291 = atomicLoad((&global_11.member[11u]));
            if (_e288 >= _e291) {
            } else {
                let _e296 = global_1.member.member[0u];
                let _e300 = global_1.member.member[1u];
                let _e305 = global_1.member.member_7[3u];
                let _e312 = atomicLoad((&global_11.member[((12u + (2u * ((_e296 * _e300) * _e305))) + _e288)]));
                if (_e312 < arrayLength((&global_10.member))) {
                } else {
                    break;
                }
                let _e316 = global_10.member[_e312];
                if (_e288 < arrayLength((&global_12.member))) {
                } else {
                    break;
                }
                global_12.member[_e288] = type_34(_e316.member, _e316.member_1, _e316.member_2, _e316.member_3, vec4<u32>(_e316.member_4.x, _e316.member_4.y, _e316.member_4.z, _e312), _e316.member_5);
            }
            break;
        }
    }
    return;
}

fn function_2() {
    var local_67: array<u32, 64>;
    var local_68: array<f32, 64>;
    var phi_11267_: type_48;
    var phi_26298_: f32;
    var phi_26310_: type_56;
    var phi_26321_: type_57;
    var phi_26322_: type_57;
    var phi_26326_: type_57;
    var phi_26327_: type_57;
    var phi_26328_: bool;
    var phi_26332_: type_57;
    var phi_26219_: type_57;
    var phi_26370_: bool;
    var phi_26450_: f32;
    var phi_26462_: type_56;
    var phi_26473_: type_57;
    var phi_26474_: type_57;
    var phi_26478_: type_57;
    var phi_26479_: type_57;
    var phi_26480_: bool;
    var phi_26484_: type_57;
    var phi_26418_: f32;
    var phi_26522_: bool;
    var phi_26247_: f32;
    var phi_26249_: f32;
    var phi_26258_: type_57;
    var phi_26262_: type_57;
    var phi_26266_: type_57;
    var phi_26537_: bool;
    var phi_12796_: vec3<f32>;
    var phi_12799_: u32;
    var phi_12883_: u32;
    var phi_12898_: type_56;
    var phi_12899_: type_56;
    var phi_12900_: type_56;
    var phi_26736_: bool;
    var phi_12797_: vec3<f32>;
    var phi_12800_: u32;
    var phi_34218_: bool;
    var local_69: vec3<f32>;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var phi_36358_: bool;
    var phi_12925_: vec3<f32>;
    var phi_36357_: bool;
    var phi_12927_: vec3<f32>;
    var phi_12928_: bool;
    var phi_36355_: bool;
    var phi_12935_: vec3<f32>;
    var phi_12936_: type_41;
    var phi_36354_: bool;
    var phi_12937_: vec3<f32>;
    var phi_12938_: type_41;
    var phi_13217_: type_60;
    var phi_13184_: vec3<f32>;
    var phi_13185_: vec3<f32>;
    var phi_13218_: type_60;
    var phi_13341_: i32;
    var phi_13344_: u32;
    var phi_13398_: bool;
    var phi_13404_: bool;
    var phi_13411_: u32;
    var phi_13413_: i32;
    var phi_13414_: u32;
    var phi_13342_: i32;
    var phi_13345_: u32;
    var phi_13416_: bool;
    var local_72: u32;
    var phi_13970_: vec3<f32>;
    var phi_13984_: vec3<f32>;
    var phi_26803_: bool;
    var phi_14017_: f32;
    var phi_14125_: vec3<f32>;
    var phi_14131_: f32;
    var phi_26876_: bool;
    var phi_26865_: f32;
    var phi_14212_: vec3<f32>;
    var phi_14213_: vec3<f32>;
    var phi_14214_: vec3<f32>;
    var phi_34352_: u32;
    var phi_14216_: vec3<f32>;
    var phi_14217_: vec3<f32>;
    var phi_14218_: vec3<f32>;
    var phi_14219_: bool;
    var phi_14234_: vec3<f32>;
    var phi_14235_: vec3<f32>;
    var phi_34350_: u32;
    var phi_14237_: vec3<f32>;
    var phi_14238_: vec3<f32>;
    var phi_14239_: vec3<f32>;
    var phi_34349_: u32;
    var phi_14240_: vec3<f32>;
    var phi_14241_: vec3<f32>;
    var phi_14242_: vec3<f32>;
    var phi_14243_: bool;
    var phi_34348_: u32;
    var phi_14244_: vec3<f32>;
    var phi_14245_: vec3<f32>;
    var phi_14246_: vec3<f32>;
    var phi_14247_: bool;
    var phi_14248_: bool;
    var phi_26949_: bool;
    var phi_27111_: bool;
    var phi_27126_: bool;
    var phi_27141_: bool;
    var phi_27168_: bool;
    var phi_27183_: bool;
    var phi_27198_: bool;
    var phi_27225_: bool;
    var phi_27240_: bool;
    var phi_27255_: bool;
    var phi_14427_: f32;
    var phi_14435_: f32;
    var phi_14443_: f32;
    var phi_27452_: bool;
    var phi_27441_: f32;
    var phi_14550_: vec3<f32>;
    var phi_27480_: bool;
    var phi_14551_: vec3<f32>;
    var phi_14552_: vec3<f32>;
    var phi_14553_: vec3<f32>;
    var phi_34736_: vec3<f32>;
    var phi_34659_: vec3<f32>;
    var phi_34580_: u32;
    var phi_34498_: f32;
    var phi_14555_: vec3<f32>;
    var phi_14556_: vec3<f32>;
    var phi_14557_: bool;
    var phi_27495_: bool;
    var phi_27546_: bool;
    var phi_27535_: f32;
    var phi_34707_: vec3<f32>;
    var phi_14090_: vec3<f32>;
    var phi_35059_: vec3<f32>;
    var phi_34853_: f32;
    var phi_34706_: vec3<f32>;
    var phi_34394_: u32;
    var phi_14092_: vec3<f32>;
    var phi_14093_: bool;
    var phi_35005_: vec3<f32>;
    var phi_34813_: f32;
    var phi_34734_: vec3<f32>;
    var phi_34657_: vec3<f32>;
    var phi_34578_: u32;
    var phi_34496_: f32;
    var phi_34419_: u32;
    var phi_34345_: u32;
    var phi_14565_: vec3<f32>;
    var phi_14566_: vec3<f32>;
    var phi_14567_: vec3<f32>;
    var phi_14568_: bool;
    var phi_27612_: bool;
    var phi_14637_: vec3<f32>;
    var phi_34402_: u32;
    var phi_14639_: vec3<f32>;
    var phi_14640_: bool;
    var phi_34401_: u32;
    var phi_14641_: vec3<f32>;
    var phi_14642_: bool;
    var phi_34399_: u32;
    var phi_14648_: vec3<f32>;
    var phi_14649_: bool;
    var phi_14683_: type_69;
    var phi_34404_: u32;
    var phi_14685_: vec3<f32>;
    var phi_14686_: type_69;
    var phi_14687_: bool;
    var phi_14693_: type_69;
    var phi_34408_: u32;
    var phi_14694_: type_69;
    var phi_35002_: vec3<f32>;
    var phi_34810_: f32;
    var phi_34731_: vec3<f32>;
    var phi_34654_: vec3<f32>;
    var phi_34575_: u32;
    var phi_34493_: f32;
    var phi_34416_: u32;
    var phi_34407_: u32;
    var phi_14696_: type_69;
    var phi_14697_: bool;
    var phi_36331_: bool;
    var phi_34406_: u32;
    var phi_14708_: vec3<f32>;
    var phi_14711_: type_41;
    var phi_14713_: u32;
    var phi_14715_: u32;
    var phi_17879_: u32;
    var phi_17882_: u32;
    var phi_17880_: u32;
    var phi_17883_: u32;
    var local_73: u32;
    var phi_17920_: u32;
    var phi_17923_: u32;
    var phi_17921_: u32;
    var phi_17924_: u32;
    var local_74: u32;
    var local_75: u32;
    var phi_18000_: f32;
    var local_76: u32;
    var local_77: u32;
    var phi_18049_: f32;
    var phi_18205_: f32;
    var phi_18212_: vec3<f32>;
    var phi_36751_: u32;
    var phi_18214_: f32;
    var phi_18215_: vec3<f32>;
    var phi_18216_: vec3<f32>;
    var phi_16877_: u32;
    var phi_16880_: u32;
    var phi_16878_: u32;
    var phi_16881_: u32;
    var local_78: u32;
    var local_79: u32;
    var phi_17650_: f32;
    var phi_17653_: f32;
    var phi_27885_: bool;
    var phi_27866_: f32;
    var phi_17654_: f32;
    var phi_27900_: bool;
    var phi_27923_: bool;
    var phi_27938_: bool;
    var phi_17163_: f32;
    var phi_27953_: bool;
    var phi_27968_: bool;
    var phi_27983_: bool;
    var phi_28048_: bool;
    var phi_28136_: bool;
    var phi_28117_: f32;
    var phi_17228_: type_72;
    var phi_17656_: type_72;
    var phi_17832_: bool;
    var phi_17841_: vec3<f32>;
    var phi_17843_: vec3<f32>;
    var phi_17844_: bool;
    var phi_17849_: f32;
    var phi_17850_: vec3<f32>;
    var phi_36759_: u32;
    var phi_17851_: f32;
    var phi_17852_: vec3<f32>;
    var phi_17853_: vec3<f32>;
    var phi_17854_: f32;
    var phi_36758_: u32;
    var phi_17855_: f32;
    var phi_17856_: vec3<f32>;
    var phi_17857_: vec3<f32>;
    var phi_17858_: f32;
    var phi_36750_: u32;
    var phi_18217_: f32;
    var phi_18218_: vec3<f32>;
    var phi_18219_: vec3<f32>;
    var phi_18220_: f32;
    var phi_28151_: bool;
    var phi_28261_: f32;
    var phi_28289_: f32;
    var phi_28317_: f32;
    var phi_28451_: bool;
    var phi_28466_: bool;
    var phi_28481_: bool;
    var phi_28508_: bool;
    var phi_28523_: bool;
    var phi_28538_: bool;
    var phi_28565_: bool;
    var phi_28580_: bool;
    var phi_28595_: bool;
    var phi_15049_: f32;
    var phi_15057_: f32;
    var phi_15065_: f32;
    var phi_28792_: bool;
    var phi_28781_: f32;
    var phi_15172_: vec3<f32>;
    var phi_28820_: bool;
    var phi_15173_: vec3<f32>;
    var phi_15185_: type_59;
    var phi_15186_: type_59;
    var phi_15187_: bool;
    var phi_15192_: type_59;
    var phi_28857_: bool;
    var phi_28887_: bool;
    var phi_28876_: f32;
    var phi_28926_: vec3<f32>;
    var phi_15220_: type_59;
    var phi_15221_: type_59;
    var phi_28948_: vec3<f32>;
    var phi_15222_: type_59;
    var phi_15223_: type_59;
    var phi_15224_: bool;
    var phi_15228_: type_59;
    var phi_28980_: bool;
    var phi_29051_: bool;
    var phi_16048_: f32;
    var phi_29066_: bool;
    var phi_29081_: bool;
    var phi_16521_: vec3<f32>;
    var phi_16522_: vec3<f32>;
    var phi_16523_: bool;
    var phi_16528_: vec3<f32>;
    var phi_16529_: bool;
    var phi_29205_: bool;
    var phi_29220_: bool;
    var phi_16820_: vec3<f32>;
    var phi_16822_: vec3<f32>;
    var phi_16849_: vec3<f32>;
    var phi_16850_: vec3<f32>;
    var phi_15666_: vec3<f32>;
    var phi_15667_: vec3<f32>;
    var phi_15668_: bool;
    var phi_15673_: vec3<f32>;
    var phi_15674_: bool;
    var phi_29382_: bool;
    var phi_29397_: bool;
    var phi_15965_: vec3<f32>;
    var phi_15967_: vec3<f32>;
    var phi_36793_: u32;
    var phi_16851_: f32;
    var phi_16852_: vec3<f32>;
    var phi_16853_: vec3<f32>;
    var phi_36792_: u32;
    var phi_16854_: f32;
    var phi_16855_: vec3<f32>;
    var phi_16856_: vec3<f32>;
    var phi_36749_: u32;
    var phi_18221_: f32;
    var phi_18222_: vec3<f32>;
    var phi_18223_: vec3<f32>;
    var phi_18224_: f32;
    var phi_14811_: f32;
    var phi_14812_: f32;
    var phi_36748_: u32;
    var phi_18225_: f32;
    var phi_18226_: vec3<f32>;
    var phi_18227_: vec3<f32>;
    var phi_18228_: f32;
    var phi_29450_: bool;
    var phi_29560_: f32;
    var phi_29588_: f32;
    var phi_29616_: f32;
    var phi_29750_: bool;
    var phi_29765_: bool;
    var phi_29780_: bool;
    var phi_29807_: bool;
    var phi_29822_: bool;
    var phi_29837_: bool;
    var phi_29864_: bool;
    var phi_29879_: bool;
    var phi_29894_: bool;
    var phi_18451_: f32;
    var phi_18459_: f32;
    var phi_18467_: f32;
    var phi_30091_: bool;
    var phi_30080_: f32;
    var phi_18574_: vec3<f32>;
    var phi_30119_: bool;
    var phi_18575_: vec3<f32>;
    var phi_18587_: type_59;
    var phi_18588_: type_59;
    var phi_18589_: bool;
    var phi_18594_: type_59;
    var phi_30156_: bool;
    var phi_30186_: bool;
    var phi_30175_: f32;
    var phi_30225_: vec3<f32>;
    var phi_18622_: type_59;
    var phi_18623_: type_59;
    var phi_30247_: vec3<f32>;
    var phi_18624_: type_59;
    var phi_18625_: type_59;
    var phi_18626_: bool;
    var phi_18630_: type_59;
    var phi_30261_: f32;
    var phi_30264_: f32;
    var phi_18639_: f32;
    var phi_18672_: u32;
    var phi_18675_: u32;
    var phi_18677_: u32;
    var phi_18679_: u32;
    var phi_18681_: u32;
    var phi_18683_: vec4<f32>;
    var phi_18685_: f32;
    var phi_19072_: u32;
    var phi_19074_: u32;
    var phi_19075_: u32;
    var phi_19077_: bool;
    var phi_19078_: bool;
    var phi_19079_: u32;
    var phi_19080_: u32;
    var phi_19081_: u32;
    var phi_19082_: bool;
    var phi_19083_: bool;
    var phi_19084_: bool;
    var phi_30362_: bool;
    var phi_30392_: bool;
    var phi_30339_: type_56;
    var phi_30342_: type_56;
    var phi_30343_: bool;
    var phi_30348_: type_56;
    var phi_30449_: bool;
    var phi_30464_: bool;
    var phi_30479_: bool;
    var phi_30494_: bool;
    var phi_30426_: type_56;
    var phi_30429_: type_56;
    var phi_30430_: bool;
    var phi_30435_: type_56;
    var phi_30536_: bool;
    var phi_30551_: bool;
    var phi_30566_: bool;
    var phi_30581_: bool;
    var phi_30513_: type_56;
    var phi_30516_: type_56;
    var phi_30517_: bool;
    var phi_30522_: type_56;
    var phi_30317_: bool;
    var phi_30320_: bool;
    var phi_19113_: f32;
    var phi_30683_: bool;
    var phi_30713_: bool;
    var phi_30660_: type_56;
    var phi_30663_: type_56;
    var phi_30664_: bool;
    var phi_30669_: type_56;
    var phi_30770_: bool;
    var phi_30785_: bool;
    var phi_30800_: bool;
    var phi_30815_: bool;
    var phi_30747_: type_56;
    var phi_30750_: type_56;
    var phi_30751_: bool;
    var phi_30756_: type_56;
    var phi_30857_: bool;
    var phi_30872_: bool;
    var phi_30887_: bool;
    var phi_30902_: bool;
    var phi_30834_: type_56;
    var phi_30837_: type_56;
    var phi_30838_: bool;
    var phi_30843_: type_56;
    var phi_30638_: bool;
    var phi_30641_: bool;
    var phi_19122_: f32;
    var phi_19133_: u32;
    var phi_19144_: u32;
    var phi_19183_: u32;
    var phi_19184_: u32;
    var phi_19195_: u32;
    var phi_19196_: u32;
    var phi_19197_: u32;
    var phi_19198_: u32;
    var phi_19200_: u32;
    var phi_19201_: u32;
    var phi_19203_: u32;
    var phi_19204_: u32;
    var phi_19205_: bool;
    var phi_19206_: bool;
    var phi_18998_: vec4<f32>;
    var phi_18999_: vec4<f32>;
    var phi_19000_: bool;
    var phi_19004_: vec4<f32>;
    var phi_19005_: bool;
    var phi_18910_: bool;
    var phi_18947_: bool;
    var phi_18953_: f32;
    var phi_18963_: vec4<f32>;
    var phi_18964_: vec4<f32>;
    var phi_18965_: bool;
    var phi_18969_: vec4<f32>;
    var phi_18970_: bool;
    var phi_18974_: vec4<f32>;
    var phi_18975_: bool;
    var phi_19006_: vec4<f32>;
    var phi_19007_: bool;
    var phi_18840_: vec4<f32>;
    var phi_18841_: vec4<f32>;
    var phi_18842_: bool;
    var phi_18846_: vec4<f32>;
    var phi_18847_: bool;
    var phi_18848_: vec4<f32>;
    var phi_18849_: bool;
    var phi_18850_: bool;
    var phi_18858_: vec4<f32>;
    var phi_18859_: bool;
    var phi_18860_: vec4<f32>;
    var phi_18861_: bool;
    var phi_18862_: bool;
    var phi_18870_: vec4<f32>;
    var phi_18871_: bool;
    var phi_19016_: vec4<f32>;
    var phi_19017_: bool;
    var phi_19038_: f32;
    var phi_19044_: u32;
    var phi_18673_: u32;
    var phi_18676_: u32;
    var phi_18678_: u32;
    var phi_18680_: u32;
    var phi_18682_: u32;
    var phi_18684_: vec4<f32>;
    var phi_18686_: f32;
    var phi_19217_: vec4<f32>;
    var phi_19218_: bool;
    var phi_19219_: bool;
    var phi_36259_: bool;
    var phi_21865_: bool;
    var phi_21864_: vec4<f32>;
    var local_80: u32;
    var local_81: vec4<f32>;
    var phi_19224_: type_74;
    var local_82: u32;
    var phi_19230_: type_74;
    var phi_19282_: vec3<f32>;
    var phi_19283_: type_41;
    var phi_19284_: vec3<f32>;
    var phi_19285_: type_41;
    var phi_36701_: bool;
    var phi_19286_: vec3<f32>;
    var phi_19287_: type_41;
    var phi_19288_: u32;
    var phi_36700_: bool;
    var phi_19289_: vec3<f32>;
    var phi_19290_: type_41;
    var phi_19291_: u32;
    var phi_36986_: u32;
    var phi_36699_: bool;
    var phi_14709_: vec3<f32>;
    var phi_14712_: type_41;
    var phi_14714_: u32;
    var phi_14716_: u32;
    var phi_36706_: u32;
    var phi_36698_: bool;
    var phi_31053_: bool;
    var phi_31083_: bool;
    var phi_31072_: f32;
    var phi_31003_: bool;
    var phi_31139_: vec3<f32>;
    var phi_31295_: bool;
    var phi_31310_: bool;
    var phi_31325_: bool;
    var phi_31234_: vec3<f32>;
    var phi_31237_: vec3<f32>;
    var phi_31238_: bool;
    var phi_31008_: vec3<f32>;
    var phi_31012_: vec3<f32>;
    var phi_31013_: bool;
    var phi_31425_: bool;
    var phi_31399_: vec3<f32>;
    var phi_31020_: vec3<f32>;
    var phi_31440_: bool;
    var phi_31550_: f32;
    var phi_31578_: f32;
    var phi_31606_: f32;
    var phi_31740_: bool;
    var phi_31755_: bool;
    var phi_31770_: bool;
    var phi_31797_: bool;
    var phi_31812_: bool;
    var phi_31827_: bool;
    var phi_31854_: bool;
    var phi_31869_: bool;
    var phi_31884_: bool;
    var phi_19510_: f32;
    var phi_19518_: f32;
    var phi_19526_: f32;
    var phi_32081_: bool;
    var phi_32070_: f32;
    var phi_19633_: vec3<f32>;
    var phi_32109_: bool;
    var phi_19634_: vec3<f32>;
    var phi_19646_: type_59;
    var phi_19647_: type_59;
    var phi_19648_: bool;
    var phi_19653_: type_59;
    var phi_32146_: bool;
    var phi_32176_: bool;
    var phi_32165_: f32;
    var phi_32215_: vec3<f32>;
    var phi_19681_: type_59;
    var phi_19682_: type_59;
    var phi_32237_: vec3<f32>;
    var phi_19683_: type_59;
    var phi_19684_: type_59;
    var phi_19685_: bool;
    var phi_19689_: type_59;
    var phi_32262_: bool;
    var phi_19765_: vec3<f32>;
    var phi_38540_: u32;
    var phi_19767_: vec3<f32>;
    var phi_19768_: bool;
    var phi_38539_: u32;
    var phi_19769_: vec3<f32>;
    var phi_19770_: bool;
    var phi_38537_: u32;
    var phi_19776_: vec3<f32>;
    var phi_19777_: bool;
    var local_83: u32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: type_41;
    var phi_19806_: vec3<f32>;
    var local_88: u32;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: type_41;
    var phi_19822_: type_69;
    var phi_19823_: type_69;
    var phi_38544_: u32;
    var phi_19825_: type_69;
    var phi_19826_: type_69;
    var phi_19827_: bool;
    var phi_38543_: u32;
    var phi_19828_: type_69;
    var phi_19829_: type_69;
    var phi_19830_: bool;
    var phi_19831_: bool;
    var local_93: u32;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: vec3<f32>;
    var local_97: type_41;
    var phi_19837_: type_69;
    var local_98: u32;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: type_41;
    var phi_19842_: type_69;
    var phi_19843_: type_69;
    var phi_19844_: bool;
    var phi_19848_: type_69;
    var phi_19849_: type_69;
    var phi_19850_: bool;
    var phi_19854_: type_69;
    var phi_19855_: type_69;
    var phi_19856_: bool;
    var phi_13475_: bool;
    var phi_13484_: vec3<f32>;
    var phi_13486_: vec3<f32>;
    var phi_13487_: bool;
    var phi_13905_: f32;
    var phi_13908_: f32;
    var phi_32512_: bool;
    var phi_32493_: f32;
    var phi_13909_: f32;
    var phi_32530_: f32;
    var phi_32533_: f32;
    var phi_13915_: f32;
    var phi_13916_: f32;
    var phi_13917_: f32;
    var phi_13918_: f32;
    var phi_19857_: type_69;
    var phi_19858_: type_69;
    var phi_19859_: bool;
    var phi_11531_: f32;
    var phi_32580_: f32;
    var phi_32583_: f32;
    var phi_11533_: f32;
    var phi_11534_: f32;
    var phi_11535_: f32;
    var phi_32609_: bool;
    var phi_11589_: f32;
    var phi_32627_: f32;
    var phi_32630_: f32;
    var phi_11591_: f32;
    var phi_11592_: f32;
    var phi_32641_: bool;
    var phi_32656_: bool;
    var phi_12061_: vec3<f32>;
    var phi_12062_: vec3<f32>;
    var phi_12063_: bool;
    var phi_12068_: vec3<f32>;
    var phi_12069_: bool;
    var phi_32780_: bool;
    var phi_32795_: bool;
    var phi_12360_: vec3<f32>;
    var phi_12362_: vec3<f32>;
    var phi_12389_: vec3<f32>;
    var phi_12403_: vec3<f32>;
    var phi_19863_: type_69;
    var phi_19864_: type_69;
    var phi_19865_: bool;
    var phi_19869_: type_69;

    switch bitcast<i32>(0u) {
        default: {
            let _e282 = global;
            let _e292 = arrayLength((&global_9.member));
            let _e296 = global_1.member.member_7[2u];
            let _e300 = (_e282.x + (_e282.y * 4194240u));
            let _e305 = atomicLoad((&global_8.member[((_e296 * 4u) + 3u)]));
            if (_e300 >= _e305) {
                phi_11267_ = type_48(0u, type_48().member_1);
            } else {
                let _e310 = global_1.member.member[0u];
                let _e314 = global_1.member.member[1u];
                let _e319 = global_1.member.member_7[3u];
                let _e326 = atomicLoad((&global_8.member[((12u + (_e296 * ((_e310 * _e314) * _e319))) + _e300)]));
                phi_11267_ = type_48(1u, _e326);
            }
            let _e332 = phi_11267_;
            if (bitcast<i32>(_e332.member) != 0i) {
                let _e337 = (_e332.member_1 < arrayLength((&global_7.member)));
                if _e337 {
                } else {
                    break;
                }
                let _e340 = global_7.member[_e332.member_1];
                let _e354 = vec3<f32>(_e340.member_1.x, _e340.member_1.y, _e340.member_1.z);
                let _e359 = vec3<f32>(_e340.member_2.x, _e340.member_2.y, _e340.member_2.z);
                let _e365 = select(select(u32(_e340.member_2.w), 0u, (_e340.member_2.w < 0f)), 4294967295u, (_e340.member_2.w > 4294967000f));
                let _e369 = vec3<f32>(_e340.member_3.x, _e340.member_3.y, _e340.member_3.z);
                let _e379 = ((_e340.member_4.y & 2u) != 0u);
                let _e381 = ((_e340.member_4.y & 32u) != 0u);
                let _e384 = global_1.member.member[3u];
                let _e387 = global_1.member.member_3[3u];
                let _e390 = global_1.member.member_1[2u];
                let _e393 = (select(select(u32(_e340.member_3.w), 0u, (_e340.member_3.w < 0f)), 4294967295u, (_e340.member_3.w > 4294967000f)) + 1u);
                if (_e340.member_4.z == 4294967295u) {
                    if _e379 {
                        phi_11535_ = 1f;
                    } else {
                        if _e381 {
                            phi_11534_ = 1f;
                        } else {
                            if ((_e387 & 2u) == 0u) {
                                phi_11533_ = 1f;
                            } else {
                                let _e6604 = global_1.member.member_3[1u];
                                let _e6607 = global_1.member.member_3[2u];
                                let _e6610 = ((atan2(_e340.member_1.z, _e340.member_1.x) * 0.15915494f) + 0.5f);
                                let _e6614 = select(_e340.member_1.y, -1f, (_e340.member_1.y < -1f));
                                let _e6617 = acos(select(_e6614, 1f, (_e6614 > 1f)));
                                let _e6619 = f32(_e6604);
                                let _e6620 = ((_e6610 - floor(_e6610)) * _e6619);
                                let _e6625 = select(select(u32(_e6620), 0u, (_e6620 < 0f)), 4294967295u, (_e6620 > 4294967000f));
                                let _e6626 = (_e6604 - 1u);
                                let _e6629 = f32(_e6607);
                                let _e6630 = ((_e6617 * 0.31830987f) * _e6629);
                                let _e6635 = select(select(u32(_e6630), 0u, (_e6630 < 0f)), 4294967295u, (_e6630 > 4294967000f));
                                let _e6636 = (_e6607 - 1u);
                                let _e6638 = select(_e6635, _e6636, (_e6636 < _e6635));
                                let _e6639 = sin(_e6617);
                                if (_e6639 <= 0f) {
                                    phi_11531_ = 0f;
                                } else {
                                    let _e6643 = global_1.member.member_2[3u];
                                    let _e6646 = global_1.member.member_3[0u];
                                    let _e6650 = (_e6643 + _e6638);
                                    let _e6653 = global_6.member[(_e6650 + 1u)];
                                    let _e6655 = global_6.member[_e6650];
                                    let _e6658 = ((_e6646 + (_e6638 * (_e6604 + 1u))) + select(_e6625, _e6626, (_e6626 < _e6625)));
                                    let _e6661 = global_6.member[(_e6658 + 1u)];
                                    let _e6663 = global_6.member[_e6658];
                                    phi_11531_ = ((((_e6653 - _e6655) * _e6629) * ((_e6661 - _e6663) * _e6619)) / (19.73921f * _e6639));
                                }
                                let _e6670 = phi_11531_;
                                if (_e340.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32583_ = 1f;
                                } else {
                                    let _e6672 = (_e340.member_1.w * _e340.member_1.w);
                                    let _e6674 = (_e6672 + (_e6670 * _e6670));
                                    if (_e6674 == 0f) {
                                        phi_32580_ = 0f;
                                    } else {
                                        phi_32580_ = (_e6672 / _e6674);
                                    }
                                    let _e6678 = phi_32580_;
                                    phi_32583_ = _e6678;
                                }
                                let _e6680 = phi_32583_;
                                phi_11533_ = _e6680;
                            }
                            let _e6682 = phi_11533_;
                            phi_11534_ = _e6682;
                        }
                        let _e6684 = phi_11534_;
                        phi_11535_ = _e6684;
                    }
                    let _e6686 = phi_11535_;
                    if ((_e387 & 4u) == 0u) {
                        phi_12403_ = _e369;
                    } else {
                        if _e379 {
                            phi_11592_ = 1f;
                        } else {
                            if _e381 {
                                phi_11591_ = 0f;
                            } else {
                                let _e6697 = global_1.member.member_5;
                                let _e6701 = (_e340.member_1.x - _e6697.x);
                                let _e6702 = (_e340.member_1.y - _e6697.y);
                                let _e6703 = (_e340.member_1.z - _e6697.z);
                                let _e6710 = (sqrt((((_e6701 * _e6701) + (_e6702 * _e6702)) + (_e6703 * _e6703))) * 0.5f);
                                if (_e6710 != _e6710) {
                                    phi_32609_ = true;
                                } else {
                                    phi_32609_ = (1f <= _e6710);
                                }
                                let _e6714 = phi_32609_;
                                let _e6720 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e6710, 1f, _e6714))) >= _e6720) {
                                    phi_11589_ = 0f;
                                } else {
                                    let _e6723 = sin((_e6720 * 0.5f));
                                    phi_11589_ = (1f / ((12.566371f * _e6723) * _e6723));
                                }
                                let _e6728 = phi_11589_;
                                if (_e340.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32630_ = 1f;
                                } else {
                                    let _e6730 = (_e340.member_1.w * _e340.member_1.w);
                                    let _e6732 = (_e6730 + (_e6728 * _e6728));
                                    if (_e6732 == 0f) {
                                        phi_32627_ = 0f;
                                    } else {
                                        phi_32627_ = (_e6730 / _e6732);
                                    }
                                    let _e6736 = phi_32627_;
                                    phi_32630_ = _e6736;
                                }
                                let _e6738 = phi_32630_;
                                phi_11591_ = _e6738;
                            }
                            let _e6740 = phi_11591_;
                            phi_11592_ = _e6740;
                        }
                        let _e6742 = phi_11592_;
                        let _e6744 = global_1.member.member_5;
                        let _e6748 = (_e340.member_1.x - _e6744.x);
                        let _e6749 = (_e340.member_1.y - _e6744.y);
                        let _e6750 = (_e340.member_1.z - _e6744.z);
                        let _e6757 = (sqrt((((_e6748 * _e6748) + (_e6749 * _e6749)) + (_e6750 * _e6750))) * 0.5f);
                        if (_e6757 != _e6757) {
                            phi_32641_ = true;
                        } else {
                            phi_32641_ = (1f <= _e6757);
                        }
                        let _e6761 = phi_32641_;
                        let _e6764 = (2f * asin(select(_e6757, 1f, _e6761)));
                        let _e6768 = global_1.member.member_6[3u];
                        if (_e6764 >= _e6768) {
                            phi_12389_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e6772 = (1f - pow((_e6764 / _e6768), 2f));
                            if (_e6772 != _e6772) {
                                phi_32656_ = true;
                            } else {
                                phi_32656_ = (0f >= _e6772);
                            }
                            let _e6776 = phi_32656_;
                            let _e6779 = global_1.member.member_6;
                            let _e6785 = global_1.member.member_8[2u];
                            if (_e6785 == 4294967295u) {
                                phi_12362_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e6788 = global_6.member[_e6785];
                                let _e6791 = global_6.member[(_e6785 + 1u)];
                                let _e6794 = global_6.member[(_e6785 + 2u)];
                                let _e6797 = global_6.member[(_e6785 + 3u)];
                                let _e6800 = global_6.member[(_e6785 + 4u)];
                                let _e6803 = global_6.member[(_e6785 + 5u)];
                                let _e6806 = global_6.member[(_e6785 + 6u)];
                                let _e6809 = global_6.member[(_e6785 + 7u)];
                                let _e6812 = global_6.member[(_e6785 + 8u)];
                                let _e6815 = global_6.member[(_e6785 + 9u)];
                                let _e6818 = global_6.member[(_e6785 + 13u)];
                                let _e6821 = global_6.member[(_e6785 + 14u)];
                                let _e6824 = global_6.member[(_e6785 + 15u)];
                                let _e6827 = global_6.member[(_e6785 + 16u)];
                                let _e6830 = global_6.member[(_e6785 + 18u)];
                                let _e6833 = global_6.member[(_e6785 + 19u)];
                                let _e6836 = global_6.member[(_e6785 + 20u)];
                                let _e6839 = global_6.member[(_e6785 + 29u)];
                                let _e6840 = bitcast<u32>(_e6839);
                                let _e6853 = (((((_e340.member.x * _e340.member.x) + (_e340.member.y * _e340.member.y)) + (_e340.member.z * _e340.member.z)) - (2f * (((_e340.member.x * _e6788) + (_e340.member.y * _e6791)) + (_e340.member.z * _e6794)))) + _e6806);
                                if (_e6853 > 0f) {
                                    let _e6862 = ((((_e340.member.x - _e6788) * _e6809) + ((_e340.member.y - _e6791) * _e6812)) + ((_e340.member.z - _e6794) * _e6815));
                                    let _e6864 = ((_e6862 * _e6862) - _e6853);
                                    if (_e6862 >= 0f) {
                                        phi_12062_ = vec3<f32>();
                                        phi_12063_ = true;
                                    } else {
                                        let _e6866 = (_e6864 <= 0f);
                                        if _e6866 {
                                            phi_12061_ = vec3<f32>();
                                        } else {
                                            let _e6869 = (-(_e6862) - sqrt(_e6864));
                                            phi_12061_ = vec3<f32>((_e340.member.x + (_e6809 * _e6869)), (_e340.member.y + (_e6812 * _e6869)), (_e340.member.z + (_e6815 * _e6869)));
                                        }
                                        let _e6878 = phi_12061_;
                                        phi_12062_ = _e6878;
                                        phi_12063_ = _e6866;
                                    }
                                    let _e6880 = phi_12062_;
                                    let _e6882 = phi_12063_;
                                    phi_12068_ = _e6880;
                                    phi_12069_ = select(true, false, _e6882);
                                } else {
                                    phi_12068_ = vec3<f32>(_e340.member.x, _e340.member.y, _e340.member.z);
                                    phi_12069_ = true;
                                }
                                let _e6885 = phi_12068_;
                                let _e6887 = phi_12069_;
                                if _e6887 {
                                    let _e6889 = (_e6885.x - _e6788);
                                    let _e6891 = (_e6885.y - _e6791);
                                    let _e6893 = (_e6885.z - _e6794);
                                    let _e6899 = sqrt((((_e6889 * _e6889) + (_e6891 * _e6891)) + (_e6893 * _e6893)));
                                    let _e6900 = (1f / _e6899);
                                    let _e6918 = ((((((_e6885.x * _e6885.x) + (_e6885.y * _e6885.y)) + (_e6885.z * _e6885.z)) - (2f * (((_e6885.x * _e6788) + (_e6885.y * _e6791)) + (_e6885.z * _e6794)))) + _e6803) / (_e6899 + _e6797));
                                    if (_e6918 != _e6918) {
                                        phi_32780_ = true;
                                    } else {
                                        phi_32780_ = (0f >= _e6918);
                                    }
                                    let _e6922 = phi_32780_;
                                    let _e6926 = sqrt((select(_e6918, 0f, _e6922) / (_e6800 - _e6797)));
                                    if (_e6926 != _e6926) {
                                        phi_32795_ = true;
                                    } else {
                                        phi_32795_ = (1f <= _e6926);
                                    }
                                    let _e6930 = phi_32795_;
                                    let _e6932 = (select(_e6926, 1f, _e6930) * 63f);
                                    let _e6937 = ((((_e6889 * _e6900) * _e6809) + ((_e6891 * _e6900) * _e6812)) + ((_e6893 * _e6900) * _e6815));
                                    let _e6939 = select(_e6937, -1f, (_e6937 < -1f));
                                    let _e6943 = ((select(_e6939, 1f, (_e6939 > 1f)) + 1f) * 127.5f);
                                    let _e6948 = select(select(u32(_e6932), 0u, (_e6932 < 0f)), 4294967295u, (_e6932 > 4294967000f));
                                    let _e6950 = select(_e6948, 62u, (62u < _e6948));
                                    let _e6955 = select(select(u32(_e6943), 0u, (_e6943 < 0f)), 4294967295u, (_e6943 > 4294967000f));
                                    let _e6957 = select(_e6955, 254u, (254u < _e6955));
                                    let _e6959 = (_e6932 - f32(_e6950));
                                    let _e6961 = (_e6943 - f32(_e6957));
                                    let _e6965 = (_e6840 + (3u * ((_e6950 * 256u) + _e6957)));
                                    let _e6970 = (_e6840 + (3u * (((_e6950 + 1u) * 256u) + _e6957)));
                                    let _e6972 = global_6.member[_e6965];
                                    let _e6975 = global_6.member[(_e6965 + 1u)];
                                    let _e6978 = global_6.member[(_e6965 + 2u)];
                                    let _e6979 = (1f - _e6961);
                                    let _e6985 = global_6.member[(_e6965 + 3u)];
                                    let _e6988 = global_6.member[(_e6965 + 4u)];
                                    let _e6991 = global_6.member[(_e6965 + 5u)];
                                    let _e6998 = (1f - _e6959);
                                    let _e7003 = global_6.member[_e6970];
                                    let _e7006 = global_6.member[(_e6970 + 1u)];
                                    let _e7009 = global_6.member[(_e6970 + 2u)];
                                    let _e7015 = global_6.member[(_e6970 + 3u)];
                                    let _e7018 = global_6.member[(_e6970 + 4u)];
                                    let _e7021 = global_6.member[(_e6970 + 5u)];
                                    phi_12360_ = vec3<f32>(((((_e6972 * _e6979) + (_e6985 * _e6961)) * _e6998) + (((_e7003 * _e6979) + (_e7015 * _e6961)) * _e6959)), ((((_e6975 * _e6979) + (_e6988 * _e6961)) * _e6998) + (((_e7006 * _e6979) + (_e7018 * _e6961)) * _e6959)), ((((_e6978 * _e6979) + (_e6991 * _e6961)) * _e6998) + (((_e7009 * _e6979) + (_e7021 * _e6961)) * _e6959)));
                                } else {
                                    phi_12360_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e7036 = phi_12360_;
                                let _e7043 = ((_e6827 * 1.1f) * _e7036.y);
                                phi_12362_ = vec3<f32>(exp(-((((_e6818 * _e7036.x) + _e7043) + (_e6830 * _e7036.z)))), exp(-((((_e6821 * _e7036.x) + _e7043) + (_e6833 * _e7036.z)))), exp(-((((_e6824 * _e7036.x) + _e7043) + (_e6836 * _e7036.z)))));
                            }
                            let _e7062 = phi_12362_;
                            let _e7074 = sin((_e6768 * 0.5f));
                            let _e7077 = (((1f - (0.6f * (1f - sqrt(select(_e6772, 0f, _e6776))))) * 1.25f) / ((12.566371f * _e7074) * _e7074));
                            phi_12389_ = vec3<f32>(((_e6779.x * _e7062.x) * _e7077), ((_e6779.y * _e7062.y) * _e7077), ((_e6779.z * _e7062.z) * _e7077));
                        }
                        let _e7083 = phi_12389_;
                        phi_12403_ = vec3<f32>((_e340.member_3.x + ((_e340.member_2.x * _e7083.x) * _e6742)), (_e340.member_3.y + ((_e340.member_2.y * _e7083.y) * _e6742)), (_e340.member_3.z + ((_e340.member_2.z * _e7083.z) * _e6742)));
                    }
                    let _e7098 = phi_12403_;
                    phi_19863_ = type_69(type_34(_e340.member, _e340.member_1, vec4<f32>(_e340.member_2.x, _e340.member_2.y, _e340.member_2.z, f32(_e365)), vec4<f32>(_e7098.x, _e7098.y, _e7098.z, f32(_e393)), vec4<u32>(_e340.member_4.x, (_e340.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), type_41(vec4<f32>(_e340.member.x, _e340.member.y, _e340.member.z, -1f), vec4<f32>(_e340.member_1.x, _e340.member_1.y, _e340.member_1.z, 0f), vec4<f32>((_e340.member_2.x * _e6686), (_e340.member_2.y * _e6686), (_e340.member_2.z * _e6686), 0f)), type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_19864_ = type_69();
                    phi_19865_ = true;
                } else {
                    if ((_e387 & 8u) == 0u) {
                        phi_36354_ = false;
                        phi_12937_ = _e359;
                        phi_12938_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    } else {
                        if (_e340.member_5.x > 100f) {
                            let _e404 = global_1.member.member_8[1u];
                            let _e406 = global_6.member[_e404];
                            let _e409 = global_6.member[(_e404 + 1u)];
                            let _e412 = global_6.member[(_e404 + 2u)];
                            let _e415 = global_6.member[(_e404 + 3u)];
                            let _e418 = global_6.member[(_e404 + 5u)];
                            let _e421 = global_6.member[(_e404 + 6u)];
                            let _e424 = global_6.member[(_e404 + 10u)];
                            let _e427 = global_6.member[(_e404 + 11u)];
                            let _e430 = global_6.member[(_e404 + 13u)];
                            let _e433 = global_6.member[(_e404 + 14u)];
                            let _e436 = global_6.member[(_e404 + 15u)];
                            let _e439 = global_6.member[(_e404 + 16u)];
                            let _e442 = global_6.member[(_e404 + 18u)];
                            let _e445 = global_6.member[(_e404 + 19u)];
                            let _e448 = global_6.member[(_e404 + 20u)];
                            let _e456 = ((((_e340.member.x - _e406) * _e340.member_1.x) + ((_e340.member.y - _e409) * _e340.member_1.y)) + ((_e340.member.z - _e412) * _e340.member_1.z));
                            let _e468 = ((((_e340.member.x * _e340.member.x) + (_e340.member.y * _e340.member.y)) + (_e340.member.z * _e340.member.z)) - (2f * (((_e340.member.x * _e406) + (_e340.member.y * _e409)) + (_e340.member.z * _e412))));
                            let _e469 = (_e468 + _e421);
                            let _e470 = (_e456 * _e456);
                            let _e471 = (_e470 - _e469);
                            if (_e471 < 0f) {
                                phi_26326_ = type_57(0u, type_57().member_1);
                                phi_26327_ = type_57();
                                phi_26328_ = true;
                            } else {
                                if (_e456 < 0f) {
                                    phi_26298_ = -(sqrt(_e471));
                                } else {
                                    phi_26298_ = sqrt(_e471);
                                }
                                let _e478 = phi_26298_;
                                let _e480 = -((_e456 + _e478));
                                let _e481 = (_e480 == 0f);
                                if _e481 {
                                    phi_26321_ = type_57(1u, type_56(0f, 0f));
                                    phi_26322_ = type_57();
                                } else {
                                    let _e482 = (_e469 / _e480);
                                    if (_e480 < _e482) {
                                        phi_26310_ = type_56(_e480, _e482);
                                    } else {
                                        phi_26310_ = type_56(_e482, _e480);
                                    }
                                    let _e487 = phi_26310_;
                                    phi_26321_ = type_57();
                                    phi_26322_ = type_57(1u, _e487);
                                }
                                let _e490 = phi_26321_;
                                let _e492 = phi_26322_;
                                phi_26326_ = _e490;
                                phi_26327_ = _e492;
                                phi_26328_ = _e481;
                            }
                            let _e497 = phi_26326_;
                            let _e499 = phi_26327_;
                            let _e501 = phi_26328_;
                            if _e501 {
                                phi_26332_ = _e497;
                            } else {
                                phi_26332_ = _e499;
                            }
                            let _e503 = phi_26332_;
                            if (bitcast<i32>(_e503.member) != 0i) {
                                phi_26219_ = type_57(0u, _e503.member_1);
                            } else {
                                phi_26219_ = type_57(1u, type_57().member_1);
                            }
                            let _e513 = phi_26219_;
                            if (bitcast<i32>(_e513.member) != 0i) {
                                phi_26266_ = type_57(0u, type_57().member_1);
                            } else {
                                if (_e513.member_1.member_1 <= 0f) {
                                    phi_26262_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e513.member_1.member != _e513.member_1.member) {
                                        phi_26370_ = true;
                                    } else {
                                        phi_26370_ = (0f >= _e513.member_1.member);
                                    }
                                    let _e525 = phi_26370_;
                                    let _e526 = select(_e513.member_1.member, 0f, _e525);
                                    let _e527 = (_e468 + _e418);
                                    if (_e527 > 0f) {
                                        let _e529 = (_e470 - _e527);
                                        if (_e529 < 0f) {
                                            phi_26478_ = type_57(0u, type_57().member_1);
                                            phi_26479_ = type_57();
                                            phi_26480_ = true;
                                        } else {
                                            if (_e456 < 0f) {
                                                phi_26450_ = -(sqrt(_e529));
                                            } else {
                                                phi_26450_ = sqrt(_e529);
                                            }
                                            let _e536 = phi_26450_;
                                            let _e538 = -((_e456 + _e536));
                                            let _e539 = (_e538 == 0f);
                                            if _e539 {
                                                phi_26473_ = type_57(1u, type_56(0f, 0f));
                                                phi_26474_ = type_57();
                                            } else {
                                                let _e540 = (_e527 / _e538);
                                                if (_e538 < _e540) {
                                                    phi_26462_ = type_56(_e538, _e540);
                                                } else {
                                                    phi_26462_ = type_56(_e540, _e538);
                                                }
                                                let _e545 = phi_26462_;
                                                phi_26473_ = type_57();
                                                phi_26474_ = type_57(1u, _e545);
                                            }
                                            let _e548 = phi_26473_;
                                            let _e550 = phi_26474_;
                                            phi_26478_ = _e548;
                                            phi_26479_ = _e550;
                                            phi_26480_ = _e539;
                                        }
                                        let _e555 = phi_26478_;
                                        let _e557 = phi_26479_;
                                        let _e559 = phi_26480_;
                                        if _e559 {
                                            phi_26484_ = _e555;
                                        } else {
                                            phi_26484_ = _e557;
                                        }
                                        let _e561 = phi_26484_;
                                        if (bitcast<i32>(_e561.member) != 0i) {
                                            phi_26418_ = select(_e561.member_1.member_1, _e561.member_1.member, (_e561.member_1.member >= 0f));
                                        } else {
                                            phi_26418_ = -1f;
                                        }
                                        let _e572 = phi_26418_;
                                        if (_e572 >= 0f) {
                                            if (_e513.member_1.member_1 != _e513.member_1.member_1) {
                                                phi_26522_ = true;
                                            } else {
                                                phi_26522_ = (_e572 <= _e513.member_1.member_1);
                                            }
                                            let _e577 = phi_26522_;
                                            phi_26247_ = select(_e513.member_1.member_1, _e572, _e577);
                                        } else {
                                            phi_26247_ = _e513.member_1.member_1;
                                        }
                                        let _e580 = phi_26247_;
                                        phi_26249_ = _e580;
                                    } else {
                                        phi_26249_ = _e513.member_1.member_1;
                                    }
                                    let _e582 = phi_26249_;
                                    if (_e582 <= _e526) {
                                        phi_26258_ = type_57(0u, type_57().member_1);
                                    } else {
                                        phi_26258_ = type_57(1u, type_56(_e526, _e582));
                                    }
                                    let _e590 = phi_26258_;
                                    phi_26262_ = _e590;
                                }
                                let _e595 = phi_26262_;
                                phi_26266_ = _e595;
                            }
                            let _e600 = phi_26266_;
                            if (bitcast<i32>(_e600.member) != 0i) {
                                let _e606 = (_e600.member_1.member < _e340.member_5.x);
                                if _e606 {
                                    if (_e600.member_1.member_1 != _e600.member_1.member_1) {
                                        phi_26537_ = true;
                                    } else {
                                        phi_26537_ = (_e340.member_5.x <= _e600.member_1.member_1);
                                    }
                                    let _e612 = phi_26537_;
                                    let _e613 = select(_e600.member_1.member_1, _e340.member_5.x, _e612);
                                    phi_12796_ = vec3<f32>(0f, 0f, 0f);
                                    phi_12799_ = 0u;
                                    loop {
                                        let _e615 = phi_12796_;
                                        let _e617 = phi_12799_;
                                        local_69 = _e615;
                                        local_70 = _e615;
                                        local_71 = _e615;
                                        let _e618 = (_e617 < 32u);
                                        if _e618 {
                                            let _e619 = -(_e456);
                                            if (_e600.member_1.member <= _e613) {
                                            } else {
                                                phi_34218_ = true;
                                                break;
                                            }
                                            let _e622 = select(_e619, _e600.member_1.member, (_e619 < _e600.member_1.member));
                                            let _e624 = select(_e622, _e613, (_e622 > _e613));
                                            let _e625 = (_e624 - _e600.member_1.member);
                                            let _e626 = (_e613 - _e624);
                                            let _e627 = (_e613 - _e600.member_1.member);
                                            let _e628 = (0.000000001f * _e627);
                                            if (_e625 <= _e628) {
                                                let _e703 = (f32(_e617) * 0.03125f);
                                                let _e706 = (f32((_e617 + 1u)) * 0.03125f);
                                                let _e708 = ((_e703 + _e706) * 0.5f);
                                                phi_12900_ = type_56((_e600.member_1.member + ((_e627 * _e708) * _e708)), (_e627 * ((_e706 * _e706) - (_e703 * _e703))));
                                            } else {
                                                if (_e626 <= _e628) {
                                                    let _e686 = (f32((31u - _e617)) * 0.03125f);
                                                    let _e689 = (f32((32u - _e617)) * 0.03125f);
                                                    let _e691 = ((_e686 + _e689) * 0.5f);
                                                    phi_12899_ = type_56((_e613 - ((_e627 * _e691) * _e691)), (_e627 * ((_e689 * _e689) - (_e686 * _e686))));
                                                } else {
                                                    let _e634 = round(((32f * _e625) / (_e625 + _e626)));
                                                    let _e639 = select(select(u32(_e634), 0u, (_e634 < 0f)), 4294967295u, (_e634 > 4294967000f));
                                                    if (_e639 < 1u) {
                                                        phi_12883_ = 1u;
                                                    } else {
                                                        phi_12883_ = select(_e639, 31u, (_e639 > 31u));
                                                    }
                                                    let _e644 = phi_12883_;
                                                    if (_e617 < _e644) {
                                                        let _e665 = ((_e644 - 1u) - _e617);
                                                        let _e667 = f32(_e644);
                                                        let _e668 = (f32(_e665) / _e667);
                                                        let _e671 = (f32((_e665 + 1u)) / _e667);
                                                        let _e673 = ((_e668 + _e671) * 0.5f);
                                                        phi_12898_ = type_56((_e624 - ((_e625 * _e673) * _e673)), (_e625 * ((_e671 * _e671) - (_e668 * _e668))));
                                                    } else {
                                                        let _e647 = (_e617 - _e644);
                                                        let _e649 = f32((32u - _e644));
                                                        let _e650 = (f32(_e647) / _e649);
                                                        let _e653 = (f32((_e647 + 1u)) / _e649);
                                                        let _e655 = ((_e650 + _e653) * 0.5f);
                                                        phi_12898_ = type_56((_e624 + ((_e626 * _e655) * _e655)), (_e626 * ((_e653 * _e653) - (_e650 * _e650))));
                                                    }
                                                    let _e683 = phi_12898_;
                                                    phi_12899_ = _e683;
                                                }
                                                let _e701 = phi_12899_;
                                                phi_12900_ = _e701;
                                            }
                                            let _e718 = phi_12900_;
                                            let _e724 = (_e340.member.x + (_e340.member_1.x * _e718.member));
                                            let _e725 = (_e340.member.y + (_e340.member_1.y * _e718.member));
                                            let _e726 = (_e340.member.z + (_e340.member_1.z * _e718.member));
                                            let _e740 = (_e724 - _e406);
                                            let _e741 = (_e725 - _e409);
                                            let _e742 = (_e726 - _e412);
                                            let _e750 = ((((((_e724 * _e724) + (_e725 * _e725)) + (_e726 * _e726)) - (2f * (((_e724 * _e406) + (_e725 * _e409)) + (_e726 * _e412)))) + _e418) / (sqrt((((_e740 * _e740) + (_e741 * _e741)) + (_e742 * _e742))) + _e415));
                                            let _e754 = (1f - (abs((_e750 - 25000f)) * 0.00006666667f));
                                            if (_e754 != _e754) {
                                                phi_26736_ = true;
                                            } else {
                                                phi_26736_ = (0f >= _e754);
                                            }
                                            let _e758 = phi_26736_;
                                            let _e760 = -(_e750);
                                            phi_12797_ = (_e615 + vec3<f32>((exp((_e760 / _e424)) * _e718.member_1), (exp((_e760 / _e427)) * _e718.member_1), (select(_e754, 0f, _e758) * _e718.member_1)));
                                            phi_12800_ = (_e617 + 1u);
                                        } else {
                                            phi_12797_ = vec3<f32>();
                                            phi_12800_ = u32();
                                        }
                                        let _e772 = phi_12797_;
                                        let _e774 = phi_12800_;
                                        phi_12796_ = _e772;
                                        phi_12799_ = _e774;
                                        phi_34218_ = false;
                                        if !(_e618) {
                                            break;
                                        }
                                    }
                                    let _e777 = phi_34218_;
                                    if _e777 {
                                        break;
                                    }
                                    let _e779 = local_69;
                                    let _e786 = local_70;
                                    let _e788 = ((_e439 * 1.1f) * _e786.y);
                                    let _e793 = local_71;
                                    phi_36358_ = _e777;
                                    phi_12925_ = vec3<f32>(exp(-((((_e430 * _e779.x) + _e788) + (_e442 * _e793.z)))), exp(-((((_e433 * _e779.x) + _e788) + (_e445 * _e793.z)))), exp(-((((_e436 * _e779.x) + _e788) + (_e448 * _e793.z)))));
                                } else {
                                    phi_36358_ = false;
                                    phi_12925_ = vec3<f32>();
                                }
                                let _e809 = phi_36358_;
                                let _e811 = phi_12925_;
                                phi_36357_ = _e809;
                                phi_12927_ = _e811;
                                phi_12928_ = select(true, false, _e606);
                            } else {
                                phi_36357_ = false;
                                phi_12927_ = vec3<f32>();
                                phi_12928_ = true;
                            }
                            let _e814 = phi_36357_;
                            let _e816 = phi_12927_;
                            let _e818 = phi_12928_;
                            phi_36355_ = _e814;
                            phi_12935_ = (_e359 * select(_e816, vec3<f32>(1f, 1f, 1f), vec3(_e818)));
                            phi_12936_ = type_41(vec4<f32>(_e340.member.x, _e340.member.y, _e340.member.z, _e340.member_5.x), vec4<f32>(_e340.member_1.x, _e340.member_1.y, _e340.member_1.z, 0f), vec4<f32>(_e340.member_2.x, _e340.member_2.y, _e340.member_2.z, 0f));
                        } else {
                            phi_36355_ = false;
                            phi_12935_ = _e359;
                            phi_12936_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e823 = phi_36355_;
                        let _e825 = phi_12935_;
                        let _e827 = phi_12936_;
                        phi_36354_ = _e823;
                        phi_12937_ = _e825;
                        phi_12938_ = _e827;
                    }
                    let _e829 = phi_36354_;
                    let _e831 = phi_12937_;
                    let _e833 = phi_12938_;
                    let _e837 = (_e340.member.x + (_e340.member_1.x * _e340.member_5.x));
                    let _e838 = (_e340.member.y + (_e340.member_1.y * _e340.member_5.x));
                    let _e839 = (_e340.member.z + (_e340.member_1.z * _e340.member_5.x));
                    let _e842 = global_1.member.member_1[0u];
                    let _e845 = global_1.member.member_1[1u];
                    let _e846 = (_e842 + _e845);
                    let _e847 = (3u * _e340.member_4.z);
                    let _e849 = global_2.member[_e847];
                    let _e852 = global_2.member[(_e847 + 1u)];
                    let _e858 = select(select(u32(_e852.w), 0u, (_e852.w < 0f)), 4294967295u, (_e852.w > 4294967000f));
                    let _e859 = (3u * _e846);
                    let _e861 = (_e859 + (5u * _e340.member_4.z));
                    let _e863 = global_2.member[_e861];
                    let _e866 = global_2.member[(_e861 + 1u)];
                    let _e869 = global_2.member[(_e861 + 2u)];
                    let _e881 = global_2.member[(_e861 + 3u)];
                    let _e884 = global_2.member[(_e861 + 4u)];
                    let _e891 = (_e858 == 0u);
                    if _e891 {
                        let _e914 = (_e863.x - _e849.x);
                        let _e915 = (_e863.y - _e849.y);
                        let _e916 = (_e863.z - _e849.z);
                        let _e917 = (_e866.x - _e849.x);
                        let _e918 = (_e866.y - _e849.y);
                        let _e919 = (_e866.z - _e849.z);
                        let _e922 = ((_e915 * _e919) - (_e918 * _e916));
                        let _e925 = ((_e916 * _e917) - (_e919 * _e914));
                        let _e928 = ((_e914 * _e918) - (_e917 * _e915));
                        let _e935 = (1f / sqrt((((_e922 * _e922) + (_e925 * _e925)) + (_e928 * _e928))));
                        let _e939 = vec3<f32>((_e922 * _e935), (_e925 * _e935), (_e928 * _e935));
                        if (select(select(u32(_e869.w), 0u, (_e869.w < 0f)), 4294967295u, (_e869.w > 4294967000f)) == 1u) {
                            let _e944 = ((1f - _e340.member_5.y) - _e340.member_5.z);
                            let _e963 = (((_e869.x * _e944) + (_e881.x * _e340.member_5.y)) + (_e884.x * _e340.member_5.z));
                            let _e964 = (((_e869.y * _e944) + (_e881.y * _e340.member_5.y)) + (_e884.y * _e340.member_5.z));
                            let _e965 = (((_e869.z * _e944) + (_e881.z * _e340.member_5.y)) + (_e884.z * _e340.member_5.z));
                            let _e970 = (((_e963 * _e963) + (_e964 * _e964)) + (_e965 * _e965));
                            if (_e970 > 0f) {
                                let _e973 = (1f / sqrt(_e970));
                                phi_13184_ = vec3<f32>((_e963 * _e973), (_e964 * _e973), (_e965 * _e973));
                            } else {
                                phi_13184_ = _e939;
                            }
                            let _e979 = phi_13184_;
                            phi_13185_ = _e979;
                        } else {
                            phi_13185_ = _e939;
                        }
                        let _e981 = phi_13185_;
                        phi_13218_ = type_60(_e939, _e981);
                    } else {
                        if (_e858 == 1u) {
                            let _e907 = vec3<f32>(((_e837 - _e849.x) / _e849.w), ((_e838 - _e849.y) / _e849.w), ((_e839 - _e849.z) / _e849.w));
                            phi_13217_ = type_60(_e907, _e907);
                        } else {
                            let _e895 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e340.member_1.y > 0f)));
                            phi_13217_ = type_60(_e895, _e895);
                        }
                        let _e910 = phi_13217_;
                        phi_13218_ = _e910;
                    }
                    let _e984 = phi_13218_;
                    let _e998 = select(select(u32(_e866.w), 0u, (_e866.w < 0f)), 4294967295u, (_e866.w > 4294967000f));
                    phi_13341_ = 0i;
                    phi_13344_ = select(select(u32(_e863.w), 0u, (_e863.w < 0f)), 4294967295u, (_e863.w > 4294967000f));
                    loop {
                        let _e1000 = phi_13341_;
                        let _e1002 = phi_13344_;
                        local_72 = _e1002;
                        if (_e1000 < 4i) {
                            let _e1005 = global_4.member[_e1002];
                            let _e1007 = (_e1005.member != 6u);
                            if _e1007 {
                                phi_13413_ = i32();
                                phi_13414_ = u32();
                            } else {
                                let _e1011 = (abs(_e837) / _e1005.member_6.z);
                                let _e1022 = (abs(_e839) / _e1005.member_6.w);
                                if ((select(select(u32(_e1011), 0u, (_e1011 < 0f)), 4294967295u, (_e1011 > 4294967000f)) % 2u) == 0u) {
                                    phi_13398_ = (_e837 > 0f);
                                } else {
                                    phi_13398_ = (_e837 <= 0f);
                                }
                                let _e1033 = phi_13398_;
                                if ((select(select(u32(_e1022), 0u, (_e1022 < 0f)), 4294967295u, (_e1022 > 4294967000f)) % 2u) == 0u) {
                                    phi_13404_ = (_e839 > 0f);
                                } else {
                                    phi_13404_ = (_e839 <= 0f);
                                }
                                let _e1037 = phi_13404_;
                                if (_e1033 != _e1037) {
                                    phi_13411_ = _e1005.member_1;
                                } else {
                                    phi_13411_ = _e1005.member_2;
                                }
                                let _e1042 = phi_13411_;
                                phi_13413_ = (_e1000 + 1i);
                                phi_13414_ = _e1042;
                            }
                            let _e1045 = phi_13413_;
                            let _e1047 = phi_13414_;
                            phi_13342_ = _e1045;
                            phi_13345_ = _e1047;
                            phi_13416_ = select(true, false, _e1007);
                        } else {
                            phi_13342_ = i32();
                            phi_13345_ = u32();
                            phi_13416_ = false;
                        }
                        let _e1050 = phi_13342_;
                        let _e1052 = phi_13345_;
                        let _e1054 = phi_13416_;
                        phi_13341_ = _e1050;
                        phi_13344_ = _e1052;
                        if !(_e1054) {
                            break;
                        }
                    }
                    let _e1057 = local_72;
                    let _e1059 = global_4.member[_e1057];
                    let _e1061 = (_e1059.member == 4u);
                    if _e1061 {
                        if _e1061 {
                            if (_e1059.member_1 == 1u) {
                                phi_13475_ = true;
                            } else {
                                phi_13475_ = ((((-(_e340.member_1.x) * _e984.member_1.x) + (-(_e340.member_1.y) * _e984.member_1.y)) + (-(_e340.member_1.z) * _e984.member_1.z)) > 0f);
                            }
                            let _e6416 = phi_13475_;
                            if _e6416 {
                                phi_13484_ = vec3<f32>(_e1059.member_4.x, _e1059.member_4.y, _e1059.member_4.z);
                            } else {
                                phi_13484_ = vec3<f32>();
                            }
                            let _e6423 = phi_13484_;
                            phi_13486_ = _e6423;
                            phi_13487_ = select(true, false, _e6416);
                        } else {
                            phi_13486_ = vec3<f32>();
                            phi_13487_ = true;
                        }
                        let _e6426 = phi_13486_;
                        let _e6428 = phi_13487_;
                        let _e6430 = select(_e6426, vec3<f32>(0f, 0f, 0f), vec3(_e6428));
                        if _e379 {
                            phi_13918_ = 1f;
                        } else {
                            if ((_e387 & 1u) == 0u) {
                                phi_13917_ = 1f;
                            } else {
                                if (_e998 > 0u) {
                                    let _e6437 = global_1.member.member_2[1u];
                                    let _e6438 = (_e6437 + (_e998 - 1u));
                                    let _e6441 = global_6.member[(_e6438 + 1u)];
                                    let _e6443 = global_6.member[_e6438];
                                    if (_e858 == 1u) {
                                        let _e6504 = (_e849.x - _e340.member.x);
                                        let _e6505 = (_e849.y - _e340.member.y);
                                        let _e6506 = (_e849.z - _e340.member.z);
                                        let _e6511 = (((_e6504 * _e6504) + (_e6505 * _e6505)) + (_e6506 * _e6506));
                                        let _e6512 = (_e849.w * _e849.w);
                                        if (_e6511 <= (_e6512 * 1.000001f)) {
                                            let _e6526 = (_e837 - _e340.member.x);
                                            let _e6527 = (_e838 - _e340.member.y);
                                            let _e6528 = (_e839 - _e340.member.z);
                                            let _e6533 = (((_e6526 * _e6526) + (_e6527 * _e6527)) + (_e6528 * _e6528));
                                            phi_32493_ = (_e6533 / (((abs((((_e984.member.x * _e6526) + (_e984.member.y * _e6527)) + (_e984.member.z * _e6528))) / sqrt(_e6533)) * 12.566371f) * _e6512));
                                        } else {
                                            let _e6515 = (_e6512 / _e6511);
                                            if (_e6515 != _e6515) {
                                                phi_32512_ = true;
                                            } else {
                                                phi_32512_ = (1f <= _e6515);
                                            }
                                            let _e6519 = phi_32512_;
                                            let _e6520 = select(_e6515, 1f, _e6519);
                                            phi_32493_ = (0.15915494f / (_e6520 / (1f + sqrt((1f - _e6520)))));
                                        }
                                        let _e6549 = phi_32493_;
                                        phi_13909_ = _e6549;
                                    } else {
                                        let _e6446 = (_e837 - _e340.member.x);
                                        let _e6447 = (_e838 - _e340.member.y);
                                        let _e6448 = (_e839 - _e340.member.z);
                                        let _e6453 = (((_e6446 * _e6446) + (_e6447 * _e6447)) + (_e6448 * _e6448));
                                        let _e6464 = (abs((((_e984.member.x * _e6446) + (_e984.member.y * _e6447)) + (_e984.member.z * _e6448))) / sqrt(_e6453));
                                        if (_e6464 <= 0f) {
                                            phi_13908_ = 0f;
                                        } else {
                                            if _e891 {
                                                let _e6472 = (_e863.x - _e849.x);
                                                let _e6473 = (_e863.y - _e849.y);
                                                let _e6474 = (_e863.z - _e849.z);
                                                let _e6475 = (_e866.x - _e849.x);
                                                let _e6476 = (_e866.y - _e849.y);
                                                let _e6477 = (_e866.z - _e849.z);
                                                let _e6480 = ((_e6473 * _e6477) - (_e6476 * _e6474));
                                                let _e6483 = ((_e6474 * _e6475) - (_e6477 * _e6472));
                                                let _e6486 = ((_e6472 * _e6476) - (_e6475 * _e6473));
                                                phi_13905_ = (sqrt((((_e6480 * _e6480) + (_e6483 * _e6483)) + (_e6486 * _e6486))) * 0.5f);
                                            } else {
                                                phi_13905_ = ((12.566371f * _e849.w) * _e849.w);
                                            }
                                            let _e6495 = phi_13905_;
                                            phi_13908_ = (_e6453 / (_e6464 * _e6495));
                                        }
                                        let _e6499 = phi_13908_;
                                        phi_13909_ = _e6499;
                                    }
                                    let _e6551 = phi_13909_;
                                    let _e6552 = ((_e6441 - _e6443) * _e6551);
                                    if _e381 {
                                        phi_13915_ = 0f;
                                    } else {
                                        if (_e340.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_32533_ = 1f;
                                        } else {
                                            let _e6554 = (_e340.member_1.w * _e340.member_1.w);
                                            let _e6556 = (_e6554 + (_e6552 * _e6552));
                                            if (_e6556 == 0f) {
                                                phi_32530_ = 0f;
                                            } else {
                                                phi_32530_ = (_e6554 / _e6556);
                                            }
                                            let _e6560 = phi_32530_;
                                            phi_32533_ = _e6560;
                                        }
                                        let _e6562 = phi_32533_;
                                        phi_13915_ = _e6562;
                                    }
                                    let _e6564 = phi_13915_;
                                    phi_13916_ = _e6564;
                                } else {
                                    phi_13916_ = 1f;
                                }
                                let _e6566 = phi_13916_;
                                phi_13917_ = _e6566;
                            }
                            let _e6568 = phi_13917_;
                            phi_13918_ = _e6568;
                        }
                        let _e6570 = phi_13918_;
                        phi_19857_ = type_69(type_34(_e340.member, _e340.member_1, vec4<f32>(_e831.x, _e831.y, _e831.z, f32(_e365)), vec4<f32>((_e340.member_3.x + ((_e831.x * _e6430.x) * _e6570)), (_e340.member_3.y + ((_e831.y * _e6430.y) * _e6570)), (_e340.member_3.z + ((_e831.z * _e6430.z) * _e6570)), f32(_e393)), vec4<u32>(_e340.member_4.x, (_e340.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19858_ = type_69();
                        phi_19859_ = true;
                    } else {
                        if (_e1059.member == 7u) {
                            let _e6374 = type_34(_e340.member, _e340.member_1, vec4<f32>(_e831.x, _e831.y, _e831.z, f32(_e365)), vec4<f32>(_e340.member_3.x, _e340.member_3.y, _e340.member_3.z, f32(_e393)), vec4<u32>(_e340.member_4.x, ((_e340.member_4.y | 16u) & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f));
                            let _e6386 = type_34(_e6374.member, _e6374.member_1, _e6374.member_2, _e6374.member_3, vec4<u32>(_e6374.member_4.x, _e6374.member_4.y, _e340.member_4.z, _e6374.member_4.w), _e6374.member_5);
                            phi_19854_ = type_69(type_34(_e6386.member, _e6386.member_1, _e6386.member_2, _e6386.member_3, _e6386.member_4, _e340.member_5), _e833, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19855_ = type_69();
                            phi_19856_ = true;
                        } else {
                            let _e1063 = -(_e340.member_1.x);
                            let _e1064 = -(_e340.member_1.y);
                            let _e1065 = -(_e340.member_1.z);
                            if ((((_e984.member.x * _e1063) + (_e984.member.y * _e1064)) + (_e984.member.z * _e1065)) < 0f) {
                                phi_13970_ = -(_e984.member);
                            } else {
                                phi_13970_ = _e984.member;
                            }
                            let _e1077 = phi_13970_;
                            if ((((_e984.member_1.x * _e1063) + (_e984.member_1.y * _e1064)) + (_e984.member_1.z * _e1065)) < 0f) {
                                phi_13984_ = -(_e984.member_1);
                            } else {
                                phi_13984_ = _e984.member_1;
                            }
                            let _e1089 = phi_13984_;
                            let _e1092 = global_1.member.member_4[2u];
                            let _e1093 = abs(_e837);
                            let _e1094 = abs(_e838);
                            let _e1095 = abs(_e839);
                            let _e1097 = select(_e1094, _e1093, (_e1093 > _e1094));
                            let _e1101 = (0.00002f * (1f + select(_e1095, _e1097, (_e1097 > _e1095))));
                            if (_e1092 != _e1092) {
                                phi_26803_ = true;
                            } else {
                                phi_26803_ = (_e1101 >= _e1092);
                            }
                            let _e1105 = phi_26803_;
                            let _e1107 = (select(_e1092, _e1101, _e1105) * 2f);
                            let _e1114 = (_e837 + (_e1077.x * _e1107));
                            let _e1115 = (_e838 + (_e1077.y * _e1107));
                            let _e1116 = (_e839 + (_e1077.z * _e1107));
                            let _e1117 = vec3<f32>(_e1114, _e1115, _e1116);
                            if ((_e340.member_4.y & 4u) == 0u) {
                                phi_14017_ = 0f;
                            } else {
                                let _e1122 = global_1.member.member_4[0u];
                                phi_14017_ = _e1122;
                            }
                            let _e1124 = phi_14017_;
                            let _e1127 = select(-1f, 1f, (_e1089.z >= 0f));
                            let _e1129 = (-1f / (_e1127 + _e1089.z));
                            let _e1133 = ((_e1089.x * _e1089.y) * _e1129);
                            let _e1137 = (1f + (((_e1127 * _e1089.x) * _e1089.x) * _e1129));
                            let _e1138 = (_e1127 * _e1133);
                            let _e1140 = (-(_e1127) * _e1089.x);
                            let _e1143 = (_e1127 + ((_e1089.y * _e1089.y) * _e1129));
                            let _e1144 = -(_e1089.y);
                            if (_e1059.member == 0u) {
                                phi_35002_ = vec3<f32>(0f, 0f, 0f);
                                phi_34810_ = 1.5f;
                                phi_34731_ = vec3<f32>(0f, 0f, 0f);
                                phi_34654_ = vec3<f32>(_e1059.member_4.x, _e1059.member_4.y, _e1059.member_4.z);
                                phi_34575_ = 0u;
                                phi_34493_ = 1f;
                                phi_34416_ = 0u;
                                phi_34407_ = _e340.member_4.x;
                                phi_14696_ = type_69();
                                phi_14697_ = true;
                            } else {
                                if (_e1059.member == 1u) {
                                    let _e1693 = (_e1059.member_6.y * _e1059.member_6.y);
                                    if (_e1693 != _e1693) {
                                        phi_27495_ = true;
                                    } else {
                                        phi_27495_ = (_e1124 >= _e1693);
                                    }
                                    let _e1697 = phi_27495_;
                                    let _e1698 = select(_e1693, _e1124, _e1697);
                                    let _e1699 = (_e1698 >= 0.001f);
                                    if _e1699 {
                                        phi_35059_ = vec3<f32>(_e1059.member_4.x, _e1059.member_4.y, _e1059.member_4.z);
                                        phi_34853_ = _e1059.member_6.x;
                                        phi_34706_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34394_ = _e340.member_4.x;
                                        phi_14092_ = vec3<f32>();
                                        phi_14093_ = true;
                                    } else {
                                        let _e1701 = ((_e340.member_4.x * 747796405u) + 2891336453u);
                                        let _e1709 = (((_e1701 >> bitcast<u32>((((_e1701 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1701) * 277803737u);
                                        let _e1712 = ((_e1709 >> bitcast<u32>(22i)) ^ _e1709);
                                        let _e1722 = abs((((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z)));
                                        let _e1725 = (1f / _e1059.member_6.x);
                                        let _e1729 = ((_e1725 * _e1725) * (1f - (_e1722 * _e1722)));
                                        if (_e1729 >= 1f) {
                                            phi_27535_ = 1f;
                                        } else {
                                            let _e1732 = sqrt((1f - _e1729));
                                            if (_e1722 != _e1722) {
                                                phi_27546_ = true;
                                            } else {
                                                phi_27546_ = (_e1732 <= _e1722);
                                            }
                                            let _e1736 = phi_27546_;
                                            let _e1741 = pow(((1f - _e1059.member_6.x) / (1f + _e1059.member_6.x)), 2f);
                                            phi_27535_ = (_e1741 + ((1f - _e1741) * pow((1f - select(_e1722, _e1732, _e1736)), 5f)));
                                        }
                                        let _e1748 = phi_27535_;
                                        let _e1749 = ((f32((_e1712 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1748);
                                        if _e1749 {
                                            let _e1762 = (((_e1089.x * _e340.member_1.x) + (_e1089.y * _e340.member_1.y)) + (_e1089.z * _e340.member_1.z));
                                            phi_34707_ = vec3<f32>(0f, 0f, 0f);
                                            phi_14090_ = (_e354 - vec3<f32>(((_e1089.x * 2f) * _e1762), ((_e1089.y * 2f) * _e1762), ((_e1089.z * 2f) * _e1762)));
                                        } else {
                                            phi_34707_ = vec3<f32>(_e1059.member_4.x, _e1059.member_4.y, _e1059.member_4.z);
                                            phi_14090_ = vec3<f32>();
                                        }
                                        let _e1769 = phi_34707_;
                                        let _e1771 = phi_14090_;
                                        phi_35059_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34853_ = 1.5f;
                                        phi_34706_ = _e1769;
                                        phi_34394_ = _e1712;
                                        phi_14092_ = _e1771;
                                        phi_14093_ = select(true, false, _e1749);
                                    }
                                    let _e1781 = phi_35059_;
                                    let _e1783 = phi_34853_;
                                    let _e1785 = phi_34706_;
                                    let _e1787 = phi_34394_;
                                    let _e1789 = phi_14092_;
                                    let _e1791 = phi_14093_;
                                    phi_35005_ = _e1781;
                                    phi_34813_ = _e1783;
                                    phi_34734_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34657_ = _e1785;
                                    phi_34578_ = select(0u, 1u, _e1699);
                                    phi_34496_ = select(1f, _e1698, _e1699);
                                    phi_34419_ = select(0u, 2u, _e1699);
                                    phi_34345_ = _e1787;
                                    phi_14565_ = _e1117;
                                    phi_14566_ = _e1789;
                                    phi_14567_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14568_ = _e1791;
                                } else {
                                    if (_e1059.member == 3u) {
                                        phi_34348_ = _e340.member_4.x;
                                        phi_14244_ = vec3<f32>();
                                        phi_14245_ = vec3<f32>();
                                        phi_14246_ = vec3<f32>();
                                        phi_14247_ = true;
                                        phi_14248_ = false;
                                    } else {
                                        let _e1148 = (_e1059.member == 5u);
                                        if _e1148 {
                                            phi_34349_ = _e340.member_4.x;
                                            phi_14240_ = vec3<f32>();
                                            phi_14241_ = vec3<f32>();
                                            phi_14242_ = vec3<f32>();
                                            phi_14243_ = false;
                                        } else {
                                            let _e1149 = (_e1059.member == 2u);
                                            if _e1149 {
                                                let _e1154 = (((_e340.member_1.x * _e984.member_1.x) + (_e340.member_1.y * _e984.member_1.y)) + (_e340.member_1.z * _e984.member_1.z));
                                                let _e1155 = (_e1154 > 0f);
                                                if _e1155 {
                                                    phi_14125_ = -(_e984.member_1);
                                                } else {
                                                    phi_14125_ = _e984.member_1;
                                                }
                                                let _e1158 = phi_14125_;
                                                if _e1155 {
                                                    phi_14131_ = _e1059.member_6.x;
                                                } else {
                                                    phi_14131_ = (1f / _e1059.member_6.x);
                                                }
                                                let _e1163 = phi_14131_;
                                                let _e1164 = abs(_e1154);
                                                let _e1165 = (_e1163 * _e1163);
                                                let _e1168 = (_e1165 * (1f - (_e1164 * _e1164)));
                                                if (_e1168 >= 1f) {
                                                    phi_26865_ = 1f;
                                                } else {
                                                    let _e1171 = sqrt((1f - _e1168));
                                                    if (_e1164 != _e1164) {
                                                        phi_26876_ = true;
                                                    } else {
                                                        phi_26876_ = (_e1171 <= _e1164);
                                                    }
                                                    let _e1175 = phi_26876_;
                                                    let _e1180 = pow(((1f - _e1059.member_6.x) / (1f + _e1059.member_6.x)), 2f);
                                                    phi_26865_ = (_e1180 + ((1f - _e1180) * pow((1f - select(_e1164, _e1171, _e1175)), 5f)));
                                                }
                                                let _e1187 = phi_26865_;
                                                let _e1195 = (((_e340.member_1.x * _e1158.x) + (_e340.member_1.y * _e1158.y)) + (_e340.member_1.z * _e1158.z));
                                                let _e1199 = (1f - (_e1165 * (1f - (_e1195 * _e1195))));
                                                if (_e1199 > 0f) {
                                                    let _e1202 = ((_e340.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1210 = (((_e1202 >> bitcast<u32>((((_e1202 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1202) * 277803737u);
                                                    let _e1213 = ((_e1210 >> bitcast<u32>(22i)) ^ _e1210);
                                                    let _e1218 = ((f32((_e1213 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1187);
                                                    if _e1218 {
                                                        let _e1228 = sqrt(_e1199);
                                                        let _e1232 = (((_e340.member_1.x - (_e1158.x * _e1195)) * _e1163) - (_e1158.x * _e1228));
                                                        let _e1233 = (((_e340.member_1.y - (_e1158.y * _e1195)) * _e1163) - (_e1158.y * _e1228));
                                                        let _e1234 = (((_e340.member_1.z - (_e1158.z * _e1195)) * _e1163) - (_e1158.z * _e1228));
                                                        let _e1241 = (1f / sqrt((((_e1232 * _e1232) + (_e1233 * _e1233)) + (_e1234 * _e1234))));
                                                        phi_14212_ = vec3<f32>((_e837 - (_e1158.x * _e1107)), (_e838 - (_e1158.y * _e1107)), (_e839 - (_e1158.z * _e1107)));
                                                        phi_14213_ = vec3<f32>((_e1232 * _e1241), (_e1233 * _e1241), (_e1234 * _e1241));
                                                        phi_14214_ = vec3<f32>(_e1059.member_4.x, _e1059.member_4.y, _e1059.member_4.z);
                                                    } else {
                                                        phi_14212_ = vec3<f32>();
                                                        phi_14213_ = vec3<f32>();
                                                        phi_14214_ = vec3<f32>();
                                                    }
                                                    let _e1259 = phi_14212_;
                                                    let _e1261 = phi_14213_;
                                                    let _e1263 = phi_14214_;
                                                    phi_34352_ = _e1213;
                                                    phi_14216_ = _e1259;
                                                    phi_14217_ = _e1261;
                                                    phi_14218_ = _e1263;
                                                    phi_14219_ = select(true, false, _e1218);
                                                } else {
                                                    phi_34352_ = _e340.member_4.x;
                                                    phi_14216_ = vec3<f32>();
                                                    phi_14217_ = vec3<f32>();
                                                    phi_14218_ = vec3<f32>();
                                                    phi_14219_ = true;
                                                }
                                                let _e1266 = phi_34352_;
                                                let _e1268 = phi_14216_;
                                                let _e1270 = phi_14217_;
                                                let _e1272 = phi_14218_;
                                                let _e1274 = phi_14219_;
                                                if _e1274 {
                                                    let _e1282 = (((_e984.member_1.x * _e340.member_1.x) + (_e984.member_1.y * _e340.member_1.y)) + (_e984.member_1.z * _e340.member_1.z));
                                                    phi_14234_ = vec3<f32>((_e837 + (_e1158.x * _e1107)), (_e838 + (_e1158.y * _e1107)), (_e839 + (_e1158.z * _e1107)));
                                                    phi_14235_ = (_e354 - vec3<f32>(((_e984.member_1.x * 2f) * _e1282), ((_e984.member_1.y * 2f) * _e1282), ((_e984.member_1.z * 2f) * _e1282)));
                                                } else {
                                                    phi_14234_ = _e1268;
                                                    phi_14235_ = _e1270;
                                                }
                                                let _e1296 = phi_14234_;
                                                let _e1298 = phi_14235_;
                                                phi_34350_ = _e1266;
                                                phi_14237_ = _e1296;
                                                phi_14238_ = _e1298;
                                                phi_14239_ = select(_e1272, vec3<f32>(1f, 1f, 1f), vec3(_e1274));
                                            } else {
                                                phi_34350_ = _e340.member_4.x;
                                                phi_14237_ = vec3<f32>();
                                                phi_14238_ = vec3<f32>();
                                                phi_14239_ = vec3<f32>();
                                            }
                                            let _e1302 = phi_34350_;
                                            let _e1304 = phi_14237_;
                                            let _e1306 = phi_14238_;
                                            let _e1308 = phi_14239_;
                                            phi_34349_ = _e1302;
                                            phi_14240_ = _e1304;
                                            phi_14241_ = _e1306;
                                            phi_14242_ = _e1308;
                                            phi_14243_ = _e1149;
                                        }
                                        let _e1310 = phi_34349_;
                                        let _e1312 = phi_14240_;
                                        let _e1314 = phi_14241_;
                                        let _e1316 = phi_14242_;
                                        let _e1318 = phi_14243_;
                                        phi_34348_ = _e1310;
                                        phi_14244_ = _e1312;
                                        phi_14245_ = _e1314;
                                        phi_14246_ = _e1316;
                                        phi_14247_ = _e1148;
                                        phi_14248_ = _e1318;
                                    }
                                    let _e1320 = phi_34348_;
                                    let _e1322 = phi_14244_;
                                    let _e1324 = phi_14245_;
                                    let _e1326 = phi_14246_;
                                    let _e1328 = phi_14247_;
                                    let _e1330 = phi_14248_;
                                    if _e1328 {
                                        let _e1333 = (_e1059.member_6.y * _e1059.member_6.y);
                                        if (_e1333 != _e1333) {
                                            phi_26949_ = true;
                                        } else {
                                            phi_26949_ = (_e1124 >= _e1333);
                                        }
                                        let _e1337 = phi_26949_;
                                        let _e1338 = select(_e1333, _e1124, _e1337);
                                        let _e1350 = select(2u, 0u, (_e1059.member == 5u));
                                        let _e1351 = (_e1338 < 0.001f);
                                        if _e1351 {
                                            let _e1359 = (((_e1089.x * _e340.member_1.x) + (_e1089.y * _e340.member_1.y)) + (_e1089.z * _e340.member_1.z));
                                            let _e1370 = abs((((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z)));
                                            if (_e1350 == 0u) {
                                                let _e1652 = (1f - _e1370);
                                                if (_e1652 != _e1652) {
                                                    phi_27480_ = true;
                                                } else {
                                                    phi_27480_ = (0f >= _e1652);
                                                }
                                                let _e1656 = phi_27480_;
                                                let _e1658 = pow(select(_e1652, 0f, _e1656), 5f);
                                                let _e1659 = (1f - _e1658);
                                                phi_14551_ = vec3<f32>(((_e1059.member_4.x * _e1659) + _e1658), ((_e1059.member_4.y * _e1659) + _e1658), ((_e1059.member_4.z * _e1659) + _e1658));
                                            } else {
                                                if (_e1350 == 1u) {
                                                    let _e1634 = (0.44444448f * (1f - (_e1370 * _e1370)));
                                                    if (_e1634 >= 1f) {
                                                        phi_27441_ = 1f;
                                                    } else {
                                                        let _e1637 = sqrt((1f - _e1634));
                                                        if (_e1370 != _e1370) {
                                                            phi_27452_ = true;
                                                        } else {
                                                            phi_27452_ = (_e1637 <= _e1370);
                                                        }
                                                        let _e1641 = phi_27452_;
                                                        phi_27441_ = (0.040000003f + (0.96f * pow((1f - select(_e1370, _e1637, _e1641)), 5f)));
                                                    }
                                                    let _e1648 = phi_27441_;
                                                    phi_14550_ = vec3(_e1648);
                                                } else {
                                                    let _e1374 = select(_e1370, 0f, (_e1370 < 0f));
                                                    let _e1376 = select(_e1374, 1f, (_e1374 > 1f));
                                                    let _e1378 = (1f - (_e1376 * _e1376));
                                                    let _e1385 = ((_e1059.member_4.x * _e1059.member_4.x) - (_e1059.member_5.x * _e1059.member_5.x));
                                                    let _e1386 = ((_e1059.member_4.y * _e1059.member_4.y) - (_e1059.member_5.y * _e1059.member_5.y));
                                                    let _e1387 = ((_e1059.member_4.z * _e1059.member_4.z) - (_e1059.member_5.z * _e1059.member_5.z));
                                                    let _e1389 = (_e1059.member_4.x * (_e1059.member_5.x + _e1059.member_5.x));
                                                    let _e1391 = (_e1059.member_4.y * (_e1059.member_5.y + _e1059.member_5.y));
                                                    let _e1393 = (_e1059.member_4.z * (_e1059.member_5.z + _e1059.member_5.z));
                                                    let _e1400 = ((_e1385 * _e1385) + (_e1389 * _e1389));
                                                    let _e1401 = ((_e1386 * _e1386) + (_e1391 * _e1391));
                                                    let _e1402 = ((_e1387 * _e1387) + (_e1393 * _e1393));
                                                    let _e1407 = ((_e1378 * _e1385) / _e1400);
                                                    let _e1408 = ((_e1378 * _e1386) / _e1401);
                                                    let _e1409 = ((_e1378 * _e1387) / _e1402);
                                                    let _e1410 = (1f - _e1407);
                                                    let _e1411 = (1f - _e1408);
                                                    let _e1412 = (1f - _e1409);
                                                    let _e1414 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1407, _e1408, _e1409));
                                                    let _e1415 = (_e1378 * _e1389);
                                                    let _e1416 = (_e1378 * _e1391);
                                                    let _e1417 = (_e1378 * _e1393);
                                                    let _e1422 = (vec3<f32>(_e1415, _e1416, _e1417) / vec3<f32>(_e1400, _e1401, _e1402));
                                                    let _e1435 = ((_e1414.x * _e1414.x) + (_e1422.x * _e1422.x));
                                                    let _e1436 = ((_e1414.y * _e1414.y) + (_e1422.y * _e1422.y));
                                                    let _e1437 = ((_e1414.z * _e1414.z) + (_e1422.z * _e1422.z));
                                                    if (_e1435 != _e1435) {
                                                        phi_27111_ = true;
                                                    } else {
                                                        phi_27111_ = (0f >= _e1435);
                                                    }
                                                    let _e1441 = phi_27111_;
                                                    let _e1443 = sqrt(select(_e1435, 0f, _e1441));
                                                    if (_e1436 != _e1436) {
                                                        phi_27126_ = true;
                                                    } else {
                                                        phi_27126_ = (0f >= _e1436);
                                                    }
                                                    let _e1447 = phi_27126_;
                                                    let _e1449 = sqrt(select(_e1436, 0f, _e1447));
                                                    if (_e1437 != _e1437) {
                                                        phi_27141_ = true;
                                                    } else {
                                                        phi_27141_ = (0f >= _e1437);
                                                    }
                                                    let _e1453 = phi_27141_;
                                                    let _e1455 = sqrt(select(_e1437, 0f, _e1453));
                                                    let _e1459 = ((_e1443 + _e1410) * 0.5f);
                                                    let _e1460 = ((_e1449 + _e1411) * 0.5f);
                                                    let _e1461 = ((_e1455 + _e1412) * 0.5f);
                                                    if (_e1459 != _e1459) {
                                                        phi_27168_ = true;
                                                    } else {
                                                        phi_27168_ = (0f >= _e1459);
                                                    }
                                                    let _e1465 = phi_27168_;
                                                    let _e1467 = sqrt(select(_e1459, 0f, _e1465));
                                                    if (_e1460 != _e1460) {
                                                        phi_27183_ = true;
                                                    } else {
                                                        phi_27183_ = (0f >= _e1460);
                                                    }
                                                    let _e1471 = phi_27183_;
                                                    let _e1473 = sqrt(select(_e1460, 0f, _e1471));
                                                    if (_e1461 != _e1461) {
                                                        phi_27198_ = true;
                                                    } else {
                                                        phi_27198_ = (0f >= _e1461);
                                                    }
                                                    let _e1477 = phi_27198_;
                                                    let _e1479 = sqrt(select(_e1461, 0f, _e1477));
                                                    let _e1480 = vec3<f32>(_e1467, _e1473, _e1479);
                                                    let _e1484 = ((_e1443 - _e1410) * 0.5f);
                                                    let _e1485 = ((_e1449 - _e1411) * 0.5f);
                                                    let _e1486 = ((_e1455 - _e1412) * 0.5f);
                                                    if (_e1484 != _e1484) {
                                                        phi_27225_ = true;
                                                    } else {
                                                        phi_27225_ = (0f >= _e1484);
                                                    }
                                                    let _e1490 = phi_27225_;
                                                    let _e1492 = sqrt(select(_e1484, 0f, _e1490));
                                                    if (_e1485 != _e1485) {
                                                        phi_27240_ = true;
                                                    } else {
                                                        phi_27240_ = (0f >= _e1485);
                                                    }
                                                    let _e1496 = phi_27240_;
                                                    let _e1498 = sqrt(select(_e1485, 0f, _e1496));
                                                    if (_e1486 != _e1486) {
                                                        phi_27255_ = true;
                                                    } else {
                                                        phi_27255_ = (0f >= _e1486);
                                                    }
                                                    let _e1502 = phi_27255_;
                                                    let _e1504 = sqrt(select(_e1486, 0f, _e1502));
                                                    if ((_e1415 / _e1400) < 0f) {
                                                        phi_14427_ = -(_e1492);
                                                    } else {
                                                        phi_14427_ = _e1492;
                                                    }
                                                    let _e1508 = phi_14427_;
                                                    if ((_e1416 / _e1401) < 0f) {
                                                        phi_14435_ = -(_e1498);
                                                    } else {
                                                        phi_14435_ = _e1498;
                                                    }
                                                    let _e1512 = phi_14435_;
                                                    if ((_e1417 / _e1402) < 0f) {
                                                        phi_14443_ = -(_e1504);
                                                    } else {
                                                        phi_14443_ = _e1504;
                                                    }
                                                    let _e1516 = phi_14443_;
                                                    let _e1520 = (_e1059.member_5.x * _e1376);
                                                    let _e1521 = (_e1059.member_5.y * _e1376);
                                                    let _e1522 = (_e1059.member_5.z * _e1376);
                                                    let _e1532 = vec3<f32>(((_e1059.member_4.x * _e1467) - (_e1059.member_5.x * _e1508)), ((_e1059.member_4.y * _e1473) - (_e1059.member_5.y * _e1512)), ((_e1059.member_4.z * _e1479) - (_e1059.member_5.z * _e1516)));
                                                    let _e1539 = ((_e1059.member_4.x * _e1508) + (_e1059.member_5.x * _e1467));
                                                    let _e1540 = ((_e1059.member_4.y * _e1512) + (_e1059.member_5.y * _e1473));
                                                    let _e1541 = ((_e1059.member_4.z * _e1516) + (_e1059.member_5.z * _e1479));
                                                    let _e1543 = vec3<f32>((_e1059.member_4.x * _e1376), (_e1059.member_4.y * _e1376), (_e1059.member_4.z * _e1376));
                                                    let _e1544 = (_e1543 - _e1480);
                                                    let _e1545 = (_e1520 - _e1508);
                                                    let _e1546 = (_e1521 - _e1512);
                                                    let _e1547 = (_e1522 - _e1516);
                                                    let _e1560 = (_e1543 + _e1480);
                                                    let _e1561 = (_e1520 + _e1508);
                                                    let _e1562 = (_e1521 + _e1512);
                                                    let _e1563 = (_e1522 + _e1516);
                                                    let _e1573 = ((_e1560.x * _e1560.x) + (_e1561 * _e1561));
                                                    let _e1574 = ((_e1560.y * _e1560.y) + (_e1562 * _e1562));
                                                    let _e1575 = ((_e1560.z * _e1560.z) + (_e1563 * _e1563));
                                                    let _e1585 = vec3(_e1376);
                                                    let _e1586 = (_e1585 - _e1532);
                                                    let _e1587 = -(vec3<f32>(_e1539, _e1540, _e1541));
                                                    let _e1603 = (_e1585 + _e1532);
                                                    let _e1613 = ((_e1603.x * _e1603.x) + (_e1539 * _e1539));
                                                    let _e1614 = ((_e1603.y * _e1603.y) + (_e1540 * _e1540));
                                                    let _e1615 = ((_e1603.z * _e1603.z) + (_e1541 * _e1541));
                                                    phi_14550_ = vec3<f32>((((((_e1544.x * _e1544.x) + (_e1545 * _e1545)) / select(0.000000000000000000000000000001f, _e1573, (_e1573 > 0.000000000000000000000000000001f))) + (((_e1586.x * _e1586.x) + (_e1587.x * _e1587.x)) / select(0.000000000000000000000000000001f, _e1613, (_e1613 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1544.y * _e1544.y) + (_e1546 * _e1546)) / select(0.000000000000000000000000000001f, _e1574, (_e1574 > 0.000000000000000000000000000001f))) + (((_e1586.y * _e1586.y) + (_e1587.y * _e1587.y)) / select(0.000000000000000000000000000001f, _e1614, (_e1614 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1544.z * _e1544.z) + (_e1547 * _e1547)) / select(0.000000000000000000000000000001f, _e1575, (_e1575 > 0.000000000000000000000000000001f))) + (((_e1586.z * _e1586.z) + (_e1587.z * _e1587.z)) / select(0.000000000000000000000000000001f, _e1615, (_e1615 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e1651 = phi_14550_;
                                                phi_14551_ = _e1651;
                                            }
                                            let _e1668 = phi_14551_;
                                            phi_14552_ = (_e354 - vec3<f32>(((_e1089.x * 2f) * _e1359), ((_e1089.y * 2f) * _e1359), ((_e1089.z * 2f) * _e1359)));
                                            phi_14553_ = _e1668;
                                        } else {
                                            phi_14552_ = vec3<f32>();
                                            phi_14553_ = vec3<f32>();
                                        }
                                        let _e1670 = phi_14552_;
                                        let _e1672 = phi_14553_;
                                        phi_34736_ = vec3<f32>(_e1059.member_5.x, _e1059.member_5.y, _e1059.member_5.z);
                                        phi_34659_ = vec3<f32>(_e1059.member_4.x, _e1059.member_4.y, _e1059.member_4.z);
                                        phi_34580_ = _e1350;
                                        phi_34498_ = _e1338;
                                        phi_14555_ = _e1670;
                                        phi_14556_ = _e1672;
                                        phi_14557_ = _e1351;
                                    } else {
                                        phi_34736_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34659_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34580_ = 0u;
                                        phi_34498_ = 1f;
                                        phi_14555_ = _e1324;
                                        phi_14556_ = _e1326;
                                        phi_14557_ = _e1330;
                                    }
                                    let _e1674 = phi_34736_;
                                    let _e1676 = phi_34659_;
                                    let _e1678 = phi_34580_;
                                    let _e1680 = phi_34498_;
                                    let _e1682 = phi_14555_;
                                    let _e1684 = phi_14556_;
                                    let _e1686 = phi_14557_;
                                    phi_35005_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34813_ = 1.5f;
                                    phi_34734_ = _e1674;
                                    phi_34657_ = _e1676;
                                    phi_34578_ = _e1678;
                                    phi_34496_ = _e1680;
                                    phi_34419_ = select(0u, 1u, _e1328);
                                    phi_34345_ = _e1320;
                                    phi_14565_ = select(_e1322, _e1117, vec3(_e1328));
                                    phi_14566_ = _e1682;
                                    phi_14567_ = _e1684;
                                    phi_14568_ = select(true, false, _e1686);
                                }
                                let _e1796 = phi_35005_;
                                let _e1798 = phi_34813_;
                                let _e1800 = phi_34734_;
                                let _e1802 = phi_34657_;
                                let _e1804 = phi_34578_;
                                let _e1806 = phi_34496_;
                                let _e1808 = phi_34419_;
                                let _e1810 = phi_34345_;
                                let _e1812 = phi_14565_;
                                let _e1814 = phi_14566_;
                                let _e1816 = phi_14567_;
                                let _e1818 = phi_14568_;
                                let _e1819 = select(true, false, _e1818);
                                if _e1819 {
                                    if (_e365 >= _e384) {
                                        phi_34404_ = _e1810;
                                        phi_14685_ = _e831;
                                        phi_14686_ = type_69();
                                        phi_14687_ = true;
                                    } else {
                                        let _e1832 = global_1.member.member_4[1u];
                                        if (sqrt((((_e1816.x * _e1816.x) + (_e1816.y * _e1816.y)) + (_e1816.z * _e1816.z))) <= _e1832) {
                                            phi_34399_ = _e1810;
                                            phi_14648_ = _e831;
                                            phi_14649_ = false;
                                        } else {
                                            let _e1836 = global_1.member.member_7[1u];
                                            if (_e1836 == 1u) {
                                                let _e1841 = global_1.member.member_4[3u];
                                                if (f32(_e365) >= _e1841) {
                                                    let _e1844 = select(_e1816.y, _e1816.x, (_e1816.x > _e1816.y));
                                                    let _e1846 = select(_e1816.z, _e1844, (_e1844 > _e1816.z));
                                                    if (_e1846 != _e1846) {
                                                        phi_27612_ = true;
                                                    } else {
                                                        phi_27612_ = (1f <= _e1846);
                                                    }
                                                    let _e1850 = phi_27612_;
                                                    let _e1851 = select(_e1846, 1f, _e1850);
                                                    let _e1853 = ((_e1810 * 747796405u) + 2891336453u);
                                                    let _e1861 = (((_e1853 >> bitcast<u32>((((_e1853 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1853) * 277803737u);
                                                    let _e1864 = ((_e1861 >> bitcast<u32>(22i)) ^ _e1861);
                                                    let _e1869 = ((f32((_e1864 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1851);
                                                    if _e1869 {
                                                        phi_14637_ = vec3<f32>();
                                                    } else {
                                                        phi_14637_ = vec3<f32>((_e831.x / _e1851), (_e831.y / _e1851), (_e831.z / _e1851));
                                                    }
                                                    let _e1878 = phi_14637_;
                                                    phi_34402_ = _e1864;
                                                    phi_14639_ = _e1878;
                                                    phi_14640_ = select(true, false, _e1869);
                                                } else {
                                                    phi_34402_ = _e1810;
                                                    phi_14639_ = _e831;
                                                    phi_14640_ = true;
                                                }
                                                let _e1881 = phi_34402_;
                                                let _e1883 = phi_14639_;
                                                let _e1885 = phi_14640_;
                                                phi_34401_ = _e1881;
                                                phi_14641_ = _e1883;
                                                phi_14642_ = _e1885;
                                            } else {
                                                phi_34401_ = _e1810;
                                                phi_14641_ = _e831;
                                                phi_14642_ = true;
                                            }
                                            let _e1887 = phi_34401_;
                                            let _e1889 = phi_14641_;
                                            let _e1891 = phi_14642_;
                                            phi_34399_ = _e1887;
                                            phi_14648_ = select(_e831, _e1889, vec3(_e1891));
                                            phi_14649_ = select(false, true, _e1891);
                                        }
                                        let _e1896 = phi_34399_;
                                        let _e1898 = phi_14648_;
                                        let _e1900 = phi_14649_;
                                        if _e1900 {
                                            let _e1901 = (_e1898 * _e1816);
                                            let _e1912 = (_e1814 * (1f / sqrt((((_e1814.x * _e1814.x) + (_e1814.y * _e1814.y)) + (_e1814.z * _e1814.z)))));
                                            phi_14683_ = type_69(type_34(vec4<f32>(_e1812.x, _e1812.y, _e1812.z, _e340.member.w), vec4<f32>(_e1912.x, _e1912.y, _e1912.z, _e340.member_1.w), vec4<f32>(_e1901.x, _e1901.y, _e1901.z, f32((_e365 + 1u))), vec4<f32>(_e340.member_3.x, _e340.member_3.y, _e340.member_3.z, f32(_e393)), vec4<u32>(_e1896, ((_e340.member_4.y | 2u) & 4294967263u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14683_ = type_69();
                                        }
                                        let _e1936 = phi_14683_;
                                        phi_34404_ = _e1896;
                                        phi_14685_ = _e1898;
                                        phi_14686_ = _e1936;
                                        phi_14687_ = select(true, false, _e1900);
                                    }
                                    let _e1939 = phi_34404_;
                                    let _e1941 = phi_14685_;
                                    let _e1943 = phi_14686_;
                                    let _e1945 = phi_14687_;
                                    if _e1945 {
                                        phi_14693_ = type_69(type_34(_e340.member, _e340.member_1, vec4<f32>(_e1941.x, _e1941.y, _e1941.z, f32(_e365)), vec4<f32>(_e340.member_3.x, _e340.member_3.y, _e340.member_3.z, f32(_e393)), vec4<u32>(_e1939, (_e340.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14693_ = _e1943;
                                    }
                                    let _e1958 = phi_14693_;
                                    phi_34408_ = _e1939;
                                    phi_14694_ = _e1958;
                                } else {
                                    phi_34408_ = _e1810;
                                    phi_14694_ = type_69();
                                }
                                let _e1960 = phi_34408_;
                                let _e1962 = phi_14694_;
                                phi_35002_ = _e1796;
                                phi_34810_ = _e1798;
                                phi_34731_ = _e1800;
                                phi_34654_ = _e1802;
                                phi_34575_ = _e1804;
                                phi_34493_ = _e1806;
                                phi_34416_ = _e1808;
                                phi_34407_ = _e1960;
                                phi_14696_ = _e1962;
                                phi_14697_ = select(true, false, _e1819);
                            }
                            let _e1970 = phi_35002_;
                            let _e1972 = phi_34810_;
                            let _e1974 = phi_34731_;
                            let _e1976 = phi_34654_;
                            let _e1978 = phi_34575_;
                            let _e1980 = phi_34493_;
                            let _e1982 = phi_34416_;
                            let _e1984 = phi_34407_;
                            let _e1986 = phi_14696_;
                            let _e1988 = phi_14697_;
                            if _e1988 {
                                let _e1990 = ((_e340.member_4.y | 4u) & 4294967261u);
                                let _e1993 = global_1.member.member_1[3u];
                                phi_36331_ = _e829;
                                phi_34406_ = _e1984;
                                phi_14708_ = _e369;
                                phi_14711_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14713_ = 0u;
                                phi_14715_ = _e393;
                                loop {
                                    let _e1995 = phi_36331_;
                                    let _e1997 = phi_34406_;
                                    let _e1999 = phi_14708_;
                                    let _e2001 = phi_14711_;
                                    let _e2003 = phi_14713_;
                                    let _e2005 = phi_14715_;
                                    local_83 = _e2005;
                                    local_84 = _e1999;
                                    local_85 = _e1999;
                                    local_86 = _e1999;
                                    local_87 = _e2001;
                                    local_88 = _e2005;
                                    local_89 = _e1999;
                                    local_90 = _e1999;
                                    local_91 = _e1999;
                                    local_92 = _e2001;
                                    local_93 = _e2005;
                                    local_94 = _e1999;
                                    local_95 = _e1999;
                                    local_96 = _e1999;
                                    local_97 = _e2001;
                                    local_98 = _e2005;
                                    local_99 = _e1999;
                                    local_100 = _e1999;
                                    local_101 = _e1999;
                                    local_102 = _e2001;
                                    let _e2007 = (_e2003 < (_e1993 + 3u));
                                    if _e2007 {
                                        if (_e2003 < _e1993) {
                                            let _e4004 = global_5.member[_e2003];
                                            let _e4009 = (_e4004.member.x - _e1114);
                                            let _e4010 = (_e4004.member.y - _e1115);
                                            let _e4011 = (_e4004.member.z - _e1116);
                                            let _e4016 = (((_e4009 * _e4009) + (_e4010 * _e4010)) + (_e4011 * _e4011));
                                            let _e4017 = sqrt(_e4016);
                                            let _e4018 = (_e4009 / _e4017);
                                            let _e4019 = (_e4010 / _e4017);
                                            let _e4020 = (_e4011 / _e4017);
                                            if (_e4004.member.w < 0.5f) {
                                                phi_14812_ = 1f;
                                            } else {
                                                let _e4044 = (((_e4004.member_2.x * -(_e4018)) + (_e4004.member_2.y * -(_e4019))) + (_e4004.member_2.z * -(_e4020)));
                                                if (_e4004.member_3.x == _e4004.member_2.w) {
                                                    phi_14811_ = select(1f, 0f, (_e4044 < _e4004.member_3.x));
                                                } else {
                                                    let _e4048 = ((_e4044 - _e4004.member_3.x) / (_e4004.member_2.w - _e4004.member_3.x));
                                                    let _e4050 = select(_e4048, 0f, (_e4048 < 0f));
                                                    let _e4052 = select(_e4050, 1f, (_e4050 > 1f));
                                                    phi_14811_ = ((_e4052 * _e4052) * (3f - (2f * _e4052)));
                                                }
                                                let _e4060 = phi_14811_;
                                                phi_14812_ = _e4060;
                                            }
                                            let _e4062 = phi_14812_;
                                            phi_36748_ = _e1997;
                                            phi_18225_ = 0f;
                                            phi_18226_ = vec3<f32>(_e4018, _e4019, _e4020);
                                            phi_18227_ = vec3<f32>(((_e4004.member_1.x * _e4062) / _e4016), ((_e4004.member_1.y * _e4062) / _e4016), ((_e4004.member_1.z * _e4062) / _e4016));
                                            phi_18228_ = _e4017;
                                        } else {
                                            if (_e2003 == _e1993) {
                                                if ((_e387 & 4u) == 0u) {
                                                    phi_36792_ = _e1997;
                                                    phi_16854_ = 0f;
                                                    phi_16855_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16856_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e2756 = global_1.member.member_5;
                                                    let _e2764 = global_1.member.member_6[3u];
                                                    let _e2766 = sin((_e2764 * 0.5f));
                                                    let _e2768 = ((12.566371f * _e2766) * _e2766);
                                                    let _e2773 = (((_e2756.x * _e1089.x) + (_e2756.y * _e1089.y)) + (_e2756.z * _e1089.z));
                                                    if (_e2773 != _e2773) {
                                                        phi_28151_ = true;
                                                    } else {
                                                        phi_28151_ = (0f >= _e2773);
                                                    }
                                                    let _e2777 = phi_28151_;
                                                    let _e2779 = (select(_e2773, 0f, _e2777) * 0.31830987f);
                                                    if (_e1982 == 0u) {
                                                        if (_e2773 <= 0f) {
                                                            phi_28948_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28948_ = (_e1976 * (_e2773 * 0.31830987f));
                                                        }
                                                        let _e3255 = phi_28948_;
                                                        phi_15222_ = type_59(_e3255, _e2779);
                                                        phi_15223_ = type_59();
                                                        phi_15224_ = true;
                                                    } else {
                                                        let _e2785 = (((_e1063 * _e1137) + (_e1064 * _e1138)) + (_e1065 * _e1140));
                                                        let _e2790 = (((_e1063 * _e1133) + (_e1064 * _e1143)) + (_e1065 * _e1144));
                                                        let _e2795 = (((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z));
                                                        let _e2800 = (((_e2756.x * _e1137) + (_e2756.y * _e1138)) + (_e2756.z * _e1140));
                                                        let _e2805 = (((_e2756.x * _e1133) + (_e2756.y * _e1143)) + (_e2756.z * _e1144));
                                                        if (_e2795 <= 0f) {
                                                            phi_15186_ = type_59();
                                                            phi_15187_ = true;
                                                        } else {
                                                            let _e2807 = (_e2773 <= 0f);
                                                            if _e2807 {
                                                                phi_15185_ = type_59();
                                                            } else {
                                                                let _e2808 = (_e2785 + _e2800);
                                                                let _e2809 = (_e2790 + _e2805);
                                                                let _e2810 = (_e2795 + _e2773);
                                                                let _e2817 = (1f / sqrt((((_e2808 * _e2808) + (_e2809 * _e2809)) + (_e2810 * _e2810))));
                                                                let _e2820 = (_e2810 * _e2817);
                                                                if (_e2820 <= 0f) {
                                                                    phi_28261_ = 0f;
                                                                } else {
                                                                    let _e2822 = (_e1980 * _e1980);
                                                                    let _e2826 = ((((_e2822 - 1f) * _e2820) * _e2820) + 1f);
                                                                    phi_28261_ = (_e2822 / ((3.1415927f * _e2826) * _e2826));
                                                                }
                                                                let _e2831 = phi_28261_;
                                                                let _e2832 = (_e2795 * _e2795);
                                                                if (_e2832 >= 1f) {
                                                                    phi_28289_ = 0f;
                                                                } else {
                                                                    phi_28289_ = ((sqrt((1f + (((_e1980 * _e1980) * ((_e2785 * _e2785) + (_e2790 * _e2790))) / _e2832))) - 1f) * 0.5f);
                                                                }
                                                                let _e2845 = phi_28289_;
                                                                let _e2846 = (1f + _e2845);
                                                                let _e2847 = (_e2773 * _e2773);
                                                                if (_e2847 >= 1f) {
                                                                    phi_28317_ = 0f;
                                                                } else {
                                                                    phi_28317_ = ((sqrt((1f + (((_e1980 * _e1980) * ((_e2800 * _e2800) + (_e2805 * _e2805))) / _e2847))) - 1f) * 0.5f);
                                                                }
                                                                let _e2860 = phi_28317_;
                                                                let _e2867 = (((_e2785 * (_e2808 * _e2817)) + (_e2790 * (_e2809 * _e2817))) + (_e2795 * _e2820));
                                                                if (_e1978 == 0u) {
                                                                    let _e3162 = (1f - _e2867);
                                                                    if (_e3162 != _e3162) {
                                                                        phi_28820_ = true;
                                                                    } else {
                                                                        phi_28820_ = (0f >= _e3162);
                                                                    }
                                                                    let _e3166 = phi_28820_;
                                                                    let _e3168 = pow(select(_e3162, 0f, _e3166), 5f);
                                                                    let _e3169 = (1f - _e3168);
                                                                    phi_15173_ = vec3<f32>(((_e1976.x * _e3169) + _e3168), ((_e1976.y * _e3169) + _e3168), ((_e1976.z * _e3169) + _e3168));
                                                                } else {
                                                                    if (_e1978 == 1u) {
                                                                        let _e3135 = (1f / _e1972);
                                                                        let _e3139 = ((_e3135 * _e3135) * (1f - (_e2867 * _e2867)));
                                                                        if (_e3139 >= 1f) {
                                                                            phi_28781_ = 1f;
                                                                        } else {
                                                                            let _e3142 = sqrt((1f - _e3139));
                                                                            if (_e2867 != _e2867) {
                                                                                phi_28792_ = true;
                                                                            } else {
                                                                                phi_28792_ = (_e3142 <= _e2867);
                                                                            }
                                                                            let _e3146 = phi_28792_;
                                                                            let _e3151 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                                            phi_28781_ = (_e3151 + ((1f - _e3151) * pow((1f - select(_e2867, _e3142, _e3146)), 5f)));
                                                                        }
                                                                        let _e3158 = phi_28781_;
                                                                        phi_15172_ = vec3(_e3158);
                                                                    } else {
                                                                        let _e2871 = select(_e2867, 0f, (_e2867 < 0f));
                                                                        let _e2873 = select(_e2871, 1f, (_e2871 > 1f));
                                                                        let _e2875 = (1f - (_e2873 * _e2873));
                                                                        let _e2888 = ((_e1976.x * _e1976.x) - (_e1974.x * _e1974.x));
                                                                        let _e2889 = ((_e1976.y * _e1976.y) - (_e1974.y * _e1974.y));
                                                                        let _e2890 = ((_e1976.z * _e1976.z) - (_e1974.z * _e1974.z));
                                                                        let _e2892 = (_e1976.x * (_e1974.x + _e1974.x));
                                                                        let _e2894 = (_e1976.y * (_e1974.y + _e1974.y));
                                                                        let _e2896 = (_e1976.z * (_e1974.z + _e1974.z));
                                                                        let _e2903 = ((_e2888 * _e2888) + (_e2892 * _e2892));
                                                                        let _e2904 = ((_e2889 * _e2889) + (_e2894 * _e2894));
                                                                        let _e2905 = ((_e2890 * _e2890) + (_e2896 * _e2896));
                                                                        let _e2910 = ((_e2875 * _e2888) / _e2903);
                                                                        let _e2911 = ((_e2875 * _e2889) / _e2904);
                                                                        let _e2912 = ((_e2875 * _e2890) / _e2905);
                                                                        let _e2913 = (1f - _e2910);
                                                                        let _e2914 = (1f - _e2911);
                                                                        let _e2915 = (1f - _e2912);
                                                                        let _e2917 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2910, _e2911, _e2912));
                                                                        let _e2918 = (_e2875 * _e2892);
                                                                        let _e2919 = (_e2875 * _e2894);
                                                                        let _e2920 = (_e2875 * _e2896);
                                                                        let _e2925 = (vec3<f32>(_e2918, _e2919, _e2920) / vec3<f32>(_e2903, _e2904, _e2905));
                                                                        let _e2938 = ((_e2917.x * _e2917.x) + (_e2925.x * _e2925.x));
                                                                        let _e2939 = ((_e2917.y * _e2917.y) + (_e2925.y * _e2925.y));
                                                                        let _e2940 = ((_e2917.z * _e2917.z) + (_e2925.z * _e2925.z));
                                                                        if (_e2938 != _e2938) {
                                                                            phi_28451_ = true;
                                                                        } else {
                                                                            phi_28451_ = (0f >= _e2938);
                                                                        }
                                                                        let _e2944 = phi_28451_;
                                                                        let _e2946 = sqrt(select(_e2938, 0f, _e2944));
                                                                        if (_e2939 != _e2939) {
                                                                            phi_28466_ = true;
                                                                        } else {
                                                                            phi_28466_ = (0f >= _e2939);
                                                                        }
                                                                        let _e2950 = phi_28466_;
                                                                        let _e2952 = sqrt(select(_e2939, 0f, _e2950));
                                                                        if (_e2940 != _e2940) {
                                                                            phi_28481_ = true;
                                                                        } else {
                                                                            phi_28481_ = (0f >= _e2940);
                                                                        }
                                                                        let _e2956 = phi_28481_;
                                                                        let _e2958 = sqrt(select(_e2940, 0f, _e2956));
                                                                        let _e2962 = ((_e2946 + _e2913) * 0.5f);
                                                                        let _e2963 = ((_e2952 + _e2914) * 0.5f);
                                                                        let _e2964 = ((_e2958 + _e2915) * 0.5f);
                                                                        if (_e2962 != _e2962) {
                                                                            phi_28508_ = true;
                                                                        } else {
                                                                            phi_28508_ = (0f >= _e2962);
                                                                        }
                                                                        let _e2968 = phi_28508_;
                                                                        let _e2970 = sqrt(select(_e2962, 0f, _e2968));
                                                                        if (_e2963 != _e2963) {
                                                                            phi_28523_ = true;
                                                                        } else {
                                                                            phi_28523_ = (0f >= _e2963);
                                                                        }
                                                                        let _e2974 = phi_28523_;
                                                                        let _e2976 = sqrt(select(_e2963, 0f, _e2974));
                                                                        if (_e2964 != _e2964) {
                                                                            phi_28538_ = true;
                                                                        } else {
                                                                            phi_28538_ = (0f >= _e2964);
                                                                        }
                                                                        let _e2980 = phi_28538_;
                                                                        let _e2982 = sqrt(select(_e2964, 0f, _e2980));
                                                                        let _e2983 = vec3<f32>(_e2970, _e2976, _e2982);
                                                                        let _e2987 = ((_e2946 - _e2913) * 0.5f);
                                                                        let _e2988 = ((_e2952 - _e2914) * 0.5f);
                                                                        let _e2989 = ((_e2958 - _e2915) * 0.5f);
                                                                        if (_e2987 != _e2987) {
                                                                            phi_28565_ = true;
                                                                        } else {
                                                                            phi_28565_ = (0f >= _e2987);
                                                                        }
                                                                        let _e2993 = phi_28565_;
                                                                        let _e2995 = sqrt(select(_e2987, 0f, _e2993));
                                                                        if (_e2988 != _e2988) {
                                                                            phi_28580_ = true;
                                                                        } else {
                                                                            phi_28580_ = (0f >= _e2988);
                                                                        }
                                                                        let _e2999 = phi_28580_;
                                                                        let _e3001 = sqrt(select(_e2988, 0f, _e2999));
                                                                        if (_e2989 != _e2989) {
                                                                            phi_28595_ = true;
                                                                        } else {
                                                                            phi_28595_ = (0f >= _e2989);
                                                                        }
                                                                        let _e3005 = phi_28595_;
                                                                        let _e3007 = sqrt(select(_e2989, 0f, _e3005));
                                                                        if ((_e2918 / _e2903) < 0f) {
                                                                            phi_15049_ = -(_e2995);
                                                                        } else {
                                                                            phi_15049_ = _e2995;
                                                                        }
                                                                        let _e3011 = phi_15049_;
                                                                        if ((_e2919 / _e2904) < 0f) {
                                                                            phi_15057_ = -(_e3001);
                                                                        } else {
                                                                            phi_15057_ = _e3001;
                                                                        }
                                                                        let _e3015 = phi_15057_;
                                                                        if ((_e2920 / _e2905) < 0f) {
                                                                            phi_15065_ = -(_e3007);
                                                                        } else {
                                                                            phi_15065_ = _e3007;
                                                                        }
                                                                        let _e3019 = phi_15065_;
                                                                        let _e3023 = (_e1974.x * _e2873);
                                                                        let _e3024 = (_e1974.y * _e2873);
                                                                        let _e3025 = (_e1974.z * _e2873);
                                                                        let _e3035 = vec3<f32>(((_e1976.x * _e2970) - (_e1974.x * _e3011)), ((_e1976.y * _e2976) - (_e1974.y * _e3015)), ((_e1976.z * _e2982) - (_e1974.z * _e3019)));
                                                                        let _e3042 = ((_e1976.x * _e3011) + (_e1974.x * _e2970));
                                                                        let _e3043 = ((_e1976.y * _e3015) + (_e1974.y * _e2976));
                                                                        let _e3044 = ((_e1976.z * _e3019) + (_e1974.z * _e2982));
                                                                        let _e3046 = vec3<f32>((_e1976.x * _e2873), (_e1976.y * _e2873), (_e1976.z * _e2873));
                                                                        let _e3047 = (_e3046 - _e2983);
                                                                        let _e3048 = (_e3023 - _e3011);
                                                                        let _e3049 = (_e3024 - _e3015);
                                                                        let _e3050 = (_e3025 - _e3019);
                                                                        let _e3063 = (_e3046 + _e2983);
                                                                        let _e3064 = (_e3023 + _e3011);
                                                                        let _e3065 = (_e3024 + _e3015);
                                                                        let _e3066 = (_e3025 + _e3019);
                                                                        let _e3076 = ((_e3063.x * _e3063.x) + (_e3064 * _e3064));
                                                                        let _e3077 = ((_e3063.y * _e3063.y) + (_e3065 * _e3065));
                                                                        let _e3078 = ((_e3063.z * _e3063.z) + (_e3066 * _e3066));
                                                                        let _e3088 = vec3(_e2873);
                                                                        let _e3089 = (_e3088 - _e3035);
                                                                        let _e3090 = -(vec3<f32>(_e3042, _e3043, _e3044));
                                                                        let _e3106 = (_e3088 + _e3035);
                                                                        let _e3116 = ((_e3106.x * _e3106.x) + (_e3042 * _e3042));
                                                                        let _e3117 = ((_e3106.y * _e3106.y) + (_e3043 * _e3043));
                                                                        let _e3118 = ((_e3106.z * _e3106.z) + (_e3044 * _e3044));
                                                                        phi_15172_ = vec3<f32>((((((_e3047.x * _e3047.x) + (_e3048 * _e3048)) / select(0.000000000000000000000000000001f, _e3076, (_e3076 > 0.000000000000000000000000000001f))) + (((_e3089.x * _e3089.x) + (_e3090.x * _e3090.x)) / select(0.000000000000000000000000000001f, _e3116, (_e3116 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3047.y * _e3047.y) + (_e3049 * _e3049)) / select(0.000000000000000000000000000001f, _e3077, (_e3077 > 0.000000000000000000000000000001f))) + (((_e3089.y * _e3089.y) + (_e3090.y * _e3090.y)) / select(0.000000000000000000000000000001f, _e3117, (_e3117 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3047.z * _e3047.z) + (_e3050 * _e3050)) / select(0.000000000000000000000000000001f, _e3078, (_e3078 > 0.000000000000000000000000000001f))) + (((_e3089.z * _e3089.z) + (_e3090.z * _e3090.z)) / select(0.000000000000000000000000000001f, _e3118, (_e3118 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3161 = phi_15172_;
                                                                    phi_15173_ = _e3161;
                                                                }
                                                                let _e3181 = phi_15173_;
                                                                let _e3183 = (4f * _e2795);
                                                                phi_15185_ = type_59((_e3181 * (((1f / (_e2846 + _e2860)) * _e2831) / _e3183)), ((_e2831 / _e2846) / _e3183));
                                                            }
                                                            let _e3190 = phi_15185_;
                                                            phi_15186_ = _e3190;
                                                            phi_15187_ = _e2807;
                                                        }
                                                        let _e3192 = phi_15186_;
                                                        let _e3194 = phi_15187_;
                                                        if _e3194 {
                                                            phi_15192_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15192_ = _e3192;
                                                        }
                                                        let _e3196 = phi_15192_;
                                                        let _e3199 = (_e1982 == 1u);
                                                        if _e3199 {
                                                            phi_15220_ = _e3196;
                                                            phi_15221_ = type_59();
                                                        } else {
                                                            if (_e2795 != _e2795) {
                                                                phi_28857_ = true;
                                                            } else {
                                                                phi_28857_ = (0f >= _e2795);
                                                            }
                                                            let _e3203 = phi_28857_;
                                                            let _e3204 = select(_e2795, 0f, _e3203);
                                                            let _e3205 = (1f / _e1972);
                                                            let _e3209 = ((_e3205 * _e3205) * (1f - (_e3204 * _e3204)));
                                                            if (_e3209 >= 1f) {
                                                                phi_28876_ = 1f;
                                                            } else {
                                                                let _e3212 = sqrt((1f - _e3209));
                                                                if (_e3204 != _e3204) {
                                                                    phi_28887_ = true;
                                                                } else {
                                                                    phi_28887_ = (_e3212 <= _e3204);
                                                                }
                                                                let _e3216 = phi_28887_;
                                                                let _e3221 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                                phi_28876_ = (_e3221 + ((1f - _e3221) * pow((1f - select(_e3204, _e3212, _e3216)), 5f)));
                                                            }
                                                            let _e3228 = phi_28876_;
                                                            if (_e2773 <= 0f) {
                                                                phi_28926_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_28926_ = (_e1970 * (_e2773 * 0.31830987f));
                                                            }
                                                            let _e3233 = phi_28926_;
                                                            let _e3234 = (1f - _e3228);
                                                            phi_15220_ = type_59();
                                                            phi_15221_ = type_59((_e3196.member + vec3<f32>((_e3233.x * _e3234), (_e3233.y * _e3234), (_e3233.z * _e3234))), ((_e3228 * _e3196.member_1) + (_e3234 * _e2779)));
                                                        }
                                                        let _e3248 = phi_15220_;
                                                        let _e3250 = phi_15221_;
                                                        phi_15222_ = _e3248;
                                                        phi_15223_ = _e3250;
                                                        phi_15224_ = _e3199;
                                                    }
                                                    let _e3258 = phi_15222_;
                                                    let _e3260 = phi_15223_;
                                                    let _e3262 = phi_15224_;
                                                    if _e3262 {
                                                        phi_15228_ = _e3258;
                                                    } else {
                                                        phi_15228_ = _e3260;
                                                    }
                                                    let _e3264 = phi_15228_;
                                                    if ((_e3264.member_1 * _e2768) < 0.001f) {
                                                        let _e3691 = global_1.member.member_6;
                                                        let _e3697 = global_1.member.member_8[2u];
                                                        if (_e3697 == 4294967295u) {
                                                            phi_15967_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e3700 = global_6.member[_e3697];
                                                            let _e3703 = global_6.member[(_e3697 + 1u)];
                                                            let _e3706 = global_6.member[(_e3697 + 2u)];
                                                            let _e3709 = global_6.member[(_e3697 + 3u)];
                                                            let _e3712 = global_6.member[(_e3697 + 4u)];
                                                            let _e3715 = global_6.member[(_e3697 + 5u)];
                                                            let _e3718 = global_6.member[(_e3697 + 6u)];
                                                            let _e3721 = global_6.member[(_e3697 + 7u)];
                                                            let _e3724 = global_6.member[(_e3697 + 8u)];
                                                            let _e3727 = global_6.member[(_e3697 + 9u)];
                                                            let _e3730 = global_6.member[(_e3697 + 13u)];
                                                            let _e3733 = global_6.member[(_e3697 + 14u)];
                                                            let _e3736 = global_6.member[(_e3697 + 15u)];
                                                            let _e3739 = global_6.member[(_e3697 + 16u)];
                                                            let _e3742 = global_6.member[(_e3697 + 18u)];
                                                            let _e3745 = global_6.member[(_e3697 + 19u)];
                                                            let _e3748 = global_6.member[(_e3697 + 20u)];
                                                            let _e3751 = global_6.member[(_e3697 + 29u)];
                                                            let _e3752 = bitcast<u32>(_e3751);
                                                            let _e3765 = (((((_e1114 * _e1114) + (_e1115 * _e1115)) + (_e1116 * _e1116)) - (2f * (((_e1114 * _e3700) + (_e1115 * _e3703)) + (_e1116 * _e3706)))) + _e3718);
                                                            if (_e3765 > 0f) {
                                                                let _e3774 = ((((_e1114 - _e3700) * _e3721) + ((_e1115 - _e3703) * _e3724)) + ((_e1116 - _e3706) * _e3727));
                                                                let _e3776 = ((_e3774 * _e3774) - _e3765);
                                                                if (_e3774 >= 0f) {
                                                                    phi_15667_ = vec3<f32>();
                                                                    phi_15668_ = true;
                                                                } else {
                                                                    let _e3778 = (_e3776 <= 0f);
                                                                    if _e3778 {
                                                                        phi_15666_ = vec3<f32>();
                                                                    } else {
                                                                        let _e3781 = (-(_e3774) - sqrt(_e3776));
                                                                        phi_15666_ = vec3<f32>((_e1114 + (_e3721 * _e3781)), (_e1115 + (_e3724 * _e3781)), (_e1116 + (_e3727 * _e3781)));
                                                                    }
                                                                    let _e3790 = phi_15666_;
                                                                    phi_15667_ = _e3790;
                                                                    phi_15668_ = _e3778;
                                                                }
                                                                let _e3792 = phi_15667_;
                                                                let _e3794 = phi_15668_;
                                                                phi_15673_ = _e3792;
                                                                phi_15674_ = select(true, false, _e3794);
                                                            } else {
                                                                phi_15673_ = _e1117;
                                                                phi_15674_ = true;
                                                            }
                                                            let _e3797 = phi_15673_;
                                                            let _e3799 = phi_15674_;
                                                            if _e3799 {
                                                                let _e3801 = (_e3797.x - _e3700);
                                                                let _e3803 = (_e3797.y - _e3703);
                                                                let _e3805 = (_e3797.z - _e3706);
                                                                let _e3811 = sqrt((((_e3801 * _e3801) + (_e3803 * _e3803)) + (_e3805 * _e3805)));
                                                                let _e3812 = (1f / _e3811);
                                                                let _e3830 = ((((((_e3797.x * _e3797.x) + (_e3797.y * _e3797.y)) + (_e3797.z * _e3797.z)) - (2f * (((_e3797.x * _e3700) + (_e3797.y * _e3703)) + (_e3797.z * _e3706)))) + _e3715) / (_e3811 + _e3709));
                                                                if (_e3830 != _e3830) {
                                                                    phi_29382_ = true;
                                                                } else {
                                                                    phi_29382_ = (0f >= _e3830);
                                                                }
                                                                let _e3834 = phi_29382_;
                                                                let _e3838 = sqrt((select(_e3830, 0f, _e3834) / (_e3712 - _e3709)));
                                                                if (_e3838 != _e3838) {
                                                                    phi_29397_ = true;
                                                                } else {
                                                                    phi_29397_ = (1f <= _e3838);
                                                                }
                                                                let _e3842 = phi_29397_;
                                                                let _e3844 = (select(_e3838, 1f, _e3842) * 63f);
                                                                let _e3849 = ((((_e3801 * _e3812) * _e3721) + ((_e3803 * _e3812) * _e3724)) + ((_e3805 * _e3812) * _e3727));
                                                                let _e3851 = select(_e3849, -1f, (_e3849 < -1f));
                                                                let _e3855 = ((select(_e3851, 1f, (_e3851 > 1f)) + 1f) * 127.5f);
                                                                let _e3860 = select(select(u32(_e3844), 0u, (_e3844 < 0f)), 4294967295u, (_e3844 > 4294967000f));
                                                                let _e3862 = select(_e3860, 62u, (62u < _e3860));
                                                                let _e3867 = select(select(u32(_e3855), 0u, (_e3855 < 0f)), 4294967295u, (_e3855 > 4294967000f));
                                                                let _e3869 = select(_e3867, 254u, (254u < _e3867));
                                                                let _e3871 = (_e3844 - f32(_e3862));
                                                                let _e3873 = (_e3855 - f32(_e3869));
                                                                let _e3877 = (_e3752 + (3u * ((_e3862 * 256u) + _e3869)));
                                                                let _e3882 = (_e3752 + (3u * (((_e3862 + 1u) * 256u) + _e3869)));
                                                                let _e3884 = global_6.member[_e3877];
                                                                let _e3887 = global_6.member[(_e3877 + 1u)];
                                                                let _e3890 = global_6.member[(_e3877 + 2u)];
                                                                let _e3891 = (1f - _e3873);
                                                                let _e3897 = global_6.member[(_e3877 + 3u)];
                                                                let _e3900 = global_6.member[(_e3877 + 4u)];
                                                                let _e3903 = global_6.member[(_e3877 + 5u)];
                                                                let _e3910 = (1f - _e3871);
                                                                let _e3915 = global_6.member[_e3882];
                                                                let _e3918 = global_6.member[(_e3882 + 1u)];
                                                                let _e3921 = global_6.member[(_e3882 + 2u)];
                                                                let _e3927 = global_6.member[(_e3882 + 3u)];
                                                                let _e3930 = global_6.member[(_e3882 + 4u)];
                                                                let _e3933 = global_6.member[(_e3882 + 5u)];
                                                                phi_15965_ = vec3<f32>(((((_e3884 * _e3891) + (_e3897 * _e3873)) * _e3910) + (((_e3915 * _e3891) + (_e3927 * _e3873)) * _e3871)), ((((_e3887 * _e3891) + (_e3900 * _e3873)) * _e3910) + (((_e3918 * _e3891) + (_e3930 * _e3873)) * _e3871)), ((((_e3890 * _e3891) + (_e3903 * _e3873)) * _e3910) + (((_e3921 * _e3891) + (_e3933 * _e3873)) * _e3871)));
                                                            } else {
                                                                phi_15965_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3948 = phi_15965_;
                                                            let _e3955 = ((_e3739 * 1.1f) * _e3948.y);
                                                            phi_15967_ = vec3<f32>(exp(-((((_e3730 * _e3948.x) + _e3955) + (_e3742 * _e3948.z)))), exp(-((((_e3733 * _e3948.x) + _e3955) + (_e3745 * _e3948.z)))), exp(-((((_e3736 * _e3948.x) + _e3955) + (_e3748 * _e3948.z)))));
                                                        }
                                                        let _e3974 = phi_15967_;
                                                        phi_36793_ = _e1997;
                                                        phi_16851_ = 0f;
                                                        phi_16852_ = vec3<f32>(_e2756.x, _e2756.y, _e2756.z);
                                                        phi_16853_ = (vec3<f32>(_e3691.x, _e3691.y, _e3691.z) * _e3974);
                                                    } else {
                                                        let _e3269 = ((_e1997 * 747796405u) + 2891336453u);
                                                        let _e3277 = (((_e3269 >> bitcast<u32>((((_e3269 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3269) * 277803737u);
                                                        let _e3280 = ((_e3277 >> bitcast<u32>(22i)) ^ _e3277);
                                                        let _e3286 = (((f32((_e3280 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2768) * 0.15915494f);
                                                        let _e3287 = (1f - _e3286);
                                                        let _e3289 = (_e3286 * (2f - _e3286));
                                                        if (_e3289 != _e3289) {
                                                            phi_28980_ = true;
                                                        } else {
                                                            phi_28980_ = (0f >= _e3289);
                                                        }
                                                        let _e3293 = phi_28980_;
                                                        let _e3295 = sqrt(select(_e3289, 0f, _e3293));
                                                        let _e3297 = ((_e3280 * 747796405u) + 2891336453u);
                                                        let _e3305 = (((_e3297 >> bitcast<u32>((((_e3297 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3297) * 277803737u);
                                                        let _e3308 = ((_e3305 >> bitcast<u32>(22i)) ^ _e3305);
                                                        let _e3312 = (f32((_e3308 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3314 = select(-1f, 1f, (_e2756.z >= 0f));
                                                        let _e3316 = (-1f / (_e3314 + _e2756.z));
                                                        let _e3318 = ((_e2756.x * _e2756.y) * _e3316);
                                                        let _e3331 = (_e3295 * cos(_e3312));
                                                        let _e3336 = (_e3295 * sin(_e3312));
                                                        let _e3346 = ((((1f + (((_e3314 * _e2756.x) * _e2756.x) * _e3316)) * _e3331) + (_e3318 * _e3336)) + (_e2756.x * _e3287));
                                                        let _e3347 = ((((_e3314 * _e3318) * _e3331) + ((_e3314 + ((_e2756.y * _e2756.y) * _e3316)) * _e3336)) + (_e2756.y * _e3287));
                                                        let _e3348 = ((((-(_e3314) * _e2756.x) * _e3331) + (-(_e2756.y) * _e3336)) + (_e2756.z * _e3287));
                                                        let _e3350 = (_e3346 - _e2756.x);
                                                        let _e3351 = (_e3347 - _e2756.y);
                                                        let _e3352 = (_e3348 - _e2756.z);
                                                        let _e3359 = (sqrt((((_e3350 * _e3350) + (_e3351 * _e3351)) + (_e3352 * _e3352))) * 0.5f);
                                                        let _e3360 = (_e3359 != _e3359);
                                                        if _e3360 {
                                                            phi_29051_ = true;
                                                        } else {
                                                            phi_29051_ = (1f <= _e3359);
                                                        }
                                                        let _e3363 = phi_29051_;
                                                        if ((2f * asin(select(_e3359, 1f, _e3363))) >= _e2764) {
                                                            phi_16048_ = 0f;
                                                        } else {
                                                            phi_16048_ = (1f / _e2768);
                                                        }
                                                        let _e3370 = phi_16048_;
                                                        if (_e3370 > 0f) {
                                                            if _e3360 {
                                                                phi_29066_ = true;
                                                            } else {
                                                                phi_29066_ = (1f <= _e3359);
                                                            }
                                                            let _e3374 = phi_29066_;
                                                            let _e3377 = (2f * asin(select(_e3359, 1f, _e3374)));
                                                            if (_e3377 >= _e2764) {
                                                                phi_16849_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3381 = (1f - pow((_e3377 / _e2764), 2f));
                                                                if (_e3381 != _e3381) {
                                                                    phi_29081_ = true;
                                                                } else {
                                                                    phi_29081_ = (0f >= _e3381);
                                                                }
                                                                let _e3385 = phi_29081_;
                                                                let _e3388 = global_1.member.member_6;
                                                                let _e3394 = global_1.member.member_8[2u];
                                                                if (_e3394 == 4294967295u) {
                                                                    phi_16822_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e3397 = global_6.member[_e3394];
                                                                    let _e3400 = global_6.member[(_e3394 + 1u)];
                                                                    let _e3403 = global_6.member[(_e3394 + 2u)];
                                                                    let _e3406 = global_6.member[(_e3394 + 3u)];
                                                                    let _e3409 = global_6.member[(_e3394 + 4u)];
                                                                    let _e3412 = global_6.member[(_e3394 + 5u)];
                                                                    let _e3415 = global_6.member[(_e3394 + 6u)];
                                                                    let _e3418 = global_6.member[(_e3394 + 7u)];
                                                                    let _e3421 = global_6.member[(_e3394 + 8u)];
                                                                    let _e3424 = global_6.member[(_e3394 + 9u)];
                                                                    let _e3427 = global_6.member[(_e3394 + 13u)];
                                                                    let _e3430 = global_6.member[(_e3394 + 14u)];
                                                                    let _e3433 = global_6.member[(_e3394 + 15u)];
                                                                    let _e3436 = global_6.member[(_e3394 + 16u)];
                                                                    let _e3439 = global_6.member[(_e3394 + 18u)];
                                                                    let _e3442 = global_6.member[(_e3394 + 19u)];
                                                                    let _e3445 = global_6.member[(_e3394 + 20u)];
                                                                    let _e3448 = global_6.member[(_e3394 + 29u)];
                                                                    let _e3449 = bitcast<u32>(_e3448);
                                                                    let _e3462 = (((((_e1114 * _e1114) + (_e1115 * _e1115)) + (_e1116 * _e1116)) - (2f * (((_e1114 * _e3397) + (_e1115 * _e3400)) + (_e1116 * _e3403)))) + _e3415);
                                                                    if (_e3462 > 0f) {
                                                                        let _e3471 = ((((_e1114 - _e3397) * _e3418) + ((_e1115 - _e3400) * _e3421)) + ((_e1116 - _e3403) * _e3424));
                                                                        let _e3473 = ((_e3471 * _e3471) - _e3462);
                                                                        if (_e3471 >= 0f) {
                                                                            phi_16522_ = vec3<f32>();
                                                                            phi_16523_ = true;
                                                                        } else {
                                                                            let _e3475 = (_e3473 <= 0f);
                                                                            if _e3475 {
                                                                                phi_16521_ = vec3<f32>();
                                                                            } else {
                                                                                let _e3478 = (-(_e3471) - sqrt(_e3473));
                                                                                phi_16521_ = vec3<f32>((_e1114 + (_e3418 * _e3478)), (_e1115 + (_e3421 * _e3478)), (_e1116 + (_e3424 * _e3478)));
                                                                            }
                                                                            let _e3487 = phi_16521_;
                                                                            phi_16522_ = _e3487;
                                                                            phi_16523_ = _e3475;
                                                                        }
                                                                        let _e3489 = phi_16522_;
                                                                        let _e3491 = phi_16523_;
                                                                        phi_16528_ = _e3489;
                                                                        phi_16529_ = select(true, false, _e3491);
                                                                    } else {
                                                                        phi_16528_ = _e1117;
                                                                        phi_16529_ = true;
                                                                    }
                                                                    let _e3494 = phi_16528_;
                                                                    let _e3496 = phi_16529_;
                                                                    if _e3496 {
                                                                        let _e3498 = (_e3494.x - _e3397);
                                                                        let _e3500 = (_e3494.y - _e3400);
                                                                        let _e3502 = (_e3494.z - _e3403);
                                                                        let _e3508 = sqrt((((_e3498 * _e3498) + (_e3500 * _e3500)) + (_e3502 * _e3502)));
                                                                        let _e3509 = (1f / _e3508);
                                                                        let _e3527 = ((((((_e3494.x * _e3494.x) + (_e3494.y * _e3494.y)) + (_e3494.z * _e3494.z)) - (2f * (((_e3494.x * _e3397) + (_e3494.y * _e3400)) + (_e3494.z * _e3403)))) + _e3412) / (_e3508 + _e3406));
                                                                        if (_e3527 != _e3527) {
                                                                            phi_29205_ = true;
                                                                        } else {
                                                                            phi_29205_ = (0f >= _e3527);
                                                                        }
                                                                        let _e3531 = phi_29205_;
                                                                        let _e3535 = sqrt((select(_e3527, 0f, _e3531) / (_e3409 - _e3406)));
                                                                        if (_e3535 != _e3535) {
                                                                            phi_29220_ = true;
                                                                        } else {
                                                                            phi_29220_ = (1f <= _e3535);
                                                                        }
                                                                        let _e3539 = phi_29220_;
                                                                        let _e3541 = (select(_e3535, 1f, _e3539) * 63f);
                                                                        let _e3546 = ((((_e3498 * _e3509) * _e3418) + ((_e3500 * _e3509) * _e3421)) + ((_e3502 * _e3509) * _e3424));
                                                                        let _e3548 = select(_e3546, -1f, (_e3546 < -1f));
                                                                        let _e3552 = ((select(_e3548, 1f, (_e3548 > 1f)) + 1f) * 127.5f);
                                                                        let _e3557 = select(select(u32(_e3541), 0u, (_e3541 < 0f)), 4294967295u, (_e3541 > 4294967000f));
                                                                        let _e3559 = select(_e3557, 62u, (62u < _e3557));
                                                                        let _e3564 = select(select(u32(_e3552), 0u, (_e3552 < 0f)), 4294967295u, (_e3552 > 4294967000f));
                                                                        let _e3566 = select(_e3564, 254u, (254u < _e3564));
                                                                        let _e3568 = (_e3541 - f32(_e3559));
                                                                        let _e3570 = (_e3552 - f32(_e3566));
                                                                        let _e3574 = (_e3449 + (3u * ((_e3559 * 256u) + _e3566)));
                                                                        let _e3579 = (_e3449 + (3u * (((_e3559 + 1u) * 256u) + _e3566)));
                                                                        let _e3581 = global_6.member[_e3574];
                                                                        let _e3584 = global_6.member[(_e3574 + 1u)];
                                                                        let _e3587 = global_6.member[(_e3574 + 2u)];
                                                                        let _e3588 = (1f - _e3570);
                                                                        let _e3594 = global_6.member[(_e3574 + 3u)];
                                                                        let _e3597 = global_6.member[(_e3574 + 4u)];
                                                                        let _e3600 = global_6.member[(_e3574 + 5u)];
                                                                        let _e3607 = (1f - _e3568);
                                                                        let _e3612 = global_6.member[_e3579];
                                                                        let _e3615 = global_6.member[(_e3579 + 1u)];
                                                                        let _e3618 = global_6.member[(_e3579 + 2u)];
                                                                        let _e3624 = global_6.member[(_e3579 + 3u)];
                                                                        let _e3627 = global_6.member[(_e3579 + 4u)];
                                                                        let _e3630 = global_6.member[(_e3579 + 5u)];
                                                                        phi_16820_ = vec3<f32>(((((_e3581 * _e3588) + (_e3594 * _e3570)) * _e3607) + (((_e3612 * _e3588) + (_e3624 * _e3570)) * _e3568)), ((((_e3584 * _e3588) + (_e3597 * _e3570)) * _e3607) + (((_e3615 * _e3588) + (_e3627 * _e3570)) * _e3568)), ((((_e3587 * _e3588) + (_e3600 * _e3570)) * _e3607) + (((_e3618 * _e3588) + (_e3630 * _e3570)) * _e3568)));
                                                                    } else {
                                                                        phi_16820_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e3645 = phi_16820_;
                                                                    let _e3652 = ((_e3436 * 1.1f) * _e3645.y);
                                                                    phi_16822_ = vec3<f32>(exp(-((((_e3427 * _e3645.x) + _e3652) + (_e3439 * _e3645.z)))), exp(-((((_e3430 * _e3645.x) + _e3652) + (_e3442 * _e3645.z)))), exp(-((((_e3433 * _e3645.x) + _e3652) + (_e3445 * _e3645.z)))));
                                                                }
                                                                let _e3671 = phi_16822_;
                                                                let _e3682 = (((1f - (0.6f * (1f - sqrt(select(_e3381, 0f, _e3385))))) * 1.25f) / _e2768);
                                                                phi_16849_ = vec3<f32>(((_e3388.x * _e3671.x) * _e3682), ((_e3388.y * _e3671.y) * _e3682), ((_e3388.z * _e3671.z) * _e3682));
                                                            }
                                                            let _e3688 = phi_16849_;
                                                            phi_16850_ = _e3688;
                                                        } else {
                                                            phi_16850_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3690 = phi_16850_;
                                                        phi_36793_ = _e3308;
                                                        phi_16851_ = _e3370;
                                                        phi_16852_ = vec3<f32>(_e3346, _e3347, _e3348);
                                                        phi_16853_ = _e3690;
                                                    }
                                                    let _e3978 = phi_36793_;
                                                    let _e3980 = phi_16851_;
                                                    let _e3982 = phi_16852_;
                                                    let _e3984 = phi_16853_;
                                                    phi_36792_ = _e3978;
                                                    phi_16854_ = _e3980;
                                                    phi_16855_ = _e3982;
                                                    phi_16856_ = _e3984;
                                                }
                                                let _e3986 = phi_36792_;
                                                let _e3988 = phi_16854_;
                                                let _e3990 = phi_16855_;
                                                let _e3992 = phi_16856_;
                                                phi_36749_ = _e3986;
                                                phi_18221_ = _e3988;
                                                phi_18222_ = _e3990;
                                                phi_18223_ = _e3992;
                                                phi_18224_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e2003 == (_e1993 + 1u)) {
                                                    if ((_e387 & 1u) == 0u) {
                                                        phi_36758_ = _e1997;
                                                        phi_17855_ = 0f;
                                                        phi_17856_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17857_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17858_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e390 > 0u) {
                                                            let _e2234 = global_1.member.member_2[1u];
                                                            let _e2236 = ((_e1997 * 747796405u) + 2891336453u);
                                                            let _e2244 = (((_e2236 >> bitcast<u32>((((_e2236 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2236) * 277803737u);
                                                            let _e2247 = ((_e2244 >> bitcast<u32>(22i)) ^ _e2244);
                                                            phi_16877_ = 0u;
                                                            phi_16880_ = _e390;
                                                            loop {
                                                                let _e2253 = phi_16877_;
                                                                let _e2255 = phi_16880_;
                                                                local_78 = _e2253;
                                                                local_79 = _e2253;
                                                                let _e2257 = ((_e2253 + 1u) < _e2255);
                                                                if _e2257 {
                                                                    let _e2259 = ((_e2253 + _e2255) / 2u);
                                                                    let _e2262 = global_6.member[(_e2234 + _e2259)];
                                                                    let _e2263 = (_e2262 <= (f32((_e2247 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_16878_ = select(_e2253, _e2259, _e2263);
                                                                    phi_16881_ = select(_e2259, _e2255, _e2263);
                                                                } else {
                                                                    phi_16878_ = u32();
                                                                    phi_16881_ = u32();
                                                                }
                                                                let _e2267 = phi_16878_;
                                                                let _e2269 = phi_16881_;
                                                                phi_16877_ = _e2267;
                                                                phi_16880_ = _e2269;
                                                                if !(_e2257) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2272 = local_78;
                                                            let _e2273 = (_e2234 + _e2272);
                                                            let _e2276 = global_6.member[(_e2273 + 1u)];
                                                            let _e2278 = global_6.member[_e2273];
                                                            let _e2282 = global_1.member.member_2[0u];
                                                            let _e2284 = local_79;
                                                            let _e2287 = global_6.member[(_e2282 + _e2284)];
                                                            let _e2288 = bitcast<u32>(_e2287);
                                                            let _e2290 = ((_e2247 * 747796405u) + 2891336453u);
                                                            let _e2298 = (((_e2290 >> bitcast<u32>((((_e2290 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2290) * 277803737u);
                                                            let _e2301 = ((_e2298 >> bitcast<u32>(22i)) ^ _e2298);
                                                            let _e2304 = f32((_e2301 >> bitcast<u32>(8i)));
                                                            let _e2305 = (_e2304 * 0.000000059604645f);
                                                            let _e2307 = ((_e2301 * 747796405u) + 2891336453u);
                                                            let _e2315 = (((_e2307 >> bitcast<u32>((((_e2307 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2307) * 277803737u);
                                                            let _e2318 = ((_e2315 >> bitcast<u32>(22i)) ^ _e2315);
                                                            let _e2321 = f32((_e2318 >> bitcast<u32>(8i)));
                                                            let _e2323 = (3u * _e2288);
                                                            let _e2325 = global_2.member[_e2323];
                                                            let _e2328 = global_2.member[(_e2323 + 1u)];
                                                            let _e2334 = select(select(u32(_e2328.w), 0u, (_e2328.w < 0f)), 4294967295u, (_e2328.w > 4294967000f));
                                                            let _e2336 = (_e859 + (5u * _e2288));
                                                            let _e2338 = global_2.member[_e2336];
                                                            let _e2341 = global_2.member[(_e2336 + 1u)];
                                                            let _e2348 = (_e2334 == 1u);
                                                            if _e2348 {
                                                                let _e2477 = (_e2325.x - _e1114);
                                                                let _e2478 = (_e2325.y - _e1115);
                                                                let _e2479 = (_e2325.z - _e1116);
                                                                let _e2484 = (((_e2477 * _e2477) + (_e2478 * _e2478)) + (_e2479 * _e2479));
                                                                let _e2485 = (_e2325.w * _e2325.w);
                                                                let _e2487 = (_e2484 <= (_e2485 * 1.000001f));
                                                                if _e2487 {
                                                                    let _e2596 = (_e2304 * 0.00000037450704f);
                                                                    let _e2598 = ((_e2321 * 0.00000011920929f) - 1f);
                                                                    let _e2600 = (1f - (_e2598 * _e2598));
                                                                    if (_e2600 != _e2600) {
                                                                        phi_28048_ = true;
                                                                    } else {
                                                                        phi_28048_ = (0f >= _e2600);
                                                                    }
                                                                    let _e2604 = phi_28048_;
                                                                    let _e2606 = sqrt(select(_e2600, 0f, _e2604));
                                                                    let _e2608 = (_e2606 * cos(_e2596));
                                                                    let _e2610 = (_e2606 * sin(_e2596));
                                                                    let _e2615 = (_e2325.x + (_e2608 * _e2325.w));
                                                                    let _e2616 = (_e2325.y + (_e2610 * _e2325.w));
                                                                    let _e2617 = (_e2325.z + (_e2598 * _e2325.w));
                                                                    if _e2487 {
                                                                        let _e2630 = (_e2615 - _e1114);
                                                                        let _e2631 = (_e2616 - _e1115);
                                                                        let _e2632 = (_e2617 - _e1116);
                                                                        let _e2637 = (((_e2630 * _e2630) + (_e2631 * _e2631)) + (_e2632 * _e2632));
                                                                        phi_28117_ = (_e2637 / (((abs((((_e2608 * _e2630) + (_e2610 * _e2631)) + (_e2598 * _e2632))) / sqrt(_e2637)) * 12.566371f) * _e2485));
                                                                    } else {
                                                                        let _e2619 = (_e2485 / _e2484);
                                                                        if (_e2619 != _e2619) {
                                                                            phi_28136_ = true;
                                                                        } else {
                                                                            phi_28136_ = (1f <= _e2619);
                                                                        }
                                                                        let _e2623 = phi_28136_;
                                                                        let _e2624 = select(_e2619, 1f, _e2623);
                                                                        phi_28117_ = (0.15915494f / (_e2624 / (1f + sqrt((1f - _e2624)))));
                                                                    }
                                                                    let _e2650 = phi_28117_;
                                                                    phi_17228_ = type_72(vec3<f32>(_e2615, _e2616, _e2617), vec3<f32>(_e2608, _e2610, _e2598), _e2650);
                                                                } else {
                                                                    let _e2488 = (_e2485 / _e2484);
                                                                    let _e2489 = (_e2488 != _e2488);
                                                                    if _e2489 {
                                                                        phi_27900_ = true;
                                                                    } else {
                                                                        phi_27900_ = (1f <= _e2488);
                                                                    }
                                                                    let _e2492 = phi_27900_;
                                                                    let _e2493 = select(_e2488, 1f, _e2492);
                                                                    if _e2489 {
                                                                        phi_27923_ = true;
                                                                    } else {
                                                                        phi_27923_ = (1f <= _e2488);
                                                                    }
                                                                    let _e2497 = phi_27923_;
                                                                    let _e2498 = select(_e2488, 1f, _e2497);
                                                                    let _e2502 = (_e2498 / (1f + sqrt((1f - _e2498))));
                                                                    if (_e2493 < 0.00068523f) {
                                                                        phi_17163_ = (_e2493 * _e2305);
                                                                    } else {
                                                                        let _e2505 = (1f - (_e2305 * _e2502));
                                                                        let _e2507 = (1f - (_e2505 * _e2505));
                                                                        if (_e2507 != _e2507) {
                                                                            phi_27938_ = true;
                                                                        } else {
                                                                            phi_27938_ = (0f >= _e2507);
                                                                        }
                                                                        let _e2511 = phi_27938_;
                                                                        phi_17163_ = select(_e2507, 0f, _e2511);
                                                                    }
                                                                    let _e2515 = phi_17163_;
                                                                    let _e2516 = (1f - _e2515);
                                                                    if (_e2516 != _e2516) {
                                                                        phi_27953_ = true;
                                                                    } else {
                                                                        phi_27953_ = (0f >= _e2516);
                                                                    }
                                                                    let _e2520 = phi_27953_;
                                                                    let _e2525 = (1f - (_e2515 / _e2493));
                                                                    if (_e2525 != _e2525) {
                                                                        phi_27968_ = true;
                                                                    } else {
                                                                        phi_27968_ = (0f >= _e2525);
                                                                    }
                                                                    let _e2529 = phi_27968_;
                                                                    let _e2533 = ((_e2515 / sqrt(_e2493)) + (sqrt(select(_e2516, 0f, _e2520)) * sqrt(select(_e2525, 0f, _e2529))));
                                                                    let _e2535 = (1f - (_e2533 * _e2533));
                                                                    if (_e2535 != _e2535) {
                                                                        phi_27983_ = true;
                                                                    } else {
                                                                        phi_27983_ = (0f >= _e2535);
                                                                    }
                                                                    let _e2539 = phi_27983_;
                                                                    let _e2541 = sqrt(select(_e2535, 0f, _e2539));
                                                                    let _e2542 = (_e2321 * 0.00000037450704f);
                                                                    let _e2543 = sqrt(_e2484);
                                                                    let _e2544 = (_e2477 / _e2543);
                                                                    let _e2545 = (_e2478 / _e2543);
                                                                    let _e2546 = (_e2479 / _e2543);
                                                                    let _e2548 = select(-1f, 1f, (_e2546 >= 0f));
                                                                    let _e2550 = (-1f / (_e2548 + _e2546));
                                                                    let _e2552 = ((_e2544 * _e2545) * _e2550);
                                                                    let _e2565 = (_e2541 * cos(_e2542));
                                                                    let _e2570 = (_e2541 * sin(_e2542));
                                                                    let _e2583 = -(((((1f + (((_e2548 * _e2544) * _e2544) * _e2550)) * _e2565) + (_e2552 * _e2570)) + (_e2544 * _e2533)));
                                                                    let _e2584 = -(((((_e2548 * _e2552) * _e2565) + ((_e2548 + ((_e2545 * _e2545) * _e2550)) * _e2570)) + (_e2545 * _e2533)));
                                                                    let _e2585 = -(((((-(_e2548) * _e2544) * _e2565) + (-(_e2545) * _e2570)) + (_e2546 * _e2533)));
                                                                    phi_17228_ = type_72(vec3<f32>((_e2325.x + (_e2583 * _e2325.w)), (_e2325.y + (_e2584 * _e2325.w)), (_e2325.z + (_e2585 * _e2325.w))), vec3<f32>(_e2583, _e2584, _e2585), (0.15915494f / _e2502));
                                                                }
                                                                let _e2653 = phi_17228_;
                                                                phi_17656_ = _e2653;
                                                            } else {
                                                                let _e2349 = sqrt(_e2305);
                                                                let _e2350 = (1f - _e2349);
                                                                let _e2351 = ((_e2321 * 0.000000059604645f) * _e2349);
                                                                let _e2364 = (_e2349 - _e2351);
                                                                let _e2368 = (((_e2325.x * _e2350) + (_e2338.x * _e2351)) + (_e2341.x * _e2364));
                                                                let _e2369 = (((_e2325.y * _e2350) + (_e2338.y * _e2351)) + (_e2341.y * _e2364));
                                                                let _e2370 = (((_e2325.z * _e2350) + (_e2338.z * _e2351)) + (_e2341.z * _e2364));
                                                                let _e2372 = (_e2338.x - _e2325.x);
                                                                let _e2373 = (_e2338.y - _e2325.y);
                                                                let _e2374 = (_e2338.z - _e2325.z);
                                                                let _e2375 = (_e2341.x - _e2325.x);
                                                                let _e2376 = (_e2341.y - _e2325.y);
                                                                let _e2377 = (_e2341.z - _e2325.z);
                                                                let _e2380 = ((_e2373 * _e2377) - (_e2376 * _e2374));
                                                                let _e2383 = ((_e2374 * _e2375) - (_e2377 * _e2372));
                                                                let _e2386 = ((_e2372 * _e2376) - (_e2375 * _e2373));
                                                                let _e2392 = sqrt((((_e2380 * _e2380) + (_e2383 * _e2383)) + (_e2386 * _e2386)));
                                                                let _e2393 = (1f / _e2392);
                                                                let _e2394 = (_e2380 * _e2393);
                                                                let _e2395 = (_e2383 * _e2393);
                                                                let _e2396 = (_e2386 * _e2393);
                                                                if _e2348 {
                                                                    let _e2427 = (_e2325.x - _e1114);
                                                                    let _e2428 = (_e2325.y - _e1115);
                                                                    let _e2429 = (_e2325.z - _e1116);
                                                                    let _e2434 = (((_e2427 * _e2427) + (_e2428 * _e2428)) + (_e2429 * _e2429));
                                                                    let _e2435 = (_e2325.w * _e2325.w);
                                                                    if (_e2434 <= (_e2435 * 1.000001f)) {
                                                                        let _e2449 = (_e2368 - _e1114);
                                                                        let _e2450 = (_e2369 - _e1115);
                                                                        let _e2451 = (_e2370 - _e1116);
                                                                        let _e2456 = (((_e2449 * _e2449) + (_e2450 * _e2450)) + (_e2451 * _e2451));
                                                                        phi_27866_ = (_e2456 / (((abs((((_e2394 * _e2449) + (_e2395 * _e2450)) + (_e2396 * _e2451))) / sqrt(_e2456)) * 12.566371f) * _e2435));
                                                                    } else {
                                                                        let _e2438 = (_e2435 / _e2434);
                                                                        if (_e2438 != _e2438) {
                                                                            phi_27885_ = true;
                                                                        } else {
                                                                            phi_27885_ = (1f <= _e2438);
                                                                        }
                                                                        let _e2442 = phi_27885_;
                                                                        let _e2443 = select(_e2438, 1f, _e2442);
                                                                        phi_27866_ = (0.15915494f / (_e2443 / (1f + sqrt((1f - _e2443)))));
                                                                    }
                                                                    let _e2469 = phi_27866_;
                                                                    phi_17654_ = _e2469;
                                                                } else {
                                                                    let _e2398 = (_e2368 - _e1114);
                                                                    let _e2399 = (_e2369 - _e1115);
                                                                    let _e2400 = (_e2370 - _e1116);
                                                                    let _e2405 = (((_e2398 * _e2398) + (_e2399 * _e2399)) + (_e2400 * _e2400));
                                                                    let _e2413 = (abs((((_e2394 * _e2398) + (_e2395 * _e2399)) + (_e2396 * _e2400))) / sqrt(_e2405));
                                                                    if (_e2413 <= 0f) {
                                                                        phi_17653_ = 0f;
                                                                    } else {
                                                                        if (_e2334 == 0u) {
                                                                            phi_17650_ = (_e2392 * 0.5f);
                                                                        } else {
                                                                            phi_17650_ = ((12.566371f * _e2325.w) * _e2325.w);
                                                                        }
                                                                        let _e2421 = phi_17650_;
                                                                        phi_17653_ = (_e2405 / (_e2413 * _e2421));
                                                                    }
                                                                    let _e2425 = phi_17653_;
                                                                    phi_17654_ = _e2425;
                                                                }
                                                                let _e2471 = phi_17654_;
                                                                phi_17656_ = type_72(vec3<f32>(_e2368, _e2369, _e2370), vec3<f32>(_e2394, _e2395, _e2396), _e2471);
                                                            }
                                                            let _e2655 = phi_17656_;
                                                            let _e2660 = (_e2655.member.x - _e1114);
                                                            let _e2662 = (_e2655.member.y - _e1115);
                                                            let _e2664 = (_e2655.member.z - _e1116);
                                                            let _e2670 = sqrt((((_e2660 * _e2660) + (_e2662 * _e2662)) + (_e2664 * _e2664)));
                                                            let _e2671 = (_e2660 / _e2670);
                                                            let _e2672 = (_e2662 / _e2670);
                                                            let _e2673 = (_e2664 / _e2670);
                                                            if (_e2655.member_2 > 0f) {
                                                                let _e2689 = global_4.member[select(select(u32(_e2338.w), 0u, (_e2338.w < 0f)), 4294967295u, (_e2338.w > 4294967000f))];
                                                                if (_e2689.member == 4u) {
                                                                    if (_e2689.member_1 == 1u) {
                                                                        phi_17832_ = true;
                                                                    } else {
                                                                        phi_17832_ = ((((-(_e2671) * _e2655.member_1.x) + (-(_e2672) * _e2655.member_1.y)) + (-(_e2673) * _e2655.member_1.z)) > 0f);
                                                                    }
                                                                    let _e2704 = phi_17832_;
                                                                    if _e2704 {
                                                                        phi_17841_ = vec3<f32>(_e2689.member_4.x, _e2689.member_4.y, _e2689.member_4.z);
                                                                    } else {
                                                                        phi_17841_ = vec3<f32>();
                                                                    }
                                                                    let _e2711 = phi_17841_;
                                                                    phi_17843_ = _e2711;
                                                                    phi_17844_ = select(true, false, _e2704);
                                                                } else {
                                                                    phi_17843_ = vec3<f32>();
                                                                    phi_17844_ = true;
                                                                }
                                                                let _e2714 = phi_17843_;
                                                                let _e2716 = phi_17844_;
                                                                phi_17849_ = ((_e2276 - _e2278) * _e2655.member_2);
                                                                phi_17850_ = select(_e2714, vec3<f32>(0f, 0f, 0f), vec3(_e2716));
                                                            } else {
                                                                phi_17849_ = 0f;
                                                                phi_17850_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2720 = phi_17849_;
                                                            let _e2722 = phi_17850_;
                                                            phi_36759_ = _e2318;
                                                            phi_17851_ = _e2720;
                                                            phi_17852_ = vec3<f32>(_e2671, _e2672, _e2673);
                                                            phi_17853_ = _e2722;
                                                            phi_17854_ = ((_e2670 * 0.9999f) - _e1107);
                                                        } else {
                                                            phi_36759_ = _e1997;
                                                            phi_17851_ = 0f;
                                                            phi_17852_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17853_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17854_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e2724 = phi_36759_;
                                                        let _e2726 = phi_17851_;
                                                        let _e2728 = phi_17852_;
                                                        let _e2730 = phi_17853_;
                                                        let _e2732 = phi_17854_;
                                                        phi_36758_ = _e2724;
                                                        phi_17855_ = _e2726;
                                                        phi_17856_ = _e2728;
                                                        phi_17857_ = _e2730;
                                                        phi_17858_ = _e2732;
                                                    }
                                                    let _e2734 = phi_36758_;
                                                    let _e2736 = phi_17855_;
                                                    let _e2738 = phi_17856_;
                                                    let _e2740 = phi_17857_;
                                                    let _e2742 = phi_17858_;
                                                    phi_36750_ = _e2734;
                                                    phi_18217_ = _e2736;
                                                    phi_18218_ = _e2738;
                                                    phi_18219_ = _e2740;
                                                    phi_18220_ = _e2742;
                                                } else {
                                                    if ((_e387 & 2u) == 0u) {
                                                        phi_36751_ = _e1997;
                                                        phi_18214_ = 0f;
                                                        phi_18215_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_18216_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2015 = ((_e1997 * 747796405u) + 2891336453u);
                                                        let _e2023 = (((_e2015 >> bitcast<u32>((((_e2015 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2015) * 277803737u);
                                                        let _e2026 = ((_e2023 >> bitcast<u32>(22i)) ^ _e2023);
                                                        let _e2030 = (f32((_e2026 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2032 = ((_e2026 * 747796405u) + 2891336453u);
                                                        let _e2040 = (((_e2032 >> bitcast<u32>((((_e2032 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2032) * 277803737u);
                                                        let _e2043 = ((_e2040 >> bitcast<u32>(22i)) ^ _e2040);
                                                        let _e2047 = (f32((_e2043 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2050 = global_1.member.member_3[1u];
                                                        let _e2053 = global_1.member.member_3[2u];
                                                        let _e2056 = global_1.member.member_2[3u];
                                                        phi_17879_ = 0u;
                                                        phi_17882_ = _e2053;
                                                        loop {
                                                            let _e2058 = phi_17879_;
                                                            let _e2060 = phi_17882_;
                                                            local_73 = _e2058;
                                                            local_76 = _e2058;
                                                            local_77 = _e2058;
                                                            let _e2062 = ((_e2058 + 1u) < _e2060);
                                                            if _e2062 {
                                                                let _e2064 = ((_e2058 + _e2060) / 2u);
                                                                let _e2067 = global_6.member[(_e2056 + _e2064)];
                                                                let _e2068 = (_e2067 <= _e2030);
                                                                phi_17880_ = select(_e2058, _e2064, _e2068);
                                                                phi_17883_ = select(_e2064, _e2060, _e2068);
                                                            } else {
                                                                phi_17880_ = u32();
                                                                phi_17883_ = u32();
                                                            }
                                                            let _e2072 = phi_17880_;
                                                            let _e2074 = phi_17883_;
                                                            phi_17879_ = _e2072;
                                                            phi_17882_ = _e2074;
                                                            if !(_e2062) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2078 = global_1.member.member_3[0u];
                                                        let _e2079 = (_e2050 + 1u);
                                                        let _e2081 = local_73;
                                                        let _e2083 = (_e2078 + (_e2081 * _e2079));
                                                        phi_17920_ = 0u;
                                                        phi_17923_ = _e2050;
                                                        loop {
                                                            let _e2085 = phi_17920_;
                                                            let _e2087 = phi_17923_;
                                                            local_74 = _e2085;
                                                            local_75 = _e2085;
                                                            let _e2089 = ((_e2085 + 1u) < _e2087);
                                                            if _e2089 {
                                                                let _e2091 = ((_e2085 + _e2087) / 2u);
                                                                let _e2094 = global_6.member[(_e2083 + _e2091)];
                                                                let _e2095 = (_e2094 <= _e2047);
                                                                phi_17921_ = select(_e2085, _e2091, _e2095);
                                                                phi_17924_ = select(_e2091, _e2087, _e2095);
                                                            } else {
                                                                phi_17921_ = u32();
                                                                phi_17924_ = u32();
                                                            }
                                                            let _e2099 = phi_17921_;
                                                            let _e2101 = phi_17924_;
                                                            phi_17920_ = _e2099;
                                                            phi_17923_ = _e2101;
                                                            if !(_e2089) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2104 = local_74;
                                                        let _e2107 = local_75;
                                                        let _e2108 = (_e2083 + _e2107);
                                                        let _e2110 = global_6.member[_e2108];
                                                        let _e2113 = global_6.member[(_e2108 + 1u)];
                                                        if (_e2113 > _e2110) {
                                                            let _e2117 = ((_e2047 - _e2110) / (_e2113 - _e2110));
                                                            let _e2119 = select(_e2117, 0f, (_e2117 < 0f));
                                                            phi_18000_ = select(_e2119, 0.99999f, (_e2119 > 0.99999f));
                                                        } else {
                                                            phi_18000_ = 0.5f;
                                                        }
                                                        let _e2123 = phi_18000_;
                                                        let _e2125 = f32(_e2050);
                                                        let _e2128 = local_76;
                                                        let _e2131 = local_77;
                                                        let _e2132 = (_e2056 + _e2131);
                                                        let _e2134 = global_6.member[_e2132];
                                                        let _e2137 = global_6.member[(_e2132 + 1u)];
                                                        if (_e2137 > _e2134) {
                                                            let _e2141 = ((_e2030 - _e2134) / (_e2137 - _e2134));
                                                            let _e2143 = select(_e2141, 0f, (_e2141 < 0f));
                                                            phi_18049_ = select(_e2143, 0.99999f, (_e2143 > 0.99999f));
                                                        } else {
                                                            phi_18049_ = 0.5f;
                                                        }
                                                        let _e2147 = phi_18049_;
                                                        let _e2149 = f32(_e2053);
                                                        let _e2152 = ((((f32(_e2104) + _e2123) / _e2125) - 0.5f) * 6.2831855f);
                                                        let _e2153 = (((f32(_e2128) + _e2147) / _e2149) * 3.1415927f);
                                                        let _e2154 = sin(_e2153);
                                                        let _e2156 = (_e2154 * cos(_e2152));
                                                        let _e2157 = cos(_e2153);
                                                        let _e2159 = (_e2154 * sin(_e2152));
                                                        let _e2162 = ((atan2(_e2159, _e2156) * 0.15915494f) + 0.5f);
                                                        let _e2166 = select(_e2157, -1f, (_e2157 < -1f));
                                                        let _e2169 = acos(select(_e2166, 1f, (_e2166 > 1f)));
                                                        let _e2171 = ((_e2162 - floor(_e2162)) * _e2125);
                                                        let _e2176 = select(select(u32(_e2171), 0u, (_e2171 < 0f)), 4294967295u, (_e2171 > 4294967000f));
                                                        let _e2177 = (_e2050 - 1u);
                                                        let _e2180 = ((_e2169 * 0.31830987f) * _e2149);
                                                        let _e2185 = select(select(u32(_e2180), 0u, (_e2180 < 0f)), 4294967295u, (_e2180 > 4294967000f));
                                                        let _e2186 = (_e2053 - 1u);
                                                        let _e2188 = select(_e2185, _e2186, (_e2186 < _e2185));
                                                        let _e2189 = sin(_e2169);
                                                        if (_e2189 <= 0f) {
                                                            phi_18205_ = 0f;
                                                        } else {
                                                            let _e2193 = (_e2056 + _e2188);
                                                            let _e2196 = global_6.member[(_e2193 + 1u)];
                                                            let _e2198 = global_6.member[_e2193];
                                                            let _e2201 = ((_e2078 + (_e2188 * _e2079)) + select(_e2176, _e2177, (_e2177 < _e2176)));
                                                            let _e2204 = global_6.member[(_e2201 + 1u)];
                                                            let _e2206 = global_6.member[_e2201];
                                                            phi_18205_ = ((((_e2196 - _e2198) * _e2149) * ((_e2204 - _e2206) * _e2125)) / (19.73921f * _e2189));
                                                        }
                                                        let _e2213 = phi_18205_;
                                                        let _e2214 = (_e2213 > 0f);
                                                        if _e2214 {
                                                            phi_18212_ = vec3<f32>(_e2156, _e2157, _e2159);
                                                        } else {
                                                            phi_18212_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2217 = phi_18212_;
                                                        phi_36751_ = _e2043;
                                                        phi_18214_ = select(0f, _e2213, _e2214);
                                                        phi_18215_ = _e2217;
                                                        phi_18216_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2214));
                                                    }
                                                    let _e2222 = phi_36751_;
                                                    let _e2224 = phi_18214_;
                                                    let _e2226 = phi_18215_;
                                                    let _e2228 = phi_18216_;
                                                    phi_36750_ = _e2222;
                                                    phi_18217_ = _e2224;
                                                    phi_18218_ = _e2226;
                                                    phi_18219_ = _e2228;
                                                    phi_18220_ = 1000000000000000000000000000000f;
                                                }
                                                let _e2744 = phi_36750_;
                                                let _e2746 = phi_18217_;
                                                let _e2748 = phi_18218_;
                                                let _e2750 = phi_18219_;
                                                let _e2752 = phi_18220_;
                                                phi_36749_ = _e2744;
                                                phi_18221_ = _e2746;
                                                phi_18222_ = _e2748;
                                                phi_18223_ = _e2750;
                                                phi_18224_ = _e2752;
                                            }
                                            let _e3994 = phi_36749_;
                                            let _e3996 = phi_18221_;
                                            let _e3998 = phi_18222_;
                                            let _e4000 = phi_18223_;
                                            let _e4002 = phi_18224_;
                                            phi_36748_ = _e3994;
                                            phi_18225_ = _e3996;
                                            phi_18226_ = _e3998;
                                            phi_18227_ = _e4000;
                                            phi_18228_ = _e4002;
                                        }
                                        let _e4071 = phi_36748_;
                                        let _e4073 = phi_18225_;
                                        let _e4075 = phi_18226_;
                                        let _e4077 = phi_18227_;
                                        let _e4079 = phi_18228_;
                                        if ((((_e4077.x * _e4077.x) + (_e4077.y * _e4077.y)) + (_e4077.z * _e4077.z)) > 0f) {
                                            let _e4096 = (((_e4075.x * _e1089.x) + (_e4075.y * _e1089.y)) + (_e4075.z * _e1089.z));
                                            if (_e4096 != _e4096) {
                                                phi_29450_ = true;
                                            } else {
                                                phi_29450_ = (0f >= _e4096);
                                            }
                                            let _e4100 = phi_29450_;
                                            let _e4102 = (select(_e4096, 0f, _e4100) * 0.31830987f);
                                            if (_e1982 == 0u) {
                                                if (_e4096 <= 0f) {
                                                    phi_30247_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_30247_ = (_e1976 * (_e4096 * 0.31830987f));
                                                }
                                                let _e4578 = phi_30247_;
                                                phi_18624_ = type_59(_e4578, _e4102);
                                                phi_18625_ = type_59();
                                                phi_18626_ = true;
                                            } else {
                                                let _e4108 = (((_e1063 * _e1137) + (_e1064 * _e1138)) + (_e1065 * _e1140));
                                                let _e4113 = (((_e1063 * _e1133) + (_e1064 * _e1143)) + (_e1065 * _e1144));
                                                let _e4118 = (((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z));
                                                let _e4123 = (((_e4075.x * _e1137) + (_e4075.y * _e1138)) + (_e4075.z * _e1140));
                                                let _e4128 = (((_e4075.x * _e1133) + (_e4075.y * _e1143)) + (_e4075.z * _e1144));
                                                if (_e4118 <= 0f) {
                                                    phi_18588_ = type_59();
                                                    phi_18589_ = true;
                                                } else {
                                                    let _e4130 = (_e4096 <= 0f);
                                                    if _e4130 {
                                                        phi_18587_ = type_59();
                                                    } else {
                                                        let _e4131 = (_e4108 + _e4123);
                                                        let _e4132 = (_e4113 + _e4128);
                                                        let _e4133 = (_e4118 + _e4096);
                                                        let _e4140 = (1f / sqrt((((_e4131 * _e4131) + (_e4132 * _e4132)) + (_e4133 * _e4133))));
                                                        let _e4143 = (_e4133 * _e4140);
                                                        if (_e4143 <= 0f) {
                                                            phi_29560_ = 0f;
                                                        } else {
                                                            let _e4145 = (_e1980 * _e1980);
                                                            let _e4149 = ((((_e4145 - 1f) * _e4143) * _e4143) + 1f);
                                                            phi_29560_ = (_e4145 / ((3.1415927f * _e4149) * _e4149));
                                                        }
                                                        let _e4154 = phi_29560_;
                                                        let _e4155 = (_e4118 * _e4118);
                                                        if (_e4155 >= 1f) {
                                                            phi_29588_ = 0f;
                                                        } else {
                                                            phi_29588_ = ((sqrt((1f + (((_e1980 * _e1980) * ((_e4108 * _e4108) + (_e4113 * _e4113))) / _e4155))) - 1f) * 0.5f);
                                                        }
                                                        let _e4168 = phi_29588_;
                                                        let _e4169 = (1f + _e4168);
                                                        let _e4170 = (_e4096 * _e4096);
                                                        if (_e4170 >= 1f) {
                                                            phi_29616_ = 0f;
                                                        } else {
                                                            phi_29616_ = ((sqrt((1f + (((_e1980 * _e1980) * ((_e4123 * _e4123) + (_e4128 * _e4128))) / _e4170))) - 1f) * 0.5f);
                                                        }
                                                        let _e4183 = phi_29616_;
                                                        let _e4190 = (((_e4108 * (_e4131 * _e4140)) + (_e4113 * (_e4132 * _e4140))) + (_e4118 * _e4143));
                                                        if (_e1978 == 0u) {
                                                            let _e4485 = (1f - _e4190);
                                                            if (_e4485 != _e4485) {
                                                                phi_30119_ = true;
                                                            } else {
                                                                phi_30119_ = (0f >= _e4485);
                                                            }
                                                            let _e4489 = phi_30119_;
                                                            let _e4491 = pow(select(_e4485, 0f, _e4489), 5f);
                                                            let _e4492 = (1f - _e4491);
                                                            phi_18575_ = vec3<f32>(((_e1976.x * _e4492) + _e4491), ((_e1976.y * _e4492) + _e4491), ((_e1976.z * _e4492) + _e4491));
                                                        } else {
                                                            if (_e1978 == 1u) {
                                                                let _e4458 = (1f / _e1972);
                                                                let _e4462 = ((_e4458 * _e4458) * (1f - (_e4190 * _e4190)));
                                                                if (_e4462 >= 1f) {
                                                                    phi_30080_ = 1f;
                                                                } else {
                                                                    let _e4465 = sqrt((1f - _e4462));
                                                                    if (_e4190 != _e4190) {
                                                                        phi_30091_ = true;
                                                                    } else {
                                                                        phi_30091_ = (_e4465 <= _e4190);
                                                                    }
                                                                    let _e4469 = phi_30091_;
                                                                    let _e4474 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                                    phi_30080_ = (_e4474 + ((1f - _e4474) * pow((1f - select(_e4190, _e4465, _e4469)), 5f)));
                                                                }
                                                                let _e4481 = phi_30080_;
                                                                phi_18574_ = vec3(_e4481);
                                                            } else {
                                                                let _e4194 = select(_e4190, 0f, (_e4190 < 0f));
                                                                let _e4196 = select(_e4194, 1f, (_e4194 > 1f));
                                                                let _e4198 = (1f - (_e4196 * _e4196));
                                                                let _e4211 = ((_e1976.x * _e1976.x) - (_e1974.x * _e1974.x));
                                                                let _e4212 = ((_e1976.y * _e1976.y) - (_e1974.y * _e1974.y));
                                                                let _e4213 = ((_e1976.z * _e1976.z) - (_e1974.z * _e1974.z));
                                                                let _e4215 = (_e1976.x * (_e1974.x + _e1974.x));
                                                                let _e4217 = (_e1976.y * (_e1974.y + _e1974.y));
                                                                let _e4219 = (_e1976.z * (_e1974.z + _e1974.z));
                                                                let _e4226 = ((_e4211 * _e4211) + (_e4215 * _e4215));
                                                                let _e4227 = ((_e4212 * _e4212) + (_e4217 * _e4217));
                                                                let _e4228 = ((_e4213 * _e4213) + (_e4219 * _e4219));
                                                                let _e4233 = ((_e4198 * _e4211) / _e4226);
                                                                let _e4234 = ((_e4198 * _e4212) / _e4227);
                                                                let _e4235 = ((_e4198 * _e4213) / _e4228);
                                                                let _e4236 = (1f - _e4233);
                                                                let _e4237 = (1f - _e4234);
                                                                let _e4238 = (1f - _e4235);
                                                                let _e4240 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4233, _e4234, _e4235));
                                                                let _e4241 = (_e4198 * _e4215);
                                                                let _e4242 = (_e4198 * _e4217);
                                                                let _e4243 = (_e4198 * _e4219);
                                                                let _e4248 = (vec3<f32>(_e4241, _e4242, _e4243) / vec3<f32>(_e4226, _e4227, _e4228));
                                                                let _e4261 = ((_e4240.x * _e4240.x) + (_e4248.x * _e4248.x));
                                                                let _e4262 = ((_e4240.y * _e4240.y) + (_e4248.y * _e4248.y));
                                                                let _e4263 = ((_e4240.z * _e4240.z) + (_e4248.z * _e4248.z));
                                                                if (_e4261 != _e4261) {
                                                                    phi_29750_ = true;
                                                                } else {
                                                                    phi_29750_ = (0f >= _e4261);
                                                                }
                                                                let _e4267 = phi_29750_;
                                                                let _e4269 = sqrt(select(_e4261, 0f, _e4267));
                                                                if (_e4262 != _e4262) {
                                                                    phi_29765_ = true;
                                                                } else {
                                                                    phi_29765_ = (0f >= _e4262);
                                                                }
                                                                let _e4273 = phi_29765_;
                                                                let _e4275 = sqrt(select(_e4262, 0f, _e4273));
                                                                if (_e4263 != _e4263) {
                                                                    phi_29780_ = true;
                                                                } else {
                                                                    phi_29780_ = (0f >= _e4263);
                                                                }
                                                                let _e4279 = phi_29780_;
                                                                let _e4281 = sqrt(select(_e4263, 0f, _e4279));
                                                                let _e4285 = ((_e4269 + _e4236) * 0.5f);
                                                                let _e4286 = ((_e4275 + _e4237) * 0.5f);
                                                                let _e4287 = ((_e4281 + _e4238) * 0.5f);
                                                                if (_e4285 != _e4285) {
                                                                    phi_29807_ = true;
                                                                } else {
                                                                    phi_29807_ = (0f >= _e4285);
                                                                }
                                                                let _e4291 = phi_29807_;
                                                                let _e4293 = sqrt(select(_e4285, 0f, _e4291));
                                                                if (_e4286 != _e4286) {
                                                                    phi_29822_ = true;
                                                                } else {
                                                                    phi_29822_ = (0f >= _e4286);
                                                                }
                                                                let _e4297 = phi_29822_;
                                                                let _e4299 = sqrt(select(_e4286, 0f, _e4297));
                                                                if (_e4287 != _e4287) {
                                                                    phi_29837_ = true;
                                                                } else {
                                                                    phi_29837_ = (0f >= _e4287);
                                                                }
                                                                let _e4303 = phi_29837_;
                                                                let _e4305 = sqrt(select(_e4287, 0f, _e4303));
                                                                let _e4306 = vec3<f32>(_e4293, _e4299, _e4305);
                                                                let _e4310 = ((_e4269 - _e4236) * 0.5f);
                                                                let _e4311 = ((_e4275 - _e4237) * 0.5f);
                                                                let _e4312 = ((_e4281 - _e4238) * 0.5f);
                                                                if (_e4310 != _e4310) {
                                                                    phi_29864_ = true;
                                                                } else {
                                                                    phi_29864_ = (0f >= _e4310);
                                                                }
                                                                let _e4316 = phi_29864_;
                                                                let _e4318 = sqrt(select(_e4310, 0f, _e4316));
                                                                if (_e4311 != _e4311) {
                                                                    phi_29879_ = true;
                                                                } else {
                                                                    phi_29879_ = (0f >= _e4311);
                                                                }
                                                                let _e4322 = phi_29879_;
                                                                let _e4324 = sqrt(select(_e4311, 0f, _e4322));
                                                                if (_e4312 != _e4312) {
                                                                    phi_29894_ = true;
                                                                } else {
                                                                    phi_29894_ = (0f >= _e4312);
                                                                }
                                                                let _e4328 = phi_29894_;
                                                                let _e4330 = sqrt(select(_e4312, 0f, _e4328));
                                                                if ((_e4241 / _e4226) < 0f) {
                                                                    phi_18451_ = -(_e4318);
                                                                } else {
                                                                    phi_18451_ = _e4318;
                                                                }
                                                                let _e4334 = phi_18451_;
                                                                if ((_e4242 / _e4227) < 0f) {
                                                                    phi_18459_ = -(_e4324);
                                                                } else {
                                                                    phi_18459_ = _e4324;
                                                                }
                                                                let _e4338 = phi_18459_;
                                                                if ((_e4243 / _e4228) < 0f) {
                                                                    phi_18467_ = -(_e4330);
                                                                } else {
                                                                    phi_18467_ = _e4330;
                                                                }
                                                                let _e4342 = phi_18467_;
                                                                let _e4346 = (_e1974.x * _e4196);
                                                                let _e4347 = (_e1974.y * _e4196);
                                                                let _e4348 = (_e1974.z * _e4196);
                                                                let _e4358 = vec3<f32>(((_e1976.x * _e4293) - (_e1974.x * _e4334)), ((_e1976.y * _e4299) - (_e1974.y * _e4338)), ((_e1976.z * _e4305) - (_e1974.z * _e4342)));
                                                                let _e4365 = ((_e1976.x * _e4334) + (_e1974.x * _e4293));
                                                                let _e4366 = ((_e1976.y * _e4338) + (_e1974.y * _e4299));
                                                                let _e4367 = ((_e1976.z * _e4342) + (_e1974.z * _e4305));
                                                                let _e4369 = vec3<f32>((_e1976.x * _e4196), (_e1976.y * _e4196), (_e1976.z * _e4196));
                                                                let _e4370 = (_e4369 - _e4306);
                                                                let _e4371 = (_e4346 - _e4334);
                                                                let _e4372 = (_e4347 - _e4338);
                                                                let _e4373 = (_e4348 - _e4342);
                                                                let _e4386 = (_e4369 + _e4306);
                                                                let _e4387 = (_e4346 + _e4334);
                                                                let _e4388 = (_e4347 + _e4338);
                                                                let _e4389 = (_e4348 + _e4342);
                                                                let _e4399 = ((_e4386.x * _e4386.x) + (_e4387 * _e4387));
                                                                let _e4400 = ((_e4386.y * _e4386.y) + (_e4388 * _e4388));
                                                                let _e4401 = ((_e4386.z * _e4386.z) + (_e4389 * _e4389));
                                                                let _e4411 = vec3(_e4196);
                                                                let _e4412 = (_e4411 - _e4358);
                                                                let _e4413 = -(vec3<f32>(_e4365, _e4366, _e4367));
                                                                let _e4429 = (_e4411 + _e4358);
                                                                let _e4439 = ((_e4429.x * _e4429.x) + (_e4365 * _e4365));
                                                                let _e4440 = ((_e4429.y * _e4429.y) + (_e4366 * _e4366));
                                                                let _e4441 = ((_e4429.z * _e4429.z) + (_e4367 * _e4367));
                                                                phi_18574_ = vec3<f32>((((((_e4370.x * _e4370.x) + (_e4371 * _e4371)) / select(0.000000000000000000000000000001f, _e4399, (_e4399 > 0.000000000000000000000000000001f))) + (((_e4412.x * _e4412.x) + (_e4413.x * _e4413.x)) / select(0.000000000000000000000000000001f, _e4439, (_e4439 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4370.y * _e4370.y) + (_e4372 * _e4372)) / select(0.000000000000000000000000000001f, _e4400, (_e4400 > 0.000000000000000000000000000001f))) + (((_e4412.y * _e4412.y) + (_e4413.y * _e4413.y)) / select(0.000000000000000000000000000001f, _e4440, (_e4440 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4370.z * _e4370.z) + (_e4373 * _e4373)) / select(0.000000000000000000000000000001f, _e4401, (_e4401 > 0.000000000000000000000000000001f))) + (((_e4412.z * _e4412.z) + (_e4413.z * _e4413.z)) / select(0.000000000000000000000000000001f, _e4441, (_e4441 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e4484 = phi_18574_;
                                                            phi_18575_ = _e4484;
                                                        }
                                                        let _e4504 = phi_18575_;
                                                        let _e4506 = (4f * _e4118);
                                                        phi_18587_ = type_59((_e4504 * (((1f / (_e4169 + _e4183)) * _e4154) / _e4506)), ((_e4154 / _e4169) / _e4506));
                                                    }
                                                    let _e4513 = phi_18587_;
                                                    phi_18588_ = _e4513;
                                                    phi_18589_ = _e4130;
                                                }
                                                let _e4515 = phi_18588_;
                                                let _e4517 = phi_18589_;
                                                if _e4517 {
                                                    phi_18594_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_18594_ = _e4515;
                                                }
                                                let _e4519 = phi_18594_;
                                                let _e4522 = (_e1982 == 1u);
                                                if _e4522 {
                                                    phi_18622_ = _e4519;
                                                    phi_18623_ = type_59();
                                                } else {
                                                    if (_e4118 != _e4118) {
                                                        phi_30156_ = true;
                                                    } else {
                                                        phi_30156_ = (0f >= _e4118);
                                                    }
                                                    let _e4526 = phi_30156_;
                                                    let _e4527 = select(_e4118, 0f, _e4526);
                                                    let _e4528 = (1f / _e1972);
                                                    let _e4532 = ((_e4528 * _e4528) * (1f - (_e4527 * _e4527)));
                                                    if (_e4532 >= 1f) {
                                                        phi_30175_ = 1f;
                                                    } else {
                                                        let _e4535 = sqrt((1f - _e4532));
                                                        if (_e4527 != _e4527) {
                                                            phi_30186_ = true;
                                                        } else {
                                                            phi_30186_ = (_e4535 <= _e4527);
                                                        }
                                                        let _e4539 = phi_30186_;
                                                        let _e4544 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                        phi_30175_ = (_e4544 + ((1f - _e4544) * pow((1f - select(_e4527, _e4535, _e4539)), 5f)));
                                                    }
                                                    let _e4551 = phi_30175_;
                                                    if (_e4096 <= 0f) {
                                                        phi_30225_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_30225_ = (_e1970 * (_e4096 * 0.31830987f));
                                                    }
                                                    let _e4556 = phi_30225_;
                                                    let _e4557 = (1f - _e4551);
                                                    phi_18622_ = type_59();
                                                    phi_18623_ = type_59((_e4519.member + vec3<f32>((_e4556.x * _e4557), (_e4556.y * _e4557), (_e4556.z * _e4557))), ((_e4551 * _e4519.member_1) + (_e4557 * _e4102)));
                                                }
                                                let _e4571 = phi_18622_;
                                                let _e4573 = phi_18623_;
                                                phi_18624_ = _e4571;
                                                phi_18625_ = _e4573;
                                                phi_18626_ = _e4522;
                                            }
                                            let _e4581 = phi_18624_;
                                            let _e4583 = phi_18625_;
                                            let _e4585 = phi_18626_;
                                            if _e4585 {
                                                phi_18630_ = _e4581;
                                            } else {
                                                phi_18630_ = _e4583;
                                            }
                                            let _e4587 = phi_18630_;
                                            if (_e4073 > 0f) {
                                                if (_e4073 >= 300000000000000000000000000000000000000f) {
                                                    phi_30264_ = 1f;
                                                } else {
                                                    let _e4592 = (_e4073 * _e4073);
                                                    let _e4594 = (_e4592 + (_e4587.member_1 * _e4587.member_1));
                                                    if (_e4594 == 0f) {
                                                        phi_30261_ = 0f;
                                                    } else {
                                                        phi_30261_ = (_e4592 / _e4594);
                                                    }
                                                    let _e4598 = phi_30261_;
                                                    phi_30264_ = _e4598;
                                                }
                                                let _e4600 = phi_30264_;
                                                phi_18639_ = (_e4600 / _e4073);
                                            } else {
                                                phi_18639_ = 1f;
                                            }
                                            let _e4603 = phi_18639_;
                                            if ((((_e4587.member.x * _e4587.member.x) + (_e4587.member.y * _e4587.member.y)) + (_e4587.member.z * _e4587.member.z)) > 0f) {
                                                let _e4614 = (vec3<f32>(1f, 1f, 1f) / _e4075);
                                                local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                                phi_18672_ = 4294967295u;
                                                phi_18675_ = select(4294967295u, 0u, (_e842 > 0u));
                                                phi_18677_ = 0u;
                                                phi_18679_ = _e846;
                                                phi_18681_ = _e842;
                                                phi_18683_ = vec4<f32>(0f, 0f, 0f, 0f);
                                                phi_18685_ = _e4079;
                                                loop {
                                                    let _e4618 = phi_18672_;
                                                    let _e4620 = phi_18675_;
                                                    let _e4622 = phi_18677_;
                                                    let _e4624 = phi_18679_;
                                                    let _e4626 = phi_18681_;
                                                    let _e4628 = phi_18683_;
                                                    let _e4630 = phi_18685_;
                                                    local_80 = _e4618;
                                                    local_81 = _e4628;
                                                    local_82 = _e4626;
                                                    if (_e4626 < _e4624) {
                                                        let _e4993 = (3u * _e4626);
                                                        let _e4995 = global_2.member[_e4993];
                                                        let _e4998 = global_2.member[(_e4993 + 1u)];
                                                        let _e5004 = select(select(u32(_e4998.w), 0u, (_e4998.w < 0f)), 4294967295u, (_e4998.w > 4294967000f));
                                                        if (_e5004 == 0u) {
                                                            let _e5098 = global_2.member[(_e4993 + 2u)];
                                                            let _e5107 = ((_e4075.y * _e5098.z) - (_e5098.y * _e4075.z));
                                                            let _e5110 = ((_e4075.z * _e5098.x) - (_e5098.z * _e4075.x));
                                                            let _e5113 = ((_e4075.x * _e5098.y) - (_e5098.x * _e4075.y));
                                                            let _e5118 = (((_e4998.x * _e5107) + (_e4998.y * _e5110)) + (_e4998.z * _e5113));
                                                            if (abs(_e5118) < 0.000000000001f) {
                                                                phi_18870_ = vec4<f32>();
                                                                phi_18871_ = true;
                                                            } else {
                                                                let _e5121 = (1f / _e5118);
                                                                let _e5125 = (_e1114 - _e4995.x);
                                                                let _e5126 = (_e1115 - _e4995.y);
                                                                let _e5127 = (_e1116 - _e4995.z);
                                                                let _e5133 = ((((_e5125 * _e5107) + (_e5126 * _e5110)) + (_e5127 * _e5113)) * _e5121);
                                                                if (_e5133 < 0f) {
                                                                    phi_18860_ = vec4<f32>();
                                                                    phi_18861_ = true;
                                                                    phi_18862_ = false;
                                                                } else {
                                                                    let _e5135 = (_e5133 > 1f);
                                                                    if _e5135 {
                                                                        phi_18858_ = vec4<f32>();
                                                                        phi_18859_ = false;
                                                                    } else {
                                                                        let _e5138 = ((_e5126 * _e4998.z) - (_e4998.y * _e5127));
                                                                        let _e5141 = ((_e5127 * _e4998.x) - (_e4998.z * _e5125));
                                                                        let _e5144 = ((_e5125 * _e4998.y) - (_e4998.x * _e5126));
                                                                        let _e5150 = ((((_e4075.x * _e5138) + (_e4075.y * _e5141)) + (_e4075.z * _e5144)) * _e5121);
                                                                        if (_e5150 < 0f) {
                                                                            phi_18848_ = vec4<f32>();
                                                                            phi_18849_ = true;
                                                                            phi_18850_ = false;
                                                                        } else {
                                                                            let _e5153 = ((_e5133 + _e5150) > 1f);
                                                                            if _e5153 {
                                                                                phi_18846_ = vec4<f32>();
                                                                                phi_18847_ = false;
                                                                            } else {
                                                                                let _e5159 = ((((_e5098.x * _e5138) + (_e5098.y * _e5141)) + (_e5098.z * _e5144)) * _e5121);
                                                                                if (_e5159 <= 0f) {
                                                                                    phi_18841_ = vec4<f32>();
                                                                                    phi_18842_ = true;
                                                                                } else {
                                                                                    let _e5161 = (_e5159 >= _e4630);
                                                                                    if _e5161 {
                                                                                        phi_18840_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_18840_ = vec4<f32>(_e5159, _e5133, _e5150, 1f);
                                                                                    }
                                                                                    let _e5164 = phi_18840_;
                                                                                    phi_18841_ = _e5164;
                                                                                    phi_18842_ = _e5161;
                                                                                }
                                                                                let _e5166 = phi_18841_;
                                                                                let _e5168 = phi_18842_;
                                                                                phi_18846_ = _e5166;
                                                                                phi_18847_ = _e5168;
                                                                            }
                                                                            let _e5170 = phi_18846_;
                                                                            let _e5172 = phi_18847_;
                                                                            phi_18848_ = _e5170;
                                                                            phi_18849_ = _e5153;
                                                                            phi_18850_ = _e5172;
                                                                        }
                                                                        let _e5174 = phi_18848_;
                                                                        let _e5176 = phi_18849_;
                                                                        let _e5178 = phi_18850_;
                                                                        phi_18858_ = _e5174;
                                                                        phi_18859_ = select(_e5178, true, _e5176);
                                                                    }
                                                                    let _e5181 = phi_18858_;
                                                                    let _e5183 = phi_18859_;
                                                                    phi_18860_ = _e5181;
                                                                    phi_18861_ = _e5135;
                                                                    phi_18862_ = _e5183;
                                                                }
                                                                let _e5185 = phi_18860_;
                                                                let _e5187 = phi_18861_;
                                                                let _e5189 = phi_18862_;
                                                                phi_18870_ = _e5185;
                                                                phi_18871_ = select(_e5189, true, _e5187);
                                                            }
                                                            let _e5192 = phi_18870_;
                                                            let _e5194 = phi_18871_;
                                                            phi_19016_ = _e5192;
                                                            phi_19017_ = _e5194;
                                                        } else {
                                                            if (_e5004 == 1u) {
                                                                let _e5028 = (_e1114 - _e4995.x);
                                                                let _e5029 = (_e1115 - _e4995.y);
                                                                let _e5030 = (_e1116 - _e4995.z);
                                                                let _e5035 = (((_e5028 * _e4075.x) + (_e5029 * _e4075.y)) + (_e5030 * _e4075.z));
                                                                if (_e5035 > 0f) {
                                                                    phi_18910_ = select(true, false, ((((_e5028 * _e5028) + (_e5029 * _e5029)) + (_e5030 * _e5030)) > (_e4995.w * _e4995.w)));
                                                                } else {
                                                                    phi_18910_ = true;
                                                                }
                                                                let _e5047 = phi_18910_;
                                                                if _e5047 {
                                                                    let _e5051 = (_e5028 - (_e4075.x * _e5035));
                                                                    let _e5052 = (_e5029 - (_e4075.y * _e5035));
                                                                    let _e5053 = (_e5030 - (_e4075.z * _e5035));
                                                                    let _e5061 = ((_e4995.w * _e4995.w) - (((_e5051 * _e5051) + (_e5052 * _e5052)) + (_e5053 * _e5053)));
                                                                    if (_e5061 < 0f) {
                                                                        phi_18969_ = vec4<f32>();
                                                                        phi_18970_ = true;
                                                                    } else {
                                                                        let _e5063 = sqrt(_e5061);
                                                                        let _e5064 = -(_e5035);
                                                                        let _e5065 = (_e5064 - _e5063);
                                                                        if (_e5065 <= 0f) {
                                                                            phi_18947_ = true;
                                                                        } else {
                                                                            phi_18947_ = (_e5065 >= _e4630);
                                                                        }
                                                                        let _e5069 = phi_18947_;
                                                                        if _e5069 {
                                                                            phi_18953_ = (_e5064 + _e5063);
                                                                        } else {
                                                                            phi_18953_ = _e5065;
                                                                        }
                                                                        let _e5072 = phi_18953_;
                                                                        if (_e5072 <= 0f) {
                                                                            phi_18964_ = vec4<f32>();
                                                                            phi_18965_ = true;
                                                                        } else {
                                                                            let _e5074 = (_e5072 >= _e4630);
                                                                            if _e5074 {
                                                                                phi_18963_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18963_ = vec4<f32>(_e5072, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5077 = phi_18963_;
                                                                            phi_18964_ = _e5077;
                                                                            phi_18965_ = _e5074;
                                                                        }
                                                                        let _e5079 = phi_18964_;
                                                                        let _e5081 = phi_18965_;
                                                                        phi_18969_ = _e5079;
                                                                        phi_18970_ = _e5081;
                                                                    }
                                                                    let _e5083 = phi_18969_;
                                                                    let _e5085 = phi_18970_;
                                                                    phi_18974_ = _e5083;
                                                                    phi_18975_ = _e5085;
                                                                } else {
                                                                    phi_18974_ = vec4<f32>();
                                                                    phi_18975_ = true;
                                                                }
                                                                let _e5087 = phi_18974_;
                                                                let _e5089 = phi_18975_;
                                                                phi_19006_ = _e5087;
                                                                phi_19007_ = _e5089;
                                                            } else {
                                                                if (abs(_e4075.y) < 0.000000000001f) {
                                                                    phi_19004_ = vec4<f32>();
                                                                    phi_19005_ = true;
                                                                } else {
                                                                    let _e5011 = ((_e4995.x - _e1115) / _e4075.y);
                                                                    if (_e5011 <= 0f) {
                                                                        phi_18999_ = vec4<f32>();
                                                                        phi_19000_ = true;
                                                                    } else {
                                                                        let _e5013 = (_e5011 >= _e4630);
                                                                        if _e5013 {
                                                                            phi_18998_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18998_ = vec4<f32>(_e5011, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5016 = phi_18998_;
                                                                        phi_18999_ = _e5016;
                                                                        phi_19000_ = _e5013;
                                                                    }
                                                                    let _e5018 = phi_18999_;
                                                                    let _e5020 = phi_19000_;
                                                                    phi_19004_ = _e5018;
                                                                    phi_19005_ = _e5020;
                                                                }
                                                                let _e5022 = phi_19004_;
                                                                let _e5024 = phi_19005_;
                                                                phi_19006_ = _e5022;
                                                                phi_19007_ = _e5024;
                                                            }
                                                            let _e5091 = phi_19006_;
                                                            let _e5093 = phi_19007_;
                                                            phi_19016_ = _e5091;
                                                            phi_19017_ = select(true, false, select(true, false, _e5093));
                                                        }
                                                        let _e5196 = phi_19016_;
                                                        let _e5198 = phi_19017_;
                                                        let _e5201 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5196, vec4(select(true, false, _e5198)));
                                                        let _e5203 = (_e5201.w > 0f);
                                                        if _e5203 {
                                                            phi_19038_ = _e5201.x;
                                                        } else {
                                                            phi_19038_ = _e4630;
                                                        }
                                                        let _e5206 = phi_19038_;
                                                        let _e5210 = select(true, false, _e5203);
                                                        if _e5210 {
                                                            phi_19044_ = (_e4626 + 1u);
                                                        } else {
                                                            phi_19044_ = u32();
                                                        }
                                                        let _e5213 = phi_19044_;
                                                        phi_18673_ = select(_e4618, _e4626, _e5203);
                                                        phi_18676_ = _e4620;
                                                        phi_18678_ = _e4622;
                                                        phi_18680_ = _e4624;
                                                        phi_18682_ = _e5213;
                                                        phi_18684_ = select(_e4628, _e5201, vec4(_e5203));
                                                        phi_18686_ = _e5206;
                                                        phi_19217_ = _e5201;
                                                        phi_19218_ = _e5210;
                                                        phi_19219_ = false;
                                                    } else {
                                                        if (_e4620 == 4294967295u) {
                                                            let _e4633 = (_e4622 == 0u);
                                                            if _e4633 {
                                                                phi_19074_ = u32();
                                                                phi_19075_ = u32();
                                                                phi_19077_ = false;
                                                                phi_19078_ = false;
                                                            } else {
                                                                let _e4634 = (_e4622 - 1u);
                                                                let _e4635 = (_e4634 < 64u);
                                                                if _e4635 {
                                                                } else {
                                                                    phi_36259_ = true;
                                                                    phi_21865_ = bool();
                                                                    phi_21864_ = vec4<f32>();
                                                                    break;
                                                                }
                                                                let _e4637 = local_68[_e4634];
                                                                let _e4638 = (_e4637 >= _e4630);
                                                                if _e4638 {
                                                                    phi_19072_ = u32();
                                                                } else {
                                                                    if _e4635 {
                                                                    } else {
                                                                        phi_36259_ = true;
                                                                        phi_21865_ = bool();
                                                                        phi_21864_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    let _e4640 = local_67[_e4634];
                                                                    phi_19072_ = _e4640;
                                                                }
                                                                let _e4642 = phi_19072_;
                                                                phi_19074_ = _e4634;
                                                                phi_19075_ = _e4642;
                                                                phi_19077_ = select(true, false, _e4638);
                                                                phi_19078_ = _e4638;
                                                            }
                                                            let _e4645 = phi_19074_;
                                                            let _e4647 = phi_19075_;
                                                            let _e4649 = phi_19077_;
                                                            let _e4651 = phi_19078_;
                                                            phi_19079_ = _e4645;
                                                            phi_19080_ = _e4647;
                                                            phi_19081_ = _e4645;
                                                            phi_19082_ = _e4649;
                                                            phi_19083_ = _e4651;
                                                            phi_19084_ = _e4633;
                                                        } else {
                                                            phi_19079_ = u32();
                                                            phi_19080_ = _e4620;
                                                            phi_19081_ = _e4622;
                                                            phi_19082_ = true;
                                                            phi_19083_ = false;
                                                            phi_19084_ = false;
                                                        }
                                                        let _e4653 = phi_19079_;
                                                        let _e4655 = phi_19080_;
                                                        let _e4657 = phi_19081_;
                                                        let _e4659 = phi_19082_;
                                                        let _e4661 = phi_19083_;
                                                        let _e4663 = phi_19084_;
                                                        if _e4659 {
                                                            let _e4665 = ((_e4655 & 2147483648u) == 0u);
                                                            if _e4665 {
                                                                let _e4673 = global_3.member[_e4655];
                                                                if (_e4673.member_1 == 4294967295u) {
                                                                    phi_19113_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4685 = ((_e4673.member.x - _e1114) * _e4614.x);
                                                                    let _e4687 = ((_e4673.member.y - _e1115) * _e4614.y);
                                                                    let _e4689 = ((_e4673.member.z - _e1116) * _e4614.z);
                                                                    let _e4696 = ((_e4673.member_2.x - _e1114) * _e4614.x);
                                                                    let _e4697 = ((_e4673.member_2.y - _e1115) * _e4614.y);
                                                                    let _e4698 = ((_e4673.member_2.z - _e1116) * _e4614.z);
                                                                    if (_e4685 == _e4685) {
                                                                        let _e4700 = (_e4696 == _e4696);
                                                                        if _e4700 {
                                                                            let _e4701 = (_e4685 != _e4685);
                                                                            if _e4701 {
                                                                                phi_30362_ = true;
                                                                            } else {
                                                                                phi_30362_ = (_e4696 <= _e4685);
                                                                            }
                                                                            let _e4704 = phi_30362_;
                                                                            let _e4705 = select(_e4685, _e4696, _e4704);
                                                                            if _e4701 {
                                                                                phi_30392_ = true;
                                                                            } else {
                                                                                phi_30392_ = (_e4696 >= _e4685);
                                                                            }
                                                                            let _e4710 = phi_30392_;
                                                                            let _e4711 = select(_e4685, _e4696, _e4710);
                                                                            phi_30339_ = type_56(select(-300000000000000000000000000000000000000f, _e4705, (_e4705 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4711, (_e4711 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30339_ = type_56();
                                                                        }
                                                                        let _e4716 = phi_30339_;
                                                                        phi_30342_ = _e4716;
                                                                        phi_30343_ = select(true, false, _e4700);
                                                                    } else {
                                                                        phi_30342_ = type_56();
                                                                        phi_30343_ = true;
                                                                    }
                                                                    let _e4719 = phi_30342_;
                                                                    let _e4721 = phi_30343_;
                                                                    if _e4721 {
                                                                        phi_30348_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30348_ = _e4719;
                                                                    }
                                                                    let _e4723 = phi_30348_;
                                                                    if (_e4687 == _e4687) {
                                                                        let _e4727 = (_e4697 == _e4697);
                                                                        if _e4727 {
                                                                            let _e4728 = (_e4687 != _e4687);
                                                                            if _e4728 {
                                                                                phi_30449_ = true;
                                                                            } else {
                                                                                phi_30449_ = (_e4697 <= _e4687);
                                                                            }
                                                                            let _e4731 = phi_30449_;
                                                                            let _e4732 = select(_e4687, _e4697, _e4731);
                                                                            if (_e4723.member != _e4723.member) {
                                                                                phi_30464_ = true;
                                                                            } else {
                                                                                phi_30464_ = (_e4732 >= _e4723.member);
                                                                            }
                                                                            let _e4736 = phi_30464_;
                                                                            if _e4728 {
                                                                                phi_30479_ = true;
                                                                            } else {
                                                                                phi_30479_ = (_e4697 >= _e4687);
                                                                            }
                                                                            let _e4740 = phi_30479_;
                                                                            let _e4741 = select(_e4687, _e4697, _e4740);
                                                                            if (_e4723.member_1 != _e4723.member_1) {
                                                                                phi_30494_ = true;
                                                                            } else {
                                                                                phi_30494_ = (_e4741 <= _e4723.member_1);
                                                                            }
                                                                            let _e4745 = phi_30494_;
                                                                            phi_30426_ = type_56(select(_e4723.member, _e4732, _e4736), select(_e4723.member_1, _e4741, _e4745));
                                                                        } else {
                                                                            phi_30426_ = type_56();
                                                                        }
                                                                        let _e4749 = phi_30426_;
                                                                        phi_30429_ = _e4749;
                                                                        phi_30430_ = select(true, false, _e4727);
                                                                    } else {
                                                                        phi_30429_ = type_56();
                                                                        phi_30430_ = true;
                                                                    }
                                                                    let _e4752 = phi_30429_;
                                                                    let _e4754 = phi_30430_;
                                                                    if _e4754 {
                                                                        phi_30435_ = _e4723;
                                                                    } else {
                                                                        phi_30435_ = _e4752;
                                                                    }
                                                                    let _e4756 = phi_30435_;
                                                                    if (_e4689 == _e4689) {
                                                                        let _e4760 = (_e4698 == _e4698);
                                                                        if _e4760 {
                                                                            let _e4761 = (_e4689 != _e4689);
                                                                            if _e4761 {
                                                                                phi_30536_ = true;
                                                                            } else {
                                                                                phi_30536_ = (_e4698 <= _e4689);
                                                                            }
                                                                            let _e4764 = phi_30536_;
                                                                            let _e4765 = select(_e4689, _e4698, _e4764);
                                                                            if (_e4756.member != _e4756.member) {
                                                                                phi_30551_ = true;
                                                                            } else {
                                                                                phi_30551_ = (_e4765 >= _e4756.member);
                                                                            }
                                                                            let _e4769 = phi_30551_;
                                                                            if _e4761 {
                                                                                phi_30566_ = true;
                                                                            } else {
                                                                                phi_30566_ = (_e4698 >= _e4689);
                                                                            }
                                                                            let _e4773 = phi_30566_;
                                                                            let _e4774 = select(_e4689, _e4698, _e4773);
                                                                            if (_e4756.member_1 != _e4756.member_1) {
                                                                                phi_30581_ = true;
                                                                            } else {
                                                                                phi_30581_ = (_e4774 <= _e4756.member_1);
                                                                            }
                                                                            let _e4778 = phi_30581_;
                                                                            phi_30513_ = type_56(select(_e4756.member, _e4765, _e4769), select(_e4756.member_1, _e4774, _e4778));
                                                                        } else {
                                                                            phi_30513_ = type_56();
                                                                        }
                                                                        let _e4782 = phi_30513_;
                                                                        phi_30516_ = _e4782;
                                                                        phi_30517_ = select(true, false, _e4760);
                                                                    } else {
                                                                        phi_30516_ = type_56();
                                                                        phi_30517_ = true;
                                                                    }
                                                                    let _e4785 = phi_30516_;
                                                                    let _e4787 = phi_30517_;
                                                                    if _e4787 {
                                                                        phi_30522_ = _e4756;
                                                                    } else {
                                                                        phi_30522_ = _e4785;
                                                                    }
                                                                    let _e4789 = phi_30522_;
                                                                    if (_e4789.member_1 < 0f) {
                                                                        phi_30320_ = true;
                                                                    } else {
                                                                        if (_e4789.member > _e4789.member_1) {
                                                                            phi_30317_ = true;
                                                                        } else {
                                                                            phi_30317_ = (_e4789.member > _e4630);
                                                                        }
                                                                        let _e4796 = phi_30317_;
                                                                        phi_30320_ = _e4796;
                                                                    }
                                                                    let _e4798 = phi_30320_;
                                                                    phi_19113_ = select(_e4789.member, 300000000000000000000000000000000000000f, _e4798);
                                                                }
                                                                let _e4801 = phi_19113_;
                                                                if (_e4673.member_5 == 4294967295u) {
                                                                    phi_19122_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4813 = ((_e4673.member_4.x - _e1114) * _e4614.x);
                                                                    let _e4815 = ((_e4673.member_4.y - _e1115) * _e4614.y);
                                                                    let _e4817 = ((_e4673.member_4.z - _e1116) * _e4614.z);
                                                                    let _e4824 = ((_e4673.member_6.x - _e1114) * _e4614.x);
                                                                    let _e4825 = ((_e4673.member_6.y - _e1115) * _e4614.y);
                                                                    let _e4826 = ((_e4673.member_6.z - _e1116) * _e4614.z);
                                                                    if (_e4813 == _e4813) {
                                                                        let _e4828 = (_e4824 == _e4824);
                                                                        if _e4828 {
                                                                            let _e4829 = (_e4813 != _e4813);
                                                                            if _e4829 {
                                                                                phi_30683_ = true;
                                                                            } else {
                                                                                phi_30683_ = (_e4824 <= _e4813);
                                                                            }
                                                                            let _e4832 = phi_30683_;
                                                                            let _e4833 = select(_e4813, _e4824, _e4832);
                                                                            if _e4829 {
                                                                                phi_30713_ = true;
                                                                            } else {
                                                                                phi_30713_ = (_e4824 >= _e4813);
                                                                            }
                                                                            let _e4838 = phi_30713_;
                                                                            let _e4839 = select(_e4813, _e4824, _e4838);
                                                                            phi_30660_ = type_56(select(-300000000000000000000000000000000000000f, _e4833, (_e4833 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4839, (_e4839 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30660_ = type_56();
                                                                        }
                                                                        let _e4844 = phi_30660_;
                                                                        phi_30663_ = _e4844;
                                                                        phi_30664_ = select(true, false, _e4828);
                                                                    } else {
                                                                        phi_30663_ = type_56();
                                                                        phi_30664_ = true;
                                                                    }
                                                                    let _e4847 = phi_30663_;
                                                                    let _e4849 = phi_30664_;
                                                                    if _e4849 {
                                                                        phi_30669_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30669_ = _e4847;
                                                                    }
                                                                    let _e4851 = phi_30669_;
                                                                    if (_e4815 == _e4815) {
                                                                        let _e4855 = (_e4825 == _e4825);
                                                                        if _e4855 {
                                                                            let _e4856 = (_e4815 != _e4815);
                                                                            if _e4856 {
                                                                                phi_30770_ = true;
                                                                            } else {
                                                                                phi_30770_ = (_e4825 <= _e4815);
                                                                            }
                                                                            let _e4859 = phi_30770_;
                                                                            let _e4860 = select(_e4815, _e4825, _e4859);
                                                                            if (_e4851.member != _e4851.member) {
                                                                                phi_30785_ = true;
                                                                            } else {
                                                                                phi_30785_ = (_e4860 >= _e4851.member);
                                                                            }
                                                                            let _e4864 = phi_30785_;
                                                                            if _e4856 {
                                                                                phi_30800_ = true;
                                                                            } else {
                                                                                phi_30800_ = (_e4825 >= _e4815);
                                                                            }
                                                                            let _e4868 = phi_30800_;
                                                                            let _e4869 = select(_e4815, _e4825, _e4868);
                                                                            if (_e4851.member_1 != _e4851.member_1) {
                                                                                phi_30815_ = true;
                                                                            } else {
                                                                                phi_30815_ = (_e4869 <= _e4851.member_1);
                                                                            }
                                                                            let _e4873 = phi_30815_;
                                                                            phi_30747_ = type_56(select(_e4851.member, _e4860, _e4864), select(_e4851.member_1, _e4869, _e4873));
                                                                        } else {
                                                                            phi_30747_ = type_56();
                                                                        }
                                                                        let _e4877 = phi_30747_;
                                                                        phi_30750_ = _e4877;
                                                                        phi_30751_ = select(true, false, _e4855);
                                                                    } else {
                                                                        phi_30750_ = type_56();
                                                                        phi_30751_ = true;
                                                                    }
                                                                    let _e4880 = phi_30750_;
                                                                    let _e4882 = phi_30751_;
                                                                    if _e4882 {
                                                                        phi_30756_ = _e4851;
                                                                    } else {
                                                                        phi_30756_ = _e4880;
                                                                    }
                                                                    let _e4884 = phi_30756_;
                                                                    if (_e4817 == _e4817) {
                                                                        let _e4888 = (_e4826 == _e4826);
                                                                        if _e4888 {
                                                                            let _e4889 = (_e4817 != _e4817);
                                                                            if _e4889 {
                                                                                phi_30857_ = true;
                                                                            } else {
                                                                                phi_30857_ = (_e4826 <= _e4817);
                                                                            }
                                                                            let _e4892 = phi_30857_;
                                                                            let _e4893 = select(_e4817, _e4826, _e4892);
                                                                            if (_e4884.member != _e4884.member) {
                                                                                phi_30872_ = true;
                                                                            } else {
                                                                                phi_30872_ = (_e4893 >= _e4884.member);
                                                                            }
                                                                            let _e4897 = phi_30872_;
                                                                            if _e4889 {
                                                                                phi_30887_ = true;
                                                                            } else {
                                                                                phi_30887_ = (_e4826 >= _e4817);
                                                                            }
                                                                            let _e4901 = phi_30887_;
                                                                            let _e4902 = select(_e4817, _e4826, _e4901);
                                                                            if (_e4884.member_1 != _e4884.member_1) {
                                                                                phi_30902_ = true;
                                                                            } else {
                                                                                phi_30902_ = (_e4902 <= _e4884.member_1);
                                                                            }
                                                                            let _e4906 = phi_30902_;
                                                                            phi_30834_ = type_56(select(_e4884.member, _e4893, _e4897), select(_e4884.member_1, _e4902, _e4906));
                                                                        } else {
                                                                            phi_30834_ = type_56();
                                                                        }
                                                                        let _e4910 = phi_30834_;
                                                                        phi_30837_ = _e4910;
                                                                        phi_30838_ = select(true, false, _e4888);
                                                                    } else {
                                                                        phi_30837_ = type_56();
                                                                        phi_30838_ = true;
                                                                    }
                                                                    let _e4913 = phi_30837_;
                                                                    let _e4915 = phi_30838_;
                                                                    if _e4915 {
                                                                        phi_30843_ = _e4884;
                                                                    } else {
                                                                        phi_30843_ = _e4913;
                                                                    }
                                                                    let _e4917 = phi_30843_;
                                                                    if (_e4917.member_1 < 0f) {
                                                                        phi_30641_ = true;
                                                                    } else {
                                                                        if (_e4917.member > _e4917.member_1) {
                                                                            phi_30638_ = true;
                                                                        } else {
                                                                            phi_30638_ = (_e4917.member > _e4630);
                                                                        }
                                                                        let _e4924 = phi_30638_;
                                                                        phi_30641_ = _e4924;
                                                                    }
                                                                    let _e4926 = phi_30641_;
                                                                    phi_19122_ = select(_e4917.member, 300000000000000000000000000000000000000f, _e4926);
                                                                }
                                                                let _e4929 = phi_19122_;
                                                                if (_e4673.member_3 > 0u) {
                                                                    phi_19133_ = ((2147483648u | ((_e4673.member_3 - 1u) << bitcast<u32>(28u))) | _e4673.member_1);
                                                                } else {
                                                                    phi_19133_ = _e4673.member_1;
                                                                }
                                                                let _e4938 = phi_19133_;
                                                                if (_e4673.member_7 > 0u) {
                                                                    phi_19144_ = ((2147483648u | ((_e4673.member_7 - 1u) << bitcast<u32>(28u))) | _e4673.member_5);
                                                                } else {
                                                                    phi_19144_ = _e4673.member_5;
                                                                }
                                                                let _e4947 = phi_19144_;
                                                                let _e4948 = (_e4801 <= _e4929);
                                                                let _e4952 = select(_e4801, _e4929, _e4948);
                                                                if (_e4952 < 300000000000000000000000000000000000000f) {
                                                                    let _e4954 = (_e4657 < 64u);
                                                                    if _e4954 {
                                                                        if _e4954 {
                                                                        } else {
                                                                            phi_36259_ = true;
                                                                            phi_21865_ = bool();
                                                                            phi_21864_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_67[_e4657] = select(_e4938, _e4947, _e4948);
                                                                        if _e4954 {
                                                                        } else {
                                                                            phi_36259_ = true;
                                                                            phi_21865_ = bool();
                                                                            phi_21864_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_68[_e4657] = _e4952;
                                                                        phi_19183_ = (_e4657 + 1u);
                                                                    } else {
                                                                        phi_19183_ = _e4657;
                                                                    }
                                                                    let _e4959 = phi_19183_;
                                                                    phi_19184_ = _e4959;
                                                                } else {
                                                                    phi_19184_ = _e4657;
                                                                }
                                                                let _e4961 = phi_19184_;
                                                                phi_19195_ = select(4294967295u, select(_e4947, _e4938, _e4948), (select(_e4929, _e4801, _e4948) < 300000000000000000000000000000000000000f));
                                                                phi_19196_ = _e4961;
                                                                phi_19197_ = u32();
                                                                phi_19198_ = u32();
                                                            } else {
                                                                let _e4666 = (_e4655 & 268435455u);
                                                                phi_19195_ = u32();
                                                                phi_19196_ = u32();
                                                                phi_19197_ = ((_e4666 + ((_e4655 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                                phi_19198_ = _e4666;
                                                            }
                                                            let _e4965 = phi_19195_;
                                                            let _e4967 = phi_19196_;
                                                            let _e4969 = phi_19197_;
                                                            let _e4971 = phi_19198_;
                                                            phi_19200_ = _e4965;
                                                            phi_19201_ = _e4967;
                                                            phi_19203_ = _e4969;
                                                            phi_19204_ = _e4971;
                                                            phi_19205_ = select(true, false, _e4665);
                                                            phi_19206_ = _e4665;
                                                        } else {
                                                            phi_19200_ = u32();
                                                            phi_19201_ = u32();
                                                            phi_19203_ = _e4624;
                                                            phi_19204_ = _e4626;
                                                            phi_19205_ = _e4661;
                                                            phi_19206_ = false;
                                                        }
                                                        let _e4974 = phi_19200_;
                                                        let _e4976 = phi_19201_;
                                                        let _e4978 = phi_19203_;
                                                        let _e4980 = phi_19204_;
                                                        let _e4982 = phi_19205_;
                                                        let _e4984 = phi_19206_;
                                                        phi_18673_ = _e4618;
                                                        phi_18676_ = select(_e4974, 4294967295u, _e4982);
                                                        phi_18678_ = select(_e4976, select(_e4653, _e4657, _e4659), _e4982);
                                                        phi_18680_ = select(_e4624, _e4978, _e4982);
                                                        phi_18682_ = select(_e4626, _e4980, _e4982);
                                                        phi_18684_ = _e4628;
                                                        phi_18686_ = _e4630;
                                                        phi_19217_ = vec4<f32>();
                                                        phi_19218_ = select(_e4984, true, _e4982);
                                                        phi_19219_ = select(select(_e4663, false, _e4659), false, _e4982);
                                                    }
                                                    let _e5215 = phi_18673_;
                                                    let _e5217 = phi_18676_;
                                                    let _e5219 = phi_18678_;
                                                    let _e5221 = phi_18680_;
                                                    let _e5223 = phi_18682_;
                                                    let _e5225 = phi_18684_;
                                                    let _e5227 = phi_18686_;
                                                    let _e5229 = phi_19217_;
                                                    let _e5231 = phi_19218_;
                                                    let _e5233 = phi_19219_;
                                                    phi_18672_ = _e5215;
                                                    phi_18675_ = _e5217;
                                                    phi_18677_ = _e5219;
                                                    phi_18679_ = _e5221;
                                                    phi_18681_ = _e5223;
                                                    phi_18683_ = _e5225;
                                                    phi_18685_ = _e5227;
                                                    phi_36259_ = _e1995;
                                                    phi_21865_ = _e5233;
                                                    phi_21864_ = _e5229;
                                                    if !(_e5231) {
                                                        break;
                                                    }
                                                }
                                                let _e5236 = phi_36259_;
                                                let _e5238 = phi_21865_;
                                                let _e5240 = phi_21864_;
                                                phi_36706_ = _e4071;
                                                phi_36698_ = _e5236;
                                                if _e5236 {
                                                    break;
                                                }
                                                if _e5238 {
                                                    let _e5242 = local_80;
                                                    let _e5244 = local_81;
                                                    phi_19224_ = type_74(_e5242, _e5244);
                                                } else {
                                                    phi_19224_ = type_74();
                                                }
                                                let _e5247 = phi_19224_;
                                                if select(true, false, _e5238) {
                                                    let _e5250 = local_82;
                                                    phi_19230_ = type_74(_e5250, _e5240);
                                                } else {
                                                    phi_19230_ = _e5247;
                                                }
                                                let _e5253 = phi_19230_;
                                                if (_e5253.member != 4294967295u) {
                                                    phi_19284_ = _e1999;
                                                    phi_19285_ = _e2001;
                                                } else {
                                                    if (_e2003 == (_e1993 + 2u)) {
                                                        phi_19282_ = _e1999;
                                                        phi_19283_ = type_41(vec4<f32>(_e1114, _e1115, _e1116, -1f), vec4<f32>(_e4075.x, _e4075.y, _e4075.z, 0f), vec4<f32>(((_e831.x * _e4587.member.x) * _e4603), ((_e831.y * _e4587.member.y) * _e4603), ((_e831.z * _e4587.member.z) * _e4603), 0f));
                                                    } else {
                                                        phi_19282_ = vec3<f32>((_e1999.x + (((_e831.x * _e4587.member.x) * _e4077.x) * _e4603)), (_e1999.y + (((_e831.y * _e4587.member.y) * _e4077.y) * _e4603)), (_e1999.z + (((_e831.z * _e4587.member.z) * _e4077.z) * _e4603)));
                                                        phi_19283_ = _e2001;
                                                    }
                                                    let _e5291 = phi_19282_;
                                                    let _e5293 = phi_19283_;
                                                    phi_19284_ = _e5291;
                                                    phi_19285_ = _e5293;
                                                }
                                                let _e5295 = phi_19284_;
                                                let _e5297 = phi_19285_;
                                                phi_36701_ = _e5236;
                                                phi_19286_ = _e5295;
                                                phi_19287_ = _e5297;
                                                phi_19288_ = (_e2005 + 1u);
                                            } else {
                                                phi_36701_ = _e1995;
                                                phi_19286_ = _e1999;
                                                phi_19287_ = _e2001;
                                                phi_19288_ = _e2005;
                                            }
                                            let _e5299 = phi_36701_;
                                            let _e5301 = phi_19286_;
                                            let _e5303 = phi_19287_;
                                            let _e5305 = phi_19288_;
                                            phi_36700_ = _e5299;
                                            phi_19289_ = _e5301;
                                            phi_19290_ = _e5303;
                                            phi_19291_ = _e5305;
                                        } else {
                                            phi_36700_ = _e1995;
                                            phi_19289_ = _e1999;
                                            phi_19290_ = _e2001;
                                            phi_19291_ = _e2005;
                                        }
                                        let _e5307 = phi_36700_;
                                        let _e5309 = phi_19289_;
                                        let _e5311 = phi_19290_;
                                        let _e5313 = phi_19291_;
                                        phi_36986_ = _e4071;
                                        phi_36699_ = _e5307;
                                        phi_14709_ = _e5309;
                                        phi_14712_ = _e5311;
                                        phi_14714_ = (_e2003 + 1u);
                                        phi_14716_ = _e5313;
                                    } else {
                                        phi_36986_ = _e1997;
                                        phi_36699_ = _e1995;
                                        phi_14709_ = vec3<f32>();
                                        phi_14712_ = type_41();
                                        phi_14714_ = u32();
                                        phi_14716_ = u32();
                                    }
                                    let _e5316 = phi_36986_;
                                    let _e5318 = phi_36699_;
                                    let _e5320 = phi_14709_;
                                    let _e5322 = phi_14712_;
                                    let _e5324 = phi_14714_;
                                    let _e5326 = phi_14716_;
                                    phi_36331_ = _e5318;
                                    phi_34406_ = _e5316;
                                    phi_14708_ = _e5320;
                                    phi_14711_ = _e5322;
                                    phi_14713_ = _e5324;
                                    phi_14715_ = _e5326;
                                    phi_36706_ = _e5316;
                                    phi_36698_ = _e5318;
                                    if !(_e2007) {
                                        break;
                                    }
                                }
                                let _e5329 = phi_36706_;
                                let _e5331 = phi_36698_;
                                if _e5331 {
                                    break;
                                }
                                if (_e365 >= _e384) {
                                    let _e6335 = local_98;
                                    let _e6338 = local_99;
                                    let _e6341 = local_100;
                                    let _e6344 = local_101;
                                    let _e6350 = local_102;
                                    phi_19842_ = type_69(type_34(_e340.member, _e340.member_1, vec4<f32>(_e831.x, _e831.y, _e831.z, f32(_e365)), vec4<f32>(_e6338.x, _e6341.y, _e6344.z, f32(_e6335)), vec4<u32>(_e5329, (_e1990 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, _e6350);
                                    phi_19843_ = type_69();
                                    phi_19844_ = true;
                                } else {
                                    let _e5334 = ((_e5329 * 747796405u) + 2891336453u);
                                    let _e5342 = (((_e5334 >> bitcast<u32>((((_e5334 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5334) * 277803737u);
                                    let _e5345 = ((_e5342 >> bitcast<u32>(22i)) ^ _e5342);
                                    let _e5348 = f32((_e5345 >> bitcast<u32>(8i)));
                                    let _e5351 = ((_e5345 * 747796405u) + 2891336453u);
                                    let _e5359 = (((_e5351 >> bitcast<u32>((((_e5351 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5351) * 277803737u);
                                    let _e5362 = ((_e5359 >> bitcast<u32>(22i)) ^ _e5359);
                                    let _e5365 = f32((_e5362 >> bitcast<u32>(8i)));
                                    let _e5367 = ((_e5362 * 747796405u) + 2891336453u);
                                    let _e5375 = (((_e5367 >> bitcast<u32>((((_e5367 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5367) * 277803737u);
                                    let _e5378 = ((_e5375 >> bitcast<u32>(22i)) ^ _e5375);
                                    let _e5383 = (_e1982 == 0u);
                                    if _e5383 {
                                        phi_31012_ = vec3<f32>();
                                        phi_31013_ = true;
                                    } else {
                                        if (_e1982 == 2u) {
                                            let _e5389 = (((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z));
                                            if (_e5389 != _e5389) {
                                                phi_31053_ = true;
                                            } else {
                                                phi_31053_ = (0f >= _e5389);
                                            }
                                            let _e5393 = phi_31053_;
                                            let _e5394 = select(_e5389, 0f, _e5393);
                                            let _e5395 = (1f / _e1972);
                                            let _e5399 = ((_e5395 * _e5395) * (1f - (_e5394 * _e5394)));
                                            if (_e5399 >= 1f) {
                                                phi_31072_ = 1f;
                                            } else {
                                                let _e5402 = sqrt((1f - _e5399));
                                                if (_e5394 != _e5394) {
                                                    phi_31083_ = true;
                                                } else {
                                                    phi_31083_ = (_e5402 <= _e5394);
                                                }
                                                let _e5406 = phi_31083_;
                                                let _e5411 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                phi_31072_ = (_e5411 + ((1f - _e5411) * pow((1f - select(_e5394, _e5402, _e5406)), 5f)));
                                            }
                                            let _e5418 = phi_31072_;
                                            phi_31003_ = select(true, false, ((f32((_e5378 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5418));
                                        } else {
                                            phi_31003_ = true;
                                        }
                                        let _e5422 = phi_31003_;
                                        if _e5422 {
                                            let _e5427 = (((_e1063 * _e1137) + (_e1064 * _e1138)) + (_e1065 * _e1140));
                                            let _e5432 = (((_e1063 * _e1133) + (_e1064 * _e1143)) + (_e1065 * _e1144));
                                            let _e5437 = (((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z));
                                            if (_e5437 <= 0f) {
                                                phi_31237_ = vec3<f32>();
                                                phi_31238_ = true;
                                            } else {
                                                let _e5440 = (_e1980 * _e5427);
                                                let _e5441 = (_e1980 * _e5432);
                                                let _e5448 = (1f / sqrt((((_e5440 * _e5440) + (_e5441 * _e5441)) + (_e5437 * _e5437))));
                                                let _e5449 = (_e5440 * _e5448);
                                                let _e5450 = (_e5441 * _e5448);
                                                let _e5451 = (_e5437 * _e5448);
                                                let _e5454 = ((_e5449 * _e5449) + (_e5450 * _e5450));
                                                if (_e5454 > 0f) {
                                                    let _e5457 = sqrt(_e5454);
                                                    phi_31139_ = vec3<f32>((-(_e5450) / _e5457), (_e5449 / _e5457), 0f);
                                                } else {
                                                    phi_31139_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e5462 = phi_31139_;
                                                let _e5475 = sqrt((_e5348 * 0.000000059604645f));
                                                let _e5476 = (_e5365 * 0.00000037450704f);
                                                let _e5478 = (_e5475 * cos(_e5476));
                                                let _e5480 = (0.5f * (1f + _e5451));
                                                let _e5483 = (1f - (_e5478 * _e5478));
                                                if (_e5483 != _e5483) {
                                                    phi_31295_ = true;
                                                } else {
                                                    phi_31295_ = (0f >= _e5483);
                                                }
                                                let _e5487 = phi_31295_;
                                                let _e5494 = (((1f - _e5480) * sqrt(select(_e5483, 0f, _e5487))) + ((_e5480 * _e5475) * sin(_e5476)));
                                                let _e5505 = (_e5483 - (_e5494 * _e5494));
                                                if (_e5505 != _e5505) {
                                                    phi_31310_ = true;
                                                } else {
                                                    phi_31310_ = (0f >= _e5505);
                                                }
                                                let _e5509 = phi_31310_;
                                                let _e5511 = sqrt(select(_e5505, 0f, _e5509));
                                                let _e5517 = (((_e5462.z * _e5478) + (((_e5449 * _e5462.y) - (_e5462.x * _e5450)) * _e5494)) + (_e5451 * _e5511));
                                                let _e5518 = (_e1980 * (((_e5462.x * _e5478) + (((_e5450 * _e5462.z) - (_e5462.y * _e5451)) * _e5494)) + (_e5449 * _e5511)));
                                                let _e5519 = (_e1980 * (((_e5462.y * _e5478) + (((_e5451 * _e5462.x) - (_e5462.z * _e5449)) * _e5494)) + (_e5450 * _e5511)));
                                                if (_e5517 != _e5517) {
                                                    phi_31325_ = true;
                                                } else {
                                                    phi_31325_ = (0f >= _e5517);
                                                }
                                                let _e5523 = phi_31325_;
                                                let _e5524 = select(_e5517, 0f, _e5523);
                                                let _e5531 = (1f / sqrt((((_e5518 * _e5518) + (_e5519 * _e5519)) + (_e5524 * _e5524))));
                                                let _e5532 = (_e5518 * _e5531);
                                                let _e5533 = (_e5519 * _e5531);
                                                let _e5534 = (_e5524 * _e5531);
                                                let _e5540 = (2f * (((_e5427 * _e5532) + (_e5432 * _e5533)) + (_e5437 * _e5534)));
                                                let _e5543 = (_e5534 * _e5540);
                                                let _e5546 = (vec3<f32>((_e5532 * _e5540), (_e5533 * _e5540), _e5543) - vec3<f32>(_e5427, _e5432, _e5437));
                                                let _e5547 = ((_e5543 - _e5437) <= 0f);
                                                if _e5547 {
                                                    phi_31234_ = vec3<f32>();
                                                } else {
                                                    phi_31234_ = vec3<f32>((((_e1137 * _e5546.x) + (_e1133 * _e5546.y)) + (_e1089.x * _e5546.z)), (((_e1138 * _e5546.x) + (_e1143 * _e5546.y)) + (_e1089.y * _e5546.z)), (((_e1140 * _e5546.x) + (_e1144 * _e5546.y)) + (_e1089.z * _e5546.z)));
                                                }
                                                let _e5568 = phi_31234_;
                                                phi_31237_ = _e5568;
                                                phi_31238_ = _e5547;
                                            }
                                            let _e5570 = phi_31237_;
                                            let _e5572 = phi_31238_;
                                            phi_31008_ = select(_e5570, vec3<f32>(0f, 0f, 0f), vec3(_e5572));
                                        } else {
                                            phi_31008_ = vec3<f32>();
                                        }
                                        let _e5576 = phi_31008_;
                                        phi_31012_ = _e5576;
                                        phi_31013_ = select(true, false, _e5422);
                                    }
                                    let _e5579 = phi_31012_;
                                    let _e5581 = phi_31013_;
                                    if _e5581 {
                                        let _e5582 = (_e5348 * 0.00000037450704f);
                                        let _e5584 = ((_e5365 * 0.00000011920929f) - 1f);
                                        let _e5586 = (1f - (_e5584 * _e5584));
                                        if (_e5586 != _e5586) {
                                            phi_31425_ = true;
                                        } else {
                                            phi_31425_ = (0f >= _e5586);
                                        }
                                        let _e5590 = phi_31425_;
                                        let _e5592 = sqrt(select(_e5586, 0f, _e5590));
                                        let _e5597 = (_e1089.x + (_e5592 * cos(_e5582)));
                                        let _e5598 = (_e1089.y + (_e5592 * sin(_e5582)));
                                        let _e5599 = (_e1089.z + _e5584);
                                        let _e5605 = sqrt((((_e5597 * _e5597) + (_e5598 * _e5598)) + (_e5599 * _e5599)));
                                        if (_e5605 < 0.000001f) {
                                            phi_31399_ = _e1089;
                                        } else {
                                            let _e5607 = (1f / _e5605);
                                            phi_31399_ = vec3<f32>((_e5597 * _e5607), (_e5598 * _e5607), (_e5599 * _e5607));
                                        }
                                        let _e5613 = phi_31399_;
                                        phi_31020_ = _e5613;
                                    } else {
                                        phi_31020_ = _e5579;
                                    }
                                    let _e5615 = phi_31020_;
                                    let _e5623 = (((_e5615.x * _e1089.x) + (_e5615.y * _e1089.y)) + (_e5615.z * _e1089.z));
                                    if (_e5623 != _e5623) {
                                        phi_31440_ = true;
                                    } else {
                                        phi_31440_ = (0f >= _e5623);
                                    }
                                    let _e5627 = phi_31440_;
                                    let _e5629 = (select(_e5623, 0f, _e5627) * 0.31830987f);
                                    if _e5383 {
                                        if (_e5623 <= 0f) {
                                            phi_32237_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_32237_ = (_e1976 * (_e5623 * 0.31830987f));
                                        }
                                        let _e6104 = phi_32237_;
                                        phi_19683_ = type_59(_e6104, _e5629);
                                        phi_19684_ = type_59();
                                        phi_19685_ = true;
                                    } else {
                                        let _e5634 = (((_e1063 * _e1137) + (_e1064 * _e1138)) + (_e1065 * _e1140));
                                        let _e5639 = (((_e1063 * _e1133) + (_e1064 * _e1143)) + (_e1065 * _e1144));
                                        let _e5644 = (((_e1063 * _e1089.x) + (_e1064 * _e1089.y)) + (_e1065 * _e1089.z));
                                        let _e5649 = (((_e5615.x * _e1137) + (_e5615.y * _e1138)) + (_e5615.z * _e1140));
                                        let _e5654 = (((_e5615.x * _e1133) + (_e5615.y * _e1143)) + (_e5615.z * _e1144));
                                        if (_e5644 <= 0f) {
                                            phi_19647_ = type_59();
                                            phi_19648_ = true;
                                        } else {
                                            let _e5656 = (_e5623 <= 0f);
                                            if _e5656 {
                                                phi_19646_ = type_59();
                                            } else {
                                                let _e5657 = (_e5634 + _e5649);
                                                let _e5658 = (_e5639 + _e5654);
                                                let _e5659 = (_e5644 + _e5623);
                                                let _e5666 = (1f / sqrt((((_e5657 * _e5657) + (_e5658 * _e5658)) + (_e5659 * _e5659))));
                                                let _e5669 = (_e5659 * _e5666);
                                                if (_e5669 <= 0f) {
                                                    phi_31550_ = 0f;
                                                } else {
                                                    let _e5671 = (_e1980 * _e1980);
                                                    let _e5675 = ((((_e5671 - 1f) * _e5669) * _e5669) + 1f);
                                                    phi_31550_ = (_e5671 / ((3.1415927f * _e5675) * _e5675));
                                                }
                                                let _e5680 = phi_31550_;
                                                let _e5681 = (_e5644 * _e5644);
                                                if (_e5681 >= 1f) {
                                                    phi_31578_ = 0f;
                                                } else {
                                                    phi_31578_ = ((sqrt((1f + (((_e1980 * _e1980) * ((_e5634 * _e5634) + (_e5639 * _e5639))) / _e5681))) - 1f) * 0.5f);
                                                }
                                                let _e5694 = phi_31578_;
                                                let _e5695 = (1f + _e5694);
                                                let _e5696 = (_e5623 * _e5623);
                                                if (_e5696 >= 1f) {
                                                    phi_31606_ = 0f;
                                                } else {
                                                    phi_31606_ = ((sqrt((1f + (((_e1980 * _e1980) * ((_e5649 * _e5649) + (_e5654 * _e5654))) / _e5696))) - 1f) * 0.5f);
                                                }
                                                let _e5709 = phi_31606_;
                                                let _e5716 = (((_e5634 * (_e5657 * _e5666)) + (_e5639 * (_e5658 * _e5666))) + (_e5644 * _e5669));
                                                if (_e1978 == 0u) {
                                                    let _e6011 = (1f - _e5716);
                                                    if (_e6011 != _e6011) {
                                                        phi_32109_ = true;
                                                    } else {
                                                        phi_32109_ = (0f >= _e6011);
                                                    }
                                                    let _e6015 = phi_32109_;
                                                    let _e6017 = pow(select(_e6011, 0f, _e6015), 5f);
                                                    let _e6018 = (1f - _e6017);
                                                    phi_19634_ = vec3<f32>(((_e1976.x * _e6018) + _e6017), ((_e1976.y * _e6018) + _e6017), ((_e1976.z * _e6018) + _e6017));
                                                } else {
                                                    if (_e1978 == 1u) {
                                                        let _e5984 = (1f / _e1972);
                                                        let _e5988 = ((_e5984 * _e5984) * (1f - (_e5716 * _e5716)));
                                                        if (_e5988 >= 1f) {
                                                            phi_32070_ = 1f;
                                                        } else {
                                                            let _e5991 = sqrt((1f - _e5988));
                                                            if (_e5716 != _e5716) {
                                                                phi_32081_ = true;
                                                            } else {
                                                                phi_32081_ = (_e5991 <= _e5716);
                                                            }
                                                            let _e5995 = phi_32081_;
                                                            let _e6000 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                            phi_32070_ = (_e6000 + ((1f - _e6000) * pow((1f - select(_e5716, _e5991, _e5995)), 5f)));
                                                        }
                                                        let _e6007 = phi_32070_;
                                                        phi_19633_ = vec3(_e6007);
                                                    } else {
                                                        let _e5720 = select(_e5716, 0f, (_e5716 < 0f));
                                                        let _e5722 = select(_e5720, 1f, (_e5720 > 1f));
                                                        let _e5724 = (1f - (_e5722 * _e5722));
                                                        let _e5737 = ((_e1976.x * _e1976.x) - (_e1974.x * _e1974.x));
                                                        let _e5738 = ((_e1976.y * _e1976.y) - (_e1974.y * _e1974.y));
                                                        let _e5739 = ((_e1976.z * _e1976.z) - (_e1974.z * _e1974.z));
                                                        let _e5741 = (_e1976.x * (_e1974.x + _e1974.x));
                                                        let _e5743 = (_e1976.y * (_e1974.y + _e1974.y));
                                                        let _e5745 = (_e1976.z * (_e1974.z + _e1974.z));
                                                        let _e5752 = ((_e5737 * _e5737) + (_e5741 * _e5741));
                                                        let _e5753 = ((_e5738 * _e5738) + (_e5743 * _e5743));
                                                        let _e5754 = ((_e5739 * _e5739) + (_e5745 * _e5745));
                                                        let _e5759 = ((_e5724 * _e5737) / _e5752);
                                                        let _e5760 = ((_e5724 * _e5738) / _e5753);
                                                        let _e5761 = ((_e5724 * _e5739) / _e5754);
                                                        let _e5762 = (1f - _e5759);
                                                        let _e5763 = (1f - _e5760);
                                                        let _e5764 = (1f - _e5761);
                                                        let _e5766 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5759, _e5760, _e5761));
                                                        let _e5767 = (_e5724 * _e5741);
                                                        let _e5768 = (_e5724 * _e5743);
                                                        let _e5769 = (_e5724 * _e5745);
                                                        let _e5774 = (vec3<f32>(_e5767, _e5768, _e5769) / vec3<f32>(_e5752, _e5753, _e5754));
                                                        let _e5787 = ((_e5766.x * _e5766.x) + (_e5774.x * _e5774.x));
                                                        let _e5788 = ((_e5766.y * _e5766.y) + (_e5774.y * _e5774.y));
                                                        let _e5789 = ((_e5766.z * _e5766.z) + (_e5774.z * _e5774.z));
                                                        if (_e5787 != _e5787) {
                                                            phi_31740_ = true;
                                                        } else {
                                                            phi_31740_ = (0f >= _e5787);
                                                        }
                                                        let _e5793 = phi_31740_;
                                                        let _e5795 = sqrt(select(_e5787, 0f, _e5793));
                                                        if (_e5788 != _e5788) {
                                                            phi_31755_ = true;
                                                        } else {
                                                            phi_31755_ = (0f >= _e5788);
                                                        }
                                                        let _e5799 = phi_31755_;
                                                        let _e5801 = sqrt(select(_e5788, 0f, _e5799));
                                                        if (_e5789 != _e5789) {
                                                            phi_31770_ = true;
                                                        } else {
                                                            phi_31770_ = (0f >= _e5789);
                                                        }
                                                        let _e5805 = phi_31770_;
                                                        let _e5807 = sqrt(select(_e5789, 0f, _e5805));
                                                        let _e5811 = ((_e5795 + _e5762) * 0.5f);
                                                        let _e5812 = ((_e5801 + _e5763) * 0.5f);
                                                        let _e5813 = ((_e5807 + _e5764) * 0.5f);
                                                        if (_e5811 != _e5811) {
                                                            phi_31797_ = true;
                                                        } else {
                                                            phi_31797_ = (0f >= _e5811);
                                                        }
                                                        let _e5817 = phi_31797_;
                                                        let _e5819 = sqrt(select(_e5811, 0f, _e5817));
                                                        if (_e5812 != _e5812) {
                                                            phi_31812_ = true;
                                                        } else {
                                                            phi_31812_ = (0f >= _e5812);
                                                        }
                                                        let _e5823 = phi_31812_;
                                                        let _e5825 = sqrt(select(_e5812, 0f, _e5823));
                                                        if (_e5813 != _e5813) {
                                                            phi_31827_ = true;
                                                        } else {
                                                            phi_31827_ = (0f >= _e5813);
                                                        }
                                                        let _e5829 = phi_31827_;
                                                        let _e5831 = sqrt(select(_e5813, 0f, _e5829));
                                                        let _e5832 = vec3<f32>(_e5819, _e5825, _e5831);
                                                        let _e5836 = ((_e5795 - _e5762) * 0.5f);
                                                        let _e5837 = ((_e5801 - _e5763) * 0.5f);
                                                        let _e5838 = ((_e5807 - _e5764) * 0.5f);
                                                        if (_e5836 != _e5836) {
                                                            phi_31854_ = true;
                                                        } else {
                                                            phi_31854_ = (0f >= _e5836);
                                                        }
                                                        let _e5842 = phi_31854_;
                                                        let _e5844 = sqrt(select(_e5836, 0f, _e5842));
                                                        if (_e5837 != _e5837) {
                                                            phi_31869_ = true;
                                                        } else {
                                                            phi_31869_ = (0f >= _e5837);
                                                        }
                                                        let _e5848 = phi_31869_;
                                                        let _e5850 = sqrt(select(_e5837, 0f, _e5848));
                                                        if (_e5838 != _e5838) {
                                                            phi_31884_ = true;
                                                        } else {
                                                            phi_31884_ = (0f >= _e5838);
                                                        }
                                                        let _e5854 = phi_31884_;
                                                        let _e5856 = sqrt(select(_e5838, 0f, _e5854));
                                                        if ((_e5767 / _e5752) < 0f) {
                                                            phi_19510_ = -(_e5844);
                                                        } else {
                                                            phi_19510_ = _e5844;
                                                        }
                                                        let _e5860 = phi_19510_;
                                                        if ((_e5768 / _e5753) < 0f) {
                                                            phi_19518_ = -(_e5850);
                                                        } else {
                                                            phi_19518_ = _e5850;
                                                        }
                                                        let _e5864 = phi_19518_;
                                                        if ((_e5769 / _e5754) < 0f) {
                                                            phi_19526_ = -(_e5856);
                                                        } else {
                                                            phi_19526_ = _e5856;
                                                        }
                                                        let _e5868 = phi_19526_;
                                                        let _e5872 = (_e1974.x * _e5722);
                                                        let _e5873 = (_e1974.y * _e5722);
                                                        let _e5874 = (_e1974.z * _e5722);
                                                        let _e5884 = vec3<f32>(((_e1976.x * _e5819) - (_e1974.x * _e5860)), ((_e1976.y * _e5825) - (_e1974.y * _e5864)), ((_e1976.z * _e5831) - (_e1974.z * _e5868)));
                                                        let _e5891 = ((_e1976.x * _e5860) + (_e1974.x * _e5819));
                                                        let _e5892 = ((_e1976.y * _e5864) + (_e1974.y * _e5825));
                                                        let _e5893 = ((_e1976.z * _e5868) + (_e1974.z * _e5831));
                                                        let _e5895 = vec3<f32>((_e1976.x * _e5722), (_e1976.y * _e5722), (_e1976.z * _e5722));
                                                        let _e5896 = (_e5895 - _e5832);
                                                        let _e5897 = (_e5872 - _e5860);
                                                        let _e5898 = (_e5873 - _e5864);
                                                        let _e5899 = (_e5874 - _e5868);
                                                        let _e5912 = (_e5895 + _e5832);
                                                        let _e5913 = (_e5872 + _e5860);
                                                        let _e5914 = (_e5873 + _e5864);
                                                        let _e5915 = (_e5874 + _e5868);
                                                        let _e5925 = ((_e5912.x * _e5912.x) + (_e5913 * _e5913));
                                                        let _e5926 = ((_e5912.y * _e5912.y) + (_e5914 * _e5914));
                                                        let _e5927 = ((_e5912.z * _e5912.z) + (_e5915 * _e5915));
                                                        let _e5937 = vec3(_e5722);
                                                        let _e5938 = (_e5937 - _e5884);
                                                        let _e5939 = -(vec3<f32>(_e5891, _e5892, _e5893));
                                                        let _e5955 = (_e5937 + _e5884);
                                                        let _e5965 = ((_e5955.x * _e5955.x) + (_e5891 * _e5891));
                                                        let _e5966 = ((_e5955.y * _e5955.y) + (_e5892 * _e5892));
                                                        let _e5967 = ((_e5955.z * _e5955.z) + (_e5893 * _e5893));
                                                        phi_19633_ = vec3<f32>((((((_e5896.x * _e5896.x) + (_e5897 * _e5897)) / select(0.000000000000000000000000000001f, _e5925, (_e5925 > 0.000000000000000000000000000001f))) + (((_e5938.x * _e5938.x) + (_e5939.x * _e5939.x)) / select(0.000000000000000000000000000001f, _e5965, (_e5965 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5896.y * _e5896.y) + (_e5898 * _e5898)) / select(0.000000000000000000000000000001f, _e5926, (_e5926 > 0.000000000000000000000000000001f))) + (((_e5938.y * _e5938.y) + (_e5939.y * _e5939.y)) / select(0.000000000000000000000000000001f, _e5966, (_e5966 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5896.z * _e5896.z) + (_e5899 * _e5899)) / select(0.000000000000000000000000000001f, _e5927, (_e5927 > 0.000000000000000000000000000001f))) + (((_e5938.z * _e5938.z) + (_e5939.z * _e5939.z)) / select(0.000000000000000000000000000001f, _e5967, (_e5967 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6010 = phi_19633_;
                                                    phi_19634_ = _e6010;
                                                }
                                                let _e6030 = phi_19634_;
                                                let _e6032 = (4f * _e5644);
                                                phi_19646_ = type_59((_e6030 * (((1f / (_e5695 + _e5709)) * _e5680) / _e6032)), ((_e5680 / _e5695) / _e6032));
                                            }
                                            let _e6039 = phi_19646_;
                                            phi_19647_ = _e6039;
                                            phi_19648_ = _e5656;
                                        }
                                        let _e6041 = phi_19647_;
                                        let _e6043 = phi_19648_;
                                        if _e6043 {
                                            phi_19653_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19653_ = _e6041;
                                        }
                                        let _e6045 = phi_19653_;
                                        let _e6048 = (_e1982 == 1u);
                                        if _e6048 {
                                            phi_19681_ = _e6045;
                                            phi_19682_ = type_59();
                                        } else {
                                            if (_e5644 != _e5644) {
                                                phi_32146_ = true;
                                            } else {
                                                phi_32146_ = (0f >= _e5644);
                                            }
                                            let _e6052 = phi_32146_;
                                            let _e6053 = select(_e5644, 0f, _e6052);
                                            let _e6054 = (1f / _e1972);
                                            let _e6058 = ((_e6054 * _e6054) * (1f - (_e6053 * _e6053)));
                                            if (_e6058 >= 1f) {
                                                phi_32165_ = 1f;
                                            } else {
                                                let _e6061 = sqrt((1f - _e6058));
                                                if (_e6053 != _e6053) {
                                                    phi_32176_ = true;
                                                } else {
                                                    phi_32176_ = (_e6061 <= _e6053);
                                                }
                                                let _e6065 = phi_32176_;
                                                let _e6070 = pow(((1f - _e1972) / (1f + _e1972)), 2f);
                                                phi_32165_ = (_e6070 + ((1f - _e6070) * pow((1f - select(_e6053, _e6061, _e6065)), 5f)));
                                            }
                                            let _e6077 = phi_32165_;
                                            if (_e5623 <= 0f) {
                                                phi_32215_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_32215_ = (_e1970 * (_e5623 * 0.31830987f));
                                            }
                                            let _e6082 = phi_32215_;
                                            let _e6083 = (1f - _e6077);
                                            phi_19681_ = type_59();
                                            phi_19682_ = type_59((_e6045.member + vec3<f32>((_e6082.x * _e6083), (_e6082.y * _e6083), (_e6082.z * _e6083))), ((_e6077 * _e6045.member_1) + (_e6083 * _e5629)));
                                        }
                                        let _e6097 = phi_19681_;
                                        let _e6099 = phi_19682_;
                                        phi_19683_ = _e6097;
                                        phi_19684_ = _e6099;
                                        phi_19685_ = _e6048;
                                    }
                                    let _e6107 = phi_19683_;
                                    let _e6109 = phi_19684_;
                                    let _e6111 = phi_19685_;
                                    if _e6111 {
                                        phi_19689_ = _e6107;
                                    } else {
                                        phi_19689_ = _e6109;
                                    }
                                    let _e6113 = phi_19689_;
                                    if ((((_e5615.x * _e5615.x) + (_e5615.y * _e5615.y)) + (_e5615.z * _e5615.z)) == 0f) {
                                        phi_38543_ = _e5378;
                                        phi_19828_ = type_69();
                                        phi_19829_ = type_69();
                                        phi_19830_ = true;
                                        phi_19831_ = false;
                                    } else {
                                        let _e6122 = (_e6113.member_1 <= 0f);
                                        if _e6122 {
                                            phi_38544_ = _e5378;
                                            phi_19825_ = type_69();
                                            phi_19826_ = type_69();
                                            phi_19827_ = false;
                                        } else {
                                            let _e6124 = (_e6113.member.x / _e6113.member_1);
                                            let _e6126 = (_e6113.member.y / _e6113.member_1);
                                            let _e6128 = (_e6113.member.z / _e6113.member_1);
                                            let _e6137 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6124 * _e6124) + (_e6126 * _e6126)) + (_e6128 * _e6128))) <= _e6137) {
                                                phi_38537_ = _e5378;
                                                phi_19776_ = _e831;
                                                phi_19777_ = false;
                                            } else {
                                                let _e6141 = global_1.member.member_7[1u];
                                                if (_e6141 == 1u) {
                                                    let _e6146 = global_1.member.member_4[3u];
                                                    if (f32(_e365) >= _e6146) {
                                                        let _e6149 = select(_e6126, _e6124, (_e6124 > _e6126));
                                                        let _e6151 = select(_e6128, _e6149, (_e6149 > _e6128));
                                                        if (_e6151 != _e6151) {
                                                            phi_32262_ = true;
                                                        } else {
                                                            phi_32262_ = (1f <= _e6151);
                                                        }
                                                        let _e6155 = phi_32262_;
                                                        let _e6156 = select(_e6151, 1f, _e6155);
                                                        let _e6158 = ((_e5378 * 747796405u) + 2891336453u);
                                                        let _e6166 = (((_e6158 >> bitcast<u32>((((_e6158 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6158) * 277803737u);
                                                        let _e6169 = ((_e6166 >> bitcast<u32>(22i)) ^ _e6166);
                                                        let _e6174 = ((f32((_e6169 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6156);
                                                        if _e6174 {
                                                            phi_19765_ = vec3<f32>();
                                                        } else {
                                                            phi_19765_ = vec3<f32>((_e831.x / _e6156), (_e831.y / _e6156), (_e831.z / _e6156));
                                                        }
                                                        let _e6183 = phi_19765_;
                                                        phi_38540_ = _e6169;
                                                        phi_19767_ = _e6183;
                                                        phi_19768_ = select(true, false, _e6174);
                                                    } else {
                                                        phi_38540_ = _e5378;
                                                        phi_19767_ = _e831;
                                                        phi_19768_ = true;
                                                    }
                                                    let _e6186 = phi_38540_;
                                                    let _e6188 = phi_19767_;
                                                    let _e6190 = phi_19768_;
                                                    phi_38539_ = _e6186;
                                                    phi_19769_ = _e6188;
                                                    phi_19770_ = _e6190;
                                                } else {
                                                    phi_38539_ = _e5378;
                                                    phi_19769_ = _e831;
                                                    phi_19770_ = true;
                                                }
                                                let _e6192 = phi_38539_;
                                                let _e6194 = phi_19769_;
                                                let _e6196 = phi_19770_;
                                                phi_38537_ = _e6192;
                                                phi_19776_ = select(_e831, _e6194, vec3(_e6196));
                                                phi_19777_ = select(false, true, _e6196);
                                            }
                                            let _e6201 = phi_38537_;
                                            let _e6203 = phi_19776_;
                                            let _e6205 = phi_19777_;
                                            if _e6205 {
                                                if ((((_e5615.x * _e1077.x) + (_e5615.y * _e1077.y)) + (_e5615.z * _e1077.z)) < 0f) {
                                                    phi_19806_ = -(_e1077);
                                                } else {
                                                    phi_19806_ = _e1077;
                                                }
                                                let _e6244 = phi_19806_;
                                                let _e6261 = local_88;
                                                let _e6264 = local_89;
                                                let _e6267 = local_90;
                                                let _e6270 = local_91;
                                                let _e6276 = local_92;
                                                phi_19822_ = type_69(type_34(vec4<f32>((_e837 + (_e6244.x * _e1107)), (_e838 + (_e6244.y * _e1107)), (_e839 + (_e6244.z * _e1107)), _e340.member.w), vec4<f32>(_e5615.x, _e5615.y, _e5615.z, _e6113.member_1), vec4<f32>((_e6203.x * _e6124), (_e6203.y * _e6126), (_e6203.z * _e6128), f32((_e365 + 1u))), vec4<f32>(_e6264.x, _e6267.y, _e6270.z, f32(_e6261)), vec4<u32>(_e6201, _e1990, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, _e6276);
                                                phi_19823_ = type_69();
                                            } else {
                                                let _e6213 = local_83;
                                                let _e6216 = local_84;
                                                let _e6219 = local_85;
                                                let _e6222 = local_86;
                                                let _e6228 = local_87;
                                                phi_19822_ = type_69();
                                                phi_19823_ = type_69(type_34(_e340.member, _e340.member_1, vec4<f32>(_e6203.x, _e6203.y, _e6203.z, f32(_e365)), vec4<f32>(_e6216.x, _e6219.y, _e6222.z, f32(_e6213)), vec4<u32>(_e6201, (_e1990 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, _e6228);
                                            }
                                            let _e6279 = phi_19822_;
                                            let _e6281 = phi_19823_;
                                            phi_38544_ = _e6201;
                                            phi_19825_ = _e6279;
                                            phi_19826_ = _e6281;
                                            phi_19827_ = select(true, false, _e6205);
                                        }
                                        let _e6284 = phi_38544_;
                                        let _e6286 = phi_19825_;
                                        let _e6288 = phi_19826_;
                                        let _e6290 = phi_19827_;
                                        phi_38543_ = _e6284;
                                        phi_19828_ = _e6286;
                                        phi_19829_ = _e6288;
                                        phi_19830_ = _e6122;
                                        phi_19831_ = _e6290;
                                    }
                                    let _e6292 = phi_38543_;
                                    let _e6294 = phi_19828_;
                                    let _e6296 = phi_19829_;
                                    let _e6298 = phi_19830_;
                                    let _e6300 = phi_19831_;
                                    if _e6298 {
                                        let _e6308 = local_93;
                                        let _e6311 = local_94;
                                        let _e6314 = local_95;
                                        let _e6317 = local_96;
                                        let _e6323 = local_97;
                                        phi_19837_ = type_69(type_34(_e340.member, _e340.member_1, vec4<f32>(_e831.x, _e831.y, _e831.z, f32(_e365)), vec4<f32>(_e6311.x, _e6314.y, _e6317.z, f32(_e6308)), vec4<u32>(_e6292, (_e1990 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e833, _e6323);
                                    } else {
                                        phi_19837_ = _e6296;
                                    }
                                    let _e6326 = phi_19837_;
                                    phi_19842_ = _e6326;
                                    phi_19843_ = _e6294;
                                    phi_19844_ = select(_e6300, true, _e6298);
                                }
                                let _e6353 = phi_19842_;
                                let _e6355 = phi_19843_;
                                let _e6357 = phi_19844_;
                                phi_19848_ = _e6353;
                                phi_19849_ = _e6355;
                                phi_19850_ = _e6357;
                            } else {
                                phi_19848_ = _e1986;
                                phi_19849_ = type_69();
                                phi_19850_ = true;
                            }
                            let _e6359 = phi_19848_;
                            let _e6361 = phi_19849_;
                            let _e6363 = phi_19850_;
                            phi_19854_ = _e6359;
                            phi_19855_ = _e6361;
                            phi_19856_ = _e6363;
                        }
                        let _e6396 = phi_19854_;
                        let _e6398 = phi_19855_;
                        let _e6400 = phi_19856_;
                        phi_19857_ = _e6396;
                        phi_19858_ = _e6398;
                        phi_19859_ = _e6400;
                    }
                    let _e6595 = phi_19857_;
                    let _e6597 = phi_19858_;
                    let _e6599 = phi_19859_;
                    phi_19863_ = _e6595;
                    phi_19864_ = _e6597;
                    phi_19865_ = _e6599;
                }
                let _e7111 = phi_19863_;
                let _e7113 = phi_19864_;
                let _e7115 = phi_19865_;
                if _e7115 {
                    phi_19869_ = _e7111;
                } else {
                    phi_19869_ = _e7113;
                }
                let _e7117 = phi_19869_;
                if _e337 {
                } else {
                    break;
                }
                global_7.member[_e332.member_1] = _e7117.member;
                let _e7121 = (2u * _e332.member_1);
                if (_e7121 < _e292) {
                } else {
                    break;
                }
                global_9.member[_e7121] = _e7117.member_1;
                let _e7125 = (_e7121 + 1u);
                if (_e7125 < _e292) {
                } else {
                    break;
                }
                global_9.member[_e7125] = _e7117.member_2;
                if ((_e7117.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7133 = (1u - _e296);
                    let _e7134 = (_e7133 * 4u);
                    let _e7138 = atomicAdd((&global_8.member[(_e7134 + 3u)]), 1u);
                    let _e7142 = global_1.member.member[0u];
                    let _e7146 = global_1.member.member[1u];
                    let _e7151 = global_1.member.member_7[3u];
                    atomicStore((&global_8.member[((12u + (_e7133 * ((_e7142 * _e7146) * _e7151))) + _e7138)]), _e332.member_1);
                    if ((_e7138 % 64u) == 0u) {
                        let _e7160 = (_e7138 / 64u);
                        if (_e7160 < 65535u) {
                            let _e7164 = atomicAdd((&global_8.member[_e7134]), 1u);
                        }
                        if ((_e7160 % 65535u) == 0u) {
                            let _e7170 = atomicAdd((&global_8.member[(_e7134 + 1u)]), 1u);
                        }
                    }
                }
                if ((_e7117.member.member_4.y & 16u) == 0u) {
                } else {
                    let _e7175 = atomicAdd((&global_8.member[11u]), 1u);
                    let _e7179 = global_1.member.member[0u];
                    let _e7183 = global_1.member.member[1u];
                    let _e7188 = global_1.member.member_7[3u];
                    atomicStore((&global_8.member[((12u + (2u * ((_e7179 * _e7183) * _e7188))) + _e7175)]), _e332.member_1);
                    if ((_e7175 % 64u) == 0u) {
                        let _e7197 = (_e7175 / 64u);
                        if (_e7197 < 65535u) {
                            let _e7201 = atomicAdd((&global_8.member[8u]), 1u);
                        }
                        if ((_e7197 % 65535u) == 0u) {
                            let _e7206 = atomicAdd((&global_8.member[9u]), 1u);
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

fn function_3() {
    switch bitcast<i32>(0u) {
        default: {
            let _e280 = global;
            let _e288 = (_e280.x + (_e280.y * 4194240u));
            let _e291 = atomicLoad((&global_11.member[11u]));
            if (_e288 >= _e291) {
            } else {
                if (_e288 < arrayLength((&global_12.member))) {
                } else {
                    break;
                }
                let _e296 = global_12.member[_e288];
                if (_e296.member_4.w < arrayLength((&global_10.member))) {
                } else {
                    break;
                }
                global_10.member[_e296.member_4.w] = type_34(_e296.member, _e296.member_1, _e296.member_2, _e296.member_3, vec4<u32>(_e296.member_4.x, _e296.member_4.y, _e296.member_4.z, 0u), _e296.member_5);
                if ((_e296.member_4.y & 1u) == 0u) {
                } else {
                    let _e321 = global_1.member.member_7[2u];
                    let _e322 = (1u - _e321);
                    let _e323 = (_e322 * 4u);
                    let _e327 = atomicAdd((&global_11.member[(_e323 + 3u)]), 1u);
                    let _e331 = global_1.member.member[0u];
                    let _e335 = global_1.member.member[1u];
                    let _e340 = global_1.member.member_7[3u];
                    atomicStore((&global_11.member[((12u + (_e322 * ((_e331 * _e335) * _e340))) + _e327)]), _e296.member_4.w);
                    if ((_e327 % 64u) == 0u) {
                        let _e349 = (_e327 / 64u);
                        if (_e349 < 65535u) {
                            let _e353 = atomicAdd((&global_11.member[_e323]), 1u);
                        }
                        if ((_e349 % 65535u) == 0u) {
                            let _e359 = atomicAdd((&global_11.member[(_e323 + 1u)]), 1u);
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

fn function_4() {
    var local_103: array<u32, 64>;
    var local_104: array<f32, 64>;
    var phi_21058_: type_48;
    var phi_21111_: u32;
    var phi_21114_: u32;
    var phi_21116_: u32;
    var phi_21118_: u32;
    var phi_21120_: u32;
    var phi_21122_: vec4<f32>;
    var phi_21124_: f32;
    var phi_21494_: u32;
    var phi_21496_: u32;
    var phi_21497_: u32;
    var phi_21499_: bool;
    var phi_21500_: bool;
    var phi_21501_: u32;
    var phi_21502_: u32;
    var phi_21503_: u32;
    var phi_21504_: bool;
    var phi_21505_: bool;
    var phi_21506_: bool;
    var phi_32964_: bool;
    var phi_32994_: bool;
    var phi_32941_: type_56;
    var phi_32944_: type_56;
    var phi_32945_: bool;
    var phi_32950_: type_56;
    var phi_33051_: bool;
    var phi_33066_: bool;
    var phi_33081_: bool;
    var phi_33096_: bool;
    var phi_33028_: type_56;
    var phi_33031_: type_56;
    var phi_33032_: bool;
    var phi_33037_: type_56;
    var phi_33138_: bool;
    var phi_33153_: bool;
    var phi_33168_: bool;
    var phi_33183_: bool;
    var phi_33115_: type_56;
    var phi_33118_: type_56;
    var phi_33119_: bool;
    var phi_33124_: type_56;
    var phi_32919_: bool;
    var phi_32922_: bool;
    var phi_21535_: f32;
    var phi_33285_: bool;
    var phi_33315_: bool;
    var phi_33262_: type_56;
    var phi_33265_: type_56;
    var phi_33266_: bool;
    var phi_33271_: type_56;
    var phi_33372_: bool;
    var phi_33387_: bool;
    var phi_33402_: bool;
    var phi_33417_: bool;
    var phi_33349_: type_56;
    var phi_33352_: type_56;
    var phi_33353_: bool;
    var phi_33358_: type_56;
    var phi_33459_: bool;
    var phi_33474_: bool;
    var phi_33489_: bool;
    var phi_33504_: bool;
    var phi_33436_: type_56;
    var phi_33439_: type_56;
    var phi_33440_: bool;
    var phi_33445_: type_56;
    var phi_33240_: bool;
    var phi_33243_: bool;
    var phi_21544_: f32;
    var phi_21555_: u32;
    var phi_21566_: u32;
    var phi_21605_: u32;
    var phi_21606_: u32;
    var phi_21617_: u32;
    var phi_21618_: u32;
    var phi_21619_: u32;
    var phi_21620_: u32;
    var phi_21622_: u32;
    var phi_21623_: u32;
    var phi_21625_: u32;
    var phi_21626_: u32;
    var phi_21627_: bool;
    var phi_21628_: bool;
    var phi_21420_: vec4<f32>;
    var phi_21421_: vec4<f32>;
    var phi_21422_: bool;
    var phi_21426_: vec4<f32>;
    var phi_21427_: bool;
    var phi_21337_: bool;
    var phi_21371_: bool;
    var phi_21377_: f32;
    var phi_21387_: vec4<f32>;
    var phi_21388_: vec4<f32>;
    var phi_21389_: bool;
    var phi_21393_: vec4<f32>;
    var phi_21394_: bool;
    var phi_21398_: vec4<f32>;
    var phi_21399_: bool;
    var phi_21428_: vec4<f32>;
    var phi_21429_: bool;
    var phi_21270_: vec4<f32>;
    var phi_21271_: vec4<f32>;
    var phi_21272_: bool;
    var phi_21276_: vec4<f32>;
    var phi_21277_: bool;
    var phi_21278_: vec4<f32>;
    var phi_21279_: bool;
    var phi_21280_: bool;
    var phi_21288_: vec4<f32>;
    var phi_21289_: bool;
    var phi_21290_: vec4<f32>;
    var phi_21291_: bool;
    var phi_21292_: bool;
    var phi_21300_: vec4<f32>;
    var phi_21301_: bool;
    var phi_21438_: vec4<f32>;
    var phi_21439_: bool;
    var phi_21460_: f32;
    var phi_21112_: u32;
    var phi_21115_: u32;
    var phi_21117_: u32;
    var phi_21119_: u32;
    var phi_21121_: u32;
    var phi_21123_: vec4<f32>;
    var phi_21125_: f32;
    var phi_21639_: vec4<f32>;
    var phi_21640_: bool;
    var phi_21641_: bool;
    var phi_38599_: bool;
    var phi_21875_: bool;
    var phi_21874_: vec4<f32>;
    var local_105: u32;
    var local_106: vec4<f32>;
    var phi_21646_: type_74;
    var local_107: u32;
    var phi_21652_: type_74;

    switch bitcast<i32>(0u) {
        default: {
            let _e282 = global;
            let _e291 = global_1.member.member_7[2u];
            let _e295 = (_e282.x + (_e282.y * 4194240u));
            let _e300 = atomicLoad((&global_8.member[((_e291 * 4u) + 3u)]));
            if (_e295 >= _e300) {
                phi_21058_ = type_48(0u, type_48().member_1);
            } else {
                let _e305 = global_1.member.member[0u];
                let _e309 = global_1.member.member[1u];
                let _e314 = global_1.member.member_7[3u];
                let _e321 = atomicLoad((&global_8.member[((12u + (_e291 * ((_e305 * _e309) * _e314))) + _e295)]));
                phi_21058_ = type_48(1u, _e321);
            }
            let _e327 = phi_21058_;
            if (bitcast<i32>(_e327.member) != 0i) {
                let _e332 = (_e327.member_1 < arrayLength((&global_7.member)));
                if _e332 {
                } else {
                    break;
                }
                let _e338 = global_7.member[_e327.member_1].member;
                let _e345 = global_7.member[_e327.member_1].member_1;
                let _e351 = global_1.member.member_1[0u];
                let _e354 = global_1.member.member_1[1u];
                let _e357 = (vec3<f32>(1f, 1f, 1f) / vec3<f32>(_e345.x, _e345.y, _e345.z));
                local_103 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                local_104 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                phi_21111_ = 4294967295u;
                phi_21114_ = select(4294967295u, 0u, (_e351 > 0u));
                phi_21116_ = 0u;
                phi_21118_ = (_e351 + _e354);
                phi_21120_ = _e351;
                phi_21122_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_21124_ = 300000000000000000000000000000000000000f;
                loop {
                    let _e361 = phi_21111_;
                    let _e363 = phi_21114_;
                    let _e365 = phi_21116_;
                    let _e367 = phi_21118_;
                    let _e369 = phi_21120_;
                    let _e371 = phi_21122_;
                    let _e373 = phi_21124_;
                    local_105 = _e361;
                    local_106 = _e371;
                    local_107 = _e369;
                    if (_e369 < _e367) {
                        let _e736 = (3u * _e369);
                        let _e738 = global_2.member[_e736];
                        let _e741 = global_2.member[(_e736 + 1u)];
                        let _e747 = select(select(u32(_e741.w), 0u, (_e741.w < 0f)), 4294967295u, (_e741.w > 4294967000f));
                        if (_e747 == 0u) {
                            let _e841 = global_2.member[(_e736 + 2u)];
                            let _e850 = ((_e345.y * _e841.z) - (_e841.y * _e345.z));
                            let _e853 = ((_e345.z * _e841.x) - (_e841.z * _e345.x));
                            let _e856 = ((_e345.x * _e841.y) - (_e841.x * _e345.y));
                            let _e861 = (((_e741.x * _e850) + (_e741.y * _e853)) + (_e741.z * _e856));
                            if (abs(_e861) < 0.000000000001f) {
                                phi_21300_ = vec4<f32>();
                                phi_21301_ = true;
                            } else {
                                let _e864 = (1f / _e861);
                                let _e868 = (_e338.x - _e738.x);
                                let _e869 = (_e338.y - _e738.y);
                                let _e870 = (_e338.z - _e738.z);
                                let _e876 = ((((_e868 * _e850) + (_e869 * _e853)) + (_e870 * _e856)) * _e864);
                                if (_e876 < 0f) {
                                    phi_21290_ = vec4<f32>();
                                    phi_21291_ = true;
                                    phi_21292_ = false;
                                } else {
                                    let _e878 = (_e876 > 1f);
                                    if _e878 {
                                        phi_21288_ = vec4<f32>();
                                        phi_21289_ = false;
                                    } else {
                                        let _e881 = ((_e869 * _e741.z) - (_e741.y * _e870));
                                        let _e884 = ((_e870 * _e741.x) - (_e741.z * _e868));
                                        let _e887 = ((_e868 * _e741.y) - (_e741.x * _e869));
                                        let _e893 = ((((_e345.x * _e881) + (_e345.y * _e884)) + (_e345.z * _e887)) * _e864);
                                        if (_e893 < 0f) {
                                            phi_21278_ = vec4<f32>();
                                            phi_21279_ = true;
                                            phi_21280_ = false;
                                        } else {
                                            let _e896 = ((_e876 + _e893) > 1f);
                                            if _e896 {
                                                phi_21276_ = vec4<f32>();
                                                phi_21277_ = false;
                                            } else {
                                                let _e902 = ((((_e841.x * _e881) + (_e841.y * _e884)) + (_e841.z * _e887)) * _e864);
                                                if (_e902 <= 0f) {
                                                    phi_21271_ = vec4<f32>();
                                                    phi_21272_ = true;
                                                } else {
                                                    let _e904 = (_e902 >= _e373);
                                                    if _e904 {
                                                        phi_21270_ = vec4<f32>();
                                                    } else {
                                                        phi_21270_ = vec4<f32>(_e902, _e876, _e893, 1f);
                                                    }
                                                    let _e907 = phi_21270_;
                                                    phi_21271_ = _e907;
                                                    phi_21272_ = _e904;
                                                }
                                                let _e909 = phi_21271_;
                                                let _e911 = phi_21272_;
                                                phi_21276_ = _e909;
                                                phi_21277_ = _e911;
                                            }
                                            let _e913 = phi_21276_;
                                            let _e915 = phi_21277_;
                                            phi_21278_ = _e913;
                                            phi_21279_ = _e896;
                                            phi_21280_ = _e915;
                                        }
                                        let _e917 = phi_21278_;
                                        let _e919 = phi_21279_;
                                        let _e921 = phi_21280_;
                                        phi_21288_ = _e917;
                                        phi_21289_ = select(_e921, true, _e919);
                                    }
                                    let _e924 = phi_21288_;
                                    let _e926 = phi_21289_;
                                    phi_21290_ = _e924;
                                    phi_21291_ = _e878;
                                    phi_21292_ = _e926;
                                }
                                let _e928 = phi_21290_;
                                let _e930 = phi_21291_;
                                let _e932 = phi_21292_;
                                phi_21300_ = _e928;
                                phi_21301_ = select(_e932, true, _e930);
                            }
                            let _e935 = phi_21300_;
                            let _e937 = phi_21301_;
                            phi_21438_ = _e935;
                            phi_21439_ = _e937;
                        } else {
                            if (_e747 == 1u) {
                                let _e771 = (_e338.x - _e738.x);
                                let _e772 = (_e338.y - _e738.y);
                                let _e773 = (_e338.z - _e738.z);
                                let _e778 = (((_e771 * _e345.x) + (_e772 * _e345.y)) + (_e773 * _e345.z));
                                if (_e778 > 0f) {
                                    phi_21337_ = select(true, false, ((((_e771 * _e771) + (_e772 * _e772)) + (_e773 * _e773)) > (_e738.w * _e738.w)));
                                } else {
                                    phi_21337_ = true;
                                }
                                let _e790 = phi_21337_;
                                if _e790 {
                                    let _e794 = (_e771 - (_e345.x * _e778));
                                    let _e795 = (_e772 - (_e345.y * _e778));
                                    let _e796 = (_e773 - (_e345.z * _e778));
                                    let _e804 = ((_e738.w * _e738.w) - (((_e794 * _e794) + (_e795 * _e795)) + (_e796 * _e796)));
                                    if (_e804 < 0f) {
                                        phi_21393_ = vec4<f32>();
                                        phi_21394_ = true;
                                    } else {
                                        let _e806 = sqrt(_e804);
                                        let _e807 = -(_e778);
                                        let _e808 = (_e807 - _e806);
                                        if (_e808 <= 0f) {
                                            phi_21371_ = true;
                                        } else {
                                            phi_21371_ = (_e808 >= _e373);
                                        }
                                        let _e812 = phi_21371_;
                                        if _e812 {
                                            phi_21377_ = (_e807 + _e806);
                                        } else {
                                            phi_21377_ = _e808;
                                        }
                                        let _e815 = phi_21377_;
                                        if (_e815 <= 0f) {
                                            phi_21388_ = vec4<f32>();
                                            phi_21389_ = true;
                                        } else {
                                            let _e817 = (_e815 >= _e373);
                                            if _e817 {
                                                phi_21387_ = vec4<f32>();
                                            } else {
                                                phi_21387_ = vec4<f32>(_e815, 0f, 0f, 1f);
                                            }
                                            let _e820 = phi_21387_;
                                            phi_21388_ = _e820;
                                            phi_21389_ = _e817;
                                        }
                                        let _e822 = phi_21388_;
                                        let _e824 = phi_21389_;
                                        phi_21393_ = _e822;
                                        phi_21394_ = _e824;
                                    }
                                    let _e826 = phi_21393_;
                                    let _e828 = phi_21394_;
                                    phi_21398_ = _e826;
                                    phi_21399_ = _e828;
                                } else {
                                    phi_21398_ = vec4<f32>();
                                    phi_21399_ = true;
                                }
                                let _e830 = phi_21398_;
                                let _e832 = phi_21399_;
                                phi_21428_ = _e830;
                                phi_21429_ = _e832;
                            } else {
                                if (abs(_e345.y) < 0.000000000001f) {
                                    phi_21426_ = vec4<f32>();
                                    phi_21427_ = true;
                                } else {
                                    let _e754 = ((_e738.x - _e338.y) / _e345.y);
                                    if (_e754 <= 0f) {
                                        phi_21421_ = vec4<f32>();
                                        phi_21422_ = true;
                                    } else {
                                        let _e756 = (_e754 >= _e373);
                                        if _e756 {
                                            phi_21420_ = vec4<f32>();
                                        } else {
                                            phi_21420_ = vec4<f32>(_e754, 0f, 0f, 1f);
                                        }
                                        let _e759 = phi_21420_;
                                        phi_21421_ = _e759;
                                        phi_21422_ = _e756;
                                    }
                                    let _e761 = phi_21421_;
                                    let _e763 = phi_21422_;
                                    phi_21426_ = _e761;
                                    phi_21427_ = _e763;
                                }
                                let _e765 = phi_21426_;
                                let _e767 = phi_21427_;
                                phi_21428_ = _e765;
                                phi_21429_ = _e767;
                            }
                            let _e834 = phi_21428_;
                            let _e836 = phi_21429_;
                            phi_21438_ = _e834;
                            phi_21439_ = select(true, false, select(true, false, _e836));
                        }
                        let _e939 = phi_21438_;
                        let _e941 = phi_21439_;
                        let _e944 = select(vec4<f32>(0f, 0f, 0f, 0f), _e939, vec4(select(true, false, _e941)));
                        let _e946 = (_e944.w > 0f);
                        if _e946 {
                            phi_21460_ = _e944.x;
                        } else {
                            phi_21460_ = _e373;
                        }
                        let _e949 = phi_21460_;
                        phi_21112_ = select(_e361, _e369, _e946);
                        phi_21115_ = _e363;
                        phi_21117_ = _e365;
                        phi_21119_ = _e367;
                        phi_21121_ = (_e369 + 1u);
                        phi_21123_ = select(_e371, _e944, vec4(_e946));
                        phi_21125_ = _e949;
                        phi_21639_ = _e944;
                        phi_21640_ = true;
                        phi_21641_ = false;
                    } else {
                        if (_e363 == 4294967295u) {
                            let _e376 = (_e365 == 0u);
                            if _e376 {
                                phi_21496_ = u32();
                                phi_21497_ = u32();
                                phi_21499_ = false;
                                phi_21500_ = false;
                            } else {
                                let _e377 = (_e365 - 1u);
                                let _e378 = (_e377 < 64u);
                                if _e378 {
                                } else {
                                    phi_38599_ = true;
                                    phi_21875_ = bool();
                                    phi_21874_ = vec4<f32>();
                                    break;
                                }
                                let _e380 = local_104[_e377];
                                let _e381 = (_e380 >= _e373);
                                if _e381 {
                                    phi_21494_ = u32();
                                } else {
                                    if _e378 {
                                    } else {
                                        phi_38599_ = true;
                                        phi_21875_ = bool();
                                        phi_21874_ = vec4<f32>();
                                        break;
                                    }
                                    let _e383 = local_103[_e377];
                                    phi_21494_ = _e383;
                                }
                                let _e385 = phi_21494_;
                                phi_21496_ = _e377;
                                phi_21497_ = _e385;
                                phi_21499_ = select(true, false, _e381);
                                phi_21500_ = _e381;
                            }
                            let _e388 = phi_21496_;
                            let _e390 = phi_21497_;
                            let _e392 = phi_21499_;
                            let _e394 = phi_21500_;
                            phi_21501_ = _e388;
                            phi_21502_ = _e390;
                            phi_21503_ = _e388;
                            phi_21504_ = _e392;
                            phi_21505_ = _e394;
                            phi_21506_ = _e376;
                        } else {
                            phi_21501_ = u32();
                            phi_21502_ = _e363;
                            phi_21503_ = _e365;
                            phi_21504_ = true;
                            phi_21505_ = false;
                            phi_21506_ = false;
                        }
                        let _e396 = phi_21501_;
                        let _e398 = phi_21502_;
                        let _e400 = phi_21503_;
                        let _e402 = phi_21504_;
                        let _e404 = phi_21505_;
                        let _e406 = phi_21506_;
                        if _e402 {
                            let _e408 = ((_e398 & 2147483648u) == 0u);
                            if _e408 {
                                let _e416 = global_3.member[_e398];
                                if (_e416.member_1 == 4294967295u) {
                                    phi_21535_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e428 = ((_e416.member.x - _e338.x) * _e357.x);
                                    let _e430 = ((_e416.member.y - _e338.y) * _e357.y);
                                    let _e432 = ((_e416.member.z - _e338.z) * _e357.z);
                                    let _e439 = ((_e416.member_2.x - _e338.x) * _e357.x);
                                    let _e440 = ((_e416.member_2.y - _e338.y) * _e357.y);
                                    let _e441 = ((_e416.member_2.z - _e338.z) * _e357.z);
                                    if (_e428 == _e428) {
                                        let _e443 = (_e439 == _e439);
                                        if _e443 {
                                            let _e444 = (_e428 != _e428);
                                            if _e444 {
                                                phi_32964_ = true;
                                            } else {
                                                phi_32964_ = (_e439 <= _e428);
                                            }
                                            let _e447 = phi_32964_;
                                            let _e448 = select(_e428, _e439, _e447);
                                            if _e444 {
                                                phi_32994_ = true;
                                            } else {
                                                phi_32994_ = (_e439 >= _e428);
                                            }
                                            let _e453 = phi_32994_;
                                            let _e454 = select(_e428, _e439, _e453);
                                            phi_32941_ = type_56(select(-300000000000000000000000000000000000000f, _e448, (_e448 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e454, (_e454 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_32941_ = type_56();
                                        }
                                        let _e459 = phi_32941_;
                                        phi_32944_ = _e459;
                                        phi_32945_ = select(true, false, _e443);
                                    } else {
                                        phi_32944_ = type_56();
                                        phi_32945_ = true;
                                    }
                                    let _e462 = phi_32944_;
                                    let _e464 = phi_32945_;
                                    if _e464 {
                                        phi_32950_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_32950_ = _e462;
                                    }
                                    let _e466 = phi_32950_;
                                    if (_e430 == _e430) {
                                        let _e470 = (_e440 == _e440);
                                        if _e470 {
                                            let _e471 = (_e430 != _e430);
                                            if _e471 {
                                                phi_33051_ = true;
                                            } else {
                                                phi_33051_ = (_e440 <= _e430);
                                            }
                                            let _e474 = phi_33051_;
                                            let _e475 = select(_e430, _e440, _e474);
                                            if (_e466.member != _e466.member) {
                                                phi_33066_ = true;
                                            } else {
                                                phi_33066_ = (_e475 >= _e466.member);
                                            }
                                            let _e479 = phi_33066_;
                                            if _e471 {
                                                phi_33081_ = true;
                                            } else {
                                                phi_33081_ = (_e440 >= _e430);
                                            }
                                            let _e483 = phi_33081_;
                                            let _e484 = select(_e430, _e440, _e483);
                                            if (_e466.member_1 != _e466.member_1) {
                                                phi_33096_ = true;
                                            } else {
                                                phi_33096_ = (_e484 <= _e466.member_1);
                                            }
                                            let _e488 = phi_33096_;
                                            phi_33028_ = type_56(select(_e466.member, _e475, _e479), select(_e466.member_1, _e484, _e488));
                                        } else {
                                            phi_33028_ = type_56();
                                        }
                                        let _e492 = phi_33028_;
                                        phi_33031_ = _e492;
                                        phi_33032_ = select(true, false, _e470);
                                    } else {
                                        phi_33031_ = type_56();
                                        phi_33032_ = true;
                                    }
                                    let _e495 = phi_33031_;
                                    let _e497 = phi_33032_;
                                    if _e497 {
                                        phi_33037_ = _e466;
                                    } else {
                                        phi_33037_ = _e495;
                                    }
                                    let _e499 = phi_33037_;
                                    if (_e432 == _e432) {
                                        let _e503 = (_e441 == _e441);
                                        if _e503 {
                                            let _e504 = (_e432 != _e432);
                                            if _e504 {
                                                phi_33138_ = true;
                                            } else {
                                                phi_33138_ = (_e441 <= _e432);
                                            }
                                            let _e507 = phi_33138_;
                                            let _e508 = select(_e432, _e441, _e507);
                                            if (_e499.member != _e499.member) {
                                                phi_33153_ = true;
                                            } else {
                                                phi_33153_ = (_e508 >= _e499.member);
                                            }
                                            let _e512 = phi_33153_;
                                            if _e504 {
                                                phi_33168_ = true;
                                            } else {
                                                phi_33168_ = (_e441 >= _e432);
                                            }
                                            let _e516 = phi_33168_;
                                            let _e517 = select(_e432, _e441, _e516);
                                            if (_e499.member_1 != _e499.member_1) {
                                                phi_33183_ = true;
                                            } else {
                                                phi_33183_ = (_e517 <= _e499.member_1);
                                            }
                                            let _e521 = phi_33183_;
                                            phi_33115_ = type_56(select(_e499.member, _e508, _e512), select(_e499.member_1, _e517, _e521));
                                        } else {
                                            phi_33115_ = type_56();
                                        }
                                        let _e525 = phi_33115_;
                                        phi_33118_ = _e525;
                                        phi_33119_ = select(true, false, _e503);
                                    } else {
                                        phi_33118_ = type_56();
                                        phi_33119_ = true;
                                    }
                                    let _e528 = phi_33118_;
                                    let _e530 = phi_33119_;
                                    if _e530 {
                                        phi_33124_ = _e499;
                                    } else {
                                        phi_33124_ = _e528;
                                    }
                                    let _e532 = phi_33124_;
                                    if (_e532.member_1 < 0f) {
                                        phi_32922_ = true;
                                    } else {
                                        if (_e532.member > _e532.member_1) {
                                            phi_32919_ = true;
                                        } else {
                                            phi_32919_ = (_e532.member > _e373);
                                        }
                                        let _e539 = phi_32919_;
                                        phi_32922_ = _e539;
                                    }
                                    let _e541 = phi_32922_;
                                    phi_21535_ = select(_e532.member, 300000000000000000000000000000000000000f, _e541);
                                }
                                let _e544 = phi_21535_;
                                if (_e416.member_5 == 4294967295u) {
                                    phi_21544_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e556 = ((_e416.member_4.x - _e338.x) * _e357.x);
                                    let _e558 = ((_e416.member_4.y - _e338.y) * _e357.y);
                                    let _e560 = ((_e416.member_4.z - _e338.z) * _e357.z);
                                    let _e567 = ((_e416.member_6.x - _e338.x) * _e357.x);
                                    let _e568 = ((_e416.member_6.y - _e338.y) * _e357.y);
                                    let _e569 = ((_e416.member_6.z - _e338.z) * _e357.z);
                                    if (_e556 == _e556) {
                                        let _e571 = (_e567 == _e567);
                                        if _e571 {
                                            let _e572 = (_e556 != _e556);
                                            if _e572 {
                                                phi_33285_ = true;
                                            } else {
                                                phi_33285_ = (_e567 <= _e556);
                                            }
                                            let _e575 = phi_33285_;
                                            let _e576 = select(_e556, _e567, _e575);
                                            if _e572 {
                                                phi_33315_ = true;
                                            } else {
                                                phi_33315_ = (_e567 >= _e556);
                                            }
                                            let _e581 = phi_33315_;
                                            let _e582 = select(_e556, _e567, _e581);
                                            phi_33262_ = type_56(select(-300000000000000000000000000000000000000f, _e576, (_e576 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e582, (_e582 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_33262_ = type_56();
                                        }
                                        let _e587 = phi_33262_;
                                        phi_33265_ = _e587;
                                        phi_33266_ = select(true, false, _e571);
                                    } else {
                                        phi_33265_ = type_56();
                                        phi_33266_ = true;
                                    }
                                    let _e590 = phi_33265_;
                                    let _e592 = phi_33266_;
                                    if _e592 {
                                        phi_33271_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_33271_ = _e590;
                                    }
                                    let _e594 = phi_33271_;
                                    if (_e558 == _e558) {
                                        let _e598 = (_e568 == _e568);
                                        if _e598 {
                                            let _e599 = (_e558 != _e558);
                                            if _e599 {
                                                phi_33372_ = true;
                                            } else {
                                                phi_33372_ = (_e568 <= _e558);
                                            }
                                            let _e602 = phi_33372_;
                                            let _e603 = select(_e558, _e568, _e602);
                                            if (_e594.member != _e594.member) {
                                                phi_33387_ = true;
                                            } else {
                                                phi_33387_ = (_e603 >= _e594.member);
                                            }
                                            let _e607 = phi_33387_;
                                            if _e599 {
                                                phi_33402_ = true;
                                            } else {
                                                phi_33402_ = (_e568 >= _e558);
                                            }
                                            let _e611 = phi_33402_;
                                            let _e612 = select(_e558, _e568, _e611);
                                            if (_e594.member_1 != _e594.member_1) {
                                                phi_33417_ = true;
                                            } else {
                                                phi_33417_ = (_e612 <= _e594.member_1);
                                            }
                                            let _e616 = phi_33417_;
                                            phi_33349_ = type_56(select(_e594.member, _e603, _e607), select(_e594.member_1, _e612, _e616));
                                        } else {
                                            phi_33349_ = type_56();
                                        }
                                        let _e620 = phi_33349_;
                                        phi_33352_ = _e620;
                                        phi_33353_ = select(true, false, _e598);
                                    } else {
                                        phi_33352_ = type_56();
                                        phi_33353_ = true;
                                    }
                                    let _e623 = phi_33352_;
                                    let _e625 = phi_33353_;
                                    if _e625 {
                                        phi_33358_ = _e594;
                                    } else {
                                        phi_33358_ = _e623;
                                    }
                                    let _e627 = phi_33358_;
                                    if (_e560 == _e560) {
                                        let _e631 = (_e569 == _e569);
                                        if _e631 {
                                            let _e632 = (_e560 != _e560);
                                            if _e632 {
                                                phi_33459_ = true;
                                            } else {
                                                phi_33459_ = (_e569 <= _e560);
                                            }
                                            let _e635 = phi_33459_;
                                            let _e636 = select(_e560, _e569, _e635);
                                            if (_e627.member != _e627.member) {
                                                phi_33474_ = true;
                                            } else {
                                                phi_33474_ = (_e636 >= _e627.member);
                                            }
                                            let _e640 = phi_33474_;
                                            if _e632 {
                                                phi_33489_ = true;
                                            } else {
                                                phi_33489_ = (_e569 >= _e560);
                                            }
                                            let _e644 = phi_33489_;
                                            let _e645 = select(_e560, _e569, _e644);
                                            if (_e627.member_1 != _e627.member_1) {
                                                phi_33504_ = true;
                                            } else {
                                                phi_33504_ = (_e645 <= _e627.member_1);
                                            }
                                            let _e649 = phi_33504_;
                                            phi_33436_ = type_56(select(_e627.member, _e636, _e640), select(_e627.member_1, _e645, _e649));
                                        } else {
                                            phi_33436_ = type_56();
                                        }
                                        let _e653 = phi_33436_;
                                        phi_33439_ = _e653;
                                        phi_33440_ = select(true, false, _e631);
                                    } else {
                                        phi_33439_ = type_56();
                                        phi_33440_ = true;
                                    }
                                    let _e656 = phi_33439_;
                                    let _e658 = phi_33440_;
                                    if _e658 {
                                        phi_33445_ = _e627;
                                    } else {
                                        phi_33445_ = _e656;
                                    }
                                    let _e660 = phi_33445_;
                                    if (_e660.member_1 < 0f) {
                                        phi_33243_ = true;
                                    } else {
                                        if (_e660.member > _e660.member_1) {
                                            phi_33240_ = true;
                                        } else {
                                            phi_33240_ = (_e660.member > _e373);
                                        }
                                        let _e667 = phi_33240_;
                                        phi_33243_ = _e667;
                                    }
                                    let _e669 = phi_33243_;
                                    phi_21544_ = select(_e660.member, 300000000000000000000000000000000000000f, _e669);
                                }
                                let _e672 = phi_21544_;
                                if (_e416.member_3 > 0u) {
                                    phi_21555_ = ((2147483648u | ((_e416.member_3 - 1u) << bitcast<u32>(28u))) | _e416.member_1);
                                } else {
                                    phi_21555_ = _e416.member_1;
                                }
                                let _e681 = phi_21555_;
                                if (_e416.member_7 > 0u) {
                                    phi_21566_ = ((2147483648u | ((_e416.member_7 - 1u) << bitcast<u32>(28u))) | _e416.member_5);
                                } else {
                                    phi_21566_ = _e416.member_5;
                                }
                                let _e690 = phi_21566_;
                                let _e691 = (_e544 <= _e672);
                                let _e695 = select(_e544, _e672, _e691);
                                if (_e695 < 300000000000000000000000000000000000000f) {
                                    let _e697 = (_e400 < 64u);
                                    if _e697 {
                                        if _e697 {
                                        } else {
                                            phi_38599_ = true;
                                            phi_21875_ = bool();
                                            phi_21874_ = vec4<f32>();
                                            break;
                                        }
                                        local_103[_e400] = select(_e681, _e690, _e691);
                                        if _e697 {
                                        } else {
                                            phi_38599_ = true;
                                            phi_21875_ = bool();
                                            phi_21874_ = vec4<f32>();
                                            break;
                                        }
                                        local_104[_e400] = _e695;
                                        phi_21605_ = (_e400 + 1u);
                                    } else {
                                        phi_21605_ = _e400;
                                    }
                                    let _e702 = phi_21605_;
                                    phi_21606_ = _e702;
                                } else {
                                    phi_21606_ = _e400;
                                }
                                let _e704 = phi_21606_;
                                phi_21617_ = select(4294967295u, select(_e690, _e681, _e691), (select(_e672, _e544, _e691) < 300000000000000000000000000000000000000f));
                                phi_21618_ = _e704;
                                phi_21619_ = u32();
                                phi_21620_ = u32();
                            } else {
                                let _e409 = (_e398 & 268435455u);
                                phi_21617_ = u32();
                                phi_21618_ = u32();
                                phi_21619_ = ((_e409 + ((_e398 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_21620_ = _e409;
                            }
                            let _e708 = phi_21617_;
                            let _e710 = phi_21618_;
                            let _e712 = phi_21619_;
                            let _e714 = phi_21620_;
                            phi_21622_ = _e708;
                            phi_21623_ = _e710;
                            phi_21625_ = _e712;
                            phi_21626_ = _e714;
                            phi_21627_ = select(true, false, _e408);
                            phi_21628_ = _e408;
                        } else {
                            phi_21622_ = u32();
                            phi_21623_ = u32();
                            phi_21625_ = _e367;
                            phi_21626_ = _e369;
                            phi_21627_ = _e404;
                            phi_21628_ = false;
                        }
                        let _e717 = phi_21622_;
                        let _e719 = phi_21623_;
                        let _e721 = phi_21625_;
                        let _e723 = phi_21626_;
                        let _e725 = phi_21627_;
                        let _e727 = phi_21628_;
                        phi_21112_ = _e361;
                        phi_21115_ = select(_e717, 4294967295u, _e725);
                        phi_21117_ = select(_e719, select(_e396, _e400, _e402), _e725);
                        phi_21119_ = select(_e367, _e721, _e725);
                        phi_21121_ = select(_e369, _e723, _e725);
                        phi_21123_ = _e371;
                        phi_21125_ = _e373;
                        phi_21639_ = vec4<f32>();
                        phi_21640_ = select(_e727, true, _e725);
                        phi_21641_ = select(select(_e406, false, _e402), false, _e725);
                    }
                    let _e955 = phi_21112_;
                    let _e957 = phi_21115_;
                    let _e959 = phi_21117_;
                    let _e961 = phi_21119_;
                    let _e963 = phi_21121_;
                    let _e965 = phi_21123_;
                    let _e967 = phi_21125_;
                    let _e969 = phi_21639_;
                    let _e971 = phi_21640_;
                    let _e973 = phi_21641_;
                    phi_21111_ = _e955;
                    phi_21114_ = _e957;
                    phi_21116_ = _e959;
                    phi_21118_ = _e961;
                    phi_21120_ = _e963;
                    phi_21122_ = _e965;
                    phi_21124_ = _e967;
                    phi_38599_ = false;
                    phi_21875_ = _e973;
                    phi_21874_ = _e969;
                    if !(_e971) {
                        break;
                    }
                }
                let _e976 = phi_38599_;
                let _e978 = phi_21875_;
                let _e980 = phi_21874_;
                if _e976 {
                    break;
                }
                if _e978 {
                    let _e982 = local_105;
                    let _e984 = local_106;
                    phi_21646_ = type_74(_e982, _e984);
                } else {
                    phi_21646_ = type_74();
                }
                let _e987 = phi_21646_;
                if select(true, false, _e978) {
                    let _e990 = local_107;
                    phi_21652_ = type_74(_e990, _e980);
                } else {
                    phi_21652_ = _e987;
                }
                let _e993 = phi_21652_;
                let _e996 = global_7.member[_e327.member_1];
                let _e1008 = type_34(_e996.member, _e996.member_1, _e996.member_2, _e996.member_3, vec4<u32>(_e996.member_4.x, _e996.member_4.y, _e993.member, _e996.member_4.w), _e996.member_5);
                if _e332 {
                } else {
                    break;
                }
                global_7.member[_e327.member_1] = type_34(_e1008.member, _e1008.member_1, _e1008.member_2, _e1008.member_3, _e1008.member_4, _e993.member_1);
            }
            break;
        }
    }
    return;
}

fn function_5() {
    var phi_21686_: type_48;
    var phi_21700_: u32;
    var phi_21753_: bool;
    var phi_21754_: bool;
    var phi_21755_: bool;
    var phi_21771_: vec4<f32>;
    var phi_21701_: u32;
    var phi_38721_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e280 = global;
            let _e284 = arrayLength((&global_14.member));
            let _e288 = (_e280.x + (_e280.y * 4194240u));
            let _e292 = global_1.member.member[0u];
            let _e296 = global_1.member.member[1u];
            let _e297 = (_e292 * _e296);
            if (_e288 >= _e297) {
                phi_21686_ = type_48(0u, type_48().member_1);
            } else {
                phi_21686_ = type_48(1u, _e288);
            }
            let _e304 = phi_21686_;
            if (bitcast<i32>(_e304.member) != 0i) {
                phi_21700_ = 0u;
                loop {
                    let _e310 = phi_21700_;
                    let _e314 = global_1.member.member_7[3u];
                    let _e315 = (_e310 < _e314);
                    if _e315 {
                        let _e317 = ((_e310 * _e297) + _e304.member_1);
                        if (_e317 < arrayLength((&global_13.member))) {
                        } else {
                            phi_38721_ = true;
                            break;
                        }
                        let _e323 = global_13.member[_e317].member_4[1u];
                        if ((_e323 & 8u) == 0u) {
                            let _e329 = global_13.member[_e317].member_3;
                            let _e337 = global_13.member[_e317].member[3u];
                            let _e338 = (_e329.x * _e337);
                            let _e339 = (_e329.y * _e337);
                            let _e340 = (_e329.z * _e337);
                            if (_e338 == _e338) {
                                let _e343 = (_e339 == _e339);
                                if _e343 {
                                    phi_21753_ = select(true, false, (_e340 == _e340));
                                } else {
                                    phi_21753_ = false;
                                }
                                let _e347 = phi_21753_;
                                phi_21754_ = select(true, false, _e343);
                                phi_21755_ = _e347;
                            } else {
                                phi_21754_ = true;
                                phi_21755_ = false;
                            }
                            let _e350 = phi_21754_;
                            let _e352 = phi_21755_;
                            let _e355 = select(vec3<f32>(_e338, _e339, _e340), vec3<f32>(0f, 0f, 0f), vec3(select(_e352, true, _e350)));
                            let _e360 = global_13.member[_e317].member_3[3u];
                            phi_21771_ = vec4<f32>(_e355.x, _e355.y, _e355.z, _e360);
                        } else {
                            phi_21771_ = vec4<f32>(0f, 0f, 0f, 0f);
                        }
                        let _e366 = phi_21771_;
                        if (_e366.w > 0f) {
                            let _e372 = (2u * _e304.member_1);
                            if (_e372 < _e284) {
                            } else {
                                phi_38721_ = true;
                                break;
                            }
                            let _e377 = global_14.member[_e372][0u];
                            global_14.member[_e372][0u] = (_e377 + _e366.x);
                            let _e382 = global_14.member[_e372][1u];
                            global_14.member[_e372][1u] = (_e382 + _e366.y);
                            let _e387 = global_14.member[_e372][2u];
                            global_14.member[_e372][2u] = (_e387 + _e366.z);
                            let _e392 = global_14.member[_e372][3u];
                            global_14.member[_e372][3u] = (_e392 + _e366.w);
                            let _e394 = (_e372 + 1u);
                            if (_e394 < _e284) {
                            } else {
                                phi_38721_ = true;
                                break;
                            }
                            let _e402 = global_14.member[_e394][0u];
                            global_14.member[_e394][0u] = (_e402 + (_e366.x * _e366.x));
                            let _e407 = global_14.member[_e394][1u];
                            global_14.member[_e394][1u] = (_e407 + (_e366.y * _e366.y));
                            let _e412 = global_14.member[_e394][2u];
                            global_14.member[_e394][2u] = (_e412 + (_e366.z * _e366.z));
                            let _e417 = global_14.member[_e394][3u];
                            global_14.member[_e394][3u] = (_e417 + 1f);
                        }
                        phi_21701_ = (_e310 + 1u);
                    } else {
                        phi_21701_ = u32();
                    }
                    let _e421 = phi_21701_;
                    phi_21700_ = _e421;
                    phi_38721_ = false;
                    if !(_e315) {
                        break;
                    }
                }
                let _e424 = phi_38721_;
                if _e424 {
                    break;
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
fn gather_host(@builtin(global_invocation_id) param_1: vec3<u32>) {
    global = param_1;
    function_1();
}

@compute @workgroup_size(64, 1, 1) 
fn bounce_paths(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}

@compute @workgroup_size(64, 1, 1) 
fn scatter_host(@builtin(global_invocation_id) param_3: vec3<u32>) {
    global = param_3;
    function_3();
}

@compute @workgroup_size(64, 1, 1) 
fn intersect_paths(@builtin(global_invocation_id) param_4: vec3<u32>) {
    global = param_4;
    function_4();
}

@compute @workgroup_size(64, 1, 1) 
fn add_paths(@builtin(global_invocation_id) param_5: vec3<u32>) {
    global = param_5;
    function_5();
}
