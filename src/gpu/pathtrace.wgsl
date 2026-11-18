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

struct type_40 {
    member: array<u32>,
}

struct type_42 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
}

struct type_44 {
    member: array<type_42>,
}

struct type_49 {
    member: u32,
    member_1: u32,
}

struct type_53 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_56 {
    member: u32,
    member_1: type_53,
}

struct type_57 {
    member: f32,
    member_1: f32,
}

struct type_58 {
    member: u32,
    member_1: type_57,
}

struct type_59 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_60 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_61 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_68 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_73 {
    member: type_35,
    member_1: type_42,
    member_2: type_42,
}

struct type_76 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

struct type_78 {
    member: array<vec4<f32>>,
}

struct type_84 {
    member: array<atomic<u32>>,
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
@group(0) @binding(7) 
var<storage, read_write> global_8: type_84;
@group(0) @binding(8) 
var<storage, read_write> global_9: type_44;
@group(0) @binding(1) 
var<storage> global_10: type_37;
@group(0) @binding(2) 
var<storage, read_write> global_11: type_78;

fn function_() {
    var phi_491_: type_49;
    var phi_32486_: bool;
    var phi_511_: vec3<f32>;
    var phi_514_: u32;
    var phi_20755_: f32;
    var phi_20767_: type_57;
    var phi_20778_: type_58;
    var phi_20779_: type_58;
    var phi_20783_: type_58;
    var phi_20784_: type_58;
    var phi_20785_: bool;
    var phi_20789_: type_58;
    var phi_20676_: type_58;
    var phi_20827_: bool;
    var phi_20907_: f32;
    var phi_20919_: type_57;
    var phi_20930_: type_58;
    var phi_20931_: type_58;
    var phi_20935_: type_58;
    var phi_20936_: type_58;
    var phi_20937_: bool;
    var phi_20941_: type_58;
    var phi_20875_: f32;
    var phi_20979_: bool;
    var phi_20704_: f32;
    var phi_20706_: f32;
    var phi_20715_: type_58;
    var phi_20719_: type_58;
    var phi_20723_: type_58;
    var phi_20994_: bool;
    var phi_9514_: vec3<f32>;
    var phi_9517_: vec3<f32>;
    var phi_9519_: vec3<f32>;
    var phi_9521_: vec3<f32>;
    var phi_9523_: u32;
    var phi_9607_: u32;
    var phi_9622_: type_57;
    var phi_9623_: type_57;
    var phi_9624_: type_57;
    var phi_21194_: bool;
    var phi_21321_: bool;
    var phi_21336_: bool;
    var phi_9952_: vec3<f32>;
    var phi_10025_: vec3<f32>;
    var phi_10026_: vec3<f32>;
    var phi_10027_: bool;
    var phi_10032_: vec3<f32>;
    var phi_10033_: bool;
    var phi_21460_: bool;
    var phi_21475_: bool;
    var phi_10324_: vec3<f32>;
    var phi_9515_: vec3<f32>;
    var phi_9518_: vec3<f32>;
    var phi_9520_: vec3<f32>;
    var phi_9522_: vec3<f32>;
    var phi_9524_: u32;
    var phi_32469_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_32953_: bool;
    var phi_10381_: type_61;
    var phi_32952_: bool;
    var phi_10383_: type_61;
    var phi_10384_: bool;
    var phi_10389_: type_61;
    var phi_890_: vec3<f32>;
    var phi_892_: vec3<f32>;
    var phi_893_: bool;
    var phi_32561_: bool;
    var phi_903_: vec3<f32>;
    var phi_906_: i32;
    var phi_908_: u32;
    var phi_21744_: bool;
    var phi_1330_: u32;
    var phi_1360_: u32;
    var phi_1331_: u32;
    var phi_1361_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1379_: u32;
    var phi_2684_: type_56;
    var phi_2692_: type_53;
    var phi_2694_: type_53;
    var phi_2695_: bool;
    var phi_2696_: type_53;
    var phi_2697_: bool;
    var phi_2698_: bool;
    var phi_21855_: f32;
    var phi_21867_: type_57;
    var phi_21878_: type_58;
    var phi_21879_: type_58;
    var phi_21883_: type_58;
    var phi_21884_: type_58;
    var phi_21885_: bool;
    var phi_21889_: type_58;
    var phi_21776_: type_58;
    var phi_21927_: bool;
    var phi_22007_: f32;
    var phi_22019_: type_57;
    var phi_22030_: type_58;
    var phi_22031_: type_58;
    var phi_22035_: type_58;
    var phi_22036_: type_58;
    var phi_22037_: bool;
    var phi_22041_: type_58;
    var phi_21975_: f32;
    var phi_22079_: bool;
    var phi_21804_: f32;
    var phi_21806_: f32;
    var phi_21815_: type_58;
    var phi_21819_: type_58;
    var phi_21823_: type_58;
    var phi_2716_: vec3<f32>;
    var phi_2719_: vec3<f32>;
    var phi_2721_: vec3<f32>;
    var phi_2723_: vec3<f32>;
    var phi_2725_: u32;
    var phi_2809_: u32;
    var phi_2824_: type_57;
    var phi_2825_: type_57;
    var phi_2826_: type_57;
    var phi_22278_: bool;
    var phi_22405_: bool;
    var phi_22420_: bool;
    var phi_3154_: vec3<f32>;
    var phi_22542_: f32;
    var phi_22554_: type_57;
    var phi_22565_: type_58;
    var phi_22566_: type_58;
    var phi_22570_: type_58;
    var phi_22571_: type_58;
    var phi_22572_: bool;
    var phi_22576_: type_58;
    var phi_22510_: f32;
    var phi_22652_: f32;
    var phi_22664_: type_57;
    var phi_22675_: type_58;
    var phi_22676_: type_58;
    var phi_22680_: type_58;
    var phi_22681_: type_58;
    var phi_22682_: bool;
    var phi_22686_: type_58;
    var phi_22620_: f32;
    var phi_22447_: vec3<f32>;
    var phi_22448_: u32;
    var phi_22818_: bool;
    var phi_22481_: vec3<f32>;
    var phi_22482_: u32;
    var local_11: vec3<f32>;
    var phi_22489_: type_59;
    var phi_22493_: type_59;
    var phi_3213_: vec3<f32>;
    var phi_3214_: vec3<f32>;
    var phi_2717_: vec3<f32>;
    var phi_2720_: vec3<f32>;
    var phi_2722_: vec3<f32>;
    var phi_2724_: vec3<f32>;
    var phi_2726_: u32;
    var phi_32529_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_32623_: bool;
    var phi_3234_: type_53;
    var phi_32622_: bool;
    var phi_3235_: type_53;
    var phi_32621_: bool;
    var phi_3258_: vec3<f32>;
    var phi_3259_: i32;
    var phi_3260_: u32;
    var phi_32620_: bool;
    var phi_904_: vec3<f32>;
    var phi_907_: i32;
    var phi_909_: u32;
    var phi_3261_: bool;
    var phi_32619_: bool;
    var phi_3595_: vec3<f32>;
    var phi_3598_: u32;
    var phi_22946_: f32;
    var phi_22961_: f32;
    var phi_22976_: f32;
    var phi_3669_: i32;
    var phi_3699_: i32;
    var phi_3707_: vec3<f32>;
    var phi_3710_: i32;
    var phi_3733_: vec3<f32>;
    var phi_3736_: i32;
    var phi_3734_: vec3<f32>;
    var phi_3737_: i32;
    var phi_3708_: vec3<f32>;
    var phi_3711_: i32;
    var phi_3935_: vec3<f32>;
    var phi_3936_: vec3<f32>;
    var phi_3937_: vec3<f32>;
    var phi_3596_: vec3<f32>;
    var phi_3599_: u32;
    var phi_32624_: bool;
    var local_15: vec3<f32>;
    var phi_32672_: bool;
    var phi_3948_: f32;
    var phi_3972_: f32;
    var phi_3974_: u32;
    var phi_3977_: f32;
    var phi_4080_: f32;
    var phi_4081_: f32;
    var phi_4097_: f32;
    var phi_3975_: u32;
    var phi_3978_: f32;
    var phi_4100_: u32;
    var phi_4103_: f32;
    var phi_4211_: f32;
    var phi_4101_: u32;
    var phi_4104_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_23092_: f32;
    var phi_23104_: type_57;
    var phi_23115_: type_58;
    var phi_23116_: type_58;
    var phi_23120_: type_58;
    var phi_23121_: type_58;
    var phi_23122_: bool;
    var phi_23126_: type_58;
    var phi_23060_: f32;
    var phi_4612_: bool;
    var phi_23252_: f32;
    var phi_23264_: type_57;
    var phi_23275_: type_58;
    var phi_23276_: type_58;
    var phi_23280_: type_58;
    var phi_23281_: type_58;
    var phi_23282_: bool;
    var phi_23286_: type_58;
    var phi_23173_: type_58;
    var phi_23324_: bool;
    var phi_23404_: f32;
    var phi_23416_: type_57;
    var phi_23427_: type_58;
    var phi_23428_: type_58;
    var phi_23432_: type_58;
    var phi_23433_: type_58;
    var phi_23434_: bool;
    var phi_23438_: type_58;
    var phi_23372_: f32;
    var phi_23476_: bool;
    var phi_23201_: f32;
    var phi_23203_: f32;
    var phi_23212_: type_58;
    var phi_23216_: type_58;
    var phi_23220_: type_58;
    var phi_4626_: vec3<f32>;
    var phi_4629_: u32;
    var phi_4713_: u32;
    var phi_4728_: type_57;
    var phi_4729_: type_57;
    var phi_4730_: type_57;
    var phi_23675_: bool;
    var phi_4627_: vec3<f32>;
    var phi_4630_: u32;
    var phi_32651_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_32718_: bool;
    var phi_4755_: vec3<f32>;
    var phi_32717_: bool;
    var phi_4757_: vec3<f32>;
    var phi_4759_: bool;
    var phi_32713_: bool;
    var phi_4770_: vec3<f32>;
    var phi_4771_: bool;
    var phi_32712_: bool;
    var phi_4773_: vec3<f32>;
    var phi_4774_: bool;
    var phi_4775_: bool;
    var phi_32711_: bool;
    var phi_4779_: vec3<f32>;
    var phi_4780_: vec3<f32>;
    var phi_4781_: bool;
    var phi_4782_: vec3<f32>;
    var phi_4783_: bool;
    var phi_5103_: vec3<f32>;
    var phi_5106_: u32;
    var phi_23746_: f32;
    var phi_23761_: f32;
    var phi_23776_: f32;
    var phi_5177_: i32;
    var phi_5207_: i32;
    var phi_5215_: vec3<f32>;
    var phi_5218_: i32;
    var phi_5241_: vec3<f32>;
    var phi_5244_: i32;
    var phi_5315_: vec3<f32>;
    var phi_5318_: u32;
    var phi_5438_: vec3<f32>;
    var phi_5316_: vec3<f32>;
    var phi_5319_: u32;
    var phi_5242_: vec3<f32>;
    var phi_5245_: i32;
    var phi_5216_: vec3<f32>;
    var phi_5219_: i32;
    var phi_5443_: vec3<f32>;
    var phi_5444_: vec3<f32>;
    var phi_5445_: vec3<f32>;
    var phi_5104_: vec3<f32>;
    var phi_5107_: u32;
    var phi_32706_: bool;
    var phi_32748_: bool;
    var phi_5465_: vec3<f32>;
    var phi_5678_: u32;
    var phi_5679_: bool;
    var phi_5682_: u32;
    var phi_5683_: bool;
    var phi_5684_: bool;
    var phi_5694_: f32;
    var phi_5697_: f32;
    var phi_5699_: u32;
    var phi_5701_: vec3<f32>;
    var phi_5703_: vec3<f32>;
    var phi_5705_: f32;
    var phi_5707_: f32;
    var phi_5745_: u32;
    var phi_5760_: vec3<f32>;
    var phi_5780_: vec3<f32>;
    var phi_23791_: vec3<f32>;
    var phi_5781_: u32;
    var phi_5782_: vec3<f32>;
    var phi_5783_: u32;
    var phi_5784_: vec3<f32>;
    var phi_5785_: u32;
    var phi_5787_: vec3<f32>;
    var phi_5922_: u32;
    var phi_5925_: f32;
    var phi_5927_: f32;
    var phi_5929_: f32;
    var phi_5931_: vec3<f32>;
    var phi_23846_: f32;
    var phi_23852_: f32;
    var phi_23880_: f32;
    var phi_23886_: f32;
    var phi_23914_: f32;
    var phi_23920_: f32;
    var phi_23948_: f32;
    var phi_23954_: f32;
    var phi_23982_: f32;
    var phi_23988_: f32;
    var phi_24016_: f32;
    var phi_24022_: f32;
    var phi_24050_: f32;
    var phi_24056_: f32;
    var phi_24084_: f32;
    var phi_24090_: f32;
    var phi_5923_: u32;
    var phi_5926_: f32;
    var phi_5928_: f32;
    var phi_5930_: f32;
    var phi_5932_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6348_: f32;
    var phi_6400_: vec3<f32>;
    var phi_6401_: f32;
    var phi_6402_: vec3<f32>;
    var phi_6403_: f32;
    var phi_6404_: vec3<f32>;
    var phi_6405_: f32;
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
    var phi_6420_: f32;
    var phi_6421_: f32;
    var phi_6422_: vec3<f32>;
    var phi_6423_: vec3<f32>;
    var phi_6425_: f32;
    var phi_6426_: f32;
    var phi_6427_: vec3<f32>;
    var phi_6428_: vec3<f32>;
    var phi_6429_: f32;
    var phi_6430_: f32;
    var phi_6431_: f32;
    var phi_6432_: f32;
    var phi_6433_: vec3<f32>;
    var phi_6434_: vec3<f32>;
    var phi_6435_: f32;
    var phi_6436_: f32;
    var phi_5695_: f32;
    var phi_5698_: f32;
    var phi_5700_: u32;
    var phi_5702_: vec3<f32>;
    var phi_5704_: vec3<f32>;
    var phi_5706_: f32;
    var phi_5708_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_24166_: bool;
    var local_29: f32;
    var phi_6468_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6547_: f32;
    var phi_6548_: vec3<f32>;
    var phi_6549_: f32;
    var phi_6551_: vec3<f32>;
    var phi_6554_: u32;
    var phi_7002_: f32;
    var phi_7182_: u32;
    var phi_7185_: f32;
    var phi_7187_: f32;
    var phi_7189_: f32;
    var phi_7191_: vec3<f32>;
    var phi_24255_: f32;
    var phi_24261_: f32;
    var phi_24289_: f32;
    var phi_24295_: f32;
    var phi_24323_: f32;
    var phi_24329_: f32;
    var phi_24357_: f32;
    var phi_24363_: f32;
    var phi_24391_: f32;
    var phi_24397_: f32;
    var phi_24425_: f32;
    var phi_24431_: f32;
    var phi_24459_: f32;
    var phi_24465_: f32;
    var phi_24493_: f32;
    var phi_24499_: f32;
    var phi_7183_: u32;
    var phi_7186_: f32;
    var phi_7188_: f32;
    var phi_7190_: f32;
    var phi_7192_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7605_: f32;
    var phi_24545_: bool;
    var phi_24560_: bool;
    var phi_7698_: vec3<f32>;
    var phi_7699_: vec3<f32>;
    var phi_7700_: vec3<f32>;
    var phi_7701_: vec3<f32>;
    var phi_6552_: vec3<f32>;
    var phi_6555_: u32;
    var phi_7720_: vec3<f32>;
    var phi_7723_: u32;
    var phi_7725_: f32;
    var phi_8026_: vec3<f32>;
    var phi_8027_: u32;
    var phi_8028_: f32;
    var phi_8029_: vec3<f32>;
    var phi_8046_: vec3<f32>;
    var phi_8048_: u32;
    var phi_8051_: vec3<f32>;
    var phi_8053_: f32;
    var phi_8055_: f32;
    var phi_8057_: f32;
    var phi_8077_: bool;
    var phi_8093_: bool;
    var phi_8095_: bool;
    var phi_8097_: bool;
    var phi_8098_: bool;
    var phi_8099_: bool;
    var phi_8100_: bool;
    var phi_8101_: bool;
    var phi_8145_: vec3<f32>;
    var phi_24579_: vec3<f32>;
    var phi_8146_: u32;
    var phi_8147_: vec3<f32>;
    var phi_8148_: u32;
    var phi_8149_: vec3<f32>;
    var phi_8284_: u32;
    var phi_8287_: f32;
    var phi_8289_: f32;
    var phi_8291_: f32;
    var phi_8293_: vec3<f32>;
    var phi_24634_: f32;
    var phi_24640_: f32;
    var phi_24668_: f32;
    var phi_24674_: f32;
    var phi_24702_: f32;
    var phi_24708_: f32;
    var phi_24736_: f32;
    var phi_24742_: f32;
    var phi_24770_: f32;
    var phi_24776_: f32;
    var phi_24804_: f32;
    var phi_24810_: f32;
    var phi_24838_: f32;
    var phi_24844_: f32;
    var phi_24872_: f32;
    var phi_24878_: f32;
    var phi_8285_: u32;
    var phi_8288_: f32;
    var phi_8290_: f32;
    var phi_8292_: f32;
    var phi_8294_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8707_: f32;
    var phi_8733_: vec3<f32>;
    var phi_8734_: f32;
    var phi_8735_: f32;
    var phi_8736_: vec3<f32>;
    var phi_8737_: f32;
    var phi_8738_: f32;
    var phi_8739_: vec3<f32>;
    var phi_8740_: f32;
    var phi_8741_: f32;
    var phi_8742_: vec3<f32>;
    var phi_8743_: f32;
    var phi_8744_: f32;
    var phi_8746_: vec3<f32>;
    var phi_8747_: f32;
    var phi_8748_: f32;
    var phi_8749_: f32;
    var phi_8049_: u32;
    var phi_8052_: vec3<f32>;
    var phi_8054_: f32;
    var phi_8056_: f32;
    var phi_8058_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9039_: type_60;
    var local_42: f32;
    var phi_9040_: type_60;
    var local_43: f32;
    var local_44: f32;
    var phi_9041_: type_60;
    var phi_9042_: type_60;
    var phi_7721_: vec3<f32>;
    var phi_7724_: u32;
    var phi_7726_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9071_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_32956_: bool;
    var phi_9118_: vec3<f32>;
    var phi_32955_: bool;
    var phi_9119_: vec3<f32>;
    var phi_32954_: bool;
    var phi_9120_: vec3<f32>;
    var phi_32950_: bool;
    var phi_10393_: vec3<f32>;
    var phi_32949_: bool;
    var phi_10394_: vec3<f32>;
    var phi_32948_: bool;
    var phi_512_: vec3<f32>;
    var phi_515_: u32;
    var phi_32947_: bool;
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
                phi_491_ = type_49(0u, type_49().member_1);
            } else {
                let _e301 = global_1.member.member[0u];
                let _e306 = global_1.member.member[1u];
                let _e312 = atomicLoad((&global_8.member[((8u + ((_e287 * _e301) * _e306)) + _e291)]));
                phi_491_ = type_49(1u, _e312);
            }
            let _e318 = phi_491_;
            if (bitcast<i32>(_e318.member) != 0i) {
                phi_32486_ = false;
                phi_511_ = vec3<f32>(0f, 0f, 0f);
                phi_514_ = 0u;
                loop {
                    let _e324 = phi_32486_;
                    let _e326 = phi_511_;
                    let _e328 = phi_514_;
                    local_55 = _e326;
                    local_56 = _e326;
                    local_57 = _e326;
                    let _e329 = (_e328 < 2u);
                    if _e329 {
                        let _e331 = ((2u * _e318.member_1) + _e328);
                        if (_e331 < arrayLength((&global_9.member))) {
                        } else {
                            phi_32947_ = true;
                            break;
                        }
                        let _e335 = global_9.member[_e331];
                        if ((((_e335.member_2.x * _e335.member_2.x) + (_e335.member_2.y * _e335.member_2.y)) + (_e335.member_2.z * _e335.member_2.z)) == 0f) {
                            phi_32949_ = _e324;
                            phi_10394_ = vec3<f32>(0f, 0f, 0f);
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
                                            phi_32947_ = true;
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
                                            phi_32947_ = true;
                                            break;
                                        }
                                        let _e1341 = (bitcast<u32>((select(0i, select(select(i32(_e1326), i32(-2147483648), (_e1326 < -2147483600f)), 2147483647i, (_e1326 > 2147483500f)), (_e1326 == _e1326)) + bitcast<i32>(_e1298))) % _e1298);
                                        let _e1346 = select(select(u32(_e1327), 0u, (_e1327 < 0f)), 4294967295u, (_e1327 > 4294967000f));
                                        let _e1347 = (_e1346 + 1u);
                                        let _e1352 = global_1.member.member_2[2u];
                                        let _e1353 = (_e1346 * _e1298);
                                        if _e1340 {
                                            phi_32947_ = true;
                                            break;
                                        }
                                        let _e1354 = (_e1341 % _e1298);
                                        let _e1357 = (_e1352 + (3u * (_e1353 + _e1354)));
                                        let _e1359 = global_6.member[_e1357];
                                        let _e1362 = global_6.member[(_e1357 + 1u)];
                                        let _e1365 = global_6.member[(_e1357 + 2u)];
                                        if _e1340 {
                                            phi_32947_ = true;
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
                                            phi_32947_ = true;
                                            break;
                                        }
                                        let _e1391 = (_e1352 + (3u * (_e1388 + _e1354)));
                                        let _e1393 = global_6.member[_e1391];
                                        let _e1396 = global_6.member[(_e1391 + 1u)];
                                        let _e1399 = global_6.member[(_e1391 + 2u)];
                                        if _e1340 {
                                            phi_32947_ = true;
                                            break;
                                        }
                                        let _e1403 = (_e1352 + (3u * (_e1388 + _e1368)));
                                        let _e1405 = global_6.member[_e1403];
                                        let _e1408 = global_6.member[(_e1403 + 1u)];
                                        let _e1411 = global_6.member[(_e1403 + 2u)];
                                        let _e1419 = (vec3<f32>(_e1393, _e1396, _e1399) + vec3<f32>(((_e1405 - _e1393) * _e1328), ((_e1408 - _e1396) * _e1328), ((_e1411 - _e1399) * _e1328)));
                                        phi_890_ = (_e1387 + vec3<f32>(((_e1419.x - _e1387.x) * _e1329), ((_e1419.y - _e1387.y) * _e1329), ((_e1419.z - _e1387.z) * _e1329)));
                                    } else {
                                        phi_890_ = vec3<f32>();
                                    }
                                    let _e1435 = phi_890_;
                                    phi_892_ = _e1435;
                                    phi_893_ = select(true, false, _e1295);
                                } else {
                                    phi_892_ = vec3<f32>();
                                    phi_893_ = true;
                                }
                                let _e1438 = phi_892_;
                                let _e1440 = phi_893_;
                                if _e1440 {
                                    let _e1441 = (_e1293 != 4294967295u);
                                    if (_e1290 != 4294967295u) {
                                        phi_32561_ = _e324;
                                        phi_903_ = vec3<f32>(0f, 0f, 0f);
                                        phi_906_ = 0i;
                                        phi_908_ = _e1290;
                                        loop {
                                            let _e1444 = phi_32561_;
                                            let _e1446 = phi_903_;
                                            let _e1448 = phi_906_;
                                            let _e1450 = phi_908_;
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
                                                        phi_2696_ = type_53();
                                                        phi_2697_ = true;
                                                        phi_2698_ = false;
                                                    } else {
                                                        let _e1529 = (_e1523 == 4294967295u);
                                                        if _e1529 {
                                                            phi_2694_ = type_53();
                                                            phi_2695_ = false;
                                                        } else {
                                                            let _e1543 = (_e335.member.x - _e1454);
                                                            let _e1544 = (_e335.member.y - _e1457);
                                                            let _e1545 = (_e335.member.z - _e1460);
                                                            let _e1553 = ((((((_e335.member.x * _e335.member.x) + (_e335.member.y * _e335.member.y)) + (_e335.member.z * _e335.member.z)) - (2f * (((_e335.member.x * _e1454) + (_e335.member.y * _e1457)) + (_e335.member.z * _e1460)))) + _e1469) / (sqrt((((_e1543 * _e1543) + (_e1544 * _e1544)) + (_e1545 * _e1545))) + _e1463));
                                                            let _e1554 = (_e1450 + 21u);
                                                            let _e1557 = global_6.member[(_e1450 + 28u)];
                                                            if (_e1553 > _e1557) {
                                                                phi_2684_ = type_56(0u, type_56().member_1);
                                                            } else {
                                                                let _e1560 = global_6.member[_e1554];
                                                                if (_e1553 != _e1553) {
                                                                    phi_21744_ = true;
                                                                } else {
                                                                    phi_21744_ = (_e1560 >= _e1553);
                                                                }
                                                                let _e1564 = phi_21744_;
                                                                let _e1565 = select(_e1553, _e1560, _e1564);
                                                                phi_1330_ = 0u;
                                                                loop {
                                                                    let _e1567 = phi_1330_;
                                                                    local_8 = _e1567;
                                                                    local_9 = _e1567;
                                                                    local_10 = _e1567;
                                                                    if (_e1567 < 8u) {
                                                                        let _e1571 = global_6.member[(_e1554 + _e1567)];
                                                                        let _e1572 = (_e1571 <= _e1565);
                                                                        if _e1572 {
                                                                            phi_1360_ = (_e1567 + 1u);
                                                                        } else {
                                                                            phi_1360_ = u32();
                                                                        }
                                                                        let _e1575 = phi_1360_;
                                                                        phi_1331_ = _e1575;
                                                                        phi_1361_ = _e1572;
                                                                    } else {
                                                                        phi_1331_ = u32();
                                                                        phi_1361_ = false;
                                                                    }
                                                                    let _e1577 = phi_1331_;
                                                                    let _e1579 = phi_1361_;
                                                                    phi_1330_ = _e1577;
                                                                    if !(_e1579) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1582 = local_8;
                                                                if (_e1582 < 1u) {
                                                                    phi_1379_ = 1u;
                                                                } else {
                                                                    let _e1585 = local_9;
                                                                    let _e1588 = local_10;
                                                                    phi_1379_ = select(_e1588, 7u, (_e1585 > 7u));
                                                                }
                                                                let _e1591 = phi_1379_;
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
                                                                phi_2684_ = type_56(1u, type_53(vec3<f32>(((((((_e1658 * _e1653) + (_e1669 * _e1654)) + (_e1683 * _e1655)) + (_e1697 * _e1656)) * _e1997) + (((((_e1831 * _e1653) + (_e1842 * _e1654)) + (_e1856 * _e1655)) + (_e1870 * _e1656)) * _e1601)), ((((((_e1661 * _e1653) + (_e1672 * _e1654)) + (_e1686 * _e1655)) + (_e1700 * _e1656)) * _e1997) + (((((_e1834 * _e1653) + (_e1845 * _e1654)) + (_e1859 * _e1655)) + (_e1873 * _e1656)) * _e1601)), ((((((_e1664 * _e1653) + (_e1675 * _e1654)) + (_e1689 * _e1655)) + (_e1703 * _e1656)) * _e1997) + (((((_e1837 * _e1653) + (_e1848 * _e1654)) + (_e1862 * _e1655)) + (_e1876 * _e1656)) * _e1601))), vec3<f32>(((((((_e1712 * _e1653) + (_e1724 * _e1654)) + (_e1739 * _e1655)) + (_e1754 * _e1656)) * _e1997) + (((((_e1885 * _e1653) + (_e1897 * _e1654)) + (_e1912 * _e1655)) + (_e1927 * _e1656)) * _e1601)), ((((((_e1715 * _e1653) + (_e1727 * _e1654)) + (_e1742 * _e1655)) + (_e1757 * _e1656)) * _e1997) + (((((_e1888 * _e1653) + (_e1900 * _e1654)) + (_e1915 * _e1655)) + (_e1930 * _e1656)) * _e1601)), ((((((_e1718 * _e1653) + (_e1730 * _e1654)) + (_e1745 * _e1655)) + (_e1760 * _e1656)) * _e1997) + (((((_e1891 * _e1653) + (_e1903 * _e1654)) + (_e1918 * _e1655)) + (_e1933 * _e1656)) * _e1601))), vec3<f32>(((((((_e1769 * _e1653) + (_e1781 * _e1654)) + (_e1796 * _e1655)) + (_e1811 * _e1656)) * _e1997) + (((((_e1942 * _e1653) + (_e1954 * _e1654)) + (_e1969 * _e1655)) + (_e1984 * _e1656)) * _e1601)), ((((((_e1772 * _e1653) + (_e1784 * _e1654)) + (_e1799 * _e1655)) + (_e1814 * _e1656)) * _e1997) + (((((_e1945 * _e1653) + (_e1957 * _e1654)) + (_e1972 * _e1655)) + (_e1987 * _e1656)) * _e1601)), ((((((_e1775 * _e1653) + (_e1787 * _e1654)) + (_e1802 * _e1655)) + (_e1817 * _e1656)) * _e1997) + (((((_e1948 * _e1653) + (_e1960 * _e1654)) + (_e1975 * _e1655)) + (_e1990 * _e1656)) * _e1601)))));
                                                            }
                                                            let _e2034 = phi_2684_;
                                                            let _e2037 = (bitcast<i32>(_e2034.member) != 0i);
                                                            if _e2037 {
                                                                phi_2692_ = _e2034.member_1;
                                                            } else {
                                                                phi_2692_ = type_53();
                                                            }
                                                            let _e2040 = phi_2692_;
                                                            phi_2694_ = _e2040;
                                                            phi_2695_ = select(true, false, _e2037);
                                                        }
                                                        let _e2043 = phi_2694_;
                                                        let _e2045 = phi_2695_;
                                                        phi_2696_ = _e2043;
                                                        phi_2697_ = _e1529;
                                                        phi_2698_ = _e2045;
                                                    }
                                                    let _e2047 = phi_2696_;
                                                    let _e2049 = phi_2697_;
                                                    let _e2051 = phi_2698_;
                                                    if select(_e2051, true, _e2049) {
                                                        let _e2060 = ((((_e335.member.x - _e1454) * _e335.member_1.x) + ((_e335.member.y - _e1457) * _e335.member_1.y)) + ((_e335.member.z - _e1460) * _e335.member_1.z));
                                                        let _e2072 = ((((_e335.member.x * _e335.member.x) + (_e335.member.y * _e335.member.y)) + (_e335.member.z * _e335.member.z)) - (2f * (((_e335.member.x * _e1454) + (_e335.member.y * _e1457)) + (_e335.member.z * _e1460))));
                                                        let _e2073 = (_e2072 + _e1472);
                                                        let _e2074 = (_e2060 * _e2060);
                                                        let _e2075 = (_e2074 - _e2073);
                                                        if (_e2075 < 0f) {
                                                            phi_21883_ = type_58(0u, type_58().member_1);
                                                            phi_21884_ = type_58();
                                                            phi_21885_ = true;
                                                        } else {
                                                            if (_e2060 < 0f) {
                                                                phi_21855_ = -(sqrt(_e2075));
                                                            } else {
                                                                phi_21855_ = sqrt(_e2075);
                                                            }
                                                            let _e2082 = phi_21855_;
                                                            let _e2084 = -((_e2060 + _e2082));
                                                            let _e2085 = (_e2084 == 0f);
                                                            if _e2085 {
                                                                phi_21878_ = type_58(1u, type_57(0f, 0f));
                                                                phi_21879_ = type_58();
                                                            } else {
                                                                let _e2086 = (_e2073 / _e2084);
                                                                if (_e2084 < _e2086) {
                                                                    phi_21867_ = type_57(_e2084, _e2086);
                                                                } else {
                                                                    phi_21867_ = type_57(_e2086, _e2084);
                                                                }
                                                                let _e2091 = phi_21867_;
                                                                phi_21878_ = type_58();
                                                                phi_21879_ = type_58(1u, _e2091);
                                                            }
                                                            let _e2094 = phi_21878_;
                                                            let _e2096 = phi_21879_;
                                                            phi_21883_ = _e2094;
                                                            phi_21884_ = _e2096;
                                                            phi_21885_ = _e2085;
                                                        }
                                                        let _e2101 = phi_21883_;
                                                        let _e2103 = phi_21884_;
                                                        let _e2105 = phi_21885_;
                                                        if _e2105 {
                                                            phi_21889_ = _e2101;
                                                        } else {
                                                            phi_21889_ = _e2103;
                                                        }
                                                        let _e2107 = phi_21889_;
                                                        if (bitcast<i32>(_e2107.member) != 0i) {
                                                            phi_21776_ = type_58(0u, _e2107.member_1);
                                                        } else {
                                                            phi_21776_ = type_58(1u, type_58().member_1);
                                                        }
                                                        let _e2117 = phi_21776_;
                                                        if (bitcast<i32>(_e2117.member) != 0i) {
                                                            phi_21823_ = type_58(0u, type_58().member_1);
                                                        } else {
                                                            if (_e2117.member_1.member_1 <= 0f) {
                                                                phi_21819_ = type_58(0u, type_58().member_1);
                                                            } else {
                                                                if (_e2117.member_1.member != _e2117.member_1.member) {
                                                                    phi_21927_ = true;
                                                                } else {
                                                                    phi_21927_ = (0f >= _e2117.member_1.member);
                                                                }
                                                                let _e2129 = phi_21927_;
                                                                let _e2130 = select(_e2117.member_1.member, 0f, _e2129);
                                                                let _e2131 = (_e2072 + _e1469);
                                                                if (_e2131 > 0f) {
                                                                    let _e2133 = (_e2074 - _e2131);
                                                                    if (_e2133 < 0f) {
                                                                        phi_22035_ = type_58(0u, type_58().member_1);
                                                                        phi_22036_ = type_58();
                                                                        phi_22037_ = true;
                                                                    } else {
                                                                        if (_e2060 < 0f) {
                                                                            phi_22007_ = -(sqrt(_e2133));
                                                                        } else {
                                                                            phi_22007_ = sqrt(_e2133);
                                                                        }
                                                                        let _e2140 = phi_22007_;
                                                                        let _e2142 = -((_e2060 + _e2140));
                                                                        let _e2143 = (_e2142 == 0f);
                                                                        if _e2143 {
                                                                            phi_22030_ = type_58(1u, type_57(0f, 0f));
                                                                            phi_22031_ = type_58();
                                                                        } else {
                                                                            let _e2144 = (_e2131 / _e2142);
                                                                            if (_e2142 < _e2144) {
                                                                                phi_22019_ = type_57(_e2142, _e2144);
                                                                            } else {
                                                                                phi_22019_ = type_57(_e2144, _e2142);
                                                                            }
                                                                            let _e2149 = phi_22019_;
                                                                            phi_22030_ = type_58();
                                                                            phi_22031_ = type_58(1u, _e2149);
                                                                        }
                                                                        let _e2152 = phi_22030_;
                                                                        let _e2154 = phi_22031_;
                                                                        phi_22035_ = _e2152;
                                                                        phi_22036_ = _e2154;
                                                                        phi_22037_ = _e2143;
                                                                    }
                                                                    let _e2159 = phi_22035_;
                                                                    let _e2161 = phi_22036_;
                                                                    let _e2163 = phi_22037_;
                                                                    if _e2163 {
                                                                        phi_22041_ = _e2159;
                                                                    } else {
                                                                        phi_22041_ = _e2161;
                                                                    }
                                                                    let _e2165 = phi_22041_;
                                                                    if (bitcast<i32>(_e2165.member) != 0i) {
                                                                        phi_21975_ = select(_e2165.member_1.member_1, _e2165.member_1.member, (_e2165.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_21975_ = -1f;
                                                                    }
                                                                    let _e2176 = phi_21975_;
                                                                    if (_e2176 >= 0f) {
                                                                        if (_e2117.member_1.member_1 != _e2117.member_1.member_1) {
                                                                            phi_22079_ = true;
                                                                        } else {
                                                                            phi_22079_ = (_e2176 <= _e2117.member_1.member_1);
                                                                        }
                                                                        let _e2181 = phi_22079_;
                                                                        phi_21804_ = select(_e2117.member_1.member_1, _e2176, _e2181);
                                                                    } else {
                                                                        phi_21804_ = _e2117.member_1.member_1;
                                                                    }
                                                                    let _e2184 = phi_21804_;
                                                                    phi_21806_ = _e2184;
                                                                } else {
                                                                    phi_21806_ = _e2117.member_1.member_1;
                                                                }
                                                                let _e2186 = phi_21806_;
                                                                if (_e2186 <= _e2130) {
                                                                    phi_21815_ = type_58(0u, type_58().member_1);
                                                                } else {
                                                                    phi_21815_ = type_58(1u, type_57(_e2130, _e2186));
                                                                }
                                                                let _e2194 = phi_21815_;
                                                                phi_21819_ = _e2194;
                                                            }
                                                            let _e2199 = phi_21819_;
                                                            phi_21823_ = _e2199;
                                                        }
                                                        let _e2204 = phi_21823_;
                                                        if (bitcast<i32>(_e2204.member) != 0i) {
                                                            phi_2716_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2719_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2721_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2723_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2725_ = 0u;
                                                            loop {
                                                                let _e2213 = phi_2716_;
                                                                let _e2215 = phi_2719_;
                                                                let _e2217 = phi_2721_;
                                                                let _e2219 = phi_2723_;
                                                                let _e2221 = phi_2725_;
                                                                local_12 = _e2219;
                                                                local_13 = _e2217;
                                                                local_14 = _e2215;
                                                                let _e2222 = (_e2221 < 32u);
                                                                if _e2222 {
                                                                    let _e2223 = -(_e2060);
                                                                    if (_e2204.member_1.member <= _e2204.member_1.member_1) {
                                                                    } else {
                                                                        phi_32529_ = true;
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
                                                                        phi_2826_ = type_57((_e2204.member_1.member + ((_e2231 * _e2312) * _e2312)), (_e2231 * ((_e2310 * _e2310) - (_e2307 * _e2307))));
                                                                    } else {
                                                                        if (_e2230 <= _e2232) {
                                                                            let _e2290 = (f32((31u - _e2221)) * 0.03125f);
                                                                            let _e2293 = (f32((32u - _e2221)) * 0.03125f);
                                                                            let _e2295 = ((_e2290 + _e2293) * 0.5f);
                                                                            phi_2825_ = type_57((_e2204.member_1.member_1 - ((_e2231 * _e2295) * _e2295)), (_e2231 * ((_e2293 * _e2293) - (_e2290 * _e2290))));
                                                                        } else {
                                                                            let _e2238 = round(((32f * _e2229) / (_e2229 + _e2230)));
                                                                            let _e2243 = select(select(u32(_e2238), 0u, (_e2238 < 0f)), 4294967295u, (_e2238 > 4294967000f));
                                                                            if (_e2243 < 1u) {
                                                                                phi_2809_ = 1u;
                                                                            } else {
                                                                                phi_2809_ = select(_e2243, 31u, (_e2243 > 31u));
                                                                            }
                                                                            let _e2248 = phi_2809_;
                                                                            if (_e2221 < _e2248) {
                                                                                let _e2269 = ((_e2248 - 1u) - _e2221);
                                                                                let _e2271 = f32(_e2248);
                                                                                let _e2272 = (f32(_e2269) / _e2271);
                                                                                let _e2275 = (f32((_e2269 + 1u)) / _e2271);
                                                                                let _e2277 = ((_e2272 + _e2275) * 0.5f);
                                                                                phi_2824_ = type_57((_e2228 - ((_e2229 * _e2277) * _e2277)), (_e2229 * ((_e2275 * _e2275) - (_e2272 * _e2272))));
                                                                            } else {
                                                                                let _e2251 = (_e2221 - _e2248);
                                                                                let _e2253 = f32((32u - _e2248));
                                                                                let _e2254 = (f32(_e2251) / _e2253);
                                                                                let _e2257 = (f32((_e2251 + 1u)) / _e2253);
                                                                                let _e2259 = ((_e2254 + _e2257) * 0.5f);
                                                                                phi_2824_ = type_57((_e2228 + ((_e2230 * _e2259) * _e2259)), (_e2230 * ((_e2257 * _e2257) - (_e2254 * _e2254))));
                                                                            }
                                                                            let _e2287 = phi_2824_;
                                                                            phi_2825_ = _e2287;
                                                                        }
                                                                        let _e2305 = phi_2825_;
                                                                        phi_2826_ = _e2305;
                                                                    }
                                                                    let _e2322 = phi_2826_;
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
                                                                        phi_22278_ = true;
                                                                    } else {
                                                                        phi_22278_ = (0f >= _e2359);
                                                                    }
                                                                    let _e2363 = phi_22278_;
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
                                                                        phi_3154_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2414 = (1f / _e2353);
                                                                        let _e2422 = ((((_e2345 * _e2414) * _e1475) + ((_e2346 * _e2414) * _e1478)) + ((_e2347 * _e2414) * _e1481));
                                                                        if (_e2355 != _e2355) {
                                                                            phi_22405_ = true;
                                                                        } else {
                                                                            phi_22405_ = (0f >= _e2355);
                                                                        }
                                                                        let _e2426 = phi_22405_;
                                                                        let _e2430 = sqrt((select(_e2355, 0f, _e2426) / (_e1466 - _e1463)));
                                                                        if (_e2430 != _e2430) {
                                                                            phi_22420_ = true;
                                                                        } else {
                                                                            phi_22420_ = (1f <= _e2430);
                                                                        }
                                                                        let _e2434 = phi_22420_;
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
                                                                        phi_3154_ = vec3<f32>(((((_e2471 * _e2478) + (_e2484 * _e2460)) * _e2497) + (((_e2502 * _e2478) + (_e2514 * _e2460)) * _e2458)), ((((_e2474 * _e2478) + (_e2487 * _e2460)) * _e2497) + (((_e2505 * _e2478) + (_e2517 * _e2460)) * _e2458)), ((((_e2477 * _e2478) + (_e2490 * _e2460)) * _e2497) + (((_e2508 * _e2478) + (_e2520 * _e2460)) * _e2458)));
                                                                    }
                                                                    let _e2535 = phi_3154_;
                                                                    let _e2553 = (((_e2345 * _e1475) + (_e2346 * _e1478)) + (_e2347 * _e1481));
                                                                    let _e2554 = (_e2553 * _e2553);
                                                                    let _e2555 = (_e2554 - _e2344);
                                                                    if (_e2555 < 0f) {
                                                                        phi_22570_ = type_58(0u, type_58().member_1);
                                                                        phi_22571_ = type_58();
                                                                        phi_22572_ = true;
                                                                    } else {
                                                                        if (_e2553 < 0f) {
                                                                            phi_22542_ = -(sqrt(_e2555));
                                                                        } else {
                                                                            phi_22542_ = sqrt(_e2555);
                                                                        }
                                                                        let _e2562 = phi_22542_;
                                                                        let _e2564 = -((_e2553 + _e2562));
                                                                        let _e2565 = (_e2564 == 0f);
                                                                        if _e2565 {
                                                                            phi_22565_ = type_58(1u, type_57(0f, 0f));
                                                                            phi_22566_ = type_58();
                                                                        } else {
                                                                            let _e2566 = (_e2344 / _e2564);
                                                                            if (_e2564 < _e2566) {
                                                                                phi_22554_ = type_57(_e2564, _e2566);
                                                                            } else {
                                                                                phi_22554_ = type_57(_e2566, _e2564);
                                                                            }
                                                                            let _e2571 = phi_22554_;
                                                                            phi_22565_ = type_58();
                                                                            phi_22566_ = type_58(1u, _e2571);
                                                                        }
                                                                        let _e2574 = phi_22565_;
                                                                        let _e2576 = phi_22566_;
                                                                        phi_22570_ = _e2574;
                                                                        phi_22571_ = _e2576;
                                                                        phi_22572_ = _e2565;
                                                                    }
                                                                    let _e2581 = phi_22570_;
                                                                    let _e2583 = phi_22571_;
                                                                    let _e2585 = phi_22572_;
                                                                    if _e2585 {
                                                                        phi_22576_ = _e2581;
                                                                    } else {
                                                                        phi_22576_ = _e2583;
                                                                    }
                                                                    let _e2587 = phi_22576_;
                                                                    if (bitcast<i32>(_e2587.member) != 0i) {
                                                                        phi_22510_ = select(_e2587.member_1.member_1, _e2587.member_1.member, (_e2587.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22510_ = -1f;
                                                                    }
                                                                    let _e2598 = phi_22510_;
                                                                    if (_e2598 >= 0f) {
                                                                        phi_22493_ = type_59(0u, type_59().member_1);
                                                                    } else {
                                                                        let _e2600 = (_e2343 + _e1472);
                                                                        let _e2601 = (_e2554 - _e2600);
                                                                        if (_e2601 < 0f) {
                                                                            phi_22680_ = type_58(0u, type_58().member_1);
                                                                            phi_22681_ = type_58();
                                                                            phi_22682_ = true;
                                                                        } else {
                                                                            if (_e2553 < 0f) {
                                                                                phi_22652_ = -(sqrt(_e2601));
                                                                            } else {
                                                                                phi_22652_ = sqrt(_e2601);
                                                                            }
                                                                            let _e2608 = phi_22652_;
                                                                            let _e2610 = -((_e2553 + _e2608));
                                                                            let _e2611 = (_e2610 == 0f);
                                                                            if _e2611 {
                                                                                phi_22675_ = type_58(1u, type_57(0f, 0f));
                                                                                phi_22676_ = type_58();
                                                                            } else {
                                                                                let _e2612 = (_e2600 / _e2610);
                                                                                if (_e2610 < _e2612) {
                                                                                    phi_22664_ = type_57(_e2610, _e2612);
                                                                                } else {
                                                                                    phi_22664_ = type_57(_e2612, _e2610);
                                                                                }
                                                                                let _e2617 = phi_22664_;
                                                                                phi_22675_ = type_58();
                                                                                phi_22676_ = type_58(1u, _e2617);
                                                                            }
                                                                            let _e2620 = phi_22675_;
                                                                            let _e2622 = phi_22676_;
                                                                            phi_22680_ = _e2620;
                                                                            phi_22681_ = _e2622;
                                                                            phi_22682_ = _e2611;
                                                                        }
                                                                        let _e2627 = phi_22680_;
                                                                        let _e2629 = phi_22681_;
                                                                        let _e2631 = phi_22682_;
                                                                        if _e2631 {
                                                                            phi_22686_ = _e2627;
                                                                        } else {
                                                                            phi_22686_ = _e2629;
                                                                        }
                                                                        let _e2633 = phi_22686_;
                                                                        if (bitcast<i32>(_e2633.member) != 0i) {
                                                                            phi_22620_ = select(_e2633.member_1.member_1, _e2633.member_1.member, (_e2633.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22620_ = -1f;
                                                                        }
                                                                        let _e2644 = phi_22620_;
                                                                        if (_e2644 < 0f) {
                                                                            phi_22489_ = type_59(0u, type_59().member_1);
                                                                        } else {
                                                                            phi_22447_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_22448_ = 0u;
                                                                            loop {
                                                                                let _e2647 = phi_22447_;
                                                                                let _e2649 = phi_22448_;
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
                                                                                        phi_22818_ = true;
                                                                                    } else {
                                                                                        phi_22818_ = (0f >= _e2699);
                                                                                    }
                                                                                    let _e2703 = phi_22818_;
                                                                                    let _e2705 = -(_e2695);
                                                                                    phi_22481_ = (_e2647 + vec3<f32>((exp((_e2705 / _e1484)) * _e2663), (exp((_e2705 / _e1487)) * _e2663), (select(_e2699, 0f, _e2703) * _e2663)));
                                                                                    phi_22482_ = _e2653;
                                                                                } else {
                                                                                    phi_22481_ = vec3<f32>();
                                                                                    phi_22482_ = u32();
                                                                                }
                                                                                let _e2716 = phi_22481_;
                                                                                let _e2718 = phi_22482_;
                                                                                phi_22447_ = _e2716;
                                                                                phi_22448_ = _e2718;
                                                                                if !(_e2650) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2721 = local_11;
                                                                            phi_22489_ = type_59(1u, _e2721);
                                                                        }
                                                                        let _e2727 = phi_22489_;
                                                                        phi_22493_ = _e2727;
                                                                    }
                                                                    let _e2732 = phi_22493_;
                                                                    if (bitcast<i32>(_e2732.member) != 0i) {
                                                                        let _e2737 = (vec3<f32>(_e2377, _e2379, _e2381) + _e2732.member_1);
                                                                        let _e2743 = (_e2386 * _e2737.y);
                                                                        let _e2755 = exp(-((((_e1493 * _e2737.x) + _e2743) + (_e1509 * _e2737.z))));
                                                                        let _e2757 = exp(-((((_e1496 * _e2737.x) + _e2743) + (_e1512 * _e2737.z))));
                                                                        let _e2759 = exp(-((((_e1499 * _e2737.x) + _e2743) + (_e1515 * _e2737.z))));
                                                                        phi_3213_ = vec3<f32>((_e2217.x + (_e2755 * _e2371)), (_e2217.y + (_e2757 * _e2371)), (_e2217.z + (_e2759 * _e2371)));
                                                                        phi_3214_ = vec3<f32>((_e2219.x + (_e2755 * _e2368)), (_e2219.y + (_e2757 * _e2368)), (_e2219.z + (_e2759 * _e2368)));
                                                                    } else {
                                                                        phi_3213_ = _e2217;
                                                                        phi_3214_ = _e2219;
                                                                    }
                                                                    let _e2781 = phi_3213_;
                                                                    let _e2783 = phi_3214_;
                                                                    phi_2717_ = vec3<f32>((_e2377 + _e2373), (_e2379 + _e2374), (_e2381 + _e2375));
                                                                    phi_2720_ = vec3<f32>((_e2215.x + ((exp(-((((_e1493 * _e2377) + _e2387) + (_e1509 * _e2381)))) * ((_e1493 * _e2368) + _e2406)) * _e2535.x)), (_e2215.y + ((exp(-((((_e1496 * _e2377) + _e2387) + (_e1512 * _e2381)))) * ((_e1496 * _e2368) + _e2406)) * _e2535.y)), (_e2215.z + ((exp(-((((_e1499 * _e2377) + _e2387) + (_e1515 * _e2381)))) * ((_e1499 * _e2368) + _e2406)) * _e2535.z)));
                                                                    phi_2722_ = _e2781;
                                                                    phi_2724_ = _e2783;
                                                                    phi_2726_ = (_e2221 + 1u);
                                                                } else {
                                                                    phi_2717_ = vec3<f32>();
                                                                    phi_2720_ = vec3<f32>();
                                                                    phi_2722_ = vec3<f32>();
                                                                    phi_2724_ = vec3<f32>();
                                                                    phi_2726_ = u32();
                                                                }
                                                                let _e2790 = phi_2717_;
                                                                let _e2792 = phi_2720_;
                                                                let _e2794 = phi_2722_;
                                                                let _e2796 = phi_2724_;
                                                                let _e2798 = phi_2726_;
                                                                phi_2716_ = _e2790;
                                                                phi_2719_ = _e2792;
                                                                phi_2721_ = _e2794;
                                                                phi_2723_ = _e2796;
                                                                phi_2725_ = _e2798;
                                                                phi_32529_ = _e1444;
                                                                if !(_e2222) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2801 = phi_32529_;
                                                            phi_32619_ = _e2801;
                                                            if _e2801 {
                                                                break;
                                                            }
                                                            let _e2803 = local_12;
                                                            let _e2806 = local_13;
                                                            let _e2809 = local_14;
                                                            phi_32623_ = _e2801;
                                                            phi_3234_ = type_53((_e2803 * vec3<f32>(_e1493, _e1496, _e1499)), (_e2806 * _e1503), _e2809);
                                                        } else {
                                                            phi_32623_ = _e1444;
                                                            phi_3234_ = type_53(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2812 = phi_32623_;
                                                        let _e2814 = phi_3234_;
                                                        phi_32622_ = _e2812;
                                                        phi_3235_ = _e2814;
                                                    } else {
                                                        phi_32622_ = _e1444;
                                                        phi_3235_ = _e2047;
                                                    }
                                                    let _e2816 = phi_32622_;
                                                    let _e2818 = phi_3235_;
                                                    let _e2826 = (((_e335.member_1.x * _e1475) + (_e335.member_1.y * _e1478)) + (_e335.member_1.z * _e1481));
                                                    let _e2828 = (1f + (_e2826 * _e2826));
                                                    let _e2829 = (0.059683103f * _e2828);
                                                    let _e2836 = (_e1506 * _e1506);
                                                    let _e2847 = ((0.119366206f * ((1f - _e2836) * _e2828)) / ((2f + _e2836) * pow(((1f + _e2836) - ((2f * _e1506) * _e2826)), 1.5f)));
                                                    phi_32621_ = _e2816;
                                                    phi_3258_ = vec3<f32>((_e1446.x + (((((_e2818.member.x * _e2829) + (_e2818.member_1.x * _e2847)) + _e2818.member_2.x) * _e1490) * 0.38317f)), (_e1446.y + (((((_e2818.member.y * _e2829) + (_e2818.member_1.y * _e2847)) + _e2818.member_2.y) * _e1490) * 0.38317f)), (_e1446.z + (((((_e2818.member.z * _e2829) + (_e2818.member_1.z * _e2847)) + _e2818.member_2.z) * _e1490) * 0.38317f)));
                                                    phi_3259_ = (_e1448 + 1i);
                                                    phi_3260_ = bitcast<u32>(_e1526);
                                                } else {
                                                    phi_32621_ = _e1444;
                                                    phi_3258_ = vec3<f32>();
                                                    phi_3259_ = i32();
                                                    phi_3260_ = u32();
                                                }
                                                let _e2878 = phi_32621_;
                                                let _e2880 = phi_3258_;
                                                let _e2882 = phi_3259_;
                                                let _e2884 = phi_3260_;
                                                phi_32620_ = _e2878;
                                                phi_904_ = _e2880;
                                                phi_907_ = _e2882;
                                                phi_909_ = _e2884;
                                                phi_3261_ = _e1452;
                                            } else {
                                                phi_32620_ = _e1444;
                                                phi_904_ = vec3<f32>();
                                                phi_907_ = i32();
                                                phi_909_ = u32();
                                                phi_3261_ = false;
                                            }
                                            let _e2886 = phi_32620_;
                                            let _e2888 = phi_904_;
                                            let _e2890 = phi_907_;
                                            let _e2892 = phi_909_;
                                            let _e2894 = phi_3261_;
                                            phi_32561_ = _e2886;
                                            phi_903_ = _e2888;
                                            phi_906_ = _e2890;
                                            phi_908_ = _e2892;
                                            phi_32619_ = _e2886;
                                            if !(_e2894) {
                                                break;
                                            }
                                        }
                                        let _e2897 = phi_32619_;
                                        phi_32947_ = _e2897;
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
                                                phi_3595_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3598_ = 0u;
                                                loop {
                                                    let _e2987 = phi_3595_;
                                                    let _e2989 = phi_3598_;
                                                    local_15 = _e2987;
                                                    let _e2990 = (_e2989 < 6u);
                                                    if _e2990 {
                                                        let _e2991 = (_e2989 / 2u);
                                                        if (_e2991 == 0u) {
                                                            phi_22946_ = _e335.member_1.x;
                                                        } else {
                                                            phi_22946_ = select(_e335.member_1.z, _e335.member_1.y, (_e2991 == 1u));
                                                        }
                                                        let _e2999 = phi_22946_;
                                                        let _e3000 = (_e2999 * select(-1f, 1f, ((_e2989 % 2u) == 0u)));
                                                        if (_e3000 > 0f) {
                                                            let _e3004 = ((_e2991 + 1u) % 3u);
                                                            if (_e3004 == 0u) {
                                                                phi_22961_ = _e335.member_1.x;
                                                            } else {
                                                                phi_22961_ = select(_e335.member_1.z, _e335.member_1.y, (_e3004 == 1u));
                                                            }
                                                            let _e3009 = phi_22961_;
                                                            let _e3010 = (_e3009 / _e3000);
                                                            let _e3012 = ((_e2991 + 2u) % 3u);
                                                            if (_e3012 == 0u) {
                                                                phi_22976_ = _e335.member_1.x;
                                                            } else {
                                                                phi_22976_ = select(_e335.member_1.z, _e335.member_1.y, (_e3012 == 1u));
                                                            }
                                                            let _e3017 = phi_22976_;
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
                                                                        phi_32624_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3035 < 0i) {
                                                                        phi_3669_ = 0i;
                                                                    } else {
                                                                        phi_3669_ = select(_e3035, _e3037, (_e3035 > _e3037));
                                                                    }
                                                                    let _e3043 = phi_3669_;
                                                                    let _e3047 = floor((((_e3018 + 1f) * 0.5f) * _e3026));
                                                                    let _e3054 = select(0i, select(select(i32(_e3047), i32(-2147483648), (_e3047 < -2147483600f)), 2147483647i, (_e3047 > 2147483500f)), (_e3047 == _e3047));
                                                                    if _e3038 {
                                                                    } else {
                                                                        phi_32624_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3054 < 0i) {
                                                                        phi_3699_ = 0i;
                                                                    } else {
                                                                        phi_3699_ = select(_e3054, _e3037, (_e3054 > _e3037));
                                                                    }
                                                                    let _e3059 = phi_3699_;
                                                                    let _e3060 = (_e3043 - 1i);
                                                                    phi_3707_ = _e2987;
                                                                    phi_3710_ = select(0i, _e3060, (0i < _e3060));
                                                                    loop {
                                                                        let _e3064 = phi_3707_;
                                                                        let _e3066 = phi_3710_;
                                                                        let _e3067 = (_e3043 + 1i);
                                                                        local_59 = _e3064;
                                                                        let _e3070 = (_e3066 <= select(_e3067, _e3037, (_e3037 < _e3067)));
                                                                        if _e3070 {
                                                                            let _e3071 = (_e3059 - 1i);
                                                                            phi_3733_ = _e3064;
                                                                            phi_3736_ = select(0i, _e3071, (0i < _e3071));
                                                                            loop {
                                                                                let _e3075 = phi_3733_;
                                                                                let _e3077 = phi_3736_;
                                                                                let _e3078 = (_e3059 + 1i);
                                                                                local_58 = _e3075;
                                                                                let _e3081 = (_e3077 <= select(_e3078, _e3037, (_e3037 < _e3078)));
                                                                                if _e3081 {
                                                                                    let _e3090 = global_6.member[(bitcast<u32>(_e2981) + ((((_e2989 * _e2978) + bitcast<u32>(_e3066)) * _e2978) + bitcast<u32>(_e3077)))];
                                                                                    phi_3734_ = vec3<f32>((_e3075.x + _e3090), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3737_ = (_e3077 + 1i);
                                                                                } else {
                                                                                    phi_3734_ = vec3<f32>();
                                                                                    phi_3737_ = i32();
                                                                                }
                                                                                let _e3099 = phi_3734_;
                                                                                let _e3101 = phi_3737_;
                                                                                phi_3733_ = _e3099;
                                                                                phi_3736_ = _e3101;
                                                                                if !(_e3081) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6936 = local_58;
                                                                            phi_3708_ = _e6936;
                                                                            phi_3711_ = (_e3066 + 1i);
                                                                        } else {
                                                                            phi_3708_ = vec3<f32>();
                                                                            phi_3711_ = i32();
                                                                        }
                                                                        let _e3105 = phi_3708_;
                                                                        let _e3107 = phi_3711_;
                                                                        phi_3707_ = _e3105;
                                                                        phi_3710_ = _e3107;
                                                                        if !(_e3070) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6940 = local_59;
                                                                    phi_3935_ = _e6940;
                                                                } else {
                                                                    phi_3935_ = _e2987;
                                                                }
                                                                let _e3110 = phi_3935_;
                                                                phi_3936_ = _e3110;
                                                            } else {
                                                                phi_3936_ = _e2987;
                                                            }
                                                            let _e3112 = phi_3936_;
                                                            phi_3937_ = _e3112;
                                                        } else {
                                                            phi_3937_ = _e2987;
                                                        }
                                                        let _e3114 = phi_3937_;
                                                        phi_3596_ = _e3114;
                                                        phi_3599_ = (_e2989 + 1u);
                                                    } else {
                                                        phi_3596_ = vec3<f32>();
                                                        phi_3599_ = u32();
                                                    }
                                                    let _e3117 = phi_3596_;
                                                    let _e3119 = phi_3599_;
                                                    phi_3595_ = _e3117;
                                                    phi_3598_ = _e3119;
                                                    phi_32624_ = _e2897;
                                                    if !(_e2990) {
                                                        break;
                                                    }
                                                }
                                                let _e3122 = phi_32624_;
                                                phi_32947_ = _e3122;
                                                if _e3122 {
                                                    break;
                                                }
                                                let _e3124 = local_15;
                                                phi_32672_ = _e3122;
                                                phi_3948_ = (_e3124.x * (1f / ((6.2831855f * _e2974) * _e2974)));
                                            } else {
                                                phi_32672_ = _e2897;
                                                phi_3948_ = 0f;
                                            }
                                            let _e3128 = phi_32672_;
                                            let _e3130 = phi_3948_;
                                            if (_e2908 != 4294967295u) {
                                                let _e3147 = (_e2960 * 1.3333334f);
                                                let _e3154 = global_6.member[_e2908];
                                                phi_3972_ = (_e3130 + ((_e3154 * ((exp((-(abs(_e2960)) / (0.06f + (abs(_e2971) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2971 * -0.71428573f) * (_e2971 * 0.71428573f)))))) + (1.3f * exp((((_e2971 * _e2971) + (_e3147 * _e3147)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3972_ = _e3130;
                                            }
                                            let _e3160 = phi_3972_;
                                            phi_3974_ = 0u;
                                            phi_3977_ = _e3160;
                                            loop {
                                                let _e3162 = phi_3974_;
                                                let _e3164 = phi_3977_;
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
                                                            phi_4081_ = 1.8f;
                                                        } else {
                                                            if (_e3192 == 1u) {
                                                                phi_4080_ = 1f;
                                                            } else {
                                                                phi_4080_ = select(0f, 4.7f, (_e3192 == 2u));
                                                            }
                                                            let _e3198 = phi_4080_;
                                                            phi_4081_ = _e3198;
                                                        }
                                                        let _e3200 = phi_4081_;
                                                        let _e3203 = global_6.member[(_e3167 + 6u)];
                                                        phi_4097_ = (_e3164 + (_e3203 * _e3200));
                                                    } else {
                                                        phi_4097_ = _e3164;
                                                    }
                                                    let _e3207 = phi_4097_;
                                                    phi_3975_ = (_e3162 + 1u);
                                                    phi_3978_ = _e3207;
                                                } else {
                                                    phi_3975_ = u32();
                                                    phi_3978_ = f32();
                                                }
                                                let _e3210 = phi_3975_;
                                                let _e3212 = phi_3978_;
                                                phi_3974_ = _e3210;
                                                phi_3977_ = _e3212;
                                                if !(_e3165) {
                                                    break;
                                                }
                                            }
                                            phi_4100_ = 0u;
                                            let _e6960 = local_60;
                                            phi_4103_ = _e6960;
                                            loop {
                                                let _e3215 = phi_4100_;
                                                let _e3217 = phi_4103_;
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
                                                        phi_4211_ = (_e3217 + (_e3244 * select(1f, 3.3808f, (_e3240 < 0.5f))));
                                                    } else {
                                                        phi_4211_ = _e3217;
                                                    }
                                                    let _e3249 = phi_4211_;
                                                    phi_4101_ = (_e3215 + 1u);
                                                    phi_4104_ = _e3249;
                                                } else {
                                                    phi_4101_ = u32();
                                                    phi_4104_ = f32();
                                                }
                                                let _e3252 = phi_4101_;
                                                let _e3254 = phi_4104_;
                                                phi_4100_ = _e3252;
                                                phi_4103_ = _e3254;
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
                                                phi_32713_ = _e3128;
                                                phi_4770_ = vec3<f32>();
                                                phi_4771_ = bool();
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
                                                        phi_23120_ = type_58(0u, type_58().member_1);
                                                        phi_23121_ = type_58();
                                                        phi_23122_ = true;
                                                    } else {
                                                        if (_e3343 < 0f) {
                                                            phi_23092_ = -(sqrt(_e3345));
                                                        } else {
                                                            phi_23092_ = sqrt(_e3345);
                                                        }
                                                        let _e3352 = phi_23092_;
                                                        let _e3354 = -((_e3343 + _e3352));
                                                        let _e3355 = (_e3354 == 0f);
                                                        if _e3355 {
                                                            phi_23115_ = type_58(1u, type_57(0f, 0f));
                                                            phi_23116_ = type_58();
                                                        } else {
                                                            let _e3356 = (_e3334 / _e3354);
                                                            if (_e3354 < _e3356) {
                                                                phi_23104_ = type_57(_e3354, _e3356);
                                                            } else {
                                                                phi_23104_ = type_57(_e3356, _e3354);
                                                            }
                                                            let _e3361 = phi_23104_;
                                                            phi_23115_ = type_58();
                                                            phi_23116_ = type_58(1u, _e3361);
                                                        }
                                                        let _e3364 = phi_23115_;
                                                        let _e3366 = phi_23116_;
                                                        phi_23120_ = _e3364;
                                                        phi_23121_ = _e3366;
                                                        phi_23122_ = _e3355;
                                                    }
                                                    let _e3371 = phi_23120_;
                                                    let _e3373 = phi_23121_;
                                                    let _e3375 = phi_23122_;
                                                    if _e3375 {
                                                        phi_23126_ = _e3371;
                                                    } else {
                                                        phi_23126_ = _e3373;
                                                    }
                                                    let _e3377 = phi_23126_;
                                                    if (bitcast<i32>(_e3377.member) != 0i) {
                                                        phi_23060_ = select(_e3377.member_1.member_1, _e3377.member_1.member, (_e3377.member_1.member >= 0f));
                                                    } else {
                                                        phi_23060_ = -1f;
                                                    }
                                                    let _e3388 = phi_23060_;
                                                    phi_4612_ = select(true, false, (_e3388 >= 0f));
                                                } else {
                                                    phi_4612_ = true;
                                                }
                                                let _e3392 = phi_4612_;
                                                if _e3392 {
                                                    let _e3400 = ((((_e335.member.x - _e3279) * _e335.member_1.x) + ((_e335.member.y - _e3282) * _e335.member_1.y)) + ((_e335.member.z - _e3285) * _e335.member_1.z));
                                                    let _e3401 = (_e3333 + _e3294);
                                                    let _e3402 = (_e3400 * _e3400);
                                                    let _e3403 = (_e3402 - _e3401);
                                                    if (_e3403 < 0f) {
                                                        phi_23280_ = type_58(0u, type_58().member_1);
                                                        phi_23281_ = type_58();
                                                        phi_23282_ = true;
                                                    } else {
                                                        if (_e3400 < 0f) {
                                                            phi_23252_ = -(sqrt(_e3403));
                                                        } else {
                                                            phi_23252_ = sqrt(_e3403);
                                                        }
                                                        let _e3410 = phi_23252_;
                                                        let _e3412 = -((_e3400 + _e3410));
                                                        let _e3413 = (_e3412 == 0f);
                                                        if _e3413 {
                                                            phi_23275_ = type_58(1u, type_57(0f, 0f));
                                                            phi_23276_ = type_58();
                                                        } else {
                                                            let _e3414 = (_e3401 / _e3412);
                                                            if (_e3412 < _e3414) {
                                                                phi_23264_ = type_57(_e3412, _e3414);
                                                            } else {
                                                                phi_23264_ = type_57(_e3414, _e3412);
                                                            }
                                                            let _e3419 = phi_23264_;
                                                            phi_23275_ = type_58();
                                                            phi_23276_ = type_58(1u, _e3419);
                                                        }
                                                        let _e3422 = phi_23275_;
                                                        let _e3424 = phi_23276_;
                                                        phi_23280_ = _e3422;
                                                        phi_23281_ = _e3424;
                                                        phi_23282_ = _e3413;
                                                    }
                                                    let _e3429 = phi_23280_;
                                                    let _e3431 = phi_23281_;
                                                    let _e3433 = phi_23282_;
                                                    if _e3433 {
                                                        phi_23286_ = _e3429;
                                                    } else {
                                                        phi_23286_ = _e3431;
                                                    }
                                                    let _e3435 = phi_23286_;
                                                    if (bitcast<i32>(_e3435.member) != 0i) {
                                                        phi_23173_ = type_58(0u, _e3435.member_1);
                                                    } else {
                                                        phi_23173_ = type_58(1u, type_58().member_1);
                                                    }
                                                    let _e3445 = phi_23173_;
                                                    if (bitcast<i32>(_e3445.member) != 0i) {
                                                        phi_23220_ = type_58(0u, type_58().member_1);
                                                    } else {
                                                        if (_e3445.member_1.member_1 <= 0f) {
                                                            phi_23216_ = type_58(0u, type_58().member_1);
                                                        } else {
                                                            if (_e3445.member_1.member != _e3445.member_1.member) {
                                                                phi_23324_ = true;
                                                            } else {
                                                                phi_23324_ = (0f >= _e3445.member_1.member);
                                                            }
                                                            let _e3457 = phi_23324_;
                                                            let _e3458 = select(_e3445.member_1.member, 0f, _e3457);
                                                            if _e3335 {
                                                                let _e3459 = (_e3402 - _e3334);
                                                                if (_e3459 < 0f) {
                                                                    phi_23432_ = type_58(0u, type_58().member_1);
                                                                    phi_23433_ = type_58();
                                                                    phi_23434_ = true;
                                                                } else {
                                                                    if (_e3400 < 0f) {
                                                                        phi_23404_ = -(sqrt(_e3459));
                                                                    } else {
                                                                        phi_23404_ = sqrt(_e3459);
                                                                    }
                                                                    let _e3466 = phi_23404_;
                                                                    let _e3468 = -((_e3400 + _e3466));
                                                                    let _e3469 = (_e3468 == 0f);
                                                                    if _e3469 {
                                                                        phi_23427_ = type_58(1u, type_57(0f, 0f));
                                                                        phi_23428_ = type_58();
                                                                    } else {
                                                                        let _e3470 = (_e3334 / _e3468);
                                                                        if (_e3468 < _e3470) {
                                                                            phi_23416_ = type_57(_e3468, _e3470);
                                                                        } else {
                                                                            phi_23416_ = type_57(_e3470, _e3468);
                                                                        }
                                                                        let _e3475 = phi_23416_;
                                                                        phi_23427_ = type_58();
                                                                        phi_23428_ = type_58(1u, _e3475);
                                                                    }
                                                                    let _e3478 = phi_23427_;
                                                                    let _e3480 = phi_23428_;
                                                                    phi_23432_ = _e3478;
                                                                    phi_23433_ = _e3480;
                                                                    phi_23434_ = _e3469;
                                                                }
                                                                let _e3485 = phi_23432_;
                                                                let _e3487 = phi_23433_;
                                                                let _e3489 = phi_23434_;
                                                                if _e3489 {
                                                                    phi_23438_ = _e3485;
                                                                } else {
                                                                    phi_23438_ = _e3487;
                                                                }
                                                                let _e3491 = phi_23438_;
                                                                if (bitcast<i32>(_e3491.member) != 0i) {
                                                                    phi_23372_ = select(_e3491.member_1.member_1, _e3491.member_1.member, (_e3491.member_1.member >= 0f));
                                                                } else {
                                                                    phi_23372_ = -1f;
                                                                }
                                                                let _e3502 = phi_23372_;
                                                                if (_e3502 >= 0f) {
                                                                    if (_e3445.member_1.member_1 != _e3445.member_1.member_1) {
                                                                        phi_23476_ = true;
                                                                    } else {
                                                                        phi_23476_ = (_e3502 <= _e3445.member_1.member_1);
                                                                    }
                                                                    let _e3507 = phi_23476_;
                                                                    phi_23201_ = select(_e3445.member_1.member_1, _e3502, _e3507);
                                                                } else {
                                                                    phi_23201_ = _e3445.member_1.member_1;
                                                                }
                                                                let _e3510 = phi_23201_;
                                                                phi_23203_ = _e3510;
                                                            } else {
                                                                phi_23203_ = _e3445.member_1.member_1;
                                                            }
                                                            let _e3512 = phi_23203_;
                                                            if (_e3512 <= _e3458) {
                                                                phi_23212_ = type_58(0u, type_58().member_1);
                                                            } else {
                                                                phi_23212_ = type_58(1u, type_57(_e3458, _e3512));
                                                            }
                                                            let _e3520 = phi_23212_;
                                                            phi_23216_ = _e3520;
                                                        }
                                                        let _e3525 = phi_23216_;
                                                        phi_23220_ = _e3525;
                                                    }
                                                    let _e3530 = phi_23220_;
                                                    let _e3533 = (bitcast<i32>(_e3530.member) != 0i);
                                                    if _e3533 {
                                                        phi_4626_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4629_ = 0u;
                                                        loop {
                                                            let _e3539 = phi_4626_;
                                                            let _e3541 = phi_4629_;
                                                            local_20 = _e3539;
                                                            local_21 = _e3539;
                                                            local_22 = _e3539;
                                                            let _e3542 = (_e3541 < 32u);
                                                            if _e3542 {
                                                                let _e3543 = -(_e3400);
                                                                if (_e3530.member_1.member <= _e3530.member_1.member_1) {
                                                                } else {
                                                                    phi_32651_ = true;
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
                                                                    phi_4730_ = type_57((_e3530.member_1.member + ((_e3551 * _e3632) * _e3632)), (_e3551 * ((_e3630 * _e3630) - (_e3627 * _e3627))));
                                                                } else {
                                                                    if (_e3550 <= _e3552) {
                                                                        let _e3610 = (f32((31u - _e3541)) * 0.03125f);
                                                                        let _e3613 = (f32((32u - _e3541)) * 0.03125f);
                                                                        let _e3615 = ((_e3610 + _e3613) * 0.5f);
                                                                        phi_4729_ = type_57((_e3530.member_1.member_1 - ((_e3551 * _e3615) * _e3615)), (_e3551 * ((_e3613 * _e3613) - (_e3610 * _e3610))));
                                                                    } else {
                                                                        let _e3558 = round(((32f * _e3549) / (_e3549 + _e3550)));
                                                                        let _e3563 = select(select(u32(_e3558), 0u, (_e3558 < 0f)), 4294967295u, (_e3558 > 4294967000f));
                                                                        if (_e3563 < 1u) {
                                                                            phi_4713_ = 1u;
                                                                        } else {
                                                                            phi_4713_ = select(_e3563, 31u, (_e3563 > 31u));
                                                                        }
                                                                        let _e3568 = phi_4713_;
                                                                        if (_e3541 < _e3568) {
                                                                            let _e3589 = ((_e3568 - 1u) - _e3541);
                                                                            let _e3591 = f32(_e3568);
                                                                            let _e3592 = (f32(_e3589) / _e3591);
                                                                            let _e3595 = (f32((_e3589 + 1u)) / _e3591);
                                                                            let _e3597 = ((_e3592 + _e3595) * 0.5f);
                                                                            phi_4728_ = type_57((_e3548 - ((_e3549 * _e3597) * _e3597)), (_e3549 * ((_e3595 * _e3595) - (_e3592 * _e3592))));
                                                                        } else {
                                                                            let _e3571 = (_e3541 - _e3568);
                                                                            let _e3573 = f32((32u - _e3568));
                                                                            let _e3574 = (f32(_e3571) / _e3573);
                                                                            let _e3577 = (f32((_e3571 + 1u)) / _e3573);
                                                                            let _e3579 = ((_e3574 + _e3577) * 0.5f);
                                                                            phi_4728_ = type_57((_e3548 + ((_e3550 * _e3579) * _e3579)), (_e3550 * ((_e3577 * _e3577) - (_e3574 * _e3574))));
                                                                        }
                                                                        let _e3607 = phi_4728_;
                                                                        phi_4729_ = _e3607;
                                                                    }
                                                                    let _e3625 = phi_4729_;
                                                                    phi_4730_ = _e3625;
                                                                }
                                                                let _e3642 = phi_4730_;
                                                                let _e3648 = (_e335.member.x + (_e335.member_1.x * _e3642.member));
                                                                let _e3649 = (_e335.member.y + (_e335.member_1.y * _e3642.member));
                                                                let _e3650 = (_e335.member.z + (_e335.member_1.z * _e3642.member));
                                                                let _e3664 = (_e3648 - _e3279);
                                                                let _e3665 = (_e3649 - _e3282);
                                                                let _e3666 = (_e3650 - _e3285);
                                                                let _e3674 = ((((((_e3648 * _e3648) + (_e3649 * _e3649)) + (_e3650 * _e3650)) - (2f * (((_e3648 * _e3279) + (_e3649 * _e3282)) + (_e3650 * _e3285)))) + _e3291) / (sqrt((((_e3664 * _e3664) + (_e3665 * _e3665)) + (_e3666 * _e3666))) + _e3288));
                                                                let _e3678 = (1f - (abs((_e3674 - 25000f)) * 0.00006666667f));
                                                                if (_e3678 != _e3678) {
                                                                    phi_23675_ = true;
                                                                } else {
                                                                    phi_23675_ = (0f >= _e3678);
                                                                }
                                                                let _e3682 = phi_23675_;
                                                                let _e3684 = -(_e3674);
                                                                phi_4627_ = (_e3539 + vec3<f32>((exp((_e3684 / _e3297)) * _e3642.member_1), (exp((_e3684 / _e3300)) * _e3642.member_1), (select(_e3678, 0f, _e3682) * _e3642.member_1)));
                                                                phi_4630_ = (_e3541 + 1u);
                                                            } else {
                                                                phi_4627_ = vec3<f32>();
                                                                phi_4630_ = u32();
                                                            }
                                                            let _e3696 = phi_4627_;
                                                            let _e3698 = phi_4630_;
                                                            phi_4626_ = _e3696;
                                                            phi_4629_ = _e3698;
                                                            phi_32651_ = _e3128;
                                                            if !(_e3542) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3701 = phi_32651_;
                                                        phi_32947_ = _e3701;
                                                        if _e3701 {
                                                            break;
                                                        }
                                                        let _e3703 = local_20;
                                                        let _e3710 = local_21;
                                                        let _e3712 = ((_e3312 * 1.1f) * _e3710.y);
                                                        let _e3717 = local_22;
                                                        phi_32718_ = _e3701;
                                                        phi_4755_ = vec3<f32>(exp(-((((_e3303 * _e3703.x) + _e3712) + (_e3315 * _e3717.z)))), exp(-((((_e3306 * _e3703.x) + _e3712) + (_e3318 * _e3717.z)))), exp(-((((_e3309 * _e3703.x) + _e3712) + (_e3321 * _e3717.z)))));
                                                    } else {
                                                        phi_32718_ = _e3128;
                                                        phi_4755_ = vec3<f32>();
                                                    }
                                                    let _e3733 = phi_32718_;
                                                    let _e3735 = phi_4755_;
                                                    phi_32717_ = _e3733;
                                                    phi_4757_ = _e3735;
                                                    phi_4759_ = select(true, false, _e3533);
                                                } else {
                                                    phi_32717_ = _e3128;
                                                    phi_4757_ = vec3<f32>();
                                                    phi_4759_ = true;
                                                }
                                                let _e3738 = phi_32717_;
                                                let _e3740 = phi_4757_;
                                                let _e3742 = phi_4759_;
                                                let _e3746 = select(_e3740, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3392)), vec3(_e3742));
                                                let _e3750 = select(_e3746.y, _e3746.x, (_e3746.x > _e3746.y));
                                                phi_32713_ = _e3738;
                                                phi_4770_ = _e3746;
                                                phi_4771_ = (select(_e3746.z, _e3750, (_e3750 > _e3746.z)) > 0f);
                                            }
                                            let _e3756 = phi_32713_;
                                            let _e3758 = phi_4770_;
                                            let _e3760 = phi_4771_;
                                            phi_32712_ = _e3756;
                                            phi_4773_ = _e3758;
                                            phi_4774_ = _e3760;
                                            phi_4775_ = select(true, false, _e3277);
                                        } else {
                                            phi_32712_ = _e2897;
                                            phi_4773_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4774_ = _e1441;
                                            phi_4775_ = true;
                                        }
                                        let _e3763 = phi_32712_;
                                        let _e3765 = phi_4773_;
                                        let _e3767 = phi_4774_;
                                        let _e3769 = phi_4775_;
                                        phi_32711_ = _e3763;
                                        let _e7031 = local_61;
                                        phi_4779_ = _e7031;
                                        phi_4780_ = _e3765;
                                        phi_4781_ = _e3767;
                                        let _e7036 = local_62;
                                        phi_4782_ = _e7036;
                                        phi_4783_ = _e3769;
                                    } else {
                                        phi_32711_ = _e324;
                                        phi_4779_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4780_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4781_ = _e1441;
                                        phi_4782_ = vec3<f32>();
                                        phi_4783_ = true;
                                    }
                                    let _e3771 = phi_32711_;
                                    let _e3773 = phi_4779_;
                                    let _e3775 = phi_4780_;
                                    let _e3777 = phi_4781_;
                                    let _e3779 = phi_4782_;
                                    let _e3781 = phi_4783_;
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
                                                phi_5103_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5106_ = 0u;
                                                loop {
                                                    let _e3874 = phi_5103_;
                                                    let _e3876 = phi_5106_;
                                                    local_66 = _e3874;
                                                    let _e3877 = (_e3876 < 6u);
                                                    if _e3877 {
                                                        let _e3878 = (_e3876 / 2u);
                                                        if (_e3878 == 0u) {
                                                            phi_23746_ = _e335.member_1.x;
                                                        } else {
                                                            phi_23746_ = select(_e335.member_1.z, _e335.member_1.y, (_e3878 == 1u));
                                                        }
                                                        let _e3886 = phi_23746_;
                                                        let _e3887 = (_e3886 * select(-1f, 1f, ((_e3876 % 2u) == 0u)));
                                                        if (_e3887 > 0f) {
                                                            let _e3891 = ((_e3878 + 1u) % 3u);
                                                            if (_e3891 == 0u) {
                                                                phi_23761_ = _e335.member_1.x;
                                                            } else {
                                                                phi_23761_ = select(_e335.member_1.z, _e335.member_1.y, (_e3891 == 1u));
                                                            }
                                                            let _e3896 = phi_23761_;
                                                            let _e3897 = (_e3896 / _e3887);
                                                            let _e3899 = ((_e3878 + 2u) % 3u);
                                                            if (_e3899 == 0u) {
                                                                phi_23776_ = _e335.member_1.x;
                                                            } else {
                                                                phi_23776_ = select(_e335.member_1.z, _e335.member_1.y, (_e3899 == 1u));
                                                            }
                                                            let _e3904 = phi_23776_;
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
                                                                        phi_32706_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3922 < 0i) {
                                                                        phi_5177_ = 0i;
                                                                    } else {
                                                                        phi_5177_ = select(_e3922, _e3924, (_e3922 > _e3924));
                                                                    }
                                                                    let _e3930 = phi_5177_;
                                                                    let _e3934 = floor((((_e3905 + 1f) * 0.5f) * _e3913));
                                                                    let _e3941 = select(0i, select(select(i32(_e3934), i32(-2147483648), (_e3934 < -2147483600f)), 2147483647i, (_e3934 > 2147483500f)), (_e3934 == _e3934));
                                                                    if _e3925 {
                                                                    } else {
                                                                        phi_32706_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3941 < 0i) {
                                                                        phi_5207_ = 0i;
                                                                    } else {
                                                                        phi_5207_ = select(_e3941, _e3924, (_e3941 > _e3924));
                                                                    }
                                                                    let _e3946 = phi_5207_;
                                                                    let _e3947 = (_e3930 - 1i);
                                                                    phi_5215_ = _e3874;
                                                                    phi_5218_ = select(0i, _e3947, (0i < _e3947));
                                                                    loop {
                                                                        let _e3951 = phi_5215_;
                                                                        let _e3953 = phi_5218_;
                                                                        let _e3954 = (_e3930 + 1i);
                                                                        local_65 = _e3951;
                                                                        let _e3957 = (_e3953 <= select(_e3954, _e3924, (_e3924 < _e3954)));
                                                                        if _e3957 {
                                                                            let _e3958 = (_e3946 - 1i);
                                                                            phi_5241_ = _e3951;
                                                                            phi_5244_ = select(0i, _e3958, (0i < _e3958));
                                                                            loop {
                                                                                let _e3962 = phi_5241_;
                                                                                let _e3964 = phi_5244_;
                                                                                let _e3965 = (_e3946 + 1i);
                                                                                local_64 = _e3962;
                                                                                let _e3968 = (_e3964 <= select(_e3965, _e3924, (_e3924 < _e3965)));
                                                                                if _e3968 {
                                                                                    let _e3975 = (bitcast<u32>(_e3863) + ((((_e3876 * _e3860) + bitcast<u32>(_e3953)) * _e3860) + bitcast<u32>(_e3964)));
                                                                                    let _e3977 = global_6.member[_e3975];
                                                                                    let _e3981 = global_6.member[(_e3975 + 1u)];
                                                                                    phi_5315_ = _e3962;
                                                                                    phi_5318_ = bitcast<u32>(_e3977);
                                                                                    loop {
                                                                                        let _e3984 = phi_5315_;
                                                                                        let _e3986 = phi_5318_;
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
                                                                                                phi_5438_ = vec3<f32>((_e3984.x + (_e4000 * _e4022)), (_e3984.y + (_e4003 * _e4022)), (_e3984.z + (_e4006 * _e4022)));
                                                                                            } else {
                                                                                                phi_5438_ = _e3984;
                                                                                            }
                                                                                            let _e4034 = phi_5438_;
                                                                                            phi_5316_ = _e4034;
                                                                                            phi_5319_ = (_e3986 + 1u);
                                                                                        } else {
                                                                                            phi_5316_ = vec3<f32>();
                                                                                            phi_5319_ = u32();
                                                                                        }
                                                                                        let _e4037 = phi_5316_;
                                                                                        let _e4039 = phi_5319_;
                                                                                        phi_5315_ = _e4037;
                                                                                        phi_5318_ = _e4039;
                                                                                        if !(_e3987) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7056 = local_63;
                                                                                    phi_5242_ = _e7056;
                                                                                    phi_5245_ = (_e3964 + 1i);
                                                                                } else {
                                                                                    phi_5242_ = vec3<f32>();
                                                                                    phi_5245_ = i32();
                                                                                }
                                                                                let _e4043 = phi_5242_;
                                                                                let _e4045 = phi_5245_;
                                                                                phi_5241_ = _e4043;
                                                                                phi_5244_ = _e4045;
                                                                                if !(_e3968) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7060 = local_64;
                                                                            phi_5216_ = _e7060;
                                                                            phi_5219_ = (_e3953 + 1i);
                                                                        } else {
                                                                            phi_5216_ = vec3<f32>();
                                                                            phi_5219_ = i32();
                                                                        }
                                                                        let _e4049 = phi_5216_;
                                                                        let _e4051 = phi_5219_;
                                                                        phi_5215_ = _e4049;
                                                                        phi_5218_ = _e4051;
                                                                        if !(_e3957) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7064 = local_65;
                                                                    phi_5443_ = _e7064;
                                                                } else {
                                                                    phi_5443_ = _e3874;
                                                                }
                                                                let _e4054 = phi_5443_;
                                                                phi_5444_ = _e4054;
                                                            } else {
                                                                phi_5444_ = _e3874;
                                                            }
                                                            let _e4056 = phi_5444_;
                                                            phi_5445_ = _e4056;
                                                        } else {
                                                            phi_5445_ = _e3874;
                                                        }
                                                        let _e4058 = phi_5445_;
                                                        phi_5104_ = _e4058;
                                                        phi_5107_ = (_e3876 + 1u);
                                                    } else {
                                                        phi_5104_ = vec3<f32>();
                                                        phi_5107_ = u32();
                                                    }
                                                    let _e4061 = phi_5104_;
                                                    let _e4063 = phi_5107_;
                                                    phi_5103_ = _e4061;
                                                    phi_5106_ = _e4063;
                                                    phi_32706_ = _e3771;
                                                    if !(_e3877) {
                                                        break;
                                                    }
                                                }
                                                let _e4066 = phi_32706_;
                                                phi_32947_ = _e4066;
                                                if _e4066 {
                                                    break;
                                                }
                                                phi_32748_ = _e4066;
                                                let _e7073 = local_66;
                                                phi_5465_ = _e7073;
                                            } else {
                                                phi_32748_ = _e3771;
                                                phi_5465_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4068 = phi_32748_;
                                            let _e4070 = phi_5465_;
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
                                                    phi_5678_ = _e4120;
                                                    phi_5679_ = _e4119;
                                                    phi_5682_ = _e4120;
                                                    phi_5683_ = _e4119;
                                                    phi_5684_ = select(true, false, _e4119);
                                                } else {
                                                    phi_5678_ = u32();
                                                    phi_5679_ = bool();
                                                    phi_5682_ = select(2u, 0u, _e4115);
                                                    phi_5683_ = false;
                                                    phi_5684_ = true;
                                                }
                                                let _e4123 = phi_5678_;
                                                let _e4125 = phi_5679_;
                                                let _e4127 = phi_5682_;
                                                let _e4129 = phi_5683_;
                                                let _e4131 = phi_5684_;
                                                let _e4133 = select(select(f32(), 0f, _e4116), 0f, _e4131);
                                                phi_5694_ = _e4133;
                                                phi_5697_ = _e4133;
                                                phi_5699_ = select(_e4123, _e4127, _e4131);
                                                phi_5701_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5703_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5705_ = 0f;
                                                phi_5707_ = 1f;
                                                loop {
                                                    let _e4138 = phi_5694_;
                                                    let _e4140 = phi_5697_;
                                                    let _e4142 = phi_5699_;
                                                    let _e4144 = phi_5701_;
                                                    let _e4146 = phi_5703_;
                                                    let _e4148 = phi_5705_;
                                                    let _e4150 = phi_5707_;
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
                                                            phi_5785_ = 4u;
                                                            phi_5787_ = vec3<f32>((_e335.member_1.x * _e4208), (_e335.member_1.y * _e4208), (_e335.member_1.z * _e4208));
                                                        } else {
                                                            if (_e4142 == 2u) {
                                                                phi_5783_ = 3u;
                                                                phi_5784_ = vec3<f32>(((_e335.member_1.x * 1.5f) + 5f), ((_e335.member_1.y * 1.5f) + 5f), ((_e335.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4154 = (_e4142 >= 7u);
                                                                if _e4154 {
                                                                    phi_5745_ = (_e4142 - 7u);
                                                                } else {
                                                                    phi_5745_ = (_e4142 - 3u);
                                                                }
                                                                let _e4158 = phi_5745_;
                                                                if _e4154 {
                                                                    phi_5760_ = vec3<f32>(((_e335.member_1.x * 11f) + -3f), ((_e335.member_1.y * 11f) + 1f), ((_e335.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5760_ = vec3<f32>((_e335.member_1.x * 7f), (_e335.member_1.y * 7f), (_e335.member_1.z * 7f));
                                                                }
                                                                let _e4171 = phi_5760_;
                                                                if (_e4158 < 3u) {
                                                                    if (_e4158 == 0u) {
                                                                        phi_23791_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_23791_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4158 == 1u)));
                                                                    }
                                                                    let _e4190 = phi_23791_;
                                                                    phi_5781_ = 4u;
                                                                    phi_5782_ = (_e4171 + _e4190);
                                                                } else {
                                                                    if _e4154 {
                                                                        phi_5780_ = (_e4171 + _e4144);
                                                                    } else {
                                                                        phi_5780_ = (_e4171 + vec3<f32>((_e4146.x * 1.1f), (_e4146.y * 1.1f), (_e4146.z * 1.1f)));
                                                                    }
                                                                    let _e4183 = phi_5780_;
                                                                    phi_5781_ = select(7u, 5u, _e4154);
                                                                    phi_5782_ = _e4183;
                                                                }
                                                                let _e4193 = phi_5781_;
                                                                let _e4195 = phi_5782_;
                                                                phi_5783_ = _e4193;
                                                                phi_5784_ = _e4195;
                                                            }
                                                            let _e4204 = phi_5783_;
                                                            let _e4206 = phi_5784_;
                                                            phi_5785_ = _e4204;
                                                            phi_5787_ = _e4206;
                                                        }
                                                        let _e4214 = phi_5785_;
                                                        let _e4216 = phi_5787_;
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
                                                        phi_5922_ = 0u;
                                                        phi_5925_ = 0f;
                                                        phi_5927_ = 1f;
                                                        phi_5929_ = 0f;
                                                        phi_5931_ = _e4216;
                                                        loop {
                                                            let _e4250 = phi_5922_;
                                                            let _e4252 = phi_5925_;
                                                            let _e4254 = phi_5927_;
                                                            let _e4256 = phi_5929_;
                                                            let _e4258 = phi_5931_;
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
                                                                    phi_23852_ = ((_e4391 * _e4299) + (_e4394 * _e4300));
                                                                } else {
                                                                    if (_e4388 < 8u) {
                                                                        phi_23846_ = ((_e4391 * _e4299) + (_e4394 * _e4301));
                                                                    } else {
                                                                        phi_23846_ = ((_e4391 * _e4300) + (_e4394 * _e4301));
                                                                    }
                                                                    let _e4404 = phi_23846_;
                                                                    phi_23852_ = _e4404;
                                                                }
                                                                let _e4409 = phi_23852_;
                                                                let _e4412 = global_6.member[(_e4221 + _e4369)];
                                                                let _e4418 = (_e4299 - 1f);
                                                                let _e4419 = (select(select(u32(_e4412), 0u, (_e4412 < 0f)), 4294967295u, (_e4412 > 4294967000f)) % 12u);
                                                                let _e4422 = select(-1f, 1f, ((_e4419 & 1u) == 0u));
                                                                let _e4425 = select(-1f, 1f, ((_e4419 & 2u) == 0u));
                                                                if (_e4419 < 4u) {
                                                                    phi_23886_ = ((_e4422 * _e4418) + (_e4425 * _e4300));
                                                                } else {
                                                                    if (_e4419 < 8u) {
                                                                        phi_23880_ = ((_e4422 * _e4418) + (_e4425 * _e4301));
                                                                    } else {
                                                                        phi_23880_ = ((_e4422 * _e4300) + (_e4425 * _e4301));
                                                                    }
                                                                    let _e4435 = phi_23880_;
                                                                    phi_23886_ = _e4435;
                                                                }
                                                                let _e4440 = phi_23886_;
                                                                let _e4443 = global_6.member[(_e4221 + _e4350)];
                                                                let _e4449 = (_e4300 - 1f);
                                                                let _e4450 = (select(select(u32(_e4443), 0u, (_e4443 < 0f)), 4294967295u, (_e4443 > 4294967000f)) % 12u);
                                                                let _e4453 = select(-1f, 1f, ((_e4450 & 1u) == 0u));
                                                                let _e4456 = select(-1f, 1f, ((_e4450 & 2u) == 0u));
                                                                if (_e4450 < 4u) {
                                                                    phi_23920_ = ((_e4453 * _e4299) + (_e4456 * _e4449));
                                                                } else {
                                                                    if (_e4450 < 8u) {
                                                                        phi_23914_ = ((_e4453 * _e4299) + (_e4456 * _e4301));
                                                                    } else {
                                                                        phi_23914_ = ((_e4453 * _e4449) + (_e4456 * _e4301));
                                                                    }
                                                                    let _e4466 = phi_23914_;
                                                                    phi_23920_ = _e4466;
                                                                }
                                                                let _e4471 = phi_23920_;
                                                                let _e4474 = global_6.member[(_e4221 + _e4379)];
                                                                let _e4480 = (select(select(u32(_e4474), 0u, (_e4474 < 0f)), 4294967295u, (_e4474 > 4294967000f)) % 12u);
                                                                let _e4483 = select(-1f, 1f, ((_e4480 & 1u) == 0u));
                                                                let _e4486 = select(-1f, 1f, ((_e4480 & 2u) == 0u));
                                                                if (_e4480 < 4u) {
                                                                    phi_23954_ = ((_e4483 * _e4418) + (_e4486 * _e4449));
                                                                } else {
                                                                    if (_e4480 < 8u) {
                                                                        phi_23948_ = ((_e4483 * _e4418) + (_e4486 * _e4301));
                                                                    } else {
                                                                        phi_23948_ = ((_e4483 * _e4449) + (_e4486 * _e4301));
                                                                    }
                                                                    let _e4496 = phi_23948_;
                                                                    phi_23954_ = _e4496;
                                                                }
                                                                let _e4501 = phi_23954_;
                                                                let _e4505 = global_6.member[(_e4221 + (_e4340 + 1u))];
                                                                let _e4511 = (_e4301 - 1f);
                                                                let _e4512 = (select(select(u32(_e4505), 0u, (_e4505 < 0f)), 4294967295u, (_e4505 > 4294967000f)) % 12u);
                                                                let _e4515 = select(-1f, 1f, ((_e4512 & 1u) == 0u));
                                                                let _e4518 = select(-1f, 1f, ((_e4512 & 2u) == 0u));
                                                                if (_e4512 < 4u) {
                                                                    phi_23988_ = ((_e4515 * _e4299) + (_e4518 * _e4300));
                                                                } else {
                                                                    if (_e4512 < 8u) {
                                                                        phi_23982_ = ((_e4515 * _e4299) + (_e4518 * _e4511));
                                                                    } else {
                                                                        phi_23982_ = ((_e4515 * _e4300) + (_e4518 * _e4511));
                                                                    }
                                                                    let _e4528 = phi_23982_;
                                                                    phi_23988_ = _e4528;
                                                                }
                                                                let _e4533 = phi_23988_;
                                                                let _e4537 = global_6.member[(_e4221 + (_e4369 + 1u))];
                                                                let _e4543 = (select(select(u32(_e4537), 0u, (_e4537 < 0f)), 4294967295u, (_e4537 > 4294967000f)) % 12u);
                                                                let _e4546 = select(-1f, 1f, ((_e4543 & 1u) == 0u));
                                                                let _e4549 = select(-1f, 1f, ((_e4543 & 2u) == 0u));
                                                                if (_e4543 < 4u) {
                                                                    phi_24022_ = ((_e4546 * _e4418) + (_e4549 * _e4300));
                                                                } else {
                                                                    if (_e4543 < 8u) {
                                                                        phi_24016_ = ((_e4546 * _e4418) + (_e4549 * _e4511));
                                                                    } else {
                                                                        phi_24016_ = ((_e4546 * _e4300) + (_e4549 * _e4511));
                                                                    }
                                                                    let _e4559 = phi_24016_;
                                                                    phi_24022_ = _e4559;
                                                                }
                                                                let _e4564 = phi_24022_;
                                                                let _e4568 = global_6.member[(_e4221 + (_e4350 + 1u))];
                                                                let _e4574 = (select(select(u32(_e4568), 0u, (_e4568 < 0f)), 4294967295u, (_e4568 > 4294967000f)) % 12u);
                                                                let _e4577 = select(-1f, 1f, ((_e4574 & 1u) == 0u));
                                                                let _e4580 = select(-1f, 1f, ((_e4574 & 2u) == 0u));
                                                                if (_e4574 < 4u) {
                                                                    phi_24056_ = ((_e4577 * _e4299) + (_e4580 * _e4449));
                                                                } else {
                                                                    if (_e4574 < 8u) {
                                                                        phi_24050_ = ((_e4577 * _e4299) + (_e4580 * _e4511));
                                                                    } else {
                                                                        phi_24050_ = ((_e4577 * _e4449) + (_e4580 * _e4511));
                                                                    }
                                                                    let _e4590 = phi_24050_;
                                                                    phi_24056_ = _e4590;
                                                                }
                                                                let _e4595 = phi_24056_;
                                                                let _e4599 = global_6.member[(_e4221 + (_e4379 + 1u))];
                                                                let _e4605 = (select(select(u32(_e4599), 0u, (_e4599 < 0f)), 4294967295u, (_e4599 > 4294967000f)) % 12u);
                                                                let _e4608 = select(-1f, 1f, ((_e4605 & 1u) == 0u));
                                                                let _e4611 = select(-1f, 1f, ((_e4605 & 2u) == 0u));
                                                                if (_e4605 < 4u) {
                                                                    phi_24090_ = ((_e4608 * _e4418) + (_e4611 * _e4449));
                                                                } else {
                                                                    if (_e4605 < 8u) {
                                                                        phi_24084_ = ((_e4608 * _e4418) + (_e4611 * _e4511));
                                                                    } else {
                                                                        phi_24084_ = ((_e4608 * _e4449) + (_e4611 * _e4511));
                                                                    }
                                                                    let _e4621 = phi_24084_;
                                                                    phi_24090_ = _e4621;
                                                                }
                                                                let _e4626 = phi_24090_;
                                                                let _e4629 = (_e4409 + (_e4308 * (_e4440 - _e4409)));
                                                                let _e4635 = (_e4533 + (_e4308 * (_e4564 - _e4533)));
                                                                let _e4641 = (_e4629 + (_e4315 * ((_e4471 + (_e4308 * (_e4501 - _e4471))) - _e4629)));
                                                                phi_5923_ = (_e4250 + 1u);
                                                                phi_5926_ = (_e4252 + _e4254);
                                                                phi_5928_ = (_e4254 * 0.5f);
                                                                phi_5930_ = (_e4256 + ((_e4641 + ((((_e4301 * _e4301) * _e4301) * ((_e4301 * ((_e4301 * 6f) - 15f)) + 10f)) * ((_e4635 + (_e4315 * ((_e4595 + (_e4308 * (_e4626 - _e4595))) - _e4635))) - _e4641))) * _e4254));
                                                                phi_5932_ = vec3<f32>(((((_e4224 * _e4258.x) + (_e4227 * _e4258.y)) + (_e4230 * _e4258.z)) * 2.03f), ((((_e4233 * _e4258.x) + (_e4236 * _e4258.y)) + (_e4239 * _e4258.z)) * 2.03f), ((((_e4242 * _e4258.x) + (_e4245 * _e4258.y)) + (_e4248 * _e4258.z)) * 2.03f));
                                                            } else {
                                                                phi_5923_ = u32();
                                                                phi_5926_ = f32();
                                                                phi_5928_ = f32();
                                                                phi_5930_ = f32();
                                                                phi_5932_ = vec3<f32>();
                                                            }
                                                            let _e4673 = phi_5923_;
                                                            let _e4675 = phi_5926_;
                                                            let _e4677 = phi_5928_;
                                                            let _e4679 = phi_5930_;
                                                            let _e4681 = phi_5932_;
                                                            phi_5922_ = _e4673;
                                                            phi_5925_ = _e4675;
                                                            phi_5927_ = _e4677;
                                                            phi_5929_ = _e4679;
                                                            phi_5931_ = _e4681;
                                                            if !(_e4259) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4684 = local_23;
                                                        if (_e4684 > 0f) {
                                                            let _e4687 = local_24;
                                                            let _e4689 = local_25;
                                                            phi_6348_ = (_e4687 / _e4689);
                                                        } else {
                                                            phi_6348_ = 0f;
                                                        }
                                                        let _e4692 = phi_6348_;
                                                        if (_e4142 == 0u) {
                                                            phi_6431_ = _e4138;
                                                            phi_6432_ = _e4140;
                                                            phi_6433_ = _e4144;
                                                            phi_6434_ = _e4146;
                                                            phi_6435_ = _e4148;
                                                            phi_6436_ = (_e4150 + (1.5f * _e4692));
                                                        } else {
                                                            if (_e4142 == 1u) {
                                                                phi_6425_ = _e4138;
                                                                phi_6426_ = _e4140;
                                                                phi_6427_ = _e4144;
                                                                phi_6428_ = _e4146;
                                                                phi_6429_ = _e4148;
                                                                phi_6430_ = (_e4150 + (0.8f * _e4692));
                                                            } else {
                                                                let _e4695 = (_e4142 == 2u);
                                                                if _e4695 {
                                                                    phi_6420_ = _e4138;
                                                                    phi_6421_ = _e4140;
                                                                    phi_6422_ = _e4144;
                                                                    phi_6423_ = _e4146;
                                                                } else {
                                                                    if (_e4142 == 3u) {
                                                                        phi_6416_ = _e4138;
                                                                        phi_6417_ = _e4140;
                                                                        phi_6418_ = _e4144;
                                                                        phi_6419_ = vec3<f32>(_e4692, _e4146.y, _e4146.z);
                                                                    } else {
                                                                        if (_e4142 == 4u) {
                                                                            phi_6412_ = _e4138;
                                                                            phi_6413_ = _e4140;
                                                                            phi_6414_ = _e4144;
                                                                            phi_6415_ = vec3<f32>(_e4146.x, _e4692, _e4146.z);
                                                                        } else {
                                                                            if (_e4142 == 5u) {
                                                                                phi_6408_ = _e4138;
                                                                                phi_6409_ = _e4140;
                                                                                phi_6410_ = _e4144;
                                                                                phi_6411_ = vec3<f32>(_e4146.x, _e4146.y, _e4692);
                                                                            } else {
                                                                                let _e4699 = (_e4142 == 6u);
                                                                                if _e4699 {
                                                                                    phi_6405_ = _e4138;
                                                                                    phi_6407_ = _e4144;
                                                                                } else {
                                                                                    if (_e4142 == 7u) {
                                                                                        phi_6403_ = _e4138;
                                                                                        phi_6404_ = vec3<f32>(_e4692, _e4144.y, _e4144.z);
                                                                                    } else {
                                                                                        if (_e4142 == 8u) {
                                                                                            phi_6401_ = _e4138;
                                                                                            phi_6402_ = vec3<f32>(_e4144.x, _e4692, _e4144.z);
                                                                                        } else {
                                                                                            let _e4702 = (_e4142 == 9u);
                                                                                            if _e4702 {
                                                                                                phi_6400_ = vec3<f32>(_e4144.x, _e4144.y, _e4692);
                                                                                            } else {
                                                                                                phi_6400_ = _e4144;
                                                                                            }
                                                                                            let _e4708 = phi_6400_;
                                                                                            phi_6401_ = select(_e4692, _e4138, _e4702);
                                                                                            phi_6402_ = _e4708;
                                                                                        }
                                                                                        let _e4715 = phi_6401_;
                                                                                        let _e4717 = phi_6402_;
                                                                                        phi_6403_ = _e4715;
                                                                                        phi_6404_ = _e4717;
                                                                                    }
                                                                                    let _e4723 = phi_6403_;
                                                                                    let _e4725 = phi_6404_;
                                                                                    phi_6405_ = _e4723;
                                                                                    phi_6407_ = _e4725;
                                                                                }
                                                                                let _e4727 = phi_6405_;
                                                                                let _e4729 = phi_6407_;
                                                                                phi_6408_ = _e4727;
                                                                                phi_6409_ = select(_e4140, _e4692, _e4699);
                                                                                phi_6410_ = _e4729;
                                                                                phi_6411_ = _e4146;
                                                                            }
                                                                            let _e4736 = phi_6408_;
                                                                            let _e4738 = phi_6409_;
                                                                            let _e4740 = phi_6410_;
                                                                            let _e4742 = phi_6411_;
                                                                            phi_6412_ = _e4736;
                                                                            phi_6413_ = _e4738;
                                                                            phi_6414_ = _e4740;
                                                                            phi_6415_ = _e4742;
                                                                        }
                                                                        let _e4748 = phi_6412_;
                                                                        let _e4750 = phi_6413_;
                                                                        let _e4752 = phi_6414_;
                                                                        let _e4754 = phi_6415_;
                                                                        phi_6416_ = _e4748;
                                                                        phi_6417_ = _e4750;
                                                                        phi_6418_ = _e4752;
                                                                        phi_6419_ = _e4754;
                                                                    }
                                                                    let _e4760 = phi_6416_;
                                                                    let _e4762 = phi_6417_;
                                                                    let _e4764 = phi_6418_;
                                                                    let _e4766 = phi_6419_;
                                                                    phi_6420_ = _e4760;
                                                                    phi_6421_ = _e4762;
                                                                    phi_6422_ = _e4764;
                                                                    phi_6423_ = _e4766;
                                                                }
                                                                let _e4768 = phi_6420_;
                                                                let _e4770 = phi_6421_;
                                                                let _e4772 = phi_6422_;
                                                                let _e4774 = phi_6423_;
                                                                phi_6425_ = _e4768;
                                                                phi_6426_ = _e4770;
                                                                phi_6427_ = _e4772;
                                                                phi_6428_ = _e4774;
                                                                phi_6429_ = select(_e4148, _e4692, _e4695);
                                                                phi_6430_ = _e4150;
                                                            }
                                                            let _e4779 = phi_6425_;
                                                            let _e4781 = phi_6426_;
                                                            let _e4783 = phi_6427_;
                                                            let _e4785 = phi_6428_;
                                                            let _e4787 = phi_6429_;
                                                            let _e4789 = phi_6430_;
                                                            phi_6431_ = _e4779;
                                                            phi_6432_ = _e4781;
                                                            phi_6433_ = _e4783;
                                                            phi_6434_ = _e4785;
                                                            phi_6435_ = _e4787;
                                                            phi_6436_ = _e4789;
                                                        }
                                                        let _e4793 = phi_6431_;
                                                        let _e4795 = phi_6432_;
                                                        let _e4797 = phi_6433_;
                                                        let _e4799 = phi_6434_;
                                                        let _e4801 = phi_6435_;
                                                        let _e4803 = phi_6436_;
                                                        phi_5695_ = _e4793;
                                                        phi_5698_ = _e4795;
                                                        phi_5700_ = (_e4142 + 1u);
                                                        phi_5702_ = _e4797;
                                                        phi_5704_ = _e4799;
                                                        phi_5706_ = _e4801;
                                                        phi_5708_ = _e4803;
                                                    } else {
                                                        phi_5695_ = f32();
                                                        phi_5698_ = f32();
                                                        phi_5700_ = u32();
                                                        phi_5702_ = vec3<f32>();
                                                        phi_5704_ = vec3<f32>();
                                                        phi_5706_ = f32();
                                                        phi_5708_ = f32();
                                                    }
                                                    let _e4806 = phi_5695_;
                                                    let _e4808 = phi_5698_;
                                                    let _e4810 = phi_5700_;
                                                    let _e4812 = phi_5702_;
                                                    let _e4814 = phi_5704_;
                                                    let _e4816 = phi_5706_;
                                                    let _e4818 = phi_5708_;
                                                    phi_5694_ = _e4806;
                                                    phi_5697_ = _e4808;
                                                    phi_5699_ = _e4810;
                                                    phi_5701_ = _e4812;
                                                    phi_5703_ = _e4814;
                                                    phi_5705_ = _e4816;
                                                    phi_5707_ = _e4818;
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
                                                        phi_24166_ = true;
                                                    } else {
                                                        let _e4856 = local_28;
                                                        phi_24166_ = (0.1f >= _e4856);
                                                    }
                                                    let _e4859 = phi_24166_;
                                                    let _e4861 = local_29;
                                                    let _e4864 = (pow(select(_e4861, 0.1f, _e4859), 1.3f) * _e4073);
                                                    phi_6468_ = vec3<f32>((_e4070.x + (((_e4079 * _e4833) + (_e4088 * _e4840)) * _e4864)), (_e4070.y + (((_e4082 * _e4833) + (_e4091 * _e4840)) * _e4864)), (_e4070.z + (((_e4085 * _e4833) + (_e4094 * _e4840)) * _e4864)));
                                                } else {
                                                    phi_6468_ = _e4070;
                                                }
                                                let _e4876 = phi_6468_;
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
                                                    phi_6547_ = (_e4076 * (((exp((-(_e4884) * _e4884)) * (0.15f + (3f * ((_e4894 * _e4894) * (3f - (2f * _e4894)))))) * (0.5f + (0.9f * exp(((_e3853 * -0.6666667f) * (_e3853 * 0.6666667f)))))) + (1.2f * (exp(((_e3842 * -3.3333333f) * (_e3842 * 3.3333333f))) * ((_e4916 * _e4916) * (3f - (2f * _e4916)))))));
                                                } else {
                                                    phi_6547_ = 0f;
                                                }
                                                let _e4930 = phi_6547_;
                                                phi_6548_ = _e4876;
                                                phi_6549_ = _e4930;
                                            } else {
                                                phi_6548_ = _e4070;
                                                phi_6549_ = 0f;
                                            }
                                            let _e4932 = phi_6548_;
                                            let _e4934 = phi_6549_;
                                            phi_6551_ = _e4932;
                                            phi_6554_ = 0u;
                                            loop {
                                                let _e4936 = phi_6551_;
                                                let _e4938 = phi_6554_;
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
                                                        phi_7700_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7701_ = vec3<f32>();
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
                                                            phi_7698_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4962 * exp((-2.5f * pow((sqrt(((_e5648 * _e5648) + (_e5654 * _e5654))) * 3.3333333f), 0.7f)))));
                                                            phi_7699_ = vec3<f32>();
                                                        } else {
                                                            let _e5048 = (((_e4971 * _e4947) + (_e4974 * _e4950)) + (_e4977 * _e4953));
                                                            if (abs(_e5048) < 0.12f) {
                                                                phi_7002_ = select(0.12f, -0.12f, (_e5048 < 0f));
                                                            } else {
                                                                phi_7002_ = _e5048;
                                                            }
                                                            let _e5054 = phi_7002_;
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
                                                            phi_7182_ = 0u;
                                                            phi_7185_ = 0f;
                                                            phi_7187_ = 1f;
                                                            phi_7189_ = 0f;
                                                            phi_7191_ = vec3<f32>((_e5071 * 9f), (_e5076 * 9f), 0f);
                                                            loop {
                                                                let _e5129 = phi_7182_;
                                                                let _e5131 = phi_7185_;
                                                                let _e5133 = phi_7187_;
                                                                let _e5135 = phi_7189_;
                                                                let _e5137 = phi_7191_;
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
                                                                        phi_24261_ = ((_e5267 * _e5175) + (_e5270 * _e5176));
                                                                    } else {
                                                                        if (_e5264 < 8u) {
                                                                            phi_24255_ = ((_e5267 * _e5175) + (_e5270 * _e5177));
                                                                        } else {
                                                                            phi_24255_ = ((_e5267 * _e5176) + (_e5270 * _e5177));
                                                                        }
                                                                        let _e5280 = phi_24255_;
                                                                        phi_24261_ = _e5280;
                                                                    }
                                                                    let _e5285 = phi_24261_;
                                                                    let _e5288 = global_6.member[(_e5100 + _e5245)];
                                                                    let _e5294 = (_e5175 - 1f);
                                                                    let _e5295 = (select(select(u32(_e5288), 0u, (_e5288 < 0f)), 4294967295u, (_e5288 > 4294967000f)) % 12u);
                                                                    let _e5298 = select(-1f, 1f, ((_e5295 & 1u) == 0u));
                                                                    let _e5301 = select(-1f, 1f, ((_e5295 & 2u) == 0u));
                                                                    if (_e5295 < 4u) {
                                                                        phi_24295_ = ((_e5298 * _e5294) + (_e5301 * _e5176));
                                                                    } else {
                                                                        if (_e5295 < 8u) {
                                                                            phi_24289_ = ((_e5298 * _e5294) + (_e5301 * _e5177));
                                                                        } else {
                                                                            phi_24289_ = ((_e5298 * _e5176) + (_e5301 * _e5177));
                                                                        }
                                                                        let _e5311 = phi_24289_;
                                                                        phi_24295_ = _e5311;
                                                                    }
                                                                    let _e5316 = phi_24295_;
                                                                    let _e5319 = global_6.member[(_e5100 + _e5226)];
                                                                    let _e5325 = (_e5176 - 1f);
                                                                    let _e5326 = (select(select(u32(_e5319), 0u, (_e5319 < 0f)), 4294967295u, (_e5319 > 4294967000f)) % 12u);
                                                                    let _e5329 = select(-1f, 1f, ((_e5326 & 1u) == 0u));
                                                                    let _e5332 = select(-1f, 1f, ((_e5326 & 2u) == 0u));
                                                                    if (_e5326 < 4u) {
                                                                        phi_24329_ = ((_e5329 * _e5175) + (_e5332 * _e5325));
                                                                    } else {
                                                                        if (_e5326 < 8u) {
                                                                            phi_24323_ = ((_e5329 * _e5175) + (_e5332 * _e5177));
                                                                        } else {
                                                                            phi_24323_ = ((_e5329 * _e5325) + (_e5332 * _e5177));
                                                                        }
                                                                        let _e5342 = phi_24323_;
                                                                        phi_24329_ = _e5342;
                                                                    }
                                                                    let _e5347 = phi_24329_;
                                                                    let _e5350 = global_6.member[(_e5100 + _e5255)];
                                                                    let _e5356 = (select(select(u32(_e5350), 0u, (_e5350 < 0f)), 4294967295u, (_e5350 > 4294967000f)) % 12u);
                                                                    let _e5359 = select(-1f, 1f, ((_e5356 & 1u) == 0u));
                                                                    let _e5362 = select(-1f, 1f, ((_e5356 & 2u) == 0u));
                                                                    if (_e5356 < 4u) {
                                                                        phi_24363_ = ((_e5359 * _e5294) + (_e5362 * _e5325));
                                                                    } else {
                                                                        if (_e5356 < 8u) {
                                                                            phi_24357_ = ((_e5359 * _e5294) + (_e5362 * _e5177));
                                                                        } else {
                                                                            phi_24357_ = ((_e5359 * _e5325) + (_e5362 * _e5177));
                                                                        }
                                                                        let _e5372 = phi_24357_;
                                                                        phi_24363_ = _e5372;
                                                                    }
                                                                    let _e5377 = phi_24363_;
                                                                    let _e5381 = global_6.member[(_e5100 + (_e5216 + 1u))];
                                                                    let _e5387 = (_e5177 - 1f);
                                                                    let _e5388 = (select(select(u32(_e5381), 0u, (_e5381 < 0f)), 4294967295u, (_e5381 > 4294967000f)) % 12u);
                                                                    let _e5391 = select(-1f, 1f, ((_e5388 & 1u) == 0u));
                                                                    let _e5394 = select(-1f, 1f, ((_e5388 & 2u) == 0u));
                                                                    if (_e5388 < 4u) {
                                                                        phi_24397_ = ((_e5391 * _e5175) + (_e5394 * _e5176));
                                                                    } else {
                                                                        if (_e5388 < 8u) {
                                                                            phi_24391_ = ((_e5391 * _e5175) + (_e5394 * _e5387));
                                                                        } else {
                                                                            phi_24391_ = ((_e5391 * _e5176) + (_e5394 * _e5387));
                                                                        }
                                                                        let _e5404 = phi_24391_;
                                                                        phi_24397_ = _e5404;
                                                                    }
                                                                    let _e5409 = phi_24397_;
                                                                    let _e5413 = global_6.member[(_e5100 + (_e5245 + 1u))];
                                                                    let _e5419 = (select(select(u32(_e5413), 0u, (_e5413 < 0f)), 4294967295u, (_e5413 > 4294967000f)) % 12u);
                                                                    let _e5422 = select(-1f, 1f, ((_e5419 & 1u) == 0u));
                                                                    let _e5425 = select(-1f, 1f, ((_e5419 & 2u) == 0u));
                                                                    if (_e5419 < 4u) {
                                                                        phi_24431_ = ((_e5422 * _e5294) + (_e5425 * _e5176));
                                                                    } else {
                                                                        if (_e5419 < 8u) {
                                                                            phi_24425_ = ((_e5422 * _e5294) + (_e5425 * _e5387));
                                                                        } else {
                                                                            phi_24425_ = ((_e5422 * _e5176) + (_e5425 * _e5387));
                                                                        }
                                                                        let _e5435 = phi_24425_;
                                                                        phi_24431_ = _e5435;
                                                                    }
                                                                    let _e5440 = phi_24431_;
                                                                    let _e5444 = global_6.member[(_e5100 + (_e5226 + 1u))];
                                                                    let _e5450 = (select(select(u32(_e5444), 0u, (_e5444 < 0f)), 4294967295u, (_e5444 > 4294967000f)) % 12u);
                                                                    let _e5453 = select(-1f, 1f, ((_e5450 & 1u) == 0u));
                                                                    let _e5456 = select(-1f, 1f, ((_e5450 & 2u) == 0u));
                                                                    if (_e5450 < 4u) {
                                                                        phi_24465_ = ((_e5453 * _e5175) + (_e5456 * _e5325));
                                                                    } else {
                                                                        if (_e5450 < 8u) {
                                                                            phi_24459_ = ((_e5453 * _e5175) + (_e5456 * _e5387));
                                                                        } else {
                                                                            phi_24459_ = ((_e5453 * _e5325) + (_e5456 * _e5387));
                                                                        }
                                                                        let _e5466 = phi_24459_;
                                                                        phi_24465_ = _e5466;
                                                                    }
                                                                    let _e5471 = phi_24465_;
                                                                    let _e5475 = global_6.member[(_e5100 + (_e5255 + 1u))];
                                                                    let _e5481 = (select(select(u32(_e5475), 0u, (_e5475 < 0f)), 4294967295u, (_e5475 > 4294967000f)) % 12u);
                                                                    let _e5484 = select(-1f, 1f, ((_e5481 & 1u) == 0u));
                                                                    let _e5487 = select(-1f, 1f, ((_e5481 & 2u) == 0u));
                                                                    if (_e5481 < 4u) {
                                                                        phi_24499_ = ((_e5484 * _e5294) + (_e5487 * _e5325));
                                                                    } else {
                                                                        if (_e5481 < 8u) {
                                                                            phi_24493_ = ((_e5484 * _e5294) + (_e5487 * _e5387));
                                                                        } else {
                                                                            phi_24493_ = ((_e5484 * _e5325) + (_e5487 * _e5387));
                                                                        }
                                                                        let _e5497 = phi_24493_;
                                                                        phi_24499_ = _e5497;
                                                                    }
                                                                    let _e5502 = phi_24499_;
                                                                    let _e5505 = (_e5285 + (_e5184 * (_e5316 - _e5285)));
                                                                    let _e5511 = (_e5409 + (_e5184 * (_e5440 - _e5409)));
                                                                    let _e5517 = (_e5505 + (_e5191 * ((_e5347 + (_e5184 * (_e5377 - _e5347))) - _e5505)));
                                                                    phi_7183_ = (_e5129 + 1u);
                                                                    phi_7186_ = (_e5131 + _e5133);
                                                                    phi_7188_ = (_e5133 * 0.5f);
                                                                    phi_7190_ = (_e5135 + ((_e5517 + ((((_e5177 * _e5177) * _e5177) * ((_e5177 * ((_e5177 * 6f) - 15f)) + 10f)) * ((_e5511 + (_e5191 * ((_e5471 + (_e5184 * (_e5502 - _e5471))) - _e5511))) - _e5517))) * _e5133));
                                                                    phi_7192_ = vec3<f32>(((((_e5103 * _e5137.x) + (_e5106 * _e5137.y)) + (_e5109 * _e5137.z)) * 2.03f), ((((_e5112 * _e5137.x) + (_e5115 * _e5137.y)) + (_e5118 * _e5137.z)) * 2.03f), ((((_e5121 * _e5137.x) + (_e5124 * _e5137.y)) + (_e5127 * _e5137.z)) * 2.03f));
                                                                } else {
                                                                    phi_7183_ = u32();
                                                                    phi_7186_ = f32();
                                                                    phi_7188_ = f32();
                                                                    phi_7190_ = f32();
                                                                    phi_7192_ = vec3<f32>();
                                                                }
                                                                let _e5549 = phi_7183_;
                                                                let _e5551 = phi_7186_;
                                                                let _e5553 = phi_7188_;
                                                                let _e5555 = phi_7190_;
                                                                let _e5557 = phi_7192_;
                                                                phi_7182_ = _e5549;
                                                                phi_7185_ = _e5551;
                                                                phi_7187_ = _e5553;
                                                                phi_7189_ = _e5555;
                                                                phi_7191_ = _e5557;
                                                                if !(_e5138) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5560 = local_33;
                                                            if (_e5560 > 0f) {
                                                                let _e5563 = local_34;
                                                                let _e5565 = local_35;
                                                                phi_7605_ = (_e5563 / _e5565);
                                                            } else {
                                                                phi_7605_ = 0f;
                                                            }
                                                            let _e5568 = phi_7605_;
                                                            let _e5570 = (1f + (1.2f * _e5568));
                                                            if (_e5570 != _e5570) {
                                                                phi_24545_ = true;
                                                            } else {
                                                                phi_24545_ = (0f >= _e5570);
                                                            }
                                                            let _e5574 = phi_24545_;
                                                            let _e5579 = ((_e5080 - 1.15f) * -2.8571432f);
                                                            let _e5581 = select(_e5579, 0f, (_e5579 < 0f));
                                                            let _e5583 = select(_e5581, 1f, (_e5581 > 1f));
                                                            let _e5589 = (1f / _e5092);
                                                            if (_e5589 != _e5589) {
                                                                phi_24560_ = true;
                                                            } else {
                                                                phi_24560_ = (4f <= _e5589);
                                                            }
                                                            let _e5593 = phi_24560_;
                                                            let _e5596 = (((exp((_e5080 * -3.5714285f)) * ((_e5583 * _e5583) * (3f - (2f * _e5583)))) * select(_e5589, 4f, _e5593)) * 0.12f);
                                                            let _e5602 = sqrt((((_e5041 * _e5041) + (_e5042 * _e5042)) + (_e5043 * _e5043)));
                                                            let _e5609 = (((_e5041 * _e5007) + (_e5042 * _e5010)) + (_e5043 * _e5013));
                                                            let _e5610 = (1f - _e5092);
                                                            let _e5626 = ((_e5596 * (0.6f + (0.4f * (1f - _e5094)))) + exp((_e5602 * -16.666668f)));
                                                            let _e5632 = (((_e5596 * _e5094) * 1.8f) * select(_e5570, 0f, _e5574));
                                                            let _e5639 = (_e4962 * (1f - ((((0.75f * _e5610) * _e5610) * exp(((_e5609 * -28.571428f) * (_e5609 * 28.571428f)))) * exp((_e5602 * -1.4285715f)))));
                                                            phi_7698_ = vec3<f32>();
                                                            phi_7699_ = vec3<f32>((((1.1633643f * _e5626) + (0.86689025f * _e5632)) * _e5639), (((0.97722596f * _e5626) + (1.0155001f * _e5632)) * _e5639), (((0.74455315f * _e5626) + (1.2384146f * _e5632)) * _e5639));
                                                        }
                                                        let _e5666 = phi_7698_;
                                                        let _e5668 = phi_7699_;
                                                        phi_7700_ = _e5666;
                                                        phi_7701_ = _e5668;
                                                    }
                                                    let _e5670 = phi_7700_;
                                                    let _e5672 = phi_7701_;
                                                    phi_6552_ = (_e4936 + select(_e5672, _e5670, vec3(select(_e4944, true, _e4968))));
                                                    phi_6555_ = (_e4938 + 1u);
                                                } else {
                                                    phi_6552_ = vec3<f32>();
                                                    phi_6555_ = u32();
                                                }
                                                let _e5679 = phi_6552_;
                                                let _e5681 = phi_6555_;
                                                phi_6551_ = _e5679;
                                                phi_6554_ = _e5681;
                                                if !(_e4939) {
                                                    break;
                                                }
                                            }
                                            phi_32947_ = _e4068;
                                            if _e4068 {
                                                break;
                                            }
                                            phi_7720_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7723_ = 0u;
                                            phi_7725_ = _e4934;
                                            loop {
                                                let _e5684 = phi_7720_;
                                                let _e5686 = phi_7723_;
                                                let _e5688 = phi_7725_;
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
                                                        phi_9042_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
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
                                                            phi_8027_ = 6u;
                                                            phi_8028_ = 1.2f;
                                                            phi_8029_ = _e5781;
                                                        } else {
                                                            let _e5783 = (_e5698 == 1u);
                                                            if _e5783 {
                                                                phi_8026_ = vec3<f32>((_e5768 * 1.54f), (_e5774 * 1.54f), 0f);
                                                            } else {
                                                                phi_8026_ = _e5781;
                                                            }
                                                            let _e5788 = phi_8026_;
                                                            phi_8027_ = select(6u, 5u, _e5783);
                                                            phi_8028_ = select(1f, 0.8f, _e5783);
                                                            phi_8029_ = _e5788;
                                                        }
                                                        let _e5792 = phi_8027_;
                                                        let _e5794 = phi_8028_;
                                                        let _e5796 = phi_8029_;
                                                        if _e5782 {
                                                            phi_8046_ = vec3<f32>(((_e5768 * 3.7400002f) + -7f), ((_e5774 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8046_ = vec3<f32>((_e5768 * 3.3000002f), (_e5774 * 3.3000002f), 0f);
                                                        }
                                                        let _e5806 = phi_8046_;
                                                        phi_8048_ = 0u;
                                                        phi_8051_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8053_ = 0f;
                                                        phi_8055_ = 0f;
                                                        phi_8057_ = 0f;
                                                        loop {
                                                            let _e5809 = phi_8048_;
                                                            let _e5811 = phi_8051_;
                                                            let _e5813 = phi_8053_;
                                                            let _e5815 = phi_8055_;
                                                            let _e5817 = phi_8057_;
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
                                                                    phi_8099_ = true;
                                                                    phi_8100_ = false;
                                                                    phi_8101_ = false;
                                                                } else {
                                                                    if (_e5809 <= 4u) {
                                                                        phi_8077_ = (_e5698 == 2u);
                                                                    } else {
                                                                        phi_8077_ = true;
                                                                    }
                                                                    let _e5823 = phi_8077_;
                                                                    if _e5823 {
                                                                        let _e5824 = (_e5809 == 5u);
                                                                        if _e5824 {
                                                                            if _e5782 {
                                                                                phi_8093_ = false;
                                                                            } else {
                                                                                phi_8093_ = (_e5698 == 2u);
                                                                            }
                                                                            let _e5827 = phi_8093_;
                                                                            phi_8095_ = _e5827;
                                                                        } else {
                                                                            phi_8095_ = false;
                                                                        }
                                                                        let _e5829 = phi_8095_;
                                                                        phi_8097_ = select(false, _e5782, _e5824);
                                                                        phi_8098_ = _e5829;
                                                                    } else {
                                                                        phi_8097_ = false;
                                                                        phi_8098_ = false;
                                                                    }
                                                                    let _e5832 = phi_8097_;
                                                                    let _e5834 = phi_8098_;
                                                                    phi_8099_ = select(true, false, _e5823);
                                                                    phi_8100_ = _e5832;
                                                                    phi_8101_ = _e5834;
                                                                }
                                                                let _e5837 = phi_8099_;
                                                                let _e5839 = phi_8100_;
                                                                let _e5841 = phi_8101_;
                                                                if select(select(_e5841, false, _e5837), true, select(_e5839, true, _e5837)) {
                                                                    if _e5819 {
                                                                        phi_8148_ = 3u;
                                                                        phi_8149_ = vec3<f32>(((_e5768 * 1.7600001f) + 3f), ((_e5774 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5809 < 4u) {
                                                                            let _e5858 = (_e5809 - 1u);
                                                                            if (_e5858 == 0u) {
                                                                                phi_24579_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_24579_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5858 == 1u)));
                                                                            }
                                                                            let _e5864 = phi_24579_;
                                                                            phi_8146_ = 4u;
                                                                            phi_8147_ = (_e5796 + _e5864);
                                                                        } else {
                                                                            let _e5846 = (_e5809 == 4u);
                                                                            if _e5846 {
                                                                                phi_8145_ = (_e5796 + vec3<f32>((_e5811.x * _e5794), (_e5811.y * _e5794), (_e5811.z * _e5794)));
                                                                            } else {
                                                                                phi_8145_ = _e5806;
                                                                            }
                                                                            let _e5856 = phi_8145_;
                                                                            phi_8146_ = select(select(4u, 5u, _e5782), _e5792, _e5846);
                                                                            phi_8147_ = _e5856;
                                                                        }
                                                                        let _e5867 = phi_8146_;
                                                                        let _e5869 = phi_8147_;
                                                                        phi_8148_ = _e5867;
                                                                        phi_8149_ = _e5869;
                                                                    }
                                                                    let _e5876 = phi_8148_;
                                                                    let _e5878 = phi_8149_;
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
                                                                    phi_8284_ = 0u;
                                                                    phi_8287_ = 0f;
                                                                    phi_8289_ = 1f;
                                                                    phi_8291_ = 0f;
                                                                    phi_8293_ = _e5878;
                                                                    loop {
                                                                        let _e5910 = phi_8284_;
                                                                        let _e5912 = phi_8287_;
                                                                        let _e5914 = phi_8289_;
                                                                        let _e5916 = phi_8291_;
                                                                        let _e5918 = phi_8293_;
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
                                                                                phi_24640_ = ((_e6048 * _e5956) + (_e6051 * _e5957));
                                                                            } else {
                                                                                if (_e6045 < 8u) {
                                                                                    phi_24634_ = ((_e6048 * _e5956) + (_e6051 * _e5958));
                                                                                } else {
                                                                                    phi_24634_ = ((_e6048 * _e5957) + (_e6051 * _e5958));
                                                                                }
                                                                                let _e6061 = phi_24634_;
                                                                                phi_24640_ = _e6061;
                                                                            }
                                                                            let _e6066 = phi_24640_;
                                                                            let _e6069 = global_6.member[(_e5881 + _e6026)];
                                                                            let _e6075 = (_e5956 - 1f);
                                                                            let _e6076 = (select(select(u32(_e6069), 0u, (_e6069 < 0f)), 4294967295u, (_e6069 > 4294967000f)) % 12u);
                                                                            let _e6079 = select(-1f, 1f, ((_e6076 & 1u) == 0u));
                                                                            let _e6082 = select(-1f, 1f, ((_e6076 & 2u) == 0u));
                                                                            if (_e6076 < 4u) {
                                                                                phi_24674_ = ((_e6079 * _e6075) + (_e6082 * _e5957));
                                                                            } else {
                                                                                if (_e6076 < 8u) {
                                                                                    phi_24668_ = ((_e6079 * _e6075) + (_e6082 * _e5958));
                                                                                } else {
                                                                                    phi_24668_ = ((_e6079 * _e5957) + (_e6082 * _e5958));
                                                                                }
                                                                                let _e6092 = phi_24668_;
                                                                                phi_24674_ = _e6092;
                                                                            }
                                                                            let _e6097 = phi_24674_;
                                                                            let _e6100 = global_6.member[(_e5881 + _e6007)];
                                                                            let _e6106 = (_e5957 - 1f);
                                                                            let _e6107 = (select(select(u32(_e6100), 0u, (_e6100 < 0f)), 4294967295u, (_e6100 > 4294967000f)) % 12u);
                                                                            let _e6110 = select(-1f, 1f, ((_e6107 & 1u) == 0u));
                                                                            let _e6113 = select(-1f, 1f, ((_e6107 & 2u) == 0u));
                                                                            if (_e6107 < 4u) {
                                                                                phi_24708_ = ((_e6110 * _e5956) + (_e6113 * _e6106));
                                                                            } else {
                                                                                if (_e6107 < 8u) {
                                                                                    phi_24702_ = ((_e6110 * _e5956) + (_e6113 * _e5958));
                                                                                } else {
                                                                                    phi_24702_ = ((_e6110 * _e6106) + (_e6113 * _e5958));
                                                                                }
                                                                                let _e6123 = phi_24702_;
                                                                                phi_24708_ = _e6123;
                                                                            }
                                                                            let _e6128 = phi_24708_;
                                                                            let _e6131 = global_6.member[(_e5881 + _e6036)];
                                                                            let _e6137 = (select(select(u32(_e6131), 0u, (_e6131 < 0f)), 4294967295u, (_e6131 > 4294967000f)) % 12u);
                                                                            let _e6140 = select(-1f, 1f, ((_e6137 & 1u) == 0u));
                                                                            let _e6143 = select(-1f, 1f, ((_e6137 & 2u) == 0u));
                                                                            if (_e6137 < 4u) {
                                                                                phi_24742_ = ((_e6140 * _e6075) + (_e6143 * _e6106));
                                                                            } else {
                                                                                if (_e6137 < 8u) {
                                                                                    phi_24736_ = ((_e6140 * _e6075) + (_e6143 * _e5958));
                                                                                } else {
                                                                                    phi_24736_ = ((_e6140 * _e6106) + (_e6143 * _e5958));
                                                                                }
                                                                                let _e6153 = phi_24736_;
                                                                                phi_24742_ = _e6153;
                                                                            }
                                                                            let _e6158 = phi_24742_;
                                                                            let _e6162 = global_6.member[(_e5881 + (_e5997 + 1u))];
                                                                            let _e6168 = (_e5958 - 1f);
                                                                            let _e6169 = (select(select(u32(_e6162), 0u, (_e6162 < 0f)), 4294967295u, (_e6162 > 4294967000f)) % 12u);
                                                                            let _e6172 = select(-1f, 1f, ((_e6169 & 1u) == 0u));
                                                                            let _e6175 = select(-1f, 1f, ((_e6169 & 2u) == 0u));
                                                                            if (_e6169 < 4u) {
                                                                                phi_24776_ = ((_e6172 * _e5956) + (_e6175 * _e5957));
                                                                            } else {
                                                                                if (_e6169 < 8u) {
                                                                                    phi_24770_ = ((_e6172 * _e5956) + (_e6175 * _e6168));
                                                                                } else {
                                                                                    phi_24770_ = ((_e6172 * _e5957) + (_e6175 * _e6168));
                                                                                }
                                                                                let _e6185 = phi_24770_;
                                                                                phi_24776_ = _e6185;
                                                                            }
                                                                            let _e6190 = phi_24776_;
                                                                            let _e6194 = global_6.member[(_e5881 + (_e6026 + 1u))];
                                                                            let _e6200 = (select(select(u32(_e6194), 0u, (_e6194 < 0f)), 4294967295u, (_e6194 > 4294967000f)) % 12u);
                                                                            let _e6203 = select(-1f, 1f, ((_e6200 & 1u) == 0u));
                                                                            let _e6206 = select(-1f, 1f, ((_e6200 & 2u) == 0u));
                                                                            if (_e6200 < 4u) {
                                                                                phi_24810_ = ((_e6203 * _e6075) + (_e6206 * _e5957));
                                                                            } else {
                                                                                if (_e6200 < 8u) {
                                                                                    phi_24804_ = ((_e6203 * _e6075) + (_e6206 * _e6168));
                                                                                } else {
                                                                                    phi_24804_ = ((_e6203 * _e5957) + (_e6206 * _e6168));
                                                                                }
                                                                                let _e6216 = phi_24804_;
                                                                                phi_24810_ = _e6216;
                                                                            }
                                                                            let _e6221 = phi_24810_;
                                                                            let _e6225 = global_6.member[(_e5881 + (_e6007 + 1u))];
                                                                            let _e6231 = (select(select(u32(_e6225), 0u, (_e6225 < 0f)), 4294967295u, (_e6225 > 4294967000f)) % 12u);
                                                                            let _e6234 = select(-1f, 1f, ((_e6231 & 1u) == 0u));
                                                                            let _e6237 = select(-1f, 1f, ((_e6231 & 2u) == 0u));
                                                                            if (_e6231 < 4u) {
                                                                                phi_24844_ = ((_e6234 * _e5956) + (_e6237 * _e6106));
                                                                            } else {
                                                                                if (_e6231 < 8u) {
                                                                                    phi_24838_ = ((_e6234 * _e5956) + (_e6237 * _e6168));
                                                                                } else {
                                                                                    phi_24838_ = ((_e6234 * _e6106) + (_e6237 * _e6168));
                                                                                }
                                                                                let _e6247 = phi_24838_;
                                                                                phi_24844_ = _e6247;
                                                                            }
                                                                            let _e6252 = phi_24844_;
                                                                            let _e6256 = global_6.member[(_e5881 + (_e6036 + 1u))];
                                                                            let _e6262 = (select(select(u32(_e6256), 0u, (_e6256 < 0f)), 4294967295u, (_e6256 > 4294967000f)) % 12u);
                                                                            let _e6265 = select(-1f, 1f, ((_e6262 & 1u) == 0u));
                                                                            let _e6268 = select(-1f, 1f, ((_e6262 & 2u) == 0u));
                                                                            if (_e6262 < 4u) {
                                                                                phi_24878_ = ((_e6265 * _e6075) + (_e6268 * _e6106));
                                                                            } else {
                                                                                if (_e6262 < 8u) {
                                                                                    phi_24872_ = ((_e6265 * _e6075) + (_e6268 * _e6168));
                                                                                } else {
                                                                                    phi_24872_ = ((_e6265 * _e6106) + (_e6268 * _e6168));
                                                                                }
                                                                                let _e6278 = phi_24872_;
                                                                                phi_24878_ = _e6278;
                                                                            }
                                                                            let _e6283 = phi_24878_;
                                                                            let _e6286 = (_e6066 + (_e5965 * (_e6097 - _e6066)));
                                                                            let _e6292 = (_e6190 + (_e5965 * (_e6221 - _e6190)));
                                                                            let _e6298 = (_e6286 + (_e5972 * ((_e6128 + (_e5965 * (_e6158 - _e6128))) - _e6286)));
                                                                            phi_8285_ = (_e5910 + 1u);
                                                                            phi_8288_ = (_e5912 + _e5914);
                                                                            phi_8290_ = (_e5914 * 0.5f);
                                                                            phi_8292_ = (_e5916 + ((_e6298 + ((((_e5958 * _e5958) * _e5958) * ((_e5958 * ((_e5958 * 6f) - 15f)) + 10f)) * ((_e6292 + (_e5972 * ((_e6252 + (_e5965 * (_e6283 - _e6252))) - _e6292))) - _e6298))) * _e5914));
                                                                            phi_8294_ = vec3<f32>(((((_e5884 * _e5918.x) + (_e5887 * _e5918.y)) + (_e5890 * _e5918.z)) * 2.03f), ((((_e5893 * _e5918.x) + (_e5896 * _e5918.y)) + (_e5899 * _e5918.z)) * 2.03f), ((((_e5902 * _e5918.x) + (_e5905 * _e5918.y)) + (_e5908 * _e5918.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8285_ = u32();
                                                                            phi_8288_ = f32();
                                                                            phi_8290_ = f32();
                                                                            phi_8292_ = f32();
                                                                            phi_8294_ = vec3<f32>();
                                                                        }
                                                                        let _e6330 = phi_8285_;
                                                                        let _e6332 = phi_8288_;
                                                                        let _e6334 = phi_8290_;
                                                                        let _e6336 = phi_8292_;
                                                                        let _e6338 = phi_8294_;
                                                                        phi_8284_ = _e6330;
                                                                        phi_8287_ = _e6332;
                                                                        phi_8289_ = _e6334;
                                                                        phi_8291_ = _e6336;
                                                                        phi_8293_ = _e6338;
                                                                        if !(_e5919) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6341 = local_36;
                                                                    if (_e6341 > 0f) {
                                                                        let _e6344 = local_37;
                                                                        let _e6346 = local_38;
                                                                        phi_8707_ = (_e6344 / _e6346);
                                                                    } else {
                                                                        phi_8707_ = 0f;
                                                                    }
                                                                    let _e6349 = phi_8707_;
                                                                    if _e5819 {
                                                                        phi_8742_ = _e5811;
                                                                        phi_8743_ = _e5813;
                                                                        phi_8744_ = _e5815;
                                                                    } else {
                                                                        if (_e5809 == 1u) {
                                                                            phi_8739_ = vec3<f32>(_e6349, _e5811.y, _e5811.z);
                                                                            phi_8740_ = _e5813;
                                                                            phi_8741_ = _e5815;
                                                                        } else {
                                                                            if (_e5809 == 2u) {
                                                                                phi_8736_ = vec3<f32>(_e5811.x, _e6349, _e5811.z);
                                                                                phi_8737_ = _e5813;
                                                                                phi_8738_ = _e5815;
                                                                            } else {
                                                                                if (_e5809 == 3u) {
                                                                                    phi_8733_ = vec3<f32>(_e5811.x, _e5811.y, _e6349);
                                                                                    phi_8734_ = _e5813;
                                                                                    phi_8735_ = _e5815;
                                                                                } else {
                                                                                    let _e6353 = (_e5809 == 4u);
                                                                                    phi_8733_ = _e5811;
                                                                                    phi_8734_ = select(_e6349, _e5813, _e6353);
                                                                                    phi_8735_ = select(_e5815, _e6349, _e6353);
                                                                                }
                                                                                let _e6361 = phi_8733_;
                                                                                let _e6363 = phi_8734_;
                                                                                let _e6365 = phi_8735_;
                                                                                phi_8736_ = _e6361;
                                                                                phi_8737_ = _e6363;
                                                                                phi_8738_ = _e6365;
                                                                            }
                                                                            let _e6371 = phi_8736_;
                                                                            let _e6373 = phi_8737_;
                                                                            let _e6375 = phi_8738_;
                                                                            phi_8739_ = _e6371;
                                                                            phi_8740_ = _e6373;
                                                                            phi_8741_ = _e6375;
                                                                        }
                                                                        let _e6381 = phi_8739_;
                                                                        let _e6383 = phi_8740_;
                                                                        let _e6385 = phi_8741_;
                                                                        phi_8742_ = _e6381;
                                                                        phi_8743_ = _e6383;
                                                                        phi_8744_ = _e6385;
                                                                    }
                                                                    let _e6387 = phi_8742_;
                                                                    let _e6389 = phi_8743_;
                                                                    let _e6391 = phi_8744_;
                                                                    phi_8746_ = _e6387;
                                                                    phi_8747_ = _e6389;
                                                                    phi_8748_ = _e6391;
                                                                    phi_8749_ = select(_e5817, _e6349, _e5819);
                                                                } else {
                                                                    phi_8746_ = _e5811;
                                                                    phi_8747_ = _e5813;
                                                                    phi_8748_ = _e5815;
                                                                    phi_8749_ = _e5817;
                                                                }
                                                                let _e6394 = phi_8746_;
                                                                let _e6396 = phi_8747_;
                                                                let _e6398 = phi_8748_;
                                                                let _e6400 = phi_8749_;
                                                                phi_8049_ = (_e5809 + 1u);
                                                                phi_8052_ = _e6394;
                                                                phi_8054_ = _e6396;
                                                                phi_8056_ = _e6398;
                                                                phi_8058_ = _e6400;
                                                            } else {
                                                                phi_8049_ = u32();
                                                                phi_8052_ = vec3<f32>();
                                                                phi_8054_ = f32();
                                                                phi_8056_ = f32();
                                                                phi_8058_ = f32();
                                                            }
                                                            let _e6403 = phi_8049_;
                                                            let _e6405 = phi_8052_;
                                                            let _e6407 = phi_8054_;
                                                            let _e6409 = phi_8056_;
                                                            let _e6411 = phi_8058_;
                                                            phi_8048_ = _e6403;
                                                            phi_8051_ = _e6405;
                                                            phi_8053_ = _e6407;
                                                            phi_8055_ = _e6409;
                                                            phi_8057_ = _e6411;
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
                                                            phi_9041_ = type_60(vec3<f32>((((3.1815925f * _e6541) + (0.14634217f * _e6570)) * _e6579), (((0.38179109f * _e6541) + (1.2439084f * _e6570)) * _e6579), (((0.69995034f * _e6541) + (1.0975662f * _e6570)) * _e6579)), (_e5719 * _e6565));
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
                                                                phi_9040_ = type_60((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5716 * _e6512)), (_e5719 * _e6512));
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
                                                                    phi_9039_ = type_60(vec3<f32>(((((3.1815925f * _e6454) + (_e6464 * 0.10243952f)) + _e6469) * _e5716), ((((0.38179109f * _e6454) + (_e6464 * 0.8707359f)) + _e6469) * _e5716), ((((0.69995034f * _e6454) + (_e6464 * 0.76829636f)) + _e6469) * _e5716)), 0f);
                                                                } else {
                                                                    let _e6421 = local_40;
                                                                    let _e6423 = ((_e6421 - -0.2f) * 1.6666666f);
                                                                    let _e6425 = select(_e6423, 0f, (_e6423 < 0f));
                                                                    let _e6427 = select(_e6425, 1f, (_e6425 > 1f));
                                                                    let _e6433 = ((_e6417 - 1f) * -1.4285715f);
                                                                    let _e6435 = select(_e6433, 0f, (_e6433 < 0f));
                                                                    let _e6437 = select(_e6435, 1f, (_e6435 > 1f));
                                                                    phi_9039_ = type_60(vec3<f32>(0f, 0f, 0f), (_e5719 * (((_e6427 * _e6427) * (3f - (2f * _e6427))) * ((_e6437 * _e6437) * (3f - (2f * _e6437))))));
                                                                }
                                                                let _e6488 = phi_9039_;
                                                                phi_9040_ = _e6488;
                                                            }
                                                            let _e6518 = phi_9040_;
                                                            phi_9041_ = _e6518;
                                                        }
                                                        let _e6587 = phi_9041_;
                                                        phi_9042_ = _e6587;
                                                    }
                                                    let _e6589 = phi_9042_;
                                                    phi_7721_ = (_e5684 + _e6589.member);
                                                    phi_7724_ = (_e5686 + 1u);
                                                    phi_7726_ = (_e5688 + _e6589.member_1);
                                                } else {
                                                    phi_7721_ = vec3<f32>();
                                                    phi_7724_ = u32();
                                                    phi_7726_ = f32();
                                                }
                                                let _e6596 = phi_7721_;
                                                let _e6598 = phi_7724_;
                                                let _e6600 = phi_7726_;
                                                phi_7720_ = _e6596;
                                                phi_7723_ = _e6598;
                                                phi_7725_ = _e6600;
                                                if !(_e5689) {
                                                    break;
                                                }
                                            }
                                            phi_32947_ = _e4068;
                                            if _e4068 {
                                                break;
                                            }
                                            let _e6603 = local_45;
                                            if (_e6603 <= 0f) {
                                                phi_9071_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6606 = local_46;
                                                let _e6609 = local_47;
                                                let _e6614 = local_48;
                                                phi_9071_ = vec3<f32>(exp((_e6609 * -0.81f)), exp(-(_e6606)), exp((_e6614 * -1.25f)));
                                            }
                                            let _e6619 = phi_9071_;
                                            let _e6621 = local_49;
                                            let _e6626 = local_50;
                                            let _e6631 = local_51;
                                            let _e6636 = local_52;
                                            let _e6640 = local_53;
                                            let _e6644 = local_54;
                                            let _e6649 = global_6.member[(_e1293 + 9u)];
                                            phi_32956_ = _e4068;
                                            phi_9118_ = vec3<f32>((_e3773.x + ((((_e6621.x * _e6619.x) + _e6636.x) * _e6649) * _e3775.x)), (_e3773.y + ((((_e6626.y * _e6619.y) + _e6640.y) * _e6649) * _e3775.y)), (_e3773.z + ((((_e6631.z * _e6619.z) + _e6644.z) * _e6649) * _e3775.z)));
                                        } else {
                                            phi_32956_ = _e3771;
                                            phi_9118_ = _e3773;
                                        }
                                        let _e6667 = phi_32956_;
                                        let _e6669 = phi_9118_;
                                        phi_32955_ = _e6667;
                                        phi_9119_ = _e6669;
                                    } else {
                                        phi_32955_ = _e3771;
                                        phi_9119_ = _e3779;
                                    }
                                    let _e6671 = phi_32955_;
                                    let _e6673 = phi_9119_;
                                    phi_32954_ = _e6671;
                                    phi_9120_ = _e6673;
                                } else {
                                    phi_32954_ = _e324;
                                    phi_9120_ = _e1438;
                                }
                                let _e6675 = phi_32954_;
                                let _e6677 = phi_9120_;
                                phi_32950_ = _e6675;
                                phi_10393_ = (vec3<f32>(_e335.member_2.x, _e335.member_2.y, _e335.member_2.z) * _e6677);
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
                                    phi_20783_ = type_58(0u, type_58().member_1);
                                    phi_20784_ = type_58();
                                    phi_20785_ = true;
                                } else {
                                    if (_e438 < 0f) {
                                        phi_20755_ = -(sqrt(_e453));
                                    } else {
                                        phi_20755_ = sqrt(_e453);
                                    }
                                    let _e460 = phi_20755_;
                                    let _e462 = -((_e438 + _e460));
                                    let _e463 = (_e462 == 0f);
                                    if _e463 {
                                        phi_20778_ = type_58(1u, type_57(0f, 0f));
                                        phi_20779_ = type_58();
                                    } else {
                                        let _e464 = (_e451 / _e462);
                                        if (_e462 < _e464) {
                                            phi_20767_ = type_57(_e462, _e464);
                                        } else {
                                            phi_20767_ = type_57(_e464, _e462);
                                        }
                                        let _e469 = phi_20767_;
                                        phi_20778_ = type_58();
                                        phi_20779_ = type_58(1u, _e469);
                                    }
                                    let _e472 = phi_20778_;
                                    let _e474 = phi_20779_;
                                    phi_20783_ = _e472;
                                    phi_20784_ = _e474;
                                    phi_20785_ = _e463;
                                }
                                let _e479 = phi_20783_;
                                let _e481 = phi_20784_;
                                let _e483 = phi_20785_;
                                if _e483 {
                                    phi_20789_ = _e479;
                                } else {
                                    phi_20789_ = _e481;
                                }
                                let _e485 = phi_20789_;
                                if (bitcast<i32>(_e485.member) != 0i) {
                                    phi_20676_ = type_58(0u, _e485.member_1);
                                } else {
                                    phi_20676_ = type_58(1u, type_58().member_1);
                                }
                                let _e495 = phi_20676_;
                                if (bitcast<i32>(_e495.member) != 0i) {
                                    phi_20723_ = type_58(0u, type_58().member_1);
                                } else {
                                    if (_e495.member_1.member_1 <= 0f) {
                                        phi_20719_ = type_58(0u, type_58().member_1);
                                    } else {
                                        if (_e495.member_1.member != _e495.member_1.member) {
                                            phi_20827_ = true;
                                        } else {
                                            phi_20827_ = (0f >= _e495.member_1.member);
                                        }
                                        let _e507 = phi_20827_;
                                        let _e508 = select(_e495.member_1.member, 0f, _e507);
                                        let _e509 = (_e450 + _e376);
                                        if (_e509 > 0f) {
                                            let _e511 = (_e452 - _e509);
                                            if (_e511 < 0f) {
                                                phi_20935_ = type_58(0u, type_58().member_1);
                                                phi_20936_ = type_58();
                                                phi_20937_ = true;
                                            } else {
                                                if (_e438 < 0f) {
                                                    phi_20907_ = -(sqrt(_e511));
                                                } else {
                                                    phi_20907_ = sqrt(_e511);
                                                }
                                                let _e518 = phi_20907_;
                                                let _e520 = -((_e438 + _e518));
                                                let _e521 = (_e520 == 0f);
                                                if _e521 {
                                                    phi_20930_ = type_58(1u, type_57(0f, 0f));
                                                    phi_20931_ = type_58();
                                                } else {
                                                    let _e522 = (_e509 / _e520);
                                                    if (_e520 < _e522) {
                                                        phi_20919_ = type_57(_e520, _e522);
                                                    } else {
                                                        phi_20919_ = type_57(_e522, _e520);
                                                    }
                                                    let _e527 = phi_20919_;
                                                    phi_20930_ = type_58();
                                                    phi_20931_ = type_58(1u, _e527);
                                                }
                                                let _e530 = phi_20930_;
                                                let _e532 = phi_20931_;
                                                phi_20935_ = _e530;
                                                phi_20936_ = _e532;
                                                phi_20937_ = _e521;
                                            }
                                            let _e537 = phi_20935_;
                                            let _e539 = phi_20936_;
                                            let _e541 = phi_20937_;
                                            if _e541 {
                                                phi_20941_ = _e537;
                                            } else {
                                                phi_20941_ = _e539;
                                            }
                                            let _e543 = phi_20941_;
                                            if (bitcast<i32>(_e543.member) != 0i) {
                                                phi_20875_ = select(_e543.member_1.member_1, _e543.member_1.member, (_e543.member_1.member >= 0f));
                                            } else {
                                                phi_20875_ = -1f;
                                            }
                                            let _e554 = phi_20875_;
                                            if (_e554 >= 0f) {
                                                if (_e495.member_1.member_1 != _e495.member_1.member_1) {
                                                    phi_20979_ = true;
                                                } else {
                                                    phi_20979_ = (_e554 <= _e495.member_1.member_1);
                                                }
                                                let _e559 = phi_20979_;
                                                phi_20704_ = select(_e495.member_1.member_1, _e554, _e559);
                                            } else {
                                                phi_20704_ = _e495.member_1.member_1;
                                            }
                                            let _e562 = phi_20704_;
                                            phi_20706_ = _e562;
                                        } else {
                                            phi_20706_ = _e495.member_1.member_1;
                                        }
                                        let _e564 = phi_20706_;
                                        if (_e564 <= _e508) {
                                            phi_20715_ = type_58(0u, type_58().member_1);
                                        } else {
                                            phi_20715_ = type_58(1u, type_57(_e508, _e564));
                                        }
                                        let _e572 = phi_20715_;
                                        phi_20719_ = _e572;
                                    }
                                    let _e577 = phi_20719_;
                                    phi_20723_ = _e577;
                                }
                                let _e582 = phi_20723_;
                                if (bitcast<i32>(_e582.member) != 0i) {
                                    let _e588 = (_e582.member_1.member < _e335.member.w);
                                    if _e588 {
                                        if (_e582.member_1.member_1 != _e582.member_1.member_1) {
                                            phi_20994_ = true;
                                        } else {
                                            phi_20994_ = (_e335.member.w <= _e582.member_1.member_1);
                                        }
                                        let _e594 = phi_20994_;
                                        let _e595 = select(_e582.member_1.member_1, _e335.member.w, _e594);
                                        phi_9514_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9517_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9519_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9521_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9523_ = 0u;
                                        loop {
                                            let _e597 = phi_9514_;
                                            let _e599 = phi_9517_;
                                            let _e601 = phi_9519_;
                                            let _e603 = phi_9521_;
                                            let _e605 = phi_9523_;
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
                                                    phi_32469_ = true;
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
                                                    phi_9624_ = type_57((_e582.member_1.member + ((_e615 * _e696) * _e696)), (_e615 * ((_e694 * _e694) - (_e691 * _e691))));
                                                } else {
                                                    if (_e614 <= _e616) {
                                                        let _e674 = (f32((31u - _e605)) * 0.03125f);
                                                        let _e677 = (f32((32u - _e605)) * 0.03125f);
                                                        let _e679 = ((_e674 + _e677) * 0.5f);
                                                        phi_9623_ = type_57((_e595 - ((_e615 * _e679) * _e679)), (_e615 * ((_e677 * _e677) - (_e674 * _e674))));
                                                    } else {
                                                        let _e622 = round(((32f * _e613) / (_e613 + _e614)));
                                                        let _e627 = select(select(u32(_e622), 0u, (_e622 < 0f)), 4294967295u, (_e622 > 4294967000f));
                                                        if (_e627 < 1u) {
                                                            phi_9607_ = 1u;
                                                        } else {
                                                            phi_9607_ = select(_e627, 31u, (_e627 > 31u));
                                                        }
                                                        let _e632 = phi_9607_;
                                                        if (_e605 < _e632) {
                                                            let _e653 = ((_e632 - 1u) - _e605);
                                                            let _e655 = f32(_e632);
                                                            let _e656 = (f32(_e653) / _e655);
                                                            let _e659 = (f32((_e653 + 1u)) / _e655);
                                                            let _e661 = ((_e656 + _e659) * 0.5f);
                                                            phi_9622_ = type_57((_e612 - ((_e613 * _e661) * _e661)), (_e613 * ((_e659 * _e659) - (_e656 * _e656))));
                                                        } else {
                                                            let _e635 = (_e605 - _e632);
                                                            let _e637 = f32((32u - _e632));
                                                            let _e638 = (f32(_e635) / _e637);
                                                            let _e641 = (f32((_e635 + 1u)) / _e637);
                                                            let _e643 = ((_e638 + _e641) * 0.5f);
                                                            phi_9622_ = type_57((_e612 + ((_e614 * _e643) * _e643)), (_e614 * ((_e641 * _e641) - (_e638 * _e638))));
                                                        }
                                                        let _e671 = phi_9622_;
                                                        phi_9623_ = _e671;
                                                    }
                                                    let _e689 = phi_9623_;
                                                    phi_9624_ = _e689;
                                                }
                                                let _e706 = phi_9624_;
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
                                                    phi_21194_ = true;
                                                } else {
                                                    phi_21194_ = (0f >= _e743);
                                                }
                                                let _e747 = phi_21194_;
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
                                                    phi_9952_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e798 = (1f / _e737);
                                                    let _e806 = ((((_e729 * _e798) * _e382) + ((_e730 * _e798) * _e385)) + ((_e731 * _e798) * _e388));
                                                    if (_e739 != _e739) {
                                                        phi_21321_ = true;
                                                    } else {
                                                        phi_21321_ = (0f >= _e739);
                                                    }
                                                    let _e810 = phi_21321_;
                                                    let _e814 = sqrt((select(_e739, 0f, _e810) / (_e373 - _e370)));
                                                    if (_e814 != _e814) {
                                                        phi_21336_ = true;
                                                    } else {
                                                        phi_21336_ = (1f <= _e814);
                                                    }
                                                    let _e818 = phi_21336_;
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
                                                    phi_9952_ = vec3<f32>(((((_e855 * _e862) + (_e868 * _e844)) * _e881) + (((_e886 * _e862) + (_e898 * _e844)) * _e842)), ((((_e858 * _e862) + (_e871 * _e844)) * _e881) + (((_e889 * _e862) + (_e901 * _e844)) * _e842)), ((((_e861 * _e862) + (_e874 * _e844)) * _e881) + (((_e892 * _e862) + (_e904 * _e844)) * _e842)));
                                                }
                                                let _e919 = phi_9952_;
                                                let _e933 = (_e727 + _e379);
                                                if (_e933 > 0f) {
                                                    let _e939 = (((_e729 * _e382) + (_e730 * _e385)) + (_e731 * _e388));
                                                    let _e941 = ((_e939 * _e939) - _e933);
                                                    if (_e939 >= 0f) {
                                                        phi_10026_ = vec3<f32>();
                                                        phi_10027_ = true;
                                                    } else {
                                                        let _e943 = (_e941 <= 0f);
                                                        if _e943 {
                                                            phi_10025_ = vec3<f32>();
                                                        } else {
                                                            let _e946 = (-(_e939) - sqrt(_e941));
                                                            phi_10025_ = vec3<f32>((_e712 + (_e382 * _e946)), (_e713 + (_e385 * _e946)), (_e714 + (_e388 * _e946)));
                                                        }
                                                        let _e955 = phi_10025_;
                                                        phi_10026_ = _e955;
                                                        phi_10027_ = _e943;
                                                    }
                                                    let _e957 = phi_10026_;
                                                    let _e959 = phi_10027_;
                                                    phi_10032_ = _e957;
                                                    phi_10033_ = select(true, false, _e959);
                                                } else {
                                                    phi_10032_ = vec3<f32>(_e712, _e713, _e714);
                                                    phi_10033_ = true;
                                                }
                                                let _e962 = phi_10032_;
                                                let _e964 = phi_10033_;
                                                if _e964 {
                                                    let _e966 = (_e962.x - _e361);
                                                    let _e968 = (_e962.y - _e364);
                                                    let _e970 = (_e962.z - _e367);
                                                    let _e976 = sqrt((((_e966 * _e966) + (_e968 * _e968)) + (_e970 * _e970)));
                                                    let _e977 = (1f / _e976);
                                                    let _e995 = ((((((_e962.x * _e962.x) + (_e962.y * _e962.y)) + (_e962.z * _e962.z)) - (2f * (((_e962.x * _e361) + (_e962.y * _e364)) + (_e962.z * _e367)))) + _e376) / (_e976 + _e370));
                                                    if (_e995 != _e995) {
                                                        phi_21460_ = true;
                                                    } else {
                                                        phi_21460_ = (0f >= _e995);
                                                    }
                                                    let _e999 = phi_21460_;
                                                    let _e1003 = sqrt((select(_e995, 0f, _e999) / (_e373 - _e370)));
                                                    if (_e1003 != _e1003) {
                                                        phi_21475_ = true;
                                                    } else {
                                                        phi_21475_ = (1f <= _e1003);
                                                    }
                                                    let _e1007 = phi_21475_;
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
                                                    phi_10324_ = vec3<f32>(((((_e1049 * _e1056) + (_e1062 * _e1038)) * _e1075) + (((_e1080 * _e1056) + (_e1092 * _e1038)) * _e1036)), ((((_e1052 * _e1056) + (_e1065 * _e1038)) * _e1075) + (((_e1083 * _e1056) + (_e1095 * _e1038)) * _e1036)), ((((_e1055 * _e1056) + (_e1068 * _e1038)) * _e1075) + (((_e1086 * _e1056) + (_e1098 * _e1038)) * _e1036)));
                                                } else {
                                                    phi_10324_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1113 = phi_10324_;
                                                let _e1114 = (vec3<f32>(_e761, _e763, _e765) + _e1113);
                                                let _e1120 = (_e770 * _e1114.y);
                                                let _e1132 = exp(-((((_e400 * _e1114.x) + _e1120) + (_e416 * _e1114.z))));
                                                let _e1134 = exp(-((((_e403 * _e1114.x) + _e1120) + (_e419 * _e1114.z))));
                                                let _e1136 = exp(-((((_e406 * _e1114.x) + _e1120) + (_e422 * _e1114.z))));
                                                phi_9515_ = vec3<f32>((_e597.x + ((exp(-((((_e400 * _e761) + _e771) + (_e416 * _e765)))) * ((_e400 * _e752) + _e790)) * _e919.x)), (_e597.y + ((exp(-((((_e403 * _e761) + _e771) + (_e419 * _e765)))) * ((_e403 * _e752) + _e790)) * _e919.y)), (_e597.z + ((exp(-((((_e406 * _e761) + _e771) + (_e422 * _e765)))) * ((_e406 * _e752) + _e790)) * _e919.z)));
                                                phi_9518_ = vec3<f32>((_e599.x + (_e1132 * _e755)), (_e599.y + (_e1134 * _e755)), (_e599.z + (_e1136 * _e755)));
                                                phi_9520_ = vec3<f32>((_e601.x + (_e1132 * _e752)), (_e601.y + (_e1134 * _e752)), (_e601.z + (_e1136 * _e752)));
                                                phi_9522_ = vec3<f32>((_e761 + _e757), (_e763 + _e758), (_e765 + _e759));
                                                phi_9524_ = (_e605 + 1u);
                                            } else {
                                                phi_9515_ = vec3<f32>();
                                                phi_9518_ = vec3<f32>();
                                                phi_9520_ = vec3<f32>();
                                                phi_9522_ = vec3<f32>();
                                                phi_9524_ = u32();
                                            }
                                            let _e1163 = phi_9515_;
                                            let _e1165 = phi_9518_;
                                            let _e1167 = phi_9520_;
                                            let _e1169 = phi_9522_;
                                            let _e1171 = phi_9524_;
                                            phi_9514_ = _e1163;
                                            phi_9517_ = _e1165;
                                            phi_9519_ = _e1167;
                                            phi_9521_ = _e1169;
                                            phi_9523_ = _e1171;
                                            phi_32469_ = _e324;
                                            if !(_e606) {
                                                break;
                                            }
                                        }
                                        let _e1174 = phi_32469_;
                                        phi_32947_ = _e1174;
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
                                        phi_32953_ = _e1174;
                                        phi_10381_ = type_61(vec3<f32>(exp(-((((_e400 * _e1234.x) + _e1243) + (_e416 * _e1248.z)))), exp(-((((_e403 * _e1234.x) + _e1243) + (_e419 * _e1248.z)))), exp(-((((_e406 * _e1234.x) + _e1243) + (_e422 * _e1248.z))))), (vec3<f32>(((((_e1177.x * _e1188) + (_e1180.x * _e1206)) + _e1217.x) * _e397), ((((_e1177.y * _e1188) + (_e1180.y * _e1206)) + _e1221.y) * _e397), ((((_e1177.z * _e1188) + (_e1180.z * _e1206)) + _e1225.z) * _e397)) * 0.38317f));
                                    } else {
                                        phi_32953_ = _e324;
                                        phi_10381_ = type_61();
                                    }
                                    let _e1265 = phi_32953_;
                                    let _e1267 = phi_10381_;
                                    phi_32952_ = _e1265;
                                    phi_10383_ = _e1267;
                                    phi_10384_ = select(true, false, _e588);
                                } else {
                                    phi_32952_ = _e324;
                                    phi_10383_ = type_61();
                                    phi_10384_ = true;
                                }
                                let _e1270 = phi_32952_;
                                let _e1272 = phi_10383_;
                                let _e1274 = phi_10384_;
                                if _e1274 {
                                    phi_10389_ = type_61(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10389_ = _e1272;
                                }
                                let _e1276 = phi_10389_;
                                phi_32950_ = _e1270;
                                phi_10393_ = (vec3<f32>(_e335.member_2.x, _e335.member_2.y, _e335.member_2.z) * _e1276.member_1);
                            }
                            let _e6681 = phi_32950_;
                            let _e6683 = phi_10393_;
                            phi_32949_ = _e6681;
                            phi_10394_ = _e6683;
                        }
                        let _e6685 = phi_32949_;
                        let _e6687 = phi_10394_;
                        phi_32948_ = _e6685;
                        phi_512_ = (_e326 + _e6687);
                        phi_515_ = (_e328 + 1u);
                    } else {
                        phi_32948_ = _e324;
                        phi_512_ = vec3<f32>();
                        phi_515_ = u32();
                    }
                    let _e6691 = phi_32948_;
                    let _e6693 = phi_512_;
                    let _e6695 = phi_515_;
                    phi_32486_ = _e6691;
                    phi_511_ = _e6693;
                    phi_514_ = _e6695;
                    phi_32947_ = _e6691;
                    if !(_e329) {
                        break;
                    }
                }
                let _e6698 = phi_32947_;
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
    var local_69: array<u32, 64>;
    var local_70: array<f32, 64>;
    var phi_11196_: type_49;
    var phi_11312_: u32;
    var phi_11315_: u32;
    var phi_11317_: u32;
    var phi_11319_: u32;
    var phi_11321_: vec4<f32>;
    var phi_11323_: type_68;
    var phi_11679_: u32;
    var phi_11681_: u32;
    var phi_11682_: u32;
    var phi_11684_: bool;
    var phi_11685_: bool;
    var phi_11686_: u32;
    var phi_11687_: u32;
    var phi_11688_: u32;
    var phi_11689_: bool;
    var phi_11690_: bool;
    var phi_11691_: bool;
    var phi_25056_: bool;
    var phi_25086_: bool;
    var phi_25033_: type_57;
    var phi_25036_: type_57;
    var phi_25037_: bool;
    var phi_25042_: type_57;
    var phi_25143_: bool;
    var phi_25158_: bool;
    var phi_25173_: bool;
    var phi_25188_: bool;
    var phi_25120_: type_57;
    var phi_25123_: type_57;
    var phi_25124_: bool;
    var phi_25129_: type_57;
    var phi_25230_: bool;
    var phi_25245_: bool;
    var phi_25260_: bool;
    var phi_25275_: bool;
    var phi_25207_: type_57;
    var phi_25210_: type_57;
    var phi_25211_: bool;
    var phi_25216_: type_57;
    var phi_25011_: bool;
    var phi_25014_: bool;
    var phi_11721_: f32;
    var phi_25377_: bool;
    var phi_25407_: bool;
    var phi_25354_: type_57;
    var phi_25357_: type_57;
    var phi_25358_: bool;
    var phi_25363_: type_57;
    var phi_25464_: bool;
    var phi_25479_: bool;
    var phi_25494_: bool;
    var phi_25509_: bool;
    var phi_25441_: type_57;
    var phi_25444_: type_57;
    var phi_25445_: bool;
    var phi_25450_: type_57;
    var phi_25551_: bool;
    var phi_25566_: bool;
    var phi_25581_: bool;
    var phi_25596_: bool;
    var phi_25528_: type_57;
    var phi_25531_: type_57;
    var phi_25532_: bool;
    var phi_25537_: type_57;
    var phi_25332_: bool;
    var phi_25335_: bool;
    var phi_11731_: f32;
    var phi_11742_: u32;
    var phi_11753_: u32;
    var phi_11792_: u32;
    var phi_11793_: u32;
    var phi_11804_: u32;
    var phi_11805_: u32;
    var phi_11806_: u32;
    var phi_11807_: u32;
    var phi_11809_: u32;
    var phi_11810_: u32;
    var phi_11812_: u32;
    var phi_11813_: u32;
    var phi_11814_: bool;
    var phi_11815_: bool;
    var phi_11602_: vec4<f32>;
    var phi_11603_: vec4<f32>;
    var phi_11604_: bool;
    var phi_11608_: vec4<f32>;
    var phi_11609_: bool;
    var phi_11519_: bool;
    var phi_11553_: bool;
    var phi_11559_: f32;
    var phi_11569_: vec4<f32>;
    var phi_11570_: vec4<f32>;
    var phi_11571_: bool;
    var phi_11575_: vec4<f32>;
    var phi_11576_: bool;
    var phi_11580_: vec4<f32>;
    var phi_11581_: bool;
    var phi_11610_: vec4<f32>;
    var phi_11611_: bool;
    var phi_11451_: vec4<f32>;
    var phi_11452_: vec4<f32>;
    var phi_11453_: bool;
    var phi_11457_: vec4<f32>;
    var phi_11458_: bool;
    var phi_11459_: vec4<f32>;
    var phi_11460_: bool;
    var phi_11461_: bool;
    var phi_11469_: vec4<f32>;
    var phi_11470_: bool;
    var phi_11471_: vec4<f32>;
    var phi_11472_: bool;
    var phi_11473_: bool;
    var phi_11481_: vec4<f32>;
    var phi_11482_: bool;
    var phi_11620_: vec4<f32>;
    var phi_11621_: bool;
    var phi_11642_: type_68;
    var phi_11644_: type_68;
    var phi_11313_: u32;
    var phi_11316_: u32;
    var phi_11318_: u32;
    var phi_11320_: u32;
    var phi_11322_: vec4<f32>;
    var phi_11324_: type_68;
    var phi_11826_: type_68;
    var phi_11827_: bool;
    var phi_11828_: bool;
    var phi_32958_: bool;
    var phi_20650_: bool;
    var phi_20649_: type_68;
    var local_71: type_68;
    var local_72: vec4<f32>;
    var phi_11989_: type_61;
    var local_73: vec4<f32>;
    var local_74: vec4<f32>;
    var phi_11955_: vec3<f32>;
    var phi_11956_: vec3<f32>;
    var phi_11990_: type_61;
    var local_75: type_68;
    var phi_11995_: type_68;
    var phi_11996_: type_68;
    var phi_12001_: type_68;
    var phi_25715_: f32;
    var phi_25727_: type_57;
    var phi_25738_: type_58;
    var phi_25739_: type_58;
    var phi_25743_: type_58;
    var phi_25744_: type_58;
    var phi_25745_: bool;
    var phi_25749_: type_58;
    var phi_25636_: type_58;
    var phi_25787_: bool;
    var phi_25867_: f32;
    var phi_25879_: type_57;
    var phi_25890_: type_58;
    var phi_25891_: type_58;
    var phi_25895_: type_58;
    var phi_25896_: type_58;
    var phi_25897_: bool;
    var phi_25901_: type_58;
    var phi_25835_: f32;
    var phi_25939_: bool;
    var phi_25664_: f32;
    var phi_25666_: f32;
    var phi_25675_: type_58;
    var phi_25679_: type_58;
    var phi_25683_: type_58;
    var phi_25954_: bool;
    var phi_13420_: vec3<f32>;
    var phi_13423_: u32;
    var phi_13507_: u32;
    var phi_13522_: type_57;
    var phi_13523_: type_57;
    var phi_13524_: type_57;
    var phi_26153_: bool;
    var phi_13421_: vec3<f32>;
    var phi_13424_: u32;
    var phi_33080_: bool;
    var local_76: vec3<f32>;
    var local_77: vec3<f32>;
    var local_78: vec3<f32>;
    var phi_35348_: bool;
    var phi_13549_: vec3<f32>;
    var phi_35347_: bool;
    var phi_13551_: vec3<f32>;
    var phi_13552_: bool;
    var phi_35345_: bool;
    var phi_13559_: vec3<f32>;
    var phi_13560_: type_42;
    var phi_35344_: bool;
    var phi_13561_: vec3<f32>;
    var phi_13562_: type_42;
    var phi_13586_: i32;
    var phi_13589_: u32;
    var phi_13643_: bool;
    var phi_13649_: bool;
    var phi_13656_: u32;
    var phi_13658_: i32;
    var phi_13659_: u32;
    var phi_13587_: i32;
    var phi_13590_: u32;
    var phi_13661_: bool;
    var local_79: u32;
    var phi_13942_: vec3<f32>;
    var phi_13959_: vec3<f32>;
    var phi_26220_: bool;
    var phi_13992_: f32;
    var phi_14101_: vec3<f32>;
    var phi_14107_: f32;
    var phi_26293_: bool;
    var phi_26282_: f32;
    var phi_14188_: vec3<f32>;
    var phi_14189_: vec3<f32>;
    var phi_14190_: vec3<f32>;
    var phi_33345_: u32;
    var phi_14192_: vec3<f32>;
    var phi_14193_: vec3<f32>;
    var phi_14194_: vec3<f32>;
    var phi_14195_: bool;
    var phi_14210_: vec3<f32>;
    var phi_14211_: vec3<f32>;
    var phi_33343_: u32;
    var phi_14213_: vec3<f32>;
    var phi_14214_: vec3<f32>;
    var phi_14215_: vec3<f32>;
    var phi_33342_: u32;
    var phi_14216_: vec3<f32>;
    var phi_14217_: vec3<f32>;
    var phi_14218_: vec3<f32>;
    var phi_14219_: bool;
    var phi_33341_: u32;
    var phi_14220_: vec3<f32>;
    var phi_14221_: vec3<f32>;
    var phi_14222_: vec3<f32>;
    var phi_14223_: bool;
    var phi_14224_: bool;
    var phi_26366_: bool;
    var phi_26528_: bool;
    var phi_26543_: bool;
    var phi_26558_: bool;
    var phi_26585_: bool;
    var phi_26600_: bool;
    var phi_26615_: bool;
    var phi_26642_: bool;
    var phi_26657_: bool;
    var phi_26672_: bool;
    var phi_14403_: f32;
    var phi_14411_: f32;
    var phi_14419_: f32;
    var phi_26869_: bool;
    var phi_26858_: f32;
    var phi_14526_: vec3<f32>;
    var phi_26897_: bool;
    var phi_14527_: vec3<f32>;
    var phi_14528_: vec3<f32>;
    var phi_14529_: vec3<f32>;
    var phi_33729_: vec3<f32>;
    var phi_33652_: vec3<f32>;
    var phi_33573_: u32;
    var phi_33491_: f32;
    var phi_14531_: vec3<f32>;
    var phi_14532_: vec3<f32>;
    var phi_14533_: bool;
    var phi_26912_: bool;
    var phi_26963_: bool;
    var phi_26952_: f32;
    var phi_33700_: vec3<f32>;
    var phi_14065_: vec3<f32>;
    var phi_34052_: vec3<f32>;
    var phi_33846_: f32;
    var phi_33699_: vec3<f32>;
    var phi_33387_: u32;
    var phi_14067_: vec3<f32>;
    var phi_14068_: bool;
    var phi_33998_: vec3<f32>;
    var phi_33806_: f32;
    var phi_33727_: vec3<f32>;
    var phi_33650_: vec3<f32>;
    var phi_33571_: u32;
    var phi_33489_: f32;
    var phi_33412_: u32;
    var phi_33338_: u32;
    var phi_14541_: vec3<f32>;
    var phi_14542_: vec3<f32>;
    var phi_14543_: vec3<f32>;
    var phi_14544_: bool;
    var phi_27029_: bool;
    var phi_14613_: vec3<f32>;
    var phi_33395_: u32;
    var phi_14615_: vec3<f32>;
    var phi_14616_: bool;
    var phi_33394_: u32;
    var phi_14617_: vec3<f32>;
    var phi_14618_: bool;
    var phi_33392_: u32;
    var phi_14624_: vec3<f32>;
    var phi_14625_: bool;
    var phi_14658_: type_73;
    var phi_33397_: u32;
    var phi_14660_: vec3<f32>;
    var phi_14661_: type_73;
    var phi_14662_: bool;
    var phi_14668_: type_73;
    var phi_33401_: u32;
    var phi_14669_: type_73;
    var phi_33995_: vec3<f32>;
    var phi_33803_: f32;
    var phi_33724_: vec3<f32>;
    var phi_33647_: vec3<f32>;
    var phi_33568_: u32;
    var phi_33486_: f32;
    var phi_33409_: u32;
    var phi_33400_: u32;
    var phi_14671_: type_73;
    var phi_14672_: bool;
    var phi_35324_: bool;
    var phi_33399_: u32;
    var phi_14683_: vec3<f32>;
    var phi_14686_: type_42;
    var phi_14688_: u32;
    var phi_14690_: u32;
    var phi_17379_: u32;
    var phi_17382_: u32;
    var phi_17380_: u32;
    var phi_17383_: u32;
    var local_80: u32;
    var phi_17420_: u32;
    var phi_17423_: u32;
    var phi_17421_: u32;
    var phi_17424_: u32;
    var local_81: u32;
    var local_82: u32;
    var phi_17500_: f32;
    var local_83: u32;
    var local_84: u32;
    var phi_17549_: f32;
    var phi_17705_: f32;
    var phi_17712_: vec3<f32>;
    var phi_35745_: u32;
    var phi_17714_: f32;
    var phi_17715_: vec3<f32>;
    var phi_17716_: vec3<f32>;
    var phi_16852_: u32;
    var phi_16855_: u32;
    var phi_16853_: u32;
    var phi_16856_: u32;
    var local_85: u32;
    var local_86: u32;
    var phi_17248_: f32;
    var phi_17251_: f32;
    var phi_27302_: bool;
    var phi_27283_: f32;
    var phi_17252_: f32;
    var phi_27317_: bool;
    var phi_27340_: bool;
    var phi_27355_: bool;
    var phi_17003_: f32;
    var phi_27370_: bool;
    var phi_27385_: bool;
    var phi_27400_: bool;
    var phi_27465_: bool;
    var phi_27553_: bool;
    var phi_27534_: f32;
    var phi_17068_: type_76;
    var phi_17254_: type_76;
    var phi_17332_: bool;
    var phi_17341_: vec3<f32>;
    var phi_17343_: vec3<f32>;
    var phi_17344_: bool;
    var phi_17349_: f32;
    var phi_17350_: vec3<f32>;
    var phi_35753_: u32;
    var phi_17351_: f32;
    var phi_17352_: vec3<f32>;
    var phi_17353_: vec3<f32>;
    var phi_17354_: f32;
    var phi_35752_: u32;
    var phi_17355_: f32;
    var phi_17356_: vec3<f32>;
    var phi_17357_: vec3<f32>;
    var phi_17358_: f32;
    var phi_35744_: u32;
    var phi_17717_: f32;
    var phi_17718_: vec3<f32>;
    var phi_17719_: vec3<f32>;
    var phi_17720_: f32;
    var phi_27568_: bool;
    var phi_27678_: f32;
    var phi_27706_: f32;
    var phi_27734_: f32;
    var phi_27868_: bool;
    var phi_27883_: bool;
    var phi_27898_: bool;
    var phi_27925_: bool;
    var phi_27940_: bool;
    var phi_27955_: bool;
    var phi_27982_: bool;
    var phi_27997_: bool;
    var phi_28012_: bool;
    var phi_15024_: f32;
    var phi_15032_: f32;
    var phi_15040_: f32;
    var phi_28209_: bool;
    var phi_28198_: f32;
    var phi_15147_: vec3<f32>;
    var phi_28237_: bool;
    var phi_15148_: vec3<f32>;
    var phi_15160_: type_60;
    var phi_15161_: type_60;
    var phi_15162_: bool;
    var phi_15167_: type_60;
    var phi_28274_: bool;
    var phi_28304_: bool;
    var phi_28293_: f32;
    var phi_28343_: vec3<f32>;
    var phi_15195_: type_60;
    var phi_15196_: type_60;
    var phi_28365_: vec3<f32>;
    var phi_15197_: type_60;
    var phi_15198_: type_60;
    var phi_15199_: bool;
    var phi_15203_: type_60;
    var phi_28397_: bool;
    var phi_28468_: bool;
    var phi_16023_: f32;
    var phi_28483_: bool;
    var phi_28498_: bool;
    var phi_16496_: vec3<f32>;
    var phi_16497_: vec3<f32>;
    var phi_16498_: bool;
    var phi_16503_: vec3<f32>;
    var phi_16504_: bool;
    var phi_28622_: bool;
    var phi_28637_: bool;
    var phi_16795_: vec3<f32>;
    var phi_16797_: vec3<f32>;
    var phi_16824_: vec3<f32>;
    var phi_16825_: vec3<f32>;
    var phi_15641_: vec3<f32>;
    var phi_15642_: vec3<f32>;
    var phi_15643_: bool;
    var phi_15648_: vec3<f32>;
    var phi_15649_: bool;
    var phi_28799_: bool;
    var phi_28814_: bool;
    var phi_15940_: vec3<f32>;
    var phi_15942_: vec3<f32>;
    var phi_35787_: u32;
    var phi_16826_: f32;
    var phi_16827_: vec3<f32>;
    var phi_16828_: vec3<f32>;
    var phi_35786_: u32;
    var phi_16829_: f32;
    var phi_16830_: vec3<f32>;
    var phi_16831_: vec3<f32>;
    var phi_35743_: u32;
    var phi_17721_: f32;
    var phi_17722_: vec3<f32>;
    var phi_17723_: vec3<f32>;
    var phi_17724_: f32;
    var phi_14786_: f32;
    var phi_14787_: f32;
    var phi_35742_: u32;
    var phi_17725_: f32;
    var phi_17726_: vec3<f32>;
    var phi_17727_: vec3<f32>;
    var phi_17728_: f32;
    var phi_28867_: bool;
    var phi_28977_: f32;
    var phi_29005_: f32;
    var phi_29033_: f32;
    var phi_29167_: bool;
    var phi_29182_: bool;
    var phi_29197_: bool;
    var phi_29224_: bool;
    var phi_29239_: bool;
    var phi_29254_: bool;
    var phi_29281_: bool;
    var phi_29296_: bool;
    var phi_29311_: bool;
    var phi_17951_: f32;
    var phi_17959_: f32;
    var phi_17967_: f32;
    var phi_29508_: bool;
    var phi_29497_: f32;
    var phi_18074_: vec3<f32>;
    var phi_29536_: bool;
    var phi_18075_: vec3<f32>;
    var phi_18087_: type_60;
    var phi_18088_: type_60;
    var phi_18089_: bool;
    var phi_18094_: type_60;
    var phi_29573_: bool;
    var phi_29603_: bool;
    var phi_29592_: f32;
    var phi_29642_: vec3<f32>;
    var phi_18122_: type_60;
    var phi_18123_: type_60;
    var phi_29664_: vec3<f32>;
    var phi_18124_: type_60;
    var phi_18125_: type_60;
    var phi_18126_: bool;
    var phi_18130_: type_60;
    var phi_29678_: f32;
    var phi_29681_: f32;
    var phi_18139_: f32;
    var phi_18173_: u32;
    var phi_18176_: u32;
    var phi_18178_: u32;
    var phi_18180_: u32;
    var phi_18184_: type_68;
    var phi_18557_: u32;
    var phi_18559_: u32;
    var phi_18560_: u32;
    var phi_18562_: bool;
    var phi_18563_: bool;
    var phi_18564_: u32;
    var phi_18565_: u32;
    var phi_18566_: u32;
    var phi_18567_: bool;
    var phi_18568_: bool;
    var phi_18569_: bool;
    var phi_29779_: bool;
    var phi_29809_: bool;
    var phi_29756_: type_57;
    var phi_29759_: type_57;
    var phi_29760_: bool;
    var phi_29765_: type_57;
    var phi_29866_: bool;
    var phi_29881_: bool;
    var phi_29896_: bool;
    var phi_29911_: bool;
    var phi_29843_: type_57;
    var phi_29846_: type_57;
    var phi_29847_: bool;
    var phi_29852_: type_57;
    var phi_29953_: bool;
    var phi_29968_: bool;
    var phi_29983_: bool;
    var phi_29998_: bool;
    var phi_29930_: type_57;
    var phi_29933_: type_57;
    var phi_29934_: bool;
    var phi_29939_: type_57;
    var phi_29734_: bool;
    var phi_29737_: bool;
    var phi_18599_: f32;
    var phi_30100_: bool;
    var phi_30130_: bool;
    var phi_30077_: type_57;
    var phi_30080_: type_57;
    var phi_30081_: bool;
    var phi_30086_: type_57;
    var phi_30187_: bool;
    var phi_30202_: bool;
    var phi_30217_: bool;
    var phi_30232_: bool;
    var phi_30164_: type_57;
    var phi_30167_: type_57;
    var phi_30168_: bool;
    var phi_30173_: type_57;
    var phi_30274_: bool;
    var phi_30289_: bool;
    var phi_30304_: bool;
    var phi_30319_: bool;
    var phi_30251_: type_57;
    var phi_30254_: type_57;
    var phi_30255_: bool;
    var phi_30260_: type_57;
    var phi_30055_: bool;
    var phi_30058_: bool;
    var phi_18609_: f32;
    var phi_18620_: u32;
    var phi_18631_: u32;
    var phi_18670_: u32;
    var phi_18671_: u32;
    var phi_18682_: u32;
    var phi_18683_: u32;
    var phi_18684_: u32;
    var phi_18685_: u32;
    var phi_18687_: u32;
    var phi_18688_: u32;
    var phi_18690_: u32;
    var phi_18691_: u32;
    var phi_18692_: bool;
    var phi_18693_: bool;
    var phi_18480_: vec4<f32>;
    var phi_18481_: vec4<f32>;
    var phi_18482_: bool;
    var phi_18486_: vec4<f32>;
    var phi_18487_: bool;
    var phi_18392_: bool;
    var phi_18429_: bool;
    var phi_18435_: f32;
    var phi_18445_: vec4<f32>;
    var phi_18446_: vec4<f32>;
    var phi_18447_: bool;
    var phi_18451_: vec4<f32>;
    var phi_18452_: bool;
    var phi_18456_: vec4<f32>;
    var phi_18457_: bool;
    var phi_18488_: vec4<f32>;
    var phi_18489_: bool;
    var phi_18321_: vec4<f32>;
    var phi_18322_: vec4<f32>;
    var phi_18323_: bool;
    var phi_18327_: vec4<f32>;
    var phi_18328_: bool;
    var phi_18329_: vec4<f32>;
    var phi_18330_: bool;
    var phi_18331_: bool;
    var phi_18339_: vec4<f32>;
    var phi_18340_: bool;
    var phi_18341_: vec4<f32>;
    var phi_18342_: bool;
    var phi_18343_: bool;
    var phi_18351_: vec4<f32>;
    var phi_18352_: bool;
    var phi_18498_: vec4<f32>;
    var phi_18499_: bool;
    var phi_18520_: type_68;
    var phi_18522_: type_68;
    var phi_18528_: u32;
    var phi_18174_: u32;
    var phi_18177_: u32;
    var phi_18179_: u32;
    var phi_18181_: u32;
    var phi_18185_: type_68;
    var phi_18704_: type_68;
    var phi_18705_: bool;
    var phi_18706_: bool;
    var phi_35252_: bool;
    var phi_20652_: bool;
    var phi_20651_: type_68;
    var phi_18878_: type_68;
    var phi_18883_: type_68;
    var phi_18935_: vec3<f32>;
    var phi_18936_: type_42;
    var phi_18937_: vec3<f32>;
    var phi_18938_: type_42;
    var phi_35689_: bool;
    var phi_18939_: vec3<f32>;
    var phi_18940_: type_42;
    var phi_18941_: u32;
    var phi_35688_: bool;
    var phi_18942_: vec3<f32>;
    var phi_18943_: type_42;
    var phi_18944_: u32;
    var phi_35980_: u32;
    var phi_35687_: bool;
    var phi_14684_: vec3<f32>;
    var phi_14687_: type_42;
    var phi_14689_: u32;
    var phi_14691_: u32;
    var phi_35700_: u32;
    var phi_35686_: bool;
    var phi_30470_: bool;
    var phi_30500_: bool;
    var phi_30489_: f32;
    var phi_30420_: bool;
    var phi_30556_: vec3<f32>;
    var phi_30712_: bool;
    var phi_30727_: bool;
    var phi_30742_: bool;
    var phi_30651_: vec3<f32>;
    var phi_30654_: vec3<f32>;
    var phi_30655_: bool;
    var phi_30425_: vec3<f32>;
    var phi_30429_: vec3<f32>;
    var phi_30430_: bool;
    var phi_30842_: bool;
    var phi_30816_: vec3<f32>;
    var phi_30437_: vec3<f32>;
    var phi_30857_: bool;
    var phi_30967_: f32;
    var phi_30995_: f32;
    var phi_31023_: f32;
    var phi_31157_: bool;
    var phi_31172_: bool;
    var phi_31187_: bool;
    var phi_31214_: bool;
    var phi_31229_: bool;
    var phi_31244_: bool;
    var phi_31271_: bool;
    var phi_31286_: bool;
    var phi_31301_: bool;
    var phi_19163_: f32;
    var phi_19171_: f32;
    var phi_19179_: f32;
    var phi_31498_: bool;
    var phi_31487_: f32;
    var phi_19286_: vec3<f32>;
    var phi_31526_: bool;
    var phi_19287_: vec3<f32>;
    var phi_19299_: type_60;
    var phi_19300_: type_60;
    var phi_19301_: bool;
    var phi_19306_: type_60;
    var phi_31563_: bool;
    var phi_31593_: bool;
    var phi_31582_: f32;
    var phi_31632_: vec3<f32>;
    var phi_19334_: type_60;
    var phi_19335_: type_60;
    var phi_31654_: vec3<f32>;
    var phi_19336_: type_60;
    var phi_19337_: type_60;
    var phi_19338_: bool;
    var phi_19342_: type_60;
    var phi_31679_: bool;
    var phi_19418_: vec3<f32>;
    var phi_37582_: u32;
    var phi_19420_: vec3<f32>;
    var phi_19421_: bool;
    var phi_37581_: u32;
    var phi_19422_: vec3<f32>;
    var phi_19423_: bool;
    var phi_37579_: u32;
    var phi_19429_: vec3<f32>;
    var phi_19430_: bool;
    var local_87: u32;
    var local_88: vec3<f32>;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: type_42;
    var phi_19459_: vec3<f32>;
    var local_92: u32;
    var local_93: vec3<f32>;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: type_42;
    var phi_19475_: type_73;
    var phi_19476_: type_73;
    var phi_37586_: u32;
    var phi_19478_: type_73;
    var phi_19479_: type_73;
    var phi_19480_: bool;
    var phi_37585_: u32;
    var phi_19481_: type_73;
    var phi_19482_: type_73;
    var phi_19483_: bool;
    var phi_19484_: bool;
    var local_97: u32;
    var local_98: vec3<f32>;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: type_42;
    var phi_19490_: type_73;
    var local_102: u32;
    var local_103: vec3<f32>;
    var local_104: vec3<f32>;
    var local_105: vec3<f32>;
    var local_106: type_42;
    var phi_19495_: type_73;
    var phi_19496_: type_73;
    var phi_19497_: bool;
    var phi_19501_: type_73;
    var phi_19502_: type_73;
    var phi_19503_: bool;
    var phi_13721_: bool;
    var phi_13730_: vec3<f32>;
    var phi_13732_: vec3<f32>;
    var phi_13733_: bool;
    var phi_13894_: f32;
    var phi_13897_: f32;
    var phi_31900_: bool;
    var phi_31881_: f32;
    var phi_13898_: f32;
    var phi_31918_: f32;
    var phi_31921_: f32;
    var phi_13901_: f32;
    var phi_13902_: f32;
    var phi_13903_: f32;
    var phi_19507_: type_73;
    var phi_19508_: type_73;
    var phi_19509_: bool;
    var phi_12157_: f32;
    var phi_31968_: f32;
    var phi_31971_: f32;
    var phi_12159_: f32;
    var phi_12160_: f32;
    var phi_31997_: bool;
    var phi_12211_: f32;
    var phi_32015_: f32;
    var phi_32018_: f32;
    var phi_12213_: f32;
    var phi_32029_: bool;
    var phi_32044_: bool;
    var phi_12682_: vec3<f32>;
    var phi_12683_: vec3<f32>;
    var phi_12684_: bool;
    var phi_12689_: vec3<f32>;
    var phi_12690_: bool;
    var phi_32168_: bool;
    var phi_32183_: bool;
    var phi_12981_: vec3<f32>;
    var phi_12983_: vec3<f32>;
    var phi_13010_: vec3<f32>;
    var phi_13024_: vec3<f32>;
    var phi_19513_: type_73;
    var phi_19514_: type_73;
    var phi_19515_: bool;
    var phi_19519_: type_73;
    var local_107: type_68;
    var local_108: type_68;

    switch bitcast<i32>(0u) {
        default: {
            let _e281 = global;
            let _e291 = arrayLength((&global_9.member));
            let _e295 = global_1.member.member_7[2u];
            let _e299 = (_e281.x + (_e281.y * 4194240u));
            let _e304 = atomicLoad((&global_8.member[((_e295 * 4u) + 3u)]));
            if (_e299 >= _e304) {
                phi_11196_ = type_49(0u, type_49().member_1);
            } else {
                let _e309 = global_1.member.member[0u];
                let _e314 = global_1.member.member[1u];
                let _e320 = atomicLoad((&global_8.member[((8u + ((_e295 * _e309) * _e314)) + _e299)]));
                phi_11196_ = type_49(1u, _e320);
            }
            let _e326 = phi_11196_;
            if (bitcast<i32>(_e326.member) != 0i) {
                let _e331 = (_e326.member_1 < arrayLength((&global_7.member)));
                if _e331 {
                } else {
                    break;
                }
                let _e334 = global_7.member[_e326.member_1];
                let _e347 = vec3<f32>(_e334.member_1.x, _e334.member_1.y, _e334.member_1.z);
                let _e352 = vec3<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z);
                let _e358 = select(select(u32(_e334.member_2.w), 0u, (_e334.member_2.w < 0f)), 4294967295u, (_e334.member_2.w > 4294967000f));
                let _e362 = vec3<f32>(_e334.member_3.x, _e334.member_3.y, _e334.member_3.z);
                let _e372 = ((_e334.member_4.y & 2u) != 0u);
                let _e375 = global_1.member.member[3u];
                let _e378 = global_1.member.member_3[3u];
                let _e381 = global_1.member.member_1[2u];
                let _e384 = global_1.member.member_1[0u];
                let _e387 = global_1.member.member_1[1u];
                let _e388 = (_e384 + _e387);
                let _e389 = (vec3<f32>(1f, 1f, 1f) / _e347);
                local_69 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                local_70 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                let _e391 = select(4294967295u, 0u, (_e384 > 0u));
                phi_11312_ = _e391;
                phi_11315_ = 0u;
                phi_11317_ = _e388;
                phi_11319_ = _e384;
                phi_11321_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_11323_ = type_68(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                loop {
                    let _e393 = phi_11312_;
                    let _e395 = phi_11315_;
                    let _e397 = phi_11317_;
                    let _e399 = phi_11319_;
                    let _e401 = phi_11321_;
                    let _e403 = phi_11323_;
                    local_71 = _e403;
                    local_72 = _e401;
                    local_73 = _e401;
                    local_74 = _e401;
                    local_75 = _e403;
                    local_107 = _e403;
                    if (_e399 < _e397) {
                        let _e771 = global_2.member[_e399];
                        if (_e771.member_6.x == 0u) {
                            let _e877 = (_e771.member_1.x - _e771.member.x);
                            let _e878 = (_e771.member_1.y - _e771.member.y);
                            let _e879 = (_e771.member_1.z - _e771.member.z);
                            let _e884 = (_e771.member_2.x - _e771.member.x);
                            let _e885 = (_e771.member_2.y - _e771.member.y);
                            let _e886 = (_e771.member_2.z - _e771.member.z);
                            let _e889 = ((_e334.member_1.y * _e886) - (_e885 * _e334.member_1.z));
                            let _e892 = ((_e334.member_1.z * _e884) - (_e886 * _e334.member_1.x));
                            let _e895 = ((_e334.member_1.x * _e885) - (_e884 * _e334.member_1.y));
                            let _e900 = (((_e877 * _e889) + (_e878 * _e892)) + (_e879 * _e895));
                            if (abs(_e900) < 0.000000000001f) {
                                phi_11481_ = vec4<f32>();
                                phi_11482_ = true;
                            } else {
                                let _e903 = (1f / _e900);
                                let _e904 = (_e334.member.x - _e771.member.x);
                                let _e905 = (_e334.member.y - _e771.member.y);
                                let _e906 = (_e334.member.z - _e771.member.z);
                                let _e912 = ((((_e904 * _e889) + (_e905 * _e892)) + (_e906 * _e895)) * _e903);
                                if (_e912 < 0f) {
                                    phi_11471_ = vec4<f32>();
                                    phi_11472_ = true;
                                    phi_11473_ = false;
                                } else {
                                    let _e914 = (_e912 > 1f);
                                    if _e914 {
                                        phi_11469_ = vec4<f32>();
                                        phi_11470_ = false;
                                    } else {
                                        let _e917 = ((_e905 * _e879) - (_e878 * _e906));
                                        let _e920 = ((_e906 * _e877) - (_e879 * _e904));
                                        let _e923 = ((_e904 * _e878) - (_e877 * _e905));
                                        let _e929 = ((((_e334.member_1.x * _e917) + (_e334.member_1.y * _e920)) + (_e334.member_1.z * _e923)) * _e903);
                                        if (_e929 < 0f) {
                                            phi_11459_ = vec4<f32>();
                                            phi_11460_ = true;
                                            phi_11461_ = false;
                                        } else {
                                            let _e932 = ((_e912 + _e929) > 1f);
                                            if _e932 {
                                                phi_11457_ = vec4<f32>();
                                                phi_11458_ = false;
                                            } else {
                                                let _e938 = ((((_e884 * _e917) + (_e885 * _e920)) + (_e886 * _e923)) * _e903);
                                                if (_e938 <= 0f) {
                                                    phi_11452_ = vec4<f32>();
                                                    phi_11453_ = true;
                                                } else {
                                                    let _e940 = (_e938 >= _e403.member);
                                                    if _e940 {
                                                        phi_11451_ = vec4<f32>();
                                                    } else {
                                                        phi_11451_ = vec4<f32>(_e938, _e912, _e929, 1f);
                                                    }
                                                    let _e943 = phi_11451_;
                                                    phi_11452_ = _e943;
                                                    phi_11453_ = _e940;
                                                }
                                                let _e945 = phi_11452_;
                                                let _e947 = phi_11453_;
                                                phi_11457_ = _e945;
                                                phi_11458_ = _e947;
                                            }
                                            let _e949 = phi_11457_;
                                            let _e951 = phi_11458_;
                                            phi_11459_ = _e949;
                                            phi_11460_ = _e932;
                                            phi_11461_ = _e951;
                                        }
                                        let _e953 = phi_11459_;
                                        let _e955 = phi_11460_;
                                        let _e957 = phi_11461_;
                                        phi_11469_ = _e953;
                                        phi_11470_ = select(_e957, true, _e955);
                                    }
                                    let _e960 = phi_11469_;
                                    let _e962 = phi_11470_;
                                    phi_11471_ = _e960;
                                    phi_11472_ = _e914;
                                    phi_11473_ = _e962;
                                }
                                let _e964 = phi_11471_;
                                let _e966 = phi_11472_;
                                let _e968 = phi_11473_;
                                phi_11481_ = _e964;
                                phi_11482_ = select(_e968, true, _e966);
                            }
                            let _e971 = phi_11481_;
                            let _e973 = phi_11482_;
                            phi_11620_ = _e971;
                            phi_11621_ = _e973;
                        } else {
                            if (_e771.member_6.x == 1u) {
                                let _e799 = (_e334.member.x - _e771.member.x);
                                let _e800 = (_e334.member.y - _e771.member.y);
                                let _e801 = (_e334.member.z - _e771.member.z);
                                let _e806 = (((_e799 * _e334.member_1.x) + (_e800 * _e334.member_1.y)) + (_e801 * _e334.member_1.z));
                                if (_e806 > 0f) {
                                    phi_11519_ = select(true, false, ((((_e799 * _e799) + (_e800 * _e800)) + (_e801 * _e801)) > (_e771.member.w * _e771.member.w)));
                                } else {
                                    phi_11519_ = true;
                                }
                                let _e819 = phi_11519_;
                                if _e819 {
                                    let _e823 = (_e799 - (_e334.member_1.x * _e806));
                                    let _e824 = (_e800 - (_e334.member_1.y * _e806));
                                    let _e825 = (_e801 - (_e334.member_1.z * _e806));
                                    let _e834 = ((_e771.member.w * _e771.member.w) - (((_e823 * _e823) + (_e824 * _e824)) + (_e825 * _e825)));
                                    if (_e834 < 0f) {
                                        phi_11575_ = vec4<f32>();
                                        phi_11576_ = true;
                                    } else {
                                        let _e836 = sqrt(_e834);
                                        let _e837 = -(_e806);
                                        let _e838 = (_e837 - _e836);
                                        if (_e838 <= 0f) {
                                            phi_11553_ = true;
                                        } else {
                                            phi_11553_ = (_e838 >= _e403.member);
                                        }
                                        let _e842 = phi_11553_;
                                        if _e842 {
                                            phi_11559_ = (_e837 + _e836);
                                        } else {
                                            phi_11559_ = _e838;
                                        }
                                        let _e845 = phi_11559_;
                                        if (_e845 <= 0f) {
                                            phi_11570_ = vec4<f32>();
                                            phi_11571_ = true;
                                        } else {
                                            let _e847 = (_e845 >= _e403.member);
                                            if _e847 {
                                                phi_11569_ = vec4<f32>();
                                            } else {
                                                phi_11569_ = vec4<f32>(_e845, 0f, 0f, 1f);
                                            }
                                            let _e850 = phi_11569_;
                                            phi_11570_ = _e850;
                                            phi_11571_ = _e847;
                                        }
                                        let _e852 = phi_11570_;
                                        let _e854 = phi_11571_;
                                        phi_11575_ = _e852;
                                        phi_11576_ = _e854;
                                    }
                                    let _e856 = phi_11575_;
                                    let _e858 = phi_11576_;
                                    phi_11580_ = _e856;
                                    phi_11581_ = _e858;
                                } else {
                                    phi_11580_ = vec4<f32>();
                                    phi_11581_ = true;
                                }
                                let _e860 = phi_11580_;
                                let _e862 = phi_11581_;
                                phi_11610_ = _e860;
                                phi_11611_ = _e862;
                            } else {
                                if (abs(_e334.member_1.y) < 0.000000000001f) {
                                    phi_11608_ = vec4<f32>();
                                    phi_11609_ = true;
                                } else {
                                    let _e781 = ((_e771.member.x - _e334.member.y) / _e334.member_1.y);
                                    if (_e781 <= 0f) {
                                        phi_11603_ = vec4<f32>();
                                        phi_11604_ = true;
                                    } else {
                                        let _e783 = (_e781 >= _e403.member);
                                        if _e783 {
                                            phi_11602_ = vec4<f32>();
                                        } else {
                                            phi_11602_ = vec4<f32>(_e781, 0f, 0f, 1f);
                                        }
                                        let _e786 = phi_11602_;
                                        phi_11603_ = _e786;
                                        phi_11604_ = _e783;
                                    }
                                    let _e788 = phi_11603_;
                                    let _e790 = phi_11604_;
                                    phi_11608_ = _e788;
                                    phi_11609_ = _e790;
                                }
                                let _e792 = phi_11608_;
                                let _e794 = phi_11609_;
                                phi_11610_ = _e792;
                                phi_11611_ = _e794;
                            }
                            let _e864 = phi_11610_;
                            let _e866 = phi_11611_;
                            phi_11620_ = _e864;
                            phi_11621_ = select(true, false, select(true, false, _e866));
                        }
                        let _e975 = phi_11620_;
                        let _e977 = phi_11621_;
                        let _e980 = select(vec4<f32>(0f, 0f, 0f, 0f), _e975, vec4(select(true, false, _e977)));
                        let _e982 = (_e980.w > 0f);
                        if _e982 {
                            let _e988 = type_68(_e980.x, _e403.member_1, _e403.member_2, _e403.member_3);
                            let _e993 = type_68(_e988.member, _e399, _e988.member_2, _e988.member_3);
                            phi_11642_ = _e993;
                            phi_11644_ = _e993;
                        } else {
                            phi_11642_ = type_68();
                            phi_11644_ = _e403;
                        }
                        let _e995 = phi_11642_;
                        let _e997 = phi_11644_;
                        phi_11313_ = _e393;
                        phi_11316_ = _e395;
                        phi_11318_ = _e397;
                        phi_11320_ = (_e399 + 1u);
                        phi_11322_ = select(_e401, _e980, vec4(_e982));
                        phi_11324_ = _e997;
                        phi_11826_ = _e995;
                        phi_11827_ = true;
                        phi_11828_ = false;
                    } else {
                        if (_e393 == 4294967295u) {
                            let _e406 = (_e395 == 0u);
                            if _e406 {
                                phi_11681_ = u32();
                                phi_11682_ = u32();
                                phi_11684_ = false;
                                phi_11685_ = false;
                            } else {
                                let _e407 = (_e395 - 1u);
                                let _e408 = (_e407 < 64u);
                                if _e408 {
                                } else {
                                    phi_32958_ = true;
                                    phi_20650_ = bool();
                                    phi_20649_ = type_68();
                                    break;
                                }
                                let _e410 = local_70[_e407];
                                let _e412 = (_e410 >= _e403.member);
                                if _e412 {
                                    phi_11679_ = u32();
                                } else {
                                    if _e408 {
                                    } else {
                                        phi_32958_ = true;
                                        phi_20650_ = bool();
                                        phi_20649_ = type_68();
                                        break;
                                    }
                                    let _e414 = local_69[_e407];
                                    phi_11679_ = _e414;
                                }
                                let _e416 = phi_11679_;
                                phi_11681_ = _e407;
                                phi_11682_ = _e416;
                                phi_11684_ = select(true, false, _e412);
                                phi_11685_ = _e412;
                            }
                            let _e419 = phi_11681_;
                            let _e421 = phi_11682_;
                            let _e423 = phi_11684_;
                            let _e425 = phi_11685_;
                            phi_11686_ = _e419;
                            phi_11687_ = _e421;
                            phi_11688_ = _e419;
                            phi_11689_ = _e423;
                            phi_11690_ = _e425;
                            phi_11691_ = _e406;
                        } else {
                            phi_11686_ = u32();
                            phi_11687_ = _e393;
                            phi_11688_ = _e395;
                            phi_11689_ = true;
                            phi_11690_ = false;
                            phi_11691_ = false;
                        }
                        let _e427 = phi_11686_;
                        let _e429 = phi_11687_;
                        let _e431 = phi_11688_;
                        let _e433 = phi_11689_;
                        let _e435 = phi_11690_;
                        let _e437 = phi_11691_;
                        if _e433 {
                            let _e439 = ((_e429 & 2147483648u) == 0u);
                            if _e439 {
                                let _e447 = global_3.member[_e429];
                                if (_e447.member_1 == 4294967295u) {
                                    phi_11721_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e460 = ((_e447.member.x - _e334.member.x) * _e389.x);
                                    let _e462 = ((_e447.member.y - _e334.member.y) * _e389.y);
                                    let _e464 = ((_e447.member.z - _e334.member.z) * _e389.z);
                                    let _e471 = ((_e447.member_2.x - _e334.member.x) * _e389.x);
                                    let _e472 = ((_e447.member_2.y - _e334.member.y) * _e389.y);
                                    let _e473 = ((_e447.member_2.z - _e334.member.z) * _e389.z);
                                    if (_e460 == _e460) {
                                        let _e475 = (_e471 == _e471);
                                        if _e475 {
                                            let _e476 = (_e460 != _e460);
                                            if _e476 {
                                                phi_25056_ = true;
                                            } else {
                                                phi_25056_ = (_e471 <= _e460);
                                            }
                                            let _e479 = phi_25056_;
                                            let _e480 = select(_e460, _e471, _e479);
                                            if _e476 {
                                                phi_25086_ = true;
                                            } else {
                                                phi_25086_ = (_e471 >= _e460);
                                            }
                                            let _e485 = phi_25086_;
                                            let _e486 = select(_e460, _e471, _e485);
                                            phi_25033_ = type_57(select(-300000000000000000000000000000000000000f, _e480, (_e480 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e486, (_e486 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_25033_ = type_57();
                                        }
                                        let _e491 = phi_25033_;
                                        phi_25036_ = _e491;
                                        phi_25037_ = select(true, false, _e475);
                                    } else {
                                        phi_25036_ = type_57();
                                        phi_25037_ = true;
                                    }
                                    let _e494 = phi_25036_;
                                    let _e496 = phi_25037_;
                                    if _e496 {
                                        phi_25042_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_25042_ = _e494;
                                    }
                                    let _e498 = phi_25042_;
                                    if (_e462 == _e462) {
                                        let _e502 = (_e472 == _e472);
                                        if _e502 {
                                            let _e503 = (_e462 != _e462);
                                            if _e503 {
                                                phi_25143_ = true;
                                            } else {
                                                phi_25143_ = (_e472 <= _e462);
                                            }
                                            let _e506 = phi_25143_;
                                            let _e507 = select(_e462, _e472, _e506);
                                            if (_e498.member != _e498.member) {
                                                phi_25158_ = true;
                                            } else {
                                                phi_25158_ = (_e507 >= _e498.member);
                                            }
                                            let _e511 = phi_25158_;
                                            if _e503 {
                                                phi_25173_ = true;
                                            } else {
                                                phi_25173_ = (_e472 >= _e462);
                                            }
                                            let _e515 = phi_25173_;
                                            let _e516 = select(_e462, _e472, _e515);
                                            if (_e498.member_1 != _e498.member_1) {
                                                phi_25188_ = true;
                                            } else {
                                                phi_25188_ = (_e516 <= _e498.member_1);
                                            }
                                            let _e520 = phi_25188_;
                                            phi_25120_ = type_57(select(_e498.member, _e507, _e511), select(_e498.member_1, _e516, _e520));
                                        } else {
                                            phi_25120_ = type_57();
                                        }
                                        let _e524 = phi_25120_;
                                        phi_25123_ = _e524;
                                        phi_25124_ = select(true, false, _e502);
                                    } else {
                                        phi_25123_ = type_57();
                                        phi_25124_ = true;
                                    }
                                    let _e527 = phi_25123_;
                                    let _e529 = phi_25124_;
                                    if _e529 {
                                        phi_25129_ = _e498;
                                    } else {
                                        phi_25129_ = _e527;
                                    }
                                    let _e531 = phi_25129_;
                                    if (_e464 == _e464) {
                                        let _e535 = (_e473 == _e473);
                                        if _e535 {
                                            let _e536 = (_e464 != _e464);
                                            if _e536 {
                                                phi_25230_ = true;
                                            } else {
                                                phi_25230_ = (_e473 <= _e464);
                                            }
                                            let _e539 = phi_25230_;
                                            let _e540 = select(_e464, _e473, _e539);
                                            if (_e531.member != _e531.member) {
                                                phi_25245_ = true;
                                            } else {
                                                phi_25245_ = (_e540 >= _e531.member);
                                            }
                                            let _e544 = phi_25245_;
                                            if _e536 {
                                                phi_25260_ = true;
                                            } else {
                                                phi_25260_ = (_e473 >= _e464);
                                            }
                                            let _e548 = phi_25260_;
                                            let _e549 = select(_e464, _e473, _e548);
                                            if (_e531.member_1 != _e531.member_1) {
                                                phi_25275_ = true;
                                            } else {
                                                phi_25275_ = (_e549 <= _e531.member_1);
                                            }
                                            let _e553 = phi_25275_;
                                            phi_25207_ = type_57(select(_e531.member, _e540, _e544), select(_e531.member_1, _e549, _e553));
                                        } else {
                                            phi_25207_ = type_57();
                                        }
                                        let _e557 = phi_25207_;
                                        phi_25210_ = _e557;
                                        phi_25211_ = select(true, false, _e535);
                                    } else {
                                        phi_25210_ = type_57();
                                        phi_25211_ = true;
                                    }
                                    let _e560 = phi_25210_;
                                    let _e562 = phi_25211_;
                                    if _e562 {
                                        phi_25216_ = _e531;
                                    } else {
                                        phi_25216_ = _e560;
                                    }
                                    let _e564 = phi_25216_;
                                    if (_e564.member_1 < 0f) {
                                        phi_25014_ = true;
                                    } else {
                                        if (_e564.member > _e564.member_1) {
                                            phi_25011_ = true;
                                        } else {
                                            phi_25011_ = (_e564.member > _e403.member);
                                        }
                                        let _e571 = phi_25011_;
                                        phi_25014_ = _e571;
                                    }
                                    let _e573 = phi_25014_;
                                    phi_11721_ = select(_e564.member, 300000000000000000000000000000000000000f, _e573);
                                }
                                let _e576 = phi_11721_;
                                if (_e447.member_5 == 4294967295u) {
                                    phi_11731_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e589 = ((_e447.member_4.x - _e334.member.x) * _e389.x);
                                    let _e591 = ((_e447.member_4.y - _e334.member.y) * _e389.y);
                                    let _e593 = ((_e447.member_4.z - _e334.member.z) * _e389.z);
                                    let _e600 = ((_e447.member_6.x - _e334.member.x) * _e389.x);
                                    let _e601 = ((_e447.member_6.y - _e334.member.y) * _e389.y);
                                    let _e602 = ((_e447.member_6.z - _e334.member.z) * _e389.z);
                                    if (_e589 == _e589) {
                                        let _e604 = (_e600 == _e600);
                                        if _e604 {
                                            let _e605 = (_e589 != _e589);
                                            if _e605 {
                                                phi_25377_ = true;
                                            } else {
                                                phi_25377_ = (_e600 <= _e589);
                                            }
                                            let _e608 = phi_25377_;
                                            let _e609 = select(_e589, _e600, _e608);
                                            if _e605 {
                                                phi_25407_ = true;
                                            } else {
                                                phi_25407_ = (_e600 >= _e589);
                                            }
                                            let _e614 = phi_25407_;
                                            let _e615 = select(_e589, _e600, _e614);
                                            phi_25354_ = type_57(select(-300000000000000000000000000000000000000f, _e609, (_e609 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e615, (_e615 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_25354_ = type_57();
                                        }
                                        let _e620 = phi_25354_;
                                        phi_25357_ = _e620;
                                        phi_25358_ = select(true, false, _e604);
                                    } else {
                                        phi_25357_ = type_57();
                                        phi_25358_ = true;
                                    }
                                    let _e623 = phi_25357_;
                                    let _e625 = phi_25358_;
                                    if _e625 {
                                        phi_25363_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_25363_ = _e623;
                                    }
                                    let _e627 = phi_25363_;
                                    if (_e591 == _e591) {
                                        let _e631 = (_e601 == _e601);
                                        if _e631 {
                                            let _e632 = (_e591 != _e591);
                                            if _e632 {
                                                phi_25464_ = true;
                                            } else {
                                                phi_25464_ = (_e601 <= _e591);
                                            }
                                            let _e635 = phi_25464_;
                                            let _e636 = select(_e591, _e601, _e635);
                                            if (_e627.member != _e627.member) {
                                                phi_25479_ = true;
                                            } else {
                                                phi_25479_ = (_e636 >= _e627.member);
                                            }
                                            let _e640 = phi_25479_;
                                            if _e632 {
                                                phi_25494_ = true;
                                            } else {
                                                phi_25494_ = (_e601 >= _e591);
                                            }
                                            let _e644 = phi_25494_;
                                            let _e645 = select(_e591, _e601, _e644);
                                            if (_e627.member_1 != _e627.member_1) {
                                                phi_25509_ = true;
                                            } else {
                                                phi_25509_ = (_e645 <= _e627.member_1);
                                            }
                                            let _e649 = phi_25509_;
                                            phi_25441_ = type_57(select(_e627.member, _e636, _e640), select(_e627.member_1, _e645, _e649));
                                        } else {
                                            phi_25441_ = type_57();
                                        }
                                        let _e653 = phi_25441_;
                                        phi_25444_ = _e653;
                                        phi_25445_ = select(true, false, _e631);
                                    } else {
                                        phi_25444_ = type_57();
                                        phi_25445_ = true;
                                    }
                                    let _e656 = phi_25444_;
                                    let _e658 = phi_25445_;
                                    if _e658 {
                                        phi_25450_ = _e627;
                                    } else {
                                        phi_25450_ = _e656;
                                    }
                                    let _e660 = phi_25450_;
                                    if (_e593 == _e593) {
                                        let _e664 = (_e602 == _e602);
                                        if _e664 {
                                            let _e665 = (_e593 != _e593);
                                            if _e665 {
                                                phi_25551_ = true;
                                            } else {
                                                phi_25551_ = (_e602 <= _e593);
                                            }
                                            let _e668 = phi_25551_;
                                            let _e669 = select(_e593, _e602, _e668);
                                            if (_e660.member != _e660.member) {
                                                phi_25566_ = true;
                                            } else {
                                                phi_25566_ = (_e669 >= _e660.member);
                                            }
                                            let _e673 = phi_25566_;
                                            if _e665 {
                                                phi_25581_ = true;
                                            } else {
                                                phi_25581_ = (_e602 >= _e593);
                                            }
                                            let _e677 = phi_25581_;
                                            let _e678 = select(_e593, _e602, _e677);
                                            if (_e660.member_1 != _e660.member_1) {
                                                phi_25596_ = true;
                                            } else {
                                                phi_25596_ = (_e678 <= _e660.member_1);
                                            }
                                            let _e682 = phi_25596_;
                                            phi_25528_ = type_57(select(_e660.member, _e669, _e673), select(_e660.member_1, _e678, _e682));
                                        } else {
                                            phi_25528_ = type_57();
                                        }
                                        let _e686 = phi_25528_;
                                        phi_25531_ = _e686;
                                        phi_25532_ = select(true, false, _e664);
                                    } else {
                                        phi_25531_ = type_57();
                                        phi_25532_ = true;
                                    }
                                    let _e689 = phi_25531_;
                                    let _e691 = phi_25532_;
                                    if _e691 {
                                        phi_25537_ = _e660;
                                    } else {
                                        phi_25537_ = _e689;
                                    }
                                    let _e693 = phi_25537_;
                                    if (_e693.member_1 < 0f) {
                                        phi_25335_ = true;
                                    } else {
                                        if (_e693.member > _e693.member_1) {
                                            phi_25332_ = true;
                                        } else {
                                            phi_25332_ = (_e693.member > _e403.member);
                                        }
                                        let _e700 = phi_25332_;
                                        phi_25335_ = _e700;
                                    }
                                    let _e702 = phi_25335_;
                                    phi_11731_ = select(_e693.member, 300000000000000000000000000000000000000f, _e702);
                                }
                                let _e705 = phi_11731_;
                                if (_e447.member_3 > 0u) {
                                    phi_11742_ = ((2147483648u | ((_e447.member_3 - 1u) << bitcast<u32>(28u))) | _e447.member_1);
                                } else {
                                    phi_11742_ = _e447.member_1;
                                }
                                let _e714 = phi_11742_;
                                if (_e447.member_7 > 0u) {
                                    phi_11753_ = ((2147483648u | ((_e447.member_7 - 1u) << bitcast<u32>(28u))) | _e447.member_5);
                                } else {
                                    phi_11753_ = _e447.member_5;
                                }
                                let _e723 = phi_11753_;
                                let _e724 = (_e576 <= _e705);
                                let _e728 = select(_e576, _e705, _e724);
                                if (_e728 < 300000000000000000000000000000000000000f) {
                                    let _e730 = (_e431 < 64u);
                                    if _e730 {
                                        if _e730 {
                                        } else {
                                            phi_32958_ = true;
                                            phi_20650_ = bool();
                                            phi_20649_ = type_68();
                                            break;
                                        }
                                        local_69[_e431] = select(_e714, _e723, _e724);
                                        if _e730 {
                                        } else {
                                            phi_32958_ = true;
                                            phi_20650_ = bool();
                                            phi_20649_ = type_68();
                                            break;
                                        }
                                        local_70[_e431] = _e728;
                                        phi_11792_ = (_e431 + 1u);
                                    } else {
                                        phi_11792_ = _e431;
                                    }
                                    let _e735 = phi_11792_;
                                    phi_11793_ = _e735;
                                } else {
                                    phi_11793_ = _e431;
                                }
                                let _e737 = phi_11793_;
                                phi_11804_ = select(4294967295u, select(_e723, _e714, _e724), (select(_e705, _e576, _e724) < 300000000000000000000000000000000000000f));
                                phi_11805_ = _e737;
                                phi_11806_ = u32();
                                phi_11807_ = u32();
                            } else {
                                let _e440 = (_e429 & 268435455u);
                                phi_11804_ = u32();
                                phi_11805_ = u32();
                                phi_11806_ = ((_e440 + ((_e429 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_11807_ = _e440;
                            }
                            let _e741 = phi_11804_;
                            let _e743 = phi_11805_;
                            let _e745 = phi_11806_;
                            let _e747 = phi_11807_;
                            phi_11809_ = _e741;
                            phi_11810_ = _e743;
                            phi_11812_ = _e745;
                            phi_11813_ = _e747;
                            phi_11814_ = select(true, false, _e439);
                            phi_11815_ = _e439;
                        } else {
                            phi_11809_ = u32();
                            phi_11810_ = u32();
                            phi_11812_ = _e397;
                            phi_11813_ = _e399;
                            phi_11814_ = _e435;
                            phi_11815_ = false;
                        }
                        let _e750 = phi_11809_;
                        let _e752 = phi_11810_;
                        let _e754 = phi_11812_;
                        let _e756 = phi_11813_;
                        let _e758 = phi_11814_;
                        let _e760 = phi_11815_;
                        phi_11313_ = select(_e750, 4294967295u, _e758);
                        phi_11316_ = select(_e752, select(_e427, _e431, _e433), _e758);
                        phi_11318_ = select(_e397, _e754, _e758);
                        phi_11320_ = select(_e399, _e756, _e758);
                        phi_11322_ = _e401;
                        phi_11324_ = _e403;
                        phi_11826_ = type_68();
                        phi_11827_ = select(_e760, true, _e758);
                        phi_11828_ = select(select(_e437, false, _e433), false, _e758);
                    }
                    let _e1002 = phi_11313_;
                    let _e1004 = phi_11316_;
                    let _e1006 = phi_11318_;
                    let _e1008 = phi_11320_;
                    let _e1010 = phi_11322_;
                    let _e1012 = phi_11324_;
                    let _e1014 = phi_11826_;
                    let _e1016 = phi_11827_;
                    let _e1018 = phi_11828_;
                    phi_11312_ = _e1002;
                    phi_11315_ = _e1004;
                    phi_11317_ = _e1006;
                    phi_11319_ = _e1008;
                    phi_11321_ = _e1010;
                    phi_11323_ = _e1012;
                    phi_32958_ = false;
                    phi_20650_ = _e1018;
                    phi_20649_ = _e1014;
                    if !(_e1016) {
                        break;
                    }
                }
                let _e1021 = phi_32958_;
                let _e1023 = phi_20650_;
                let _e1025 = phi_20649_;
                if _e1021 {
                    break;
                }
                if _e1023 {
                    let _e1027 = local_71;
                    if (_e1027.member_1 != 4294967295u) {
                        let _e1031 = global_2.member[_e1027.member_1];
                        if (_e1031.member_6.x == 0u) {
                            let _e1073 = (_e1031.member_1.x - _e1031.member.x);
                            let _e1074 = (_e1031.member_1.y - _e1031.member.y);
                            let _e1075 = (_e1031.member_1.z - _e1031.member.z);
                            let _e1080 = (_e1031.member_2.x - _e1031.member.x);
                            let _e1081 = (_e1031.member_2.y - _e1031.member.y);
                            let _e1082 = (_e1031.member_2.z - _e1031.member.z);
                            let _e1085 = ((_e1074 * _e1082) - (_e1081 * _e1075));
                            let _e1088 = ((_e1075 * _e1080) - (_e1082 * _e1073));
                            let _e1091 = ((_e1073 * _e1081) - (_e1080 * _e1074));
                            let _e1098 = (1f / sqrt((((_e1085 * _e1085) + (_e1088 * _e1088)) + (_e1091 * _e1091))));
                            let _e1102 = vec3<f32>((_e1085 * _e1098), (_e1088 * _e1098), (_e1091 * _e1098));
                            if (_e1031.member_6.w == 1u) {
                                let _e1111 = local_73;
                                let _e1115 = local_74;
                                let _e1117 = ((1f - _e1111.y) - _e1115.z);
                                let _e1138 = (((_e1031.member_3.x * _e1117) + (_e1031.member_4.x * _e1111.y)) + (_e1031.member_5.x * _e1115.z));
                                let _e1139 = (((_e1031.member_3.y * _e1117) + (_e1031.member_4.y * _e1111.y)) + (_e1031.member_5.y * _e1115.z));
                                let _e1140 = (((_e1031.member_3.z * _e1117) + (_e1031.member_4.z * _e1111.y)) + (_e1031.member_5.z * _e1115.z));
                                let _e1145 = (((_e1138 * _e1138) + (_e1139 * _e1139)) + (_e1140 * _e1140));
                                if (_e1145 > 0f) {
                                    let _e1148 = (1f / sqrt(_e1145));
                                    phi_11955_ = vec3<f32>((_e1138 * _e1148), (_e1139 * _e1148), (_e1140 * _e1148));
                                } else {
                                    phi_11955_ = _e1102;
                                }
                                let _e1154 = phi_11955_;
                                phi_11956_ = _e1154;
                            } else {
                                phi_11956_ = _e1102;
                            }
                            let _e1156 = phi_11956_;
                            phi_11990_ = type_61(_e1102, _e1156);
                        } else {
                            if (_e1031.member_6.x == 1u) {
                                let _e1041 = local_72;
                                let _e1061 = vec3<f32>((((_e334.member.x + (_e334.member_1.x * _e1041.x)) - _e1031.member.x) / _e1031.member.w), (((_e334.member.y + (_e334.member_1.y * _e1041.x)) - _e1031.member.y) / _e1031.member.w), (((_e334.member.z + (_e334.member_1.z * _e1041.x)) - _e1031.member.z) / _e1031.member.w));
                                phi_11989_ = type_61(_e1061, _e1061);
                            } else {
                                let _e1038 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e334.member_1.y > 0f)));
                                phi_11989_ = type_61(_e1038, _e1038);
                            }
                            let _e1064 = phi_11989_;
                            phi_11990_ = _e1064;
                        }
                        let _e1159 = phi_11990_;
                        let _e1163 = local_75;
                        let _e1168 = type_68(_e1163.member, _e1163.member_1, _e1159.member, _e1163.member_3);
                        phi_11995_ = type_68(_e1168.member, _e1168.member_1, _e1168.member_2, _e1159.member_1);
                    } else {
                        let _e7905 = local_107;
                        phi_11995_ = _e7905;
                    }
                    let _e1175 = phi_11995_;
                    phi_11996_ = _e1175;
                } else {
                    phi_11996_ = type_68();
                }
                let _e1177 = phi_11996_;
                if select(true, false, _e1023) {
                    phi_12001_ = _e1025;
                } else {
                    phi_12001_ = _e1177;
                }
                let _e1180 = phi_12001_;
                let _e1181 = (select(select(u32(_e334.member_3.w), 0u, (_e334.member_3.w < 0f)), 4294967295u, (_e334.member_3.w > 4294967000f)) + 1u);
                if (_e1180.member_1 == 4294967295u) {
                    if _e372 {
                        phi_12160_ = 1f;
                    } else {
                        if ((_e378 & 2u) == 0u) {
                            phi_12159_ = 1f;
                        } else {
                            let _e7206 = global_1.member.member_3[1u];
                            let _e7209 = global_1.member.member_3[2u];
                            let _e7212 = ((atan2(_e334.member_1.z, _e334.member_1.x) * 0.15915494f) + 0.5f);
                            let _e7216 = select(_e334.member_1.y, -1f, (_e334.member_1.y < -1f));
                            let _e7219 = acos(select(_e7216, 1f, (_e7216 > 1f)));
                            let _e7221 = f32(_e7206);
                            let _e7222 = ((_e7212 - floor(_e7212)) * _e7221);
                            let _e7227 = select(select(u32(_e7222), 0u, (_e7222 < 0f)), 4294967295u, (_e7222 > 4294967000f));
                            let _e7228 = (_e7206 - 1u);
                            let _e7231 = f32(_e7209);
                            let _e7232 = ((_e7219 * 0.31830987f) * _e7231);
                            let _e7237 = select(select(u32(_e7232), 0u, (_e7232 < 0f)), 4294967295u, (_e7232 > 4294967000f));
                            let _e7238 = (_e7209 - 1u);
                            let _e7240 = select(_e7237, _e7238, (_e7238 < _e7237));
                            let _e7241 = sin(_e7219);
                            if (_e7241 <= 0f) {
                                phi_12157_ = 0f;
                            } else {
                                let _e7245 = global_1.member.member_2[3u];
                                let _e7248 = global_1.member.member_3[0u];
                                let _e7252 = (_e7245 + _e7240);
                                let _e7255 = global_6.member[(_e7252 + 1u)];
                                let _e7257 = global_6.member[_e7252];
                                let _e7260 = ((_e7248 + (_e7240 * (_e7206 + 1u))) + select(_e7227, _e7228, (_e7228 < _e7227)));
                                let _e7263 = global_6.member[(_e7260 + 1u)];
                                let _e7265 = global_6.member[_e7260];
                                phi_12157_ = ((((_e7255 - _e7257) * _e7231) * ((_e7263 - _e7265) * _e7221)) / (19.73921f * _e7241));
                            }
                            let _e7272 = phi_12157_;
                            if (_e334.member_1.w >= 300000000000000000000000000000000000000f) {
                                phi_31971_ = 1f;
                            } else {
                                let _e7274 = (_e334.member_1.w * _e334.member_1.w);
                                let _e7276 = (_e7274 + (_e7272 * _e7272));
                                if (_e7276 == 0f) {
                                    phi_31968_ = 0f;
                                } else {
                                    phi_31968_ = (_e7274 / _e7276);
                                }
                                let _e7280 = phi_31968_;
                                phi_31971_ = _e7280;
                            }
                            let _e7282 = phi_31971_;
                            phi_12159_ = _e7282;
                        }
                        let _e7284 = phi_12159_;
                        phi_12160_ = _e7284;
                    }
                    let _e7286 = phi_12160_;
                    if ((_e378 & 4u) == 0u) {
                        phi_13024_ = _e362;
                    } else {
                        if _e372 {
                            phi_12213_ = 1f;
                        } else {
                            let _e7297 = global_1.member.member_5;
                            let _e7301 = (_e334.member_1.x - _e7297.x);
                            let _e7302 = (_e334.member_1.y - _e7297.y);
                            let _e7303 = (_e334.member_1.z - _e7297.z);
                            let _e7310 = (sqrt((((_e7301 * _e7301) + (_e7302 * _e7302)) + (_e7303 * _e7303))) * 0.5f);
                            if (_e7310 != _e7310) {
                                phi_31997_ = true;
                            } else {
                                phi_31997_ = (1f <= _e7310);
                            }
                            let _e7314 = phi_31997_;
                            let _e7320 = global_1.member.member_6[3u];
                            if ((2f * asin(select(_e7310, 1f, _e7314))) >= _e7320) {
                                phi_12211_ = 0f;
                            } else {
                                let _e7323 = sin((_e7320 * 0.5f));
                                phi_12211_ = (1f / ((12.566371f * _e7323) * _e7323));
                            }
                            let _e7328 = phi_12211_;
                            if (_e334.member_1.w >= 300000000000000000000000000000000000000f) {
                                phi_32018_ = 1f;
                            } else {
                                let _e7330 = (_e334.member_1.w * _e334.member_1.w);
                                let _e7332 = (_e7330 + (_e7328 * _e7328));
                                if (_e7332 == 0f) {
                                    phi_32015_ = 0f;
                                } else {
                                    phi_32015_ = (_e7330 / _e7332);
                                }
                                let _e7336 = phi_32015_;
                                phi_32018_ = _e7336;
                            }
                            let _e7338 = phi_32018_;
                            phi_12213_ = _e7338;
                        }
                        let _e7340 = phi_12213_;
                        let _e7342 = global_1.member.member_5;
                        let _e7346 = (_e334.member_1.x - _e7342.x);
                        let _e7347 = (_e334.member_1.y - _e7342.y);
                        let _e7348 = (_e334.member_1.z - _e7342.z);
                        let _e7355 = (sqrt((((_e7346 * _e7346) + (_e7347 * _e7347)) + (_e7348 * _e7348))) * 0.5f);
                        if (_e7355 != _e7355) {
                            phi_32029_ = true;
                        } else {
                            phi_32029_ = (1f <= _e7355);
                        }
                        let _e7359 = phi_32029_;
                        let _e7362 = (2f * asin(select(_e7355, 1f, _e7359)));
                        let _e7366 = global_1.member.member_6[3u];
                        if (_e7362 >= _e7366) {
                            phi_13010_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e7370 = (1f - pow((_e7362 / _e7366), 2f));
                            if (_e7370 != _e7370) {
                                phi_32044_ = true;
                            } else {
                                phi_32044_ = (0f >= _e7370);
                            }
                            let _e7374 = phi_32044_;
                            let _e7377 = global_1.member.member_6;
                            let _e7383 = global_1.member.member_8[2u];
                            if (_e7383 == 4294967295u) {
                                phi_12983_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e7386 = global_6.member[_e7383];
                                let _e7389 = global_6.member[(_e7383 + 1u)];
                                let _e7392 = global_6.member[(_e7383 + 2u)];
                                let _e7395 = global_6.member[(_e7383 + 3u)];
                                let _e7398 = global_6.member[(_e7383 + 4u)];
                                let _e7401 = global_6.member[(_e7383 + 5u)];
                                let _e7404 = global_6.member[(_e7383 + 6u)];
                                let _e7407 = global_6.member[(_e7383 + 7u)];
                                let _e7410 = global_6.member[(_e7383 + 8u)];
                                let _e7413 = global_6.member[(_e7383 + 9u)];
                                let _e7416 = global_6.member[(_e7383 + 13u)];
                                let _e7419 = global_6.member[(_e7383 + 14u)];
                                let _e7422 = global_6.member[(_e7383 + 15u)];
                                let _e7425 = global_6.member[(_e7383 + 16u)];
                                let _e7428 = global_6.member[(_e7383 + 18u)];
                                let _e7431 = global_6.member[(_e7383 + 19u)];
                                let _e7434 = global_6.member[(_e7383 + 20u)];
                                let _e7437 = global_6.member[(_e7383 + 29u)];
                                let _e7438 = bitcast<u32>(_e7437);
                                let _e7451 = (((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e7386) + (_e334.member.y * _e7389)) + (_e334.member.z * _e7392)))) + _e7404);
                                if (_e7451 > 0f) {
                                    let _e7460 = ((((_e334.member.x - _e7386) * _e7407) + ((_e334.member.y - _e7389) * _e7410)) + ((_e334.member.z - _e7392) * _e7413));
                                    let _e7462 = ((_e7460 * _e7460) - _e7451);
                                    if (_e7460 >= 0f) {
                                        phi_12683_ = vec3<f32>();
                                        phi_12684_ = true;
                                    } else {
                                        let _e7464 = (_e7462 <= 0f);
                                        if _e7464 {
                                            phi_12682_ = vec3<f32>();
                                        } else {
                                            let _e7467 = (-(_e7460) - sqrt(_e7462));
                                            phi_12682_ = vec3<f32>((_e334.member.x + (_e7407 * _e7467)), (_e334.member.y + (_e7410 * _e7467)), (_e334.member.z + (_e7413 * _e7467)));
                                        }
                                        let _e7476 = phi_12682_;
                                        phi_12683_ = _e7476;
                                        phi_12684_ = _e7464;
                                    }
                                    let _e7478 = phi_12683_;
                                    let _e7480 = phi_12684_;
                                    phi_12689_ = _e7478;
                                    phi_12690_ = select(true, false, _e7480);
                                } else {
                                    phi_12689_ = vec3<f32>(_e334.member.x, _e334.member.y, _e334.member.z);
                                    phi_12690_ = true;
                                }
                                let _e7483 = phi_12689_;
                                let _e7485 = phi_12690_;
                                if _e7485 {
                                    let _e7487 = (_e7483.x - _e7386);
                                    let _e7489 = (_e7483.y - _e7389);
                                    let _e7491 = (_e7483.z - _e7392);
                                    let _e7497 = sqrt((((_e7487 * _e7487) + (_e7489 * _e7489)) + (_e7491 * _e7491)));
                                    let _e7498 = (1f / _e7497);
                                    let _e7516 = ((((((_e7483.x * _e7483.x) + (_e7483.y * _e7483.y)) + (_e7483.z * _e7483.z)) - (2f * (((_e7483.x * _e7386) + (_e7483.y * _e7389)) + (_e7483.z * _e7392)))) + _e7401) / (_e7497 + _e7395));
                                    if (_e7516 != _e7516) {
                                        phi_32168_ = true;
                                    } else {
                                        phi_32168_ = (0f >= _e7516);
                                    }
                                    let _e7520 = phi_32168_;
                                    let _e7524 = sqrt((select(_e7516, 0f, _e7520) / (_e7398 - _e7395)));
                                    if (_e7524 != _e7524) {
                                        phi_32183_ = true;
                                    } else {
                                        phi_32183_ = (1f <= _e7524);
                                    }
                                    let _e7528 = phi_32183_;
                                    let _e7530 = (select(_e7524, 1f, _e7528) * 63f);
                                    let _e7535 = ((((_e7487 * _e7498) * _e7407) + ((_e7489 * _e7498) * _e7410)) + ((_e7491 * _e7498) * _e7413));
                                    let _e7537 = select(_e7535, -1f, (_e7535 < -1f));
                                    let _e7541 = ((select(_e7537, 1f, (_e7537 > 1f)) + 1f) * 127.5f);
                                    let _e7546 = select(select(u32(_e7530), 0u, (_e7530 < 0f)), 4294967295u, (_e7530 > 4294967000f));
                                    let _e7548 = select(_e7546, 62u, (62u < _e7546));
                                    let _e7553 = select(select(u32(_e7541), 0u, (_e7541 < 0f)), 4294967295u, (_e7541 > 4294967000f));
                                    let _e7555 = select(_e7553, 254u, (254u < _e7553));
                                    let _e7557 = (_e7530 - f32(_e7548));
                                    let _e7559 = (_e7541 - f32(_e7555));
                                    let _e7563 = (_e7438 + (3u * ((_e7548 * 256u) + _e7555)));
                                    let _e7568 = (_e7438 + (3u * (((_e7548 + 1u) * 256u) + _e7555)));
                                    let _e7570 = global_6.member[_e7563];
                                    let _e7573 = global_6.member[(_e7563 + 1u)];
                                    let _e7576 = global_6.member[(_e7563 + 2u)];
                                    let _e7577 = (1f - _e7559);
                                    let _e7583 = global_6.member[(_e7563 + 3u)];
                                    let _e7586 = global_6.member[(_e7563 + 4u)];
                                    let _e7589 = global_6.member[(_e7563 + 5u)];
                                    let _e7596 = (1f - _e7557);
                                    let _e7601 = global_6.member[_e7568];
                                    let _e7604 = global_6.member[(_e7568 + 1u)];
                                    let _e7607 = global_6.member[(_e7568 + 2u)];
                                    let _e7613 = global_6.member[(_e7568 + 3u)];
                                    let _e7616 = global_6.member[(_e7568 + 4u)];
                                    let _e7619 = global_6.member[(_e7568 + 5u)];
                                    phi_12981_ = vec3<f32>(((((_e7570 * _e7577) + (_e7583 * _e7559)) * _e7596) + (((_e7601 * _e7577) + (_e7613 * _e7559)) * _e7557)), ((((_e7573 * _e7577) + (_e7586 * _e7559)) * _e7596) + (((_e7604 * _e7577) + (_e7616 * _e7559)) * _e7557)), ((((_e7576 * _e7577) + (_e7589 * _e7559)) * _e7596) + (((_e7607 * _e7577) + (_e7619 * _e7559)) * _e7557)));
                                } else {
                                    phi_12981_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e7634 = phi_12981_;
                                let _e7641 = ((_e7425 * 1.1f) * _e7634.y);
                                phi_12983_ = vec3<f32>(exp(-((((_e7416 * _e7634.x) + _e7641) + (_e7428 * _e7634.z)))), exp(-((((_e7419 * _e7634.x) + _e7641) + (_e7431 * _e7634.z)))), exp(-((((_e7422 * _e7634.x) + _e7641) + (_e7434 * _e7634.z)))));
                            }
                            let _e7660 = phi_12983_;
                            let _e7672 = sin((_e7366 * 0.5f));
                            let _e7675 = (((1f - (0.6f * (1f - sqrt(select(_e7370, 0f, _e7374))))) * 1.25f) / ((12.566371f * _e7672) * _e7672));
                            phi_13010_ = vec3<f32>(((_e7377.x * _e7660.x) * _e7675), ((_e7377.y * _e7660.y) * _e7675), ((_e7377.z * _e7660.z) * _e7675));
                        }
                        let _e7681 = phi_13010_;
                        phi_13024_ = vec3<f32>((_e334.member_3.x + ((_e334.member_2.x * _e7681.x) * _e7340)), (_e334.member_3.y + ((_e334.member_2.y * _e7681.y) * _e7340)), (_e334.member_3.z + ((_e334.member_2.z * _e7681.z) * _e7340)));
                    }
                    let _e7696 = phi_13024_;
                    phi_19513_ = type_73(type_35(_e334.member, _e334.member_1, vec4<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z, f32(_e358)), vec4<f32>(_e7696.x, _e7696.y, _e7696.z, f32(_e1181)), vec4<u32>(_e334.member_4.x, (_e334.member_4.y & 4294967294u), 0u, 0u)), type_42(vec4<f32>(_e334.member.x, _e334.member.y, _e334.member.z, -1f), vec4<f32>(_e334.member_1.x, _e334.member_1.y, _e334.member_1.z, 0f), vec4<f32>((_e334.member_2.x * _e7286), (_e334.member_2.y * _e7286), (_e334.member_2.z * _e7286), 0f)), type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_19514_ = type_73();
                    phi_19515_ = true;
                } else {
                    if ((_e378 & 8u) == 0u) {
                        phi_35344_ = _e1021;
                        phi_13561_ = _e352;
                        phi_13562_ = type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    } else {
                        if (_e1180.member > 100f) {
                            let _e1194 = global_1.member.member_8[1u];
                            let _e1196 = global_6.member[_e1194];
                            let _e1199 = global_6.member[(_e1194 + 1u)];
                            let _e1202 = global_6.member[(_e1194 + 2u)];
                            let _e1205 = global_6.member[(_e1194 + 3u)];
                            let _e1208 = global_6.member[(_e1194 + 5u)];
                            let _e1211 = global_6.member[(_e1194 + 6u)];
                            let _e1214 = global_6.member[(_e1194 + 10u)];
                            let _e1217 = global_6.member[(_e1194 + 11u)];
                            let _e1220 = global_6.member[(_e1194 + 13u)];
                            let _e1223 = global_6.member[(_e1194 + 14u)];
                            let _e1226 = global_6.member[(_e1194 + 15u)];
                            let _e1229 = global_6.member[(_e1194 + 16u)];
                            let _e1232 = global_6.member[(_e1194 + 18u)];
                            let _e1235 = global_6.member[(_e1194 + 19u)];
                            let _e1238 = global_6.member[(_e1194 + 20u)];
                            let _e1246 = ((((_e334.member.x - _e1196) * _e334.member_1.x) + ((_e334.member.y - _e1199) * _e334.member_1.y)) + ((_e334.member.z - _e1202) * _e334.member_1.z));
                            let _e1258 = ((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e1196) + (_e334.member.y * _e1199)) + (_e334.member.z * _e1202))));
                            let _e1259 = (_e1258 + _e1211);
                            let _e1260 = (_e1246 * _e1246);
                            let _e1261 = (_e1260 - _e1259);
                            if (_e1261 < 0f) {
                                phi_25743_ = type_58(0u, type_58().member_1);
                                phi_25744_ = type_58();
                                phi_25745_ = true;
                            } else {
                                if (_e1246 < 0f) {
                                    phi_25715_ = -(sqrt(_e1261));
                                } else {
                                    phi_25715_ = sqrt(_e1261);
                                }
                                let _e1268 = phi_25715_;
                                let _e1270 = -((_e1246 + _e1268));
                                let _e1271 = (_e1270 == 0f);
                                if _e1271 {
                                    phi_25738_ = type_58(1u, type_57(0f, 0f));
                                    phi_25739_ = type_58();
                                } else {
                                    let _e1272 = (_e1259 / _e1270);
                                    if (_e1270 < _e1272) {
                                        phi_25727_ = type_57(_e1270, _e1272);
                                    } else {
                                        phi_25727_ = type_57(_e1272, _e1270);
                                    }
                                    let _e1277 = phi_25727_;
                                    phi_25738_ = type_58();
                                    phi_25739_ = type_58(1u, _e1277);
                                }
                                let _e1280 = phi_25738_;
                                let _e1282 = phi_25739_;
                                phi_25743_ = _e1280;
                                phi_25744_ = _e1282;
                                phi_25745_ = _e1271;
                            }
                            let _e1287 = phi_25743_;
                            let _e1289 = phi_25744_;
                            let _e1291 = phi_25745_;
                            if _e1291 {
                                phi_25749_ = _e1287;
                            } else {
                                phi_25749_ = _e1289;
                            }
                            let _e1293 = phi_25749_;
                            if (bitcast<i32>(_e1293.member) != 0i) {
                                phi_25636_ = type_58(0u, _e1293.member_1);
                            } else {
                                phi_25636_ = type_58(1u, type_58().member_1);
                            }
                            let _e1303 = phi_25636_;
                            if (bitcast<i32>(_e1303.member) != 0i) {
                                phi_25683_ = type_58(0u, type_58().member_1);
                            } else {
                                if (_e1303.member_1.member_1 <= 0f) {
                                    phi_25679_ = type_58(0u, type_58().member_1);
                                } else {
                                    if (_e1303.member_1.member != _e1303.member_1.member) {
                                        phi_25787_ = true;
                                    } else {
                                        phi_25787_ = (0f >= _e1303.member_1.member);
                                    }
                                    let _e1315 = phi_25787_;
                                    let _e1316 = select(_e1303.member_1.member, 0f, _e1315);
                                    let _e1317 = (_e1258 + _e1208);
                                    if (_e1317 > 0f) {
                                        let _e1319 = (_e1260 - _e1317);
                                        if (_e1319 < 0f) {
                                            phi_25895_ = type_58(0u, type_58().member_1);
                                            phi_25896_ = type_58();
                                            phi_25897_ = true;
                                        } else {
                                            if (_e1246 < 0f) {
                                                phi_25867_ = -(sqrt(_e1319));
                                            } else {
                                                phi_25867_ = sqrt(_e1319);
                                            }
                                            let _e1326 = phi_25867_;
                                            let _e1328 = -((_e1246 + _e1326));
                                            let _e1329 = (_e1328 == 0f);
                                            if _e1329 {
                                                phi_25890_ = type_58(1u, type_57(0f, 0f));
                                                phi_25891_ = type_58();
                                            } else {
                                                let _e1330 = (_e1317 / _e1328);
                                                if (_e1328 < _e1330) {
                                                    phi_25879_ = type_57(_e1328, _e1330);
                                                } else {
                                                    phi_25879_ = type_57(_e1330, _e1328);
                                                }
                                                let _e1335 = phi_25879_;
                                                phi_25890_ = type_58();
                                                phi_25891_ = type_58(1u, _e1335);
                                            }
                                            let _e1338 = phi_25890_;
                                            let _e1340 = phi_25891_;
                                            phi_25895_ = _e1338;
                                            phi_25896_ = _e1340;
                                            phi_25897_ = _e1329;
                                        }
                                        let _e1345 = phi_25895_;
                                        let _e1347 = phi_25896_;
                                        let _e1349 = phi_25897_;
                                        if _e1349 {
                                            phi_25901_ = _e1345;
                                        } else {
                                            phi_25901_ = _e1347;
                                        }
                                        let _e1351 = phi_25901_;
                                        if (bitcast<i32>(_e1351.member) != 0i) {
                                            phi_25835_ = select(_e1351.member_1.member_1, _e1351.member_1.member, (_e1351.member_1.member >= 0f));
                                        } else {
                                            phi_25835_ = -1f;
                                        }
                                        let _e1362 = phi_25835_;
                                        if (_e1362 >= 0f) {
                                            if (_e1303.member_1.member_1 != _e1303.member_1.member_1) {
                                                phi_25939_ = true;
                                            } else {
                                                phi_25939_ = (_e1362 <= _e1303.member_1.member_1);
                                            }
                                            let _e1367 = phi_25939_;
                                            phi_25664_ = select(_e1303.member_1.member_1, _e1362, _e1367);
                                        } else {
                                            phi_25664_ = _e1303.member_1.member_1;
                                        }
                                        let _e1370 = phi_25664_;
                                        phi_25666_ = _e1370;
                                    } else {
                                        phi_25666_ = _e1303.member_1.member_1;
                                    }
                                    let _e1372 = phi_25666_;
                                    if (_e1372 <= _e1316) {
                                        phi_25675_ = type_58(0u, type_58().member_1);
                                    } else {
                                        phi_25675_ = type_58(1u, type_57(_e1316, _e1372));
                                    }
                                    let _e1380 = phi_25675_;
                                    phi_25679_ = _e1380;
                                }
                                let _e1385 = phi_25679_;
                                phi_25683_ = _e1385;
                            }
                            let _e1390 = phi_25683_;
                            if (bitcast<i32>(_e1390.member) != 0i) {
                                let _e1396 = (_e1390.member_1.member < _e1180.member);
                                if _e1396 {
                                    if (_e1390.member_1.member_1 != _e1390.member_1.member_1) {
                                        phi_25954_ = true;
                                    } else {
                                        phi_25954_ = (_e1180.member <= _e1390.member_1.member_1);
                                    }
                                    let _e1402 = phi_25954_;
                                    let _e1403 = select(_e1390.member_1.member_1, _e1180.member, _e1402);
                                    phi_13420_ = vec3<f32>(0f, 0f, 0f);
                                    phi_13423_ = 0u;
                                    loop {
                                        let _e1405 = phi_13420_;
                                        let _e1407 = phi_13423_;
                                        local_76 = _e1405;
                                        local_77 = _e1405;
                                        local_78 = _e1405;
                                        let _e1408 = (_e1407 < 32u);
                                        if _e1408 {
                                            let _e1409 = -(_e1246);
                                            if (_e1390.member_1.member <= _e1403) {
                                            } else {
                                                phi_33080_ = true;
                                                break;
                                            }
                                            let _e1412 = select(_e1409, _e1390.member_1.member, (_e1409 < _e1390.member_1.member));
                                            let _e1414 = select(_e1412, _e1403, (_e1412 > _e1403));
                                            let _e1415 = (_e1414 - _e1390.member_1.member);
                                            let _e1416 = (_e1403 - _e1414);
                                            let _e1417 = (_e1403 - _e1390.member_1.member);
                                            let _e1418 = (0.000000001f * _e1417);
                                            if (_e1415 <= _e1418) {
                                                let _e1493 = (f32(_e1407) * 0.03125f);
                                                let _e1496 = (f32((_e1407 + 1u)) * 0.03125f);
                                                let _e1498 = ((_e1493 + _e1496) * 0.5f);
                                                phi_13524_ = type_57((_e1390.member_1.member + ((_e1417 * _e1498) * _e1498)), (_e1417 * ((_e1496 * _e1496) - (_e1493 * _e1493))));
                                            } else {
                                                if (_e1416 <= _e1418) {
                                                    let _e1476 = (f32((31u - _e1407)) * 0.03125f);
                                                    let _e1479 = (f32((32u - _e1407)) * 0.03125f);
                                                    let _e1481 = ((_e1476 + _e1479) * 0.5f);
                                                    phi_13523_ = type_57((_e1403 - ((_e1417 * _e1481) * _e1481)), (_e1417 * ((_e1479 * _e1479) - (_e1476 * _e1476))));
                                                } else {
                                                    let _e1424 = round(((32f * _e1415) / (_e1415 + _e1416)));
                                                    let _e1429 = select(select(u32(_e1424), 0u, (_e1424 < 0f)), 4294967295u, (_e1424 > 4294967000f));
                                                    if (_e1429 < 1u) {
                                                        phi_13507_ = 1u;
                                                    } else {
                                                        phi_13507_ = select(_e1429, 31u, (_e1429 > 31u));
                                                    }
                                                    let _e1434 = phi_13507_;
                                                    if (_e1407 < _e1434) {
                                                        let _e1455 = ((_e1434 - 1u) - _e1407);
                                                        let _e1457 = f32(_e1434);
                                                        let _e1458 = (f32(_e1455) / _e1457);
                                                        let _e1461 = (f32((_e1455 + 1u)) / _e1457);
                                                        let _e1463 = ((_e1458 + _e1461) * 0.5f);
                                                        phi_13522_ = type_57((_e1414 - ((_e1415 * _e1463) * _e1463)), (_e1415 * ((_e1461 * _e1461) - (_e1458 * _e1458))));
                                                    } else {
                                                        let _e1437 = (_e1407 - _e1434);
                                                        let _e1439 = f32((32u - _e1434));
                                                        let _e1440 = (f32(_e1437) / _e1439);
                                                        let _e1443 = (f32((_e1437 + 1u)) / _e1439);
                                                        let _e1445 = ((_e1440 + _e1443) * 0.5f);
                                                        phi_13522_ = type_57((_e1414 + ((_e1416 * _e1445) * _e1445)), (_e1416 * ((_e1443 * _e1443) - (_e1440 * _e1440))));
                                                    }
                                                    let _e1473 = phi_13522_;
                                                    phi_13523_ = _e1473;
                                                }
                                                let _e1491 = phi_13523_;
                                                phi_13524_ = _e1491;
                                            }
                                            let _e1508 = phi_13524_;
                                            let _e1514 = (_e334.member.x + (_e334.member_1.x * _e1508.member));
                                            let _e1515 = (_e334.member.y + (_e334.member_1.y * _e1508.member));
                                            let _e1516 = (_e334.member.z + (_e334.member_1.z * _e1508.member));
                                            let _e1530 = (_e1514 - _e1196);
                                            let _e1531 = (_e1515 - _e1199);
                                            let _e1532 = (_e1516 - _e1202);
                                            let _e1540 = ((((((_e1514 * _e1514) + (_e1515 * _e1515)) + (_e1516 * _e1516)) - (2f * (((_e1514 * _e1196) + (_e1515 * _e1199)) + (_e1516 * _e1202)))) + _e1208) / (sqrt((((_e1530 * _e1530) + (_e1531 * _e1531)) + (_e1532 * _e1532))) + _e1205));
                                            let _e1544 = (1f - (abs((_e1540 - 25000f)) * 0.00006666667f));
                                            if (_e1544 != _e1544) {
                                                phi_26153_ = true;
                                            } else {
                                                phi_26153_ = (0f >= _e1544);
                                            }
                                            let _e1548 = phi_26153_;
                                            let _e1550 = -(_e1540);
                                            phi_13421_ = (_e1405 + vec3<f32>((exp((_e1550 / _e1214)) * _e1508.member_1), (exp((_e1550 / _e1217)) * _e1508.member_1), (select(_e1544, 0f, _e1548) * _e1508.member_1)));
                                            phi_13424_ = (_e1407 + 1u);
                                        } else {
                                            phi_13421_ = vec3<f32>();
                                            phi_13424_ = u32();
                                        }
                                        let _e1562 = phi_13421_;
                                        let _e1564 = phi_13424_;
                                        phi_13420_ = _e1562;
                                        phi_13423_ = _e1564;
                                        phi_33080_ = _e1021;
                                        if !(_e1408) {
                                            break;
                                        }
                                    }
                                    let _e1567 = phi_33080_;
                                    if _e1567 {
                                        break;
                                    }
                                    let _e1569 = local_76;
                                    let _e1576 = local_77;
                                    let _e1578 = ((_e1229 * 1.1f) * _e1576.y);
                                    let _e1583 = local_78;
                                    phi_35348_ = _e1567;
                                    phi_13549_ = vec3<f32>(exp(-((((_e1220 * _e1569.x) + _e1578) + (_e1232 * _e1583.z)))), exp(-((((_e1223 * _e1569.x) + _e1578) + (_e1235 * _e1583.z)))), exp(-((((_e1226 * _e1569.x) + _e1578) + (_e1238 * _e1583.z)))));
                                } else {
                                    phi_35348_ = _e1021;
                                    phi_13549_ = vec3<f32>();
                                }
                                let _e1599 = phi_35348_;
                                let _e1601 = phi_13549_;
                                phi_35347_ = _e1599;
                                phi_13551_ = _e1601;
                                phi_13552_ = select(true, false, _e1396);
                            } else {
                                phi_35347_ = _e1021;
                                phi_13551_ = vec3<f32>();
                                phi_13552_ = true;
                            }
                            let _e1604 = phi_35347_;
                            let _e1606 = phi_13551_;
                            let _e1608 = phi_13552_;
                            phi_35345_ = _e1604;
                            phi_13559_ = (_e352 * select(_e1606, vec3<f32>(1f, 1f, 1f), vec3(_e1608)));
                            phi_13560_ = type_42(vec4<f32>(_e334.member.x, _e334.member.y, _e334.member.z, _e1180.member), vec4<f32>(_e334.member_1.x, _e334.member_1.y, _e334.member_1.z, 0f), vec4<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z, 0f));
                        } else {
                            phi_35345_ = _e1021;
                            phi_13559_ = _e352;
                            phi_13560_ = type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e1613 = phi_35345_;
                        let _e1615 = phi_13559_;
                        let _e1617 = phi_13560_;
                        phi_35344_ = _e1613;
                        phi_13561_ = _e1615;
                        phi_13562_ = _e1617;
                    }
                    let _e1619 = phi_35344_;
                    let _e1621 = phi_13561_;
                    let _e1623 = phi_13562_;
                    let _e1628 = (_e334.member.x + (_e334.member_1.x * _e1180.member));
                    let _e1629 = (_e334.member.y + (_e334.member_1.y * _e1180.member));
                    let _e1630 = (_e334.member.z + (_e334.member_1.z * _e1180.member));
                    let _e1632 = global_2.member[_e1180.member_1];
                    phi_13586_ = 0i;
                    phi_13589_ = _e1632.member_6.y;
                    loop {
                        let _e1636 = phi_13586_;
                        let _e1638 = phi_13589_;
                        local_79 = _e1638;
                        if (_e1636 < 4i) {
                            let _e1641 = global_4.member[_e1638];
                            let _e1643 = (_e1641.member != 6u);
                            if _e1643 {
                                phi_13658_ = i32();
                                phi_13659_ = u32();
                            } else {
                                let _e1647 = (abs(_e1628) / _e1641.member_6.z);
                                let _e1658 = (abs(_e1630) / _e1641.member_6.w);
                                if ((select(select(u32(_e1647), 0u, (_e1647 < 0f)), 4294967295u, (_e1647 > 4294967000f)) % 2u) == 0u) {
                                    phi_13643_ = (_e1628 > 0f);
                                } else {
                                    phi_13643_ = (_e1628 <= 0f);
                                }
                                let _e1669 = phi_13643_;
                                if ((select(select(u32(_e1658), 0u, (_e1658 < 0f)), 4294967295u, (_e1658 > 4294967000f)) % 2u) == 0u) {
                                    phi_13649_ = (_e1630 > 0f);
                                } else {
                                    phi_13649_ = (_e1630 <= 0f);
                                }
                                let _e1673 = phi_13649_;
                                if (_e1669 != _e1673) {
                                    phi_13656_ = _e1641.member_1;
                                } else {
                                    phi_13656_ = _e1641.member_2;
                                }
                                let _e1678 = phi_13656_;
                                phi_13658_ = (_e1636 + 1i);
                                phi_13659_ = _e1678;
                            }
                            let _e1681 = phi_13658_;
                            let _e1683 = phi_13659_;
                            phi_13587_ = _e1681;
                            phi_13590_ = _e1683;
                            phi_13661_ = select(true, false, _e1643);
                        } else {
                            phi_13587_ = i32();
                            phi_13590_ = u32();
                            phi_13661_ = false;
                        }
                        let _e1686 = phi_13587_;
                        let _e1688 = phi_13590_;
                        let _e1690 = phi_13661_;
                        phi_13586_ = _e1686;
                        phi_13589_ = _e1688;
                        if !(_e1690) {
                            break;
                        }
                    }
                    let _e1693 = local_79;
                    let _e1695 = global_4.member[_e1693];
                    let _e1697 = (_e1695.member == 4u);
                    if _e1697 {
                        if _e1697 {
                            if (_e1695.member_1 == 1u) {
                                phi_13721_ = true;
                            } else {
                                phi_13721_ = ((((-(_e334.member_1.x) * _e1180.member_3.x) + (-(_e334.member_1.y) * _e1180.member_3.y)) + (-(_e334.member_1.z) * _e1180.member_3.z)) > 0f);
                            }
                            let _e7002 = phi_13721_;
                            if _e7002 {
                                phi_13730_ = vec3<f32>(_e1695.member_4.x, _e1695.member_4.y, _e1695.member_4.z);
                            } else {
                                phi_13730_ = vec3<f32>();
                            }
                            let _e7009 = phi_13730_;
                            phi_13732_ = _e7009;
                            phi_13733_ = select(true, false, _e7002);
                        } else {
                            phi_13732_ = vec3<f32>();
                            phi_13733_ = true;
                        }
                        let _e7012 = phi_13732_;
                        let _e7014 = phi_13733_;
                        let _e7016 = select(_e7012, vec3<f32>(0f, 0f, 0f), vec3(_e7014));
                        if _e372 {
                            phi_13903_ = 1f;
                        } else {
                            if ((_e378 & 1u) == 0u) {
                                phi_13902_ = 1f;
                            } else {
                                if (_e1632.member_6.z > 0u) {
                                    let _e7025 = global_1.member.member_2[1u];
                                    let _e7026 = (_e7025 + (_e1632.member_6.z - 1u));
                                    let _e7029 = global_6.member[(_e7026 + 1u)];
                                    let _e7031 = global_6.member[_e7026];
                                    if (_e1632.member_6.x == 1u) {
                                        let _e7108 = (_e1632.member.x - _e334.member.x);
                                        let _e7109 = (_e1632.member.y - _e334.member.y);
                                        let _e7110 = (_e1632.member.z - _e334.member.z);
                                        let _e7115 = (((_e7108 * _e7108) + (_e7109 * _e7109)) + (_e7110 * _e7110));
                                        let _e7116 = (_e1632.member.w * _e1632.member.w);
                                        if (_e7115 <= (_e7116 * 1.000001f)) {
                                            let _e7130 = (_e1628 - _e334.member.x);
                                            let _e7131 = (_e1629 - _e334.member.y);
                                            let _e7132 = (_e1630 - _e334.member.z);
                                            let _e7137 = (((_e7130 * _e7130) + (_e7131 * _e7131)) + (_e7132 * _e7132));
                                            phi_31881_ = (_e7137 / (((abs((((_e1180.member_2.x * _e7130) + (_e1180.member_2.y * _e7131)) + (_e1180.member_2.z * _e7132))) / sqrt(_e7137)) * 12.566371f) * _e7116));
                                        } else {
                                            let _e7119 = (_e7116 / _e7115);
                                            if (_e7119 != _e7119) {
                                                phi_31900_ = true;
                                            } else {
                                                phi_31900_ = (1f <= _e7119);
                                            }
                                            let _e7123 = phi_31900_;
                                            let _e7124 = select(_e7119, 1f, _e7123);
                                            phi_31881_ = (0.15915494f / (_e7124 / (1f + sqrt((1f - _e7124)))));
                                        }
                                        let _e7153 = phi_31881_;
                                        phi_13898_ = _e7153;
                                    } else {
                                        let _e7037 = (_e1628 - _e334.member.x);
                                        let _e7038 = (_e1629 - _e334.member.y);
                                        let _e7039 = (_e1630 - _e334.member.z);
                                        let _e7044 = (((_e7037 * _e7037) + (_e7038 * _e7038)) + (_e7039 * _e7039));
                                        let _e7055 = (abs((((_e1180.member_2.x * _e7037) + (_e1180.member_2.y * _e7038)) + (_e1180.member_2.z * _e7039))) / sqrt(_e7044));
                                        if (_e7055 <= 0f) {
                                            phi_13897_ = 0f;
                                        } else {
                                            if (_e1632.member_6.x == 0u) {
                                                let _e7070 = (_e1632.member_1.x - _e1632.member.x);
                                                let _e7071 = (_e1632.member_1.y - _e1632.member.y);
                                                let _e7072 = (_e1632.member_1.z - _e1632.member.z);
                                                let _e7077 = (_e1632.member_2.x - _e1632.member.x);
                                                let _e7078 = (_e1632.member_2.y - _e1632.member.y);
                                                let _e7079 = (_e1632.member_2.z - _e1632.member.z);
                                                let _e7082 = ((_e7071 * _e7079) - (_e7078 * _e7072));
                                                let _e7085 = ((_e7072 * _e7077) - (_e7079 * _e7070));
                                                let _e7088 = ((_e7070 * _e7078) - (_e7077 * _e7071));
                                                phi_13894_ = (sqrt((((_e7082 * _e7082) + (_e7085 * _e7085)) + (_e7088 * _e7088))) * 0.5f);
                                            } else {
                                                phi_13894_ = ((12.566371f * _e1632.member.w) * _e1632.member.w);
                                            }
                                            let _e7097 = phi_13894_;
                                            phi_13897_ = (_e7044 / (_e7055 * _e7097));
                                        }
                                        let _e7101 = phi_13897_;
                                        phi_13898_ = _e7101;
                                    }
                                    let _e7155 = phi_13898_;
                                    let _e7156 = ((_e7029 - _e7031) * _e7155);
                                    if (_e334.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_31921_ = 1f;
                                    } else {
                                        let _e7158 = (_e334.member_1.w * _e334.member_1.w);
                                        let _e7160 = (_e7158 + (_e7156 * _e7156));
                                        if (_e7160 == 0f) {
                                            phi_31918_ = 0f;
                                        } else {
                                            phi_31918_ = (_e7158 / _e7160);
                                        }
                                        let _e7164 = phi_31918_;
                                        phi_31921_ = _e7164;
                                    }
                                    let _e7166 = phi_31921_;
                                    phi_13901_ = _e7166;
                                } else {
                                    phi_13901_ = 1f;
                                }
                                let _e7168 = phi_13901_;
                                phi_13902_ = _e7168;
                            }
                            let _e7170 = phi_13902_;
                            phi_13903_ = _e7170;
                        }
                        let _e7172 = phi_13903_;
                        phi_19507_ = type_73(type_35(_e334.member, _e334.member_1, vec4<f32>(_e1621.x, _e1621.y, _e1621.z, f32(_e358)), vec4<f32>((_e334.member_3.x + ((_e1621.x * _e7016.x) * _e7172)), (_e334.member_3.y + ((_e1621.y * _e7016.y) * _e7172)), (_e334.member_3.z + ((_e1621.z * _e7016.z) * _e7172)), f32(_e1181)), vec4<u32>(_e334.member_4.x, (_e334.member_4.y & 4294967294u), 0u, 0u)), _e1623, type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19508_ = type_73();
                        phi_19509_ = true;
                    } else {
                        let _e1698 = -(_e334.member_1.x);
                        let _e1699 = -(_e334.member_1.y);
                        let _e1700 = -(_e334.member_1.z);
                        if ((((_e1180.member_2.x * _e1698) + (_e1180.member_2.y * _e1699)) + (_e1180.member_2.z * _e1700)) < 0f) {
                            phi_13942_ = -(_e1180.member_2);
                        } else {
                            phi_13942_ = _e1180.member_2;
                        }
                        let _e1713 = phi_13942_;
                        if ((((_e1180.member_3.x * _e1698) + (_e1180.member_3.y * _e1699)) + (_e1180.member_3.z * _e1700)) < 0f) {
                            phi_13959_ = -(_e1180.member_3);
                        } else {
                            phi_13959_ = _e1180.member_3;
                        }
                        let _e1726 = phi_13959_;
                        let _e1729 = global_1.member.member_4[2u];
                        let _e1730 = abs(_e1628);
                        let _e1731 = abs(_e1629);
                        let _e1732 = abs(_e1630);
                        let _e1734 = select(_e1731, _e1730, (_e1730 > _e1731));
                        let _e1738 = (0.00002f * (1f + select(_e1732, _e1734, (_e1734 > _e1732))));
                        if (_e1729 != _e1729) {
                            phi_26220_ = true;
                        } else {
                            phi_26220_ = (_e1738 >= _e1729);
                        }
                        let _e1742 = phi_26220_;
                        let _e1744 = (select(_e1729, _e1738, _e1742) * 2f);
                        let _e1751 = (_e1628 + (_e1713.x * _e1744));
                        let _e1752 = (_e1629 + (_e1713.y * _e1744));
                        let _e1753 = (_e1630 + (_e1713.z * _e1744));
                        let _e1754 = vec3<f32>(_e1751, _e1752, _e1753);
                        if ((_e334.member_4.y & 4u) == 0u) {
                            phi_13992_ = 0f;
                        } else {
                            let _e1759 = global_1.member.member_4[0u];
                            phi_13992_ = _e1759;
                        }
                        let _e1761 = phi_13992_;
                        let _e1764 = select(-1f, 1f, (_e1726.z >= 0f));
                        let _e1766 = (-1f / (_e1764 + _e1726.z));
                        let _e1770 = ((_e1726.x * _e1726.y) * _e1766);
                        let _e1774 = (1f + (((_e1764 * _e1726.x) * _e1726.x) * _e1766));
                        let _e1775 = (_e1764 * _e1770);
                        let _e1777 = (-(_e1764) * _e1726.x);
                        let _e1780 = (_e1764 + ((_e1726.y * _e1726.y) * _e1766));
                        let _e1781 = -(_e1726.y);
                        if (_e1695.member == 0u) {
                            phi_33995_ = vec3<f32>(0f, 0f, 0f);
                            phi_33803_ = 1.5f;
                            phi_33724_ = vec3<f32>(0f, 0f, 0f);
                            phi_33647_ = vec3<f32>(_e1695.member_4.x, _e1695.member_4.y, _e1695.member_4.z);
                            phi_33568_ = 0u;
                            phi_33486_ = 1f;
                            phi_33409_ = 0u;
                            phi_33400_ = _e334.member_4.x;
                            phi_14671_ = type_73();
                            phi_14672_ = true;
                        } else {
                            if (_e1695.member == 1u) {
                                let _e2330 = (_e1695.member_6.y * _e1695.member_6.y);
                                if (_e2330 != _e2330) {
                                    phi_26912_ = true;
                                } else {
                                    phi_26912_ = (_e1761 >= _e2330);
                                }
                                let _e2334 = phi_26912_;
                                let _e2335 = select(_e2330, _e1761, _e2334);
                                let _e2336 = (_e2335 >= 0.001f);
                                if _e2336 {
                                    phi_34052_ = vec3<f32>(_e1695.member_4.x, _e1695.member_4.y, _e1695.member_4.z);
                                    phi_33846_ = _e1695.member_6.x;
                                    phi_33699_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33387_ = _e334.member_4.x;
                                    phi_14067_ = vec3<f32>();
                                    phi_14068_ = true;
                                } else {
                                    let _e2338 = ((_e334.member_4.x * 747796405u) + 2891336453u);
                                    let _e2346 = (((_e2338 >> bitcast<u32>((((_e2338 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2338) * 277803737u);
                                    let _e2349 = ((_e2346 >> bitcast<u32>(22i)) ^ _e2346);
                                    let _e2359 = abs((((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z)));
                                    let _e2362 = (1f / _e1695.member_6.x);
                                    let _e2366 = ((_e2362 * _e2362) * (1f - (_e2359 * _e2359)));
                                    if (_e2366 >= 1f) {
                                        phi_26952_ = 1f;
                                    } else {
                                        let _e2369 = sqrt((1f - _e2366));
                                        if (_e2359 != _e2359) {
                                            phi_26963_ = true;
                                        } else {
                                            phi_26963_ = (_e2369 <= _e2359);
                                        }
                                        let _e2373 = phi_26963_;
                                        let _e2378 = pow(((1f - _e1695.member_6.x) / (1f + _e1695.member_6.x)), 2f);
                                        phi_26952_ = (_e2378 + ((1f - _e2378) * pow((1f - select(_e2359, _e2369, _e2373)), 5f)));
                                    }
                                    let _e2385 = phi_26952_;
                                    let _e2386 = ((f32((_e2349 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e2385);
                                    if _e2386 {
                                        let _e2399 = (((_e1726.x * _e334.member_1.x) + (_e1726.y * _e334.member_1.y)) + (_e1726.z * _e334.member_1.z));
                                        phi_33700_ = vec3<f32>(0f, 0f, 0f);
                                        phi_14065_ = (_e347 - vec3<f32>(((_e1726.x * 2f) * _e2399), ((_e1726.y * 2f) * _e2399), ((_e1726.z * 2f) * _e2399)));
                                    } else {
                                        phi_33700_ = vec3<f32>(_e1695.member_4.x, _e1695.member_4.y, _e1695.member_4.z);
                                        phi_14065_ = vec3<f32>();
                                    }
                                    let _e2406 = phi_33700_;
                                    let _e2408 = phi_14065_;
                                    phi_34052_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33846_ = 1.5f;
                                    phi_33699_ = _e2406;
                                    phi_33387_ = _e2349;
                                    phi_14067_ = _e2408;
                                    phi_14068_ = select(true, false, _e2386);
                                }
                                let _e2418 = phi_34052_;
                                let _e2420 = phi_33846_;
                                let _e2422 = phi_33699_;
                                let _e2424 = phi_33387_;
                                let _e2426 = phi_14067_;
                                let _e2428 = phi_14068_;
                                phi_33998_ = _e2418;
                                phi_33806_ = _e2420;
                                phi_33727_ = vec3<f32>(0f, 0f, 0f);
                                phi_33650_ = _e2422;
                                phi_33571_ = select(0u, 1u, _e2336);
                                phi_33489_ = select(1f, _e2335, _e2336);
                                phi_33412_ = select(0u, 2u, _e2336);
                                phi_33338_ = _e2424;
                                phi_14541_ = _e1754;
                                phi_14542_ = _e2426;
                                phi_14543_ = vec3<f32>(1f, 1f, 1f);
                                phi_14544_ = _e2428;
                            } else {
                                if (_e1695.member == 3u) {
                                    phi_33341_ = _e334.member_4.x;
                                    phi_14220_ = vec3<f32>();
                                    phi_14221_ = vec3<f32>();
                                    phi_14222_ = vec3<f32>();
                                    phi_14223_ = true;
                                    phi_14224_ = false;
                                } else {
                                    let _e1785 = (_e1695.member == 5u);
                                    if _e1785 {
                                        phi_33342_ = _e334.member_4.x;
                                        phi_14216_ = vec3<f32>();
                                        phi_14217_ = vec3<f32>();
                                        phi_14218_ = vec3<f32>();
                                        phi_14219_ = false;
                                    } else {
                                        let _e1786 = (_e1695.member == 2u);
                                        if _e1786 {
                                            let _e1791 = (((_e334.member_1.x * _e1180.member_3.x) + (_e334.member_1.y * _e1180.member_3.y)) + (_e334.member_1.z * _e1180.member_3.z));
                                            let _e1792 = (_e1791 > 0f);
                                            if _e1792 {
                                                phi_14101_ = -(_e1180.member_3);
                                            } else {
                                                phi_14101_ = _e1180.member_3;
                                            }
                                            let _e1795 = phi_14101_;
                                            if _e1792 {
                                                phi_14107_ = _e1695.member_6.x;
                                            } else {
                                                phi_14107_ = (1f / _e1695.member_6.x);
                                            }
                                            let _e1800 = phi_14107_;
                                            let _e1801 = abs(_e1791);
                                            let _e1802 = (_e1800 * _e1800);
                                            let _e1805 = (_e1802 * (1f - (_e1801 * _e1801)));
                                            if (_e1805 >= 1f) {
                                                phi_26282_ = 1f;
                                            } else {
                                                let _e1808 = sqrt((1f - _e1805));
                                                if (_e1801 != _e1801) {
                                                    phi_26293_ = true;
                                                } else {
                                                    phi_26293_ = (_e1808 <= _e1801);
                                                }
                                                let _e1812 = phi_26293_;
                                                let _e1817 = pow(((1f - _e1695.member_6.x) / (1f + _e1695.member_6.x)), 2f);
                                                phi_26282_ = (_e1817 + ((1f - _e1817) * pow((1f - select(_e1801, _e1808, _e1812)), 5f)));
                                            }
                                            let _e1824 = phi_26282_;
                                            let _e1832 = (((_e334.member_1.x * _e1795.x) + (_e334.member_1.y * _e1795.y)) + (_e334.member_1.z * _e1795.z));
                                            let _e1836 = (1f - (_e1802 * (1f - (_e1832 * _e1832))));
                                            if (_e1836 > 0f) {
                                                let _e1839 = ((_e334.member_4.x * 747796405u) + 2891336453u);
                                                let _e1847 = (((_e1839 >> bitcast<u32>((((_e1839 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1839) * 277803737u);
                                                let _e1850 = ((_e1847 >> bitcast<u32>(22i)) ^ _e1847);
                                                let _e1855 = ((f32((_e1850 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1824);
                                                if _e1855 {
                                                    let _e1865 = sqrt(_e1836);
                                                    let _e1869 = (((_e334.member_1.x - (_e1795.x * _e1832)) * _e1800) - (_e1795.x * _e1865));
                                                    let _e1870 = (((_e334.member_1.y - (_e1795.y * _e1832)) * _e1800) - (_e1795.y * _e1865));
                                                    let _e1871 = (((_e334.member_1.z - (_e1795.z * _e1832)) * _e1800) - (_e1795.z * _e1865));
                                                    let _e1878 = (1f / sqrt((((_e1869 * _e1869) + (_e1870 * _e1870)) + (_e1871 * _e1871))));
                                                    phi_14188_ = vec3<f32>((_e1628 - (_e1795.x * _e1744)), (_e1629 - (_e1795.y * _e1744)), (_e1630 - (_e1795.z * _e1744)));
                                                    phi_14189_ = vec3<f32>((_e1869 * _e1878), (_e1870 * _e1878), (_e1871 * _e1878));
                                                    phi_14190_ = vec3<f32>(_e1695.member_4.x, _e1695.member_4.y, _e1695.member_4.z);
                                                } else {
                                                    phi_14188_ = vec3<f32>();
                                                    phi_14189_ = vec3<f32>();
                                                    phi_14190_ = vec3<f32>();
                                                }
                                                let _e1896 = phi_14188_;
                                                let _e1898 = phi_14189_;
                                                let _e1900 = phi_14190_;
                                                phi_33345_ = _e1850;
                                                phi_14192_ = _e1896;
                                                phi_14193_ = _e1898;
                                                phi_14194_ = _e1900;
                                                phi_14195_ = select(true, false, _e1855);
                                            } else {
                                                phi_33345_ = _e334.member_4.x;
                                                phi_14192_ = vec3<f32>();
                                                phi_14193_ = vec3<f32>();
                                                phi_14194_ = vec3<f32>();
                                                phi_14195_ = true;
                                            }
                                            let _e1903 = phi_33345_;
                                            let _e1905 = phi_14192_;
                                            let _e1907 = phi_14193_;
                                            let _e1909 = phi_14194_;
                                            let _e1911 = phi_14195_;
                                            if _e1911 {
                                                let _e1919 = (((_e1180.member_3.x * _e334.member_1.x) + (_e1180.member_3.y * _e334.member_1.y)) + (_e1180.member_3.z * _e334.member_1.z));
                                                phi_14210_ = vec3<f32>((_e1628 + (_e1795.x * _e1744)), (_e1629 + (_e1795.y * _e1744)), (_e1630 + (_e1795.z * _e1744)));
                                                phi_14211_ = (_e347 - vec3<f32>(((_e1180.member_3.x * 2f) * _e1919), ((_e1180.member_3.y * 2f) * _e1919), ((_e1180.member_3.z * 2f) * _e1919)));
                                            } else {
                                                phi_14210_ = _e1905;
                                                phi_14211_ = _e1907;
                                            }
                                            let _e1933 = phi_14210_;
                                            let _e1935 = phi_14211_;
                                            phi_33343_ = _e1903;
                                            phi_14213_ = _e1933;
                                            phi_14214_ = _e1935;
                                            phi_14215_ = select(_e1909, vec3<f32>(1f, 1f, 1f), vec3(_e1911));
                                        } else {
                                            phi_33343_ = _e334.member_4.x;
                                            phi_14213_ = vec3<f32>();
                                            phi_14214_ = vec3<f32>();
                                            phi_14215_ = vec3<f32>();
                                        }
                                        let _e1939 = phi_33343_;
                                        let _e1941 = phi_14213_;
                                        let _e1943 = phi_14214_;
                                        let _e1945 = phi_14215_;
                                        phi_33342_ = _e1939;
                                        phi_14216_ = _e1941;
                                        phi_14217_ = _e1943;
                                        phi_14218_ = _e1945;
                                        phi_14219_ = _e1786;
                                    }
                                    let _e1947 = phi_33342_;
                                    let _e1949 = phi_14216_;
                                    let _e1951 = phi_14217_;
                                    let _e1953 = phi_14218_;
                                    let _e1955 = phi_14219_;
                                    phi_33341_ = _e1947;
                                    phi_14220_ = _e1949;
                                    phi_14221_ = _e1951;
                                    phi_14222_ = _e1953;
                                    phi_14223_ = _e1785;
                                    phi_14224_ = _e1955;
                                }
                                let _e1957 = phi_33341_;
                                let _e1959 = phi_14220_;
                                let _e1961 = phi_14221_;
                                let _e1963 = phi_14222_;
                                let _e1965 = phi_14223_;
                                let _e1967 = phi_14224_;
                                if _e1965 {
                                    let _e1970 = (_e1695.member_6.y * _e1695.member_6.y);
                                    if (_e1970 != _e1970) {
                                        phi_26366_ = true;
                                    } else {
                                        phi_26366_ = (_e1761 >= _e1970);
                                    }
                                    let _e1974 = phi_26366_;
                                    let _e1975 = select(_e1970, _e1761, _e1974);
                                    let _e1987 = select(2u, 0u, (_e1695.member == 5u));
                                    let _e1988 = (_e1975 < 0.001f);
                                    if _e1988 {
                                        let _e1996 = (((_e1726.x * _e334.member_1.x) + (_e1726.y * _e334.member_1.y)) + (_e1726.z * _e334.member_1.z));
                                        let _e2007 = abs((((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z)));
                                        if (_e1987 == 0u) {
                                            let _e2289 = (1f - _e2007);
                                            if (_e2289 != _e2289) {
                                                phi_26897_ = true;
                                            } else {
                                                phi_26897_ = (0f >= _e2289);
                                            }
                                            let _e2293 = phi_26897_;
                                            let _e2295 = pow(select(_e2289, 0f, _e2293), 5f);
                                            let _e2296 = (1f - _e2295);
                                            phi_14527_ = vec3<f32>(((_e1695.member_4.x * _e2296) + _e2295), ((_e1695.member_4.y * _e2296) + _e2295), ((_e1695.member_4.z * _e2296) + _e2295));
                                        } else {
                                            if (_e1987 == 1u) {
                                                let _e2271 = (0.44444448f * (1f - (_e2007 * _e2007)));
                                                if (_e2271 >= 1f) {
                                                    phi_26858_ = 1f;
                                                } else {
                                                    let _e2274 = sqrt((1f - _e2271));
                                                    if (_e2007 != _e2007) {
                                                        phi_26869_ = true;
                                                    } else {
                                                        phi_26869_ = (_e2274 <= _e2007);
                                                    }
                                                    let _e2278 = phi_26869_;
                                                    phi_26858_ = (0.040000003f + (0.96f * pow((1f - select(_e2007, _e2274, _e2278)), 5f)));
                                                }
                                                let _e2285 = phi_26858_;
                                                phi_14526_ = vec3(_e2285);
                                            } else {
                                                let _e2011 = select(_e2007, 0f, (_e2007 < 0f));
                                                let _e2013 = select(_e2011, 1f, (_e2011 > 1f));
                                                let _e2015 = (1f - (_e2013 * _e2013));
                                                let _e2022 = ((_e1695.member_4.x * _e1695.member_4.x) - (_e1695.member_5.x * _e1695.member_5.x));
                                                let _e2023 = ((_e1695.member_4.y * _e1695.member_4.y) - (_e1695.member_5.y * _e1695.member_5.y));
                                                let _e2024 = ((_e1695.member_4.z * _e1695.member_4.z) - (_e1695.member_5.z * _e1695.member_5.z));
                                                let _e2026 = (_e1695.member_4.x * (_e1695.member_5.x + _e1695.member_5.x));
                                                let _e2028 = (_e1695.member_4.y * (_e1695.member_5.y + _e1695.member_5.y));
                                                let _e2030 = (_e1695.member_4.z * (_e1695.member_5.z + _e1695.member_5.z));
                                                let _e2037 = ((_e2022 * _e2022) + (_e2026 * _e2026));
                                                let _e2038 = ((_e2023 * _e2023) + (_e2028 * _e2028));
                                                let _e2039 = ((_e2024 * _e2024) + (_e2030 * _e2030));
                                                let _e2044 = ((_e2015 * _e2022) / _e2037);
                                                let _e2045 = ((_e2015 * _e2023) / _e2038);
                                                let _e2046 = ((_e2015 * _e2024) / _e2039);
                                                let _e2047 = (1f - _e2044);
                                                let _e2048 = (1f - _e2045);
                                                let _e2049 = (1f - _e2046);
                                                let _e2051 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2044, _e2045, _e2046));
                                                let _e2052 = (_e2015 * _e2026);
                                                let _e2053 = (_e2015 * _e2028);
                                                let _e2054 = (_e2015 * _e2030);
                                                let _e2059 = (vec3<f32>(_e2052, _e2053, _e2054) / vec3<f32>(_e2037, _e2038, _e2039));
                                                let _e2072 = ((_e2051.x * _e2051.x) + (_e2059.x * _e2059.x));
                                                let _e2073 = ((_e2051.y * _e2051.y) + (_e2059.y * _e2059.y));
                                                let _e2074 = ((_e2051.z * _e2051.z) + (_e2059.z * _e2059.z));
                                                if (_e2072 != _e2072) {
                                                    phi_26528_ = true;
                                                } else {
                                                    phi_26528_ = (0f >= _e2072);
                                                }
                                                let _e2078 = phi_26528_;
                                                let _e2080 = sqrt(select(_e2072, 0f, _e2078));
                                                if (_e2073 != _e2073) {
                                                    phi_26543_ = true;
                                                } else {
                                                    phi_26543_ = (0f >= _e2073);
                                                }
                                                let _e2084 = phi_26543_;
                                                let _e2086 = sqrt(select(_e2073, 0f, _e2084));
                                                if (_e2074 != _e2074) {
                                                    phi_26558_ = true;
                                                } else {
                                                    phi_26558_ = (0f >= _e2074);
                                                }
                                                let _e2090 = phi_26558_;
                                                let _e2092 = sqrt(select(_e2074, 0f, _e2090));
                                                let _e2096 = ((_e2080 + _e2047) * 0.5f);
                                                let _e2097 = ((_e2086 + _e2048) * 0.5f);
                                                let _e2098 = ((_e2092 + _e2049) * 0.5f);
                                                if (_e2096 != _e2096) {
                                                    phi_26585_ = true;
                                                } else {
                                                    phi_26585_ = (0f >= _e2096);
                                                }
                                                let _e2102 = phi_26585_;
                                                let _e2104 = sqrt(select(_e2096, 0f, _e2102));
                                                if (_e2097 != _e2097) {
                                                    phi_26600_ = true;
                                                } else {
                                                    phi_26600_ = (0f >= _e2097);
                                                }
                                                let _e2108 = phi_26600_;
                                                let _e2110 = sqrt(select(_e2097, 0f, _e2108));
                                                if (_e2098 != _e2098) {
                                                    phi_26615_ = true;
                                                } else {
                                                    phi_26615_ = (0f >= _e2098);
                                                }
                                                let _e2114 = phi_26615_;
                                                let _e2116 = sqrt(select(_e2098, 0f, _e2114));
                                                let _e2117 = vec3<f32>(_e2104, _e2110, _e2116);
                                                let _e2121 = ((_e2080 - _e2047) * 0.5f);
                                                let _e2122 = ((_e2086 - _e2048) * 0.5f);
                                                let _e2123 = ((_e2092 - _e2049) * 0.5f);
                                                if (_e2121 != _e2121) {
                                                    phi_26642_ = true;
                                                } else {
                                                    phi_26642_ = (0f >= _e2121);
                                                }
                                                let _e2127 = phi_26642_;
                                                let _e2129 = sqrt(select(_e2121, 0f, _e2127));
                                                if (_e2122 != _e2122) {
                                                    phi_26657_ = true;
                                                } else {
                                                    phi_26657_ = (0f >= _e2122);
                                                }
                                                let _e2133 = phi_26657_;
                                                let _e2135 = sqrt(select(_e2122, 0f, _e2133));
                                                if (_e2123 != _e2123) {
                                                    phi_26672_ = true;
                                                } else {
                                                    phi_26672_ = (0f >= _e2123);
                                                }
                                                let _e2139 = phi_26672_;
                                                let _e2141 = sqrt(select(_e2123, 0f, _e2139));
                                                if ((_e2052 / _e2037) < 0f) {
                                                    phi_14403_ = -(_e2129);
                                                } else {
                                                    phi_14403_ = _e2129;
                                                }
                                                let _e2145 = phi_14403_;
                                                if ((_e2053 / _e2038) < 0f) {
                                                    phi_14411_ = -(_e2135);
                                                } else {
                                                    phi_14411_ = _e2135;
                                                }
                                                let _e2149 = phi_14411_;
                                                if ((_e2054 / _e2039) < 0f) {
                                                    phi_14419_ = -(_e2141);
                                                } else {
                                                    phi_14419_ = _e2141;
                                                }
                                                let _e2153 = phi_14419_;
                                                let _e2157 = (_e1695.member_5.x * _e2013);
                                                let _e2158 = (_e1695.member_5.y * _e2013);
                                                let _e2159 = (_e1695.member_5.z * _e2013);
                                                let _e2169 = vec3<f32>(((_e1695.member_4.x * _e2104) - (_e1695.member_5.x * _e2145)), ((_e1695.member_4.y * _e2110) - (_e1695.member_5.y * _e2149)), ((_e1695.member_4.z * _e2116) - (_e1695.member_5.z * _e2153)));
                                                let _e2176 = ((_e1695.member_4.x * _e2145) + (_e1695.member_5.x * _e2104));
                                                let _e2177 = ((_e1695.member_4.y * _e2149) + (_e1695.member_5.y * _e2110));
                                                let _e2178 = ((_e1695.member_4.z * _e2153) + (_e1695.member_5.z * _e2116));
                                                let _e2180 = vec3<f32>((_e1695.member_4.x * _e2013), (_e1695.member_4.y * _e2013), (_e1695.member_4.z * _e2013));
                                                let _e2181 = (_e2180 - _e2117);
                                                let _e2182 = (_e2157 - _e2145);
                                                let _e2183 = (_e2158 - _e2149);
                                                let _e2184 = (_e2159 - _e2153);
                                                let _e2197 = (_e2180 + _e2117);
                                                let _e2198 = (_e2157 + _e2145);
                                                let _e2199 = (_e2158 + _e2149);
                                                let _e2200 = (_e2159 + _e2153);
                                                let _e2210 = ((_e2197.x * _e2197.x) + (_e2198 * _e2198));
                                                let _e2211 = ((_e2197.y * _e2197.y) + (_e2199 * _e2199));
                                                let _e2212 = ((_e2197.z * _e2197.z) + (_e2200 * _e2200));
                                                let _e2222 = vec3(_e2013);
                                                let _e2223 = (_e2222 - _e2169);
                                                let _e2224 = -(vec3<f32>(_e2176, _e2177, _e2178));
                                                let _e2240 = (_e2222 + _e2169);
                                                let _e2250 = ((_e2240.x * _e2240.x) + (_e2176 * _e2176));
                                                let _e2251 = ((_e2240.y * _e2240.y) + (_e2177 * _e2177));
                                                let _e2252 = ((_e2240.z * _e2240.z) + (_e2178 * _e2178));
                                                phi_14526_ = vec3<f32>((((((_e2181.x * _e2181.x) + (_e2182 * _e2182)) / select(0.000000000000000000000000000001f, _e2210, (_e2210 > 0.000000000000000000000000000001f))) + (((_e2223.x * _e2223.x) + (_e2224.x * _e2224.x)) / select(0.000000000000000000000000000001f, _e2250, (_e2250 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2181.y * _e2181.y) + (_e2183 * _e2183)) / select(0.000000000000000000000000000001f, _e2211, (_e2211 > 0.000000000000000000000000000001f))) + (((_e2223.y * _e2223.y) + (_e2224.y * _e2224.y)) / select(0.000000000000000000000000000001f, _e2251, (_e2251 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2181.z * _e2181.z) + (_e2184 * _e2184)) / select(0.000000000000000000000000000001f, _e2212, (_e2212 > 0.000000000000000000000000000001f))) + (((_e2223.z * _e2223.z) + (_e2224.z * _e2224.z)) / select(0.000000000000000000000000000001f, _e2252, (_e2252 > 0.000000000000000000000000000001f)))) * 0.5f));
                                            }
                                            let _e2288 = phi_14526_;
                                            phi_14527_ = _e2288;
                                        }
                                        let _e2305 = phi_14527_;
                                        phi_14528_ = (_e347 - vec3<f32>(((_e1726.x * 2f) * _e1996), ((_e1726.y * 2f) * _e1996), ((_e1726.z * 2f) * _e1996)));
                                        phi_14529_ = _e2305;
                                    } else {
                                        phi_14528_ = vec3<f32>();
                                        phi_14529_ = vec3<f32>();
                                    }
                                    let _e2307 = phi_14528_;
                                    let _e2309 = phi_14529_;
                                    phi_33729_ = vec3<f32>(_e1695.member_5.x, _e1695.member_5.y, _e1695.member_5.z);
                                    phi_33652_ = vec3<f32>(_e1695.member_4.x, _e1695.member_4.y, _e1695.member_4.z);
                                    phi_33573_ = _e1987;
                                    phi_33491_ = _e1975;
                                    phi_14531_ = _e2307;
                                    phi_14532_ = _e2309;
                                    phi_14533_ = _e1988;
                                } else {
                                    phi_33729_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33652_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33573_ = 0u;
                                    phi_33491_ = 1f;
                                    phi_14531_ = _e1961;
                                    phi_14532_ = _e1963;
                                    phi_14533_ = _e1967;
                                }
                                let _e2311 = phi_33729_;
                                let _e2313 = phi_33652_;
                                let _e2315 = phi_33573_;
                                let _e2317 = phi_33491_;
                                let _e2319 = phi_14531_;
                                let _e2321 = phi_14532_;
                                let _e2323 = phi_14533_;
                                phi_33998_ = vec3<f32>(0f, 0f, 0f);
                                phi_33806_ = 1.5f;
                                phi_33727_ = _e2311;
                                phi_33650_ = _e2313;
                                phi_33571_ = _e2315;
                                phi_33489_ = _e2317;
                                phi_33412_ = select(0u, 1u, _e1965);
                                phi_33338_ = _e1957;
                                phi_14541_ = select(_e1959, _e1754, vec3(_e1965));
                                phi_14542_ = _e2319;
                                phi_14543_ = _e2321;
                                phi_14544_ = select(true, false, _e2323);
                            }
                            let _e2433 = phi_33998_;
                            let _e2435 = phi_33806_;
                            let _e2437 = phi_33727_;
                            let _e2439 = phi_33650_;
                            let _e2441 = phi_33571_;
                            let _e2443 = phi_33489_;
                            let _e2445 = phi_33412_;
                            let _e2447 = phi_33338_;
                            let _e2449 = phi_14541_;
                            let _e2451 = phi_14542_;
                            let _e2453 = phi_14543_;
                            let _e2455 = phi_14544_;
                            let _e2456 = select(true, false, _e2455);
                            if _e2456 {
                                if (_e358 >= _e375) {
                                    phi_33397_ = _e2447;
                                    phi_14660_ = _e1621;
                                    phi_14661_ = type_73();
                                    phi_14662_ = true;
                                } else {
                                    let _e2469 = global_1.member.member_4[1u];
                                    if (sqrt((((_e2453.x * _e2453.x) + (_e2453.y * _e2453.y)) + (_e2453.z * _e2453.z))) <= _e2469) {
                                        phi_33392_ = _e2447;
                                        phi_14624_ = _e1621;
                                        phi_14625_ = false;
                                    } else {
                                        let _e2473 = global_1.member.member_7[1u];
                                        if (_e2473 == 1u) {
                                            let _e2478 = global_1.member.member_4[3u];
                                            if (f32(_e358) >= _e2478) {
                                                let _e2481 = select(_e2453.y, _e2453.x, (_e2453.x > _e2453.y));
                                                let _e2483 = select(_e2453.z, _e2481, (_e2481 > _e2453.z));
                                                if (_e2483 != _e2483) {
                                                    phi_27029_ = true;
                                                } else {
                                                    phi_27029_ = (1f <= _e2483);
                                                }
                                                let _e2487 = phi_27029_;
                                                let _e2488 = select(_e2483, 1f, _e2487);
                                                let _e2490 = ((_e2447 * 747796405u) + 2891336453u);
                                                let _e2498 = (((_e2490 >> bitcast<u32>((((_e2490 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2490) * 277803737u);
                                                let _e2501 = ((_e2498 >> bitcast<u32>(22i)) ^ _e2498);
                                                let _e2506 = ((f32((_e2501 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e2488);
                                                if _e2506 {
                                                    phi_14613_ = vec3<f32>();
                                                } else {
                                                    phi_14613_ = vec3<f32>((_e1621.x / _e2488), (_e1621.y / _e2488), (_e1621.z / _e2488));
                                                }
                                                let _e2515 = phi_14613_;
                                                phi_33395_ = _e2501;
                                                phi_14615_ = _e2515;
                                                phi_14616_ = select(true, false, _e2506);
                                            } else {
                                                phi_33395_ = _e2447;
                                                phi_14615_ = _e1621;
                                                phi_14616_ = true;
                                            }
                                            let _e2518 = phi_33395_;
                                            let _e2520 = phi_14615_;
                                            let _e2522 = phi_14616_;
                                            phi_33394_ = _e2518;
                                            phi_14617_ = _e2520;
                                            phi_14618_ = _e2522;
                                        } else {
                                            phi_33394_ = _e2447;
                                            phi_14617_ = _e1621;
                                            phi_14618_ = true;
                                        }
                                        let _e2524 = phi_33394_;
                                        let _e2526 = phi_14617_;
                                        let _e2528 = phi_14618_;
                                        phi_33392_ = _e2524;
                                        phi_14624_ = select(_e1621, _e2526, vec3(_e2528));
                                        phi_14625_ = select(false, true, _e2528);
                                    }
                                    let _e2533 = phi_33392_;
                                    let _e2535 = phi_14624_;
                                    let _e2537 = phi_14625_;
                                    if _e2537 {
                                        let _e2538 = (_e2535 * _e2453);
                                        let _e2549 = (_e2451 * (1f / sqrt((((_e2451.x * _e2451.x) + (_e2451.y * _e2451.y)) + (_e2451.z * _e2451.z)))));
                                        phi_14658_ = type_73(type_35(vec4<f32>(_e2449.x, _e2449.y, _e2449.z, _e334.member.w), vec4<f32>(_e2549.x, _e2549.y, _e2549.z, _e334.member_1.w), vec4<f32>(_e2538.x, _e2538.y, _e2538.z, f32((_e358 + 1u))), vec4<f32>(_e334.member_3.x, _e334.member_3.y, _e334.member_3.z, f32(_e1181)), vec4<u32>(_e2533, (_e334.member_4.y | 2u), 0u, 0u)), _e1623, type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14658_ = type_73();
                                    }
                                    let _e2572 = phi_14658_;
                                    phi_33397_ = _e2533;
                                    phi_14660_ = _e2535;
                                    phi_14661_ = _e2572;
                                    phi_14662_ = select(true, false, _e2537);
                                }
                                let _e2575 = phi_33397_;
                                let _e2577 = phi_14660_;
                                let _e2579 = phi_14661_;
                                let _e2581 = phi_14662_;
                                if _e2581 {
                                    phi_14668_ = type_73(type_35(_e334.member, _e334.member_1, vec4<f32>(_e2577.x, _e2577.y, _e2577.z, f32(_e358)), vec4<f32>(_e334.member_3.x, _e334.member_3.y, _e334.member_3.z, f32(_e1181)), vec4<u32>(_e2575, (_e334.member_4.y & 4294967294u), 0u, 0u)), _e1623, type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                } else {
                                    phi_14668_ = _e2579;
                                }
                                let _e2594 = phi_14668_;
                                phi_33401_ = _e2575;
                                phi_14669_ = _e2594;
                            } else {
                                phi_33401_ = _e2447;
                                phi_14669_ = type_73();
                            }
                            let _e2596 = phi_33401_;
                            let _e2598 = phi_14669_;
                            phi_33995_ = _e2433;
                            phi_33803_ = _e2435;
                            phi_33724_ = _e2437;
                            phi_33647_ = _e2439;
                            phi_33568_ = _e2441;
                            phi_33486_ = _e2443;
                            phi_33409_ = _e2445;
                            phi_33400_ = _e2596;
                            phi_14671_ = _e2598;
                            phi_14672_ = select(true, false, _e2456);
                        }
                        let _e2606 = phi_33995_;
                        let _e2608 = phi_33803_;
                        let _e2610 = phi_33724_;
                        let _e2612 = phi_33647_;
                        let _e2614 = phi_33568_;
                        let _e2616 = phi_33486_;
                        let _e2618 = phi_33409_;
                        let _e2620 = phi_33400_;
                        let _e2622 = phi_14671_;
                        let _e2624 = phi_14672_;
                        if _e2624 {
                            let _e2626 = ((_e334.member_4.y | 4u) & 4294967293u);
                            let _e2629 = global_1.member.member_1[3u];
                            phi_35324_ = _e1619;
                            phi_33399_ = _e2620;
                            phi_14683_ = _e362;
                            phi_14686_ = type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_14688_ = 0u;
                            phi_14690_ = _e1181;
                            loop {
                                let _e2631 = phi_35324_;
                                let _e2633 = phi_33399_;
                                let _e2635 = phi_14683_;
                                let _e2637 = phi_14686_;
                                let _e2639 = phi_14688_;
                                let _e2641 = phi_14690_;
                                local_87 = _e2641;
                                local_88 = _e2635;
                                local_89 = _e2635;
                                local_90 = _e2635;
                                local_91 = _e2637;
                                local_92 = _e2641;
                                local_93 = _e2635;
                                local_94 = _e2635;
                                local_95 = _e2635;
                                local_96 = _e2637;
                                local_97 = _e2641;
                                local_98 = _e2635;
                                local_99 = _e2635;
                                local_100 = _e2635;
                                local_101 = _e2637;
                                local_102 = _e2641;
                                local_103 = _e2635;
                                local_104 = _e2635;
                                local_105 = _e2635;
                                local_106 = _e2637;
                                let _e2643 = (_e2639 < (_e2629 + 3u));
                                if _e2643 {
                                    if (_e2639 < _e2629) {
                                        let _e4628 = global_5.member[_e2639];
                                        let _e4633 = (_e4628.member.x - _e1751);
                                        let _e4634 = (_e4628.member.y - _e1752);
                                        let _e4635 = (_e4628.member.z - _e1753);
                                        let _e4640 = (((_e4633 * _e4633) + (_e4634 * _e4634)) + (_e4635 * _e4635));
                                        let _e4641 = sqrt(_e4640);
                                        let _e4642 = (_e4633 / _e4641);
                                        let _e4643 = (_e4634 / _e4641);
                                        let _e4644 = (_e4635 / _e4641);
                                        if (_e4628.member.w < 0.5f) {
                                            phi_14787_ = 1f;
                                        } else {
                                            let _e4668 = (((_e4628.member_2.x * -(_e4642)) + (_e4628.member_2.y * -(_e4643))) + (_e4628.member_2.z * -(_e4644)));
                                            if (_e4628.member_3.x == _e4628.member_2.w) {
                                                phi_14786_ = select(1f, 0f, (_e4668 < _e4628.member_3.x));
                                            } else {
                                                let _e4672 = ((_e4668 - _e4628.member_3.x) / (_e4628.member_2.w - _e4628.member_3.x));
                                                let _e4674 = select(_e4672, 0f, (_e4672 < 0f));
                                                let _e4676 = select(_e4674, 1f, (_e4674 > 1f));
                                                phi_14786_ = ((_e4676 * _e4676) * (3f - (2f * _e4676)));
                                            }
                                            let _e4684 = phi_14786_;
                                            phi_14787_ = _e4684;
                                        }
                                        let _e4686 = phi_14787_;
                                        phi_35742_ = _e2633;
                                        phi_17725_ = 0f;
                                        phi_17726_ = vec3<f32>(_e4642, _e4643, _e4644);
                                        phi_17727_ = vec3<f32>(((_e4628.member_1.x * _e4686) / _e4640), ((_e4628.member_1.y * _e4686) / _e4640), ((_e4628.member_1.z * _e4686) / _e4640));
                                        phi_17728_ = _e4641;
                                    } else {
                                        if (_e2639 == _e2629) {
                                            if ((_e378 & 4u) == 0u) {
                                                phi_35786_ = _e2633;
                                                phi_16829_ = 0f;
                                                phi_16830_ = vec3<f32>(0f, 0f, 0f);
                                                phi_16831_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                let _e3380 = global_1.member.member_5;
                                                let _e3388 = global_1.member.member_6[3u];
                                                let _e3390 = sin((_e3388 * 0.5f));
                                                let _e3392 = ((12.566371f * _e3390) * _e3390);
                                                let _e3397 = (((_e3380.x * _e1726.x) + (_e3380.y * _e1726.y)) + (_e3380.z * _e1726.z));
                                                if (_e3397 != _e3397) {
                                                    phi_27568_ = true;
                                                } else {
                                                    phi_27568_ = (0f >= _e3397);
                                                }
                                                let _e3401 = phi_27568_;
                                                let _e3403 = (select(_e3397, 0f, _e3401) * 0.31830987f);
                                                if (_e2618 == 0u) {
                                                    if (_e3397 <= 0f) {
                                                        phi_28365_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_28365_ = (_e2612 * (_e3397 * 0.31830987f));
                                                    }
                                                    let _e3879 = phi_28365_;
                                                    phi_15197_ = type_60(_e3879, _e3403);
                                                    phi_15198_ = type_60();
                                                    phi_15199_ = true;
                                                } else {
                                                    let _e3409 = (((_e1698 * _e1774) + (_e1699 * _e1775)) + (_e1700 * _e1777));
                                                    let _e3414 = (((_e1698 * _e1770) + (_e1699 * _e1780)) + (_e1700 * _e1781));
                                                    let _e3419 = (((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z));
                                                    let _e3424 = (((_e3380.x * _e1774) + (_e3380.y * _e1775)) + (_e3380.z * _e1777));
                                                    let _e3429 = (((_e3380.x * _e1770) + (_e3380.y * _e1780)) + (_e3380.z * _e1781));
                                                    if (_e3419 <= 0f) {
                                                        phi_15161_ = type_60();
                                                        phi_15162_ = true;
                                                    } else {
                                                        let _e3431 = (_e3397 <= 0f);
                                                        if _e3431 {
                                                            phi_15160_ = type_60();
                                                        } else {
                                                            let _e3432 = (_e3409 + _e3424);
                                                            let _e3433 = (_e3414 + _e3429);
                                                            let _e3434 = (_e3419 + _e3397);
                                                            let _e3441 = (1f / sqrt((((_e3432 * _e3432) + (_e3433 * _e3433)) + (_e3434 * _e3434))));
                                                            let _e3444 = (_e3434 * _e3441);
                                                            if (_e3444 <= 0f) {
                                                                phi_27678_ = 0f;
                                                            } else {
                                                                let _e3446 = (_e2616 * _e2616);
                                                                let _e3450 = ((((_e3446 - 1f) * _e3444) * _e3444) + 1f);
                                                                phi_27678_ = (_e3446 / ((3.1415927f * _e3450) * _e3450));
                                                            }
                                                            let _e3455 = phi_27678_;
                                                            let _e3456 = (_e3419 * _e3419);
                                                            if (_e3456 >= 1f) {
                                                                phi_27706_ = 0f;
                                                            } else {
                                                                phi_27706_ = ((sqrt((1f + (((_e2616 * _e2616) * ((_e3409 * _e3409) + (_e3414 * _e3414))) / _e3456))) - 1f) * 0.5f);
                                                            }
                                                            let _e3469 = phi_27706_;
                                                            let _e3470 = (1f + _e3469);
                                                            let _e3471 = (_e3397 * _e3397);
                                                            if (_e3471 >= 1f) {
                                                                phi_27734_ = 0f;
                                                            } else {
                                                                phi_27734_ = ((sqrt((1f + (((_e2616 * _e2616) * ((_e3424 * _e3424) + (_e3429 * _e3429))) / _e3471))) - 1f) * 0.5f);
                                                            }
                                                            let _e3484 = phi_27734_;
                                                            let _e3491 = (((_e3409 * (_e3432 * _e3441)) + (_e3414 * (_e3433 * _e3441))) + (_e3419 * _e3444));
                                                            if (_e2614 == 0u) {
                                                                let _e3786 = (1f - _e3491);
                                                                if (_e3786 != _e3786) {
                                                                    phi_28237_ = true;
                                                                } else {
                                                                    phi_28237_ = (0f >= _e3786);
                                                                }
                                                                let _e3790 = phi_28237_;
                                                                let _e3792 = pow(select(_e3786, 0f, _e3790), 5f);
                                                                let _e3793 = (1f - _e3792);
                                                                phi_15148_ = vec3<f32>(((_e2612.x * _e3793) + _e3792), ((_e2612.y * _e3793) + _e3792), ((_e2612.z * _e3793) + _e3792));
                                                            } else {
                                                                if (_e2614 == 1u) {
                                                                    let _e3759 = (1f / _e2608);
                                                                    let _e3763 = ((_e3759 * _e3759) * (1f - (_e3491 * _e3491)));
                                                                    if (_e3763 >= 1f) {
                                                                        phi_28198_ = 1f;
                                                                    } else {
                                                                        let _e3766 = sqrt((1f - _e3763));
                                                                        if (_e3491 != _e3491) {
                                                                            phi_28209_ = true;
                                                                        } else {
                                                                            phi_28209_ = (_e3766 <= _e3491);
                                                                        }
                                                                        let _e3770 = phi_28209_;
                                                                        let _e3775 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                                                        phi_28198_ = (_e3775 + ((1f - _e3775) * pow((1f - select(_e3491, _e3766, _e3770)), 5f)));
                                                                    }
                                                                    let _e3782 = phi_28198_;
                                                                    phi_15147_ = vec3(_e3782);
                                                                } else {
                                                                    let _e3495 = select(_e3491, 0f, (_e3491 < 0f));
                                                                    let _e3497 = select(_e3495, 1f, (_e3495 > 1f));
                                                                    let _e3499 = (1f - (_e3497 * _e3497));
                                                                    let _e3512 = ((_e2612.x * _e2612.x) - (_e2610.x * _e2610.x));
                                                                    let _e3513 = ((_e2612.y * _e2612.y) - (_e2610.y * _e2610.y));
                                                                    let _e3514 = ((_e2612.z * _e2612.z) - (_e2610.z * _e2610.z));
                                                                    let _e3516 = (_e2612.x * (_e2610.x + _e2610.x));
                                                                    let _e3518 = (_e2612.y * (_e2610.y + _e2610.y));
                                                                    let _e3520 = (_e2612.z * (_e2610.z + _e2610.z));
                                                                    let _e3527 = ((_e3512 * _e3512) + (_e3516 * _e3516));
                                                                    let _e3528 = ((_e3513 * _e3513) + (_e3518 * _e3518));
                                                                    let _e3529 = ((_e3514 * _e3514) + (_e3520 * _e3520));
                                                                    let _e3534 = ((_e3499 * _e3512) / _e3527);
                                                                    let _e3535 = ((_e3499 * _e3513) / _e3528);
                                                                    let _e3536 = ((_e3499 * _e3514) / _e3529);
                                                                    let _e3537 = (1f - _e3534);
                                                                    let _e3538 = (1f - _e3535);
                                                                    let _e3539 = (1f - _e3536);
                                                                    let _e3541 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3534, _e3535, _e3536));
                                                                    let _e3542 = (_e3499 * _e3516);
                                                                    let _e3543 = (_e3499 * _e3518);
                                                                    let _e3544 = (_e3499 * _e3520);
                                                                    let _e3549 = (vec3<f32>(_e3542, _e3543, _e3544) / vec3<f32>(_e3527, _e3528, _e3529));
                                                                    let _e3562 = ((_e3541.x * _e3541.x) + (_e3549.x * _e3549.x));
                                                                    let _e3563 = ((_e3541.y * _e3541.y) + (_e3549.y * _e3549.y));
                                                                    let _e3564 = ((_e3541.z * _e3541.z) + (_e3549.z * _e3549.z));
                                                                    if (_e3562 != _e3562) {
                                                                        phi_27868_ = true;
                                                                    } else {
                                                                        phi_27868_ = (0f >= _e3562);
                                                                    }
                                                                    let _e3568 = phi_27868_;
                                                                    let _e3570 = sqrt(select(_e3562, 0f, _e3568));
                                                                    if (_e3563 != _e3563) {
                                                                        phi_27883_ = true;
                                                                    } else {
                                                                        phi_27883_ = (0f >= _e3563);
                                                                    }
                                                                    let _e3574 = phi_27883_;
                                                                    let _e3576 = sqrt(select(_e3563, 0f, _e3574));
                                                                    if (_e3564 != _e3564) {
                                                                        phi_27898_ = true;
                                                                    } else {
                                                                        phi_27898_ = (0f >= _e3564);
                                                                    }
                                                                    let _e3580 = phi_27898_;
                                                                    let _e3582 = sqrt(select(_e3564, 0f, _e3580));
                                                                    let _e3586 = ((_e3570 + _e3537) * 0.5f);
                                                                    let _e3587 = ((_e3576 + _e3538) * 0.5f);
                                                                    let _e3588 = ((_e3582 + _e3539) * 0.5f);
                                                                    if (_e3586 != _e3586) {
                                                                        phi_27925_ = true;
                                                                    } else {
                                                                        phi_27925_ = (0f >= _e3586);
                                                                    }
                                                                    let _e3592 = phi_27925_;
                                                                    let _e3594 = sqrt(select(_e3586, 0f, _e3592));
                                                                    if (_e3587 != _e3587) {
                                                                        phi_27940_ = true;
                                                                    } else {
                                                                        phi_27940_ = (0f >= _e3587);
                                                                    }
                                                                    let _e3598 = phi_27940_;
                                                                    let _e3600 = sqrt(select(_e3587, 0f, _e3598));
                                                                    if (_e3588 != _e3588) {
                                                                        phi_27955_ = true;
                                                                    } else {
                                                                        phi_27955_ = (0f >= _e3588);
                                                                    }
                                                                    let _e3604 = phi_27955_;
                                                                    let _e3606 = sqrt(select(_e3588, 0f, _e3604));
                                                                    let _e3607 = vec3<f32>(_e3594, _e3600, _e3606);
                                                                    let _e3611 = ((_e3570 - _e3537) * 0.5f);
                                                                    let _e3612 = ((_e3576 - _e3538) * 0.5f);
                                                                    let _e3613 = ((_e3582 - _e3539) * 0.5f);
                                                                    if (_e3611 != _e3611) {
                                                                        phi_27982_ = true;
                                                                    } else {
                                                                        phi_27982_ = (0f >= _e3611);
                                                                    }
                                                                    let _e3617 = phi_27982_;
                                                                    let _e3619 = sqrt(select(_e3611, 0f, _e3617));
                                                                    if (_e3612 != _e3612) {
                                                                        phi_27997_ = true;
                                                                    } else {
                                                                        phi_27997_ = (0f >= _e3612);
                                                                    }
                                                                    let _e3623 = phi_27997_;
                                                                    let _e3625 = sqrt(select(_e3612, 0f, _e3623));
                                                                    if (_e3613 != _e3613) {
                                                                        phi_28012_ = true;
                                                                    } else {
                                                                        phi_28012_ = (0f >= _e3613);
                                                                    }
                                                                    let _e3629 = phi_28012_;
                                                                    let _e3631 = sqrt(select(_e3613, 0f, _e3629));
                                                                    if ((_e3542 / _e3527) < 0f) {
                                                                        phi_15024_ = -(_e3619);
                                                                    } else {
                                                                        phi_15024_ = _e3619;
                                                                    }
                                                                    let _e3635 = phi_15024_;
                                                                    if ((_e3543 / _e3528) < 0f) {
                                                                        phi_15032_ = -(_e3625);
                                                                    } else {
                                                                        phi_15032_ = _e3625;
                                                                    }
                                                                    let _e3639 = phi_15032_;
                                                                    if ((_e3544 / _e3529) < 0f) {
                                                                        phi_15040_ = -(_e3631);
                                                                    } else {
                                                                        phi_15040_ = _e3631;
                                                                    }
                                                                    let _e3643 = phi_15040_;
                                                                    let _e3647 = (_e2610.x * _e3497);
                                                                    let _e3648 = (_e2610.y * _e3497);
                                                                    let _e3649 = (_e2610.z * _e3497);
                                                                    let _e3659 = vec3<f32>(((_e2612.x * _e3594) - (_e2610.x * _e3635)), ((_e2612.y * _e3600) - (_e2610.y * _e3639)), ((_e2612.z * _e3606) - (_e2610.z * _e3643)));
                                                                    let _e3666 = ((_e2612.x * _e3635) + (_e2610.x * _e3594));
                                                                    let _e3667 = ((_e2612.y * _e3639) + (_e2610.y * _e3600));
                                                                    let _e3668 = ((_e2612.z * _e3643) + (_e2610.z * _e3606));
                                                                    let _e3670 = vec3<f32>((_e2612.x * _e3497), (_e2612.y * _e3497), (_e2612.z * _e3497));
                                                                    let _e3671 = (_e3670 - _e3607);
                                                                    let _e3672 = (_e3647 - _e3635);
                                                                    let _e3673 = (_e3648 - _e3639);
                                                                    let _e3674 = (_e3649 - _e3643);
                                                                    let _e3687 = (_e3670 + _e3607);
                                                                    let _e3688 = (_e3647 + _e3635);
                                                                    let _e3689 = (_e3648 + _e3639);
                                                                    let _e3690 = (_e3649 + _e3643);
                                                                    let _e3700 = ((_e3687.x * _e3687.x) + (_e3688 * _e3688));
                                                                    let _e3701 = ((_e3687.y * _e3687.y) + (_e3689 * _e3689));
                                                                    let _e3702 = ((_e3687.z * _e3687.z) + (_e3690 * _e3690));
                                                                    let _e3712 = vec3(_e3497);
                                                                    let _e3713 = (_e3712 - _e3659);
                                                                    let _e3714 = -(vec3<f32>(_e3666, _e3667, _e3668));
                                                                    let _e3730 = (_e3712 + _e3659);
                                                                    let _e3740 = ((_e3730.x * _e3730.x) + (_e3666 * _e3666));
                                                                    let _e3741 = ((_e3730.y * _e3730.y) + (_e3667 * _e3667));
                                                                    let _e3742 = ((_e3730.z * _e3730.z) + (_e3668 * _e3668));
                                                                    phi_15147_ = vec3<f32>((((((_e3671.x * _e3671.x) + (_e3672 * _e3672)) / select(0.000000000000000000000000000001f, _e3700, (_e3700 > 0.000000000000000000000000000001f))) + (((_e3713.x * _e3713.x) + (_e3714.x * _e3714.x)) / select(0.000000000000000000000000000001f, _e3740, (_e3740 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3671.y * _e3671.y) + (_e3673 * _e3673)) / select(0.000000000000000000000000000001f, _e3701, (_e3701 > 0.000000000000000000000000000001f))) + (((_e3713.y * _e3713.y) + (_e3714.y * _e3714.y)) / select(0.000000000000000000000000000001f, _e3741, (_e3741 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3671.z * _e3671.z) + (_e3674 * _e3674)) / select(0.000000000000000000000000000001f, _e3702, (_e3702 > 0.000000000000000000000000000001f))) + (((_e3713.z * _e3713.z) + (_e3714.z * _e3714.z)) / select(0.000000000000000000000000000001f, _e3742, (_e3742 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e3785 = phi_15147_;
                                                                phi_15148_ = _e3785;
                                                            }
                                                            let _e3805 = phi_15148_;
                                                            let _e3807 = (4f * _e3419);
                                                            phi_15160_ = type_60((_e3805 * (((1f / (_e3470 + _e3484)) * _e3455) / _e3807)), ((_e3455 / _e3470) / _e3807));
                                                        }
                                                        let _e3814 = phi_15160_;
                                                        phi_15161_ = _e3814;
                                                        phi_15162_ = _e3431;
                                                    }
                                                    let _e3816 = phi_15161_;
                                                    let _e3818 = phi_15162_;
                                                    if _e3818 {
                                                        phi_15167_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_15167_ = _e3816;
                                                    }
                                                    let _e3820 = phi_15167_;
                                                    let _e3823 = (_e2618 == 1u);
                                                    if _e3823 {
                                                        phi_15195_ = _e3820;
                                                        phi_15196_ = type_60();
                                                    } else {
                                                        if (_e3419 != _e3419) {
                                                            phi_28274_ = true;
                                                        } else {
                                                            phi_28274_ = (0f >= _e3419);
                                                        }
                                                        let _e3827 = phi_28274_;
                                                        let _e3828 = select(_e3419, 0f, _e3827);
                                                        let _e3829 = (1f / _e2608);
                                                        let _e3833 = ((_e3829 * _e3829) * (1f - (_e3828 * _e3828)));
                                                        if (_e3833 >= 1f) {
                                                            phi_28293_ = 1f;
                                                        } else {
                                                            let _e3836 = sqrt((1f - _e3833));
                                                            if (_e3828 != _e3828) {
                                                                phi_28304_ = true;
                                                            } else {
                                                                phi_28304_ = (_e3836 <= _e3828);
                                                            }
                                                            let _e3840 = phi_28304_;
                                                            let _e3845 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                                            phi_28293_ = (_e3845 + ((1f - _e3845) * pow((1f - select(_e3828, _e3836, _e3840)), 5f)));
                                                        }
                                                        let _e3852 = phi_28293_;
                                                        if (_e3397 <= 0f) {
                                                            phi_28343_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28343_ = (_e2606 * (_e3397 * 0.31830987f));
                                                        }
                                                        let _e3857 = phi_28343_;
                                                        let _e3858 = (1f - _e3852);
                                                        phi_15195_ = type_60();
                                                        phi_15196_ = type_60((_e3820.member + vec3<f32>((_e3857.x * _e3858), (_e3857.y * _e3858), (_e3857.z * _e3858))), ((_e3852 * _e3820.member_1) + (_e3858 * _e3403)));
                                                    }
                                                    let _e3872 = phi_15195_;
                                                    let _e3874 = phi_15196_;
                                                    phi_15197_ = _e3872;
                                                    phi_15198_ = _e3874;
                                                    phi_15199_ = _e3823;
                                                }
                                                let _e3882 = phi_15197_;
                                                let _e3884 = phi_15198_;
                                                let _e3886 = phi_15199_;
                                                if _e3886 {
                                                    phi_15203_ = _e3882;
                                                } else {
                                                    phi_15203_ = _e3884;
                                                }
                                                let _e3888 = phi_15203_;
                                                if ((_e3888.member_1 * _e3392) < 0.001f) {
                                                    let _e4315 = global_1.member.member_6;
                                                    let _e4321 = global_1.member.member_8[2u];
                                                    if (_e4321 == 4294967295u) {
                                                        phi_15942_ = vec3<f32>(1f, 1f, 1f);
                                                    } else {
                                                        let _e4324 = global_6.member[_e4321];
                                                        let _e4327 = global_6.member[(_e4321 + 1u)];
                                                        let _e4330 = global_6.member[(_e4321 + 2u)];
                                                        let _e4333 = global_6.member[(_e4321 + 3u)];
                                                        let _e4336 = global_6.member[(_e4321 + 4u)];
                                                        let _e4339 = global_6.member[(_e4321 + 5u)];
                                                        let _e4342 = global_6.member[(_e4321 + 6u)];
                                                        let _e4345 = global_6.member[(_e4321 + 7u)];
                                                        let _e4348 = global_6.member[(_e4321 + 8u)];
                                                        let _e4351 = global_6.member[(_e4321 + 9u)];
                                                        let _e4354 = global_6.member[(_e4321 + 13u)];
                                                        let _e4357 = global_6.member[(_e4321 + 14u)];
                                                        let _e4360 = global_6.member[(_e4321 + 15u)];
                                                        let _e4363 = global_6.member[(_e4321 + 16u)];
                                                        let _e4366 = global_6.member[(_e4321 + 18u)];
                                                        let _e4369 = global_6.member[(_e4321 + 19u)];
                                                        let _e4372 = global_6.member[(_e4321 + 20u)];
                                                        let _e4375 = global_6.member[(_e4321 + 29u)];
                                                        let _e4376 = bitcast<u32>(_e4375);
                                                        let _e4389 = (((((_e1751 * _e1751) + (_e1752 * _e1752)) + (_e1753 * _e1753)) - (2f * (((_e1751 * _e4324) + (_e1752 * _e4327)) + (_e1753 * _e4330)))) + _e4342);
                                                        if (_e4389 > 0f) {
                                                            let _e4398 = ((((_e1751 - _e4324) * _e4345) + ((_e1752 - _e4327) * _e4348)) + ((_e1753 - _e4330) * _e4351));
                                                            let _e4400 = ((_e4398 * _e4398) - _e4389);
                                                            if (_e4398 >= 0f) {
                                                                phi_15642_ = vec3<f32>();
                                                                phi_15643_ = true;
                                                            } else {
                                                                let _e4402 = (_e4400 <= 0f);
                                                                if _e4402 {
                                                                    phi_15641_ = vec3<f32>();
                                                                } else {
                                                                    let _e4405 = (-(_e4398) - sqrt(_e4400));
                                                                    phi_15641_ = vec3<f32>((_e1751 + (_e4345 * _e4405)), (_e1752 + (_e4348 * _e4405)), (_e1753 + (_e4351 * _e4405)));
                                                                }
                                                                let _e4414 = phi_15641_;
                                                                phi_15642_ = _e4414;
                                                                phi_15643_ = _e4402;
                                                            }
                                                            let _e4416 = phi_15642_;
                                                            let _e4418 = phi_15643_;
                                                            phi_15648_ = _e4416;
                                                            phi_15649_ = select(true, false, _e4418);
                                                        } else {
                                                            phi_15648_ = _e1754;
                                                            phi_15649_ = true;
                                                        }
                                                        let _e4421 = phi_15648_;
                                                        let _e4423 = phi_15649_;
                                                        if _e4423 {
                                                            let _e4425 = (_e4421.x - _e4324);
                                                            let _e4427 = (_e4421.y - _e4327);
                                                            let _e4429 = (_e4421.z - _e4330);
                                                            let _e4435 = sqrt((((_e4425 * _e4425) + (_e4427 * _e4427)) + (_e4429 * _e4429)));
                                                            let _e4436 = (1f / _e4435);
                                                            let _e4454 = ((((((_e4421.x * _e4421.x) + (_e4421.y * _e4421.y)) + (_e4421.z * _e4421.z)) - (2f * (((_e4421.x * _e4324) + (_e4421.y * _e4327)) + (_e4421.z * _e4330)))) + _e4339) / (_e4435 + _e4333));
                                                            if (_e4454 != _e4454) {
                                                                phi_28799_ = true;
                                                            } else {
                                                                phi_28799_ = (0f >= _e4454);
                                                            }
                                                            let _e4458 = phi_28799_;
                                                            let _e4462 = sqrt((select(_e4454, 0f, _e4458) / (_e4336 - _e4333)));
                                                            if (_e4462 != _e4462) {
                                                                phi_28814_ = true;
                                                            } else {
                                                                phi_28814_ = (1f <= _e4462);
                                                            }
                                                            let _e4466 = phi_28814_;
                                                            let _e4468 = (select(_e4462, 1f, _e4466) * 63f);
                                                            let _e4473 = ((((_e4425 * _e4436) * _e4345) + ((_e4427 * _e4436) * _e4348)) + ((_e4429 * _e4436) * _e4351));
                                                            let _e4475 = select(_e4473, -1f, (_e4473 < -1f));
                                                            let _e4479 = ((select(_e4475, 1f, (_e4475 > 1f)) + 1f) * 127.5f);
                                                            let _e4484 = select(select(u32(_e4468), 0u, (_e4468 < 0f)), 4294967295u, (_e4468 > 4294967000f));
                                                            let _e4486 = select(_e4484, 62u, (62u < _e4484));
                                                            let _e4491 = select(select(u32(_e4479), 0u, (_e4479 < 0f)), 4294967295u, (_e4479 > 4294967000f));
                                                            let _e4493 = select(_e4491, 254u, (254u < _e4491));
                                                            let _e4495 = (_e4468 - f32(_e4486));
                                                            let _e4497 = (_e4479 - f32(_e4493));
                                                            let _e4501 = (_e4376 + (3u * ((_e4486 * 256u) + _e4493)));
                                                            let _e4506 = (_e4376 + (3u * (((_e4486 + 1u) * 256u) + _e4493)));
                                                            let _e4508 = global_6.member[_e4501];
                                                            let _e4511 = global_6.member[(_e4501 + 1u)];
                                                            let _e4514 = global_6.member[(_e4501 + 2u)];
                                                            let _e4515 = (1f - _e4497);
                                                            let _e4521 = global_6.member[(_e4501 + 3u)];
                                                            let _e4524 = global_6.member[(_e4501 + 4u)];
                                                            let _e4527 = global_6.member[(_e4501 + 5u)];
                                                            let _e4534 = (1f - _e4495);
                                                            let _e4539 = global_6.member[_e4506];
                                                            let _e4542 = global_6.member[(_e4506 + 1u)];
                                                            let _e4545 = global_6.member[(_e4506 + 2u)];
                                                            let _e4551 = global_6.member[(_e4506 + 3u)];
                                                            let _e4554 = global_6.member[(_e4506 + 4u)];
                                                            let _e4557 = global_6.member[(_e4506 + 5u)];
                                                            phi_15940_ = vec3<f32>(((((_e4508 * _e4515) + (_e4521 * _e4497)) * _e4534) + (((_e4539 * _e4515) + (_e4551 * _e4497)) * _e4495)), ((((_e4511 * _e4515) + (_e4524 * _e4497)) * _e4534) + (((_e4542 * _e4515) + (_e4554 * _e4497)) * _e4495)), ((((_e4514 * _e4515) + (_e4527 * _e4497)) * _e4534) + (((_e4545 * _e4515) + (_e4557 * _e4497)) * _e4495)));
                                                        } else {
                                                            phi_15940_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e4572 = phi_15940_;
                                                        let _e4579 = ((_e4363 * 1.1f) * _e4572.y);
                                                        phi_15942_ = vec3<f32>(exp(-((((_e4354 * _e4572.x) + _e4579) + (_e4366 * _e4572.z)))), exp(-((((_e4357 * _e4572.x) + _e4579) + (_e4369 * _e4572.z)))), exp(-((((_e4360 * _e4572.x) + _e4579) + (_e4372 * _e4572.z)))));
                                                    }
                                                    let _e4598 = phi_15942_;
                                                    phi_35787_ = _e2633;
                                                    phi_16826_ = 0f;
                                                    phi_16827_ = vec3<f32>(_e3380.x, _e3380.y, _e3380.z);
                                                    phi_16828_ = (vec3<f32>(_e4315.x, _e4315.y, _e4315.z) * _e4598);
                                                } else {
                                                    let _e3893 = ((_e2633 * 747796405u) + 2891336453u);
                                                    let _e3901 = (((_e3893 >> bitcast<u32>((((_e3893 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3893) * 277803737u);
                                                    let _e3904 = ((_e3901 >> bitcast<u32>(22i)) ^ _e3901);
                                                    let _e3910 = (((f32((_e3904 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e3392) * 0.15915494f);
                                                    let _e3911 = (1f - _e3910);
                                                    let _e3913 = (_e3910 * (2f - _e3910));
                                                    if (_e3913 != _e3913) {
                                                        phi_28397_ = true;
                                                    } else {
                                                        phi_28397_ = (0f >= _e3913);
                                                    }
                                                    let _e3917 = phi_28397_;
                                                    let _e3919 = sqrt(select(_e3913, 0f, _e3917));
                                                    let _e3921 = ((_e3904 * 747796405u) + 2891336453u);
                                                    let _e3929 = (((_e3921 >> bitcast<u32>((((_e3921 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3921) * 277803737u);
                                                    let _e3932 = ((_e3929 >> bitcast<u32>(22i)) ^ _e3929);
                                                    let _e3936 = (f32((_e3932 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                    let _e3938 = select(-1f, 1f, (_e3380.z >= 0f));
                                                    let _e3940 = (-1f / (_e3938 + _e3380.z));
                                                    let _e3942 = ((_e3380.x * _e3380.y) * _e3940);
                                                    let _e3955 = (_e3919 * cos(_e3936));
                                                    let _e3960 = (_e3919 * sin(_e3936));
                                                    let _e3970 = ((((1f + (((_e3938 * _e3380.x) * _e3380.x) * _e3940)) * _e3955) + (_e3942 * _e3960)) + (_e3380.x * _e3911));
                                                    let _e3971 = ((((_e3938 * _e3942) * _e3955) + ((_e3938 + ((_e3380.y * _e3380.y) * _e3940)) * _e3960)) + (_e3380.y * _e3911));
                                                    let _e3972 = ((((-(_e3938) * _e3380.x) * _e3955) + (-(_e3380.y) * _e3960)) + (_e3380.z * _e3911));
                                                    let _e3974 = (_e3970 - _e3380.x);
                                                    let _e3975 = (_e3971 - _e3380.y);
                                                    let _e3976 = (_e3972 - _e3380.z);
                                                    let _e3983 = (sqrt((((_e3974 * _e3974) + (_e3975 * _e3975)) + (_e3976 * _e3976))) * 0.5f);
                                                    let _e3984 = (_e3983 != _e3983);
                                                    if _e3984 {
                                                        phi_28468_ = true;
                                                    } else {
                                                        phi_28468_ = (1f <= _e3983);
                                                    }
                                                    let _e3987 = phi_28468_;
                                                    if ((2f * asin(select(_e3983, 1f, _e3987))) >= _e3388) {
                                                        phi_16023_ = 0f;
                                                    } else {
                                                        phi_16023_ = (1f / _e3392);
                                                    }
                                                    let _e3994 = phi_16023_;
                                                    if (_e3994 > 0f) {
                                                        if _e3984 {
                                                            phi_28483_ = true;
                                                        } else {
                                                            phi_28483_ = (1f <= _e3983);
                                                        }
                                                        let _e3998 = phi_28483_;
                                                        let _e4001 = (2f * asin(select(_e3983, 1f, _e3998)));
                                                        if (_e4001 >= _e3388) {
                                                            phi_16824_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e4005 = (1f - pow((_e4001 / _e3388), 2f));
                                                            if (_e4005 != _e4005) {
                                                                phi_28498_ = true;
                                                            } else {
                                                                phi_28498_ = (0f >= _e4005);
                                                            }
                                                            let _e4009 = phi_28498_;
                                                            let _e4012 = global_1.member.member_6;
                                                            let _e4018 = global_1.member.member_8[2u];
                                                            if (_e4018 == 4294967295u) {
                                                                phi_16797_ = vec3<f32>(1f, 1f, 1f);
                                                            } else {
                                                                let _e4021 = global_6.member[_e4018];
                                                                let _e4024 = global_6.member[(_e4018 + 1u)];
                                                                let _e4027 = global_6.member[(_e4018 + 2u)];
                                                                let _e4030 = global_6.member[(_e4018 + 3u)];
                                                                let _e4033 = global_6.member[(_e4018 + 4u)];
                                                                let _e4036 = global_6.member[(_e4018 + 5u)];
                                                                let _e4039 = global_6.member[(_e4018 + 6u)];
                                                                let _e4042 = global_6.member[(_e4018 + 7u)];
                                                                let _e4045 = global_6.member[(_e4018 + 8u)];
                                                                let _e4048 = global_6.member[(_e4018 + 9u)];
                                                                let _e4051 = global_6.member[(_e4018 + 13u)];
                                                                let _e4054 = global_6.member[(_e4018 + 14u)];
                                                                let _e4057 = global_6.member[(_e4018 + 15u)];
                                                                let _e4060 = global_6.member[(_e4018 + 16u)];
                                                                let _e4063 = global_6.member[(_e4018 + 18u)];
                                                                let _e4066 = global_6.member[(_e4018 + 19u)];
                                                                let _e4069 = global_6.member[(_e4018 + 20u)];
                                                                let _e4072 = global_6.member[(_e4018 + 29u)];
                                                                let _e4073 = bitcast<u32>(_e4072);
                                                                let _e4086 = (((((_e1751 * _e1751) + (_e1752 * _e1752)) + (_e1753 * _e1753)) - (2f * (((_e1751 * _e4021) + (_e1752 * _e4024)) + (_e1753 * _e4027)))) + _e4039);
                                                                if (_e4086 > 0f) {
                                                                    let _e4095 = ((((_e1751 - _e4021) * _e4042) + ((_e1752 - _e4024) * _e4045)) + ((_e1753 - _e4027) * _e4048));
                                                                    let _e4097 = ((_e4095 * _e4095) - _e4086);
                                                                    if (_e4095 >= 0f) {
                                                                        phi_16497_ = vec3<f32>();
                                                                        phi_16498_ = true;
                                                                    } else {
                                                                        let _e4099 = (_e4097 <= 0f);
                                                                        if _e4099 {
                                                                            phi_16496_ = vec3<f32>();
                                                                        } else {
                                                                            let _e4102 = (-(_e4095) - sqrt(_e4097));
                                                                            phi_16496_ = vec3<f32>((_e1751 + (_e4042 * _e4102)), (_e1752 + (_e4045 * _e4102)), (_e1753 + (_e4048 * _e4102)));
                                                                        }
                                                                        let _e4111 = phi_16496_;
                                                                        phi_16497_ = _e4111;
                                                                        phi_16498_ = _e4099;
                                                                    }
                                                                    let _e4113 = phi_16497_;
                                                                    let _e4115 = phi_16498_;
                                                                    phi_16503_ = _e4113;
                                                                    phi_16504_ = select(true, false, _e4115);
                                                                } else {
                                                                    phi_16503_ = _e1754;
                                                                    phi_16504_ = true;
                                                                }
                                                                let _e4118 = phi_16503_;
                                                                let _e4120 = phi_16504_;
                                                                if _e4120 {
                                                                    let _e4122 = (_e4118.x - _e4021);
                                                                    let _e4124 = (_e4118.y - _e4024);
                                                                    let _e4126 = (_e4118.z - _e4027);
                                                                    let _e4132 = sqrt((((_e4122 * _e4122) + (_e4124 * _e4124)) + (_e4126 * _e4126)));
                                                                    let _e4133 = (1f / _e4132);
                                                                    let _e4151 = ((((((_e4118.x * _e4118.x) + (_e4118.y * _e4118.y)) + (_e4118.z * _e4118.z)) - (2f * (((_e4118.x * _e4021) + (_e4118.y * _e4024)) + (_e4118.z * _e4027)))) + _e4036) / (_e4132 + _e4030));
                                                                    if (_e4151 != _e4151) {
                                                                        phi_28622_ = true;
                                                                    } else {
                                                                        phi_28622_ = (0f >= _e4151);
                                                                    }
                                                                    let _e4155 = phi_28622_;
                                                                    let _e4159 = sqrt((select(_e4151, 0f, _e4155) / (_e4033 - _e4030)));
                                                                    if (_e4159 != _e4159) {
                                                                        phi_28637_ = true;
                                                                    } else {
                                                                        phi_28637_ = (1f <= _e4159);
                                                                    }
                                                                    let _e4163 = phi_28637_;
                                                                    let _e4165 = (select(_e4159, 1f, _e4163) * 63f);
                                                                    let _e4170 = ((((_e4122 * _e4133) * _e4042) + ((_e4124 * _e4133) * _e4045)) + ((_e4126 * _e4133) * _e4048));
                                                                    let _e4172 = select(_e4170, -1f, (_e4170 < -1f));
                                                                    let _e4176 = ((select(_e4172, 1f, (_e4172 > 1f)) + 1f) * 127.5f);
                                                                    let _e4181 = select(select(u32(_e4165), 0u, (_e4165 < 0f)), 4294967295u, (_e4165 > 4294967000f));
                                                                    let _e4183 = select(_e4181, 62u, (62u < _e4181));
                                                                    let _e4188 = select(select(u32(_e4176), 0u, (_e4176 < 0f)), 4294967295u, (_e4176 > 4294967000f));
                                                                    let _e4190 = select(_e4188, 254u, (254u < _e4188));
                                                                    let _e4192 = (_e4165 - f32(_e4183));
                                                                    let _e4194 = (_e4176 - f32(_e4190));
                                                                    let _e4198 = (_e4073 + (3u * ((_e4183 * 256u) + _e4190)));
                                                                    let _e4203 = (_e4073 + (3u * (((_e4183 + 1u) * 256u) + _e4190)));
                                                                    let _e4205 = global_6.member[_e4198];
                                                                    let _e4208 = global_6.member[(_e4198 + 1u)];
                                                                    let _e4211 = global_6.member[(_e4198 + 2u)];
                                                                    let _e4212 = (1f - _e4194);
                                                                    let _e4218 = global_6.member[(_e4198 + 3u)];
                                                                    let _e4221 = global_6.member[(_e4198 + 4u)];
                                                                    let _e4224 = global_6.member[(_e4198 + 5u)];
                                                                    let _e4231 = (1f - _e4192);
                                                                    let _e4236 = global_6.member[_e4203];
                                                                    let _e4239 = global_6.member[(_e4203 + 1u)];
                                                                    let _e4242 = global_6.member[(_e4203 + 2u)];
                                                                    let _e4248 = global_6.member[(_e4203 + 3u)];
                                                                    let _e4251 = global_6.member[(_e4203 + 4u)];
                                                                    let _e4254 = global_6.member[(_e4203 + 5u)];
                                                                    phi_16795_ = vec3<f32>(((((_e4205 * _e4212) + (_e4218 * _e4194)) * _e4231) + (((_e4236 * _e4212) + (_e4248 * _e4194)) * _e4192)), ((((_e4208 * _e4212) + (_e4221 * _e4194)) * _e4231) + (((_e4239 * _e4212) + (_e4251 * _e4194)) * _e4192)), ((((_e4211 * _e4212) + (_e4224 * _e4194)) * _e4231) + (((_e4242 * _e4212) + (_e4254 * _e4194)) * _e4192)));
                                                                } else {
                                                                    phi_16795_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e4269 = phi_16795_;
                                                                let _e4276 = ((_e4060 * 1.1f) * _e4269.y);
                                                                phi_16797_ = vec3<f32>(exp(-((((_e4051 * _e4269.x) + _e4276) + (_e4063 * _e4269.z)))), exp(-((((_e4054 * _e4269.x) + _e4276) + (_e4066 * _e4269.z)))), exp(-((((_e4057 * _e4269.x) + _e4276) + (_e4069 * _e4269.z)))));
                                                            }
                                                            let _e4295 = phi_16797_;
                                                            let _e4306 = (((1f - (0.6f * (1f - sqrt(select(_e4005, 0f, _e4009))))) * 1.25f) / _e3392);
                                                            phi_16824_ = vec3<f32>(((_e4012.x * _e4295.x) * _e4306), ((_e4012.y * _e4295.y) * _e4306), ((_e4012.z * _e4295.z) * _e4306));
                                                        }
                                                        let _e4312 = phi_16824_;
                                                        phi_16825_ = _e4312;
                                                    } else {
                                                        phi_16825_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e4314 = phi_16825_;
                                                    phi_35787_ = _e3932;
                                                    phi_16826_ = _e3994;
                                                    phi_16827_ = vec3<f32>(_e3970, _e3971, _e3972);
                                                    phi_16828_ = _e4314;
                                                }
                                                let _e4602 = phi_35787_;
                                                let _e4604 = phi_16826_;
                                                let _e4606 = phi_16827_;
                                                let _e4608 = phi_16828_;
                                                phi_35786_ = _e4602;
                                                phi_16829_ = _e4604;
                                                phi_16830_ = _e4606;
                                                phi_16831_ = _e4608;
                                            }
                                            let _e4610 = phi_35786_;
                                            let _e4612 = phi_16829_;
                                            let _e4614 = phi_16830_;
                                            let _e4616 = phi_16831_;
                                            phi_35743_ = _e4610;
                                            phi_17721_ = _e4612;
                                            phi_17722_ = _e4614;
                                            phi_17723_ = _e4616;
                                            phi_17724_ = 1000000000000000000000000000000f;
                                        } else {
                                            if (_e2639 == (_e2629 + 1u)) {
                                                if ((_e378 & 1u) == 0u) {
                                                    phi_35752_ = _e2633;
                                                    phi_17355_ = 0f;
                                                    phi_17356_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17357_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17358_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e381 > 0u) {
                                                        let _e2870 = global_1.member.member_2[1u];
                                                        let _e2872 = ((_e2633 * 747796405u) + 2891336453u);
                                                        let _e2880 = (((_e2872 >> bitcast<u32>((((_e2872 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2872) * 277803737u);
                                                        let _e2883 = ((_e2880 >> bitcast<u32>(22i)) ^ _e2880);
                                                        phi_16852_ = 0u;
                                                        phi_16855_ = _e381;
                                                        loop {
                                                            let _e2889 = phi_16852_;
                                                            let _e2891 = phi_16855_;
                                                            local_85 = _e2889;
                                                            local_86 = _e2889;
                                                            let _e2893 = ((_e2889 + 1u) < _e2891);
                                                            if _e2893 {
                                                                let _e2895 = ((_e2889 + _e2891) / 2u);
                                                                let _e2898 = global_6.member[(_e2870 + _e2895)];
                                                                let _e2899 = (_e2898 <= (f32((_e2883 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                phi_16853_ = select(_e2889, _e2895, _e2899);
                                                                phi_16856_ = select(_e2895, _e2891, _e2899);
                                                            } else {
                                                                phi_16853_ = u32();
                                                                phi_16856_ = u32();
                                                            }
                                                            let _e2903 = phi_16853_;
                                                            let _e2905 = phi_16856_;
                                                            phi_16852_ = _e2903;
                                                            phi_16855_ = _e2905;
                                                            if !(_e2893) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2908 = local_85;
                                                        let _e2909 = (_e2870 + _e2908);
                                                        let _e2912 = global_6.member[(_e2909 + 1u)];
                                                        let _e2914 = global_6.member[_e2909];
                                                        let _e2918 = global_1.member.member_2[0u];
                                                        let _e2920 = local_86;
                                                        let _e2923 = global_6.member[(_e2918 + _e2920)];
                                                        let _e2926 = ((_e2883 * 747796405u) + 2891336453u);
                                                        let _e2934 = (((_e2926 >> bitcast<u32>((((_e2926 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2926) * 277803737u);
                                                        let _e2937 = ((_e2934 >> bitcast<u32>(22i)) ^ _e2934);
                                                        let _e2940 = f32((_e2937 >> bitcast<u32>(8i)));
                                                        let _e2941 = (_e2940 * 0.000000059604645f);
                                                        let _e2943 = ((_e2937 * 747796405u) + 2891336453u);
                                                        let _e2951 = (((_e2943 >> bitcast<u32>((((_e2943 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2943) * 277803737u);
                                                        let _e2954 = ((_e2951 >> bitcast<u32>(22i)) ^ _e2951);
                                                        let _e2957 = f32((_e2954 >> bitcast<u32>(8i)));
                                                        let _e2960 = global_2.member[bitcast<u32>(_e2923)];
                                                        let _e2963 = (_e2960.member_6.x == 1u);
                                                        if _e2963 {
                                                            let _e3105 = (_e2960.member.x - _e1751);
                                                            let _e3106 = (_e2960.member.y - _e1752);
                                                            let _e3107 = (_e2960.member.z - _e1753);
                                                            let _e3112 = (((_e3105 * _e3105) + (_e3106 * _e3106)) + (_e3107 * _e3107));
                                                            let _e3113 = (_e2960.member.w * _e2960.member.w);
                                                            let _e3115 = (_e3112 <= (_e3113 * 1.000001f));
                                                            if _e3115 {
                                                                let _e3224 = (_e2940 * 0.00000037450704f);
                                                                let _e3226 = ((_e2957 * 0.00000011920929f) - 1f);
                                                                let _e3228 = (1f - (_e3226 * _e3226));
                                                                if (_e3228 != _e3228) {
                                                                    phi_27465_ = true;
                                                                } else {
                                                                    phi_27465_ = (0f >= _e3228);
                                                                }
                                                                let _e3232 = phi_27465_;
                                                                let _e3234 = sqrt(select(_e3228, 0f, _e3232));
                                                                let _e3236 = (_e3234 * cos(_e3224));
                                                                let _e3238 = (_e3234 * sin(_e3224));
                                                                let _e3243 = (_e2960.member.x + (_e3236 * _e2960.member.w));
                                                                let _e3244 = (_e2960.member.y + (_e3238 * _e2960.member.w));
                                                                let _e3245 = (_e2960.member.z + (_e3226 * _e2960.member.w));
                                                                if _e3115 {
                                                                    let _e3258 = (_e3243 - _e1751);
                                                                    let _e3259 = (_e3244 - _e1752);
                                                                    let _e3260 = (_e3245 - _e1753);
                                                                    let _e3265 = (((_e3258 * _e3258) + (_e3259 * _e3259)) + (_e3260 * _e3260));
                                                                    phi_27534_ = (_e3265 / (((abs((((_e3236 * _e3258) + (_e3238 * _e3259)) + (_e3226 * _e3260))) / sqrt(_e3265)) * 12.566371f) * _e3113));
                                                                } else {
                                                                    let _e3247 = (_e3113 / _e3112);
                                                                    if (_e3247 != _e3247) {
                                                                        phi_27553_ = true;
                                                                    } else {
                                                                        phi_27553_ = (1f <= _e3247);
                                                                    }
                                                                    let _e3251 = phi_27553_;
                                                                    let _e3252 = select(_e3247, 1f, _e3251);
                                                                    phi_27534_ = (0.15915494f / (_e3252 / (1f + sqrt((1f - _e3252)))));
                                                                }
                                                                let _e3278 = phi_27534_;
                                                                phi_17068_ = type_76(vec3<f32>(_e3243, _e3244, _e3245), vec3<f32>(_e3236, _e3238, _e3226), _e3278);
                                                            } else {
                                                                let _e3116 = (_e3113 / _e3112);
                                                                let _e3117 = (_e3116 != _e3116);
                                                                if _e3117 {
                                                                    phi_27317_ = true;
                                                                } else {
                                                                    phi_27317_ = (1f <= _e3116);
                                                                }
                                                                let _e3120 = phi_27317_;
                                                                let _e3121 = select(_e3116, 1f, _e3120);
                                                                if _e3117 {
                                                                    phi_27340_ = true;
                                                                } else {
                                                                    phi_27340_ = (1f <= _e3116);
                                                                }
                                                                let _e3125 = phi_27340_;
                                                                let _e3126 = select(_e3116, 1f, _e3125);
                                                                let _e3130 = (_e3126 / (1f + sqrt((1f - _e3126))));
                                                                if (_e3121 < 0.00068523f) {
                                                                    phi_17003_ = (_e3121 * _e2941);
                                                                } else {
                                                                    let _e3133 = (1f - (_e2941 * _e3130));
                                                                    let _e3135 = (1f - (_e3133 * _e3133));
                                                                    if (_e3135 != _e3135) {
                                                                        phi_27355_ = true;
                                                                    } else {
                                                                        phi_27355_ = (0f >= _e3135);
                                                                    }
                                                                    let _e3139 = phi_27355_;
                                                                    phi_17003_ = select(_e3135, 0f, _e3139);
                                                                }
                                                                let _e3143 = phi_17003_;
                                                                let _e3144 = (1f - _e3143);
                                                                if (_e3144 != _e3144) {
                                                                    phi_27370_ = true;
                                                                } else {
                                                                    phi_27370_ = (0f >= _e3144);
                                                                }
                                                                let _e3148 = phi_27370_;
                                                                let _e3153 = (1f - (_e3143 / _e3121));
                                                                if (_e3153 != _e3153) {
                                                                    phi_27385_ = true;
                                                                } else {
                                                                    phi_27385_ = (0f >= _e3153);
                                                                }
                                                                let _e3157 = phi_27385_;
                                                                let _e3161 = ((_e3143 / sqrt(_e3121)) + (sqrt(select(_e3144, 0f, _e3148)) * sqrt(select(_e3153, 0f, _e3157))));
                                                                let _e3163 = (1f - (_e3161 * _e3161));
                                                                if (_e3163 != _e3163) {
                                                                    phi_27400_ = true;
                                                                } else {
                                                                    phi_27400_ = (0f >= _e3163);
                                                                }
                                                                let _e3167 = phi_27400_;
                                                                let _e3169 = sqrt(select(_e3163, 0f, _e3167));
                                                                let _e3170 = (_e2957 * 0.00000037450704f);
                                                                let _e3171 = sqrt(_e3112);
                                                                let _e3172 = (_e3105 / _e3171);
                                                                let _e3173 = (_e3106 / _e3171);
                                                                let _e3174 = (_e3107 / _e3171);
                                                                let _e3176 = select(-1f, 1f, (_e3174 >= 0f));
                                                                let _e3178 = (-1f / (_e3176 + _e3174));
                                                                let _e3180 = ((_e3172 * _e3173) * _e3178);
                                                                let _e3193 = (_e3169 * cos(_e3170));
                                                                let _e3198 = (_e3169 * sin(_e3170));
                                                                let _e3211 = -(((((1f + (((_e3176 * _e3172) * _e3172) * _e3178)) * _e3193) + (_e3180 * _e3198)) + (_e3172 * _e3161)));
                                                                let _e3212 = -(((((_e3176 * _e3180) * _e3193) + ((_e3176 + ((_e3173 * _e3173) * _e3178)) * _e3198)) + (_e3173 * _e3161)));
                                                                let _e3213 = -(((((-(_e3176) * _e3172) * _e3193) + (-(_e3173) * _e3198)) + (_e3174 * _e3161)));
                                                                phi_17068_ = type_76(vec3<f32>((_e2960.member.x + (_e3211 * _e2960.member.w)), (_e2960.member.y + (_e3212 * _e2960.member.w)), (_e2960.member.z + (_e3213 * _e2960.member.w))), vec3<f32>(_e3211, _e3212, _e3213), (0.15915494f / _e3130));
                                                            }
                                                            let _e3281 = phi_17068_;
                                                            phi_17254_ = _e3281;
                                                        } else {
                                                            let _e2964 = sqrt(_e2941);
                                                            let _e2965 = (1f - _e2964);
                                                            let _e2966 = ((_e2957 * 0.000000059604645f) * _e2964);
                                                            let _e2988 = (_e2964 - _e2966);
                                                            let _e2992 = (((_e2960.member.x * _e2965) + (_e2960.member_1.x * _e2966)) + (_e2960.member_2.x * _e2988));
                                                            let _e2993 = (((_e2960.member.y * _e2965) + (_e2960.member_1.y * _e2966)) + (_e2960.member_2.y * _e2988));
                                                            let _e2994 = (((_e2960.member.z * _e2965) + (_e2960.member_1.z * _e2966)) + (_e2960.member_2.z * _e2988));
                                                            let _e2996 = (_e2960.member_1.x - _e2960.member.x);
                                                            let _e2997 = (_e2960.member_1.y - _e2960.member.y);
                                                            let _e2998 = (_e2960.member_1.z - _e2960.member.z);
                                                            let _e2999 = (_e2960.member_2.x - _e2960.member.x);
                                                            let _e3000 = (_e2960.member_2.y - _e2960.member.y);
                                                            let _e3001 = (_e2960.member_2.z - _e2960.member.z);
                                                            let _e3004 = ((_e2997 * _e3001) - (_e3000 * _e2998));
                                                            let _e3007 = ((_e2998 * _e2999) - (_e3001 * _e2996));
                                                            let _e3010 = ((_e2996 * _e3000) - (_e2999 * _e2997));
                                                            let _e3016 = sqrt((((_e3004 * _e3004) + (_e3007 * _e3007)) + (_e3010 * _e3010)));
                                                            let _e3017 = (1f / _e3016);
                                                            let _e3018 = (_e3004 * _e3017);
                                                            let _e3019 = (_e3007 * _e3017);
                                                            let _e3020 = (_e3010 * _e3017);
                                                            if _e2963 {
                                                                let _e3053 = (_e2960.member.x - _e1751);
                                                                let _e3054 = (_e2960.member.y - _e1752);
                                                                let _e3055 = (_e2960.member.z - _e1753);
                                                                let _e3060 = (((_e3053 * _e3053) + (_e3054 * _e3054)) + (_e3055 * _e3055));
                                                                let _e3061 = (_e2960.member.w * _e2960.member.w);
                                                                if (_e3060 <= (_e3061 * 1.000001f)) {
                                                                    let _e3075 = (_e2992 - _e1751);
                                                                    let _e3076 = (_e2993 - _e1752);
                                                                    let _e3077 = (_e2994 - _e1753);
                                                                    let _e3082 = (((_e3075 * _e3075) + (_e3076 * _e3076)) + (_e3077 * _e3077));
                                                                    phi_27283_ = (_e3082 / (((abs((((_e3018 * _e3075) + (_e3019 * _e3076)) + (_e3020 * _e3077))) / sqrt(_e3082)) * 12.566371f) * _e3061));
                                                                } else {
                                                                    let _e3064 = (_e3061 / _e3060);
                                                                    if (_e3064 != _e3064) {
                                                                        phi_27302_ = true;
                                                                    } else {
                                                                        phi_27302_ = (1f <= _e3064);
                                                                    }
                                                                    let _e3068 = phi_27302_;
                                                                    let _e3069 = select(_e3064, 1f, _e3068);
                                                                    phi_27283_ = (0.15915494f / (_e3069 / (1f + sqrt((1f - _e3069)))));
                                                                }
                                                                let _e3095 = phi_27283_;
                                                                phi_17252_ = _e3095;
                                                            } else {
                                                                let _e3022 = (_e2992 - _e1751);
                                                                let _e3023 = (_e2993 - _e1752);
                                                                let _e3024 = (_e2994 - _e1753);
                                                                let _e3029 = (((_e3022 * _e3022) + (_e3023 * _e3023)) + (_e3024 * _e3024));
                                                                let _e3037 = (abs((((_e3018 * _e3022) + (_e3019 * _e3023)) + (_e3020 * _e3024))) / sqrt(_e3029));
                                                                if (_e3037 <= 0f) {
                                                                    phi_17251_ = 0f;
                                                                } else {
                                                                    if (_e2960.member_6.x == 0u) {
                                                                        phi_17248_ = (_e3016 * 0.5f);
                                                                    } else {
                                                                        phi_17248_ = ((12.566371f * _e2960.member.w) * _e2960.member.w);
                                                                    }
                                                                    let _e3046 = phi_17248_;
                                                                    phi_17251_ = (_e3029 / (_e3037 * _e3046));
                                                                }
                                                                let _e3050 = phi_17251_;
                                                                phi_17252_ = _e3050;
                                                            }
                                                            let _e3097 = phi_17252_;
                                                            phi_17254_ = type_76(vec3<f32>(_e2992, _e2993, _e2994), vec3<f32>(_e3018, _e3019, _e3020), _e3097);
                                                        }
                                                        let _e3283 = phi_17254_;
                                                        let _e3288 = (_e3283.member.x - _e1751);
                                                        let _e3290 = (_e3283.member.y - _e1752);
                                                        let _e3292 = (_e3283.member.z - _e1753);
                                                        let _e3298 = sqrt((((_e3288 * _e3288) + (_e3290 * _e3290)) + (_e3292 * _e3292)));
                                                        let _e3299 = (_e3288 / _e3298);
                                                        let _e3300 = (_e3290 / _e3298);
                                                        let _e3301 = (_e3292 / _e3298);
                                                        if (_e3283.member_2 > 0f) {
                                                            let _e3313 = global_4.member[_e2960.member_6.y];
                                                            if (_e3313.member == 4u) {
                                                                if (_e3313.member_1 == 1u) {
                                                                    phi_17332_ = true;
                                                                } else {
                                                                    phi_17332_ = ((((-(_e3299) * _e3283.member_1.x) + (-(_e3300) * _e3283.member_1.y)) + (-(_e3301) * _e3283.member_1.z)) > 0f);
                                                                }
                                                                let _e3328 = phi_17332_;
                                                                if _e3328 {
                                                                    phi_17341_ = vec3<f32>(_e3313.member_4.x, _e3313.member_4.y, _e3313.member_4.z);
                                                                } else {
                                                                    phi_17341_ = vec3<f32>();
                                                                }
                                                                let _e3335 = phi_17341_;
                                                                phi_17343_ = _e3335;
                                                                phi_17344_ = select(true, false, _e3328);
                                                            } else {
                                                                phi_17343_ = vec3<f32>();
                                                                phi_17344_ = true;
                                                            }
                                                            let _e3338 = phi_17343_;
                                                            let _e3340 = phi_17344_;
                                                            phi_17349_ = ((_e2912 - _e2914) * _e3283.member_2);
                                                            phi_17350_ = select(_e3338, vec3<f32>(0f, 0f, 0f), vec3(_e3340));
                                                        } else {
                                                            phi_17349_ = 0f;
                                                            phi_17350_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3344 = phi_17349_;
                                                        let _e3346 = phi_17350_;
                                                        phi_35753_ = _e2954;
                                                        phi_17351_ = _e3344;
                                                        phi_17352_ = vec3<f32>(_e3299, _e3300, _e3301);
                                                        phi_17353_ = _e3346;
                                                        phi_17354_ = ((_e3298 * 0.9999f) - _e1744);
                                                    } else {
                                                        phi_35753_ = _e2633;
                                                        phi_17351_ = 0f;
                                                        phi_17352_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17353_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17354_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e3348 = phi_35753_;
                                                    let _e3350 = phi_17351_;
                                                    let _e3352 = phi_17352_;
                                                    let _e3354 = phi_17353_;
                                                    let _e3356 = phi_17354_;
                                                    phi_35752_ = _e3348;
                                                    phi_17355_ = _e3350;
                                                    phi_17356_ = _e3352;
                                                    phi_17357_ = _e3354;
                                                    phi_17358_ = _e3356;
                                                }
                                                let _e3358 = phi_35752_;
                                                let _e3360 = phi_17355_;
                                                let _e3362 = phi_17356_;
                                                let _e3364 = phi_17357_;
                                                let _e3366 = phi_17358_;
                                                phi_35744_ = _e3358;
                                                phi_17717_ = _e3360;
                                                phi_17718_ = _e3362;
                                                phi_17719_ = _e3364;
                                                phi_17720_ = _e3366;
                                            } else {
                                                if ((_e378 & 2u) == 0u) {
                                                    phi_35745_ = _e2633;
                                                    phi_17714_ = 0f;
                                                    phi_17715_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17716_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e2651 = ((_e2633 * 747796405u) + 2891336453u);
                                                    let _e2659 = (((_e2651 >> bitcast<u32>((((_e2651 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2651) * 277803737u);
                                                    let _e2662 = ((_e2659 >> bitcast<u32>(22i)) ^ _e2659);
                                                    let _e2666 = (f32((_e2662 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                    let _e2668 = ((_e2662 * 747796405u) + 2891336453u);
                                                    let _e2676 = (((_e2668 >> bitcast<u32>((((_e2668 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2668) * 277803737u);
                                                    let _e2679 = ((_e2676 >> bitcast<u32>(22i)) ^ _e2676);
                                                    let _e2683 = (f32((_e2679 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                    let _e2686 = global_1.member.member_3[1u];
                                                    let _e2689 = global_1.member.member_3[2u];
                                                    let _e2692 = global_1.member.member_2[3u];
                                                    phi_17379_ = 0u;
                                                    phi_17382_ = _e2689;
                                                    loop {
                                                        let _e2694 = phi_17379_;
                                                        let _e2696 = phi_17382_;
                                                        local_80 = _e2694;
                                                        local_83 = _e2694;
                                                        local_84 = _e2694;
                                                        let _e2698 = ((_e2694 + 1u) < _e2696);
                                                        if _e2698 {
                                                            let _e2700 = ((_e2694 + _e2696) / 2u);
                                                            let _e2703 = global_6.member[(_e2692 + _e2700)];
                                                            let _e2704 = (_e2703 <= _e2666);
                                                            phi_17380_ = select(_e2694, _e2700, _e2704);
                                                            phi_17383_ = select(_e2700, _e2696, _e2704);
                                                        } else {
                                                            phi_17380_ = u32();
                                                            phi_17383_ = u32();
                                                        }
                                                        let _e2708 = phi_17380_;
                                                        let _e2710 = phi_17383_;
                                                        phi_17379_ = _e2708;
                                                        phi_17382_ = _e2710;
                                                        if !(_e2698) {
                                                            break;
                                                        }
                                                    }
                                                    let _e2714 = global_1.member.member_3[0u];
                                                    let _e2715 = (_e2686 + 1u);
                                                    let _e2717 = local_80;
                                                    let _e2719 = (_e2714 + (_e2717 * _e2715));
                                                    phi_17420_ = 0u;
                                                    phi_17423_ = _e2686;
                                                    loop {
                                                        let _e2721 = phi_17420_;
                                                        let _e2723 = phi_17423_;
                                                        local_81 = _e2721;
                                                        local_82 = _e2721;
                                                        let _e2725 = ((_e2721 + 1u) < _e2723);
                                                        if _e2725 {
                                                            let _e2727 = ((_e2721 + _e2723) / 2u);
                                                            let _e2730 = global_6.member[(_e2719 + _e2727)];
                                                            let _e2731 = (_e2730 <= _e2683);
                                                            phi_17421_ = select(_e2721, _e2727, _e2731);
                                                            phi_17424_ = select(_e2727, _e2723, _e2731);
                                                        } else {
                                                            phi_17421_ = u32();
                                                            phi_17424_ = u32();
                                                        }
                                                        let _e2735 = phi_17421_;
                                                        let _e2737 = phi_17424_;
                                                        phi_17420_ = _e2735;
                                                        phi_17423_ = _e2737;
                                                        if !(_e2725) {
                                                            break;
                                                        }
                                                    }
                                                    let _e2740 = local_81;
                                                    let _e2743 = local_82;
                                                    let _e2744 = (_e2719 + _e2743);
                                                    let _e2746 = global_6.member[_e2744];
                                                    let _e2749 = global_6.member[(_e2744 + 1u)];
                                                    if (_e2749 > _e2746) {
                                                        let _e2753 = ((_e2683 - _e2746) / (_e2749 - _e2746));
                                                        let _e2755 = select(_e2753, 0f, (_e2753 < 0f));
                                                        phi_17500_ = select(_e2755, 0.99999f, (_e2755 > 0.99999f));
                                                    } else {
                                                        phi_17500_ = 0.5f;
                                                    }
                                                    let _e2759 = phi_17500_;
                                                    let _e2761 = f32(_e2686);
                                                    let _e2764 = local_83;
                                                    let _e2767 = local_84;
                                                    let _e2768 = (_e2692 + _e2767);
                                                    let _e2770 = global_6.member[_e2768];
                                                    let _e2773 = global_6.member[(_e2768 + 1u)];
                                                    if (_e2773 > _e2770) {
                                                        let _e2777 = ((_e2666 - _e2770) / (_e2773 - _e2770));
                                                        let _e2779 = select(_e2777, 0f, (_e2777 < 0f));
                                                        phi_17549_ = select(_e2779, 0.99999f, (_e2779 > 0.99999f));
                                                    } else {
                                                        phi_17549_ = 0.5f;
                                                    }
                                                    let _e2783 = phi_17549_;
                                                    let _e2785 = f32(_e2689);
                                                    let _e2788 = ((((f32(_e2740) + _e2759) / _e2761) - 0.5f) * 6.2831855f);
                                                    let _e2789 = (((f32(_e2764) + _e2783) / _e2785) * 3.1415927f);
                                                    let _e2790 = sin(_e2789);
                                                    let _e2792 = (_e2790 * cos(_e2788));
                                                    let _e2793 = cos(_e2789);
                                                    let _e2795 = (_e2790 * sin(_e2788));
                                                    let _e2798 = ((atan2(_e2795, _e2792) * 0.15915494f) + 0.5f);
                                                    let _e2802 = select(_e2793, -1f, (_e2793 < -1f));
                                                    let _e2805 = acos(select(_e2802, 1f, (_e2802 > 1f)));
                                                    let _e2807 = ((_e2798 - floor(_e2798)) * _e2761);
                                                    let _e2812 = select(select(u32(_e2807), 0u, (_e2807 < 0f)), 4294967295u, (_e2807 > 4294967000f));
                                                    let _e2813 = (_e2686 - 1u);
                                                    let _e2816 = ((_e2805 * 0.31830987f) * _e2785);
                                                    let _e2821 = select(select(u32(_e2816), 0u, (_e2816 < 0f)), 4294967295u, (_e2816 > 4294967000f));
                                                    let _e2822 = (_e2689 - 1u);
                                                    let _e2824 = select(_e2821, _e2822, (_e2822 < _e2821));
                                                    let _e2825 = sin(_e2805);
                                                    if (_e2825 <= 0f) {
                                                        phi_17705_ = 0f;
                                                    } else {
                                                        let _e2829 = (_e2692 + _e2824);
                                                        let _e2832 = global_6.member[(_e2829 + 1u)];
                                                        let _e2834 = global_6.member[_e2829];
                                                        let _e2837 = ((_e2714 + (_e2824 * _e2715)) + select(_e2812, _e2813, (_e2813 < _e2812)));
                                                        let _e2840 = global_6.member[(_e2837 + 1u)];
                                                        let _e2842 = global_6.member[_e2837];
                                                        phi_17705_ = ((((_e2832 - _e2834) * _e2785) * ((_e2840 - _e2842) * _e2761)) / (19.73921f * _e2825));
                                                    }
                                                    let _e2849 = phi_17705_;
                                                    let _e2850 = (_e2849 > 0f);
                                                    if _e2850 {
                                                        phi_17712_ = vec3<f32>(_e2792, _e2793, _e2795);
                                                    } else {
                                                        phi_17712_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e2853 = phi_17712_;
                                                    phi_35745_ = _e2679;
                                                    phi_17714_ = select(0f, _e2849, _e2850);
                                                    phi_17715_ = _e2853;
                                                    phi_17716_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2850));
                                                }
                                                let _e2858 = phi_35745_;
                                                let _e2860 = phi_17714_;
                                                let _e2862 = phi_17715_;
                                                let _e2864 = phi_17716_;
                                                phi_35744_ = _e2858;
                                                phi_17717_ = _e2860;
                                                phi_17718_ = _e2862;
                                                phi_17719_ = _e2864;
                                                phi_17720_ = 1000000000000000000000000000000f;
                                            }
                                            let _e3368 = phi_35744_;
                                            let _e3370 = phi_17717_;
                                            let _e3372 = phi_17718_;
                                            let _e3374 = phi_17719_;
                                            let _e3376 = phi_17720_;
                                            phi_35743_ = _e3368;
                                            phi_17721_ = _e3370;
                                            phi_17722_ = _e3372;
                                            phi_17723_ = _e3374;
                                            phi_17724_ = _e3376;
                                        }
                                        let _e4618 = phi_35743_;
                                        let _e4620 = phi_17721_;
                                        let _e4622 = phi_17722_;
                                        let _e4624 = phi_17723_;
                                        let _e4626 = phi_17724_;
                                        phi_35742_ = _e4618;
                                        phi_17725_ = _e4620;
                                        phi_17726_ = _e4622;
                                        phi_17727_ = _e4624;
                                        phi_17728_ = _e4626;
                                    }
                                    let _e4695 = phi_35742_;
                                    let _e4697 = phi_17725_;
                                    let _e4699 = phi_17726_;
                                    let _e4701 = phi_17727_;
                                    let _e4703 = phi_17728_;
                                    if ((((_e4701.x * _e4701.x) + (_e4701.y * _e4701.y)) + (_e4701.z * _e4701.z)) > 0f) {
                                        let _e4720 = (((_e4699.x * _e1726.x) + (_e4699.y * _e1726.y)) + (_e4699.z * _e1726.z));
                                        if (_e4720 != _e4720) {
                                            phi_28867_ = true;
                                        } else {
                                            phi_28867_ = (0f >= _e4720);
                                        }
                                        let _e4724 = phi_28867_;
                                        let _e4726 = (select(_e4720, 0f, _e4724) * 0.31830987f);
                                        if (_e2618 == 0u) {
                                            if (_e4720 <= 0f) {
                                                phi_29664_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_29664_ = (_e2612 * (_e4720 * 0.31830987f));
                                            }
                                            let _e5202 = phi_29664_;
                                            phi_18124_ = type_60(_e5202, _e4726);
                                            phi_18125_ = type_60();
                                            phi_18126_ = true;
                                        } else {
                                            let _e4732 = (((_e1698 * _e1774) + (_e1699 * _e1775)) + (_e1700 * _e1777));
                                            let _e4737 = (((_e1698 * _e1770) + (_e1699 * _e1780)) + (_e1700 * _e1781));
                                            let _e4742 = (((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z));
                                            let _e4747 = (((_e4699.x * _e1774) + (_e4699.y * _e1775)) + (_e4699.z * _e1777));
                                            let _e4752 = (((_e4699.x * _e1770) + (_e4699.y * _e1780)) + (_e4699.z * _e1781));
                                            if (_e4742 <= 0f) {
                                                phi_18088_ = type_60();
                                                phi_18089_ = true;
                                            } else {
                                                let _e4754 = (_e4720 <= 0f);
                                                if _e4754 {
                                                    phi_18087_ = type_60();
                                                } else {
                                                    let _e4755 = (_e4732 + _e4747);
                                                    let _e4756 = (_e4737 + _e4752);
                                                    let _e4757 = (_e4742 + _e4720);
                                                    let _e4764 = (1f / sqrt((((_e4755 * _e4755) + (_e4756 * _e4756)) + (_e4757 * _e4757))));
                                                    let _e4767 = (_e4757 * _e4764);
                                                    if (_e4767 <= 0f) {
                                                        phi_28977_ = 0f;
                                                    } else {
                                                        let _e4769 = (_e2616 * _e2616);
                                                        let _e4773 = ((((_e4769 - 1f) * _e4767) * _e4767) + 1f);
                                                        phi_28977_ = (_e4769 / ((3.1415927f * _e4773) * _e4773));
                                                    }
                                                    let _e4778 = phi_28977_;
                                                    let _e4779 = (_e4742 * _e4742);
                                                    if (_e4779 >= 1f) {
                                                        phi_29005_ = 0f;
                                                    } else {
                                                        phi_29005_ = ((sqrt((1f + (((_e2616 * _e2616) * ((_e4732 * _e4732) + (_e4737 * _e4737))) / _e4779))) - 1f) * 0.5f);
                                                    }
                                                    let _e4792 = phi_29005_;
                                                    let _e4793 = (1f + _e4792);
                                                    let _e4794 = (_e4720 * _e4720);
                                                    if (_e4794 >= 1f) {
                                                        phi_29033_ = 0f;
                                                    } else {
                                                        phi_29033_ = ((sqrt((1f + (((_e2616 * _e2616) * ((_e4747 * _e4747) + (_e4752 * _e4752))) / _e4794))) - 1f) * 0.5f);
                                                    }
                                                    let _e4807 = phi_29033_;
                                                    let _e4814 = (((_e4732 * (_e4755 * _e4764)) + (_e4737 * (_e4756 * _e4764))) + (_e4742 * _e4767));
                                                    if (_e2614 == 0u) {
                                                        let _e5109 = (1f - _e4814);
                                                        if (_e5109 != _e5109) {
                                                            phi_29536_ = true;
                                                        } else {
                                                            phi_29536_ = (0f >= _e5109);
                                                        }
                                                        let _e5113 = phi_29536_;
                                                        let _e5115 = pow(select(_e5109, 0f, _e5113), 5f);
                                                        let _e5116 = (1f - _e5115);
                                                        phi_18075_ = vec3<f32>(((_e2612.x * _e5116) + _e5115), ((_e2612.y * _e5116) + _e5115), ((_e2612.z * _e5116) + _e5115));
                                                    } else {
                                                        if (_e2614 == 1u) {
                                                            let _e5082 = (1f / _e2608);
                                                            let _e5086 = ((_e5082 * _e5082) * (1f - (_e4814 * _e4814)));
                                                            if (_e5086 >= 1f) {
                                                                phi_29497_ = 1f;
                                                            } else {
                                                                let _e5089 = sqrt((1f - _e5086));
                                                                if (_e4814 != _e4814) {
                                                                    phi_29508_ = true;
                                                                } else {
                                                                    phi_29508_ = (_e5089 <= _e4814);
                                                                }
                                                                let _e5093 = phi_29508_;
                                                                let _e5098 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                                                phi_29497_ = (_e5098 + ((1f - _e5098) * pow((1f - select(_e4814, _e5089, _e5093)), 5f)));
                                                            }
                                                            let _e5105 = phi_29497_;
                                                            phi_18074_ = vec3(_e5105);
                                                        } else {
                                                            let _e4818 = select(_e4814, 0f, (_e4814 < 0f));
                                                            let _e4820 = select(_e4818, 1f, (_e4818 > 1f));
                                                            let _e4822 = (1f - (_e4820 * _e4820));
                                                            let _e4835 = ((_e2612.x * _e2612.x) - (_e2610.x * _e2610.x));
                                                            let _e4836 = ((_e2612.y * _e2612.y) - (_e2610.y * _e2610.y));
                                                            let _e4837 = ((_e2612.z * _e2612.z) - (_e2610.z * _e2610.z));
                                                            let _e4839 = (_e2612.x * (_e2610.x + _e2610.x));
                                                            let _e4841 = (_e2612.y * (_e2610.y + _e2610.y));
                                                            let _e4843 = (_e2612.z * (_e2610.z + _e2610.z));
                                                            let _e4850 = ((_e4835 * _e4835) + (_e4839 * _e4839));
                                                            let _e4851 = ((_e4836 * _e4836) + (_e4841 * _e4841));
                                                            let _e4852 = ((_e4837 * _e4837) + (_e4843 * _e4843));
                                                            let _e4857 = ((_e4822 * _e4835) / _e4850);
                                                            let _e4858 = ((_e4822 * _e4836) / _e4851);
                                                            let _e4859 = ((_e4822 * _e4837) / _e4852);
                                                            let _e4860 = (1f - _e4857);
                                                            let _e4861 = (1f - _e4858);
                                                            let _e4862 = (1f - _e4859);
                                                            let _e4864 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4857, _e4858, _e4859));
                                                            let _e4865 = (_e4822 * _e4839);
                                                            let _e4866 = (_e4822 * _e4841);
                                                            let _e4867 = (_e4822 * _e4843);
                                                            let _e4872 = (vec3<f32>(_e4865, _e4866, _e4867) / vec3<f32>(_e4850, _e4851, _e4852));
                                                            let _e4885 = ((_e4864.x * _e4864.x) + (_e4872.x * _e4872.x));
                                                            let _e4886 = ((_e4864.y * _e4864.y) + (_e4872.y * _e4872.y));
                                                            let _e4887 = ((_e4864.z * _e4864.z) + (_e4872.z * _e4872.z));
                                                            if (_e4885 != _e4885) {
                                                                phi_29167_ = true;
                                                            } else {
                                                                phi_29167_ = (0f >= _e4885);
                                                            }
                                                            let _e4891 = phi_29167_;
                                                            let _e4893 = sqrt(select(_e4885, 0f, _e4891));
                                                            if (_e4886 != _e4886) {
                                                                phi_29182_ = true;
                                                            } else {
                                                                phi_29182_ = (0f >= _e4886);
                                                            }
                                                            let _e4897 = phi_29182_;
                                                            let _e4899 = sqrt(select(_e4886, 0f, _e4897));
                                                            if (_e4887 != _e4887) {
                                                                phi_29197_ = true;
                                                            } else {
                                                                phi_29197_ = (0f >= _e4887);
                                                            }
                                                            let _e4903 = phi_29197_;
                                                            let _e4905 = sqrt(select(_e4887, 0f, _e4903));
                                                            let _e4909 = ((_e4893 + _e4860) * 0.5f);
                                                            let _e4910 = ((_e4899 + _e4861) * 0.5f);
                                                            let _e4911 = ((_e4905 + _e4862) * 0.5f);
                                                            if (_e4909 != _e4909) {
                                                                phi_29224_ = true;
                                                            } else {
                                                                phi_29224_ = (0f >= _e4909);
                                                            }
                                                            let _e4915 = phi_29224_;
                                                            let _e4917 = sqrt(select(_e4909, 0f, _e4915));
                                                            if (_e4910 != _e4910) {
                                                                phi_29239_ = true;
                                                            } else {
                                                                phi_29239_ = (0f >= _e4910);
                                                            }
                                                            let _e4921 = phi_29239_;
                                                            let _e4923 = sqrt(select(_e4910, 0f, _e4921));
                                                            if (_e4911 != _e4911) {
                                                                phi_29254_ = true;
                                                            } else {
                                                                phi_29254_ = (0f >= _e4911);
                                                            }
                                                            let _e4927 = phi_29254_;
                                                            let _e4929 = sqrt(select(_e4911, 0f, _e4927));
                                                            let _e4930 = vec3<f32>(_e4917, _e4923, _e4929);
                                                            let _e4934 = ((_e4893 - _e4860) * 0.5f);
                                                            let _e4935 = ((_e4899 - _e4861) * 0.5f);
                                                            let _e4936 = ((_e4905 - _e4862) * 0.5f);
                                                            if (_e4934 != _e4934) {
                                                                phi_29281_ = true;
                                                            } else {
                                                                phi_29281_ = (0f >= _e4934);
                                                            }
                                                            let _e4940 = phi_29281_;
                                                            let _e4942 = sqrt(select(_e4934, 0f, _e4940));
                                                            if (_e4935 != _e4935) {
                                                                phi_29296_ = true;
                                                            } else {
                                                                phi_29296_ = (0f >= _e4935);
                                                            }
                                                            let _e4946 = phi_29296_;
                                                            let _e4948 = sqrt(select(_e4935, 0f, _e4946));
                                                            if (_e4936 != _e4936) {
                                                                phi_29311_ = true;
                                                            } else {
                                                                phi_29311_ = (0f >= _e4936);
                                                            }
                                                            let _e4952 = phi_29311_;
                                                            let _e4954 = sqrt(select(_e4936, 0f, _e4952));
                                                            if ((_e4865 / _e4850) < 0f) {
                                                                phi_17951_ = -(_e4942);
                                                            } else {
                                                                phi_17951_ = _e4942;
                                                            }
                                                            let _e4958 = phi_17951_;
                                                            if ((_e4866 / _e4851) < 0f) {
                                                                phi_17959_ = -(_e4948);
                                                            } else {
                                                                phi_17959_ = _e4948;
                                                            }
                                                            let _e4962 = phi_17959_;
                                                            if ((_e4867 / _e4852) < 0f) {
                                                                phi_17967_ = -(_e4954);
                                                            } else {
                                                                phi_17967_ = _e4954;
                                                            }
                                                            let _e4966 = phi_17967_;
                                                            let _e4970 = (_e2610.x * _e4820);
                                                            let _e4971 = (_e2610.y * _e4820);
                                                            let _e4972 = (_e2610.z * _e4820);
                                                            let _e4982 = vec3<f32>(((_e2612.x * _e4917) - (_e2610.x * _e4958)), ((_e2612.y * _e4923) - (_e2610.y * _e4962)), ((_e2612.z * _e4929) - (_e2610.z * _e4966)));
                                                            let _e4989 = ((_e2612.x * _e4958) + (_e2610.x * _e4917));
                                                            let _e4990 = ((_e2612.y * _e4962) + (_e2610.y * _e4923));
                                                            let _e4991 = ((_e2612.z * _e4966) + (_e2610.z * _e4929));
                                                            let _e4993 = vec3<f32>((_e2612.x * _e4820), (_e2612.y * _e4820), (_e2612.z * _e4820));
                                                            let _e4994 = (_e4993 - _e4930);
                                                            let _e4995 = (_e4970 - _e4958);
                                                            let _e4996 = (_e4971 - _e4962);
                                                            let _e4997 = (_e4972 - _e4966);
                                                            let _e5010 = (_e4993 + _e4930);
                                                            let _e5011 = (_e4970 + _e4958);
                                                            let _e5012 = (_e4971 + _e4962);
                                                            let _e5013 = (_e4972 + _e4966);
                                                            let _e5023 = ((_e5010.x * _e5010.x) + (_e5011 * _e5011));
                                                            let _e5024 = ((_e5010.y * _e5010.y) + (_e5012 * _e5012));
                                                            let _e5025 = ((_e5010.z * _e5010.z) + (_e5013 * _e5013));
                                                            let _e5035 = vec3(_e4820);
                                                            let _e5036 = (_e5035 - _e4982);
                                                            let _e5037 = -(vec3<f32>(_e4989, _e4990, _e4991));
                                                            let _e5053 = (_e5035 + _e4982);
                                                            let _e5063 = ((_e5053.x * _e5053.x) + (_e4989 * _e4989));
                                                            let _e5064 = ((_e5053.y * _e5053.y) + (_e4990 * _e4990));
                                                            let _e5065 = ((_e5053.z * _e5053.z) + (_e4991 * _e4991));
                                                            phi_18074_ = vec3<f32>((((((_e4994.x * _e4994.x) + (_e4995 * _e4995)) / select(0.000000000000000000000000000001f, _e5023, (_e5023 > 0.000000000000000000000000000001f))) + (((_e5036.x * _e5036.x) + (_e5037.x * _e5037.x)) / select(0.000000000000000000000000000001f, _e5063, (_e5063 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4994.y * _e4994.y) + (_e4996 * _e4996)) / select(0.000000000000000000000000000001f, _e5024, (_e5024 > 0.000000000000000000000000000001f))) + (((_e5036.y * _e5036.y) + (_e5037.y * _e5037.y)) / select(0.000000000000000000000000000001f, _e5064, (_e5064 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4994.z * _e4994.z) + (_e4997 * _e4997)) / select(0.000000000000000000000000000001f, _e5025, (_e5025 > 0.000000000000000000000000000001f))) + (((_e5036.z * _e5036.z) + (_e5037.z * _e5037.z)) / select(0.000000000000000000000000000001f, _e5065, (_e5065 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e5108 = phi_18074_;
                                                        phi_18075_ = _e5108;
                                                    }
                                                    let _e5128 = phi_18075_;
                                                    let _e5130 = (4f * _e4742);
                                                    phi_18087_ = type_60((_e5128 * (((1f / (_e4793 + _e4807)) * _e4778) / _e5130)), ((_e4778 / _e4793) / _e5130));
                                                }
                                                let _e5137 = phi_18087_;
                                                phi_18088_ = _e5137;
                                                phi_18089_ = _e4754;
                                            }
                                            let _e5139 = phi_18088_;
                                            let _e5141 = phi_18089_;
                                            if _e5141 {
                                                phi_18094_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_18094_ = _e5139;
                                            }
                                            let _e5143 = phi_18094_;
                                            let _e5146 = (_e2618 == 1u);
                                            if _e5146 {
                                                phi_18122_ = _e5143;
                                                phi_18123_ = type_60();
                                            } else {
                                                if (_e4742 != _e4742) {
                                                    phi_29573_ = true;
                                                } else {
                                                    phi_29573_ = (0f >= _e4742);
                                                }
                                                let _e5150 = phi_29573_;
                                                let _e5151 = select(_e4742, 0f, _e5150);
                                                let _e5152 = (1f / _e2608);
                                                let _e5156 = ((_e5152 * _e5152) * (1f - (_e5151 * _e5151)));
                                                if (_e5156 >= 1f) {
                                                    phi_29592_ = 1f;
                                                } else {
                                                    let _e5159 = sqrt((1f - _e5156));
                                                    if (_e5151 != _e5151) {
                                                        phi_29603_ = true;
                                                    } else {
                                                        phi_29603_ = (_e5159 <= _e5151);
                                                    }
                                                    let _e5163 = phi_29603_;
                                                    let _e5168 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                                    phi_29592_ = (_e5168 + ((1f - _e5168) * pow((1f - select(_e5151, _e5159, _e5163)), 5f)));
                                                }
                                                let _e5175 = phi_29592_;
                                                if (_e4720 <= 0f) {
                                                    phi_29642_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_29642_ = (_e2606 * (_e4720 * 0.31830987f));
                                                }
                                                let _e5180 = phi_29642_;
                                                let _e5181 = (1f - _e5175);
                                                phi_18122_ = type_60();
                                                phi_18123_ = type_60((_e5143.member + vec3<f32>((_e5180.x * _e5181), (_e5180.y * _e5181), (_e5180.z * _e5181))), ((_e5175 * _e5143.member_1) + (_e5181 * _e4726)));
                                            }
                                            let _e5195 = phi_18122_;
                                            let _e5197 = phi_18123_;
                                            phi_18124_ = _e5195;
                                            phi_18125_ = _e5197;
                                            phi_18126_ = _e5146;
                                        }
                                        let _e5205 = phi_18124_;
                                        let _e5207 = phi_18125_;
                                        let _e5209 = phi_18126_;
                                        if _e5209 {
                                            phi_18130_ = _e5205;
                                        } else {
                                            phi_18130_ = _e5207;
                                        }
                                        let _e5211 = phi_18130_;
                                        if (_e4697 > 0f) {
                                            if (_e4697 >= 300000000000000000000000000000000000000f) {
                                                phi_29681_ = 1f;
                                            } else {
                                                let _e5216 = (_e4697 * _e4697);
                                                let _e5218 = (_e5216 + (_e5211.member_1 * _e5211.member_1));
                                                if (_e5218 == 0f) {
                                                    phi_29678_ = 0f;
                                                } else {
                                                    phi_29678_ = (_e5216 / _e5218);
                                                }
                                                let _e5222 = phi_29678_;
                                                phi_29681_ = _e5222;
                                            }
                                            let _e5224 = phi_29681_;
                                            phi_18139_ = (_e5224 / _e4697);
                                        } else {
                                            phi_18139_ = 1f;
                                        }
                                        let _e5227 = phi_18139_;
                                        if ((((_e5211.member.x * _e5211.member.x) + (_e5211.member.y * _e5211.member.y)) + (_e5211.member.z * _e5211.member.z)) > 0f) {
                                            let _e5239 = (vec3<f32>(1f, 1f, 1f) / _e4699);
                                            local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                            local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                            phi_18173_ = _e391;
                                            phi_18176_ = 0u;
                                            phi_18178_ = _e388;
                                            phi_18180_ = _e384;
                                            phi_18184_ = type_68(_e4703, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                            loop {
                                                let _e5241 = phi_18173_;
                                                let _e5243 = phi_18176_;
                                                let _e5245 = phi_18178_;
                                                let _e5247 = phi_18180_;
                                                let _e5249 = phi_18184_;
                                                local_108 = _e5249;
                                                if (_e5247 < _e5245) {
                                                    let _e5617 = global_2.member[_e5247];
                                                    if (_e5617.member_6.x == 0u) {
                                                        let _e5723 = (_e5617.member_1.x - _e5617.member.x);
                                                        let _e5724 = (_e5617.member_1.y - _e5617.member.y);
                                                        let _e5725 = (_e5617.member_1.z - _e5617.member.z);
                                                        let _e5730 = (_e5617.member_2.x - _e5617.member.x);
                                                        let _e5731 = (_e5617.member_2.y - _e5617.member.y);
                                                        let _e5732 = (_e5617.member_2.z - _e5617.member.z);
                                                        let _e5735 = ((_e4699.y * _e5732) - (_e5731 * _e4699.z));
                                                        let _e5738 = ((_e4699.z * _e5730) - (_e5732 * _e4699.x));
                                                        let _e5741 = ((_e4699.x * _e5731) - (_e5730 * _e4699.y));
                                                        let _e5746 = (((_e5723 * _e5735) + (_e5724 * _e5738)) + (_e5725 * _e5741));
                                                        if (abs(_e5746) < 0.000000000001f) {
                                                            phi_18351_ = vec4<f32>();
                                                            phi_18352_ = true;
                                                        } else {
                                                            let _e5749 = (1f / _e5746);
                                                            let _e5750 = (_e1751 - _e5617.member.x);
                                                            let _e5751 = (_e1752 - _e5617.member.y);
                                                            let _e5752 = (_e1753 - _e5617.member.z);
                                                            let _e5758 = ((((_e5750 * _e5735) + (_e5751 * _e5738)) + (_e5752 * _e5741)) * _e5749);
                                                            if (_e5758 < 0f) {
                                                                phi_18341_ = vec4<f32>();
                                                                phi_18342_ = true;
                                                                phi_18343_ = false;
                                                            } else {
                                                                let _e5760 = (_e5758 > 1f);
                                                                if _e5760 {
                                                                    phi_18339_ = vec4<f32>();
                                                                    phi_18340_ = false;
                                                                } else {
                                                                    let _e5763 = ((_e5751 * _e5725) - (_e5724 * _e5752));
                                                                    let _e5766 = ((_e5752 * _e5723) - (_e5725 * _e5750));
                                                                    let _e5769 = ((_e5750 * _e5724) - (_e5723 * _e5751));
                                                                    let _e5775 = ((((_e4699.x * _e5763) + (_e4699.y * _e5766)) + (_e4699.z * _e5769)) * _e5749);
                                                                    if (_e5775 < 0f) {
                                                                        phi_18329_ = vec4<f32>();
                                                                        phi_18330_ = true;
                                                                        phi_18331_ = false;
                                                                    } else {
                                                                        let _e5778 = ((_e5758 + _e5775) > 1f);
                                                                        if _e5778 {
                                                                            phi_18327_ = vec4<f32>();
                                                                            phi_18328_ = false;
                                                                        } else {
                                                                            let _e5784 = ((((_e5730 * _e5763) + (_e5731 * _e5766)) + (_e5732 * _e5769)) * _e5749);
                                                                            if (_e5784 <= 0f) {
                                                                                phi_18322_ = vec4<f32>();
                                                                                phi_18323_ = true;
                                                                            } else {
                                                                                let _e5786 = (_e5784 >= _e5249.member);
                                                                                if _e5786 {
                                                                                    phi_18321_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_18321_ = vec4<f32>(_e5784, _e5758, _e5775, 1f);
                                                                                }
                                                                                let _e5789 = phi_18321_;
                                                                                phi_18322_ = _e5789;
                                                                                phi_18323_ = _e5786;
                                                                            }
                                                                            let _e5791 = phi_18322_;
                                                                            let _e5793 = phi_18323_;
                                                                            phi_18327_ = _e5791;
                                                                            phi_18328_ = _e5793;
                                                                        }
                                                                        let _e5795 = phi_18327_;
                                                                        let _e5797 = phi_18328_;
                                                                        phi_18329_ = _e5795;
                                                                        phi_18330_ = _e5778;
                                                                        phi_18331_ = _e5797;
                                                                    }
                                                                    let _e5799 = phi_18329_;
                                                                    let _e5801 = phi_18330_;
                                                                    let _e5803 = phi_18331_;
                                                                    phi_18339_ = _e5799;
                                                                    phi_18340_ = select(_e5803, true, _e5801);
                                                                }
                                                                let _e5806 = phi_18339_;
                                                                let _e5808 = phi_18340_;
                                                                phi_18341_ = _e5806;
                                                                phi_18342_ = _e5760;
                                                                phi_18343_ = _e5808;
                                                            }
                                                            let _e5810 = phi_18341_;
                                                            let _e5812 = phi_18342_;
                                                            let _e5814 = phi_18343_;
                                                            phi_18351_ = _e5810;
                                                            phi_18352_ = select(_e5814, true, _e5812);
                                                        }
                                                        let _e5817 = phi_18351_;
                                                        let _e5819 = phi_18352_;
                                                        phi_18498_ = _e5817;
                                                        phi_18499_ = _e5819;
                                                    } else {
                                                        if (_e5617.member_6.x == 1u) {
                                                            let _e5645 = (_e1751 - _e5617.member.x);
                                                            let _e5646 = (_e1752 - _e5617.member.y);
                                                            let _e5647 = (_e1753 - _e5617.member.z);
                                                            let _e5652 = (((_e5645 * _e4699.x) + (_e5646 * _e4699.y)) + (_e5647 * _e4699.z));
                                                            if (_e5652 > 0f) {
                                                                phi_18392_ = select(true, false, ((((_e5645 * _e5645) + (_e5646 * _e5646)) + (_e5647 * _e5647)) > (_e5617.member.w * _e5617.member.w)));
                                                            } else {
                                                                phi_18392_ = true;
                                                            }
                                                            let _e5665 = phi_18392_;
                                                            if _e5665 {
                                                                let _e5669 = (_e5645 - (_e4699.x * _e5652));
                                                                let _e5670 = (_e5646 - (_e4699.y * _e5652));
                                                                let _e5671 = (_e5647 - (_e4699.z * _e5652));
                                                                let _e5680 = ((_e5617.member.w * _e5617.member.w) - (((_e5669 * _e5669) + (_e5670 * _e5670)) + (_e5671 * _e5671)));
                                                                if (_e5680 < 0f) {
                                                                    phi_18451_ = vec4<f32>();
                                                                    phi_18452_ = true;
                                                                } else {
                                                                    let _e5682 = sqrt(_e5680);
                                                                    let _e5683 = -(_e5652);
                                                                    let _e5684 = (_e5683 - _e5682);
                                                                    if (_e5684 <= 0f) {
                                                                        phi_18429_ = true;
                                                                    } else {
                                                                        phi_18429_ = (_e5684 >= _e5249.member);
                                                                    }
                                                                    let _e5688 = phi_18429_;
                                                                    if _e5688 {
                                                                        phi_18435_ = (_e5683 + _e5682);
                                                                    } else {
                                                                        phi_18435_ = _e5684;
                                                                    }
                                                                    let _e5691 = phi_18435_;
                                                                    if (_e5691 <= 0f) {
                                                                        phi_18446_ = vec4<f32>();
                                                                        phi_18447_ = true;
                                                                    } else {
                                                                        let _e5693 = (_e5691 >= _e5249.member);
                                                                        if _e5693 {
                                                                            phi_18445_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18445_ = vec4<f32>(_e5691, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5696 = phi_18445_;
                                                                        phi_18446_ = _e5696;
                                                                        phi_18447_ = _e5693;
                                                                    }
                                                                    let _e5698 = phi_18446_;
                                                                    let _e5700 = phi_18447_;
                                                                    phi_18451_ = _e5698;
                                                                    phi_18452_ = _e5700;
                                                                }
                                                                let _e5702 = phi_18451_;
                                                                let _e5704 = phi_18452_;
                                                                phi_18456_ = _e5702;
                                                                phi_18457_ = _e5704;
                                                            } else {
                                                                phi_18456_ = vec4<f32>();
                                                                phi_18457_ = true;
                                                            }
                                                            let _e5706 = phi_18456_;
                                                            let _e5708 = phi_18457_;
                                                            phi_18488_ = _e5706;
                                                            phi_18489_ = _e5708;
                                                        } else {
                                                            if (abs(_e4699.y) < 0.000000000001f) {
                                                                phi_18486_ = vec4<f32>();
                                                                phi_18487_ = true;
                                                            } else {
                                                                let _e5627 = ((_e5617.member.x - _e1752) / _e4699.y);
                                                                if (_e5627 <= 0f) {
                                                                    phi_18481_ = vec4<f32>();
                                                                    phi_18482_ = true;
                                                                } else {
                                                                    let _e5629 = (_e5627 >= _e5249.member);
                                                                    if _e5629 {
                                                                        phi_18480_ = vec4<f32>();
                                                                    } else {
                                                                        phi_18480_ = vec4<f32>(_e5627, 0f, 0f, 1f);
                                                                    }
                                                                    let _e5632 = phi_18480_;
                                                                    phi_18481_ = _e5632;
                                                                    phi_18482_ = _e5629;
                                                                }
                                                                let _e5634 = phi_18481_;
                                                                let _e5636 = phi_18482_;
                                                                phi_18486_ = _e5634;
                                                                phi_18487_ = _e5636;
                                                            }
                                                            let _e5638 = phi_18486_;
                                                            let _e5640 = phi_18487_;
                                                            phi_18488_ = _e5638;
                                                            phi_18489_ = _e5640;
                                                        }
                                                        let _e5710 = phi_18488_;
                                                        let _e5712 = phi_18489_;
                                                        phi_18498_ = _e5710;
                                                        phi_18499_ = select(true, false, select(true, false, _e5712));
                                                    }
                                                    let _e5821 = phi_18498_;
                                                    let _e5823 = phi_18499_;
                                                    let _e5826 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5821, vec4(select(true, false, _e5823)));
                                                    let _e5828 = (_e5826.w > 0f);
                                                    if _e5828 {
                                                        let _e5834 = type_68(_e5826.x, _e5249.member_1, _e5249.member_2, _e5249.member_3);
                                                        let _e5839 = type_68(_e5834.member, _e5247, _e5834.member_2, _e5834.member_3);
                                                        phi_18520_ = _e5839;
                                                        phi_18522_ = _e5839;
                                                    } else {
                                                        phi_18520_ = type_68();
                                                        phi_18522_ = _e5249;
                                                    }
                                                    let _e5841 = phi_18520_;
                                                    let _e5843 = phi_18522_;
                                                    let _e5844 = select(true, false, _e5828);
                                                    if _e5844 {
                                                        phi_18528_ = (_e5247 + 1u);
                                                    } else {
                                                        phi_18528_ = u32();
                                                    }
                                                    let _e5847 = phi_18528_;
                                                    phi_18174_ = _e5241;
                                                    phi_18177_ = _e5243;
                                                    phi_18179_ = _e5245;
                                                    phi_18181_ = _e5847;
                                                    phi_18185_ = _e5843;
                                                    phi_18704_ = _e5841;
                                                    phi_18705_ = _e5844;
                                                    phi_18706_ = false;
                                                } else {
                                                    if (_e5241 == 4294967295u) {
                                                        let _e5252 = (_e5243 == 0u);
                                                        if _e5252 {
                                                            phi_18559_ = u32();
                                                            phi_18560_ = u32();
                                                            phi_18562_ = false;
                                                            phi_18563_ = false;
                                                        } else {
                                                            let _e5253 = (_e5243 - 1u);
                                                            let _e5254 = (_e5253 < 64u);
                                                            if _e5254 {
                                                            } else {
                                                                phi_35252_ = true;
                                                                phi_20652_ = bool();
                                                                phi_20651_ = type_68();
                                                                break;
                                                            }
                                                            let _e5256 = local_68[_e5253];
                                                            let _e5258 = (_e5256 >= _e5249.member);
                                                            if _e5258 {
                                                                phi_18557_ = u32();
                                                            } else {
                                                                if _e5254 {
                                                                } else {
                                                                    phi_35252_ = true;
                                                                    phi_20652_ = bool();
                                                                    phi_20651_ = type_68();
                                                                    break;
                                                                }
                                                                let _e5260 = local_67[_e5253];
                                                                phi_18557_ = _e5260;
                                                            }
                                                            let _e5262 = phi_18557_;
                                                            phi_18559_ = _e5253;
                                                            phi_18560_ = _e5262;
                                                            phi_18562_ = select(true, false, _e5258);
                                                            phi_18563_ = _e5258;
                                                        }
                                                        let _e5265 = phi_18559_;
                                                        let _e5267 = phi_18560_;
                                                        let _e5269 = phi_18562_;
                                                        let _e5271 = phi_18563_;
                                                        phi_18564_ = _e5265;
                                                        phi_18565_ = _e5267;
                                                        phi_18566_ = _e5265;
                                                        phi_18567_ = _e5269;
                                                        phi_18568_ = _e5271;
                                                        phi_18569_ = _e5252;
                                                    } else {
                                                        phi_18564_ = u32();
                                                        phi_18565_ = _e5241;
                                                        phi_18566_ = _e5243;
                                                        phi_18567_ = true;
                                                        phi_18568_ = false;
                                                        phi_18569_ = false;
                                                    }
                                                    let _e5273 = phi_18564_;
                                                    let _e5275 = phi_18565_;
                                                    let _e5277 = phi_18566_;
                                                    let _e5279 = phi_18567_;
                                                    let _e5281 = phi_18568_;
                                                    let _e5283 = phi_18569_;
                                                    if _e5279 {
                                                        let _e5285 = ((_e5275 & 2147483648u) == 0u);
                                                        if _e5285 {
                                                            let _e5293 = global_3.member[_e5275];
                                                            if (_e5293.member_1 == 4294967295u) {
                                                                phi_18599_ = 300000000000000000000000000000000000000f;
                                                            } else {
                                                                let _e5306 = ((_e5293.member.x - _e1751) * _e5239.x);
                                                                let _e5308 = ((_e5293.member.y - _e1752) * _e5239.y);
                                                                let _e5310 = ((_e5293.member.z - _e1753) * _e5239.z);
                                                                let _e5317 = ((_e5293.member_2.x - _e1751) * _e5239.x);
                                                                let _e5318 = ((_e5293.member_2.y - _e1752) * _e5239.y);
                                                                let _e5319 = ((_e5293.member_2.z - _e1753) * _e5239.z);
                                                                if (_e5306 == _e5306) {
                                                                    let _e5321 = (_e5317 == _e5317);
                                                                    if _e5321 {
                                                                        let _e5322 = (_e5306 != _e5306);
                                                                        if _e5322 {
                                                                            phi_29779_ = true;
                                                                        } else {
                                                                            phi_29779_ = (_e5317 <= _e5306);
                                                                        }
                                                                        let _e5325 = phi_29779_;
                                                                        let _e5326 = select(_e5306, _e5317, _e5325);
                                                                        if _e5322 {
                                                                            phi_29809_ = true;
                                                                        } else {
                                                                            phi_29809_ = (_e5317 >= _e5306);
                                                                        }
                                                                        let _e5331 = phi_29809_;
                                                                        let _e5332 = select(_e5306, _e5317, _e5331);
                                                                        phi_29756_ = type_57(select(-300000000000000000000000000000000000000f, _e5326, (_e5326 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5332, (_e5332 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29756_ = type_57();
                                                                    }
                                                                    let _e5337 = phi_29756_;
                                                                    phi_29759_ = _e5337;
                                                                    phi_29760_ = select(true, false, _e5321);
                                                                } else {
                                                                    phi_29759_ = type_57();
                                                                    phi_29760_ = true;
                                                                }
                                                                let _e5340 = phi_29759_;
                                                                let _e5342 = phi_29760_;
                                                                if _e5342 {
                                                                    phi_29765_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29765_ = _e5340;
                                                                }
                                                                let _e5344 = phi_29765_;
                                                                if (_e5308 == _e5308) {
                                                                    let _e5348 = (_e5318 == _e5318);
                                                                    if _e5348 {
                                                                        let _e5349 = (_e5308 != _e5308);
                                                                        if _e5349 {
                                                                            phi_29866_ = true;
                                                                        } else {
                                                                            phi_29866_ = (_e5318 <= _e5308);
                                                                        }
                                                                        let _e5352 = phi_29866_;
                                                                        let _e5353 = select(_e5308, _e5318, _e5352);
                                                                        if (_e5344.member != _e5344.member) {
                                                                            phi_29881_ = true;
                                                                        } else {
                                                                            phi_29881_ = (_e5353 >= _e5344.member);
                                                                        }
                                                                        let _e5357 = phi_29881_;
                                                                        if _e5349 {
                                                                            phi_29896_ = true;
                                                                        } else {
                                                                            phi_29896_ = (_e5318 >= _e5308);
                                                                        }
                                                                        let _e5361 = phi_29896_;
                                                                        let _e5362 = select(_e5308, _e5318, _e5361);
                                                                        if (_e5344.member_1 != _e5344.member_1) {
                                                                            phi_29911_ = true;
                                                                        } else {
                                                                            phi_29911_ = (_e5362 <= _e5344.member_1);
                                                                        }
                                                                        let _e5366 = phi_29911_;
                                                                        phi_29843_ = type_57(select(_e5344.member, _e5353, _e5357), select(_e5344.member_1, _e5362, _e5366));
                                                                    } else {
                                                                        phi_29843_ = type_57();
                                                                    }
                                                                    let _e5370 = phi_29843_;
                                                                    phi_29846_ = _e5370;
                                                                    phi_29847_ = select(true, false, _e5348);
                                                                } else {
                                                                    phi_29846_ = type_57();
                                                                    phi_29847_ = true;
                                                                }
                                                                let _e5373 = phi_29846_;
                                                                let _e5375 = phi_29847_;
                                                                if _e5375 {
                                                                    phi_29852_ = _e5344;
                                                                } else {
                                                                    phi_29852_ = _e5373;
                                                                }
                                                                let _e5377 = phi_29852_;
                                                                if (_e5310 == _e5310) {
                                                                    let _e5381 = (_e5319 == _e5319);
                                                                    if _e5381 {
                                                                        let _e5382 = (_e5310 != _e5310);
                                                                        if _e5382 {
                                                                            phi_29953_ = true;
                                                                        } else {
                                                                            phi_29953_ = (_e5319 <= _e5310);
                                                                        }
                                                                        let _e5385 = phi_29953_;
                                                                        let _e5386 = select(_e5310, _e5319, _e5385);
                                                                        if (_e5377.member != _e5377.member) {
                                                                            phi_29968_ = true;
                                                                        } else {
                                                                            phi_29968_ = (_e5386 >= _e5377.member);
                                                                        }
                                                                        let _e5390 = phi_29968_;
                                                                        if _e5382 {
                                                                            phi_29983_ = true;
                                                                        } else {
                                                                            phi_29983_ = (_e5319 >= _e5310);
                                                                        }
                                                                        let _e5394 = phi_29983_;
                                                                        let _e5395 = select(_e5310, _e5319, _e5394);
                                                                        if (_e5377.member_1 != _e5377.member_1) {
                                                                            phi_29998_ = true;
                                                                        } else {
                                                                            phi_29998_ = (_e5395 <= _e5377.member_1);
                                                                        }
                                                                        let _e5399 = phi_29998_;
                                                                        phi_29930_ = type_57(select(_e5377.member, _e5386, _e5390), select(_e5377.member_1, _e5395, _e5399));
                                                                    } else {
                                                                        phi_29930_ = type_57();
                                                                    }
                                                                    let _e5403 = phi_29930_;
                                                                    phi_29933_ = _e5403;
                                                                    phi_29934_ = select(true, false, _e5381);
                                                                } else {
                                                                    phi_29933_ = type_57();
                                                                    phi_29934_ = true;
                                                                }
                                                                let _e5406 = phi_29933_;
                                                                let _e5408 = phi_29934_;
                                                                if _e5408 {
                                                                    phi_29939_ = _e5377;
                                                                } else {
                                                                    phi_29939_ = _e5406;
                                                                }
                                                                let _e5410 = phi_29939_;
                                                                if (_e5410.member_1 < 0f) {
                                                                    phi_29737_ = true;
                                                                } else {
                                                                    if (_e5410.member > _e5410.member_1) {
                                                                        phi_29734_ = true;
                                                                    } else {
                                                                        phi_29734_ = (_e5410.member > _e5249.member);
                                                                    }
                                                                    let _e5417 = phi_29734_;
                                                                    phi_29737_ = _e5417;
                                                                }
                                                                let _e5419 = phi_29737_;
                                                                phi_18599_ = select(_e5410.member, 300000000000000000000000000000000000000f, _e5419);
                                                            }
                                                            let _e5422 = phi_18599_;
                                                            if (_e5293.member_5 == 4294967295u) {
                                                                phi_18609_ = 300000000000000000000000000000000000000f;
                                                            } else {
                                                                let _e5435 = ((_e5293.member_4.x - _e1751) * _e5239.x);
                                                                let _e5437 = ((_e5293.member_4.y - _e1752) * _e5239.y);
                                                                let _e5439 = ((_e5293.member_4.z - _e1753) * _e5239.z);
                                                                let _e5446 = ((_e5293.member_6.x - _e1751) * _e5239.x);
                                                                let _e5447 = ((_e5293.member_6.y - _e1752) * _e5239.y);
                                                                let _e5448 = ((_e5293.member_6.z - _e1753) * _e5239.z);
                                                                if (_e5435 == _e5435) {
                                                                    let _e5450 = (_e5446 == _e5446);
                                                                    if _e5450 {
                                                                        let _e5451 = (_e5435 != _e5435);
                                                                        if _e5451 {
                                                                            phi_30100_ = true;
                                                                        } else {
                                                                            phi_30100_ = (_e5446 <= _e5435);
                                                                        }
                                                                        let _e5454 = phi_30100_;
                                                                        let _e5455 = select(_e5435, _e5446, _e5454);
                                                                        if _e5451 {
                                                                            phi_30130_ = true;
                                                                        } else {
                                                                            phi_30130_ = (_e5446 >= _e5435);
                                                                        }
                                                                        let _e5460 = phi_30130_;
                                                                        let _e5461 = select(_e5435, _e5446, _e5460);
                                                                        phi_30077_ = type_57(select(-300000000000000000000000000000000000000f, _e5455, (_e5455 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5461, (_e5461 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_30077_ = type_57();
                                                                    }
                                                                    let _e5466 = phi_30077_;
                                                                    phi_30080_ = _e5466;
                                                                    phi_30081_ = select(true, false, _e5450);
                                                                } else {
                                                                    phi_30080_ = type_57();
                                                                    phi_30081_ = true;
                                                                }
                                                                let _e5469 = phi_30080_;
                                                                let _e5471 = phi_30081_;
                                                                if _e5471 {
                                                                    phi_30086_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_30086_ = _e5469;
                                                                }
                                                                let _e5473 = phi_30086_;
                                                                if (_e5437 == _e5437) {
                                                                    let _e5477 = (_e5447 == _e5447);
                                                                    if _e5477 {
                                                                        let _e5478 = (_e5437 != _e5437);
                                                                        if _e5478 {
                                                                            phi_30187_ = true;
                                                                        } else {
                                                                            phi_30187_ = (_e5447 <= _e5437);
                                                                        }
                                                                        let _e5481 = phi_30187_;
                                                                        let _e5482 = select(_e5437, _e5447, _e5481);
                                                                        if (_e5473.member != _e5473.member) {
                                                                            phi_30202_ = true;
                                                                        } else {
                                                                            phi_30202_ = (_e5482 >= _e5473.member);
                                                                        }
                                                                        let _e5486 = phi_30202_;
                                                                        if _e5478 {
                                                                            phi_30217_ = true;
                                                                        } else {
                                                                            phi_30217_ = (_e5447 >= _e5437);
                                                                        }
                                                                        let _e5490 = phi_30217_;
                                                                        let _e5491 = select(_e5437, _e5447, _e5490);
                                                                        if (_e5473.member_1 != _e5473.member_1) {
                                                                            phi_30232_ = true;
                                                                        } else {
                                                                            phi_30232_ = (_e5491 <= _e5473.member_1);
                                                                        }
                                                                        let _e5495 = phi_30232_;
                                                                        phi_30164_ = type_57(select(_e5473.member, _e5482, _e5486), select(_e5473.member_1, _e5491, _e5495));
                                                                    } else {
                                                                        phi_30164_ = type_57();
                                                                    }
                                                                    let _e5499 = phi_30164_;
                                                                    phi_30167_ = _e5499;
                                                                    phi_30168_ = select(true, false, _e5477);
                                                                } else {
                                                                    phi_30167_ = type_57();
                                                                    phi_30168_ = true;
                                                                }
                                                                let _e5502 = phi_30167_;
                                                                let _e5504 = phi_30168_;
                                                                if _e5504 {
                                                                    phi_30173_ = _e5473;
                                                                } else {
                                                                    phi_30173_ = _e5502;
                                                                }
                                                                let _e5506 = phi_30173_;
                                                                if (_e5439 == _e5439) {
                                                                    let _e5510 = (_e5448 == _e5448);
                                                                    if _e5510 {
                                                                        let _e5511 = (_e5439 != _e5439);
                                                                        if _e5511 {
                                                                            phi_30274_ = true;
                                                                        } else {
                                                                            phi_30274_ = (_e5448 <= _e5439);
                                                                        }
                                                                        let _e5514 = phi_30274_;
                                                                        let _e5515 = select(_e5439, _e5448, _e5514);
                                                                        if (_e5506.member != _e5506.member) {
                                                                            phi_30289_ = true;
                                                                        } else {
                                                                            phi_30289_ = (_e5515 >= _e5506.member);
                                                                        }
                                                                        let _e5519 = phi_30289_;
                                                                        if _e5511 {
                                                                            phi_30304_ = true;
                                                                        } else {
                                                                            phi_30304_ = (_e5448 >= _e5439);
                                                                        }
                                                                        let _e5523 = phi_30304_;
                                                                        let _e5524 = select(_e5439, _e5448, _e5523);
                                                                        if (_e5506.member_1 != _e5506.member_1) {
                                                                            phi_30319_ = true;
                                                                        } else {
                                                                            phi_30319_ = (_e5524 <= _e5506.member_1);
                                                                        }
                                                                        let _e5528 = phi_30319_;
                                                                        phi_30251_ = type_57(select(_e5506.member, _e5515, _e5519), select(_e5506.member_1, _e5524, _e5528));
                                                                    } else {
                                                                        phi_30251_ = type_57();
                                                                    }
                                                                    let _e5532 = phi_30251_;
                                                                    phi_30254_ = _e5532;
                                                                    phi_30255_ = select(true, false, _e5510);
                                                                } else {
                                                                    phi_30254_ = type_57();
                                                                    phi_30255_ = true;
                                                                }
                                                                let _e5535 = phi_30254_;
                                                                let _e5537 = phi_30255_;
                                                                if _e5537 {
                                                                    phi_30260_ = _e5506;
                                                                } else {
                                                                    phi_30260_ = _e5535;
                                                                }
                                                                let _e5539 = phi_30260_;
                                                                if (_e5539.member_1 < 0f) {
                                                                    phi_30058_ = true;
                                                                } else {
                                                                    if (_e5539.member > _e5539.member_1) {
                                                                        phi_30055_ = true;
                                                                    } else {
                                                                        phi_30055_ = (_e5539.member > _e5249.member);
                                                                    }
                                                                    let _e5546 = phi_30055_;
                                                                    phi_30058_ = _e5546;
                                                                }
                                                                let _e5548 = phi_30058_;
                                                                phi_18609_ = select(_e5539.member, 300000000000000000000000000000000000000f, _e5548);
                                                            }
                                                            let _e5551 = phi_18609_;
                                                            if (_e5293.member_3 > 0u) {
                                                                phi_18620_ = ((2147483648u | ((_e5293.member_3 - 1u) << bitcast<u32>(28u))) | _e5293.member_1);
                                                            } else {
                                                                phi_18620_ = _e5293.member_1;
                                                            }
                                                            let _e5560 = phi_18620_;
                                                            if (_e5293.member_7 > 0u) {
                                                                phi_18631_ = ((2147483648u | ((_e5293.member_7 - 1u) << bitcast<u32>(28u))) | _e5293.member_5);
                                                            } else {
                                                                phi_18631_ = _e5293.member_5;
                                                            }
                                                            let _e5569 = phi_18631_;
                                                            let _e5570 = (_e5422 <= _e5551);
                                                            let _e5574 = select(_e5422, _e5551, _e5570);
                                                            if (_e5574 < 300000000000000000000000000000000000000f) {
                                                                let _e5576 = (_e5277 < 64u);
                                                                if _e5576 {
                                                                    if _e5576 {
                                                                    } else {
                                                                        phi_35252_ = true;
                                                                        phi_20652_ = bool();
                                                                        phi_20651_ = type_68();
                                                                        break;
                                                                    }
                                                                    local_67[_e5277] = select(_e5560, _e5569, _e5570);
                                                                    if _e5576 {
                                                                    } else {
                                                                        phi_35252_ = true;
                                                                        phi_20652_ = bool();
                                                                        phi_20651_ = type_68();
                                                                        break;
                                                                    }
                                                                    local_68[_e5277] = _e5574;
                                                                    phi_18670_ = (_e5277 + 1u);
                                                                } else {
                                                                    phi_18670_ = _e5277;
                                                                }
                                                                let _e5581 = phi_18670_;
                                                                phi_18671_ = _e5581;
                                                            } else {
                                                                phi_18671_ = _e5277;
                                                            }
                                                            let _e5583 = phi_18671_;
                                                            phi_18682_ = select(4294967295u, select(_e5569, _e5560, _e5570), (select(_e5551, _e5422, _e5570) < 300000000000000000000000000000000000000f));
                                                            phi_18683_ = _e5583;
                                                            phi_18684_ = u32();
                                                            phi_18685_ = u32();
                                                        } else {
                                                            let _e5286 = (_e5275 & 268435455u);
                                                            phi_18682_ = u32();
                                                            phi_18683_ = u32();
                                                            phi_18684_ = ((_e5286 + ((_e5275 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                            phi_18685_ = _e5286;
                                                        }
                                                        let _e5587 = phi_18682_;
                                                        let _e5589 = phi_18683_;
                                                        let _e5591 = phi_18684_;
                                                        let _e5593 = phi_18685_;
                                                        phi_18687_ = _e5587;
                                                        phi_18688_ = _e5589;
                                                        phi_18690_ = _e5591;
                                                        phi_18691_ = _e5593;
                                                        phi_18692_ = select(true, false, _e5285);
                                                        phi_18693_ = _e5285;
                                                    } else {
                                                        phi_18687_ = u32();
                                                        phi_18688_ = u32();
                                                        phi_18690_ = _e5245;
                                                        phi_18691_ = _e5247;
                                                        phi_18692_ = _e5281;
                                                        phi_18693_ = false;
                                                    }
                                                    let _e5596 = phi_18687_;
                                                    let _e5598 = phi_18688_;
                                                    let _e5600 = phi_18690_;
                                                    let _e5602 = phi_18691_;
                                                    let _e5604 = phi_18692_;
                                                    let _e5606 = phi_18693_;
                                                    phi_18174_ = select(_e5596, 4294967295u, _e5604);
                                                    phi_18177_ = select(_e5598, select(_e5273, _e5277, _e5279), _e5604);
                                                    phi_18179_ = select(_e5245, _e5600, _e5604);
                                                    phi_18181_ = select(_e5247, _e5602, _e5604);
                                                    phi_18185_ = _e5249;
                                                    phi_18704_ = type_68();
                                                    phi_18705_ = select(_e5606, true, _e5604);
                                                    phi_18706_ = select(select(_e5283, false, _e5279), false, _e5604);
                                                }
                                                let _e5849 = phi_18174_;
                                                let _e5851 = phi_18177_;
                                                let _e5853 = phi_18179_;
                                                let _e5855 = phi_18181_;
                                                let _e5857 = phi_18185_;
                                                let _e5859 = phi_18704_;
                                                let _e5861 = phi_18705_;
                                                let _e5863 = phi_18706_;
                                                phi_18173_ = _e5849;
                                                phi_18176_ = _e5851;
                                                phi_18178_ = _e5853;
                                                phi_18180_ = _e5855;
                                                phi_18184_ = _e5857;
                                                phi_35252_ = _e2631;
                                                phi_20652_ = _e5863;
                                                phi_20651_ = _e5859;
                                                if !(_e5861) {
                                                    break;
                                                }
                                            }
                                            let _e5866 = phi_35252_;
                                            let _e5868 = phi_20652_;
                                            let _e5870 = phi_20651_;
                                            phi_35700_ = _e4695;
                                            phi_35686_ = _e5866;
                                            if _e5866 {
                                                break;
                                            }
                                            if _e5868 {
                                                let _e8402 = local_108;
                                                phi_18878_ = _e8402;
                                            } else {
                                                phi_18878_ = type_68();
                                            }
                                            let _e5872 = phi_18878_;
                                            if select(true, false, _e5868) {
                                                phi_18883_ = _e5870;
                                            } else {
                                                phi_18883_ = _e5872;
                                            }
                                            let _e5875 = phi_18883_;
                                            if (_e5875.member_1 != 4294967295u) {
                                                phi_18937_ = _e2635;
                                                phi_18938_ = _e2637;
                                            } else {
                                                if (_e2639 == (_e2629 + 2u)) {
                                                    phi_18935_ = _e2635;
                                                    phi_18936_ = type_42(vec4<f32>(_e1751, _e1752, _e1753, -1f), vec4<f32>(_e4699.x, _e4699.y, _e4699.z, 0f), vec4<f32>(((_e1621.x * _e5211.member.x) * _e5227), ((_e1621.y * _e5211.member.y) * _e5227), ((_e1621.z * _e5211.member.z) * _e5227), 0f));
                                                } else {
                                                    phi_18935_ = vec3<f32>((_e2635.x + (((_e1621.x * _e5211.member.x) * _e4701.x) * _e5227)), (_e2635.y + (((_e1621.y * _e5211.member.y) * _e4701.y) * _e5227)), (_e2635.z + (((_e1621.z * _e5211.member.z) * _e4701.z) * _e5227)));
                                                    phi_18936_ = _e2637;
                                                }
                                                let _e5913 = phi_18935_;
                                                let _e5915 = phi_18936_;
                                                phi_18937_ = _e5913;
                                                phi_18938_ = _e5915;
                                            }
                                            let _e5917 = phi_18937_;
                                            let _e5919 = phi_18938_;
                                            phi_35689_ = _e5866;
                                            phi_18939_ = _e5917;
                                            phi_18940_ = _e5919;
                                            phi_18941_ = (_e2641 + 1u);
                                        } else {
                                            phi_35689_ = _e2631;
                                            phi_18939_ = _e2635;
                                            phi_18940_ = _e2637;
                                            phi_18941_ = _e2641;
                                        }
                                        let _e5921 = phi_35689_;
                                        let _e5923 = phi_18939_;
                                        let _e5925 = phi_18940_;
                                        let _e5927 = phi_18941_;
                                        phi_35688_ = _e5921;
                                        phi_18942_ = _e5923;
                                        phi_18943_ = _e5925;
                                        phi_18944_ = _e5927;
                                    } else {
                                        phi_35688_ = _e2631;
                                        phi_18942_ = _e2635;
                                        phi_18943_ = _e2637;
                                        phi_18944_ = _e2641;
                                    }
                                    let _e5929 = phi_35688_;
                                    let _e5931 = phi_18942_;
                                    let _e5933 = phi_18943_;
                                    let _e5935 = phi_18944_;
                                    phi_35980_ = _e4695;
                                    phi_35687_ = _e5929;
                                    phi_14684_ = _e5931;
                                    phi_14687_ = _e5933;
                                    phi_14689_ = (_e2639 + 1u);
                                    phi_14691_ = _e5935;
                                } else {
                                    phi_35980_ = _e2633;
                                    phi_35687_ = _e2631;
                                    phi_14684_ = vec3<f32>();
                                    phi_14687_ = type_42();
                                    phi_14689_ = u32();
                                    phi_14691_ = u32();
                                }
                                let _e5938 = phi_35980_;
                                let _e5940 = phi_35687_;
                                let _e5942 = phi_14684_;
                                let _e5944 = phi_14687_;
                                let _e5946 = phi_14689_;
                                let _e5948 = phi_14691_;
                                phi_35324_ = _e5940;
                                phi_33399_ = _e5938;
                                phi_14683_ = _e5942;
                                phi_14686_ = _e5944;
                                phi_14688_ = _e5946;
                                phi_14690_ = _e5948;
                                phi_35700_ = _e5938;
                                phi_35686_ = _e5940;
                                if !(_e2643) {
                                    break;
                                }
                            }
                            let _e5951 = phi_35700_;
                            let _e5953 = phi_35686_;
                            if _e5953 {
                                break;
                            }
                            if (_e358 >= _e375) {
                                let _e6957 = local_102;
                                let _e6960 = local_103;
                                let _e6963 = local_104;
                                let _e6966 = local_105;
                                let _e6972 = local_106;
                                phi_19495_ = type_73(type_35(_e334.member, _e334.member_1, vec4<f32>(_e1621.x, _e1621.y, _e1621.z, f32(_e358)), vec4<f32>(_e6960.x, _e6963.y, _e6966.z, f32(_e6957)), vec4<u32>(_e5951, (_e2626 & 4294967294u), 0u, 0u)), _e1623, _e6972);
                                phi_19496_ = type_73();
                                phi_19497_ = true;
                            } else {
                                let _e5956 = ((_e5951 * 747796405u) + 2891336453u);
                                let _e5964 = (((_e5956 >> bitcast<u32>((((_e5956 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5956) * 277803737u);
                                let _e5967 = ((_e5964 >> bitcast<u32>(22i)) ^ _e5964);
                                let _e5970 = f32((_e5967 >> bitcast<u32>(8i)));
                                let _e5973 = ((_e5967 * 747796405u) + 2891336453u);
                                let _e5981 = (((_e5973 >> bitcast<u32>((((_e5973 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5973) * 277803737u);
                                let _e5984 = ((_e5981 >> bitcast<u32>(22i)) ^ _e5981);
                                let _e5987 = f32((_e5984 >> bitcast<u32>(8i)));
                                let _e5989 = ((_e5984 * 747796405u) + 2891336453u);
                                let _e5997 = (((_e5989 >> bitcast<u32>((((_e5989 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5989) * 277803737u);
                                let _e6000 = ((_e5997 >> bitcast<u32>(22i)) ^ _e5997);
                                let _e6005 = (_e2618 == 0u);
                                if _e6005 {
                                    phi_30429_ = vec3<f32>();
                                    phi_30430_ = true;
                                } else {
                                    if (_e2618 == 2u) {
                                        let _e6011 = (((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z));
                                        if (_e6011 != _e6011) {
                                            phi_30470_ = true;
                                        } else {
                                            phi_30470_ = (0f >= _e6011);
                                        }
                                        let _e6015 = phi_30470_;
                                        let _e6016 = select(_e6011, 0f, _e6015);
                                        let _e6017 = (1f / _e2608);
                                        let _e6021 = ((_e6017 * _e6017) * (1f - (_e6016 * _e6016)));
                                        if (_e6021 >= 1f) {
                                            phi_30489_ = 1f;
                                        } else {
                                            let _e6024 = sqrt((1f - _e6021));
                                            if (_e6016 != _e6016) {
                                                phi_30500_ = true;
                                            } else {
                                                phi_30500_ = (_e6024 <= _e6016);
                                            }
                                            let _e6028 = phi_30500_;
                                            let _e6033 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                            phi_30489_ = (_e6033 + ((1f - _e6033) * pow((1f - select(_e6016, _e6024, _e6028)), 5f)));
                                        }
                                        let _e6040 = phi_30489_;
                                        phi_30420_ = select(true, false, ((f32((_e6000 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6040));
                                    } else {
                                        phi_30420_ = true;
                                    }
                                    let _e6044 = phi_30420_;
                                    if _e6044 {
                                        let _e6049 = (((_e1698 * _e1774) + (_e1699 * _e1775)) + (_e1700 * _e1777));
                                        let _e6054 = (((_e1698 * _e1770) + (_e1699 * _e1780)) + (_e1700 * _e1781));
                                        let _e6059 = (((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z));
                                        if (_e6059 <= 0f) {
                                            phi_30654_ = vec3<f32>();
                                            phi_30655_ = true;
                                        } else {
                                            let _e6062 = (_e2616 * _e6049);
                                            let _e6063 = (_e2616 * _e6054);
                                            let _e6070 = (1f / sqrt((((_e6062 * _e6062) + (_e6063 * _e6063)) + (_e6059 * _e6059))));
                                            let _e6071 = (_e6062 * _e6070);
                                            let _e6072 = (_e6063 * _e6070);
                                            let _e6073 = (_e6059 * _e6070);
                                            let _e6076 = ((_e6071 * _e6071) + (_e6072 * _e6072));
                                            if (_e6076 > 0f) {
                                                let _e6079 = sqrt(_e6076);
                                                phi_30556_ = vec3<f32>((-(_e6072) / _e6079), (_e6071 / _e6079), 0f);
                                            } else {
                                                phi_30556_ = vec3<f32>(1f, 0f, 0f);
                                            }
                                            let _e6084 = phi_30556_;
                                            let _e6097 = sqrt((_e5970 * 0.000000059604645f));
                                            let _e6098 = (_e5987 * 0.00000037450704f);
                                            let _e6100 = (_e6097 * cos(_e6098));
                                            let _e6102 = (0.5f * (1f + _e6073));
                                            let _e6105 = (1f - (_e6100 * _e6100));
                                            if (_e6105 != _e6105) {
                                                phi_30712_ = true;
                                            } else {
                                                phi_30712_ = (0f >= _e6105);
                                            }
                                            let _e6109 = phi_30712_;
                                            let _e6116 = (((1f - _e6102) * sqrt(select(_e6105, 0f, _e6109))) + ((_e6102 * _e6097) * sin(_e6098)));
                                            let _e6127 = (_e6105 - (_e6116 * _e6116));
                                            if (_e6127 != _e6127) {
                                                phi_30727_ = true;
                                            } else {
                                                phi_30727_ = (0f >= _e6127);
                                            }
                                            let _e6131 = phi_30727_;
                                            let _e6133 = sqrt(select(_e6127, 0f, _e6131));
                                            let _e6139 = (((_e6084.z * _e6100) + (((_e6071 * _e6084.y) - (_e6084.x * _e6072)) * _e6116)) + (_e6073 * _e6133));
                                            let _e6140 = (_e2616 * (((_e6084.x * _e6100) + (((_e6072 * _e6084.z) - (_e6084.y * _e6073)) * _e6116)) + (_e6071 * _e6133)));
                                            let _e6141 = (_e2616 * (((_e6084.y * _e6100) + (((_e6073 * _e6084.x) - (_e6084.z * _e6071)) * _e6116)) + (_e6072 * _e6133)));
                                            if (_e6139 != _e6139) {
                                                phi_30742_ = true;
                                            } else {
                                                phi_30742_ = (0f >= _e6139);
                                            }
                                            let _e6145 = phi_30742_;
                                            let _e6146 = select(_e6139, 0f, _e6145);
                                            let _e6153 = (1f / sqrt((((_e6140 * _e6140) + (_e6141 * _e6141)) + (_e6146 * _e6146))));
                                            let _e6154 = (_e6140 * _e6153);
                                            let _e6155 = (_e6141 * _e6153);
                                            let _e6156 = (_e6146 * _e6153);
                                            let _e6162 = (2f * (((_e6049 * _e6154) + (_e6054 * _e6155)) + (_e6059 * _e6156)));
                                            let _e6165 = (_e6156 * _e6162);
                                            let _e6168 = (vec3<f32>((_e6154 * _e6162), (_e6155 * _e6162), _e6165) - vec3<f32>(_e6049, _e6054, _e6059));
                                            let _e6169 = ((_e6165 - _e6059) <= 0f);
                                            if _e6169 {
                                                phi_30651_ = vec3<f32>();
                                            } else {
                                                phi_30651_ = vec3<f32>((((_e1774 * _e6168.x) + (_e1770 * _e6168.y)) + (_e1726.x * _e6168.z)), (((_e1775 * _e6168.x) + (_e1780 * _e6168.y)) + (_e1726.y * _e6168.z)), (((_e1777 * _e6168.x) + (_e1781 * _e6168.y)) + (_e1726.z * _e6168.z)));
                                            }
                                            let _e6190 = phi_30651_;
                                            phi_30654_ = _e6190;
                                            phi_30655_ = _e6169;
                                        }
                                        let _e6192 = phi_30654_;
                                        let _e6194 = phi_30655_;
                                        phi_30425_ = select(_e6192, vec3<f32>(0f, 0f, 0f), vec3(_e6194));
                                    } else {
                                        phi_30425_ = vec3<f32>();
                                    }
                                    let _e6198 = phi_30425_;
                                    phi_30429_ = _e6198;
                                    phi_30430_ = select(true, false, _e6044);
                                }
                                let _e6201 = phi_30429_;
                                let _e6203 = phi_30430_;
                                if _e6203 {
                                    let _e6204 = (_e5970 * 0.00000037450704f);
                                    let _e6206 = ((_e5987 * 0.00000011920929f) - 1f);
                                    let _e6208 = (1f - (_e6206 * _e6206));
                                    if (_e6208 != _e6208) {
                                        phi_30842_ = true;
                                    } else {
                                        phi_30842_ = (0f >= _e6208);
                                    }
                                    let _e6212 = phi_30842_;
                                    let _e6214 = sqrt(select(_e6208, 0f, _e6212));
                                    let _e6219 = (_e1726.x + (_e6214 * cos(_e6204)));
                                    let _e6220 = (_e1726.y + (_e6214 * sin(_e6204)));
                                    let _e6221 = (_e1726.z + _e6206);
                                    let _e6227 = sqrt((((_e6219 * _e6219) + (_e6220 * _e6220)) + (_e6221 * _e6221)));
                                    if (_e6227 < 0.000001f) {
                                        phi_30816_ = _e1726;
                                    } else {
                                        let _e6229 = (1f / _e6227);
                                        phi_30816_ = vec3<f32>((_e6219 * _e6229), (_e6220 * _e6229), (_e6221 * _e6229));
                                    }
                                    let _e6235 = phi_30816_;
                                    phi_30437_ = _e6235;
                                } else {
                                    phi_30437_ = _e6201;
                                }
                                let _e6237 = phi_30437_;
                                let _e6245 = (((_e6237.x * _e1726.x) + (_e6237.y * _e1726.y)) + (_e6237.z * _e1726.z));
                                if (_e6245 != _e6245) {
                                    phi_30857_ = true;
                                } else {
                                    phi_30857_ = (0f >= _e6245);
                                }
                                let _e6249 = phi_30857_;
                                let _e6251 = (select(_e6245, 0f, _e6249) * 0.31830987f);
                                if _e6005 {
                                    if (_e6245 <= 0f) {
                                        phi_31654_ = vec3<f32>(0f, 0f, 0f);
                                    } else {
                                        phi_31654_ = (_e2612 * (_e6245 * 0.31830987f));
                                    }
                                    let _e6726 = phi_31654_;
                                    phi_19336_ = type_60(_e6726, _e6251);
                                    phi_19337_ = type_60();
                                    phi_19338_ = true;
                                } else {
                                    let _e6256 = (((_e1698 * _e1774) + (_e1699 * _e1775)) + (_e1700 * _e1777));
                                    let _e6261 = (((_e1698 * _e1770) + (_e1699 * _e1780)) + (_e1700 * _e1781));
                                    let _e6266 = (((_e1698 * _e1726.x) + (_e1699 * _e1726.y)) + (_e1700 * _e1726.z));
                                    let _e6271 = (((_e6237.x * _e1774) + (_e6237.y * _e1775)) + (_e6237.z * _e1777));
                                    let _e6276 = (((_e6237.x * _e1770) + (_e6237.y * _e1780)) + (_e6237.z * _e1781));
                                    if (_e6266 <= 0f) {
                                        phi_19300_ = type_60();
                                        phi_19301_ = true;
                                    } else {
                                        let _e6278 = (_e6245 <= 0f);
                                        if _e6278 {
                                            phi_19299_ = type_60();
                                        } else {
                                            let _e6279 = (_e6256 + _e6271);
                                            let _e6280 = (_e6261 + _e6276);
                                            let _e6281 = (_e6266 + _e6245);
                                            let _e6288 = (1f / sqrt((((_e6279 * _e6279) + (_e6280 * _e6280)) + (_e6281 * _e6281))));
                                            let _e6291 = (_e6281 * _e6288);
                                            if (_e6291 <= 0f) {
                                                phi_30967_ = 0f;
                                            } else {
                                                let _e6293 = (_e2616 * _e2616);
                                                let _e6297 = ((((_e6293 - 1f) * _e6291) * _e6291) + 1f);
                                                phi_30967_ = (_e6293 / ((3.1415927f * _e6297) * _e6297));
                                            }
                                            let _e6302 = phi_30967_;
                                            let _e6303 = (_e6266 * _e6266);
                                            if (_e6303 >= 1f) {
                                                phi_30995_ = 0f;
                                            } else {
                                                phi_30995_ = ((sqrt((1f + (((_e2616 * _e2616) * ((_e6256 * _e6256) + (_e6261 * _e6261))) / _e6303))) - 1f) * 0.5f);
                                            }
                                            let _e6316 = phi_30995_;
                                            let _e6317 = (1f + _e6316);
                                            let _e6318 = (_e6245 * _e6245);
                                            if (_e6318 >= 1f) {
                                                phi_31023_ = 0f;
                                            } else {
                                                phi_31023_ = ((sqrt((1f + (((_e2616 * _e2616) * ((_e6271 * _e6271) + (_e6276 * _e6276))) / _e6318))) - 1f) * 0.5f);
                                            }
                                            let _e6331 = phi_31023_;
                                            let _e6338 = (((_e6256 * (_e6279 * _e6288)) + (_e6261 * (_e6280 * _e6288))) + (_e6266 * _e6291));
                                            if (_e2614 == 0u) {
                                                let _e6633 = (1f - _e6338);
                                                if (_e6633 != _e6633) {
                                                    phi_31526_ = true;
                                                } else {
                                                    phi_31526_ = (0f >= _e6633);
                                                }
                                                let _e6637 = phi_31526_;
                                                let _e6639 = pow(select(_e6633, 0f, _e6637), 5f);
                                                let _e6640 = (1f - _e6639);
                                                phi_19287_ = vec3<f32>(((_e2612.x * _e6640) + _e6639), ((_e2612.y * _e6640) + _e6639), ((_e2612.z * _e6640) + _e6639));
                                            } else {
                                                if (_e2614 == 1u) {
                                                    let _e6606 = (1f / _e2608);
                                                    let _e6610 = ((_e6606 * _e6606) * (1f - (_e6338 * _e6338)));
                                                    if (_e6610 >= 1f) {
                                                        phi_31487_ = 1f;
                                                    } else {
                                                        let _e6613 = sqrt((1f - _e6610));
                                                        if (_e6338 != _e6338) {
                                                            phi_31498_ = true;
                                                        } else {
                                                            phi_31498_ = (_e6613 <= _e6338);
                                                        }
                                                        let _e6617 = phi_31498_;
                                                        let _e6622 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                                        phi_31487_ = (_e6622 + ((1f - _e6622) * pow((1f - select(_e6338, _e6613, _e6617)), 5f)));
                                                    }
                                                    let _e6629 = phi_31487_;
                                                    phi_19286_ = vec3(_e6629);
                                                } else {
                                                    let _e6342 = select(_e6338, 0f, (_e6338 < 0f));
                                                    let _e6344 = select(_e6342, 1f, (_e6342 > 1f));
                                                    let _e6346 = (1f - (_e6344 * _e6344));
                                                    let _e6359 = ((_e2612.x * _e2612.x) - (_e2610.x * _e2610.x));
                                                    let _e6360 = ((_e2612.y * _e2612.y) - (_e2610.y * _e2610.y));
                                                    let _e6361 = ((_e2612.z * _e2612.z) - (_e2610.z * _e2610.z));
                                                    let _e6363 = (_e2612.x * (_e2610.x + _e2610.x));
                                                    let _e6365 = (_e2612.y * (_e2610.y + _e2610.y));
                                                    let _e6367 = (_e2612.z * (_e2610.z + _e2610.z));
                                                    let _e6374 = ((_e6359 * _e6359) + (_e6363 * _e6363));
                                                    let _e6375 = ((_e6360 * _e6360) + (_e6365 * _e6365));
                                                    let _e6376 = ((_e6361 * _e6361) + (_e6367 * _e6367));
                                                    let _e6381 = ((_e6346 * _e6359) / _e6374);
                                                    let _e6382 = ((_e6346 * _e6360) / _e6375);
                                                    let _e6383 = ((_e6346 * _e6361) / _e6376);
                                                    let _e6384 = (1f - _e6381);
                                                    let _e6385 = (1f - _e6382);
                                                    let _e6386 = (1f - _e6383);
                                                    let _e6388 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e6381, _e6382, _e6383));
                                                    let _e6389 = (_e6346 * _e6363);
                                                    let _e6390 = (_e6346 * _e6365);
                                                    let _e6391 = (_e6346 * _e6367);
                                                    let _e6396 = (vec3<f32>(_e6389, _e6390, _e6391) / vec3<f32>(_e6374, _e6375, _e6376));
                                                    let _e6409 = ((_e6388.x * _e6388.x) + (_e6396.x * _e6396.x));
                                                    let _e6410 = ((_e6388.y * _e6388.y) + (_e6396.y * _e6396.y));
                                                    let _e6411 = ((_e6388.z * _e6388.z) + (_e6396.z * _e6396.z));
                                                    if (_e6409 != _e6409) {
                                                        phi_31157_ = true;
                                                    } else {
                                                        phi_31157_ = (0f >= _e6409);
                                                    }
                                                    let _e6415 = phi_31157_;
                                                    let _e6417 = sqrt(select(_e6409, 0f, _e6415));
                                                    if (_e6410 != _e6410) {
                                                        phi_31172_ = true;
                                                    } else {
                                                        phi_31172_ = (0f >= _e6410);
                                                    }
                                                    let _e6421 = phi_31172_;
                                                    let _e6423 = sqrt(select(_e6410, 0f, _e6421));
                                                    if (_e6411 != _e6411) {
                                                        phi_31187_ = true;
                                                    } else {
                                                        phi_31187_ = (0f >= _e6411);
                                                    }
                                                    let _e6427 = phi_31187_;
                                                    let _e6429 = sqrt(select(_e6411, 0f, _e6427));
                                                    let _e6433 = ((_e6417 + _e6384) * 0.5f);
                                                    let _e6434 = ((_e6423 + _e6385) * 0.5f);
                                                    let _e6435 = ((_e6429 + _e6386) * 0.5f);
                                                    if (_e6433 != _e6433) {
                                                        phi_31214_ = true;
                                                    } else {
                                                        phi_31214_ = (0f >= _e6433);
                                                    }
                                                    let _e6439 = phi_31214_;
                                                    let _e6441 = sqrt(select(_e6433, 0f, _e6439));
                                                    if (_e6434 != _e6434) {
                                                        phi_31229_ = true;
                                                    } else {
                                                        phi_31229_ = (0f >= _e6434);
                                                    }
                                                    let _e6445 = phi_31229_;
                                                    let _e6447 = sqrt(select(_e6434, 0f, _e6445));
                                                    if (_e6435 != _e6435) {
                                                        phi_31244_ = true;
                                                    } else {
                                                        phi_31244_ = (0f >= _e6435);
                                                    }
                                                    let _e6451 = phi_31244_;
                                                    let _e6453 = sqrt(select(_e6435, 0f, _e6451));
                                                    let _e6454 = vec3<f32>(_e6441, _e6447, _e6453);
                                                    let _e6458 = ((_e6417 - _e6384) * 0.5f);
                                                    let _e6459 = ((_e6423 - _e6385) * 0.5f);
                                                    let _e6460 = ((_e6429 - _e6386) * 0.5f);
                                                    if (_e6458 != _e6458) {
                                                        phi_31271_ = true;
                                                    } else {
                                                        phi_31271_ = (0f >= _e6458);
                                                    }
                                                    let _e6464 = phi_31271_;
                                                    let _e6466 = sqrt(select(_e6458, 0f, _e6464));
                                                    if (_e6459 != _e6459) {
                                                        phi_31286_ = true;
                                                    } else {
                                                        phi_31286_ = (0f >= _e6459);
                                                    }
                                                    let _e6470 = phi_31286_;
                                                    let _e6472 = sqrt(select(_e6459, 0f, _e6470));
                                                    if (_e6460 != _e6460) {
                                                        phi_31301_ = true;
                                                    } else {
                                                        phi_31301_ = (0f >= _e6460);
                                                    }
                                                    let _e6476 = phi_31301_;
                                                    let _e6478 = sqrt(select(_e6460, 0f, _e6476));
                                                    if ((_e6389 / _e6374) < 0f) {
                                                        phi_19163_ = -(_e6466);
                                                    } else {
                                                        phi_19163_ = _e6466;
                                                    }
                                                    let _e6482 = phi_19163_;
                                                    if ((_e6390 / _e6375) < 0f) {
                                                        phi_19171_ = -(_e6472);
                                                    } else {
                                                        phi_19171_ = _e6472;
                                                    }
                                                    let _e6486 = phi_19171_;
                                                    if ((_e6391 / _e6376) < 0f) {
                                                        phi_19179_ = -(_e6478);
                                                    } else {
                                                        phi_19179_ = _e6478;
                                                    }
                                                    let _e6490 = phi_19179_;
                                                    let _e6494 = (_e2610.x * _e6344);
                                                    let _e6495 = (_e2610.y * _e6344);
                                                    let _e6496 = (_e2610.z * _e6344);
                                                    let _e6506 = vec3<f32>(((_e2612.x * _e6441) - (_e2610.x * _e6482)), ((_e2612.y * _e6447) - (_e2610.y * _e6486)), ((_e2612.z * _e6453) - (_e2610.z * _e6490)));
                                                    let _e6513 = ((_e2612.x * _e6482) + (_e2610.x * _e6441));
                                                    let _e6514 = ((_e2612.y * _e6486) + (_e2610.y * _e6447));
                                                    let _e6515 = ((_e2612.z * _e6490) + (_e2610.z * _e6453));
                                                    let _e6517 = vec3<f32>((_e2612.x * _e6344), (_e2612.y * _e6344), (_e2612.z * _e6344));
                                                    let _e6518 = (_e6517 - _e6454);
                                                    let _e6519 = (_e6494 - _e6482);
                                                    let _e6520 = (_e6495 - _e6486);
                                                    let _e6521 = (_e6496 - _e6490);
                                                    let _e6534 = (_e6517 + _e6454);
                                                    let _e6535 = (_e6494 + _e6482);
                                                    let _e6536 = (_e6495 + _e6486);
                                                    let _e6537 = (_e6496 + _e6490);
                                                    let _e6547 = ((_e6534.x * _e6534.x) + (_e6535 * _e6535));
                                                    let _e6548 = ((_e6534.y * _e6534.y) + (_e6536 * _e6536));
                                                    let _e6549 = ((_e6534.z * _e6534.z) + (_e6537 * _e6537));
                                                    let _e6559 = vec3(_e6344);
                                                    let _e6560 = (_e6559 - _e6506);
                                                    let _e6561 = -(vec3<f32>(_e6513, _e6514, _e6515));
                                                    let _e6577 = (_e6559 + _e6506);
                                                    let _e6587 = ((_e6577.x * _e6577.x) + (_e6513 * _e6513));
                                                    let _e6588 = ((_e6577.y * _e6577.y) + (_e6514 * _e6514));
                                                    let _e6589 = ((_e6577.z * _e6577.z) + (_e6515 * _e6515));
                                                    phi_19286_ = vec3<f32>((((((_e6518.x * _e6518.x) + (_e6519 * _e6519)) / select(0.000000000000000000000000000001f, _e6547, (_e6547 > 0.000000000000000000000000000001f))) + (((_e6560.x * _e6560.x) + (_e6561.x * _e6561.x)) / select(0.000000000000000000000000000001f, _e6587, (_e6587 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6518.y * _e6518.y) + (_e6520 * _e6520)) / select(0.000000000000000000000000000001f, _e6548, (_e6548 > 0.000000000000000000000000000001f))) + (((_e6560.y * _e6560.y) + (_e6561.y * _e6561.y)) / select(0.000000000000000000000000000001f, _e6588, (_e6588 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6518.z * _e6518.z) + (_e6521 * _e6521)) / select(0.000000000000000000000000000001f, _e6549, (_e6549 > 0.000000000000000000000000000001f))) + (((_e6560.z * _e6560.z) + (_e6561.z * _e6561.z)) / select(0.000000000000000000000000000001f, _e6589, (_e6589 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e6632 = phi_19286_;
                                                phi_19287_ = _e6632;
                                            }
                                            let _e6652 = phi_19287_;
                                            let _e6654 = (4f * _e6266);
                                            phi_19299_ = type_60((_e6652 * (((1f / (_e6317 + _e6331)) * _e6302) / _e6654)), ((_e6302 / _e6317) / _e6654));
                                        }
                                        let _e6661 = phi_19299_;
                                        phi_19300_ = _e6661;
                                        phi_19301_ = _e6278;
                                    }
                                    let _e6663 = phi_19300_;
                                    let _e6665 = phi_19301_;
                                    if _e6665 {
                                        phi_19306_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
                                    } else {
                                        phi_19306_ = _e6663;
                                    }
                                    let _e6667 = phi_19306_;
                                    let _e6670 = (_e2618 == 1u);
                                    if _e6670 {
                                        phi_19334_ = _e6667;
                                        phi_19335_ = type_60();
                                    } else {
                                        if (_e6266 != _e6266) {
                                            phi_31563_ = true;
                                        } else {
                                            phi_31563_ = (0f >= _e6266);
                                        }
                                        let _e6674 = phi_31563_;
                                        let _e6675 = select(_e6266, 0f, _e6674);
                                        let _e6676 = (1f / _e2608);
                                        let _e6680 = ((_e6676 * _e6676) * (1f - (_e6675 * _e6675)));
                                        if (_e6680 >= 1f) {
                                            phi_31582_ = 1f;
                                        } else {
                                            let _e6683 = sqrt((1f - _e6680));
                                            if (_e6675 != _e6675) {
                                                phi_31593_ = true;
                                            } else {
                                                phi_31593_ = (_e6683 <= _e6675);
                                            }
                                            let _e6687 = phi_31593_;
                                            let _e6692 = pow(((1f - _e2608) / (1f + _e2608)), 2f);
                                            phi_31582_ = (_e6692 + ((1f - _e6692) * pow((1f - select(_e6675, _e6683, _e6687)), 5f)));
                                        }
                                        let _e6699 = phi_31582_;
                                        if (_e6245 <= 0f) {
                                            phi_31632_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_31632_ = (_e2606 * (_e6245 * 0.31830987f));
                                        }
                                        let _e6704 = phi_31632_;
                                        let _e6705 = (1f - _e6699);
                                        phi_19334_ = type_60();
                                        phi_19335_ = type_60((_e6667.member + vec3<f32>((_e6704.x * _e6705), (_e6704.y * _e6705), (_e6704.z * _e6705))), ((_e6699 * _e6667.member_1) + (_e6705 * _e6251)));
                                    }
                                    let _e6719 = phi_19334_;
                                    let _e6721 = phi_19335_;
                                    phi_19336_ = _e6719;
                                    phi_19337_ = _e6721;
                                    phi_19338_ = _e6670;
                                }
                                let _e6729 = phi_19336_;
                                let _e6731 = phi_19337_;
                                let _e6733 = phi_19338_;
                                if _e6733 {
                                    phi_19342_ = _e6729;
                                } else {
                                    phi_19342_ = _e6731;
                                }
                                let _e6735 = phi_19342_;
                                if ((((_e6237.x * _e6237.x) + (_e6237.y * _e6237.y)) + (_e6237.z * _e6237.z)) == 0f) {
                                    phi_37585_ = _e6000;
                                    phi_19481_ = type_73();
                                    phi_19482_ = type_73();
                                    phi_19483_ = true;
                                    phi_19484_ = false;
                                } else {
                                    let _e6744 = (_e6735.member_1 <= 0f);
                                    if _e6744 {
                                        phi_37586_ = _e6000;
                                        phi_19478_ = type_73();
                                        phi_19479_ = type_73();
                                        phi_19480_ = false;
                                    } else {
                                        let _e6746 = (_e6735.member.x / _e6735.member_1);
                                        let _e6748 = (_e6735.member.y / _e6735.member_1);
                                        let _e6750 = (_e6735.member.z / _e6735.member_1);
                                        let _e6759 = global_1.member.member_4[1u];
                                        if (sqrt((((_e6746 * _e6746) + (_e6748 * _e6748)) + (_e6750 * _e6750))) <= _e6759) {
                                            phi_37579_ = _e6000;
                                            phi_19429_ = _e1621;
                                            phi_19430_ = false;
                                        } else {
                                            let _e6763 = global_1.member.member_7[1u];
                                            if (_e6763 == 1u) {
                                                let _e6768 = global_1.member.member_4[3u];
                                                if (f32(_e358) >= _e6768) {
                                                    let _e6771 = select(_e6748, _e6746, (_e6746 > _e6748));
                                                    let _e6773 = select(_e6750, _e6771, (_e6771 > _e6750));
                                                    if (_e6773 != _e6773) {
                                                        phi_31679_ = true;
                                                    } else {
                                                        phi_31679_ = (1f <= _e6773);
                                                    }
                                                    let _e6777 = phi_31679_;
                                                    let _e6778 = select(_e6773, 1f, _e6777);
                                                    let _e6780 = ((_e6000 * 747796405u) + 2891336453u);
                                                    let _e6788 = (((_e6780 >> bitcast<u32>((((_e6780 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6780) * 277803737u);
                                                    let _e6791 = ((_e6788 >> bitcast<u32>(22i)) ^ _e6788);
                                                    let _e6796 = ((f32((_e6791 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6778);
                                                    if _e6796 {
                                                        phi_19418_ = vec3<f32>();
                                                    } else {
                                                        phi_19418_ = vec3<f32>((_e1621.x / _e6778), (_e1621.y / _e6778), (_e1621.z / _e6778));
                                                    }
                                                    let _e6805 = phi_19418_;
                                                    phi_37582_ = _e6791;
                                                    phi_19420_ = _e6805;
                                                    phi_19421_ = select(true, false, _e6796);
                                                } else {
                                                    phi_37582_ = _e6000;
                                                    phi_19420_ = _e1621;
                                                    phi_19421_ = true;
                                                }
                                                let _e6808 = phi_37582_;
                                                let _e6810 = phi_19420_;
                                                let _e6812 = phi_19421_;
                                                phi_37581_ = _e6808;
                                                phi_19422_ = _e6810;
                                                phi_19423_ = _e6812;
                                            } else {
                                                phi_37581_ = _e6000;
                                                phi_19422_ = _e1621;
                                                phi_19423_ = true;
                                            }
                                            let _e6814 = phi_37581_;
                                            let _e6816 = phi_19422_;
                                            let _e6818 = phi_19423_;
                                            phi_37579_ = _e6814;
                                            phi_19429_ = select(_e1621, _e6816, vec3(_e6818));
                                            phi_19430_ = select(false, true, _e6818);
                                        }
                                        let _e6823 = phi_37579_;
                                        let _e6825 = phi_19429_;
                                        let _e6827 = phi_19430_;
                                        if _e6827 {
                                            if ((((_e6237.x * _e1713.x) + (_e6237.y * _e1713.y)) + (_e6237.z * _e1713.z)) < 0f) {
                                                phi_19459_ = -(_e1713);
                                            } else {
                                                phi_19459_ = _e1713;
                                            }
                                            let _e6866 = phi_19459_;
                                            let _e6883 = local_92;
                                            let _e6886 = local_93;
                                            let _e6889 = local_94;
                                            let _e6892 = local_95;
                                            let _e6898 = local_96;
                                            phi_19475_ = type_73(type_35(vec4<f32>((_e1628 + (_e6866.x * _e1744)), (_e1629 + (_e6866.y * _e1744)), (_e1630 + (_e6866.z * _e1744)), _e334.member.w), vec4<f32>(_e6237.x, _e6237.y, _e6237.z, _e6735.member_1), vec4<f32>((_e6825.x * _e6746), (_e6825.y * _e6748), (_e6825.z * _e6750), f32((_e358 + 1u))), vec4<f32>(_e6886.x, _e6889.y, _e6892.z, f32(_e6883)), vec4<u32>(_e6823, _e2626, 0u, 0u)), _e1623, _e6898);
                                            phi_19476_ = type_73();
                                        } else {
                                            let _e6835 = local_87;
                                            let _e6838 = local_88;
                                            let _e6841 = local_89;
                                            let _e6844 = local_90;
                                            let _e6850 = local_91;
                                            phi_19475_ = type_73();
                                            phi_19476_ = type_73(type_35(_e334.member, _e334.member_1, vec4<f32>(_e6825.x, _e6825.y, _e6825.z, f32(_e358)), vec4<f32>(_e6838.x, _e6841.y, _e6844.z, f32(_e6835)), vec4<u32>(_e6823, (_e2626 & 4294967294u), 0u, 0u)), _e1623, _e6850);
                                        }
                                        let _e6901 = phi_19475_;
                                        let _e6903 = phi_19476_;
                                        phi_37586_ = _e6823;
                                        phi_19478_ = _e6901;
                                        phi_19479_ = _e6903;
                                        phi_19480_ = select(true, false, _e6827);
                                    }
                                    let _e6906 = phi_37586_;
                                    let _e6908 = phi_19478_;
                                    let _e6910 = phi_19479_;
                                    let _e6912 = phi_19480_;
                                    phi_37585_ = _e6906;
                                    phi_19481_ = _e6908;
                                    phi_19482_ = _e6910;
                                    phi_19483_ = _e6744;
                                    phi_19484_ = _e6912;
                                }
                                let _e6914 = phi_37585_;
                                let _e6916 = phi_19481_;
                                let _e6918 = phi_19482_;
                                let _e6920 = phi_19483_;
                                let _e6922 = phi_19484_;
                                if _e6920 {
                                    let _e6930 = local_97;
                                    let _e6933 = local_98;
                                    let _e6936 = local_99;
                                    let _e6939 = local_100;
                                    let _e6945 = local_101;
                                    phi_19490_ = type_73(type_35(_e334.member, _e334.member_1, vec4<f32>(_e1621.x, _e1621.y, _e1621.z, f32(_e358)), vec4<f32>(_e6933.x, _e6936.y, _e6939.z, f32(_e6930)), vec4<u32>(_e6914, (_e2626 & 4294967294u), 0u, 0u)), _e1623, _e6945);
                                } else {
                                    phi_19490_ = _e6918;
                                }
                                let _e6948 = phi_19490_;
                                phi_19495_ = _e6948;
                                phi_19496_ = _e6916;
                                phi_19497_ = select(_e6922, true, _e6920);
                            }
                            let _e6975 = phi_19495_;
                            let _e6977 = phi_19496_;
                            let _e6979 = phi_19497_;
                            phi_19501_ = _e6975;
                            phi_19502_ = _e6977;
                            phi_19503_ = _e6979;
                        } else {
                            phi_19501_ = _e2622;
                            phi_19502_ = type_73();
                            phi_19503_ = true;
                        }
                        let _e6981 = phi_19501_;
                        let _e6983 = phi_19502_;
                        let _e6985 = phi_19503_;
                        phi_19507_ = _e6981;
                        phi_19508_ = _e6983;
                        phi_19509_ = _e6985;
                    }
                    let _e7197 = phi_19507_;
                    let _e7199 = phi_19508_;
                    let _e7201 = phi_19509_;
                    phi_19513_ = _e7197;
                    phi_19514_ = _e7199;
                    phi_19515_ = _e7201;
                }
                let _e7709 = phi_19513_;
                let _e7711 = phi_19514_;
                let _e7713 = phi_19515_;
                if _e7713 {
                    phi_19519_ = _e7709;
                } else {
                    phi_19519_ = _e7711;
                }
                let _e7715 = phi_19519_;
                if _e331 {
                } else {
                    break;
                }
                global_7.member[_e326.member_1] = _e7715.member;
                let _e7719 = (2u * _e326.member_1);
                if (_e7719 < _e291) {
                } else {
                    break;
                }
                global_9.member[_e7719] = _e7715.member_1;
                let _e7723 = (_e7719 + 1u);
                if (_e7723 < _e291) {
                } else {
                    break;
                }
                global_9.member[_e7723] = _e7715.member_2;
                if ((_e7715.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7731 = (1u - _e295);
                    let _e7732 = (_e7731 * 4u);
                    let _e7736 = atomicAdd((&global_8.member[(_e7732 + 3u)]), 1u);
                    let _e7740 = global_1.member.member[0u];
                    let _e7745 = global_1.member.member[1u];
                    atomicStore((&global_8.member[((8u + ((_e7731 * _e7740) * _e7745)) + _e7736)]), _e326.member_1);
                    if ((_e7736 % 64u) == 0u) {
                        let _e7753 = (_e7736 / 64u);
                        if (_e7753 < 65535u) {
                            let _e7757 = atomicAdd((&global_8.member[_e7732]), 1u);
                        }
                        if ((_e7753 % 65535u) == 0u) {
                            let _e7763 = atomicAdd((&global_8.member[(_e7732 + 1u)]), 1u);
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
    var phi_20520_: type_49;
    var phi_20567_: bool;
    var phi_20568_: bool;
    var phi_20569_: bool;
    var phi_20585_: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e277 = global;
            let _e285 = (_e277.x + (_e277.y * 4194240u));
            let _e289 = global_1.member.member[0u];
            let _e293 = global_1.member.member[1u];
            if (_e285 >= (_e289 * _e293)) {
                phi_20520_ = type_49(0u, type_49().member_1);
            } else {
                phi_20520_ = type_49(1u, _e285);
            }
            let _e301 = phi_20520_;
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
                            phi_20567_ = select(true, false, (_e328 == _e328));
                        } else {
                            phi_20567_ = false;
                        }
                        let _e335 = phi_20567_;
                        phi_20568_ = select(true, false, _e331);
                        phi_20569_ = _e335;
                    } else {
                        phi_20568_ = true;
                        phi_20569_ = false;
                    }
                    let _e338 = phi_20568_;
                    let _e340 = phi_20569_;
                    let _e343 = select(vec3<f32>(_e326, _e327, _e328), vec3<f32>(0f, 0f, 0f), vec3(select(_e340, true, _e338)));
                    let _e348 = global_10.member[_e301.member_1].member_3[3u];
                    phi_20585_ = vec4<f32>(_e343.x, _e343.y, _e343.z, _e348);
                } else {
                    phi_20585_ = vec4<f32>(0f, 0f, 0f, 0f);
                }
                let _e354 = phi_20585_;
                if (_e354.w > 0f) {
                    if (_e301.member_1 < arrayLength((&global_11.member))) {
                    } else {
                        break;
                    }
                    let _e362 = global_11.member[_e301.member_1][0u];
                    global_11.member[_e301.member_1][0u] = (_e362 + _e354.x);
                    let _e368 = global_11.member[_e301.member_1][1u];
                    global_11.member[_e301.member_1][1u] = (_e368 + _e354.y);
                    let _e374 = global_11.member[_e301.member_1][2u];
                    global_11.member[_e301.member_1][2u] = (_e374 + _e354.z);
                    let _e379 = global_11.member[_e301.member_1][3u];
                    global_11.member[_e301.member_1][3u] = (_e379 + _e354.w);
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
fn add_paths(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}
