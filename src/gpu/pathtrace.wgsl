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
    var phi_488_: type_48;
    var phi_33512_: bool;
    var phi_508_: vec3<f32>;
    var phi_511_: u32;
    var phi_21746_: f32;
    var phi_21758_: type_56;
    var phi_21769_: type_57;
    var phi_21770_: type_57;
    var phi_21774_: type_57;
    var phi_21775_: type_57;
    var phi_21776_: bool;
    var phi_21780_: type_57;
    var phi_21667_: type_57;
    var phi_21818_: bool;
    var phi_21898_: f32;
    var phi_21910_: type_56;
    var phi_21921_: type_57;
    var phi_21922_: type_57;
    var phi_21926_: type_57;
    var phi_21927_: type_57;
    var phi_21928_: bool;
    var phi_21932_: type_57;
    var phi_21866_: f32;
    var phi_21970_: bool;
    var phi_21695_: f32;
    var phi_21697_: f32;
    var phi_21706_: type_57;
    var phi_21710_: type_57;
    var phi_21714_: type_57;
    var phi_21985_: bool;
    var phi_9511_: vec3<f32>;
    var phi_9514_: vec3<f32>;
    var phi_9516_: vec3<f32>;
    var phi_9518_: vec3<f32>;
    var phi_9520_: u32;
    var phi_9604_: u32;
    var phi_9619_: type_56;
    var phi_9620_: type_56;
    var phi_9621_: type_56;
    var phi_22185_: bool;
    var phi_22312_: bool;
    var phi_22327_: bool;
    var phi_9949_: vec3<f32>;
    var phi_10022_: vec3<f32>;
    var phi_10023_: vec3<f32>;
    var phi_10024_: bool;
    var phi_10029_: vec3<f32>;
    var phi_10030_: bool;
    var phi_22451_: bool;
    var phi_22466_: bool;
    var phi_10321_: vec3<f32>;
    var phi_9512_: vec3<f32>;
    var phi_9515_: vec3<f32>;
    var phi_9517_: vec3<f32>;
    var phi_9519_: vec3<f32>;
    var phi_9521_: u32;
    var phi_33495_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_33979_: bool;
    var phi_10378_: type_60;
    var phi_33978_: bool;
    var phi_10380_: type_60;
    var phi_10381_: bool;
    var phi_10386_: type_60;
    var phi_887_: vec3<f32>;
    var phi_889_: vec3<f32>;
    var phi_890_: bool;
    var phi_33587_: bool;
    var phi_900_: vec3<f32>;
    var phi_903_: i32;
    var phi_905_: u32;
    var phi_22735_: bool;
    var phi_1327_: u32;
    var phi_1357_: u32;
    var phi_1328_: u32;
    var phi_1358_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1376_: u32;
    var phi_2681_: type_55;
    var phi_2689_: type_52;
    var phi_2691_: type_52;
    var phi_2692_: bool;
    var phi_2693_: type_52;
    var phi_2694_: bool;
    var phi_2695_: bool;
    var phi_22846_: f32;
    var phi_22858_: type_56;
    var phi_22869_: type_57;
    var phi_22870_: type_57;
    var phi_22874_: type_57;
    var phi_22875_: type_57;
    var phi_22876_: bool;
    var phi_22880_: type_57;
    var phi_22767_: type_57;
    var phi_22918_: bool;
    var phi_22998_: f32;
    var phi_23010_: type_56;
    var phi_23021_: type_57;
    var phi_23022_: type_57;
    var phi_23026_: type_57;
    var phi_23027_: type_57;
    var phi_23028_: bool;
    var phi_23032_: type_57;
    var phi_22966_: f32;
    var phi_23070_: bool;
    var phi_22795_: f32;
    var phi_22797_: f32;
    var phi_22806_: type_57;
    var phi_22810_: type_57;
    var phi_22814_: type_57;
    var phi_2713_: vec3<f32>;
    var phi_2716_: vec3<f32>;
    var phi_2718_: vec3<f32>;
    var phi_2720_: vec3<f32>;
    var phi_2722_: u32;
    var phi_2806_: u32;
    var phi_2821_: type_56;
    var phi_2822_: type_56;
    var phi_2823_: type_56;
    var phi_23269_: bool;
    var phi_23396_: bool;
    var phi_23411_: bool;
    var phi_3151_: vec3<f32>;
    var phi_23533_: f32;
    var phi_23545_: type_56;
    var phi_23556_: type_57;
    var phi_23557_: type_57;
    var phi_23561_: type_57;
    var phi_23562_: type_57;
    var phi_23563_: bool;
    var phi_23567_: type_57;
    var phi_23501_: f32;
    var phi_23643_: f32;
    var phi_23655_: type_56;
    var phi_23666_: type_57;
    var phi_23667_: type_57;
    var phi_23671_: type_57;
    var phi_23672_: type_57;
    var phi_23673_: bool;
    var phi_23677_: type_57;
    var phi_23611_: f32;
    var phi_23438_: vec3<f32>;
    var phi_23439_: u32;
    var phi_23809_: bool;
    var phi_23472_: vec3<f32>;
    var phi_23473_: u32;
    var local_11: vec3<f32>;
    var phi_23480_: type_58;
    var phi_23484_: type_58;
    var phi_3210_: vec3<f32>;
    var phi_3211_: vec3<f32>;
    var phi_2714_: vec3<f32>;
    var phi_2717_: vec3<f32>;
    var phi_2719_: vec3<f32>;
    var phi_2721_: vec3<f32>;
    var phi_2723_: u32;
    var phi_33555_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_33649_: bool;
    var phi_3231_: type_52;
    var phi_33648_: bool;
    var phi_3232_: type_52;
    var phi_33647_: bool;
    var phi_3255_: vec3<f32>;
    var phi_3256_: i32;
    var phi_3257_: u32;
    var phi_33646_: bool;
    var phi_901_: vec3<f32>;
    var phi_904_: i32;
    var phi_906_: u32;
    var phi_3258_: bool;
    var phi_33645_: bool;
    var phi_3592_: vec3<f32>;
    var phi_3595_: u32;
    var phi_23937_: f32;
    var phi_23952_: f32;
    var phi_23967_: f32;
    var phi_3666_: i32;
    var phi_3696_: i32;
    var phi_3704_: vec3<f32>;
    var phi_3707_: i32;
    var phi_3730_: vec3<f32>;
    var phi_3733_: i32;
    var phi_3731_: vec3<f32>;
    var phi_3734_: i32;
    var phi_3705_: vec3<f32>;
    var phi_3708_: i32;
    var phi_3932_: vec3<f32>;
    var phi_3933_: vec3<f32>;
    var phi_3934_: vec3<f32>;
    var phi_3593_: vec3<f32>;
    var phi_3596_: u32;
    var phi_33650_: bool;
    var local_15: vec3<f32>;
    var phi_33698_: bool;
    var phi_3945_: f32;
    var phi_3969_: f32;
    var phi_3971_: u32;
    var phi_3974_: f32;
    var phi_4077_: f32;
    var phi_4078_: f32;
    var phi_4094_: f32;
    var phi_3972_: u32;
    var phi_3975_: f32;
    var phi_4097_: u32;
    var phi_4100_: f32;
    var phi_4208_: f32;
    var phi_4098_: u32;
    var phi_4101_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_24083_: f32;
    var phi_24095_: type_56;
    var phi_24106_: type_57;
    var phi_24107_: type_57;
    var phi_24111_: type_57;
    var phi_24112_: type_57;
    var phi_24113_: bool;
    var phi_24117_: type_57;
    var phi_24051_: f32;
    var phi_4609_: bool;
    var phi_24243_: f32;
    var phi_24255_: type_56;
    var phi_24266_: type_57;
    var phi_24267_: type_57;
    var phi_24271_: type_57;
    var phi_24272_: type_57;
    var phi_24273_: bool;
    var phi_24277_: type_57;
    var phi_24164_: type_57;
    var phi_24315_: bool;
    var phi_24395_: f32;
    var phi_24407_: type_56;
    var phi_24418_: type_57;
    var phi_24419_: type_57;
    var phi_24423_: type_57;
    var phi_24424_: type_57;
    var phi_24425_: bool;
    var phi_24429_: type_57;
    var phi_24363_: f32;
    var phi_24467_: bool;
    var phi_24192_: f32;
    var phi_24194_: f32;
    var phi_24203_: type_57;
    var phi_24207_: type_57;
    var phi_24211_: type_57;
    var phi_4623_: vec3<f32>;
    var phi_4626_: u32;
    var phi_4710_: u32;
    var phi_4725_: type_56;
    var phi_4726_: type_56;
    var phi_4727_: type_56;
    var phi_24666_: bool;
    var phi_4624_: vec3<f32>;
    var phi_4627_: u32;
    var phi_33677_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_33744_: bool;
    var phi_4752_: vec3<f32>;
    var phi_33743_: bool;
    var phi_4754_: vec3<f32>;
    var phi_4756_: bool;
    var phi_33739_: bool;
    var phi_4767_: vec3<f32>;
    var phi_4768_: bool;
    var phi_33738_: bool;
    var phi_4770_: vec3<f32>;
    var phi_4771_: bool;
    var phi_4772_: bool;
    var phi_33737_: bool;
    var phi_4776_: vec3<f32>;
    var phi_4777_: vec3<f32>;
    var phi_4778_: bool;
    var phi_4779_: vec3<f32>;
    var phi_4780_: bool;
    var phi_5100_: vec3<f32>;
    var phi_5103_: u32;
    var phi_24737_: f32;
    var phi_24752_: f32;
    var phi_24767_: f32;
    var phi_5174_: i32;
    var phi_5204_: i32;
    var phi_5212_: vec3<f32>;
    var phi_5215_: i32;
    var phi_5238_: vec3<f32>;
    var phi_5241_: i32;
    var phi_5312_: vec3<f32>;
    var phi_5315_: u32;
    var phi_5435_: vec3<f32>;
    var phi_5313_: vec3<f32>;
    var phi_5316_: u32;
    var phi_5239_: vec3<f32>;
    var phi_5242_: i32;
    var phi_5213_: vec3<f32>;
    var phi_5216_: i32;
    var phi_5440_: vec3<f32>;
    var phi_5441_: vec3<f32>;
    var phi_5442_: vec3<f32>;
    var phi_5101_: vec3<f32>;
    var phi_5104_: u32;
    var phi_33732_: bool;
    var phi_33774_: bool;
    var phi_5462_: vec3<f32>;
    var phi_5675_: u32;
    var phi_5676_: bool;
    var phi_5679_: u32;
    var phi_5680_: bool;
    var phi_5681_: bool;
    var phi_5691_: f32;
    var phi_5694_: f32;
    var phi_5696_: u32;
    var phi_5698_: vec3<f32>;
    var phi_5700_: vec3<f32>;
    var phi_5702_: f32;
    var phi_5704_: f32;
    var phi_5742_: u32;
    var phi_5757_: vec3<f32>;
    var phi_5777_: vec3<f32>;
    var phi_24782_: vec3<f32>;
    var phi_5778_: u32;
    var phi_5779_: vec3<f32>;
    var phi_5780_: u32;
    var phi_5781_: vec3<f32>;
    var phi_5782_: u32;
    var phi_5784_: vec3<f32>;
    var phi_5919_: u32;
    var phi_5922_: f32;
    var phi_5924_: f32;
    var phi_5926_: f32;
    var phi_5928_: vec3<f32>;
    var phi_24837_: f32;
    var phi_24843_: f32;
    var phi_24871_: f32;
    var phi_24877_: f32;
    var phi_24905_: f32;
    var phi_24911_: f32;
    var phi_24939_: f32;
    var phi_24945_: f32;
    var phi_24973_: f32;
    var phi_24979_: f32;
    var phi_25007_: f32;
    var phi_25013_: f32;
    var phi_25041_: f32;
    var phi_25047_: f32;
    var phi_25075_: f32;
    var phi_25081_: f32;
    var phi_5920_: u32;
    var phi_5923_: f32;
    var phi_5925_: f32;
    var phi_5927_: f32;
    var phi_5929_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6345_: f32;
    var phi_6397_: vec3<f32>;
    var phi_6398_: f32;
    var phi_6399_: vec3<f32>;
    var phi_6400_: f32;
    var phi_6401_: vec3<f32>;
    var phi_6402_: f32;
    var phi_6404_: vec3<f32>;
    var phi_6405_: f32;
    var phi_6406_: f32;
    var phi_6407_: vec3<f32>;
    var phi_6408_: vec3<f32>;
    var phi_6409_: f32;
    var phi_6410_: f32;
    var phi_6411_: vec3<f32>;
    var phi_6412_: vec3<f32>;
    var phi_6413_: f32;
    var phi_6414_: f32;
    var phi_6415_: vec3<f32>;
    var phi_6416_: vec3<f32>;
    var phi_6417_: f32;
    var phi_6418_: f32;
    var phi_6419_: vec3<f32>;
    var phi_6420_: vec3<f32>;
    var phi_6422_: f32;
    var phi_6423_: f32;
    var phi_6424_: vec3<f32>;
    var phi_6425_: vec3<f32>;
    var phi_6426_: f32;
    var phi_6427_: f32;
    var phi_6428_: f32;
    var phi_6429_: f32;
    var phi_6430_: vec3<f32>;
    var phi_6431_: vec3<f32>;
    var phi_6432_: f32;
    var phi_6433_: f32;
    var phi_5692_: f32;
    var phi_5695_: f32;
    var phi_5697_: u32;
    var phi_5699_: vec3<f32>;
    var phi_5701_: vec3<f32>;
    var phi_5703_: f32;
    var phi_5705_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_25157_: bool;
    var local_29: f32;
    var phi_6465_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6544_: f32;
    var phi_6545_: vec3<f32>;
    var phi_6546_: f32;
    var phi_6548_: vec3<f32>;
    var phi_6551_: u32;
    var phi_6999_: f32;
    var phi_7179_: u32;
    var phi_7182_: f32;
    var phi_7184_: f32;
    var phi_7186_: f32;
    var phi_7188_: vec3<f32>;
    var phi_25246_: f32;
    var phi_25252_: f32;
    var phi_25280_: f32;
    var phi_25286_: f32;
    var phi_25314_: f32;
    var phi_25320_: f32;
    var phi_25348_: f32;
    var phi_25354_: f32;
    var phi_25382_: f32;
    var phi_25388_: f32;
    var phi_25416_: f32;
    var phi_25422_: f32;
    var phi_25450_: f32;
    var phi_25456_: f32;
    var phi_25484_: f32;
    var phi_25490_: f32;
    var phi_7180_: u32;
    var phi_7183_: f32;
    var phi_7185_: f32;
    var phi_7187_: f32;
    var phi_7189_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7602_: f32;
    var phi_25536_: bool;
    var phi_25551_: bool;
    var phi_7695_: vec3<f32>;
    var phi_7696_: vec3<f32>;
    var phi_7697_: vec3<f32>;
    var phi_7698_: vec3<f32>;
    var phi_6549_: vec3<f32>;
    var phi_6552_: u32;
    var phi_7717_: vec3<f32>;
    var phi_7720_: u32;
    var phi_7722_: f32;
    var phi_8023_: vec3<f32>;
    var phi_8024_: u32;
    var phi_8025_: f32;
    var phi_8026_: vec3<f32>;
    var phi_8043_: vec3<f32>;
    var phi_8045_: u32;
    var phi_8048_: vec3<f32>;
    var phi_8050_: f32;
    var phi_8052_: f32;
    var phi_8054_: f32;
    var phi_8074_: bool;
    var phi_8090_: bool;
    var phi_8092_: bool;
    var phi_8094_: bool;
    var phi_8095_: bool;
    var phi_8096_: bool;
    var phi_8097_: bool;
    var phi_8098_: bool;
    var phi_8142_: vec3<f32>;
    var phi_25570_: vec3<f32>;
    var phi_8143_: u32;
    var phi_8144_: vec3<f32>;
    var phi_8145_: u32;
    var phi_8146_: vec3<f32>;
    var phi_8281_: u32;
    var phi_8284_: f32;
    var phi_8286_: f32;
    var phi_8288_: f32;
    var phi_8290_: vec3<f32>;
    var phi_25625_: f32;
    var phi_25631_: f32;
    var phi_25659_: f32;
    var phi_25665_: f32;
    var phi_25693_: f32;
    var phi_25699_: f32;
    var phi_25727_: f32;
    var phi_25733_: f32;
    var phi_25761_: f32;
    var phi_25767_: f32;
    var phi_25795_: f32;
    var phi_25801_: f32;
    var phi_25829_: f32;
    var phi_25835_: f32;
    var phi_25863_: f32;
    var phi_25869_: f32;
    var phi_8282_: u32;
    var phi_8285_: f32;
    var phi_8287_: f32;
    var phi_8289_: f32;
    var phi_8291_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8704_: f32;
    var phi_8730_: vec3<f32>;
    var phi_8731_: f32;
    var phi_8732_: f32;
    var phi_8733_: vec3<f32>;
    var phi_8734_: f32;
    var phi_8735_: f32;
    var phi_8736_: vec3<f32>;
    var phi_8737_: f32;
    var phi_8738_: f32;
    var phi_8739_: vec3<f32>;
    var phi_8740_: f32;
    var phi_8741_: f32;
    var phi_8743_: vec3<f32>;
    var phi_8744_: f32;
    var phi_8745_: f32;
    var phi_8746_: f32;
    var phi_8046_: u32;
    var phi_8049_: vec3<f32>;
    var phi_8051_: f32;
    var phi_8053_: f32;
    var phi_8055_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9036_: type_59;
    var local_42: f32;
    var phi_9037_: type_59;
    var local_43: f32;
    var local_44: f32;
    var phi_9038_: type_59;
    var phi_9039_: type_59;
    var phi_7718_: vec3<f32>;
    var phi_7721_: u32;
    var phi_7723_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9068_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_33982_: bool;
    var phi_9115_: vec3<f32>;
    var phi_33981_: bool;
    var phi_9116_: vec3<f32>;
    var phi_33980_: bool;
    var phi_9117_: vec3<f32>;
    var phi_33976_: bool;
    var phi_10390_: vec3<f32>;
    var phi_33975_: bool;
    var phi_10391_: vec3<f32>;
    var phi_33974_: bool;
    var phi_509_: vec3<f32>;
    var phi_512_: u32;
    var phi_33973_: bool;
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
            let _e277 = global;
            let _e287 = global_1.member.member_7[2u];
            let _e291 = (_e277.x + (_e277.y * 4194240u));
            let _e296 = atomicLoad((&global_8.member[((_e287 * 4u) + 3u)]));
            if (_e291 >= _e296) {
                phi_488_ = type_48(0u, type_48().member_1);
            } else {
                let _e301 = global_1.member.member[0u];
                let _e306 = global_1.member.member[1u];
                let _e312 = atomicLoad((&global_8.member[((12u + ((_e287 * _e301) * _e306)) + _e291)]));
                phi_488_ = type_48(1u, _e312);
            }
            let _e318 = phi_488_;
            if (bitcast<i32>(_e318.member) != 0i) {
                phi_33512_ = false;
                phi_508_ = vec3<f32>(0f, 0f, 0f);
                phi_511_ = 0u;
                loop {
                    let _e324 = phi_33512_;
                    let _e326 = phi_508_;
                    let _e328 = phi_511_;
                    local_55 = _e326;
                    local_56 = _e326;
                    local_57 = _e326;
                    let _e329 = (_e328 < 2u);
                    if _e329 {
                        let _e331 = ((2u * _e318.member_1) + _e328);
                        if (_e331 < arrayLength((&global_9.member))) {
                        } else {
                            phi_33973_ = true;
                            break;
                        }
                        let _e335 = global_9.member[_e331];
                        if ((((_e335.member_2.x * _e335.member_2.x) + (_e335.member_2.y * _e335.member_2.y)) + (_e335.member_2.z * _e335.member_2.z)) == 0f) {
                            phi_33975_ = _e324;
                            phi_10391_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e335.member.w < 0f) {
                                let _e1290 = global_1.member.member_8[0u];
                                let _e1293 = global_1.member.member_8[3u];
                                if (_e1290 == 4294967295u) {
                                    let _e1295 = (_e1293 == 4294967295u);
                                    if _e1295 {
                                        let _e1298 = global_1.member.member_3[1u];
                                        let _e1301 = global_1.member.member_3[2u];
                                        let _e1304 = ((atan2(_e335.member_1.z, _e335.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1309 = (((_e1304 - floor(_e1304)) * f32(_e1298)) - 0.5f);
                                        let _e1311 = select(_e335.member_1.y, -1f, (_e335.member_1.y < -1f));
                                        let _e1318 = (((acos(select(_e1311, 1f, (_e1311 > 1f))) * 0.31830987f) * f32(_e1301)) - 0.5f);
                                        let _e1319 = (_e1301 - 1u);
                                        let _e1320 = f32(_e1319);
                                        if (0f <= _e1320) {
                                        } else {
                                            phi_33973_ = true;
                                            break;
                                        }
                                        let _e1323 = select(_e1318, 0f, (_e1318 < 0f));
                                        let _e1325 = select(_e1323, _e1320, (_e1323 > _e1320));
                                        let _e1326 = floor(_e1309);
                                        let _e1327 = floor(_e1325);
                                        let _e1328 = (_e1309 - _e1326);
                                        let _e1329 = (_e1325 - _e1327);
                                        let _e1340 = (_e1298 == 0u);
                                        if _e1340 {
                                            phi_33973_ = true;
                                            break;
                                        }
                                        let _e1341 = (bitcast<u32>((select(0i, select(select(i32(_e1326), i32(-2147483648), (_e1326 < -2147483600f)), 2147483647i, (_e1326 > 2147483500f)), (_e1326 == _e1326)) + bitcast<i32>(_e1298))) % _e1298);
                                        let _e1346 = select(select(u32(_e1327), 0u, (_e1327 < 0f)), 4294967295u, (_e1327 > 4294967000f));
                                        let _e1347 = (_e1346 + 1u);
                                        let _e1352 = global_1.member.member_2[2u];
                                        let _e1353 = (_e1346 * _e1298);
                                        if _e1340 {
                                            phi_33973_ = true;
                                            break;
                                        }
                                        let _e1354 = (_e1341 % _e1298);
                                        let _e1357 = (_e1352 + (3u * (_e1353 + _e1354)));
                                        let _e1359 = global_6.member[_e1357];
                                        let _e1362 = global_6.member[(_e1357 + 1u)];
                                        let _e1365 = global_6.member[(_e1357 + 2u)];
                                        if _e1340 {
                                            phi_33973_ = true;
                                            break;
                                        }
                                        let _e1368 = ((_e1341 + 1u) % _e1298);
                                        let _e1371 = (_e1352 + (3u * (_e1353 + _e1368)));
                                        let _e1373 = global_6.member[_e1371];
                                        let _e1376 = global_6.member[(_e1371 + 1u)];
                                        let _e1379 = global_6.member[(_e1371 + 2u)];
                                        let _e1387 = (vec3<f32>(_e1359, _e1362, _e1365) + vec3<f32>(((_e1373 - _e1359) * _e1328), ((_e1376 - _e1362) * _e1328), ((_e1379 - _e1365) * _e1328)));
                                        let _e1388 = (select(_e1347, _e1319, (_e1319 < _e1347)) * _e1298);
                                        if _e1340 {
                                            phi_33973_ = true;
                                            break;
                                        }
                                        let _e1391 = (_e1352 + (3u * (_e1388 + _e1354)));
                                        let _e1393 = global_6.member[_e1391];
                                        let _e1396 = global_6.member[(_e1391 + 1u)];
                                        let _e1399 = global_6.member[(_e1391 + 2u)];
                                        if _e1340 {
                                            phi_33973_ = true;
                                            break;
                                        }
                                        let _e1403 = (_e1352 + (3u * (_e1388 + _e1368)));
                                        let _e1405 = global_6.member[_e1403];
                                        let _e1408 = global_6.member[(_e1403 + 1u)];
                                        let _e1411 = global_6.member[(_e1403 + 2u)];
                                        let _e1419 = (vec3<f32>(_e1393, _e1396, _e1399) + vec3<f32>(((_e1405 - _e1393) * _e1328), ((_e1408 - _e1396) * _e1328), ((_e1411 - _e1399) * _e1328)));
                                        phi_887_ = (_e1387 + vec3<f32>(((_e1419.x - _e1387.x) * _e1329), ((_e1419.y - _e1387.y) * _e1329), ((_e1419.z - _e1387.z) * _e1329)));
                                    } else {
                                        phi_887_ = vec3<f32>();
                                    }
                                    let _e1435 = phi_887_;
                                    phi_889_ = _e1435;
                                    phi_890_ = select(true, false, _e1295);
                                } else {
                                    phi_889_ = vec3<f32>();
                                    phi_890_ = true;
                                }
                                let _e1438 = phi_889_;
                                let _e1440 = phi_890_;
                                if _e1440 {
                                    let _e1441 = (_e1293 != 4294967295u);
                                    if (_e1290 != 4294967295u) {
                                        phi_33587_ = _e324;
                                        phi_900_ = vec3<f32>(0f, 0f, 0f);
                                        phi_903_ = 0i;
                                        phi_905_ = _e1290;
                                        loop {
                                            let _e1444 = phi_33587_;
                                            let _e1446 = phi_900_;
                                            let _e1448 = phi_903_;
                                            let _e1450 = phi_905_;
                                            local_17 = _e1446;
                                            local_18 = _e1446;
                                            local_19 = _e1446;
                                            local_61 = _e1446;
                                            local_62 = _e1446;
                                            if (_e1450 != 4294967295u) {
                                                let _e1452 = (_e1448 < 2i);
                                                if _e1452 {
                                                    let _e1454 = global_6.member[_e1450];
                                                    let _e1457 = global_6.member[(_e1450 + 1u)];
                                                    let _e1460 = global_6.member[(_e1450 + 2u)];
                                                    let _e1463 = global_6.member[(_e1450 + 3u)];
                                                    let _e1466 = global_6.member[(_e1450 + 4u)];
                                                    let _e1469 = global_6.member[(_e1450 + 5u)];
                                                    let _e1472 = global_6.member[(_e1450 + 6u)];
                                                    let _e1475 = global_6.member[(_e1450 + 7u)];
                                                    let _e1478 = global_6.member[(_e1450 + 8u)];
                                                    let _e1481 = global_6.member[(_e1450 + 9u)];
                                                    let _e1484 = global_6.member[(_e1450 + 10u)];
                                                    let _e1487 = global_6.member[(_e1450 + 11u)];
                                                    let _e1490 = global_6.member[(_e1450 + 12u)];
                                                    let _e1493 = global_6.member[(_e1450 + 13u)];
                                                    let _e1496 = global_6.member[(_e1450 + 14u)];
                                                    let _e1499 = global_6.member[(_e1450 + 15u)];
                                                    let _e1503 = global_6.member[(_e1450 + 16u)];
                                                    let _e1506 = global_6.member[(_e1450 + 17u)];
                                                    let _e1509 = global_6.member[(_e1450 + 18u)];
                                                    let _e1512 = global_6.member[(_e1450 + 19u)];
                                                    let _e1515 = global_6.member[(_e1450 + 20u)];
                                                    let _e1518 = global_6.member[(_e1450 + 30u)];
                                                    let _e1519 = bitcast<u32>(_e1518);
                                                    let _e1522 = global_6.member[(_e1450 + 31u)];
                                                    let _e1523 = bitcast<u32>(_e1522);
                                                    let _e1526 = global_6.member[(_e1450 + 32u)];
                                                    let _e1528 = (_e335.member_1.y < 0f);
                                                    if _e1528 {
                                                        phi_2693_ = type_52();
                                                        phi_2694_ = true;
                                                        phi_2695_ = false;
                                                    } else {
                                                        let _e1529 = (_e1523 == 4294967295u);
                                                        if _e1529 {
                                                            phi_2691_ = type_52();
                                                            phi_2692_ = false;
                                                        } else {
                                                            let _e1543 = (_e335.member.x - _e1454);
                                                            let _e1544 = (_e335.member.y - _e1457);
                                                            let _e1545 = (_e335.member.z - _e1460);
                                                            let _e1553 = ((((((_e335.member.x * _e335.member.x) + (_e335.member.y * _e335.member.y)) + (_e335.member.z * _e335.member.z)) - (2f * (((_e335.member.x * _e1454) + (_e335.member.y * _e1457)) + (_e335.member.z * _e1460)))) + _e1469) / (sqrt((((_e1543 * _e1543) + (_e1544 * _e1544)) + (_e1545 * _e1545))) + _e1463));
                                                            let _e1554 = (_e1450 + 21u);
                                                            let _e1557 = global_6.member[(_e1450 + 28u)];
                                                            if (_e1553 > _e1557) {
                                                                phi_2681_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                let _e1560 = global_6.member[_e1554];
                                                                if (_e1553 != _e1553) {
                                                                    phi_22735_ = true;
                                                                } else {
                                                                    phi_22735_ = (_e1560 >= _e1553);
                                                                }
                                                                let _e1564 = phi_22735_;
                                                                let _e1565 = select(_e1553, _e1560, _e1564);
                                                                phi_1327_ = 0u;
                                                                loop {
                                                                    let _e1567 = phi_1327_;
                                                                    local_8 = _e1567;
                                                                    local_9 = _e1567;
                                                                    local_10 = _e1567;
                                                                    if (_e1567 < 8u) {
                                                                        let _e1571 = global_6.member[(_e1554 + _e1567)];
                                                                        let _e1572 = (_e1571 <= _e1565);
                                                                        if _e1572 {
                                                                            phi_1357_ = (_e1567 + 1u);
                                                                        } else {
                                                                            phi_1357_ = u32();
                                                                        }
                                                                        let _e1575 = phi_1357_;
                                                                        phi_1328_ = _e1575;
                                                                        phi_1358_ = _e1572;
                                                                    } else {
                                                                        phi_1328_ = u32();
                                                                        phi_1358_ = false;
                                                                    }
                                                                    let _e1577 = phi_1328_;
                                                                    let _e1579 = phi_1358_;
                                                                    phi_1327_ = _e1577;
                                                                    if !(_e1579) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1582 = local_8;
                                                                if (_e1582 < 1u) {
                                                                    phi_1376_ = 1u;
                                                                } else {
                                                                    let _e1585 = local_9;
                                                                    let _e1588 = local_10;
                                                                    phi_1376_ = select(_e1588, 7u, (_e1585 > 7u));
                                                                }
                                                                let _e1591 = phi_1376_;
                                                                let _e1592 = (_e1591 - 1u);
                                                                let _e1595 = global_6.member[(_e1554 + _e1592)];
                                                                let _e1598 = global_6.member[(_e1554 + _e1591)];
                                                                let _e1601 = ((_e1565 - _e1595) / (_e1598 - _e1595));
                                                                let _e1602 = select(_e335.member_1.y, 0f, _e1528);
                                                                let _e1608 = (sqrt((asin(select(_e1602, 1f, (_e1602 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1610 = (atan2(_e335.member_1.z, _e335.member_1.x) * 0.15915494f);
                                                                let _e1613 = ((_e1610 - floor(_e1610)) * 256f);
                                                                let _e1614 = floor(_e1613);
                                                                let _e1620 = (select(select(u32(_e1614), 0u, (_e1614 < 0f)), 4294967295u, (_e1614 > 4294967000f)) % 256u);
                                                                let _e1621 = floor(_e1608);
                                                                let _e1626 = select(select(u32(_e1621), 0u, (_e1621 < 0f)), 4294967295u, (_e1621 > 4294967000f));
                                                                let _e1628 = select(_e1626, 126u, (126u < _e1626));
                                                                let _e1630 = ((_e1620 + 1u) % 256u);
                                                                let _e1632 = (_e1613 - _e1614);
                                                                let _e1634 = (_e1608 - f32(_e1628));
                                                                let _e1636 = (_e1523 + (_e1592 * 294912u));
                                                                let _e1637 = (_e1628 * 256u);
                                                                let _e1639 = (9u * (_e1637 + _e1620));
                                                                let _e1640 = (_e1636 + _e1639);
                                                                let _e1642 = (9u * (_e1637 + _e1630));
                                                                let _e1643 = (_e1636 + _e1642);
                                                                let _e1644 = ((_e1628 + 1u) * 256u);
                                                                let _e1646 = (9u * (_e1644 + _e1620));
                                                                let _e1647 = (_e1636 + _e1646);
                                                                let _e1649 = (9u * (_e1644 + _e1630));
                                                                let _e1650 = (_e1636 + _e1649);
                                                                let _e1651 = (1f - _e1632);
                                                                let _e1652 = (1f - _e1634);
                                                                let _e1653 = (_e1651 * _e1652);
                                                                let _e1654 = (_e1632 * _e1652);
                                                                let _e1655 = (_e1651 * _e1634);
                                                                let _e1656 = (_e1632 * _e1634);
                                                                let _e1658 = global_6.member[_e1640];
                                                                let _e1661 = global_6.member[(_e1640 + 1u)];
                                                                let _e1664 = global_6.member[(_e1640 + 2u)];
                                                                let _e1669 = global_6.member[_e1643];
                                                                let _e1672 = global_6.member[(_e1643 + 1u)];
                                                                let _e1675 = global_6.member[(_e1643 + 2u)];
                                                                let _e1683 = global_6.member[_e1647];
                                                                let _e1686 = global_6.member[(_e1647 + 1u)];
                                                                let _e1689 = global_6.member[(_e1647 + 2u)];
                                                                let _e1697 = global_6.member[_e1650];
                                                                let _e1700 = global_6.member[(_e1650 + 1u)];
                                                                let _e1703 = global_6.member[(_e1650 + 2u)];
                                                                let _e1712 = global_6.member[(_e1640 + 3u)];
                                                                let _e1715 = global_6.member[(_e1640 + 4u)];
                                                                let _e1718 = global_6.member[(_e1640 + 5u)];
                                                                let _e1724 = global_6.member[(_e1643 + 3u)];
                                                                let _e1727 = global_6.member[(_e1643 + 4u)];
                                                                let _e1730 = global_6.member[(_e1643 + 5u)];
                                                                let _e1739 = global_6.member[(_e1647 + 3u)];
                                                                let _e1742 = global_6.member[(_e1647 + 4u)];
                                                                let _e1745 = global_6.member[(_e1647 + 5u)];
                                                                let _e1754 = global_6.member[(_e1650 + 3u)];
                                                                let _e1757 = global_6.member[(_e1650 + 4u)];
                                                                let _e1760 = global_6.member[(_e1650 + 5u)];
                                                                let _e1769 = global_6.member[(_e1640 + 6u)];
                                                                let _e1772 = global_6.member[(_e1640 + 7u)];
                                                                let _e1775 = global_6.member[(_e1640 + 8u)];
                                                                let _e1781 = global_6.member[(_e1643 + 6u)];
                                                                let _e1784 = global_6.member[(_e1643 + 7u)];
                                                                let _e1787 = global_6.member[(_e1643 + 8u)];
                                                                let _e1796 = global_6.member[(_e1647 + 6u)];
                                                                let _e1799 = global_6.member[(_e1647 + 7u)];
                                                                let _e1802 = global_6.member[(_e1647 + 8u)];
                                                                let _e1811 = global_6.member[(_e1650 + 6u)];
                                                                let _e1814 = global_6.member[(_e1650 + 7u)];
                                                                let _e1817 = global_6.member[(_e1650 + 8u)];
                                                                let _e1825 = (_e1523 + (_e1591 * 294912u));
                                                                let _e1826 = (_e1825 + _e1639);
                                                                let _e1827 = (_e1825 + _e1642);
                                                                let _e1828 = (_e1825 + _e1646);
                                                                let _e1829 = (_e1825 + _e1649);
                                                                let _e1831 = global_6.member[_e1826];
                                                                let _e1834 = global_6.member[(_e1826 + 1u)];
                                                                let _e1837 = global_6.member[(_e1826 + 2u)];
                                                                let _e1842 = global_6.member[_e1827];
                                                                let _e1845 = global_6.member[(_e1827 + 1u)];
                                                                let _e1848 = global_6.member[(_e1827 + 2u)];
                                                                let _e1856 = global_6.member[_e1828];
                                                                let _e1859 = global_6.member[(_e1828 + 1u)];
                                                                let _e1862 = global_6.member[(_e1828 + 2u)];
                                                                let _e1870 = global_6.member[_e1829];
                                                                let _e1873 = global_6.member[(_e1829 + 1u)];
                                                                let _e1876 = global_6.member[(_e1829 + 2u)];
                                                                let _e1885 = global_6.member[(_e1826 + 3u)];
                                                                let _e1888 = global_6.member[(_e1826 + 4u)];
                                                                let _e1891 = global_6.member[(_e1826 + 5u)];
                                                                let _e1897 = global_6.member[(_e1827 + 3u)];
                                                                let _e1900 = global_6.member[(_e1827 + 4u)];
                                                                let _e1903 = global_6.member[(_e1827 + 5u)];
                                                                let _e1912 = global_6.member[(_e1828 + 3u)];
                                                                let _e1915 = global_6.member[(_e1828 + 4u)];
                                                                let _e1918 = global_6.member[(_e1828 + 5u)];
                                                                let _e1927 = global_6.member[(_e1829 + 3u)];
                                                                let _e1930 = global_6.member[(_e1829 + 4u)];
                                                                let _e1933 = global_6.member[(_e1829 + 5u)];
                                                                let _e1942 = global_6.member[(_e1826 + 6u)];
                                                                let _e1945 = global_6.member[(_e1826 + 7u)];
                                                                let _e1948 = global_6.member[(_e1826 + 8u)];
                                                                let _e1954 = global_6.member[(_e1827 + 6u)];
                                                                let _e1957 = global_6.member[(_e1827 + 7u)];
                                                                let _e1960 = global_6.member[(_e1827 + 8u)];
                                                                let _e1969 = global_6.member[(_e1828 + 6u)];
                                                                let _e1972 = global_6.member[(_e1828 + 7u)];
                                                                let _e1975 = global_6.member[(_e1828 + 8u)];
                                                                let _e1984 = global_6.member[(_e1829 + 6u)];
                                                                let _e1987 = global_6.member[(_e1829 + 7u)];
                                                                let _e1990 = global_6.member[(_e1829 + 8u)];
                                                                let _e1997 = (1f - _e1601);
                                                                phi_2681_ = type_55(1u, type_52(vec3<f32>(((((((_e1658 * _e1653) + (_e1669 * _e1654)) + (_e1683 * _e1655)) + (_e1697 * _e1656)) * _e1997) + (((((_e1831 * _e1653) + (_e1842 * _e1654)) + (_e1856 * _e1655)) + (_e1870 * _e1656)) * _e1601)), ((((((_e1661 * _e1653) + (_e1672 * _e1654)) + (_e1686 * _e1655)) + (_e1700 * _e1656)) * _e1997) + (((((_e1834 * _e1653) + (_e1845 * _e1654)) + (_e1859 * _e1655)) + (_e1873 * _e1656)) * _e1601)), ((((((_e1664 * _e1653) + (_e1675 * _e1654)) + (_e1689 * _e1655)) + (_e1703 * _e1656)) * _e1997) + (((((_e1837 * _e1653) + (_e1848 * _e1654)) + (_e1862 * _e1655)) + (_e1876 * _e1656)) * _e1601))), vec3<f32>(((((((_e1712 * _e1653) + (_e1724 * _e1654)) + (_e1739 * _e1655)) + (_e1754 * _e1656)) * _e1997) + (((((_e1885 * _e1653) + (_e1897 * _e1654)) + (_e1912 * _e1655)) + (_e1927 * _e1656)) * _e1601)), ((((((_e1715 * _e1653) + (_e1727 * _e1654)) + (_e1742 * _e1655)) + (_e1757 * _e1656)) * _e1997) + (((((_e1888 * _e1653) + (_e1900 * _e1654)) + (_e1915 * _e1655)) + (_e1930 * _e1656)) * _e1601)), ((((((_e1718 * _e1653) + (_e1730 * _e1654)) + (_e1745 * _e1655)) + (_e1760 * _e1656)) * _e1997) + (((((_e1891 * _e1653) + (_e1903 * _e1654)) + (_e1918 * _e1655)) + (_e1933 * _e1656)) * _e1601))), vec3<f32>(((((((_e1769 * _e1653) + (_e1781 * _e1654)) + (_e1796 * _e1655)) + (_e1811 * _e1656)) * _e1997) + (((((_e1942 * _e1653) + (_e1954 * _e1654)) + (_e1969 * _e1655)) + (_e1984 * _e1656)) * _e1601)), ((((((_e1772 * _e1653) + (_e1784 * _e1654)) + (_e1799 * _e1655)) + (_e1814 * _e1656)) * _e1997) + (((((_e1945 * _e1653) + (_e1957 * _e1654)) + (_e1972 * _e1655)) + (_e1987 * _e1656)) * _e1601)), ((((((_e1775 * _e1653) + (_e1787 * _e1654)) + (_e1802 * _e1655)) + (_e1817 * _e1656)) * _e1997) + (((((_e1948 * _e1653) + (_e1960 * _e1654)) + (_e1975 * _e1655)) + (_e1990 * _e1656)) * _e1601)))));
                                                            }
                                                            let _e2034 = phi_2681_;
                                                            let _e2037 = (bitcast<i32>(_e2034.member) != 0i);
                                                            if _e2037 {
                                                                phi_2689_ = _e2034.member_1;
                                                            } else {
                                                                phi_2689_ = type_52();
                                                            }
                                                            let _e2040 = phi_2689_;
                                                            phi_2691_ = _e2040;
                                                            phi_2692_ = select(true, false, _e2037);
                                                        }
                                                        let _e2043 = phi_2691_;
                                                        let _e2045 = phi_2692_;
                                                        phi_2693_ = _e2043;
                                                        phi_2694_ = _e1529;
                                                        phi_2695_ = _e2045;
                                                    }
                                                    let _e2047 = phi_2693_;
                                                    let _e2049 = phi_2694_;
                                                    let _e2051 = phi_2695_;
                                                    if select(_e2051, true, _e2049) {
                                                        let _e2060 = ((((_e335.member.x - _e1454) * _e335.member_1.x) + ((_e335.member.y - _e1457) * _e335.member_1.y)) + ((_e335.member.z - _e1460) * _e335.member_1.z));
                                                        let _e2072 = ((((_e335.member.x * _e335.member.x) + (_e335.member.y * _e335.member.y)) + (_e335.member.z * _e335.member.z)) - (2f * (((_e335.member.x * _e1454) + (_e335.member.y * _e1457)) + (_e335.member.z * _e1460))));
                                                        let _e2073 = (_e2072 + _e1472);
                                                        let _e2074 = (_e2060 * _e2060);
                                                        let _e2075 = (_e2074 - _e2073);
                                                        if (_e2075 < 0f) {
                                                            phi_22874_ = type_57(0u, type_57().member_1);
                                                            phi_22875_ = type_57();
                                                            phi_22876_ = true;
                                                        } else {
                                                            if (_e2060 < 0f) {
                                                                phi_22846_ = -(sqrt(_e2075));
                                                            } else {
                                                                phi_22846_ = sqrt(_e2075);
                                                            }
                                                            let _e2082 = phi_22846_;
                                                            let _e2084 = -((_e2060 + _e2082));
                                                            let _e2085 = (_e2084 == 0f);
                                                            if _e2085 {
                                                                phi_22869_ = type_57(1u, type_56(0f, 0f));
                                                                phi_22870_ = type_57();
                                                            } else {
                                                                let _e2086 = (_e2073 / _e2084);
                                                                if (_e2084 < _e2086) {
                                                                    phi_22858_ = type_56(_e2084, _e2086);
                                                                } else {
                                                                    phi_22858_ = type_56(_e2086, _e2084);
                                                                }
                                                                let _e2091 = phi_22858_;
                                                                phi_22869_ = type_57();
                                                                phi_22870_ = type_57(1u, _e2091);
                                                            }
                                                            let _e2094 = phi_22869_;
                                                            let _e2096 = phi_22870_;
                                                            phi_22874_ = _e2094;
                                                            phi_22875_ = _e2096;
                                                            phi_22876_ = _e2085;
                                                        }
                                                        let _e2101 = phi_22874_;
                                                        let _e2103 = phi_22875_;
                                                        let _e2105 = phi_22876_;
                                                        if _e2105 {
                                                            phi_22880_ = _e2101;
                                                        } else {
                                                            phi_22880_ = _e2103;
                                                        }
                                                        let _e2107 = phi_22880_;
                                                        if (bitcast<i32>(_e2107.member) != 0i) {
                                                            phi_22767_ = type_57(0u, _e2107.member_1);
                                                        } else {
                                                            phi_22767_ = type_57(1u, type_57().member_1);
                                                        }
                                                        let _e2117 = phi_22767_;
                                                        if (bitcast<i32>(_e2117.member) != 0i) {
                                                            phi_22814_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e2117.member_1.member_1 <= 0f) {
                                                                phi_22810_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e2117.member_1.member != _e2117.member_1.member) {
                                                                    phi_22918_ = true;
                                                                } else {
                                                                    phi_22918_ = (0f >= _e2117.member_1.member);
                                                                }
                                                                let _e2129 = phi_22918_;
                                                                let _e2130 = select(_e2117.member_1.member, 0f, _e2129);
                                                                let _e2131 = (_e2072 + _e1469);
                                                                if (_e2131 > 0f) {
                                                                    let _e2133 = (_e2074 - _e2131);
                                                                    if (_e2133 < 0f) {
                                                                        phi_23026_ = type_57(0u, type_57().member_1);
                                                                        phi_23027_ = type_57();
                                                                        phi_23028_ = true;
                                                                    } else {
                                                                        if (_e2060 < 0f) {
                                                                            phi_22998_ = -(sqrt(_e2133));
                                                                        } else {
                                                                            phi_22998_ = sqrt(_e2133);
                                                                        }
                                                                        let _e2140 = phi_22998_;
                                                                        let _e2142 = -((_e2060 + _e2140));
                                                                        let _e2143 = (_e2142 == 0f);
                                                                        if _e2143 {
                                                                            phi_23021_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23022_ = type_57();
                                                                        } else {
                                                                            let _e2144 = (_e2131 / _e2142);
                                                                            if (_e2142 < _e2144) {
                                                                                phi_23010_ = type_56(_e2142, _e2144);
                                                                            } else {
                                                                                phi_23010_ = type_56(_e2144, _e2142);
                                                                            }
                                                                            let _e2149 = phi_23010_;
                                                                            phi_23021_ = type_57();
                                                                            phi_23022_ = type_57(1u, _e2149);
                                                                        }
                                                                        let _e2152 = phi_23021_;
                                                                        let _e2154 = phi_23022_;
                                                                        phi_23026_ = _e2152;
                                                                        phi_23027_ = _e2154;
                                                                        phi_23028_ = _e2143;
                                                                    }
                                                                    let _e2159 = phi_23026_;
                                                                    let _e2161 = phi_23027_;
                                                                    let _e2163 = phi_23028_;
                                                                    if _e2163 {
                                                                        phi_23032_ = _e2159;
                                                                    } else {
                                                                        phi_23032_ = _e2161;
                                                                    }
                                                                    let _e2165 = phi_23032_;
                                                                    if (bitcast<i32>(_e2165.member) != 0i) {
                                                                        phi_22966_ = select(_e2165.member_1.member_1, _e2165.member_1.member, (_e2165.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22966_ = -1f;
                                                                    }
                                                                    let _e2176 = phi_22966_;
                                                                    if (_e2176 >= 0f) {
                                                                        if (_e2117.member_1.member_1 != _e2117.member_1.member_1) {
                                                                            phi_23070_ = true;
                                                                        } else {
                                                                            phi_23070_ = (_e2176 <= _e2117.member_1.member_1);
                                                                        }
                                                                        let _e2181 = phi_23070_;
                                                                        phi_22795_ = select(_e2117.member_1.member_1, _e2176, _e2181);
                                                                    } else {
                                                                        phi_22795_ = _e2117.member_1.member_1;
                                                                    }
                                                                    let _e2184 = phi_22795_;
                                                                    phi_22797_ = _e2184;
                                                                } else {
                                                                    phi_22797_ = _e2117.member_1.member_1;
                                                                }
                                                                let _e2186 = phi_22797_;
                                                                if (_e2186 <= _e2130) {
                                                                    phi_22806_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    phi_22806_ = type_57(1u, type_56(_e2130, _e2186));
                                                                }
                                                                let _e2194 = phi_22806_;
                                                                phi_22810_ = _e2194;
                                                            }
                                                            let _e2199 = phi_22810_;
                                                            phi_22814_ = _e2199;
                                                        }
                                                        let _e2204 = phi_22814_;
                                                        if (bitcast<i32>(_e2204.member) != 0i) {
                                                            phi_2713_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2716_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2718_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2720_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2722_ = 0u;
                                                            loop {
                                                                let _e2213 = phi_2713_;
                                                                let _e2215 = phi_2716_;
                                                                let _e2217 = phi_2718_;
                                                                let _e2219 = phi_2720_;
                                                                let _e2221 = phi_2722_;
                                                                local_12 = _e2219;
                                                                local_13 = _e2217;
                                                                local_14 = _e2215;
                                                                let _e2222 = (_e2221 < 32u);
                                                                if _e2222 {
                                                                    let _e2223 = -(_e2060);
                                                                    if (_e2204.member_1.member <= _e2204.member_1.member_1) {
                                                                    } else {
                                                                        phi_33555_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2226 = select(_e2223, _e2204.member_1.member, (_e2223 < _e2204.member_1.member));
                                                                    let _e2228 = select(_e2226, _e2204.member_1.member_1, (_e2226 > _e2204.member_1.member_1));
                                                                    let _e2229 = (_e2228 - _e2204.member_1.member);
                                                                    let _e2230 = (_e2204.member_1.member_1 - _e2228);
                                                                    let _e2231 = (_e2204.member_1.member_1 - _e2204.member_1.member);
                                                                    let _e2232 = (0.000000001f * _e2231);
                                                                    if (_e2229 <= _e2232) {
                                                                        let _e2307 = (f32(_e2221) * 0.03125f);
                                                                        let _e2310 = (f32((_e2221 + 1u)) * 0.03125f);
                                                                        let _e2312 = ((_e2307 + _e2310) * 0.5f);
                                                                        phi_2823_ = type_56((_e2204.member_1.member + ((_e2231 * _e2312) * _e2312)), (_e2231 * ((_e2310 * _e2310) - (_e2307 * _e2307))));
                                                                    } else {
                                                                        if (_e2230 <= _e2232) {
                                                                            let _e2290 = (f32((31u - _e2221)) * 0.03125f);
                                                                            let _e2293 = (f32((32u - _e2221)) * 0.03125f);
                                                                            let _e2295 = ((_e2290 + _e2293) * 0.5f);
                                                                            phi_2822_ = type_56((_e2204.member_1.member_1 - ((_e2231 * _e2295) * _e2295)), (_e2231 * ((_e2293 * _e2293) - (_e2290 * _e2290))));
                                                                        } else {
                                                                            let _e2238 = round(((32f * _e2229) / (_e2229 + _e2230)));
                                                                            let _e2243 = select(select(u32(_e2238), 0u, (_e2238 < 0f)), 4294967295u, (_e2238 > 4294967000f));
                                                                            if (_e2243 < 1u) {
                                                                                phi_2806_ = 1u;
                                                                            } else {
                                                                                phi_2806_ = select(_e2243, 31u, (_e2243 > 31u));
                                                                            }
                                                                            let _e2248 = phi_2806_;
                                                                            if (_e2221 < _e2248) {
                                                                                let _e2269 = ((_e2248 - 1u) - _e2221);
                                                                                let _e2271 = f32(_e2248);
                                                                                let _e2272 = (f32(_e2269) / _e2271);
                                                                                let _e2275 = (f32((_e2269 + 1u)) / _e2271);
                                                                                let _e2277 = ((_e2272 + _e2275) * 0.5f);
                                                                                phi_2821_ = type_56((_e2228 - ((_e2229 * _e2277) * _e2277)), (_e2229 * ((_e2275 * _e2275) - (_e2272 * _e2272))));
                                                                            } else {
                                                                                let _e2251 = (_e2221 - _e2248);
                                                                                let _e2253 = f32((32u - _e2248));
                                                                                let _e2254 = (f32(_e2251) / _e2253);
                                                                                let _e2257 = (f32((_e2251 + 1u)) / _e2253);
                                                                                let _e2259 = ((_e2254 + _e2257) * 0.5f);
                                                                                phi_2821_ = type_56((_e2228 + ((_e2230 * _e2259) * _e2259)), (_e2230 * ((_e2257 * _e2257) - (_e2254 * _e2254))));
                                                                            }
                                                                            let _e2287 = phi_2821_;
                                                                            phi_2822_ = _e2287;
                                                                        }
                                                                        let _e2305 = phi_2822_;
                                                                        phi_2823_ = _e2305;
                                                                    }
                                                                    let _e2322 = phi_2823_;
                                                                    let _e2328 = (_e335.member.x + (_e335.member_1.x * _e2322.member));
                                                                    let _e2329 = (_e335.member.y + (_e335.member_1.y * _e2322.member));
                                                                    let _e2330 = (_e335.member.z + (_e335.member_1.z * _e2322.member));
                                                                    let _e2343 = ((((_e2328 * _e2328) + (_e2329 * _e2329)) + (_e2330 * _e2330)) - (2f * (((_e2328 * _e1454) + (_e2329 * _e1457)) + (_e2330 * _e1460))));
                                                                    let _e2344 = (_e2343 + _e1469);
                                                                    let _e2345 = (_e2328 - _e1454);
                                                                    let _e2346 = (_e2329 - _e1457);
                                                                    let _e2347 = (_e2330 - _e1460);
                                                                    let _e2353 = sqrt((((_e2345 * _e2345) + (_e2346 * _e2346)) + (_e2347 * _e2347)));
                                                                    let _e2355 = (_e2344 / (_e2353 + _e1463));
                                                                    let _e2359 = (1f - (abs((_e2355 - 25000f)) * 0.00006666667f));
                                                                    if (_e2359 != _e2359) {
                                                                        phi_23269_ = true;
                                                                    } else {
                                                                        phi_23269_ = (0f >= _e2359);
                                                                    }
                                                                    let _e2363 = phi_23269_;
                                                                    let _e2365 = -(_e2355);
                                                                    let _e2368 = (exp((_e2365 / _e1484)) * _e2322.member_1);
                                                                    let _e2371 = (exp((_e2365 / _e1487)) * _e2322.member_1);
                                                                    let _e2373 = (_e2368 * 0.5f);
                                                                    let _e2374 = (_e2371 * 0.5f);
                                                                    let _e2375 = ((select(_e2359, 0f, _e2363) * _e2322.member_1) * 0.5f);
                                                                    let _e2377 = (_e2213.x + _e2373);
                                                                    let _e2379 = (_e2213.y + _e2374);
                                                                    let _e2381 = (_e2213.z + _e2375);
                                                                    let _e2386 = (_e1503 * 1.1f);
                                                                    let _e2387 = (_e2386 * _e2379);
                                                                    let _e2406 = (_e1503 * _e2371);
                                                                    if (_e1519 == 4294967295u) {
                                                                        phi_3151_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2414 = (1f / _e2353);
                                                                        let _e2422 = ((((_e2345 * _e2414) * _e1475) + ((_e2346 * _e2414) * _e1478)) + ((_e2347 * _e2414) * _e1481));
                                                                        if (_e2355 != _e2355) {
                                                                            phi_23396_ = true;
                                                                        } else {
                                                                            phi_23396_ = (0f >= _e2355);
                                                                        }
                                                                        let _e2426 = phi_23396_;
                                                                        let _e2430 = sqrt((select(_e2355, 0f, _e2426) / (_e1466 - _e1463)));
                                                                        if (_e2430 != _e2430) {
                                                                            phi_23411_ = true;
                                                                        } else {
                                                                            phi_23411_ = (1f <= _e2430);
                                                                        }
                                                                        let _e2434 = phi_23411_;
                                                                        let _e2436 = (select(_e2430, 1f, _e2434) * 31f);
                                                                        let _e2438 = select(_e2422, -1f, (_e2422 < -1f));
                                                                        let _e2442 = ((select(_e2438, 1f, (_e2438 > 1f)) + 1f) * 15.5f);
                                                                        let _e2447 = select(select(u32(_e2436), 0u, (_e2436 < 0f)), 4294967295u, (_e2436 > 4294967000f));
                                                                        let _e2449 = select(_e2447, 30u, (30u < _e2447));
                                                                        let _e2454 = select(select(u32(_e2442), 0u, (_e2442 < 0f)), 4294967295u, (_e2442 > 4294967000f));
                                                                        let _e2456 = select(_e2454, 30u, (30u < _e2454));
                                                                        let _e2458 = (_e2436 - f32(_e2449));
                                                                        let _e2460 = (_e2442 - f32(_e2456));
                                                                        let _e2464 = (_e1519 + (3u * ((_e2449 * 32u) + _e2456)));
                                                                        let _e2469 = (_e1519 + (3u * (((_e2449 + 1u) * 32u) + _e2456)));
                                                                        let _e2471 = global_6.member[_e2464];
                                                                        let _e2474 = global_6.member[(_e2464 + 1u)];
                                                                        let _e2477 = global_6.member[(_e2464 + 2u)];
                                                                        let _e2478 = (1f - _e2460);
                                                                        let _e2484 = global_6.member[(_e2464 + 3u)];
                                                                        let _e2487 = global_6.member[(_e2464 + 4u)];
                                                                        let _e2490 = global_6.member[(_e2464 + 5u)];
                                                                        let _e2497 = (1f - _e2458);
                                                                        let _e2502 = global_6.member[_e2469];
                                                                        let _e2505 = global_6.member[(_e2469 + 1u)];
                                                                        let _e2508 = global_6.member[(_e2469 + 2u)];
                                                                        let _e2514 = global_6.member[(_e2469 + 3u)];
                                                                        let _e2517 = global_6.member[(_e2469 + 4u)];
                                                                        let _e2520 = global_6.member[(_e2469 + 5u)];
                                                                        phi_3151_ = vec3<f32>(((((_e2471 * _e2478) + (_e2484 * _e2460)) * _e2497) + (((_e2502 * _e2478) + (_e2514 * _e2460)) * _e2458)), ((((_e2474 * _e2478) + (_e2487 * _e2460)) * _e2497) + (((_e2505 * _e2478) + (_e2517 * _e2460)) * _e2458)), ((((_e2477 * _e2478) + (_e2490 * _e2460)) * _e2497) + (((_e2508 * _e2478) + (_e2520 * _e2460)) * _e2458)));
                                                                    }
                                                                    let _e2535 = phi_3151_;
                                                                    let _e2553 = (((_e2345 * _e1475) + (_e2346 * _e1478)) + (_e2347 * _e1481));
                                                                    let _e2554 = (_e2553 * _e2553);
                                                                    let _e2555 = (_e2554 - _e2344);
                                                                    if (_e2555 < 0f) {
                                                                        phi_23561_ = type_57(0u, type_57().member_1);
                                                                        phi_23562_ = type_57();
                                                                        phi_23563_ = true;
                                                                    } else {
                                                                        if (_e2553 < 0f) {
                                                                            phi_23533_ = -(sqrt(_e2555));
                                                                        } else {
                                                                            phi_23533_ = sqrt(_e2555);
                                                                        }
                                                                        let _e2562 = phi_23533_;
                                                                        let _e2564 = -((_e2553 + _e2562));
                                                                        let _e2565 = (_e2564 == 0f);
                                                                        if _e2565 {
                                                                            phi_23556_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23557_ = type_57();
                                                                        } else {
                                                                            let _e2566 = (_e2344 / _e2564);
                                                                            if (_e2564 < _e2566) {
                                                                                phi_23545_ = type_56(_e2564, _e2566);
                                                                            } else {
                                                                                phi_23545_ = type_56(_e2566, _e2564);
                                                                            }
                                                                            let _e2571 = phi_23545_;
                                                                            phi_23556_ = type_57();
                                                                            phi_23557_ = type_57(1u, _e2571);
                                                                        }
                                                                        let _e2574 = phi_23556_;
                                                                        let _e2576 = phi_23557_;
                                                                        phi_23561_ = _e2574;
                                                                        phi_23562_ = _e2576;
                                                                        phi_23563_ = _e2565;
                                                                    }
                                                                    let _e2581 = phi_23561_;
                                                                    let _e2583 = phi_23562_;
                                                                    let _e2585 = phi_23563_;
                                                                    if _e2585 {
                                                                        phi_23567_ = _e2581;
                                                                    } else {
                                                                        phi_23567_ = _e2583;
                                                                    }
                                                                    let _e2587 = phi_23567_;
                                                                    if (bitcast<i32>(_e2587.member) != 0i) {
                                                                        phi_23501_ = select(_e2587.member_1.member_1, _e2587.member_1.member, (_e2587.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23501_ = -1f;
                                                                    }
                                                                    let _e2598 = phi_23501_;
                                                                    if (_e2598 >= 0f) {
                                                                        phi_23484_ = type_58(0u, type_58().member_1);
                                                                    } else {
                                                                        let _e2600 = (_e2343 + _e1472);
                                                                        let _e2601 = (_e2554 - _e2600);
                                                                        if (_e2601 < 0f) {
                                                                            phi_23671_ = type_57(0u, type_57().member_1);
                                                                            phi_23672_ = type_57();
                                                                            phi_23673_ = true;
                                                                        } else {
                                                                            if (_e2553 < 0f) {
                                                                                phi_23643_ = -(sqrt(_e2601));
                                                                            } else {
                                                                                phi_23643_ = sqrt(_e2601);
                                                                            }
                                                                            let _e2608 = phi_23643_;
                                                                            let _e2610 = -((_e2553 + _e2608));
                                                                            let _e2611 = (_e2610 == 0f);
                                                                            if _e2611 {
                                                                                phi_23666_ = type_57(1u, type_56(0f, 0f));
                                                                                phi_23667_ = type_57();
                                                                            } else {
                                                                                let _e2612 = (_e2600 / _e2610);
                                                                                if (_e2610 < _e2612) {
                                                                                    phi_23655_ = type_56(_e2610, _e2612);
                                                                                } else {
                                                                                    phi_23655_ = type_56(_e2612, _e2610);
                                                                                }
                                                                                let _e2617 = phi_23655_;
                                                                                phi_23666_ = type_57();
                                                                                phi_23667_ = type_57(1u, _e2617);
                                                                            }
                                                                            let _e2620 = phi_23666_;
                                                                            let _e2622 = phi_23667_;
                                                                            phi_23671_ = _e2620;
                                                                            phi_23672_ = _e2622;
                                                                            phi_23673_ = _e2611;
                                                                        }
                                                                        let _e2627 = phi_23671_;
                                                                        let _e2629 = phi_23672_;
                                                                        let _e2631 = phi_23673_;
                                                                        if _e2631 {
                                                                            phi_23677_ = _e2627;
                                                                        } else {
                                                                            phi_23677_ = _e2629;
                                                                        }
                                                                        let _e2633 = phi_23677_;
                                                                        if (bitcast<i32>(_e2633.member) != 0i) {
                                                                            phi_23611_ = select(_e2633.member_1.member_1, _e2633.member_1.member, (_e2633.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_23611_ = -1f;
                                                                        }
                                                                        let _e2644 = phi_23611_;
                                                                        if (_e2644 < 0f) {
                                                                            phi_23480_ = type_58(0u, type_58().member_1);
                                                                        } else {
                                                                            phi_23438_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_23439_ = 0u;
                                                                            loop {
                                                                                let _e2647 = phi_23438_;
                                                                                let _e2649 = phi_23439_;
                                                                                local_11 = _e2647;
                                                                                let _e2650 = (_e2649 < 16u);
                                                                                if _e2650 {
                                                                                    let _e2652 = (f32(_e2649) * 0.0625f);
                                                                                    let _e2653 = (_e2649 + 1u);
                                                                                    let _e2655 = (f32(_e2653) * 0.0625f);
                                                                                    let _e2657 = ((_e2652 + _e2655) * 0.5f);
                                                                                    let _e2659 = ((_e2644 * _e2657) * _e2657);
                                                                                    let _e2663 = (_e2644 * ((_e2655 * _e2655) - (_e2652 * _e2652)));
                                                                                    let _e2668 = (vec3<f32>(_e2328, _e2329, _e2330) + vec3<f32>((_e1475 * _e2659), (_e1478 * _e2659), (_e1481 * _e2659)));
                                                                                    let _e2685 = (_e2668.x - _e1454);
                                                                                    let _e2686 = (_e2668.y - _e1457);
                                                                                    let _e2687 = (_e2668.z - _e1460);
                                                                                    let _e2695 = ((((((_e2668.x * _e2668.x) + (_e2668.y * _e2668.y)) + (_e2668.z * _e2668.z)) - (2f * (((_e2668.x * _e1454) + (_e2668.y * _e1457)) + (_e2668.z * _e1460)))) + _e1469) / (sqrt((((_e2685 * _e2685) + (_e2686 * _e2686)) + (_e2687 * _e2687))) + _e1463));
                                                                                    let _e2699 = (1f - (abs((_e2695 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2699 != _e2699) {
                                                                                        phi_23809_ = true;
                                                                                    } else {
                                                                                        phi_23809_ = (0f >= _e2699);
                                                                                    }
                                                                                    let _e2703 = phi_23809_;
                                                                                    let _e2705 = -(_e2695);
                                                                                    phi_23472_ = (_e2647 + vec3<f32>((exp((_e2705 / _e1484)) * _e2663), (exp((_e2705 / _e1487)) * _e2663), (select(_e2699, 0f, _e2703) * _e2663)));
                                                                                    phi_23473_ = _e2653;
                                                                                } else {
                                                                                    phi_23472_ = vec3<f32>();
                                                                                    phi_23473_ = u32();
                                                                                }
                                                                                let _e2716 = phi_23472_;
                                                                                let _e2718 = phi_23473_;
                                                                                phi_23438_ = _e2716;
                                                                                phi_23439_ = _e2718;
                                                                                if !(_e2650) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2721 = local_11;
                                                                            phi_23480_ = type_58(1u, _e2721);
                                                                        }
                                                                        let _e2727 = phi_23480_;
                                                                        phi_23484_ = _e2727;
                                                                    }
                                                                    let _e2732 = phi_23484_;
                                                                    if (bitcast<i32>(_e2732.member) != 0i) {
                                                                        let _e2737 = (vec3<f32>(_e2377, _e2379, _e2381) + _e2732.member_1);
                                                                        let _e2743 = (_e2386 * _e2737.y);
                                                                        let _e2755 = exp(-((((_e1493 * _e2737.x) + _e2743) + (_e1509 * _e2737.z))));
                                                                        let _e2757 = exp(-((((_e1496 * _e2737.x) + _e2743) + (_e1512 * _e2737.z))));
                                                                        let _e2759 = exp(-((((_e1499 * _e2737.x) + _e2743) + (_e1515 * _e2737.z))));
                                                                        phi_3210_ = vec3<f32>((_e2217.x + (_e2755 * _e2371)), (_e2217.y + (_e2757 * _e2371)), (_e2217.z + (_e2759 * _e2371)));
                                                                        phi_3211_ = vec3<f32>((_e2219.x + (_e2755 * _e2368)), (_e2219.y + (_e2757 * _e2368)), (_e2219.z + (_e2759 * _e2368)));
                                                                    } else {
                                                                        phi_3210_ = _e2217;
                                                                        phi_3211_ = _e2219;
                                                                    }
                                                                    let _e2781 = phi_3210_;
                                                                    let _e2783 = phi_3211_;
                                                                    phi_2714_ = vec3<f32>((_e2377 + _e2373), (_e2379 + _e2374), (_e2381 + _e2375));
                                                                    phi_2717_ = vec3<f32>((_e2215.x + ((exp(-((((_e1493 * _e2377) + _e2387) + (_e1509 * _e2381)))) * ((_e1493 * _e2368) + _e2406)) * _e2535.x)), (_e2215.y + ((exp(-((((_e1496 * _e2377) + _e2387) + (_e1512 * _e2381)))) * ((_e1496 * _e2368) + _e2406)) * _e2535.y)), (_e2215.z + ((exp(-((((_e1499 * _e2377) + _e2387) + (_e1515 * _e2381)))) * ((_e1499 * _e2368) + _e2406)) * _e2535.z)));
                                                                    phi_2719_ = _e2781;
                                                                    phi_2721_ = _e2783;
                                                                    phi_2723_ = (_e2221 + 1u);
                                                                } else {
                                                                    phi_2714_ = vec3<f32>();
                                                                    phi_2717_ = vec3<f32>();
                                                                    phi_2719_ = vec3<f32>();
                                                                    phi_2721_ = vec3<f32>();
                                                                    phi_2723_ = u32();
                                                                }
                                                                let _e2790 = phi_2714_;
                                                                let _e2792 = phi_2717_;
                                                                let _e2794 = phi_2719_;
                                                                let _e2796 = phi_2721_;
                                                                let _e2798 = phi_2723_;
                                                                phi_2713_ = _e2790;
                                                                phi_2716_ = _e2792;
                                                                phi_2718_ = _e2794;
                                                                phi_2720_ = _e2796;
                                                                phi_2722_ = _e2798;
                                                                phi_33555_ = _e1444;
                                                                if !(_e2222) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2801 = phi_33555_;
                                                            phi_33645_ = _e2801;
                                                            if _e2801 {
                                                                break;
                                                            }
                                                            let _e2803 = local_12;
                                                            let _e2806 = local_13;
                                                            let _e2809 = local_14;
                                                            phi_33649_ = _e2801;
                                                            phi_3231_ = type_52((_e2803 * vec3<f32>(_e1493, _e1496, _e1499)), (_e2806 * _e1503), _e2809);
                                                        } else {
                                                            phi_33649_ = _e1444;
                                                            phi_3231_ = type_52(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2812 = phi_33649_;
                                                        let _e2814 = phi_3231_;
                                                        phi_33648_ = _e2812;
                                                        phi_3232_ = _e2814;
                                                    } else {
                                                        phi_33648_ = _e1444;
                                                        phi_3232_ = _e2047;
                                                    }
                                                    let _e2816 = phi_33648_;
                                                    let _e2818 = phi_3232_;
                                                    let _e2826 = (((_e335.member_1.x * _e1475) + (_e335.member_1.y * _e1478)) + (_e335.member_1.z * _e1481));
                                                    let _e2828 = (1f + (_e2826 * _e2826));
                                                    let _e2829 = (0.059683103f * _e2828);
                                                    let _e2836 = (_e1506 * _e1506);
                                                    let _e2847 = ((0.119366206f * ((1f - _e2836) * _e2828)) / ((2f + _e2836) * pow(((1f + _e2836) - ((2f * _e1506) * _e2826)), 1.5f)));
                                                    phi_33647_ = _e2816;
                                                    phi_3255_ = vec3<f32>((_e1446.x + (((((_e2818.member.x * _e2829) + (_e2818.member_1.x * _e2847)) + _e2818.member_2.x) * _e1490) * 0.38317f)), (_e1446.y + (((((_e2818.member.y * _e2829) + (_e2818.member_1.y * _e2847)) + _e2818.member_2.y) * _e1490) * 0.38317f)), (_e1446.z + (((((_e2818.member.z * _e2829) + (_e2818.member_1.z * _e2847)) + _e2818.member_2.z) * _e1490) * 0.38317f)));
                                                    phi_3256_ = (_e1448 + 1i);
                                                    phi_3257_ = bitcast<u32>(_e1526);
                                                } else {
                                                    phi_33647_ = _e1444;
                                                    phi_3255_ = vec3<f32>();
                                                    phi_3256_ = i32();
                                                    phi_3257_ = u32();
                                                }
                                                let _e2878 = phi_33647_;
                                                let _e2880 = phi_3255_;
                                                let _e2882 = phi_3256_;
                                                let _e2884 = phi_3257_;
                                                phi_33646_ = _e2878;
                                                phi_901_ = _e2880;
                                                phi_904_ = _e2882;
                                                phi_906_ = _e2884;
                                                phi_3258_ = _e1452;
                                            } else {
                                                phi_33646_ = _e1444;
                                                phi_901_ = vec3<f32>();
                                                phi_904_ = i32();
                                                phi_906_ = u32();
                                                phi_3258_ = false;
                                            }
                                            let _e2886 = phi_33646_;
                                            let _e2888 = phi_901_;
                                            let _e2890 = phi_904_;
                                            let _e2892 = phi_906_;
                                            let _e2894 = phi_3258_;
                                            phi_33587_ = _e2886;
                                            phi_900_ = _e2888;
                                            phi_903_ = _e2890;
                                            phi_905_ = _e2892;
                                            phi_33645_ = _e2886;
                                            if !(_e2894) {
                                                break;
                                            }
                                        }
                                        let _e2897 = phi_33645_;
                                        phi_33973_ = _e2897;
                                        if _e2897 {
                                            break;
                                        }
                                        if _e1441 {
                                            let _e2900 = global_6.member[(_e1293 + 10u)];
                                            let _e2903 = global_6.member[(_e1293 + 21u)];
                                            let _e2904 = bitcast<u32>(_e2903);
                                            let _e2907 = global_6.member[(_e1293 + 22u)];
                                            let _e2908 = bitcast<u32>(_e2907);
                                            let _e2911 = global_6.member[(_e1293 + 23u)];
                                            let _e2915 = global_6.member[(_e1293 + 24u)];
                                            let _e2919 = global_6.member[(_e1293 + 25u)];
                                            let _e2923 = global_6.member[(_e1293 + 26u)];
                                            let _e2926 = global_6.member[_e1293];
                                            let _e2929 = global_6.member[(_e1293 + 1u)];
                                            let _e2932 = global_6.member[(_e1293 + 2u)];
                                            let _e2935 = global_6.member[(_e1293 + 3u)];
                                            let _e2938 = global_6.member[(_e1293 + 4u)];
                                            let _e2941 = global_6.member[(_e1293 + 5u)];
                                            let _e2944 = global_6.member[(_e1293 + 6u)];
                                            let _e2947 = global_6.member[(_e1293 + 7u)];
                                            let _e2950 = global_6.member[(_e1293 + 8u)];
                                            let _e2955 = (((_e335.member_1.x * _e2935) + (_e335.member_1.y * _e2938)) + (_e335.member_1.z * _e2941));
                                            let _e2957 = select(_e2955, -1f, (_e2955 < -1f));
                                            let _e2960 = asin(select(_e2957, 1f, (_e2957 > 1f)));
                                            let _e2971 = atan2((((_e335.member_1.x * _e2944) + (_e335.member_1.y * _e2947)) + (_e335.member_1.z * _e2950)), (((_e335.member_1.x * _e2926) + (_e335.member_1.y * _e2929)) + (_e335.member_1.z * _e2932)));
                                            if (_e2904 != 4294967295u) {
                                                let _e2974 = global_6.member[_e2904];
                                                let _e2977 = global_6.member[(_e2904 + 1u)];
                                                let _e2978 = bitcast<u32>(_e2977);
                                                let _e2981 = global_6.member[(_e2904 + 4u)];
                                                phi_3592_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3595_ = 0u;
                                                loop {
                                                    let _e2987 = phi_3592_;
                                                    let _e2989 = phi_3595_;
                                                    local_15 = _e2987;
                                                    let _e2990 = (_e2989 < 6u);
                                                    if _e2990 {
                                                        let _e2991 = (_e2989 / 2u);
                                                        if (_e2991 == 0u) {
                                                            phi_23937_ = _e335.member_1.x;
                                                        } else {
                                                            phi_23937_ = select(_e335.member_1.z, _e335.member_1.y, (_e2991 == 1u));
                                                        }
                                                        let _e2999 = phi_23937_;
                                                        let _e3000 = (_e2999 * select(-1f, 1f, ((_e2989 % 2u) == 0u)));
                                                        if (_e3000 > 0f) {
                                                            let _e3004 = ((_e2991 + 1u) % 3u);
                                                            if (_e3004 == 0u) {
                                                                phi_23952_ = _e335.member_1.x;
                                                            } else {
                                                                phi_23952_ = select(_e335.member_1.z, _e335.member_1.y, (_e3004 == 1u));
                                                            }
                                                            let _e3009 = phi_23952_;
                                                            let _e3010 = (_e3009 / _e3000);
                                                            let _e3012 = ((_e2991 + 2u) % 3u);
                                                            if (_e3012 == 0u) {
                                                                phi_23967_ = _e335.member_1.x;
                                                            } else {
                                                                phi_23967_ = select(_e335.member_1.z, _e335.member_1.y, (_e3012 == 1u));
                                                            }
                                                            let _e3017 = phi_23967_;
                                                            let _e3018 = (_e3017 / _e3000);
                                                            let _e3020 = (1f + (_e2974 * 12f));
                                                            if (abs(_e3010) <= _e3020) {
                                                                if (abs(_e3018) <= _e3020) {
                                                                    let _e3026 = f32(_e2978);
                                                                    let _e3028 = floor((((_e3010 + 1f) * 0.5f) * _e3026));
                                                                    let _e3035 = select(0i, select(select(i32(_e3028), i32(-2147483648), (_e3028 < -2147483600f)), 2147483647i, (_e3028 > 2147483500f)), (_e3028 == _e3028));
                                                                    let _e3037 = (bitcast<i32>(_e2978) - 1i);
                                                                    let _e3038 = (0i <= _e3037);
                                                                    if _e3038 {
                                                                    } else {
                                                                        phi_33650_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3035 < 0i) {
                                                                        phi_3666_ = 0i;
                                                                    } else {
                                                                        phi_3666_ = select(_e3035, _e3037, (_e3035 > _e3037));
                                                                    }
                                                                    let _e3043 = phi_3666_;
                                                                    let _e3047 = floor((((_e3018 + 1f) * 0.5f) * _e3026));
                                                                    let _e3054 = select(0i, select(select(i32(_e3047), i32(-2147483648), (_e3047 < -2147483600f)), 2147483647i, (_e3047 > 2147483500f)), (_e3047 == _e3047));
                                                                    if _e3038 {
                                                                    } else {
                                                                        phi_33650_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3054 < 0i) {
                                                                        phi_3696_ = 0i;
                                                                    } else {
                                                                        phi_3696_ = select(_e3054, _e3037, (_e3054 > _e3037));
                                                                    }
                                                                    let _e3059 = phi_3696_;
                                                                    let _e3060 = (_e3043 - 1i);
                                                                    phi_3704_ = _e2987;
                                                                    phi_3707_ = select(0i, _e3060, (0i < _e3060));
                                                                    loop {
                                                                        let _e3064 = phi_3704_;
                                                                        let _e3066 = phi_3707_;
                                                                        let _e3067 = (_e3043 + 1i);
                                                                        local_59 = _e3064;
                                                                        let _e3070 = (_e3066 <= select(_e3067, _e3037, (_e3037 < _e3067)));
                                                                        if _e3070 {
                                                                            let _e3071 = (_e3059 - 1i);
                                                                            phi_3730_ = _e3064;
                                                                            phi_3733_ = select(0i, _e3071, (0i < _e3071));
                                                                            loop {
                                                                                let _e3075 = phi_3730_;
                                                                                let _e3077 = phi_3733_;
                                                                                let _e3078 = (_e3059 + 1i);
                                                                                local_58 = _e3075;
                                                                                let _e3081 = (_e3077 <= select(_e3078, _e3037, (_e3037 < _e3078)));
                                                                                if _e3081 {
                                                                                    let _e3090 = global_6.member[(bitcast<u32>(_e2981) + ((((_e2989 * _e2978) + bitcast<u32>(_e3066)) * _e2978) + bitcast<u32>(_e3077)))];
                                                                                    phi_3731_ = vec3<f32>((_e3075.x + _e3090), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3734_ = (_e3077 + 1i);
                                                                                } else {
                                                                                    phi_3731_ = vec3<f32>();
                                                                                    phi_3734_ = i32();
                                                                                }
                                                                                let _e3099 = phi_3731_;
                                                                                let _e3101 = phi_3734_;
                                                                                phi_3730_ = _e3099;
                                                                                phi_3733_ = _e3101;
                                                                                if !(_e3081) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6936 = local_58;
                                                                            phi_3705_ = _e6936;
                                                                            phi_3708_ = (_e3066 + 1i);
                                                                        } else {
                                                                            phi_3705_ = vec3<f32>();
                                                                            phi_3708_ = i32();
                                                                        }
                                                                        let _e3105 = phi_3705_;
                                                                        let _e3107 = phi_3708_;
                                                                        phi_3704_ = _e3105;
                                                                        phi_3707_ = _e3107;
                                                                        if !(_e3070) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6940 = local_59;
                                                                    phi_3932_ = _e6940;
                                                                } else {
                                                                    phi_3932_ = _e2987;
                                                                }
                                                                let _e3110 = phi_3932_;
                                                                phi_3933_ = _e3110;
                                                            } else {
                                                                phi_3933_ = _e2987;
                                                            }
                                                            let _e3112 = phi_3933_;
                                                            phi_3934_ = _e3112;
                                                        } else {
                                                            phi_3934_ = _e2987;
                                                        }
                                                        let _e3114 = phi_3934_;
                                                        phi_3593_ = _e3114;
                                                        phi_3596_ = (_e2989 + 1u);
                                                    } else {
                                                        phi_3593_ = vec3<f32>();
                                                        phi_3596_ = u32();
                                                    }
                                                    let _e3117 = phi_3593_;
                                                    let _e3119 = phi_3596_;
                                                    phi_3592_ = _e3117;
                                                    phi_3595_ = _e3119;
                                                    phi_33650_ = _e2897;
                                                    if !(_e2990) {
                                                        break;
                                                    }
                                                }
                                                let _e3122 = phi_33650_;
                                                phi_33973_ = _e3122;
                                                if _e3122 {
                                                    break;
                                                }
                                                let _e3124 = local_15;
                                                phi_33698_ = _e3122;
                                                phi_3945_ = (_e3124.x * (1f / ((6.2831855f * _e2974) * _e2974)));
                                            } else {
                                                phi_33698_ = _e2897;
                                                phi_3945_ = 0f;
                                            }
                                            let _e3128 = phi_33698_;
                                            let _e3130 = phi_3945_;
                                            if (_e2908 != 4294967295u) {
                                                let _e3147 = (_e2960 * 1.3333334f);
                                                let _e3154 = global_6.member[_e2908];
                                                phi_3969_ = (_e3130 + ((_e3154 * ((exp((-(abs(_e2960)) / (0.06f + (abs(_e2971) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2971 * -0.71428573f) * (_e2971 * 0.71428573f)))))) + (1.3f * exp((((_e2971 * _e2971) + (_e3147 * _e3147)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3969_ = _e3130;
                                            }
                                            let _e3160 = phi_3969_;
                                            phi_3971_ = 0u;
                                            phi_3974_ = _e3160;
                                            loop {
                                                let _e3162 = phi_3971_;
                                                let _e3164 = phi_3974_;
                                                local_60 = _e3164;
                                                let _e3165 = (_e3162 < bitcast<u32>(_e2915));
                                                if _e3165 {
                                                    let _e3167 = (bitcast<u32>(_e2911) + (_e3162 * 18u));
                                                    let _e3170 = global_6.member[(_e3167 + 1u)];
                                                    let _e3173 = global_6.member[(_e3167 + 2u)];
                                                    let _e3176 = global_6.member[(_e3167 + 3u)];
                                                    let _e3184 = global_6.member[(_e3167 + 5u)];
                                                    if ((((_e335.member_1.x * _e3170) + (_e335.member_1.y * _e3173)) + (_e335.member_1.z * _e3176)) >= _e3184) {
                                                        let _e3187 = global_6.member[_e3167];
                                                        let _e3192 = select(select(u32(_e3187), 0u, (_e3187 < 0f)), 4294967295u, (_e3187 > 4294967000f));
                                                        if (_e3192 == 0u) {
                                                            phi_4078_ = 1.8f;
                                                        } else {
                                                            if (_e3192 == 1u) {
                                                                phi_4077_ = 1f;
                                                            } else {
                                                                phi_4077_ = select(0f, 4.7f, (_e3192 == 2u));
                                                            }
                                                            let _e3198 = phi_4077_;
                                                            phi_4078_ = _e3198;
                                                        }
                                                        let _e3200 = phi_4078_;
                                                        let _e3203 = global_6.member[(_e3167 + 6u)];
                                                        phi_4094_ = (_e3164 + (_e3203 * _e3200));
                                                    } else {
                                                        phi_4094_ = _e3164;
                                                    }
                                                    let _e3207 = phi_4094_;
                                                    phi_3972_ = (_e3162 + 1u);
                                                    phi_3975_ = _e3207;
                                                } else {
                                                    phi_3972_ = u32();
                                                    phi_3975_ = f32();
                                                }
                                                let _e3210 = phi_3972_;
                                                let _e3212 = phi_3975_;
                                                phi_3971_ = _e3210;
                                                phi_3974_ = _e3212;
                                                if !(_e3165) {
                                                    break;
                                                }
                                            }
                                            phi_4097_ = 0u;
                                            let _e6960 = local_60;
                                            phi_4100_ = _e6960;
                                            loop {
                                                let _e3215 = phi_4097_;
                                                let _e3217 = phi_4100_;
                                                local_16 = _e3217;
                                                let _e3218 = (_e3215 < bitcast<u32>(_e2923));
                                                if _e3218 {
                                                    let _e3220 = (bitcast<u32>(_e2919) + (_e3215 * 29u));
                                                    let _e3223 = global_6.member[(_e3220 + 1u)];
                                                    let _e3226 = global_6.member[(_e3220 + 2u)];
                                                    let _e3229 = global_6.member[(_e3220 + 3u)];
                                                    let _e3237 = global_6.member[(_e3220 + 5u)];
                                                    if ((((_e335.member_1.x * _e3223) + (_e335.member_1.y * _e3226)) + (_e335.member_1.z * _e3229)) >= _e3237) {
                                                        let _e3240 = global_6.member[_e3220];
                                                        let _e3244 = global_6.member[(_e3220 + 6u)];
                                                        phi_4208_ = (_e3217 + (_e3244 * select(1f, 3.3808f, (_e3240 < 0.5f))));
                                                    } else {
                                                        phi_4208_ = _e3217;
                                                    }
                                                    let _e3249 = phi_4208_;
                                                    phi_4098_ = (_e3215 + 1u);
                                                    phi_4101_ = _e3249;
                                                } else {
                                                    phi_4098_ = u32();
                                                    phi_4101_ = f32();
                                                }
                                                let _e3252 = phi_4098_;
                                                let _e3254 = phi_4101_;
                                                phi_4097_ = _e3252;
                                                phi_4100_ = _e3254;
                                                if !(_e3218) {
                                                    break;
                                                }
                                            }
                                            let _e3258 = global_6.member[(_e1293 + 9u)];
                                            let _e3260 = local_16;
                                            let _e3263 = local_17;
                                            let _e3267 = local_18;
                                            let _e3272 = local_19;
                                            let _e3277 = ((_e3260 * _e3258) <= (_e2900 * (((0.2126f * _e3263.x) + (0.7152f * _e3267.y)) + (0.0722f * _e3272.z))));
                                            if _e3277 {
                                                phi_33739_ = _e3128;
                                                phi_4767_ = vec3<f32>();
                                                phi_4768_ = bool();
                                            } else {
                                                let _e3279 = global_6.member[_e1290];
                                                let _e3282 = global_6.member[(_e1290 + 1u)];
                                                let _e3285 = global_6.member[(_e1290 + 2u)];
                                                let _e3288 = global_6.member[(_e1290 + 3u)];
                                                let _e3291 = global_6.member[(_e1290 + 5u)];
                                                let _e3294 = global_6.member[(_e1290 + 6u)];
                                                let _e3297 = global_6.member[(_e1290 + 10u)];
                                                let _e3300 = global_6.member[(_e1290 + 11u)];
                                                let _e3303 = global_6.member[(_e1290 + 13u)];
                                                let _e3306 = global_6.member[(_e1290 + 14u)];
                                                let _e3309 = global_6.member[(_e1290 + 15u)];
                                                let _e3312 = global_6.member[(_e1290 + 16u)];
                                                let _e3315 = global_6.member[(_e1290 + 18u)];
                                                let _e3318 = global_6.member[(_e1290 + 19u)];
                                                let _e3321 = global_6.member[(_e1290 + 20u)];
                                                let _e3333 = ((((_e335.member.x * _e335.member.x) + (_e335.member.y * _e335.member.y)) + (_e335.member.z * _e335.member.z)) - (2f * (((_e335.member.x * _e3279) + (_e335.member.y * _e3282)) + (_e335.member.z * _e3285))));
                                                let _e3334 = (_e3333 + _e3291);
                                                let _e3335 = (_e3334 > 0f);
                                                if _e3335 {
                                                    let _e3343 = ((((_e335.member.x - _e3279) * _e335.member_1.x) + ((_e335.member.y - _e3282) * _e335.member_1.y)) + ((_e335.member.z - _e3285) * _e335.member_1.z));
                                                    let _e3345 = ((_e3343 * _e3343) - _e3334);
                                                    if (_e3345 < 0f) {
                                                        phi_24111_ = type_57(0u, type_57().member_1);
                                                        phi_24112_ = type_57();
                                                        phi_24113_ = true;
                                                    } else {
                                                        if (_e3343 < 0f) {
                                                            phi_24083_ = -(sqrt(_e3345));
                                                        } else {
                                                            phi_24083_ = sqrt(_e3345);
                                                        }
                                                        let _e3352 = phi_24083_;
                                                        let _e3354 = -((_e3343 + _e3352));
                                                        let _e3355 = (_e3354 == 0f);
                                                        if _e3355 {
                                                            phi_24106_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24107_ = type_57();
                                                        } else {
                                                            let _e3356 = (_e3334 / _e3354);
                                                            if (_e3354 < _e3356) {
                                                                phi_24095_ = type_56(_e3354, _e3356);
                                                            } else {
                                                                phi_24095_ = type_56(_e3356, _e3354);
                                                            }
                                                            let _e3361 = phi_24095_;
                                                            phi_24106_ = type_57();
                                                            phi_24107_ = type_57(1u, _e3361);
                                                        }
                                                        let _e3364 = phi_24106_;
                                                        let _e3366 = phi_24107_;
                                                        phi_24111_ = _e3364;
                                                        phi_24112_ = _e3366;
                                                        phi_24113_ = _e3355;
                                                    }
                                                    let _e3371 = phi_24111_;
                                                    let _e3373 = phi_24112_;
                                                    let _e3375 = phi_24113_;
                                                    if _e3375 {
                                                        phi_24117_ = _e3371;
                                                    } else {
                                                        phi_24117_ = _e3373;
                                                    }
                                                    let _e3377 = phi_24117_;
                                                    if (bitcast<i32>(_e3377.member) != 0i) {
                                                        phi_24051_ = select(_e3377.member_1.member_1, _e3377.member_1.member, (_e3377.member_1.member >= 0f));
                                                    } else {
                                                        phi_24051_ = -1f;
                                                    }
                                                    let _e3388 = phi_24051_;
                                                    phi_4609_ = select(true, false, (_e3388 >= 0f));
                                                } else {
                                                    phi_4609_ = true;
                                                }
                                                let _e3392 = phi_4609_;
                                                if _e3392 {
                                                    let _e3400 = ((((_e335.member.x - _e3279) * _e335.member_1.x) + ((_e335.member.y - _e3282) * _e335.member_1.y)) + ((_e335.member.z - _e3285) * _e335.member_1.z));
                                                    let _e3401 = (_e3333 + _e3294);
                                                    let _e3402 = (_e3400 * _e3400);
                                                    let _e3403 = (_e3402 - _e3401);
                                                    if (_e3403 < 0f) {
                                                        phi_24271_ = type_57(0u, type_57().member_1);
                                                        phi_24272_ = type_57();
                                                        phi_24273_ = true;
                                                    } else {
                                                        if (_e3400 < 0f) {
                                                            phi_24243_ = -(sqrt(_e3403));
                                                        } else {
                                                            phi_24243_ = sqrt(_e3403);
                                                        }
                                                        let _e3410 = phi_24243_;
                                                        let _e3412 = -((_e3400 + _e3410));
                                                        let _e3413 = (_e3412 == 0f);
                                                        if _e3413 {
                                                            phi_24266_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24267_ = type_57();
                                                        } else {
                                                            let _e3414 = (_e3401 / _e3412);
                                                            if (_e3412 < _e3414) {
                                                                phi_24255_ = type_56(_e3412, _e3414);
                                                            } else {
                                                                phi_24255_ = type_56(_e3414, _e3412);
                                                            }
                                                            let _e3419 = phi_24255_;
                                                            phi_24266_ = type_57();
                                                            phi_24267_ = type_57(1u, _e3419);
                                                        }
                                                        let _e3422 = phi_24266_;
                                                        let _e3424 = phi_24267_;
                                                        phi_24271_ = _e3422;
                                                        phi_24272_ = _e3424;
                                                        phi_24273_ = _e3413;
                                                    }
                                                    let _e3429 = phi_24271_;
                                                    let _e3431 = phi_24272_;
                                                    let _e3433 = phi_24273_;
                                                    if _e3433 {
                                                        phi_24277_ = _e3429;
                                                    } else {
                                                        phi_24277_ = _e3431;
                                                    }
                                                    let _e3435 = phi_24277_;
                                                    if (bitcast<i32>(_e3435.member) != 0i) {
                                                        phi_24164_ = type_57(0u, _e3435.member_1);
                                                    } else {
                                                        phi_24164_ = type_57(1u, type_57().member_1);
                                                    }
                                                    let _e3445 = phi_24164_;
                                                    if (bitcast<i32>(_e3445.member) != 0i) {
                                                        phi_24211_ = type_57(0u, type_57().member_1);
                                                    } else {
                                                        if (_e3445.member_1.member_1 <= 0f) {
                                                            phi_24207_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e3445.member_1.member != _e3445.member_1.member) {
                                                                phi_24315_ = true;
                                                            } else {
                                                                phi_24315_ = (0f >= _e3445.member_1.member);
                                                            }
                                                            let _e3457 = phi_24315_;
                                                            let _e3458 = select(_e3445.member_1.member, 0f, _e3457);
                                                            if _e3335 {
                                                                let _e3459 = (_e3402 - _e3334);
                                                                if (_e3459 < 0f) {
                                                                    phi_24423_ = type_57(0u, type_57().member_1);
                                                                    phi_24424_ = type_57();
                                                                    phi_24425_ = true;
                                                                } else {
                                                                    if (_e3400 < 0f) {
                                                                        phi_24395_ = -(sqrt(_e3459));
                                                                    } else {
                                                                        phi_24395_ = sqrt(_e3459);
                                                                    }
                                                                    let _e3466 = phi_24395_;
                                                                    let _e3468 = -((_e3400 + _e3466));
                                                                    let _e3469 = (_e3468 == 0f);
                                                                    if _e3469 {
                                                                        phi_24418_ = type_57(1u, type_56(0f, 0f));
                                                                        phi_24419_ = type_57();
                                                                    } else {
                                                                        let _e3470 = (_e3334 / _e3468);
                                                                        if (_e3468 < _e3470) {
                                                                            phi_24407_ = type_56(_e3468, _e3470);
                                                                        } else {
                                                                            phi_24407_ = type_56(_e3470, _e3468);
                                                                        }
                                                                        let _e3475 = phi_24407_;
                                                                        phi_24418_ = type_57();
                                                                        phi_24419_ = type_57(1u, _e3475);
                                                                    }
                                                                    let _e3478 = phi_24418_;
                                                                    let _e3480 = phi_24419_;
                                                                    phi_24423_ = _e3478;
                                                                    phi_24424_ = _e3480;
                                                                    phi_24425_ = _e3469;
                                                                }
                                                                let _e3485 = phi_24423_;
                                                                let _e3487 = phi_24424_;
                                                                let _e3489 = phi_24425_;
                                                                if _e3489 {
                                                                    phi_24429_ = _e3485;
                                                                } else {
                                                                    phi_24429_ = _e3487;
                                                                }
                                                                let _e3491 = phi_24429_;
                                                                if (bitcast<i32>(_e3491.member) != 0i) {
                                                                    phi_24363_ = select(_e3491.member_1.member_1, _e3491.member_1.member, (_e3491.member_1.member >= 0f));
                                                                } else {
                                                                    phi_24363_ = -1f;
                                                                }
                                                                let _e3502 = phi_24363_;
                                                                if (_e3502 >= 0f) {
                                                                    if (_e3445.member_1.member_1 != _e3445.member_1.member_1) {
                                                                        phi_24467_ = true;
                                                                    } else {
                                                                        phi_24467_ = (_e3502 <= _e3445.member_1.member_1);
                                                                    }
                                                                    let _e3507 = phi_24467_;
                                                                    phi_24192_ = select(_e3445.member_1.member_1, _e3502, _e3507);
                                                                } else {
                                                                    phi_24192_ = _e3445.member_1.member_1;
                                                                }
                                                                let _e3510 = phi_24192_;
                                                                phi_24194_ = _e3510;
                                                            } else {
                                                                phi_24194_ = _e3445.member_1.member_1;
                                                            }
                                                            let _e3512 = phi_24194_;
                                                            if (_e3512 <= _e3458) {
                                                                phi_24203_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                phi_24203_ = type_57(1u, type_56(_e3458, _e3512));
                                                            }
                                                            let _e3520 = phi_24203_;
                                                            phi_24207_ = _e3520;
                                                        }
                                                        let _e3525 = phi_24207_;
                                                        phi_24211_ = _e3525;
                                                    }
                                                    let _e3530 = phi_24211_;
                                                    let _e3533 = (bitcast<i32>(_e3530.member) != 0i);
                                                    if _e3533 {
                                                        phi_4623_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4626_ = 0u;
                                                        loop {
                                                            let _e3539 = phi_4623_;
                                                            let _e3541 = phi_4626_;
                                                            local_20 = _e3539;
                                                            local_21 = _e3539;
                                                            local_22 = _e3539;
                                                            let _e3542 = (_e3541 < 32u);
                                                            if _e3542 {
                                                                let _e3543 = -(_e3400);
                                                                if (_e3530.member_1.member <= _e3530.member_1.member_1) {
                                                                } else {
                                                                    phi_33677_ = true;
                                                                    break;
                                                                }
                                                                let _e3546 = select(_e3543, _e3530.member_1.member, (_e3543 < _e3530.member_1.member));
                                                                let _e3548 = select(_e3546, _e3530.member_1.member_1, (_e3546 > _e3530.member_1.member_1));
                                                                let _e3549 = (_e3548 - _e3530.member_1.member);
                                                                let _e3550 = (_e3530.member_1.member_1 - _e3548);
                                                                let _e3551 = (_e3530.member_1.member_1 - _e3530.member_1.member);
                                                                let _e3552 = (0.000000001f * _e3551);
                                                                if (_e3549 <= _e3552) {
                                                                    let _e3627 = (f32(_e3541) * 0.03125f);
                                                                    let _e3630 = (f32((_e3541 + 1u)) * 0.03125f);
                                                                    let _e3632 = ((_e3627 + _e3630) * 0.5f);
                                                                    phi_4727_ = type_56((_e3530.member_1.member + ((_e3551 * _e3632) * _e3632)), (_e3551 * ((_e3630 * _e3630) - (_e3627 * _e3627))));
                                                                } else {
                                                                    if (_e3550 <= _e3552) {
                                                                        let _e3610 = (f32((31u - _e3541)) * 0.03125f);
                                                                        let _e3613 = (f32((32u - _e3541)) * 0.03125f);
                                                                        let _e3615 = ((_e3610 + _e3613) * 0.5f);
                                                                        phi_4726_ = type_56((_e3530.member_1.member_1 - ((_e3551 * _e3615) * _e3615)), (_e3551 * ((_e3613 * _e3613) - (_e3610 * _e3610))));
                                                                    } else {
                                                                        let _e3558 = round(((32f * _e3549) / (_e3549 + _e3550)));
                                                                        let _e3563 = select(select(u32(_e3558), 0u, (_e3558 < 0f)), 4294967295u, (_e3558 > 4294967000f));
                                                                        if (_e3563 < 1u) {
                                                                            phi_4710_ = 1u;
                                                                        } else {
                                                                            phi_4710_ = select(_e3563, 31u, (_e3563 > 31u));
                                                                        }
                                                                        let _e3568 = phi_4710_;
                                                                        if (_e3541 < _e3568) {
                                                                            let _e3589 = ((_e3568 - 1u) - _e3541);
                                                                            let _e3591 = f32(_e3568);
                                                                            let _e3592 = (f32(_e3589) / _e3591);
                                                                            let _e3595 = (f32((_e3589 + 1u)) / _e3591);
                                                                            let _e3597 = ((_e3592 + _e3595) * 0.5f);
                                                                            phi_4725_ = type_56((_e3548 - ((_e3549 * _e3597) * _e3597)), (_e3549 * ((_e3595 * _e3595) - (_e3592 * _e3592))));
                                                                        } else {
                                                                            let _e3571 = (_e3541 - _e3568);
                                                                            let _e3573 = f32((32u - _e3568));
                                                                            let _e3574 = (f32(_e3571) / _e3573);
                                                                            let _e3577 = (f32((_e3571 + 1u)) / _e3573);
                                                                            let _e3579 = ((_e3574 + _e3577) * 0.5f);
                                                                            phi_4725_ = type_56((_e3548 + ((_e3550 * _e3579) * _e3579)), (_e3550 * ((_e3577 * _e3577) - (_e3574 * _e3574))));
                                                                        }
                                                                        let _e3607 = phi_4725_;
                                                                        phi_4726_ = _e3607;
                                                                    }
                                                                    let _e3625 = phi_4726_;
                                                                    phi_4727_ = _e3625;
                                                                }
                                                                let _e3642 = phi_4727_;
                                                                let _e3648 = (_e335.member.x + (_e335.member_1.x * _e3642.member));
                                                                let _e3649 = (_e335.member.y + (_e335.member_1.y * _e3642.member));
                                                                let _e3650 = (_e335.member.z + (_e335.member_1.z * _e3642.member));
                                                                let _e3664 = (_e3648 - _e3279);
                                                                let _e3665 = (_e3649 - _e3282);
                                                                let _e3666 = (_e3650 - _e3285);
                                                                let _e3674 = ((((((_e3648 * _e3648) + (_e3649 * _e3649)) + (_e3650 * _e3650)) - (2f * (((_e3648 * _e3279) + (_e3649 * _e3282)) + (_e3650 * _e3285)))) + _e3291) / (sqrt((((_e3664 * _e3664) + (_e3665 * _e3665)) + (_e3666 * _e3666))) + _e3288));
                                                                let _e3678 = (1f - (abs((_e3674 - 25000f)) * 0.00006666667f));
                                                                if (_e3678 != _e3678) {
                                                                    phi_24666_ = true;
                                                                } else {
                                                                    phi_24666_ = (0f >= _e3678);
                                                                }
                                                                let _e3682 = phi_24666_;
                                                                let _e3684 = -(_e3674);
                                                                phi_4624_ = (_e3539 + vec3<f32>((exp((_e3684 / _e3297)) * _e3642.member_1), (exp((_e3684 / _e3300)) * _e3642.member_1), (select(_e3678, 0f, _e3682) * _e3642.member_1)));
                                                                phi_4627_ = (_e3541 + 1u);
                                                            } else {
                                                                phi_4624_ = vec3<f32>();
                                                                phi_4627_ = u32();
                                                            }
                                                            let _e3696 = phi_4624_;
                                                            let _e3698 = phi_4627_;
                                                            phi_4623_ = _e3696;
                                                            phi_4626_ = _e3698;
                                                            phi_33677_ = _e3128;
                                                            if !(_e3542) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3701 = phi_33677_;
                                                        phi_33973_ = _e3701;
                                                        if _e3701 {
                                                            break;
                                                        }
                                                        let _e3703 = local_20;
                                                        let _e3710 = local_21;
                                                        let _e3712 = ((_e3312 * 1.1f) * _e3710.y);
                                                        let _e3717 = local_22;
                                                        phi_33744_ = _e3701;
                                                        phi_4752_ = vec3<f32>(exp(-((((_e3303 * _e3703.x) + _e3712) + (_e3315 * _e3717.z)))), exp(-((((_e3306 * _e3703.x) + _e3712) + (_e3318 * _e3717.z)))), exp(-((((_e3309 * _e3703.x) + _e3712) + (_e3321 * _e3717.z)))));
                                                    } else {
                                                        phi_33744_ = _e3128;
                                                        phi_4752_ = vec3<f32>();
                                                    }
                                                    let _e3733 = phi_33744_;
                                                    let _e3735 = phi_4752_;
                                                    phi_33743_ = _e3733;
                                                    phi_4754_ = _e3735;
                                                    phi_4756_ = select(true, false, _e3533);
                                                } else {
                                                    phi_33743_ = _e3128;
                                                    phi_4754_ = vec3<f32>();
                                                    phi_4756_ = true;
                                                }
                                                let _e3738 = phi_33743_;
                                                let _e3740 = phi_4754_;
                                                let _e3742 = phi_4756_;
                                                let _e3746 = select(_e3740, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3392)), vec3(_e3742));
                                                let _e3750 = select(_e3746.y, _e3746.x, (_e3746.x > _e3746.y));
                                                phi_33739_ = _e3738;
                                                phi_4767_ = _e3746;
                                                phi_4768_ = (select(_e3746.z, _e3750, (_e3750 > _e3746.z)) > 0f);
                                            }
                                            let _e3756 = phi_33739_;
                                            let _e3758 = phi_4767_;
                                            let _e3760 = phi_4768_;
                                            phi_33738_ = _e3756;
                                            phi_4770_ = _e3758;
                                            phi_4771_ = _e3760;
                                            phi_4772_ = select(true, false, _e3277);
                                        } else {
                                            phi_33738_ = _e2897;
                                            phi_4770_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4771_ = _e1441;
                                            phi_4772_ = true;
                                        }
                                        let _e3763 = phi_33738_;
                                        let _e3765 = phi_4770_;
                                        let _e3767 = phi_4771_;
                                        let _e3769 = phi_4772_;
                                        phi_33737_ = _e3763;
                                        let _e7031 = local_61;
                                        phi_4776_ = _e7031;
                                        phi_4777_ = _e3765;
                                        phi_4778_ = _e3767;
                                        let _e7036 = local_62;
                                        phi_4779_ = _e7036;
                                        phi_4780_ = _e3769;
                                    } else {
                                        phi_33737_ = _e324;
                                        phi_4776_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4777_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4778_ = _e1441;
                                        phi_4779_ = vec3<f32>();
                                        phi_4780_ = true;
                                    }
                                    let _e3771 = phi_33737_;
                                    let _e3773 = phi_4776_;
                                    let _e3775 = phi_4777_;
                                    let _e3777 = phi_4778_;
                                    let _e3779 = phi_4779_;
                                    let _e3781 = phi_4780_;
                                    if _e3781 {
                                        if _e3777 {
                                            let _e3782 = (_e1293 + 20u);
                                            let _e3785 = global_6.member[(_e1293 + 21u)];
                                            let _e3786 = bitcast<u32>(_e3785);
                                            let _e3789 = global_6.member[(_e1293 + 22u)];
                                            let _e3790 = bitcast<u32>(_e3789);
                                            let _e3793 = global_6.member[(_e1293 + 23u)];
                                            let _e3797 = global_6.member[(_e1293 + 24u)];
                                            let _e3801 = global_6.member[(_e1293 + 25u)];
                                            let _e3805 = global_6.member[(_e1293 + 26u)];
                                            let _e3808 = global_6.member[_e1293];
                                            let _e3811 = global_6.member[(_e1293 + 1u)];
                                            let _e3814 = global_6.member[(_e1293 + 2u)];
                                            let _e3817 = global_6.member[(_e1293 + 3u)];
                                            let _e3820 = global_6.member[(_e1293 + 4u)];
                                            let _e3823 = global_6.member[(_e1293 + 5u)];
                                            let _e3826 = global_6.member[(_e1293 + 6u)];
                                            let _e3829 = global_6.member[(_e1293 + 7u)];
                                            let _e3832 = global_6.member[(_e1293 + 8u)];
                                            let _e3837 = (((_e335.member_1.x * _e3817) + (_e335.member_1.y * _e3820)) + (_e335.member_1.z * _e3823));
                                            let _e3839 = select(_e3837, -1f, (_e3837 < -1f));
                                            let _e3842 = asin(select(_e3839, 1f, (_e3839 > 1f)));
                                            let _e3853 = atan2((((_e335.member_1.x * _e3826) + (_e335.member_1.y * _e3829)) + (_e335.member_1.z * _e3832)), (((_e335.member_1.x * _e3808) + (_e335.member_1.y * _e3811)) + (_e335.member_1.z * _e3814)));
                                            if (_e3786 != 4294967295u) {
                                                let _e3856 = global_6.member[_e3786];
                                                let _e3859 = global_6.member[(_e3786 + 1u)];
                                                let _e3860 = bitcast<u32>(_e3859);
                                                let _e3863 = global_6.member[(_e3786 + 2u)];
                                                let _e3867 = global_6.member[(_e3786 + 3u)];
                                                let _e3869 = (4f * _e3856);
                                                phi_5100_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5103_ = 0u;
                                                loop {
                                                    let _e3874 = phi_5100_;
                                                    let _e3876 = phi_5103_;
                                                    local_66 = _e3874;
                                                    let _e3877 = (_e3876 < 6u);
                                                    if _e3877 {
                                                        let _e3878 = (_e3876 / 2u);
                                                        if (_e3878 == 0u) {
                                                            phi_24737_ = _e335.member_1.x;
                                                        } else {
                                                            phi_24737_ = select(_e335.member_1.z, _e335.member_1.y, (_e3878 == 1u));
                                                        }
                                                        let _e3886 = phi_24737_;
                                                        let _e3887 = (_e3886 * select(-1f, 1f, ((_e3876 % 2u) == 0u)));
                                                        if (_e3887 > 0f) {
                                                            let _e3891 = ((_e3878 + 1u) % 3u);
                                                            if (_e3891 == 0u) {
                                                                phi_24752_ = _e335.member_1.x;
                                                            } else {
                                                                phi_24752_ = select(_e335.member_1.z, _e335.member_1.y, (_e3891 == 1u));
                                                            }
                                                            let _e3896 = phi_24752_;
                                                            let _e3897 = (_e3896 / _e3887);
                                                            let _e3899 = ((_e3878 + 2u) % 3u);
                                                            if (_e3899 == 0u) {
                                                                phi_24767_ = _e335.member_1.x;
                                                            } else {
                                                                phi_24767_ = select(_e335.member_1.z, _e335.member_1.y, (_e3899 == 1u));
                                                            }
                                                            let _e3904 = phi_24767_;
                                                            let _e3905 = (_e3904 / _e3887);
                                                            let _e3907 = (1f + (_e3856 * 12f));
                                                            if (abs(_e3897) <= _e3907) {
                                                                if (abs(_e3905) <= _e3907) {
                                                                    let _e3913 = f32(_e3860);
                                                                    let _e3915 = floor((((_e3897 + 1f) * 0.5f) * _e3913));
                                                                    let _e3922 = select(0i, select(select(i32(_e3915), i32(-2147483648), (_e3915 < -2147483600f)), 2147483647i, (_e3915 > 2147483500f)), (_e3915 == _e3915));
                                                                    let _e3924 = (bitcast<i32>(_e3860) - 1i);
                                                                    let _e3925 = (0i <= _e3924);
                                                                    if _e3925 {
                                                                    } else {
                                                                        phi_33732_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3922 < 0i) {
                                                                        phi_5174_ = 0i;
                                                                    } else {
                                                                        phi_5174_ = select(_e3922, _e3924, (_e3922 > _e3924));
                                                                    }
                                                                    let _e3930 = phi_5174_;
                                                                    let _e3934 = floor((((_e3905 + 1f) * 0.5f) * _e3913));
                                                                    let _e3941 = select(0i, select(select(i32(_e3934), i32(-2147483648), (_e3934 < -2147483600f)), 2147483647i, (_e3934 > 2147483500f)), (_e3934 == _e3934));
                                                                    if _e3925 {
                                                                    } else {
                                                                        phi_33732_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3941 < 0i) {
                                                                        phi_5204_ = 0i;
                                                                    } else {
                                                                        phi_5204_ = select(_e3941, _e3924, (_e3941 > _e3924));
                                                                    }
                                                                    let _e3946 = phi_5204_;
                                                                    let _e3947 = (_e3930 - 1i);
                                                                    phi_5212_ = _e3874;
                                                                    phi_5215_ = select(0i, _e3947, (0i < _e3947));
                                                                    loop {
                                                                        let _e3951 = phi_5212_;
                                                                        let _e3953 = phi_5215_;
                                                                        let _e3954 = (_e3930 + 1i);
                                                                        local_65 = _e3951;
                                                                        let _e3957 = (_e3953 <= select(_e3954, _e3924, (_e3924 < _e3954)));
                                                                        if _e3957 {
                                                                            let _e3958 = (_e3946 - 1i);
                                                                            phi_5238_ = _e3951;
                                                                            phi_5241_ = select(0i, _e3958, (0i < _e3958));
                                                                            loop {
                                                                                let _e3962 = phi_5238_;
                                                                                let _e3964 = phi_5241_;
                                                                                let _e3965 = (_e3946 + 1i);
                                                                                local_64 = _e3962;
                                                                                let _e3968 = (_e3964 <= select(_e3965, _e3924, (_e3924 < _e3965)));
                                                                                if _e3968 {
                                                                                    let _e3975 = (bitcast<u32>(_e3863) + ((((_e3876 * _e3860) + bitcast<u32>(_e3953)) * _e3860) + bitcast<u32>(_e3964)));
                                                                                    let _e3977 = global_6.member[_e3975];
                                                                                    let _e3981 = global_6.member[(_e3975 + 1u)];
                                                                                    phi_5312_ = _e3962;
                                                                                    phi_5315_ = bitcast<u32>(_e3977);
                                                                                    loop {
                                                                                        let _e3984 = phi_5312_;
                                                                                        let _e3986 = phi_5315_;
                                                                                        local_63 = _e3984;
                                                                                        let _e3987 = (_e3986 < bitcast<u32>(_e3981));
                                                                                        if _e3987 {
                                                                                            let _e3989 = (bitcast<u32>(_e3867) + (6u * _e3986));
                                                                                            let _e3991 = global_6.member[_e3989];
                                                                                            let _e3994 = global_6.member[(_e3989 + 1u)];
                                                                                            let _e3997 = global_6.member[(_e3989 + 2u)];
                                                                                            let _e4000 = global_6.member[(_e3989 + 3u)];
                                                                                            let _e4003 = global_6.member[(_e3989 + 4u)];
                                                                                            let _e4006 = global_6.member[(_e3989 + 5u)];
                                                                                            let _e4007 = (_e335.member_1.x - _e3991);
                                                                                            let _e4008 = (_e335.member_1.y - _e3994);
                                                                                            let _e4009 = (_e335.member_1.z - _e3997);
                                                                                            let _e4014 = (((_e4007 * _e4007) + (_e4008 * _e4008)) + (_e4009 * _e4009));
                                                                                            if (_e4014 < (_e3869 * _e3869)) {
                                                                                                let _e4022 = ((1f / ((6.2831855f * _e3856) * _e3856)) * exp((-(_e4014) / ((2f * _e3856) * _e3856))));
                                                                                                phi_5435_ = vec3<f32>((_e3984.x + (_e4000 * _e4022)), (_e3984.y + (_e4003 * _e4022)), (_e3984.z + (_e4006 * _e4022)));
                                                                                            } else {
                                                                                                phi_5435_ = _e3984;
                                                                                            }
                                                                                            let _e4034 = phi_5435_;
                                                                                            phi_5313_ = _e4034;
                                                                                            phi_5316_ = (_e3986 + 1u);
                                                                                        } else {
                                                                                            phi_5313_ = vec3<f32>();
                                                                                            phi_5316_ = u32();
                                                                                        }
                                                                                        let _e4037 = phi_5313_;
                                                                                        let _e4039 = phi_5316_;
                                                                                        phi_5312_ = _e4037;
                                                                                        phi_5315_ = _e4039;
                                                                                        if !(_e3987) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7056 = local_63;
                                                                                    phi_5239_ = _e7056;
                                                                                    phi_5242_ = (_e3964 + 1i);
                                                                                } else {
                                                                                    phi_5239_ = vec3<f32>();
                                                                                    phi_5242_ = i32();
                                                                                }
                                                                                let _e4043 = phi_5239_;
                                                                                let _e4045 = phi_5242_;
                                                                                phi_5238_ = _e4043;
                                                                                phi_5241_ = _e4045;
                                                                                if !(_e3968) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7060 = local_64;
                                                                            phi_5213_ = _e7060;
                                                                            phi_5216_ = (_e3953 + 1i);
                                                                        } else {
                                                                            phi_5213_ = vec3<f32>();
                                                                            phi_5216_ = i32();
                                                                        }
                                                                        let _e4049 = phi_5213_;
                                                                        let _e4051 = phi_5216_;
                                                                        phi_5212_ = _e4049;
                                                                        phi_5215_ = _e4051;
                                                                        if !(_e3957) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7064 = local_65;
                                                                    phi_5440_ = _e7064;
                                                                } else {
                                                                    phi_5440_ = _e3874;
                                                                }
                                                                let _e4054 = phi_5440_;
                                                                phi_5441_ = _e4054;
                                                            } else {
                                                                phi_5441_ = _e3874;
                                                            }
                                                            let _e4056 = phi_5441_;
                                                            phi_5442_ = _e4056;
                                                        } else {
                                                            phi_5442_ = _e3874;
                                                        }
                                                        let _e4058 = phi_5442_;
                                                        phi_5101_ = _e4058;
                                                        phi_5104_ = (_e3876 + 1u);
                                                    } else {
                                                        phi_5101_ = vec3<f32>();
                                                        phi_5104_ = u32();
                                                    }
                                                    let _e4061 = phi_5101_;
                                                    let _e4063 = phi_5104_;
                                                    phi_5100_ = _e4061;
                                                    phi_5103_ = _e4063;
                                                    phi_33732_ = _e3771;
                                                    if !(_e3877) {
                                                        break;
                                                    }
                                                }
                                                let _e4066 = phi_33732_;
                                                phi_33973_ = _e4066;
                                                if _e4066 {
                                                    break;
                                                }
                                                phi_33774_ = _e4066;
                                                let _e7073 = local_66;
                                                phi_5462_ = _e7073;
                                            } else {
                                                phi_33774_ = _e3771;
                                                phi_5462_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4068 = phi_33774_;
                                            let _e4070 = phi_5462_;
                                            if (_e3790 != 4294967295u) {
                                                let _e4073 = global_6.member[_e3790];
                                                let _e4076 = global_6.member[(_e3790 + 1u)];
                                                let _e4079 = global_6.member[(_e3790 + 2u)];
                                                let _e4082 = global_6.member[(_e3790 + 3u)];
                                                let _e4085 = global_6.member[(_e3790 + 4u)];
                                                let _e4088 = global_6.member[(_e3790 + 5u)];
                                                let _e4091 = global_6.member[(_e3790 + 6u)];
                                                let _e4094 = global_6.member[(_e3790 + 7u)];
                                                let _e4097 = global_6.member[(_e3790 + 8u)];
                                                let _e4100 = global_6.member[(_e3790 + 9u)];
                                                let _e4103 = global_6.member[(_e3790 + 10u)];
                                                let _e4107 = global_6.member[(_e3790 + 11u)];
                                                let _e4110 = global_6.member[(_e3790 + 12u)];
                                                let _e4113 = global_6.member[(_e3790 + 13u)];
                                                let _e4115 = (_e4073 > 0f);
                                                let _e4116 = (_e4076 > 0f);
                                                if _e4116 {
                                                    let _e4119 = (abs(_e3842) <= 0.8f);
                                                    let _e4120 = select(2u, 0u, _e4115);
                                                    phi_5675_ = _e4120;
                                                    phi_5676_ = _e4119;
                                                    phi_5679_ = _e4120;
                                                    phi_5680_ = _e4119;
                                                    phi_5681_ = select(true, false, _e4119);
                                                } else {
                                                    phi_5675_ = u32();
                                                    phi_5676_ = bool();
                                                    phi_5679_ = select(2u, 0u, _e4115);
                                                    phi_5680_ = false;
                                                    phi_5681_ = true;
                                                }
                                                let _e4123 = phi_5675_;
                                                let _e4125 = phi_5676_;
                                                let _e4127 = phi_5679_;
                                                let _e4129 = phi_5680_;
                                                let _e4131 = phi_5681_;
                                                let _e4133 = select(select(f32(), 0f, _e4116), 0f, _e4131);
                                                phi_5691_ = _e4133;
                                                phi_5694_ = _e4133;
                                                phi_5696_ = select(_e4123, _e4127, _e4131);
                                                phi_5698_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5700_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5702_ = 0f;
                                                phi_5704_ = 1f;
                                                loop {
                                                    let _e4138 = phi_5691_;
                                                    let _e4140 = phi_5694_;
                                                    let _e4142 = phi_5696_;
                                                    let _e4144 = phi_5698_;
                                                    let _e4146 = phi_5700_;
                                                    let _e4148 = phi_5702_;
                                                    let _e4150 = phi_5704_;
                                                    local_26 = _e4150;
                                                    local_27 = _e4150;
                                                    local_28 = _e4150;
                                                    local_29 = _e4150;
                                                    local_30 = _e4148;
                                                    local_31 = _e4140;
                                                    local_32 = _e4138;
                                                    let _e4151 = (_e4142 < select(11u, 2u, _e4131));
                                                    if _e4151 {
                                                        let _e4152 = (_e4142 < 2u);
                                                        if _e4152 {
                                                            let _e4208 = select(23f, 4f, (_e4142 == 0u));
                                                            phi_5782_ = 4u;
                                                            phi_5784_ = vec3<f32>((_e335.member_1.x * _e4208), (_e335.member_1.y * _e4208), (_e335.member_1.z * _e4208));
                                                        } else {
                                                            if (_e4142 == 2u) {
                                                                phi_5780_ = 3u;
                                                                phi_5781_ = vec3<f32>(((_e335.member_1.x * 1.5f) + 5f), ((_e335.member_1.y * 1.5f) + 5f), ((_e335.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4154 = (_e4142 >= 7u);
                                                                if _e4154 {
                                                                    phi_5742_ = (_e4142 - 7u);
                                                                } else {
                                                                    phi_5742_ = (_e4142 - 3u);
                                                                }
                                                                let _e4158 = phi_5742_;
                                                                if _e4154 {
                                                                    phi_5757_ = vec3<f32>(((_e335.member_1.x * 11f) + -3f), ((_e335.member_1.y * 11f) + 1f), ((_e335.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5757_ = vec3<f32>((_e335.member_1.x * 7f), (_e335.member_1.y * 7f), (_e335.member_1.z * 7f));
                                                                }
                                                                let _e4171 = phi_5757_;
                                                                if (_e4158 < 3u) {
                                                                    if (_e4158 == 0u) {
                                                                        phi_24782_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_24782_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4158 == 1u)));
                                                                    }
                                                                    let _e4190 = phi_24782_;
                                                                    phi_5778_ = 4u;
                                                                    phi_5779_ = (_e4171 + _e4190);
                                                                } else {
                                                                    if _e4154 {
                                                                        phi_5777_ = (_e4171 + _e4144);
                                                                    } else {
                                                                        phi_5777_ = (_e4171 + vec3<f32>((_e4146.x * 1.1f), (_e4146.y * 1.1f), (_e4146.z * 1.1f)));
                                                                    }
                                                                    let _e4183 = phi_5777_;
                                                                    phi_5778_ = select(7u, 5u, _e4154);
                                                                    phi_5779_ = _e4183;
                                                                }
                                                                let _e4193 = phi_5778_;
                                                                let _e4195 = phi_5779_;
                                                                phi_5780_ = _e4193;
                                                                phi_5781_ = _e4195;
                                                            }
                                                            let _e4204 = phi_5780_;
                                                            let _e4206 = phi_5781_;
                                                            phi_5782_ = _e4204;
                                                            phi_5784_ = _e4206;
                                                        }
                                                        let _e4214 = phi_5782_;
                                                        let _e4216 = phi_5784_;
                                                        let _e4218 = select(vec3<f32>(_e4107, _e4110, _e4113), vec3<f32>(_e4097, _e4100, _e4103), vec3(_e4152));
                                                        let _e4220 = global_6.member[_e3782];
                                                        let _e4221 = bitcast<u32>(_e4220);
                                                        let _e4224 = global_6.member[(_e1293 + 11u)];
                                                        let _e4227 = global_6.member[(_e1293 + 12u)];
                                                        let _e4230 = global_6.member[(_e1293 + 13u)];
                                                        let _e4233 = global_6.member[(_e1293 + 14u)];
                                                        let _e4236 = global_6.member[(_e1293 + 15u)];
                                                        let _e4239 = global_6.member[(_e1293 + 16u)];
                                                        let _e4242 = global_6.member[(_e1293 + 17u)];
                                                        let _e4245 = global_6.member[(_e1293 + 18u)];
                                                        let _e4248 = global_6.member[(_e1293 + 19u)];
                                                        phi_5919_ = 0u;
                                                        phi_5922_ = 0f;
                                                        phi_5924_ = 1f;
                                                        phi_5926_ = 0f;
                                                        phi_5928_ = _e4216;
                                                        loop {
                                                            let _e4250 = phi_5919_;
                                                            let _e4252 = phi_5922_;
                                                            let _e4254 = phi_5924_;
                                                            let _e4256 = phi_5926_;
                                                            let _e4258 = phi_5928_;
                                                            local_23 = _e4252;
                                                            local_24 = _e4256;
                                                            local_25 = _e4252;
                                                            let _e4259 = (_e4250 < _e4214);
                                                            if _e4259 {
                                                                let _e4262 = (_e4258.x + _e4218.x);
                                                                let _e4265 = (_e4258.y + _e4218.y);
                                                                let _e4268 = (_e4258.z + _e4218.z);
                                                                let _e4269 = floor(_e4262);
                                                                let _e4270 = floor(_e4265);
                                                                let _e4271 = floor(_e4268);
                                                                let _e4280 = bitcast<u32>((select(0i, select(select(i32(_e4269), i32(-2147483648), (_e4269 < -2147483600f)), 2147483647i, (_e4269 > 2147483500f)), (_e4269 == _e4269)) & 255i));
                                                                let _e4289 = bitcast<u32>((select(0i, select(select(i32(_e4270), i32(-2147483648), (_e4270 < -2147483600f)), 2147483647i, (_e4270 > 2147483500f)), (_e4270 == _e4270)) & 255i));
                                                                let _e4298 = bitcast<u32>((select(0i, select(select(i32(_e4271), i32(-2147483648), (_e4271 < -2147483600f)), 2147483647i, (_e4271 > 2147483500f)), (_e4271 == _e4271)) & 255i));
                                                                let _e4299 = (_e4262 - _e4269);
                                                                let _e4300 = (_e4265 - _e4270);
                                                                let _e4301 = (_e4268 - _e4271);
                                                                let _e4308 = (((_e4299 * _e4299) * _e4299) * ((_e4299 * ((_e4299 * 6f) - 15f)) + 10f));
                                                                let _e4315 = (((_e4300 * _e4300) * _e4300) * ((_e4300 * ((_e4300 * 6f) - 15f)) + 10f));
                                                                let _e4325 = global_6.member[(_e4221 + _e4280)];
                                                                let _e4331 = (select(select(u32(_e4325), 0u, (_e4325 < 0f)), 4294967295u, (_e4325 > 4294967000f)) + _e4289);
                                                                let _e4334 = global_6.member[(_e4221 + _e4331)];
                                                                let _e4340 = (select(select(u32(_e4334), 0u, (_e4334 < 0f)), 4294967295u, (_e4334 > 4294967000f)) + _e4298);
                                                                let _e4344 = global_6.member[(_e4221 + (_e4331 + 1u))];
                                                                let _e4350 = (select(select(u32(_e4344), 0u, (_e4344 < 0f)), 4294967295u, (_e4344 > 4294967000f)) + _e4298);
                                                                let _e4354 = global_6.member[(_e4221 + (_e4280 + 1u))];
                                                                let _e4360 = (select(select(u32(_e4354), 0u, (_e4354 < 0f)), 4294967295u, (_e4354 > 4294967000f)) + _e4289);
                                                                let _e4363 = global_6.member[(_e4221 + _e4360)];
                                                                let _e4369 = (select(select(u32(_e4363), 0u, (_e4363 < 0f)), 4294967295u, (_e4363 > 4294967000f)) + _e4298);
                                                                let _e4373 = global_6.member[(_e4221 + (_e4360 + 1u))];
                                                                let _e4379 = (select(select(u32(_e4373), 0u, (_e4373 < 0f)), 4294967295u, (_e4373 > 4294967000f)) + _e4298);
                                                                let _e4382 = global_6.member[(_e4221 + _e4340)];
                                                                let _e4388 = (select(select(u32(_e4382), 0u, (_e4382 < 0f)), 4294967295u, (_e4382 > 4294967000f)) % 12u);
                                                                let _e4391 = select(-1f, 1f, ((_e4388 & 1u) == 0u));
                                                                let _e4394 = select(-1f, 1f, ((_e4388 & 2u) == 0u));
                                                                if (_e4388 < 4u) {
                                                                    phi_24843_ = ((_e4391 * _e4299) + (_e4394 * _e4300));
                                                                } else {
                                                                    if (_e4388 < 8u) {
                                                                        phi_24837_ = ((_e4391 * _e4299) + (_e4394 * _e4301));
                                                                    } else {
                                                                        phi_24837_ = ((_e4391 * _e4300) + (_e4394 * _e4301));
                                                                    }
                                                                    let _e4404 = phi_24837_;
                                                                    phi_24843_ = _e4404;
                                                                }
                                                                let _e4409 = phi_24843_;
                                                                let _e4412 = global_6.member[(_e4221 + _e4369)];
                                                                let _e4418 = (_e4299 - 1f);
                                                                let _e4419 = (select(select(u32(_e4412), 0u, (_e4412 < 0f)), 4294967295u, (_e4412 > 4294967000f)) % 12u);
                                                                let _e4422 = select(-1f, 1f, ((_e4419 & 1u) == 0u));
                                                                let _e4425 = select(-1f, 1f, ((_e4419 & 2u) == 0u));
                                                                if (_e4419 < 4u) {
                                                                    phi_24877_ = ((_e4422 * _e4418) + (_e4425 * _e4300));
                                                                } else {
                                                                    if (_e4419 < 8u) {
                                                                        phi_24871_ = ((_e4422 * _e4418) + (_e4425 * _e4301));
                                                                    } else {
                                                                        phi_24871_ = ((_e4422 * _e4300) + (_e4425 * _e4301));
                                                                    }
                                                                    let _e4435 = phi_24871_;
                                                                    phi_24877_ = _e4435;
                                                                }
                                                                let _e4440 = phi_24877_;
                                                                let _e4443 = global_6.member[(_e4221 + _e4350)];
                                                                let _e4449 = (_e4300 - 1f);
                                                                let _e4450 = (select(select(u32(_e4443), 0u, (_e4443 < 0f)), 4294967295u, (_e4443 > 4294967000f)) % 12u);
                                                                let _e4453 = select(-1f, 1f, ((_e4450 & 1u) == 0u));
                                                                let _e4456 = select(-1f, 1f, ((_e4450 & 2u) == 0u));
                                                                if (_e4450 < 4u) {
                                                                    phi_24911_ = ((_e4453 * _e4299) + (_e4456 * _e4449));
                                                                } else {
                                                                    if (_e4450 < 8u) {
                                                                        phi_24905_ = ((_e4453 * _e4299) + (_e4456 * _e4301));
                                                                    } else {
                                                                        phi_24905_ = ((_e4453 * _e4449) + (_e4456 * _e4301));
                                                                    }
                                                                    let _e4466 = phi_24905_;
                                                                    phi_24911_ = _e4466;
                                                                }
                                                                let _e4471 = phi_24911_;
                                                                let _e4474 = global_6.member[(_e4221 + _e4379)];
                                                                let _e4480 = (select(select(u32(_e4474), 0u, (_e4474 < 0f)), 4294967295u, (_e4474 > 4294967000f)) % 12u);
                                                                let _e4483 = select(-1f, 1f, ((_e4480 & 1u) == 0u));
                                                                let _e4486 = select(-1f, 1f, ((_e4480 & 2u) == 0u));
                                                                if (_e4480 < 4u) {
                                                                    phi_24945_ = ((_e4483 * _e4418) + (_e4486 * _e4449));
                                                                } else {
                                                                    if (_e4480 < 8u) {
                                                                        phi_24939_ = ((_e4483 * _e4418) + (_e4486 * _e4301));
                                                                    } else {
                                                                        phi_24939_ = ((_e4483 * _e4449) + (_e4486 * _e4301));
                                                                    }
                                                                    let _e4496 = phi_24939_;
                                                                    phi_24945_ = _e4496;
                                                                }
                                                                let _e4501 = phi_24945_;
                                                                let _e4505 = global_6.member[(_e4221 + (_e4340 + 1u))];
                                                                let _e4511 = (_e4301 - 1f);
                                                                let _e4512 = (select(select(u32(_e4505), 0u, (_e4505 < 0f)), 4294967295u, (_e4505 > 4294967000f)) % 12u);
                                                                let _e4515 = select(-1f, 1f, ((_e4512 & 1u) == 0u));
                                                                let _e4518 = select(-1f, 1f, ((_e4512 & 2u) == 0u));
                                                                if (_e4512 < 4u) {
                                                                    phi_24979_ = ((_e4515 * _e4299) + (_e4518 * _e4300));
                                                                } else {
                                                                    if (_e4512 < 8u) {
                                                                        phi_24973_ = ((_e4515 * _e4299) + (_e4518 * _e4511));
                                                                    } else {
                                                                        phi_24973_ = ((_e4515 * _e4300) + (_e4518 * _e4511));
                                                                    }
                                                                    let _e4528 = phi_24973_;
                                                                    phi_24979_ = _e4528;
                                                                }
                                                                let _e4533 = phi_24979_;
                                                                let _e4537 = global_6.member[(_e4221 + (_e4369 + 1u))];
                                                                let _e4543 = (select(select(u32(_e4537), 0u, (_e4537 < 0f)), 4294967295u, (_e4537 > 4294967000f)) % 12u);
                                                                let _e4546 = select(-1f, 1f, ((_e4543 & 1u) == 0u));
                                                                let _e4549 = select(-1f, 1f, ((_e4543 & 2u) == 0u));
                                                                if (_e4543 < 4u) {
                                                                    phi_25013_ = ((_e4546 * _e4418) + (_e4549 * _e4300));
                                                                } else {
                                                                    if (_e4543 < 8u) {
                                                                        phi_25007_ = ((_e4546 * _e4418) + (_e4549 * _e4511));
                                                                    } else {
                                                                        phi_25007_ = ((_e4546 * _e4300) + (_e4549 * _e4511));
                                                                    }
                                                                    let _e4559 = phi_25007_;
                                                                    phi_25013_ = _e4559;
                                                                }
                                                                let _e4564 = phi_25013_;
                                                                let _e4568 = global_6.member[(_e4221 + (_e4350 + 1u))];
                                                                let _e4574 = (select(select(u32(_e4568), 0u, (_e4568 < 0f)), 4294967295u, (_e4568 > 4294967000f)) % 12u);
                                                                let _e4577 = select(-1f, 1f, ((_e4574 & 1u) == 0u));
                                                                let _e4580 = select(-1f, 1f, ((_e4574 & 2u) == 0u));
                                                                if (_e4574 < 4u) {
                                                                    phi_25047_ = ((_e4577 * _e4299) + (_e4580 * _e4449));
                                                                } else {
                                                                    if (_e4574 < 8u) {
                                                                        phi_25041_ = ((_e4577 * _e4299) + (_e4580 * _e4511));
                                                                    } else {
                                                                        phi_25041_ = ((_e4577 * _e4449) + (_e4580 * _e4511));
                                                                    }
                                                                    let _e4590 = phi_25041_;
                                                                    phi_25047_ = _e4590;
                                                                }
                                                                let _e4595 = phi_25047_;
                                                                let _e4599 = global_6.member[(_e4221 + (_e4379 + 1u))];
                                                                let _e4605 = (select(select(u32(_e4599), 0u, (_e4599 < 0f)), 4294967295u, (_e4599 > 4294967000f)) % 12u);
                                                                let _e4608 = select(-1f, 1f, ((_e4605 & 1u) == 0u));
                                                                let _e4611 = select(-1f, 1f, ((_e4605 & 2u) == 0u));
                                                                if (_e4605 < 4u) {
                                                                    phi_25081_ = ((_e4608 * _e4418) + (_e4611 * _e4449));
                                                                } else {
                                                                    if (_e4605 < 8u) {
                                                                        phi_25075_ = ((_e4608 * _e4418) + (_e4611 * _e4511));
                                                                    } else {
                                                                        phi_25075_ = ((_e4608 * _e4449) + (_e4611 * _e4511));
                                                                    }
                                                                    let _e4621 = phi_25075_;
                                                                    phi_25081_ = _e4621;
                                                                }
                                                                let _e4626 = phi_25081_;
                                                                let _e4629 = (_e4409 + (_e4308 * (_e4440 - _e4409)));
                                                                let _e4635 = (_e4533 + (_e4308 * (_e4564 - _e4533)));
                                                                let _e4641 = (_e4629 + (_e4315 * ((_e4471 + (_e4308 * (_e4501 - _e4471))) - _e4629)));
                                                                phi_5920_ = (_e4250 + 1u);
                                                                phi_5923_ = (_e4252 + _e4254);
                                                                phi_5925_ = (_e4254 * 0.5f);
                                                                phi_5927_ = (_e4256 + ((_e4641 + ((((_e4301 * _e4301) * _e4301) * ((_e4301 * ((_e4301 * 6f) - 15f)) + 10f)) * ((_e4635 + (_e4315 * ((_e4595 + (_e4308 * (_e4626 - _e4595))) - _e4635))) - _e4641))) * _e4254));
                                                                phi_5929_ = vec3<f32>(((((_e4224 * _e4258.x) + (_e4227 * _e4258.y)) + (_e4230 * _e4258.z)) * 2.03f), ((((_e4233 * _e4258.x) + (_e4236 * _e4258.y)) + (_e4239 * _e4258.z)) * 2.03f), ((((_e4242 * _e4258.x) + (_e4245 * _e4258.y)) + (_e4248 * _e4258.z)) * 2.03f));
                                                            } else {
                                                                phi_5920_ = u32();
                                                                phi_5923_ = f32();
                                                                phi_5925_ = f32();
                                                                phi_5927_ = f32();
                                                                phi_5929_ = vec3<f32>();
                                                            }
                                                            let _e4673 = phi_5920_;
                                                            let _e4675 = phi_5923_;
                                                            let _e4677 = phi_5925_;
                                                            let _e4679 = phi_5927_;
                                                            let _e4681 = phi_5929_;
                                                            phi_5919_ = _e4673;
                                                            phi_5922_ = _e4675;
                                                            phi_5924_ = _e4677;
                                                            phi_5926_ = _e4679;
                                                            phi_5928_ = _e4681;
                                                            if !(_e4259) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4684 = local_23;
                                                        if (_e4684 > 0f) {
                                                            let _e4687 = local_24;
                                                            let _e4689 = local_25;
                                                            phi_6345_ = (_e4687 / _e4689);
                                                        } else {
                                                            phi_6345_ = 0f;
                                                        }
                                                        let _e4692 = phi_6345_;
                                                        if (_e4142 == 0u) {
                                                            phi_6428_ = _e4138;
                                                            phi_6429_ = _e4140;
                                                            phi_6430_ = _e4144;
                                                            phi_6431_ = _e4146;
                                                            phi_6432_ = _e4148;
                                                            phi_6433_ = (_e4150 + (1.5f * _e4692));
                                                        } else {
                                                            if (_e4142 == 1u) {
                                                                phi_6422_ = _e4138;
                                                                phi_6423_ = _e4140;
                                                                phi_6424_ = _e4144;
                                                                phi_6425_ = _e4146;
                                                                phi_6426_ = _e4148;
                                                                phi_6427_ = (_e4150 + (0.8f * _e4692));
                                                            } else {
                                                                let _e4695 = (_e4142 == 2u);
                                                                if _e4695 {
                                                                    phi_6417_ = _e4138;
                                                                    phi_6418_ = _e4140;
                                                                    phi_6419_ = _e4144;
                                                                    phi_6420_ = _e4146;
                                                                } else {
                                                                    if (_e4142 == 3u) {
                                                                        phi_6413_ = _e4138;
                                                                        phi_6414_ = _e4140;
                                                                        phi_6415_ = _e4144;
                                                                        phi_6416_ = vec3<f32>(_e4692, _e4146.y, _e4146.z);
                                                                    } else {
                                                                        if (_e4142 == 4u) {
                                                                            phi_6409_ = _e4138;
                                                                            phi_6410_ = _e4140;
                                                                            phi_6411_ = _e4144;
                                                                            phi_6412_ = vec3<f32>(_e4146.x, _e4692, _e4146.z);
                                                                        } else {
                                                                            if (_e4142 == 5u) {
                                                                                phi_6405_ = _e4138;
                                                                                phi_6406_ = _e4140;
                                                                                phi_6407_ = _e4144;
                                                                                phi_6408_ = vec3<f32>(_e4146.x, _e4146.y, _e4692);
                                                                            } else {
                                                                                let _e4699 = (_e4142 == 6u);
                                                                                if _e4699 {
                                                                                    phi_6402_ = _e4138;
                                                                                    phi_6404_ = _e4144;
                                                                                } else {
                                                                                    if (_e4142 == 7u) {
                                                                                        phi_6400_ = _e4138;
                                                                                        phi_6401_ = vec3<f32>(_e4692, _e4144.y, _e4144.z);
                                                                                    } else {
                                                                                        if (_e4142 == 8u) {
                                                                                            phi_6398_ = _e4138;
                                                                                            phi_6399_ = vec3<f32>(_e4144.x, _e4692, _e4144.z);
                                                                                        } else {
                                                                                            let _e4702 = (_e4142 == 9u);
                                                                                            if _e4702 {
                                                                                                phi_6397_ = vec3<f32>(_e4144.x, _e4144.y, _e4692);
                                                                                            } else {
                                                                                                phi_6397_ = _e4144;
                                                                                            }
                                                                                            let _e4708 = phi_6397_;
                                                                                            phi_6398_ = select(_e4692, _e4138, _e4702);
                                                                                            phi_6399_ = _e4708;
                                                                                        }
                                                                                        let _e4715 = phi_6398_;
                                                                                        let _e4717 = phi_6399_;
                                                                                        phi_6400_ = _e4715;
                                                                                        phi_6401_ = _e4717;
                                                                                    }
                                                                                    let _e4723 = phi_6400_;
                                                                                    let _e4725 = phi_6401_;
                                                                                    phi_6402_ = _e4723;
                                                                                    phi_6404_ = _e4725;
                                                                                }
                                                                                let _e4727 = phi_6402_;
                                                                                let _e4729 = phi_6404_;
                                                                                phi_6405_ = _e4727;
                                                                                phi_6406_ = select(_e4140, _e4692, _e4699);
                                                                                phi_6407_ = _e4729;
                                                                                phi_6408_ = _e4146;
                                                                            }
                                                                            let _e4736 = phi_6405_;
                                                                            let _e4738 = phi_6406_;
                                                                            let _e4740 = phi_6407_;
                                                                            let _e4742 = phi_6408_;
                                                                            phi_6409_ = _e4736;
                                                                            phi_6410_ = _e4738;
                                                                            phi_6411_ = _e4740;
                                                                            phi_6412_ = _e4742;
                                                                        }
                                                                        let _e4748 = phi_6409_;
                                                                        let _e4750 = phi_6410_;
                                                                        let _e4752 = phi_6411_;
                                                                        let _e4754 = phi_6412_;
                                                                        phi_6413_ = _e4748;
                                                                        phi_6414_ = _e4750;
                                                                        phi_6415_ = _e4752;
                                                                        phi_6416_ = _e4754;
                                                                    }
                                                                    let _e4760 = phi_6413_;
                                                                    let _e4762 = phi_6414_;
                                                                    let _e4764 = phi_6415_;
                                                                    let _e4766 = phi_6416_;
                                                                    phi_6417_ = _e4760;
                                                                    phi_6418_ = _e4762;
                                                                    phi_6419_ = _e4764;
                                                                    phi_6420_ = _e4766;
                                                                }
                                                                let _e4768 = phi_6417_;
                                                                let _e4770 = phi_6418_;
                                                                let _e4772 = phi_6419_;
                                                                let _e4774 = phi_6420_;
                                                                phi_6422_ = _e4768;
                                                                phi_6423_ = _e4770;
                                                                phi_6424_ = _e4772;
                                                                phi_6425_ = _e4774;
                                                                phi_6426_ = select(_e4148, _e4692, _e4695);
                                                                phi_6427_ = _e4150;
                                                            }
                                                            let _e4779 = phi_6422_;
                                                            let _e4781 = phi_6423_;
                                                            let _e4783 = phi_6424_;
                                                            let _e4785 = phi_6425_;
                                                            let _e4787 = phi_6426_;
                                                            let _e4789 = phi_6427_;
                                                            phi_6428_ = _e4779;
                                                            phi_6429_ = _e4781;
                                                            phi_6430_ = _e4783;
                                                            phi_6431_ = _e4785;
                                                            phi_6432_ = _e4787;
                                                            phi_6433_ = _e4789;
                                                        }
                                                        let _e4793 = phi_6428_;
                                                        let _e4795 = phi_6429_;
                                                        let _e4797 = phi_6430_;
                                                        let _e4799 = phi_6431_;
                                                        let _e4801 = phi_6432_;
                                                        let _e4803 = phi_6433_;
                                                        phi_5692_ = _e4793;
                                                        phi_5695_ = _e4795;
                                                        phi_5697_ = (_e4142 + 1u);
                                                        phi_5699_ = _e4797;
                                                        phi_5701_ = _e4799;
                                                        phi_5703_ = _e4801;
                                                        phi_5705_ = _e4803;
                                                    } else {
                                                        phi_5692_ = f32();
                                                        phi_5695_ = f32();
                                                        phi_5697_ = u32();
                                                        phi_5699_ = vec3<f32>();
                                                        phi_5701_ = vec3<f32>();
                                                        phi_5703_ = f32();
                                                        phi_5705_ = f32();
                                                    }
                                                    let _e4806 = phi_5692_;
                                                    let _e4808 = phi_5695_;
                                                    let _e4810 = phi_5697_;
                                                    let _e4812 = phi_5699_;
                                                    let _e4814 = phi_5701_;
                                                    let _e4816 = phi_5703_;
                                                    let _e4818 = phi_5705_;
                                                    phi_5691_ = _e4806;
                                                    phi_5694_ = _e4808;
                                                    phi_5696_ = _e4810;
                                                    phi_5698_ = _e4812;
                                                    phi_5700_ = _e4814;
                                                    phi_5702_ = _e4816;
                                                    phi_5704_ = _e4818;
                                                    if !(_e4151) {
                                                        break;
                                                    }
                                                }
                                                if _e4115 {
                                                    let _e4833 = (exp((-(abs(_e3842)) / (0.06f + (abs(_e3853) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3853 * -0.71428573f) * (_e3853 * 0.71428573f))))));
                                                    let _e4835 = (_e3842 * 1.3333334f);
                                                    let _e4840 = (1.3f * exp((((_e3853 * _e3853) + (_e4835 * _e4835)) * -12.499999f)));
                                                    let _e4851 = local_26;
                                                    let _e4853 = local_27;
                                                    if (_e4851 != _e4853) {
                                                        phi_25157_ = true;
                                                    } else {
                                                        let _e4856 = local_28;
                                                        phi_25157_ = (0.1f >= _e4856);
                                                    }
                                                    let _e4859 = phi_25157_;
                                                    let _e4861 = local_29;
                                                    let _e4864 = (pow(select(_e4861, 0.1f, _e4859), 1.3f) * _e4073);
                                                    phi_6465_ = vec3<f32>((_e4070.x + (((_e4079 * _e4833) + (_e4088 * _e4840)) * _e4864)), (_e4070.y + (((_e4082 * _e4833) + (_e4091 * _e4840)) * _e4864)), (_e4070.z + (((_e4085 * _e4833) + (_e4094 * _e4840)) * _e4864)));
                                                } else {
                                                    phi_6465_ = _e4070;
                                                }
                                                let _e4876 = phi_6465_;
                                                if select(_e4125, _e4129, _e4131) {
                                                    let _e4878 = local_30;
                                                    let _e4884 = ((_e3842 - (0.025f * _e4878)) / (0.045f + (abs(_e3853) * 0.0095492955f)));
                                                    let _e4889 = local_31;
                                                    let _e4890 = (_e4889 * 3.3333333f);
                                                    let _e4892 = select(_e4890, 0f, (_e4890 < 0f));
                                                    let _e4894 = select(_e4892, 1f, (_e4892 > 1f));
                                                    let _e4910 = local_32;
                                                    let _e4912 = ((_e4910 - 0.25f) * 5.0000005f);
                                                    let _e4914 = select(_e4912, 0f, (_e4912 < 0f));
                                                    let _e4916 = select(_e4914, 1f, (_e4914 > 1f));
                                                    phi_6544_ = (_e4076 * (((exp((-(_e4884) * _e4884)) * (0.15f + (3f * ((_e4894 * _e4894) * (3f - (2f * _e4894)))))) * (0.5f + (0.9f * exp(((_e3853 * -0.6666667f) * (_e3853 * 0.6666667f)))))) + (1.2f * (exp(((_e3842 * -3.3333333f) * (_e3842 * 3.3333333f))) * ((_e4916 * _e4916) * (3f - (2f * _e4916)))))));
                                                } else {
                                                    phi_6544_ = 0f;
                                                }
                                                let _e4930 = phi_6544_;
                                                phi_6545_ = _e4876;
                                                phi_6546_ = _e4930;
                                            } else {
                                                phi_6545_ = _e4070;
                                                phi_6546_ = 0f;
                                            }
                                            let _e4932 = phi_6545_;
                                            let _e4934 = phi_6546_;
                                            phi_6548_ = _e4932;
                                            phi_6551_ = 0u;
                                            loop {
                                                let _e4936 = phi_6548_;
                                                let _e4938 = phi_6551_;
                                                local_49 = _e4936;
                                                local_50 = _e4936;
                                                local_51 = _e4936;
                                                let _e4939 = (_e4938 < bitcast<u32>(_e3805));
                                                if _e4939 {
                                                    let _e4941 = (bitcast<u32>(_e3801) + (_e4938 * 29u));
                                                    let _e4943 = global_6.member[_e4941];
                                                    let _e4944 = (_e4943 > 0.5f);
                                                    let _e4947 = global_6.member[(_e4941 + 1u)];
                                                    let _e4950 = global_6.member[(_e4941 + 2u)];
                                                    let _e4953 = global_6.member[(_e4941 + 3u)];
                                                    let _e4956 = global_6.member[(_e4941 + 4u)];
                                                    let _e4959 = global_6.member[(_e4941 + 5u)];
                                                    let _e4962 = global_6.member[(_e4941 + 6u)];
                                                    let _e4967 = (((_e335.member_1.x * _e4947) + (_e335.member_1.y * _e4950)) + (_e335.member_1.z * _e4953));
                                                    let _e4968 = (_e4967 < _e4959);
                                                    if _e4968 {
                                                        phi_7697_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7698_ = vec3<f32>();
                                                    } else {
                                                        let _e4971 = global_6.member[(_e4941 + 7u)];
                                                        let _e4974 = global_6.member[(_e4941 + 8u)];
                                                        let _e4977 = global_6.member[(_e4941 + 9u)];
                                                        let _e4980 = global_6.member[(_e4941 + 10u)];
                                                        let _e4983 = global_6.member[(_e4941 + 11u)];
                                                        let _e4986 = global_6.member[(_e4941 + 12u)];
                                                        let _e4989 = global_6.member[(_e4941 + 13u)];
                                                        let _e4992 = global_6.member[(_e4941 + 14u)];
                                                        let _e4995 = global_6.member[(_e4941 + 15u)];
                                                        let _e4998 = global_6.member[(_e4941 + 16u)];
                                                        let _e5001 = global_6.member[(_e4941 + 17u)];
                                                        let _e5004 = global_6.member[(_e4941 + 18u)];
                                                        let _e5007 = global_6.member[(_e4941 + 19u)];
                                                        let _e5010 = global_6.member[(_e4941 + 20u)];
                                                        let _e5013 = global_6.member[(_e4941 + 21u)];
                                                        let _e5016 = global_6.member[(_e4941 + 22u)];
                                                        let _e5019 = global_6.member[(_e4941 + 23u)];
                                                        let _e5022 = global_6.member[(_e4941 + 24u)];
                                                        let _e5025 = global_6.member[(_e4941 + 25u)];
                                                        let _e5028 = global_6.member[(_e4941 + 26u)];
                                                        let _e5031 = global_6.member[(_e4941 + 27u)];
                                                        let _e5034 = global_6.member[(_e4941 + 28u)];
                                                        let _e5041 = (((_e335.member_1.x / _e4967) - _e4947) / _e4956);
                                                        let _e5042 = (((_e335.member_1.y / _e4967) - _e4950) / _e4956);
                                                        let _e5043 = (((_e335.member_1.z / _e4967) - _e4953) / _e4956);
                                                        if _e4944 {
                                                            let _e5648 = (((_e5041 * _e4998) + (_e5042 * _e5001)) + (_e5043 * _e5004));
                                                            let _e5654 = ((((_e5041 * _e5007) + (_e5042 * _e5010)) + (_e5043 * _e5013)) / _e5016);
                                                            phi_7695_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4962 * exp((-2.5f * pow((sqrt(((_e5648 * _e5648) + (_e5654 * _e5654))) * 3.3333333f), 0.7f)))));
                                                            phi_7696_ = vec3<f32>();
                                                        } else {
                                                            let _e5048 = (((_e4971 * _e4947) + (_e4974 * _e4950)) + (_e4977 * _e4953));
                                                            if (abs(_e5048) < 0.12f) {
                                                                phi_6999_ = select(0.12f, -0.12f, (_e5048 < 0f));
                                                            } else {
                                                                phi_6999_ = _e5048;
                                                            }
                                                            let _e5054 = phi_6999_;
                                                            let _e5060 = ((((_e4971 * _e5041) + (_e4974 * _e5042)) + (_e4977 * _e5043)) / _e5054);
                                                            let _e5064 = (_e5041 - (_e4947 * _e5060));
                                                            let _e5065 = (_e5042 - (_e4950 * _e5060));
                                                            let _e5066 = (_e5043 - (_e4953 * _e5060));
                                                            let _e5071 = (((_e5064 * _e4980) + (_e5065 * _e4983)) + (_e5066 * _e4986));
                                                            let _e5076 = (((_e5064 * _e4989) + (_e5065 * _e4992)) + (_e5066 * _e4995));
                                                            let _e5080 = sqrt(((_e5071 * _e5071) + (_e5076 * _e5076)));
                                                            let _e5092 = abs(_e5054);
                                                            let _e5094 = (pow((0.5f + (0.5f * cos(((_e5019 * (atan2(_e5076, _e5071) - (log((_e5080 + 0.03f)) / _e5022))) + _e5025)))), 3f) * sqrt(_e5092));
                                                            let _e5099 = global_6.member[_e3782];
                                                            let _e5100 = bitcast<u32>(_e5099);
                                                            let _e5103 = global_6.member[(_e1293 + 11u)];
                                                            let _e5106 = global_6.member[(_e1293 + 12u)];
                                                            let _e5109 = global_6.member[(_e1293 + 13u)];
                                                            let _e5112 = global_6.member[(_e1293 + 14u)];
                                                            let _e5115 = global_6.member[(_e1293 + 15u)];
                                                            let _e5118 = global_6.member[(_e1293 + 16u)];
                                                            let _e5121 = global_6.member[(_e1293 + 17u)];
                                                            let _e5124 = global_6.member[(_e1293 + 18u)];
                                                            let _e5127 = global_6.member[(_e1293 + 19u)];
                                                            phi_7179_ = 0u;
                                                            phi_7182_ = 0f;
                                                            phi_7184_ = 1f;
                                                            phi_7186_ = 0f;
                                                            phi_7188_ = vec3<f32>((_e5071 * 9f), (_e5076 * 9f), 0f);
                                                            loop {
                                                                let _e5129 = phi_7179_;
                                                                let _e5131 = phi_7182_;
                                                                let _e5133 = phi_7184_;
                                                                let _e5135 = phi_7186_;
                                                                let _e5137 = phi_7188_;
                                                                local_33 = _e5131;
                                                                local_34 = _e5135;
                                                                local_35 = _e5131;
                                                                let _e5138 = (_e5129 < 4u);
                                                                if _e5138 {
                                                                    let _e5140 = (_e5137.x + _e5028);
                                                                    let _e5142 = (_e5137.y + _e5031);
                                                                    let _e5144 = (_e5137.z + _e5034);
                                                                    let _e5145 = floor(_e5140);
                                                                    let _e5146 = floor(_e5142);
                                                                    let _e5147 = floor(_e5144);
                                                                    let _e5156 = bitcast<u32>((select(0i, select(select(i32(_e5145), i32(-2147483648), (_e5145 < -2147483600f)), 2147483647i, (_e5145 > 2147483500f)), (_e5145 == _e5145)) & 255i));
                                                                    let _e5165 = bitcast<u32>((select(0i, select(select(i32(_e5146), i32(-2147483648), (_e5146 < -2147483600f)), 2147483647i, (_e5146 > 2147483500f)), (_e5146 == _e5146)) & 255i));
                                                                    let _e5174 = bitcast<u32>((select(0i, select(select(i32(_e5147), i32(-2147483648), (_e5147 < -2147483600f)), 2147483647i, (_e5147 > 2147483500f)), (_e5147 == _e5147)) & 255i));
                                                                    let _e5175 = (_e5140 - _e5145);
                                                                    let _e5176 = (_e5142 - _e5146);
                                                                    let _e5177 = (_e5144 - _e5147);
                                                                    let _e5184 = (((_e5175 * _e5175) * _e5175) * ((_e5175 * ((_e5175 * 6f) - 15f)) + 10f));
                                                                    let _e5191 = (((_e5176 * _e5176) * _e5176) * ((_e5176 * ((_e5176 * 6f) - 15f)) + 10f));
                                                                    let _e5201 = global_6.member[(_e5100 + _e5156)];
                                                                    let _e5207 = (select(select(u32(_e5201), 0u, (_e5201 < 0f)), 4294967295u, (_e5201 > 4294967000f)) + _e5165);
                                                                    let _e5210 = global_6.member[(_e5100 + _e5207)];
                                                                    let _e5216 = (select(select(u32(_e5210), 0u, (_e5210 < 0f)), 4294967295u, (_e5210 > 4294967000f)) + _e5174);
                                                                    let _e5220 = global_6.member[(_e5100 + (_e5207 + 1u))];
                                                                    let _e5226 = (select(select(u32(_e5220), 0u, (_e5220 < 0f)), 4294967295u, (_e5220 > 4294967000f)) + _e5174);
                                                                    let _e5230 = global_6.member[(_e5100 + (_e5156 + 1u))];
                                                                    let _e5236 = (select(select(u32(_e5230), 0u, (_e5230 < 0f)), 4294967295u, (_e5230 > 4294967000f)) + _e5165);
                                                                    let _e5239 = global_6.member[(_e5100 + _e5236)];
                                                                    let _e5245 = (select(select(u32(_e5239), 0u, (_e5239 < 0f)), 4294967295u, (_e5239 > 4294967000f)) + _e5174);
                                                                    let _e5249 = global_6.member[(_e5100 + (_e5236 + 1u))];
                                                                    let _e5255 = (select(select(u32(_e5249), 0u, (_e5249 < 0f)), 4294967295u, (_e5249 > 4294967000f)) + _e5174);
                                                                    let _e5258 = global_6.member[(_e5100 + _e5216)];
                                                                    let _e5264 = (select(select(u32(_e5258), 0u, (_e5258 < 0f)), 4294967295u, (_e5258 > 4294967000f)) % 12u);
                                                                    let _e5267 = select(-1f, 1f, ((_e5264 & 1u) == 0u));
                                                                    let _e5270 = select(-1f, 1f, ((_e5264 & 2u) == 0u));
                                                                    if (_e5264 < 4u) {
                                                                        phi_25252_ = ((_e5267 * _e5175) + (_e5270 * _e5176));
                                                                    } else {
                                                                        if (_e5264 < 8u) {
                                                                            phi_25246_ = ((_e5267 * _e5175) + (_e5270 * _e5177));
                                                                        } else {
                                                                            phi_25246_ = ((_e5267 * _e5176) + (_e5270 * _e5177));
                                                                        }
                                                                        let _e5280 = phi_25246_;
                                                                        phi_25252_ = _e5280;
                                                                    }
                                                                    let _e5285 = phi_25252_;
                                                                    let _e5288 = global_6.member[(_e5100 + _e5245)];
                                                                    let _e5294 = (_e5175 - 1f);
                                                                    let _e5295 = (select(select(u32(_e5288), 0u, (_e5288 < 0f)), 4294967295u, (_e5288 > 4294967000f)) % 12u);
                                                                    let _e5298 = select(-1f, 1f, ((_e5295 & 1u) == 0u));
                                                                    let _e5301 = select(-1f, 1f, ((_e5295 & 2u) == 0u));
                                                                    if (_e5295 < 4u) {
                                                                        phi_25286_ = ((_e5298 * _e5294) + (_e5301 * _e5176));
                                                                    } else {
                                                                        if (_e5295 < 8u) {
                                                                            phi_25280_ = ((_e5298 * _e5294) + (_e5301 * _e5177));
                                                                        } else {
                                                                            phi_25280_ = ((_e5298 * _e5176) + (_e5301 * _e5177));
                                                                        }
                                                                        let _e5311 = phi_25280_;
                                                                        phi_25286_ = _e5311;
                                                                    }
                                                                    let _e5316 = phi_25286_;
                                                                    let _e5319 = global_6.member[(_e5100 + _e5226)];
                                                                    let _e5325 = (_e5176 - 1f);
                                                                    let _e5326 = (select(select(u32(_e5319), 0u, (_e5319 < 0f)), 4294967295u, (_e5319 > 4294967000f)) % 12u);
                                                                    let _e5329 = select(-1f, 1f, ((_e5326 & 1u) == 0u));
                                                                    let _e5332 = select(-1f, 1f, ((_e5326 & 2u) == 0u));
                                                                    if (_e5326 < 4u) {
                                                                        phi_25320_ = ((_e5329 * _e5175) + (_e5332 * _e5325));
                                                                    } else {
                                                                        if (_e5326 < 8u) {
                                                                            phi_25314_ = ((_e5329 * _e5175) + (_e5332 * _e5177));
                                                                        } else {
                                                                            phi_25314_ = ((_e5329 * _e5325) + (_e5332 * _e5177));
                                                                        }
                                                                        let _e5342 = phi_25314_;
                                                                        phi_25320_ = _e5342;
                                                                    }
                                                                    let _e5347 = phi_25320_;
                                                                    let _e5350 = global_6.member[(_e5100 + _e5255)];
                                                                    let _e5356 = (select(select(u32(_e5350), 0u, (_e5350 < 0f)), 4294967295u, (_e5350 > 4294967000f)) % 12u);
                                                                    let _e5359 = select(-1f, 1f, ((_e5356 & 1u) == 0u));
                                                                    let _e5362 = select(-1f, 1f, ((_e5356 & 2u) == 0u));
                                                                    if (_e5356 < 4u) {
                                                                        phi_25354_ = ((_e5359 * _e5294) + (_e5362 * _e5325));
                                                                    } else {
                                                                        if (_e5356 < 8u) {
                                                                            phi_25348_ = ((_e5359 * _e5294) + (_e5362 * _e5177));
                                                                        } else {
                                                                            phi_25348_ = ((_e5359 * _e5325) + (_e5362 * _e5177));
                                                                        }
                                                                        let _e5372 = phi_25348_;
                                                                        phi_25354_ = _e5372;
                                                                    }
                                                                    let _e5377 = phi_25354_;
                                                                    let _e5381 = global_6.member[(_e5100 + (_e5216 + 1u))];
                                                                    let _e5387 = (_e5177 - 1f);
                                                                    let _e5388 = (select(select(u32(_e5381), 0u, (_e5381 < 0f)), 4294967295u, (_e5381 > 4294967000f)) % 12u);
                                                                    let _e5391 = select(-1f, 1f, ((_e5388 & 1u) == 0u));
                                                                    let _e5394 = select(-1f, 1f, ((_e5388 & 2u) == 0u));
                                                                    if (_e5388 < 4u) {
                                                                        phi_25388_ = ((_e5391 * _e5175) + (_e5394 * _e5176));
                                                                    } else {
                                                                        if (_e5388 < 8u) {
                                                                            phi_25382_ = ((_e5391 * _e5175) + (_e5394 * _e5387));
                                                                        } else {
                                                                            phi_25382_ = ((_e5391 * _e5176) + (_e5394 * _e5387));
                                                                        }
                                                                        let _e5404 = phi_25382_;
                                                                        phi_25388_ = _e5404;
                                                                    }
                                                                    let _e5409 = phi_25388_;
                                                                    let _e5413 = global_6.member[(_e5100 + (_e5245 + 1u))];
                                                                    let _e5419 = (select(select(u32(_e5413), 0u, (_e5413 < 0f)), 4294967295u, (_e5413 > 4294967000f)) % 12u);
                                                                    let _e5422 = select(-1f, 1f, ((_e5419 & 1u) == 0u));
                                                                    let _e5425 = select(-1f, 1f, ((_e5419 & 2u) == 0u));
                                                                    if (_e5419 < 4u) {
                                                                        phi_25422_ = ((_e5422 * _e5294) + (_e5425 * _e5176));
                                                                    } else {
                                                                        if (_e5419 < 8u) {
                                                                            phi_25416_ = ((_e5422 * _e5294) + (_e5425 * _e5387));
                                                                        } else {
                                                                            phi_25416_ = ((_e5422 * _e5176) + (_e5425 * _e5387));
                                                                        }
                                                                        let _e5435 = phi_25416_;
                                                                        phi_25422_ = _e5435;
                                                                    }
                                                                    let _e5440 = phi_25422_;
                                                                    let _e5444 = global_6.member[(_e5100 + (_e5226 + 1u))];
                                                                    let _e5450 = (select(select(u32(_e5444), 0u, (_e5444 < 0f)), 4294967295u, (_e5444 > 4294967000f)) % 12u);
                                                                    let _e5453 = select(-1f, 1f, ((_e5450 & 1u) == 0u));
                                                                    let _e5456 = select(-1f, 1f, ((_e5450 & 2u) == 0u));
                                                                    if (_e5450 < 4u) {
                                                                        phi_25456_ = ((_e5453 * _e5175) + (_e5456 * _e5325));
                                                                    } else {
                                                                        if (_e5450 < 8u) {
                                                                            phi_25450_ = ((_e5453 * _e5175) + (_e5456 * _e5387));
                                                                        } else {
                                                                            phi_25450_ = ((_e5453 * _e5325) + (_e5456 * _e5387));
                                                                        }
                                                                        let _e5466 = phi_25450_;
                                                                        phi_25456_ = _e5466;
                                                                    }
                                                                    let _e5471 = phi_25456_;
                                                                    let _e5475 = global_6.member[(_e5100 + (_e5255 + 1u))];
                                                                    let _e5481 = (select(select(u32(_e5475), 0u, (_e5475 < 0f)), 4294967295u, (_e5475 > 4294967000f)) % 12u);
                                                                    let _e5484 = select(-1f, 1f, ((_e5481 & 1u) == 0u));
                                                                    let _e5487 = select(-1f, 1f, ((_e5481 & 2u) == 0u));
                                                                    if (_e5481 < 4u) {
                                                                        phi_25490_ = ((_e5484 * _e5294) + (_e5487 * _e5325));
                                                                    } else {
                                                                        if (_e5481 < 8u) {
                                                                            phi_25484_ = ((_e5484 * _e5294) + (_e5487 * _e5387));
                                                                        } else {
                                                                            phi_25484_ = ((_e5484 * _e5325) + (_e5487 * _e5387));
                                                                        }
                                                                        let _e5497 = phi_25484_;
                                                                        phi_25490_ = _e5497;
                                                                    }
                                                                    let _e5502 = phi_25490_;
                                                                    let _e5505 = (_e5285 + (_e5184 * (_e5316 - _e5285)));
                                                                    let _e5511 = (_e5409 + (_e5184 * (_e5440 - _e5409)));
                                                                    let _e5517 = (_e5505 + (_e5191 * ((_e5347 + (_e5184 * (_e5377 - _e5347))) - _e5505)));
                                                                    phi_7180_ = (_e5129 + 1u);
                                                                    phi_7183_ = (_e5131 + _e5133);
                                                                    phi_7185_ = (_e5133 * 0.5f);
                                                                    phi_7187_ = (_e5135 + ((_e5517 + ((((_e5177 * _e5177) * _e5177) * ((_e5177 * ((_e5177 * 6f) - 15f)) + 10f)) * ((_e5511 + (_e5191 * ((_e5471 + (_e5184 * (_e5502 - _e5471))) - _e5511))) - _e5517))) * _e5133));
                                                                    phi_7189_ = vec3<f32>(((((_e5103 * _e5137.x) + (_e5106 * _e5137.y)) + (_e5109 * _e5137.z)) * 2.03f), ((((_e5112 * _e5137.x) + (_e5115 * _e5137.y)) + (_e5118 * _e5137.z)) * 2.03f), ((((_e5121 * _e5137.x) + (_e5124 * _e5137.y)) + (_e5127 * _e5137.z)) * 2.03f));
                                                                } else {
                                                                    phi_7180_ = u32();
                                                                    phi_7183_ = f32();
                                                                    phi_7185_ = f32();
                                                                    phi_7187_ = f32();
                                                                    phi_7189_ = vec3<f32>();
                                                                }
                                                                let _e5549 = phi_7180_;
                                                                let _e5551 = phi_7183_;
                                                                let _e5553 = phi_7185_;
                                                                let _e5555 = phi_7187_;
                                                                let _e5557 = phi_7189_;
                                                                phi_7179_ = _e5549;
                                                                phi_7182_ = _e5551;
                                                                phi_7184_ = _e5553;
                                                                phi_7186_ = _e5555;
                                                                phi_7188_ = _e5557;
                                                                if !(_e5138) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5560 = local_33;
                                                            if (_e5560 > 0f) {
                                                                let _e5563 = local_34;
                                                                let _e5565 = local_35;
                                                                phi_7602_ = (_e5563 / _e5565);
                                                            } else {
                                                                phi_7602_ = 0f;
                                                            }
                                                            let _e5568 = phi_7602_;
                                                            let _e5570 = (1f + (1.2f * _e5568));
                                                            if (_e5570 != _e5570) {
                                                                phi_25536_ = true;
                                                            } else {
                                                                phi_25536_ = (0f >= _e5570);
                                                            }
                                                            let _e5574 = phi_25536_;
                                                            let _e5579 = ((_e5080 - 1.15f) * -2.8571432f);
                                                            let _e5581 = select(_e5579, 0f, (_e5579 < 0f));
                                                            let _e5583 = select(_e5581, 1f, (_e5581 > 1f));
                                                            let _e5589 = (1f / _e5092);
                                                            if (_e5589 != _e5589) {
                                                                phi_25551_ = true;
                                                            } else {
                                                                phi_25551_ = (4f <= _e5589);
                                                            }
                                                            let _e5593 = phi_25551_;
                                                            let _e5596 = (((exp((_e5080 * -3.5714285f)) * ((_e5583 * _e5583) * (3f - (2f * _e5583)))) * select(_e5589, 4f, _e5593)) * 0.12f);
                                                            let _e5602 = sqrt((((_e5041 * _e5041) + (_e5042 * _e5042)) + (_e5043 * _e5043)));
                                                            let _e5609 = (((_e5041 * _e5007) + (_e5042 * _e5010)) + (_e5043 * _e5013));
                                                            let _e5610 = (1f - _e5092);
                                                            let _e5626 = ((_e5596 * (0.6f + (0.4f * (1f - _e5094)))) + exp((_e5602 * -16.666668f)));
                                                            let _e5632 = (((_e5596 * _e5094) * 1.8f) * select(_e5570, 0f, _e5574));
                                                            let _e5639 = (_e4962 * (1f - ((((0.75f * _e5610) * _e5610) * exp(((_e5609 * -28.571428f) * (_e5609 * 28.571428f)))) * exp((_e5602 * -1.4285715f)))));
                                                            phi_7695_ = vec3<f32>();
                                                            phi_7696_ = vec3<f32>((((1.1633643f * _e5626) + (0.86689025f * _e5632)) * _e5639), (((0.97722596f * _e5626) + (1.0155001f * _e5632)) * _e5639), (((0.74455315f * _e5626) + (1.2384146f * _e5632)) * _e5639));
                                                        }
                                                        let _e5666 = phi_7695_;
                                                        let _e5668 = phi_7696_;
                                                        phi_7697_ = _e5666;
                                                        phi_7698_ = _e5668;
                                                    }
                                                    let _e5670 = phi_7697_;
                                                    let _e5672 = phi_7698_;
                                                    phi_6549_ = (_e4936 + select(_e5672, _e5670, vec3(select(_e4944, true, _e4968))));
                                                    phi_6552_ = (_e4938 + 1u);
                                                } else {
                                                    phi_6549_ = vec3<f32>();
                                                    phi_6552_ = u32();
                                                }
                                                let _e5679 = phi_6549_;
                                                let _e5681 = phi_6552_;
                                                phi_6548_ = _e5679;
                                                phi_6551_ = _e5681;
                                                if !(_e4939) {
                                                    break;
                                                }
                                            }
                                            phi_33973_ = _e4068;
                                            if _e4068 {
                                                break;
                                            }
                                            phi_7717_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7720_ = 0u;
                                            phi_7722_ = _e4934;
                                            loop {
                                                let _e5684 = phi_7717_;
                                                let _e5686 = phi_7720_;
                                                let _e5688 = phi_7722_;
                                                local_45 = _e5688;
                                                local_46 = _e5688;
                                                local_47 = _e5688;
                                                local_48 = _e5688;
                                                local_52 = _e5684;
                                                local_53 = _e5684;
                                                local_54 = _e5684;
                                                let _e5689 = (_e5686 < bitcast<u32>(_e3797));
                                                if _e5689 {
                                                    let _e5691 = (bitcast<u32>(_e3793) + (_e5686 * 18u));
                                                    let _e5693 = global_6.member[_e5691];
                                                    let _e5698 = select(select(u32(_e5693), 0u, (_e5693 < 0f)), 4294967295u, (_e5693 > 4294967000f));
                                                    let _e5701 = global_6.member[(_e5691 + 1u)];
                                                    let _e5704 = global_6.member[(_e5691 + 2u)];
                                                    let _e5707 = global_6.member[(_e5691 + 3u)];
                                                    let _e5710 = global_6.member[(_e5691 + 4u)];
                                                    let _e5713 = global_6.member[(_e5691 + 5u)];
                                                    let _e5716 = global_6.member[(_e5691 + 6u)];
                                                    let _e5719 = global_6.member[(_e5691 + 7u)];
                                                    let _e5722 = global_6.member[(_e5691 + 8u)];
                                                    let _e5725 = global_6.member[(_e5691 + 9u)];
                                                    let _e5728 = global_6.member[(_e5691 + 10u)];
                                                    let _e5731 = global_6.member[(_e5691 + 11u)];
                                                    let _e5734 = global_6.member[(_e5691 + 12u)];
                                                    let _e5737 = global_6.member[(_e5691 + 13u)];
                                                    let _e5740 = global_6.member[(_e5691 + 14u)];
                                                    let _e5743 = global_6.member[(_e5691 + 15u)];
                                                    let _e5746 = global_6.member[(_e5691 + 16u)];
                                                    let _e5749 = global_6.member[(_e5691 + 17u)];
                                                    let _e5754 = (((_e335.member_1.x * _e5701) + (_e335.member_1.y * _e5704)) + (_e335.member_1.z * _e5707));
                                                    if (_e5754 < _e5713) {
                                                        phi_9039_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5759 = ((_e335.member_1.x / _e5754) - _e5701);
                                                        let _e5760 = ((_e335.member_1.y / _e5754) - _e5704);
                                                        let _e5761 = ((_e335.member_1.z / _e5754) - _e5707);
                                                        let _e5768 = ((((_e5759 * _e5722) + (_e5760 * _e5725)) + (_e5761 * _e5728)) / (_e5710 * _e5740));
                                                        let _e5774 = ((((_e5759 * _e5731) + (_e5760 * _e5734)) + (_e5761 * _e5737)) / _e5710);
                                                        let _e5778 = sqrt(((_e5768 * _e5768) + (_e5774 * _e5774)));
                                                        let _e5781 = vec3<f32>((_e5768 * 2.2f), (_e5774 * 2.2f), 0f);
                                                        let _e5782 = (_e5698 == 0u);
                                                        if _e5782 {
                                                            phi_8024_ = 6u;
                                                            phi_8025_ = 1.2f;
                                                            phi_8026_ = _e5781;
                                                        } else {
                                                            let _e5783 = (_e5698 == 1u);
                                                            if _e5783 {
                                                                phi_8023_ = vec3<f32>((_e5768 * 1.54f), (_e5774 * 1.54f), 0f);
                                                            } else {
                                                                phi_8023_ = _e5781;
                                                            }
                                                            let _e5788 = phi_8023_;
                                                            phi_8024_ = select(6u, 5u, _e5783);
                                                            phi_8025_ = select(1f, 0.8f, _e5783);
                                                            phi_8026_ = _e5788;
                                                        }
                                                        let _e5792 = phi_8024_;
                                                        let _e5794 = phi_8025_;
                                                        let _e5796 = phi_8026_;
                                                        if _e5782 {
                                                            phi_8043_ = vec3<f32>(((_e5768 * 3.7400002f) + -7f), ((_e5774 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8043_ = vec3<f32>((_e5768 * 3.3000002f), (_e5774 * 3.3000002f), 0f);
                                                        }
                                                        let _e5806 = phi_8043_;
                                                        phi_8045_ = 0u;
                                                        phi_8048_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8050_ = 0f;
                                                        phi_8052_ = 0f;
                                                        phi_8054_ = 0f;
                                                        loop {
                                                            let _e5809 = phi_8045_;
                                                            let _e5811 = phi_8048_;
                                                            let _e5813 = phi_8050_;
                                                            let _e5815 = phi_8052_;
                                                            let _e5817 = phi_8054_;
                                                            local_39 = _e5817;
                                                            local_40 = _e5815;
                                                            local_41 = _e5813;
                                                            local_42 = _e5815;
                                                            local_43 = _e5815;
                                                            local_44 = _e5813;
                                                            let _e5818 = (_e5809 < 6u);
                                                            if _e5818 {
                                                                let _e5819 = (_e5809 == 0u);
                                                                if _e5819 {
                                                                    phi_8096_ = true;
                                                                    phi_8097_ = false;
                                                                    phi_8098_ = false;
                                                                } else {
                                                                    if (_e5809 <= 4u) {
                                                                        phi_8074_ = (_e5698 == 2u);
                                                                    } else {
                                                                        phi_8074_ = true;
                                                                    }
                                                                    let _e5823 = phi_8074_;
                                                                    if _e5823 {
                                                                        let _e5824 = (_e5809 == 5u);
                                                                        if _e5824 {
                                                                            if _e5782 {
                                                                                phi_8090_ = false;
                                                                            } else {
                                                                                phi_8090_ = (_e5698 == 2u);
                                                                            }
                                                                            let _e5827 = phi_8090_;
                                                                            phi_8092_ = _e5827;
                                                                        } else {
                                                                            phi_8092_ = false;
                                                                        }
                                                                        let _e5829 = phi_8092_;
                                                                        phi_8094_ = select(false, _e5782, _e5824);
                                                                        phi_8095_ = _e5829;
                                                                    } else {
                                                                        phi_8094_ = false;
                                                                        phi_8095_ = false;
                                                                    }
                                                                    let _e5832 = phi_8094_;
                                                                    let _e5834 = phi_8095_;
                                                                    phi_8096_ = select(true, false, _e5823);
                                                                    phi_8097_ = _e5832;
                                                                    phi_8098_ = _e5834;
                                                                }
                                                                let _e5837 = phi_8096_;
                                                                let _e5839 = phi_8097_;
                                                                let _e5841 = phi_8098_;
                                                                if select(select(_e5841, false, _e5837), true, select(_e5839, true, _e5837)) {
                                                                    if _e5819 {
                                                                        phi_8145_ = 3u;
                                                                        phi_8146_ = vec3<f32>(((_e5768 * 1.7600001f) + 3f), ((_e5774 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5809 < 4u) {
                                                                            let _e5858 = (_e5809 - 1u);
                                                                            if (_e5858 == 0u) {
                                                                                phi_25570_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_25570_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5858 == 1u)));
                                                                            }
                                                                            let _e5864 = phi_25570_;
                                                                            phi_8143_ = 4u;
                                                                            phi_8144_ = (_e5796 + _e5864);
                                                                        } else {
                                                                            let _e5846 = (_e5809 == 4u);
                                                                            if _e5846 {
                                                                                phi_8142_ = (_e5796 + vec3<f32>((_e5811.x * _e5794), (_e5811.y * _e5794), (_e5811.z * _e5794)));
                                                                            } else {
                                                                                phi_8142_ = _e5806;
                                                                            }
                                                                            let _e5856 = phi_8142_;
                                                                            phi_8143_ = select(select(4u, 5u, _e5782), _e5792, _e5846);
                                                                            phi_8144_ = _e5856;
                                                                        }
                                                                        let _e5867 = phi_8143_;
                                                                        let _e5869 = phi_8144_;
                                                                        phi_8145_ = _e5867;
                                                                        phi_8146_ = _e5869;
                                                                    }
                                                                    let _e5876 = phi_8145_;
                                                                    let _e5878 = phi_8146_;
                                                                    let _e5880 = global_6.member[_e3782];
                                                                    let _e5881 = bitcast<u32>(_e5880);
                                                                    let _e5884 = global_6.member[(_e1293 + 11u)];
                                                                    let _e5887 = global_6.member[(_e1293 + 12u)];
                                                                    let _e5890 = global_6.member[(_e1293 + 13u)];
                                                                    let _e5893 = global_6.member[(_e1293 + 14u)];
                                                                    let _e5896 = global_6.member[(_e1293 + 15u)];
                                                                    let _e5899 = global_6.member[(_e1293 + 16u)];
                                                                    let _e5902 = global_6.member[(_e1293 + 17u)];
                                                                    let _e5905 = global_6.member[(_e1293 + 18u)];
                                                                    let _e5908 = global_6.member[(_e1293 + 19u)];
                                                                    phi_8281_ = 0u;
                                                                    phi_8284_ = 0f;
                                                                    phi_8286_ = 1f;
                                                                    phi_8288_ = 0f;
                                                                    phi_8290_ = _e5878;
                                                                    loop {
                                                                        let _e5910 = phi_8281_;
                                                                        let _e5912 = phi_8284_;
                                                                        let _e5914 = phi_8286_;
                                                                        let _e5916 = phi_8288_;
                                                                        let _e5918 = phi_8290_;
                                                                        local_36 = _e5912;
                                                                        local_37 = _e5916;
                                                                        local_38 = _e5912;
                                                                        let _e5919 = (_e5910 < _e5876);
                                                                        if _e5919 {
                                                                            let _e5921 = (_e5918.x + _e5743);
                                                                            let _e5923 = (_e5918.y + _e5746);
                                                                            let _e5925 = (_e5918.z + _e5749);
                                                                            let _e5926 = floor(_e5921);
                                                                            let _e5927 = floor(_e5923);
                                                                            let _e5928 = floor(_e5925);
                                                                            let _e5937 = bitcast<u32>((select(0i, select(select(i32(_e5926), i32(-2147483648), (_e5926 < -2147483600f)), 2147483647i, (_e5926 > 2147483500f)), (_e5926 == _e5926)) & 255i));
                                                                            let _e5946 = bitcast<u32>((select(0i, select(select(i32(_e5927), i32(-2147483648), (_e5927 < -2147483600f)), 2147483647i, (_e5927 > 2147483500f)), (_e5927 == _e5927)) & 255i));
                                                                            let _e5955 = bitcast<u32>((select(0i, select(select(i32(_e5928), i32(-2147483648), (_e5928 < -2147483600f)), 2147483647i, (_e5928 > 2147483500f)), (_e5928 == _e5928)) & 255i));
                                                                            let _e5956 = (_e5921 - _e5926);
                                                                            let _e5957 = (_e5923 - _e5927);
                                                                            let _e5958 = (_e5925 - _e5928);
                                                                            let _e5965 = (((_e5956 * _e5956) * _e5956) * ((_e5956 * ((_e5956 * 6f) - 15f)) + 10f));
                                                                            let _e5972 = (((_e5957 * _e5957) * _e5957) * ((_e5957 * ((_e5957 * 6f) - 15f)) + 10f));
                                                                            let _e5982 = global_6.member[(_e5881 + _e5937)];
                                                                            let _e5988 = (select(select(u32(_e5982), 0u, (_e5982 < 0f)), 4294967295u, (_e5982 > 4294967000f)) + _e5946);
                                                                            let _e5991 = global_6.member[(_e5881 + _e5988)];
                                                                            let _e5997 = (select(select(u32(_e5991), 0u, (_e5991 < 0f)), 4294967295u, (_e5991 > 4294967000f)) + _e5955);
                                                                            let _e6001 = global_6.member[(_e5881 + (_e5988 + 1u))];
                                                                            let _e6007 = (select(select(u32(_e6001), 0u, (_e6001 < 0f)), 4294967295u, (_e6001 > 4294967000f)) + _e5955);
                                                                            let _e6011 = global_6.member[(_e5881 + (_e5937 + 1u))];
                                                                            let _e6017 = (select(select(u32(_e6011), 0u, (_e6011 < 0f)), 4294967295u, (_e6011 > 4294967000f)) + _e5946);
                                                                            let _e6020 = global_6.member[(_e5881 + _e6017)];
                                                                            let _e6026 = (select(select(u32(_e6020), 0u, (_e6020 < 0f)), 4294967295u, (_e6020 > 4294967000f)) + _e5955);
                                                                            let _e6030 = global_6.member[(_e5881 + (_e6017 + 1u))];
                                                                            let _e6036 = (select(select(u32(_e6030), 0u, (_e6030 < 0f)), 4294967295u, (_e6030 > 4294967000f)) + _e5955);
                                                                            let _e6039 = global_6.member[(_e5881 + _e5997)];
                                                                            let _e6045 = (select(select(u32(_e6039), 0u, (_e6039 < 0f)), 4294967295u, (_e6039 > 4294967000f)) % 12u);
                                                                            let _e6048 = select(-1f, 1f, ((_e6045 & 1u) == 0u));
                                                                            let _e6051 = select(-1f, 1f, ((_e6045 & 2u) == 0u));
                                                                            if (_e6045 < 4u) {
                                                                                phi_25631_ = ((_e6048 * _e5956) + (_e6051 * _e5957));
                                                                            } else {
                                                                                if (_e6045 < 8u) {
                                                                                    phi_25625_ = ((_e6048 * _e5956) + (_e6051 * _e5958));
                                                                                } else {
                                                                                    phi_25625_ = ((_e6048 * _e5957) + (_e6051 * _e5958));
                                                                                }
                                                                                let _e6061 = phi_25625_;
                                                                                phi_25631_ = _e6061;
                                                                            }
                                                                            let _e6066 = phi_25631_;
                                                                            let _e6069 = global_6.member[(_e5881 + _e6026)];
                                                                            let _e6075 = (_e5956 - 1f);
                                                                            let _e6076 = (select(select(u32(_e6069), 0u, (_e6069 < 0f)), 4294967295u, (_e6069 > 4294967000f)) % 12u);
                                                                            let _e6079 = select(-1f, 1f, ((_e6076 & 1u) == 0u));
                                                                            let _e6082 = select(-1f, 1f, ((_e6076 & 2u) == 0u));
                                                                            if (_e6076 < 4u) {
                                                                                phi_25665_ = ((_e6079 * _e6075) + (_e6082 * _e5957));
                                                                            } else {
                                                                                if (_e6076 < 8u) {
                                                                                    phi_25659_ = ((_e6079 * _e6075) + (_e6082 * _e5958));
                                                                                } else {
                                                                                    phi_25659_ = ((_e6079 * _e5957) + (_e6082 * _e5958));
                                                                                }
                                                                                let _e6092 = phi_25659_;
                                                                                phi_25665_ = _e6092;
                                                                            }
                                                                            let _e6097 = phi_25665_;
                                                                            let _e6100 = global_6.member[(_e5881 + _e6007)];
                                                                            let _e6106 = (_e5957 - 1f);
                                                                            let _e6107 = (select(select(u32(_e6100), 0u, (_e6100 < 0f)), 4294967295u, (_e6100 > 4294967000f)) % 12u);
                                                                            let _e6110 = select(-1f, 1f, ((_e6107 & 1u) == 0u));
                                                                            let _e6113 = select(-1f, 1f, ((_e6107 & 2u) == 0u));
                                                                            if (_e6107 < 4u) {
                                                                                phi_25699_ = ((_e6110 * _e5956) + (_e6113 * _e6106));
                                                                            } else {
                                                                                if (_e6107 < 8u) {
                                                                                    phi_25693_ = ((_e6110 * _e5956) + (_e6113 * _e5958));
                                                                                } else {
                                                                                    phi_25693_ = ((_e6110 * _e6106) + (_e6113 * _e5958));
                                                                                }
                                                                                let _e6123 = phi_25693_;
                                                                                phi_25699_ = _e6123;
                                                                            }
                                                                            let _e6128 = phi_25699_;
                                                                            let _e6131 = global_6.member[(_e5881 + _e6036)];
                                                                            let _e6137 = (select(select(u32(_e6131), 0u, (_e6131 < 0f)), 4294967295u, (_e6131 > 4294967000f)) % 12u);
                                                                            let _e6140 = select(-1f, 1f, ((_e6137 & 1u) == 0u));
                                                                            let _e6143 = select(-1f, 1f, ((_e6137 & 2u) == 0u));
                                                                            if (_e6137 < 4u) {
                                                                                phi_25733_ = ((_e6140 * _e6075) + (_e6143 * _e6106));
                                                                            } else {
                                                                                if (_e6137 < 8u) {
                                                                                    phi_25727_ = ((_e6140 * _e6075) + (_e6143 * _e5958));
                                                                                } else {
                                                                                    phi_25727_ = ((_e6140 * _e6106) + (_e6143 * _e5958));
                                                                                }
                                                                                let _e6153 = phi_25727_;
                                                                                phi_25733_ = _e6153;
                                                                            }
                                                                            let _e6158 = phi_25733_;
                                                                            let _e6162 = global_6.member[(_e5881 + (_e5997 + 1u))];
                                                                            let _e6168 = (_e5958 - 1f);
                                                                            let _e6169 = (select(select(u32(_e6162), 0u, (_e6162 < 0f)), 4294967295u, (_e6162 > 4294967000f)) % 12u);
                                                                            let _e6172 = select(-1f, 1f, ((_e6169 & 1u) == 0u));
                                                                            let _e6175 = select(-1f, 1f, ((_e6169 & 2u) == 0u));
                                                                            if (_e6169 < 4u) {
                                                                                phi_25767_ = ((_e6172 * _e5956) + (_e6175 * _e5957));
                                                                            } else {
                                                                                if (_e6169 < 8u) {
                                                                                    phi_25761_ = ((_e6172 * _e5956) + (_e6175 * _e6168));
                                                                                } else {
                                                                                    phi_25761_ = ((_e6172 * _e5957) + (_e6175 * _e6168));
                                                                                }
                                                                                let _e6185 = phi_25761_;
                                                                                phi_25767_ = _e6185;
                                                                            }
                                                                            let _e6190 = phi_25767_;
                                                                            let _e6194 = global_6.member[(_e5881 + (_e6026 + 1u))];
                                                                            let _e6200 = (select(select(u32(_e6194), 0u, (_e6194 < 0f)), 4294967295u, (_e6194 > 4294967000f)) % 12u);
                                                                            let _e6203 = select(-1f, 1f, ((_e6200 & 1u) == 0u));
                                                                            let _e6206 = select(-1f, 1f, ((_e6200 & 2u) == 0u));
                                                                            if (_e6200 < 4u) {
                                                                                phi_25801_ = ((_e6203 * _e6075) + (_e6206 * _e5957));
                                                                            } else {
                                                                                if (_e6200 < 8u) {
                                                                                    phi_25795_ = ((_e6203 * _e6075) + (_e6206 * _e6168));
                                                                                } else {
                                                                                    phi_25795_ = ((_e6203 * _e5957) + (_e6206 * _e6168));
                                                                                }
                                                                                let _e6216 = phi_25795_;
                                                                                phi_25801_ = _e6216;
                                                                            }
                                                                            let _e6221 = phi_25801_;
                                                                            let _e6225 = global_6.member[(_e5881 + (_e6007 + 1u))];
                                                                            let _e6231 = (select(select(u32(_e6225), 0u, (_e6225 < 0f)), 4294967295u, (_e6225 > 4294967000f)) % 12u);
                                                                            let _e6234 = select(-1f, 1f, ((_e6231 & 1u) == 0u));
                                                                            let _e6237 = select(-1f, 1f, ((_e6231 & 2u) == 0u));
                                                                            if (_e6231 < 4u) {
                                                                                phi_25835_ = ((_e6234 * _e5956) + (_e6237 * _e6106));
                                                                            } else {
                                                                                if (_e6231 < 8u) {
                                                                                    phi_25829_ = ((_e6234 * _e5956) + (_e6237 * _e6168));
                                                                                } else {
                                                                                    phi_25829_ = ((_e6234 * _e6106) + (_e6237 * _e6168));
                                                                                }
                                                                                let _e6247 = phi_25829_;
                                                                                phi_25835_ = _e6247;
                                                                            }
                                                                            let _e6252 = phi_25835_;
                                                                            let _e6256 = global_6.member[(_e5881 + (_e6036 + 1u))];
                                                                            let _e6262 = (select(select(u32(_e6256), 0u, (_e6256 < 0f)), 4294967295u, (_e6256 > 4294967000f)) % 12u);
                                                                            let _e6265 = select(-1f, 1f, ((_e6262 & 1u) == 0u));
                                                                            let _e6268 = select(-1f, 1f, ((_e6262 & 2u) == 0u));
                                                                            if (_e6262 < 4u) {
                                                                                phi_25869_ = ((_e6265 * _e6075) + (_e6268 * _e6106));
                                                                            } else {
                                                                                if (_e6262 < 8u) {
                                                                                    phi_25863_ = ((_e6265 * _e6075) + (_e6268 * _e6168));
                                                                                } else {
                                                                                    phi_25863_ = ((_e6265 * _e6106) + (_e6268 * _e6168));
                                                                                }
                                                                                let _e6278 = phi_25863_;
                                                                                phi_25869_ = _e6278;
                                                                            }
                                                                            let _e6283 = phi_25869_;
                                                                            let _e6286 = (_e6066 + (_e5965 * (_e6097 - _e6066)));
                                                                            let _e6292 = (_e6190 + (_e5965 * (_e6221 - _e6190)));
                                                                            let _e6298 = (_e6286 + (_e5972 * ((_e6128 + (_e5965 * (_e6158 - _e6128))) - _e6286)));
                                                                            phi_8282_ = (_e5910 + 1u);
                                                                            phi_8285_ = (_e5912 + _e5914);
                                                                            phi_8287_ = (_e5914 * 0.5f);
                                                                            phi_8289_ = (_e5916 + ((_e6298 + ((((_e5958 * _e5958) * _e5958) * ((_e5958 * ((_e5958 * 6f) - 15f)) + 10f)) * ((_e6292 + (_e5972 * ((_e6252 + (_e5965 * (_e6283 - _e6252))) - _e6292))) - _e6298))) * _e5914));
                                                                            phi_8291_ = vec3<f32>(((((_e5884 * _e5918.x) + (_e5887 * _e5918.y)) + (_e5890 * _e5918.z)) * 2.03f), ((((_e5893 * _e5918.x) + (_e5896 * _e5918.y)) + (_e5899 * _e5918.z)) * 2.03f), ((((_e5902 * _e5918.x) + (_e5905 * _e5918.y)) + (_e5908 * _e5918.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8282_ = u32();
                                                                            phi_8285_ = f32();
                                                                            phi_8287_ = f32();
                                                                            phi_8289_ = f32();
                                                                            phi_8291_ = vec3<f32>();
                                                                        }
                                                                        let _e6330 = phi_8282_;
                                                                        let _e6332 = phi_8285_;
                                                                        let _e6334 = phi_8287_;
                                                                        let _e6336 = phi_8289_;
                                                                        let _e6338 = phi_8291_;
                                                                        phi_8281_ = _e6330;
                                                                        phi_8284_ = _e6332;
                                                                        phi_8286_ = _e6334;
                                                                        phi_8288_ = _e6336;
                                                                        phi_8290_ = _e6338;
                                                                        if !(_e5919) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6341 = local_36;
                                                                    if (_e6341 > 0f) {
                                                                        let _e6344 = local_37;
                                                                        let _e6346 = local_38;
                                                                        phi_8704_ = (_e6344 / _e6346);
                                                                    } else {
                                                                        phi_8704_ = 0f;
                                                                    }
                                                                    let _e6349 = phi_8704_;
                                                                    if _e5819 {
                                                                        phi_8739_ = _e5811;
                                                                        phi_8740_ = _e5813;
                                                                        phi_8741_ = _e5815;
                                                                    } else {
                                                                        if (_e5809 == 1u) {
                                                                            phi_8736_ = vec3<f32>(_e6349, _e5811.y, _e5811.z);
                                                                            phi_8737_ = _e5813;
                                                                            phi_8738_ = _e5815;
                                                                        } else {
                                                                            if (_e5809 == 2u) {
                                                                                phi_8733_ = vec3<f32>(_e5811.x, _e6349, _e5811.z);
                                                                                phi_8734_ = _e5813;
                                                                                phi_8735_ = _e5815;
                                                                            } else {
                                                                                if (_e5809 == 3u) {
                                                                                    phi_8730_ = vec3<f32>(_e5811.x, _e5811.y, _e6349);
                                                                                    phi_8731_ = _e5813;
                                                                                    phi_8732_ = _e5815;
                                                                                } else {
                                                                                    let _e6353 = (_e5809 == 4u);
                                                                                    phi_8730_ = _e5811;
                                                                                    phi_8731_ = select(_e6349, _e5813, _e6353);
                                                                                    phi_8732_ = select(_e5815, _e6349, _e6353);
                                                                                }
                                                                                let _e6361 = phi_8730_;
                                                                                let _e6363 = phi_8731_;
                                                                                let _e6365 = phi_8732_;
                                                                                phi_8733_ = _e6361;
                                                                                phi_8734_ = _e6363;
                                                                                phi_8735_ = _e6365;
                                                                            }
                                                                            let _e6371 = phi_8733_;
                                                                            let _e6373 = phi_8734_;
                                                                            let _e6375 = phi_8735_;
                                                                            phi_8736_ = _e6371;
                                                                            phi_8737_ = _e6373;
                                                                            phi_8738_ = _e6375;
                                                                        }
                                                                        let _e6381 = phi_8736_;
                                                                        let _e6383 = phi_8737_;
                                                                        let _e6385 = phi_8738_;
                                                                        phi_8739_ = _e6381;
                                                                        phi_8740_ = _e6383;
                                                                        phi_8741_ = _e6385;
                                                                    }
                                                                    let _e6387 = phi_8739_;
                                                                    let _e6389 = phi_8740_;
                                                                    let _e6391 = phi_8741_;
                                                                    phi_8743_ = _e6387;
                                                                    phi_8744_ = _e6389;
                                                                    phi_8745_ = _e6391;
                                                                    phi_8746_ = select(_e5817, _e6349, _e5819);
                                                                } else {
                                                                    phi_8743_ = _e5811;
                                                                    phi_8744_ = _e5813;
                                                                    phi_8745_ = _e5815;
                                                                    phi_8746_ = _e5817;
                                                                }
                                                                let _e6394 = phi_8743_;
                                                                let _e6396 = phi_8744_;
                                                                let _e6398 = phi_8745_;
                                                                let _e6400 = phi_8746_;
                                                                phi_8046_ = (_e5809 + 1u);
                                                                phi_8049_ = _e6394;
                                                                phi_8051_ = _e6396;
                                                                phi_8053_ = _e6398;
                                                                phi_8055_ = _e6400;
                                                            } else {
                                                                phi_8046_ = u32();
                                                                phi_8049_ = vec3<f32>();
                                                                phi_8051_ = f32();
                                                                phi_8053_ = f32();
                                                                phi_8055_ = f32();
                                                            }
                                                            let _e6403 = phi_8046_;
                                                            let _e6405 = phi_8049_;
                                                            let _e6407 = phi_8051_;
                                                            let _e6409 = phi_8053_;
                                                            let _e6411 = phi_8055_;
                                                            phi_8045_ = _e6403;
                                                            phi_8048_ = _e6405;
                                                            phi_8050_ = _e6407;
                                                            phi_8052_ = _e6409;
                                                            phi_8054_ = _e6411;
                                                            if !(_e5818) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6414 = local_39;
                                                        let _e6417 = (_e5778 * (1f + (0.45f * _e6414)));
                                                        if _e5782 {
                                                            let _e6520 = local_43;
                                                            let _e6522 = ((_e6520 - -0.25f) * 1.4285715f);
                                                            let _e6524 = select(_e6522, 0f, (_e6522 < 0f));
                                                            let _e6526 = select(_e6524, 1f, (_e6524 > 1f));
                                                            let _e6530 = ((_e6526 * _e6526) * (3f - (2f * _e6526)));
                                                            let _e6532 = ((_e6417 - 1.05f) * -1.1764706f);
                                                            let _e6534 = select(_e6532, 0f, (_e6532 < 0f));
                                                            let _e6536 = select(_e6534, 1f, (_e6534 > 1f));
                                                            let _e6540 = ((_e6536 * _e6536) * (3f - (2f * _e6536)));
                                                            let _e6541 = (_e6530 * _e6540);
                                                            let _e6543 = ((_e6417 - 0.65f) * -1.5384616f);
                                                            let _e6545 = select(_e6543, 0f, (_e6543 < 0f));
                                                            let _e6547 = select(_e6545, 1f, (_e6545 > 1f));
                                                            let _e6552 = (((_e6547 * _e6547) * (3f - (2f * _e6547))) * _e6530);
                                                            let _e6554 = local_44;
                                                            let _e6556 = ((_e6554 - 0.1f) * 2.857143f);
                                                            let _e6558 = select(_e6556, 0f, (_e6556 < 0f));
                                                            let _e6560 = select(_e6558, 1f, (_e6558 > 1f));
                                                            let _e6565 = (((_e6560 * _e6560) * (3f - (2f * _e6560))) * _e6540);
                                                            let _e6570 = ((0.8f * _e6552) * _e6552);
                                                            let _e6579 = (_e5716 * exp((-2f * _e6565)));
                                                            phi_9038_ = type_59(vec3<f32>((((3.1815925f * _e6541) + (0.14634217f * _e6570)) * _e6579), (((0.38179109f * _e6541) + (1.2439084f * _e6570)) * _e6579), (((0.69995034f * _e6541) + (1.0975662f * _e6570)) * _e6579)), (_e5719 * _e6565));
                                                        } else {
                                                            if (_e5698 == 1u) {
                                                                let _e6490 = local_42;
                                                                let _e6492 = ((_e6490 - -0.3f) * 1.25f);
                                                                let _e6494 = select(_e6492, 0f, (_e6492 < 0f));
                                                                let _e6496 = select(_e6494, 1f, (_e6494 > 1f));
                                                                let _e6502 = ((_e6417 - 1f) * -1f);
                                                                let _e6504 = select(_e6502, 0f, (_e6502 < 0f));
                                                                let _e6506 = select(_e6504, 1f, (_e6504 > 1f));
                                                                let _e6512 = (((_e6496 * _e6496) * (3f - (2f * _e6496))) * pow(((_e6506 * _e6506) * (3f - (2f * _e6506))), 1.5f));
                                                                phi_9037_ = type_59((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5716 * _e6512)), (_e5719 * _e6512));
                                                            } else {
                                                                if (_e5698 == 2u) {
                                                                    let _e6446 = local_41;
                                                                    let _e6449 = (_e5778 * (1f + (0.15f * _e6446)));
                                                                    let _e6450 = (_e6449 - 0.65f);
                                                                    let _e6454 = exp(((_e6450 * -5.5555553f) * (_e6450 * 5.5555553f)));
                                                                    let _e6456 = ((_e6449 - 0.75f) * -1.8181818f);
                                                                    let _e6458 = select(_e6456, 0f, (_e6456 < 0f));
                                                                    let _e6460 = select(_e6458, 1f, (_e6458 > 1f));
                                                                    let _e6464 = ((_e6460 * _e6460) * (3f - (2f * _e6460)));
                                                                    let _e6469 = (exp(((_e5778 * -25f) * (_e5778 * 25f))) * 3f);
                                                                    phi_9036_ = type_59(vec3<f32>(((((3.1815925f * _e6454) + (_e6464 * 0.10243952f)) + _e6469) * _e5716), ((((0.38179109f * _e6454) + (_e6464 * 0.8707359f)) + _e6469) * _e5716), ((((0.69995034f * _e6454) + (_e6464 * 0.76829636f)) + _e6469) * _e5716)), 0f);
                                                                } else {
                                                                    let _e6421 = local_40;
                                                                    let _e6423 = ((_e6421 - -0.2f) * 1.6666666f);
                                                                    let _e6425 = select(_e6423, 0f, (_e6423 < 0f));
                                                                    let _e6427 = select(_e6425, 1f, (_e6425 > 1f));
                                                                    let _e6433 = ((_e6417 - 1f) * -1.4285715f);
                                                                    let _e6435 = select(_e6433, 0f, (_e6433 < 0f));
                                                                    let _e6437 = select(_e6435, 1f, (_e6435 > 1f));
                                                                    phi_9036_ = type_59(vec3<f32>(0f, 0f, 0f), (_e5719 * (((_e6427 * _e6427) * (3f - (2f * _e6427))) * ((_e6437 * _e6437) * (3f - (2f * _e6437))))));
                                                                }
                                                                let _e6488 = phi_9036_;
                                                                phi_9037_ = _e6488;
                                                            }
                                                            let _e6518 = phi_9037_;
                                                            phi_9038_ = _e6518;
                                                        }
                                                        let _e6587 = phi_9038_;
                                                        phi_9039_ = _e6587;
                                                    }
                                                    let _e6589 = phi_9039_;
                                                    phi_7718_ = (_e5684 + _e6589.member);
                                                    phi_7721_ = (_e5686 + 1u);
                                                    phi_7723_ = (_e5688 + _e6589.member_1);
                                                } else {
                                                    phi_7718_ = vec3<f32>();
                                                    phi_7721_ = u32();
                                                    phi_7723_ = f32();
                                                }
                                                let _e6596 = phi_7718_;
                                                let _e6598 = phi_7721_;
                                                let _e6600 = phi_7723_;
                                                phi_7717_ = _e6596;
                                                phi_7720_ = _e6598;
                                                phi_7722_ = _e6600;
                                                if !(_e5689) {
                                                    break;
                                                }
                                            }
                                            phi_33973_ = _e4068;
                                            if _e4068 {
                                                break;
                                            }
                                            let _e6603 = local_45;
                                            if (_e6603 <= 0f) {
                                                phi_9068_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6606 = local_46;
                                                let _e6609 = local_47;
                                                let _e6614 = local_48;
                                                phi_9068_ = vec3<f32>(exp((_e6609 * -0.81f)), exp(-(_e6606)), exp((_e6614 * -1.25f)));
                                            }
                                            let _e6619 = phi_9068_;
                                            let _e6621 = local_49;
                                            let _e6626 = local_50;
                                            let _e6631 = local_51;
                                            let _e6636 = local_52;
                                            let _e6640 = local_53;
                                            let _e6644 = local_54;
                                            let _e6649 = global_6.member[(_e1293 + 9u)];
                                            phi_33982_ = _e4068;
                                            phi_9115_ = vec3<f32>((_e3773.x + ((((_e6621.x * _e6619.x) + _e6636.x) * _e6649) * _e3775.x)), (_e3773.y + ((((_e6626.y * _e6619.y) + _e6640.y) * _e6649) * _e3775.y)), (_e3773.z + ((((_e6631.z * _e6619.z) + _e6644.z) * _e6649) * _e3775.z)));
                                        } else {
                                            phi_33982_ = _e3771;
                                            phi_9115_ = _e3773;
                                        }
                                        let _e6667 = phi_33982_;
                                        let _e6669 = phi_9115_;
                                        phi_33981_ = _e6667;
                                        phi_9116_ = _e6669;
                                    } else {
                                        phi_33981_ = _e3771;
                                        phi_9116_ = _e3779;
                                    }
                                    let _e6671 = phi_33981_;
                                    let _e6673 = phi_9116_;
                                    phi_33980_ = _e6671;
                                    phi_9117_ = _e6673;
                                } else {
                                    phi_33980_ = _e324;
                                    phi_9117_ = _e1438;
                                }
                                let _e6675 = phi_33980_;
                                let _e6677 = phi_9117_;
                                phi_33976_ = _e6675;
                                phi_10390_ = (vec3<f32>(_e335.member_2.x, _e335.member_2.y, _e335.member_2.z) * _e6677);
                            } else {
                                let _e351 = global_1.member.member_8[1u];
                                let _e361 = global_6.member[_e351];
                                let _e364 = global_6.member[(_e351 + 1u)];
                                let _e367 = global_6.member[(_e351 + 2u)];
                                let _e370 = global_6.member[(_e351 + 3u)];
                                let _e373 = global_6.member[(_e351 + 4u)];
                                let _e376 = global_6.member[(_e351 + 5u)];
                                let _e379 = global_6.member[(_e351 + 6u)];
                                let _e382 = global_6.member[(_e351 + 7u)];
                                let _e385 = global_6.member[(_e351 + 8u)];
                                let _e388 = global_6.member[(_e351 + 9u)];
                                let _e391 = global_6.member[(_e351 + 10u)];
                                let _e394 = global_6.member[(_e351 + 11u)];
                                let _e397 = global_6.member[(_e351 + 12u)];
                                let _e400 = global_6.member[(_e351 + 13u)];
                                let _e403 = global_6.member[(_e351 + 14u)];
                                let _e406 = global_6.member[(_e351 + 15u)];
                                let _e410 = global_6.member[(_e351 + 16u)];
                                let _e413 = global_6.member[(_e351 + 17u)];
                                let _e416 = global_6.member[(_e351 + 18u)];
                                let _e419 = global_6.member[(_e351 + 19u)];
                                let _e422 = global_6.member[(_e351 + 20u)];
                                let _e425 = global_6.member[(_e351 + 29u)];
                                let _e426 = bitcast<u32>(_e425);
                                let _e429 = global_6.member[(_e351 + 30u)];
                                let _e430 = bitcast<u32>(_e429);
                                let _e438 = ((((_e335.member.x - _e361) * _e335.member_1.x) + ((_e335.member.y - _e364) * _e335.member_1.y)) + ((_e335.member.z - _e367) * _e335.member_1.z));
                                let _e450 = ((((_e335.member.x * _e335.member.x) + (_e335.member.y * _e335.member.y)) + (_e335.member.z * _e335.member.z)) - (2f * (((_e335.member.x * _e361) + (_e335.member.y * _e364)) + (_e335.member.z * _e367))));
                                let _e451 = (_e450 + _e379);
                                let _e452 = (_e438 * _e438);
                                let _e453 = (_e452 - _e451);
                                if (_e453 < 0f) {
                                    phi_21774_ = type_57(0u, type_57().member_1);
                                    phi_21775_ = type_57();
                                    phi_21776_ = true;
                                } else {
                                    if (_e438 < 0f) {
                                        phi_21746_ = -(sqrt(_e453));
                                    } else {
                                        phi_21746_ = sqrt(_e453);
                                    }
                                    let _e460 = phi_21746_;
                                    let _e462 = -((_e438 + _e460));
                                    let _e463 = (_e462 == 0f);
                                    if _e463 {
                                        phi_21769_ = type_57(1u, type_56(0f, 0f));
                                        phi_21770_ = type_57();
                                    } else {
                                        let _e464 = (_e451 / _e462);
                                        if (_e462 < _e464) {
                                            phi_21758_ = type_56(_e462, _e464);
                                        } else {
                                            phi_21758_ = type_56(_e464, _e462);
                                        }
                                        let _e469 = phi_21758_;
                                        phi_21769_ = type_57();
                                        phi_21770_ = type_57(1u, _e469);
                                    }
                                    let _e472 = phi_21769_;
                                    let _e474 = phi_21770_;
                                    phi_21774_ = _e472;
                                    phi_21775_ = _e474;
                                    phi_21776_ = _e463;
                                }
                                let _e479 = phi_21774_;
                                let _e481 = phi_21775_;
                                let _e483 = phi_21776_;
                                if _e483 {
                                    phi_21780_ = _e479;
                                } else {
                                    phi_21780_ = _e481;
                                }
                                let _e485 = phi_21780_;
                                if (bitcast<i32>(_e485.member) != 0i) {
                                    phi_21667_ = type_57(0u, _e485.member_1);
                                } else {
                                    phi_21667_ = type_57(1u, type_57().member_1);
                                }
                                let _e495 = phi_21667_;
                                if (bitcast<i32>(_e495.member) != 0i) {
                                    phi_21714_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e495.member_1.member_1 <= 0f) {
                                        phi_21710_ = type_57(0u, type_57().member_1);
                                    } else {
                                        if (_e495.member_1.member != _e495.member_1.member) {
                                            phi_21818_ = true;
                                        } else {
                                            phi_21818_ = (0f >= _e495.member_1.member);
                                        }
                                        let _e507 = phi_21818_;
                                        let _e508 = select(_e495.member_1.member, 0f, _e507);
                                        let _e509 = (_e450 + _e376);
                                        if (_e509 > 0f) {
                                            let _e511 = (_e452 - _e509);
                                            if (_e511 < 0f) {
                                                phi_21926_ = type_57(0u, type_57().member_1);
                                                phi_21927_ = type_57();
                                                phi_21928_ = true;
                                            } else {
                                                if (_e438 < 0f) {
                                                    phi_21898_ = -(sqrt(_e511));
                                                } else {
                                                    phi_21898_ = sqrt(_e511);
                                                }
                                                let _e518 = phi_21898_;
                                                let _e520 = -((_e438 + _e518));
                                                let _e521 = (_e520 == 0f);
                                                if _e521 {
                                                    phi_21921_ = type_57(1u, type_56(0f, 0f));
                                                    phi_21922_ = type_57();
                                                } else {
                                                    let _e522 = (_e509 / _e520);
                                                    if (_e520 < _e522) {
                                                        phi_21910_ = type_56(_e520, _e522);
                                                    } else {
                                                        phi_21910_ = type_56(_e522, _e520);
                                                    }
                                                    let _e527 = phi_21910_;
                                                    phi_21921_ = type_57();
                                                    phi_21922_ = type_57(1u, _e527);
                                                }
                                                let _e530 = phi_21921_;
                                                let _e532 = phi_21922_;
                                                phi_21926_ = _e530;
                                                phi_21927_ = _e532;
                                                phi_21928_ = _e521;
                                            }
                                            let _e537 = phi_21926_;
                                            let _e539 = phi_21927_;
                                            let _e541 = phi_21928_;
                                            if _e541 {
                                                phi_21932_ = _e537;
                                            } else {
                                                phi_21932_ = _e539;
                                            }
                                            let _e543 = phi_21932_;
                                            if (bitcast<i32>(_e543.member) != 0i) {
                                                phi_21866_ = select(_e543.member_1.member_1, _e543.member_1.member, (_e543.member_1.member >= 0f));
                                            } else {
                                                phi_21866_ = -1f;
                                            }
                                            let _e554 = phi_21866_;
                                            if (_e554 >= 0f) {
                                                if (_e495.member_1.member_1 != _e495.member_1.member_1) {
                                                    phi_21970_ = true;
                                                } else {
                                                    phi_21970_ = (_e554 <= _e495.member_1.member_1);
                                                }
                                                let _e559 = phi_21970_;
                                                phi_21695_ = select(_e495.member_1.member_1, _e554, _e559);
                                            } else {
                                                phi_21695_ = _e495.member_1.member_1;
                                            }
                                            let _e562 = phi_21695_;
                                            phi_21697_ = _e562;
                                        } else {
                                            phi_21697_ = _e495.member_1.member_1;
                                        }
                                        let _e564 = phi_21697_;
                                        if (_e564 <= _e508) {
                                            phi_21706_ = type_57(0u, type_57().member_1);
                                        } else {
                                            phi_21706_ = type_57(1u, type_56(_e508, _e564));
                                        }
                                        let _e572 = phi_21706_;
                                        phi_21710_ = _e572;
                                    }
                                    let _e577 = phi_21710_;
                                    phi_21714_ = _e577;
                                }
                                let _e582 = phi_21714_;
                                if (bitcast<i32>(_e582.member) != 0i) {
                                    let _e588 = (_e582.member_1.member < _e335.member.w);
                                    if _e588 {
                                        if (_e582.member_1.member_1 != _e582.member_1.member_1) {
                                            phi_21985_ = true;
                                        } else {
                                            phi_21985_ = (_e335.member.w <= _e582.member_1.member_1);
                                        }
                                        let _e594 = phi_21985_;
                                        let _e595 = select(_e582.member_1.member_1, _e335.member.w, _e594);
                                        phi_9511_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9514_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9516_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9518_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9520_ = 0u;
                                        loop {
                                            let _e597 = phi_9511_;
                                            let _e599 = phi_9514_;
                                            let _e601 = phi_9516_;
                                            let _e603 = phi_9518_;
                                            let _e605 = phi_9520_;
                                            local = _e601;
                                            local_1 = _e599;
                                            local_2 = _e597;
                                            local_3 = _e597;
                                            local_4 = _e597;
                                            local_5 = _e603;
                                            local_6 = _e603;
                                            local_7 = _e603;
                                            let _e606 = (_e605 < 32u);
                                            if _e606 {
                                                let _e607 = -(_e438);
                                                if (_e582.member_1.member <= _e595) {
                                                } else {
                                                    phi_33495_ = true;
                                                    break;
                                                }
                                                let _e610 = select(_e607, _e582.member_1.member, (_e607 < _e582.member_1.member));
                                                let _e612 = select(_e610, _e595, (_e610 > _e595));
                                                let _e613 = (_e612 - _e582.member_1.member);
                                                let _e614 = (_e595 - _e612);
                                                let _e615 = (_e595 - _e582.member_1.member);
                                                let _e616 = (0.000000001f * _e615);
                                                if (_e613 <= _e616) {
                                                    let _e691 = (f32(_e605) * 0.03125f);
                                                    let _e694 = (f32((_e605 + 1u)) * 0.03125f);
                                                    let _e696 = ((_e691 + _e694) * 0.5f);
                                                    phi_9621_ = type_56((_e582.member_1.member + ((_e615 * _e696) * _e696)), (_e615 * ((_e694 * _e694) - (_e691 * _e691))));
                                                } else {
                                                    if (_e614 <= _e616) {
                                                        let _e674 = (f32((31u - _e605)) * 0.03125f);
                                                        let _e677 = (f32((32u - _e605)) * 0.03125f);
                                                        let _e679 = ((_e674 + _e677) * 0.5f);
                                                        phi_9620_ = type_56((_e595 - ((_e615 * _e679) * _e679)), (_e615 * ((_e677 * _e677) - (_e674 * _e674))));
                                                    } else {
                                                        let _e622 = round(((32f * _e613) / (_e613 + _e614)));
                                                        let _e627 = select(select(u32(_e622), 0u, (_e622 < 0f)), 4294967295u, (_e622 > 4294967000f));
                                                        if (_e627 < 1u) {
                                                            phi_9604_ = 1u;
                                                        } else {
                                                            phi_9604_ = select(_e627, 31u, (_e627 > 31u));
                                                        }
                                                        let _e632 = phi_9604_;
                                                        if (_e605 < _e632) {
                                                            let _e653 = ((_e632 - 1u) - _e605);
                                                            let _e655 = f32(_e632);
                                                            let _e656 = (f32(_e653) / _e655);
                                                            let _e659 = (f32((_e653 + 1u)) / _e655);
                                                            let _e661 = ((_e656 + _e659) * 0.5f);
                                                            phi_9619_ = type_56((_e612 - ((_e613 * _e661) * _e661)), (_e613 * ((_e659 * _e659) - (_e656 * _e656))));
                                                        } else {
                                                            let _e635 = (_e605 - _e632);
                                                            let _e637 = f32((32u - _e632));
                                                            let _e638 = (f32(_e635) / _e637);
                                                            let _e641 = (f32((_e635 + 1u)) / _e637);
                                                            let _e643 = ((_e638 + _e641) * 0.5f);
                                                            phi_9619_ = type_56((_e612 + ((_e614 * _e643) * _e643)), (_e614 * ((_e641 * _e641) - (_e638 * _e638))));
                                                        }
                                                        let _e671 = phi_9619_;
                                                        phi_9620_ = _e671;
                                                    }
                                                    let _e689 = phi_9620_;
                                                    phi_9621_ = _e689;
                                                }
                                                let _e706 = phi_9621_;
                                                let _e712 = (_e335.member.x + (_e335.member_1.x * _e706.member));
                                                let _e713 = (_e335.member.y + (_e335.member_1.y * _e706.member));
                                                let _e714 = (_e335.member.z + (_e335.member_1.z * _e706.member));
                                                let _e727 = ((((_e712 * _e712) + (_e713 * _e713)) + (_e714 * _e714)) - (2f * (((_e712 * _e361) + (_e713 * _e364)) + (_e714 * _e367))));
                                                let _e729 = (_e712 - _e361);
                                                let _e730 = (_e713 - _e364);
                                                let _e731 = (_e714 - _e367);
                                                let _e737 = sqrt((((_e729 * _e729) + (_e730 * _e730)) + (_e731 * _e731)));
                                                let _e739 = ((_e727 + _e376) / (_e737 + _e370));
                                                let _e743 = (1f - (abs((_e739 - 25000f)) * 0.00006666667f));
                                                if (_e743 != _e743) {
                                                    phi_22185_ = true;
                                                } else {
                                                    phi_22185_ = (0f >= _e743);
                                                }
                                                let _e747 = phi_22185_;
                                                let _e749 = -(_e739);
                                                let _e752 = (exp((_e749 / _e391)) * _e706.member_1);
                                                let _e755 = (exp((_e749 / _e394)) * _e706.member_1);
                                                let _e757 = (_e752 * 0.5f);
                                                let _e758 = (_e755 * 0.5f);
                                                let _e759 = ((select(_e743, 0f, _e747) * _e706.member_1) * 0.5f);
                                                let _e761 = (_e603.x + _e757);
                                                let _e763 = (_e603.y + _e758);
                                                let _e765 = (_e603.z + _e759);
                                                let _e770 = (_e410 * 1.1f);
                                                let _e771 = (_e770 * _e763);
                                                let _e790 = (_e410 * _e755);
                                                if (_e430 == 4294967295u) {
                                                    phi_9949_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e798 = (1f / _e737);
                                                    let _e806 = ((((_e729 * _e798) * _e382) + ((_e730 * _e798) * _e385)) + ((_e731 * _e798) * _e388));
                                                    if (_e739 != _e739) {
                                                        phi_22312_ = true;
                                                    } else {
                                                        phi_22312_ = (0f >= _e739);
                                                    }
                                                    let _e810 = phi_22312_;
                                                    let _e814 = sqrt((select(_e739, 0f, _e810) / (_e373 - _e370)));
                                                    if (_e814 != _e814) {
                                                        phi_22327_ = true;
                                                    } else {
                                                        phi_22327_ = (1f <= _e814);
                                                    }
                                                    let _e818 = phi_22327_;
                                                    let _e820 = (select(_e814, 1f, _e818) * 31f);
                                                    let _e822 = select(_e806, -1f, (_e806 < -1f));
                                                    let _e826 = ((select(_e822, 1f, (_e822 > 1f)) + 1f) * 15.5f);
                                                    let _e831 = select(select(u32(_e820), 0u, (_e820 < 0f)), 4294967295u, (_e820 > 4294967000f));
                                                    let _e833 = select(_e831, 30u, (30u < _e831));
                                                    let _e838 = select(select(u32(_e826), 0u, (_e826 < 0f)), 4294967295u, (_e826 > 4294967000f));
                                                    let _e840 = select(_e838, 30u, (30u < _e838));
                                                    let _e842 = (_e820 - f32(_e833));
                                                    let _e844 = (_e826 - f32(_e840));
                                                    let _e848 = (_e430 + (3u * ((_e833 * 32u) + _e840)));
                                                    let _e853 = (_e430 + (3u * (((_e833 + 1u) * 32u) + _e840)));
                                                    let _e855 = global_6.member[_e848];
                                                    let _e858 = global_6.member[(_e848 + 1u)];
                                                    let _e861 = global_6.member[(_e848 + 2u)];
                                                    let _e862 = (1f - _e844);
                                                    let _e868 = global_6.member[(_e848 + 3u)];
                                                    let _e871 = global_6.member[(_e848 + 4u)];
                                                    let _e874 = global_6.member[(_e848 + 5u)];
                                                    let _e881 = (1f - _e842);
                                                    let _e886 = global_6.member[_e853];
                                                    let _e889 = global_6.member[(_e853 + 1u)];
                                                    let _e892 = global_6.member[(_e853 + 2u)];
                                                    let _e898 = global_6.member[(_e853 + 3u)];
                                                    let _e901 = global_6.member[(_e853 + 4u)];
                                                    let _e904 = global_6.member[(_e853 + 5u)];
                                                    phi_9949_ = vec3<f32>(((((_e855 * _e862) + (_e868 * _e844)) * _e881) + (((_e886 * _e862) + (_e898 * _e844)) * _e842)), ((((_e858 * _e862) + (_e871 * _e844)) * _e881) + (((_e889 * _e862) + (_e901 * _e844)) * _e842)), ((((_e861 * _e862) + (_e874 * _e844)) * _e881) + (((_e892 * _e862) + (_e904 * _e844)) * _e842)));
                                                }
                                                let _e919 = phi_9949_;
                                                let _e933 = (_e727 + _e379);
                                                if (_e933 > 0f) {
                                                    let _e939 = (((_e729 * _e382) + (_e730 * _e385)) + (_e731 * _e388));
                                                    let _e941 = ((_e939 * _e939) - _e933);
                                                    if (_e939 >= 0f) {
                                                        phi_10023_ = vec3<f32>();
                                                        phi_10024_ = true;
                                                    } else {
                                                        let _e943 = (_e941 <= 0f);
                                                        if _e943 {
                                                            phi_10022_ = vec3<f32>();
                                                        } else {
                                                            let _e946 = (-(_e939) - sqrt(_e941));
                                                            phi_10022_ = vec3<f32>((_e712 + (_e382 * _e946)), (_e713 + (_e385 * _e946)), (_e714 + (_e388 * _e946)));
                                                        }
                                                        let _e955 = phi_10022_;
                                                        phi_10023_ = _e955;
                                                        phi_10024_ = _e943;
                                                    }
                                                    let _e957 = phi_10023_;
                                                    let _e959 = phi_10024_;
                                                    phi_10029_ = _e957;
                                                    phi_10030_ = select(true, false, _e959);
                                                } else {
                                                    phi_10029_ = vec3<f32>(_e712, _e713, _e714);
                                                    phi_10030_ = true;
                                                }
                                                let _e962 = phi_10029_;
                                                let _e964 = phi_10030_;
                                                if _e964 {
                                                    let _e966 = (_e962.x - _e361);
                                                    let _e968 = (_e962.y - _e364);
                                                    let _e970 = (_e962.z - _e367);
                                                    let _e976 = sqrt((((_e966 * _e966) + (_e968 * _e968)) + (_e970 * _e970)));
                                                    let _e977 = (1f / _e976);
                                                    let _e995 = ((((((_e962.x * _e962.x) + (_e962.y * _e962.y)) + (_e962.z * _e962.z)) - (2f * (((_e962.x * _e361) + (_e962.y * _e364)) + (_e962.z * _e367)))) + _e376) / (_e976 + _e370));
                                                    if (_e995 != _e995) {
                                                        phi_22451_ = true;
                                                    } else {
                                                        phi_22451_ = (0f >= _e995);
                                                    }
                                                    let _e999 = phi_22451_;
                                                    let _e1003 = sqrt((select(_e995, 0f, _e999) / (_e373 - _e370)));
                                                    if (_e1003 != _e1003) {
                                                        phi_22466_ = true;
                                                    } else {
                                                        phi_22466_ = (1f <= _e1003);
                                                    }
                                                    let _e1007 = phi_22466_;
                                                    let _e1009 = (select(_e1003, 1f, _e1007) * 63f);
                                                    let _e1014 = ((((_e966 * _e977) * _e382) + ((_e968 * _e977) * _e385)) + ((_e970 * _e977) * _e388));
                                                    let _e1016 = select(_e1014, -1f, (_e1014 < -1f));
                                                    let _e1020 = ((select(_e1016, 1f, (_e1016 > 1f)) + 1f) * 127.5f);
                                                    let _e1025 = select(select(u32(_e1009), 0u, (_e1009 < 0f)), 4294967295u, (_e1009 > 4294967000f));
                                                    let _e1027 = select(_e1025, 62u, (62u < _e1025));
                                                    let _e1032 = select(select(u32(_e1020), 0u, (_e1020 < 0f)), 4294967295u, (_e1020 > 4294967000f));
                                                    let _e1034 = select(_e1032, 254u, (254u < _e1032));
                                                    let _e1036 = (_e1009 - f32(_e1027));
                                                    let _e1038 = (_e1020 - f32(_e1034));
                                                    let _e1042 = (_e426 + (3u * ((_e1027 * 256u) + _e1034)));
                                                    let _e1047 = (_e426 + (3u * (((_e1027 + 1u) * 256u) + _e1034)));
                                                    let _e1049 = global_6.member[_e1042];
                                                    let _e1052 = global_6.member[(_e1042 + 1u)];
                                                    let _e1055 = global_6.member[(_e1042 + 2u)];
                                                    let _e1056 = (1f - _e1038);
                                                    let _e1062 = global_6.member[(_e1042 + 3u)];
                                                    let _e1065 = global_6.member[(_e1042 + 4u)];
                                                    let _e1068 = global_6.member[(_e1042 + 5u)];
                                                    let _e1075 = (1f - _e1036);
                                                    let _e1080 = global_6.member[_e1047];
                                                    let _e1083 = global_6.member[(_e1047 + 1u)];
                                                    let _e1086 = global_6.member[(_e1047 + 2u)];
                                                    let _e1092 = global_6.member[(_e1047 + 3u)];
                                                    let _e1095 = global_6.member[(_e1047 + 4u)];
                                                    let _e1098 = global_6.member[(_e1047 + 5u)];
                                                    phi_10321_ = vec3<f32>(((((_e1049 * _e1056) + (_e1062 * _e1038)) * _e1075) + (((_e1080 * _e1056) + (_e1092 * _e1038)) * _e1036)), ((((_e1052 * _e1056) + (_e1065 * _e1038)) * _e1075) + (((_e1083 * _e1056) + (_e1095 * _e1038)) * _e1036)), ((((_e1055 * _e1056) + (_e1068 * _e1038)) * _e1075) + (((_e1086 * _e1056) + (_e1098 * _e1038)) * _e1036)));
                                                } else {
                                                    phi_10321_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1113 = phi_10321_;
                                                let _e1114 = (vec3<f32>(_e761, _e763, _e765) + _e1113);
                                                let _e1120 = (_e770 * _e1114.y);
                                                let _e1132 = exp(-((((_e400 * _e1114.x) + _e1120) + (_e416 * _e1114.z))));
                                                let _e1134 = exp(-((((_e403 * _e1114.x) + _e1120) + (_e419 * _e1114.z))));
                                                let _e1136 = exp(-((((_e406 * _e1114.x) + _e1120) + (_e422 * _e1114.z))));
                                                phi_9512_ = vec3<f32>((_e597.x + ((exp(-((((_e400 * _e761) + _e771) + (_e416 * _e765)))) * ((_e400 * _e752) + _e790)) * _e919.x)), (_e597.y + ((exp(-((((_e403 * _e761) + _e771) + (_e419 * _e765)))) * ((_e403 * _e752) + _e790)) * _e919.y)), (_e597.z + ((exp(-((((_e406 * _e761) + _e771) + (_e422 * _e765)))) * ((_e406 * _e752) + _e790)) * _e919.z)));
                                                phi_9515_ = vec3<f32>((_e599.x + (_e1132 * _e755)), (_e599.y + (_e1134 * _e755)), (_e599.z + (_e1136 * _e755)));
                                                phi_9517_ = vec3<f32>((_e601.x + (_e1132 * _e752)), (_e601.y + (_e1134 * _e752)), (_e601.z + (_e1136 * _e752)));
                                                phi_9519_ = vec3<f32>((_e761 + _e757), (_e763 + _e758), (_e765 + _e759));
                                                phi_9521_ = (_e605 + 1u);
                                            } else {
                                                phi_9512_ = vec3<f32>();
                                                phi_9515_ = vec3<f32>();
                                                phi_9517_ = vec3<f32>();
                                                phi_9519_ = vec3<f32>();
                                                phi_9521_ = u32();
                                            }
                                            let _e1163 = phi_9512_;
                                            let _e1165 = phi_9515_;
                                            let _e1167 = phi_9517_;
                                            let _e1169 = phi_9519_;
                                            let _e1171 = phi_9521_;
                                            phi_9511_ = _e1163;
                                            phi_9514_ = _e1165;
                                            phi_9516_ = _e1167;
                                            phi_9518_ = _e1169;
                                            phi_9520_ = _e1171;
                                            phi_33495_ = _e324;
                                            if !(_e606) {
                                                break;
                                            }
                                        }
                                        let _e1174 = phi_33495_;
                                        phi_33973_ = _e1174;
                                        if _e1174 {
                                            break;
                                        }
                                        let _e1176 = local;
                                        let _e1177 = (_e1176 * vec3<f32>(_e400, _e403, _e406));
                                        let _e1179 = local_1;
                                        let _e1180 = (_e1179 * _e410);
                                        let _e1185 = (((_e335.member_1.x * _e382) + (_e335.member_1.y * _e385)) + (_e335.member_1.z * _e388));
                                        let _e1187 = (1f + (_e1185 * _e1185));
                                        let _e1188 = (0.059683103f * _e1187);
                                        let _e1195 = (_e413 * _e413);
                                        let _e1206 = ((0.119366206f * ((1f - _e1195) * _e1187)) / ((2f + _e1195) * pow(((1f + _e1195) - ((2f * _e413) * _e1185)), 1.5f)));
                                        let _e1217 = local_2;
                                        let _e1221 = local_3;
                                        let _e1225 = local_4;
                                        let _e1234 = local_5;
                                        let _e1241 = local_6;
                                        let _e1243 = ((_e410 * 1.1f) * _e1241.y);
                                        let _e1248 = local_7;
                                        phi_33979_ = _e1174;
                                        phi_10378_ = type_60(vec3<f32>(exp(-((((_e400 * _e1234.x) + _e1243) + (_e416 * _e1248.z)))), exp(-((((_e403 * _e1234.x) + _e1243) + (_e419 * _e1248.z)))), exp(-((((_e406 * _e1234.x) + _e1243) + (_e422 * _e1248.z))))), (vec3<f32>(((((_e1177.x * _e1188) + (_e1180.x * _e1206)) + _e1217.x) * _e397), ((((_e1177.y * _e1188) + (_e1180.y * _e1206)) + _e1221.y) * _e397), ((((_e1177.z * _e1188) + (_e1180.z * _e1206)) + _e1225.z) * _e397)) * 0.38317f));
                                    } else {
                                        phi_33979_ = _e324;
                                        phi_10378_ = type_60();
                                    }
                                    let _e1265 = phi_33979_;
                                    let _e1267 = phi_10378_;
                                    phi_33978_ = _e1265;
                                    phi_10380_ = _e1267;
                                    phi_10381_ = select(true, false, _e588);
                                } else {
                                    phi_33978_ = _e324;
                                    phi_10380_ = type_60();
                                    phi_10381_ = true;
                                }
                                let _e1270 = phi_33978_;
                                let _e1272 = phi_10380_;
                                let _e1274 = phi_10381_;
                                if _e1274 {
                                    phi_10386_ = type_60(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10386_ = _e1272;
                                }
                                let _e1276 = phi_10386_;
                                phi_33976_ = _e1270;
                                phi_10390_ = (vec3<f32>(_e335.member_2.x, _e335.member_2.y, _e335.member_2.z) * _e1276.member_1);
                            }
                            let _e6681 = phi_33976_;
                            let _e6683 = phi_10390_;
                            phi_33975_ = _e6681;
                            phi_10391_ = _e6683;
                        }
                        let _e6685 = phi_33975_;
                        let _e6687 = phi_10391_;
                        phi_33974_ = _e6685;
                        phi_509_ = (_e326 + _e6687);
                        phi_512_ = (_e328 + 1u);
                    } else {
                        phi_33974_ = _e324;
                        phi_509_ = vec3<f32>();
                        phi_512_ = u32();
                    }
                    let _e6691 = phi_33974_;
                    let _e6693 = phi_509_;
                    let _e6695 = phi_512_;
                    phi_33512_ = _e6691;
                    phi_508_ = _e6693;
                    phi_511_ = _e6695;
                    phi_33973_ = _e6691;
                    if !(_e329) {
                        break;
                    }
                }
                let _e6698 = phi_33973_;
                if _e6698 {
                    break;
                }
                let _e6700 = local_55;
                let _e6704 = local_56;
                let _e6709 = local_57;
                if ((((_e6700.x * _e6700.x) + (_e6704.y * _e6704.y)) + (_e6709.z * _e6709.z)) > 0f) {
                    if (_e318.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6719 = global_7.member[_e318.member_1].member_3[0u];
                    global_7.member[_e318.member_1].member_3[0u] = (_e6719 + _e6700.x);
                    let _e6725 = global_7.member[_e318.member_1].member_3[1u];
                    global_7.member[_e318.member_1].member_3[1u] = (_e6725 + _e6704.y);
                    let _e6731 = global_7.member[_e318.member_1].member_3[2u];
                    global_7.member[_e318.member_1].member_3[2u] = (_e6731 + _e6709.z);
                    let _e6737 = global_7.member[_e318.member_1].member_3[3u];
                    global_7.member[_e318.member_1].member_3[3u] = _e6737;
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
    var phi_11191_: type_48;
    var phi_26064_: f32;
    var phi_26076_: type_56;
    var phi_26087_: type_57;
    var phi_26088_: type_57;
    var phi_26092_: type_57;
    var phi_26093_: type_57;
    var phi_26094_: bool;
    var phi_26098_: type_57;
    var phi_25985_: type_57;
    var phi_26136_: bool;
    var phi_26216_: f32;
    var phi_26228_: type_56;
    var phi_26239_: type_57;
    var phi_26240_: type_57;
    var phi_26244_: type_57;
    var phi_26245_: type_57;
    var phi_26246_: bool;
    var phi_26250_: type_57;
    var phi_26184_: f32;
    var phi_26288_: bool;
    var phi_26013_: f32;
    var phi_26015_: f32;
    var phi_26024_: type_57;
    var phi_26028_: type_57;
    var phi_26032_: type_57;
    var phi_26303_: bool;
    var phi_12720_: vec3<f32>;
    var phi_12723_: u32;
    var phi_12807_: u32;
    var phi_12822_: type_56;
    var phi_12823_: type_56;
    var phi_12824_: type_56;
    var phi_26502_: bool;
    var phi_12721_: vec3<f32>;
    var phi_12724_: u32;
    var phi_33984_: bool;
    var local_69: vec3<f32>;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var phi_36124_: bool;
    var phi_12849_: vec3<f32>;
    var phi_36123_: bool;
    var phi_12851_: vec3<f32>;
    var phi_12852_: bool;
    var phi_36121_: bool;
    var phi_12859_: vec3<f32>;
    var phi_12860_: type_41;
    var phi_36120_: bool;
    var phi_12861_: vec3<f32>;
    var phi_12862_: type_41;
    var phi_13141_: type_60;
    var phi_13108_: vec3<f32>;
    var phi_13109_: vec3<f32>;
    var phi_13142_: type_60;
    var phi_13265_: i32;
    var phi_13268_: u32;
    var phi_13322_: bool;
    var phi_13328_: bool;
    var phi_13335_: u32;
    var phi_13337_: i32;
    var phi_13338_: u32;
    var phi_13266_: i32;
    var phi_13269_: u32;
    var phi_13340_: bool;
    var local_72: u32;
    var phi_13894_: vec3<f32>;
    var phi_13908_: vec3<f32>;
    var phi_26569_: bool;
    var phi_13941_: f32;
    var phi_14049_: vec3<f32>;
    var phi_14055_: f32;
    var phi_26642_: bool;
    var phi_26631_: f32;
    var phi_14136_: vec3<f32>;
    var phi_14137_: vec3<f32>;
    var phi_14138_: vec3<f32>;
    var phi_34118_: u32;
    var phi_14140_: vec3<f32>;
    var phi_14141_: vec3<f32>;
    var phi_14142_: vec3<f32>;
    var phi_14143_: bool;
    var phi_14158_: vec3<f32>;
    var phi_14159_: vec3<f32>;
    var phi_34116_: u32;
    var phi_14161_: vec3<f32>;
    var phi_14162_: vec3<f32>;
    var phi_14163_: vec3<f32>;
    var phi_34115_: u32;
    var phi_14164_: vec3<f32>;
    var phi_14165_: vec3<f32>;
    var phi_14166_: vec3<f32>;
    var phi_14167_: bool;
    var phi_34114_: u32;
    var phi_14168_: vec3<f32>;
    var phi_14169_: vec3<f32>;
    var phi_14170_: vec3<f32>;
    var phi_14171_: bool;
    var phi_14172_: bool;
    var phi_26715_: bool;
    var phi_26877_: bool;
    var phi_26892_: bool;
    var phi_26907_: bool;
    var phi_26934_: bool;
    var phi_26949_: bool;
    var phi_26964_: bool;
    var phi_26991_: bool;
    var phi_27006_: bool;
    var phi_27021_: bool;
    var phi_14351_: f32;
    var phi_14359_: f32;
    var phi_14367_: f32;
    var phi_27218_: bool;
    var phi_27207_: f32;
    var phi_14474_: vec3<f32>;
    var phi_27246_: bool;
    var phi_14475_: vec3<f32>;
    var phi_14476_: vec3<f32>;
    var phi_14477_: vec3<f32>;
    var phi_34502_: vec3<f32>;
    var phi_34425_: vec3<f32>;
    var phi_34346_: u32;
    var phi_34264_: f32;
    var phi_14479_: vec3<f32>;
    var phi_14480_: vec3<f32>;
    var phi_14481_: bool;
    var phi_27261_: bool;
    var phi_27312_: bool;
    var phi_27301_: f32;
    var phi_34473_: vec3<f32>;
    var phi_14014_: vec3<f32>;
    var phi_34825_: vec3<f32>;
    var phi_34619_: f32;
    var phi_34472_: vec3<f32>;
    var phi_34160_: u32;
    var phi_14016_: vec3<f32>;
    var phi_14017_: bool;
    var phi_34771_: vec3<f32>;
    var phi_34579_: f32;
    var phi_34500_: vec3<f32>;
    var phi_34423_: vec3<f32>;
    var phi_34344_: u32;
    var phi_34262_: f32;
    var phi_34185_: u32;
    var phi_34111_: u32;
    var phi_14489_: vec3<f32>;
    var phi_14490_: vec3<f32>;
    var phi_14491_: vec3<f32>;
    var phi_14492_: bool;
    var phi_27378_: bool;
    var phi_14561_: vec3<f32>;
    var phi_34168_: u32;
    var phi_14563_: vec3<f32>;
    var phi_14564_: bool;
    var phi_34167_: u32;
    var phi_14565_: vec3<f32>;
    var phi_14566_: bool;
    var phi_34165_: u32;
    var phi_14572_: vec3<f32>;
    var phi_14573_: bool;
    var phi_14607_: type_69;
    var phi_34170_: u32;
    var phi_14609_: vec3<f32>;
    var phi_14610_: type_69;
    var phi_14611_: bool;
    var phi_14617_: type_69;
    var phi_34174_: u32;
    var phi_14618_: type_69;
    var phi_34768_: vec3<f32>;
    var phi_34576_: f32;
    var phi_34497_: vec3<f32>;
    var phi_34420_: vec3<f32>;
    var phi_34341_: u32;
    var phi_34259_: f32;
    var phi_34182_: u32;
    var phi_34173_: u32;
    var phi_14620_: type_69;
    var phi_14621_: bool;
    var phi_36097_: bool;
    var phi_34172_: u32;
    var phi_14632_: vec3<f32>;
    var phi_14635_: type_41;
    var phi_14637_: u32;
    var phi_14639_: u32;
    var phi_17803_: u32;
    var phi_17806_: u32;
    var phi_17804_: u32;
    var phi_17807_: u32;
    var local_73: u32;
    var phi_17844_: u32;
    var phi_17847_: u32;
    var phi_17845_: u32;
    var phi_17848_: u32;
    var local_74: u32;
    var local_75: u32;
    var phi_17924_: f32;
    var local_76: u32;
    var local_77: u32;
    var phi_17973_: f32;
    var phi_18129_: f32;
    var phi_18136_: vec3<f32>;
    var phi_36517_: u32;
    var phi_18138_: f32;
    var phi_18139_: vec3<f32>;
    var phi_18140_: vec3<f32>;
    var phi_16801_: u32;
    var phi_16804_: u32;
    var phi_16802_: u32;
    var phi_16805_: u32;
    var local_78: u32;
    var local_79: u32;
    var phi_17574_: f32;
    var phi_17577_: f32;
    var phi_27651_: bool;
    var phi_27632_: f32;
    var phi_17578_: f32;
    var phi_27666_: bool;
    var phi_27689_: bool;
    var phi_27704_: bool;
    var phi_17087_: f32;
    var phi_27719_: bool;
    var phi_27734_: bool;
    var phi_27749_: bool;
    var phi_27814_: bool;
    var phi_27902_: bool;
    var phi_27883_: f32;
    var phi_17152_: type_72;
    var phi_17580_: type_72;
    var phi_17756_: bool;
    var phi_17765_: vec3<f32>;
    var phi_17767_: vec3<f32>;
    var phi_17768_: bool;
    var phi_17773_: f32;
    var phi_17774_: vec3<f32>;
    var phi_36525_: u32;
    var phi_17775_: f32;
    var phi_17776_: vec3<f32>;
    var phi_17777_: vec3<f32>;
    var phi_17778_: f32;
    var phi_36524_: u32;
    var phi_17779_: f32;
    var phi_17780_: vec3<f32>;
    var phi_17781_: vec3<f32>;
    var phi_17782_: f32;
    var phi_36516_: u32;
    var phi_18141_: f32;
    var phi_18142_: vec3<f32>;
    var phi_18143_: vec3<f32>;
    var phi_18144_: f32;
    var phi_27917_: bool;
    var phi_28027_: f32;
    var phi_28055_: f32;
    var phi_28083_: f32;
    var phi_28217_: bool;
    var phi_28232_: bool;
    var phi_28247_: bool;
    var phi_28274_: bool;
    var phi_28289_: bool;
    var phi_28304_: bool;
    var phi_28331_: bool;
    var phi_28346_: bool;
    var phi_28361_: bool;
    var phi_14973_: f32;
    var phi_14981_: f32;
    var phi_14989_: f32;
    var phi_28558_: bool;
    var phi_28547_: f32;
    var phi_15096_: vec3<f32>;
    var phi_28586_: bool;
    var phi_15097_: vec3<f32>;
    var phi_15109_: type_59;
    var phi_15110_: type_59;
    var phi_15111_: bool;
    var phi_15116_: type_59;
    var phi_28623_: bool;
    var phi_28653_: bool;
    var phi_28642_: f32;
    var phi_28692_: vec3<f32>;
    var phi_15144_: type_59;
    var phi_15145_: type_59;
    var phi_28714_: vec3<f32>;
    var phi_15146_: type_59;
    var phi_15147_: type_59;
    var phi_15148_: bool;
    var phi_15152_: type_59;
    var phi_28746_: bool;
    var phi_28817_: bool;
    var phi_15972_: f32;
    var phi_28832_: bool;
    var phi_28847_: bool;
    var phi_16445_: vec3<f32>;
    var phi_16446_: vec3<f32>;
    var phi_16447_: bool;
    var phi_16452_: vec3<f32>;
    var phi_16453_: bool;
    var phi_28971_: bool;
    var phi_28986_: bool;
    var phi_16744_: vec3<f32>;
    var phi_16746_: vec3<f32>;
    var phi_16773_: vec3<f32>;
    var phi_16774_: vec3<f32>;
    var phi_15590_: vec3<f32>;
    var phi_15591_: vec3<f32>;
    var phi_15592_: bool;
    var phi_15597_: vec3<f32>;
    var phi_15598_: bool;
    var phi_29148_: bool;
    var phi_29163_: bool;
    var phi_15889_: vec3<f32>;
    var phi_15891_: vec3<f32>;
    var phi_36559_: u32;
    var phi_16775_: f32;
    var phi_16776_: vec3<f32>;
    var phi_16777_: vec3<f32>;
    var phi_36558_: u32;
    var phi_16778_: f32;
    var phi_16779_: vec3<f32>;
    var phi_16780_: vec3<f32>;
    var phi_36515_: u32;
    var phi_18145_: f32;
    var phi_18146_: vec3<f32>;
    var phi_18147_: vec3<f32>;
    var phi_18148_: f32;
    var phi_14735_: f32;
    var phi_14736_: f32;
    var phi_36514_: u32;
    var phi_18149_: f32;
    var phi_18150_: vec3<f32>;
    var phi_18151_: vec3<f32>;
    var phi_18152_: f32;
    var phi_29216_: bool;
    var phi_29326_: f32;
    var phi_29354_: f32;
    var phi_29382_: f32;
    var phi_29516_: bool;
    var phi_29531_: bool;
    var phi_29546_: bool;
    var phi_29573_: bool;
    var phi_29588_: bool;
    var phi_29603_: bool;
    var phi_29630_: bool;
    var phi_29645_: bool;
    var phi_29660_: bool;
    var phi_18375_: f32;
    var phi_18383_: f32;
    var phi_18391_: f32;
    var phi_29857_: bool;
    var phi_29846_: f32;
    var phi_18498_: vec3<f32>;
    var phi_29885_: bool;
    var phi_18499_: vec3<f32>;
    var phi_18511_: type_59;
    var phi_18512_: type_59;
    var phi_18513_: bool;
    var phi_18518_: type_59;
    var phi_29922_: bool;
    var phi_29952_: bool;
    var phi_29941_: f32;
    var phi_29991_: vec3<f32>;
    var phi_18546_: type_59;
    var phi_18547_: type_59;
    var phi_30013_: vec3<f32>;
    var phi_18548_: type_59;
    var phi_18549_: type_59;
    var phi_18550_: bool;
    var phi_18554_: type_59;
    var phi_30027_: f32;
    var phi_30030_: f32;
    var phi_18563_: f32;
    var phi_18596_: u32;
    var phi_18599_: u32;
    var phi_18601_: u32;
    var phi_18603_: u32;
    var phi_18605_: u32;
    var phi_18607_: vec4<f32>;
    var phi_18609_: f32;
    var phi_18996_: u32;
    var phi_18998_: u32;
    var phi_18999_: u32;
    var phi_19001_: bool;
    var phi_19002_: bool;
    var phi_19003_: u32;
    var phi_19004_: u32;
    var phi_19005_: u32;
    var phi_19006_: bool;
    var phi_19007_: bool;
    var phi_19008_: bool;
    var phi_30128_: bool;
    var phi_30158_: bool;
    var phi_30105_: type_56;
    var phi_30108_: type_56;
    var phi_30109_: bool;
    var phi_30114_: type_56;
    var phi_30215_: bool;
    var phi_30230_: bool;
    var phi_30245_: bool;
    var phi_30260_: bool;
    var phi_30192_: type_56;
    var phi_30195_: type_56;
    var phi_30196_: bool;
    var phi_30201_: type_56;
    var phi_30302_: bool;
    var phi_30317_: bool;
    var phi_30332_: bool;
    var phi_30347_: bool;
    var phi_30279_: type_56;
    var phi_30282_: type_56;
    var phi_30283_: bool;
    var phi_30288_: type_56;
    var phi_30083_: bool;
    var phi_30086_: bool;
    var phi_19037_: f32;
    var phi_30449_: bool;
    var phi_30479_: bool;
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
    var phi_30623_: bool;
    var phi_30638_: bool;
    var phi_30653_: bool;
    var phi_30668_: bool;
    var phi_30600_: type_56;
    var phi_30603_: type_56;
    var phi_30604_: bool;
    var phi_30609_: type_56;
    var phi_30404_: bool;
    var phi_30407_: bool;
    var phi_19046_: f32;
    var phi_19057_: u32;
    var phi_19068_: u32;
    var phi_19107_: u32;
    var phi_19108_: u32;
    var phi_19119_: u32;
    var phi_19120_: u32;
    var phi_19121_: u32;
    var phi_19122_: u32;
    var phi_19124_: u32;
    var phi_19125_: u32;
    var phi_19127_: u32;
    var phi_19128_: u32;
    var phi_19129_: bool;
    var phi_19130_: bool;
    var phi_18922_: vec4<f32>;
    var phi_18923_: vec4<f32>;
    var phi_18924_: bool;
    var phi_18928_: vec4<f32>;
    var phi_18929_: bool;
    var phi_18834_: bool;
    var phi_18871_: bool;
    var phi_18877_: f32;
    var phi_18887_: vec4<f32>;
    var phi_18888_: vec4<f32>;
    var phi_18889_: bool;
    var phi_18893_: vec4<f32>;
    var phi_18894_: bool;
    var phi_18898_: vec4<f32>;
    var phi_18899_: bool;
    var phi_18930_: vec4<f32>;
    var phi_18931_: bool;
    var phi_18764_: vec4<f32>;
    var phi_18765_: vec4<f32>;
    var phi_18766_: bool;
    var phi_18770_: vec4<f32>;
    var phi_18771_: bool;
    var phi_18772_: vec4<f32>;
    var phi_18773_: bool;
    var phi_18774_: bool;
    var phi_18782_: vec4<f32>;
    var phi_18783_: bool;
    var phi_18784_: vec4<f32>;
    var phi_18785_: bool;
    var phi_18786_: bool;
    var phi_18794_: vec4<f32>;
    var phi_18795_: bool;
    var phi_18940_: vec4<f32>;
    var phi_18941_: bool;
    var phi_18962_: f32;
    var phi_18968_: u32;
    var phi_18597_: u32;
    var phi_18600_: u32;
    var phi_18602_: u32;
    var phi_18604_: u32;
    var phi_18606_: u32;
    var phi_18608_: vec4<f32>;
    var phi_18610_: f32;
    var phi_19141_: vec4<f32>;
    var phi_19142_: bool;
    var phi_19143_: bool;
    var phi_36025_: bool;
    var phi_21636_: bool;
    var phi_21635_: vec4<f32>;
    var local_80: u32;
    var local_81: vec4<f32>;
    var phi_19148_: type_74;
    var local_82: u32;
    var phi_19154_: type_74;
    var phi_19206_: vec3<f32>;
    var phi_19207_: type_41;
    var phi_19208_: vec3<f32>;
    var phi_19209_: type_41;
    var phi_36467_: bool;
    var phi_19210_: vec3<f32>;
    var phi_19211_: type_41;
    var phi_19212_: u32;
    var phi_36466_: bool;
    var phi_19213_: vec3<f32>;
    var phi_19214_: type_41;
    var phi_19215_: u32;
    var phi_36752_: u32;
    var phi_36465_: bool;
    var phi_14633_: vec3<f32>;
    var phi_14636_: type_41;
    var phi_14638_: u32;
    var phi_14640_: u32;
    var phi_36472_: u32;
    var phi_36464_: bool;
    var phi_30819_: bool;
    var phi_30849_: bool;
    var phi_30838_: f32;
    var phi_30769_: bool;
    var phi_30905_: vec3<f32>;
    var phi_31061_: bool;
    var phi_31076_: bool;
    var phi_31091_: bool;
    var phi_31000_: vec3<f32>;
    var phi_31003_: vec3<f32>;
    var phi_31004_: bool;
    var phi_30774_: vec3<f32>;
    var phi_30778_: vec3<f32>;
    var phi_30779_: bool;
    var phi_31191_: bool;
    var phi_31165_: vec3<f32>;
    var phi_30786_: vec3<f32>;
    var phi_31206_: bool;
    var phi_31316_: f32;
    var phi_31344_: f32;
    var phi_31372_: f32;
    var phi_31506_: bool;
    var phi_31521_: bool;
    var phi_31536_: bool;
    var phi_31563_: bool;
    var phi_31578_: bool;
    var phi_31593_: bool;
    var phi_31620_: bool;
    var phi_31635_: bool;
    var phi_31650_: bool;
    var phi_19434_: f32;
    var phi_19442_: f32;
    var phi_19450_: f32;
    var phi_31847_: bool;
    var phi_31836_: f32;
    var phi_19557_: vec3<f32>;
    var phi_31875_: bool;
    var phi_19558_: vec3<f32>;
    var phi_19570_: type_59;
    var phi_19571_: type_59;
    var phi_19572_: bool;
    var phi_19577_: type_59;
    var phi_31912_: bool;
    var phi_31942_: bool;
    var phi_31931_: f32;
    var phi_31981_: vec3<f32>;
    var phi_19605_: type_59;
    var phi_19606_: type_59;
    var phi_32003_: vec3<f32>;
    var phi_19607_: type_59;
    var phi_19608_: type_59;
    var phi_19609_: bool;
    var phi_19613_: type_59;
    var phi_32028_: bool;
    var phi_19689_: vec3<f32>;
    var phi_38306_: u32;
    var phi_19691_: vec3<f32>;
    var phi_19692_: bool;
    var phi_38305_: u32;
    var phi_19693_: vec3<f32>;
    var phi_19694_: bool;
    var phi_38303_: u32;
    var phi_19700_: vec3<f32>;
    var phi_19701_: bool;
    var local_83: u32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: type_41;
    var phi_19730_: vec3<f32>;
    var local_88: u32;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: type_41;
    var phi_19746_: type_69;
    var phi_19747_: type_69;
    var phi_38310_: u32;
    var phi_19749_: type_69;
    var phi_19750_: type_69;
    var phi_19751_: bool;
    var phi_38309_: u32;
    var phi_19752_: type_69;
    var phi_19753_: type_69;
    var phi_19754_: bool;
    var phi_19755_: bool;
    var local_93: u32;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: vec3<f32>;
    var local_97: type_41;
    var phi_19761_: type_69;
    var local_98: u32;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: type_41;
    var phi_19766_: type_69;
    var phi_19767_: type_69;
    var phi_19768_: bool;
    var phi_19772_: type_69;
    var phi_19773_: type_69;
    var phi_19774_: bool;
    var phi_19778_: type_69;
    var phi_19779_: type_69;
    var phi_19780_: bool;
    var phi_13399_: bool;
    var phi_13408_: vec3<f32>;
    var phi_13410_: vec3<f32>;
    var phi_13411_: bool;
    var phi_13829_: f32;
    var phi_13832_: f32;
    var phi_32278_: bool;
    var phi_32259_: f32;
    var phi_13833_: f32;
    var phi_32296_: f32;
    var phi_32299_: f32;
    var phi_13839_: f32;
    var phi_13840_: f32;
    var phi_13841_: f32;
    var phi_13842_: f32;
    var phi_19781_: type_69;
    var phi_19782_: type_69;
    var phi_19783_: bool;
    var phi_11455_: f32;
    var phi_32346_: f32;
    var phi_32349_: f32;
    var phi_11457_: f32;
    var phi_11458_: f32;
    var phi_11459_: f32;
    var phi_32375_: bool;
    var phi_11513_: f32;
    var phi_32393_: f32;
    var phi_32396_: f32;
    var phi_11515_: f32;
    var phi_11516_: f32;
    var phi_32407_: bool;
    var phi_32422_: bool;
    var phi_11985_: vec3<f32>;
    var phi_11986_: vec3<f32>;
    var phi_11987_: bool;
    var phi_11992_: vec3<f32>;
    var phi_11993_: bool;
    var phi_32546_: bool;
    var phi_32561_: bool;
    var phi_12284_: vec3<f32>;
    var phi_12286_: vec3<f32>;
    var phi_12313_: vec3<f32>;
    var phi_12327_: vec3<f32>;
    var phi_19787_: type_69;
    var phi_19788_: type_69;
    var phi_19789_: bool;
    var phi_19793_: type_69;

    switch bitcast<i32>(0u) {
        default: {
            let _e279 = global;
            let _e289 = arrayLength((&global_9.member));
            let _e293 = global_1.member.member_7[2u];
            let _e297 = (_e279.x + (_e279.y * 4194240u));
            let _e302 = atomicLoad((&global_8.member[((_e293 * 4u) + 3u)]));
            if (_e297 >= _e302) {
                phi_11191_ = type_48(0u, type_48().member_1);
            } else {
                let _e307 = global_1.member.member[0u];
                let _e312 = global_1.member.member[1u];
                let _e318 = atomicLoad((&global_8.member[((12u + ((_e293 * _e307) * _e312)) + _e297)]));
                phi_11191_ = type_48(1u, _e318);
            }
            let _e324 = phi_11191_;
            if (bitcast<i32>(_e324.member) != 0i) {
                let _e329 = (_e324.member_1 < arrayLength((&global_7.member)));
                if _e329 {
                } else {
                    break;
                }
                let _e332 = global_7.member[_e324.member_1];
                let _e346 = vec3<f32>(_e332.member_1.x, _e332.member_1.y, _e332.member_1.z);
                let _e351 = vec3<f32>(_e332.member_2.x, _e332.member_2.y, _e332.member_2.z);
                let _e357 = select(select(u32(_e332.member_2.w), 0u, (_e332.member_2.w < 0f)), 4294967295u, (_e332.member_2.w > 4294967000f));
                let _e361 = vec3<f32>(_e332.member_3.x, _e332.member_3.y, _e332.member_3.z);
                let _e371 = ((_e332.member_4.y & 2u) != 0u);
                let _e373 = ((_e332.member_4.y & 32u) != 0u);
                let _e376 = global_1.member.member[3u];
                let _e379 = global_1.member.member_3[3u];
                let _e382 = global_1.member.member_1[2u];
                let _e385 = (select(select(u32(_e332.member_3.w), 0u, (_e332.member_3.w < 0f)), 4294967295u, (_e332.member_3.w > 4294967000f)) + 1u);
                if (_e332.member_4.z == 4294967295u) {
                    if _e371 {
                        phi_11459_ = 1f;
                    } else {
                        if _e373 {
                            phi_11458_ = 1f;
                        } else {
                            if ((_e379 & 2u) == 0u) {
                                phi_11457_ = 1f;
                            } else {
                                let _e6596 = global_1.member.member_3[1u];
                                let _e6599 = global_1.member.member_3[2u];
                                let _e6602 = ((atan2(_e332.member_1.z, _e332.member_1.x) * 0.15915494f) + 0.5f);
                                let _e6606 = select(_e332.member_1.y, -1f, (_e332.member_1.y < -1f));
                                let _e6609 = acos(select(_e6606, 1f, (_e6606 > 1f)));
                                let _e6611 = f32(_e6596);
                                let _e6612 = ((_e6602 - floor(_e6602)) * _e6611);
                                let _e6617 = select(select(u32(_e6612), 0u, (_e6612 < 0f)), 4294967295u, (_e6612 > 4294967000f));
                                let _e6618 = (_e6596 - 1u);
                                let _e6621 = f32(_e6599);
                                let _e6622 = ((_e6609 * 0.31830987f) * _e6621);
                                let _e6627 = select(select(u32(_e6622), 0u, (_e6622 < 0f)), 4294967295u, (_e6622 > 4294967000f));
                                let _e6628 = (_e6599 - 1u);
                                let _e6630 = select(_e6627, _e6628, (_e6628 < _e6627));
                                let _e6631 = sin(_e6609);
                                if (_e6631 <= 0f) {
                                    phi_11455_ = 0f;
                                } else {
                                    let _e6635 = global_1.member.member_2[3u];
                                    let _e6638 = global_1.member.member_3[0u];
                                    let _e6642 = (_e6635 + _e6630);
                                    let _e6645 = global_6.member[(_e6642 + 1u)];
                                    let _e6647 = global_6.member[_e6642];
                                    let _e6650 = ((_e6638 + (_e6630 * (_e6596 + 1u))) + select(_e6617, _e6618, (_e6618 < _e6617)));
                                    let _e6653 = global_6.member[(_e6650 + 1u)];
                                    let _e6655 = global_6.member[_e6650];
                                    phi_11455_ = ((((_e6645 - _e6647) * _e6621) * ((_e6653 - _e6655) * _e6611)) / (19.73921f * _e6631));
                                }
                                let _e6662 = phi_11455_;
                                if (_e332.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32349_ = 1f;
                                } else {
                                    let _e6664 = (_e332.member_1.w * _e332.member_1.w);
                                    let _e6666 = (_e6664 + (_e6662 * _e6662));
                                    if (_e6666 == 0f) {
                                        phi_32346_ = 0f;
                                    } else {
                                        phi_32346_ = (_e6664 / _e6666);
                                    }
                                    let _e6670 = phi_32346_;
                                    phi_32349_ = _e6670;
                                }
                                let _e6672 = phi_32349_;
                                phi_11457_ = _e6672;
                            }
                            let _e6674 = phi_11457_;
                            phi_11458_ = _e6674;
                        }
                        let _e6676 = phi_11458_;
                        phi_11459_ = _e6676;
                    }
                    let _e6678 = phi_11459_;
                    if ((_e379 & 4u) == 0u) {
                        phi_12327_ = _e361;
                    } else {
                        if _e371 {
                            phi_11516_ = 1f;
                        } else {
                            if _e373 {
                                phi_11515_ = 0f;
                            } else {
                                let _e6689 = global_1.member.member_5;
                                let _e6693 = (_e332.member_1.x - _e6689.x);
                                let _e6694 = (_e332.member_1.y - _e6689.y);
                                let _e6695 = (_e332.member_1.z - _e6689.z);
                                let _e6702 = (sqrt((((_e6693 * _e6693) + (_e6694 * _e6694)) + (_e6695 * _e6695))) * 0.5f);
                                if (_e6702 != _e6702) {
                                    phi_32375_ = true;
                                } else {
                                    phi_32375_ = (1f <= _e6702);
                                }
                                let _e6706 = phi_32375_;
                                let _e6712 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e6702, 1f, _e6706))) >= _e6712) {
                                    phi_11513_ = 0f;
                                } else {
                                    let _e6715 = sin((_e6712 * 0.5f));
                                    phi_11513_ = (1f / ((12.566371f * _e6715) * _e6715));
                                }
                                let _e6720 = phi_11513_;
                                if (_e332.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32396_ = 1f;
                                } else {
                                    let _e6722 = (_e332.member_1.w * _e332.member_1.w);
                                    let _e6724 = (_e6722 + (_e6720 * _e6720));
                                    if (_e6724 == 0f) {
                                        phi_32393_ = 0f;
                                    } else {
                                        phi_32393_ = (_e6722 / _e6724);
                                    }
                                    let _e6728 = phi_32393_;
                                    phi_32396_ = _e6728;
                                }
                                let _e6730 = phi_32396_;
                                phi_11515_ = _e6730;
                            }
                            let _e6732 = phi_11515_;
                            phi_11516_ = _e6732;
                        }
                        let _e6734 = phi_11516_;
                        let _e6736 = global_1.member.member_5;
                        let _e6740 = (_e332.member_1.x - _e6736.x);
                        let _e6741 = (_e332.member_1.y - _e6736.y);
                        let _e6742 = (_e332.member_1.z - _e6736.z);
                        let _e6749 = (sqrt((((_e6740 * _e6740) + (_e6741 * _e6741)) + (_e6742 * _e6742))) * 0.5f);
                        if (_e6749 != _e6749) {
                            phi_32407_ = true;
                        } else {
                            phi_32407_ = (1f <= _e6749);
                        }
                        let _e6753 = phi_32407_;
                        let _e6756 = (2f * asin(select(_e6749, 1f, _e6753)));
                        let _e6760 = global_1.member.member_6[3u];
                        if (_e6756 >= _e6760) {
                            phi_12313_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e6764 = (1f - pow((_e6756 / _e6760), 2f));
                            if (_e6764 != _e6764) {
                                phi_32422_ = true;
                            } else {
                                phi_32422_ = (0f >= _e6764);
                            }
                            let _e6768 = phi_32422_;
                            let _e6771 = global_1.member.member_6;
                            let _e6777 = global_1.member.member_8[2u];
                            if (_e6777 == 4294967295u) {
                                phi_12286_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e6780 = global_6.member[_e6777];
                                let _e6783 = global_6.member[(_e6777 + 1u)];
                                let _e6786 = global_6.member[(_e6777 + 2u)];
                                let _e6789 = global_6.member[(_e6777 + 3u)];
                                let _e6792 = global_6.member[(_e6777 + 4u)];
                                let _e6795 = global_6.member[(_e6777 + 5u)];
                                let _e6798 = global_6.member[(_e6777 + 6u)];
                                let _e6801 = global_6.member[(_e6777 + 7u)];
                                let _e6804 = global_6.member[(_e6777 + 8u)];
                                let _e6807 = global_6.member[(_e6777 + 9u)];
                                let _e6810 = global_6.member[(_e6777 + 13u)];
                                let _e6813 = global_6.member[(_e6777 + 14u)];
                                let _e6816 = global_6.member[(_e6777 + 15u)];
                                let _e6819 = global_6.member[(_e6777 + 16u)];
                                let _e6822 = global_6.member[(_e6777 + 18u)];
                                let _e6825 = global_6.member[(_e6777 + 19u)];
                                let _e6828 = global_6.member[(_e6777 + 20u)];
                                let _e6831 = global_6.member[(_e6777 + 29u)];
                                let _e6832 = bitcast<u32>(_e6831);
                                let _e6845 = (((((_e332.member.x * _e332.member.x) + (_e332.member.y * _e332.member.y)) + (_e332.member.z * _e332.member.z)) - (2f * (((_e332.member.x * _e6780) + (_e332.member.y * _e6783)) + (_e332.member.z * _e6786)))) + _e6798);
                                if (_e6845 > 0f) {
                                    let _e6854 = ((((_e332.member.x - _e6780) * _e6801) + ((_e332.member.y - _e6783) * _e6804)) + ((_e332.member.z - _e6786) * _e6807));
                                    let _e6856 = ((_e6854 * _e6854) - _e6845);
                                    if (_e6854 >= 0f) {
                                        phi_11986_ = vec3<f32>();
                                        phi_11987_ = true;
                                    } else {
                                        let _e6858 = (_e6856 <= 0f);
                                        if _e6858 {
                                            phi_11985_ = vec3<f32>();
                                        } else {
                                            let _e6861 = (-(_e6854) - sqrt(_e6856));
                                            phi_11985_ = vec3<f32>((_e332.member.x + (_e6801 * _e6861)), (_e332.member.y + (_e6804 * _e6861)), (_e332.member.z + (_e6807 * _e6861)));
                                        }
                                        let _e6870 = phi_11985_;
                                        phi_11986_ = _e6870;
                                        phi_11987_ = _e6858;
                                    }
                                    let _e6872 = phi_11986_;
                                    let _e6874 = phi_11987_;
                                    phi_11992_ = _e6872;
                                    phi_11993_ = select(true, false, _e6874);
                                } else {
                                    phi_11992_ = vec3<f32>(_e332.member.x, _e332.member.y, _e332.member.z);
                                    phi_11993_ = true;
                                }
                                let _e6877 = phi_11992_;
                                let _e6879 = phi_11993_;
                                if _e6879 {
                                    let _e6881 = (_e6877.x - _e6780);
                                    let _e6883 = (_e6877.y - _e6783);
                                    let _e6885 = (_e6877.z - _e6786);
                                    let _e6891 = sqrt((((_e6881 * _e6881) + (_e6883 * _e6883)) + (_e6885 * _e6885)));
                                    let _e6892 = (1f / _e6891);
                                    let _e6910 = ((((((_e6877.x * _e6877.x) + (_e6877.y * _e6877.y)) + (_e6877.z * _e6877.z)) - (2f * (((_e6877.x * _e6780) + (_e6877.y * _e6783)) + (_e6877.z * _e6786)))) + _e6795) / (_e6891 + _e6789));
                                    if (_e6910 != _e6910) {
                                        phi_32546_ = true;
                                    } else {
                                        phi_32546_ = (0f >= _e6910);
                                    }
                                    let _e6914 = phi_32546_;
                                    let _e6918 = sqrt((select(_e6910, 0f, _e6914) / (_e6792 - _e6789)));
                                    if (_e6918 != _e6918) {
                                        phi_32561_ = true;
                                    } else {
                                        phi_32561_ = (1f <= _e6918);
                                    }
                                    let _e6922 = phi_32561_;
                                    let _e6924 = (select(_e6918, 1f, _e6922) * 63f);
                                    let _e6929 = ((((_e6881 * _e6892) * _e6801) + ((_e6883 * _e6892) * _e6804)) + ((_e6885 * _e6892) * _e6807));
                                    let _e6931 = select(_e6929, -1f, (_e6929 < -1f));
                                    let _e6935 = ((select(_e6931, 1f, (_e6931 > 1f)) + 1f) * 127.5f);
                                    let _e6940 = select(select(u32(_e6924), 0u, (_e6924 < 0f)), 4294967295u, (_e6924 > 4294967000f));
                                    let _e6942 = select(_e6940, 62u, (62u < _e6940));
                                    let _e6947 = select(select(u32(_e6935), 0u, (_e6935 < 0f)), 4294967295u, (_e6935 > 4294967000f));
                                    let _e6949 = select(_e6947, 254u, (254u < _e6947));
                                    let _e6951 = (_e6924 - f32(_e6942));
                                    let _e6953 = (_e6935 - f32(_e6949));
                                    let _e6957 = (_e6832 + (3u * ((_e6942 * 256u) + _e6949)));
                                    let _e6962 = (_e6832 + (3u * (((_e6942 + 1u) * 256u) + _e6949)));
                                    let _e6964 = global_6.member[_e6957];
                                    let _e6967 = global_6.member[(_e6957 + 1u)];
                                    let _e6970 = global_6.member[(_e6957 + 2u)];
                                    let _e6971 = (1f - _e6953);
                                    let _e6977 = global_6.member[(_e6957 + 3u)];
                                    let _e6980 = global_6.member[(_e6957 + 4u)];
                                    let _e6983 = global_6.member[(_e6957 + 5u)];
                                    let _e6990 = (1f - _e6951);
                                    let _e6995 = global_6.member[_e6962];
                                    let _e6998 = global_6.member[(_e6962 + 1u)];
                                    let _e7001 = global_6.member[(_e6962 + 2u)];
                                    let _e7007 = global_6.member[(_e6962 + 3u)];
                                    let _e7010 = global_6.member[(_e6962 + 4u)];
                                    let _e7013 = global_6.member[(_e6962 + 5u)];
                                    phi_12284_ = vec3<f32>(((((_e6964 * _e6971) + (_e6977 * _e6953)) * _e6990) + (((_e6995 * _e6971) + (_e7007 * _e6953)) * _e6951)), ((((_e6967 * _e6971) + (_e6980 * _e6953)) * _e6990) + (((_e6998 * _e6971) + (_e7010 * _e6953)) * _e6951)), ((((_e6970 * _e6971) + (_e6983 * _e6953)) * _e6990) + (((_e7001 * _e6971) + (_e7013 * _e6953)) * _e6951)));
                                } else {
                                    phi_12284_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e7028 = phi_12284_;
                                let _e7035 = ((_e6819 * 1.1f) * _e7028.y);
                                phi_12286_ = vec3<f32>(exp(-((((_e6810 * _e7028.x) + _e7035) + (_e6822 * _e7028.z)))), exp(-((((_e6813 * _e7028.x) + _e7035) + (_e6825 * _e7028.z)))), exp(-((((_e6816 * _e7028.x) + _e7035) + (_e6828 * _e7028.z)))));
                            }
                            let _e7054 = phi_12286_;
                            let _e7066 = sin((_e6760 * 0.5f));
                            let _e7069 = (((1f - (0.6f * (1f - sqrt(select(_e6764, 0f, _e6768))))) * 1.25f) / ((12.566371f * _e7066) * _e7066));
                            phi_12313_ = vec3<f32>(((_e6771.x * _e7054.x) * _e7069), ((_e6771.y * _e7054.y) * _e7069), ((_e6771.z * _e7054.z) * _e7069));
                        }
                        let _e7075 = phi_12313_;
                        phi_12327_ = vec3<f32>((_e332.member_3.x + ((_e332.member_2.x * _e7075.x) * _e6734)), (_e332.member_3.y + ((_e332.member_2.y * _e7075.y) * _e6734)), (_e332.member_3.z + ((_e332.member_2.z * _e7075.z) * _e6734)));
                    }
                    let _e7090 = phi_12327_;
                    phi_19787_ = type_69(type_34(_e332.member, _e332.member_1, vec4<f32>(_e332.member_2.x, _e332.member_2.y, _e332.member_2.z, f32(_e357)), vec4<f32>(_e7090.x, _e7090.y, _e7090.z, f32(_e385)), vec4<u32>(_e332.member_4.x, (_e332.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), type_41(vec4<f32>(_e332.member.x, _e332.member.y, _e332.member.z, -1f), vec4<f32>(_e332.member_1.x, _e332.member_1.y, _e332.member_1.z, 0f), vec4<f32>((_e332.member_2.x * _e6678), (_e332.member_2.y * _e6678), (_e332.member_2.z * _e6678), 0f)), type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_19788_ = type_69();
                    phi_19789_ = true;
                } else {
                    if ((_e379 & 8u) == 0u) {
                        phi_36120_ = false;
                        phi_12861_ = _e351;
                        phi_12862_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    } else {
                        if (_e332.member_5.x > 100f) {
                            let _e396 = global_1.member.member_8[1u];
                            let _e398 = global_6.member[_e396];
                            let _e401 = global_6.member[(_e396 + 1u)];
                            let _e404 = global_6.member[(_e396 + 2u)];
                            let _e407 = global_6.member[(_e396 + 3u)];
                            let _e410 = global_6.member[(_e396 + 5u)];
                            let _e413 = global_6.member[(_e396 + 6u)];
                            let _e416 = global_6.member[(_e396 + 10u)];
                            let _e419 = global_6.member[(_e396 + 11u)];
                            let _e422 = global_6.member[(_e396 + 13u)];
                            let _e425 = global_6.member[(_e396 + 14u)];
                            let _e428 = global_6.member[(_e396 + 15u)];
                            let _e431 = global_6.member[(_e396 + 16u)];
                            let _e434 = global_6.member[(_e396 + 18u)];
                            let _e437 = global_6.member[(_e396 + 19u)];
                            let _e440 = global_6.member[(_e396 + 20u)];
                            let _e448 = ((((_e332.member.x - _e398) * _e332.member_1.x) + ((_e332.member.y - _e401) * _e332.member_1.y)) + ((_e332.member.z - _e404) * _e332.member_1.z));
                            let _e460 = ((((_e332.member.x * _e332.member.x) + (_e332.member.y * _e332.member.y)) + (_e332.member.z * _e332.member.z)) - (2f * (((_e332.member.x * _e398) + (_e332.member.y * _e401)) + (_e332.member.z * _e404))));
                            let _e461 = (_e460 + _e413);
                            let _e462 = (_e448 * _e448);
                            let _e463 = (_e462 - _e461);
                            if (_e463 < 0f) {
                                phi_26092_ = type_57(0u, type_57().member_1);
                                phi_26093_ = type_57();
                                phi_26094_ = true;
                            } else {
                                if (_e448 < 0f) {
                                    phi_26064_ = -(sqrt(_e463));
                                } else {
                                    phi_26064_ = sqrt(_e463);
                                }
                                let _e470 = phi_26064_;
                                let _e472 = -((_e448 + _e470));
                                let _e473 = (_e472 == 0f);
                                if _e473 {
                                    phi_26087_ = type_57(1u, type_56(0f, 0f));
                                    phi_26088_ = type_57();
                                } else {
                                    let _e474 = (_e461 / _e472);
                                    if (_e472 < _e474) {
                                        phi_26076_ = type_56(_e472, _e474);
                                    } else {
                                        phi_26076_ = type_56(_e474, _e472);
                                    }
                                    let _e479 = phi_26076_;
                                    phi_26087_ = type_57();
                                    phi_26088_ = type_57(1u, _e479);
                                }
                                let _e482 = phi_26087_;
                                let _e484 = phi_26088_;
                                phi_26092_ = _e482;
                                phi_26093_ = _e484;
                                phi_26094_ = _e473;
                            }
                            let _e489 = phi_26092_;
                            let _e491 = phi_26093_;
                            let _e493 = phi_26094_;
                            if _e493 {
                                phi_26098_ = _e489;
                            } else {
                                phi_26098_ = _e491;
                            }
                            let _e495 = phi_26098_;
                            if (bitcast<i32>(_e495.member) != 0i) {
                                phi_25985_ = type_57(0u, _e495.member_1);
                            } else {
                                phi_25985_ = type_57(1u, type_57().member_1);
                            }
                            let _e505 = phi_25985_;
                            if (bitcast<i32>(_e505.member) != 0i) {
                                phi_26032_ = type_57(0u, type_57().member_1);
                            } else {
                                if (_e505.member_1.member_1 <= 0f) {
                                    phi_26028_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e505.member_1.member != _e505.member_1.member) {
                                        phi_26136_ = true;
                                    } else {
                                        phi_26136_ = (0f >= _e505.member_1.member);
                                    }
                                    let _e517 = phi_26136_;
                                    let _e518 = select(_e505.member_1.member, 0f, _e517);
                                    let _e519 = (_e460 + _e410);
                                    if (_e519 > 0f) {
                                        let _e521 = (_e462 - _e519);
                                        if (_e521 < 0f) {
                                            phi_26244_ = type_57(0u, type_57().member_1);
                                            phi_26245_ = type_57();
                                            phi_26246_ = true;
                                        } else {
                                            if (_e448 < 0f) {
                                                phi_26216_ = -(sqrt(_e521));
                                            } else {
                                                phi_26216_ = sqrt(_e521);
                                            }
                                            let _e528 = phi_26216_;
                                            let _e530 = -((_e448 + _e528));
                                            let _e531 = (_e530 == 0f);
                                            if _e531 {
                                                phi_26239_ = type_57(1u, type_56(0f, 0f));
                                                phi_26240_ = type_57();
                                            } else {
                                                let _e532 = (_e519 / _e530);
                                                if (_e530 < _e532) {
                                                    phi_26228_ = type_56(_e530, _e532);
                                                } else {
                                                    phi_26228_ = type_56(_e532, _e530);
                                                }
                                                let _e537 = phi_26228_;
                                                phi_26239_ = type_57();
                                                phi_26240_ = type_57(1u, _e537);
                                            }
                                            let _e540 = phi_26239_;
                                            let _e542 = phi_26240_;
                                            phi_26244_ = _e540;
                                            phi_26245_ = _e542;
                                            phi_26246_ = _e531;
                                        }
                                        let _e547 = phi_26244_;
                                        let _e549 = phi_26245_;
                                        let _e551 = phi_26246_;
                                        if _e551 {
                                            phi_26250_ = _e547;
                                        } else {
                                            phi_26250_ = _e549;
                                        }
                                        let _e553 = phi_26250_;
                                        if (bitcast<i32>(_e553.member) != 0i) {
                                            phi_26184_ = select(_e553.member_1.member_1, _e553.member_1.member, (_e553.member_1.member >= 0f));
                                        } else {
                                            phi_26184_ = -1f;
                                        }
                                        let _e564 = phi_26184_;
                                        if (_e564 >= 0f) {
                                            if (_e505.member_1.member_1 != _e505.member_1.member_1) {
                                                phi_26288_ = true;
                                            } else {
                                                phi_26288_ = (_e564 <= _e505.member_1.member_1);
                                            }
                                            let _e569 = phi_26288_;
                                            phi_26013_ = select(_e505.member_1.member_1, _e564, _e569);
                                        } else {
                                            phi_26013_ = _e505.member_1.member_1;
                                        }
                                        let _e572 = phi_26013_;
                                        phi_26015_ = _e572;
                                    } else {
                                        phi_26015_ = _e505.member_1.member_1;
                                    }
                                    let _e574 = phi_26015_;
                                    if (_e574 <= _e518) {
                                        phi_26024_ = type_57(0u, type_57().member_1);
                                    } else {
                                        phi_26024_ = type_57(1u, type_56(_e518, _e574));
                                    }
                                    let _e582 = phi_26024_;
                                    phi_26028_ = _e582;
                                }
                                let _e587 = phi_26028_;
                                phi_26032_ = _e587;
                            }
                            let _e592 = phi_26032_;
                            if (bitcast<i32>(_e592.member) != 0i) {
                                let _e598 = (_e592.member_1.member < _e332.member_5.x);
                                if _e598 {
                                    if (_e592.member_1.member_1 != _e592.member_1.member_1) {
                                        phi_26303_ = true;
                                    } else {
                                        phi_26303_ = (_e332.member_5.x <= _e592.member_1.member_1);
                                    }
                                    let _e604 = phi_26303_;
                                    let _e605 = select(_e592.member_1.member_1, _e332.member_5.x, _e604);
                                    phi_12720_ = vec3<f32>(0f, 0f, 0f);
                                    phi_12723_ = 0u;
                                    loop {
                                        let _e607 = phi_12720_;
                                        let _e609 = phi_12723_;
                                        local_69 = _e607;
                                        local_70 = _e607;
                                        local_71 = _e607;
                                        let _e610 = (_e609 < 32u);
                                        if _e610 {
                                            let _e611 = -(_e448);
                                            if (_e592.member_1.member <= _e605) {
                                            } else {
                                                phi_33984_ = true;
                                                break;
                                            }
                                            let _e614 = select(_e611, _e592.member_1.member, (_e611 < _e592.member_1.member));
                                            let _e616 = select(_e614, _e605, (_e614 > _e605));
                                            let _e617 = (_e616 - _e592.member_1.member);
                                            let _e618 = (_e605 - _e616);
                                            let _e619 = (_e605 - _e592.member_1.member);
                                            let _e620 = (0.000000001f * _e619);
                                            if (_e617 <= _e620) {
                                                let _e695 = (f32(_e609) * 0.03125f);
                                                let _e698 = (f32((_e609 + 1u)) * 0.03125f);
                                                let _e700 = ((_e695 + _e698) * 0.5f);
                                                phi_12824_ = type_56((_e592.member_1.member + ((_e619 * _e700) * _e700)), (_e619 * ((_e698 * _e698) - (_e695 * _e695))));
                                            } else {
                                                if (_e618 <= _e620) {
                                                    let _e678 = (f32((31u - _e609)) * 0.03125f);
                                                    let _e681 = (f32((32u - _e609)) * 0.03125f);
                                                    let _e683 = ((_e678 + _e681) * 0.5f);
                                                    phi_12823_ = type_56((_e605 - ((_e619 * _e683) * _e683)), (_e619 * ((_e681 * _e681) - (_e678 * _e678))));
                                                } else {
                                                    let _e626 = round(((32f * _e617) / (_e617 + _e618)));
                                                    let _e631 = select(select(u32(_e626), 0u, (_e626 < 0f)), 4294967295u, (_e626 > 4294967000f));
                                                    if (_e631 < 1u) {
                                                        phi_12807_ = 1u;
                                                    } else {
                                                        phi_12807_ = select(_e631, 31u, (_e631 > 31u));
                                                    }
                                                    let _e636 = phi_12807_;
                                                    if (_e609 < _e636) {
                                                        let _e657 = ((_e636 - 1u) - _e609);
                                                        let _e659 = f32(_e636);
                                                        let _e660 = (f32(_e657) / _e659);
                                                        let _e663 = (f32((_e657 + 1u)) / _e659);
                                                        let _e665 = ((_e660 + _e663) * 0.5f);
                                                        phi_12822_ = type_56((_e616 - ((_e617 * _e665) * _e665)), (_e617 * ((_e663 * _e663) - (_e660 * _e660))));
                                                    } else {
                                                        let _e639 = (_e609 - _e636);
                                                        let _e641 = f32((32u - _e636));
                                                        let _e642 = (f32(_e639) / _e641);
                                                        let _e645 = (f32((_e639 + 1u)) / _e641);
                                                        let _e647 = ((_e642 + _e645) * 0.5f);
                                                        phi_12822_ = type_56((_e616 + ((_e618 * _e647) * _e647)), (_e618 * ((_e645 * _e645) - (_e642 * _e642))));
                                                    }
                                                    let _e675 = phi_12822_;
                                                    phi_12823_ = _e675;
                                                }
                                                let _e693 = phi_12823_;
                                                phi_12824_ = _e693;
                                            }
                                            let _e710 = phi_12824_;
                                            let _e716 = (_e332.member.x + (_e332.member_1.x * _e710.member));
                                            let _e717 = (_e332.member.y + (_e332.member_1.y * _e710.member));
                                            let _e718 = (_e332.member.z + (_e332.member_1.z * _e710.member));
                                            let _e732 = (_e716 - _e398);
                                            let _e733 = (_e717 - _e401);
                                            let _e734 = (_e718 - _e404);
                                            let _e742 = ((((((_e716 * _e716) + (_e717 * _e717)) + (_e718 * _e718)) - (2f * (((_e716 * _e398) + (_e717 * _e401)) + (_e718 * _e404)))) + _e410) / (sqrt((((_e732 * _e732) + (_e733 * _e733)) + (_e734 * _e734))) + _e407));
                                            let _e746 = (1f - (abs((_e742 - 25000f)) * 0.00006666667f));
                                            if (_e746 != _e746) {
                                                phi_26502_ = true;
                                            } else {
                                                phi_26502_ = (0f >= _e746);
                                            }
                                            let _e750 = phi_26502_;
                                            let _e752 = -(_e742);
                                            phi_12721_ = (_e607 + vec3<f32>((exp((_e752 / _e416)) * _e710.member_1), (exp((_e752 / _e419)) * _e710.member_1), (select(_e746, 0f, _e750) * _e710.member_1)));
                                            phi_12724_ = (_e609 + 1u);
                                        } else {
                                            phi_12721_ = vec3<f32>();
                                            phi_12724_ = u32();
                                        }
                                        let _e764 = phi_12721_;
                                        let _e766 = phi_12724_;
                                        phi_12720_ = _e764;
                                        phi_12723_ = _e766;
                                        phi_33984_ = false;
                                        if !(_e610) {
                                            break;
                                        }
                                    }
                                    let _e769 = phi_33984_;
                                    if _e769 {
                                        break;
                                    }
                                    let _e771 = local_69;
                                    let _e778 = local_70;
                                    let _e780 = ((_e431 * 1.1f) * _e778.y);
                                    let _e785 = local_71;
                                    phi_36124_ = _e769;
                                    phi_12849_ = vec3<f32>(exp(-((((_e422 * _e771.x) + _e780) + (_e434 * _e785.z)))), exp(-((((_e425 * _e771.x) + _e780) + (_e437 * _e785.z)))), exp(-((((_e428 * _e771.x) + _e780) + (_e440 * _e785.z)))));
                                } else {
                                    phi_36124_ = false;
                                    phi_12849_ = vec3<f32>();
                                }
                                let _e801 = phi_36124_;
                                let _e803 = phi_12849_;
                                phi_36123_ = _e801;
                                phi_12851_ = _e803;
                                phi_12852_ = select(true, false, _e598);
                            } else {
                                phi_36123_ = false;
                                phi_12851_ = vec3<f32>();
                                phi_12852_ = true;
                            }
                            let _e806 = phi_36123_;
                            let _e808 = phi_12851_;
                            let _e810 = phi_12852_;
                            phi_36121_ = _e806;
                            phi_12859_ = (_e351 * select(_e808, vec3<f32>(1f, 1f, 1f), vec3(_e810)));
                            phi_12860_ = type_41(vec4<f32>(_e332.member.x, _e332.member.y, _e332.member.z, _e332.member_5.x), vec4<f32>(_e332.member_1.x, _e332.member_1.y, _e332.member_1.z, 0f), vec4<f32>(_e332.member_2.x, _e332.member_2.y, _e332.member_2.z, 0f));
                        } else {
                            phi_36121_ = false;
                            phi_12859_ = _e351;
                            phi_12860_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e815 = phi_36121_;
                        let _e817 = phi_12859_;
                        let _e819 = phi_12860_;
                        phi_36120_ = _e815;
                        phi_12861_ = _e817;
                        phi_12862_ = _e819;
                    }
                    let _e821 = phi_36120_;
                    let _e823 = phi_12861_;
                    let _e825 = phi_12862_;
                    let _e829 = (_e332.member.x + (_e332.member_1.x * _e332.member_5.x));
                    let _e830 = (_e332.member.y + (_e332.member_1.y * _e332.member_5.x));
                    let _e831 = (_e332.member.z + (_e332.member_1.z * _e332.member_5.x));
                    let _e834 = global_1.member.member_1[0u];
                    let _e837 = global_1.member.member_1[1u];
                    let _e838 = (_e834 + _e837);
                    let _e839 = (3u * _e332.member_4.z);
                    let _e841 = global_2.member[_e839];
                    let _e844 = global_2.member[(_e839 + 1u)];
                    let _e850 = select(select(u32(_e844.w), 0u, (_e844.w < 0f)), 4294967295u, (_e844.w > 4294967000f));
                    let _e851 = (3u * _e838);
                    let _e853 = (_e851 + (5u * _e332.member_4.z));
                    let _e855 = global_2.member[_e853];
                    let _e858 = global_2.member[(_e853 + 1u)];
                    let _e861 = global_2.member[(_e853 + 2u)];
                    let _e873 = global_2.member[(_e853 + 3u)];
                    let _e876 = global_2.member[(_e853 + 4u)];
                    let _e883 = (_e850 == 0u);
                    if _e883 {
                        let _e906 = (_e855.x - _e841.x);
                        let _e907 = (_e855.y - _e841.y);
                        let _e908 = (_e855.z - _e841.z);
                        let _e909 = (_e858.x - _e841.x);
                        let _e910 = (_e858.y - _e841.y);
                        let _e911 = (_e858.z - _e841.z);
                        let _e914 = ((_e907 * _e911) - (_e910 * _e908));
                        let _e917 = ((_e908 * _e909) - (_e911 * _e906));
                        let _e920 = ((_e906 * _e910) - (_e909 * _e907));
                        let _e927 = (1f / sqrt((((_e914 * _e914) + (_e917 * _e917)) + (_e920 * _e920))));
                        let _e931 = vec3<f32>((_e914 * _e927), (_e917 * _e927), (_e920 * _e927));
                        if (select(select(u32(_e861.w), 0u, (_e861.w < 0f)), 4294967295u, (_e861.w > 4294967000f)) == 1u) {
                            let _e936 = ((1f - _e332.member_5.y) - _e332.member_5.z);
                            let _e955 = (((_e861.x * _e936) + (_e873.x * _e332.member_5.y)) + (_e876.x * _e332.member_5.z));
                            let _e956 = (((_e861.y * _e936) + (_e873.y * _e332.member_5.y)) + (_e876.y * _e332.member_5.z));
                            let _e957 = (((_e861.z * _e936) + (_e873.z * _e332.member_5.y)) + (_e876.z * _e332.member_5.z));
                            let _e962 = (((_e955 * _e955) + (_e956 * _e956)) + (_e957 * _e957));
                            if (_e962 > 0f) {
                                let _e965 = (1f / sqrt(_e962));
                                phi_13108_ = vec3<f32>((_e955 * _e965), (_e956 * _e965), (_e957 * _e965));
                            } else {
                                phi_13108_ = _e931;
                            }
                            let _e971 = phi_13108_;
                            phi_13109_ = _e971;
                        } else {
                            phi_13109_ = _e931;
                        }
                        let _e973 = phi_13109_;
                        phi_13142_ = type_60(_e931, _e973);
                    } else {
                        if (_e850 == 1u) {
                            let _e899 = vec3<f32>(((_e829 - _e841.x) / _e841.w), ((_e830 - _e841.y) / _e841.w), ((_e831 - _e841.z) / _e841.w));
                            phi_13141_ = type_60(_e899, _e899);
                        } else {
                            let _e887 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e332.member_1.y > 0f)));
                            phi_13141_ = type_60(_e887, _e887);
                        }
                        let _e902 = phi_13141_;
                        phi_13142_ = _e902;
                    }
                    let _e976 = phi_13142_;
                    let _e990 = select(select(u32(_e858.w), 0u, (_e858.w < 0f)), 4294967295u, (_e858.w > 4294967000f));
                    phi_13265_ = 0i;
                    phi_13268_ = select(select(u32(_e855.w), 0u, (_e855.w < 0f)), 4294967295u, (_e855.w > 4294967000f));
                    loop {
                        let _e992 = phi_13265_;
                        let _e994 = phi_13268_;
                        local_72 = _e994;
                        if (_e992 < 4i) {
                            let _e997 = global_4.member[_e994];
                            let _e999 = (_e997.member != 6u);
                            if _e999 {
                                phi_13337_ = i32();
                                phi_13338_ = u32();
                            } else {
                                let _e1003 = (abs(_e829) / _e997.member_6.z);
                                let _e1014 = (abs(_e831) / _e997.member_6.w);
                                if ((select(select(u32(_e1003), 0u, (_e1003 < 0f)), 4294967295u, (_e1003 > 4294967000f)) % 2u) == 0u) {
                                    phi_13322_ = (_e829 > 0f);
                                } else {
                                    phi_13322_ = (_e829 <= 0f);
                                }
                                let _e1025 = phi_13322_;
                                if ((select(select(u32(_e1014), 0u, (_e1014 < 0f)), 4294967295u, (_e1014 > 4294967000f)) % 2u) == 0u) {
                                    phi_13328_ = (_e831 > 0f);
                                } else {
                                    phi_13328_ = (_e831 <= 0f);
                                }
                                let _e1029 = phi_13328_;
                                if (_e1025 != _e1029) {
                                    phi_13335_ = _e997.member_1;
                                } else {
                                    phi_13335_ = _e997.member_2;
                                }
                                let _e1034 = phi_13335_;
                                phi_13337_ = (_e992 + 1i);
                                phi_13338_ = _e1034;
                            }
                            let _e1037 = phi_13337_;
                            let _e1039 = phi_13338_;
                            phi_13266_ = _e1037;
                            phi_13269_ = _e1039;
                            phi_13340_ = select(true, false, _e999);
                        } else {
                            phi_13266_ = i32();
                            phi_13269_ = u32();
                            phi_13340_ = false;
                        }
                        let _e1042 = phi_13266_;
                        let _e1044 = phi_13269_;
                        let _e1046 = phi_13340_;
                        phi_13265_ = _e1042;
                        phi_13268_ = _e1044;
                        if !(_e1046) {
                            break;
                        }
                    }
                    let _e1049 = local_72;
                    let _e1051 = global_4.member[_e1049];
                    let _e1053 = (_e1051.member == 4u);
                    if _e1053 {
                        if _e1053 {
                            if (_e1051.member_1 == 1u) {
                                phi_13399_ = true;
                            } else {
                                phi_13399_ = ((((-(_e332.member_1.x) * _e976.member_1.x) + (-(_e332.member_1.y) * _e976.member_1.y)) + (-(_e332.member_1.z) * _e976.member_1.z)) > 0f);
                            }
                            let _e6408 = phi_13399_;
                            if _e6408 {
                                phi_13408_ = vec3<f32>(_e1051.member_4.x, _e1051.member_4.y, _e1051.member_4.z);
                            } else {
                                phi_13408_ = vec3<f32>();
                            }
                            let _e6415 = phi_13408_;
                            phi_13410_ = _e6415;
                            phi_13411_ = select(true, false, _e6408);
                        } else {
                            phi_13410_ = vec3<f32>();
                            phi_13411_ = true;
                        }
                        let _e6418 = phi_13410_;
                        let _e6420 = phi_13411_;
                        let _e6422 = select(_e6418, vec3<f32>(0f, 0f, 0f), vec3(_e6420));
                        if _e371 {
                            phi_13842_ = 1f;
                        } else {
                            if ((_e379 & 1u) == 0u) {
                                phi_13841_ = 1f;
                            } else {
                                if (_e990 > 0u) {
                                    let _e6429 = global_1.member.member_2[1u];
                                    let _e6430 = (_e6429 + (_e990 - 1u));
                                    let _e6433 = global_6.member[(_e6430 + 1u)];
                                    let _e6435 = global_6.member[_e6430];
                                    if (_e850 == 1u) {
                                        let _e6496 = (_e841.x - _e332.member.x);
                                        let _e6497 = (_e841.y - _e332.member.y);
                                        let _e6498 = (_e841.z - _e332.member.z);
                                        let _e6503 = (((_e6496 * _e6496) + (_e6497 * _e6497)) + (_e6498 * _e6498));
                                        let _e6504 = (_e841.w * _e841.w);
                                        if (_e6503 <= (_e6504 * 1.000001f)) {
                                            let _e6518 = (_e829 - _e332.member.x);
                                            let _e6519 = (_e830 - _e332.member.y);
                                            let _e6520 = (_e831 - _e332.member.z);
                                            let _e6525 = (((_e6518 * _e6518) + (_e6519 * _e6519)) + (_e6520 * _e6520));
                                            phi_32259_ = (_e6525 / (((abs((((_e976.member.x * _e6518) + (_e976.member.y * _e6519)) + (_e976.member.z * _e6520))) / sqrt(_e6525)) * 12.566371f) * _e6504));
                                        } else {
                                            let _e6507 = (_e6504 / _e6503);
                                            if (_e6507 != _e6507) {
                                                phi_32278_ = true;
                                            } else {
                                                phi_32278_ = (1f <= _e6507);
                                            }
                                            let _e6511 = phi_32278_;
                                            let _e6512 = select(_e6507, 1f, _e6511);
                                            phi_32259_ = (0.15915494f / (_e6512 / (1f + sqrt((1f - _e6512)))));
                                        }
                                        let _e6541 = phi_32259_;
                                        phi_13833_ = _e6541;
                                    } else {
                                        let _e6438 = (_e829 - _e332.member.x);
                                        let _e6439 = (_e830 - _e332.member.y);
                                        let _e6440 = (_e831 - _e332.member.z);
                                        let _e6445 = (((_e6438 * _e6438) + (_e6439 * _e6439)) + (_e6440 * _e6440));
                                        let _e6456 = (abs((((_e976.member.x * _e6438) + (_e976.member.y * _e6439)) + (_e976.member.z * _e6440))) / sqrt(_e6445));
                                        if (_e6456 <= 0f) {
                                            phi_13832_ = 0f;
                                        } else {
                                            if _e883 {
                                                let _e6464 = (_e855.x - _e841.x);
                                                let _e6465 = (_e855.y - _e841.y);
                                                let _e6466 = (_e855.z - _e841.z);
                                                let _e6467 = (_e858.x - _e841.x);
                                                let _e6468 = (_e858.y - _e841.y);
                                                let _e6469 = (_e858.z - _e841.z);
                                                let _e6472 = ((_e6465 * _e6469) - (_e6468 * _e6466));
                                                let _e6475 = ((_e6466 * _e6467) - (_e6469 * _e6464));
                                                let _e6478 = ((_e6464 * _e6468) - (_e6467 * _e6465));
                                                phi_13829_ = (sqrt((((_e6472 * _e6472) + (_e6475 * _e6475)) + (_e6478 * _e6478))) * 0.5f);
                                            } else {
                                                phi_13829_ = ((12.566371f * _e841.w) * _e841.w);
                                            }
                                            let _e6487 = phi_13829_;
                                            phi_13832_ = (_e6445 / (_e6456 * _e6487));
                                        }
                                        let _e6491 = phi_13832_;
                                        phi_13833_ = _e6491;
                                    }
                                    let _e6543 = phi_13833_;
                                    let _e6544 = ((_e6433 - _e6435) * _e6543);
                                    if _e373 {
                                        phi_13839_ = 0f;
                                    } else {
                                        if (_e332.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_32299_ = 1f;
                                        } else {
                                            let _e6546 = (_e332.member_1.w * _e332.member_1.w);
                                            let _e6548 = (_e6546 + (_e6544 * _e6544));
                                            if (_e6548 == 0f) {
                                                phi_32296_ = 0f;
                                            } else {
                                                phi_32296_ = (_e6546 / _e6548);
                                            }
                                            let _e6552 = phi_32296_;
                                            phi_32299_ = _e6552;
                                        }
                                        let _e6554 = phi_32299_;
                                        phi_13839_ = _e6554;
                                    }
                                    let _e6556 = phi_13839_;
                                    phi_13840_ = _e6556;
                                } else {
                                    phi_13840_ = 1f;
                                }
                                let _e6558 = phi_13840_;
                                phi_13841_ = _e6558;
                            }
                            let _e6560 = phi_13841_;
                            phi_13842_ = _e6560;
                        }
                        let _e6562 = phi_13842_;
                        phi_19781_ = type_69(type_34(_e332.member, _e332.member_1, vec4<f32>(_e823.x, _e823.y, _e823.z, f32(_e357)), vec4<f32>((_e332.member_3.x + ((_e823.x * _e6422.x) * _e6562)), (_e332.member_3.y + ((_e823.y * _e6422.y) * _e6562)), (_e332.member_3.z + ((_e823.z * _e6422.z) * _e6562)), f32(_e385)), vec4<u32>(_e332.member_4.x, (_e332.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19782_ = type_69();
                        phi_19783_ = true;
                    } else {
                        if (_e1051.member == 7u) {
                            let _e6366 = type_34(_e332.member, _e332.member_1, vec4<f32>(_e823.x, _e823.y, _e823.z, f32(_e357)), vec4<f32>(_e332.member_3.x, _e332.member_3.y, _e332.member_3.z, f32(_e385)), vec4<u32>(_e332.member_4.x, ((_e332.member_4.y | 16u) & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f));
                            let _e6378 = type_34(_e6366.member, _e6366.member_1, _e6366.member_2, _e6366.member_3, vec4<u32>(_e6366.member_4.x, _e6366.member_4.y, _e332.member_4.z, _e6366.member_4.w), _e6366.member_5);
                            phi_19778_ = type_69(type_34(_e6378.member, _e6378.member_1, _e6378.member_2, _e6378.member_3, _e6378.member_4, _e332.member_5), _e825, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19779_ = type_69();
                            phi_19780_ = true;
                        } else {
                            let _e1055 = -(_e332.member_1.x);
                            let _e1056 = -(_e332.member_1.y);
                            let _e1057 = -(_e332.member_1.z);
                            if ((((_e976.member.x * _e1055) + (_e976.member.y * _e1056)) + (_e976.member.z * _e1057)) < 0f) {
                                phi_13894_ = -(_e976.member);
                            } else {
                                phi_13894_ = _e976.member;
                            }
                            let _e1069 = phi_13894_;
                            if ((((_e976.member_1.x * _e1055) + (_e976.member_1.y * _e1056)) + (_e976.member_1.z * _e1057)) < 0f) {
                                phi_13908_ = -(_e976.member_1);
                            } else {
                                phi_13908_ = _e976.member_1;
                            }
                            let _e1081 = phi_13908_;
                            let _e1084 = global_1.member.member_4[2u];
                            let _e1085 = abs(_e829);
                            let _e1086 = abs(_e830);
                            let _e1087 = abs(_e831);
                            let _e1089 = select(_e1086, _e1085, (_e1085 > _e1086));
                            let _e1093 = (0.00002f * (1f + select(_e1087, _e1089, (_e1089 > _e1087))));
                            if (_e1084 != _e1084) {
                                phi_26569_ = true;
                            } else {
                                phi_26569_ = (_e1093 >= _e1084);
                            }
                            let _e1097 = phi_26569_;
                            let _e1099 = (select(_e1084, _e1093, _e1097) * 2f);
                            let _e1106 = (_e829 + (_e1069.x * _e1099));
                            let _e1107 = (_e830 + (_e1069.y * _e1099));
                            let _e1108 = (_e831 + (_e1069.z * _e1099));
                            let _e1109 = vec3<f32>(_e1106, _e1107, _e1108);
                            if ((_e332.member_4.y & 4u) == 0u) {
                                phi_13941_ = 0f;
                            } else {
                                let _e1114 = global_1.member.member_4[0u];
                                phi_13941_ = _e1114;
                            }
                            let _e1116 = phi_13941_;
                            let _e1119 = select(-1f, 1f, (_e1081.z >= 0f));
                            let _e1121 = (-1f / (_e1119 + _e1081.z));
                            let _e1125 = ((_e1081.x * _e1081.y) * _e1121);
                            let _e1129 = (1f + (((_e1119 * _e1081.x) * _e1081.x) * _e1121));
                            let _e1130 = (_e1119 * _e1125);
                            let _e1132 = (-(_e1119) * _e1081.x);
                            let _e1135 = (_e1119 + ((_e1081.y * _e1081.y) * _e1121));
                            let _e1136 = -(_e1081.y);
                            if (_e1051.member == 0u) {
                                phi_34768_ = vec3<f32>(0f, 0f, 0f);
                                phi_34576_ = 1.5f;
                                phi_34497_ = vec3<f32>(0f, 0f, 0f);
                                phi_34420_ = vec3<f32>(_e1051.member_4.x, _e1051.member_4.y, _e1051.member_4.z);
                                phi_34341_ = 0u;
                                phi_34259_ = 1f;
                                phi_34182_ = 0u;
                                phi_34173_ = _e332.member_4.x;
                                phi_14620_ = type_69();
                                phi_14621_ = true;
                            } else {
                                if (_e1051.member == 1u) {
                                    let _e1685 = (_e1051.member_6.y * _e1051.member_6.y);
                                    if (_e1685 != _e1685) {
                                        phi_27261_ = true;
                                    } else {
                                        phi_27261_ = (_e1116 >= _e1685);
                                    }
                                    let _e1689 = phi_27261_;
                                    let _e1690 = select(_e1685, _e1116, _e1689);
                                    let _e1691 = (_e1690 >= 0.001f);
                                    if _e1691 {
                                        phi_34825_ = vec3<f32>(_e1051.member_4.x, _e1051.member_4.y, _e1051.member_4.z);
                                        phi_34619_ = _e1051.member_6.x;
                                        phi_34472_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34160_ = _e332.member_4.x;
                                        phi_14016_ = vec3<f32>();
                                        phi_14017_ = true;
                                    } else {
                                        let _e1693 = ((_e332.member_4.x * 747796405u) + 2891336453u);
                                        let _e1701 = (((_e1693 >> bitcast<u32>((((_e1693 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1693) * 277803737u);
                                        let _e1704 = ((_e1701 >> bitcast<u32>(22i)) ^ _e1701);
                                        let _e1714 = abs((((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z)));
                                        let _e1717 = (1f / _e1051.member_6.x);
                                        let _e1721 = ((_e1717 * _e1717) * (1f - (_e1714 * _e1714)));
                                        if (_e1721 >= 1f) {
                                            phi_27301_ = 1f;
                                        } else {
                                            let _e1724 = sqrt((1f - _e1721));
                                            if (_e1714 != _e1714) {
                                                phi_27312_ = true;
                                            } else {
                                                phi_27312_ = (_e1724 <= _e1714);
                                            }
                                            let _e1728 = phi_27312_;
                                            let _e1733 = pow(((1f - _e1051.member_6.x) / (1f + _e1051.member_6.x)), 2f);
                                            phi_27301_ = (_e1733 + ((1f - _e1733) * pow((1f - select(_e1714, _e1724, _e1728)), 5f)));
                                        }
                                        let _e1740 = phi_27301_;
                                        let _e1741 = ((f32((_e1704 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1740);
                                        if _e1741 {
                                            let _e1754 = (((_e1081.x * _e332.member_1.x) + (_e1081.y * _e332.member_1.y)) + (_e1081.z * _e332.member_1.z));
                                            phi_34473_ = vec3<f32>(0f, 0f, 0f);
                                            phi_14014_ = (_e346 - vec3<f32>(((_e1081.x * 2f) * _e1754), ((_e1081.y * 2f) * _e1754), ((_e1081.z * 2f) * _e1754)));
                                        } else {
                                            phi_34473_ = vec3<f32>(_e1051.member_4.x, _e1051.member_4.y, _e1051.member_4.z);
                                            phi_14014_ = vec3<f32>();
                                        }
                                        let _e1761 = phi_34473_;
                                        let _e1763 = phi_14014_;
                                        phi_34825_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34619_ = 1.5f;
                                        phi_34472_ = _e1761;
                                        phi_34160_ = _e1704;
                                        phi_14016_ = _e1763;
                                        phi_14017_ = select(true, false, _e1741);
                                    }
                                    let _e1773 = phi_34825_;
                                    let _e1775 = phi_34619_;
                                    let _e1777 = phi_34472_;
                                    let _e1779 = phi_34160_;
                                    let _e1781 = phi_14016_;
                                    let _e1783 = phi_14017_;
                                    phi_34771_ = _e1773;
                                    phi_34579_ = _e1775;
                                    phi_34500_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34423_ = _e1777;
                                    phi_34344_ = select(0u, 1u, _e1691);
                                    phi_34262_ = select(1f, _e1690, _e1691);
                                    phi_34185_ = select(0u, 2u, _e1691);
                                    phi_34111_ = _e1779;
                                    phi_14489_ = _e1109;
                                    phi_14490_ = _e1781;
                                    phi_14491_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14492_ = _e1783;
                                } else {
                                    if (_e1051.member == 3u) {
                                        phi_34114_ = _e332.member_4.x;
                                        phi_14168_ = vec3<f32>();
                                        phi_14169_ = vec3<f32>();
                                        phi_14170_ = vec3<f32>();
                                        phi_14171_ = true;
                                        phi_14172_ = false;
                                    } else {
                                        let _e1140 = (_e1051.member == 5u);
                                        if _e1140 {
                                            phi_34115_ = _e332.member_4.x;
                                            phi_14164_ = vec3<f32>();
                                            phi_14165_ = vec3<f32>();
                                            phi_14166_ = vec3<f32>();
                                            phi_14167_ = false;
                                        } else {
                                            let _e1141 = (_e1051.member == 2u);
                                            if _e1141 {
                                                let _e1146 = (((_e332.member_1.x * _e976.member_1.x) + (_e332.member_1.y * _e976.member_1.y)) + (_e332.member_1.z * _e976.member_1.z));
                                                let _e1147 = (_e1146 > 0f);
                                                if _e1147 {
                                                    phi_14049_ = -(_e976.member_1);
                                                } else {
                                                    phi_14049_ = _e976.member_1;
                                                }
                                                let _e1150 = phi_14049_;
                                                if _e1147 {
                                                    phi_14055_ = _e1051.member_6.x;
                                                } else {
                                                    phi_14055_ = (1f / _e1051.member_6.x);
                                                }
                                                let _e1155 = phi_14055_;
                                                let _e1156 = abs(_e1146);
                                                let _e1157 = (_e1155 * _e1155);
                                                let _e1160 = (_e1157 * (1f - (_e1156 * _e1156)));
                                                if (_e1160 >= 1f) {
                                                    phi_26631_ = 1f;
                                                } else {
                                                    let _e1163 = sqrt((1f - _e1160));
                                                    if (_e1156 != _e1156) {
                                                        phi_26642_ = true;
                                                    } else {
                                                        phi_26642_ = (_e1163 <= _e1156);
                                                    }
                                                    let _e1167 = phi_26642_;
                                                    let _e1172 = pow(((1f - _e1051.member_6.x) / (1f + _e1051.member_6.x)), 2f);
                                                    phi_26631_ = (_e1172 + ((1f - _e1172) * pow((1f - select(_e1156, _e1163, _e1167)), 5f)));
                                                }
                                                let _e1179 = phi_26631_;
                                                let _e1187 = (((_e332.member_1.x * _e1150.x) + (_e332.member_1.y * _e1150.y)) + (_e332.member_1.z * _e1150.z));
                                                let _e1191 = (1f - (_e1157 * (1f - (_e1187 * _e1187))));
                                                if (_e1191 > 0f) {
                                                    let _e1194 = ((_e332.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1202 = (((_e1194 >> bitcast<u32>((((_e1194 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1194) * 277803737u);
                                                    let _e1205 = ((_e1202 >> bitcast<u32>(22i)) ^ _e1202);
                                                    let _e1210 = ((f32((_e1205 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1179);
                                                    if _e1210 {
                                                        let _e1220 = sqrt(_e1191);
                                                        let _e1224 = (((_e332.member_1.x - (_e1150.x * _e1187)) * _e1155) - (_e1150.x * _e1220));
                                                        let _e1225 = (((_e332.member_1.y - (_e1150.y * _e1187)) * _e1155) - (_e1150.y * _e1220));
                                                        let _e1226 = (((_e332.member_1.z - (_e1150.z * _e1187)) * _e1155) - (_e1150.z * _e1220));
                                                        let _e1233 = (1f / sqrt((((_e1224 * _e1224) + (_e1225 * _e1225)) + (_e1226 * _e1226))));
                                                        phi_14136_ = vec3<f32>((_e829 - (_e1150.x * _e1099)), (_e830 - (_e1150.y * _e1099)), (_e831 - (_e1150.z * _e1099)));
                                                        phi_14137_ = vec3<f32>((_e1224 * _e1233), (_e1225 * _e1233), (_e1226 * _e1233));
                                                        phi_14138_ = vec3<f32>(_e1051.member_4.x, _e1051.member_4.y, _e1051.member_4.z);
                                                    } else {
                                                        phi_14136_ = vec3<f32>();
                                                        phi_14137_ = vec3<f32>();
                                                        phi_14138_ = vec3<f32>();
                                                    }
                                                    let _e1251 = phi_14136_;
                                                    let _e1253 = phi_14137_;
                                                    let _e1255 = phi_14138_;
                                                    phi_34118_ = _e1205;
                                                    phi_14140_ = _e1251;
                                                    phi_14141_ = _e1253;
                                                    phi_14142_ = _e1255;
                                                    phi_14143_ = select(true, false, _e1210);
                                                } else {
                                                    phi_34118_ = _e332.member_4.x;
                                                    phi_14140_ = vec3<f32>();
                                                    phi_14141_ = vec3<f32>();
                                                    phi_14142_ = vec3<f32>();
                                                    phi_14143_ = true;
                                                }
                                                let _e1258 = phi_34118_;
                                                let _e1260 = phi_14140_;
                                                let _e1262 = phi_14141_;
                                                let _e1264 = phi_14142_;
                                                let _e1266 = phi_14143_;
                                                if _e1266 {
                                                    let _e1274 = (((_e976.member_1.x * _e332.member_1.x) + (_e976.member_1.y * _e332.member_1.y)) + (_e976.member_1.z * _e332.member_1.z));
                                                    phi_14158_ = vec3<f32>((_e829 + (_e1150.x * _e1099)), (_e830 + (_e1150.y * _e1099)), (_e831 + (_e1150.z * _e1099)));
                                                    phi_14159_ = (_e346 - vec3<f32>(((_e976.member_1.x * 2f) * _e1274), ((_e976.member_1.y * 2f) * _e1274), ((_e976.member_1.z * 2f) * _e1274)));
                                                } else {
                                                    phi_14158_ = _e1260;
                                                    phi_14159_ = _e1262;
                                                }
                                                let _e1288 = phi_14158_;
                                                let _e1290 = phi_14159_;
                                                phi_34116_ = _e1258;
                                                phi_14161_ = _e1288;
                                                phi_14162_ = _e1290;
                                                phi_14163_ = select(_e1264, vec3<f32>(1f, 1f, 1f), vec3(_e1266));
                                            } else {
                                                phi_34116_ = _e332.member_4.x;
                                                phi_14161_ = vec3<f32>();
                                                phi_14162_ = vec3<f32>();
                                                phi_14163_ = vec3<f32>();
                                            }
                                            let _e1294 = phi_34116_;
                                            let _e1296 = phi_14161_;
                                            let _e1298 = phi_14162_;
                                            let _e1300 = phi_14163_;
                                            phi_34115_ = _e1294;
                                            phi_14164_ = _e1296;
                                            phi_14165_ = _e1298;
                                            phi_14166_ = _e1300;
                                            phi_14167_ = _e1141;
                                        }
                                        let _e1302 = phi_34115_;
                                        let _e1304 = phi_14164_;
                                        let _e1306 = phi_14165_;
                                        let _e1308 = phi_14166_;
                                        let _e1310 = phi_14167_;
                                        phi_34114_ = _e1302;
                                        phi_14168_ = _e1304;
                                        phi_14169_ = _e1306;
                                        phi_14170_ = _e1308;
                                        phi_14171_ = _e1140;
                                        phi_14172_ = _e1310;
                                    }
                                    let _e1312 = phi_34114_;
                                    let _e1314 = phi_14168_;
                                    let _e1316 = phi_14169_;
                                    let _e1318 = phi_14170_;
                                    let _e1320 = phi_14171_;
                                    let _e1322 = phi_14172_;
                                    if _e1320 {
                                        let _e1325 = (_e1051.member_6.y * _e1051.member_6.y);
                                        if (_e1325 != _e1325) {
                                            phi_26715_ = true;
                                        } else {
                                            phi_26715_ = (_e1116 >= _e1325);
                                        }
                                        let _e1329 = phi_26715_;
                                        let _e1330 = select(_e1325, _e1116, _e1329);
                                        let _e1342 = select(2u, 0u, (_e1051.member == 5u));
                                        let _e1343 = (_e1330 < 0.001f);
                                        if _e1343 {
                                            let _e1351 = (((_e1081.x * _e332.member_1.x) + (_e1081.y * _e332.member_1.y)) + (_e1081.z * _e332.member_1.z));
                                            let _e1362 = abs((((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z)));
                                            if (_e1342 == 0u) {
                                                let _e1644 = (1f - _e1362);
                                                if (_e1644 != _e1644) {
                                                    phi_27246_ = true;
                                                } else {
                                                    phi_27246_ = (0f >= _e1644);
                                                }
                                                let _e1648 = phi_27246_;
                                                let _e1650 = pow(select(_e1644, 0f, _e1648), 5f);
                                                let _e1651 = (1f - _e1650);
                                                phi_14475_ = vec3<f32>(((_e1051.member_4.x * _e1651) + _e1650), ((_e1051.member_4.y * _e1651) + _e1650), ((_e1051.member_4.z * _e1651) + _e1650));
                                            } else {
                                                if (_e1342 == 1u) {
                                                    let _e1626 = (0.44444448f * (1f - (_e1362 * _e1362)));
                                                    if (_e1626 >= 1f) {
                                                        phi_27207_ = 1f;
                                                    } else {
                                                        let _e1629 = sqrt((1f - _e1626));
                                                        if (_e1362 != _e1362) {
                                                            phi_27218_ = true;
                                                        } else {
                                                            phi_27218_ = (_e1629 <= _e1362);
                                                        }
                                                        let _e1633 = phi_27218_;
                                                        phi_27207_ = (0.040000003f + (0.96f * pow((1f - select(_e1362, _e1629, _e1633)), 5f)));
                                                    }
                                                    let _e1640 = phi_27207_;
                                                    phi_14474_ = vec3(_e1640);
                                                } else {
                                                    let _e1366 = select(_e1362, 0f, (_e1362 < 0f));
                                                    let _e1368 = select(_e1366, 1f, (_e1366 > 1f));
                                                    let _e1370 = (1f - (_e1368 * _e1368));
                                                    let _e1377 = ((_e1051.member_4.x * _e1051.member_4.x) - (_e1051.member_5.x * _e1051.member_5.x));
                                                    let _e1378 = ((_e1051.member_4.y * _e1051.member_4.y) - (_e1051.member_5.y * _e1051.member_5.y));
                                                    let _e1379 = ((_e1051.member_4.z * _e1051.member_4.z) - (_e1051.member_5.z * _e1051.member_5.z));
                                                    let _e1381 = (_e1051.member_4.x * (_e1051.member_5.x + _e1051.member_5.x));
                                                    let _e1383 = (_e1051.member_4.y * (_e1051.member_5.y + _e1051.member_5.y));
                                                    let _e1385 = (_e1051.member_4.z * (_e1051.member_5.z + _e1051.member_5.z));
                                                    let _e1392 = ((_e1377 * _e1377) + (_e1381 * _e1381));
                                                    let _e1393 = ((_e1378 * _e1378) + (_e1383 * _e1383));
                                                    let _e1394 = ((_e1379 * _e1379) + (_e1385 * _e1385));
                                                    let _e1399 = ((_e1370 * _e1377) / _e1392);
                                                    let _e1400 = ((_e1370 * _e1378) / _e1393);
                                                    let _e1401 = ((_e1370 * _e1379) / _e1394);
                                                    let _e1402 = (1f - _e1399);
                                                    let _e1403 = (1f - _e1400);
                                                    let _e1404 = (1f - _e1401);
                                                    let _e1406 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1399, _e1400, _e1401));
                                                    let _e1407 = (_e1370 * _e1381);
                                                    let _e1408 = (_e1370 * _e1383);
                                                    let _e1409 = (_e1370 * _e1385);
                                                    let _e1414 = (vec3<f32>(_e1407, _e1408, _e1409) / vec3<f32>(_e1392, _e1393, _e1394));
                                                    let _e1427 = ((_e1406.x * _e1406.x) + (_e1414.x * _e1414.x));
                                                    let _e1428 = ((_e1406.y * _e1406.y) + (_e1414.y * _e1414.y));
                                                    let _e1429 = ((_e1406.z * _e1406.z) + (_e1414.z * _e1414.z));
                                                    if (_e1427 != _e1427) {
                                                        phi_26877_ = true;
                                                    } else {
                                                        phi_26877_ = (0f >= _e1427);
                                                    }
                                                    let _e1433 = phi_26877_;
                                                    let _e1435 = sqrt(select(_e1427, 0f, _e1433));
                                                    if (_e1428 != _e1428) {
                                                        phi_26892_ = true;
                                                    } else {
                                                        phi_26892_ = (0f >= _e1428);
                                                    }
                                                    let _e1439 = phi_26892_;
                                                    let _e1441 = sqrt(select(_e1428, 0f, _e1439));
                                                    if (_e1429 != _e1429) {
                                                        phi_26907_ = true;
                                                    } else {
                                                        phi_26907_ = (0f >= _e1429);
                                                    }
                                                    let _e1445 = phi_26907_;
                                                    let _e1447 = sqrt(select(_e1429, 0f, _e1445));
                                                    let _e1451 = ((_e1435 + _e1402) * 0.5f);
                                                    let _e1452 = ((_e1441 + _e1403) * 0.5f);
                                                    let _e1453 = ((_e1447 + _e1404) * 0.5f);
                                                    if (_e1451 != _e1451) {
                                                        phi_26934_ = true;
                                                    } else {
                                                        phi_26934_ = (0f >= _e1451);
                                                    }
                                                    let _e1457 = phi_26934_;
                                                    let _e1459 = sqrt(select(_e1451, 0f, _e1457));
                                                    if (_e1452 != _e1452) {
                                                        phi_26949_ = true;
                                                    } else {
                                                        phi_26949_ = (0f >= _e1452);
                                                    }
                                                    let _e1463 = phi_26949_;
                                                    let _e1465 = sqrt(select(_e1452, 0f, _e1463));
                                                    if (_e1453 != _e1453) {
                                                        phi_26964_ = true;
                                                    } else {
                                                        phi_26964_ = (0f >= _e1453);
                                                    }
                                                    let _e1469 = phi_26964_;
                                                    let _e1471 = sqrt(select(_e1453, 0f, _e1469));
                                                    let _e1472 = vec3<f32>(_e1459, _e1465, _e1471);
                                                    let _e1476 = ((_e1435 - _e1402) * 0.5f);
                                                    let _e1477 = ((_e1441 - _e1403) * 0.5f);
                                                    let _e1478 = ((_e1447 - _e1404) * 0.5f);
                                                    if (_e1476 != _e1476) {
                                                        phi_26991_ = true;
                                                    } else {
                                                        phi_26991_ = (0f >= _e1476);
                                                    }
                                                    let _e1482 = phi_26991_;
                                                    let _e1484 = sqrt(select(_e1476, 0f, _e1482));
                                                    if (_e1477 != _e1477) {
                                                        phi_27006_ = true;
                                                    } else {
                                                        phi_27006_ = (0f >= _e1477);
                                                    }
                                                    let _e1488 = phi_27006_;
                                                    let _e1490 = sqrt(select(_e1477, 0f, _e1488));
                                                    if (_e1478 != _e1478) {
                                                        phi_27021_ = true;
                                                    } else {
                                                        phi_27021_ = (0f >= _e1478);
                                                    }
                                                    let _e1494 = phi_27021_;
                                                    let _e1496 = sqrt(select(_e1478, 0f, _e1494));
                                                    if ((_e1407 / _e1392) < 0f) {
                                                        phi_14351_ = -(_e1484);
                                                    } else {
                                                        phi_14351_ = _e1484;
                                                    }
                                                    let _e1500 = phi_14351_;
                                                    if ((_e1408 / _e1393) < 0f) {
                                                        phi_14359_ = -(_e1490);
                                                    } else {
                                                        phi_14359_ = _e1490;
                                                    }
                                                    let _e1504 = phi_14359_;
                                                    if ((_e1409 / _e1394) < 0f) {
                                                        phi_14367_ = -(_e1496);
                                                    } else {
                                                        phi_14367_ = _e1496;
                                                    }
                                                    let _e1508 = phi_14367_;
                                                    let _e1512 = (_e1051.member_5.x * _e1368);
                                                    let _e1513 = (_e1051.member_5.y * _e1368);
                                                    let _e1514 = (_e1051.member_5.z * _e1368);
                                                    let _e1524 = vec3<f32>(((_e1051.member_4.x * _e1459) - (_e1051.member_5.x * _e1500)), ((_e1051.member_4.y * _e1465) - (_e1051.member_5.y * _e1504)), ((_e1051.member_4.z * _e1471) - (_e1051.member_5.z * _e1508)));
                                                    let _e1531 = ((_e1051.member_4.x * _e1500) + (_e1051.member_5.x * _e1459));
                                                    let _e1532 = ((_e1051.member_4.y * _e1504) + (_e1051.member_5.y * _e1465));
                                                    let _e1533 = ((_e1051.member_4.z * _e1508) + (_e1051.member_5.z * _e1471));
                                                    let _e1535 = vec3<f32>((_e1051.member_4.x * _e1368), (_e1051.member_4.y * _e1368), (_e1051.member_4.z * _e1368));
                                                    let _e1536 = (_e1535 - _e1472);
                                                    let _e1537 = (_e1512 - _e1500);
                                                    let _e1538 = (_e1513 - _e1504);
                                                    let _e1539 = (_e1514 - _e1508);
                                                    let _e1552 = (_e1535 + _e1472);
                                                    let _e1553 = (_e1512 + _e1500);
                                                    let _e1554 = (_e1513 + _e1504);
                                                    let _e1555 = (_e1514 + _e1508);
                                                    let _e1565 = ((_e1552.x * _e1552.x) + (_e1553 * _e1553));
                                                    let _e1566 = ((_e1552.y * _e1552.y) + (_e1554 * _e1554));
                                                    let _e1567 = ((_e1552.z * _e1552.z) + (_e1555 * _e1555));
                                                    let _e1577 = vec3(_e1368);
                                                    let _e1578 = (_e1577 - _e1524);
                                                    let _e1579 = -(vec3<f32>(_e1531, _e1532, _e1533));
                                                    let _e1595 = (_e1577 + _e1524);
                                                    let _e1605 = ((_e1595.x * _e1595.x) + (_e1531 * _e1531));
                                                    let _e1606 = ((_e1595.y * _e1595.y) + (_e1532 * _e1532));
                                                    let _e1607 = ((_e1595.z * _e1595.z) + (_e1533 * _e1533));
                                                    phi_14474_ = vec3<f32>((((((_e1536.x * _e1536.x) + (_e1537 * _e1537)) / select(0.000000000000000000000000000001f, _e1565, (_e1565 > 0.000000000000000000000000000001f))) + (((_e1578.x * _e1578.x) + (_e1579.x * _e1579.x)) / select(0.000000000000000000000000000001f, _e1605, (_e1605 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1536.y * _e1536.y) + (_e1538 * _e1538)) / select(0.000000000000000000000000000001f, _e1566, (_e1566 > 0.000000000000000000000000000001f))) + (((_e1578.y * _e1578.y) + (_e1579.y * _e1579.y)) / select(0.000000000000000000000000000001f, _e1606, (_e1606 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1536.z * _e1536.z) + (_e1539 * _e1539)) / select(0.000000000000000000000000000001f, _e1567, (_e1567 > 0.000000000000000000000000000001f))) + (((_e1578.z * _e1578.z) + (_e1579.z * _e1579.z)) / select(0.000000000000000000000000000001f, _e1607, (_e1607 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e1643 = phi_14474_;
                                                phi_14475_ = _e1643;
                                            }
                                            let _e1660 = phi_14475_;
                                            phi_14476_ = (_e346 - vec3<f32>(((_e1081.x * 2f) * _e1351), ((_e1081.y * 2f) * _e1351), ((_e1081.z * 2f) * _e1351)));
                                            phi_14477_ = _e1660;
                                        } else {
                                            phi_14476_ = vec3<f32>();
                                            phi_14477_ = vec3<f32>();
                                        }
                                        let _e1662 = phi_14476_;
                                        let _e1664 = phi_14477_;
                                        phi_34502_ = vec3<f32>(_e1051.member_5.x, _e1051.member_5.y, _e1051.member_5.z);
                                        phi_34425_ = vec3<f32>(_e1051.member_4.x, _e1051.member_4.y, _e1051.member_4.z);
                                        phi_34346_ = _e1342;
                                        phi_34264_ = _e1330;
                                        phi_14479_ = _e1662;
                                        phi_14480_ = _e1664;
                                        phi_14481_ = _e1343;
                                    } else {
                                        phi_34502_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34425_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34346_ = 0u;
                                        phi_34264_ = 1f;
                                        phi_14479_ = _e1316;
                                        phi_14480_ = _e1318;
                                        phi_14481_ = _e1322;
                                    }
                                    let _e1666 = phi_34502_;
                                    let _e1668 = phi_34425_;
                                    let _e1670 = phi_34346_;
                                    let _e1672 = phi_34264_;
                                    let _e1674 = phi_14479_;
                                    let _e1676 = phi_14480_;
                                    let _e1678 = phi_14481_;
                                    phi_34771_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34579_ = 1.5f;
                                    phi_34500_ = _e1666;
                                    phi_34423_ = _e1668;
                                    phi_34344_ = _e1670;
                                    phi_34262_ = _e1672;
                                    phi_34185_ = select(0u, 1u, _e1320);
                                    phi_34111_ = _e1312;
                                    phi_14489_ = select(_e1314, _e1109, vec3(_e1320));
                                    phi_14490_ = _e1674;
                                    phi_14491_ = _e1676;
                                    phi_14492_ = select(true, false, _e1678);
                                }
                                let _e1788 = phi_34771_;
                                let _e1790 = phi_34579_;
                                let _e1792 = phi_34500_;
                                let _e1794 = phi_34423_;
                                let _e1796 = phi_34344_;
                                let _e1798 = phi_34262_;
                                let _e1800 = phi_34185_;
                                let _e1802 = phi_34111_;
                                let _e1804 = phi_14489_;
                                let _e1806 = phi_14490_;
                                let _e1808 = phi_14491_;
                                let _e1810 = phi_14492_;
                                let _e1811 = select(true, false, _e1810);
                                if _e1811 {
                                    if (_e357 >= _e376) {
                                        phi_34170_ = _e1802;
                                        phi_14609_ = _e823;
                                        phi_14610_ = type_69();
                                        phi_14611_ = true;
                                    } else {
                                        let _e1824 = global_1.member.member_4[1u];
                                        if (sqrt((((_e1808.x * _e1808.x) + (_e1808.y * _e1808.y)) + (_e1808.z * _e1808.z))) <= _e1824) {
                                            phi_34165_ = _e1802;
                                            phi_14572_ = _e823;
                                            phi_14573_ = false;
                                        } else {
                                            let _e1828 = global_1.member.member_7[1u];
                                            if (_e1828 == 1u) {
                                                let _e1833 = global_1.member.member_4[3u];
                                                if (f32(_e357) >= _e1833) {
                                                    let _e1836 = select(_e1808.y, _e1808.x, (_e1808.x > _e1808.y));
                                                    let _e1838 = select(_e1808.z, _e1836, (_e1836 > _e1808.z));
                                                    if (_e1838 != _e1838) {
                                                        phi_27378_ = true;
                                                    } else {
                                                        phi_27378_ = (1f <= _e1838);
                                                    }
                                                    let _e1842 = phi_27378_;
                                                    let _e1843 = select(_e1838, 1f, _e1842);
                                                    let _e1845 = ((_e1802 * 747796405u) + 2891336453u);
                                                    let _e1853 = (((_e1845 >> bitcast<u32>((((_e1845 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1845) * 277803737u);
                                                    let _e1856 = ((_e1853 >> bitcast<u32>(22i)) ^ _e1853);
                                                    let _e1861 = ((f32((_e1856 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1843);
                                                    if _e1861 {
                                                        phi_14561_ = vec3<f32>();
                                                    } else {
                                                        phi_14561_ = vec3<f32>((_e823.x / _e1843), (_e823.y / _e1843), (_e823.z / _e1843));
                                                    }
                                                    let _e1870 = phi_14561_;
                                                    phi_34168_ = _e1856;
                                                    phi_14563_ = _e1870;
                                                    phi_14564_ = select(true, false, _e1861);
                                                } else {
                                                    phi_34168_ = _e1802;
                                                    phi_14563_ = _e823;
                                                    phi_14564_ = true;
                                                }
                                                let _e1873 = phi_34168_;
                                                let _e1875 = phi_14563_;
                                                let _e1877 = phi_14564_;
                                                phi_34167_ = _e1873;
                                                phi_14565_ = _e1875;
                                                phi_14566_ = _e1877;
                                            } else {
                                                phi_34167_ = _e1802;
                                                phi_14565_ = _e823;
                                                phi_14566_ = true;
                                            }
                                            let _e1879 = phi_34167_;
                                            let _e1881 = phi_14565_;
                                            let _e1883 = phi_14566_;
                                            phi_34165_ = _e1879;
                                            phi_14572_ = select(_e823, _e1881, vec3(_e1883));
                                            phi_14573_ = select(false, true, _e1883);
                                        }
                                        let _e1888 = phi_34165_;
                                        let _e1890 = phi_14572_;
                                        let _e1892 = phi_14573_;
                                        if _e1892 {
                                            let _e1893 = (_e1890 * _e1808);
                                            let _e1904 = (_e1806 * (1f / sqrt((((_e1806.x * _e1806.x) + (_e1806.y * _e1806.y)) + (_e1806.z * _e1806.z)))));
                                            phi_14607_ = type_69(type_34(vec4<f32>(_e1804.x, _e1804.y, _e1804.z, _e332.member.w), vec4<f32>(_e1904.x, _e1904.y, _e1904.z, _e332.member_1.w), vec4<f32>(_e1893.x, _e1893.y, _e1893.z, f32((_e357 + 1u))), vec4<f32>(_e332.member_3.x, _e332.member_3.y, _e332.member_3.z, f32(_e385)), vec4<u32>(_e1888, ((_e332.member_4.y | 2u) & 4294967263u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14607_ = type_69();
                                        }
                                        let _e1928 = phi_14607_;
                                        phi_34170_ = _e1888;
                                        phi_14609_ = _e1890;
                                        phi_14610_ = _e1928;
                                        phi_14611_ = select(true, false, _e1892);
                                    }
                                    let _e1931 = phi_34170_;
                                    let _e1933 = phi_14609_;
                                    let _e1935 = phi_14610_;
                                    let _e1937 = phi_14611_;
                                    if _e1937 {
                                        phi_14617_ = type_69(type_34(_e332.member, _e332.member_1, vec4<f32>(_e1933.x, _e1933.y, _e1933.z, f32(_e357)), vec4<f32>(_e332.member_3.x, _e332.member_3.y, _e332.member_3.z, f32(_e385)), vec4<u32>(_e1931, (_e332.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14617_ = _e1935;
                                    }
                                    let _e1950 = phi_14617_;
                                    phi_34174_ = _e1931;
                                    phi_14618_ = _e1950;
                                } else {
                                    phi_34174_ = _e1802;
                                    phi_14618_ = type_69();
                                }
                                let _e1952 = phi_34174_;
                                let _e1954 = phi_14618_;
                                phi_34768_ = _e1788;
                                phi_34576_ = _e1790;
                                phi_34497_ = _e1792;
                                phi_34420_ = _e1794;
                                phi_34341_ = _e1796;
                                phi_34259_ = _e1798;
                                phi_34182_ = _e1800;
                                phi_34173_ = _e1952;
                                phi_14620_ = _e1954;
                                phi_14621_ = select(true, false, _e1811);
                            }
                            let _e1962 = phi_34768_;
                            let _e1964 = phi_34576_;
                            let _e1966 = phi_34497_;
                            let _e1968 = phi_34420_;
                            let _e1970 = phi_34341_;
                            let _e1972 = phi_34259_;
                            let _e1974 = phi_34182_;
                            let _e1976 = phi_34173_;
                            let _e1978 = phi_14620_;
                            let _e1980 = phi_14621_;
                            if _e1980 {
                                let _e1982 = ((_e332.member_4.y | 4u) & 4294967261u);
                                let _e1985 = global_1.member.member_1[3u];
                                phi_36097_ = _e821;
                                phi_34172_ = _e1976;
                                phi_14632_ = _e361;
                                phi_14635_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14637_ = 0u;
                                phi_14639_ = _e385;
                                loop {
                                    let _e1987 = phi_36097_;
                                    let _e1989 = phi_34172_;
                                    let _e1991 = phi_14632_;
                                    let _e1993 = phi_14635_;
                                    let _e1995 = phi_14637_;
                                    let _e1997 = phi_14639_;
                                    local_83 = _e1997;
                                    local_84 = _e1991;
                                    local_85 = _e1991;
                                    local_86 = _e1991;
                                    local_87 = _e1993;
                                    local_88 = _e1997;
                                    local_89 = _e1991;
                                    local_90 = _e1991;
                                    local_91 = _e1991;
                                    local_92 = _e1993;
                                    local_93 = _e1997;
                                    local_94 = _e1991;
                                    local_95 = _e1991;
                                    local_96 = _e1991;
                                    local_97 = _e1993;
                                    local_98 = _e1997;
                                    local_99 = _e1991;
                                    local_100 = _e1991;
                                    local_101 = _e1991;
                                    local_102 = _e1993;
                                    let _e1999 = (_e1995 < (_e1985 + 3u));
                                    if _e1999 {
                                        if (_e1995 < _e1985) {
                                            let _e3996 = global_5.member[_e1995];
                                            let _e4001 = (_e3996.member.x - _e1106);
                                            let _e4002 = (_e3996.member.y - _e1107);
                                            let _e4003 = (_e3996.member.z - _e1108);
                                            let _e4008 = (((_e4001 * _e4001) + (_e4002 * _e4002)) + (_e4003 * _e4003));
                                            let _e4009 = sqrt(_e4008);
                                            let _e4010 = (_e4001 / _e4009);
                                            let _e4011 = (_e4002 / _e4009);
                                            let _e4012 = (_e4003 / _e4009);
                                            if (_e3996.member.w < 0.5f) {
                                                phi_14736_ = 1f;
                                            } else {
                                                let _e4036 = (((_e3996.member_2.x * -(_e4010)) + (_e3996.member_2.y * -(_e4011))) + (_e3996.member_2.z * -(_e4012)));
                                                if (_e3996.member_3.x == _e3996.member_2.w) {
                                                    phi_14735_ = select(1f, 0f, (_e4036 < _e3996.member_3.x));
                                                } else {
                                                    let _e4040 = ((_e4036 - _e3996.member_3.x) / (_e3996.member_2.w - _e3996.member_3.x));
                                                    let _e4042 = select(_e4040, 0f, (_e4040 < 0f));
                                                    let _e4044 = select(_e4042, 1f, (_e4042 > 1f));
                                                    phi_14735_ = ((_e4044 * _e4044) * (3f - (2f * _e4044)));
                                                }
                                                let _e4052 = phi_14735_;
                                                phi_14736_ = _e4052;
                                            }
                                            let _e4054 = phi_14736_;
                                            phi_36514_ = _e1989;
                                            phi_18149_ = 0f;
                                            phi_18150_ = vec3<f32>(_e4010, _e4011, _e4012);
                                            phi_18151_ = vec3<f32>(((_e3996.member_1.x * _e4054) / _e4008), ((_e3996.member_1.y * _e4054) / _e4008), ((_e3996.member_1.z * _e4054) / _e4008));
                                            phi_18152_ = _e4009;
                                        } else {
                                            if (_e1995 == _e1985) {
                                                if ((_e379 & 4u) == 0u) {
                                                    phi_36558_ = _e1989;
                                                    phi_16778_ = 0f;
                                                    phi_16779_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16780_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e2748 = global_1.member.member_5;
                                                    let _e2756 = global_1.member.member_6[3u];
                                                    let _e2758 = sin((_e2756 * 0.5f));
                                                    let _e2760 = ((12.566371f * _e2758) * _e2758);
                                                    let _e2765 = (((_e2748.x * _e1081.x) + (_e2748.y * _e1081.y)) + (_e2748.z * _e1081.z));
                                                    if (_e2765 != _e2765) {
                                                        phi_27917_ = true;
                                                    } else {
                                                        phi_27917_ = (0f >= _e2765);
                                                    }
                                                    let _e2769 = phi_27917_;
                                                    let _e2771 = (select(_e2765, 0f, _e2769) * 0.31830987f);
                                                    if (_e1974 == 0u) {
                                                        if (_e2765 <= 0f) {
                                                            phi_28714_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28714_ = (_e1968 * (_e2765 * 0.31830987f));
                                                        }
                                                        let _e3247 = phi_28714_;
                                                        phi_15146_ = type_59(_e3247, _e2771);
                                                        phi_15147_ = type_59();
                                                        phi_15148_ = true;
                                                    } else {
                                                        let _e2777 = (((_e1055 * _e1129) + (_e1056 * _e1130)) + (_e1057 * _e1132));
                                                        let _e2782 = (((_e1055 * _e1125) + (_e1056 * _e1135)) + (_e1057 * _e1136));
                                                        let _e2787 = (((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z));
                                                        let _e2792 = (((_e2748.x * _e1129) + (_e2748.y * _e1130)) + (_e2748.z * _e1132));
                                                        let _e2797 = (((_e2748.x * _e1125) + (_e2748.y * _e1135)) + (_e2748.z * _e1136));
                                                        if (_e2787 <= 0f) {
                                                            phi_15110_ = type_59();
                                                            phi_15111_ = true;
                                                        } else {
                                                            let _e2799 = (_e2765 <= 0f);
                                                            if _e2799 {
                                                                phi_15109_ = type_59();
                                                            } else {
                                                                let _e2800 = (_e2777 + _e2792);
                                                                let _e2801 = (_e2782 + _e2797);
                                                                let _e2802 = (_e2787 + _e2765);
                                                                let _e2809 = (1f / sqrt((((_e2800 * _e2800) + (_e2801 * _e2801)) + (_e2802 * _e2802))));
                                                                let _e2812 = (_e2802 * _e2809);
                                                                if (_e2812 <= 0f) {
                                                                    phi_28027_ = 0f;
                                                                } else {
                                                                    let _e2814 = (_e1972 * _e1972);
                                                                    let _e2818 = ((((_e2814 - 1f) * _e2812) * _e2812) + 1f);
                                                                    phi_28027_ = (_e2814 / ((3.1415927f * _e2818) * _e2818));
                                                                }
                                                                let _e2823 = phi_28027_;
                                                                let _e2824 = (_e2787 * _e2787);
                                                                if (_e2824 >= 1f) {
                                                                    phi_28055_ = 0f;
                                                                } else {
                                                                    phi_28055_ = ((sqrt((1f + (((_e1972 * _e1972) * ((_e2777 * _e2777) + (_e2782 * _e2782))) / _e2824))) - 1f) * 0.5f);
                                                                }
                                                                let _e2837 = phi_28055_;
                                                                let _e2838 = (1f + _e2837);
                                                                let _e2839 = (_e2765 * _e2765);
                                                                if (_e2839 >= 1f) {
                                                                    phi_28083_ = 0f;
                                                                } else {
                                                                    phi_28083_ = ((sqrt((1f + (((_e1972 * _e1972) * ((_e2792 * _e2792) + (_e2797 * _e2797))) / _e2839))) - 1f) * 0.5f);
                                                                }
                                                                let _e2852 = phi_28083_;
                                                                let _e2859 = (((_e2777 * (_e2800 * _e2809)) + (_e2782 * (_e2801 * _e2809))) + (_e2787 * _e2812));
                                                                if (_e1970 == 0u) {
                                                                    let _e3154 = (1f - _e2859);
                                                                    if (_e3154 != _e3154) {
                                                                        phi_28586_ = true;
                                                                    } else {
                                                                        phi_28586_ = (0f >= _e3154);
                                                                    }
                                                                    let _e3158 = phi_28586_;
                                                                    let _e3160 = pow(select(_e3154, 0f, _e3158), 5f);
                                                                    let _e3161 = (1f - _e3160);
                                                                    phi_15097_ = vec3<f32>(((_e1968.x * _e3161) + _e3160), ((_e1968.y * _e3161) + _e3160), ((_e1968.z * _e3161) + _e3160));
                                                                } else {
                                                                    if (_e1970 == 1u) {
                                                                        let _e3127 = (1f / _e1964);
                                                                        let _e3131 = ((_e3127 * _e3127) * (1f - (_e2859 * _e2859)));
                                                                        if (_e3131 >= 1f) {
                                                                            phi_28547_ = 1f;
                                                                        } else {
                                                                            let _e3134 = sqrt((1f - _e3131));
                                                                            if (_e2859 != _e2859) {
                                                                                phi_28558_ = true;
                                                                            } else {
                                                                                phi_28558_ = (_e3134 <= _e2859);
                                                                            }
                                                                            let _e3138 = phi_28558_;
                                                                            let _e3143 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                                            phi_28547_ = (_e3143 + ((1f - _e3143) * pow((1f - select(_e2859, _e3134, _e3138)), 5f)));
                                                                        }
                                                                        let _e3150 = phi_28547_;
                                                                        phi_15096_ = vec3(_e3150);
                                                                    } else {
                                                                        let _e2863 = select(_e2859, 0f, (_e2859 < 0f));
                                                                        let _e2865 = select(_e2863, 1f, (_e2863 > 1f));
                                                                        let _e2867 = (1f - (_e2865 * _e2865));
                                                                        let _e2880 = ((_e1968.x * _e1968.x) - (_e1966.x * _e1966.x));
                                                                        let _e2881 = ((_e1968.y * _e1968.y) - (_e1966.y * _e1966.y));
                                                                        let _e2882 = ((_e1968.z * _e1968.z) - (_e1966.z * _e1966.z));
                                                                        let _e2884 = (_e1968.x * (_e1966.x + _e1966.x));
                                                                        let _e2886 = (_e1968.y * (_e1966.y + _e1966.y));
                                                                        let _e2888 = (_e1968.z * (_e1966.z + _e1966.z));
                                                                        let _e2895 = ((_e2880 * _e2880) + (_e2884 * _e2884));
                                                                        let _e2896 = ((_e2881 * _e2881) + (_e2886 * _e2886));
                                                                        let _e2897 = ((_e2882 * _e2882) + (_e2888 * _e2888));
                                                                        let _e2902 = ((_e2867 * _e2880) / _e2895);
                                                                        let _e2903 = ((_e2867 * _e2881) / _e2896);
                                                                        let _e2904 = ((_e2867 * _e2882) / _e2897);
                                                                        let _e2905 = (1f - _e2902);
                                                                        let _e2906 = (1f - _e2903);
                                                                        let _e2907 = (1f - _e2904);
                                                                        let _e2909 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2902, _e2903, _e2904));
                                                                        let _e2910 = (_e2867 * _e2884);
                                                                        let _e2911 = (_e2867 * _e2886);
                                                                        let _e2912 = (_e2867 * _e2888);
                                                                        let _e2917 = (vec3<f32>(_e2910, _e2911, _e2912) / vec3<f32>(_e2895, _e2896, _e2897));
                                                                        let _e2930 = ((_e2909.x * _e2909.x) + (_e2917.x * _e2917.x));
                                                                        let _e2931 = ((_e2909.y * _e2909.y) + (_e2917.y * _e2917.y));
                                                                        let _e2932 = ((_e2909.z * _e2909.z) + (_e2917.z * _e2917.z));
                                                                        if (_e2930 != _e2930) {
                                                                            phi_28217_ = true;
                                                                        } else {
                                                                            phi_28217_ = (0f >= _e2930);
                                                                        }
                                                                        let _e2936 = phi_28217_;
                                                                        let _e2938 = sqrt(select(_e2930, 0f, _e2936));
                                                                        if (_e2931 != _e2931) {
                                                                            phi_28232_ = true;
                                                                        } else {
                                                                            phi_28232_ = (0f >= _e2931);
                                                                        }
                                                                        let _e2942 = phi_28232_;
                                                                        let _e2944 = sqrt(select(_e2931, 0f, _e2942));
                                                                        if (_e2932 != _e2932) {
                                                                            phi_28247_ = true;
                                                                        } else {
                                                                            phi_28247_ = (0f >= _e2932);
                                                                        }
                                                                        let _e2948 = phi_28247_;
                                                                        let _e2950 = sqrt(select(_e2932, 0f, _e2948));
                                                                        let _e2954 = ((_e2938 + _e2905) * 0.5f);
                                                                        let _e2955 = ((_e2944 + _e2906) * 0.5f);
                                                                        let _e2956 = ((_e2950 + _e2907) * 0.5f);
                                                                        if (_e2954 != _e2954) {
                                                                            phi_28274_ = true;
                                                                        } else {
                                                                            phi_28274_ = (0f >= _e2954);
                                                                        }
                                                                        let _e2960 = phi_28274_;
                                                                        let _e2962 = sqrt(select(_e2954, 0f, _e2960));
                                                                        if (_e2955 != _e2955) {
                                                                            phi_28289_ = true;
                                                                        } else {
                                                                            phi_28289_ = (0f >= _e2955);
                                                                        }
                                                                        let _e2966 = phi_28289_;
                                                                        let _e2968 = sqrt(select(_e2955, 0f, _e2966));
                                                                        if (_e2956 != _e2956) {
                                                                            phi_28304_ = true;
                                                                        } else {
                                                                            phi_28304_ = (0f >= _e2956);
                                                                        }
                                                                        let _e2972 = phi_28304_;
                                                                        let _e2974 = sqrt(select(_e2956, 0f, _e2972));
                                                                        let _e2975 = vec3<f32>(_e2962, _e2968, _e2974);
                                                                        let _e2979 = ((_e2938 - _e2905) * 0.5f);
                                                                        let _e2980 = ((_e2944 - _e2906) * 0.5f);
                                                                        let _e2981 = ((_e2950 - _e2907) * 0.5f);
                                                                        if (_e2979 != _e2979) {
                                                                            phi_28331_ = true;
                                                                        } else {
                                                                            phi_28331_ = (0f >= _e2979);
                                                                        }
                                                                        let _e2985 = phi_28331_;
                                                                        let _e2987 = sqrt(select(_e2979, 0f, _e2985));
                                                                        if (_e2980 != _e2980) {
                                                                            phi_28346_ = true;
                                                                        } else {
                                                                            phi_28346_ = (0f >= _e2980);
                                                                        }
                                                                        let _e2991 = phi_28346_;
                                                                        let _e2993 = sqrt(select(_e2980, 0f, _e2991));
                                                                        if (_e2981 != _e2981) {
                                                                            phi_28361_ = true;
                                                                        } else {
                                                                            phi_28361_ = (0f >= _e2981);
                                                                        }
                                                                        let _e2997 = phi_28361_;
                                                                        let _e2999 = sqrt(select(_e2981, 0f, _e2997));
                                                                        if ((_e2910 / _e2895) < 0f) {
                                                                            phi_14973_ = -(_e2987);
                                                                        } else {
                                                                            phi_14973_ = _e2987;
                                                                        }
                                                                        let _e3003 = phi_14973_;
                                                                        if ((_e2911 / _e2896) < 0f) {
                                                                            phi_14981_ = -(_e2993);
                                                                        } else {
                                                                            phi_14981_ = _e2993;
                                                                        }
                                                                        let _e3007 = phi_14981_;
                                                                        if ((_e2912 / _e2897) < 0f) {
                                                                            phi_14989_ = -(_e2999);
                                                                        } else {
                                                                            phi_14989_ = _e2999;
                                                                        }
                                                                        let _e3011 = phi_14989_;
                                                                        let _e3015 = (_e1966.x * _e2865);
                                                                        let _e3016 = (_e1966.y * _e2865);
                                                                        let _e3017 = (_e1966.z * _e2865);
                                                                        let _e3027 = vec3<f32>(((_e1968.x * _e2962) - (_e1966.x * _e3003)), ((_e1968.y * _e2968) - (_e1966.y * _e3007)), ((_e1968.z * _e2974) - (_e1966.z * _e3011)));
                                                                        let _e3034 = ((_e1968.x * _e3003) + (_e1966.x * _e2962));
                                                                        let _e3035 = ((_e1968.y * _e3007) + (_e1966.y * _e2968));
                                                                        let _e3036 = ((_e1968.z * _e3011) + (_e1966.z * _e2974));
                                                                        let _e3038 = vec3<f32>((_e1968.x * _e2865), (_e1968.y * _e2865), (_e1968.z * _e2865));
                                                                        let _e3039 = (_e3038 - _e2975);
                                                                        let _e3040 = (_e3015 - _e3003);
                                                                        let _e3041 = (_e3016 - _e3007);
                                                                        let _e3042 = (_e3017 - _e3011);
                                                                        let _e3055 = (_e3038 + _e2975);
                                                                        let _e3056 = (_e3015 + _e3003);
                                                                        let _e3057 = (_e3016 + _e3007);
                                                                        let _e3058 = (_e3017 + _e3011);
                                                                        let _e3068 = ((_e3055.x * _e3055.x) + (_e3056 * _e3056));
                                                                        let _e3069 = ((_e3055.y * _e3055.y) + (_e3057 * _e3057));
                                                                        let _e3070 = ((_e3055.z * _e3055.z) + (_e3058 * _e3058));
                                                                        let _e3080 = vec3(_e2865);
                                                                        let _e3081 = (_e3080 - _e3027);
                                                                        let _e3082 = -(vec3<f32>(_e3034, _e3035, _e3036));
                                                                        let _e3098 = (_e3080 + _e3027);
                                                                        let _e3108 = ((_e3098.x * _e3098.x) + (_e3034 * _e3034));
                                                                        let _e3109 = ((_e3098.y * _e3098.y) + (_e3035 * _e3035));
                                                                        let _e3110 = ((_e3098.z * _e3098.z) + (_e3036 * _e3036));
                                                                        phi_15096_ = vec3<f32>((((((_e3039.x * _e3039.x) + (_e3040 * _e3040)) / select(0.000000000000000000000000000001f, _e3068, (_e3068 > 0.000000000000000000000000000001f))) + (((_e3081.x * _e3081.x) + (_e3082.x * _e3082.x)) / select(0.000000000000000000000000000001f, _e3108, (_e3108 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3039.y * _e3039.y) + (_e3041 * _e3041)) / select(0.000000000000000000000000000001f, _e3069, (_e3069 > 0.000000000000000000000000000001f))) + (((_e3081.y * _e3081.y) + (_e3082.y * _e3082.y)) / select(0.000000000000000000000000000001f, _e3109, (_e3109 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3039.z * _e3039.z) + (_e3042 * _e3042)) / select(0.000000000000000000000000000001f, _e3070, (_e3070 > 0.000000000000000000000000000001f))) + (((_e3081.z * _e3081.z) + (_e3082.z * _e3082.z)) / select(0.000000000000000000000000000001f, _e3110, (_e3110 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3153 = phi_15096_;
                                                                    phi_15097_ = _e3153;
                                                                }
                                                                let _e3173 = phi_15097_;
                                                                let _e3175 = (4f * _e2787);
                                                                phi_15109_ = type_59((_e3173 * (((1f / (_e2838 + _e2852)) * _e2823) / _e3175)), ((_e2823 / _e2838) / _e3175));
                                                            }
                                                            let _e3182 = phi_15109_;
                                                            phi_15110_ = _e3182;
                                                            phi_15111_ = _e2799;
                                                        }
                                                        let _e3184 = phi_15110_;
                                                        let _e3186 = phi_15111_;
                                                        if _e3186 {
                                                            phi_15116_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15116_ = _e3184;
                                                        }
                                                        let _e3188 = phi_15116_;
                                                        let _e3191 = (_e1974 == 1u);
                                                        if _e3191 {
                                                            phi_15144_ = _e3188;
                                                            phi_15145_ = type_59();
                                                        } else {
                                                            if (_e2787 != _e2787) {
                                                                phi_28623_ = true;
                                                            } else {
                                                                phi_28623_ = (0f >= _e2787);
                                                            }
                                                            let _e3195 = phi_28623_;
                                                            let _e3196 = select(_e2787, 0f, _e3195);
                                                            let _e3197 = (1f / _e1964);
                                                            let _e3201 = ((_e3197 * _e3197) * (1f - (_e3196 * _e3196)));
                                                            if (_e3201 >= 1f) {
                                                                phi_28642_ = 1f;
                                                            } else {
                                                                let _e3204 = sqrt((1f - _e3201));
                                                                if (_e3196 != _e3196) {
                                                                    phi_28653_ = true;
                                                                } else {
                                                                    phi_28653_ = (_e3204 <= _e3196);
                                                                }
                                                                let _e3208 = phi_28653_;
                                                                let _e3213 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                                phi_28642_ = (_e3213 + ((1f - _e3213) * pow((1f - select(_e3196, _e3204, _e3208)), 5f)));
                                                            }
                                                            let _e3220 = phi_28642_;
                                                            if (_e2765 <= 0f) {
                                                                phi_28692_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_28692_ = (_e1962 * (_e2765 * 0.31830987f));
                                                            }
                                                            let _e3225 = phi_28692_;
                                                            let _e3226 = (1f - _e3220);
                                                            phi_15144_ = type_59();
                                                            phi_15145_ = type_59((_e3188.member + vec3<f32>((_e3225.x * _e3226), (_e3225.y * _e3226), (_e3225.z * _e3226))), ((_e3220 * _e3188.member_1) + (_e3226 * _e2771)));
                                                        }
                                                        let _e3240 = phi_15144_;
                                                        let _e3242 = phi_15145_;
                                                        phi_15146_ = _e3240;
                                                        phi_15147_ = _e3242;
                                                        phi_15148_ = _e3191;
                                                    }
                                                    let _e3250 = phi_15146_;
                                                    let _e3252 = phi_15147_;
                                                    let _e3254 = phi_15148_;
                                                    if _e3254 {
                                                        phi_15152_ = _e3250;
                                                    } else {
                                                        phi_15152_ = _e3252;
                                                    }
                                                    let _e3256 = phi_15152_;
                                                    if ((_e3256.member_1 * _e2760) < 0.001f) {
                                                        let _e3683 = global_1.member.member_6;
                                                        let _e3689 = global_1.member.member_8[2u];
                                                        if (_e3689 == 4294967295u) {
                                                            phi_15891_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e3692 = global_6.member[_e3689];
                                                            let _e3695 = global_6.member[(_e3689 + 1u)];
                                                            let _e3698 = global_6.member[(_e3689 + 2u)];
                                                            let _e3701 = global_6.member[(_e3689 + 3u)];
                                                            let _e3704 = global_6.member[(_e3689 + 4u)];
                                                            let _e3707 = global_6.member[(_e3689 + 5u)];
                                                            let _e3710 = global_6.member[(_e3689 + 6u)];
                                                            let _e3713 = global_6.member[(_e3689 + 7u)];
                                                            let _e3716 = global_6.member[(_e3689 + 8u)];
                                                            let _e3719 = global_6.member[(_e3689 + 9u)];
                                                            let _e3722 = global_6.member[(_e3689 + 13u)];
                                                            let _e3725 = global_6.member[(_e3689 + 14u)];
                                                            let _e3728 = global_6.member[(_e3689 + 15u)];
                                                            let _e3731 = global_6.member[(_e3689 + 16u)];
                                                            let _e3734 = global_6.member[(_e3689 + 18u)];
                                                            let _e3737 = global_6.member[(_e3689 + 19u)];
                                                            let _e3740 = global_6.member[(_e3689 + 20u)];
                                                            let _e3743 = global_6.member[(_e3689 + 29u)];
                                                            let _e3744 = bitcast<u32>(_e3743);
                                                            let _e3757 = (((((_e1106 * _e1106) + (_e1107 * _e1107)) + (_e1108 * _e1108)) - (2f * (((_e1106 * _e3692) + (_e1107 * _e3695)) + (_e1108 * _e3698)))) + _e3710);
                                                            if (_e3757 > 0f) {
                                                                let _e3766 = ((((_e1106 - _e3692) * _e3713) + ((_e1107 - _e3695) * _e3716)) + ((_e1108 - _e3698) * _e3719));
                                                                let _e3768 = ((_e3766 * _e3766) - _e3757);
                                                                if (_e3766 >= 0f) {
                                                                    phi_15591_ = vec3<f32>();
                                                                    phi_15592_ = true;
                                                                } else {
                                                                    let _e3770 = (_e3768 <= 0f);
                                                                    if _e3770 {
                                                                        phi_15590_ = vec3<f32>();
                                                                    } else {
                                                                        let _e3773 = (-(_e3766) - sqrt(_e3768));
                                                                        phi_15590_ = vec3<f32>((_e1106 + (_e3713 * _e3773)), (_e1107 + (_e3716 * _e3773)), (_e1108 + (_e3719 * _e3773)));
                                                                    }
                                                                    let _e3782 = phi_15590_;
                                                                    phi_15591_ = _e3782;
                                                                    phi_15592_ = _e3770;
                                                                }
                                                                let _e3784 = phi_15591_;
                                                                let _e3786 = phi_15592_;
                                                                phi_15597_ = _e3784;
                                                                phi_15598_ = select(true, false, _e3786);
                                                            } else {
                                                                phi_15597_ = _e1109;
                                                                phi_15598_ = true;
                                                            }
                                                            let _e3789 = phi_15597_;
                                                            let _e3791 = phi_15598_;
                                                            if _e3791 {
                                                                let _e3793 = (_e3789.x - _e3692);
                                                                let _e3795 = (_e3789.y - _e3695);
                                                                let _e3797 = (_e3789.z - _e3698);
                                                                let _e3803 = sqrt((((_e3793 * _e3793) + (_e3795 * _e3795)) + (_e3797 * _e3797)));
                                                                let _e3804 = (1f / _e3803);
                                                                let _e3822 = ((((((_e3789.x * _e3789.x) + (_e3789.y * _e3789.y)) + (_e3789.z * _e3789.z)) - (2f * (((_e3789.x * _e3692) + (_e3789.y * _e3695)) + (_e3789.z * _e3698)))) + _e3707) / (_e3803 + _e3701));
                                                                if (_e3822 != _e3822) {
                                                                    phi_29148_ = true;
                                                                } else {
                                                                    phi_29148_ = (0f >= _e3822);
                                                                }
                                                                let _e3826 = phi_29148_;
                                                                let _e3830 = sqrt((select(_e3822, 0f, _e3826) / (_e3704 - _e3701)));
                                                                if (_e3830 != _e3830) {
                                                                    phi_29163_ = true;
                                                                } else {
                                                                    phi_29163_ = (1f <= _e3830);
                                                                }
                                                                let _e3834 = phi_29163_;
                                                                let _e3836 = (select(_e3830, 1f, _e3834) * 63f);
                                                                let _e3841 = ((((_e3793 * _e3804) * _e3713) + ((_e3795 * _e3804) * _e3716)) + ((_e3797 * _e3804) * _e3719));
                                                                let _e3843 = select(_e3841, -1f, (_e3841 < -1f));
                                                                let _e3847 = ((select(_e3843, 1f, (_e3843 > 1f)) + 1f) * 127.5f);
                                                                let _e3852 = select(select(u32(_e3836), 0u, (_e3836 < 0f)), 4294967295u, (_e3836 > 4294967000f));
                                                                let _e3854 = select(_e3852, 62u, (62u < _e3852));
                                                                let _e3859 = select(select(u32(_e3847), 0u, (_e3847 < 0f)), 4294967295u, (_e3847 > 4294967000f));
                                                                let _e3861 = select(_e3859, 254u, (254u < _e3859));
                                                                let _e3863 = (_e3836 - f32(_e3854));
                                                                let _e3865 = (_e3847 - f32(_e3861));
                                                                let _e3869 = (_e3744 + (3u * ((_e3854 * 256u) + _e3861)));
                                                                let _e3874 = (_e3744 + (3u * (((_e3854 + 1u) * 256u) + _e3861)));
                                                                let _e3876 = global_6.member[_e3869];
                                                                let _e3879 = global_6.member[(_e3869 + 1u)];
                                                                let _e3882 = global_6.member[(_e3869 + 2u)];
                                                                let _e3883 = (1f - _e3865);
                                                                let _e3889 = global_6.member[(_e3869 + 3u)];
                                                                let _e3892 = global_6.member[(_e3869 + 4u)];
                                                                let _e3895 = global_6.member[(_e3869 + 5u)];
                                                                let _e3902 = (1f - _e3863);
                                                                let _e3907 = global_6.member[_e3874];
                                                                let _e3910 = global_6.member[(_e3874 + 1u)];
                                                                let _e3913 = global_6.member[(_e3874 + 2u)];
                                                                let _e3919 = global_6.member[(_e3874 + 3u)];
                                                                let _e3922 = global_6.member[(_e3874 + 4u)];
                                                                let _e3925 = global_6.member[(_e3874 + 5u)];
                                                                phi_15889_ = vec3<f32>(((((_e3876 * _e3883) + (_e3889 * _e3865)) * _e3902) + (((_e3907 * _e3883) + (_e3919 * _e3865)) * _e3863)), ((((_e3879 * _e3883) + (_e3892 * _e3865)) * _e3902) + (((_e3910 * _e3883) + (_e3922 * _e3865)) * _e3863)), ((((_e3882 * _e3883) + (_e3895 * _e3865)) * _e3902) + (((_e3913 * _e3883) + (_e3925 * _e3865)) * _e3863)));
                                                            } else {
                                                                phi_15889_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3940 = phi_15889_;
                                                            let _e3947 = ((_e3731 * 1.1f) * _e3940.y);
                                                            phi_15891_ = vec3<f32>(exp(-((((_e3722 * _e3940.x) + _e3947) + (_e3734 * _e3940.z)))), exp(-((((_e3725 * _e3940.x) + _e3947) + (_e3737 * _e3940.z)))), exp(-((((_e3728 * _e3940.x) + _e3947) + (_e3740 * _e3940.z)))));
                                                        }
                                                        let _e3966 = phi_15891_;
                                                        phi_36559_ = _e1989;
                                                        phi_16775_ = 0f;
                                                        phi_16776_ = vec3<f32>(_e2748.x, _e2748.y, _e2748.z);
                                                        phi_16777_ = (vec3<f32>(_e3683.x, _e3683.y, _e3683.z) * _e3966);
                                                    } else {
                                                        let _e3261 = ((_e1989 * 747796405u) + 2891336453u);
                                                        let _e3269 = (((_e3261 >> bitcast<u32>((((_e3261 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3261) * 277803737u);
                                                        let _e3272 = ((_e3269 >> bitcast<u32>(22i)) ^ _e3269);
                                                        let _e3278 = (((f32((_e3272 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2760) * 0.15915494f);
                                                        let _e3279 = (1f - _e3278);
                                                        let _e3281 = (_e3278 * (2f - _e3278));
                                                        if (_e3281 != _e3281) {
                                                            phi_28746_ = true;
                                                        } else {
                                                            phi_28746_ = (0f >= _e3281);
                                                        }
                                                        let _e3285 = phi_28746_;
                                                        let _e3287 = sqrt(select(_e3281, 0f, _e3285));
                                                        let _e3289 = ((_e3272 * 747796405u) + 2891336453u);
                                                        let _e3297 = (((_e3289 >> bitcast<u32>((((_e3289 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3289) * 277803737u);
                                                        let _e3300 = ((_e3297 >> bitcast<u32>(22i)) ^ _e3297);
                                                        let _e3304 = (f32((_e3300 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3306 = select(-1f, 1f, (_e2748.z >= 0f));
                                                        let _e3308 = (-1f / (_e3306 + _e2748.z));
                                                        let _e3310 = ((_e2748.x * _e2748.y) * _e3308);
                                                        let _e3323 = (_e3287 * cos(_e3304));
                                                        let _e3328 = (_e3287 * sin(_e3304));
                                                        let _e3338 = ((((1f + (((_e3306 * _e2748.x) * _e2748.x) * _e3308)) * _e3323) + (_e3310 * _e3328)) + (_e2748.x * _e3279));
                                                        let _e3339 = ((((_e3306 * _e3310) * _e3323) + ((_e3306 + ((_e2748.y * _e2748.y) * _e3308)) * _e3328)) + (_e2748.y * _e3279));
                                                        let _e3340 = ((((-(_e3306) * _e2748.x) * _e3323) + (-(_e2748.y) * _e3328)) + (_e2748.z * _e3279));
                                                        let _e3342 = (_e3338 - _e2748.x);
                                                        let _e3343 = (_e3339 - _e2748.y);
                                                        let _e3344 = (_e3340 - _e2748.z);
                                                        let _e3351 = (sqrt((((_e3342 * _e3342) + (_e3343 * _e3343)) + (_e3344 * _e3344))) * 0.5f);
                                                        let _e3352 = (_e3351 != _e3351);
                                                        if _e3352 {
                                                            phi_28817_ = true;
                                                        } else {
                                                            phi_28817_ = (1f <= _e3351);
                                                        }
                                                        let _e3355 = phi_28817_;
                                                        if ((2f * asin(select(_e3351, 1f, _e3355))) >= _e2756) {
                                                            phi_15972_ = 0f;
                                                        } else {
                                                            phi_15972_ = (1f / _e2760);
                                                        }
                                                        let _e3362 = phi_15972_;
                                                        if (_e3362 > 0f) {
                                                            if _e3352 {
                                                                phi_28832_ = true;
                                                            } else {
                                                                phi_28832_ = (1f <= _e3351);
                                                            }
                                                            let _e3366 = phi_28832_;
                                                            let _e3369 = (2f * asin(select(_e3351, 1f, _e3366)));
                                                            if (_e3369 >= _e2756) {
                                                                phi_16773_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3373 = (1f - pow((_e3369 / _e2756), 2f));
                                                                if (_e3373 != _e3373) {
                                                                    phi_28847_ = true;
                                                                } else {
                                                                    phi_28847_ = (0f >= _e3373);
                                                                }
                                                                let _e3377 = phi_28847_;
                                                                let _e3380 = global_1.member.member_6;
                                                                let _e3386 = global_1.member.member_8[2u];
                                                                if (_e3386 == 4294967295u) {
                                                                    phi_16746_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e3389 = global_6.member[_e3386];
                                                                    let _e3392 = global_6.member[(_e3386 + 1u)];
                                                                    let _e3395 = global_6.member[(_e3386 + 2u)];
                                                                    let _e3398 = global_6.member[(_e3386 + 3u)];
                                                                    let _e3401 = global_6.member[(_e3386 + 4u)];
                                                                    let _e3404 = global_6.member[(_e3386 + 5u)];
                                                                    let _e3407 = global_6.member[(_e3386 + 6u)];
                                                                    let _e3410 = global_6.member[(_e3386 + 7u)];
                                                                    let _e3413 = global_6.member[(_e3386 + 8u)];
                                                                    let _e3416 = global_6.member[(_e3386 + 9u)];
                                                                    let _e3419 = global_6.member[(_e3386 + 13u)];
                                                                    let _e3422 = global_6.member[(_e3386 + 14u)];
                                                                    let _e3425 = global_6.member[(_e3386 + 15u)];
                                                                    let _e3428 = global_6.member[(_e3386 + 16u)];
                                                                    let _e3431 = global_6.member[(_e3386 + 18u)];
                                                                    let _e3434 = global_6.member[(_e3386 + 19u)];
                                                                    let _e3437 = global_6.member[(_e3386 + 20u)];
                                                                    let _e3440 = global_6.member[(_e3386 + 29u)];
                                                                    let _e3441 = bitcast<u32>(_e3440);
                                                                    let _e3454 = (((((_e1106 * _e1106) + (_e1107 * _e1107)) + (_e1108 * _e1108)) - (2f * (((_e1106 * _e3389) + (_e1107 * _e3392)) + (_e1108 * _e3395)))) + _e3407);
                                                                    if (_e3454 > 0f) {
                                                                        let _e3463 = ((((_e1106 - _e3389) * _e3410) + ((_e1107 - _e3392) * _e3413)) + ((_e1108 - _e3395) * _e3416));
                                                                        let _e3465 = ((_e3463 * _e3463) - _e3454);
                                                                        if (_e3463 >= 0f) {
                                                                            phi_16446_ = vec3<f32>();
                                                                            phi_16447_ = true;
                                                                        } else {
                                                                            let _e3467 = (_e3465 <= 0f);
                                                                            if _e3467 {
                                                                                phi_16445_ = vec3<f32>();
                                                                            } else {
                                                                                let _e3470 = (-(_e3463) - sqrt(_e3465));
                                                                                phi_16445_ = vec3<f32>((_e1106 + (_e3410 * _e3470)), (_e1107 + (_e3413 * _e3470)), (_e1108 + (_e3416 * _e3470)));
                                                                            }
                                                                            let _e3479 = phi_16445_;
                                                                            phi_16446_ = _e3479;
                                                                            phi_16447_ = _e3467;
                                                                        }
                                                                        let _e3481 = phi_16446_;
                                                                        let _e3483 = phi_16447_;
                                                                        phi_16452_ = _e3481;
                                                                        phi_16453_ = select(true, false, _e3483);
                                                                    } else {
                                                                        phi_16452_ = _e1109;
                                                                        phi_16453_ = true;
                                                                    }
                                                                    let _e3486 = phi_16452_;
                                                                    let _e3488 = phi_16453_;
                                                                    if _e3488 {
                                                                        let _e3490 = (_e3486.x - _e3389);
                                                                        let _e3492 = (_e3486.y - _e3392);
                                                                        let _e3494 = (_e3486.z - _e3395);
                                                                        let _e3500 = sqrt((((_e3490 * _e3490) + (_e3492 * _e3492)) + (_e3494 * _e3494)));
                                                                        let _e3501 = (1f / _e3500);
                                                                        let _e3519 = ((((((_e3486.x * _e3486.x) + (_e3486.y * _e3486.y)) + (_e3486.z * _e3486.z)) - (2f * (((_e3486.x * _e3389) + (_e3486.y * _e3392)) + (_e3486.z * _e3395)))) + _e3404) / (_e3500 + _e3398));
                                                                        if (_e3519 != _e3519) {
                                                                            phi_28971_ = true;
                                                                        } else {
                                                                            phi_28971_ = (0f >= _e3519);
                                                                        }
                                                                        let _e3523 = phi_28971_;
                                                                        let _e3527 = sqrt((select(_e3519, 0f, _e3523) / (_e3401 - _e3398)));
                                                                        if (_e3527 != _e3527) {
                                                                            phi_28986_ = true;
                                                                        } else {
                                                                            phi_28986_ = (1f <= _e3527);
                                                                        }
                                                                        let _e3531 = phi_28986_;
                                                                        let _e3533 = (select(_e3527, 1f, _e3531) * 63f);
                                                                        let _e3538 = ((((_e3490 * _e3501) * _e3410) + ((_e3492 * _e3501) * _e3413)) + ((_e3494 * _e3501) * _e3416));
                                                                        let _e3540 = select(_e3538, -1f, (_e3538 < -1f));
                                                                        let _e3544 = ((select(_e3540, 1f, (_e3540 > 1f)) + 1f) * 127.5f);
                                                                        let _e3549 = select(select(u32(_e3533), 0u, (_e3533 < 0f)), 4294967295u, (_e3533 > 4294967000f));
                                                                        let _e3551 = select(_e3549, 62u, (62u < _e3549));
                                                                        let _e3556 = select(select(u32(_e3544), 0u, (_e3544 < 0f)), 4294967295u, (_e3544 > 4294967000f));
                                                                        let _e3558 = select(_e3556, 254u, (254u < _e3556));
                                                                        let _e3560 = (_e3533 - f32(_e3551));
                                                                        let _e3562 = (_e3544 - f32(_e3558));
                                                                        let _e3566 = (_e3441 + (3u * ((_e3551 * 256u) + _e3558)));
                                                                        let _e3571 = (_e3441 + (3u * (((_e3551 + 1u) * 256u) + _e3558)));
                                                                        let _e3573 = global_6.member[_e3566];
                                                                        let _e3576 = global_6.member[(_e3566 + 1u)];
                                                                        let _e3579 = global_6.member[(_e3566 + 2u)];
                                                                        let _e3580 = (1f - _e3562);
                                                                        let _e3586 = global_6.member[(_e3566 + 3u)];
                                                                        let _e3589 = global_6.member[(_e3566 + 4u)];
                                                                        let _e3592 = global_6.member[(_e3566 + 5u)];
                                                                        let _e3599 = (1f - _e3560);
                                                                        let _e3604 = global_6.member[_e3571];
                                                                        let _e3607 = global_6.member[(_e3571 + 1u)];
                                                                        let _e3610 = global_6.member[(_e3571 + 2u)];
                                                                        let _e3616 = global_6.member[(_e3571 + 3u)];
                                                                        let _e3619 = global_6.member[(_e3571 + 4u)];
                                                                        let _e3622 = global_6.member[(_e3571 + 5u)];
                                                                        phi_16744_ = vec3<f32>(((((_e3573 * _e3580) + (_e3586 * _e3562)) * _e3599) + (((_e3604 * _e3580) + (_e3616 * _e3562)) * _e3560)), ((((_e3576 * _e3580) + (_e3589 * _e3562)) * _e3599) + (((_e3607 * _e3580) + (_e3619 * _e3562)) * _e3560)), ((((_e3579 * _e3580) + (_e3592 * _e3562)) * _e3599) + (((_e3610 * _e3580) + (_e3622 * _e3562)) * _e3560)));
                                                                    } else {
                                                                        phi_16744_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e3637 = phi_16744_;
                                                                    let _e3644 = ((_e3428 * 1.1f) * _e3637.y);
                                                                    phi_16746_ = vec3<f32>(exp(-((((_e3419 * _e3637.x) + _e3644) + (_e3431 * _e3637.z)))), exp(-((((_e3422 * _e3637.x) + _e3644) + (_e3434 * _e3637.z)))), exp(-((((_e3425 * _e3637.x) + _e3644) + (_e3437 * _e3637.z)))));
                                                                }
                                                                let _e3663 = phi_16746_;
                                                                let _e3674 = (((1f - (0.6f * (1f - sqrt(select(_e3373, 0f, _e3377))))) * 1.25f) / _e2760);
                                                                phi_16773_ = vec3<f32>(((_e3380.x * _e3663.x) * _e3674), ((_e3380.y * _e3663.y) * _e3674), ((_e3380.z * _e3663.z) * _e3674));
                                                            }
                                                            let _e3680 = phi_16773_;
                                                            phi_16774_ = _e3680;
                                                        } else {
                                                            phi_16774_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3682 = phi_16774_;
                                                        phi_36559_ = _e3300;
                                                        phi_16775_ = _e3362;
                                                        phi_16776_ = vec3<f32>(_e3338, _e3339, _e3340);
                                                        phi_16777_ = _e3682;
                                                    }
                                                    let _e3970 = phi_36559_;
                                                    let _e3972 = phi_16775_;
                                                    let _e3974 = phi_16776_;
                                                    let _e3976 = phi_16777_;
                                                    phi_36558_ = _e3970;
                                                    phi_16778_ = _e3972;
                                                    phi_16779_ = _e3974;
                                                    phi_16780_ = _e3976;
                                                }
                                                let _e3978 = phi_36558_;
                                                let _e3980 = phi_16778_;
                                                let _e3982 = phi_16779_;
                                                let _e3984 = phi_16780_;
                                                phi_36515_ = _e3978;
                                                phi_18145_ = _e3980;
                                                phi_18146_ = _e3982;
                                                phi_18147_ = _e3984;
                                                phi_18148_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e1995 == (_e1985 + 1u)) {
                                                    if ((_e379 & 1u) == 0u) {
                                                        phi_36524_ = _e1989;
                                                        phi_17779_ = 0f;
                                                        phi_17780_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17781_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17782_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e382 > 0u) {
                                                            let _e2226 = global_1.member.member_2[1u];
                                                            let _e2228 = ((_e1989 * 747796405u) + 2891336453u);
                                                            let _e2236 = (((_e2228 >> bitcast<u32>((((_e2228 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2228) * 277803737u);
                                                            let _e2239 = ((_e2236 >> bitcast<u32>(22i)) ^ _e2236);
                                                            phi_16801_ = 0u;
                                                            phi_16804_ = _e382;
                                                            loop {
                                                                let _e2245 = phi_16801_;
                                                                let _e2247 = phi_16804_;
                                                                local_78 = _e2245;
                                                                local_79 = _e2245;
                                                                let _e2249 = ((_e2245 + 1u) < _e2247);
                                                                if _e2249 {
                                                                    let _e2251 = ((_e2245 + _e2247) / 2u);
                                                                    let _e2254 = global_6.member[(_e2226 + _e2251)];
                                                                    let _e2255 = (_e2254 <= (f32((_e2239 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_16802_ = select(_e2245, _e2251, _e2255);
                                                                    phi_16805_ = select(_e2251, _e2247, _e2255);
                                                                } else {
                                                                    phi_16802_ = u32();
                                                                    phi_16805_ = u32();
                                                                }
                                                                let _e2259 = phi_16802_;
                                                                let _e2261 = phi_16805_;
                                                                phi_16801_ = _e2259;
                                                                phi_16804_ = _e2261;
                                                                if !(_e2249) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2264 = local_78;
                                                            let _e2265 = (_e2226 + _e2264);
                                                            let _e2268 = global_6.member[(_e2265 + 1u)];
                                                            let _e2270 = global_6.member[_e2265];
                                                            let _e2274 = global_1.member.member_2[0u];
                                                            let _e2276 = local_79;
                                                            let _e2279 = global_6.member[(_e2274 + _e2276)];
                                                            let _e2280 = bitcast<u32>(_e2279);
                                                            let _e2282 = ((_e2239 * 747796405u) + 2891336453u);
                                                            let _e2290 = (((_e2282 >> bitcast<u32>((((_e2282 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2282) * 277803737u);
                                                            let _e2293 = ((_e2290 >> bitcast<u32>(22i)) ^ _e2290);
                                                            let _e2296 = f32((_e2293 >> bitcast<u32>(8i)));
                                                            let _e2297 = (_e2296 * 0.000000059604645f);
                                                            let _e2299 = ((_e2293 * 747796405u) + 2891336453u);
                                                            let _e2307 = (((_e2299 >> bitcast<u32>((((_e2299 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2299) * 277803737u);
                                                            let _e2310 = ((_e2307 >> bitcast<u32>(22i)) ^ _e2307);
                                                            let _e2313 = f32((_e2310 >> bitcast<u32>(8i)));
                                                            let _e2315 = (3u * _e2280);
                                                            let _e2317 = global_2.member[_e2315];
                                                            let _e2320 = global_2.member[(_e2315 + 1u)];
                                                            let _e2326 = select(select(u32(_e2320.w), 0u, (_e2320.w < 0f)), 4294967295u, (_e2320.w > 4294967000f));
                                                            let _e2328 = (_e851 + (5u * _e2280));
                                                            let _e2330 = global_2.member[_e2328];
                                                            let _e2333 = global_2.member[(_e2328 + 1u)];
                                                            let _e2340 = (_e2326 == 1u);
                                                            if _e2340 {
                                                                let _e2469 = (_e2317.x - _e1106);
                                                                let _e2470 = (_e2317.y - _e1107);
                                                                let _e2471 = (_e2317.z - _e1108);
                                                                let _e2476 = (((_e2469 * _e2469) + (_e2470 * _e2470)) + (_e2471 * _e2471));
                                                                let _e2477 = (_e2317.w * _e2317.w);
                                                                let _e2479 = (_e2476 <= (_e2477 * 1.000001f));
                                                                if _e2479 {
                                                                    let _e2588 = (_e2296 * 0.00000037450704f);
                                                                    let _e2590 = ((_e2313 * 0.00000011920929f) - 1f);
                                                                    let _e2592 = (1f - (_e2590 * _e2590));
                                                                    if (_e2592 != _e2592) {
                                                                        phi_27814_ = true;
                                                                    } else {
                                                                        phi_27814_ = (0f >= _e2592);
                                                                    }
                                                                    let _e2596 = phi_27814_;
                                                                    let _e2598 = sqrt(select(_e2592, 0f, _e2596));
                                                                    let _e2600 = (_e2598 * cos(_e2588));
                                                                    let _e2602 = (_e2598 * sin(_e2588));
                                                                    let _e2607 = (_e2317.x + (_e2600 * _e2317.w));
                                                                    let _e2608 = (_e2317.y + (_e2602 * _e2317.w));
                                                                    let _e2609 = (_e2317.z + (_e2590 * _e2317.w));
                                                                    if _e2479 {
                                                                        let _e2622 = (_e2607 - _e1106);
                                                                        let _e2623 = (_e2608 - _e1107);
                                                                        let _e2624 = (_e2609 - _e1108);
                                                                        let _e2629 = (((_e2622 * _e2622) + (_e2623 * _e2623)) + (_e2624 * _e2624));
                                                                        phi_27883_ = (_e2629 / (((abs((((_e2600 * _e2622) + (_e2602 * _e2623)) + (_e2590 * _e2624))) / sqrt(_e2629)) * 12.566371f) * _e2477));
                                                                    } else {
                                                                        let _e2611 = (_e2477 / _e2476);
                                                                        if (_e2611 != _e2611) {
                                                                            phi_27902_ = true;
                                                                        } else {
                                                                            phi_27902_ = (1f <= _e2611);
                                                                        }
                                                                        let _e2615 = phi_27902_;
                                                                        let _e2616 = select(_e2611, 1f, _e2615);
                                                                        phi_27883_ = (0.15915494f / (_e2616 / (1f + sqrt((1f - _e2616)))));
                                                                    }
                                                                    let _e2642 = phi_27883_;
                                                                    phi_17152_ = type_72(vec3<f32>(_e2607, _e2608, _e2609), vec3<f32>(_e2600, _e2602, _e2590), _e2642);
                                                                } else {
                                                                    let _e2480 = (_e2477 / _e2476);
                                                                    let _e2481 = (_e2480 != _e2480);
                                                                    if _e2481 {
                                                                        phi_27666_ = true;
                                                                    } else {
                                                                        phi_27666_ = (1f <= _e2480);
                                                                    }
                                                                    let _e2484 = phi_27666_;
                                                                    let _e2485 = select(_e2480, 1f, _e2484);
                                                                    if _e2481 {
                                                                        phi_27689_ = true;
                                                                    } else {
                                                                        phi_27689_ = (1f <= _e2480);
                                                                    }
                                                                    let _e2489 = phi_27689_;
                                                                    let _e2490 = select(_e2480, 1f, _e2489);
                                                                    let _e2494 = (_e2490 / (1f + sqrt((1f - _e2490))));
                                                                    if (_e2485 < 0.00068523f) {
                                                                        phi_17087_ = (_e2485 * _e2297);
                                                                    } else {
                                                                        let _e2497 = (1f - (_e2297 * _e2494));
                                                                        let _e2499 = (1f - (_e2497 * _e2497));
                                                                        if (_e2499 != _e2499) {
                                                                            phi_27704_ = true;
                                                                        } else {
                                                                            phi_27704_ = (0f >= _e2499);
                                                                        }
                                                                        let _e2503 = phi_27704_;
                                                                        phi_17087_ = select(_e2499, 0f, _e2503);
                                                                    }
                                                                    let _e2507 = phi_17087_;
                                                                    let _e2508 = (1f - _e2507);
                                                                    if (_e2508 != _e2508) {
                                                                        phi_27719_ = true;
                                                                    } else {
                                                                        phi_27719_ = (0f >= _e2508);
                                                                    }
                                                                    let _e2512 = phi_27719_;
                                                                    let _e2517 = (1f - (_e2507 / _e2485));
                                                                    if (_e2517 != _e2517) {
                                                                        phi_27734_ = true;
                                                                    } else {
                                                                        phi_27734_ = (0f >= _e2517);
                                                                    }
                                                                    let _e2521 = phi_27734_;
                                                                    let _e2525 = ((_e2507 / sqrt(_e2485)) + (sqrt(select(_e2508, 0f, _e2512)) * sqrt(select(_e2517, 0f, _e2521))));
                                                                    let _e2527 = (1f - (_e2525 * _e2525));
                                                                    if (_e2527 != _e2527) {
                                                                        phi_27749_ = true;
                                                                    } else {
                                                                        phi_27749_ = (0f >= _e2527);
                                                                    }
                                                                    let _e2531 = phi_27749_;
                                                                    let _e2533 = sqrt(select(_e2527, 0f, _e2531));
                                                                    let _e2534 = (_e2313 * 0.00000037450704f);
                                                                    let _e2535 = sqrt(_e2476);
                                                                    let _e2536 = (_e2469 / _e2535);
                                                                    let _e2537 = (_e2470 / _e2535);
                                                                    let _e2538 = (_e2471 / _e2535);
                                                                    let _e2540 = select(-1f, 1f, (_e2538 >= 0f));
                                                                    let _e2542 = (-1f / (_e2540 + _e2538));
                                                                    let _e2544 = ((_e2536 * _e2537) * _e2542);
                                                                    let _e2557 = (_e2533 * cos(_e2534));
                                                                    let _e2562 = (_e2533 * sin(_e2534));
                                                                    let _e2575 = -(((((1f + (((_e2540 * _e2536) * _e2536) * _e2542)) * _e2557) + (_e2544 * _e2562)) + (_e2536 * _e2525)));
                                                                    let _e2576 = -(((((_e2540 * _e2544) * _e2557) + ((_e2540 + ((_e2537 * _e2537) * _e2542)) * _e2562)) + (_e2537 * _e2525)));
                                                                    let _e2577 = -(((((-(_e2540) * _e2536) * _e2557) + (-(_e2537) * _e2562)) + (_e2538 * _e2525)));
                                                                    phi_17152_ = type_72(vec3<f32>((_e2317.x + (_e2575 * _e2317.w)), (_e2317.y + (_e2576 * _e2317.w)), (_e2317.z + (_e2577 * _e2317.w))), vec3<f32>(_e2575, _e2576, _e2577), (0.15915494f / _e2494));
                                                                }
                                                                let _e2645 = phi_17152_;
                                                                phi_17580_ = _e2645;
                                                            } else {
                                                                let _e2341 = sqrt(_e2297);
                                                                let _e2342 = (1f - _e2341);
                                                                let _e2343 = ((_e2313 * 0.000000059604645f) * _e2341);
                                                                let _e2356 = (_e2341 - _e2343);
                                                                let _e2360 = (((_e2317.x * _e2342) + (_e2330.x * _e2343)) + (_e2333.x * _e2356));
                                                                let _e2361 = (((_e2317.y * _e2342) + (_e2330.y * _e2343)) + (_e2333.y * _e2356));
                                                                let _e2362 = (((_e2317.z * _e2342) + (_e2330.z * _e2343)) + (_e2333.z * _e2356));
                                                                let _e2364 = (_e2330.x - _e2317.x);
                                                                let _e2365 = (_e2330.y - _e2317.y);
                                                                let _e2366 = (_e2330.z - _e2317.z);
                                                                let _e2367 = (_e2333.x - _e2317.x);
                                                                let _e2368 = (_e2333.y - _e2317.y);
                                                                let _e2369 = (_e2333.z - _e2317.z);
                                                                let _e2372 = ((_e2365 * _e2369) - (_e2368 * _e2366));
                                                                let _e2375 = ((_e2366 * _e2367) - (_e2369 * _e2364));
                                                                let _e2378 = ((_e2364 * _e2368) - (_e2367 * _e2365));
                                                                let _e2384 = sqrt((((_e2372 * _e2372) + (_e2375 * _e2375)) + (_e2378 * _e2378)));
                                                                let _e2385 = (1f / _e2384);
                                                                let _e2386 = (_e2372 * _e2385);
                                                                let _e2387 = (_e2375 * _e2385);
                                                                let _e2388 = (_e2378 * _e2385);
                                                                if _e2340 {
                                                                    let _e2419 = (_e2317.x - _e1106);
                                                                    let _e2420 = (_e2317.y - _e1107);
                                                                    let _e2421 = (_e2317.z - _e1108);
                                                                    let _e2426 = (((_e2419 * _e2419) + (_e2420 * _e2420)) + (_e2421 * _e2421));
                                                                    let _e2427 = (_e2317.w * _e2317.w);
                                                                    if (_e2426 <= (_e2427 * 1.000001f)) {
                                                                        let _e2441 = (_e2360 - _e1106);
                                                                        let _e2442 = (_e2361 - _e1107);
                                                                        let _e2443 = (_e2362 - _e1108);
                                                                        let _e2448 = (((_e2441 * _e2441) + (_e2442 * _e2442)) + (_e2443 * _e2443));
                                                                        phi_27632_ = (_e2448 / (((abs((((_e2386 * _e2441) + (_e2387 * _e2442)) + (_e2388 * _e2443))) / sqrt(_e2448)) * 12.566371f) * _e2427));
                                                                    } else {
                                                                        let _e2430 = (_e2427 / _e2426);
                                                                        if (_e2430 != _e2430) {
                                                                            phi_27651_ = true;
                                                                        } else {
                                                                            phi_27651_ = (1f <= _e2430);
                                                                        }
                                                                        let _e2434 = phi_27651_;
                                                                        let _e2435 = select(_e2430, 1f, _e2434);
                                                                        phi_27632_ = (0.15915494f / (_e2435 / (1f + sqrt((1f - _e2435)))));
                                                                    }
                                                                    let _e2461 = phi_27632_;
                                                                    phi_17578_ = _e2461;
                                                                } else {
                                                                    let _e2390 = (_e2360 - _e1106);
                                                                    let _e2391 = (_e2361 - _e1107);
                                                                    let _e2392 = (_e2362 - _e1108);
                                                                    let _e2397 = (((_e2390 * _e2390) + (_e2391 * _e2391)) + (_e2392 * _e2392));
                                                                    let _e2405 = (abs((((_e2386 * _e2390) + (_e2387 * _e2391)) + (_e2388 * _e2392))) / sqrt(_e2397));
                                                                    if (_e2405 <= 0f) {
                                                                        phi_17577_ = 0f;
                                                                    } else {
                                                                        if (_e2326 == 0u) {
                                                                            phi_17574_ = (_e2384 * 0.5f);
                                                                        } else {
                                                                            phi_17574_ = ((12.566371f * _e2317.w) * _e2317.w);
                                                                        }
                                                                        let _e2413 = phi_17574_;
                                                                        phi_17577_ = (_e2397 / (_e2405 * _e2413));
                                                                    }
                                                                    let _e2417 = phi_17577_;
                                                                    phi_17578_ = _e2417;
                                                                }
                                                                let _e2463 = phi_17578_;
                                                                phi_17580_ = type_72(vec3<f32>(_e2360, _e2361, _e2362), vec3<f32>(_e2386, _e2387, _e2388), _e2463);
                                                            }
                                                            let _e2647 = phi_17580_;
                                                            let _e2652 = (_e2647.member.x - _e1106);
                                                            let _e2654 = (_e2647.member.y - _e1107);
                                                            let _e2656 = (_e2647.member.z - _e1108);
                                                            let _e2662 = sqrt((((_e2652 * _e2652) + (_e2654 * _e2654)) + (_e2656 * _e2656)));
                                                            let _e2663 = (_e2652 / _e2662);
                                                            let _e2664 = (_e2654 / _e2662);
                                                            let _e2665 = (_e2656 / _e2662);
                                                            if (_e2647.member_2 > 0f) {
                                                                let _e2681 = global_4.member[select(select(u32(_e2330.w), 0u, (_e2330.w < 0f)), 4294967295u, (_e2330.w > 4294967000f))];
                                                                if (_e2681.member == 4u) {
                                                                    if (_e2681.member_1 == 1u) {
                                                                        phi_17756_ = true;
                                                                    } else {
                                                                        phi_17756_ = ((((-(_e2663) * _e2647.member_1.x) + (-(_e2664) * _e2647.member_1.y)) + (-(_e2665) * _e2647.member_1.z)) > 0f);
                                                                    }
                                                                    let _e2696 = phi_17756_;
                                                                    if _e2696 {
                                                                        phi_17765_ = vec3<f32>(_e2681.member_4.x, _e2681.member_4.y, _e2681.member_4.z);
                                                                    } else {
                                                                        phi_17765_ = vec3<f32>();
                                                                    }
                                                                    let _e2703 = phi_17765_;
                                                                    phi_17767_ = _e2703;
                                                                    phi_17768_ = select(true, false, _e2696);
                                                                } else {
                                                                    phi_17767_ = vec3<f32>();
                                                                    phi_17768_ = true;
                                                                }
                                                                let _e2706 = phi_17767_;
                                                                let _e2708 = phi_17768_;
                                                                phi_17773_ = ((_e2268 - _e2270) * _e2647.member_2);
                                                                phi_17774_ = select(_e2706, vec3<f32>(0f, 0f, 0f), vec3(_e2708));
                                                            } else {
                                                                phi_17773_ = 0f;
                                                                phi_17774_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2712 = phi_17773_;
                                                            let _e2714 = phi_17774_;
                                                            phi_36525_ = _e2310;
                                                            phi_17775_ = _e2712;
                                                            phi_17776_ = vec3<f32>(_e2663, _e2664, _e2665);
                                                            phi_17777_ = _e2714;
                                                            phi_17778_ = ((_e2662 * 0.9999f) - _e1099);
                                                        } else {
                                                            phi_36525_ = _e1989;
                                                            phi_17775_ = 0f;
                                                            phi_17776_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17777_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17778_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e2716 = phi_36525_;
                                                        let _e2718 = phi_17775_;
                                                        let _e2720 = phi_17776_;
                                                        let _e2722 = phi_17777_;
                                                        let _e2724 = phi_17778_;
                                                        phi_36524_ = _e2716;
                                                        phi_17779_ = _e2718;
                                                        phi_17780_ = _e2720;
                                                        phi_17781_ = _e2722;
                                                        phi_17782_ = _e2724;
                                                    }
                                                    let _e2726 = phi_36524_;
                                                    let _e2728 = phi_17779_;
                                                    let _e2730 = phi_17780_;
                                                    let _e2732 = phi_17781_;
                                                    let _e2734 = phi_17782_;
                                                    phi_36516_ = _e2726;
                                                    phi_18141_ = _e2728;
                                                    phi_18142_ = _e2730;
                                                    phi_18143_ = _e2732;
                                                    phi_18144_ = _e2734;
                                                } else {
                                                    if ((_e379 & 2u) == 0u) {
                                                        phi_36517_ = _e1989;
                                                        phi_18138_ = 0f;
                                                        phi_18139_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_18140_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2007 = ((_e1989 * 747796405u) + 2891336453u);
                                                        let _e2015 = (((_e2007 >> bitcast<u32>((((_e2007 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2007) * 277803737u);
                                                        let _e2018 = ((_e2015 >> bitcast<u32>(22i)) ^ _e2015);
                                                        let _e2022 = (f32((_e2018 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2024 = ((_e2018 * 747796405u) + 2891336453u);
                                                        let _e2032 = (((_e2024 >> bitcast<u32>((((_e2024 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2024) * 277803737u);
                                                        let _e2035 = ((_e2032 >> bitcast<u32>(22i)) ^ _e2032);
                                                        let _e2039 = (f32((_e2035 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2042 = global_1.member.member_3[1u];
                                                        let _e2045 = global_1.member.member_3[2u];
                                                        let _e2048 = global_1.member.member_2[3u];
                                                        phi_17803_ = 0u;
                                                        phi_17806_ = _e2045;
                                                        loop {
                                                            let _e2050 = phi_17803_;
                                                            let _e2052 = phi_17806_;
                                                            local_73 = _e2050;
                                                            local_76 = _e2050;
                                                            local_77 = _e2050;
                                                            let _e2054 = ((_e2050 + 1u) < _e2052);
                                                            if _e2054 {
                                                                let _e2056 = ((_e2050 + _e2052) / 2u);
                                                                let _e2059 = global_6.member[(_e2048 + _e2056)];
                                                                let _e2060 = (_e2059 <= _e2022);
                                                                phi_17804_ = select(_e2050, _e2056, _e2060);
                                                                phi_17807_ = select(_e2056, _e2052, _e2060);
                                                            } else {
                                                                phi_17804_ = u32();
                                                                phi_17807_ = u32();
                                                            }
                                                            let _e2064 = phi_17804_;
                                                            let _e2066 = phi_17807_;
                                                            phi_17803_ = _e2064;
                                                            phi_17806_ = _e2066;
                                                            if !(_e2054) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2070 = global_1.member.member_3[0u];
                                                        let _e2071 = (_e2042 + 1u);
                                                        let _e2073 = local_73;
                                                        let _e2075 = (_e2070 + (_e2073 * _e2071));
                                                        phi_17844_ = 0u;
                                                        phi_17847_ = _e2042;
                                                        loop {
                                                            let _e2077 = phi_17844_;
                                                            let _e2079 = phi_17847_;
                                                            local_74 = _e2077;
                                                            local_75 = _e2077;
                                                            let _e2081 = ((_e2077 + 1u) < _e2079);
                                                            if _e2081 {
                                                                let _e2083 = ((_e2077 + _e2079) / 2u);
                                                                let _e2086 = global_6.member[(_e2075 + _e2083)];
                                                                let _e2087 = (_e2086 <= _e2039);
                                                                phi_17845_ = select(_e2077, _e2083, _e2087);
                                                                phi_17848_ = select(_e2083, _e2079, _e2087);
                                                            } else {
                                                                phi_17845_ = u32();
                                                                phi_17848_ = u32();
                                                            }
                                                            let _e2091 = phi_17845_;
                                                            let _e2093 = phi_17848_;
                                                            phi_17844_ = _e2091;
                                                            phi_17847_ = _e2093;
                                                            if !(_e2081) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2096 = local_74;
                                                        let _e2099 = local_75;
                                                        let _e2100 = (_e2075 + _e2099);
                                                        let _e2102 = global_6.member[_e2100];
                                                        let _e2105 = global_6.member[(_e2100 + 1u)];
                                                        if (_e2105 > _e2102) {
                                                            let _e2109 = ((_e2039 - _e2102) / (_e2105 - _e2102));
                                                            let _e2111 = select(_e2109, 0f, (_e2109 < 0f));
                                                            phi_17924_ = select(_e2111, 0.99999f, (_e2111 > 0.99999f));
                                                        } else {
                                                            phi_17924_ = 0.5f;
                                                        }
                                                        let _e2115 = phi_17924_;
                                                        let _e2117 = f32(_e2042);
                                                        let _e2120 = local_76;
                                                        let _e2123 = local_77;
                                                        let _e2124 = (_e2048 + _e2123);
                                                        let _e2126 = global_6.member[_e2124];
                                                        let _e2129 = global_6.member[(_e2124 + 1u)];
                                                        if (_e2129 > _e2126) {
                                                            let _e2133 = ((_e2022 - _e2126) / (_e2129 - _e2126));
                                                            let _e2135 = select(_e2133, 0f, (_e2133 < 0f));
                                                            phi_17973_ = select(_e2135, 0.99999f, (_e2135 > 0.99999f));
                                                        } else {
                                                            phi_17973_ = 0.5f;
                                                        }
                                                        let _e2139 = phi_17973_;
                                                        let _e2141 = f32(_e2045);
                                                        let _e2144 = ((((f32(_e2096) + _e2115) / _e2117) - 0.5f) * 6.2831855f);
                                                        let _e2145 = (((f32(_e2120) + _e2139) / _e2141) * 3.1415927f);
                                                        let _e2146 = sin(_e2145);
                                                        let _e2148 = (_e2146 * cos(_e2144));
                                                        let _e2149 = cos(_e2145);
                                                        let _e2151 = (_e2146 * sin(_e2144));
                                                        let _e2154 = ((atan2(_e2151, _e2148) * 0.15915494f) + 0.5f);
                                                        let _e2158 = select(_e2149, -1f, (_e2149 < -1f));
                                                        let _e2161 = acos(select(_e2158, 1f, (_e2158 > 1f)));
                                                        let _e2163 = ((_e2154 - floor(_e2154)) * _e2117);
                                                        let _e2168 = select(select(u32(_e2163), 0u, (_e2163 < 0f)), 4294967295u, (_e2163 > 4294967000f));
                                                        let _e2169 = (_e2042 - 1u);
                                                        let _e2172 = ((_e2161 * 0.31830987f) * _e2141);
                                                        let _e2177 = select(select(u32(_e2172), 0u, (_e2172 < 0f)), 4294967295u, (_e2172 > 4294967000f));
                                                        let _e2178 = (_e2045 - 1u);
                                                        let _e2180 = select(_e2177, _e2178, (_e2178 < _e2177));
                                                        let _e2181 = sin(_e2161);
                                                        if (_e2181 <= 0f) {
                                                            phi_18129_ = 0f;
                                                        } else {
                                                            let _e2185 = (_e2048 + _e2180);
                                                            let _e2188 = global_6.member[(_e2185 + 1u)];
                                                            let _e2190 = global_6.member[_e2185];
                                                            let _e2193 = ((_e2070 + (_e2180 * _e2071)) + select(_e2168, _e2169, (_e2169 < _e2168)));
                                                            let _e2196 = global_6.member[(_e2193 + 1u)];
                                                            let _e2198 = global_6.member[_e2193];
                                                            phi_18129_ = ((((_e2188 - _e2190) * _e2141) * ((_e2196 - _e2198) * _e2117)) / (19.73921f * _e2181));
                                                        }
                                                        let _e2205 = phi_18129_;
                                                        let _e2206 = (_e2205 > 0f);
                                                        if _e2206 {
                                                            phi_18136_ = vec3<f32>(_e2148, _e2149, _e2151);
                                                        } else {
                                                            phi_18136_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2209 = phi_18136_;
                                                        phi_36517_ = _e2035;
                                                        phi_18138_ = select(0f, _e2205, _e2206);
                                                        phi_18139_ = _e2209;
                                                        phi_18140_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2206));
                                                    }
                                                    let _e2214 = phi_36517_;
                                                    let _e2216 = phi_18138_;
                                                    let _e2218 = phi_18139_;
                                                    let _e2220 = phi_18140_;
                                                    phi_36516_ = _e2214;
                                                    phi_18141_ = _e2216;
                                                    phi_18142_ = _e2218;
                                                    phi_18143_ = _e2220;
                                                    phi_18144_ = 1000000000000000000000000000000f;
                                                }
                                                let _e2736 = phi_36516_;
                                                let _e2738 = phi_18141_;
                                                let _e2740 = phi_18142_;
                                                let _e2742 = phi_18143_;
                                                let _e2744 = phi_18144_;
                                                phi_36515_ = _e2736;
                                                phi_18145_ = _e2738;
                                                phi_18146_ = _e2740;
                                                phi_18147_ = _e2742;
                                                phi_18148_ = _e2744;
                                            }
                                            let _e3986 = phi_36515_;
                                            let _e3988 = phi_18145_;
                                            let _e3990 = phi_18146_;
                                            let _e3992 = phi_18147_;
                                            let _e3994 = phi_18148_;
                                            phi_36514_ = _e3986;
                                            phi_18149_ = _e3988;
                                            phi_18150_ = _e3990;
                                            phi_18151_ = _e3992;
                                            phi_18152_ = _e3994;
                                        }
                                        let _e4063 = phi_36514_;
                                        let _e4065 = phi_18149_;
                                        let _e4067 = phi_18150_;
                                        let _e4069 = phi_18151_;
                                        let _e4071 = phi_18152_;
                                        if ((((_e4069.x * _e4069.x) + (_e4069.y * _e4069.y)) + (_e4069.z * _e4069.z)) > 0f) {
                                            let _e4088 = (((_e4067.x * _e1081.x) + (_e4067.y * _e1081.y)) + (_e4067.z * _e1081.z));
                                            if (_e4088 != _e4088) {
                                                phi_29216_ = true;
                                            } else {
                                                phi_29216_ = (0f >= _e4088);
                                            }
                                            let _e4092 = phi_29216_;
                                            let _e4094 = (select(_e4088, 0f, _e4092) * 0.31830987f);
                                            if (_e1974 == 0u) {
                                                if (_e4088 <= 0f) {
                                                    phi_30013_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_30013_ = (_e1968 * (_e4088 * 0.31830987f));
                                                }
                                                let _e4570 = phi_30013_;
                                                phi_18548_ = type_59(_e4570, _e4094);
                                                phi_18549_ = type_59();
                                                phi_18550_ = true;
                                            } else {
                                                let _e4100 = (((_e1055 * _e1129) + (_e1056 * _e1130)) + (_e1057 * _e1132));
                                                let _e4105 = (((_e1055 * _e1125) + (_e1056 * _e1135)) + (_e1057 * _e1136));
                                                let _e4110 = (((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z));
                                                let _e4115 = (((_e4067.x * _e1129) + (_e4067.y * _e1130)) + (_e4067.z * _e1132));
                                                let _e4120 = (((_e4067.x * _e1125) + (_e4067.y * _e1135)) + (_e4067.z * _e1136));
                                                if (_e4110 <= 0f) {
                                                    phi_18512_ = type_59();
                                                    phi_18513_ = true;
                                                } else {
                                                    let _e4122 = (_e4088 <= 0f);
                                                    if _e4122 {
                                                        phi_18511_ = type_59();
                                                    } else {
                                                        let _e4123 = (_e4100 + _e4115);
                                                        let _e4124 = (_e4105 + _e4120);
                                                        let _e4125 = (_e4110 + _e4088);
                                                        let _e4132 = (1f / sqrt((((_e4123 * _e4123) + (_e4124 * _e4124)) + (_e4125 * _e4125))));
                                                        let _e4135 = (_e4125 * _e4132);
                                                        if (_e4135 <= 0f) {
                                                            phi_29326_ = 0f;
                                                        } else {
                                                            let _e4137 = (_e1972 * _e1972);
                                                            let _e4141 = ((((_e4137 - 1f) * _e4135) * _e4135) + 1f);
                                                            phi_29326_ = (_e4137 / ((3.1415927f * _e4141) * _e4141));
                                                        }
                                                        let _e4146 = phi_29326_;
                                                        let _e4147 = (_e4110 * _e4110);
                                                        if (_e4147 >= 1f) {
                                                            phi_29354_ = 0f;
                                                        } else {
                                                            phi_29354_ = ((sqrt((1f + (((_e1972 * _e1972) * ((_e4100 * _e4100) + (_e4105 * _e4105))) / _e4147))) - 1f) * 0.5f);
                                                        }
                                                        let _e4160 = phi_29354_;
                                                        let _e4161 = (1f + _e4160);
                                                        let _e4162 = (_e4088 * _e4088);
                                                        if (_e4162 >= 1f) {
                                                            phi_29382_ = 0f;
                                                        } else {
                                                            phi_29382_ = ((sqrt((1f + (((_e1972 * _e1972) * ((_e4115 * _e4115) + (_e4120 * _e4120))) / _e4162))) - 1f) * 0.5f);
                                                        }
                                                        let _e4175 = phi_29382_;
                                                        let _e4182 = (((_e4100 * (_e4123 * _e4132)) + (_e4105 * (_e4124 * _e4132))) + (_e4110 * _e4135));
                                                        if (_e1970 == 0u) {
                                                            let _e4477 = (1f - _e4182);
                                                            if (_e4477 != _e4477) {
                                                                phi_29885_ = true;
                                                            } else {
                                                                phi_29885_ = (0f >= _e4477);
                                                            }
                                                            let _e4481 = phi_29885_;
                                                            let _e4483 = pow(select(_e4477, 0f, _e4481), 5f);
                                                            let _e4484 = (1f - _e4483);
                                                            phi_18499_ = vec3<f32>(((_e1968.x * _e4484) + _e4483), ((_e1968.y * _e4484) + _e4483), ((_e1968.z * _e4484) + _e4483));
                                                        } else {
                                                            if (_e1970 == 1u) {
                                                                let _e4450 = (1f / _e1964);
                                                                let _e4454 = ((_e4450 * _e4450) * (1f - (_e4182 * _e4182)));
                                                                if (_e4454 >= 1f) {
                                                                    phi_29846_ = 1f;
                                                                } else {
                                                                    let _e4457 = sqrt((1f - _e4454));
                                                                    if (_e4182 != _e4182) {
                                                                        phi_29857_ = true;
                                                                    } else {
                                                                        phi_29857_ = (_e4457 <= _e4182);
                                                                    }
                                                                    let _e4461 = phi_29857_;
                                                                    let _e4466 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                                    phi_29846_ = (_e4466 + ((1f - _e4466) * pow((1f - select(_e4182, _e4457, _e4461)), 5f)));
                                                                }
                                                                let _e4473 = phi_29846_;
                                                                phi_18498_ = vec3(_e4473);
                                                            } else {
                                                                let _e4186 = select(_e4182, 0f, (_e4182 < 0f));
                                                                let _e4188 = select(_e4186, 1f, (_e4186 > 1f));
                                                                let _e4190 = (1f - (_e4188 * _e4188));
                                                                let _e4203 = ((_e1968.x * _e1968.x) - (_e1966.x * _e1966.x));
                                                                let _e4204 = ((_e1968.y * _e1968.y) - (_e1966.y * _e1966.y));
                                                                let _e4205 = ((_e1968.z * _e1968.z) - (_e1966.z * _e1966.z));
                                                                let _e4207 = (_e1968.x * (_e1966.x + _e1966.x));
                                                                let _e4209 = (_e1968.y * (_e1966.y + _e1966.y));
                                                                let _e4211 = (_e1968.z * (_e1966.z + _e1966.z));
                                                                let _e4218 = ((_e4203 * _e4203) + (_e4207 * _e4207));
                                                                let _e4219 = ((_e4204 * _e4204) + (_e4209 * _e4209));
                                                                let _e4220 = ((_e4205 * _e4205) + (_e4211 * _e4211));
                                                                let _e4225 = ((_e4190 * _e4203) / _e4218);
                                                                let _e4226 = ((_e4190 * _e4204) / _e4219);
                                                                let _e4227 = ((_e4190 * _e4205) / _e4220);
                                                                let _e4228 = (1f - _e4225);
                                                                let _e4229 = (1f - _e4226);
                                                                let _e4230 = (1f - _e4227);
                                                                let _e4232 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4225, _e4226, _e4227));
                                                                let _e4233 = (_e4190 * _e4207);
                                                                let _e4234 = (_e4190 * _e4209);
                                                                let _e4235 = (_e4190 * _e4211);
                                                                let _e4240 = (vec3<f32>(_e4233, _e4234, _e4235) / vec3<f32>(_e4218, _e4219, _e4220));
                                                                let _e4253 = ((_e4232.x * _e4232.x) + (_e4240.x * _e4240.x));
                                                                let _e4254 = ((_e4232.y * _e4232.y) + (_e4240.y * _e4240.y));
                                                                let _e4255 = ((_e4232.z * _e4232.z) + (_e4240.z * _e4240.z));
                                                                if (_e4253 != _e4253) {
                                                                    phi_29516_ = true;
                                                                } else {
                                                                    phi_29516_ = (0f >= _e4253);
                                                                }
                                                                let _e4259 = phi_29516_;
                                                                let _e4261 = sqrt(select(_e4253, 0f, _e4259));
                                                                if (_e4254 != _e4254) {
                                                                    phi_29531_ = true;
                                                                } else {
                                                                    phi_29531_ = (0f >= _e4254);
                                                                }
                                                                let _e4265 = phi_29531_;
                                                                let _e4267 = sqrt(select(_e4254, 0f, _e4265));
                                                                if (_e4255 != _e4255) {
                                                                    phi_29546_ = true;
                                                                } else {
                                                                    phi_29546_ = (0f >= _e4255);
                                                                }
                                                                let _e4271 = phi_29546_;
                                                                let _e4273 = sqrt(select(_e4255, 0f, _e4271));
                                                                let _e4277 = ((_e4261 + _e4228) * 0.5f);
                                                                let _e4278 = ((_e4267 + _e4229) * 0.5f);
                                                                let _e4279 = ((_e4273 + _e4230) * 0.5f);
                                                                if (_e4277 != _e4277) {
                                                                    phi_29573_ = true;
                                                                } else {
                                                                    phi_29573_ = (0f >= _e4277);
                                                                }
                                                                let _e4283 = phi_29573_;
                                                                let _e4285 = sqrt(select(_e4277, 0f, _e4283));
                                                                if (_e4278 != _e4278) {
                                                                    phi_29588_ = true;
                                                                } else {
                                                                    phi_29588_ = (0f >= _e4278);
                                                                }
                                                                let _e4289 = phi_29588_;
                                                                let _e4291 = sqrt(select(_e4278, 0f, _e4289));
                                                                if (_e4279 != _e4279) {
                                                                    phi_29603_ = true;
                                                                } else {
                                                                    phi_29603_ = (0f >= _e4279);
                                                                }
                                                                let _e4295 = phi_29603_;
                                                                let _e4297 = sqrt(select(_e4279, 0f, _e4295));
                                                                let _e4298 = vec3<f32>(_e4285, _e4291, _e4297);
                                                                let _e4302 = ((_e4261 - _e4228) * 0.5f);
                                                                let _e4303 = ((_e4267 - _e4229) * 0.5f);
                                                                let _e4304 = ((_e4273 - _e4230) * 0.5f);
                                                                if (_e4302 != _e4302) {
                                                                    phi_29630_ = true;
                                                                } else {
                                                                    phi_29630_ = (0f >= _e4302);
                                                                }
                                                                let _e4308 = phi_29630_;
                                                                let _e4310 = sqrt(select(_e4302, 0f, _e4308));
                                                                if (_e4303 != _e4303) {
                                                                    phi_29645_ = true;
                                                                } else {
                                                                    phi_29645_ = (0f >= _e4303);
                                                                }
                                                                let _e4314 = phi_29645_;
                                                                let _e4316 = sqrt(select(_e4303, 0f, _e4314));
                                                                if (_e4304 != _e4304) {
                                                                    phi_29660_ = true;
                                                                } else {
                                                                    phi_29660_ = (0f >= _e4304);
                                                                }
                                                                let _e4320 = phi_29660_;
                                                                let _e4322 = sqrt(select(_e4304, 0f, _e4320));
                                                                if ((_e4233 / _e4218) < 0f) {
                                                                    phi_18375_ = -(_e4310);
                                                                } else {
                                                                    phi_18375_ = _e4310;
                                                                }
                                                                let _e4326 = phi_18375_;
                                                                if ((_e4234 / _e4219) < 0f) {
                                                                    phi_18383_ = -(_e4316);
                                                                } else {
                                                                    phi_18383_ = _e4316;
                                                                }
                                                                let _e4330 = phi_18383_;
                                                                if ((_e4235 / _e4220) < 0f) {
                                                                    phi_18391_ = -(_e4322);
                                                                } else {
                                                                    phi_18391_ = _e4322;
                                                                }
                                                                let _e4334 = phi_18391_;
                                                                let _e4338 = (_e1966.x * _e4188);
                                                                let _e4339 = (_e1966.y * _e4188);
                                                                let _e4340 = (_e1966.z * _e4188);
                                                                let _e4350 = vec3<f32>(((_e1968.x * _e4285) - (_e1966.x * _e4326)), ((_e1968.y * _e4291) - (_e1966.y * _e4330)), ((_e1968.z * _e4297) - (_e1966.z * _e4334)));
                                                                let _e4357 = ((_e1968.x * _e4326) + (_e1966.x * _e4285));
                                                                let _e4358 = ((_e1968.y * _e4330) + (_e1966.y * _e4291));
                                                                let _e4359 = ((_e1968.z * _e4334) + (_e1966.z * _e4297));
                                                                let _e4361 = vec3<f32>((_e1968.x * _e4188), (_e1968.y * _e4188), (_e1968.z * _e4188));
                                                                let _e4362 = (_e4361 - _e4298);
                                                                let _e4363 = (_e4338 - _e4326);
                                                                let _e4364 = (_e4339 - _e4330);
                                                                let _e4365 = (_e4340 - _e4334);
                                                                let _e4378 = (_e4361 + _e4298);
                                                                let _e4379 = (_e4338 + _e4326);
                                                                let _e4380 = (_e4339 + _e4330);
                                                                let _e4381 = (_e4340 + _e4334);
                                                                let _e4391 = ((_e4378.x * _e4378.x) + (_e4379 * _e4379));
                                                                let _e4392 = ((_e4378.y * _e4378.y) + (_e4380 * _e4380));
                                                                let _e4393 = ((_e4378.z * _e4378.z) + (_e4381 * _e4381));
                                                                let _e4403 = vec3(_e4188);
                                                                let _e4404 = (_e4403 - _e4350);
                                                                let _e4405 = -(vec3<f32>(_e4357, _e4358, _e4359));
                                                                let _e4421 = (_e4403 + _e4350);
                                                                let _e4431 = ((_e4421.x * _e4421.x) + (_e4357 * _e4357));
                                                                let _e4432 = ((_e4421.y * _e4421.y) + (_e4358 * _e4358));
                                                                let _e4433 = ((_e4421.z * _e4421.z) + (_e4359 * _e4359));
                                                                phi_18498_ = vec3<f32>((((((_e4362.x * _e4362.x) + (_e4363 * _e4363)) / select(0.000000000000000000000000000001f, _e4391, (_e4391 > 0.000000000000000000000000000001f))) + (((_e4404.x * _e4404.x) + (_e4405.x * _e4405.x)) / select(0.000000000000000000000000000001f, _e4431, (_e4431 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4362.y * _e4362.y) + (_e4364 * _e4364)) / select(0.000000000000000000000000000001f, _e4392, (_e4392 > 0.000000000000000000000000000001f))) + (((_e4404.y * _e4404.y) + (_e4405.y * _e4405.y)) / select(0.000000000000000000000000000001f, _e4432, (_e4432 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4362.z * _e4362.z) + (_e4365 * _e4365)) / select(0.000000000000000000000000000001f, _e4393, (_e4393 > 0.000000000000000000000000000001f))) + (((_e4404.z * _e4404.z) + (_e4405.z * _e4405.z)) / select(0.000000000000000000000000000001f, _e4433, (_e4433 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e4476 = phi_18498_;
                                                            phi_18499_ = _e4476;
                                                        }
                                                        let _e4496 = phi_18499_;
                                                        let _e4498 = (4f * _e4110);
                                                        phi_18511_ = type_59((_e4496 * (((1f / (_e4161 + _e4175)) * _e4146) / _e4498)), ((_e4146 / _e4161) / _e4498));
                                                    }
                                                    let _e4505 = phi_18511_;
                                                    phi_18512_ = _e4505;
                                                    phi_18513_ = _e4122;
                                                }
                                                let _e4507 = phi_18512_;
                                                let _e4509 = phi_18513_;
                                                if _e4509 {
                                                    phi_18518_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_18518_ = _e4507;
                                                }
                                                let _e4511 = phi_18518_;
                                                let _e4514 = (_e1974 == 1u);
                                                if _e4514 {
                                                    phi_18546_ = _e4511;
                                                    phi_18547_ = type_59();
                                                } else {
                                                    if (_e4110 != _e4110) {
                                                        phi_29922_ = true;
                                                    } else {
                                                        phi_29922_ = (0f >= _e4110);
                                                    }
                                                    let _e4518 = phi_29922_;
                                                    let _e4519 = select(_e4110, 0f, _e4518);
                                                    let _e4520 = (1f / _e1964);
                                                    let _e4524 = ((_e4520 * _e4520) * (1f - (_e4519 * _e4519)));
                                                    if (_e4524 >= 1f) {
                                                        phi_29941_ = 1f;
                                                    } else {
                                                        let _e4527 = sqrt((1f - _e4524));
                                                        if (_e4519 != _e4519) {
                                                            phi_29952_ = true;
                                                        } else {
                                                            phi_29952_ = (_e4527 <= _e4519);
                                                        }
                                                        let _e4531 = phi_29952_;
                                                        let _e4536 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                        phi_29941_ = (_e4536 + ((1f - _e4536) * pow((1f - select(_e4519, _e4527, _e4531)), 5f)));
                                                    }
                                                    let _e4543 = phi_29941_;
                                                    if (_e4088 <= 0f) {
                                                        phi_29991_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_29991_ = (_e1962 * (_e4088 * 0.31830987f));
                                                    }
                                                    let _e4548 = phi_29991_;
                                                    let _e4549 = (1f - _e4543);
                                                    phi_18546_ = type_59();
                                                    phi_18547_ = type_59((_e4511.member + vec3<f32>((_e4548.x * _e4549), (_e4548.y * _e4549), (_e4548.z * _e4549))), ((_e4543 * _e4511.member_1) + (_e4549 * _e4094)));
                                                }
                                                let _e4563 = phi_18546_;
                                                let _e4565 = phi_18547_;
                                                phi_18548_ = _e4563;
                                                phi_18549_ = _e4565;
                                                phi_18550_ = _e4514;
                                            }
                                            let _e4573 = phi_18548_;
                                            let _e4575 = phi_18549_;
                                            let _e4577 = phi_18550_;
                                            if _e4577 {
                                                phi_18554_ = _e4573;
                                            } else {
                                                phi_18554_ = _e4575;
                                            }
                                            let _e4579 = phi_18554_;
                                            if (_e4065 > 0f) {
                                                if (_e4065 >= 300000000000000000000000000000000000000f) {
                                                    phi_30030_ = 1f;
                                                } else {
                                                    let _e4584 = (_e4065 * _e4065);
                                                    let _e4586 = (_e4584 + (_e4579.member_1 * _e4579.member_1));
                                                    if (_e4586 == 0f) {
                                                        phi_30027_ = 0f;
                                                    } else {
                                                        phi_30027_ = (_e4584 / _e4586);
                                                    }
                                                    let _e4590 = phi_30027_;
                                                    phi_30030_ = _e4590;
                                                }
                                                let _e4592 = phi_30030_;
                                                phi_18563_ = (_e4592 / _e4065);
                                            } else {
                                                phi_18563_ = 1f;
                                            }
                                            let _e4595 = phi_18563_;
                                            if ((((_e4579.member.x * _e4579.member.x) + (_e4579.member.y * _e4579.member.y)) + (_e4579.member.z * _e4579.member.z)) > 0f) {
                                                let _e4606 = (vec3<f32>(1f, 1f, 1f) / _e4067);
                                                local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                                phi_18596_ = 4294967295u;
                                                phi_18599_ = select(4294967295u, 0u, (_e834 > 0u));
                                                phi_18601_ = 0u;
                                                phi_18603_ = _e838;
                                                phi_18605_ = _e834;
                                                phi_18607_ = vec4<f32>(0f, 0f, 0f, 0f);
                                                phi_18609_ = _e4071;
                                                loop {
                                                    let _e4610 = phi_18596_;
                                                    let _e4612 = phi_18599_;
                                                    let _e4614 = phi_18601_;
                                                    let _e4616 = phi_18603_;
                                                    let _e4618 = phi_18605_;
                                                    let _e4620 = phi_18607_;
                                                    let _e4622 = phi_18609_;
                                                    local_80 = _e4610;
                                                    local_81 = _e4620;
                                                    local_82 = _e4618;
                                                    if (_e4618 < _e4616) {
                                                        let _e4985 = (3u * _e4618);
                                                        let _e4987 = global_2.member[_e4985];
                                                        let _e4990 = global_2.member[(_e4985 + 1u)];
                                                        let _e4996 = select(select(u32(_e4990.w), 0u, (_e4990.w < 0f)), 4294967295u, (_e4990.w > 4294967000f));
                                                        if (_e4996 == 0u) {
                                                            let _e5090 = global_2.member[(_e4985 + 2u)];
                                                            let _e5099 = ((_e4067.y * _e5090.z) - (_e5090.y * _e4067.z));
                                                            let _e5102 = ((_e4067.z * _e5090.x) - (_e5090.z * _e4067.x));
                                                            let _e5105 = ((_e4067.x * _e5090.y) - (_e5090.x * _e4067.y));
                                                            let _e5110 = (((_e4990.x * _e5099) + (_e4990.y * _e5102)) + (_e4990.z * _e5105));
                                                            if (abs(_e5110) < 0.000000000001f) {
                                                                phi_18794_ = vec4<f32>();
                                                                phi_18795_ = true;
                                                            } else {
                                                                let _e5113 = (1f / _e5110);
                                                                let _e5117 = (_e1106 - _e4987.x);
                                                                let _e5118 = (_e1107 - _e4987.y);
                                                                let _e5119 = (_e1108 - _e4987.z);
                                                                let _e5125 = ((((_e5117 * _e5099) + (_e5118 * _e5102)) + (_e5119 * _e5105)) * _e5113);
                                                                if (_e5125 < 0f) {
                                                                    phi_18784_ = vec4<f32>();
                                                                    phi_18785_ = true;
                                                                    phi_18786_ = false;
                                                                } else {
                                                                    let _e5127 = (_e5125 > 1f);
                                                                    if _e5127 {
                                                                        phi_18782_ = vec4<f32>();
                                                                        phi_18783_ = false;
                                                                    } else {
                                                                        let _e5130 = ((_e5118 * _e4990.z) - (_e4990.y * _e5119));
                                                                        let _e5133 = ((_e5119 * _e4990.x) - (_e4990.z * _e5117));
                                                                        let _e5136 = ((_e5117 * _e4990.y) - (_e4990.x * _e5118));
                                                                        let _e5142 = ((((_e4067.x * _e5130) + (_e4067.y * _e5133)) + (_e4067.z * _e5136)) * _e5113);
                                                                        if (_e5142 < 0f) {
                                                                            phi_18772_ = vec4<f32>();
                                                                            phi_18773_ = true;
                                                                            phi_18774_ = false;
                                                                        } else {
                                                                            let _e5145 = ((_e5125 + _e5142) > 1f);
                                                                            if _e5145 {
                                                                                phi_18770_ = vec4<f32>();
                                                                                phi_18771_ = false;
                                                                            } else {
                                                                                let _e5151 = ((((_e5090.x * _e5130) + (_e5090.y * _e5133)) + (_e5090.z * _e5136)) * _e5113);
                                                                                if (_e5151 <= 0f) {
                                                                                    phi_18765_ = vec4<f32>();
                                                                                    phi_18766_ = true;
                                                                                } else {
                                                                                    let _e5153 = (_e5151 >= _e4622);
                                                                                    if _e5153 {
                                                                                        phi_18764_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_18764_ = vec4<f32>(_e5151, _e5125, _e5142, 1f);
                                                                                    }
                                                                                    let _e5156 = phi_18764_;
                                                                                    phi_18765_ = _e5156;
                                                                                    phi_18766_ = _e5153;
                                                                                }
                                                                                let _e5158 = phi_18765_;
                                                                                let _e5160 = phi_18766_;
                                                                                phi_18770_ = _e5158;
                                                                                phi_18771_ = _e5160;
                                                                            }
                                                                            let _e5162 = phi_18770_;
                                                                            let _e5164 = phi_18771_;
                                                                            phi_18772_ = _e5162;
                                                                            phi_18773_ = _e5145;
                                                                            phi_18774_ = _e5164;
                                                                        }
                                                                        let _e5166 = phi_18772_;
                                                                        let _e5168 = phi_18773_;
                                                                        let _e5170 = phi_18774_;
                                                                        phi_18782_ = _e5166;
                                                                        phi_18783_ = select(_e5170, true, _e5168);
                                                                    }
                                                                    let _e5173 = phi_18782_;
                                                                    let _e5175 = phi_18783_;
                                                                    phi_18784_ = _e5173;
                                                                    phi_18785_ = _e5127;
                                                                    phi_18786_ = _e5175;
                                                                }
                                                                let _e5177 = phi_18784_;
                                                                let _e5179 = phi_18785_;
                                                                let _e5181 = phi_18786_;
                                                                phi_18794_ = _e5177;
                                                                phi_18795_ = select(_e5181, true, _e5179);
                                                            }
                                                            let _e5184 = phi_18794_;
                                                            let _e5186 = phi_18795_;
                                                            phi_18940_ = _e5184;
                                                            phi_18941_ = _e5186;
                                                        } else {
                                                            if (_e4996 == 1u) {
                                                                let _e5020 = (_e1106 - _e4987.x);
                                                                let _e5021 = (_e1107 - _e4987.y);
                                                                let _e5022 = (_e1108 - _e4987.z);
                                                                let _e5027 = (((_e5020 * _e4067.x) + (_e5021 * _e4067.y)) + (_e5022 * _e4067.z));
                                                                if (_e5027 > 0f) {
                                                                    phi_18834_ = select(true, false, ((((_e5020 * _e5020) + (_e5021 * _e5021)) + (_e5022 * _e5022)) > (_e4987.w * _e4987.w)));
                                                                } else {
                                                                    phi_18834_ = true;
                                                                }
                                                                let _e5039 = phi_18834_;
                                                                if _e5039 {
                                                                    let _e5043 = (_e5020 - (_e4067.x * _e5027));
                                                                    let _e5044 = (_e5021 - (_e4067.y * _e5027));
                                                                    let _e5045 = (_e5022 - (_e4067.z * _e5027));
                                                                    let _e5053 = ((_e4987.w * _e4987.w) - (((_e5043 * _e5043) + (_e5044 * _e5044)) + (_e5045 * _e5045)));
                                                                    if (_e5053 < 0f) {
                                                                        phi_18893_ = vec4<f32>();
                                                                        phi_18894_ = true;
                                                                    } else {
                                                                        let _e5055 = sqrt(_e5053);
                                                                        let _e5056 = -(_e5027);
                                                                        let _e5057 = (_e5056 - _e5055);
                                                                        if (_e5057 <= 0f) {
                                                                            phi_18871_ = true;
                                                                        } else {
                                                                            phi_18871_ = (_e5057 >= _e4622);
                                                                        }
                                                                        let _e5061 = phi_18871_;
                                                                        if _e5061 {
                                                                            phi_18877_ = (_e5056 + _e5055);
                                                                        } else {
                                                                            phi_18877_ = _e5057;
                                                                        }
                                                                        let _e5064 = phi_18877_;
                                                                        if (_e5064 <= 0f) {
                                                                            phi_18888_ = vec4<f32>();
                                                                            phi_18889_ = true;
                                                                        } else {
                                                                            let _e5066 = (_e5064 >= _e4622);
                                                                            if _e5066 {
                                                                                phi_18887_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18887_ = vec4<f32>(_e5064, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5069 = phi_18887_;
                                                                            phi_18888_ = _e5069;
                                                                            phi_18889_ = _e5066;
                                                                        }
                                                                        let _e5071 = phi_18888_;
                                                                        let _e5073 = phi_18889_;
                                                                        phi_18893_ = _e5071;
                                                                        phi_18894_ = _e5073;
                                                                    }
                                                                    let _e5075 = phi_18893_;
                                                                    let _e5077 = phi_18894_;
                                                                    phi_18898_ = _e5075;
                                                                    phi_18899_ = _e5077;
                                                                } else {
                                                                    phi_18898_ = vec4<f32>();
                                                                    phi_18899_ = true;
                                                                }
                                                                let _e5079 = phi_18898_;
                                                                let _e5081 = phi_18899_;
                                                                phi_18930_ = _e5079;
                                                                phi_18931_ = _e5081;
                                                            } else {
                                                                if (abs(_e4067.y) < 0.000000000001f) {
                                                                    phi_18928_ = vec4<f32>();
                                                                    phi_18929_ = true;
                                                                } else {
                                                                    let _e5003 = ((_e4987.x - _e1107) / _e4067.y);
                                                                    if (_e5003 <= 0f) {
                                                                        phi_18923_ = vec4<f32>();
                                                                        phi_18924_ = true;
                                                                    } else {
                                                                        let _e5005 = (_e5003 >= _e4622);
                                                                        if _e5005 {
                                                                            phi_18922_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18922_ = vec4<f32>(_e5003, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5008 = phi_18922_;
                                                                        phi_18923_ = _e5008;
                                                                        phi_18924_ = _e5005;
                                                                    }
                                                                    let _e5010 = phi_18923_;
                                                                    let _e5012 = phi_18924_;
                                                                    phi_18928_ = _e5010;
                                                                    phi_18929_ = _e5012;
                                                                }
                                                                let _e5014 = phi_18928_;
                                                                let _e5016 = phi_18929_;
                                                                phi_18930_ = _e5014;
                                                                phi_18931_ = _e5016;
                                                            }
                                                            let _e5083 = phi_18930_;
                                                            let _e5085 = phi_18931_;
                                                            phi_18940_ = _e5083;
                                                            phi_18941_ = select(true, false, select(true, false, _e5085));
                                                        }
                                                        let _e5188 = phi_18940_;
                                                        let _e5190 = phi_18941_;
                                                        let _e5193 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5188, vec4(select(true, false, _e5190)));
                                                        let _e5195 = (_e5193.w > 0f);
                                                        if _e5195 {
                                                            phi_18962_ = _e5193.x;
                                                        } else {
                                                            phi_18962_ = _e4622;
                                                        }
                                                        let _e5198 = phi_18962_;
                                                        let _e5202 = select(true, false, _e5195);
                                                        if _e5202 {
                                                            phi_18968_ = (_e4618 + 1u);
                                                        } else {
                                                            phi_18968_ = u32();
                                                        }
                                                        let _e5205 = phi_18968_;
                                                        phi_18597_ = select(_e4610, _e4618, _e5195);
                                                        phi_18600_ = _e4612;
                                                        phi_18602_ = _e4614;
                                                        phi_18604_ = _e4616;
                                                        phi_18606_ = _e5205;
                                                        phi_18608_ = select(_e4620, _e5193, vec4(_e5195));
                                                        phi_18610_ = _e5198;
                                                        phi_19141_ = _e5193;
                                                        phi_19142_ = _e5202;
                                                        phi_19143_ = false;
                                                    } else {
                                                        if (_e4612 == 4294967295u) {
                                                            let _e4625 = (_e4614 == 0u);
                                                            if _e4625 {
                                                                phi_18998_ = u32();
                                                                phi_18999_ = u32();
                                                                phi_19001_ = false;
                                                                phi_19002_ = false;
                                                            } else {
                                                                let _e4626 = (_e4614 - 1u);
                                                                let _e4627 = (_e4626 < 64u);
                                                                if _e4627 {
                                                                } else {
                                                                    phi_36025_ = true;
                                                                    phi_21636_ = bool();
                                                                    phi_21635_ = vec4<f32>();
                                                                    break;
                                                                }
                                                                let _e4629 = local_68[_e4626];
                                                                let _e4630 = (_e4629 >= _e4622);
                                                                if _e4630 {
                                                                    phi_18996_ = u32();
                                                                } else {
                                                                    if _e4627 {
                                                                    } else {
                                                                        phi_36025_ = true;
                                                                        phi_21636_ = bool();
                                                                        phi_21635_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    let _e4632 = local_67[_e4626];
                                                                    phi_18996_ = _e4632;
                                                                }
                                                                let _e4634 = phi_18996_;
                                                                phi_18998_ = _e4626;
                                                                phi_18999_ = _e4634;
                                                                phi_19001_ = select(true, false, _e4630);
                                                                phi_19002_ = _e4630;
                                                            }
                                                            let _e4637 = phi_18998_;
                                                            let _e4639 = phi_18999_;
                                                            let _e4641 = phi_19001_;
                                                            let _e4643 = phi_19002_;
                                                            phi_19003_ = _e4637;
                                                            phi_19004_ = _e4639;
                                                            phi_19005_ = _e4637;
                                                            phi_19006_ = _e4641;
                                                            phi_19007_ = _e4643;
                                                            phi_19008_ = _e4625;
                                                        } else {
                                                            phi_19003_ = u32();
                                                            phi_19004_ = _e4612;
                                                            phi_19005_ = _e4614;
                                                            phi_19006_ = true;
                                                            phi_19007_ = false;
                                                            phi_19008_ = false;
                                                        }
                                                        let _e4645 = phi_19003_;
                                                        let _e4647 = phi_19004_;
                                                        let _e4649 = phi_19005_;
                                                        let _e4651 = phi_19006_;
                                                        let _e4653 = phi_19007_;
                                                        let _e4655 = phi_19008_;
                                                        if _e4651 {
                                                            let _e4657 = ((_e4647 & 2147483648u) == 0u);
                                                            if _e4657 {
                                                                let _e4665 = global_3.member[_e4647];
                                                                if (_e4665.member_1 == 4294967295u) {
                                                                    phi_19037_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4677 = ((_e4665.member.x - _e1106) * _e4606.x);
                                                                    let _e4679 = ((_e4665.member.y - _e1107) * _e4606.y);
                                                                    let _e4681 = ((_e4665.member.z - _e1108) * _e4606.z);
                                                                    let _e4688 = ((_e4665.member_2.x - _e1106) * _e4606.x);
                                                                    let _e4689 = ((_e4665.member_2.y - _e1107) * _e4606.y);
                                                                    let _e4690 = ((_e4665.member_2.z - _e1108) * _e4606.z);
                                                                    if (_e4677 == _e4677) {
                                                                        let _e4692 = (_e4688 == _e4688);
                                                                        if _e4692 {
                                                                            let _e4693 = (_e4677 != _e4677);
                                                                            if _e4693 {
                                                                                phi_30128_ = true;
                                                                            } else {
                                                                                phi_30128_ = (_e4688 <= _e4677);
                                                                            }
                                                                            let _e4696 = phi_30128_;
                                                                            let _e4697 = select(_e4677, _e4688, _e4696);
                                                                            if _e4693 {
                                                                                phi_30158_ = true;
                                                                            } else {
                                                                                phi_30158_ = (_e4688 >= _e4677);
                                                                            }
                                                                            let _e4702 = phi_30158_;
                                                                            let _e4703 = select(_e4677, _e4688, _e4702);
                                                                            phi_30105_ = type_56(select(-300000000000000000000000000000000000000f, _e4697, (_e4697 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4703, (_e4703 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30105_ = type_56();
                                                                        }
                                                                        let _e4708 = phi_30105_;
                                                                        phi_30108_ = _e4708;
                                                                        phi_30109_ = select(true, false, _e4692);
                                                                    } else {
                                                                        phi_30108_ = type_56();
                                                                        phi_30109_ = true;
                                                                    }
                                                                    let _e4711 = phi_30108_;
                                                                    let _e4713 = phi_30109_;
                                                                    if _e4713 {
                                                                        phi_30114_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30114_ = _e4711;
                                                                    }
                                                                    let _e4715 = phi_30114_;
                                                                    if (_e4679 == _e4679) {
                                                                        let _e4719 = (_e4689 == _e4689);
                                                                        if _e4719 {
                                                                            let _e4720 = (_e4679 != _e4679);
                                                                            if _e4720 {
                                                                                phi_30215_ = true;
                                                                            } else {
                                                                                phi_30215_ = (_e4689 <= _e4679);
                                                                            }
                                                                            let _e4723 = phi_30215_;
                                                                            let _e4724 = select(_e4679, _e4689, _e4723);
                                                                            if (_e4715.member != _e4715.member) {
                                                                                phi_30230_ = true;
                                                                            } else {
                                                                                phi_30230_ = (_e4724 >= _e4715.member);
                                                                            }
                                                                            let _e4728 = phi_30230_;
                                                                            if _e4720 {
                                                                                phi_30245_ = true;
                                                                            } else {
                                                                                phi_30245_ = (_e4689 >= _e4679);
                                                                            }
                                                                            let _e4732 = phi_30245_;
                                                                            let _e4733 = select(_e4679, _e4689, _e4732);
                                                                            if (_e4715.member_1 != _e4715.member_1) {
                                                                                phi_30260_ = true;
                                                                            } else {
                                                                                phi_30260_ = (_e4733 <= _e4715.member_1);
                                                                            }
                                                                            let _e4737 = phi_30260_;
                                                                            phi_30192_ = type_56(select(_e4715.member, _e4724, _e4728), select(_e4715.member_1, _e4733, _e4737));
                                                                        } else {
                                                                            phi_30192_ = type_56();
                                                                        }
                                                                        let _e4741 = phi_30192_;
                                                                        phi_30195_ = _e4741;
                                                                        phi_30196_ = select(true, false, _e4719);
                                                                    } else {
                                                                        phi_30195_ = type_56();
                                                                        phi_30196_ = true;
                                                                    }
                                                                    let _e4744 = phi_30195_;
                                                                    let _e4746 = phi_30196_;
                                                                    if _e4746 {
                                                                        phi_30201_ = _e4715;
                                                                    } else {
                                                                        phi_30201_ = _e4744;
                                                                    }
                                                                    let _e4748 = phi_30201_;
                                                                    if (_e4681 == _e4681) {
                                                                        let _e4752 = (_e4690 == _e4690);
                                                                        if _e4752 {
                                                                            let _e4753 = (_e4681 != _e4681);
                                                                            if _e4753 {
                                                                                phi_30302_ = true;
                                                                            } else {
                                                                                phi_30302_ = (_e4690 <= _e4681);
                                                                            }
                                                                            let _e4756 = phi_30302_;
                                                                            let _e4757 = select(_e4681, _e4690, _e4756);
                                                                            if (_e4748.member != _e4748.member) {
                                                                                phi_30317_ = true;
                                                                            } else {
                                                                                phi_30317_ = (_e4757 >= _e4748.member);
                                                                            }
                                                                            let _e4761 = phi_30317_;
                                                                            if _e4753 {
                                                                                phi_30332_ = true;
                                                                            } else {
                                                                                phi_30332_ = (_e4690 >= _e4681);
                                                                            }
                                                                            let _e4765 = phi_30332_;
                                                                            let _e4766 = select(_e4681, _e4690, _e4765);
                                                                            if (_e4748.member_1 != _e4748.member_1) {
                                                                                phi_30347_ = true;
                                                                            } else {
                                                                                phi_30347_ = (_e4766 <= _e4748.member_1);
                                                                            }
                                                                            let _e4770 = phi_30347_;
                                                                            phi_30279_ = type_56(select(_e4748.member, _e4757, _e4761), select(_e4748.member_1, _e4766, _e4770));
                                                                        } else {
                                                                            phi_30279_ = type_56();
                                                                        }
                                                                        let _e4774 = phi_30279_;
                                                                        phi_30282_ = _e4774;
                                                                        phi_30283_ = select(true, false, _e4752);
                                                                    } else {
                                                                        phi_30282_ = type_56();
                                                                        phi_30283_ = true;
                                                                    }
                                                                    let _e4777 = phi_30282_;
                                                                    let _e4779 = phi_30283_;
                                                                    if _e4779 {
                                                                        phi_30288_ = _e4748;
                                                                    } else {
                                                                        phi_30288_ = _e4777;
                                                                    }
                                                                    let _e4781 = phi_30288_;
                                                                    if (_e4781.member_1 < 0f) {
                                                                        phi_30086_ = true;
                                                                    } else {
                                                                        if (_e4781.member > _e4781.member_1) {
                                                                            phi_30083_ = true;
                                                                        } else {
                                                                            phi_30083_ = (_e4781.member > _e4622);
                                                                        }
                                                                        let _e4788 = phi_30083_;
                                                                        phi_30086_ = _e4788;
                                                                    }
                                                                    let _e4790 = phi_30086_;
                                                                    phi_19037_ = select(_e4781.member, 300000000000000000000000000000000000000f, _e4790);
                                                                }
                                                                let _e4793 = phi_19037_;
                                                                if (_e4665.member_5 == 4294967295u) {
                                                                    phi_19046_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4805 = ((_e4665.member_4.x - _e1106) * _e4606.x);
                                                                    let _e4807 = ((_e4665.member_4.y - _e1107) * _e4606.y);
                                                                    let _e4809 = ((_e4665.member_4.z - _e1108) * _e4606.z);
                                                                    let _e4816 = ((_e4665.member_6.x - _e1106) * _e4606.x);
                                                                    let _e4817 = ((_e4665.member_6.y - _e1107) * _e4606.y);
                                                                    let _e4818 = ((_e4665.member_6.z - _e1108) * _e4606.z);
                                                                    if (_e4805 == _e4805) {
                                                                        let _e4820 = (_e4816 == _e4816);
                                                                        if _e4820 {
                                                                            let _e4821 = (_e4805 != _e4805);
                                                                            if _e4821 {
                                                                                phi_30449_ = true;
                                                                            } else {
                                                                                phi_30449_ = (_e4816 <= _e4805);
                                                                            }
                                                                            let _e4824 = phi_30449_;
                                                                            let _e4825 = select(_e4805, _e4816, _e4824);
                                                                            if _e4821 {
                                                                                phi_30479_ = true;
                                                                            } else {
                                                                                phi_30479_ = (_e4816 >= _e4805);
                                                                            }
                                                                            let _e4830 = phi_30479_;
                                                                            let _e4831 = select(_e4805, _e4816, _e4830);
                                                                            phi_30426_ = type_56(select(-300000000000000000000000000000000000000f, _e4825, (_e4825 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4831, (_e4831 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30426_ = type_56();
                                                                        }
                                                                        let _e4836 = phi_30426_;
                                                                        phi_30429_ = _e4836;
                                                                        phi_30430_ = select(true, false, _e4820);
                                                                    } else {
                                                                        phi_30429_ = type_56();
                                                                        phi_30430_ = true;
                                                                    }
                                                                    let _e4839 = phi_30429_;
                                                                    let _e4841 = phi_30430_;
                                                                    if _e4841 {
                                                                        phi_30435_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30435_ = _e4839;
                                                                    }
                                                                    let _e4843 = phi_30435_;
                                                                    if (_e4807 == _e4807) {
                                                                        let _e4847 = (_e4817 == _e4817);
                                                                        if _e4847 {
                                                                            let _e4848 = (_e4807 != _e4807);
                                                                            if _e4848 {
                                                                                phi_30536_ = true;
                                                                            } else {
                                                                                phi_30536_ = (_e4817 <= _e4807);
                                                                            }
                                                                            let _e4851 = phi_30536_;
                                                                            let _e4852 = select(_e4807, _e4817, _e4851);
                                                                            if (_e4843.member != _e4843.member) {
                                                                                phi_30551_ = true;
                                                                            } else {
                                                                                phi_30551_ = (_e4852 >= _e4843.member);
                                                                            }
                                                                            let _e4856 = phi_30551_;
                                                                            if _e4848 {
                                                                                phi_30566_ = true;
                                                                            } else {
                                                                                phi_30566_ = (_e4817 >= _e4807);
                                                                            }
                                                                            let _e4860 = phi_30566_;
                                                                            let _e4861 = select(_e4807, _e4817, _e4860);
                                                                            if (_e4843.member_1 != _e4843.member_1) {
                                                                                phi_30581_ = true;
                                                                            } else {
                                                                                phi_30581_ = (_e4861 <= _e4843.member_1);
                                                                            }
                                                                            let _e4865 = phi_30581_;
                                                                            phi_30513_ = type_56(select(_e4843.member, _e4852, _e4856), select(_e4843.member_1, _e4861, _e4865));
                                                                        } else {
                                                                            phi_30513_ = type_56();
                                                                        }
                                                                        let _e4869 = phi_30513_;
                                                                        phi_30516_ = _e4869;
                                                                        phi_30517_ = select(true, false, _e4847);
                                                                    } else {
                                                                        phi_30516_ = type_56();
                                                                        phi_30517_ = true;
                                                                    }
                                                                    let _e4872 = phi_30516_;
                                                                    let _e4874 = phi_30517_;
                                                                    if _e4874 {
                                                                        phi_30522_ = _e4843;
                                                                    } else {
                                                                        phi_30522_ = _e4872;
                                                                    }
                                                                    let _e4876 = phi_30522_;
                                                                    if (_e4809 == _e4809) {
                                                                        let _e4880 = (_e4818 == _e4818);
                                                                        if _e4880 {
                                                                            let _e4881 = (_e4809 != _e4809);
                                                                            if _e4881 {
                                                                                phi_30623_ = true;
                                                                            } else {
                                                                                phi_30623_ = (_e4818 <= _e4809);
                                                                            }
                                                                            let _e4884 = phi_30623_;
                                                                            let _e4885 = select(_e4809, _e4818, _e4884);
                                                                            if (_e4876.member != _e4876.member) {
                                                                                phi_30638_ = true;
                                                                            } else {
                                                                                phi_30638_ = (_e4885 >= _e4876.member);
                                                                            }
                                                                            let _e4889 = phi_30638_;
                                                                            if _e4881 {
                                                                                phi_30653_ = true;
                                                                            } else {
                                                                                phi_30653_ = (_e4818 >= _e4809);
                                                                            }
                                                                            let _e4893 = phi_30653_;
                                                                            let _e4894 = select(_e4809, _e4818, _e4893);
                                                                            if (_e4876.member_1 != _e4876.member_1) {
                                                                                phi_30668_ = true;
                                                                            } else {
                                                                                phi_30668_ = (_e4894 <= _e4876.member_1);
                                                                            }
                                                                            let _e4898 = phi_30668_;
                                                                            phi_30600_ = type_56(select(_e4876.member, _e4885, _e4889), select(_e4876.member_1, _e4894, _e4898));
                                                                        } else {
                                                                            phi_30600_ = type_56();
                                                                        }
                                                                        let _e4902 = phi_30600_;
                                                                        phi_30603_ = _e4902;
                                                                        phi_30604_ = select(true, false, _e4880);
                                                                    } else {
                                                                        phi_30603_ = type_56();
                                                                        phi_30604_ = true;
                                                                    }
                                                                    let _e4905 = phi_30603_;
                                                                    let _e4907 = phi_30604_;
                                                                    if _e4907 {
                                                                        phi_30609_ = _e4876;
                                                                    } else {
                                                                        phi_30609_ = _e4905;
                                                                    }
                                                                    let _e4909 = phi_30609_;
                                                                    if (_e4909.member_1 < 0f) {
                                                                        phi_30407_ = true;
                                                                    } else {
                                                                        if (_e4909.member > _e4909.member_1) {
                                                                            phi_30404_ = true;
                                                                        } else {
                                                                            phi_30404_ = (_e4909.member > _e4622);
                                                                        }
                                                                        let _e4916 = phi_30404_;
                                                                        phi_30407_ = _e4916;
                                                                    }
                                                                    let _e4918 = phi_30407_;
                                                                    phi_19046_ = select(_e4909.member, 300000000000000000000000000000000000000f, _e4918);
                                                                }
                                                                let _e4921 = phi_19046_;
                                                                if (_e4665.member_3 > 0u) {
                                                                    phi_19057_ = ((2147483648u | ((_e4665.member_3 - 1u) << bitcast<u32>(28u))) | _e4665.member_1);
                                                                } else {
                                                                    phi_19057_ = _e4665.member_1;
                                                                }
                                                                let _e4930 = phi_19057_;
                                                                if (_e4665.member_7 > 0u) {
                                                                    phi_19068_ = ((2147483648u | ((_e4665.member_7 - 1u) << bitcast<u32>(28u))) | _e4665.member_5);
                                                                } else {
                                                                    phi_19068_ = _e4665.member_5;
                                                                }
                                                                let _e4939 = phi_19068_;
                                                                let _e4940 = (_e4793 <= _e4921);
                                                                let _e4944 = select(_e4793, _e4921, _e4940);
                                                                if (_e4944 < 300000000000000000000000000000000000000f) {
                                                                    let _e4946 = (_e4649 < 64u);
                                                                    if _e4946 {
                                                                        if _e4946 {
                                                                        } else {
                                                                            phi_36025_ = true;
                                                                            phi_21636_ = bool();
                                                                            phi_21635_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_67[_e4649] = select(_e4930, _e4939, _e4940);
                                                                        if _e4946 {
                                                                        } else {
                                                                            phi_36025_ = true;
                                                                            phi_21636_ = bool();
                                                                            phi_21635_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_68[_e4649] = _e4944;
                                                                        phi_19107_ = (_e4649 + 1u);
                                                                    } else {
                                                                        phi_19107_ = _e4649;
                                                                    }
                                                                    let _e4951 = phi_19107_;
                                                                    phi_19108_ = _e4951;
                                                                } else {
                                                                    phi_19108_ = _e4649;
                                                                }
                                                                let _e4953 = phi_19108_;
                                                                phi_19119_ = select(4294967295u, select(_e4939, _e4930, _e4940), (select(_e4921, _e4793, _e4940) < 300000000000000000000000000000000000000f));
                                                                phi_19120_ = _e4953;
                                                                phi_19121_ = u32();
                                                                phi_19122_ = u32();
                                                            } else {
                                                                let _e4658 = (_e4647 & 268435455u);
                                                                phi_19119_ = u32();
                                                                phi_19120_ = u32();
                                                                phi_19121_ = ((_e4658 + ((_e4647 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                                phi_19122_ = _e4658;
                                                            }
                                                            let _e4957 = phi_19119_;
                                                            let _e4959 = phi_19120_;
                                                            let _e4961 = phi_19121_;
                                                            let _e4963 = phi_19122_;
                                                            phi_19124_ = _e4957;
                                                            phi_19125_ = _e4959;
                                                            phi_19127_ = _e4961;
                                                            phi_19128_ = _e4963;
                                                            phi_19129_ = select(true, false, _e4657);
                                                            phi_19130_ = _e4657;
                                                        } else {
                                                            phi_19124_ = u32();
                                                            phi_19125_ = u32();
                                                            phi_19127_ = _e4616;
                                                            phi_19128_ = _e4618;
                                                            phi_19129_ = _e4653;
                                                            phi_19130_ = false;
                                                        }
                                                        let _e4966 = phi_19124_;
                                                        let _e4968 = phi_19125_;
                                                        let _e4970 = phi_19127_;
                                                        let _e4972 = phi_19128_;
                                                        let _e4974 = phi_19129_;
                                                        let _e4976 = phi_19130_;
                                                        phi_18597_ = _e4610;
                                                        phi_18600_ = select(_e4966, 4294967295u, _e4974);
                                                        phi_18602_ = select(_e4968, select(_e4645, _e4649, _e4651), _e4974);
                                                        phi_18604_ = select(_e4616, _e4970, _e4974);
                                                        phi_18606_ = select(_e4618, _e4972, _e4974);
                                                        phi_18608_ = _e4620;
                                                        phi_18610_ = _e4622;
                                                        phi_19141_ = vec4<f32>();
                                                        phi_19142_ = select(_e4976, true, _e4974);
                                                        phi_19143_ = select(select(_e4655, false, _e4651), false, _e4974);
                                                    }
                                                    let _e5207 = phi_18597_;
                                                    let _e5209 = phi_18600_;
                                                    let _e5211 = phi_18602_;
                                                    let _e5213 = phi_18604_;
                                                    let _e5215 = phi_18606_;
                                                    let _e5217 = phi_18608_;
                                                    let _e5219 = phi_18610_;
                                                    let _e5221 = phi_19141_;
                                                    let _e5223 = phi_19142_;
                                                    let _e5225 = phi_19143_;
                                                    phi_18596_ = _e5207;
                                                    phi_18599_ = _e5209;
                                                    phi_18601_ = _e5211;
                                                    phi_18603_ = _e5213;
                                                    phi_18605_ = _e5215;
                                                    phi_18607_ = _e5217;
                                                    phi_18609_ = _e5219;
                                                    phi_36025_ = _e1987;
                                                    phi_21636_ = _e5225;
                                                    phi_21635_ = _e5221;
                                                    if !(_e5223) {
                                                        break;
                                                    }
                                                }
                                                let _e5228 = phi_36025_;
                                                let _e5230 = phi_21636_;
                                                let _e5232 = phi_21635_;
                                                phi_36472_ = _e4063;
                                                phi_36464_ = _e5228;
                                                if _e5228 {
                                                    break;
                                                }
                                                if _e5230 {
                                                    let _e5234 = local_80;
                                                    let _e5236 = local_81;
                                                    phi_19148_ = type_74(_e5234, _e5236);
                                                } else {
                                                    phi_19148_ = type_74();
                                                }
                                                let _e5239 = phi_19148_;
                                                if select(true, false, _e5230) {
                                                    let _e5242 = local_82;
                                                    phi_19154_ = type_74(_e5242, _e5232);
                                                } else {
                                                    phi_19154_ = _e5239;
                                                }
                                                let _e5245 = phi_19154_;
                                                if (_e5245.member != 4294967295u) {
                                                    phi_19208_ = _e1991;
                                                    phi_19209_ = _e1993;
                                                } else {
                                                    if (_e1995 == (_e1985 + 2u)) {
                                                        phi_19206_ = _e1991;
                                                        phi_19207_ = type_41(vec4<f32>(_e1106, _e1107, _e1108, -1f), vec4<f32>(_e4067.x, _e4067.y, _e4067.z, 0f), vec4<f32>(((_e823.x * _e4579.member.x) * _e4595), ((_e823.y * _e4579.member.y) * _e4595), ((_e823.z * _e4579.member.z) * _e4595), 0f));
                                                    } else {
                                                        phi_19206_ = vec3<f32>((_e1991.x + (((_e823.x * _e4579.member.x) * _e4069.x) * _e4595)), (_e1991.y + (((_e823.y * _e4579.member.y) * _e4069.y) * _e4595)), (_e1991.z + (((_e823.z * _e4579.member.z) * _e4069.z) * _e4595)));
                                                        phi_19207_ = _e1993;
                                                    }
                                                    let _e5283 = phi_19206_;
                                                    let _e5285 = phi_19207_;
                                                    phi_19208_ = _e5283;
                                                    phi_19209_ = _e5285;
                                                }
                                                let _e5287 = phi_19208_;
                                                let _e5289 = phi_19209_;
                                                phi_36467_ = _e5228;
                                                phi_19210_ = _e5287;
                                                phi_19211_ = _e5289;
                                                phi_19212_ = (_e1997 + 1u);
                                            } else {
                                                phi_36467_ = _e1987;
                                                phi_19210_ = _e1991;
                                                phi_19211_ = _e1993;
                                                phi_19212_ = _e1997;
                                            }
                                            let _e5291 = phi_36467_;
                                            let _e5293 = phi_19210_;
                                            let _e5295 = phi_19211_;
                                            let _e5297 = phi_19212_;
                                            phi_36466_ = _e5291;
                                            phi_19213_ = _e5293;
                                            phi_19214_ = _e5295;
                                            phi_19215_ = _e5297;
                                        } else {
                                            phi_36466_ = _e1987;
                                            phi_19213_ = _e1991;
                                            phi_19214_ = _e1993;
                                            phi_19215_ = _e1997;
                                        }
                                        let _e5299 = phi_36466_;
                                        let _e5301 = phi_19213_;
                                        let _e5303 = phi_19214_;
                                        let _e5305 = phi_19215_;
                                        phi_36752_ = _e4063;
                                        phi_36465_ = _e5299;
                                        phi_14633_ = _e5301;
                                        phi_14636_ = _e5303;
                                        phi_14638_ = (_e1995 + 1u);
                                        phi_14640_ = _e5305;
                                    } else {
                                        phi_36752_ = _e1989;
                                        phi_36465_ = _e1987;
                                        phi_14633_ = vec3<f32>();
                                        phi_14636_ = type_41();
                                        phi_14638_ = u32();
                                        phi_14640_ = u32();
                                    }
                                    let _e5308 = phi_36752_;
                                    let _e5310 = phi_36465_;
                                    let _e5312 = phi_14633_;
                                    let _e5314 = phi_14636_;
                                    let _e5316 = phi_14638_;
                                    let _e5318 = phi_14640_;
                                    phi_36097_ = _e5310;
                                    phi_34172_ = _e5308;
                                    phi_14632_ = _e5312;
                                    phi_14635_ = _e5314;
                                    phi_14637_ = _e5316;
                                    phi_14639_ = _e5318;
                                    phi_36472_ = _e5308;
                                    phi_36464_ = _e5310;
                                    if !(_e1999) {
                                        break;
                                    }
                                }
                                let _e5321 = phi_36472_;
                                let _e5323 = phi_36464_;
                                if _e5323 {
                                    break;
                                }
                                if (_e357 >= _e376) {
                                    let _e6327 = local_98;
                                    let _e6330 = local_99;
                                    let _e6333 = local_100;
                                    let _e6336 = local_101;
                                    let _e6342 = local_102;
                                    phi_19766_ = type_69(type_34(_e332.member, _e332.member_1, vec4<f32>(_e823.x, _e823.y, _e823.z, f32(_e357)), vec4<f32>(_e6330.x, _e6333.y, _e6336.z, f32(_e6327)), vec4<u32>(_e5321, (_e1982 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, _e6342);
                                    phi_19767_ = type_69();
                                    phi_19768_ = true;
                                } else {
                                    let _e5326 = ((_e5321 * 747796405u) + 2891336453u);
                                    let _e5334 = (((_e5326 >> bitcast<u32>((((_e5326 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5326) * 277803737u);
                                    let _e5337 = ((_e5334 >> bitcast<u32>(22i)) ^ _e5334);
                                    let _e5340 = f32((_e5337 >> bitcast<u32>(8i)));
                                    let _e5343 = ((_e5337 * 747796405u) + 2891336453u);
                                    let _e5351 = (((_e5343 >> bitcast<u32>((((_e5343 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5343) * 277803737u);
                                    let _e5354 = ((_e5351 >> bitcast<u32>(22i)) ^ _e5351);
                                    let _e5357 = f32((_e5354 >> bitcast<u32>(8i)));
                                    let _e5359 = ((_e5354 * 747796405u) + 2891336453u);
                                    let _e5367 = (((_e5359 >> bitcast<u32>((((_e5359 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5359) * 277803737u);
                                    let _e5370 = ((_e5367 >> bitcast<u32>(22i)) ^ _e5367);
                                    let _e5375 = (_e1974 == 0u);
                                    if _e5375 {
                                        phi_30778_ = vec3<f32>();
                                        phi_30779_ = true;
                                    } else {
                                        if (_e1974 == 2u) {
                                            let _e5381 = (((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z));
                                            if (_e5381 != _e5381) {
                                                phi_30819_ = true;
                                            } else {
                                                phi_30819_ = (0f >= _e5381);
                                            }
                                            let _e5385 = phi_30819_;
                                            let _e5386 = select(_e5381, 0f, _e5385);
                                            let _e5387 = (1f / _e1964);
                                            let _e5391 = ((_e5387 * _e5387) * (1f - (_e5386 * _e5386)));
                                            if (_e5391 >= 1f) {
                                                phi_30838_ = 1f;
                                            } else {
                                                let _e5394 = sqrt((1f - _e5391));
                                                if (_e5386 != _e5386) {
                                                    phi_30849_ = true;
                                                } else {
                                                    phi_30849_ = (_e5394 <= _e5386);
                                                }
                                                let _e5398 = phi_30849_;
                                                let _e5403 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                phi_30838_ = (_e5403 + ((1f - _e5403) * pow((1f - select(_e5386, _e5394, _e5398)), 5f)));
                                            }
                                            let _e5410 = phi_30838_;
                                            phi_30769_ = select(true, false, ((f32((_e5370 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5410));
                                        } else {
                                            phi_30769_ = true;
                                        }
                                        let _e5414 = phi_30769_;
                                        if _e5414 {
                                            let _e5419 = (((_e1055 * _e1129) + (_e1056 * _e1130)) + (_e1057 * _e1132));
                                            let _e5424 = (((_e1055 * _e1125) + (_e1056 * _e1135)) + (_e1057 * _e1136));
                                            let _e5429 = (((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z));
                                            if (_e5429 <= 0f) {
                                                phi_31003_ = vec3<f32>();
                                                phi_31004_ = true;
                                            } else {
                                                let _e5432 = (_e1972 * _e5419);
                                                let _e5433 = (_e1972 * _e5424);
                                                let _e5440 = (1f / sqrt((((_e5432 * _e5432) + (_e5433 * _e5433)) + (_e5429 * _e5429))));
                                                let _e5441 = (_e5432 * _e5440);
                                                let _e5442 = (_e5433 * _e5440);
                                                let _e5443 = (_e5429 * _e5440);
                                                let _e5446 = ((_e5441 * _e5441) + (_e5442 * _e5442));
                                                if (_e5446 > 0f) {
                                                    let _e5449 = sqrt(_e5446);
                                                    phi_30905_ = vec3<f32>((-(_e5442) / _e5449), (_e5441 / _e5449), 0f);
                                                } else {
                                                    phi_30905_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e5454 = phi_30905_;
                                                let _e5467 = sqrt((_e5340 * 0.000000059604645f));
                                                let _e5468 = (_e5357 * 0.00000037450704f);
                                                let _e5470 = (_e5467 * cos(_e5468));
                                                let _e5472 = (0.5f * (1f + _e5443));
                                                let _e5475 = (1f - (_e5470 * _e5470));
                                                if (_e5475 != _e5475) {
                                                    phi_31061_ = true;
                                                } else {
                                                    phi_31061_ = (0f >= _e5475);
                                                }
                                                let _e5479 = phi_31061_;
                                                let _e5486 = (((1f - _e5472) * sqrt(select(_e5475, 0f, _e5479))) + ((_e5472 * _e5467) * sin(_e5468)));
                                                let _e5497 = (_e5475 - (_e5486 * _e5486));
                                                if (_e5497 != _e5497) {
                                                    phi_31076_ = true;
                                                } else {
                                                    phi_31076_ = (0f >= _e5497);
                                                }
                                                let _e5501 = phi_31076_;
                                                let _e5503 = sqrt(select(_e5497, 0f, _e5501));
                                                let _e5509 = (((_e5454.z * _e5470) + (((_e5441 * _e5454.y) - (_e5454.x * _e5442)) * _e5486)) + (_e5443 * _e5503));
                                                let _e5510 = (_e1972 * (((_e5454.x * _e5470) + (((_e5442 * _e5454.z) - (_e5454.y * _e5443)) * _e5486)) + (_e5441 * _e5503)));
                                                let _e5511 = (_e1972 * (((_e5454.y * _e5470) + (((_e5443 * _e5454.x) - (_e5454.z * _e5441)) * _e5486)) + (_e5442 * _e5503)));
                                                if (_e5509 != _e5509) {
                                                    phi_31091_ = true;
                                                } else {
                                                    phi_31091_ = (0f >= _e5509);
                                                }
                                                let _e5515 = phi_31091_;
                                                let _e5516 = select(_e5509, 0f, _e5515);
                                                let _e5523 = (1f / sqrt((((_e5510 * _e5510) + (_e5511 * _e5511)) + (_e5516 * _e5516))));
                                                let _e5524 = (_e5510 * _e5523);
                                                let _e5525 = (_e5511 * _e5523);
                                                let _e5526 = (_e5516 * _e5523);
                                                let _e5532 = (2f * (((_e5419 * _e5524) + (_e5424 * _e5525)) + (_e5429 * _e5526)));
                                                let _e5535 = (_e5526 * _e5532);
                                                let _e5538 = (vec3<f32>((_e5524 * _e5532), (_e5525 * _e5532), _e5535) - vec3<f32>(_e5419, _e5424, _e5429));
                                                let _e5539 = ((_e5535 - _e5429) <= 0f);
                                                if _e5539 {
                                                    phi_31000_ = vec3<f32>();
                                                } else {
                                                    phi_31000_ = vec3<f32>((((_e1129 * _e5538.x) + (_e1125 * _e5538.y)) + (_e1081.x * _e5538.z)), (((_e1130 * _e5538.x) + (_e1135 * _e5538.y)) + (_e1081.y * _e5538.z)), (((_e1132 * _e5538.x) + (_e1136 * _e5538.y)) + (_e1081.z * _e5538.z)));
                                                }
                                                let _e5560 = phi_31000_;
                                                phi_31003_ = _e5560;
                                                phi_31004_ = _e5539;
                                            }
                                            let _e5562 = phi_31003_;
                                            let _e5564 = phi_31004_;
                                            phi_30774_ = select(_e5562, vec3<f32>(0f, 0f, 0f), vec3(_e5564));
                                        } else {
                                            phi_30774_ = vec3<f32>();
                                        }
                                        let _e5568 = phi_30774_;
                                        phi_30778_ = _e5568;
                                        phi_30779_ = select(true, false, _e5414);
                                    }
                                    let _e5571 = phi_30778_;
                                    let _e5573 = phi_30779_;
                                    if _e5573 {
                                        let _e5574 = (_e5340 * 0.00000037450704f);
                                        let _e5576 = ((_e5357 * 0.00000011920929f) - 1f);
                                        let _e5578 = (1f - (_e5576 * _e5576));
                                        if (_e5578 != _e5578) {
                                            phi_31191_ = true;
                                        } else {
                                            phi_31191_ = (0f >= _e5578);
                                        }
                                        let _e5582 = phi_31191_;
                                        let _e5584 = sqrt(select(_e5578, 0f, _e5582));
                                        let _e5589 = (_e1081.x + (_e5584 * cos(_e5574)));
                                        let _e5590 = (_e1081.y + (_e5584 * sin(_e5574)));
                                        let _e5591 = (_e1081.z + _e5576);
                                        let _e5597 = sqrt((((_e5589 * _e5589) + (_e5590 * _e5590)) + (_e5591 * _e5591)));
                                        if (_e5597 < 0.000001f) {
                                            phi_31165_ = _e1081;
                                        } else {
                                            let _e5599 = (1f / _e5597);
                                            phi_31165_ = vec3<f32>((_e5589 * _e5599), (_e5590 * _e5599), (_e5591 * _e5599));
                                        }
                                        let _e5605 = phi_31165_;
                                        phi_30786_ = _e5605;
                                    } else {
                                        phi_30786_ = _e5571;
                                    }
                                    let _e5607 = phi_30786_;
                                    let _e5615 = (((_e5607.x * _e1081.x) + (_e5607.y * _e1081.y)) + (_e5607.z * _e1081.z));
                                    if (_e5615 != _e5615) {
                                        phi_31206_ = true;
                                    } else {
                                        phi_31206_ = (0f >= _e5615);
                                    }
                                    let _e5619 = phi_31206_;
                                    let _e5621 = (select(_e5615, 0f, _e5619) * 0.31830987f);
                                    if _e5375 {
                                        if (_e5615 <= 0f) {
                                            phi_32003_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_32003_ = (_e1968 * (_e5615 * 0.31830987f));
                                        }
                                        let _e6096 = phi_32003_;
                                        phi_19607_ = type_59(_e6096, _e5621);
                                        phi_19608_ = type_59();
                                        phi_19609_ = true;
                                    } else {
                                        let _e5626 = (((_e1055 * _e1129) + (_e1056 * _e1130)) + (_e1057 * _e1132));
                                        let _e5631 = (((_e1055 * _e1125) + (_e1056 * _e1135)) + (_e1057 * _e1136));
                                        let _e5636 = (((_e1055 * _e1081.x) + (_e1056 * _e1081.y)) + (_e1057 * _e1081.z));
                                        let _e5641 = (((_e5607.x * _e1129) + (_e5607.y * _e1130)) + (_e5607.z * _e1132));
                                        let _e5646 = (((_e5607.x * _e1125) + (_e5607.y * _e1135)) + (_e5607.z * _e1136));
                                        if (_e5636 <= 0f) {
                                            phi_19571_ = type_59();
                                            phi_19572_ = true;
                                        } else {
                                            let _e5648 = (_e5615 <= 0f);
                                            if _e5648 {
                                                phi_19570_ = type_59();
                                            } else {
                                                let _e5649 = (_e5626 + _e5641);
                                                let _e5650 = (_e5631 + _e5646);
                                                let _e5651 = (_e5636 + _e5615);
                                                let _e5658 = (1f / sqrt((((_e5649 * _e5649) + (_e5650 * _e5650)) + (_e5651 * _e5651))));
                                                let _e5661 = (_e5651 * _e5658);
                                                if (_e5661 <= 0f) {
                                                    phi_31316_ = 0f;
                                                } else {
                                                    let _e5663 = (_e1972 * _e1972);
                                                    let _e5667 = ((((_e5663 - 1f) * _e5661) * _e5661) + 1f);
                                                    phi_31316_ = (_e5663 / ((3.1415927f * _e5667) * _e5667));
                                                }
                                                let _e5672 = phi_31316_;
                                                let _e5673 = (_e5636 * _e5636);
                                                if (_e5673 >= 1f) {
                                                    phi_31344_ = 0f;
                                                } else {
                                                    phi_31344_ = ((sqrt((1f + (((_e1972 * _e1972) * ((_e5626 * _e5626) + (_e5631 * _e5631))) / _e5673))) - 1f) * 0.5f);
                                                }
                                                let _e5686 = phi_31344_;
                                                let _e5687 = (1f + _e5686);
                                                let _e5688 = (_e5615 * _e5615);
                                                if (_e5688 >= 1f) {
                                                    phi_31372_ = 0f;
                                                } else {
                                                    phi_31372_ = ((sqrt((1f + (((_e1972 * _e1972) * ((_e5641 * _e5641) + (_e5646 * _e5646))) / _e5688))) - 1f) * 0.5f);
                                                }
                                                let _e5701 = phi_31372_;
                                                let _e5708 = (((_e5626 * (_e5649 * _e5658)) + (_e5631 * (_e5650 * _e5658))) + (_e5636 * _e5661));
                                                if (_e1970 == 0u) {
                                                    let _e6003 = (1f - _e5708);
                                                    if (_e6003 != _e6003) {
                                                        phi_31875_ = true;
                                                    } else {
                                                        phi_31875_ = (0f >= _e6003);
                                                    }
                                                    let _e6007 = phi_31875_;
                                                    let _e6009 = pow(select(_e6003, 0f, _e6007), 5f);
                                                    let _e6010 = (1f - _e6009);
                                                    phi_19558_ = vec3<f32>(((_e1968.x * _e6010) + _e6009), ((_e1968.y * _e6010) + _e6009), ((_e1968.z * _e6010) + _e6009));
                                                } else {
                                                    if (_e1970 == 1u) {
                                                        let _e5976 = (1f / _e1964);
                                                        let _e5980 = ((_e5976 * _e5976) * (1f - (_e5708 * _e5708)));
                                                        if (_e5980 >= 1f) {
                                                            phi_31836_ = 1f;
                                                        } else {
                                                            let _e5983 = sqrt((1f - _e5980));
                                                            if (_e5708 != _e5708) {
                                                                phi_31847_ = true;
                                                            } else {
                                                                phi_31847_ = (_e5983 <= _e5708);
                                                            }
                                                            let _e5987 = phi_31847_;
                                                            let _e5992 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                            phi_31836_ = (_e5992 + ((1f - _e5992) * pow((1f - select(_e5708, _e5983, _e5987)), 5f)));
                                                        }
                                                        let _e5999 = phi_31836_;
                                                        phi_19557_ = vec3(_e5999);
                                                    } else {
                                                        let _e5712 = select(_e5708, 0f, (_e5708 < 0f));
                                                        let _e5714 = select(_e5712, 1f, (_e5712 > 1f));
                                                        let _e5716 = (1f - (_e5714 * _e5714));
                                                        let _e5729 = ((_e1968.x * _e1968.x) - (_e1966.x * _e1966.x));
                                                        let _e5730 = ((_e1968.y * _e1968.y) - (_e1966.y * _e1966.y));
                                                        let _e5731 = ((_e1968.z * _e1968.z) - (_e1966.z * _e1966.z));
                                                        let _e5733 = (_e1968.x * (_e1966.x + _e1966.x));
                                                        let _e5735 = (_e1968.y * (_e1966.y + _e1966.y));
                                                        let _e5737 = (_e1968.z * (_e1966.z + _e1966.z));
                                                        let _e5744 = ((_e5729 * _e5729) + (_e5733 * _e5733));
                                                        let _e5745 = ((_e5730 * _e5730) + (_e5735 * _e5735));
                                                        let _e5746 = ((_e5731 * _e5731) + (_e5737 * _e5737));
                                                        let _e5751 = ((_e5716 * _e5729) / _e5744);
                                                        let _e5752 = ((_e5716 * _e5730) / _e5745);
                                                        let _e5753 = ((_e5716 * _e5731) / _e5746);
                                                        let _e5754 = (1f - _e5751);
                                                        let _e5755 = (1f - _e5752);
                                                        let _e5756 = (1f - _e5753);
                                                        let _e5758 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5751, _e5752, _e5753));
                                                        let _e5759 = (_e5716 * _e5733);
                                                        let _e5760 = (_e5716 * _e5735);
                                                        let _e5761 = (_e5716 * _e5737);
                                                        let _e5766 = (vec3<f32>(_e5759, _e5760, _e5761) / vec3<f32>(_e5744, _e5745, _e5746));
                                                        let _e5779 = ((_e5758.x * _e5758.x) + (_e5766.x * _e5766.x));
                                                        let _e5780 = ((_e5758.y * _e5758.y) + (_e5766.y * _e5766.y));
                                                        let _e5781 = ((_e5758.z * _e5758.z) + (_e5766.z * _e5766.z));
                                                        if (_e5779 != _e5779) {
                                                            phi_31506_ = true;
                                                        } else {
                                                            phi_31506_ = (0f >= _e5779);
                                                        }
                                                        let _e5785 = phi_31506_;
                                                        let _e5787 = sqrt(select(_e5779, 0f, _e5785));
                                                        if (_e5780 != _e5780) {
                                                            phi_31521_ = true;
                                                        } else {
                                                            phi_31521_ = (0f >= _e5780);
                                                        }
                                                        let _e5791 = phi_31521_;
                                                        let _e5793 = sqrt(select(_e5780, 0f, _e5791));
                                                        if (_e5781 != _e5781) {
                                                            phi_31536_ = true;
                                                        } else {
                                                            phi_31536_ = (0f >= _e5781);
                                                        }
                                                        let _e5797 = phi_31536_;
                                                        let _e5799 = sqrt(select(_e5781, 0f, _e5797));
                                                        let _e5803 = ((_e5787 + _e5754) * 0.5f);
                                                        let _e5804 = ((_e5793 + _e5755) * 0.5f);
                                                        let _e5805 = ((_e5799 + _e5756) * 0.5f);
                                                        if (_e5803 != _e5803) {
                                                            phi_31563_ = true;
                                                        } else {
                                                            phi_31563_ = (0f >= _e5803);
                                                        }
                                                        let _e5809 = phi_31563_;
                                                        let _e5811 = sqrt(select(_e5803, 0f, _e5809));
                                                        if (_e5804 != _e5804) {
                                                            phi_31578_ = true;
                                                        } else {
                                                            phi_31578_ = (0f >= _e5804);
                                                        }
                                                        let _e5815 = phi_31578_;
                                                        let _e5817 = sqrt(select(_e5804, 0f, _e5815));
                                                        if (_e5805 != _e5805) {
                                                            phi_31593_ = true;
                                                        } else {
                                                            phi_31593_ = (0f >= _e5805);
                                                        }
                                                        let _e5821 = phi_31593_;
                                                        let _e5823 = sqrt(select(_e5805, 0f, _e5821));
                                                        let _e5824 = vec3<f32>(_e5811, _e5817, _e5823);
                                                        let _e5828 = ((_e5787 - _e5754) * 0.5f);
                                                        let _e5829 = ((_e5793 - _e5755) * 0.5f);
                                                        let _e5830 = ((_e5799 - _e5756) * 0.5f);
                                                        if (_e5828 != _e5828) {
                                                            phi_31620_ = true;
                                                        } else {
                                                            phi_31620_ = (0f >= _e5828);
                                                        }
                                                        let _e5834 = phi_31620_;
                                                        let _e5836 = sqrt(select(_e5828, 0f, _e5834));
                                                        if (_e5829 != _e5829) {
                                                            phi_31635_ = true;
                                                        } else {
                                                            phi_31635_ = (0f >= _e5829);
                                                        }
                                                        let _e5840 = phi_31635_;
                                                        let _e5842 = sqrt(select(_e5829, 0f, _e5840));
                                                        if (_e5830 != _e5830) {
                                                            phi_31650_ = true;
                                                        } else {
                                                            phi_31650_ = (0f >= _e5830);
                                                        }
                                                        let _e5846 = phi_31650_;
                                                        let _e5848 = sqrt(select(_e5830, 0f, _e5846));
                                                        if ((_e5759 / _e5744) < 0f) {
                                                            phi_19434_ = -(_e5836);
                                                        } else {
                                                            phi_19434_ = _e5836;
                                                        }
                                                        let _e5852 = phi_19434_;
                                                        if ((_e5760 / _e5745) < 0f) {
                                                            phi_19442_ = -(_e5842);
                                                        } else {
                                                            phi_19442_ = _e5842;
                                                        }
                                                        let _e5856 = phi_19442_;
                                                        if ((_e5761 / _e5746) < 0f) {
                                                            phi_19450_ = -(_e5848);
                                                        } else {
                                                            phi_19450_ = _e5848;
                                                        }
                                                        let _e5860 = phi_19450_;
                                                        let _e5864 = (_e1966.x * _e5714);
                                                        let _e5865 = (_e1966.y * _e5714);
                                                        let _e5866 = (_e1966.z * _e5714);
                                                        let _e5876 = vec3<f32>(((_e1968.x * _e5811) - (_e1966.x * _e5852)), ((_e1968.y * _e5817) - (_e1966.y * _e5856)), ((_e1968.z * _e5823) - (_e1966.z * _e5860)));
                                                        let _e5883 = ((_e1968.x * _e5852) + (_e1966.x * _e5811));
                                                        let _e5884 = ((_e1968.y * _e5856) + (_e1966.y * _e5817));
                                                        let _e5885 = ((_e1968.z * _e5860) + (_e1966.z * _e5823));
                                                        let _e5887 = vec3<f32>((_e1968.x * _e5714), (_e1968.y * _e5714), (_e1968.z * _e5714));
                                                        let _e5888 = (_e5887 - _e5824);
                                                        let _e5889 = (_e5864 - _e5852);
                                                        let _e5890 = (_e5865 - _e5856);
                                                        let _e5891 = (_e5866 - _e5860);
                                                        let _e5904 = (_e5887 + _e5824);
                                                        let _e5905 = (_e5864 + _e5852);
                                                        let _e5906 = (_e5865 + _e5856);
                                                        let _e5907 = (_e5866 + _e5860);
                                                        let _e5917 = ((_e5904.x * _e5904.x) + (_e5905 * _e5905));
                                                        let _e5918 = ((_e5904.y * _e5904.y) + (_e5906 * _e5906));
                                                        let _e5919 = ((_e5904.z * _e5904.z) + (_e5907 * _e5907));
                                                        let _e5929 = vec3(_e5714);
                                                        let _e5930 = (_e5929 - _e5876);
                                                        let _e5931 = -(vec3<f32>(_e5883, _e5884, _e5885));
                                                        let _e5947 = (_e5929 + _e5876);
                                                        let _e5957 = ((_e5947.x * _e5947.x) + (_e5883 * _e5883));
                                                        let _e5958 = ((_e5947.y * _e5947.y) + (_e5884 * _e5884));
                                                        let _e5959 = ((_e5947.z * _e5947.z) + (_e5885 * _e5885));
                                                        phi_19557_ = vec3<f32>((((((_e5888.x * _e5888.x) + (_e5889 * _e5889)) / select(0.000000000000000000000000000001f, _e5917, (_e5917 > 0.000000000000000000000000000001f))) + (((_e5930.x * _e5930.x) + (_e5931.x * _e5931.x)) / select(0.000000000000000000000000000001f, _e5957, (_e5957 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5888.y * _e5888.y) + (_e5890 * _e5890)) / select(0.000000000000000000000000000001f, _e5918, (_e5918 > 0.000000000000000000000000000001f))) + (((_e5930.y * _e5930.y) + (_e5931.y * _e5931.y)) / select(0.000000000000000000000000000001f, _e5958, (_e5958 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5888.z * _e5888.z) + (_e5891 * _e5891)) / select(0.000000000000000000000000000001f, _e5919, (_e5919 > 0.000000000000000000000000000001f))) + (((_e5930.z * _e5930.z) + (_e5931.z * _e5931.z)) / select(0.000000000000000000000000000001f, _e5959, (_e5959 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6002 = phi_19557_;
                                                    phi_19558_ = _e6002;
                                                }
                                                let _e6022 = phi_19558_;
                                                let _e6024 = (4f * _e5636);
                                                phi_19570_ = type_59((_e6022 * (((1f / (_e5687 + _e5701)) * _e5672) / _e6024)), ((_e5672 / _e5687) / _e6024));
                                            }
                                            let _e6031 = phi_19570_;
                                            phi_19571_ = _e6031;
                                            phi_19572_ = _e5648;
                                        }
                                        let _e6033 = phi_19571_;
                                        let _e6035 = phi_19572_;
                                        if _e6035 {
                                            phi_19577_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19577_ = _e6033;
                                        }
                                        let _e6037 = phi_19577_;
                                        let _e6040 = (_e1974 == 1u);
                                        if _e6040 {
                                            phi_19605_ = _e6037;
                                            phi_19606_ = type_59();
                                        } else {
                                            if (_e5636 != _e5636) {
                                                phi_31912_ = true;
                                            } else {
                                                phi_31912_ = (0f >= _e5636);
                                            }
                                            let _e6044 = phi_31912_;
                                            let _e6045 = select(_e5636, 0f, _e6044);
                                            let _e6046 = (1f / _e1964);
                                            let _e6050 = ((_e6046 * _e6046) * (1f - (_e6045 * _e6045)));
                                            if (_e6050 >= 1f) {
                                                phi_31931_ = 1f;
                                            } else {
                                                let _e6053 = sqrt((1f - _e6050));
                                                if (_e6045 != _e6045) {
                                                    phi_31942_ = true;
                                                } else {
                                                    phi_31942_ = (_e6053 <= _e6045);
                                                }
                                                let _e6057 = phi_31942_;
                                                let _e6062 = pow(((1f - _e1964) / (1f + _e1964)), 2f);
                                                phi_31931_ = (_e6062 + ((1f - _e6062) * pow((1f - select(_e6045, _e6053, _e6057)), 5f)));
                                            }
                                            let _e6069 = phi_31931_;
                                            if (_e5615 <= 0f) {
                                                phi_31981_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_31981_ = (_e1962 * (_e5615 * 0.31830987f));
                                            }
                                            let _e6074 = phi_31981_;
                                            let _e6075 = (1f - _e6069);
                                            phi_19605_ = type_59();
                                            phi_19606_ = type_59((_e6037.member + vec3<f32>((_e6074.x * _e6075), (_e6074.y * _e6075), (_e6074.z * _e6075))), ((_e6069 * _e6037.member_1) + (_e6075 * _e5621)));
                                        }
                                        let _e6089 = phi_19605_;
                                        let _e6091 = phi_19606_;
                                        phi_19607_ = _e6089;
                                        phi_19608_ = _e6091;
                                        phi_19609_ = _e6040;
                                    }
                                    let _e6099 = phi_19607_;
                                    let _e6101 = phi_19608_;
                                    let _e6103 = phi_19609_;
                                    if _e6103 {
                                        phi_19613_ = _e6099;
                                    } else {
                                        phi_19613_ = _e6101;
                                    }
                                    let _e6105 = phi_19613_;
                                    if ((((_e5607.x * _e5607.x) + (_e5607.y * _e5607.y)) + (_e5607.z * _e5607.z)) == 0f) {
                                        phi_38309_ = _e5370;
                                        phi_19752_ = type_69();
                                        phi_19753_ = type_69();
                                        phi_19754_ = true;
                                        phi_19755_ = false;
                                    } else {
                                        let _e6114 = (_e6105.member_1 <= 0f);
                                        if _e6114 {
                                            phi_38310_ = _e5370;
                                            phi_19749_ = type_69();
                                            phi_19750_ = type_69();
                                            phi_19751_ = false;
                                        } else {
                                            let _e6116 = (_e6105.member.x / _e6105.member_1);
                                            let _e6118 = (_e6105.member.y / _e6105.member_1);
                                            let _e6120 = (_e6105.member.z / _e6105.member_1);
                                            let _e6129 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6116 * _e6116) + (_e6118 * _e6118)) + (_e6120 * _e6120))) <= _e6129) {
                                                phi_38303_ = _e5370;
                                                phi_19700_ = _e823;
                                                phi_19701_ = false;
                                            } else {
                                                let _e6133 = global_1.member.member_7[1u];
                                                if (_e6133 == 1u) {
                                                    let _e6138 = global_1.member.member_4[3u];
                                                    if (f32(_e357) >= _e6138) {
                                                        let _e6141 = select(_e6118, _e6116, (_e6116 > _e6118));
                                                        let _e6143 = select(_e6120, _e6141, (_e6141 > _e6120));
                                                        if (_e6143 != _e6143) {
                                                            phi_32028_ = true;
                                                        } else {
                                                            phi_32028_ = (1f <= _e6143);
                                                        }
                                                        let _e6147 = phi_32028_;
                                                        let _e6148 = select(_e6143, 1f, _e6147);
                                                        let _e6150 = ((_e5370 * 747796405u) + 2891336453u);
                                                        let _e6158 = (((_e6150 >> bitcast<u32>((((_e6150 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6150) * 277803737u);
                                                        let _e6161 = ((_e6158 >> bitcast<u32>(22i)) ^ _e6158);
                                                        let _e6166 = ((f32((_e6161 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6148);
                                                        if _e6166 {
                                                            phi_19689_ = vec3<f32>();
                                                        } else {
                                                            phi_19689_ = vec3<f32>((_e823.x / _e6148), (_e823.y / _e6148), (_e823.z / _e6148));
                                                        }
                                                        let _e6175 = phi_19689_;
                                                        phi_38306_ = _e6161;
                                                        phi_19691_ = _e6175;
                                                        phi_19692_ = select(true, false, _e6166);
                                                    } else {
                                                        phi_38306_ = _e5370;
                                                        phi_19691_ = _e823;
                                                        phi_19692_ = true;
                                                    }
                                                    let _e6178 = phi_38306_;
                                                    let _e6180 = phi_19691_;
                                                    let _e6182 = phi_19692_;
                                                    phi_38305_ = _e6178;
                                                    phi_19693_ = _e6180;
                                                    phi_19694_ = _e6182;
                                                } else {
                                                    phi_38305_ = _e5370;
                                                    phi_19693_ = _e823;
                                                    phi_19694_ = true;
                                                }
                                                let _e6184 = phi_38305_;
                                                let _e6186 = phi_19693_;
                                                let _e6188 = phi_19694_;
                                                phi_38303_ = _e6184;
                                                phi_19700_ = select(_e823, _e6186, vec3(_e6188));
                                                phi_19701_ = select(false, true, _e6188);
                                            }
                                            let _e6193 = phi_38303_;
                                            let _e6195 = phi_19700_;
                                            let _e6197 = phi_19701_;
                                            if _e6197 {
                                                if ((((_e5607.x * _e1069.x) + (_e5607.y * _e1069.y)) + (_e5607.z * _e1069.z)) < 0f) {
                                                    phi_19730_ = -(_e1069);
                                                } else {
                                                    phi_19730_ = _e1069;
                                                }
                                                let _e6236 = phi_19730_;
                                                let _e6253 = local_88;
                                                let _e6256 = local_89;
                                                let _e6259 = local_90;
                                                let _e6262 = local_91;
                                                let _e6268 = local_92;
                                                phi_19746_ = type_69(type_34(vec4<f32>((_e829 + (_e6236.x * _e1099)), (_e830 + (_e6236.y * _e1099)), (_e831 + (_e6236.z * _e1099)), _e332.member.w), vec4<f32>(_e5607.x, _e5607.y, _e5607.z, _e6105.member_1), vec4<f32>((_e6195.x * _e6116), (_e6195.y * _e6118), (_e6195.z * _e6120), f32((_e357 + 1u))), vec4<f32>(_e6256.x, _e6259.y, _e6262.z, f32(_e6253)), vec4<u32>(_e6193, _e1982, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, _e6268);
                                                phi_19747_ = type_69();
                                            } else {
                                                let _e6205 = local_83;
                                                let _e6208 = local_84;
                                                let _e6211 = local_85;
                                                let _e6214 = local_86;
                                                let _e6220 = local_87;
                                                phi_19746_ = type_69();
                                                phi_19747_ = type_69(type_34(_e332.member, _e332.member_1, vec4<f32>(_e6195.x, _e6195.y, _e6195.z, f32(_e357)), vec4<f32>(_e6208.x, _e6211.y, _e6214.z, f32(_e6205)), vec4<u32>(_e6193, (_e1982 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, _e6220);
                                            }
                                            let _e6271 = phi_19746_;
                                            let _e6273 = phi_19747_;
                                            phi_38310_ = _e6193;
                                            phi_19749_ = _e6271;
                                            phi_19750_ = _e6273;
                                            phi_19751_ = select(true, false, _e6197);
                                        }
                                        let _e6276 = phi_38310_;
                                        let _e6278 = phi_19749_;
                                        let _e6280 = phi_19750_;
                                        let _e6282 = phi_19751_;
                                        phi_38309_ = _e6276;
                                        phi_19752_ = _e6278;
                                        phi_19753_ = _e6280;
                                        phi_19754_ = _e6114;
                                        phi_19755_ = _e6282;
                                    }
                                    let _e6284 = phi_38309_;
                                    let _e6286 = phi_19752_;
                                    let _e6288 = phi_19753_;
                                    let _e6290 = phi_19754_;
                                    let _e6292 = phi_19755_;
                                    if _e6290 {
                                        let _e6300 = local_93;
                                        let _e6303 = local_94;
                                        let _e6306 = local_95;
                                        let _e6309 = local_96;
                                        let _e6315 = local_97;
                                        phi_19761_ = type_69(type_34(_e332.member, _e332.member_1, vec4<f32>(_e823.x, _e823.y, _e823.z, f32(_e357)), vec4<f32>(_e6303.x, _e6306.y, _e6309.z, f32(_e6300)), vec4<u32>(_e6284, (_e1982 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e825, _e6315);
                                    } else {
                                        phi_19761_ = _e6288;
                                    }
                                    let _e6318 = phi_19761_;
                                    phi_19766_ = _e6318;
                                    phi_19767_ = _e6286;
                                    phi_19768_ = select(_e6292, true, _e6290);
                                }
                                let _e6345 = phi_19766_;
                                let _e6347 = phi_19767_;
                                let _e6349 = phi_19768_;
                                phi_19772_ = _e6345;
                                phi_19773_ = _e6347;
                                phi_19774_ = _e6349;
                            } else {
                                phi_19772_ = _e1978;
                                phi_19773_ = type_69();
                                phi_19774_ = true;
                            }
                            let _e6351 = phi_19772_;
                            let _e6353 = phi_19773_;
                            let _e6355 = phi_19774_;
                            phi_19778_ = _e6351;
                            phi_19779_ = _e6353;
                            phi_19780_ = _e6355;
                        }
                        let _e6388 = phi_19778_;
                        let _e6390 = phi_19779_;
                        let _e6392 = phi_19780_;
                        phi_19781_ = _e6388;
                        phi_19782_ = _e6390;
                        phi_19783_ = _e6392;
                    }
                    let _e6587 = phi_19781_;
                    let _e6589 = phi_19782_;
                    let _e6591 = phi_19783_;
                    phi_19787_ = _e6587;
                    phi_19788_ = _e6589;
                    phi_19789_ = _e6591;
                }
                let _e7103 = phi_19787_;
                let _e7105 = phi_19788_;
                let _e7107 = phi_19789_;
                if _e7107 {
                    phi_19793_ = _e7103;
                } else {
                    phi_19793_ = _e7105;
                }
                let _e7109 = phi_19793_;
                if _e329 {
                } else {
                    break;
                }
                global_7.member[_e324.member_1] = _e7109.member;
                let _e7113 = (2u * _e324.member_1);
                if (_e7113 < _e289) {
                } else {
                    break;
                }
                global_9.member[_e7113] = _e7109.member_1;
                let _e7117 = (_e7113 + 1u);
                if (_e7117 < _e289) {
                } else {
                    break;
                }
                global_9.member[_e7117] = _e7109.member_2;
                if ((_e7109.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7125 = (1u - _e293);
                    let _e7126 = (_e7125 * 4u);
                    let _e7130 = atomicAdd((&global_8.member[(_e7126 + 3u)]), 1u);
                    let _e7134 = global_1.member.member[0u];
                    let _e7139 = global_1.member.member[1u];
                    atomicStore((&global_8.member[((12u + ((_e7125 * _e7134) * _e7139)) + _e7130)]), _e324.member_1);
                    if ((_e7130 % 64u) == 0u) {
                        let _e7147 = (_e7130 / 64u);
                        if (_e7147 < 65535u) {
                            let _e7151 = atomicAdd((&global_8.member[_e7126]), 1u);
                        }
                        if ((_e7147 % 65535u) == 0u) {
                            let _e7157 = atomicAdd((&global_8.member[(_e7126 + 1u)]), 1u);
                        }
                    }
                }
                if ((_e7109.member.member_4.y & 16u) == 0u) {
                } else {
                    let _e7162 = atomicAdd((&global_8.member[11u]), 1u);
                    let _e7166 = global_1.member.member[0u];
                    let _e7171 = global_1.member.member[1u];
                    atomicStore((&global_8.member[((12u + ((2u * _e7166) * _e7171)) + _e7162)]), _e324.member_1);
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
    var phi_20853_: type_48;
    var phi_20906_: u32;
    var phi_20909_: u32;
    var phi_20911_: u32;
    var phi_20913_: u32;
    var phi_20915_: u32;
    var phi_20917_: vec4<f32>;
    var phi_20919_: f32;
    var phi_21289_: u32;
    var phi_21291_: u32;
    var phi_21292_: u32;
    var phi_21294_: bool;
    var phi_21295_: bool;
    var phi_21296_: u32;
    var phi_21297_: u32;
    var phi_21298_: u32;
    var phi_21299_: bool;
    var phi_21300_: bool;
    var phi_21301_: bool;
    var phi_32730_: bool;
    var phi_32760_: bool;
    var phi_32707_: type_56;
    var phi_32710_: type_56;
    var phi_32711_: bool;
    var phi_32716_: type_56;
    var phi_32817_: bool;
    var phi_32832_: bool;
    var phi_32847_: bool;
    var phi_32862_: bool;
    var phi_32794_: type_56;
    var phi_32797_: type_56;
    var phi_32798_: bool;
    var phi_32803_: type_56;
    var phi_32904_: bool;
    var phi_32919_: bool;
    var phi_32934_: bool;
    var phi_32949_: bool;
    var phi_32881_: type_56;
    var phi_32884_: type_56;
    var phi_32885_: bool;
    var phi_32890_: type_56;
    var phi_32685_: bool;
    var phi_32688_: bool;
    var phi_21330_: f32;
    var phi_33051_: bool;
    var phi_33081_: bool;
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
    var phi_33225_: bool;
    var phi_33240_: bool;
    var phi_33255_: bool;
    var phi_33270_: bool;
    var phi_33202_: type_56;
    var phi_33205_: type_56;
    var phi_33206_: bool;
    var phi_33211_: type_56;
    var phi_33006_: bool;
    var phi_33009_: bool;
    var phi_21339_: f32;
    var phi_21350_: u32;
    var phi_21361_: u32;
    var phi_21400_: u32;
    var phi_21401_: u32;
    var phi_21412_: u32;
    var phi_21413_: u32;
    var phi_21414_: u32;
    var phi_21415_: u32;
    var phi_21417_: u32;
    var phi_21418_: u32;
    var phi_21420_: u32;
    var phi_21421_: u32;
    var phi_21422_: bool;
    var phi_21423_: bool;
    var phi_21215_: vec4<f32>;
    var phi_21216_: vec4<f32>;
    var phi_21217_: bool;
    var phi_21221_: vec4<f32>;
    var phi_21222_: bool;
    var phi_21132_: bool;
    var phi_21166_: bool;
    var phi_21172_: f32;
    var phi_21182_: vec4<f32>;
    var phi_21183_: vec4<f32>;
    var phi_21184_: bool;
    var phi_21188_: vec4<f32>;
    var phi_21189_: bool;
    var phi_21193_: vec4<f32>;
    var phi_21194_: bool;
    var phi_21223_: vec4<f32>;
    var phi_21224_: bool;
    var phi_21065_: vec4<f32>;
    var phi_21066_: vec4<f32>;
    var phi_21067_: bool;
    var phi_21071_: vec4<f32>;
    var phi_21072_: bool;
    var phi_21073_: vec4<f32>;
    var phi_21074_: bool;
    var phi_21075_: bool;
    var phi_21083_: vec4<f32>;
    var phi_21084_: bool;
    var phi_21085_: vec4<f32>;
    var phi_21086_: bool;
    var phi_21087_: bool;
    var phi_21095_: vec4<f32>;
    var phi_21096_: bool;
    var phi_21233_: vec4<f32>;
    var phi_21234_: bool;
    var phi_21255_: f32;
    var phi_20907_: u32;
    var phi_20910_: u32;
    var phi_20912_: u32;
    var phi_20914_: u32;
    var phi_20916_: u32;
    var phi_20918_: vec4<f32>;
    var phi_20920_: f32;
    var phi_21434_: vec4<f32>;
    var phi_21435_: bool;
    var phi_21436_: bool;
    var phi_38365_: bool;
    var phi_21643_: bool;
    var phi_21642_: vec4<f32>;
    var local_105: u32;
    var local_106: vec4<f32>;
    var phi_21441_: type_74;
    var local_107: u32;
    var phi_21447_: type_74;

    switch bitcast<i32>(0u) {
        default: {
            let _e279 = global;
            let _e288 = global_1.member.member_7[2u];
            let _e292 = (_e279.x + (_e279.y * 4194240u));
            let _e297 = atomicLoad((&global_8.member[((_e288 * 4u) + 3u)]));
            if (_e292 >= _e297) {
                phi_20853_ = type_48(0u, type_48().member_1);
            } else {
                let _e302 = global_1.member.member[0u];
                let _e307 = global_1.member.member[1u];
                let _e313 = atomicLoad((&global_8.member[((12u + ((_e288 * _e302) * _e307)) + _e292)]));
                phi_20853_ = type_48(1u, _e313);
            }
            let _e319 = phi_20853_;
            if (bitcast<i32>(_e319.member) != 0i) {
                let _e324 = (_e319.member_1 < arrayLength((&global_7.member)));
                if _e324 {
                } else {
                    break;
                }
                let _e330 = global_7.member[_e319.member_1].member;
                let _e337 = global_7.member[_e319.member_1].member_1;
                let _e343 = global_1.member.member_1[0u];
                let _e346 = global_1.member.member_1[1u];
                let _e349 = (vec3<f32>(1f, 1f, 1f) / vec3<f32>(_e337.x, _e337.y, _e337.z));
                local_103 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                local_104 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                phi_20906_ = 4294967295u;
                phi_20909_ = select(4294967295u, 0u, (_e343 > 0u));
                phi_20911_ = 0u;
                phi_20913_ = (_e343 + _e346);
                phi_20915_ = _e343;
                phi_20917_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_20919_ = 300000000000000000000000000000000000000f;
                loop {
                    let _e353 = phi_20906_;
                    let _e355 = phi_20909_;
                    let _e357 = phi_20911_;
                    let _e359 = phi_20913_;
                    let _e361 = phi_20915_;
                    let _e363 = phi_20917_;
                    let _e365 = phi_20919_;
                    local_105 = _e353;
                    local_106 = _e363;
                    local_107 = _e361;
                    if (_e361 < _e359) {
                        let _e728 = (3u * _e361);
                        let _e730 = global_2.member[_e728];
                        let _e733 = global_2.member[(_e728 + 1u)];
                        let _e739 = select(select(u32(_e733.w), 0u, (_e733.w < 0f)), 4294967295u, (_e733.w > 4294967000f));
                        if (_e739 == 0u) {
                            let _e833 = global_2.member[(_e728 + 2u)];
                            let _e842 = ((_e337.y * _e833.z) - (_e833.y * _e337.z));
                            let _e845 = ((_e337.z * _e833.x) - (_e833.z * _e337.x));
                            let _e848 = ((_e337.x * _e833.y) - (_e833.x * _e337.y));
                            let _e853 = (((_e733.x * _e842) + (_e733.y * _e845)) + (_e733.z * _e848));
                            if (abs(_e853) < 0.000000000001f) {
                                phi_21095_ = vec4<f32>();
                                phi_21096_ = true;
                            } else {
                                let _e856 = (1f / _e853);
                                let _e860 = (_e330.x - _e730.x);
                                let _e861 = (_e330.y - _e730.y);
                                let _e862 = (_e330.z - _e730.z);
                                let _e868 = ((((_e860 * _e842) + (_e861 * _e845)) + (_e862 * _e848)) * _e856);
                                if (_e868 < 0f) {
                                    phi_21085_ = vec4<f32>();
                                    phi_21086_ = true;
                                    phi_21087_ = false;
                                } else {
                                    let _e870 = (_e868 > 1f);
                                    if _e870 {
                                        phi_21083_ = vec4<f32>();
                                        phi_21084_ = false;
                                    } else {
                                        let _e873 = ((_e861 * _e733.z) - (_e733.y * _e862));
                                        let _e876 = ((_e862 * _e733.x) - (_e733.z * _e860));
                                        let _e879 = ((_e860 * _e733.y) - (_e733.x * _e861));
                                        let _e885 = ((((_e337.x * _e873) + (_e337.y * _e876)) + (_e337.z * _e879)) * _e856);
                                        if (_e885 < 0f) {
                                            phi_21073_ = vec4<f32>();
                                            phi_21074_ = true;
                                            phi_21075_ = false;
                                        } else {
                                            let _e888 = ((_e868 + _e885) > 1f);
                                            if _e888 {
                                                phi_21071_ = vec4<f32>();
                                                phi_21072_ = false;
                                            } else {
                                                let _e894 = ((((_e833.x * _e873) + (_e833.y * _e876)) + (_e833.z * _e879)) * _e856);
                                                if (_e894 <= 0f) {
                                                    phi_21066_ = vec4<f32>();
                                                    phi_21067_ = true;
                                                } else {
                                                    let _e896 = (_e894 >= _e365);
                                                    if _e896 {
                                                        phi_21065_ = vec4<f32>();
                                                    } else {
                                                        phi_21065_ = vec4<f32>(_e894, _e868, _e885, 1f);
                                                    }
                                                    let _e899 = phi_21065_;
                                                    phi_21066_ = _e899;
                                                    phi_21067_ = _e896;
                                                }
                                                let _e901 = phi_21066_;
                                                let _e903 = phi_21067_;
                                                phi_21071_ = _e901;
                                                phi_21072_ = _e903;
                                            }
                                            let _e905 = phi_21071_;
                                            let _e907 = phi_21072_;
                                            phi_21073_ = _e905;
                                            phi_21074_ = _e888;
                                            phi_21075_ = _e907;
                                        }
                                        let _e909 = phi_21073_;
                                        let _e911 = phi_21074_;
                                        let _e913 = phi_21075_;
                                        phi_21083_ = _e909;
                                        phi_21084_ = select(_e913, true, _e911);
                                    }
                                    let _e916 = phi_21083_;
                                    let _e918 = phi_21084_;
                                    phi_21085_ = _e916;
                                    phi_21086_ = _e870;
                                    phi_21087_ = _e918;
                                }
                                let _e920 = phi_21085_;
                                let _e922 = phi_21086_;
                                let _e924 = phi_21087_;
                                phi_21095_ = _e920;
                                phi_21096_ = select(_e924, true, _e922);
                            }
                            let _e927 = phi_21095_;
                            let _e929 = phi_21096_;
                            phi_21233_ = _e927;
                            phi_21234_ = _e929;
                        } else {
                            if (_e739 == 1u) {
                                let _e763 = (_e330.x - _e730.x);
                                let _e764 = (_e330.y - _e730.y);
                                let _e765 = (_e330.z - _e730.z);
                                let _e770 = (((_e763 * _e337.x) + (_e764 * _e337.y)) + (_e765 * _e337.z));
                                if (_e770 > 0f) {
                                    phi_21132_ = select(true, false, ((((_e763 * _e763) + (_e764 * _e764)) + (_e765 * _e765)) > (_e730.w * _e730.w)));
                                } else {
                                    phi_21132_ = true;
                                }
                                let _e782 = phi_21132_;
                                if _e782 {
                                    let _e786 = (_e763 - (_e337.x * _e770));
                                    let _e787 = (_e764 - (_e337.y * _e770));
                                    let _e788 = (_e765 - (_e337.z * _e770));
                                    let _e796 = ((_e730.w * _e730.w) - (((_e786 * _e786) + (_e787 * _e787)) + (_e788 * _e788)));
                                    if (_e796 < 0f) {
                                        phi_21188_ = vec4<f32>();
                                        phi_21189_ = true;
                                    } else {
                                        let _e798 = sqrt(_e796);
                                        let _e799 = -(_e770);
                                        let _e800 = (_e799 - _e798);
                                        if (_e800 <= 0f) {
                                            phi_21166_ = true;
                                        } else {
                                            phi_21166_ = (_e800 >= _e365);
                                        }
                                        let _e804 = phi_21166_;
                                        if _e804 {
                                            phi_21172_ = (_e799 + _e798);
                                        } else {
                                            phi_21172_ = _e800;
                                        }
                                        let _e807 = phi_21172_;
                                        if (_e807 <= 0f) {
                                            phi_21183_ = vec4<f32>();
                                            phi_21184_ = true;
                                        } else {
                                            let _e809 = (_e807 >= _e365);
                                            if _e809 {
                                                phi_21182_ = vec4<f32>();
                                            } else {
                                                phi_21182_ = vec4<f32>(_e807, 0f, 0f, 1f);
                                            }
                                            let _e812 = phi_21182_;
                                            phi_21183_ = _e812;
                                            phi_21184_ = _e809;
                                        }
                                        let _e814 = phi_21183_;
                                        let _e816 = phi_21184_;
                                        phi_21188_ = _e814;
                                        phi_21189_ = _e816;
                                    }
                                    let _e818 = phi_21188_;
                                    let _e820 = phi_21189_;
                                    phi_21193_ = _e818;
                                    phi_21194_ = _e820;
                                } else {
                                    phi_21193_ = vec4<f32>();
                                    phi_21194_ = true;
                                }
                                let _e822 = phi_21193_;
                                let _e824 = phi_21194_;
                                phi_21223_ = _e822;
                                phi_21224_ = _e824;
                            } else {
                                if (abs(_e337.y) < 0.000000000001f) {
                                    phi_21221_ = vec4<f32>();
                                    phi_21222_ = true;
                                } else {
                                    let _e746 = ((_e730.x - _e330.y) / _e337.y);
                                    if (_e746 <= 0f) {
                                        phi_21216_ = vec4<f32>();
                                        phi_21217_ = true;
                                    } else {
                                        let _e748 = (_e746 >= _e365);
                                        if _e748 {
                                            phi_21215_ = vec4<f32>();
                                        } else {
                                            phi_21215_ = vec4<f32>(_e746, 0f, 0f, 1f);
                                        }
                                        let _e751 = phi_21215_;
                                        phi_21216_ = _e751;
                                        phi_21217_ = _e748;
                                    }
                                    let _e753 = phi_21216_;
                                    let _e755 = phi_21217_;
                                    phi_21221_ = _e753;
                                    phi_21222_ = _e755;
                                }
                                let _e757 = phi_21221_;
                                let _e759 = phi_21222_;
                                phi_21223_ = _e757;
                                phi_21224_ = _e759;
                            }
                            let _e826 = phi_21223_;
                            let _e828 = phi_21224_;
                            phi_21233_ = _e826;
                            phi_21234_ = select(true, false, select(true, false, _e828));
                        }
                        let _e931 = phi_21233_;
                        let _e933 = phi_21234_;
                        let _e936 = select(vec4<f32>(0f, 0f, 0f, 0f), _e931, vec4(select(true, false, _e933)));
                        let _e938 = (_e936.w > 0f);
                        if _e938 {
                            phi_21255_ = _e936.x;
                        } else {
                            phi_21255_ = _e365;
                        }
                        let _e941 = phi_21255_;
                        phi_20907_ = select(_e353, _e361, _e938);
                        phi_20910_ = _e355;
                        phi_20912_ = _e357;
                        phi_20914_ = _e359;
                        phi_20916_ = (_e361 + 1u);
                        phi_20918_ = select(_e363, _e936, vec4(_e938));
                        phi_20920_ = _e941;
                        phi_21434_ = _e936;
                        phi_21435_ = true;
                        phi_21436_ = false;
                    } else {
                        if (_e355 == 4294967295u) {
                            let _e368 = (_e357 == 0u);
                            if _e368 {
                                phi_21291_ = u32();
                                phi_21292_ = u32();
                                phi_21294_ = false;
                                phi_21295_ = false;
                            } else {
                                let _e369 = (_e357 - 1u);
                                let _e370 = (_e369 < 64u);
                                if _e370 {
                                } else {
                                    phi_38365_ = true;
                                    phi_21643_ = bool();
                                    phi_21642_ = vec4<f32>();
                                    break;
                                }
                                let _e372 = local_104[_e369];
                                let _e373 = (_e372 >= _e365);
                                if _e373 {
                                    phi_21289_ = u32();
                                } else {
                                    if _e370 {
                                    } else {
                                        phi_38365_ = true;
                                        phi_21643_ = bool();
                                        phi_21642_ = vec4<f32>();
                                        break;
                                    }
                                    let _e375 = local_103[_e369];
                                    phi_21289_ = _e375;
                                }
                                let _e377 = phi_21289_;
                                phi_21291_ = _e369;
                                phi_21292_ = _e377;
                                phi_21294_ = select(true, false, _e373);
                                phi_21295_ = _e373;
                            }
                            let _e380 = phi_21291_;
                            let _e382 = phi_21292_;
                            let _e384 = phi_21294_;
                            let _e386 = phi_21295_;
                            phi_21296_ = _e380;
                            phi_21297_ = _e382;
                            phi_21298_ = _e380;
                            phi_21299_ = _e384;
                            phi_21300_ = _e386;
                            phi_21301_ = _e368;
                        } else {
                            phi_21296_ = u32();
                            phi_21297_ = _e355;
                            phi_21298_ = _e357;
                            phi_21299_ = true;
                            phi_21300_ = false;
                            phi_21301_ = false;
                        }
                        let _e388 = phi_21296_;
                        let _e390 = phi_21297_;
                        let _e392 = phi_21298_;
                        let _e394 = phi_21299_;
                        let _e396 = phi_21300_;
                        let _e398 = phi_21301_;
                        if _e394 {
                            let _e400 = ((_e390 & 2147483648u) == 0u);
                            if _e400 {
                                let _e408 = global_3.member[_e390];
                                if (_e408.member_1 == 4294967295u) {
                                    phi_21330_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e420 = ((_e408.member.x - _e330.x) * _e349.x);
                                    let _e422 = ((_e408.member.y - _e330.y) * _e349.y);
                                    let _e424 = ((_e408.member.z - _e330.z) * _e349.z);
                                    let _e431 = ((_e408.member_2.x - _e330.x) * _e349.x);
                                    let _e432 = ((_e408.member_2.y - _e330.y) * _e349.y);
                                    let _e433 = ((_e408.member_2.z - _e330.z) * _e349.z);
                                    if (_e420 == _e420) {
                                        let _e435 = (_e431 == _e431);
                                        if _e435 {
                                            let _e436 = (_e420 != _e420);
                                            if _e436 {
                                                phi_32730_ = true;
                                            } else {
                                                phi_32730_ = (_e431 <= _e420);
                                            }
                                            let _e439 = phi_32730_;
                                            let _e440 = select(_e420, _e431, _e439);
                                            if _e436 {
                                                phi_32760_ = true;
                                            } else {
                                                phi_32760_ = (_e431 >= _e420);
                                            }
                                            let _e445 = phi_32760_;
                                            let _e446 = select(_e420, _e431, _e445);
                                            phi_32707_ = type_56(select(-300000000000000000000000000000000000000f, _e440, (_e440 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e446, (_e446 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_32707_ = type_56();
                                        }
                                        let _e451 = phi_32707_;
                                        phi_32710_ = _e451;
                                        phi_32711_ = select(true, false, _e435);
                                    } else {
                                        phi_32710_ = type_56();
                                        phi_32711_ = true;
                                    }
                                    let _e454 = phi_32710_;
                                    let _e456 = phi_32711_;
                                    if _e456 {
                                        phi_32716_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_32716_ = _e454;
                                    }
                                    let _e458 = phi_32716_;
                                    if (_e422 == _e422) {
                                        let _e462 = (_e432 == _e432);
                                        if _e462 {
                                            let _e463 = (_e422 != _e422);
                                            if _e463 {
                                                phi_32817_ = true;
                                            } else {
                                                phi_32817_ = (_e432 <= _e422);
                                            }
                                            let _e466 = phi_32817_;
                                            let _e467 = select(_e422, _e432, _e466);
                                            if (_e458.member != _e458.member) {
                                                phi_32832_ = true;
                                            } else {
                                                phi_32832_ = (_e467 >= _e458.member);
                                            }
                                            let _e471 = phi_32832_;
                                            if _e463 {
                                                phi_32847_ = true;
                                            } else {
                                                phi_32847_ = (_e432 >= _e422);
                                            }
                                            let _e475 = phi_32847_;
                                            let _e476 = select(_e422, _e432, _e475);
                                            if (_e458.member_1 != _e458.member_1) {
                                                phi_32862_ = true;
                                            } else {
                                                phi_32862_ = (_e476 <= _e458.member_1);
                                            }
                                            let _e480 = phi_32862_;
                                            phi_32794_ = type_56(select(_e458.member, _e467, _e471), select(_e458.member_1, _e476, _e480));
                                        } else {
                                            phi_32794_ = type_56();
                                        }
                                        let _e484 = phi_32794_;
                                        phi_32797_ = _e484;
                                        phi_32798_ = select(true, false, _e462);
                                    } else {
                                        phi_32797_ = type_56();
                                        phi_32798_ = true;
                                    }
                                    let _e487 = phi_32797_;
                                    let _e489 = phi_32798_;
                                    if _e489 {
                                        phi_32803_ = _e458;
                                    } else {
                                        phi_32803_ = _e487;
                                    }
                                    let _e491 = phi_32803_;
                                    if (_e424 == _e424) {
                                        let _e495 = (_e433 == _e433);
                                        if _e495 {
                                            let _e496 = (_e424 != _e424);
                                            if _e496 {
                                                phi_32904_ = true;
                                            } else {
                                                phi_32904_ = (_e433 <= _e424);
                                            }
                                            let _e499 = phi_32904_;
                                            let _e500 = select(_e424, _e433, _e499);
                                            if (_e491.member != _e491.member) {
                                                phi_32919_ = true;
                                            } else {
                                                phi_32919_ = (_e500 >= _e491.member);
                                            }
                                            let _e504 = phi_32919_;
                                            if _e496 {
                                                phi_32934_ = true;
                                            } else {
                                                phi_32934_ = (_e433 >= _e424);
                                            }
                                            let _e508 = phi_32934_;
                                            let _e509 = select(_e424, _e433, _e508);
                                            if (_e491.member_1 != _e491.member_1) {
                                                phi_32949_ = true;
                                            } else {
                                                phi_32949_ = (_e509 <= _e491.member_1);
                                            }
                                            let _e513 = phi_32949_;
                                            phi_32881_ = type_56(select(_e491.member, _e500, _e504), select(_e491.member_1, _e509, _e513));
                                        } else {
                                            phi_32881_ = type_56();
                                        }
                                        let _e517 = phi_32881_;
                                        phi_32884_ = _e517;
                                        phi_32885_ = select(true, false, _e495);
                                    } else {
                                        phi_32884_ = type_56();
                                        phi_32885_ = true;
                                    }
                                    let _e520 = phi_32884_;
                                    let _e522 = phi_32885_;
                                    if _e522 {
                                        phi_32890_ = _e491;
                                    } else {
                                        phi_32890_ = _e520;
                                    }
                                    let _e524 = phi_32890_;
                                    if (_e524.member_1 < 0f) {
                                        phi_32688_ = true;
                                    } else {
                                        if (_e524.member > _e524.member_1) {
                                            phi_32685_ = true;
                                        } else {
                                            phi_32685_ = (_e524.member > _e365);
                                        }
                                        let _e531 = phi_32685_;
                                        phi_32688_ = _e531;
                                    }
                                    let _e533 = phi_32688_;
                                    phi_21330_ = select(_e524.member, 300000000000000000000000000000000000000f, _e533);
                                }
                                let _e536 = phi_21330_;
                                if (_e408.member_5 == 4294967295u) {
                                    phi_21339_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e548 = ((_e408.member_4.x - _e330.x) * _e349.x);
                                    let _e550 = ((_e408.member_4.y - _e330.y) * _e349.y);
                                    let _e552 = ((_e408.member_4.z - _e330.z) * _e349.z);
                                    let _e559 = ((_e408.member_6.x - _e330.x) * _e349.x);
                                    let _e560 = ((_e408.member_6.y - _e330.y) * _e349.y);
                                    let _e561 = ((_e408.member_6.z - _e330.z) * _e349.z);
                                    if (_e548 == _e548) {
                                        let _e563 = (_e559 == _e559);
                                        if _e563 {
                                            let _e564 = (_e548 != _e548);
                                            if _e564 {
                                                phi_33051_ = true;
                                            } else {
                                                phi_33051_ = (_e559 <= _e548);
                                            }
                                            let _e567 = phi_33051_;
                                            let _e568 = select(_e548, _e559, _e567);
                                            if _e564 {
                                                phi_33081_ = true;
                                            } else {
                                                phi_33081_ = (_e559 >= _e548);
                                            }
                                            let _e573 = phi_33081_;
                                            let _e574 = select(_e548, _e559, _e573);
                                            phi_33028_ = type_56(select(-300000000000000000000000000000000000000f, _e568, (_e568 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e574, (_e574 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_33028_ = type_56();
                                        }
                                        let _e579 = phi_33028_;
                                        phi_33031_ = _e579;
                                        phi_33032_ = select(true, false, _e563);
                                    } else {
                                        phi_33031_ = type_56();
                                        phi_33032_ = true;
                                    }
                                    let _e582 = phi_33031_;
                                    let _e584 = phi_33032_;
                                    if _e584 {
                                        phi_33037_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_33037_ = _e582;
                                    }
                                    let _e586 = phi_33037_;
                                    if (_e550 == _e550) {
                                        let _e590 = (_e560 == _e560);
                                        if _e590 {
                                            let _e591 = (_e550 != _e550);
                                            if _e591 {
                                                phi_33138_ = true;
                                            } else {
                                                phi_33138_ = (_e560 <= _e550);
                                            }
                                            let _e594 = phi_33138_;
                                            let _e595 = select(_e550, _e560, _e594);
                                            if (_e586.member != _e586.member) {
                                                phi_33153_ = true;
                                            } else {
                                                phi_33153_ = (_e595 >= _e586.member);
                                            }
                                            let _e599 = phi_33153_;
                                            if _e591 {
                                                phi_33168_ = true;
                                            } else {
                                                phi_33168_ = (_e560 >= _e550);
                                            }
                                            let _e603 = phi_33168_;
                                            let _e604 = select(_e550, _e560, _e603);
                                            if (_e586.member_1 != _e586.member_1) {
                                                phi_33183_ = true;
                                            } else {
                                                phi_33183_ = (_e604 <= _e586.member_1);
                                            }
                                            let _e608 = phi_33183_;
                                            phi_33115_ = type_56(select(_e586.member, _e595, _e599), select(_e586.member_1, _e604, _e608));
                                        } else {
                                            phi_33115_ = type_56();
                                        }
                                        let _e612 = phi_33115_;
                                        phi_33118_ = _e612;
                                        phi_33119_ = select(true, false, _e590);
                                    } else {
                                        phi_33118_ = type_56();
                                        phi_33119_ = true;
                                    }
                                    let _e615 = phi_33118_;
                                    let _e617 = phi_33119_;
                                    if _e617 {
                                        phi_33124_ = _e586;
                                    } else {
                                        phi_33124_ = _e615;
                                    }
                                    let _e619 = phi_33124_;
                                    if (_e552 == _e552) {
                                        let _e623 = (_e561 == _e561);
                                        if _e623 {
                                            let _e624 = (_e552 != _e552);
                                            if _e624 {
                                                phi_33225_ = true;
                                            } else {
                                                phi_33225_ = (_e561 <= _e552);
                                            }
                                            let _e627 = phi_33225_;
                                            let _e628 = select(_e552, _e561, _e627);
                                            if (_e619.member != _e619.member) {
                                                phi_33240_ = true;
                                            } else {
                                                phi_33240_ = (_e628 >= _e619.member);
                                            }
                                            let _e632 = phi_33240_;
                                            if _e624 {
                                                phi_33255_ = true;
                                            } else {
                                                phi_33255_ = (_e561 >= _e552);
                                            }
                                            let _e636 = phi_33255_;
                                            let _e637 = select(_e552, _e561, _e636);
                                            if (_e619.member_1 != _e619.member_1) {
                                                phi_33270_ = true;
                                            } else {
                                                phi_33270_ = (_e637 <= _e619.member_1);
                                            }
                                            let _e641 = phi_33270_;
                                            phi_33202_ = type_56(select(_e619.member, _e628, _e632), select(_e619.member_1, _e637, _e641));
                                        } else {
                                            phi_33202_ = type_56();
                                        }
                                        let _e645 = phi_33202_;
                                        phi_33205_ = _e645;
                                        phi_33206_ = select(true, false, _e623);
                                    } else {
                                        phi_33205_ = type_56();
                                        phi_33206_ = true;
                                    }
                                    let _e648 = phi_33205_;
                                    let _e650 = phi_33206_;
                                    if _e650 {
                                        phi_33211_ = _e619;
                                    } else {
                                        phi_33211_ = _e648;
                                    }
                                    let _e652 = phi_33211_;
                                    if (_e652.member_1 < 0f) {
                                        phi_33009_ = true;
                                    } else {
                                        if (_e652.member > _e652.member_1) {
                                            phi_33006_ = true;
                                        } else {
                                            phi_33006_ = (_e652.member > _e365);
                                        }
                                        let _e659 = phi_33006_;
                                        phi_33009_ = _e659;
                                    }
                                    let _e661 = phi_33009_;
                                    phi_21339_ = select(_e652.member, 300000000000000000000000000000000000000f, _e661);
                                }
                                let _e664 = phi_21339_;
                                if (_e408.member_3 > 0u) {
                                    phi_21350_ = ((2147483648u | ((_e408.member_3 - 1u) << bitcast<u32>(28u))) | _e408.member_1);
                                } else {
                                    phi_21350_ = _e408.member_1;
                                }
                                let _e673 = phi_21350_;
                                if (_e408.member_7 > 0u) {
                                    phi_21361_ = ((2147483648u | ((_e408.member_7 - 1u) << bitcast<u32>(28u))) | _e408.member_5);
                                } else {
                                    phi_21361_ = _e408.member_5;
                                }
                                let _e682 = phi_21361_;
                                let _e683 = (_e536 <= _e664);
                                let _e687 = select(_e536, _e664, _e683);
                                if (_e687 < 300000000000000000000000000000000000000f) {
                                    let _e689 = (_e392 < 64u);
                                    if _e689 {
                                        if _e689 {
                                        } else {
                                            phi_38365_ = true;
                                            phi_21643_ = bool();
                                            phi_21642_ = vec4<f32>();
                                            break;
                                        }
                                        local_103[_e392] = select(_e673, _e682, _e683);
                                        if _e689 {
                                        } else {
                                            phi_38365_ = true;
                                            phi_21643_ = bool();
                                            phi_21642_ = vec4<f32>();
                                            break;
                                        }
                                        local_104[_e392] = _e687;
                                        phi_21400_ = (_e392 + 1u);
                                    } else {
                                        phi_21400_ = _e392;
                                    }
                                    let _e694 = phi_21400_;
                                    phi_21401_ = _e694;
                                } else {
                                    phi_21401_ = _e392;
                                }
                                let _e696 = phi_21401_;
                                phi_21412_ = select(4294967295u, select(_e682, _e673, _e683), (select(_e664, _e536, _e683) < 300000000000000000000000000000000000000f));
                                phi_21413_ = _e696;
                                phi_21414_ = u32();
                                phi_21415_ = u32();
                            } else {
                                let _e401 = (_e390 & 268435455u);
                                phi_21412_ = u32();
                                phi_21413_ = u32();
                                phi_21414_ = ((_e401 + ((_e390 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_21415_ = _e401;
                            }
                            let _e700 = phi_21412_;
                            let _e702 = phi_21413_;
                            let _e704 = phi_21414_;
                            let _e706 = phi_21415_;
                            phi_21417_ = _e700;
                            phi_21418_ = _e702;
                            phi_21420_ = _e704;
                            phi_21421_ = _e706;
                            phi_21422_ = select(true, false, _e400);
                            phi_21423_ = _e400;
                        } else {
                            phi_21417_ = u32();
                            phi_21418_ = u32();
                            phi_21420_ = _e359;
                            phi_21421_ = _e361;
                            phi_21422_ = _e396;
                            phi_21423_ = false;
                        }
                        let _e709 = phi_21417_;
                        let _e711 = phi_21418_;
                        let _e713 = phi_21420_;
                        let _e715 = phi_21421_;
                        let _e717 = phi_21422_;
                        let _e719 = phi_21423_;
                        phi_20907_ = _e353;
                        phi_20910_ = select(_e709, 4294967295u, _e717);
                        phi_20912_ = select(_e711, select(_e388, _e392, _e394), _e717);
                        phi_20914_ = select(_e359, _e713, _e717);
                        phi_20916_ = select(_e361, _e715, _e717);
                        phi_20918_ = _e363;
                        phi_20920_ = _e365;
                        phi_21434_ = vec4<f32>();
                        phi_21435_ = select(_e719, true, _e717);
                        phi_21436_ = select(select(_e398, false, _e394), false, _e717);
                    }
                    let _e947 = phi_20907_;
                    let _e949 = phi_20910_;
                    let _e951 = phi_20912_;
                    let _e953 = phi_20914_;
                    let _e955 = phi_20916_;
                    let _e957 = phi_20918_;
                    let _e959 = phi_20920_;
                    let _e961 = phi_21434_;
                    let _e963 = phi_21435_;
                    let _e965 = phi_21436_;
                    phi_20906_ = _e947;
                    phi_20909_ = _e949;
                    phi_20911_ = _e951;
                    phi_20913_ = _e953;
                    phi_20915_ = _e955;
                    phi_20917_ = _e957;
                    phi_20919_ = _e959;
                    phi_38365_ = false;
                    phi_21643_ = _e965;
                    phi_21642_ = _e961;
                    if !(_e963) {
                        break;
                    }
                }
                let _e968 = phi_38365_;
                let _e970 = phi_21643_;
                let _e972 = phi_21642_;
                if _e968 {
                    break;
                }
                if _e970 {
                    let _e974 = local_105;
                    let _e976 = local_106;
                    phi_21441_ = type_74(_e974, _e976);
                } else {
                    phi_21441_ = type_74();
                }
                let _e979 = phi_21441_;
                if select(true, false, _e970) {
                    let _e982 = local_107;
                    phi_21447_ = type_74(_e982, _e972);
                } else {
                    phi_21447_ = _e979;
                }
                let _e985 = phi_21447_;
                let _e988 = global_7.member[_e319.member_1];
                let _e1000 = type_34(_e988.member, _e988.member_1, _e988.member_2, _e988.member_3, vec4<u32>(_e988.member_4.x, _e988.member_4.y, _e985.member, _e988.member_4.w), _e988.member_5);
                if _e324 {
                } else {
                    break;
                }
                global_7.member[_e319.member_1] = type_34(_e1000.member, _e1000.member_1, _e1000.member_2, _e1000.member_3, _e1000.member_4, _e985.member_1);
            }
            break;
        }
    }
    return;
}

fn function_3() {
    var phi_21481_: type_48;
    var phi_21528_: bool;
    var phi_21529_: bool;
    var phi_21530_: bool;
    var phi_21546_: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e277 = global;
            let _e281 = arrayLength((&global_11.member));
            let _e285 = (_e277.x + (_e277.y * 4194240u));
            let _e289 = global_1.member.member[0u];
            let _e293 = global_1.member.member[1u];
            if (_e285 >= (_e289 * _e293)) {
                phi_21481_ = type_48(0u, type_48().member_1);
            } else {
                phi_21481_ = type_48(1u, _e285);
            }
            let _e301 = phi_21481_;
            if (bitcast<i32>(_e301.member) != 0i) {
                if (_e301.member_1 < arrayLength((&global_10.member))) {
                } else {
                    break;
                }
                let _e311 = global_10.member[_e301.member_1].member_4[1u];
                if ((_e311 & 8u) == 0u) {
                    let _e317 = global_10.member[_e301.member_1].member_3;
                    let _e325 = global_10.member[_e301.member_1].member[3u];
                    let _e326 = (_e317.x * _e325);
                    let _e327 = (_e317.y * _e325);
                    let _e328 = (_e317.z * _e325);
                    if (_e326 == _e326) {
                        let _e331 = (_e327 == _e327);
                        if _e331 {
                            phi_21528_ = select(true, false, (_e328 == _e328));
                        } else {
                            phi_21528_ = false;
                        }
                        let _e335 = phi_21528_;
                        phi_21529_ = select(true, false, _e331);
                        phi_21530_ = _e335;
                    } else {
                        phi_21529_ = true;
                        phi_21530_ = false;
                    }
                    let _e338 = phi_21529_;
                    let _e340 = phi_21530_;
                    let _e343 = select(vec3<f32>(_e326, _e327, _e328), vec3<f32>(0f, 0f, 0f), vec3(select(_e340, true, _e338)));
                    let _e348 = global_10.member[_e301.member_1].member_3[3u];
                    phi_21546_ = vec4<f32>(_e343.x, _e343.y, _e343.z, _e348);
                } else {
                    phi_21546_ = vec4<f32>(0f, 0f, 0f, 0f);
                }
                let _e354 = phi_21546_;
                if (_e354.w > 0f) {
                    let _e360 = (2u * _e301.member_1);
                    if (_e360 < _e281) {
                    } else {
                        break;
                    }
                    let _e365 = global_11.member[_e360][0u];
                    global_11.member[_e360][0u] = (_e365 + _e354.x);
                    let _e370 = global_11.member[_e360][1u];
                    global_11.member[_e360][1u] = (_e370 + _e354.y);
                    let _e375 = global_11.member[_e360][2u];
                    global_11.member[_e360][2u] = (_e375 + _e354.z);
                    let _e380 = global_11.member[_e360][3u];
                    global_11.member[_e360][3u] = (_e380 + _e354.w);
                    let _e382 = (_e360 + 1u);
                    if (_e382 < _e281) {
                    } else {
                        break;
                    }
                    let _e390 = global_11.member[_e382][0u];
                    global_11.member[_e382][0u] = (_e390 + (_e354.x * _e354.x));
                    let _e395 = global_11.member[_e382][1u];
                    global_11.member[_e382][1u] = (_e395 + (_e354.y * _e354.y));
                    let _e400 = global_11.member[_e382][2u];
                    global_11.member[_e382][2u] = (_e400 + (_e354.z * _e354.z));
                    let _e405 = global_11.member[_e382][3u];
                    global_11.member[_e382][3u] = (_e405 + 1f);
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
