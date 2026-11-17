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
    var phi_32338_: bool;
    var phi_470_: vec3<f32>;
    var phi_473_: u32;
    var phi_20601_: f32;
    var phi_20613_: type_53;
    var phi_20624_: type_54;
    var phi_20625_: type_54;
    var phi_20629_: type_54;
    var phi_20630_: type_54;
    var phi_20631_: bool;
    var phi_20635_: type_54;
    var phi_20522_: type_54;
    var phi_20673_: bool;
    var phi_20753_: f32;
    var phi_20765_: type_53;
    var phi_20776_: type_54;
    var phi_20777_: type_54;
    var phi_20781_: type_54;
    var phi_20782_: type_54;
    var phi_20783_: bool;
    var phi_20787_: type_54;
    var phi_20721_: f32;
    var phi_20825_: bool;
    var phi_20550_: f32;
    var phi_20552_: f32;
    var phi_20561_: type_54;
    var phi_20565_: type_54;
    var phi_20569_: type_54;
    var phi_20840_: bool;
    var phi_9473_: vec3<f32>;
    var phi_9476_: vec3<f32>;
    var phi_9478_: vec3<f32>;
    var phi_9480_: vec3<f32>;
    var phi_9482_: u32;
    var phi_9566_: u32;
    var phi_9581_: type_53;
    var phi_9582_: type_53;
    var phi_9583_: type_53;
    var phi_21040_: bool;
    var phi_21167_: bool;
    var phi_21182_: bool;
    var phi_9911_: vec3<f32>;
    var phi_9984_: vec3<f32>;
    var phi_9985_: vec3<f32>;
    var phi_9986_: bool;
    var phi_9991_: vec3<f32>;
    var phi_9992_: bool;
    var phi_21306_: bool;
    var phi_21321_: bool;
    var phi_10283_: vec3<f32>;
    var phi_9474_: vec3<f32>;
    var phi_9477_: vec3<f32>;
    var phi_9479_: vec3<f32>;
    var phi_9481_: vec3<f32>;
    var phi_9483_: u32;
    var phi_32321_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_32805_: bool;
    var phi_10340_: type_57;
    var phi_32804_: bool;
    var phi_10342_: type_57;
    var phi_10343_: bool;
    var phi_10348_: type_57;
    var phi_849_: vec3<f32>;
    var phi_851_: vec3<f32>;
    var phi_852_: bool;
    var phi_32413_: bool;
    var phi_862_: vec3<f32>;
    var phi_865_: i32;
    var phi_867_: u32;
    var phi_21590_: bool;
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
    var phi_21701_: f32;
    var phi_21713_: type_53;
    var phi_21724_: type_54;
    var phi_21725_: type_54;
    var phi_21729_: type_54;
    var phi_21730_: type_54;
    var phi_21731_: bool;
    var phi_21735_: type_54;
    var phi_21622_: type_54;
    var phi_21773_: bool;
    var phi_21853_: f32;
    var phi_21865_: type_53;
    var phi_21876_: type_54;
    var phi_21877_: type_54;
    var phi_21881_: type_54;
    var phi_21882_: type_54;
    var phi_21883_: bool;
    var phi_21887_: type_54;
    var phi_21821_: f32;
    var phi_21925_: bool;
    var phi_21650_: f32;
    var phi_21652_: f32;
    var phi_21661_: type_54;
    var phi_21665_: type_54;
    var phi_21669_: type_54;
    var phi_2675_: vec3<f32>;
    var phi_2678_: vec3<f32>;
    var phi_2680_: vec3<f32>;
    var phi_2682_: vec3<f32>;
    var phi_2684_: u32;
    var phi_2768_: u32;
    var phi_2783_: type_53;
    var phi_2784_: type_53;
    var phi_2785_: type_53;
    var phi_22124_: bool;
    var phi_22251_: bool;
    var phi_22266_: bool;
    var phi_3113_: vec3<f32>;
    var phi_22388_: f32;
    var phi_22400_: type_53;
    var phi_22411_: type_54;
    var phi_22412_: type_54;
    var phi_22416_: type_54;
    var phi_22417_: type_54;
    var phi_22418_: bool;
    var phi_22422_: type_54;
    var phi_22356_: f32;
    var phi_22498_: f32;
    var phi_22510_: type_53;
    var phi_22521_: type_54;
    var phi_22522_: type_54;
    var phi_22526_: type_54;
    var phi_22527_: type_54;
    var phi_22528_: bool;
    var phi_22532_: type_54;
    var phi_22466_: f32;
    var phi_22293_: vec3<f32>;
    var phi_22294_: u32;
    var phi_22664_: bool;
    var phi_22327_: vec3<f32>;
    var phi_22328_: u32;
    var local_11: vec3<f32>;
    var phi_22335_: type_55;
    var phi_22339_: type_55;
    var phi_3172_: vec3<f32>;
    var phi_3173_: vec3<f32>;
    var phi_2676_: vec3<f32>;
    var phi_2679_: vec3<f32>;
    var phi_2681_: vec3<f32>;
    var phi_2683_: vec3<f32>;
    var phi_2685_: u32;
    var phi_32381_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_32475_: bool;
    var phi_3193_: type_49;
    var phi_32474_: bool;
    var phi_3194_: type_49;
    var phi_32473_: bool;
    var phi_3217_: vec3<f32>;
    var phi_3218_: i32;
    var phi_3219_: u32;
    var phi_32472_: bool;
    var phi_863_: vec3<f32>;
    var phi_866_: i32;
    var phi_868_: u32;
    var phi_3220_: bool;
    var phi_32471_: bool;
    var phi_3554_: vec3<f32>;
    var phi_3557_: u32;
    var phi_22792_: f32;
    var phi_22807_: f32;
    var phi_22822_: f32;
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
    var phi_32476_: bool;
    var local_15: vec3<f32>;
    var phi_32524_: bool;
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
    var phi_22938_: f32;
    var phi_22950_: type_53;
    var phi_22961_: type_54;
    var phi_22962_: type_54;
    var phi_22966_: type_54;
    var phi_22967_: type_54;
    var phi_22968_: bool;
    var phi_22972_: type_54;
    var phi_22906_: f32;
    var phi_4571_: bool;
    var phi_23098_: f32;
    var phi_23110_: type_53;
    var phi_23121_: type_54;
    var phi_23122_: type_54;
    var phi_23126_: type_54;
    var phi_23127_: type_54;
    var phi_23128_: bool;
    var phi_23132_: type_54;
    var phi_23019_: type_54;
    var phi_23170_: bool;
    var phi_23250_: f32;
    var phi_23262_: type_53;
    var phi_23273_: type_54;
    var phi_23274_: type_54;
    var phi_23278_: type_54;
    var phi_23279_: type_54;
    var phi_23280_: bool;
    var phi_23284_: type_54;
    var phi_23218_: f32;
    var phi_23322_: bool;
    var phi_23047_: f32;
    var phi_23049_: f32;
    var phi_23058_: type_54;
    var phi_23062_: type_54;
    var phi_23066_: type_54;
    var phi_4585_: vec3<f32>;
    var phi_4588_: u32;
    var phi_4672_: u32;
    var phi_4687_: type_53;
    var phi_4688_: type_53;
    var phi_4689_: type_53;
    var phi_23521_: bool;
    var phi_4586_: vec3<f32>;
    var phi_4589_: u32;
    var phi_32503_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_32570_: bool;
    var phi_4714_: vec3<f32>;
    var phi_32569_: bool;
    var phi_4716_: vec3<f32>;
    var phi_4718_: bool;
    var phi_32565_: bool;
    var phi_4729_: vec3<f32>;
    var phi_4730_: bool;
    var phi_32564_: bool;
    var phi_4732_: vec3<f32>;
    var phi_4733_: bool;
    var phi_4734_: bool;
    var phi_32563_: bool;
    var phi_4738_: vec3<f32>;
    var phi_4739_: vec3<f32>;
    var phi_4740_: bool;
    var phi_4741_: vec3<f32>;
    var phi_4742_: bool;
    var phi_5062_: vec3<f32>;
    var phi_5065_: u32;
    var phi_23592_: f32;
    var phi_23607_: f32;
    var phi_23622_: f32;
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
    var phi_32558_: bool;
    var phi_32600_: bool;
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
    var phi_23637_: vec3<f32>;
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
    var phi_23692_: f32;
    var phi_23698_: f32;
    var phi_23726_: f32;
    var phi_23732_: f32;
    var phi_23760_: f32;
    var phi_23766_: f32;
    var phi_23794_: f32;
    var phi_23800_: f32;
    var phi_23828_: f32;
    var phi_23834_: f32;
    var phi_23862_: f32;
    var phi_23868_: f32;
    var phi_23896_: f32;
    var phi_23902_: f32;
    var phi_23930_: f32;
    var phi_23936_: f32;
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
    var phi_24012_: bool;
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
    var phi_24101_: f32;
    var phi_24107_: f32;
    var phi_24135_: f32;
    var phi_24141_: f32;
    var phi_24169_: f32;
    var phi_24175_: f32;
    var phi_24203_: f32;
    var phi_24209_: f32;
    var phi_24237_: f32;
    var phi_24243_: f32;
    var phi_24271_: f32;
    var phi_24277_: f32;
    var phi_24305_: f32;
    var phi_24311_: f32;
    var phi_24339_: f32;
    var phi_24345_: f32;
    var phi_7142_: u32;
    var phi_7145_: f32;
    var phi_7147_: f32;
    var phi_7149_: f32;
    var phi_7151_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7564_: f32;
    var phi_24391_: bool;
    var phi_24406_: bool;
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
    var phi_24425_: vec3<f32>;
    var phi_8105_: u32;
    var phi_8106_: vec3<f32>;
    var phi_8107_: u32;
    var phi_8108_: vec3<f32>;
    var phi_8243_: u32;
    var phi_8246_: f32;
    var phi_8248_: f32;
    var phi_8250_: f32;
    var phi_8252_: vec3<f32>;
    var phi_24480_: f32;
    var phi_24486_: f32;
    var phi_24514_: f32;
    var phi_24520_: f32;
    var phi_24548_: f32;
    var phi_24554_: f32;
    var phi_24582_: f32;
    var phi_24588_: f32;
    var phi_24616_: f32;
    var phi_24622_: f32;
    var phi_24650_: f32;
    var phi_24656_: f32;
    var phi_24684_: f32;
    var phi_24690_: f32;
    var phi_24718_: f32;
    var phi_24724_: f32;
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
    var phi_32808_: bool;
    var phi_9077_: vec3<f32>;
    var phi_32807_: bool;
    var phi_9078_: vec3<f32>;
    var phi_32806_: bool;
    var phi_9079_: vec3<f32>;
    var phi_32802_: bool;
    var phi_10352_: vec3<f32>;
    var phi_32801_: bool;
    var phi_10353_: vec3<f32>;
    var phi_32800_: bool;
    var phi_471_: vec3<f32>;
    var phi_474_: u32;
    var phi_32799_: bool;
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
                phi_32338_ = false;
                phi_470_ = vec3<f32>(0f, 0f, 0f);
                phi_473_ = 0u;
                loop {
                    let _e306 = phi_32338_;
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
                            phi_32799_ = true;
                            break;
                        }
                        let _e317 = global_8.member[_e313];
                        if ((((_e317.member_2.x * _e317.member_2.x) + (_e317.member_2.y * _e317.member_2.y)) + (_e317.member_2.z * _e317.member_2.z)) == 0f) {
                            phi_32801_ = _e306;
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
                                            phi_32799_ = true;
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
                                            phi_32799_ = true;
                                            break;
                                        }
                                        let _e1323 = (bitcast<u32>((select(0i, select(select(i32(_e1308), i32(-2147483648), (_e1308 < -2147483600f)), 2147483647i, (_e1308 > 2147483500f)), (_e1308 == _e1308)) + bitcast<i32>(_e1280))) % _e1280);
                                        let _e1328 = select(select(u32(_e1309), 0u, (_e1309 < 0f)), 4294967295u, (_e1309 > 4294967000f));
                                        let _e1329 = (_e1328 + 1u);
                                        let _e1334 = global_1.member.member_2[2u];
                                        let _e1335 = (_e1328 * _e1280);
                                        if _e1322 {
                                            phi_32799_ = true;
                                            break;
                                        }
                                        let _e1336 = (_e1323 % _e1280);
                                        let _e1339 = (_e1334 + (3u * (_e1335 + _e1336)));
                                        let _e1341 = global_6.member[_e1339];
                                        let _e1344 = global_6.member[(_e1339 + 1u)];
                                        let _e1347 = global_6.member[(_e1339 + 2u)];
                                        if _e1322 {
                                            phi_32799_ = true;
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
                                            phi_32799_ = true;
                                            break;
                                        }
                                        let _e1373 = (_e1334 + (3u * (_e1370 + _e1336)));
                                        let _e1375 = global_6.member[_e1373];
                                        let _e1378 = global_6.member[(_e1373 + 1u)];
                                        let _e1381 = global_6.member[(_e1373 + 2u)];
                                        if _e1322 {
                                            phi_32799_ = true;
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
                                        phi_32413_ = _e306;
                                        phi_862_ = vec3<f32>(0f, 0f, 0f);
                                        phi_865_ = 0i;
                                        phi_867_ = _e1272;
                                        loop {
                                            let _e1426 = phi_32413_;
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
                                                                    phi_21590_ = true;
                                                                } else {
                                                                    phi_21590_ = (_e1542 >= _e1535);
                                                                }
                                                                let _e1546 = phi_21590_;
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
                                                            phi_21729_ = type_54(0u, type_54().member_1);
                                                            phi_21730_ = type_54();
                                                            phi_21731_ = true;
                                                        } else {
                                                            if (_e2042 < 0f) {
                                                                phi_21701_ = -(sqrt(_e2057));
                                                            } else {
                                                                phi_21701_ = sqrt(_e2057);
                                                            }
                                                            let _e2064 = phi_21701_;
                                                            let _e2066 = -((_e2042 + _e2064));
                                                            let _e2067 = (_e2066 == 0f);
                                                            if _e2067 {
                                                                phi_21724_ = type_54(1u, type_53(0f, 0f));
                                                                phi_21725_ = type_54();
                                                            } else {
                                                                let _e2068 = (_e2055 / _e2066);
                                                                if (_e2066 < _e2068) {
                                                                    phi_21713_ = type_53(_e2066, _e2068);
                                                                } else {
                                                                    phi_21713_ = type_53(_e2068, _e2066);
                                                                }
                                                                let _e2073 = phi_21713_;
                                                                phi_21724_ = type_54();
                                                                phi_21725_ = type_54(1u, _e2073);
                                                            }
                                                            let _e2076 = phi_21724_;
                                                            let _e2078 = phi_21725_;
                                                            phi_21729_ = _e2076;
                                                            phi_21730_ = _e2078;
                                                            phi_21731_ = _e2067;
                                                        }
                                                        let _e2083 = phi_21729_;
                                                        let _e2085 = phi_21730_;
                                                        let _e2087 = phi_21731_;
                                                        if _e2087 {
                                                            phi_21735_ = _e2083;
                                                        } else {
                                                            phi_21735_ = _e2085;
                                                        }
                                                        let _e2089 = phi_21735_;
                                                        if (bitcast<i32>(_e2089.member) != 0i) {
                                                            phi_21622_ = type_54(0u, _e2089.member_1);
                                                        } else {
                                                            phi_21622_ = type_54(1u, type_54().member_1);
                                                        }
                                                        let _e2099 = phi_21622_;
                                                        if (bitcast<i32>(_e2099.member) != 0i) {
                                                            phi_21669_ = type_54(0u, type_54().member_1);
                                                        } else {
                                                            if (_e2099.member_1.member_1 <= 0f) {
                                                                phi_21665_ = type_54(0u, type_54().member_1);
                                                            } else {
                                                                if (_e2099.member_1.member != _e2099.member_1.member) {
                                                                    phi_21773_ = true;
                                                                } else {
                                                                    phi_21773_ = (0f >= _e2099.member_1.member);
                                                                }
                                                                let _e2111 = phi_21773_;
                                                                let _e2112 = select(_e2099.member_1.member, 0f, _e2111);
                                                                let _e2113 = (_e2054 + _e1451);
                                                                if (_e2113 > 0f) {
                                                                    let _e2115 = (_e2056 - _e2113);
                                                                    if (_e2115 < 0f) {
                                                                        phi_21881_ = type_54(0u, type_54().member_1);
                                                                        phi_21882_ = type_54();
                                                                        phi_21883_ = true;
                                                                    } else {
                                                                        if (_e2042 < 0f) {
                                                                            phi_21853_ = -(sqrt(_e2115));
                                                                        } else {
                                                                            phi_21853_ = sqrt(_e2115);
                                                                        }
                                                                        let _e2122 = phi_21853_;
                                                                        let _e2124 = -((_e2042 + _e2122));
                                                                        let _e2125 = (_e2124 == 0f);
                                                                        if _e2125 {
                                                                            phi_21876_ = type_54(1u, type_53(0f, 0f));
                                                                            phi_21877_ = type_54();
                                                                        } else {
                                                                            let _e2126 = (_e2113 / _e2124);
                                                                            if (_e2124 < _e2126) {
                                                                                phi_21865_ = type_53(_e2124, _e2126);
                                                                            } else {
                                                                                phi_21865_ = type_53(_e2126, _e2124);
                                                                            }
                                                                            let _e2131 = phi_21865_;
                                                                            phi_21876_ = type_54();
                                                                            phi_21877_ = type_54(1u, _e2131);
                                                                        }
                                                                        let _e2134 = phi_21876_;
                                                                        let _e2136 = phi_21877_;
                                                                        phi_21881_ = _e2134;
                                                                        phi_21882_ = _e2136;
                                                                        phi_21883_ = _e2125;
                                                                    }
                                                                    let _e2141 = phi_21881_;
                                                                    let _e2143 = phi_21882_;
                                                                    let _e2145 = phi_21883_;
                                                                    if _e2145 {
                                                                        phi_21887_ = _e2141;
                                                                    } else {
                                                                        phi_21887_ = _e2143;
                                                                    }
                                                                    let _e2147 = phi_21887_;
                                                                    if (bitcast<i32>(_e2147.member) != 0i) {
                                                                        phi_21821_ = select(_e2147.member_1.member_1, _e2147.member_1.member, (_e2147.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_21821_ = -1f;
                                                                    }
                                                                    let _e2158 = phi_21821_;
                                                                    if (_e2158 >= 0f) {
                                                                        if (_e2099.member_1.member_1 != _e2099.member_1.member_1) {
                                                                            phi_21925_ = true;
                                                                        } else {
                                                                            phi_21925_ = (_e2158 <= _e2099.member_1.member_1);
                                                                        }
                                                                        let _e2163 = phi_21925_;
                                                                        phi_21650_ = select(_e2099.member_1.member_1, _e2158, _e2163);
                                                                    } else {
                                                                        phi_21650_ = _e2099.member_1.member_1;
                                                                    }
                                                                    let _e2166 = phi_21650_;
                                                                    phi_21652_ = _e2166;
                                                                } else {
                                                                    phi_21652_ = _e2099.member_1.member_1;
                                                                }
                                                                let _e2168 = phi_21652_;
                                                                if (_e2168 <= _e2112) {
                                                                    phi_21661_ = type_54(0u, type_54().member_1);
                                                                } else {
                                                                    phi_21661_ = type_54(1u, type_53(_e2112, _e2168));
                                                                }
                                                                let _e2176 = phi_21661_;
                                                                phi_21665_ = _e2176;
                                                            }
                                                            let _e2181 = phi_21665_;
                                                            phi_21669_ = _e2181;
                                                        }
                                                        let _e2186 = phi_21669_;
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
                                                                        phi_32381_ = true;
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
                                                                        phi_22124_ = true;
                                                                    } else {
                                                                        phi_22124_ = (0f >= _e2341);
                                                                    }
                                                                    let _e2345 = phi_22124_;
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
                                                                            phi_22251_ = true;
                                                                        } else {
                                                                            phi_22251_ = (0f >= _e2337);
                                                                        }
                                                                        let _e2408 = phi_22251_;
                                                                        let _e2412 = sqrt((select(_e2337, 0f, _e2408) / (_e1448 - _e1445)));
                                                                        if (_e2412 != _e2412) {
                                                                            phi_22266_ = true;
                                                                        } else {
                                                                            phi_22266_ = (1f <= _e2412);
                                                                        }
                                                                        let _e2416 = phi_22266_;
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
                                                                        phi_22416_ = type_54(0u, type_54().member_1);
                                                                        phi_22417_ = type_54();
                                                                        phi_22418_ = true;
                                                                    } else {
                                                                        if (_e2535 < 0f) {
                                                                            phi_22388_ = -(sqrt(_e2537));
                                                                        } else {
                                                                            phi_22388_ = sqrt(_e2537);
                                                                        }
                                                                        let _e2544 = phi_22388_;
                                                                        let _e2546 = -((_e2535 + _e2544));
                                                                        let _e2547 = (_e2546 == 0f);
                                                                        if _e2547 {
                                                                            phi_22411_ = type_54(1u, type_53(0f, 0f));
                                                                            phi_22412_ = type_54();
                                                                        } else {
                                                                            let _e2548 = (_e2326 / _e2546);
                                                                            if (_e2546 < _e2548) {
                                                                                phi_22400_ = type_53(_e2546, _e2548);
                                                                            } else {
                                                                                phi_22400_ = type_53(_e2548, _e2546);
                                                                            }
                                                                            let _e2553 = phi_22400_;
                                                                            phi_22411_ = type_54();
                                                                            phi_22412_ = type_54(1u, _e2553);
                                                                        }
                                                                        let _e2556 = phi_22411_;
                                                                        let _e2558 = phi_22412_;
                                                                        phi_22416_ = _e2556;
                                                                        phi_22417_ = _e2558;
                                                                        phi_22418_ = _e2547;
                                                                    }
                                                                    let _e2563 = phi_22416_;
                                                                    let _e2565 = phi_22417_;
                                                                    let _e2567 = phi_22418_;
                                                                    if _e2567 {
                                                                        phi_22422_ = _e2563;
                                                                    } else {
                                                                        phi_22422_ = _e2565;
                                                                    }
                                                                    let _e2569 = phi_22422_;
                                                                    if (bitcast<i32>(_e2569.member) != 0i) {
                                                                        phi_22356_ = select(_e2569.member_1.member_1, _e2569.member_1.member, (_e2569.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22356_ = -1f;
                                                                    }
                                                                    let _e2580 = phi_22356_;
                                                                    if (_e2580 >= 0f) {
                                                                        phi_22339_ = type_55(0u, type_55().member_1);
                                                                    } else {
                                                                        let _e2582 = (_e2325 + _e1454);
                                                                        let _e2583 = (_e2536 - _e2582);
                                                                        if (_e2583 < 0f) {
                                                                            phi_22526_ = type_54(0u, type_54().member_1);
                                                                            phi_22527_ = type_54();
                                                                            phi_22528_ = true;
                                                                        } else {
                                                                            if (_e2535 < 0f) {
                                                                                phi_22498_ = -(sqrt(_e2583));
                                                                            } else {
                                                                                phi_22498_ = sqrt(_e2583);
                                                                            }
                                                                            let _e2590 = phi_22498_;
                                                                            let _e2592 = -((_e2535 + _e2590));
                                                                            let _e2593 = (_e2592 == 0f);
                                                                            if _e2593 {
                                                                                phi_22521_ = type_54(1u, type_53(0f, 0f));
                                                                                phi_22522_ = type_54();
                                                                            } else {
                                                                                let _e2594 = (_e2582 / _e2592);
                                                                                if (_e2592 < _e2594) {
                                                                                    phi_22510_ = type_53(_e2592, _e2594);
                                                                                } else {
                                                                                    phi_22510_ = type_53(_e2594, _e2592);
                                                                                }
                                                                                let _e2599 = phi_22510_;
                                                                                phi_22521_ = type_54();
                                                                                phi_22522_ = type_54(1u, _e2599);
                                                                            }
                                                                            let _e2602 = phi_22521_;
                                                                            let _e2604 = phi_22522_;
                                                                            phi_22526_ = _e2602;
                                                                            phi_22527_ = _e2604;
                                                                            phi_22528_ = _e2593;
                                                                        }
                                                                        let _e2609 = phi_22526_;
                                                                        let _e2611 = phi_22527_;
                                                                        let _e2613 = phi_22528_;
                                                                        if _e2613 {
                                                                            phi_22532_ = _e2609;
                                                                        } else {
                                                                            phi_22532_ = _e2611;
                                                                        }
                                                                        let _e2615 = phi_22532_;
                                                                        if (bitcast<i32>(_e2615.member) != 0i) {
                                                                            phi_22466_ = select(_e2615.member_1.member_1, _e2615.member_1.member, (_e2615.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22466_ = -1f;
                                                                        }
                                                                        let _e2626 = phi_22466_;
                                                                        if (_e2626 < 0f) {
                                                                            phi_22335_ = type_55(0u, type_55().member_1);
                                                                        } else {
                                                                            phi_22293_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_22294_ = 0u;
                                                                            loop {
                                                                                let _e2629 = phi_22293_;
                                                                                let _e2631 = phi_22294_;
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
                                                                                        phi_22664_ = true;
                                                                                    } else {
                                                                                        phi_22664_ = (0f >= _e2681);
                                                                                    }
                                                                                    let _e2685 = phi_22664_;
                                                                                    let _e2687 = -(_e2677);
                                                                                    phi_22327_ = (_e2629 + vec3<f32>((exp((_e2687 / _e1466)) * _e2645), (exp((_e2687 / _e1469)) * _e2645), (select(_e2681, 0f, _e2685) * _e2645)));
                                                                                    phi_22328_ = _e2635;
                                                                                } else {
                                                                                    phi_22327_ = vec3<f32>();
                                                                                    phi_22328_ = u32();
                                                                                }
                                                                                let _e2698 = phi_22327_;
                                                                                let _e2700 = phi_22328_;
                                                                                phi_22293_ = _e2698;
                                                                                phi_22294_ = _e2700;
                                                                                if !(_e2632) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2703 = local_11;
                                                                            phi_22335_ = type_55(1u, _e2703);
                                                                        }
                                                                        let _e2709 = phi_22335_;
                                                                        phi_22339_ = _e2709;
                                                                    }
                                                                    let _e2714 = phi_22339_;
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
                                                                phi_32381_ = _e1426;
                                                                if !(_e2204) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2783 = phi_32381_;
                                                            phi_32471_ = _e2783;
                                                            if _e2783 {
                                                                break;
                                                            }
                                                            let _e2785 = local_12;
                                                            let _e2788 = local_13;
                                                            let _e2791 = local_14;
                                                            phi_32475_ = _e2783;
                                                            phi_3193_ = type_49((_e2785 * vec3<f32>(_e1475, _e1478, _e1481)), (_e2788 * _e1485), _e2791);
                                                        } else {
                                                            phi_32475_ = _e1426;
                                                            phi_3193_ = type_49(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2794 = phi_32475_;
                                                        let _e2796 = phi_3193_;
                                                        phi_32474_ = _e2794;
                                                        phi_3194_ = _e2796;
                                                    } else {
                                                        phi_32474_ = _e1426;
                                                        phi_3194_ = _e2029;
                                                    }
                                                    let _e2798 = phi_32474_;
                                                    let _e2800 = phi_3194_;
                                                    let _e2808 = (((_e317.member_1.x * _e1457) + (_e317.member_1.y * _e1460)) + (_e317.member_1.z * _e1463));
                                                    let _e2810 = (1f + (_e2808 * _e2808));
                                                    let _e2811 = (0.059683103f * _e2810);
                                                    let _e2818 = (_e1488 * _e1488);
                                                    let _e2829 = ((0.119366206f * ((1f - _e2818) * _e2810)) / ((2f + _e2818) * pow(((1f + _e2818) - ((2f * _e1488) * _e2808)), 1.5f)));
                                                    phi_32473_ = _e2798;
                                                    phi_3217_ = vec3<f32>((_e1428.x + (((((_e2800.member.x * _e2811) + (_e2800.member_1.x * _e2829)) + _e2800.member_2.x) * _e1472) * 0.38317f)), (_e1428.y + (((((_e2800.member.y * _e2811) + (_e2800.member_1.y * _e2829)) + _e2800.member_2.y) * _e1472) * 0.38317f)), (_e1428.z + (((((_e2800.member.z * _e2811) + (_e2800.member_1.z * _e2829)) + _e2800.member_2.z) * _e1472) * 0.38317f)));
                                                    phi_3218_ = (_e1430 + 1i);
                                                    phi_3219_ = bitcast<u32>(_e1508);
                                                } else {
                                                    phi_32473_ = _e1426;
                                                    phi_3217_ = vec3<f32>();
                                                    phi_3218_ = i32();
                                                    phi_3219_ = u32();
                                                }
                                                let _e2860 = phi_32473_;
                                                let _e2862 = phi_3217_;
                                                let _e2864 = phi_3218_;
                                                let _e2866 = phi_3219_;
                                                phi_32472_ = _e2860;
                                                phi_863_ = _e2862;
                                                phi_866_ = _e2864;
                                                phi_868_ = _e2866;
                                                phi_3220_ = _e1434;
                                            } else {
                                                phi_32472_ = _e1426;
                                                phi_863_ = vec3<f32>();
                                                phi_866_ = i32();
                                                phi_868_ = u32();
                                                phi_3220_ = false;
                                            }
                                            let _e2868 = phi_32472_;
                                            let _e2870 = phi_863_;
                                            let _e2872 = phi_866_;
                                            let _e2874 = phi_868_;
                                            let _e2876 = phi_3220_;
                                            phi_32413_ = _e2868;
                                            phi_862_ = _e2870;
                                            phi_865_ = _e2872;
                                            phi_867_ = _e2874;
                                            phi_32471_ = _e2868;
                                            if !(_e2876) {
                                                break;
                                            }
                                        }
                                        let _e2879 = phi_32471_;
                                        phi_32799_ = _e2879;
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
                                                            phi_22792_ = _e317.member_1.x;
                                                        } else {
                                                            phi_22792_ = select(_e317.member_1.z, _e317.member_1.y, (_e2973 == 1u));
                                                        }
                                                        let _e2981 = phi_22792_;
                                                        let _e2982 = (_e2981 * select(-1f, 1f, ((_e2971 % 2u) == 0u)));
                                                        if (_e2982 > 0f) {
                                                            let _e2986 = ((_e2973 + 1u) % 3u);
                                                            if (_e2986 == 0u) {
                                                                phi_22807_ = _e317.member_1.x;
                                                            } else {
                                                                phi_22807_ = select(_e317.member_1.z, _e317.member_1.y, (_e2986 == 1u));
                                                            }
                                                            let _e2991 = phi_22807_;
                                                            let _e2992 = (_e2991 / _e2982);
                                                            let _e2994 = ((_e2973 + 2u) % 3u);
                                                            if (_e2994 == 0u) {
                                                                phi_22822_ = _e317.member_1.x;
                                                            } else {
                                                                phi_22822_ = select(_e317.member_1.z, _e317.member_1.y, (_e2994 == 1u));
                                                            }
                                                            let _e2999 = phi_22822_;
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
                                                                        phi_32476_ = true;
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
                                                                        phi_32476_ = true;
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
                                                    phi_32476_ = _e2879;
                                                    if !(_e2972) {
                                                        break;
                                                    }
                                                }
                                                let _e3104 = phi_32476_;
                                                phi_32799_ = _e3104;
                                                if _e3104 {
                                                    break;
                                                }
                                                let _e3106 = local_15;
                                                phi_32524_ = _e3104;
                                                phi_3907_ = (_e3106.x * (1f / ((6.2831855f * _e2956) * _e2956)));
                                            } else {
                                                phi_32524_ = _e2879;
                                                phi_3907_ = 0f;
                                            }
                                            let _e3110 = phi_32524_;
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
                                                phi_32565_ = _e3110;
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
                                                        phi_22966_ = type_54(0u, type_54().member_1);
                                                        phi_22967_ = type_54();
                                                        phi_22968_ = true;
                                                    } else {
                                                        if (_e3325 < 0f) {
                                                            phi_22938_ = -(sqrt(_e3327));
                                                        } else {
                                                            phi_22938_ = sqrt(_e3327);
                                                        }
                                                        let _e3334 = phi_22938_;
                                                        let _e3336 = -((_e3325 + _e3334));
                                                        let _e3337 = (_e3336 == 0f);
                                                        if _e3337 {
                                                            phi_22961_ = type_54(1u, type_53(0f, 0f));
                                                            phi_22962_ = type_54();
                                                        } else {
                                                            let _e3338 = (_e3316 / _e3336);
                                                            if (_e3336 < _e3338) {
                                                                phi_22950_ = type_53(_e3336, _e3338);
                                                            } else {
                                                                phi_22950_ = type_53(_e3338, _e3336);
                                                            }
                                                            let _e3343 = phi_22950_;
                                                            phi_22961_ = type_54();
                                                            phi_22962_ = type_54(1u, _e3343);
                                                        }
                                                        let _e3346 = phi_22961_;
                                                        let _e3348 = phi_22962_;
                                                        phi_22966_ = _e3346;
                                                        phi_22967_ = _e3348;
                                                        phi_22968_ = _e3337;
                                                    }
                                                    let _e3353 = phi_22966_;
                                                    let _e3355 = phi_22967_;
                                                    let _e3357 = phi_22968_;
                                                    if _e3357 {
                                                        phi_22972_ = _e3353;
                                                    } else {
                                                        phi_22972_ = _e3355;
                                                    }
                                                    let _e3359 = phi_22972_;
                                                    if (bitcast<i32>(_e3359.member) != 0i) {
                                                        phi_22906_ = select(_e3359.member_1.member_1, _e3359.member_1.member, (_e3359.member_1.member >= 0f));
                                                    } else {
                                                        phi_22906_ = -1f;
                                                    }
                                                    let _e3370 = phi_22906_;
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
                                                        phi_23126_ = type_54(0u, type_54().member_1);
                                                        phi_23127_ = type_54();
                                                        phi_23128_ = true;
                                                    } else {
                                                        if (_e3382 < 0f) {
                                                            phi_23098_ = -(sqrt(_e3385));
                                                        } else {
                                                            phi_23098_ = sqrt(_e3385);
                                                        }
                                                        let _e3392 = phi_23098_;
                                                        let _e3394 = -((_e3382 + _e3392));
                                                        let _e3395 = (_e3394 == 0f);
                                                        if _e3395 {
                                                            phi_23121_ = type_54(1u, type_53(0f, 0f));
                                                            phi_23122_ = type_54();
                                                        } else {
                                                            let _e3396 = (_e3383 / _e3394);
                                                            if (_e3394 < _e3396) {
                                                                phi_23110_ = type_53(_e3394, _e3396);
                                                            } else {
                                                                phi_23110_ = type_53(_e3396, _e3394);
                                                            }
                                                            let _e3401 = phi_23110_;
                                                            phi_23121_ = type_54();
                                                            phi_23122_ = type_54(1u, _e3401);
                                                        }
                                                        let _e3404 = phi_23121_;
                                                        let _e3406 = phi_23122_;
                                                        phi_23126_ = _e3404;
                                                        phi_23127_ = _e3406;
                                                        phi_23128_ = _e3395;
                                                    }
                                                    let _e3411 = phi_23126_;
                                                    let _e3413 = phi_23127_;
                                                    let _e3415 = phi_23128_;
                                                    if _e3415 {
                                                        phi_23132_ = _e3411;
                                                    } else {
                                                        phi_23132_ = _e3413;
                                                    }
                                                    let _e3417 = phi_23132_;
                                                    if (bitcast<i32>(_e3417.member) != 0i) {
                                                        phi_23019_ = type_54(0u, _e3417.member_1);
                                                    } else {
                                                        phi_23019_ = type_54(1u, type_54().member_1);
                                                    }
                                                    let _e3427 = phi_23019_;
                                                    if (bitcast<i32>(_e3427.member) != 0i) {
                                                        phi_23066_ = type_54(0u, type_54().member_1);
                                                    } else {
                                                        if (_e3427.member_1.member_1 <= 0f) {
                                                            phi_23062_ = type_54(0u, type_54().member_1);
                                                        } else {
                                                            if (_e3427.member_1.member != _e3427.member_1.member) {
                                                                phi_23170_ = true;
                                                            } else {
                                                                phi_23170_ = (0f >= _e3427.member_1.member);
                                                            }
                                                            let _e3439 = phi_23170_;
                                                            let _e3440 = select(_e3427.member_1.member, 0f, _e3439);
                                                            if _e3317 {
                                                                let _e3441 = (_e3384 - _e3316);
                                                                if (_e3441 < 0f) {
                                                                    phi_23278_ = type_54(0u, type_54().member_1);
                                                                    phi_23279_ = type_54();
                                                                    phi_23280_ = true;
                                                                } else {
                                                                    if (_e3382 < 0f) {
                                                                        phi_23250_ = -(sqrt(_e3441));
                                                                    } else {
                                                                        phi_23250_ = sqrt(_e3441);
                                                                    }
                                                                    let _e3448 = phi_23250_;
                                                                    let _e3450 = -((_e3382 + _e3448));
                                                                    let _e3451 = (_e3450 == 0f);
                                                                    if _e3451 {
                                                                        phi_23273_ = type_54(1u, type_53(0f, 0f));
                                                                        phi_23274_ = type_54();
                                                                    } else {
                                                                        let _e3452 = (_e3316 / _e3450);
                                                                        if (_e3450 < _e3452) {
                                                                            phi_23262_ = type_53(_e3450, _e3452);
                                                                        } else {
                                                                            phi_23262_ = type_53(_e3452, _e3450);
                                                                        }
                                                                        let _e3457 = phi_23262_;
                                                                        phi_23273_ = type_54();
                                                                        phi_23274_ = type_54(1u, _e3457);
                                                                    }
                                                                    let _e3460 = phi_23273_;
                                                                    let _e3462 = phi_23274_;
                                                                    phi_23278_ = _e3460;
                                                                    phi_23279_ = _e3462;
                                                                    phi_23280_ = _e3451;
                                                                }
                                                                let _e3467 = phi_23278_;
                                                                let _e3469 = phi_23279_;
                                                                let _e3471 = phi_23280_;
                                                                if _e3471 {
                                                                    phi_23284_ = _e3467;
                                                                } else {
                                                                    phi_23284_ = _e3469;
                                                                }
                                                                let _e3473 = phi_23284_;
                                                                if (bitcast<i32>(_e3473.member) != 0i) {
                                                                    phi_23218_ = select(_e3473.member_1.member_1, _e3473.member_1.member, (_e3473.member_1.member >= 0f));
                                                                } else {
                                                                    phi_23218_ = -1f;
                                                                }
                                                                let _e3484 = phi_23218_;
                                                                if (_e3484 >= 0f) {
                                                                    if (_e3427.member_1.member_1 != _e3427.member_1.member_1) {
                                                                        phi_23322_ = true;
                                                                    } else {
                                                                        phi_23322_ = (_e3484 <= _e3427.member_1.member_1);
                                                                    }
                                                                    let _e3489 = phi_23322_;
                                                                    phi_23047_ = select(_e3427.member_1.member_1, _e3484, _e3489);
                                                                } else {
                                                                    phi_23047_ = _e3427.member_1.member_1;
                                                                }
                                                                let _e3492 = phi_23047_;
                                                                phi_23049_ = _e3492;
                                                            } else {
                                                                phi_23049_ = _e3427.member_1.member_1;
                                                            }
                                                            let _e3494 = phi_23049_;
                                                            if (_e3494 <= _e3440) {
                                                                phi_23058_ = type_54(0u, type_54().member_1);
                                                            } else {
                                                                phi_23058_ = type_54(1u, type_53(_e3440, _e3494));
                                                            }
                                                            let _e3502 = phi_23058_;
                                                            phi_23062_ = _e3502;
                                                        }
                                                        let _e3507 = phi_23062_;
                                                        phi_23066_ = _e3507;
                                                    }
                                                    let _e3512 = phi_23066_;
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
                                                                    phi_32503_ = true;
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
                                                                    phi_23521_ = true;
                                                                } else {
                                                                    phi_23521_ = (0f >= _e3660);
                                                                }
                                                                let _e3664 = phi_23521_;
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
                                                            phi_32503_ = _e3110;
                                                            if !(_e3524) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3683 = phi_32503_;
                                                        phi_32799_ = _e3683;
                                                        if _e3683 {
                                                            break;
                                                        }
                                                        let _e3685 = local_20;
                                                        let _e3692 = local_21;
                                                        let _e3694 = ((_e3294 * 1.1f) * _e3692.y);
                                                        let _e3699 = local_22;
                                                        phi_32570_ = _e3683;
                                                        phi_4714_ = vec3<f32>(exp(-((((_e3285 * _e3685.x) + _e3694) + (_e3297 * _e3699.z)))), exp(-((((_e3288 * _e3685.x) + _e3694) + (_e3300 * _e3699.z)))), exp(-((((_e3291 * _e3685.x) + _e3694) + (_e3303 * _e3699.z)))));
                                                    } else {
                                                        phi_32570_ = _e3110;
                                                        phi_4714_ = vec3<f32>();
                                                    }
                                                    let _e3715 = phi_32570_;
                                                    let _e3717 = phi_4714_;
                                                    phi_32569_ = _e3715;
                                                    phi_4716_ = _e3717;
                                                    phi_4718_ = select(true, false, _e3515);
                                                } else {
                                                    phi_32569_ = _e3110;
                                                    phi_4716_ = vec3<f32>();
                                                    phi_4718_ = true;
                                                }
                                                let _e3720 = phi_32569_;
                                                let _e3722 = phi_4716_;
                                                let _e3724 = phi_4718_;
                                                let _e3728 = select(_e3722, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3374)), vec3(_e3724));
                                                let _e3732 = select(_e3728.y, _e3728.x, (_e3728.x > _e3728.y));
                                                phi_32565_ = _e3720;
                                                phi_4729_ = _e3728;
                                                phi_4730_ = (select(_e3728.z, _e3732, (_e3732 > _e3728.z)) > 0f);
                                            }
                                            let _e3738 = phi_32565_;
                                            let _e3740 = phi_4729_;
                                            let _e3742 = phi_4730_;
                                            phi_32564_ = _e3738;
                                            phi_4732_ = _e3740;
                                            phi_4733_ = _e3742;
                                            phi_4734_ = select(true, false, _e3259);
                                        } else {
                                            phi_32564_ = _e2879;
                                            phi_4732_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4733_ = _e1423;
                                            phi_4734_ = true;
                                        }
                                        let _e3745 = phi_32564_;
                                        let _e3747 = phi_4732_;
                                        let _e3749 = phi_4733_;
                                        let _e3751 = phi_4734_;
                                        phi_32563_ = _e3745;
                                        let _e7013 = local_61;
                                        phi_4738_ = _e7013;
                                        phi_4739_ = _e3747;
                                        phi_4740_ = _e3749;
                                        let _e7018 = local_62;
                                        phi_4741_ = _e7018;
                                        phi_4742_ = _e3751;
                                    } else {
                                        phi_32563_ = _e306;
                                        phi_4738_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4739_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4740_ = _e1423;
                                        phi_4741_ = vec3<f32>();
                                        phi_4742_ = true;
                                    }
                                    let _e3753 = phi_32563_;
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
                                                            phi_23592_ = _e317.member_1.x;
                                                        } else {
                                                            phi_23592_ = select(_e317.member_1.z, _e317.member_1.y, (_e3860 == 1u));
                                                        }
                                                        let _e3868 = phi_23592_;
                                                        let _e3869 = (_e3868 * select(-1f, 1f, ((_e3858 % 2u) == 0u)));
                                                        if (_e3869 > 0f) {
                                                            let _e3873 = ((_e3860 + 1u) % 3u);
                                                            if (_e3873 == 0u) {
                                                                phi_23607_ = _e317.member_1.x;
                                                            } else {
                                                                phi_23607_ = select(_e317.member_1.z, _e317.member_1.y, (_e3873 == 1u));
                                                            }
                                                            let _e3878 = phi_23607_;
                                                            let _e3879 = (_e3878 / _e3869);
                                                            let _e3881 = ((_e3860 + 2u) % 3u);
                                                            if (_e3881 == 0u) {
                                                                phi_23622_ = _e317.member_1.x;
                                                            } else {
                                                                phi_23622_ = select(_e317.member_1.z, _e317.member_1.y, (_e3881 == 1u));
                                                            }
                                                            let _e3886 = phi_23622_;
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
                                                                        phi_32558_ = true;
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
                                                                        phi_32558_ = true;
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
                                                    phi_32558_ = _e3753;
                                                    if !(_e3859) {
                                                        break;
                                                    }
                                                }
                                                let _e4048 = phi_32558_;
                                                phi_32799_ = _e4048;
                                                if _e4048 {
                                                    break;
                                                }
                                                phi_32600_ = _e4048;
                                                let _e7055 = local_66;
                                                phi_5424_ = _e7055;
                                            } else {
                                                phi_32600_ = _e3753;
                                                phi_5424_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4050 = phi_32600_;
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
                                                                        phi_23637_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_23637_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4140 == 1u)));
                                                                    }
                                                                    let _e4172 = phi_23637_;
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
                                                                    phi_23698_ = ((_e4373 * _e4281) + (_e4376 * _e4282));
                                                                } else {
                                                                    if (_e4370 < 8u) {
                                                                        phi_23692_ = ((_e4373 * _e4281) + (_e4376 * _e4283));
                                                                    } else {
                                                                        phi_23692_ = ((_e4373 * _e4282) + (_e4376 * _e4283));
                                                                    }
                                                                    let _e4386 = phi_23692_;
                                                                    phi_23698_ = _e4386;
                                                                }
                                                                let _e4391 = phi_23698_;
                                                                let _e4394 = global_6.member[(_e4203 + _e4351)];
                                                                let _e4400 = (_e4281 - 1f);
                                                                let _e4401 = (select(select(u32(_e4394), 0u, (_e4394 < 0f)), 4294967295u, (_e4394 > 4294967000f)) % 12u);
                                                                let _e4404 = select(-1f, 1f, ((_e4401 & 1u) == 0u));
                                                                let _e4407 = select(-1f, 1f, ((_e4401 & 2u) == 0u));
                                                                if (_e4401 < 4u) {
                                                                    phi_23732_ = ((_e4404 * _e4400) + (_e4407 * _e4282));
                                                                } else {
                                                                    if (_e4401 < 8u) {
                                                                        phi_23726_ = ((_e4404 * _e4400) + (_e4407 * _e4283));
                                                                    } else {
                                                                        phi_23726_ = ((_e4404 * _e4282) + (_e4407 * _e4283));
                                                                    }
                                                                    let _e4417 = phi_23726_;
                                                                    phi_23732_ = _e4417;
                                                                }
                                                                let _e4422 = phi_23732_;
                                                                let _e4425 = global_6.member[(_e4203 + _e4332)];
                                                                let _e4431 = (_e4282 - 1f);
                                                                let _e4432 = (select(select(u32(_e4425), 0u, (_e4425 < 0f)), 4294967295u, (_e4425 > 4294967000f)) % 12u);
                                                                let _e4435 = select(-1f, 1f, ((_e4432 & 1u) == 0u));
                                                                let _e4438 = select(-1f, 1f, ((_e4432 & 2u) == 0u));
                                                                if (_e4432 < 4u) {
                                                                    phi_23766_ = ((_e4435 * _e4281) + (_e4438 * _e4431));
                                                                } else {
                                                                    if (_e4432 < 8u) {
                                                                        phi_23760_ = ((_e4435 * _e4281) + (_e4438 * _e4283));
                                                                    } else {
                                                                        phi_23760_ = ((_e4435 * _e4431) + (_e4438 * _e4283));
                                                                    }
                                                                    let _e4448 = phi_23760_;
                                                                    phi_23766_ = _e4448;
                                                                }
                                                                let _e4453 = phi_23766_;
                                                                let _e4456 = global_6.member[(_e4203 + _e4361)];
                                                                let _e4462 = (select(select(u32(_e4456), 0u, (_e4456 < 0f)), 4294967295u, (_e4456 > 4294967000f)) % 12u);
                                                                let _e4465 = select(-1f, 1f, ((_e4462 & 1u) == 0u));
                                                                let _e4468 = select(-1f, 1f, ((_e4462 & 2u) == 0u));
                                                                if (_e4462 < 4u) {
                                                                    phi_23800_ = ((_e4465 * _e4400) + (_e4468 * _e4431));
                                                                } else {
                                                                    if (_e4462 < 8u) {
                                                                        phi_23794_ = ((_e4465 * _e4400) + (_e4468 * _e4283));
                                                                    } else {
                                                                        phi_23794_ = ((_e4465 * _e4431) + (_e4468 * _e4283));
                                                                    }
                                                                    let _e4478 = phi_23794_;
                                                                    phi_23800_ = _e4478;
                                                                }
                                                                let _e4483 = phi_23800_;
                                                                let _e4487 = global_6.member[(_e4203 + (_e4322 + 1u))];
                                                                let _e4493 = (_e4283 - 1f);
                                                                let _e4494 = (select(select(u32(_e4487), 0u, (_e4487 < 0f)), 4294967295u, (_e4487 > 4294967000f)) % 12u);
                                                                let _e4497 = select(-1f, 1f, ((_e4494 & 1u) == 0u));
                                                                let _e4500 = select(-1f, 1f, ((_e4494 & 2u) == 0u));
                                                                if (_e4494 < 4u) {
                                                                    phi_23834_ = ((_e4497 * _e4281) + (_e4500 * _e4282));
                                                                } else {
                                                                    if (_e4494 < 8u) {
                                                                        phi_23828_ = ((_e4497 * _e4281) + (_e4500 * _e4493));
                                                                    } else {
                                                                        phi_23828_ = ((_e4497 * _e4282) + (_e4500 * _e4493));
                                                                    }
                                                                    let _e4510 = phi_23828_;
                                                                    phi_23834_ = _e4510;
                                                                }
                                                                let _e4515 = phi_23834_;
                                                                let _e4519 = global_6.member[(_e4203 + (_e4351 + 1u))];
                                                                let _e4525 = (select(select(u32(_e4519), 0u, (_e4519 < 0f)), 4294967295u, (_e4519 > 4294967000f)) % 12u);
                                                                let _e4528 = select(-1f, 1f, ((_e4525 & 1u) == 0u));
                                                                let _e4531 = select(-1f, 1f, ((_e4525 & 2u) == 0u));
                                                                if (_e4525 < 4u) {
                                                                    phi_23868_ = ((_e4528 * _e4400) + (_e4531 * _e4282));
                                                                } else {
                                                                    if (_e4525 < 8u) {
                                                                        phi_23862_ = ((_e4528 * _e4400) + (_e4531 * _e4493));
                                                                    } else {
                                                                        phi_23862_ = ((_e4528 * _e4282) + (_e4531 * _e4493));
                                                                    }
                                                                    let _e4541 = phi_23862_;
                                                                    phi_23868_ = _e4541;
                                                                }
                                                                let _e4546 = phi_23868_;
                                                                let _e4550 = global_6.member[(_e4203 + (_e4332 + 1u))];
                                                                let _e4556 = (select(select(u32(_e4550), 0u, (_e4550 < 0f)), 4294967295u, (_e4550 > 4294967000f)) % 12u);
                                                                let _e4559 = select(-1f, 1f, ((_e4556 & 1u) == 0u));
                                                                let _e4562 = select(-1f, 1f, ((_e4556 & 2u) == 0u));
                                                                if (_e4556 < 4u) {
                                                                    phi_23902_ = ((_e4559 * _e4281) + (_e4562 * _e4431));
                                                                } else {
                                                                    if (_e4556 < 8u) {
                                                                        phi_23896_ = ((_e4559 * _e4281) + (_e4562 * _e4493));
                                                                    } else {
                                                                        phi_23896_ = ((_e4559 * _e4431) + (_e4562 * _e4493));
                                                                    }
                                                                    let _e4572 = phi_23896_;
                                                                    phi_23902_ = _e4572;
                                                                }
                                                                let _e4577 = phi_23902_;
                                                                let _e4581 = global_6.member[(_e4203 + (_e4361 + 1u))];
                                                                let _e4587 = (select(select(u32(_e4581), 0u, (_e4581 < 0f)), 4294967295u, (_e4581 > 4294967000f)) % 12u);
                                                                let _e4590 = select(-1f, 1f, ((_e4587 & 1u) == 0u));
                                                                let _e4593 = select(-1f, 1f, ((_e4587 & 2u) == 0u));
                                                                if (_e4587 < 4u) {
                                                                    phi_23936_ = ((_e4590 * _e4400) + (_e4593 * _e4431));
                                                                } else {
                                                                    if (_e4587 < 8u) {
                                                                        phi_23930_ = ((_e4590 * _e4400) + (_e4593 * _e4493));
                                                                    } else {
                                                                        phi_23930_ = ((_e4590 * _e4431) + (_e4593 * _e4493));
                                                                    }
                                                                    let _e4603 = phi_23930_;
                                                                    phi_23936_ = _e4603;
                                                                }
                                                                let _e4608 = phi_23936_;
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
                                                        phi_24012_ = true;
                                                    } else {
                                                        let _e4838 = local_28;
                                                        phi_24012_ = (0.1f >= _e4838);
                                                    }
                                                    let _e4841 = phi_24012_;
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
                                                                        phi_24107_ = ((_e5249 * _e5157) + (_e5252 * _e5158));
                                                                    } else {
                                                                        if (_e5246 < 8u) {
                                                                            phi_24101_ = ((_e5249 * _e5157) + (_e5252 * _e5159));
                                                                        } else {
                                                                            phi_24101_ = ((_e5249 * _e5158) + (_e5252 * _e5159));
                                                                        }
                                                                        let _e5262 = phi_24101_;
                                                                        phi_24107_ = _e5262;
                                                                    }
                                                                    let _e5267 = phi_24107_;
                                                                    let _e5270 = global_6.member[(_e5082 + _e5227)];
                                                                    let _e5276 = (_e5157 - 1f);
                                                                    let _e5277 = (select(select(u32(_e5270), 0u, (_e5270 < 0f)), 4294967295u, (_e5270 > 4294967000f)) % 12u);
                                                                    let _e5280 = select(-1f, 1f, ((_e5277 & 1u) == 0u));
                                                                    let _e5283 = select(-1f, 1f, ((_e5277 & 2u) == 0u));
                                                                    if (_e5277 < 4u) {
                                                                        phi_24141_ = ((_e5280 * _e5276) + (_e5283 * _e5158));
                                                                    } else {
                                                                        if (_e5277 < 8u) {
                                                                            phi_24135_ = ((_e5280 * _e5276) + (_e5283 * _e5159));
                                                                        } else {
                                                                            phi_24135_ = ((_e5280 * _e5158) + (_e5283 * _e5159));
                                                                        }
                                                                        let _e5293 = phi_24135_;
                                                                        phi_24141_ = _e5293;
                                                                    }
                                                                    let _e5298 = phi_24141_;
                                                                    let _e5301 = global_6.member[(_e5082 + _e5208)];
                                                                    let _e5307 = (_e5158 - 1f);
                                                                    let _e5308 = (select(select(u32(_e5301), 0u, (_e5301 < 0f)), 4294967295u, (_e5301 > 4294967000f)) % 12u);
                                                                    let _e5311 = select(-1f, 1f, ((_e5308 & 1u) == 0u));
                                                                    let _e5314 = select(-1f, 1f, ((_e5308 & 2u) == 0u));
                                                                    if (_e5308 < 4u) {
                                                                        phi_24175_ = ((_e5311 * _e5157) + (_e5314 * _e5307));
                                                                    } else {
                                                                        if (_e5308 < 8u) {
                                                                            phi_24169_ = ((_e5311 * _e5157) + (_e5314 * _e5159));
                                                                        } else {
                                                                            phi_24169_ = ((_e5311 * _e5307) + (_e5314 * _e5159));
                                                                        }
                                                                        let _e5324 = phi_24169_;
                                                                        phi_24175_ = _e5324;
                                                                    }
                                                                    let _e5329 = phi_24175_;
                                                                    let _e5332 = global_6.member[(_e5082 + _e5237)];
                                                                    let _e5338 = (select(select(u32(_e5332), 0u, (_e5332 < 0f)), 4294967295u, (_e5332 > 4294967000f)) % 12u);
                                                                    let _e5341 = select(-1f, 1f, ((_e5338 & 1u) == 0u));
                                                                    let _e5344 = select(-1f, 1f, ((_e5338 & 2u) == 0u));
                                                                    if (_e5338 < 4u) {
                                                                        phi_24209_ = ((_e5341 * _e5276) + (_e5344 * _e5307));
                                                                    } else {
                                                                        if (_e5338 < 8u) {
                                                                            phi_24203_ = ((_e5341 * _e5276) + (_e5344 * _e5159));
                                                                        } else {
                                                                            phi_24203_ = ((_e5341 * _e5307) + (_e5344 * _e5159));
                                                                        }
                                                                        let _e5354 = phi_24203_;
                                                                        phi_24209_ = _e5354;
                                                                    }
                                                                    let _e5359 = phi_24209_;
                                                                    let _e5363 = global_6.member[(_e5082 + (_e5198 + 1u))];
                                                                    let _e5369 = (_e5159 - 1f);
                                                                    let _e5370 = (select(select(u32(_e5363), 0u, (_e5363 < 0f)), 4294967295u, (_e5363 > 4294967000f)) % 12u);
                                                                    let _e5373 = select(-1f, 1f, ((_e5370 & 1u) == 0u));
                                                                    let _e5376 = select(-1f, 1f, ((_e5370 & 2u) == 0u));
                                                                    if (_e5370 < 4u) {
                                                                        phi_24243_ = ((_e5373 * _e5157) + (_e5376 * _e5158));
                                                                    } else {
                                                                        if (_e5370 < 8u) {
                                                                            phi_24237_ = ((_e5373 * _e5157) + (_e5376 * _e5369));
                                                                        } else {
                                                                            phi_24237_ = ((_e5373 * _e5158) + (_e5376 * _e5369));
                                                                        }
                                                                        let _e5386 = phi_24237_;
                                                                        phi_24243_ = _e5386;
                                                                    }
                                                                    let _e5391 = phi_24243_;
                                                                    let _e5395 = global_6.member[(_e5082 + (_e5227 + 1u))];
                                                                    let _e5401 = (select(select(u32(_e5395), 0u, (_e5395 < 0f)), 4294967295u, (_e5395 > 4294967000f)) % 12u);
                                                                    let _e5404 = select(-1f, 1f, ((_e5401 & 1u) == 0u));
                                                                    let _e5407 = select(-1f, 1f, ((_e5401 & 2u) == 0u));
                                                                    if (_e5401 < 4u) {
                                                                        phi_24277_ = ((_e5404 * _e5276) + (_e5407 * _e5158));
                                                                    } else {
                                                                        if (_e5401 < 8u) {
                                                                            phi_24271_ = ((_e5404 * _e5276) + (_e5407 * _e5369));
                                                                        } else {
                                                                            phi_24271_ = ((_e5404 * _e5158) + (_e5407 * _e5369));
                                                                        }
                                                                        let _e5417 = phi_24271_;
                                                                        phi_24277_ = _e5417;
                                                                    }
                                                                    let _e5422 = phi_24277_;
                                                                    let _e5426 = global_6.member[(_e5082 + (_e5208 + 1u))];
                                                                    let _e5432 = (select(select(u32(_e5426), 0u, (_e5426 < 0f)), 4294967295u, (_e5426 > 4294967000f)) % 12u);
                                                                    let _e5435 = select(-1f, 1f, ((_e5432 & 1u) == 0u));
                                                                    let _e5438 = select(-1f, 1f, ((_e5432 & 2u) == 0u));
                                                                    if (_e5432 < 4u) {
                                                                        phi_24311_ = ((_e5435 * _e5157) + (_e5438 * _e5307));
                                                                    } else {
                                                                        if (_e5432 < 8u) {
                                                                            phi_24305_ = ((_e5435 * _e5157) + (_e5438 * _e5369));
                                                                        } else {
                                                                            phi_24305_ = ((_e5435 * _e5307) + (_e5438 * _e5369));
                                                                        }
                                                                        let _e5448 = phi_24305_;
                                                                        phi_24311_ = _e5448;
                                                                    }
                                                                    let _e5453 = phi_24311_;
                                                                    let _e5457 = global_6.member[(_e5082 + (_e5237 + 1u))];
                                                                    let _e5463 = (select(select(u32(_e5457), 0u, (_e5457 < 0f)), 4294967295u, (_e5457 > 4294967000f)) % 12u);
                                                                    let _e5466 = select(-1f, 1f, ((_e5463 & 1u) == 0u));
                                                                    let _e5469 = select(-1f, 1f, ((_e5463 & 2u) == 0u));
                                                                    if (_e5463 < 4u) {
                                                                        phi_24345_ = ((_e5466 * _e5276) + (_e5469 * _e5307));
                                                                    } else {
                                                                        if (_e5463 < 8u) {
                                                                            phi_24339_ = ((_e5466 * _e5276) + (_e5469 * _e5369));
                                                                        } else {
                                                                            phi_24339_ = ((_e5466 * _e5307) + (_e5469 * _e5369));
                                                                        }
                                                                        let _e5479 = phi_24339_;
                                                                        phi_24345_ = _e5479;
                                                                    }
                                                                    let _e5484 = phi_24345_;
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
                                                                phi_24391_ = true;
                                                            } else {
                                                                phi_24391_ = (0f >= _e5552);
                                                            }
                                                            let _e5556 = phi_24391_;
                                                            let _e5561 = ((_e5062 - 1.15f) * -2.8571432f);
                                                            let _e5563 = select(_e5561, 0f, (_e5561 < 0f));
                                                            let _e5565 = select(_e5563, 1f, (_e5563 > 1f));
                                                            let _e5571 = (1f / _e5074);
                                                            if (_e5571 != _e5571) {
                                                                phi_24406_ = true;
                                                            } else {
                                                                phi_24406_ = (4f <= _e5571);
                                                            }
                                                            let _e5575 = phi_24406_;
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
                                            phi_32799_ = _e4050;
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
                                                                                phi_24425_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_24425_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5840 == 1u)));
                                                                            }
                                                                            let _e5846 = phi_24425_;
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
                                                                                phi_24486_ = ((_e6030 * _e5938) + (_e6033 * _e5939));
                                                                            } else {
                                                                                if (_e6027 < 8u) {
                                                                                    phi_24480_ = ((_e6030 * _e5938) + (_e6033 * _e5940));
                                                                                } else {
                                                                                    phi_24480_ = ((_e6030 * _e5939) + (_e6033 * _e5940));
                                                                                }
                                                                                let _e6043 = phi_24480_;
                                                                                phi_24486_ = _e6043;
                                                                            }
                                                                            let _e6048 = phi_24486_;
                                                                            let _e6051 = global_6.member[(_e5863 + _e6008)];
                                                                            let _e6057 = (_e5938 - 1f);
                                                                            let _e6058 = (select(select(u32(_e6051), 0u, (_e6051 < 0f)), 4294967295u, (_e6051 > 4294967000f)) % 12u);
                                                                            let _e6061 = select(-1f, 1f, ((_e6058 & 1u) == 0u));
                                                                            let _e6064 = select(-1f, 1f, ((_e6058 & 2u) == 0u));
                                                                            if (_e6058 < 4u) {
                                                                                phi_24520_ = ((_e6061 * _e6057) + (_e6064 * _e5939));
                                                                            } else {
                                                                                if (_e6058 < 8u) {
                                                                                    phi_24514_ = ((_e6061 * _e6057) + (_e6064 * _e5940));
                                                                                } else {
                                                                                    phi_24514_ = ((_e6061 * _e5939) + (_e6064 * _e5940));
                                                                                }
                                                                                let _e6074 = phi_24514_;
                                                                                phi_24520_ = _e6074;
                                                                            }
                                                                            let _e6079 = phi_24520_;
                                                                            let _e6082 = global_6.member[(_e5863 + _e5989)];
                                                                            let _e6088 = (_e5939 - 1f);
                                                                            let _e6089 = (select(select(u32(_e6082), 0u, (_e6082 < 0f)), 4294967295u, (_e6082 > 4294967000f)) % 12u);
                                                                            let _e6092 = select(-1f, 1f, ((_e6089 & 1u) == 0u));
                                                                            let _e6095 = select(-1f, 1f, ((_e6089 & 2u) == 0u));
                                                                            if (_e6089 < 4u) {
                                                                                phi_24554_ = ((_e6092 * _e5938) + (_e6095 * _e6088));
                                                                            } else {
                                                                                if (_e6089 < 8u) {
                                                                                    phi_24548_ = ((_e6092 * _e5938) + (_e6095 * _e5940));
                                                                                } else {
                                                                                    phi_24548_ = ((_e6092 * _e6088) + (_e6095 * _e5940));
                                                                                }
                                                                                let _e6105 = phi_24548_;
                                                                                phi_24554_ = _e6105;
                                                                            }
                                                                            let _e6110 = phi_24554_;
                                                                            let _e6113 = global_6.member[(_e5863 + _e6018)];
                                                                            let _e6119 = (select(select(u32(_e6113), 0u, (_e6113 < 0f)), 4294967295u, (_e6113 > 4294967000f)) % 12u);
                                                                            let _e6122 = select(-1f, 1f, ((_e6119 & 1u) == 0u));
                                                                            let _e6125 = select(-1f, 1f, ((_e6119 & 2u) == 0u));
                                                                            if (_e6119 < 4u) {
                                                                                phi_24588_ = ((_e6122 * _e6057) + (_e6125 * _e6088));
                                                                            } else {
                                                                                if (_e6119 < 8u) {
                                                                                    phi_24582_ = ((_e6122 * _e6057) + (_e6125 * _e5940));
                                                                                } else {
                                                                                    phi_24582_ = ((_e6122 * _e6088) + (_e6125 * _e5940));
                                                                                }
                                                                                let _e6135 = phi_24582_;
                                                                                phi_24588_ = _e6135;
                                                                            }
                                                                            let _e6140 = phi_24588_;
                                                                            let _e6144 = global_6.member[(_e5863 + (_e5979 + 1u))];
                                                                            let _e6150 = (_e5940 - 1f);
                                                                            let _e6151 = (select(select(u32(_e6144), 0u, (_e6144 < 0f)), 4294967295u, (_e6144 > 4294967000f)) % 12u);
                                                                            let _e6154 = select(-1f, 1f, ((_e6151 & 1u) == 0u));
                                                                            let _e6157 = select(-1f, 1f, ((_e6151 & 2u) == 0u));
                                                                            if (_e6151 < 4u) {
                                                                                phi_24622_ = ((_e6154 * _e5938) + (_e6157 * _e5939));
                                                                            } else {
                                                                                if (_e6151 < 8u) {
                                                                                    phi_24616_ = ((_e6154 * _e5938) + (_e6157 * _e6150));
                                                                                } else {
                                                                                    phi_24616_ = ((_e6154 * _e5939) + (_e6157 * _e6150));
                                                                                }
                                                                                let _e6167 = phi_24616_;
                                                                                phi_24622_ = _e6167;
                                                                            }
                                                                            let _e6172 = phi_24622_;
                                                                            let _e6176 = global_6.member[(_e5863 + (_e6008 + 1u))];
                                                                            let _e6182 = (select(select(u32(_e6176), 0u, (_e6176 < 0f)), 4294967295u, (_e6176 > 4294967000f)) % 12u);
                                                                            let _e6185 = select(-1f, 1f, ((_e6182 & 1u) == 0u));
                                                                            let _e6188 = select(-1f, 1f, ((_e6182 & 2u) == 0u));
                                                                            if (_e6182 < 4u) {
                                                                                phi_24656_ = ((_e6185 * _e6057) + (_e6188 * _e5939));
                                                                            } else {
                                                                                if (_e6182 < 8u) {
                                                                                    phi_24650_ = ((_e6185 * _e6057) + (_e6188 * _e6150));
                                                                                } else {
                                                                                    phi_24650_ = ((_e6185 * _e5939) + (_e6188 * _e6150));
                                                                                }
                                                                                let _e6198 = phi_24650_;
                                                                                phi_24656_ = _e6198;
                                                                            }
                                                                            let _e6203 = phi_24656_;
                                                                            let _e6207 = global_6.member[(_e5863 + (_e5989 + 1u))];
                                                                            let _e6213 = (select(select(u32(_e6207), 0u, (_e6207 < 0f)), 4294967295u, (_e6207 > 4294967000f)) % 12u);
                                                                            let _e6216 = select(-1f, 1f, ((_e6213 & 1u) == 0u));
                                                                            let _e6219 = select(-1f, 1f, ((_e6213 & 2u) == 0u));
                                                                            if (_e6213 < 4u) {
                                                                                phi_24690_ = ((_e6216 * _e5938) + (_e6219 * _e6088));
                                                                            } else {
                                                                                if (_e6213 < 8u) {
                                                                                    phi_24684_ = ((_e6216 * _e5938) + (_e6219 * _e6150));
                                                                                } else {
                                                                                    phi_24684_ = ((_e6216 * _e6088) + (_e6219 * _e6150));
                                                                                }
                                                                                let _e6229 = phi_24684_;
                                                                                phi_24690_ = _e6229;
                                                                            }
                                                                            let _e6234 = phi_24690_;
                                                                            let _e6238 = global_6.member[(_e5863 + (_e6018 + 1u))];
                                                                            let _e6244 = (select(select(u32(_e6238), 0u, (_e6238 < 0f)), 4294967295u, (_e6238 > 4294967000f)) % 12u);
                                                                            let _e6247 = select(-1f, 1f, ((_e6244 & 1u) == 0u));
                                                                            let _e6250 = select(-1f, 1f, ((_e6244 & 2u) == 0u));
                                                                            if (_e6244 < 4u) {
                                                                                phi_24724_ = ((_e6247 * _e6057) + (_e6250 * _e6088));
                                                                            } else {
                                                                                if (_e6244 < 8u) {
                                                                                    phi_24718_ = ((_e6247 * _e6057) + (_e6250 * _e6150));
                                                                                } else {
                                                                                    phi_24718_ = ((_e6247 * _e6088) + (_e6250 * _e6150));
                                                                                }
                                                                                let _e6260 = phi_24718_;
                                                                                phi_24724_ = _e6260;
                                                                            }
                                                                            let _e6265 = phi_24724_;
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
                                            phi_32799_ = _e4050;
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
                                            phi_32808_ = _e4050;
                                            phi_9077_ = vec3<f32>((_e3755.x + ((((_e6603.x * _e6601.x) + _e6618.x) * _e6631) * _e3757.x)), (_e3755.y + ((((_e6608.y * _e6601.y) + _e6622.y) * _e6631) * _e3757.y)), (_e3755.z + ((((_e6613.z * _e6601.z) + _e6626.z) * _e6631) * _e3757.z)));
                                        } else {
                                            phi_32808_ = _e3753;
                                            phi_9077_ = _e3755;
                                        }
                                        let _e6649 = phi_32808_;
                                        let _e6651 = phi_9077_;
                                        phi_32807_ = _e6649;
                                        phi_9078_ = _e6651;
                                    } else {
                                        phi_32807_ = _e3753;
                                        phi_9078_ = _e3761;
                                    }
                                    let _e6653 = phi_32807_;
                                    let _e6655 = phi_9078_;
                                    phi_32806_ = _e6653;
                                    phi_9079_ = _e6655;
                                } else {
                                    phi_32806_ = _e306;
                                    phi_9079_ = _e1420;
                                }
                                let _e6657 = phi_32806_;
                                let _e6659 = phi_9079_;
                                phi_32802_ = _e6657;
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
                                    phi_20629_ = type_54(0u, type_54().member_1);
                                    phi_20630_ = type_54();
                                    phi_20631_ = true;
                                } else {
                                    if (_e420 < 0f) {
                                        phi_20601_ = -(sqrt(_e435));
                                    } else {
                                        phi_20601_ = sqrt(_e435);
                                    }
                                    let _e442 = phi_20601_;
                                    let _e444 = -((_e420 + _e442));
                                    let _e445 = (_e444 == 0f);
                                    if _e445 {
                                        phi_20624_ = type_54(1u, type_53(0f, 0f));
                                        phi_20625_ = type_54();
                                    } else {
                                        let _e446 = (_e433 / _e444);
                                        if (_e444 < _e446) {
                                            phi_20613_ = type_53(_e444, _e446);
                                        } else {
                                            phi_20613_ = type_53(_e446, _e444);
                                        }
                                        let _e451 = phi_20613_;
                                        phi_20624_ = type_54();
                                        phi_20625_ = type_54(1u, _e451);
                                    }
                                    let _e454 = phi_20624_;
                                    let _e456 = phi_20625_;
                                    phi_20629_ = _e454;
                                    phi_20630_ = _e456;
                                    phi_20631_ = _e445;
                                }
                                let _e461 = phi_20629_;
                                let _e463 = phi_20630_;
                                let _e465 = phi_20631_;
                                if _e465 {
                                    phi_20635_ = _e461;
                                } else {
                                    phi_20635_ = _e463;
                                }
                                let _e467 = phi_20635_;
                                if (bitcast<i32>(_e467.member) != 0i) {
                                    phi_20522_ = type_54(0u, _e467.member_1);
                                } else {
                                    phi_20522_ = type_54(1u, type_54().member_1);
                                }
                                let _e477 = phi_20522_;
                                if (bitcast<i32>(_e477.member) != 0i) {
                                    phi_20569_ = type_54(0u, type_54().member_1);
                                } else {
                                    if (_e477.member_1.member_1 <= 0f) {
                                        phi_20565_ = type_54(0u, type_54().member_1);
                                    } else {
                                        if (_e477.member_1.member != _e477.member_1.member) {
                                            phi_20673_ = true;
                                        } else {
                                            phi_20673_ = (0f >= _e477.member_1.member);
                                        }
                                        let _e489 = phi_20673_;
                                        let _e490 = select(_e477.member_1.member, 0f, _e489);
                                        let _e491 = (_e432 + _e358);
                                        if (_e491 > 0f) {
                                            let _e493 = (_e434 - _e491);
                                            if (_e493 < 0f) {
                                                phi_20781_ = type_54(0u, type_54().member_1);
                                                phi_20782_ = type_54();
                                                phi_20783_ = true;
                                            } else {
                                                if (_e420 < 0f) {
                                                    phi_20753_ = -(sqrt(_e493));
                                                } else {
                                                    phi_20753_ = sqrt(_e493);
                                                }
                                                let _e500 = phi_20753_;
                                                let _e502 = -((_e420 + _e500));
                                                let _e503 = (_e502 == 0f);
                                                if _e503 {
                                                    phi_20776_ = type_54(1u, type_53(0f, 0f));
                                                    phi_20777_ = type_54();
                                                } else {
                                                    let _e504 = (_e491 / _e502);
                                                    if (_e502 < _e504) {
                                                        phi_20765_ = type_53(_e502, _e504);
                                                    } else {
                                                        phi_20765_ = type_53(_e504, _e502);
                                                    }
                                                    let _e509 = phi_20765_;
                                                    phi_20776_ = type_54();
                                                    phi_20777_ = type_54(1u, _e509);
                                                }
                                                let _e512 = phi_20776_;
                                                let _e514 = phi_20777_;
                                                phi_20781_ = _e512;
                                                phi_20782_ = _e514;
                                                phi_20783_ = _e503;
                                            }
                                            let _e519 = phi_20781_;
                                            let _e521 = phi_20782_;
                                            let _e523 = phi_20783_;
                                            if _e523 {
                                                phi_20787_ = _e519;
                                            } else {
                                                phi_20787_ = _e521;
                                            }
                                            let _e525 = phi_20787_;
                                            if (bitcast<i32>(_e525.member) != 0i) {
                                                phi_20721_ = select(_e525.member_1.member_1, _e525.member_1.member, (_e525.member_1.member >= 0f));
                                            } else {
                                                phi_20721_ = -1f;
                                            }
                                            let _e536 = phi_20721_;
                                            if (_e536 >= 0f) {
                                                if (_e477.member_1.member_1 != _e477.member_1.member_1) {
                                                    phi_20825_ = true;
                                                } else {
                                                    phi_20825_ = (_e536 <= _e477.member_1.member_1);
                                                }
                                                let _e541 = phi_20825_;
                                                phi_20550_ = select(_e477.member_1.member_1, _e536, _e541);
                                            } else {
                                                phi_20550_ = _e477.member_1.member_1;
                                            }
                                            let _e544 = phi_20550_;
                                            phi_20552_ = _e544;
                                        } else {
                                            phi_20552_ = _e477.member_1.member_1;
                                        }
                                        let _e546 = phi_20552_;
                                        if (_e546 <= _e490) {
                                            phi_20561_ = type_54(0u, type_54().member_1);
                                        } else {
                                            phi_20561_ = type_54(1u, type_53(_e490, _e546));
                                        }
                                        let _e554 = phi_20561_;
                                        phi_20565_ = _e554;
                                    }
                                    let _e559 = phi_20565_;
                                    phi_20569_ = _e559;
                                }
                                let _e564 = phi_20569_;
                                if (bitcast<i32>(_e564.member) != 0i) {
                                    let _e570 = (_e564.member_1.member < _e317.member.w);
                                    if _e570 {
                                        if (_e564.member_1.member_1 != _e564.member_1.member_1) {
                                            phi_20840_ = true;
                                        } else {
                                            phi_20840_ = (_e317.member.w <= _e564.member_1.member_1);
                                        }
                                        let _e576 = phi_20840_;
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
                                                    phi_32321_ = true;
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
                                                    phi_21040_ = true;
                                                } else {
                                                    phi_21040_ = (0f >= _e725);
                                                }
                                                let _e729 = phi_21040_;
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
                                                        phi_21167_ = true;
                                                    } else {
                                                        phi_21167_ = (0f >= _e721);
                                                    }
                                                    let _e792 = phi_21167_;
                                                    let _e796 = sqrt((select(_e721, 0f, _e792) / (_e355 - _e352)));
                                                    if (_e796 != _e796) {
                                                        phi_21182_ = true;
                                                    } else {
                                                        phi_21182_ = (1f <= _e796);
                                                    }
                                                    let _e800 = phi_21182_;
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
                                                        phi_21306_ = true;
                                                    } else {
                                                        phi_21306_ = (0f >= _e977);
                                                    }
                                                    let _e981 = phi_21306_;
                                                    let _e985 = sqrt((select(_e977, 0f, _e981) / (_e355 - _e352)));
                                                    if (_e985 != _e985) {
                                                        phi_21321_ = true;
                                                    } else {
                                                        phi_21321_ = (1f <= _e985);
                                                    }
                                                    let _e989 = phi_21321_;
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
                                            phi_32321_ = _e306;
                                            if !(_e588) {
                                                break;
                                            }
                                        }
                                        let _e1156 = phi_32321_;
                                        phi_32799_ = _e1156;
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
                                        phi_32805_ = _e1156;
                                        phi_10340_ = type_57(vec3<f32>(exp(-((((_e382 * _e1216.x) + _e1225) + (_e398 * _e1230.z)))), exp(-((((_e385 * _e1216.x) + _e1225) + (_e401 * _e1230.z)))), exp(-((((_e388 * _e1216.x) + _e1225) + (_e404 * _e1230.z))))), (vec3<f32>(((((_e1159.x * _e1170) + (_e1162.x * _e1188)) + _e1199.x) * _e379), ((((_e1159.y * _e1170) + (_e1162.y * _e1188)) + _e1203.y) * _e379), ((((_e1159.z * _e1170) + (_e1162.z * _e1188)) + _e1207.z) * _e379)) * 0.38317f));
                                    } else {
                                        phi_32805_ = _e306;
                                        phi_10340_ = type_57();
                                    }
                                    let _e1247 = phi_32805_;
                                    let _e1249 = phi_10340_;
                                    phi_32804_ = _e1247;
                                    phi_10342_ = _e1249;
                                    phi_10343_ = select(true, false, _e570);
                                } else {
                                    phi_32804_ = _e306;
                                    phi_10342_ = type_57();
                                    phi_10343_ = true;
                                }
                                let _e1252 = phi_32804_;
                                let _e1254 = phi_10342_;
                                let _e1256 = phi_10343_;
                                if _e1256 {
                                    phi_10348_ = type_57(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10348_ = _e1254;
                                }
                                let _e1258 = phi_10348_;
                                phi_32802_ = _e1252;
                                phi_10352_ = (vec3<f32>(_e317.member_2.x, _e317.member_2.y, _e317.member_2.z) * _e1258.member_1);
                            }
                            let _e6663 = phi_32802_;
                            let _e6665 = phi_10352_;
                            phi_32801_ = _e6663;
                            phi_10353_ = _e6665;
                        }
                        let _e6667 = phi_32801_;
                        let _e6669 = phi_10353_;
                        phi_32800_ = _e6667;
                        phi_471_ = (_e308 + _e6669);
                        phi_474_ = (_e310 + 1u);
                    } else {
                        phi_32800_ = _e306;
                        phi_471_ = vec3<f32>();
                        phi_474_ = u32();
                    }
                    let _e6673 = phi_32800_;
                    let _e6675 = phi_471_;
                    let _e6677 = phi_474_;
                    phi_32338_ = _e6673;
                    phi_470_ = _e6675;
                    phi_473_ = _e6677;
                    phi_32799_ = _e6673;
                    if !(_e311) {
                        break;
                    }
                }
                let _e6680 = phi_32799_;
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
    var phi_24902_: bool;
    var phi_24932_: bool;
    var phi_24879_: type_53;
    var phi_24882_: type_53;
    var phi_24883_: bool;
    var phi_24888_: type_53;
    var phi_24989_: bool;
    var phi_25004_: bool;
    var phi_25019_: bool;
    var phi_25034_: bool;
    var phi_24966_: type_53;
    var phi_24969_: type_53;
    var phi_24970_: bool;
    var phi_24975_: type_53;
    var phi_25076_: bool;
    var phi_25091_: bool;
    var phi_25106_: bool;
    var phi_25121_: bool;
    var phi_25053_: type_53;
    var phi_25056_: type_53;
    var phi_25057_: bool;
    var phi_25062_: type_53;
    var phi_24857_: bool;
    var phi_24860_: bool;
    var phi_25223_: bool;
    var phi_25253_: bool;
    var phi_25200_: type_53;
    var phi_25203_: type_53;
    var phi_25204_: bool;
    var phi_25209_: type_53;
    var phi_25310_: bool;
    var phi_25325_: bool;
    var phi_25340_: bool;
    var phi_25355_: bool;
    var phi_25287_: type_53;
    var phi_25290_: type_53;
    var phi_25291_: bool;
    var phi_25296_: type_53;
    var phi_25397_: bool;
    var phi_25412_: bool;
    var phi_25427_: bool;
    var phi_25442_: bool;
    var phi_25374_: type_53;
    var phi_25377_: type_53;
    var phi_25378_: bool;
    var phi_25383_: type_53;
    var phi_25178_: bool;
    var phi_25181_: bool;
    var phi_11734_: u32;
    var phi_11745_: u32;
    var phi_11784_: u32;
    var phi_11785_: u32;
    var phi_11796_: u32;
    var phi_11797_: u32;
    var phi_11798_: u32;
    var phi_11799_: u32;
    var phi_11801_: u32;
    var phi_11802_: u32;
    var phi_11804_: u32;
    var phi_11805_: u32;
    var phi_11806_: bool;
    var phi_11807_: bool;
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
    var phi_11818_: type_66;
    var phi_11819_: bool;
    var phi_11820_: bool;
    var phi_32810_: bool;
    var phi_20499_: bool;
    var phi_20498_: type_66;
    var local_71: type_66;
    var local_72: vec4<f32>;
    var phi_11981_: type_57;
    var local_73: vec4<f32>;
    var local_74: vec4<f32>;
    var phi_11947_: vec3<f32>;
    var phi_11948_: vec3<f32>;
    var phi_11982_: type_57;
    var local_75: type_66;
    var phi_11987_: type_66;
    var phi_11988_: type_66;
    var phi_11993_: type_66;
    var phi_25561_: f32;
    var phi_25573_: type_53;
    var phi_25584_: type_54;
    var phi_25585_: type_54;
    var phi_25589_: type_54;
    var phi_25590_: type_54;
    var phi_25591_: bool;
    var phi_25595_: type_54;
    var phi_25482_: type_54;
    var phi_25633_: bool;
    var phi_25713_: f32;
    var phi_25725_: type_53;
    var phi_25736_: type_54;
    var phi_25737_: type_54;
    var phi_25741_: type_54;
    var phi_25742_: type_54;
    var phi_25743_: bool;
    var phi_25747_: type_54;
    var phi_25681_: f32;
    var phi_25785_: bool;
    var phi_25510_: f32;
    var phi_25512_: f32;
    var phi_25521_: type_54;
    var phi_25525_: type_54;
    var phi_25529_: type_54;
    var phi_25800_: bool;
    var phi_13412_: vec3<f32>;
    var phi_13415_: u32;
    var phi_13499_: u32;
    var phi_13514_: type_53;
    var phi_13515_: type_53;
    var phi_13516_: type_53;
    var phi_25999_: bool;
    var phi_13413_: vec3<f32>;
    var phi_13416_: u32;
    var phi_32930_: bool;
    var local_76: vec3<f32>;
    var local_77: vec3<f32>;
    var local_78: vec3<f32>;
    var phi_35196_: bool;
    var phi_13541_: vec3<f32>;
    var phi_35195_: bool;
    var phi_13543_: vec3<f32>;
    var phi_13544_: bool;
    var phi_35193_: bool;
    var phi_13551_: vec3<f32>;
    var phi_13552_: type_39;
    var phi_35192_: bool;
    var phi_13553_: vec3<f32>;
    var phi_13554_: type_39;
    var phi_13578_: i32;
    var phi_13581_: u32;
    var phi_13635_: bool;
    var phi_13641_: bool;
    var phi_13648_: u32;
    var phi_13650_: i32;
    var phi_13651_: u32;
    var phi_13579_: i32;
    var phi_13582_: u32;
    var phi_13653_: bool;
    var local_79: u32;
    var phi_13934_: vec3<f32>;
    var phi_13951_: vec3<f32>;
    var phi_26066_: bool;
    var phi_13984_: f32;
    var phi_14093_: vec3<f32>;
    var phi_14099_: f32;
    var phi_26139_: bool;
    var phi_26128_: f32;
    var phi_14180_: vec3<f32>;
    var phi_14181_: vec3<f32>;
    var phi_14182_: vec3<f32>;
    var phi_33193_: u32;
    var phi_14184_: vec3<f32>;
    var phi_14185_: vec3<f32>;
    var phi_14186_: vec3<f32>;
    var phi_14187_: bool;
    var phi_14202_: vec3<f32>;
    var phi_14203_: vec3<f32>;
    var phi_33191_: u32;
    var phi_14205_: vec3<f32>;
    var phi_14206_: vec3<f32>;
    var phi_14207_: vec3<f32>;
    var phi_33190_: u32;
    var phi_14208_: vec3<f32>;
    var phi_14209_: vec3<f32>;
    var phi_14210_: vec3<f32>;
    var phi_14211_: bool;
    var phi_33189_: u32;
    var phi_14212_: vec3<f32>;
    var phi_14213_: vec3<f32>;
    var phi_14214_: vec3<f32>;
    var phi_14215_: bool;
    var phi_14216_: bool;
    var phi_26212_: bool;
    var phi_26374_: bool;
    var phi_26389_: bool;
    var phi_26404_: bool;
    var phi_26431_: bool;
    var phi_26446_: bool;
    var phi_26461_: bool;
    var phi_26488_: bool;
    var phi_26503_: bool;
    var phi_26518_: bool;
    var phi_14395_: f32;
    var phi_14403_: f32;
    var phi_14411_: f32;
    var phi_26715_: bool;
    var phi_26704_: f32;
    var phi_14518_: vec3<f32>;
    var phi_26743_: bool;
    var phi_14519_: vec3<f32>;
    var phi_14520_: vec3<f32>;
    var phi_14521_: vec3<f32>;
    var phi_33577_: vec3<f32>;
    var phi_33500_: vec3<f32>;
    var phi_33421_: u32;
    var phi_33339_: f32;
    var phi_14523_: vec3<f32>;
    var phi_14524_: vec3<f32>;
    var phi_14525_: bool;
    var phi_26758_: bool;
    var phi_26809_: bool;
    var phi_26798_: f32;
    var phi_33548_: vec3<f32>;
    var phi_14057_: vec3<f32>;
    var phi_33900_: vec3<f32>;
    var phi_33694_: f32;
    var phi_33547_: vec3<f32>;
    var phi_33235_: u32;
    var phi_14059_: vec3<f32>;
    var phi_14060_: bool;
    var phi_33846_: vec3<f32>;
    var phi_33654_: f32;
    var phi_33575_: vec3<f32>;
    var phi_33498_: vec3<f32>;
    var phi_33419_: u32;
    var phi_33337_: f32;
    var phi_33260_: u32;
    var phi_33186_: u32;
    var phi_14533_: vec3<f32>;
    var phi_14534_: vec3<f32>;
    var phi_14535_: vec3<f32>;
    var phi_14536_: bool;
    var phi_26875_: bool;
    var phi_14605_: vec3<f32>;
    var phi_33243_: u32;
    var phi_14607_: vec3<f32>;
    var phi_14608_: bool;
    var phi_33242_: u32;
    var phi_14609_: vec3<f32>;
    var phi_14610_: bool;
    var phi_33240_: u32;
    var phi_14616_: vec3<f32>;
    var phi_14617_: bool;
    var phi_14650_: type_71;
    var phi_33245_: u32;
    var phi_14652_: vec3<f32>;
    var phi_14653_: type_71;
    var phi_14654_: bool;
    var phi_14660_: type_71;
    var phi_33249_: u32;
    var phi_14661_: type_71;
    var phi_33843_: vec3<f32>;
    var phi_33651_: f32;
    var phi_33572_: vec3<f32>;
    var phi_33495_: vec3<f32>;
    var phi_33416_: u32;
    var phi_33334_: f32;
    var phi_33257_: u32;
    var phi_33248_: u32;
    var phi_14663_: type_71;
    var phi_14664_: bool;
    var phi_35172_: bool;
    var phi_33247_: u32;
    var phi_14675_: vec3<f32>;
    var phi_14678_: type_39;
    var phi_14680_: u32;
    var phi_14682_: u32;
    var phi_17371_: u32;
    var phi_17374_: u32;
    var phi_17372_: u32;
    var phi_17375_: u32;
    var local_80: u32;
    var phi_17412_: u32;
    var phi_17415_: u32;
    var phi_17413_: u32;
    var phi_17416_: u32;
    var local_81: u32;
    var local_82: u32;
    var phi_17492_: f32;
    var local_83: u32;
    var local_84: u32;
    var phi_17541_: f32;
    var phi_17697_: f32;
    var phi_17704_: vec3<f32>;
    var phi_35591_: u32;
    var phi_17706_: f32;
    var phi_17707_: vec3<f32>;
    var phi_17708_: vec3<f32>;
    var phi_16844_: u32;
    var phi_16847_: u32;
    var phi_16845_: u32;
    var phi_16848_: u32;
    var local_85: u32;
    var local_86: u32;
    var phi_17240_: f32;
    var phi_17243_: f32;
    var phi_27148_: bool;
    var phi_27129_: f32;
    var phi_17244_: f32;
    var phi_27163_: bool;
    var phi_27186_: bool;
    var phi_27201_: bool;
    var phi_16995_: f32;
    var phi_27216_: bool;
    var phi_27231_: bool;
    var phi_27246_: bool;
    var phi_27311_: bool;
    var phi_27399_: bool;
    var phi_27380_: f32;
    var phi_17060_: type_74;
    var phi_17246_: type_74;
    var phi_17324_: bool;
    var phi_17333_: vec3<f32>;
    var phi_17335_: vec3<f32>;
    var phi_17336_: bool;
    var phi_17341_: f32;
    var phi_17342_: vec3<f32>;
    var phi_35599_: u32;
    var phi_17343_: f32;
    var phi_17344_: vec3<f32>;
    var phi_17345_: vec3<f32>;
    var phi_17346_: f32;
    var phi_35598_: u32;
    var phi_17347_: f32;
    var phi_17348_: vec3<f32>;
    var phi_17349_: vec3<f32>;
    var phi_17350_: f32;
    var phi_35590_: u32;
    var phi_17709_: f32;
    var phi_17710_: vec3<f32>;
    var phi_17711_: vec3<f32>;
    var phi_17712_: f32;
    var phi_27414_: bool;
    var phi_27524_: f32;
    var phi_27552_: f32;
    var phi_27580_: f32;
    var phi_27714_: bool;
    var phi_27729_: bool;
    var phi_27744_: bool;
    var phi_27771_: bool;
    var phi_27786_: bool;
    var phi_27801_: bool;
    var phi_27828_: bool;
    var phi_27843_: bool;
    var phi_27858_: bool;
    var phi_15016_: f32;
    var phi_15024_: f32;
    var phi_15032_: f32;
    var phi_28055_: bool;
    var phi_28044_: f32;
    var phi_15139_: vec3<f32>;
    var phi_28083_: bool;
    var phi_15140_: vec3<f32>;
    var phi_15152_: type_56;
    var phi_15153_: type_56;
    var phi_15154_: bool;
    var phi_15159_: type_56;
    var phi_28120_: bool;
    var phi_28150_: bool;
    var phi_28139_: f32;
    var phi_28189_: vec3<f32>;
    var phi_15187_: type_56;
    var phi_15188_: type_56;
    var phi_28211_: vec3<f32>;
    var phi_15189_: type_56;
    var phi_15190_: type_56;
    var phi_15191_: bool;
    var phi_15195_: type_56;
    var phi_28243_: bool;
    var phi_28314_: bool;
    var phi_16015_: f32;
    var phi_28329_: bool;
    var phi_28344_: bool;
    var phi_16488_: vec3<f32>;
    var phi_16489_: vec3<f32>;
    var phi_16490_: bool;
    var phi_16495_: vec3<f32>;
    var phi_16496_: bool;
    var phi_28468_: bool;
    var phi_28483_: bool;
    var phi_16787_: vec3<f32>;
    var phi_16789_: vec3<f32>;
    var phi_16816_: vec3<f32>;
    var phi_16817_: vec3<f32>;
    var phi_15633_: vec3<f32>;
    var phi_15634_: vec3<f32>;
    var phi_15635_: bool;
    var phi_15640_: vec3<f32>;
    var phi_15641_: bool;
    var phi_28645_: bool;
    var phi_28660_: bool;
    var phi_15932_: vec3<f32>;
    var phi_15934_: vec3<f32>;
    var phi_35633_: u32;
    var phi_16818_: f32;
    var phi_16819_: vec3<f32>;
    var phi_16820_: vec3<f32>;
    var phi_35632_: u32;
    var phi_16821_: f32;
    var phi_16822_: vec3<f32>;
    var phi_16823_: vec3<f32>;
    var phi_35589_: u32;
    var phi_17713_: f32;
    var phi_17714_: vec3<f32>;
    var phi_17715_: vec3<f32>;
    var phi_17716_: f32;
    var phi_14778_: f32;
    var phi_14779_: f32;
    var phi_35588_: u32;
    var phi_17717_: f32;
    var phi_17718_: vec3<f32>;
    var phi_17719_: vec3<f32>;
    var phi_17720_: f32;
    var phi_28713_: bool;
    var phi_28823_: f32;
    var phi_28851_: f32;
    var phi_28879_: f32;
    var phi_29013_: bool;
    var phi_29028_: bool;
    var phi_29043_: bool;
    var phi_29070_: bool;
    var phi_29085_: bool;
    var phi_29100_: bool;
    var phi_29127_: bool;
    var phi_29142_: bool;
    var phi_29157_: bool;
    var phi_17943_: f32;
    var phi_17951_: f32;
    var phi_17959_: f32;
    var phi_29354_: bool;
    var phi_29343_: f32;
    var phi_18066_: vec3<f32>;
    var phi_29382_: bool;
    var phi_18067_: vec3<f32>;
    var phi_18079_: type_56;
    var phi_18080_: type_56;
    var phi_18081_: bool;
    var phi_18086_: type_56;
    var phi_29419_: bool;
    var phi_29449_: bool;
    var phi_29438_: f32;
    var phi_29488_: vec3<f32>;
    var phi_18114_: type_56;
    var phi_18115_: type_56;
    var phi_29510_: vec3<f32>;
    var phi_18116_: type_56;
    var phi_18117_: type_56;
    var phi_18118_: bool;
    var phi_18122_: type_56;
    var phi_29524_: f32;
    var phi_29527_: f32;
    var phi_18131_: f32;
    var phi_18165_: u32;
    var phi_18168_: u32;
    var phi_18170_: u32;
    var phi_18172_: u32;
    var phi_18176_: type_66;
    var phi_18549_: u32;
    var phi_18551_: u32;
    var phi_18552_: u32;
    var phi_18554_: bool;
    var phi_18555_: bool;
    var phi_18556_: u32;
    var phi_18557_: u32;
    var phi_18558_: u32;
    var phi_18559_: bool;
    var phi_18560_: bool;
    var phi_18561_: bool;
    var phi_29625_: bool;
    var phi_29655_: bool;
    var phi_29602_: type_53;
    var phi_29605_: type_53;
    var phi_29606_: bool;
    var phi_29611_: type_53;
    var phi_29712_: bool;
    var phi_29727_: bool;
    var phi_29742_: bool;
    var phi_29757_: bool;
    var phi_29689_: type_53;
    var phi_29692_: type_53;
    var phi_29693_: bool;
    var phi_29698_: type_53;
    var phi_29799_: bool;
    var phi_29814_: bool;
    var phi_29829_: bool;
    var phi_29844_: bool;
    var phi_29776_: type_53;
    var phi_29779_: type_53;
    var phi_29780_: bool;
    var phi_29785_: type_53;
    var phi_29580_: bool;
    var phi_29583_: bool;
    var phi_29946_: bool;
    var phi_29976_: bool;
    var phi_29923_: type_53;
    var phi_29926_: type_53;
    var phi_29927_: bool;
    var phi_29932_: type_53;
    var phi_30033_: bool;
    var phi_30048_: bool;
    var phi_30063_: bool;
    var phi_30078_: bool;
    var phi_30010_: type_53;
    var phi_30013_: type_53;
    var phi_30014_: bool;
    var phi_30019_: type_53;
    var phi_30120_: bool;
    var phi_30135_: bool;
    var phi_30150_: bool;
    var phi_30165_: bool;
    var phi_30097_: type_53;
    var phi_30100_: type_53;
    var phi_30101_: bool;
    var phi_30106_: type_53;
    var phi_29901_: bool;
    var phi_29904_: bool;
    var phi_18600_: u32;
    var phi_18611_: u32;
    var phi_18650_: u32;
    var phi_18651_: u32;
    var phi_18662_: u32;
    var phi_18663_: u32;
    var phi_18664_: u32;
    var phi_18665_: u32;
    var phi_18667_: u32;
    var phi_18668_: u32;
    var phi_18670_: u32;
    var phi_18671_: u32;
    var phi_18672_: bool;
    var phi_18673_: bool;
    var phi_18472_: vec4<f32>;
    var phi_18473_: vec4<f32>;
    var phi_18474_: bool;
    var phi_18478_: vec4<f32>;
    var phi_18479_: bool;
    var phi_18384_: bool;
    var phi_18421_: bool;
    var phi_18427_: f32;
    var phi_18437_: vec4<f32>;
    var phi_18438_: vec4<f32>;
    var phi_18439_: bool;
    var phi_18443_: vec4<f32>;
    var phi_18444_: bool;
    var phi_18448_: vec4<f32>;
    var phi_18449_: bool;
    var phi_18480_: vec4<f32>;
    var phi_18481_: bool;
    var phi_18313_: vec4<f32>;
    var phi_18314_: vec4<f32>;
    var phi_18315_: bool;
    var phi_18319_: vec4<f32>;
    var phi_18320_: bool;
    var phi_18321_: vec4<f32>;
    var phi_18322_: bool;
    var phi_18323_: bool;
    var phi_18331_: vec4<f32>;
    var phi_18332_: bool;
    var phi_18333_: vec4<f32>;
    var phi_18334_: bool;
    var phi_18335_: bool;
    var phi_18343_: vec4<f32>;
    var phi_18344_: bool;
    var phi_18490_: vec4<f32>;
    var phi_18491_: bool;
    var phi_18512_: type_66;
    var phi_18514_: type_66;
    var phi_18520_: u32;
    var phi_18166_: u32;
    var phi_18169_: u32;
    var phi_18171_: u32;
    var phi_18173_: u32;
    var phi_18177_: type_66;
    var phi_18684_: type_66;
    var phi_18685_: bool;
    var phi_18686_: bool;
    var phi_35100_: bool;
    var phi_20501_: bool;
    var phi_20500_: type_66;
    var phi_18858_: type_66;
    var phi_18863_: type_66;
    var phi_18915_: vec3<f32>;
    var phi_18916_: type_39;
    var phi_18917_: vec3<f32>;
    var phi_18918_: type_39;
    var phi_35535_: bool;
    var phi_18919_: vec3<f32>;
    var phi_18920_: type_39;
    var phi_18921_: u32;
    var phi_35534_: bool;
    var phi_18922_: vec3<f32>;
    var phi_18923_: type_39;
    var phi_18924_: u32;
    var phi_35824_: u32;
    var phi_35533_: bool;
    var phi_14676_: vec3<f32>;
    var phi_14679_: type_39;
    var phi_14681_: u32;
    var phi_14683_: u32;
    var phi_35546_: u32;
    var phi_35532_: bool;
    var phi_30316_: bool;
    var phi_30346_: bool;
    var phi_30335_: f32;
    var phi_30266_: bool;
    var phi_30402_: vec3<f32>;
    var phi_30558_: bool;
    var phi_30573_: bool;
    var phi_30588_: bool;
    var phi_30497_: vec3<f32>;
    var phi_30500_: vec3<f32>;
    var phi_30501_: bool;
    var phi_30271_: vec3<f32>;
    var phi_30275_: vec3<f32>;
    var phi_30276_: bool;
    var phi_30688_: bool;
    var phi_30662_: vec3<f32>;
    var phi_30283_: vec3<f32>;
    var phi_30703_: bool;
    var phi_30813_: f32;
    var phi_30841_: f32;
    var phi_30869_: f32;
    var phi_31003_: bool;
    var phi_31018_: bool;
    var phi_31033_: bool;
    var phi_31060_: bool;
    var phi_31075_: bool;
    var phi_31090_: bool;
    var phi_31117_: bool;
    var phi_31132_: bool;
    var phi_31147_: bool;
    var phi_19143_: f32;
    var phi_19151_: f32;
    var phi_19159_: f32;
    var phi_31344_: bool;
    var phi_31333_: f32;
    var phi_19266_: vec3<f32>;
    var phi_31372_: bool;
    var phi_19267_: vec3<f32>;
    var phi_19279_: type_56;
    var phi_19280_: type_56;
    var phi_19281_: bool;
    var phi_19286_: type_56;
    var phi_31409_: bool;
    var phi_31439_: bool;
    var phi_31428_: f32;
    var phi_31478_: vec3<f32>;
    var phi_19314_: type_56;
    var phi_19315_: type_56;
    var phi_31500_: vec3<f32>;
    var phi_19316_: type_56;
    var phi_19317_: type_56;
    var phi_19318_: bool;
    var phi_19322_: type_56;
    var phi_31525_: bool;
    var phi_19398_: vec3<f32>;
    var phi_37412_: u32;
    var phi_19400_: vec3<f32>;
    var phi_19401_: bool;
    var phi_37411_: u32;
    var phi_19402_: vec3<f32>;
    var phi_19403_: bool;
    var phi_37409_: u32;
    var phi_19409_: vec3<f32>;
    var phi_19410_: bool;
    var local_87: u32;
    var local_88: vec3<f32>;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: type_39;
    var phi_19439_: vec3<f32>;
    var local_92: u32;
    var local_93: vec3<f32>;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: type_39;
    var phi_19455_: type_71;
    var phi_19456_: type_71;
    var phi_37416_: u32;
    var phi_19458_: type_71;
    var phi_19459_: type_71;
    var phi_19460_: bool;
    var phi_37415_: u32;
    var phi_19461_: type_71;
    var phi_19462_: type_71;
    var phi_19463_: bool;
    var phi_19464_: bool;
    var local_97: u32;
    var local_98: vec3<f32>;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: type_39;
    var phi_19470_: type_71;
    var local_102: u32;
    var local_103: vec3<f32>;
    var local_104: vec3<f32>;
    var local_105: vec3<f32>;
    var local_106: type_39;
    var phi_19475_: type_71;
    var phi_19476_: type_71;
    var phi_19477_: bool;
    var phi_19481_: type_71;
    var phi_19482_: type_71;
    var phi_19483_: bool;
    var phi_13713_: bool;
    var phi_13722_: vec3<f32>;
    var phi_13724_: vec3<f32>;
    var phi_13725_: bool;
    var phi_13886_: f32;
    var phi_13889_: f32;
    var phi_31746_: bool;
    var phi_31727_: f32;
    var phi_13890_: f32;
    var phi_31764_: f32;
    var phi_31767_: f32;
    var phi_13893_: f32;
    var phi_13894_: f32;
    var phi_13895_: f32;
    var phi_19487_: type_71;
    var phi_19488_: type_71;
    var phi_19489_: bool;
    var phi_12149_: f32;
    var phi_31814_: f32;
    var phi_31817_: f32;
    var phi_12151_: f32;
    var phi_12152_: f32;
    var phi_31843_: bool;
    var phi_12203_: f32;
    var phi_31861_: f32;
    var phi_31864_: f32;
    var phi_12205_: f32;
    var phi_31875_: bool;
    var phi_31890_: bool;
    var phi_12674_: vec3<f32>;
    var phi_12675_: vec3<f32>;
    var phi_12676_: bool;
    var phi_12681_: vec3<f32>;
    var phi_12682_: bool;
    var phi_32014_: bool;
    var phi_32029_: bool;
    var phi_12973_: vec3<f32>;
    var phi_12975_: vec3<f32>;
    var phi_13002_: vec3<f32>;
    var phi_13016_: vec3<f32>;
    var phi_19493_: type_71;
    var phi_19494_: type_71;
    var phi_19495_: bool;
    var phi_19499_: type_71;
    var phi_11196_: bool;
    var phi_11197_: bool;
    var phi_11198_: bool;
    var phi_11219_: vec4<f32>;
    var phi_11220_: type_35;
    var phi_19503_: vec4<f32>;
    var phi_19504_: type_39;
    var phi_19505_: type_39;
    var phi_19506_: type_35;
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
                        let _e7691 = (_e318.member_3.x * _e318.member.w);
                        let _e7692 = (_e318.member_3.y * _e318.member.w);
                        let _e7693 = (_e318.member_3.z * _e318.member.w);
                        if (_e7691 == _e7691) {
                            let _e7696 = (_e7692 == _e7692);
                            if _e7696 {
                                phi_11196_ = select(true, false, (_e7693 == _e7693));
                            } else {
                                phi_11196_ = false;
                            }
                            let _e7700 = phi_11196_;
                            phi_11197_ = select(true, false, _e7696);
                            phi_11198_ = _e7700;
                        } else {
                            phi_11197_ = true;
                            phi_11198_ = false;
                        }
                        let _e7703 = phi_11197_;
                        let _e7705 = phi_11198_;
                        let _e7708 = select(vec3<f32>(_e7691, _e7692, _e7693), vec3<f32>(0f, 0f, 0f), vec3(select(_e7705, true, _e7703)));
                        phi_11219_ = vec4<f32>(_e7708.x, _e7708.y, _e7708.z, _e318.member_3.w);
                        phi_11220_ = type_35(_e318.member, _e318.member_1, _e318.member_2, _e318.member_3, vec4<u32>(_e318.member_4.x, (_e318.member_4.y | 8u), _e318.member_4.z, _e318.member_4.w));
                    } else {
                        phi_11219_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_11220_ = _e318;
                    }
                    let _e7727 = phi_11219_;
                    let _e7729 = phi_11220_;
                    phi_19503_ = _e7727;
                    phi_19504_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_19505_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_19506_ = _e7729;
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
                            let _e747 = global_2.member[_e385];
                            if (_e747.member_6.x == 0u) {
                                let _e853 = (_e747.member_1.x - _e747.member.x);
                                let _e854 = (_e747.member_1.y - _e747.member.y);
                                let _e855 = (_e747.member_1.z - _e747.member.z);
                                let _e860 = (_e747.member_2.x - _e747.member.x);
                                let _e861 = (_e747.member_2.y - _e747.member.y);
                                let _e862 = (_e747.member_2.z - _e747.member.z);
                                let _e865 = ((_e318.member_1.y * _e862) - (_e861 * _e318.member_1.z));
                                let _e868 = ((_e318.member_1.z * _e860) - (_e862 * _e318.member_1.x));
                                let _e871 = ((_e318.member_1.x * _e861) - (_e860 * _e318.member_1.y));
                                let _e876 = (((_e853 * _e865) + (_e854 * _e868)) + (_e855 * _e871));
                                if (abs(_e876) < 0.000000000001f) {
                                    phi_11485_ = vec4<f32>();
                                    phi_11486_ = true;
                                } else {
                                    let _e879 = (1f / _e876);
                                    let _e880 = (_e318.member.x - _e747.member.x);
                                    let _e881 = (_e318.member.y - _e747.member.y);
                                    let _e882 = (_e318.member.z - _e747.member.z);
                                    let _e888 = ((((_e880 * _e865) + (_e881 * _e868)) + (_e882 * _e871)) * _e879);
                                    if (_e888 < 0f) {
                                        phi_11475_ = vec4<f32>();
                                        phi_11476_ = true;
                                        phi_11477_ = false;
                                    } else {
                                        let _e890 = (_e888 > 1f);
                                        if _e890 {
                                            phi_11473_ = vec4<f32>();
                                            phi_11474_ = false;
                                        } else {
                                            let _e893 = ((_e881 * _e855) - (_e854 * _e882));
                                            let _e896 = ((_e882 * _e853) - (_e855 * _e880));
                                            let _e899 = ((_e880 * _e854) - (_e853 * _e881));
                                            let _e905 = ((((_e318.member_1.x * _e893) + (_e318.member_1.y * _e896)) + (_e318.member_1.z * _e899)) * _e879);
                                            if (_e905 < 0f) {
                                                phi_11463_ = vec4<f32>();
                                                phi_11464_ = true;
                                                phi_11465_ = false;
                                            } else {
                                                let _e908 = ((_e888 + _e905) > 1f);
                                                if _e908 {
                                                    phi_11461_ = vec4<f32>();
                                                    phi_11462_ = false;
                                                } else {
                                                    let _e914 = ((((_e860 * _e893) + (_e861 * _e896)) + (_e862 * _e899)) * _e879);
                                                    if (_e914 <= 0f) {
                                                        phi_11456_ = vec4<f32>();
                                                        phi_11457_ = true;
                                                    } else {
                                                        let _e916 = (_e914 >= _e389.member);
                                                        if _e916 {
                                                            phi_11455_ = vec4<f32>();
                                                        } else {
                                                            phi_11455_ = vec4<f32>(_e914, _e888, _e905, 1f);
                                                        }
                                                        let _e919 = phi_11455_;
                                                        phi_11456_ = _e919;
                                                        phi_11457_ = _e916;
                                                    }
                                                    let _e921 = phi_11456_;
                                                    let _e923 = phi_11457_;
                                                    phi_11461_ = _e921;
                                                    phi_11462_ = _e923;
                                                }
                                                let _e925 = phi_11461_;
                                                let _e927 = phi_11462_;
                                                phi_11463_ = _e925;
                                                phi_11464_ = _e908;
                                                phi_11465_ = _e927;
                                            }
                                            let _e929 = phi_11463_;
                                            let _e931 = phi_11464_;
                                            let _e933 = phi_11465_;
                                            phi_11473_ = _e929;
                                            phi_11474_ = select(_e933, true, _e931);
                                        }
                                        let _e936 = phi_11473_;
                                        let _e938 = phi_11474_;
                                        phi_11475_ = _e936;
                                        phi_11476_ = _e890;
                                        phi_11477_ = _e938;
                                    }
                                    let _e940 = phi_11475_;
                                    let _e942 = phi_11476_;
                                    let _e944 = phi_11477_;
                                    phi_11485_ = _e940;
                                    phi_11486_ = select(_e944, true, _e942);
                                }
                                let _e947 = phi_11485_;
                                let _e949 = phi_11486_;
                                phi_11624_ = _e947;
                                phi_11625_ = _e949;
                            } else {
                                if (_e747.member_6.x == 1u) {
                                    let _e775 = (_e318.member.x - _e747.member.x);
                                    let _e776 = (_e318.member.y - _e747.member.y);
                                    let _e777 = (_e318.member.z - _e747.member.z);
                                    let _e782 = (((_e775 * _e318.member_1.x) + (_e776 * _e318.member_1.y)) + (_e777 * _e318.member_1.z));
                                    if (_e782 > 0f) {
                                        phi_11523_ = select(true, false, ((((_e775 * _e775) + (_e776 * _e776)) + (_e777 * _e777)) > (_e747.member.w * _e747.member.w)));
                                    } else {
                                        phi_11523_ = true;
                                    }
                                    let _e795 = phi_11523_;
                                    if _e795 {
                                        let _e799 = (_e775 - (_e318.member_1.x * _e782));
                                        let _e800 = (_e776 - (_e318.member_1.y * _e782));
                                        let _e801 = (_e777 - (_e318.member_1.z * _e782));
                                        let _e810 = ((_e747.member.w * _e747.member.w) - (((_e799 * _e799) + (_e800 * _e800)) + (_e801 * _e801)));
                                        if (_e810 < 0f) {
                                            phi_11579_ = vec4<f32>();
                                            phi_11580_ = true;
                                        } else {
                                            let _e812 = sqrt(_e810);
                                            let _e813 = -(_e782);
                                            let _e814 = (_e813 - _e812);
                                            if (_e814 <= 0f) {
                                                phi_11557_ = true;
                                            } else {
                                                phi_11557_ = (_e814 >= _e389.member);
                                            }
                                            let _e818 = phi_11557_;
                                            if _e818 {
                                                phi_11563_ = (_e813 + _e812);
                                            } else {
                                                phi_11563_ = _e814;
                                            }
                                            let _e821 = phi_11563_;
                                            if (_e821 <= 0f) {
                                                phi_11574_ = vec4<f32>();
                                                phi_11575_ = true;
                                            } else {
                                                let _e823 = (_e821 >= _e389.member);
                                                if _e823 {
                                                    phi_11573_ = vec4<f32>();
                                                } else {
                                                    phi_11573_ = vec4<f32>(_e821, 0f, 0f, 1f);
                                                }
                                                let _e826 = phi_11573_;
                                                phi_11574_ = _e826;
                                                phi_11575_ = _e823;
                                            }
                                            let _e828 = phi_11574_;
                                            let _e830 = phi_11575_;
                                            phi_11579_ = _e828;
                                            phi_11580_ = _e830;
                                        }
                                        let _e832 = phi_11579_;
                                        let _e834 = phi_11580_;
                                        phi_11584_ = _e832;
                                        phi_11585_ = _e834;
                                    } else {
                                        phi_11584_ = vec4<f32>();
                                        phi_11585_ = true;
                                    }
                                    let _e836 = phi_11584_;
                                    let _e838 = phi_11585_;
                                    phi_11614_ = _e836;
                                    phi_11615_ = _e838;
                                } else {
                                    if (abs(_e318.member_1.y) < 0.000000000001f) {
                                        phi_11612_ = vec4<f32>();
                                        phi_11613_ = true;
                                    } else {
                                        let _e757 = ((_e747.member.x - _e318.member.y) / _e318.member_1.y);
                                        if (_e757 <= 0f) {
                                            phi_11607_ = vec4<f32>();
                                            phi_11608_ = true;
                                        } else {
                                            let _e759 = (_e757 >= _e389.member);
                                            if _e759 {
                                                phi_11606_ = vec4<f32>();
                                            } else {
                                                phi_11606_ = vec4<f32>(_e757, 0f, 0f, 1f);
                                            }
                                            let _e762 = phi_11606_;
                                            phi_11607_ = _e762;
                                            phi_11608_ = _e759;
                                        }
                                        let _e764 = phi_11607_;
                                        let _e766 = phi_11608_;
                                        phi_11612_ = _e764;
                                        phi_11613_ = _e766;
                                    }
                                    let _e768 = phi_11612_;
                                    let _e770 = phi_11613_;
                                    phi_11614_ = _e768;
                                    phi_11615_ = _e770;
                                }
                                let _e840 = phi_11614_;
                                let _e842 = phi_11615_;
                                phi_11624_ = _e840;
                                phi_11625_ = select(true, false, select(true, false, _e842));
                            }
                            let _e951 = phi_11624_;
                            let _e953 = phi_11625_;
                            let _e956 = select(vec4<f32>(0f, 0f, 0f, 0f), _e951, vec4(select(true, false, _e953)));
                            let _e958 = (_e956.w > 0f);
                            if _e958 {
                                let _e964 = type_66(_e956.x, _e389.member_1, _e389.member_2, _e389.member_3);
                                let _e969 = type_66(_e964.member, _e385, _e964.member_2, _e964.member_3);
                                phi_11646_ = _e969;
                                phi_11648_ = _e969;
                            } else {
                                phi_11646_ = type_66();
                                phi_11648_ = _e389;
                            }
                            let _e971 = phi_11646_;
                            let _e973 = phi_11648_;
                            phi_11317_ = _e379;
                            phi_11320_ = _e381;
                            phi_11322_ = _e383;
                            phi_11324_ = (_e385 + 1u);
                            phi_11326_ = select(_e387, _e956, vec4(_e958));
                            phi_11328_ = _e973;
                            phi_11818_ = _e971;
                            phi_11819_ = true;
                            phi_11820_ = false;
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
                                        phi_32810_ = true;
                                        phi_20499_ = bool();
                                        phi_20498_ = type_66();
                                        break;
                                    }
                                    let _e396 = local_70[_e393];
                                    let _e398 = (_e396 >= _e389.member);
                                    if _e398 {
                                        phi_11683_ = u32();
                                    } else {
                                        if _e394 {
                                        } else {
                                            phi_32810_ = true;
                                            phi_20499_ = bool();
                                            phi_20498_ = type_66();
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
                                    let _e444 = ((_e433.member.x - _e318.member.x) * _e375.x);
                                    let _e446 = ((_e433.member.y - _e318.member.y) * _e375.y);
                                    let _e448 = ((_e433.member.z - _e318.member.z) * _e375.z);
                                    let _e455 = ((_e433.member_2.x - _e318.member.x) * _e375.x);
                                    let _e456 = ((_e433.member_2.y - _e318.member.y) * _e375.y);
                                    let _e457 = ((_e433.member_2.z - _e318.member.z) * _e375.z);
                                    if (_e444 == _e444) {
                                        let _e459 = (_e455 == _e455);
                                        if _e459 {
                                            let _e460 = (_e444 != _e444);
                                            if _e460 {
                                                phi_24902_ = true;
                                            } else {
                                                phi_24902_ = (_e455 <= _e444);
                                            }
                                            let _e463 = phi_24902_;
                                            let _e464 = select(_e444, _e455, _e463);
                                            if _e460 {
                                                phi_24932_ = true;
                                            } else {
                                                phi_24932_ = (_e455 >= _e444);
                                            }
                                            let _e469 = phi_24932_;
                                            let _e470 = select(_e444, _e455, _e469);
                                            phi_24879_ = type_53(select(-300000000000000000000000000000000000000f, _e464, (_e464 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e470, (_e470 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_24879_ = type_53();
                                        }
                                        let _e475 = phi_24879_;
                                        phi_24882_ = _e475;
                                        phi_24883_ = select(true, false, _e459);
                                    } else {
                                        phi_24882_ = type_53();
                                        phi_24883_ = true;
                                    }
                                    let _e478 = phi_24882_;
                                    let _e480 = phi_24883_;
                                    if _e480 {
                                        phi_24888_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_24888_ = _e478;
                                    }
                                    let _e482 = phi_24888_;
                                    if (_e446 == _e446) {
                                        let _e486 = (_e456 == _e456);
                                        if _e486 {
                                            let _e487 = (_e446 != _e446);
                                            if _e487 {
                                                phi_24989_ = true;
                                            } else {
                                                phi_24989_ = (_e456 <= _e446);
                                            }
                                            let _e490 = phi_24989_;
                                            let _e491 = select(_e446, _e456, _e490);
                                            if (_e482.member != _e482.member) {
                                                phi_25004_ = true;
                                            } else {
                                                phi_25004_ = (_e491 >= _e482.member);
                                            }
                                            let _e495 = phi_25004_;
                                            if _e487 {
                                                phi_25019_ = true;
                                            } else {
                                                phi_25019_ = (_e456 >= _e446);
                                            }
                                            let _e499 = phi_25019_;
                                            let _e500 = select(_e446, _e456, _e499);
                                            if (_e482.member_1 != _e482.member_1) {
                                                phi_25034_ = true;
                                            } else {
                                                phi_25034_ = (_e500 <= _e482.member_1);
                                            }
                                            let _e504 = phi_25034_;
                                            phi_24966_ = type_53(select(_e482.member, _e491, _e495), select(_e482.member_1, _e500, _e504));
                                        } else {
                                            phi_24966_ = type_53();
                                        }
                                        let _e508 = phi_24966_;
                                        phi_24969_ = _e508;
                                        phi_24970_ = select(true, false, _e486);
                                    } else {
                                        phi_24969_ = type_53();
                                        phi_24970_ = true;
                                    }
                                    let _e511 = phi_24969_;
                                    let _e513 = phi_24970_;
                                    if _e513 {
                                        phi_24975_ = _e482;
                                    } else {
                                        phi_24975_ = _e511;
                                    }
                                    let _e515 = phi_24975_;
                                    if (_e448 == _e448) {
                                        let _e519 = (_e457 == _e457);
                                        if _e519 {
                                            let _e520 = (_e448 != _e448);
                                            if _e520 {
                                                phi_25076_ = true;
                                            } else {
                                                phi_25076_ = (_e457 <= _e448);
                                            }
                                            let _e523 = phi_25076_;
                                            let _e524 = select(_e448, _e457, _e523);
                                            if (_e515.member != _e515.member) {
                                                phi_25091_ = true;
                                            } else {
                                                phi_25091_ = (_e524 >= _e515.member);
                                            }
                                            let _e528 = phi_25091_;
                                            if _e520 {
                                                phi_25106_ = true;
                                            } else {
                                                phi_25106_ = (_e457 >= _e448);
                                            }
                                            let _e532 = phi_25106_;
                                            let _e533 = select(_e448, _e457, _e532);
                                            if (_e515.member_1 != _e515.member_1) {
                                                phi_25121_ = true;
                                            } else {
                                                phi_25121_ = (_e533 <= _e515.member_1);
                                            }
                                            let _e537 = phi_25121_;
                                            phi_25053_ = type_53(select(_e515.member, _e524, _e528), select(_e515.member_1, _e533, _e537));
                                        } else {
                                            phi_25053_ = type_53();
                                        }
                                        let _e541 = phi_25053_;
                                        phi_25056_ = _e541;
                                        phi_25057_ = select(true, false, _e519);
                                    } else {
                                        phi_25056_ = type_53();
                                        phi_25057_ = true;
                                    }
                                    let _e544 = phi_25056_;
                                    let _e546 = phi_25057_;
                                    if _e546 {
                                        phi_25062_ = _e515;
                                    } else {
                                        phi_25062_ = _e544;
                                    }
                                    let _e548 = phi_25062_;
                                    if (_e548.member_1 < 0f) {
                                        phi_24860_ = true;
                                    } else {
                                        if (_e548.member > _e548.member_1) {
                                            phi_24857_ = true;
                                        } else {
                                            phi_24857_ = (_e548.member > _e389.member);
                                        }
                                        let _e555 = phi_24857_;
                                        phi_24860_ = _e555;
                                    }
                                    let _e557 = phi_24860_;
                                    let _e558 = select(_e548.member, 300000000000000000000000000000000000000f, _e557);
                                    let _e567 = ((_e433.member_4.x - _e318.member.x) * _e375.x);
                                    let _e568 = ((_e433.member_4.y - _e318.member.y) * _e375.y);
                                    let _e569 = ((_e433.member_4.z - _e318.member.z) * _e375.z);
                                    let _e576 = ((_e433.member_6.x - _e318.member.x) * _e375.x);
                                    let _e577 = ((_e433.member_6.y - _e318.member.y) * _e375.y);
                                    let _e578 = ((_e433.member_6.z - _e318.member.z) * _e375.z);
                                    if (_e567 == _e567) {
                                        let _e580 = (_e576 == _e576);
                                        if _e580 {
                                            let _e581 = (_e567 != _e567);
                                            if _e581 {
                                                phi_25223_ = true;
                                            } else {
                                                phi_25223_ = (_e576 <= _e567);
                                            }
                                            let _e584 = phi_25223_;
                                            let _e585 = select(_e567, _e576, _e584);
                                            if _e581 {
                                                phi_25253_ = true;
                                            } else {
                                                phi_25253_ = (_e576 >= _e567);
                                            }
                                            let _e590 = phi_25253_;
                                            let _e591 = select(_e567, _e576, _e590);
                                            phi_25200_ = type_53(select(-300000000000000000000000000000000000000f, _e585, (_e585 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e591, (_e591 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_25200_ = type_53();
                                        }
                                        let _e596 = phi_25200_;
                                        phi_25203_ = _e596;
                                        phi_25204_ = select(true, false, _e580);
                                    } else {
                                        phi_25203_ = type_53();
                                        phi_25204_ = true;
                                    }
                                    let _e599 = phi_25203_;
                                    let _e601 = phi_25204_;
                                    if _e601 {
                                        phi_25209_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_25209_ = _e599;
                                    }
                                    let _e603 = phi_25209_;
                                    if (_e568 == _e568) {
                                        let _e607 = (_e577 == _e577);
                                        if _e607 {
                                            let _e608 = (_e568 != _e568);
                                            if _e608 {
                                                phi_25310_ = true;
                                            } else {
                                                phi_25310_ = (_e577 <= _e568);
                                            }
                                            let _e611 = phi_25310_;
                                            let _e612 = select(_e568, _e577, _e611);
                                            if (_e603.member != _e603.member) {
                                                phi_25325_ = true;
                                            } else {
                                                phi_25325_ = (_e612 >= _e603.member);
                                            }
                                            let _e616 = phi_25325_;
                                            if _e608 {
                                                phi_25340_ = true;
                                            } else {
                                                phi_25340_ = (_e577 >= _e568);
                                            }
                                            let _e620 = phi_25340_;
                                            let _e621 = select(_e568, _e577, _e620);
                                            if (_e603.member_1 != _e603.member_1) {
                                                phi_25355_ = true;
                                            } else {
                                                phi_25355_ = (_e621 <= _e603.member_1);
                                            }
                                            let _e625 = phi_25355_;
                                            phi_25287_ = type_53(select(_e603.member, _e612, _e616), select(_e603.member_1, _e621, _e625));
                                        } else {
                                            phi_25287_ = type_53();
                                        }
                                        let _e629 = phi_25287_;
                                        phi_25290_ = _e629;
                                        phi_25291_ = select(true, false, _e607);
                                    } else {
                                        phi_25290_ = type_53();
                                        phi_25291_ = true;
                                    }
                                    let _e632 = phi_25290_;
                                    let _e634 = phi_25291_;
                                    if _e634 {
                                        phi_25296_ = _e603;
                                    } else {
                                        phi_25296_ = _e632;
                                    }
                                    let _e636 = phi_25296_;
                                    if (_e569 == _e569) {
                                        let _e640 = (_e578 == _e578);
                                        if _e640 {
                                            let _e641 = (_e569 != _e569);
                                            if _e641 {
                                                phi_25397_ = true;
                                            } else {
                                                phi_25397_ = (_e578 <= _e569);
                                            }
                                            let _e644 = phi_25397_;
                                            let _e645 = select(_e569, _e578, _e644);
                                            if (_e636.member != _e636.member) {
                                                phi_25412_ = true;
                                            } else {
                                                phi_25412_ = (_e645 >= _e636.member);
                                            }
                                            let _e649 = phi_25412_;
                                            if _e641 {
                                                phi_25427_ = true;
                                            } else {
                                                phi_25427_ = (_e578 >= _e569);
                                            }
                                            let _e653 = phi_25427_;
                                            let _e654 = select(_e569, _e578, _e653);
                                            if (_e636.member_1 != _e636.member_1) {
                                                phi_25442_ = true;
                                            } else {
                                                phi_25442_ = (_e654 <= _e636.member_1);
                                            }
                                            let _e658 = phi_25442_;
                                            phi_25374_ = type_53(select(_e636.member, _e645, _e649), select(_e636.member_1, _e654, _e658));
                                        } else {
                                            phi_25374_ = type_53();
                                        }
                                        let _e662 = phi_25374_;
                                        phi_25377_ = _e662;
                                        phi_25378_ = select(true, false, _e640);
                                    } else {
                                        phi_25377_ = type_53();
                                        phi_25378_ = true;
                                    }
                                    let _e665 = phi_25377_;
                                    let _e667 = phi_25378_;
                                    if _e667 {
                                        phi_25383_ = _e636;
                                    } else {
                                        phi_25383_ = _e665;
                                    }
                                    let _e669 = phi_25383_;
                                    if (_e669.member_1 < 0f) {
                                        phi_25181_ = true;
                                    } else {
                                        if (_e669.member > _e669.member_1) {
                                            phi_25178_ = true;
                                        } else {
                                            phi_25178_ = (_e669.member > _e389.member);
                                        }
                                        let _e676 = phi_25178_;
                                        phi_25181_ = _e676;
                                    }
                                    let _e678 = phi_25181_;
                                    let _e679 = select(_e669.member, 300000000000000000000000000000000000000f, _e678);
                                    if (_e433.member_3 > 0u) {
                                        phi_11734_ = ((2147483648u | ((_e433.member_3 - 1u) << bitcast<u32>(28u))) | _e433.member_1);
                                    } else {
                                        phi_11734_ = _e433.member_1;
                                    }
                                    let _e689 = phi_11734_;
                                    if (_e433.member_7 > 0u) {
                                        phi_11745_ = ((2147483648u | ((_e433.member_7 - 1u) << bitcast<u32>(28u))) | _e433.member_5);
                                    } else {
                                        phi_11745_ = _e433.member_5;
                                    }
                                    let _e699 = phi_11745_;
                                    let _e700 = (_e558 <= _e679);
                                    let _e704 = select(_e558, _e679, _e700);
                                    if (_e704 < 300000000000000000000000000000000000000f) {
                                        let _e706 = (_e417 < 64u);
                                        if _e706 {
                                            if _e706 {
                                            } else {
                                                phi_32810_ = true;
                                                phi_20499_ = bool();
                                                phi_20498_ = type_66();
                                                break;
                                            }
                                            local_69[_e417] = select(_e689, _e699, _e700);
                                            if _e706 {
                                            } else {
                                                phi_32810_ = true;
                                                phi_20499_ = bool();
                                                phi_20498_ = type_66();
                                                break;
                                            }
                                            local_70[_e417] = _e704;
                                            phi_11784_ = (_e417 + 1u);
                                        } else {
                                            phi_11784_ = _e417;
                                        }
                                        let _e711 = phi_11784_;
                                        phi_11785_ = _e711;
                                    } else {
                                        phi_11785_ = _e417;
                                    }
                                    let _e713 = phi_11785_;
                                    phi_11796_ = select(4294967295u, select(_e699, _e689, _e700), (select(_e679, _e558, _e700) < 300000000000000000000000000000000000000f));
                                    phi_11797_ = _e713;
                                    phi_11798_ = u32();
                                    phi_11799_ = u32();
                                } else {
                                    let _e426 = (_e415 & 268435455u);
                                    phi_11796_ = u32();
                                    phi_11797_ = u32();
                                    phi_11798_ = ((_e426 + ((_e415 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                    phi_11799_ = _e426;
                                }
                                let _e717 = phi_11796_;
                                let _e719 = phi_11797_;
                                let _e721 = phi_11798_;
                                let _e723 = phi_11799_;
                                phi_11801_ = _e717;
                                phi_11802_ = _e719;
                                phi_11804_ = _e721;
                                phi_11805_ = _e723;
                                phi_11806_ = select(true, false, _e425);
                                phi_11807_ = _e425;
                            } else {
                                phi_11801_ = u32();
                                phi_11802_ = u32();
                                phi_11804_ = _e383;
                                phi_11805_ = _e385;
                                phi_11806_ = _e421;
                                phi_11807_ = false;
                            }
                            let _e726 = phi_11801_;
                            let _e728 = phi_11802_;
                            let _e730 = phi_11804_;
                            let _e732 = phi_11805_;
                            let _e734 = phi_11806_;
                            let _e736 = phi_11807_;
                            phi_11317_ = select(_e726, 4294967295u, _e734);
                            phi_11320_ = select(_e728, select(_e413, _e417, _e419), _e734);
                            phi_11322_ = select(_e383, _e730, _e734);
                            phi_11324_ = select(_e385, _e732, _e734);
                            phi_11326_ = _e387;
                            phi_11328_ = _e389;
                            phi_11818_ = type_66();
                            phi_11819_ = select(_e736, true, _e734);
                            phi_11820_ = select(select(_e423, false, _e419), false, _e734);
                        }
                        let _e978 = phi_11317_;
                        let _e980 = phi_11320_;
                        let _e982 = phi_11322_;
                        let _e984 = phi_11324_;
                        let _e986 = phi_11326_;
                        let _e988 = phi_11328_;
                        let _e990 = phi_11818_;
                        let _e992 = phi_11819_;
                        let _e994 = phi_11820_;
                        phi_11316_ = _e978;
                        phi_11319_ = _e980;
                        phi_11321_ = _e982;
                        phi_11323_ = _e984;
                        phi_11325_ = _e986;
                        phi_11327_ = _e988;
                        phi_32810_ = false;
                        phi_20499_ = _e994;
                        phi_20498_ = _e990;
                        if !(_e992) {
                            break;
                        }
                    }
                    let _e997 = phi_32810_;
                    let _e999 = phi_20499_;
                    let _e1001 = phi_20498_;
                    if _e997 {
                        break;
                    }
                    if _e999 {
                        let _e1003 = local_71;
                        if (_e1003.member_1 != 4294967295u) {
                            let _e1007 = global_2.member[_e1003.member_1];
                            if (_e1007.member_6.x == 0u) {
                                let _e1049 = (_e1007.member_1.x - _e1007.member.x);
                                let _e1050 = (_e1007.member_1.y - _e1007.member.y);
                                let _e1051 = (_e1007.member_1.z - _e1007.member.z);
                                let _e1056 = (_e1007.member_2.x - _e1007.member.x);
                                let _e1057 = (_e1007.member_2.y - _e1007.member.y);
                                let _e1058 = (_e1007.member_2.z - _e1007.member.z);
                                let _e1061 = ((_e1050 * _e1058) - (_e1057 * _e1051));
                                let _e1064 = ((_e1051 * _e1056) - (_e1058 * _e1049));
                                let _e1067 = ((_e1049 * _e1057) - (_e1056 * _e1050));
                                let _e1074 = (1f / sqrt((((_e1061 * _e1061) + (_e1064 * _e1064)) + (_e1067 * _e1067))));
                                let _e1078 = vec3<f32>((_e1061 * _e1074), (_e1064 * _e1074), (_e1067 * _e1074));
                                if (_e1007.member_6.w == 1u) {
                                    let _e1087 = local_73;
                                    let _e1091 = local_74;
                                    let _e1093 = ((1f - _e1087.y) - _e1091.z);
                                    let _e1114 = (((_e1007.member_3.x * _e1093) + (_e1007.member_4.x * _e1087.y)) + (_e1007.member_5.x * _e1091.z));
                                    let _e1115 = (((_e1007.member_3.y * _e1093) + (_e1007.member_4.y * _e1087.y)) + (_e1007.member_5.y * _e1091.z));
                                    let _e1116 = (((_e1007.member_3.z * _e1093) + (_e1007.member_4.z * _e1087.y)) + (_e1007.member_5.z * _e1091.z));
                                    let _e1121 = (((_e1114 * _e1114) + (_e1115 * _e1115)) + (_e1116 * _e1116));
                                    if (_e1121 > 0f) {
                                        let _e1124 = (1f / sqrt(_e1121));
                                        phi_11947_ = vec3<f32>((_e1114 * _e1124), (_e1115 * _e1124), (_e1116 * _e1124));
                                    } else {
                                        phi_11947_ = _e1078;
                                    }
                                    let _e1130 = phi_11947_;
                                    phi_11948_ = _e1130;
                                } else {
                                    phi_11948_ = _e1078;
                                }
                                let _e1132 = phi_11948_;
                                phi_11982_ = type_57(_e1078, _e1132);
                            } else {
                                if (_e1007.member_6.x == 1u) {
                                    let _e1017 = local_72;
                                    let _e1037 = vec3<f32>((((_e318.member.x + (_e318.member_1.x * _e1017.x)) - _e1007.member.x) / _e1007.member.w), (((_e318.member.y + (_e318.member_1.y * _e1017.x)) - _e1007.member.y) / _e1007.member.w), (((_e318.member.z + (_e318.member_1.z * _e1017.x)) - _e1007.member.z) / _e1007.member.w));
                                    phi_11981_ = type_57(_e1037, _e1037);
                                } else {
                                    let _e1014 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e318.member_1.y > 0f)));
                                    phi_11981_ = type_57(_e1014, _e1014);
                                }
                                let _e1040 = phi_11981_;
                                phi_11982_ = _e1040;
                            }
                            let _e1135 = phi_11982_;
                            let _e1139 = local_75;
                            let _e1144 = type_66(_e1139.member, _e1139.member_1, _e1135.member, _e1139.member_3);
                            phi_11987_ = type_66(_e1144.member, _e1144.member_1, _e1144.member_2, _e1135.member_1);
                        } else {
                            let _e7911 = local_107;
                            phi_11987_ = _e7911;
                        }
                        let _e1151 = phi_11987_;
                        phi_11988_ = _e1151;
                    } else {
                        phi_11988_ = type_66();
                    }
                    let _e1153 = phi_11988_;
                    if select(true, false, _e999) {
                        phi_11993_ = _e1001;
                    } else {
                        phi_11993_ = _e1153;
                    }
                    let _e1156 = phi_11993_;
                    let _e1157 = (select(select(u32(_e318.member_3.w), 0u, (_e318.member_3.w < 0f)), 4294967295u, (_e318.member_3.w > 4294967000f)) + 1u);
                    if (_e1156.member_1 == 4294967295u) {
                        if _e358 {
                            phi_12152_ = 1f;
                        } else {
                            if ((_e364 & 2u) == 0u) {
                                phi_12151_ = 1f;
                            } else {
                                let _e7172 = global_1.member.member_3[1u];
                                let _e7175 = global_1.member.member_3[2u];
                                let _e7178 = ((atan2(_e318.member_1.z, _e318.member_1.x) * 0.15915494f) + 0.5f);
                                let _e7182 = select(_e318.member_1.y, -1f, (_e318.member_1.y < -1f));
                                let _e7185 = acos(select(_e7182, 1f, (_e7182 > 1f)));
                                let _e7187 = f32(_e7172);
                                let _e7188 = ((_e7178 - floor(_e7178)) * _e7187);
                                let _e7193 = select(select(u32(_e7188), 0u, (_e7188 < 0f)), 4294967295u, (_e7188 > 4294967000f));
                                let _e7194 = (_e7172 - 1u);
                                let _e7197 = f32(_e7175);
                                let _e7198 = ((_e7185 * 0.31830987f) * _e7197);
                                let _e7203 = select(select(u32(_e7198), 0u, (_e7198 < 0f)), 4294967295u, (_e7198 > 4294967000f));
                                let _e7204 = (_e7175 - 1u);
                                let _e7206 = select(_e7203, _e7204, (_e7204 < _e7203));
                                let _e7207 = sin(_e7185);
                                if (_e7207 <= 0f) {
                                    phi_12149_ = 0f;
                                } else {
                                    let _e7211 = global_1.member.member_2[3u];
                                    let _e7214 = global_1.member.member_3[0u];
                                    let _e7218 = (_e7211 + _e7206);
                                    let _e7221 = global_6.member[(_e7218 + 1u)];
                                    let _e7223 = global_6.member[_e7218];
                                    let _e7226 = ((_e7214 + (_e7206 * (_e7172 + 1u))) + select(_e7193, _e7194, (_e7194 < _e7193)));
                                    let _e7229 = global_6.member[(_e7226 + 1u)];
                                    let _e7231 = global_6.member[_e7226];
                                    phi_12149_ = ((((_e7221 - _e7223) * _e7197) * ((_e7229 - _e7231) * _e7187)) / (19.73921f * _e7207));
                                }
                                let _e7238 = phi_12149_;
                                if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_31817_ = 1f;
                                } else {
                                    let _e7240 = (_e318.member_1.w * _e318.member_1.w);
                                    let _e7242 = (_e7240 + (_e7238 * _e7238));
                                    if (_e7242 == 0f) {
                                        phi_31814_ = 0f;
                                    } else {
                                        phi_31814_ = (_e7240 / _e7242);
                                    }
                                    let _e7246 = phi_31814_;
                                    phi_31817_ = _e7246;
                                }
                                let _e7248 = phi_31817_;
                                phi_12151_ = _e7248;
                            }
                            let _e7250 = phi_12151_;
                            phi_12152_ = _e7250;
                        }
                        let _e7252 = phi_12152_;
                        if ((_e364 & 4u) == 0u) {
                            phi_13016_ = _e349;
                        } else {
                            if _e358 {
                                phi_12205_ = 1f;
                            } else {
                                let _e7263 = global_1.member.member_5;
                                let _e7267 = (_e318.member_1.x - _e7263.x);
                                let _e7268 = (_e318.member_1.y - _e7263.y);
                                let _e7269 = (_e318.member_1.z - _e7263.z);
                                let _e7276 = (sqrt((((_e7267 * _e7267) + (_e7268 * _e7268)) + (_e7269 * _e7269))) * 0.5f);
                                if (_e7276 != _e7276) {
                                    phi_31843_ = true;
                                } else {
                                    phi_31843_ = (1f <= _e7276);
                                }
                                let _e7280 = phi_31843_;
                                let _e7286 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e7276, 1f, _e7280))) >= _e7286) {
                                    phi_12203_ = 0f;
                                } else {
                                    let _e7289 = sin((_e7286 * 0.5f));
                                    phi_12203_ = (1f / ((12.566371f * _e7289) * _e7289));
                                }
                                let _e7294 = phi_12203_;
                                if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_31864_ = 1f;
                                } else {
                                    let _e7296 = (_e318.member_1.w * _e318.member_1.w);
                                    let _e7298 = (_e7296 + (_e7294 * _e7294));
                                    if (_e7298 == 0f) {
                                        phi_31861_ = 0f;
                                    } else {
                                        phi_31861_ = (_e7296 / _e7298);
                                    }
                                    let _e7302 = phi_31861_;
                                    phi_31864_ = _e7302;
                                }
                                let _e7304 = phi_31864_;
                                phi_12205_ = _e7304;
                            }
                            let _e7306 = phi_12205_;
                            let _e7308 = global_1.member.member_5;
                            let _e7312 = (_e318.member_1.x - _e7308.x);
                            let _e7313 = (_e318.member_1.y - _e7308.y);
                            let _e7314 = (_e318.member_1.z - _e7308.z);
                            let _e7321 = (sqrt((((_e7312 * _e7312) + (_e7313 * _e7313)) + (_e7314 * _e7314))) * 0.5f);
                            if (_e7321 != _e7321) {
                                phi_31875_ = true;
                            } else {
                                phi_31875_ = (1f <= _e7321);
                            }
                            let _e7325 = phi_31875_;
                            let _e7328 = (2f * asin(select(_e7321, 1f, _e7325)));
                            let _e7332 = global_1.member.member_6[3u];
                            if (_e7328 >= _e7332) {
                                phi_13002_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                let _e7336 = (1f - pow((_e7328 / _e7332), 2f));
                                if (_e7336 != _e7336) {
                                    phi_31890_ = true;
                                } else {
                                    phi_31890_ = (0f >= _e7336);
                                }
                                let _e7340 = phi_31890_;
                                let _e7343 = global_1.member.member_6;
                                let _e7349 = global_1.member.member_8[2u];
                                if (_e7349 == 4294967295u) {
                                    phi_12975_ = vec3<f32>(1f, 1f, 1f);
                                } else {
                                    let _e7352 = global_6.member[_e7349];
                                    let _e7355 = global_6.member[(_e7349 + 1u)];
                                    let _e7358 = global_6.member[(_e7349 + 2u)];
                                    let _e7361 = global_6.member[(_e7349 + 3u)];
                                    let _e7364 = global_6.member[(_e7349 + 4u)];
                                    let _e7367 = global_6.member[(_e7349 + 5u)];
                                    let _e7370 = global_6.member[(_e7349 + 6u)];
                                    let _e7373 = global_6.member[(_e7349 + 7u)];
                                    let _e7376 = global_6.member[(_e7349 + 8u)];
                                    let _e7379 = global_6.member[(_e7349 + 9u)];
                                    let _e7382 = global_6.member[(_e7349 + 13u)];
                                    let _e7385 = global_6.member[(_e7349 + 14u)];
                                    let _e7388 = global_6.member[(_e7349 + 15u)];
                                    let _e7391 = global_6.member[(_e7349 + 16u)];
                                    let _e7394 = global_6.member[(_e7349 + 18u)];
                                    let _e7397 = global_6.member[(_e7349 + 19u)];
                                    let _e7400 = global_6.member[(_e7349 + 20u)];
                                    let _e7403 = global_6.member[(_e7349 + 29u)];
                                    let _e7404 = bitcast<u32>(_e7403);
                                    let _e7417 = (((((_e318.member.x * _e318.member.x) + (_e318.member.y * _e318.member.y)) + (_e318.member.z * _e318.member.z)) - (2f * (((_e318.member.x * _e7352) + (_e318.member.y * _e7355)) + (_e318.member.z * _e7358)))) + _e7370);
                                    if (_e7417 > 0f) {
                                        let _e7426 = ((((_e318.member.x - _e7352) * _e7373) + ((_e318.member.y - _e7355) * _e7376)) + ((_e318.member.z - _e7358) * _e7379));
                                        let _e7428 = ((_e7426 * _e7426) - _e7417);
                                        if (_e7426 >= 0f) {
                                            phi_12675_ = vec3<f32>();
                                            phi_12676_ = true;
                                        } else {
                                            let _e7430 = (_e7428 <= 0f);
                                            if _e7430 {
                                                phi_12674_ = vec3<f32>();
                                            } else {
                                                let _e7433 = (-(_e7426) - sqrt(_e7428));
                                                phi_12674_ = vec3<f32>((_e318.member.x + (_e7373 * _e7433)), (_e318.member.y + (_e7376 * _e7433)), (_e318.member.z + (_e7379 * _e7433)));
                                            }
                                            let _e7442 = phi_12674_;
                                            phi_12675_ = _e7442;
                                            phi_12676_ = _e7430;
                                        }
                                        let _e7444 = phi_12675_;
                                        let _e7446 = phi_12676_;
                                        phi_12681_ = _e7444;
                                        phi_12682_ = select(true, false, _e7446);
                                    } else {
                                        phi_12681_ = vec3<f32>(_e318.member.x, _e318.member.y, _e318.member.z);
                                        phi_12682_ = true;
                                    }
                                    let _e7449 = phi_12681_;
                                    let _e7451 = phi_12682_;
                                    if _e7451 {
                                        let _e7453 = (_e7449.x - _e7352);
                                        let _e7455 = (_e7449.y - _e7355);
                                        let _e7457 = (_e7449.z - _e7358);
                                        let _e7463 = sqrt((((_e7453 * _e7453) + (_e7455 * _e7455)) + (_e7457 * _e7457)));
                                        let _e7464 = (1f / _e7463);
                                        let _e7482 = ((((((_e7449.x * _e7449.x) + (_e7449.y * _e7449.y)) + (_e7449.z * _e7449.z)) - (2f * (((_e7449.x * _e7352) + (_e7449.y * _e7355)) + (_e7449.z * _e7358)))) + _e7367) / (_e7463 + _e7361));
                                        if (_e7482 != _e7482) {
                                            phi_32014_ = true;
                                        } else {
                                            phi_32014_ = (0f >= _e7482);
                                        }
                                        let _e7486 = phi_32014_;
                                        let _e7490 = sqrt((select(_e7482, 0f, _e7486) / (_e7364 - _e7361)));
                                        if (_e7490 != _e7490) {
                                            phi_32029_ = true;
                                        } else {
                                            phi_32029_ = (1f <= _e7490);
                                        }
                                        let _e7494 = phi_32029_;
                                        let _e7496 = (select(_e7490, 1f, _e7494) * 63f);
                                        let _e7501 = ((((_e7453 * _e7464) * _e7373) + ((_e7455 * _e7464) * _e7376)) + ((_e7457 * _e7464) * _e7379));
                                        let _e7503 = select(_e7501, -1f, (_e7501 < -1f));
                                        let _e7507 = ((select(_e7503, 1f, (_e7503 > 1f)) + 1f) * 127.5f);
                                        let _e7512 = select(select(u32(_e7496), 0u, (_e7496 < 0f)), 4294967295u, (_e7496 > 4294967000f));
                                        let _e7514 = select(_e7512, 62u, (62u < _e7512));
                                        let _e7519 = select(select(u32(_e7507), 0u, (_e7507 < 0f)), 4294967295u, (_e7507 > 4294967000f));
                                        let _e7521 = select(_e7519, 254u, (254u < _e7519));
                                        let _e7523 = (_e7496 - f32(_e7514));
                                        let _e7525 = (_e7507 - f32(_e7521));
                                        let _e7529 = (_e7404 + (3u * ((_e7514 * 256u) + _e7521)));
                                        let _e7534 = (_e7404 + (3u * (((_e7514 + 1u) * 256u) + _e7521)));
                                        let _e7536 = global_6.member[_e7529];
                                        let _e7539 = global_6.member[(_e7529 + 1u)];
                                        let _e7542 = global_6.member[(_e7529 + 2u)];
                                        let _e7543 = (1f - _e7525);
                                        let _e7549 = global_6.member[(_e7529 + 3u)];
                                        let _e7552 = global_6.member[(_e7529 + 4u)];
                                        let _e7555 = global_6.member[(_e7529 + 5u)];
                                        let _e7562 = (1f - _e7523);
                                        let _e7567 = global_6.member[_e7534];
                                        let _e7570 = global_6.member[(_e7534 + 1u)];
                                        let _e7573 = global_6.member[(_e7534 + 2u)];
                                        let _e7579 = global_6.member[(_e7534 + 3u)];
                                        let _e7582 = global_6.member[(_e7534 + 4u)];
                                        let _e7585 = global_6.member[(_e7534 + 5u)];
                                        phi_12973_ = vec3<f32>(((((_e7536 * _e7543) + (_e7549 * _e7525)) * _e7562) + (((_e7567 * _e7543) + (_e7579 * _e7525)) * _e7523)), ((((_e7539 * _e7543) + (_e7552 * _e7525)) * _e7562) + (((_e7570 * _e7543) + (_e7582 * _e7525)) * _e7523)), ((((_e7542 * _e7543) + (_e7555 * _e7525)) * _e7562) + (((_e7573 * _e7543) + (_e7585 * _e7525)) * _e7523)));
                                    } else {
                                        phi_12973_ = vec3<f32>(0f, 0f, 0f);
                                    }
                                    let _e7600 = phi_12973_;
                                    let _e7607 = ((_e7391 * 1.1f) * _e7600.y);
                                    phi_12975_ = vec3<f32>(exp(-((((_e7382 * _e7600.x) + _e7607) + (_e7394 * _e7600.z)))), exp(-((((_e7385 * _e7600.x) + _e7607) + (_e7397 * _e7600.z)))), exp(-((((_e7388 * _e7600.x) + _e7607) + (_e7400 * _e7600.z)))));
                                }
                                let _e7626 = phi_12975_;
                                let _e7638 = sin((_e7332 * 0.5f));
                                let _e7641 = (((1f - (0.6f * (1f - sqrt(select(_e7336, 0f, _e7340))))) * 1.25f) / ((12.566371f * _e7638) * _e7638));
                                phi_13002_ = vec3<f32>(((_e7343.x * _e7626.x) * _e7641), ((_e7343.y * _e7626.y) * _e7641), ((_e7343.z * _e7626.z) * _e7641));
                            }
                            let _e7647 = phi_13002_;
                            phi_13016_ = vec3<f32>((_e318.member_3.x + ((_e318.member_2.x * _e7647.x) * _e7306)), (_e318.member_3.y + ((_e318.member_2.y * _e7647.y) * _e7306)), (_e318.member_3.z + ((_e318.member_2.z * _e7647.z) * _e7306)));
                        }
                        let _e7662 = phi_13016_;
                        phi_19493_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z, f32(_e345)), vec4<f32>(_e7662.x, _e7662.y, _e7662.z, f32(_e1157)), vec4<u32>(_e318.member_4.x, (_e318.member_4.y & 4294967294u), 0u, 0u)), type_39(vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, -1f), vec4<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z, 0f), vec4<f32>((_e318.member_2.x * _e7252), (_e318.member_2.y * _e7252), (_e318.member_2.z * _e7252), 0f)), type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19494_ = type_71();
                        phi_19495_ = true;
                    } else {
                        if ((_e364 & 8u) == 0u) {
                            phi_35192_ = _e997;
                            phi_13553_ = _e339;
                            phi_13554_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        } else {
                            if (_e1156.member > 100f) {
                                let _e1170 = global_1.member.member_8[1u];
                                let _e1172 = global_6.member[_e1170];
                                let _e1175 = global_6.member[(_e1170 + 1u)];
                                let _e1178 = global_6.member[(_e1170 + 2u)];
                                let _e1181 = global_6.member[(_e1170 + 3u)];
                                let _e1184 = global_6.member[(_e1170 + 5u)];
                                let _e1187 = global_6.member[(_e1170 + 6u)];
                                let _e1190 = global_6.member[(_e1170 + 10u)];
                                let _e1193 = global_6.member[(_e1170 + 11u)];
                                let _e1196 = global_6.member[(_e1170 + 13u)];
                                let _e1199 = global_6.member[(_e1170 + 14u)];
                                let _e1202 = global_6.member[(_e1170 + 15u)];
                                let _e1205 = global_6.member[(_e1170 + 16u)];
                                let _e1208 = global_6.member[(_e1170 + 18u)];
                                let _e1211 = global_6.member[(_e1170 + 19u)];
                                let _e1214 = global_6.member[(_e1170 + 20u)];
                                let _e1222 = ((((_e318.member.x - _e1172) * _e318.member_1.x) + ((_e318.member.y - _e1175) * _e318.member_1.y)) + ((_e318.member.z - _e1178) * _e318.member_1.z));
                                let _e1234 = ((((_e318.member.x * _e318.member.x) + (_e318.member.y * _e318.member.y)) + (_e318.member.z * _e318.member.z)) - (2f * (((_e318.member.x * _e1172) + (_e318.member.y * _e1175)) + (_e318.member.z * _e1178))));
                                let _e1235 = (_e1234 + _e1187);
                                let _e1236 = (_e1222 * _e1222);
                                let _e1237 = (_e1236 - _e1235);
                                if (_e1237 < 0f) {
                                    phi_25589_ = type_54(0u, type_54().member_1);
                                    phi_25590_ = type_54();
                                    phi_25591_ = true;
                                } else {
                                    if (_e1222 < 0f) {
                                        phi_25561_ = -(sqrt(_e1237));
                                    } else {
                                        phi_25561_ = sqrt(_e1237);
                                    }
                                    let _e1244 = phi_25561_;
                                    let _e1246 = -((_e1222 + _e1244));
                                    let _e1247 = (_e1246 == 0f);
                                    if _e1247 {
                                        phi_25584_ = type_54(1u, type_53(0f, 0f));
                                        phi_25585_ = type_54();
                                    } else {
                                        let _e1248 = (_e1235 / _e1246);
                                        if (_e1246 < _e1248) {
                                            phi_25573_ = type_53(_e1246, _e1248);
                                        } else {
                                            phi_25573_ = type_53(_e1248, _e1246);
                                        }
                                        let _e1253 = phi_25573_;
                                        phi_25584_ = type_54();
                                        phi_25585_ = type_54(1u, _e1253);
                                    }
                                    let _e1256 = phi_25584_;
                                    let _e1258 = phi_25585_;
                                    phi_25589_ = _e1256;
                                    phi_25590_ = _e1258;
                                    phi_25591_ = _e1247;
                                }
                                let _e1263 = phi_25589_;
                                let _e1265 = phi_25590_;
                                let _e1267 = phi_25591_;
                                if _e1267 {
                                    phi_25595_ = _e1263;
                                } else {
                                    phi_25595_ = _e1265;
                                }
                                let _e1269 = phi_25595_;
                                if (bitcast<i32>(_e1269.member) != 0i) {
                                    phi_25482_ = type_54(0u, _e1269.member_1);
                                } else {
                                    phi_25482_ = type_54(1u, type_54().member_1);
                                }
                                let _e1279 = phi_25482_;
                                if (bitcast<i32>(_e1279.member) != 0i) {
                                    phi_25529_ = type_54(0u, type_54().member_1);
                                } else {
                                    if (_e1279.member_1.member_1 <= 0f) {
                                        phi_25525_ = type_54(0u, type_54().member_1);
                                    } else {
                                        if (_e1279.member_1.member != _e1279.member_1.member) {
                                            phi_25633_ = true;
                                        } else {
                                            phi_25633_ = (0f >= _e1279.member_1.member);
                                        }
                                        let _e1291 = phi_25633_;
                                        let _e1292 = select(_e1279.member_1.member, 0f, _e1291);
                                        let _e1293 = (_e1234 + _e1184);
                                        if (_e1293 > 0f) {
                                            let _e1295 = (_e1236 - _e1293);
                                            if (_e1295 < 0f) {
                                                phi_25741_ = type_54(0u, type_54().member_1);
                                                phi_25742_ = type_54();
                                                phi_25743_ = true;
                                            } else {
                                                if (_e1222 < 0f) {
                                                    phi_25713_ = -(sqrt(_e1295));
                                                } else {
                                                    phi_25713_ = sqrt(_e1295);
                                                }
                                                let _e1302 = phi_25713_;
                                                let _e1304 = -((_e1222 + _e1302));
                                                let _e1305 = (_e1304 == 0f);
                                                if _e1305 {
                                                    phi_25736_ = type_54(1u, type_53(0f, 0f));
                                                    phi_25737_ = type_54();
                                                } else {
                                                    let _e1306 = (_e1293 / _e1304);
                                                    if (_e1304 < _e1306) {
                                                        phi_25725_ = type_53(_e1304, _e1306);
                                                    } else {
                                                        phi_25725_ = type_53(_e1306, _e1304);
                                                    }
                                                    let _e1311 = phi_25725_;
                                                    phi_25736_ = type_54();
                                                    phi_25737_ = type_54(1u, _e1311);
                                                }
                                                let _e1314 = phi_25736_;
                                                let _e1316 = phi_25737_;
                                                phi_25741_ = _e1314;
                                                phi_25742_ = _e1316;
                                                phi_25743_ = _e1305;
                                            }
                                            let _e1321 = phi_25741_;
                                            let _e1323 = phi_25742_;
                                            let _e1325 = phi_25743_;
                                            if _e1325 {
                                                phi_25747_ = _e1321;
                                            } else {
                                                phi_25747_ = _e1323;
                                            }
                                            let _e1327 = phi_25747_;
                                            if (bitcast<i32>(_e1327.member) != 0i) {
                                                phi_25681_ = select(_e1327.member_1.member_1, _e1327.member_1.member, (_e1327.member_1.member >= 0f));
                                            } else {
                                                phi_25681_ = -1f;
                                            }
                                            let _e1338 = phi_25681_;
                                            if (_e1338 >= 0f) {
                                                if (_e1279.member_1.member_1 != _e1279.member_1.member_1) {
                                                    phi_25785_ = true;
                                                } else {
                                                    phi_25785_ = (_e1338 <= _e1279.member_1.member_1);
                                                }
                                                let _e1343 = phi_25785_;
                                                phi_25510_ = select(_e1279.member_1.member_1, _e1338, _e1343);
                                            } else {
                                                phi_25510_ = _e1279.member_1.member_1;
                                            }
                                            let _e1346 = phi_25510_;
                                            phi_25512_ = _e1346;
                                        } else {
                                            phi_25512_ = _e1279.member_1.member_1;
                                        }
                                        let _e1348 = phi_25512_;
                                        if (_e1348 <= _e1292) {
                                            phi_25521_ = type_54(0u, type_54().member_1);
                                        } else {
                                            phi_25521_ = type_54(1u, type_53(_e1292, _e1348));
                                        }
                                        let _e1356 = phi_25521_;
                                        phi_25525_ = _e1356;
                                    }
                                    let _e1361 = phi_25525_;
                                    phi_25529_ = _e1361;
                                }
                                let _e1366 = phi_25529_;
                                if (bitcast<i32>(_e1366.member) != 0i) {
                                    let _e1372 = (_e1366.member_1.member < _e1156.member);
                                    if _e1372 {
                                        if (_e1366.member_1.member_1 != _e1366.member_1.member_1) {
                                            phi_25800_ = true;
                                        } else {
                                            phi_25800_ = (_e1156.member <= _e1366.member_1.member_1);
                                        }
                                        let _e1378 = phi_25800_;
                                        let _e1379 = select(_e1366.member_1.member_1, _e1156.member, _e1378);
                                        phi_13412_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13415_ = 0u;
                                        loop {
                                            let _e1381 = phi_13412_;
                                            let _e1383 = phi_13415_;
                                            local_76 = _e1381;
                                            local_77 = _e1381;
                                            local_78 = _e1381;
                                            let _e1384 = (_e1383 < 32u);
                                            if _e1384 {
                                                let _e1385 = -(_e1222);
                                                if (_e1366.member_1.member <= _e1379) {
                                                } else {
                                                    phi_32930_ = true;
                                                    break;
                                                }
                                                let _e1388 = select(_e1385, _e1366.member_1.member, (_e1385 < _e1366.member_1.member));
                                                let _e1390 = select(_e1388, _e1379, (_e1388 > _e1379));
                                                let _e1391 = (_e1390 - _e1366.member_1.member);
                                                let _e1392 = (_e1379 - _e1390);
                                                let _e1393 = (_e1379 - _e1366.member_1.member);
                                                let _e1394 = (0.000000001f * _e1393);
                                                if (_e1391 <= _e1394) {
                                                    let _e1469 = (f32(_e1383) * 0.03125f);
                                                    let _e1472 = (f32((_e1383 + 1u)) * 0.03125f);
                                                    let _e1474 = ((_e1469 + _e1472) * 0.5f);
                                                    phi_13516_ = type_53((_e1366.member_1.member + ((_e1393 * _e1474) * _e1474)), (_e1393 * ((_e1472 * _e1472) - (_e1469 * _e1469))));
                                                } else {
                                                    if (_e1392 <= _e1394) {
                                                        let _e1452 = (f32((31u - _e1383)) * 0.03125f);
                                                        let _e1455 = (f32((32u - _e1383)) * 0.03125f);
                                                        let _e1457 = ((_e1452 + _e1455) * 0.5f);
                                                        phi_13515_ = type_53((_e1379 - ((_e1393 * _e1457) * _e1457)), (_e1393 * ((_e1455 * _e1455) - (_e1452 * _e1452))));
                                                    } else {
                                                        let _e1400 = round(((32f * _e1391) / (_e1391 + _e1392)));
                                                        let _e1405 = select(select(u32(_e1400), 0u, (_e1400 < 0f)), 4294967295u, (_e1400 > 4294967000f));
                                                        if (_e1405 < 1u) {
                                                            phi_13499_ = 1u;
                                                        } else {
                                                            phi_13499_ = select(_e1405, 31u, (_e1405 > 31u));
                                                        }
                                                        let _e1410 = phi_13499_;
                                                        if (_e1383 < _e1410) {
                                                            let _e1431 = ((_e1410 - 1u) - _e1383);
                                                            let _e1433 = f32(_e1410);
                                                            let _e1434 = (f32(_e1431) / _e1433);
                                                            let _e1437 = (f32((_e1431 + 1u)) / _e1433);
                                                            let _e1439 = ((_e1434 + _e1437) * 0.5f);
                                                            phi_13514_ = type_53((_e1390 - ((_e1391 * _e1439) * _e1439)), (_e1391 * ((_e1437 * _e1437) - (_e1434 * _e1434))));
                                                        } else {
                                                            let _e1413 = (_e1383 - _e1410);
                                                            let _e1415 = f32((32u - _e1410));
                                                            let _e1416 = (f32(_e1413) / _e1415);
                                                            let _e1419 = (f32((_e1413 + 1u)) / _e1415);
                                                            let _e1421 = ((_e1416 + _e1419) * 0.5f);
                                                            phi_13514_ = type_53((_e1390 + ((_e1392 * _e1421) * _e1421)), (_e1392 * ((_e1419 * _e1419) - (_e1416 * _e1416))));
                                                        }
                                                        let _e1449 = phi_13514_;
                                                        phi_13515_ = _e1449;
                                                    }
                                                    let _e1467 = phi_13515_;
                                                    phi_13516_ = _e1467;
                                                }
                                                let _e1484 = phi_13516_;
                                                let _e1490 = (_e318.member.x + (_e318.member_1.x * _e1484.member));
                                                let _e1491 = (_e318.member.y + (_e318.member_1.y * _e1484.member));
                                                let _e1492 = (_e318.member.z + (_e318.member_1.z * _e1484.member));
                                                let _e1506 = (_e1490 - _e1172);
                                                let _e1507 = (_e1491 - _e1175);
                                                let _e1508 = (_e1492 - _e1178);
                                                let _e1516 = ((((((_e1490 * _e1490) + (_e1491 * _e1491)) + (_e1492 * _e1492)) - (2f * (((_e1490 * _e1172) + (_e1491 * _e1175)) + (_e1492 * _e1178)))) + _e1184) / (sqrt((((_e1506 * _e1506) + (_e1507 * _e1507)) + (_e1508 * _e1508))) + _e1181));
                                                let _e1520 = (1f - (abs((_e1516 - 25000f)) * 0.00006666667f));
                                                if (_e1520 != _e1520) {
                                                    phi_25999_ = true;
                                                } else {
                                                    phi_25999_ = (0f >= _e1520);
                                                }
                                                let _e1524 = phi_25999_;
                                                let _e1526 = -(_e1516);
                                                phi_13413_ = (_e1381 + vec3<f32>((exp((_e1526 / _e1190)) * _e1484.member_1), (exp((_e1526 / _e1193)) * _e1484.member_1), (select(_e1520, 0f, _e1524) * _e1484.member_1)));
                                                phi_13416_ = (_e1383 + 1u);
                                            } else {
                                                phi_13413_ = vec3<f32>();
                                                phi_13416_ = u32();
                                            }
                                            let _e1538 = phi_13413_;
                                            let _e1540 = phi_13416_;
                                            phi_13412_ = _e1538;
                                            phi_13415_ = _e1540;
                                            phi_32930_ = _e997;
                                            if !(_e1384) {
                                                break;
                                            }
                                        }
                                        let _e1543 = phi_32930_;
                                        if _e1543 {
                                            break;
                                        }
                                        let _e1545 = local_76;
                                        let _e1552 = local_77;
                                        let _e1554 = ((_e1205 * 1.1f) * _e1552.y);
                                        let _e1559 = local_78;
                                        phi_35196_ = _e1543;
                                        phi_13541_ = vec3<f32>(exp(-((((_e1196 * _e1545.x) + _e1554) + (_e1208 * _e1559.z)))), exp(-((((_e1199 * _e1545.x) + _e1554) + (_e1211 * _e1559.z)))), exp(-((((_e1202 * _e1545.x) + _e1554) + (_e1214 * _e1559.z)))));
                                    } else {
                                        phi_35196_ = _e997;
                                        phi_13541_ = vec3<f32>();
                                    }
                                    let _e1575 = phi_35196_;
                                    let _e1577 = phi_13541_;
                                    phi_35195_ = _e1575;
                                    phi_13543_ = _e1577;
                                    phi_13544_ = select(true, false, _e1372);
                                } else {
                                    phi_35195_ = _e997;
                                    phi_13543_ = vec3<f32>();
                                    phi_13544_ = true;
                                }
                                let _e1580 = phi_35195_;
                                let _e1582 = phi_13543_;
                                let _e1584 = phi_13544_;
                                phi_35193_ = _e1580;
                                phi_13551_ = (_e339 * select(_e1582, vec3<f32>(1f, 1f, 1f), vec3(_e1584)));
                                phi_13552_ = type_39(vec4<f32>(_e318.member.x, _e318.member.y, _e318.member.z, _e1156.member), vec4<f32>(_e318.member_1.x, _e318.member_1.y, _e318.member_1.z, 0f), vec4<f32>(_e318.member_2.x, _e318.member_2.y, _e318.member_2.z, 0f));
                            } else {
                                phi_35193_ = _e997;
                                phi_13551_ = _e339;
                                phi_13552_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            }
                            let _e1589 = phi_35193_;
                            let _e1591 = phi_13551_;
                            let _e1593 = phi_13552_;
                            phi_35192_ = _e1589;
                            phi_13553_ = _e1591;
                            phi_13554_ = _e1593;
                        }
                        let _e1595 = phi_35192_;
                        let _e1597 = phi_13553_;
                        let _e1599 = phi_13554_;
                        let _e1604 = (_e318.member.x + (_e318.member_1.x * _e1156.member));
                        let _e1605 = (_e318.member.y + (_e318.member_1.y * _e1156.member));
                        let _e1606 = (_e318.member.z + (_e318.member_1.z * _e1156.member));
                        let _e1608 = global_2.member[_e1156.member_1];
                        phi_13578_ = 0i;
                        phi_13581_ = _e1608.member_6.y;
                        loop {
                            let _e1612 = phi_13578_;
                            let _e1614 = phi_13581_;
                            local_79 = _e1614;
                            if (_e1612 < 4i) {
                                let _e1617 = global_4.member[_e1614];
                                let _e1619 = (_e1617.member != 6u);
                                if _e1619 {
                                    phi_13650_ = i32();
                                    phi_13651_ = u32();
                                } else {
                                    let _e1623 = (abs(_e1604) / _e1617.member_6.z);
                                    let _e1634 = (abs(_e1606) / _e1617.member_6.w);
                                    if ((select(select(u32(_e1623), 0u, (_e1623 < 0f)), 4294967295u, (_e1623 > 4294967000f)) % 2u) == 0u) {
                                        phi_13635_ = (_e1604 > 0f);
                                    } else {
                                        phi_13635_ = (_e1604 <= 0f);
                                    }
                                    let _e1645 = phi_13635_;
                                    if ((select(select(u32(_e1634), 0u, (_e1634 < 0f)), 4294967295u, (_e1634 > 4294967000f)) % 2u) == 0u) {
                                        phi_13641_ = (_e1606 > 0f);
                                    } else {
                                        phi_13641_ = (_e1606 <= 0f);
                                    }
                                    let _e1649 = phi_13641_;
                                    if (_e1645 != _e1649) {
                                        phi_13648_ = _e1617.member_1;
                                    } else {
                                        phi_13648_ = _e1617.member_2;
                                    }
                                    let _e1654 = phi_13648_;
                                    phi_13650_ = (_e1612 + 1i);
                                    phi_13651_ = _e1654;
                                }
                                let _e1657 = phi_13650_;
                                let _e1659 = phi_13651_;
                                phi_13579_ = _e1657;
                                phi_13582_ = _e1659;
                                phi_13653_ = select(true, false, _e1619);
                            } else {
                                phi_13579_ = i32();
                                phi_13582_ = u32();
                                phi_13653_ = false;
                            }
                            let _e1662 = phi_13579_;
                            let _e1664 = phi_13582_;
                            let _e1666 = phi_13653_;
                            phi_13578_ = _e1662;
                            phi_13581_ = _e1664;
                            if !(_e1666) {
                                break;
                            }
                        }
                        let _e1669 = local_79;
                        let _e1671 = global_4.member[_e1669];
                        let _e1673 = (_e1671.member == 4u);
                        if _e1673 {
                            if _e1673 {
                                if (_e1671.member_1 == 1u) {
                                    phi_13713_ = true;
                                } else {
                                    phi_13713_ = ((((-(_e318.member_1.x) * _e1156.member_3.x) + (-(_e318.member_1.y) * _e1156.member_3.y)) + (-(_e318.member_1.z) * _e1156.member_3.z)) > 0f);
                                }
                                let _e6968 = phi_13713_;
                                if _e6968 {
                                    phi_13722_ = vec3<f32>(_e1671.member_4.x, _e1671.member_4.y, _e1671.member_4.z);
                                } else {
                                    phi_13722_ = vec3<f32>();
                                }
                                let _e6975 = phi_13722_;
                                phi_13724_ = _e6975;
                                phi_13725_ = select(true, false, _e6968);
                            } else {
                                phi_13724_ = vec3<f32>();
                                phi_13725_ = true;
                            }
                            let _e6978 = phi_13724_;
                            let _e6980 = phi_13725_;
                            let _e6982 = select(_e6978, vec3<f32>(0f, 0f, 0f), vec3(_e6980));
                            if _e358 {
                                phi_13895_ = 1f;
                            } else {
                                if ((_e364 & 1u) == 0u) {
                                    phi_13894_ = 1f;
                                } else {
                                    if (_e1608.member_6.z > 0u) {
                                        let _e6991 = global_1.member.member_2[1u];
                                        let _e6992 = (_e6991 + (_e1608.member_6.z - 1u));
                                        let _e6995 = global_6.member[(_e6992 + 1u)];
                                        let _e6997 = global_6.member[_e6992];
                                        if (_e1608.member_6.x == 1u) {
                                            let _e7074 = (_e1608.member.x - _e318.member.x);
                                            let _e7075 = (_e1608.member.y - _e318.member.y);
                                            let _e7076 = (_e1608.member.z - _e318.member.z);
                                            let _e7081 = (((_e7074 * _e7074) + (_e7075 * _e7075)) + (_e7076 * _e7076));
                                            let _e7082 = (_e1608.member.w * _e1608.member.w);
                                            if (_e7081 <= (_e7082 * 1.000001f)) {
                                                let _e7096 = (_e1604 - _e318.member.x);
                                                let _e7097 = (_e1605 - _e318.member.y);
                                                let _e7098 = (_e1606 - _e318.member.z);
                                                let _e7103 = (((_e7096 * _e7096) + (_e7097 * _e7097)) + (_e7098 * _e7098));
                                                phi_31727_ = (_e7103 / (((abs((((_e1156.member_2.x * _e7096) + (_e1156.member_2.y * _e7097)) + (_e1156.member_2.z * _e7098))) / sqrt(_e7103)) * 12.566371f) * _e7082));
                                            } else {
                                                let _e7085 = (_e7082 / _e7081);
                                                if (_e7085 != _e7085) {
                                                    phi_31746_ = true;
                                                } else {
                                                    phi_31746_ = (1f <= _e7085);
                                                }
                                                let _e7089 = phi_31746_;
                                                let _e7090 = select(_e7085, 1f, _e7089);
                                                phi_31727_ = (0.15915494f / (_e7090 / (1f + sqrt((1f - _e7090)))));
                                            }
                                            let _e7119 = phi_31727_;
                                            phi_13890_ = _e7119;
                                        } else {
                                            let _e7003 = (_e1604 - _e318.member.x);
                                            let _e7004 = (_e1605 - _e318.member.y);
                                            let _e7005 = (_e1606 - _e318.member.z);
                                            let _e7010 = (((_e7003 * _e7003) + (_e7004 * _e7004)) + (_e7005 * _e7005));
                                            let _e7021 = (abs((((_e1156.member_2.x * _e7003) + (_e1156.member_2.y * _e7004)) + (_e1156.member_2.z * _e7005))) / sqrt(_e7010));
                                            if (_e7021 <= 0f) {
                                                phi_13889_ = 0f;
                                            } else {
                                                if (_e1608.member_6.x == 0u) {
                                                    let _e7036 = (_e1608.member_1.x - _e1608.member.x);
                                                    let _e7037 = (_e1608.member_1.y - _e1608.member.y);
                                                    let _e7038 = (_e1608.member_1.z - _e1608.member.z);
                                                    let _e7043 = (_e1608.member_2.x - _e1608.member.x);
                                                    let _e7044 = (_e1608.member_2.y - _e1608.member.y);
                                                    let _e7045 = (_e1608.member_2.z - _e1608.member.z);
                                                    let _e7048 = ((_e7037 * _e7045) - (_e7044 * _e7038));
                                                    let _e7051 = ((_e7038 * _e7043) - (_e7045 * _e7036));
                                                    let _e7054 = ((_e7036 * _e7044) - (_e7043 * _e7037));
                                                    phi_13886_ = (sqrt((((_e7048 * _e7048) + (_e7051 * _e7051)) + (_e7054 * _e7054))) * 0.5f);
                                                } else {
                                                    phi_13886_ = ((12.566371f * _e1608.member.w) * _e1608.member.w);
                                                }
                                                let _e7063 = phi_13886_;
                                                phi_13889_ = (_e7010 / (_e7021 * _e7063));
                                            }
                                            let _e7067 = phi_13889_;
                                            phi_13890_ = _e7067;
                                        }
                                        let _e7121 = phi_13890_;
                                        let _e7122 = ((_e6995 - _e6997) * _e7121);
                                        if (_e318.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_31767_ = 1f;
                                        } else {
                                            let _e7124 = (_e318.member_1.w * _e318.member_1.w);
                                            let _e7126 = (_e7124 + (_e7122 * _e7122));
                                            if (_e7126 == 0f) {
                                                phi_31764_ = 0f;
                                            } else {
                                                phi_31764_ = (_e7124 / _e7126);
                                            }
                                            let _e7130 = phi_31764_;
                                            phi_31767_ = _e7130;
                                        }
                                        let _e7132 = phi_31767_;
                                        phi_13893_ = _e7132;
                                    } else {
                                        phi_13893_ = 1f;
                                    }
                                    let _e7134 = phi_13893_;
                                    phi_13894_ = _e7134;
                                }
                                let _e7136 = phi_13894_;
                                phi_13895_ = _e7136;
                            }
                            let _e7138 = phi_13895_;
                            phi_19487_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e1597.x, _e1597.y, _e1597.z, f32(_e345)), vec4<f32>((_e318.member_3.x + ((_e1597.x * _e6982.x) * _e7138)), (_e318.member_3.y + ((_e1597.y * _e6982.y) * _e7138)), (_e318.member_3.z + ((_e1597.z * _e6982.z) * _e7138)), f32(_e1157)), vec4<u32>(_e318.member_4.x, (_e318.member_4.y & 4294967294u), 0u, 0u)), _e1599, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19488_ = type_71();
                            phi_19489_ = true;
                        } else {
                            let _e1674 = -(_e318.member_1.x);
                            let _e1675 = -(_e318.member_1.y);
                            let _e1676 = -(_e318.member_1.z);
                            if ((((_e1156.member_2.x * _e1674) + (_e1156.member_2.y * _e1675)) + (_e1156.member_2.z * _e1676)) < 0f) {
                                phi_13934_ = -(_e1156.member_2);
                            } else {
                                phi_13934_ = _e1156.member_2;
                            }
                            let _e1689 = phi_13934_;
                            if ((((_e1156.member_3.x * _e1674) + (_e1156.member_3.y * _e1675)) + (_e1156.member_3.z * _e1676)) < 0f) {
                                phi_13951_ = -(_e1156.member_3);
                            } else {
                                phi_13951_ = _e1156.member_3;
                            }
                            let _e1702 = phi_13951_;
                            let _e1705 = global_1.member.member_4[2u];
                            let _e1706 = abs(_e1604);
                            let _e1707 = abs(_e1605);
                            let _e1708 = abs(_e1606);
                            let _e1710 = select(_e1707, _e1706, (_e1706 > _e1707));
                            let _e1714 = (0.00002f * (1f + select(_e1708, _e1710, (_e1710 > _e1708))));
                            if (_e1705 != _e1705) {
                                phi_26066_ = true;
                            } else {
                                phi_26066_ = (_e1714 >= _e1705);
                            }
                            let _e1718 = phi_26066_;
                            let _e1720 = (select(_e1705, _e1714, _e1718) * 2f);
                            let _e1727 = (_e1604 + (_e1689.x * _e1720));
                            let _e1728 = (_e1605 + (_e1689.y * _e1720));
                            let _e1729 = (_e1606 + (_e1689.z * _e1720));
                            let _e1730 = vec3<f32>(_e1727, _e1728, _e1729);
                            if ((_e318.member_4.y & 4u) == 0u) {
                                phi_13984_ = 0f;
                            } else {
                                let _e1735 = global_1.member.member_4[0u];
                                phi_13984_ = _e1735;
                            }
                            let _e1737 = phi_13984_;
                            let _e1740 = select(-1f, 1f, (_e1702.z >= 0f));
                            let _e1742 = (-1f / (_e1740 + _e1702.z));
                            let _e1746 = ((_e1702.x * _e1702.y) * _e1742);
                            let _e1750 = (1f + (((_e1740 * _e1702.x) * _e1702.x) * _e1742));
                            let _e1751 = (_e1740 * _e1746);
                            let _e1753 = (-(_e1740) * _e1702.x);
                            let _e1756 = (_e1740 + ((_e1702.y * _e1702.y) * _e1742));
                            let _e1757 = -(_e1702.y);
                            if (_e1671.member == 0u) {
                                phi_33843_ = vec3<f32>(0f, 0f, 0f);
                                phi_33651_ = 1.5f;
                                phi_33572_ = vec3<f32>(0f, 0f, 0f);
                                phi_33495_ = vec3<f32>(_e1671.member_4.x, _e1671.member_4.y, _e1671.member_4.z);
                                phi_33416_ = 0u;
                                phi_33334_ = 1f;
                                phi_33257_ = 0u;
                                phi_33248_ = _e318.member_4.x;
                                phi_14663_ = type_71();
                                phi_14664_ = true;
                            } else {
                                if (_e1671.member == 1u) {
                                    let _e2306 = (_e1671.member_6.y * _e1671.member_6.y);
                                    if (_e2306 != _e2306) {
                                        phi_26758_ = true;
                                    } else {
                                        phi_26758_ = (_e1737 >= _e2306);
                                    }
                                    let _e2310 = phi_26758_;
                                    let _e2311 = select(_e2306, _e1737, _e2310);
                                    let _e2312 = (_e2311 >= 0.001f);
                                    if _e2312 {
                                        phi_33900_ = vec3<f32>(_e1671.member_4.x, _e1671.member_4.y, _e1671.member_4.z);
                                        phi_33694_ = _e1671.member_6.x;
                                        phi_33547_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33235_ = _e318.member_4.x;
                                        phi_14059_ = vec3<f32>();
                                        phi_14060_ = true;
                                    } else {
                                        let _e2314 = ((_e318.member_4.x * 747796405u) + 2891336453u);
                                        let _e2322 = (((_e2314 >> bitcast<u32>((((_e2314 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2314) * 277803737u);
                                        let _e2325 = ((_e2322 >> bitcast<u32>(22i)) ^ _e2322);
                                        let _e2335 = abs((((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z)));
                                        let _e2338 = (1f / _e1671.member_6.x);
                                        let _e2342 = ((_e2338 * _e2338) * (1f - (_e2335 * _e2335)));
                                        if (_e2342 >= 1f) {
                                            phi_26798_ = 1f;
                                        } else {
                                            let _e2345 = sqrt((1f - _e2342));
                                            if (_e2335 != _e2335) {
                                                phi_26809_ = true;
                                            } else {
                                                phi_26809_ = (_e2345 <= _e2335);
                                            }
                                            let _e2349 = phi_26809_;
                                            let _e2354 = pow(((1f - _e1671.member_6.x) / (1f + _e1671.member_6.x)), 2f);
                                            phi_26798_ = (_e2354 + ((1f - _e2354) * pow((1f - select(_e2335, _e2345, _e2349)), 5f)));
                                        }
                                        let _e2361 = phi_26798_;
                                        let _e2362 = ((f32((_e2325 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e2361);
                                        if _e2362 {
                                            let _e2375 = (((_e1702.x * _e318.member_1.x) + (_e1702.y * _e318.member_1.y)) + (_e1702.z * _e318.member_1.z));
                                            phi_33548_ = vec3<f32>(0f, 0f, 0f);
                                            phi_14057_ = (_e334 - vec3<f32>(((_e1702.x * 2f) * _e2375), ((_e1702.y * 2f) * _e2375), ((_e1702.z * 2f) * _e2375)));
                                        } else {
                                            phi_33548_ = vec3<f32>(_e1671.member_4.x, _e1671.member_4.y, _e1671.member_4.z);
                                            phi_14057_ = vec3<f32>();
                                        }
                                        let _e2382 = phi_33548_;
                                        let _e2384 = phi_14057_;
                                        phi_33900_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33694_ = 1.5f;
                                        phi_33547_ = _e2382;
                                        phi_33235_ = _e2325;
                                        phi_14059_ = _e2384;
                                        phi_14060_ = select(true, false, _e2362);
                                    }
                                    let _e2394 = phi_33900_;
                                    let _e2396 = phi_33694_;
                                    let _e2398 = phi_33547_;
                                    let _e2400 = phi_33235_;
                                    let _e2402 = phi_14059_;
                                    let _e2404 = phi_14060_;
                                    phi_33846_ = _e2394;
                                    phi_33654_ = _e2396;
                                    phi_33575_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33498_ = _e2398;
                                    phi_33419_ = select(0u, 1u, _e2312);
                                    phi_33337_ = select(1f, _e2311, _e2312);
                                    phi_33260_ = select(0u, 2u, _e2312);
                                    phi_33186_ = _e2400;
                                    phi_14533_ = _e1730;
                                    phi_14534_ = _e2402;
                                    phi_14535_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14536_ = _e2404;
                                } else {
                                    if (_e1671.member == 3u) {
                                        phi_33189_ = _e318.member_4.x;
                                        phi_14212_ = vec3<f32>();
                                        phi_14213_ = vec3<f32>();
                                        phi_14214_ = vec3<f32>();
                                        phi_14215_ = true;
                                        phi_14216_ = false;
                                    } else {
                                        let _e1761 = (_e1671.member == 5u);
                                        if _e1761 {
                                            phi_33190_ = _e318.member_4.x;
                                            phi_14208_ = vec3<f32>();
                                            phi_14209_ = vec3<f32>();
                                            phi_14210_ = vec3<f32>();
                                            phi_14211_ = false;
                                        } else {
                                            let _e1762 = (_e1671.member == 2u);
                                            if _e1762 {
                                                let _e1767 = (((_e318.member_1.x * _e1156.member_3.x) + (_e318.member_1.y * _e1156.member_3.y)) + (_e318.member_1.z * _e1156.member_3.z));
                                                let _e1768 = (_e1767 > 0f);
                                                if _e1768 {
                                                    phi_14093_ = -(_e1156.member_3);
                                                } else {
                                                    phi_14093_ = _e1156.member_3;
                                                }
                                                let _e1771 = phi_14093_;
                                                if _e1768 {
                                                    phi_14099_ = _e1671.member_6.x;
                                                } else {
                                                    phi_14099_ = (1f / _e1671.member_6.x);
                                                }
                                                let _e1776 = phi_14099_;
                                                let _e1777 = abs(_e1767);
                                                let _e1778 = (_e1776 * _e1776);
                                                let _e1781 = (_e1778 * (1f - (_e1777 * _e1777)));
                                                if (_e1781 >= 1f) {
                                                    phi_26128_ = 1f;
                                                } else {
                                                    let _e1784 = sqrt((1f - _e1781));
                                                    if (_e1777 != _e1777) {
                                                        phi_26139_ = true;
                                                    } else {
                                                        phi_26139_ = (_e1784 <= _e1777);
                                                    }
                                                    let _e1788 = phi_26139_;
                                                    let _e1793 = pow(((1f - _e1671.member_6.x) / (1f + _e1671.member_6.x)), 2f);
                                                    phi_26128_ = (_e1793 + ((1f - _e1793) * pow((1f - select(_e1777, _e1784, _e1788)), 5f)));
                                                }
                                                let _e1800 = phi_26128_;
                                                let _e1808 = (((_e318.member_1.x * _e1771.x) + (_e318.member_1.y * _e1771.y)) + (_e318.member_1.z * _e1771.z));
                                                let _e1812 = (1f - (_e1778 * (1f - (_e1808 * _e1808))));
                                                if (_e1812 > 0f) {
                                                    let _e1815 = ((_e318.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1823 = (((_e1815 >> bitcast<u32>((((_e1815 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1815) * 277803737u);
                                                    let _e1826 = ((_e1823 >> bitcast<u32>(22i)) ^ _e1823);
                                                    let _e1831 = ((f32((_e1826 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1800);
                                                    if _e1831 {
                                                        let _e1841 = sqrt(_e1812);
                                                        let _e1845 = (((_e318.member_1.x - (_e1771.x * _e1808)) * _e1776) - (_e1771.x * _e1841));
                                                        let _e1846 = (((_e318.member_1.y - (_e1771.y * _e1808)) * _e1776) - (_e1771.y * _e1841));
                                                        let _e1847 = (((_e318.member_1.z - (_e1771.z * _e1808)) * _e1776) - (_e1771.z * _e1841));
                                                        let _e1854 = (1f / sqrt((((_e1845 * _e1845) + (_e1846 * _e1846)) + (_e1847 * _e1847))));
                                                        phi_14180_ = vec3<f32>((_e1604 - (_e1771.x * _e1720)), (_e1605 - (_e1771.y * _e1720)), (_e1606 - (_e1771.z * _e1720)));
                                                        phi_14181_ = vec3<f32>((_e1845 * _e1854), (_e1846 * _e1854), (_e1847 * _e1854));
                                                        phi_14182_ = vec3<f32>(_e1671.member_4.x, _e1671.member_4.y, _e1671.member_4.z);
                                                    } else {
                                                        phi_14180_ = vec3<f32>();
                                                        phi_14181_ = vec3<f32>();
                                                        phi_14182_ = vec3<f32>();
                                                    }
                                                    let _e1872 = phi_14180_;
                                                    let _e1874 = phi_14181_;
                                                    let _e1876 = phi_14182_;
                                                    phi_33193_ = _e1826;
                                                    phi_14184_ = _e1872;
                                                    phi_14185_ = _e1874;
                                                    phi_14186_ = _e1876;
                                                    phi_14187_ = select(true, false, _e1831);
                                                } else {
                                                    phi_33193_ = _e318.member_4.x;
                                                    phi_14184_ = vec3<f32>();
                                                    phi_14185_ = vec3<f32>();
                                                    phi_14186_ = vec3<f32>();
                                                    phi_14187_ = true;
                                                }
                                                let _e1879 = phi_33193_;
                                                let _e1881 = phi_14184_;
                                                let _e1883 = phi_14185_;
                                                let _e1885 = phi_14186_;
                                                let _e1887 = phi_14187_;
                                                if _e1887 {
                                                    let _e1895 = (((_e1156.member_3.x * _e318.member_1.x) + (_e1156.member_3.y * _e318.member_1.y)) + (_e1156.member_3.z * _e318.member_1.z));
                                                    phi_14202_ = vec3<f32>((_e1604 + (_e1771.x * _e1720)), (_e1605 + (_e1771.y * _e1720)), (_e1606 + (_e1771.z * _e1720)));
                                                    phi_14203_ = (_e334 - vec3<f32>(((_e1156.member_3.x * 2f) * _e1895), ((_e1156.member_3.y * 2f) * _e1895), ((_e1156.member_3.z * 2f) * _e1895)));
                                                } else {
                                                    phi_14202_ = _e1881;
                                                    phi_14203_ = _e1883;
                                                }
                                                let _e1909 = phi_14202_;
                                                let _e1911 = phi_14203_;
                                                phi_33191_ = _e1879;
                                                phi_14205_ = _e1909;
                                                phi_14206_ = _e1911;
                                                phi_14207_ = select(_e1885, vec3<f32>(1f, 1f, 1f), vec3(_e1887));
                                            } else {
                                                phi_33191_ = _e318.member_4.x;
                                                phi_14205_ = vec3<f32>();
                                                phi_14206_ = vec3<f32>();
                                                phi_14207_ = vec3<f32>();
                                            }
                                            let _e1915 = phi_33191_;
                                            let _e1917 = phi_14205_;
                                            let _e1919 = phi_14206_;
                                            let _e1921 = phi_14207_;
                                            phi_33190_ = _e1915;
                                            phi_14208_ = _e1917;
                                            phi_14209_ = _e1919;
                                            phi_14210_ = _e1921;
                                            phi_14211_ = _e1762;
                                        }
                                        let _e1923 = phi_33190_;
                                        let _e1925 = phi_14208_;
                                        let _e1927 = phi_14209_;
                                        let _e1929 = phi_14210_;
                                        let _e1931 = phi_14211_;
                                        phi_33189_ = _e1923;
                                        phi_14212_ = _e1925;
                                        phi_14213_ = _e1927;
                                        phi_14214_ = _e1929;
                                        phi_14215_ = _e1761;
                                        phi_14216_ = _e1931;
                                    }
                                    let _e1933 = phi_33189_;
                                    let _e1935 = phi_14212_;
                                    let _e1937 = phi_14213_;
                                    let _e1939 = phi_14214_;
                                    let _e1941 = phi_14215_;
                                    let _e1943 = phi_14216_;
                                    if _e1941 {
                                        let _e1946 = (_e1671.member_6.y * _e1671.member_6.y);
                                        if (_e1946 != _e1946) {
                                            phi_26212_ = true;
                                        } else {
                                            phi_26212_ = (_e1737 >= _e1946);
                                        }
                                        let _e1950 = phi_26212_;
                                        let _e1951 = select(_e1946, _e1737, _e1950);
                                        let _e1963 = select(2u, 0u, (_e1671.member == 5u));
                                        let _e1964 = (_e1951 < 0.001f);
                                        if _e1964 {
                                            let _e1972 = (((_e1702.x * _e318.member_1.x) + (_e1702.y * _e318.member_1.y)) + (_e1702.z * _e318.member_1.z));
                                            let _e1983 = abs((((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z)));
                                            if (_e1963 == 0u) {
                                                let _e2265 = (1f - _e1983);
                                                if (_e2265 != _e2265) {
                                                    phi_26743_ = true;
                                                } else {
                                                    phi_26743_ = (0f >= _e2265);
                                                }
                                                let _e2269 = phi_26743_;
                                                let _e2271 = pow(select(_e2265, 0f, _e2269), 5f);
                                                let _e2272 = (1f - _e2271);
                                                phi_14519_ = vec3<f32>(((_e1671.member_4.x * _e2272) + _e2271), ((_e1671.member_4.y * _e2272) + _e2271), ((_e1671.member_4.z * _e2272) + _e2271));
                                            } else {
                                                if (_e1963 == 1u) {
                                                    let _e2247 = (0.44444448f * (1f - (_e1983 * _e1983)));
                                                    if (_e2247 >= 1f) {
                                                        phi_26704_ = 1f;
                                                    } else {
                                                        let _e2250 = sqrt((1f - _e2247));
                                                        if (_e1983 != _e1983) {
                                                            phi_26715_ = true;
                                                        } else {
                                                            phi_26715_ = (_e2250 <= _e1983);
                                                        }
                                                        let _e2254 = phi_26715_;
                                                        phi_26704_ = (0.040000003f + (0.96f * pow((1f - select(_e1983, _e2250, _e2254)), 5f)));
                                                    }
                                                    let _e2261 = phi_26704_;
                                                    phi_14518_ = vec3(_e2261);
                                                } else {
                                                    let _e1987 = select(_e1983, 0f, (_e1983 < 0f));
                                                    let _e1989 = select(_e1987, 1f, (_e1987 > 1f));
                                                    let _e1991 = (1f - (_e1989 * _e1989));
                                                    let _e1998 = ((_e1671.member_4.x * _e1671.member_4.x) - (_e1671.member_5.x * _e1671.member_5.x));
                                                    let _e1999 = ((_e1671.member_4.y * _e1671.member_4.y) - (_e1671.member_5.y * _e1671.member_5.y));
                                                    let _e2000 = ((_e1671.member_4.z * _e1671.member_4.z) - (_e1671.member_5.z * _e1671.member_5.z));
                                                    let _e2002 = (_e1671.member_4.x * (_e1671.member_5.x + _e1671.member_5.x));
                                                    let _e2004 = (_e1671.member_4.y * (_e1671.member_5.y + _e1671.member_5.y));
                                                    let _e2006 = (_e1671.member_4.z * (_e1671.member_5.z + _e1671.member_5.z));
                                                    let _e2013 = ((_e1998 * _e1998) + (_e2002 * _e2002));
                                                    let _e2014 = ((_e1999 * _e1999) + (_e2004 * _e2004));
                                                    let _e2015 = ((_e2000 * _e2000) + (_e2006 * _e2006));
                                                    let _e2020 = ((_e1991 * _e1998) / _e2013);
                                                    let _e2021 = ((_e1991 * _e1999) / _e2014);
                                                    let _e2022 = ((_e1991 * _e2000) / _e2015);
                                                    let _e2023 = (1f - _e2020);
                                                    let _e2024 = (1f - _e2021);
                                                    let _e2025 = (1f - _e2022);
                                                    let _e2027 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2020, _e2021, _e2022));
                                                    let _e2028 = (_e1991 * _e2002);
                                                    let _e2029 = (_e1991 * _e2004);
                                                    let _e2030 = (_e1991 * _e2006);
                                                    let _e2035 = (vec3<f32>(_e2028, _e2029, _e2030) / vec3<f32>(_e2013, _e2014, _e2015));
                                                    let _e2048 = ((_e2027.x * _e2027.x) + (_e2035.x * _e2035.x));
                                                    let _e2049 = ((_e2027.y * _e2027.y) + (_e2035.y * _e2035.y));
                                                    let _e2050 = ((_e2027.z * _e2027.z) + (_e2035.z * _e2035.z));
                                                    if (_e2048 != _e2048) {
                                                        phi_26374_ = true;
                                                    } else {
                                                        phi_26374_ = (0f >= _e2048);
                                                    }
                                                    let _e2054 = phi_26374_;
                                                    let _e2056 = sqrt(select(_e2048, 0f, _e2054));
                                                    if (_e2049 != _e2049) {
                                                        phi_26389_ = true;
                                                    } else {
                                                        phi_26389_ = (0f >= _e2049);
                                                    }
                                                    let _e2060 = phi_26389_;
                                                    let _e2062 = sqrt(select(_e2049, 0f, _e2060));
                                                    if (_e2050 != _e2050) {
                                                        phi_26404_ = true;
                                                    } else {
                                                        phi_26404_ = (0f >= _e2050);
                                                    }
                                                    let _e2066 = phi_26404_;
                                                    let _e2068 = sqrt(select(_e2050, 0f, _e2066));
                                                    let _e2072 = ((_e2056 + _e2023) * 0.5f);
                                                    let _e2073 = ((_e2062 + _e2024) * 0.5f);
                                                    let _e2074 = ((_e2068 + _e2025) * 0.5f);
                                                    if (_e2072 != _e2072) {
                                                        phi_26431_ = true;
                                                    } else {
                                                        phi_26431_ = (0f >= _e2072);
                                                    }
                                                    let _e2078 = phi_26431_;
                                                    let _e2080 = sqrt(select(_e2072, 0f, _e2078));
                                                    if (_e2073 != _e2073) {
                                                        phi_26446_ = true;
                                                    } else {
                                                        phi_26446_ = (0f >= _e2073);
                                                    }
                                                    let _e2084 = phi_26446_;
                                                    let _e2086 = sqrt(select(_e2073, 0f, _e2084));
                                                    if (_e2074 != _e2074) {
                                                        phi_26461_ = true;
                                                    } else {
                                                        phi_26461_ = (0f >= _e2074);
                                                    }
                                                    let _e2090 = phi_26461_;
                                                    let _e2092 = sqrt(select(_e2074, 0f, _e2090));
                                                    let _e2093 = vec3<f32>(_e2080, _e2086, _e2092);
                                                    let _e2097 = ((_e2056 - _e2023) * 0.5f);
                                                    let _e2098 = ((_e2062 - _e2024) * 0.5f);
                                                    let _e2099 = ((_e2068 - _e2025) * 0.5f);
                                                    if (_e2097 != _e2097) {
                                                        phi_26488_ = true;
                                                    } else {
                                                        phi_26488_ = (0f >= _e2097);
                                                    }
                                                    let _e2103 = phi_26488_;
                                                    let _e2105 = sqrt(select(_e2097, 0f, _e2103));
                                                    if (_e2098 != _e2098) {
                                                        phi_26503_ = true;
                                                    } else {
                                                        phi_26503_ = (0f >= _e2098);
                                                    }
                                                    let _e2109 = phi_26503_;
                                                    let _e2111 = sqrt(select(_e2098, 0f, _e2109));
                                                    if (_e2099 != _e2099) {
                                                        phi_26518_ = true;
                                                    } else {
                                                        phi_26518_ = (0f >= _e2099);
                                                    }
                                                    let _e2115 = phi_26518_;
                                                    let _e2117 = sqrt(select(_e2099, 0f, _e2115));
                                                    if ((_e2028 / _e2013) < 0f) {
                                                        phi_14395_ = -(_e2105);
                                                    } else {
                                                        phi_14395_ = _e2105;
                                                    }
                                                    let _e2121 = phi_14395_;
                                                    if ((_e2029 / _e2014) < 0f) {
                                                        phi_14403_ = -(_e2111);
                                                    } else {
                                                        phi_14403_ = _e2111;
                                                    }
                                                    let _e2125 = phi_14403_;
                                                    if ((_e2030 / _e2015) < 0f) {
                                                        phi_14411_ = -(_e2117);
                                                    } else {
                                                        phi_14411_ = _e2117;
                                                    }
                                                    let _e2129 = phi_14411_;
                                                    let _e2133 = (_e1671.member_5.x * _e1989);
                                                    let _e2134 = (_e1671.member_5.y * _e1989);
                                                    let _e2135 = (_e1671.member_5.z * _e1989);
                                                    let _e2145 = vec3<f32>(((_e1671.member_4.x * _e2080) - (_e1671.member_5.x * _e2121)), ((_e1671.member_4.y * _e2086) - (_e1671.member_5.y * _e2125)), ((_e1671.member_4.z * _e2092) - (_e1671.member_5.z * _e2129)));
                                                    let _e2152 = ((_e1671.member_4.x * _e2121) + (_e1671.member_5.x * _e2080));
                                                    let _e2153 = ((_e1671.member_4.y * _e2125) + (_e1671.member_5.y * _e2086));
                                                    let _e2154 = ((_e1671.member_4.z * _e2129) + (_e1671.member_5.z * _e2092));
                                                    let _e2156 = vec3<f32>((_e1671.member_4.x * _e1989), (_e1671.member_4.y * _e1989), (_e1671.member_4.z * _e1989));
                                                    let _e2157 = (_e2156 - _e2093);
                                                    let _e2158 = (_e2133 - _e2121);
                                                    let _e2159 = (_e2134 - _e2125);
                                                    let _e2160 = (_e2135 - _e2129);
                                                    let _e2173 = (_e2156 + _e2093);
                                                    let _e2174 = (_e2133 + _e2121);
                                                    let _e2175 = (_e2134 + _e2125);
                                                    let _e2176 = (_e2135 + _e2129);
                                                    let _e2186 = ((_e2173.x * _e2173.x) + (_e2174 * _e2174));
                                                    let _e2187 = ((_e2173.y * _e2173.y) + (_e2175 * _e2175));
                                                    let _e2188 = ((_e2173.z * _e2173.z) + (_e2176 * _e2176));
                                                    let _e2198 = vec3(_e1989);
                                                    let _e2199 = (_e2198 - _e2145);
                                                    let _e2200 = -(vec3<f32>(_e2152, _e2153, _e2154));
                                                    let _e2216 = (_e2198 + _e2145);
                                                    let _e2226 = ((_e2216.x * _e2216.x) + (_e2152 * _e2152));
                                                    let _e2227 = ((_e2216.y * _e2216.y) + (_e2153 * _e2153));
                                                    let _e2228 = ((_e2216.z * _e2216.z) + (_e2154 * _e2154));
                                                    phi_14518_ = vec3<f32>((((((_e2157.x * _e2157.x) + (_e2158 * _e2158)) / select(0.000000000000000000000000000001f, _e2186, (_e2186 > 0.000000000000000000000000000001f))) + (((_e2199.x * _e2199.x) + (_e2200.x * _e2200.x)) / select(0.000000000000000000000000000001f, _e2226, (_e2226 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2157.y * _e2157.y) + (_e2159 * _e2159)) / select(0.000000000000000000000000000001f, _e2187, (_e2187 > 0.000000000000000000000000000001f))) + (((_e2199.y * _e2199.y) + (_e2200.y * _e2200.y)) / select(0.000000000000000000000000000001f, _e2227, (_e2227 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2157.z * _e2157.z) + (_e2160 * _e2160)) / select(0.000000000000000000000000000001f, _e2188, (_e2188 > 0.000000000000000000000000000001f))) + (((_e2199.z * _e2199.z) + (_e2200.z * _e2200.z)) / select(0.000000000000000000000000000001f, _e2228, (_e2228 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e2264 = phi_14518_;
                                                phi_14519_ = _e2264;
                                            }
                                            let _e2281 = phi_14519_;
                                            phi_14520_ = (_e334 - vec3<f32>(((_e1702.x * 2f) * _e1972), ((_e1702.y * 2f) * _e1972), ((_e1702.z * 2f) * _e1972)));
                                            phi_14521_ = _e2281;
                                        } else {
                                            phi_14520_ = vec3<f32>();
                                            phi_14521_ = vec3<f32>();
                                        }
                                        let _e2283 = phi_14520_;
                                        let _e2285 = phi_14521_;
                                        phi_33577_ = vec3<f32>(_e1671.member_5.x, _e1671.member_5.y, _e1671.member_5.z);
                                        phi_33500_ = vec3<f32>(_e1671.member_4.x, _e1671.member_4.y, _e1671.member_4.z);
                                        phi_33421_ = _e1963;
                                        phi_33339_ = _e1951;
                                        phi_14523_ = _e2283;
                                        phi_14524_ = _e2285;
                                        phi_14525_ = _e1964;
                                    } else {
                                        phi_33577_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33500_ = vec3<f32>(0f, 0f, 0f);
                                        phi_33421_ = 0u;
                                        phi_33339_ = 1f;
                                        phi_14523_ = _e1937;
                                        phi_14524_ = _e1939;
                                        phi_14525_ = _e1943;
                                    }
                                    let _e2287 = phi_33577_;
                                    let _e2289 = phi_33500_;
                                    let _e2291 = phi_33421_;
                                    let _e2293 = phi_33339_;
                                    let _e2295 = phi_14523_;
                                    let _e2297 = phi_14524_;
                                    let _e2299 = phi_14525_;
                                    phi_33846_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33654_ = 1.5f;
                                    phi_33575_ = _e2287;
                                    phi_33498_ = _e2289;
                                    phi_33419_ = _e2291;
                                    phi_33337_ = _e2293;
                                    phi_33260_ = select(0u, 1u, _e1941);
                                    phi_33186_ = _e1933;
                                    phi_14533_ = select(_e1935, _e1730, vec3(_e1941));
                                    phi_14534_ = _e2295;
                                    phi_14535_ = _e2297;
                                    phi_14536_ = select(true, false, _e2299);
                                }
                                let _e2409 = phi_33846_;
                                let _e2411 = phi_33654_;
                                let _e2413 = phi_33575_;
                                let _e2415 = phi_33498_;
                                let _e2417 = phi_33419_;
                                let _e2419 = phi_33337_;
                                let _e2421 = phi_33260_;
                                let _e2423 = phi_33186_;
                                let _e2425 = phi_14533_;
                                let _e2427 = phi_14534_;
                                let _e2429 = phi_14535_;
                                let _e2431 = phi_14536_;
                                let _e2432 = select(true, false, _e2431);
                                if _e2432 {
                                    if (_e345 >= _e361) {
                                        phi_33245_ = _e2423;
                                        phi_14652_ = _e1597;
                                        phi_14653_ = type_71();
                                        phi_14654_ = true;
                                    } else {
                                        let _e2445 = global_1.member.member_4[1u];
                                        if (sqrt((((_e2429.x * _e2429.x) + (_e2429.y * _e2429.y)) + (_e2429.z * _e2429.z))) <= _e2445) {
                                            phi_33240_ = _e2423;
                                            phi_14616_ = _e1597;
                                            phi_14617_ = false;
                                        } else {
                                            let _e2449 = global_1.member.member_7[1u];
                                            if (_e2449 == 1u) {
                                                let _e2454 = global_1.member.member_4[3u];
                                                if (f32(_e345) >= _e2454) {
                                                    let _e2457 = select(_e2429.y, _e2429.x, (_e2429.x > _e2429.y));
                                                    let _e2459 = select(_e2429.z, _e2457, (_e2457 > _e2429.z));
                                                    if (_e2459 != _e2459) {
                                                        phi_26875_ = true;
                                                    } else {
                                                        phi_26875_ = (1f <= _e2459);
                                                    }
                                                    let _e2463 = phi_26875_;
                                                    let _e2464 = select(_e2459, 1f, _e2463);
                                                    let _e2466 = ((_e2423 * 747796405u) + 2891336453u);
                                                    let _e2474 = (((_e2466 >> bitcast<u32>((((_e2466 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2466) * 277803737u);
                                                    let _e2477 = ((_e2474 >> bitcast<u32>(22i)) ^ _e2474);
                                                    let _e2482 = ((f32((_e2477 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e2464);
                                                    if _e2482 {
                                                        phi_14605_ = vec3<f32>();
                                                    } else {
                                                        phi_14605_ = vec3<f32>((_e1597.x / _e2464), (_e1597.y / _e2464), (_e1597.z / _e2464));
                                                    }
                                                    let _e2491 = phi_14605_;
                                                    phi_33243_ = _e2477;
                                                    phi_14607_ = _e2491;
                                                    phi_14608_ = select(true, false, _e2482);
                                                } else {
                                                    phi_33243_ = _e2423;
                                                    phi_14607_ = _e1597;
                                                    phi_14608_ = true;
                                                }
                                                let _e2494 = phi_33243_;
                                                let _e2496 = phi_14607_;
                                                let _e2498 = phi_14608_;
                                                phi_33242_ = _e2494;
                                                phi_14609_ = _e2496;
                                                phi_14610_ = _e2498;
                                            } else {
                                                phi_33242_ = _e2423;
                                                phi_14609_ = _e1597;
                                                phi_14610_ = true;
                                            }
                                            let _e2500 = phi_33242_;
                                            let _e2502 = phi_14609_;
                                            let _e2504 = phi_14610_;
                                            phi_33240_ = _e2500;
                                            phi_14616_ = select(_e1597, _e2502, vec3(_e2504));
                                            phi_14617_ = select(false, true, _e2504);
                                        }
                                        let _e2509 = phi_33240_;
                                        let _e2511 = phi_14616_;
                                        let _e2513 = phi_14617_;
                                        if _e2513 {
                                            let _e2514 = (_e2511 * _e2429);
                                            let _e2525 = (_e2427 * (1f / sqrt((((_e2427.x * _e2427.x) + (_e2427.y * _e2427.y)) + (_e2427.z * _e2427.z)))));
                                            phi_14650_ = type_71(type_35(vec4<f32>(_e2425.x, _e2425.y, _e2425.z, _e318.member.w), vec4<f32>(_e2525.x, _e2525.y, _e2525.z, _e318.member_1.w), vec4<f32>(_e2514.x, _e2514.y, _e2514.z, f32((_e345 + 1u))), vec4<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z, f32(_e1157)), vec4<u32>(_e2509, (_e318.member_4.y | 2u), 0u, 0u)), _e1599, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14650_ = type_71();
                                        }
                                        let _e2548 = phi_14650_;
                                        phi_33245_ = _e2509;
                                        phi_14652_ = _e2511;
                                        phi_14653_ = _e2548;
                                        phi_14654_ = select(true, false, _e2513);
                                    }
                                    let _e2551 = phi_33245_;
                                    let _e2553 = phi_14652_;
                                    let _e2555 = phi_14653_;
                                    let _e2557 = phi_14654_;
                                    if _e2557 {
                                        phi_14660_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e2553.x, _e2553.y, _e2553.z, f32(_e345)), vec4<f32>(_e318.member_3.x, _e318.member_3.y, _e318.member_3.z, f32(_e1157)), vec4<u32>(_e2551, (_e318.member_4.y & 4294967294u), 0u, 0u)), _e1599, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14660_ = _e2555;
                                    }
                                    let _e2570 = phi_14660_;
                                    phi_33249_ = _e2551;
                                    phi_14661_ = _e2570;
                                } else {
                                    phi_33249_ = _e2423;
                                    phi_14661_ = type_71();
                                }
                                let _e2572 = phi_33249_;
                                let _e2574 = phi_14661_;
                                phi_33843_ = _e2409;
                                phi_33651_ = _e2411;
                                phi_33572_ = _e2413;
                                phi_33495_ = _e2415;
                                phi_33416_ = _e2417;
                                phi_33334_ = _e2419;
                                phi_33257_ = _e2421;
                                phi_33248_ = _e2572;
                                phi_14663_ = _e2574;
                                phi_14664_ = select(true, false, _e2432);
                            }
                            let _e2582 = phi_33843_;
                            let _e2584 = phi_33651_;
                            let _e2586 = phi_33572_;
                            let _e2588 = phi_33495_;
                            let _e2590 = phi_33416_;
                            let _e2592 = phi_33334_;
                            let _e2594 = phi_33257_;
                            let _e2596 = phi_33248_;
                            let _e2598 = phi_14663_;
                            let _e2600 = phi_14664_;
                            if _e2600 {
                                let _e2602 = ((_e318.member_4.y | 4u) & 4294967293u);
                                let _e2605 = global_1.member.member_1[3u];
                                phi_35172_ = _e1595;
                                phi_33247_ = _e2596;
                                phi_14675_ = _e349;
                                phi_14678_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14680_ = 0u;
                                phi_14682_ = _e1157;
                                loop {
                                    let _e2607 = phi_35172_;
                                    let _e2609 = phi_33247_;
                                    let _e2611 = phi_14675_;
                                    let _e2613 = phi_14678_;
                                    let _e2615 = phi_14680_;
                                    let _e2617 = phi_14682_;
                                    local_87 = _e2617;
                                    local_88 = _e2611;
                                    local_89 = _e2611;
                                    local_90 = _e2611;
                                    local_91 = _e2613;
                                    local_92 = _e2617;
                                    local_93 = _e2611;
                                    local_94 = _e2611;
                                    local_95 = _e2611;
                                    local_96 = _e2613;
                                    local_97 = _e2617;
                                    local_98 = _e2611;
                                    local_99 = _e2611;
                                    local_100 = _e2611;
                                    local_101 = _e2613;
                                    local_102 = _e2617;
                                    local_103 = _e2611;
                                    local_104 = _e2611;
                                    local_105 = _e2611;
                                    local_106 = _e2613;
                                    let _e2619 = (_e2615 < (_e2605 + 3u));
                                    if _e2619 {
                                        if (_e2615 < _e2605) {
                                            let _e4604 = global_5.member[_e2615];
                                            let _e4609 = (_e4604.member.x - _e1727);
                                            let _e4610 = (_e4604.member.y - _e1728);
                                            let _e4611 = (_e4604.member.z - _e1729);
                                            let _e4616 = (((_e4609 * _e4609) + (_e4610 * _e4610)) + (_e4611 * _e4611));
                                            let _e4617 = sqrt(_e4616);
                                            let _e4618 = (_e4609 / _e4617);
                                            let _e4619 = (_e4610 / _e4617);
                                            let _e4620 = (_e4611 / _e4617);
                                            if (_e4604.member.w < 0.5f) {
                                                phi_14779_ = 1f;
                                            } else {
                                                let _e4644 = (((_e4604.member_2.x * -(_e4618)) + (_e4604.member_2.y * -(_e4619))) + (_e4604.member_2.z * -(_e4620)));
                                                if (_e4604.member_3.x == _e4604.member_2.w) {
                                                    phi_14778_ = select(1f, 0f, (_e4644 < _e4604.member_3.x));
                                                } else {
                                                    let _e4648 = ((_e4644 - _e4604.member_3.x) / (_e4604.member_2.w - _e4604.member_3.x));
                                                    let _e4650 = select(_e4648, 0f, (_e4648 < 0f));
                                                    let _e4652 = select(_e4650, 1f, (_e4650 > 1f));
                                                    phi_14778_ = ((_e4652 * _e4652) * (3f - (2f * _e4652)));
                                                }
                                                let _e4660 = phi_14778_;
                                                phi_14779_ = _e4660;
                                            }
                                            let _e4662 = phi_14779_;
                                            phi_35588_ = _e2609;
                                            phi_17717_ = 0f;
                                            phi_17718_ = vec3<f32>(_e4618, _e4619, _e4620);
                                            phi_17719_ = vec3<f32>(((_e4604.member_1.x * _e4662) / _e4616), ((_e4604.member_1.y * _e4662) / _e4616), ((_e4604.member_1.z * _e4662) / _e4616));
                                            phi_17720_ = _e4617;
                                        } else {
                                            if (_e2615 == _e2605) {
                                                if ((_e364 & 4u) == 0u) {
                                                    phi_35632_ = _e2609;
                                                    phi_16821_ = 0f;
                                                    phi_16822_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16823_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e3356 = global_1.member.member_5;
                                                    let _e3364 = global_1.member.member_6[3u];
                                                    let _e3366 = sin((_e3364 * 0.5f));
                                                    let _e3368 = ((12.566371f * _e3366) * _e3366);
                                                    let _e3373 = (((_e3356.x * _e1702.x) + (_e3356.y * _e1702.y)) + (_e3356.z * _e1702.z));
                                                    if (_e3373 != _e3373) {
                                                        phi_27414_ = true;
                                                    } else {
                                                        phi_27414_ = (0f >= _e3373);
                                                    }
                                                    let _e3377 = phi_27414_;
                                                    let _e3379 = (select(_e3373, 0f, _e3377) * 0.31830987f);
                                                    if (_e2594 == 0u) {
                                                        if (_e3373 <= 0f) {
                                                            phi_28211_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28211_ = (_e2588 * (_e3373 * 0.31830987f));
                                                        }
                                                        let _e3855 = phi_28211_;
                                                        phi_15189_ = type_56(_e3855, _e3379);
                                                        phi_15190_ = type_56();
                                                        phi_15191_ = true;
                                                    } else {
                                                        let _e3385 = (((_e1674 * _e1750) + (_e1675 * _e1751)) + (_e1676 * _e1753));
                                                        let _e3390 = (((_e1674 * _e1746) + (_e1675 * _e1756)) + (_e1676 * _e1757));
                                                        let _e3395 = (((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z));
                                                        let _e3400 = (((_e3356.x * _e1750) + (_e3356.y * _e1751)) + (_e3356.z * _e1753));
                                                        let _e3405 = (((_e3356.x * _e1746) + (_e3356.y * _e1756)) + (_e3356.z * _e1757));
                                                        if (_e3395 <= 0f) {
                                                            phi_15153_ = type_56();
                                                            phi_15154_ = true;
                                                        } else {
                                                            let _e3407 = (_e3373 <= 0f);
                                                            if _e3407 {
                                                                phi_15152_ = type_56();
                                                            } else {
                                                                let _e3408 = (_e3385 + _e3400);
                                                                let _e3409 = (_e3390 + _e3405);
                                                                let _e3410 = (_e3395 + _e3373);
                                                                let _e3417 = (1f / sqrt((((_e3408 * _e3408) + (_e3409 * _e3409)) + (_e3410 * _e3410))));
                                                                let _e3420 = (_e3410 * _e3417);
                                                                if (_e3420 <= 0f) {
                                                                    phi_27524_ = 0f;
                                                                } else {
                                                                    let _e3422 = (_e2592 * _e2592);
                                                                    let _e3426 = ((((_e3422 - 1f) * _e3420) * _e3420) + 1f);
                                                                    phi_27524_ = (_e3422 / ((3.1415927f * _e3426) * _e3426));
                                                                }
                                                                let _e3431 = phi_27524_;
                                                                let _e3432 = (_e3395 * _e3395);
                                                                if (_e3432 >= 1f) {
                                                                    phi_27552_ = 0f;
                                                                } else {
                                                                    phi_27552_ = ((sqrt((1f + (((_e2592 * _e2592) * ((_e3385 * _e3385) + (_e3390 * _e3390))) / _e3432))) - 1f) * 0.5f);
                                                                }
                                                                let _e3445 = phi_27552_;
                                                                let _e3446 = (1f + _e3445);
                                                                let _e3447 = (_e3373 * _e3373);
                                                                if (_e3447 >= 1f) {
                                                                    phi_27580_ = 0f;
                                                                } else {
                                                                    phi_27580_ = ((sqrt((1f + (((_e2592 * _e2592) * ((_e3400 * _e3400) + (_e3405 * _e3405))) / _e3447))) - 1f) * 0.5f);
                                                                }
                                                                let _e3460 = phi_27580_;
                                                                let _e3467 = (((_e3385 * (_e3408 * _e3417)) + (_e3390 * (_e3409 * _e3417))) + (_e3395 * _e3420));
                                                                if (_e2590 == 0u) {
                                                                    let _e3762 = (1f - _e3467);
                                                                    if (_e3762 != _e3762) {
                                                                        phi_28083_ = true;
                                                                    } else {
                                                                        phi_28083_ = (0f >= _e3762);
                                                                    }
                                                                    let _e3766 = phi_28083_;
                                                                    let _e3768 = pow(select(_e3762, 0f, _e3766), 5f);
                                                                    let _e3769 = (1f - _e3768);
                                                                    phi_15140_ = vec3<f32>(((_e2588.x * _e3769) + _e3768), ((_e2588.y * _e3769) + _e3768), ((_e2588.z * _e3769) + _e3768));
                                                                } else {
                                                                    if (_e2590 == 1u) {
                                                                        let _e3735 = (1f / _e2584);
                                                                        let _e3739 = ((_e3735 * _e3735) * (1f - (_e3467 * _e3467)));
                                                                        if (_e3739 >= 1f) {
                                                                            phi_28044_ = 1f;
                                                                        } else {
                                                                            let _e3742 = sqrt((1f - _e3739));
                                                                            if (_e3467 != _e3467) {
                                                                                phi_28055_ = true;
                                                                            } else {
                                                                                phi_28055_ = (_e3742 <= _e3467);
                                                                            }
                                                                            let _e3746 = phi_28055_;
                                                                            let _e3751 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                                            phi_28044_ = (_e3751 + ((1f - _e3751) * pow((1f - select(_e3467, _e3742, _e3746)), 5f)));
                                                                        }
                                                                        let _e3758 = phi_28044_;
                                                                        phi_15139_ = vec3(_e3758);
                                                                    } else {
                                                                        let _e3471 = select(_e3467, 0f, (_e3467 < 0f));
                                                                        let _e3473 = select(_e3471, 1f, (_e3471 > 1f));
                                                                        let _e3475 = (1f - (_e3473 * _e3473));
                                                                        let _e3488 = ((_e2588.x * _e2588.x) - (_e2586.x * _e2586.x));
                                                                        let _e3489 = ((_e2588.y * _e2588.y) - (_e2586.y * _e2586.y));
                                                                        let _e3490 = ((_e2588.z * _e2588.z) - (_e2586.z * _e2586.z));
                                                                        let _e3492 = (_e2588.x * (_e2586.x + _e2586.x));
                                                                        let _e3494 = (_e2588.y * (_e2586.y + _e2586.y));
                                                                        let _e3496 = (_e2588.z * (_e2586.z + _e2586.z));
                                                                        let _e3503 = ((_e3488 * _e3488) + (_e3492 * _e3492));
                                                                        let _e3504 = ((_e3489 * _e3489) + (_e3494 * _e3494));
                                                                        let _e3505 = ((_e3490 * _e3490) + (_e3496 * _e3496));
                                                                        let _e3510 = ((_e3475 * _e3488) / _e3503);
                                                                        let _e3511 = ((_e3475 * _e3489) / _e3504);
                                                                        let _e3512 = ((_e3475 * _e3490) / _e3505);
                                                                        let _e3513 = (1f - _e3510);
                                                                        let _e3514 = (1f - _e3511);
                                                                        let _e3515 = (1f - _e3512);
                                                                        let _e3517 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3510, _e3511, _e3512));
                                                                        let _e3518 = (_e3475 * _e3492);
                                                                        let _e3519 = (_e3475 * _e3494);
                                                                        let _e3520 = (_e3475 * _e3496);
                                                                        let _e3525 = (vec3<f32>(_e3518, _e3519, _e3520) / vec3<f32>(_e3503, _e3504, _e3505));
                                                                        let _e3538 = ((_e3517.x * _e3517.x) + (_e3525.x * _e3525.x));
                                                                        let _e3539 = ((_e3517.y * _e3517.y) + (_e3525.y * _e3525.y));
                                                                        let _e3540 = ((_e3517.z * _e3517.z) + (_e3525.z * _e3525.z));
                                                                        if (_e3538 != _e3538) {
                                                                            phi_27714_ = true;
                                                                        } else {
                                                                            phi_27714_ = (0f >= _e3538);
                                                                        }
                                                                        let _e3544 = phi_27714_;
                                                                        let _e3546 = sqrt(select(_e3538, 0f, _e3544));
                                                                        if (_e3539 != _e3539) {
                                                                            phi_27729_ = true;
                                                                        } else {
                                                                            phi_27729_ = (0f >= _e3539);
                                                                        }
                                                                        let _e3550 = phi_27729_;
                                                                        let _e3552 = sqrt(select(_e3539, 0f, _e3550));
                                                                        if (_e3540 != _e3540) {
                                                                            phi_27744_ = true;
                                                                        } else {
                                                                            phi_27744_ = (0f >= _e3540);
                                                                        }
                                                                        let _e3556 = phi_27744_;
                                                                        let _e3558 = sqrt(select(_e3540, 0f, _e3556));
                                                                        let _e3562 = ((_e3546 + _e3513) * 0.5f);
                                                                        let _e3563 = ((_e3552 + _e3514) * 0.5f);
                                                                        let _e3564 = ((_e3558 + _e3515) * 0.5f);
                                                                        if (_e3562 != _e3562) {
                                                                            phi_27771_ = true;
                                                                        } else {
                                                                            phi_27771_ = (0f >= _e3562);
                                                                        }
                                                                        let _e3568 = phi_27771_;
                                                                        let _e3570 = sqrt(select(_e3562, 0f, _e3568));
                                                                        if (_e3563 != _e3563) {
                                                                            phi_27786_ = true;
                                                                        } else {
                                                                            phi_27786_ = (0f >= _e3563);
                                                                        }
                                                                        let _e3574 = phi_27786_;
                                                                        let _e3576 = sqrt(select(_e3563, 0f, _e3574));
                                                                        if (_e3564 != _e3564) {
                                                                            phi_27801_ = true;
                                                                        } else {
                                                                            phi_27801_ = (0f >= _e3564);
                                                                        }
                                                                        let _e3580 = phi_27801_;
                                                                        let _e3582 = sqrt(select(_e3564, 0f, _e3580));
                                                                        let _e3583 = vec3<f32>(_e3570, _e3576, _e3582);
                                                                        let _e3587 = ((_e3546 - _e3513) * 0.5f);
                                                                        let _e3588 = ((_e3552 - _e3514) * 0.5f);
                                                                        let _e3589 = ((_e3558 - _e3515) * 0.5f);
                                                                        if (_e3587 != _e3587) {
                                                                            phi_27828_ = true;
                                                                        } else {
                                                                            phi_27828_ = (0f >= _e3587);
                                                                        }
                                                                        let _e3593 = phi_27828_;
                                                                        let _e3595 = sqrt(select(_e3587, 0f, _e3593));
                                                                        if (_e3588 != _e3588) {
                                                                            phi_27843_ = true;
                                                                        } else {
                                                                            phi_27843_ = (0f >= _e3588);
                                                                        }
                                                                        let _e3599 = phi_27843_;
                                                                        let _e3601 = sqrt(select(_e3588, 0f, _e3599));
                                                                        if (_e3589 != _e3589) {
                                                                            phi_27858_ = true;
                                                                        } else {
                                                                            phi_27858_ = (0f >= _e3589);
                                                                        }
                                                                        let _e3605 = phi_27858_;
                                                                        let _e3607 = sqrt(select(_e3589, 0f, _e3605));
                                                                        if ((_e3518 / _e3503) < 0f) {
                                                                            phi_15016_ = -(_e3595);
                                                                        } else {
                                                                            phi_15016_ = _e3595;
                                                                        }
                                                                        let _e3611 = phi_15016_;
                                                                        if ((_e3519 / _e3504) < 0f) {
                                                                            phi_15024_ = -(_e3601);
                                                                        } else {
                                                                            phi_15024_ = _e3601;
                                                                        }
                                                                        let _e3615 = phi_15024_;
                                                                        if ((_e3520 / _e3505) < 0f) {
                                                                            phi_15032_ = -(_e3607);
                                                                        } else {
                                                                            phi_15032_ = _e3607;
                                                                        }
                                                                        let _e3619 = phi_15032_;
                                                                        let _e3623 = (_e2586.x * _e3473);
                                                                        let _e3624 = (_e2586.y * _e3473);
                                                                        let _e3625 = (_e2586.z * _e3473);
                                                                        let _e3635 = vec3<f32>(((_e2588.x * _e3570) - (_e2586.x * _e3611)), ((_e2588.y * _e3576) - (_e2586.y * _e3615)), ((_e2588.z * _e3582) - (_e2586.z * _e3619)));
                                                                        let _e3642 = ((_e2588.x * _e3611) + (_e2586.x * _e3570));
                                                                        let _e3643 = ((_e2588.y * _e3615) + (_e2586.y * _e3576));
                                                                        let _e3644 = ((_e2588.z * _e3619) + (_e2586.z * _e3582));
                                                                        let _e3646 = vec3<f32>((_e2588.x * _e3473), (_e2588.y * _e3473), (_e2588.z * _e3473));
                                                                        let _e3647 = (_e3646 - _e3583);
                                                                        let _e3648 = (_e3623 - _e3611);
                                                                        let _e3649 = (_e3624 - _e3615);
                                                                        let _e3650 = (_e3625 - _e3619);
                                                                        let _e3663 = (_e3646 + _e3583);
                                                                        let _e3664 = (_e3623 + _e3611);
                                                                        let _e3665 = (_e3624 + _e3615);
                                                                        let _e3666 = (_e3625 + _e3619);
                                                                        let _e3676 = ((_e3663.x * _e3663.x) + (_e3664 * _e3664));
                                                                        let _e3677 = ((_e3663.y * _e3663.y) + (_e3665 * _e3665));
                                                                        let _e3678 = ((_e3663.z * _e3663.z) + (_e3666 * _e3666));
                                                                        let _e3688 = vec3(_e3473);
                                                                        let _e3689 = (_e3688 - _e3635);
                                                                        let _e3690 = -(vec3<f32>(_e3642, _e3643, _e3644));
                                                                        let _e3706 = (_e3688 + _e3635);
                                                                        let _e3716 = ((_e3706.x * _e3706.x) + (_e3642 * _e3642));
                                                                        let _e3717 = ((_e3706.y * _e3706.y) + (_e3643 * _e3643));
                                                                        let _e3718 = ((_e3706.z * _e3706.z) + (_e3644 * _e3644));
                                                                        phi_15139_ = vec3<f32>((((((_e3647.x * _e3647.x) + (_e3648 * _e3648)) / select(0.000000000000000000000000000001f, _e3676, (_e3676 > 0.000000000000000000000000000001f))) + (((_e3689.x * _e3689.x) + (_e3690.x * _e3690.x)) / select(0.000000000000000000000000000001f, _e3716, (_e3716 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3647.y * _e3647.y) + (_e3649 * _e3649)) / select(0.000000000000000000000000000001f, _e3677, (_e3677 > 0.000000000000000000000000000001f))) + (((_e3689.y * _e3689.y) + (_e3690.y * _e3690.y)) / select(0.000000000000000000000000000001f, _e3717, (_e3717 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3647.z * _e3647.z) + (_e3650 * _e3650)) / select(0.000000000000000000000000000001f, _e3678, (_e3678 > 0.000000000000000000000000000001f))) + (((_e3689.z * _e3689.z) + (_e3690.z * _e3690.z)) / select(0.000000000000000000000000000001f, _e3718, (_e3718 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3761 = phi_15139_;
                                                                    phi_15140_ = _e3761;
                                                                }
                                                                let _e3781 = phi_15140_;
                                                                let _e3783 = (4f * _e3395);
                                                                phi_15152_ = type_56((_e3781 * (((1f / (_e3446 + _e3460)) * _e3431) / _e3783)), ((_e3431 / _e3446) / _e3783));
                                                            }
                                                            let _e3790 = phi_15152_;
                                                            phi_15153_ = _e3790;
                                                            phi_15154_ = _e3407;
                                                        }
                                                        let _e3792 = phi_15153_;
                                                        let _e3794 = phi_15154_;
                                                        if _e3794 {
                                                            phi_15159_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15159_ = _e3792;
                                                        }
                                                        let _e3796 = phi_15159_;
                                                        let _e3799 = (_e2594 == 1u);
                                                        if _e3799 {
                                                            phi_15187_ = _e3796;
                                                            phi_15188_ = type_56();
                                                        } else {
                                                            if (_e3395 != _e3395) {
                                                                phi_28120_ = true;
                                                            } else {
                                                                phi_28120_ = (0f >= _e3395);
                                                            }
                                                            let _e3803 = phi_28120_;
                                                            let _e3804 = select(_e3395, 0f, _e3803);
                                                            let _e3805 = (1f / _e2584);
                                                            let _e3809 = ((_e3805 * _e3805) * (1f - (_e3804 * _e3804)));
                                                            if (_e3809 >= 1f) {
                                                                phi_28139_ = 1f;
                                                            } else {
                                                                let _e3812 = sqrt((1f - _e3809));
                                                                if (_e3804 != _e3804) {
                                                                    phi_28150_ = true;
                                                                } else {
                                                                    phi_28150_ = (_e3812 <= _e3804);
                                                                }
                                                                let _e3816 = phi_28150_;
                                                                let _e3821 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                                phi_28139_ = (_e3821 + ((1f - _e3821) * pow((1f - select(_e3804, _e3812, _e3816)), 5f)));
                                                            }
                                                            let _e3828 = phi_28139_;
                                                            if (_e3373 <= 0f) {
                                                                phi_28189_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_28189_ = (_e2582 * (_e3373 * 0.31830987f));
                                                            }
                                                            let _e3833 = phi_28189_;
                                                            let _e3834 = (1f - _e3828);
                                                            phi_15187_ = type_56();
                                                            phi_15188_ = type_56((_e3796.member + vec3<f32>((_e3833.x * _e3834), (_e3833.y * _e3834), (_e3833.z * _e3834))), ((_e3828 * _e3796.member_1) + (_e3834 * _e3379)));
                                                        }
                                                        let _e3848 = phi_15187_;
                                                        let _e3850 = phi_15188_;
                                                        phi_15189_ = _e3848;
                                                        phi_15190_ = _e3850;
                                                        phi_15191_ = _e3799;
                                                    }
                                                    let _e3858 = phi_15189_;
                                                    let _e3860 = phi_15190_;
                                                    let _e3862 = phi_15191_;
                                                    if _e3862 {
                                                        phi_15195_ = _e3858;
                                                    } else {
                                                        phi_15195_ = _e3860;
                                                    }
                                                    let _e3864 = phi_15195_;
                                                    if ((_e3864.member_1 * _e3368) < 0.001f) {
                                                        let _e4291 = global_1.member.member_6;
                                                        let _e4297 = global_1.member.member_8[2u];
                                                        if (_e4297 == 4294967295u) {
                                                            phi_15934_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e4300 = global_6.member[_e4297];
                                                            let _e4303 = global_6.member[(_e4297 + 1u)];
                                                            let _e4306 = global_6.member[(_e4297 + 2u)];
                                                            let _e4309 = global_6.member[(_e4297 + 3u)];
                                                            let _e4312 = global_6.member[(_e4297 + 4u)];
                                                            let _e4315 = global_6.member[(_e4297 + 5u)];
                                                            let _e4318 = global_6.member[(_e4297 + 6u)];
                                                            let _e4321 = global_6.member[(_e4297 + 7u)];
                                                            let _e4324 = global_6.member[(_e4297 + 8u)];
                                                            let _e4327 = global_6.member[(_e4297 + 9u)];
                                                            let _e4330 = global_6.member[(_e4297 + 13u)];
                                                            let _e4333 = global_6.member[(_e4297 + 14u)];
                                                            let _e4336 = global_6.member[(_e4297 + 15u)];
                                                            let _e4339 = global_6.member[(_e4297 + 16u)];
                                                            let _e4342 = global_6.member[(_e4297 + 18u)];
                                                            let _e4345 = global_6.member[(_e4297 + 19u)];
                                                            let _e4348 = global_6.member[(_e4297 + 20u)];
                                                            let _e4351 = global_6.member[(_e4297 + 29u)];
                                                            let _e4352 = bitcast<u32>(_e4351);
                                                            let _e4365 = (((((_e1727 * _e1727) + (_e1728 * _e1728)) + (_e1729 * _e1729)) - (2f * (((_e1727 * _e4300) + (_e1728 * _e4303)) + (_e1729 * _e4306)))) + _e4318);
                                                            if (_e4365 > 0f) {
                                                                let _e4374 = ((((_e1727 - _e4300) * _e4321) + ((_e1728 - _e4303) * _e4324)) + ((_e1729 - _e4306) * _e4327));
                                                                let _e4376 = ((_e4374 * _e4374) - _e4365);
                                                                if (_e4374 >= 0f) {
                                                                    phi_15634_ = vec3<f32>();
                                                                    phi_15635_ = true;
                                                                } else {
                                                                    let _e4378 = (_e4376 <= 0f);
                                                                    if _e4378 {
                                                                        phi_15633_ = vec3<f32>();
                                                                    } else {
                                                                        let _e4381 = (-(_e4374) - sqrt(_e4376));
                                                                        phi_15633_ = vec3<f32>((_e1727 + (_e4321 * _e4381)), (_e1728 + (_e4324 * _e4381)), (_e1729 + (_e4327 * _e4381)));
                                                                    }
                                                                    let _e4390 = phi_15633_;
                                                                    phi_15634_ = _e4390;
                                                                    phi_15635_ = _e4378;
                                                                }
                                                                let _e4392 = phi_15634_;
                                                                let _e4394 = phi_15635_;
                                                                phi_15640_ = _e4392;
                                                                phi_15641_ = select(true, false, _e4394);
                                                            } else {
                                                                phi_15640_ = _e1730;
                                                                phi_15641_ = true;
                                                            }
                                                            let _e4397 = phi_15640_;
                                                            let _e4399 = phi_15641_;
                                                            if _e4399 {
                                                                let _e4401 = (_e4397.x - _e4300);
                                                                let _e4403 = (_e4397.y - _e4303);
                                                                let _e4405 = (_e4397.z - _e4306);
                                                                let _e4411 = sqrt((((_e4401 * _e4401) + (_e4403 * _e4403)) + (_e4405 * _e4405)));
                                                                let _e4412 = (1f / _e4411);
                                                                let _e4430 = ((((((_e4397.x * _e4397.x) + (_e4397.y * _e4397.y)) + (_e4397.z * _e4397.z)) - (2f * (((_e4397.x * _e4300) + (_e4397.y * _e4303)) + (_e4397.z * _e4306)))) + _e4315) / (_e4411 + _e4309));
                                                                if (_e4430 != _e4430) {
                                                                    phi_28645_ = true;
                                                                } else {
                                                                    phi_28645_ = (0f >= _e4430);
                                                                }
                                                                let _e4434 = phi_28645_;
                                                                let _e4438 = sqrt((select(_e4430, 0f, _e4434) / (_e4312 - _e4309)));
                                                                if (_e4438 != _e4438) {
                                                                    phi_28660_ = true;
                                                                } else {
                                                                    phi_28660_ = (1f <= _e4438);
                                                                }
                                                                let _e4442 = phi_28660_;
                                                                let _e4444 = (select(_e4438, 1f, _e4442) * 63f);
                                                                let _e4449 = ((((_e4401 * _e4412) * _e4321) + ((_e4403 * _e4412) * _e4324)) + ((_e4405 * _e4412) * _e4327));
                                                                let _e4451 = select(_e4449, -1f, (_e4449 < -1f));
                                                                let _e4455 = ((select(_e4451, 1f, (_e4451 > 1f)) + 1f) * 127.5f);
                                                                let _e4460 = select(select(u32(_e4444), 0u, (_e4444 < 0f)), 4294967295u, (_e4444 > 4294967000f));
                                                                let _e4462 = select(_e4460, 62u, (62u < _e4460));
                                                                let _e4467 = select(select(u32(_e4455), 0u, (_e4455 < 0f)), 4294967295u, (_e4455 > 4294967000f));
                                                                let _e4469 = select(_e4467, 254u, (254u < _e4467));
                                                                let _e4471 = (_e4444 - f32(_e4462));
                                                                let _e4473 = (_e4455 - f32(_e4469));
                                                                let _e4477 = (_e4352 + (3u * ((_e4462 * 256u) + _e4469)));
                                                                let _e4482 = (_e4352 + (3u * (((_e4462 + 1u) * 256u) + _e4469)));
                                                                let _e4484 = global_6.member[_e4477];
                                                                let _e4487 = global_6.member[(_e4477 + 1u)];
                                                                let _e4490 = global_6.member[(_e4477 + 2u)];
                                                                let _e4491 = (1f - _e4473);
                                                                let _e4497 = global_6.member[(_e4477 + 3u)];
                                                                let _e4500 = global_6.member[(_e4477 + 4u)];
                                                                let _e4503 = global_6.member[(_e4477 + 5u)];
                                                                let _e4510 = (1f - _e4471);
                                                                let _e4515 = global_6.member[_e4482];
                                                                let _e4518 = global_6.member[(_e4482 + 1u)];
                                                                let _e4521 = global_6.member[(_e4482 + 2u)];
                                                                let _e4527 = global_6.member[(_e4482 + 3u)];
                                                                let _e4530 = global_6.member[(_e4482 + 4u)];
                                                                let _e4533 = global_6.member[(_e4482 + 5u)];
                                                                phi_15932_ = vec3<f32>(((((_e4484 * _e4491) + (_e4497 * _e4473)) * _e4510) + (((_e4515 * _e4491) + (_e4527 * _e4473)) * _e4471)), ((((_e4487 * _e4491) + (_e4500 * _e4473)) * _e4510) + (((_e4518 * _e4491) + (_e4530 * _e4473)) * _e4471)), ((((_e4490 * _e4491) + (_e4503 * _e4473)) * _e4510) + (((_e4521 * _e4491) + (_e4533 * _e4473)) * _e4471)));
                                                            } else {
                                                                phi_15932_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e4548 = phi_15932_;
                                                            let _e4555 = ((_e4339 * 1.1f) * _e4548.y);
                                                            phi_15934_ = vec3<f32>(exp(-((((_e4330 * _e4548.x) + _e4555) + (_e4342 * _e4548.z)))), exp(-((((_e4333 * _e4548.x) + _e4555) + (_e4345 * _e4548.z)))), exp(-((((_e4336 * _e4548.x) + _e4555) + (_e4348 * _e4548.z)))));
                                                        }
                                                        let _e4574 = phi_15934_;
                                                        phi_35633_ = _e2609;
                                                        phi_16818_ = 0f;
                                                        phi_16819_ = vec3<f32>(_e3356.x, _e3356.y, _e3356.z);
                                                        phi_16820_ = (vec3<f32>(_e4291.x, _e4291.y, _e4291.z) * _e4574);
                                                    } else {
                                                        let _e3869 = ((_e2609 * 747796405u) + 2891336453u);
                                                        let _e3877 = (((_e3869 >> bitcast<u32>((((_e3869 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3869) * 277803737u);
                                                        let _e3880 = ((_e3877 >> bitcast<u32>(22i)) ^ _e3877);
                                                        let _e3886 = (((f32((_e3880 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e3368) * 0.15915494f);
                                                        let _e3887 = (1f - _e3886);
                                                        let _e3889 = (_e3886 * (2f - _e3886));
                                                        if (_e3889 != _e3889) {
                                                            phi_28243_ = true;
                                                        } else {
                                                            phi_28243_ = (0f >= _e3889);
                                                        }
                                                        let _e3893 = phi_28243_;
                                                        let _e3895 = sqrt(select(_e3889, 0f, _e3893));
                                                        let _e3897 = ((_e3880 * 747796405u) + 2891336453u);
                                                        let _e3905 = (((_e3897 >> bitcast<u32>((((_e3897 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3897) * 277803737u);
                                                        let _e3908 = ((_e3905 >> bitcast<u32>(22i)) ^ _e3905);
                                                        let _e3912 = (f32((_e3908 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3914 = select(-1f, 1f, (_e3356.z >= 0f));
                                                        let _e3916 = (-1f / (_e3914 + _e3356.z));
                                                        let _e3918 = ((_e3356.x * _e3356.y) * _e3916);
                                                        let _e3931 = (_e3895 * cos(_e3912));
                                                        let _e3936 = (_e3895 * sin(_e3912));
                                                        let _e3946 = ((((1f + (((_e3914 * _e3356.x) * _e3356.x) * _e3916)) * _e3931) + (_e3918 * _e3936)) + (_e3356.x * _e3887));
                                                        let _e3947 = ((((_e3914 * _e3918) * _e3931) + ((_e3914 + ((_e3356.y * _e3356.y) * _e3916)) * _e3936)) + (_e3356.y * _e3887));
                                                        let _e3948 = ((((-(_e3914) * _e3356.x) * _e3931) + (-(_e3356.y) * _e3936)) + (_e3356.z * _e3887));
                                                        let _e3950 = (_e3946 - _e3356.x);
                                                        let _e3951 = (_e3947 - _e3356.y);
                                                        let _e3952 = (_e3948 - _e3356.z);
                                                        let _e3959 = (sqrt((((_e3950 * _e3950) + (_e3951 * _e3951)) + (_e3952 * _e3952))) * 0.5f);
                                                        let _e3960 = (_e3959 != _e3959);
                                                        if _e3960 {
                                                            phi_28314_ = true;
                                                        } else {
                                                            phi_28314_ = (1f <= _e3959);
                                                        }
                                                        let _e3963 = phi_28314_;
                                                        if ((2f * asin(select(_e3959, 1f, _e3963))) >= _e3364) {
                                                            phi_16015_ = 0f;
                                                        } else {
                                                            phi_16015_ = (1f / _e3368);
                                                        }
                                                        let _e3970 = phi_16015_;
                                                        if (_e3970 > 0f) {
                                                            if _e3960 {
                                                                phi_28329_ = true;
                                                            } else {
                                                                phi_28329_ = (1f <= _e3959);
                                                            }
                                                            let _e3974 = phi_28329_;
                                                            let _e3977 = (2f * asin(select(_e3959, 1f, _e3974)));
                                                            if (_e3977 >= _e3364) {
                                                                phi_16816_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3981 = (1f - pow((_e3977 / _e3364), 2f));
                                                                if (_e3981 != _e3981) {
                                                                    phi_28344_ = true;
                                                                } else {
                                                                    phi_28344_ = (0f >= _e3981);
                                                                }
                                                                let _e3985 = phi_28344_;
                                                                let _e3988 = global_1.member.member_6;
                                                                let _e3994 = global_1.member.member_8[2u];
                                                                if (_e3994 == 4294967295u) {
                                                                    phi_16789_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e3997 = global_6.member[_e3994];
                                                                    let _e4000 = global_6.member[(_e3994 + 1u)];
                                                                    let _e4003 = global_6.member[(_e3994 + 2u)];
                                                                    let _e4006 = global_6.member[(_e3994 + 3u)];
                                                                    let _e4009 = global_6.member[(_e3994 + 4u)];
                                                                    let _e4012 = global_6.member[(_e3994 + 5u)];
                                                                    let _e4015 = global_6.member[(_e3994 + 6u)];
                                                                    let _e4018 = global_6.member[(_e3994 + 7u)];
                                                                    let _e4021 = global_6.member[(_e3994 + 8u)];
                                                                    let _e4024 = global_6.member[(_e3994 + 9u)];
                                                                    let _e4027 = global_6.member[(_e3994 + 13u)];
                                                                    let _e4030 = global_6.member[(_e3994 + 14u)];
                                                                    let _e4033 = global_6.member[(_e3994 + 15u)];
                                                                    let _e4036 = global_6.member[(_e3994 + 16u)];
                                                                    let _e4039 = global_6.member[(_e3994 + 18u)];
                                                                    let _e4042 = global_6.member[(_e3994 + 19u)];
                                                                    let _e4045 = global_6.member[(_e3994 + 20u)];
                                                                    let _e4048 = global_6.member[(_e3994 + 29u)];
                                                                    let _e4049 = bitcast<u32>(_e4048);
                                                                    let _e4062 = (((((_e1727 * _e1727) + (_e1728 * _e1728)) + (_e1729 * _e1729)) - (2f * (((_e1727 * _e3997) + (_e1728 * _e4000)) + (_e1729 * _e4003)))) + _e4015);
                                                                    if (_e4062 > 0f) {
                                                                        let _e4071 = ((((_e1727 - _e3997) * _e4018) + ((_e1728 - _e4000) * _e4021)) + ((_e1729 - _e4003) * _e4024));
                                                                        let _e4073 = ((_e4071 * _e4071) - _e4062);
                                                                        if (_e4071 >= 0f) {
                                                                            phi_16489_ = vec3<f32>();
                                                                            phi_16490_ = true;
                                                                        } else {
                                                                            let _e4075 = (_e4073 <= 0f);
                                                                            if _e4075 {
                                                                                phi_16488_ = vec3<f32>();
                                                                            } else {
                                                                                let _e4078 = (-(_e4071) - sqrt(_e4073));
                                                                                phi_16488_ = vec3<f32>((_e1727 + (_e4018 * _e4078)), (_e1728 + (_e4021 * _e4078)), (_e1729 + (_e4024 * _e4078)));
                                                                            }
                                                                            let _e4087 = phi_16488_;
                                                                            phi_16489_ = _e4087;
                                                                            phi_16490_ = _e4075;
                                                                        }
                                                                        let _e4089 = phi_16489_;
                                                                        let _e4091 = phi_16490_;
                                                                        phi_16495_ = _e4089;
                                                                        phi_16496_ = select(true, false, _e4091);
                                                                    } else {
                                                                        phi_16495_ = _e1730;
                                                                        phi_16496_ = true;
                                                                    }
                                                                    let _e4094 = phi_16495_;
                                                                    let _e4096 = phi_16496_;
                                                                    if _e4096 {
                                                                        let _e4098 = (_e4094.x - _e3997);
                                                                        let _e4100 = (_e4094.y - _e4000);
                                                                        let _e4102 = (_e4094.z - _e4003);
                                                                        let _e4108 = sqrt((((_e4098 * _e4098) + (_e4100 * _e4100)) + (_e4102 * _e4102)));
                                                                        let _e4109 = (1f / _e4108);
                                                                        let _e4127 = ((((((_e4094.x * _e4094.x) + (_e4094.y * _e4094.y)) + (_e4094.z * _e4094.z)) - (2f * (((_e4094.x * _e3997) + (_e4094.y * _e4000)) + (_e4094.z * _e4003)))) + _e4012) / (_e4108 + _e4006));
                                                                        if (_e4127 != _e4127) {
                                                                            phi_28468_ = true;
                                                                        } else {
                                                                            phi_28468_ = (0f >= _e4127);
                                                                        }
                                                                        let _e4131 = phi_28468_;
                                                                        let _e4135 = sqrt((select(_e4127, 0f, _e4131) / (_e4009 - _e4006)));
                                                                        if (_e4135 != _e4135) {
                                                                            phi_28483_ = true;
                                                                        } else {
                                                                            phi_28483_ = (1f <= _e4135);
                                                                        }
                                                                        let _e4139 = phi_28483_;
                                                                        let _e4141 = (select(_e4135, 1f, _e4139) * 63f);
                                                                        let _e4146 = ((((_e4098 * _e4109) * _e4018) + ((_e4100 * _e4109) * _e4021)) + ((_e4102 * _e4109) * _e4024));
                                                                        let _e4148 = select(_e4146, -1f, (_e4146 < -1f));
                                                                        let _e4152 = ((select(_e4148, 1f, (_e4148 > 1f)) + 1f) * 127.5f);
                                                                        let _e4157 = select(select(u32(_e4141), 0u, (_e4141 < 0f)), 4294967295u, (_e4141 > 4294967000f));
                                                                        let _e4159 = select(_e4157, 62u, (62u < _e4157));
                                                                        let _e4164 = select(select(u32(_e4152), 0u, (_e4152 < 0f)), 4294967295u, (_e4152 > 4294967000f));
                                                                        let _e4166 = select(_e4164, 254u, (254u < _e4164));
                                                                        let _e4168 = (_e4141 - f32(_e4159));
                                                                        let _e4170 = (_e4152 - f32(_e4166));
                                                                        let _e4174 = (_e4049 + (3u * ((_e4159 * 256u) + _e4166)));
                                                                        let _e4179 = (_e4049 + (3u * (((_e4159 + 1u) * 256u) + _e4166)));
                                                                        let _e4181 = global_6.member[_e4174];
                                                                        let _e4184 = global_6.member[(_e4174 + 1u)];
                                                                        let _e4187 = global_6.member[(_e4174 + 2u)];
                                                                        let _e4188 = (1f - _e4170);
                                                                        let _e4194 = global_6.member[(_e4174 + 3u)];
                                                                        let _e4197 = global_6.member[(_e4174 + 4u)];
                                                                        let _e4200 = global_6.member[(_e4174 + 5u)];
                                                                        let _e4207 = (1f - _e4168);
                                                                        let _e4212 = global_6.member[_e4179];
                                                                        let _e4215 = global_6.member[(_e4179 + 1u)];
                                                                        let _e4218 = global_6.member[(_e4179 + 2u)];
                                                                        let _e4224 = global_6.member[(_e4179 + 3u)];
                                                                        let _e4227 = global_6.member[(_e4179 + 4u)];
                                                                        let _e4230 = global_6.member[(_e4179 + 5u)];
                                                                        phi_16787_ = vec3<f32>(((((_e4181 * _e4188) + (_e4194 * _e4170)) * _e4207) + (((_e4212 * _e4188) + (_e4224 * _e4170)) * _e4168)), ((((_e4184 * _e4188) + (_e4197 * _e4170)) * _e4207) + (((_e4215 * _e4188) + (_e4227 * _e4170)) * _e4168)), ((((_e4187 * _e4188) + (_e4200 * _e4170)) * _e4207) + (((_e4218 * _e4188) + (_e4230 * _e4170)) * _e4168)));
                                                                    } else {
                                                                        phi_16787_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e4245 = phi_16787_;
                                                                    let _e4252 = ((_e4036 * 1.1f) * _e4245.y);
                                                                    phi_16789_ = vec3<f32>(exp(-((((_e4027 * _e4245.x) + _e4252) + (_e4039 * _e4245.z)))), exp(-((((_e4030 * _e4245.x) + _e4252) + (_e4042 * _e4245.z)))), exp(-((((_e4033 * _e4245.x) + _e4252) + (_e4045 * _e4245.z)))));
                                                                }
                                                                let _e4271 = phi_16789_;
                                                                let _e4282 = (((1f - (0.6f * (1f - sqrt(select(_e3981, 0f, _e3985))))) * 1.25f) / _e3368);
                                                                phi_16816_ = vec3<f32>(((_e3988.x * _e4271.x) * _e4282), ((_e3988.y * _e4271.y) * _e4282), ((_e3988.z * _e4271.z) * _e4282));
                                                            }
                                                            let _e4288 = phi_16816_;
                                                            phi_16817_ = _e4288;
                                                        } else {
                                                            phi_16817_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e4290 = phi_16817_;
                                                        phi_35633_ = _e3908;
                                                        phi_16818_ = _e3970;
                                                        phi_16819_ = vec3<f32>(_e3946, _e3947, _e3948);
                                                        phi_16820_ = _e4290;
                                                    }
                                                    let _e4578 = phi_35633_;
                                                    let _e4580 = phi_16818_;
                                                    let _e4582 = phi_16819_;
                                                    let _e4584 = phi_16820_;
                                                    phi_35632_ = _e4578;
                                                    phi_16821_ = _e4580;
                                                    phi_16822_ = _e4582;
                                                    phi_16823_ = _e4584;
                                                }
                                                let _e4586 = phi_35632_;
                                                let _e4588 = phi_16821_;
                                                let _e4590 = phi_16822_;
                                                let _e4592 = phi_16823_;
                                                phi_35589_ = _e4586;
                                                phi_17713_ = _e4588;
                                                phi_17714_ = _e4590;
                                                phi_17715_ = _e4592;
                                                phi_17716_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e2615 == (_e2605 + 1u)) {
                                                    if ((_e364 & 1u) == 0u) {
                                                        phi_35598_ = _e2609;
                                                        phi_17347_ = 0f;
                                                        phi_17348_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17349_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17350_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e367 > 0u) {
                                                            let _e2846 = global_1.member.member_2[1u];
                                                            let _e2848 = ((_e2609 * 747796405u) + 2891336453u);
                                                            let _e2856 = (((_e2848 >> bitcast<u32>((((_e2848 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2848) * 277803737u);
                                                            let _e2859 = ((_e2856 >> bitcast<u32>(22i)) ^ _e2856);
                                                            phi_16844_ = 0u;
                                                            phi_16847_ = _e367;
                                                            loop {
                                                                let _e2865 = phi_16844_;
                                                                let _e2867 = phi_16847_;
                                                                local_85 = _e2865;
                                                                local_86 = _e2865;
                                                                let _e2869 = ((_e2865 + 1u) < _e2867);
                                                                if _e2869 {
                                                                    let _e2871 = ((_e2865 + _e2867) / 2u);
                                                                    let _e2874 = global_6.member[(_e2846 + _e2871)];
                                                                    let _e2875 = (_e2874 <= (f32((_e2859 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_16845_ = select(_e2865, _e2871, _e2875);
                                                                    phi_16848_ = select(_e2871, _e2867, _e2875);
                                                                } else {
                                                                    phi_16845_ = u32();
                                                                    phi_16848_ = u32();
                                                                }
                                                                let _e2879 = phi_16845_;
                                                                let _e2881 = phi_16848_;
                                                                phi_16844_ = _e2879;
                                                                phi_16847_ = _e2881;
                                                                if !(_e2869) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2884 = local_85;
                                                            let _e2885 = (_e2846 + _e2884);
                                                            let _e2888 = global_6.member[(_e2885 + 1u)];
                                                            let _e2890 = global_6.member[_e2885];
                                                            let _e2894 = global_1.member.member_2[0u];
                                                            let _e2896 = local_86;
                                                            let _e2899 = global_6.member[(_e2894 + _e2896)];
                                                            let _e2902 = ((_e2859 * 747796405u) + 2891336453u);
                                                            let _e2910 = (((_e2902 >> bitcast<u32>((((_e2902 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2902) * 277803737u);
                                                            let _e2913 = ((_e2910 >> bitcast<u32>(22i)) ^ _e2910);
                                                            let _e2916 = f32((_e2913 >> bitcast<u32>(8i)));
                                                            let _e2917 = (_e2916 * 0.000000059604645f);
                                                            let _e2919 = ((_e2913 * 747796405u) + 2891336453u);
                                                            let _e2927 = (((_e2919 >> bitcast<u32>((((_e2919 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2919) * 277803737u);
                                                            let _e2930 = ((_e2927 >> bitcast<u32>(22i)) ^ _e2927);
                                                            let _e2933 = f32((_e2930 >> bitcast<u32>(8i)));
                                                            let _e2936 = global_2.member[bitcast<u32>(_e2899)];
                                                            let _e2939 = (_e2936.member_6.x == 1u);
                                                            if _e2939 {
                                                                let _e3081 = (_e2936.member.x - _e1727);
                                                                let _e3082 = (_e2936.member.y - _e1728);
                                                                let _e3083 = (_e2936.member.z - _e1729);
                                                                let _e3088 = (((_e3081 * _e3081) + (_e3082 * _e3082)) + (_e3083 * _e3083));
                                                                let _e3089 = (_e2936.member.w * _e2936.member.w);
                                                                let _e3091 = (_e3088 <= (_e3089 * 1.000001f));
                                                                if _e3091 {
                                                                    let _e3200 = (_e2916 * 0.00000037450704f);
                                                                    let _e3202 = ((_e2933 * 0.00000011920929f) - 1f);
                                                                    let _e3204 = (1f - (_e3202 * _e3202));
                                                                    if (_e3204 != _e3204) {
                                                                        phi_27311_ = true;
                                                                    } else {
                                                                        phi_27311_ = (0f >= _e3204);
                                                                    }
                                                                    let _e3208 = phi_27311_;
                                                                    let _e3210 = sqrt(select(_e3204, 0f, _e3208));
                                                                    let _e3212 = (_e3210 * cos(_e3200));
                                                                    let _e3214 = (_e3210 * sin(_e3200));
                                                                    let _e3219 = (_e2936.member.x + (_e3212 * _e2936.member.w));
                                                                    let _e3220 = (_e2936.member.y + (_e3214 * _e2936.member.w));
                                                                    let _e3221 = (_e2936.member.z + (_e3202 * _e2936.member.w));
                                                                    if _e3091 {
                                                                        let _e3234 = (_e3219 - _e1727);
                                                                        let _e3235 = (_e3220 - _e1728);
                                                                        let _e3236 = (_e3221 - _e1729);
                                                                        let _e3241 = (((_e3234 * _e3234) + (_e3235 * _e3235)) + (_e3236 * _e3236));
                                                                        phi_27380_ = (_e3241 / (((abs((((_e3212 * _e3234) + (_e3214 * _e3235)) + (_e3202 * _e3236))) / sqrt(_e3241)) * 12.566371f) * _e3089));
                                                                    } else {
                                                                        let _e3223 = (_e3089 / _e3088);
                                                                        if (_e3223 != _e3223) {
                                                                            phi_27399_ = true;
                                                                        } else {
                                                                            phi_27399_ = (1f <= _e3223);
                                                                        }
                                                                        let _e3227 = phi_27399_;
                                                                        let _e3228 = select(_e3223, 1f, _e3227);
                                                                        phi_27380_ = (0.15915494f / (_e3228 / (1f + sqrt((1f - _e3228)))));
                                                                    }
                                                                    let _e3254 = phi_27380_;
                                                                    phi_17060_ = type_74(vec3<f32>(_e3219, _e3220, _e3221), vec3<f32>(_e3212, _e3214, _e3202), _e3254);
                                                                } else {
                                                                    let _e3092 = (_e3089 / _e3088);
                                                                    let _e3093 = (_e3092 != _e3092);
                                                                    if _e3093 {
                                                                        phi_27163_ = true;
                                                                    } else {
                                                                        phi_27163_ = (1f <= _e3092);
                                                                    }
                                                                    let _e3096 = phi_27163_;
                                                                    let _e3097 = select(_e3092, 1f, _e3096);
                                                                    if _e3093 {
                                                                        phi_27186_ = true;
                                                                    } else {
                                                                        phi_27186_ = (1f <= _e3092);
                                                                    }
                                                                    let _e3101 = phi_27186_;
                                                                    let _e3102 = select(_e3092, 1f, _e3101);
                                                                    let _e3106 = (_e3102 / (1f + sqrt((1f - _e3102))));
                                                                    if (_e3097 < 0.00068523f) {
                                                                        phi_16995_ = (_e3097 * _e2917);
                                                                    } else {
                                                                        let _e3109 = (1f - (_e2917 * _e3106));
                                                                        let _e3111 = (1f - (_e3109 * _e3109));
                                                                        if (_e3111 != _e3111) {
                                                                            phi_27201_ = true;
                                                                        } else {
                                                                            phi_27201_ = (0f >= _e3111);
                                                                        }
                                                                        let _e3115 = phi_27201_;
                                                                        phi_16995_ = select(_e3111, 0f, _e3115);
                                                                    }
                                                                    let _e3119 = phi_16995_;
                                                                    let _e3120 = (1f - _e3119);
                                                                    if (_e3120 != _e3120) {
                                                                        phi_27216_ = true;
                                                                    } else {
                                                                        phi_27216_ = (0f >= _e3120);
                                                                    }
                                                                    let _e3124 = phi_27216_;
                                                                    let _e3129 = (1f - (_e3119 / _e3097));
                                                                    if (_e3129 != _e3129) {
                                                                        phi_27231_ = true;
                                                                    } else {
                                                                        phi_27231_ = (0f >= _e3129);
                                                                    }
                                                                    let _e3133 = phi_27231_;
                                                                    let _e3137 = ((_e3119 / sqrt(_e3097)) + (sqrt(select(_e3120, 0f, _e3124)) * sqrt(select(_e3129, 0f, _e3133))));
                                                                    let _e3139 = (1f - (_e3137 * _e3137));
                                                                    if (_e3139 != _e3139) {
                                                                        phi_27246_ = true;
                                                                    } else {
                                                                        phi_27246_ = (0f >= _e3139);
                                                                    }
                                                                    let _e3143 = phi_27246_;
                                                                    let _e3145 = sqrt(select(_e3139, 0f, _e3143));
                                                                    let _e3146 = (_e2933 * 0.00000037450704f);
                                                                    let _e3147 = sqrt(_e3088);
                                                                    let _e3148 = (_e3081 / _e3147);
                                                                    let _e3149 = (_e3082 / _e3147);
                                                                    let _e3150 = (_e3083 / _e3147);
                                                                    let _e3152 = select(-1f, 1f, (_e3150 >= 0f));
                                                                    let _e3154 = (-1f / (_e3152 + _e3150));
                                                                    let _e3156 = ((_e3148 * _e3149) * _e3154);
                                                                    let _e3169 = (_e3145 * cos(_e3146));
                                                                    let _e3174 = (_e3145 * sin(_e3146));
                                                                    let _e3187 = -(((((1f + (((_e3152 * _e3148) * _e3148) * _e3154)) * _e3169) + (_e3156 * _e3174)) + (_e3148 * _e3137)));
                                                                    let _e3188 = -(((((_e3152 * _e3156) * _e3169) + ((_e3152 + ((_e3149 * _e3149) * _e3154)) * _e3174)) + (_e3149 * _e3137)));
                                                                    let _e3189 = -(((((-(_e3152) * _e3148) * _e3169) + (-(_e3149) * _e3174)) + (_e3150 * _e3137)));
                                                                    phi_17060_ = type_74(vec3<f32>((_e2936.member.x + (_e3187 * _e2936.member.w)), (_e2936.member.y + (_e3188 * _e2936.member.w)), (_e2936.member.z + (_e3189 * _e2936.member.w))), vec3<f32>(_e3187, _e3188, _e3189), (0.15915494f / _e3106));
                                                                }
                                                                let _e3257 = phi_17060_;
                                                                phi_17246_ = _e3257;
                                                            } else {
                                                                let _e2940 = sqrt(_e2917);
                                                                let _e2941 = (1f - _e2940);
                                                                let _e2942 = ((_e2933 * 0.000000059604645f) * _e2940);
                                                                let _e2964 = (_e2940 - _e2942);
                                                                let _e2968 = (((_e2936.member.x * _e2941) + (_e2936.member_1.x * _e2942)) + (_e2936.member_2.x * _e2964));
                                                                let _e2969 = (((_e2936.member.y * _e2941) + (_e2936.member_1.y * _e2942)) + (_e2936.member_2.y * _e2964));
                                                                let _e2970 = (((_e2936.member.z * _e2941) + (_e2936.member_1.z * _e2942)) + (_e2936.member_2.z * _e2964));
                                                                let _e2972 = (_e2936.member_1.x - _e2936.member.x);
                                                                let _e2973 = (_e2936.member_1.y - _e2936.member.y);
                                                                let _e2974 = (_e2936.member_1.z - _e2936.member.z);
                                                                let _e2975 = (_e2936.member_2.x - _e2936.member.x);
                                                                let _e2976 = (_e2936.member_2.y - _e2936.member.y);
                                                                let _e2977 = (_e2936.member_2.z - _e2936.member.z);
                                                                let _e2980 = ((_e2973 * _e2977) - (_e2976 * _e2974));
                                                                let _e2983 = ((_e2974 * _e2975) - (_e2977 * _e2972));
                                                                let _e2986 = ((_e2972 * _e2976) - (_e2975 * _e2973));
                                                                let _e2992 = sqrt((((_e2980 * _e2980) + (_e2983 * _e2983)) + (_e2986 * _e2986)));
                                                                let _e2993 = (1f / _e2992);
                                                                let _e2994 = (_e2980 * _e2993);
                                                                let _e2995 = (_e2983 * _e2993);
                                                                let _e2996 = (_e2986 * _e2993);
                                                                if _e2939 {
                                                                    let _e3029 = (_e2936.member.x - _e1727);
                                                                    let _e3030 = (_e2936.member.y - _e1728);
                                                                    let _e3031 = (_e2936.member.z - _e1729);
                                                                    let _e3036 = (((_e3029 * _e3029) + (_e3030 * _e3030)) + (_e3031 * _e3031));
                                                                    let _e3037 = (_e2936.member.w * _e2936.member.w);
                                                                    if (_e3036 <= (_e3037 * 1.000001f)) {
                                                                        let _e3051 = (_e2968 - _e1727);
                                                                        let _e3052 = (_e2969 - _e1728);
                                                                        let _e3053 = (_e2970 - _e1729);
                                                                        let _e3058 = (((_e3051 * _e3051) + (_e3052 * _e3052)) + (_e3053 * _e3053));
                                                                        phi_27129_ = (_e3058 / (((abs((((_e2994 * _e3051) + (_e2995 * _e3052)) + (_e2996 * _e3053))) / sqrt(_e3058)) * 12.566371f) * _e3037));
                                                                    } else {
                                                                        let _e3040 = (_e3037 / _e3036);
                                                                        if (_e3040 != _e3040) {
                                                                            phi_27148_ = true;
                                                                        } else {
                                                                            phi_27148_ = (1f <= _e3040);
                                                                        }
                                                                        let _e3044 = phi_27148_;
                                                                        let _e3045 = select(_e3040, 1f, _e3044);
                                                                        phi_27129_ = (0.15915494f / (_e3045 / (1f + sqrt((1f - _e3045)))));
                                                                    }
                                                                    let _e3071 = phi_27129_;
                                                                    phi_17244_ = _e3071;
                                                                } else {
                                                                    let _e2998 = (_e2968 - _e1727);
                                                                    let _e2999 = (_e2969 - _e1728);
                                                                    let _e3000 = (_e2970 - _e1729);
                                                                    let _e3005 = (((_e2998 * _e2998) + (_e2999 * _e2999)) + (_e3000 * _e3000));
                                                                    let _e3013 = (abs((((_e2994 * _e2998) + (_e2995 * _e2999)) + (_e2996 * _e3000))) / sqrt(_e3005));
                                                                    if (_e3013 <= 0f) {
                                                                        phi_17243_ = 0f;
                                                                    } else {
                                                                        if (_e2936.member_6.x == 0u) {
                                                                            phi_17240_ = (_e2992 * 0.5f);
                                                                        } else {
                                                                            phi_17240_ = ((12.566371f * _e2936.member.w) * _e2936.member.w);
                                                                        }
                                                                        let _e3022 = phi_17240_;
                                                                        phi_17243_ = (_e3005 / (_e3013 * _e3022));
                                                                    }
                                                                    let _e3026 = phi_17243_;
                                                                    phi_17244_ = _e3026;
                                                                }
                                                                let _e3073 = phi_17244_;
                                                                phi_17246_ = type_74(vec3<f32>(_e2968, _e2969, _e2970), vec3<f32>(_e2994, _e2995, _e2996), _e3073);
                                                            }
                                                            let _e3259 = phi_17246_;
                                                            let _e3264 = (_e3259.member.x - _e1727);
                                                            let _e3266 = (_e3259.member.y - _e1728);
                                                            let _e3268 = (_e3259.member.z - _e1729);
                                                            let _e3274 = sqrt((((_e3264 * _e3264) + (_e3266 * _e3266)) + (_e3268 * _e3268)));
                                                            let _e3275 = (_e3264 / _e3274);
                                                            let _e3276 = (_e3266 / _e3274);
                                                            let _e3277 = (_e3268 / _e3274);
                                                            if (_e3259.member_2 > 0f) {
                                                                let _e3289 = global_4.member[_e2936.member_6.y];
                                                                if (_e3289.member == 4u) {
                                                                    if (_e3289.member_1 == 1u) {
                                                                        phi_17324_ = true;
                                                                    } else {
                                                                        phi_17324_ = ((((-(_e3275) * _e3259.member_1.x) + (-(_e3276) * _e3259.member_1.y)) + (-(_e3277) * _e3259.member_1.z)) > 0f);
                                                                    }
                                                                    let _e3304 = phi_17324_;
                                                                    if _e3304 {
                                                                        phi_17333_ = vec3<f32>(_e3289.member_4.x, _e3289.member_4.y, _e3289.member_4.z);
                                                                    } else {
                                                                        phi_17333_ = vec3<f32>();
                                                                    }
                                                                    let _e3311 = phi_17333_;
                                                                    phi_17335_ = _e3311;
                                                                    phi_17336_ = select(true, false, _e3304);
                                                                } else {
                                                                    phi_17335_ = vec3<f32>();
                                                                    phi_17336_ = true;
                                                                }
                                                                let _e3314 = phi_17335_;
                                                                let _e3316 = phi_17336_;
                                                                phi_17341_ = ((_e2888 - _e2890) * _e3259.member_2);
                                                                phi_17342_ = select(_e3314, vec3<f32>(0f, 0f, 0f), vec3(_e3316));
                                                            } else {
                                                                phi_17341_ = 0f;
                                                                phi_17342_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3320 = phi_17341_;
                                                            let _e3322 = phi_17342_;
                                                            phi_35599_ = _e2930;
                                                            phi_17343_ = _e3320;
                                                            phi_17344_ = vec3<f32>(_e3275, _e3276, _e3277);
                                                            phi_17345_ = _e3322;
                                                            phi_17346_ = ((_e3274 * 0.9999f) - _e1720);
                                                        } else {
                                                            phi_35599_ = _e2609;
                                                            phi_17343_ = 0f;
                                                            phi_17344_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17345_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17346_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e3324 = phi_35599_;
                                                        let _e3326 = phi_17343_;
                                                        let _e3328 = phi_17344_;
                                                        let _e3330 = phi_17345_;
                                                        let _e3332 = phi_17346_;
                                                        phi_35598_ = _e3324;
                                                        phi_17347_ = _e3326;
                                                        phi_17348_ = _e3328;
                                                        phi_17349_ = _e3330;
                                                        phi_17350_ = _e3332;
                                                    }
                                                    let _e3334 = phi_35598_;
                                                    let _e3336 = phi_17347_;
                                                    let _e3338 = phi_17348_;
                                                    let _e3340 = phi_17349_;
                                                    let _e3342 = phi_17350_;
                                                    phi_35590_ = _e3334;
                                                    phi_17709_ = _e3336;
                                                    phi_17710_ = _e3338;
                                                    phi_17711_ = _e3340;
                                                    phi_17712_ = _e3342;
                                                } else {
                                                    if ((_e364 & 2u) == 0u) {
                                                        phi_35591_ = _e2609;
                                                        phi_17706_ = 0f;
                                                        phi_17707_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17708_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2627 = ((_e2609 * 747796405u) + 2891336453u);
                                                        let _e2635 = (((_e2627 >> bitcast<u32>((((_e2627 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2627) * 277803737u);
                                                        let _e2638 = ((_e2635 >> bitcast<u32>(22i)) ^ _e2635);
                                                        let _e2642 = (f32((_e2638 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2644 = ((_e2638 * 747796405u) + 2891336453u);
                                                        let _e2652 = (((_e2644 >> bitcast<u32>((((_e2644 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2644) * 277803737u);
                                                        let _e2655 = ((_e2652 >> bitcast<u32>(22i)) ^ _e2652);
                                                        let _e2659 = (f32((_e2655 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2662 = global_1.member.member_3[1u];
                                                        let _e2665 = global_1.member.member_3[2u];
                                                        let _e2668 = global_1.member.member_2[3u];
                                                        phi_17371_ = 0u;
                                                        phi_17374_ = _e2665;
                                                        loop {
                                                            let _e2670 = phi_17371_;
                                                            let _e2672 = phi_17374_;
                                                            local_80 = _e2670;
                                                            local_83 = _e2670;
                                                            local_84 = _e2670;
                                                            let _e2674 = ((_e2670 + 1u) < _e2672);
                                                            if _e2674 {
                                                                let _e2676 = ((_e2670 + _e2672) / 2u);
                                                                let _e2679 = global_6.member[(_e2668 + _e2676)];
                                                                let _e2680 = (_e2679 <= _e2642);
                                                                phi_17372_ = select(_e2670, _e2676, _e2680);
                                                                phi_17375_ = select(_e2676, _e2672, _e2680);
                                                            } else {
                                                                phi_17372_ = u32();
                                                                phi_17375_ = u32();
                                                            }
                                                            let _e2684 = phi_17372_;
                                                            let _e2686 = phi_17375_;
                                                            phi_17371_ = _e2684;
                                                            phi_17374_ = _e2686;
                                                            if !(_e2674) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2690 = global_1.member.member_3[0u];
                                                        let _e2691 = (_e2662 + 1u);
                                                        let _e2693 = local_80;
                                                        let _e2695 = (_e2690 + (_e2693 * _e2691));
                                                        phi_17412_ = 0u;
                                                        phi_17415_ = _e2662;
                                                        loop {
                                                            let _e2697 = phi_17412_;
                                                            let _e2699 = phi_17415_;
                                                            local_81 = _e2697;
                                                            local_82 = _e2697;
                                                            let _e2701 = ((_e2697 + 1u) < _e2699);
                                                            if _e2701 {
                                                                let _e2703 = ((_e2697 + _e2699) / 2u);
                                                                let _e2706 = global_6.member[(_e2695 + _e2703)];
                                                                let _e2707 = (_e2706 <= _e2659);
                                                                phi_17413_ = select(_e2697, _e2703, _e2707);
                                                                phi_17416_ = select(_e2703, _e2699, _e2707);
                                                            } else {
                                                                phi_17413_ = u32();
                                                                phi_17416_ = u32();
                                                            }
                                                            let _e2711 = phi_17413_;
                                                            let _e2713 = phi_17416_;
                                                            phi_17412_ = _e2711;
                                                            phi_17415_ = _e2713;
                                                            if !(_e2701) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2716 = local_81;
                                                        let _e2719 = local_82;
                                                        let _e2720 = (_e2695 + _e2719);
                                                        let _e2722 = global_6.member[_e2720];
                                                        let _e2725 = global_6.member[(_e2720 + 1u)];
                                                        if (_e2725 > _e2722) {
                                                            let _e2729 = ((_e2659 - _e2722) / (_e2725 - _e2722));
                                                            let _e2731 = select(_e2729, 0f, (_e2729 < 0f));
                                                            phi_17492_ = select(_e2731, 0.99999f, (_e2731 > 0.99999f));
                                                        } else {
                                                            phi_17492_ = 0.5f;
                                                        }
                                                        let _e2735 = phi_17492_;
                                                        let _e2737 = f32(_e2662);
                                                        let _e2740 = local_83;
                                                        let _e2743 = local_84;
                                                        let _e2744 = (_e2668 + _e2743);
                                                        let _e2746 = global_6.member[_e2744];
                                                        let _e2749 = global_6.member[(_e2744 + 1u)];
                                                        if (_e2749 > _e2746) {
                                                            let _e2753 = ((_e2642 - _e2746) / (_e2749 - _e2746));
                                                            let _e2755 = select(_e2753, 0f, (_e2753 < 0f));
                                                            phi_17541_ = select(_e2755, 0.99999f, (_e2755 > 0.99999f));
                                                        } else {
                                                            phi_17541_ = 0.5f;
                                                        }
                                                        let _e2759 = phi_17541_;
                                                        let _e2761 = f32(_e2665);
                                                        let _e2764 = ((((f32(_e2716) + _e2735) / _e2737) - 0.5f) * 6.2831855f);
                                                        let _e2765 = (((f32(_e2740) + _e2759) / _e2761) * 3.1415927f);
                                                        let _e2766 = sin(_e2765);
                                                        let _e2768 = (_e2766 * cos(_e2764));
                                                        let _e2769 = cos(_e2765);
                                                        let _e2771 = (_e2766 * sin(_e2764));
                                                        let _e2774 = ((atan2(_e2771, _e2768) * 0.15915494f) + 0.5f);
                                                        let _e2778 = select(_e2769, -1f, (_e2769 < -1f));
                                                        let _e2781 = acos(select(_e2778, 1f, (_e2778 > 1f)));
                                                        let _e2783 = ((_e2774 - floor(_e2774)) * _e2737);
                                                        let _e2788 = select(select(u32(_e2783), 0u, (_e2783 < 0f)), 4294967295u, (_e2783 > 4294967000f));
                                                        let _e2789 = (_e2662 - 1u);
                                                        let _e2792 = ((_e2781 * 0.31830987f) * _e2761);
                                                        let _e2797 = select(select(u32(_e2792), 0u, (_e2792 < 0f)), 4294967295u, (_e2792 > 4294967000f));
                                                        let _e2798 = (_e2665 - 1u);
                                                        let _e2800 = select(_e2797, _e2798, (_e2798 < _e2797));
                                                        let _e2801 = sin(_e2781);
                                                        if (_e2801 <= 0f) {
                                                            phi_17697_ = 0f;
                                                        } else {
                                                            let _e2805 = (_e2668 + _e2800);
                                                            let _e2808 = global_6.member[(_e2805 + 1u)];
                                                            let _e2810 = global_6.member[_e2805];
                                                            let _e2813 = ((_e2690 + (_e2800 * _e2691)) + select(_e2788, _e2789, (_e2789 < _e2788)));
                                                            let _e2816 = global_6.member[(_e2813 + 1u)];
                                                            let _e2818 = global_6.member[_e2813];
                                                            phi_17697_ = ((((_e2808 - _e2810) * _e2761) * ((_e2816 - _e2818) * _e2737)) / (19.73921f * _e2801));
                                                        }
                                                        let _e2825 = phi_17697_;
                                                        let _e2826 = (_e2825 > 0f);
                                                        if _e2826 {
                                                            phi_17704_ = vec3<f32>(_e2768, _e2769, _e2771);
                                                        } else {
                                                            phi_17704_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2829 = phi_17704_;
                                                        phi_35591_ = _e2655;
                                                        phi_17706_ = select(0f, _e2825, _e2826);
                                                        phi_17707_ = _e2829;
                                                        phi_17708_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2826));
                                                    }
                                                    let _e2834 = phi_35591_;
                                                    let _e2836 = phi_17706_;
                                                    let _e2838 = phi_17707_;
                                                    let _e2840 = phi_17708_;
                                                    phi_35590_ = _e2834;
                                                    phi_17709_ = _e2836;
                                                    phi_17710_ = _e2838;
                                                    phi_17711_ = _e2840;
                                                    phi_17712_ = 1000000000000000000000000000000f;
                                                }
                                                let _e3344 = phi_35590_;
                                                let _e3346 = phi_17709_;
                                                let _e3348 = phi_17710_;
                                                let _e3350 = phi_17711_;
                                                let _e3352 = phi_17712_;
                                                phi_35589_ = _e3344;
                                                phi_17713_ = _e3346;
                                                phi_17714_ = _e3348;
                                                phi_17715_ = _e3350;
                                                phi_17716_ = _e3352;
                                            }
                                            let _e4594 = phi_35589_;
                                            let _e4596 = phi_17713_;
                                            let _e4598 = phi_17714_;
                                            let _e4600 = phi_17715_;
                                            let _e4602 = phi_17716_;
                                            phi_35588_ = _e4594;
                                            phi_17717_ = _e4596;
                                            phi_17718_ = _e4598;
                                            phi_17719_ = _e4600;
                                            phi_17720_ = _e4602;
                                        }
                                        let _e4671 = phi_35588_;
                                        let _e4673 = phi_17717_;
                                        let _e4675 = phi_17718_;
                                        let _e4677 = phi_17719_;
                                        let _e4679 = phi_17720_;
                                        if ((((_e4677.x * _e4677.x) + (_e4677.y * _e4677.y)) + (_e4677.z * _e4677.z)) > 0f) {
                                            let _e4696 = (((_e4675.x * _e1702.x) + (_e4675.y * _e1702.y)) + (_e4675.z * _e1702.z));
                                            if (_e4696 != _e4696) {
                                                phi_28713_ = true;
                                            } else {
                                                phi_28713_ = (0f >= _e4696);
                                            }
                                            let _e4700 = phi_28713_;
                                            let _e4702 = (select(_e4696, 0f, _e4700) * 0.31830987f);
                                            if (_e2594 == 0u) {
                                                if (_e4696 <= 0f) {
                                                    phi_29510_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_29510_ = (_e2588 * (_e4696 * 0.31830987f));
                                                }
                                                let _e5178 = phi_29510_;
                                                phi_18116_ = type_56(_e5178, _e4702);
                                                phi_18117_ = type_56();
                                                phi_18118_ = true;
                                            } else {
                                                let _e4708 = (((_e1674 * _e1750) + (_e1675 * _e1751)) + (_e1676 * _e1753));
                                                let _e4713 = (((_e1674 * _e1746) + (_e1675 * _e1756)) + (_e1676 * _e1757));
                                                let _e4718 = (((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z));
                                                let _e4723 = (((_e4675.x * _e1750) + (_e4675.y * _e1751)) + (_e4675.z * _e1753));
                                                let _e4728 = (((_e4675.x * _e1746) + (_e4675.y * _e1756)) + (_e4675.z * _e1757));
                                                if (_e4718 <= 0f) {
                                                    phi_18080_ = type_56();
                                                    phi_18081_ = true;
                                                } else {
                                                    let _e4730 = (_e4696 <= 0f);
                                                    if _e4730 {
                                                        phi_18079_ = type_56();
                                                    } else {
                                                        let _e4731 = (_e4708 + _e4723);
                                                        let _e4732 = (_e4713 + _e4728);
                                                        let _e4733 = (_e4718 + _e4696);
                                                        let _e4740 = (1f / sqrt((((_e4731 * _e4731) + (_e4732 * _e4732)) + (_e4733 * _e4733))));
                                                        let _e4743 = (_e4733 * _e4740);
                                                        if (_e4743 <= 0f) {
                                                            phi_28823_ = 0f;
                                                        } else {
                                                            let _e4745 = (_e2592 * _e2592);
                                                            let _e4749 = ((((_e4745 - 1f) * _e4743) * _e4743) + 1f);
                                                            phi_28823_ = (_e4745 / ((3.1415927f * _e4749) * _e4749));
                                                        }
                                                        let _e4754 = phi_28823_;
                                                        let _e4755 = (_e4718 * _e4718);
                                                        if (_e4755 >= 1f) {
                                                            phi_28851_ = 0f;
                                                        } else {
                                                            phi_28851_ = ((sqrt((1f + (((_e2592 * _e2592) * ((_e4708 * _e4708) + (_e4713 * _e4713))) / _e4755))) - 1f) * 0.5f);
                                                        }
                                                        let _e4768 = phi_28851_;
                                                        let _e4769 = (1f + _e4768);
                                                        let _e4770 = (_e4696 * _e4696);
                                                        if (_e4770 >= 1f) {
                                                            phi_28879_ = 0f;
                                                        } else {
                                                            phi_28879_ = ((sqrt((1f + (((_e2592 * _e2592) * ((_e4723 * _e4723) + (_e4728 * _e4728))) / _e4770))) - 1f) * 0.5f);
                                                        }
                                                        let _e4783 = phi_28879_;
                                                        let _e4790 = (((_e4708 * (_e4731 * _e4740)) + (_e4713 * (_e4732 * _e4740))) + (_e4718 * _e4743));
                                                        if (_e2590 == 0u) {
                                                            let _e5085 = (1f - _e4790);
                                                            if (_e5085 != _e5085) {
                                                                phi_29382_ = true;
                                                            } else {
                                                                phi_29382_ = (0f >= _e5085);
                                                            }
                                                            let _e5089 = phi_29382_;
                                                            let _e5091 = pow(select(_e5085, 0f, _e5089), 5f);
                                                            let _e5092 = (1f - _e5091);
                                                            phi_18067_ = vec3<f32>(((_e2588.x * _e5092) + _e5091), ((_e2588.y * _e5092) + _e5091), ((_e2588.z * _e5092) + _e5091));
                                                        } else {
                                                            if (_e2590 == 1u) {
                                                                let _e5058 = (1f / _e2584);
                                                                let _e5062 = ((_e5058 * _e5058) * (1f - (_e4790 * _e4790)));
                                                                if (_e5062 >= 1f) {
                                                                    phi_29343_ = 1f;
                                                                } else {
                                                                    let _e5065 = sqrt((1f - _e5062));
                                                                    if (_e4790 != _e4790) {
                                                                        phi_29354_ = true;
                                                                    } else {
                                                                        phi_29354_ = (_e5065 <= _e4790);
                                                                    }
                                                                    let _e5069 = phi_29354_;
                                                                    let _e5074 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                                    phi_29343_ = (_e5074 + ((1f - _e5074) * pow((1f - select(_e4790, _e5065, _e5069)), 5f)));
                                                                }
                                                                let _e5081 = phi_29343_;
                                                                phi_18066_ = vec3(_e5081);
                                                            } else {
                                                                let _e4794 = select(_e4790, 0f, (_e4790 < 0f));
                                                                let _e4796 = select(_e4794, 1f, (_e4794 > 1f));
                                                                let _e4798 = (1f - (_e4796 * _e4796));
                                                                let _e4811 = ((_e2588.x * _e2588.x) - (_e2586.x * _e2586.x));
                                                                let _e4812 = ((_e2588.y * _e2588.y) - (_e2586.y * _e2586.y));
                                                                let _e4813 = ((_e2588.z * _e2588.z) - (_e2586.z * _e2586.z));
                                                                let _e4815 = (_e2588.x * (_e2586.x + _e2586.x));
                                                                let _e4817 = (_e2588.y * (_e2586.y + _e2586.y));
                                                                let _e4819 = (_e2588.z * (_e2586.z + _e2586.z));
                                                                let _e4826 = ((_e4811 * _e4811) + (_e4815 * _e4815));
                                                                let _e4827 = ((_e4812 * _e4812) + (_e4817 * _e4817));
                                                                let _e4828 = ((_e4813 * _e4813) + (_e4819 * _e4819));
                                                                let _e4833 = ((_e4798 * _e4811) / _e4826);
                                                                let _e4834 = ((_e4798 * _e4812) / _e4827);
                                                                let _e4835 = ((_e4798 * _e4813) / _e4828);
                                                                let _e4836 = (1f - _e4833);
                                                                let _e4837 = (1f - _e4834);
                                                                let _e4838 = (1f - _e4835);
                                                                let _e4840 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4833, _e4834, _e4835));
                                                                let _e4841 = (_e4798 * _e4815);
                                                                let _e4842 = (_e4798 * _e4817);
                                                                let _e4843 = (_e4798 * _e4819);
                                                                let _e4848 = (vec3<f32>(_e4841, _e4842, _e4843) / vec3<f32>(_e4826, _e4827, _e4828));
                                                                let _e4861 = ((_e4840.x * _e4840.x) + (_e4848.x * _e4848.x));
                                                                let _e4862 = ((_e4840.y * _e4840.y) + (_e4848.y * _e4848.y));
                                                                let _e4863 = ((_e4840.z * _e4840.z) + (_e4848.z * _e4848.z));
                                                                if (_e4861 != _e4861) {
                                                                    phi_29013_ = true;
                                                                } else {
                                                                    phi_29013_ = (0f >= _e4861);
                                                                }
                                                                let _e4867 = phi_29013_;
                                                                let _e4869 = sqrt(select(_e4861, 0f, _e4867));
                                                                if (_e4862 != _e4862) {
                                                                    phi_29028_ = true;
                                                                } else {
                                                                    phi_29028_ = (0f >= _e4862);
                                                                }
                                                                let _e4873 = phi_29028_;
                                                                let _e4875 = sqrt(select(_e4862, 0f, _e4873));
                                                                if (_e4863 != _e4863) {
                                                                    phi_29043_ = true;
                                                                } else {
                                                                    phi_29043_ = (0f >= _e4863);
                                                                }
                                                                let _e4879 = phi_29043_;
                                                                let _e4881 = sqrt(select(_e4863, 0f, _e4879));
                                                                let _e4885 = ((_e4869 + _e4836) * 0.5f);
                                                                let _e4886 = ((_e4875 + _e4837) * 0.5f);
                                                                let _e4887 = ((_e4881 + _e4838) * 0.5f);
                                                                if (_e4885 != _e4885) {
                                                                    phi_29070_ = true;
                                                                } else {
                                                                    phi_29070_ = (0f >= _e4885);
                                                                }
                                                                let _e4891 = phi_29070_;
                                                                let _e4893 = sqrt(select(_e4885, 0f, _e4891));
                                                                if (_e4886 != _e4886) {
                                                                    phi_29085_ = true;
                                                                } else {
                                                                    phi_29085_ = (0f >= _e4886);
                                                                }
                                                                let _e4897 = phi_29085_;
                                                                let _e4899 = sqrt(select(_e4886, 0f, _e4897));
                                                                if (_e4887 != _e4887) {
                                                                    phi_29100_ = true;
                                                                } else {
                                                                    phi_29100_ = (0f >= _e4887);
                                                                }
                                                                let _e4903 = phi_29100_;
                                                                let _e4905 = sqrt(select(_e4887, 0f, _e4903));
                                                                let _e4906 = vec3<f32>(_e4893, _e4899, _e4905);
                                                                let _e4910 = ((_e4869 - _e4836) * 0.5f);
                                                                let _e4911 = ((_e4875 - _e4837) * 0.5f);
                                                                let _e4912 = ((_e4881 - _e4838) * 0.5f);
                                                                if (_e4910 != _e4910) {
                                                                    phi_29127_ = true;
                                                                } else {
                                                                    phi_29127_ = (0f >= _e4910);
                                                                }
                                                                let _e4916 = phi_29127_;
                                                                let _e4918 = sqrt(select(_e4910, 0f, _e4916));
                                                                if (_e4911 != _e4911) {
                                                                    phi_29142_ = true;
                                                                } else {
                                                                    phi_29142_ = (0f >= _e4911);
                                                                }
                                                                let _e4922 = phi_29142_;
                                                                let _e4924 = sqrt(select(_e4911, 0f, _e4922));
                                                                if (_e4912 != _e4912) {
                                                                    phi_29157_ = true;
                                                                } else {
                                                                    phi_29157_ = (0f >= _e4912);
                                                                }
                                                                let _e4928 = phi_29157_;
                                                                let _e4930 = sqrt(select(_e4912, 0f, _e4928));
                                                                if ((_e4841 / _e4826) < 0f) {
                                                                    phi_17943_ = -(_e4918);
                                                                } else {
                                                                    phi_17943_ = _e4918;
                                                                }
                                                                let _e4934 = phi_17943_;
                                                                if ((_e4842 / _e4827) < 0f) {
                                                                    phi_17951_ = -(_e4924);
                                                                } else {
                                                                    phi_17951_ = _e4924;
                                                                }
                                                                let _e4938 = phi_17951_;
                                                                if ((_e4843 / _e4828) < 0f) {
                                                                    phi_17959_ = -(_e4930);
                                                                } else {
                                                                    phi_17959_ = _e4930;
                                                                }
                                                                let _e4942 = phi_17959_;
                                                                let _e4946 = (_e2586.x * _e4796);
                                                                let _e4947 = (_e2586.y * _e4796);
                                                                let _e4948 = (_e2586.z * _e4796);
                                                                let _e4958 = vec3<f32>(((_e2588.x * _e4893) - (_e2586.x * _e4934)), ((_e2588.y * _e4899) - (_e2586.y * _e4938)), ((_e2588.z * _e4905) - (_e2586.z * _e4942)));
                                                                let _e4965 = ((_e2588.x * _e4934) + (_e2586.x * _e4893));
                                                                let _e4966 = ((_e2588.y * _e4938) + (_e2586.y * _e4899));
                                                                let _e4967 = ((_e2588.z * _e4942) + (_e2586.z * _e4905));
                                                                let _e4969 = vec3<f32>((_e2588.x * _e4796), (_e2588.y * _e4796), (_e2588.z * _e4796));
                                                                let _e4970 = (_e4969 - _e4906);
                                                                let _e4971 = (_e4946 - _e4934);
                                                                let _e4972 = (_e4947 - _e4938);
                                                                let _e4973 = (_e4948 - _e4942);
                                                                let _e4986 = (_e4969 + _e4906);
                                                                let _e4987 = (_e4946 + _e4934);
                                                                let _e4988 = (_e4947 + _e4938);
                                                                let _e4989 = (_e4948 + _e4942);
                                                                let _e4999 = ((_e4986.x * _e4986.x) + (_e4987 * _e4987));
                                                                let _e5000 = ((_e4986.y * _e4986.y) + (_e4988 * _e4988));
                                                                let _e5001 = ((_e4986.z * _e4986.z) + (_e4989 * _e4989));
                                                                let _e5011 = vec3(_e4796);
                                                                let _e5012 = (_e5011 - _e4958);
                                                                let _e5013 = -(vec3<f32>(_e4965, _e4966, _e4967));
                                                                let _e5029 = (_e5011 + _e4958);
                                                                let _e5039 = ((_e5029.x * _e5029.x) + (_e4965 * _e4965));
                                                                let _e5040 = ((_e5029.y * _e5029.y) + (_e4966 * _e4966));
                                                                let _e5041 = ((_e5029.z * _e5029.z) + (_e4967 * _e4967));
                                                                phi_18066_ = vec3<f32>((((((_e4970.x * _e4970.x) + (_e4971 * _e4971)) / select(0.000000000000000000000000000001f, _e4999, (_e4999 > 0.000000000000000000000000000001f))) + (((_e5012.x * _e5012.x) + (_e5013.x * _e5013.x)) / select(0.000000000000000000000000000001f, _e5039, (_e5039 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4970.y * _e4970.y) + (_e4972 * _e4972)) / select(0.000000000000000000000000000001f, _e5000, (_e5000 > 0.000000000000000000000000000001f))) + (((_e5012.y * _e5012.y) + (_e5013.y * _e5013.y)) / select(0.000000000000000000000000000001f, _e5040, (_e5040 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4970.z * _e4970.z) + (_e4973 * _e4973)) / select(0.000000000000000000000000000001f, _e5001, (_e5001 > 0.000000000000000000000000000001f))) + (((_e5012.z * _e5012.z) + (_e5013.z * _e5013.z)) / select(0.000000000000000000000000000001f, _e5041, (_e5041 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e5084 = phi_18066_;
                                                            phi_18067_ = _e5084;
                                                        }
                                                        let _e5104 = phi_18067_;
                                                        let _e5106 = (4f * _e4718);
                                                        phi_18079_ = type_56((_e5104 * (((1f / (_e4769 + _e4783)) * _e4754) / _e5106)), ((_e4754 / _e4769) / _e5106));
                                                    }
                                                    let _e5113 = phi_18079_;
                                                    phi_18080_ = _e5113;
                                                    phi_18081_ = _e4730;
                                                }
                                                let _e5115 = phi_18080_;
                                                let _e5117 = phi_18081_;
                                                if _e5117 {
                                                    phi_18086_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_18086_ = _e5115;
                                                }
                                                let _e5119 = phi_18086_;
                                                let _e5122 = (_e2594 == 1u);
                                                if _e5122 {
                                                    phi_18114_ = _e5119;
                                                    phi_18115_ = type_56();
                                                } else {
                                                    if (_e4718 != _e4718) {
                                                        phi_29419_ = true;
                                                    } else {
                                                        phi_29419_ = (0f >= _e4718);
                                                    }
                                                    let _e5126 = phi_29419_;
                                                    let _e5127 = select(_e4718, 0f, _e5126);
                                                    let _e5128 = (1f / _e2584);
                                                    let _e5132 = ((_e5128 * _e5128) * (1f - (_e5127 * _e5127)));
                                                    if (_e5132 >= 1f) {
                                                        phi_29438_ = 1f;
                                                    } else {
                                                        let _e5135 = sqrt((1f - _e5132));
                                                        if (_e5127 != _e5127) {
                                                            phi_29449_ = true;
                                                        } else {
                                                            phi_29449_ = (_e5135 <= _e5127);
                                                        }
                                                        let _e5139 = phi_29449_;
                                                        let _e5144 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                        phi_29438_ = (_e5144 + ((1f - _e5144) * pow((1f - select(_e5127, _e5135, _e5139)), 5f)));
                                                    }
                                                    let _e5151 = phi_29438_;
                                                    if (_e4696 <= 0f) {
                                                        phi_29488_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_29488_ = (_e2582 * (_e4696 * 0.31830987f));
                                                    }
                                                    let _e5156 = phi_29488_;
                                                    let _e5157 = (1f - _e5151);
                                                    phi_18114_ = type_56();
                                                    phi_18115_ = type_56((_e5119.member + vec3<f32>((_e5156.x * _e5157), (_e5156.y * _e5157), (_e5156.z * _e5157))), ((_e5151 * _e5119.member_1) + (_e5157 * _e4702)));
                                                }
                                                let _e5171 = phi_18114_;
                                                let _e5173 = phi_18115_;
                                                phi_18116_ = _e5171;
                                                phi_18117_ = _e5173;
                                                phi_18118_ = _e5122;
                                            }
                                            let _e5181 = phi_18116_;
                                            let _e5183 = phi_18117_;
                                            let _e5185 = phi_18118_;
                                            if _e5185 {
                                                phi_18122_ = _e5181;
                                            } else {
                                                phi_18122_ = _e5183;
                                            }
                                            let _e5187 = phi_18122_;
                                            if (_e4673 > 0f) {
                                                if (_e4673 >= 300000000000000000000000000000000000000f) {
                                                    phi_29527_ = 1f;
                                                } else {
                                                    let _e5192 = (_e4673 * _e4673);
                                                    let _e5194 = (_e5192 + (_e5187.member_1 * _e5187.member_1));
                                                    if (_e5194 == 0f) {
                                                        phi_29524_ = 0f;
                                                    } else {
                                                        phi_29524_ = (_e5192 / _e5194);
                                                    }
                                                    let _e5198 = phi_29524_;
                                                    phi_29527_ = _e5198;
                                                }
                                                let _e5200 = phi_29527_;
                                                phi_18131_ = (_e5200 / _e4673);
                                            } else {
                                                phi_18131_ = 1f;
                                            }
                                            let _e5203 = phi_18131_;
                                            if ((((_e5187.member.x * _e5187.member.x) + (_e5187.member.y * _e5187.member.y)) + (_e5187.member.z * _e5187.member.z)) > 0f) {
                                                let _e5215 = (vec3<f32>(1f, 1f, 1f) / _e4675);
                                                local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                                phi_18165_ = _e377;
                                                phi_18168_ = 0u;
                                                phi_18170_ = _e374;
                                                phi_18172_ = _e370;
                                                phi_18176_ = type_66(_e4679, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                loop {
                                                    let _e5217 = phi_18165_;
                                                    let _e5219 = phi_18168_;
                                                    let _e5221 = phi_18170_;
                                                    let _e5223 = phi_18172_;
                                                    let _e5225 = phi_18176_;
                                                    local_108 = _e5225;
                                                    if (_e5223 < _e5221) {
                                                        let _e5583 = global_2.member[_e5223];
                                                        if (_e5583.member_6.x == 0u) {
                                                            let _e5689 = (_e5583.member_1.x - _e5583.member.x);
                                                            let _e5690 = (_e5583.member_1.y - _e5583.member.y);
                                                            let _e5691 = (_e5583.member_1.z - _e5583.member.z);
                                                            let _e5696 = (_e5583.member_2.x - _e5583.member.x);
                                                            let _e5697 = (_e5583.member_2.y - _e5583.member.y);
                                                            let _e5698 = (_e5583.member_2.z - _e5583.member.z);
                                                            let _e5701 = ((_e4675.y * _e5698) - (_e5697 * _e4675.z));
                                                            let _e5704 = ((_e4675.z * _e5696) - (_e5698 * _e4675.x));
                                                            let _e5707 = ((_e4675.x * _e5697) - (_e5696 * _e4675.y));
                                                            let _e5712 = (((_e5689 * _e5701) + (_e5690 * _e5704)) + (_e5691 * _e5707));
                                                            if (abs(_e5712) < 0.000000000001f) {
                                                                phi_18343_ = vec4<f32>();
                                                                phi_18344_ = true;
                                                            } else {
                                                                let _e5715 = (1f / _e5712);
                                                                let _e5716 = (_e1727 - _e5583.member.x);
                                                                let _e5717 = (_e1728 - _e5583.member.y);
                                                                let _e5718 = (_e1729 - _e5583.member.z);
                                                                let _e5724 = ((((_e5716 * _e5701) + (_e5717 * _e5704)) + (_e5718 * _e5707)) * _e5715);
                                                                if (_e5724 < 0f) {
                                                                    phi_18333_ = vec4<f32>();
                                                                    phi_18334_ = true;
                                                                    phi_18335_ = false;
                                                                } else {
                                                                    let _e5726 = (_e5724 > 1f);
                                                                    if _e5726 {
                                                                        phi_18331_ = vec4<f32>();
                                                                        phi_18332_ = false;
                                                                    } else {
                                                                        let _e5729 = ((_e5717 * _e5691) - (_e5690 * _e5718));
                                                                        let _e5732 = ((_e5718 * _e5689) - (_e5691 * _e5716));
                                                                        let _e5735 = ((_e5716 * _e5690) - (_e5689 * _e5717));
                                                                        let _e5741 = ((((_e4675.x * _e5729) + (_e4675.y * _e5732)) + (_e4675.z * _e5735)) * _e5715);
                                                                        if (_e5741 < 0f) {
                                                                            phi_18321_ = vec4<f32>();
                                                                            phi_18322_ = true;
                                                                            phi_18323_ = false;
                                                                        } else {
                                                                            let _e5744 = ((_e5724 + _e5741) > 1f);
                                                                            if _e5744 {
                                                                                phi_18319_ = vec4<f32>();
                                                                                phi_18320_ = false;
                                                                            } else {
                                                                                let _e5750 = ((((_e5696 * _e5729) + (_e5697 * _e5732)) + (_e5698 * _e5735)) * _e5715);
                                                                                if (_e5750 <= 0f) {
                                                                                    phi_18314_ = vec4<f32>();
                                                                                    phi_18315_ = true;
                                                                                } else {
                                                                                    let _e5752 = (_e5750 >= _e5225.member);
                                                                                    if _e5752 {
                                                                                        phi_18313_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_18313_ = vec4<f32>(_e5750, _e5724, _e5741, 1f);
                                                                                    }
                                                                                    let _e5755 = phi_18313_;
                                                                                    phi_18314_ = _e5755;
                                                                                    phi_18315_ = _e5752;
                                                                                }
                                                                                let _e5757 = phi_18314_;
                                                                                let _e5759 = phi_18315_;
                                                                                phi_18319_ = _e5757;
                                                                                phi_18320_ = _e5759;
                                                                            }
                                                                            let _e5761 = phi_18319_;
                                                                            let _e5763 = phi_18320_;
                                                                            phi_18321_ = _e5761;
                                                                            phi_18322_ = _e5744;
                                                                            phi_18323_ = _e5763;
                                                                        }
                                                                        let _e5765 = phi_18321_;
                                                                        let _e5767 = phi_18322_;
                                                                        let _e5769 = phi_18323_;
                                                                        phi_18331_ = _e5765;
                                                                        phi_18332_ = select(_e5769, true, _e5767);
                                                                    }
                                                                    let _e5772 = phi_18331_;
                                                                    let _e5774 = phi_18332_;
                                                                    phi_18333_ = _e5772;
                                                                    phi_18334_ = _e5726;
                                                                    phi_18335_ = _e5774;
                                                                }
                                                                let _e5776 = phi_18333_;
                                                                let _e5778 = phi_18334_;
                                                                let _e5780 = phi_18335_;
                                                                phi_18343_ = _e5776;
                                                                phi_18344_ = select(_e5780, true, _e5778);
                                                            }
                                                            let _e5783 = phi_18343_;
                                                            let _e5785 = phi_18344_;
                                                            phi_18490_ = _e5783;
                                                            phi_18491_ = _e5785;
                                                        } else {
                                                            if (_e5583.member_6.x == 1u) {
                                                                let _e5611 = (_e1727 - _e5583.member.x);
                                                                let _e5612 = (_e1728 - _e5583.member.y);
                                                                let _e5613 = (_e1729 - _e5583.member.z);
                                                                let _e5618 = (((_e5611 * _e4675.x) + (_e5612 * _e4675.y)) + (_e5613 * _e4675.z));
                                                                if (_e5618 > 0f) {
                                                                    phi_18384_ = select(true, false, ((((_e5611 * _e5611) + (_e5612 * _e5612)) + (_e5613 * _e5613)) > (_e5583.member.w * _e5583.member.w)));
                                                                } else {
                                                                    phi_18384_ = true;
                                                                }
                                                                let _e5631 = phi_18384_;
                                                                if _e5631 {
                                                                    let _e5635 = (_e5611 - (_e4675.x * _e5618));
                                                                    let _e5636 = (_e5612 - (_e4675.y * _e5618));
                                                                    let _e5637 = (_e5613 - (_e4675.z * _e5618));
                                                                    let _e5646 = ((_e5583.member.w * _e5583.member.w) - (((_e5635 * _e5635) + (_e5636 * _e5636)) + (_e5637 * _e5637)));
                                                                    if (_e5646 < 0f) {
                                                                        phi_18443_ = vec4<f32>();
                                                                        phi_18444_ = true;
                                                                    } else {
                                                                        let _e5648 = sqrt(_e5646);
                                                                        let _e5649 = -(_e5618);
                                                                        let _e5650 = (_e5649 - _e5648);
                                                                        if (_e5650 <= 0f) {
                                                                            phi_18421_ = true;
                                                                        } else {
                                                                            phi_18421_ = (_e5650 >= _e5225.member);
                                                                        }
                                                                        let _e5654 = phi_18421_;
                                                                        if _e5654 {
                                                                            phi_18427_ = (_e5649 + _e5648);
                                                                        } else {
                                                                            phi_18427_ = _e5650;
                                                                        }
                                                                        let _e5657 = phi_18427_;
                                                                        if (_e5657 <= 0f) {
                                                                            phi_18438_ = vec4<f32>();
                                                                            phi_18439_ = true;
                                                                        } else {
                                                                            let _e5659 = (_e5657 >= _e5225.member);
                                                                            if _e5659 {
                                                                                phi_18437_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18437_ = vec4<f32>(_e5657, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5662 = phi_18437_;
                                                                            phi_18438_ = _e5662;
                                                                            phi_18439_ = _e5659;
                                                                        }
                                                                        let _e5664 = phi_18438_;
                                                                        let _e5666 = phi_18439_;
                                                                        phi_18443_ = _e5664;
                                                                        phi_18444_ = _e5666;
                                                                    }
                                                                    let _e5668 = phi_18443_;
                                                                    let _e5670 = phi_18444_;
                                                                    phi_18448_ = _e5668;
                                                                    phi_18449_ = _e5670;
                                                                } else {
                                                                    phi_18448_ = vec4<f32>();
                                                                    phi_18449_ = true;
                                                                }
                                                                let _e5672 = phi_18448_;
                                                                let _e5674 = phi_18449_;
                                                                phi_18480_ = _e5672;
                                                                phi_18481_ = _e5674;
                                                            } else {
                                                                if (abs(_e4675.y) < 0.000000000001f) {
                                                                    phi_18478_ = vec4<f32>();
                                                                    phi_18479_ = true;
                                                                } else {
                                                                    let _e5593 = ((_e5583.member.x - _e1728) / _e4675.y);
                                                                    if (_e5593 <= 0f) {
                                                                        phi_18473_ = vec4<f32>();
                                                                        phi_18474_ = true;
                                                                    } else {
                                                                        let _e5595 = (_e5593 >= _e5225.member);
                                                                        if _e5595 {
                                                                            phi_18472_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18472_ = vec4<f32>(_e5593, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5598 = phi_18472_;
                                                                        phi_18473_ = _e5598;
                                                                        phi_18474_ = _e5595;
                                                                    }
                                                                    let _e5600 = phi_18473_;
                                                                    let _e5602 = phi_18474_;
                                                                    phi_18478_ = _e5600;
                                                                    phi_18479_ = _e5602;
                                                                }
                                                                let _e5604 = phi_18478_;
                                                                let _e5606 = phi_18479_;
                                                                phi_18480_ = _e5604;
                                                                phi_18481_ = _e5606;
                                                            }
                                                            let _e5676 = phi_18480_;
                                                            let _e5678 = phi_18481_;
                                                            phi_18490_ = _e5676;
                                                            phi_18491_ = select(true, false, select(true, false, _e5678));
                                                        }
                                                        let _e5787 = phi_18490_;
                                                        let _e5789 = phi_18491_;
                                                        let _e5792 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5787, vec4(select(true, false, _e5789)));
                                                        let _e5794 = (_e5792.w > 0f);
                                                        if _e5794 {
                                                            let _e5800 = type_66(_e5792.x, _e5225.member_1, _e5225.member_2, _e5225.member_3);
                                                            let _e5805 = type_66(_e5800.member, _e5223, _e5800.member_2, _e5800.member_3);
                                                            phi_18512_ = _e5805;
                                                            phi_18514_ = _e5805;
                                                        } else {
                                                            phi_18512_ = type_66();
                                                            phi_18514_ = _e5225;
                                                        }
                                                        let _e5807 = phi_18512_;
                                                        let _e5809 = phi_18514_;
                                                        let _e5810 = select(true, false, _e5794);
                                                        if _e5810 {
                                                            phi_18520_ = (_e5223 + 1u);
                                                        } else {
                                                            phi_18520_ = u32();
                                                        }
                                                        let _e5813 = phi_18520_;
                                                        phi_18166_ = _e5217;
                                                        phi_18169_ = _e5219;
                                                        phi_18171_ = _e5221;
                                                        phi_18173_ = _e5813;
                                                        phi_18177_ = _e5809;
                                                        phi_18684_ = _e5807;
                                                        phi_18685_ = _e5810;
                                                        phi_18686_ = false;
                                                    } else {
                                                        if (_e5217 == 4294967295u) {
                                                            let _e5228 = (_e5219 == 0u);
                                                            if _e5228 {
                                                                phi_18551_ = u32();
                                                                phi_18552_ = u32();
                                                                phi_18554_ = false;
                                                                phi_18555_ = false;
                                                            } else {
                                                                let _e5229 = (_e5219 - 1u);
                                                                let _e5230 = (_e5229 < 64u);
                                                                if _e5230 {
                                                                } else {
                                                                    phi_35100_ = true;
                                                                    phi_20501_ = bool();
                                                                    phi_20500_ = type_66();
                                                                    break;
                                                                }
                                                                let _e5232 = local_68[_e5229];
                                                                let _e5234 = (_e5232 >= _e5225.member);
                                                                if _e5234 {
                                                                    phi_18549_ = u32();
                                                                } else {
                                                                    if _e5230 {
                                                                    } else {
                                                                        phi_35100_ = true;
                                                                        phi_20501_ = bool();
                                                                        phi_20500_ = type_66();
                                                                        break;
                                                                    }
                                                                    let _e5236 = local_67[_e5229];
                                                                    phi_18549_ = _e5236;
                                                                }
                                                                let _e5238 = phi_18549_;
                                                                phi_18551_ = _e5229;
                                                                phi_18552_ = _e5238;
                                                                phi_18554_ = select(true, false, _e5234);
                                                                phi_18555_ = _e5234;
                                                            }
                                                            let _e5241 = phi_18551_;
                                                            let _e5243 = phi_18552_;
                                                            let _e5245 = phi_18554_;
                                                            let _e5247 = phi_18555_;
                                                            phi_18556_ = _e5241;
                                                            phi_18557_ = _e5243;
                                                            phi_18558_ = _e5241;
                                                            phi_18559_ = _e5245;
                                                            phi_18560_ = _e5247;
                                                            phi_18561_ = _e5228;
                                                        } else {
                                                            phi_18556_ = u32();
                                                            phi_18557_ = _e5217;
                                                            phi_18558_ = _e5219;
                                                            phi_18559_ = true;
                                                            phi_18560_ = false;
                                                            phi_18561_ = false;
                                                        }
                                                        let _e5249 = phi_18556_;
                                                        let _e5251 = phi_18557_;
                                                        let _e5253 = phi_18558_;
                                                        let _e5255 = phi_18559_;
                                                        let _e5257 = phi_18560_;
                                                        let _e5259 = phi_18561_;
                                                        if _e5255 {
                                                            let _e5261 = ((_e5251 & 2147483648u) == 0u);
                                                            if _e5261 {
                                                                let _e5269 = global_3.member[_e5251];
                                                                let _e5280 = ((_e5269.member.x - _e1727) * _e5215.x);
                                                                let _e5282 = ((_e5269.member.y - _e1728) * _e5215.y);
                                                                let _e5284 = ((_e5269.member.z - _e1729) * _e5215.z);
                                                                let _e5291 = ((_e5269.member_2.x - _e1727) * _e5215.x);
                                                                let _e5292 = ((_e5269.member_2.y - _e1728) * _e5215.y);
                                                                let _e5293 = ((_e5269.member_2.z - _e1729) * _e5215.z);
                                                                if (_e5280 == _e5280) {
                                                                    let _e5295 = (_e5291 == _e5291);
                                                                    if _e5295 {
                                                                        let _e5296 = (_e5280 != _e5280);
                                                                        if _e5296 {
                                                                            phi_29625_ = true;
                                                                        } else {
                                                                            phi_29625_ = (_e5291 <= _e5280);
                                                                        }
                                                                        let _e5299 = phi_29625_;
                                                                        let _e5300 = select(_e5280, _e5291, _e5299);
                                                                        if _e5296 {
                                                                            phi_29655_ = true;
                                                                        } else {
                                                                            phi_29655_ = (_e5291 >= _e5280);
                                                                        }
                                                                        let _e5305 = phi_29655_;
                                                                        let _e5306 = select(_e5280, _e5291, _e5305);
                                                                        phi_29602_ = type_53(select(-300000000000000000000000000000000000000f, _e5300, (_e5300 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5306, (_e5306 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29602_ = type_53();
                                                                    }
                                                                    let _e5311 = phi_29602_;
                                                                    phi_29605_ = _e5311;
                                                                    phi_29606_ = select(true, false, _e5295);
                                                                } else {
                                                                    phi_29605_ = type_53();
                                                                    phi_29606_ = true;
                                                                }
                                                                let _e5314 = phi_29605_;
                                                                let _e5316 = phi_29606_;
                                                                if _e5316 {
                                                                    phi_29611_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29611_ = _e5314;
                                                                }
                                                                let _e5318 = phi_29611_;
                                                                if (_e5282 == _e5282) {
                                                                    let _e5322 = (_e5292 == _e5292);
                                                                    if _e5322 {
                                                                        let _e5323 = (_e5282 != _e5282);
                                                                        if _e5323 {
                                                                            phi_29712_ = true;
                                                                        } else {
                                                                            phi_29712_ = (_e5292 <= _e5282);
                                                                        }
                                                                        let _e5326 = phi_29712_;
                                                                        let _e5327 = select(_e5282, _e5292, _e5326);
                                                                        if (_e5318.member != _e5318.member) {
                                                                            phi_29727_ = true;
                                                                        } else {
                                                                            phi_29727_ = (_e5327 >= _e5318.member);
                                                                        }
                                                                        let _e5331 = phi_29727_;
                                                                        if _e5323 {
                                                                            phi_29742_ = true;
                                                                        } else {
                                                                            phi_29742_ = (_e5292 >= _e5282);
                                                                        }
                                                                        let _e5335 = phi_29742_;
                                                                        let _e5336 = select(_e5282, _e5292, _e5335);
                                                                        if (_e5318.member_1 != _e5318.member_1) {
                                                                            phi_29757_ = true;
                                                                        } else {
                                                                            phi_29757_ = (_e5336 <= _e5318.member_1);
                                                                        }
                                                                        let _e5340 = phi_29757_;
                                                                        phi_29689_ = type_53(select(_e5318.member, _e5327, _e5331), select(_e5318.member_1, _e5336, _e5340));
                                                                    } else {
                                                                        phi_29689_ = type_53();
                                                                    }
                                                                    let _e5344 = phi_29689_;
                                                                    phi_29692_ = _e5344;
                                                                    phi_29693_ = select(true, false, _e5322);
                                                                } else {
                                                                    phi_29692_ = type_53();
                                                                    phi_29693_ = true;
                                                                }
                                                                let _e5347 = phi_29692_;
                                                                let _e5349 = phi_29693_;
                                                                if _e5349 {
                                                                    phi_29698_ = _e5318;
                                                                } else {
                                                                    phi_29698_ = _e5347;
                                                                }
                                                                let _e5351 = phi_29698_;
                                                                if (_e5284 == _e5284) {
                                                                    let _e5355 = (_e5293 == _e5293);
                                                                    if _e5355 {
                                                                        let _e5356 = (_e5284 != _e5284);
                                                                        if _e5356 {
                                                                            phi_29799_ = true;
                                                                        } else {
                                                                            phi_29799_ = (_e5293 <= _e5284);
                                                                        }
                                                                        let _e5359 = phi_29799_;
                                                                        let _e5360 = select(_e5284, _e5293, _e5359);
                                                                        if (_e5351.member != _e5351.member) {
                                                                            phi_29814_ = true;
                                                                        } else {
                                                                            phi_29814_ = (_e5360 >= _e5351.member);
                                                                        }
                                                                        let _e5364 = phi_29814_;
                                                                        if _e5356 {
                                                                            phi_29829_ = true;
                                                                        } else {
                                                                            phi_29829_ = (_e5293 >= _e5284);
                                                                        }
                                                                        let _e5368 = phi_29829_;
                                                                        let _e5369 = select(_e5284, _e5293, _e5368);
                                                                        if (_e5351.member_1 != _e5351.member_1) {
                                                                            phi_29844_ = true;
                                                                        } else {
                                                                            phi_29844_ = (_e5369 <= _e5351.member_1);
                                                                        }
                                                                        let _e5373 = phi_29844_;
                                                                        phi_29776_ = type_53(select(_e5351.member, _e5360, _e5364), select(_e5351.member_1, _e5369, _e5373));
                                                                    } else {
                                                                        phi_29776_ = type_53();
                                                                    }
                                                                    let _e5377 = phi_29776_;
                                                                    phi_29779_ = _e5377;
                                                                    phi_29780_ = select(true, false, _e5355);
                                                                } else {
                                                                    phi_29779_ = type_53();
                                                                    phi_29780_ = true;
                                                                }
                                                                let _e5380 = phi_29779_;
                                                                let _e5382 = phi_29780_;
                                                                if _e5382 {
                                                                    phi_29785_ = _e5351;
                                                                } else {
                                                                    phi_29785_ = _e5380;
                                                                }
                                                                let _e5384 = phi_29785_;
                                                                if (_e5384.member_1 < 0f) {
                                                                    phi_29583_ = true;
                                                                } else {
                                                                    if (_e5384.member > _e5384.member_1) {
                                                                        phi_29580_ = true;
                                                                    } else {
                                                                        phi_29580_ = (_e5384.member > _e5225.member);
                                                                    }
                                                                    let _e5391 = phi_29580_;
                                                                    phi_29583_ = _e5391;
                                                                }
                                                                let _e5393 = phi_29583_;
                                                                let _e5394 = select(_e5384.member, 300000000000000000000000000000000000000f, _e5393);
                                                                let _e5403 = ((_e5269.member_4.x - _e1727) * _e5215.x);
                                                                let _e5404 = ((_e5269.member_4.y - _e1728) * _e5215.y);
                                                                let _e5405 = ((_e5269.member_4.z - _e1729) * _e5215.z);
                                                                let _e5412 = ((_e5269.member_6.x - _e1727) * _e5215.x);
                                                                let _e5413 = ((_e5269.member_6.y - _e1728) * _e5215.y);
                                                                let _e5414 = ((_e5269.member_6.z - _e1729) * _e5215.z);
                                                                if (_e5403 == _e5403) {
                                                                    let _e5416 = (_e5412 == _e5412);
                                                                    if _e5416 {
                                                                        let _e5417 = (_e5403 != _e5403);
                                                                        if _e5417 {
                                                                            phi_29946_ = true;
                                                                        } else {
                                                                            phi_29946_ = (_e5412 <= _e5403);
                                                                        }
                                                                        let _e5420 = phi_29946_;
                                                                        let _e5421 = select(_e5403, _e5412, _e5420);
                                                                        if _e5417 {
                                                                            phi_29976_ = true;
                                                                        } else {
                                                                            phi_29976_ = (_e5412 >= _e5403);
                                                                        }
                                                                        let _e5426 = phi_29976_;
                                                                        let _e5427 = select(_e5403, _e5412, _e5426);
                                                                        phi_29923_ = type_53(select(-300000000000000000000000000000000000000f, _e5421, (_e5421 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5427, (_e5427 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29923_ = type_53();
                                                                    }
                                                                    let _e5432 = phi_29923_;
                                                                    phi_29926_ = _e5432;
                                                                    phi_29927_ = select(true, false, _e5416);
                                                                } else {
                                                                    phi_29926_ = type_53();
                                                                    phi_29927_ = true;
                                                                }
                                                                let _e5435 = phi_29926_;
                                                                let _e5437 = phi_29927_;
                                                                if _e5437 {
                                                                    phi_29932_ = type_53(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29932_ = _e5435;
                                                                }
                                                                let _e5439 = phi_29932_;
                                                                if (_e5404 == _e5404) {
                                                                    let _e5443 = (_e5413 == _e5413);
                                                                    if _e5443 {
                                                                        let _e5444 = (_e5404 != _e5404);
                                                                        if _e5444 {
                                                                            phi_30033_ = true;
                                                                        } else {
                                                                            phi_30033_ = (_e5413 <= _e5404);
                                                                        }
                                                                        let _e5447 = phi_30033_;
                                                                        let _e5448 = select(_e5404, _e5413, _e5447);
                                                                        if (_e5439.member != _e5439.member) {
                                                                            phi_30048_ = true;
                                                                        } else {
                                                                            phi_30048_ = (_e5448 >= _e5439.member);
                                                                        }
                                                                        let _e5452 = phi_30048_;
                                                                        if _e5444 {
                                                                            phi_30063_ = true;
                                                                        } else {
                                                                            phi_30063_ = (_e5413 >= _e5404);
                                                                        }
                                                                        let _e5456 = phi_30063_;
                                                                        let _e5457 = select(_e5404, _e5413, _e5456);
                                                                        if (_e5439.member_1 != _e5439.member_1) {
                                                                            phi_30078_ = true;
                                                                        } else {
                                                                            phi_30078_ = (_e5457 <= _e5439.member_1);
                                                                        }
                                                                        let _e5461 = phi_30078_;
                                                                        phi_30010_ = type_53(select(_e5439.member, _e5448, _e5452), select(_e5439.member_1, _e5457, _e5461));
                                                                    } else {
                                                                        phi_30010_ = type_53();
                                                                    }
                                                                    let _e5465 = phi_30010_;
                                                                    phi_30013_ = _e5465;
                                                                    phi_30014_ = select(true, false, _e5443);
                                                                } else {
                                                                    phi_30013_ = type_53();
                                                                    phi_30014_ = true;
                                                                }
                                                                let _e5468 = phi_30013_;
                                                                let _e5470 = phi_30014_;
                                                                if _e5470 {
                                                                    phi_30019_ = _e5439;
                                                                } else {
                                                                    phi_30019_ = _e5468;
                                                                }
                                                                let _e5472 = phi_30019_;
                                                                if (_e5405 == _e5405) {
                                                                    let _e5476 = (_e5414 == _e5414);
                                                                    if _e5476 {
                                                                        let _e5477 = (_e5405 != _e5405);
                                                                        if _e5477 {
                                                                            phi_30120_ = true;
                                                                        } else {
                                                                            phi_30120_ = (_e5414 <= _e5405);
                                                                        }
                                                                        let _e5480 = phi_30120_;
                                                                        let _e5481 = select(_e5405, _e5414, _e5480);
                                                                        if (_e5472.member != _e5472.member) {
                                                                            phi_30135_ = true;
                                                                        } else {
                                                                            phi_30135_ = (_e5481 >= _e5472.member);
                                                                        }
                                                                        let _e5485 = phi_30135_;
                                                                        if _e5477 {
                                                                            phi_30150_ = true;
                                                                        } else {
                                                                            phi_30150_ = (_e5414 >= _e5405);
                                                                        }
                                                                        let _e5489 = phi_30150_;
                                                                        let _e5490 = select(_e5405, _e5414, _e5489);
                                                                        if (_e5472.member_1 != _e5472.member_1) {
                                                                            phi_30165_ = true;
                                                                        } else {
                                                                            phi_30165_ = (_e5490 <= _e5472.member_1);
                                                                        }
                                                                        let _e5494 = phi_30165_;
                                                                        phi_30097_ = type_53(select(_e5472.member, _e5481, _e5485), select(_e5472.member_1, _e5490, _e5494));
                                                                    } else {
                                                                        phi_30097_ = type_53();
                                                                    }
                                                                    let _e5498 = phi_30097_;
                                                                    phi_30100_ = _e5498;
                                                                    phi_30101_ = select(true, false, _e5476);
                                                                } else {
                                                                    phi_30100_ = type_53();
                                                                    phi_30101_ = true;
                                                                }
                                                                let _e5501 = phi_30100_;
                                                                let _e5503 = phi_30101_;
                                                                if _e5503 {
                                                                    phi_30106_ = _e5472;
                                                                } else {
                                                                    phi_30106_ = _e5501;
                                                                }
                                                                let _e5505 = phi_30106_;
                                                                if (_e5505.member_1 < 0f) {
                                                                    phi_29904_ = true;
                                                                } else {
                                                                    if (_e5505.member > _e5505.member_1) {
                                                                        phi_29901_ = true;
                                                                    } else {
                                                                        phi_29901_ = (_e5505.member > _e5225.member);
                                                                    }
                                                                    let _e5512 = phi_29901_;
                                                                    phi_29904_ = _e5512;
                                                                }
                                                                let _e5514 = phi_29904_;
                                                                let _e5515 = select(_e5505.member, 300000000000000000000000000000000000000f, _e5514);
                                                                if (_e5269.member_3 > 0u) {
                                                                    phi_18600_ = ((2147483648u | ((_e5269.member_3 - 1u) << bitcast<u32>(28u))) | _e5269.member_1);
                                                                } else {
                                                                    phi_18600_ = _e5269.member_1;
                                                                }
                                                                let _e5525 = phi_18600_;
                                                                if (_e5269.member_7 > 0u) {
                                                                    phi_18611_ = ((2147483648u | ((_e5269.member_7 - 1u) << bitcast<u32>(28u))) | _e5269.member_5);
                                                                } else {
                                                                    phi_18611_ = _e5269.member_5;
                                                                }
                                                                let _e5535 = phi_18611_;
                                                                let _e5536 = (_e5394 <= _e5515);
                                                                let _e5540 = select(_e5394, _e5515, _e5536);
                                                                if (_e5540 < 300000000000000000000000000000000000000f) {
                                                                    let _e5542 = (_e5253 < 64u);
                                                                    if _e5542 {
                                                                        if _e5542 {
                                                                        } else {
                                                                            phi_35100_ = true;
                                                                            phi_20501_ = bool();
                                                                            phi_20500_ = type_66();
                                                                            break;
                                                                        }
                                                                        local_67[_e5253] = select(_e5525, _e5535, _e5536);
                                                                        if _e5542 {
                                                                        } else {
                                                                            phi_35100_ = true;
                                                                            phi_20501_ = bool();
                                                                            phi_20500_ = type_66();
                                                                            break;
                                                                        }
                                                                        local_68[_e5253] = _e5540;
                                                                        phi_18650_ = (_e5253 + 1u);
                                                                    } else {
                                                                        phi_18650_ = _e5253;
                                                                    }
                                                                    let _e5547 = phi_18650_;
                                                                    phi_18651_ = _e5547;
                                                                } else {
                                                                    phi_18651_ = _e5253;
                                                                }
                                                                let _e5549 = phi_18651_;
                                                                phi_18662_ = select(4294967295u, select(_e5535, _e5525, _e5536), (select(_e5515, _e5394, _e5536) < 300000000000000000000000000000000000000f));
                                                                phi_18663_ = _e5549;
                                                                phi_18664_ = u32();
                                                                phi_18665_ = u32();
                                                            } else {
                                                                let _e5262 = (_e5251 & 268435455u);
                                                                phi_18662_ = u32();
                                                                phi_18663_ = u32();
                                                                phi_18664_ = ((_e5262 + ((_e5251 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                                phi_18665_ = _e5262;
                                                            }
                                                            let _e5553 = phi_18662_;
                                                            let _e5555 = phi_18663_;
                                                            let _e5557 = phi_18664_;
                                                            let _e5559 = phi_18665_;
                                                            phi_18667_ = _e5553;
                                                            phi_18668_ = _e5555;
                                                            phi_18670_ = _e5557;
                                                            phi_18671_ = _e5559;
                                                            phi_18672_ = select(true, false, _e5261);
                                                            phi_18673_ = _e5261;
                                                        } else {
                                                            phi_18667_ = u32();
                                                            phi_18668_ = u32();
                                                            phi_18670_ = _e5221;
                                                            phi_18671_ = _e5223;
                                                            phi_18672_ = _e5257;
                                                            phi_18673_ = false;
                                                        }
                                                        let _e5562 = phi_18667_;
                                                        let _e5564 = phi_18668_;
                                                        let _e5566 = phi_18670_;
                                                        let _e5568 = phi_18671_;
                                                        let _e5570 = phi_18672_;
                                                        let _e5572 = phi_18673_;
                                                        phi_18166_ = select(_e5562, 4294967295u, _e5570);
                                                        phi_18169_ = select(_e5564, select(_e5249, _e5253, _e5255), _e5570);
                                                        phi_18171_ = select(_e5221, _e5566, _e5570);
                                                        phi_18173_ = select(_e5223, _e5568, _e5570);
                                                        phi_18177_ = _e5225;
                                                        phi_18684_ = type_66();
                                                        phi_18685_ = select(_e5572, true, _e5570);
                                                        phi_18686_ = select(select(_e5259, false, _e5255), false, _e5570);
                                                    }
                                                    let _e5815 = phi_18166_;
                                                    let _e5817 = phi_18169_;
                                                    let _e5819 = phi_18171_;
                                                    let _e5821 = phi_18173_;
                                                    let _e5823 = phi_18177_;
                                                    let _e5825 = phi_18684_;
                                                    let _e5827 = phi_18685_;
                                                    let _e5829 = phi_18686_;
                                                    phi_18165_ = _e5815;
                                                    phi_18168_ = _e5817;
                                                    phi_18170_ = _e5819;
                                                    phi_18172_ = _e5821;
                                                    phi_18176_ = _e5823;
                                                    phi_35100_ = _e2607;
                                                    phi_20501_ = _e5829;
                                                    phi_20500_ = _e5825;
                                                    if !(_e5827) {
                                                        break;
                                                    }
                                                }
                                                let _e5832 = phi_35100_;
                                                let _e5834 = phi_20501_;
                                                let _e5836 = phi_20500_;
                                                phi_35546_ = _e4671;
                                                phi_35532_ = _e5832;
                                                if _e5832 {
                                                    break;
                                                }
                                                if _e5834 {
                                                    let _e8406 = local_108;
                                                    phi_18858_ = _e8406;
                                                } else {
                                                    phi_18858_ = type_66();
                                                }
                                                let _e5838 = phi_18858_;
                                                if select(true, false, _e5834) {
                                                    phi_18863_ = _e5836;
                                                } else {
                                                    phi_18863_ = _e5838;
                                                }
                                                let _e5841 = phi_18863_;
                                                if (_e5841.member_1 != 4294967295u) {
                                                    phi_18917_ = _e2611;
                                                    phi_18918_ = _e2613;
                                                } else {
                                                    if (_e2615 == (_e2605 + 2u)) {
                                                        phi_18915_ = _e2611;
                                                        phi_18916_ = type_39(vec4<f32>(_e1727, _e1728, _e1729, -1f), vec4<f32>(_e4675.x, _e4675.y, _e4675.z, 0f), vec4<f32>(((_e1597.x * _e5187.member.x) * _e5203), ((_e1597.y * _e5187.member.y) * _e5203), ((_e1597.z * _e5187.member.z) * _e5203), 0f));
                                                    } else {
                                                        phi_18915_ = vec3<f32>((_e2611.x + (((_e1597.x * _e5187.member.x) * _e4677.x) * _e5203)), (_e2611.y + (((_e1597.y * _e5187.member.y) * _e4677.y) * _e5203)), (_e2611.z + (((_e1597.z * _e5187.member.z) * _e4677.z) * _e5203)));
                                                        phi_18916_ = _e2613;
                                                    }
                                                    let _e5879 = phi_18915_;
                                                    let _e5881 = phi_18916_;
                                                    phi_18917_ = _e5879;
                                                    phi_18918_ = _e5881;
                                                }
                                                let _e5883 = phi_18917_;
                                                let _e5885 = phi_18918_;
                                                phi_35535_ = _e5832;
                                                phi_18919_ = _e5883;
                                                phi_18920_ = _e5885;
                                                phi_18921_ = (_e2617 + 1u);
                                            } else {
                                                phi_35535_ = _e2607;
                                                phi_18919_ = _e2611;
                                                phi_18920_ = _e2613;
                                                phi_18921_ = _e2617;
                                            }
                                            let _e5887 = phi_35535_;
                                            let _e5889 = phi_18919_;
                                            let _e5891 = phi_18920_;
                                            let _e5893 = phi_18921_;
                                            phi_35534_ = _e5887;
                                            phi_18922_ = _e5889;
                                            phi_18923_ = _e5891;
                                            phi_18924_ = _e5893;
                                        } else {
                                            phi_35534_ = _e2607;
                                            phi_18922_ = _e2611;
                                            phi_18923_ = _e2613;
                                            phi_18924_ = _e2617;
                                        }
                                        let _e5895 = phi_35534_;
                                        let _e5897 = phi_18922_;
                                        let _e5899 = phi_18923_;
                                        let _e5901 = phi_18924_;
                                        phi_35824_ = _e4671;
                                        phi_35533_ = _e5895;
                                        phi_14676_ = _e5897;
                                        phi_14679_ = _e5899;
                                        phi_14681_ = (_e2615 + 1u);
                                        phi_14683_ = _e5901;
                                    } else {
                                        phi_35824_ = _e2609;
                                        phi_35533_ = _e2607;
                                        phi_14676_ = vec3<f32>();
                                        phi_14679_ = type_39();
                                        phi_14681_ = u32();
                                        phi_14683_ = u32();
                                    }
                                    let _e5904 = phi_35824_;
                                    let _e5906 = phi_35533_;
                                    let _e5908 = phi_14676_;
                                    let _e5910 = phi_14679_;
                                    let _e5912 = phi_14681_;
                                    let _e5914 = phi_14683_;
                                    phi_35172_ = _e5906;
                                    phi_33247_ = _e5904;
                                    phi_14675_ = _e5908;
                                    phi_14678_ = _e5910;
                                    phi_14680_ = _e5912;
                                    phi_14682_ = _e5914;
                                    phi_35546_ = _e5904;
                                    phi_35532_ = _e5906;
                                    if !(_e2619) {
                                        break;
                                    }
                                }
                                let _e5917 = phi_35546_;
                                let _e5919 = phi_35532_;
                                if _e5919 {
                                    break;
                                }
                                if (_e345 >= _e361) {
                                    let _e6923 = local_102;
                                    let _e6926 = local_103;
                                    let _e6929 = local_104;
                                    let _e6932 = local_105;
                                    let _e6938 = local_106;
                                    phi_19475_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e1597.x, _e1597.y, _e1597.z, f32(_e345)), vec4<f32>(_e6926.x, _e6929.y, _e6932.z, f32(_e6923)), vec4<u32>(_e5917, (_e2602 & 4294967294u), 0u, 0u)), _e1599, _e6938);
                                    phi_19476_ = type_71();
                                    phi_19477_ = true;
                                } else {
                                    let _e5922 = ((_e5917 * 747796405u) + 2891336453u);
                                    let _e5930 = (((_e5922 >> bitcast<u32>((((_e5922 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5922) * 277803737u);
                                    let _e5933 = ((_e5930 >> bitcast<u32>(22i)) ^ _e5930);
                                    let _e5936 = f32((_e5933 >> bitcast<u32>(8i)));
                                    let _e5939 = ((_e5933 * 747796405u) + 2891336453u);
                                    let _e5947 = (((_e5939 >> bitcast<u32>((((_e5939 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5939) * 277803737u);
                                    let _e5950 = ((_e5947 >> bitcast<u32>(22i)) ^ _e5947);
                                    let _e5953 = f32((_e5950 >> bitcast<u32>(8i)));
                                    let _e5955 = ((_e5950 * 747796405u) + 2891336453u);
                                    let _e5963 = (((_e5955 >> bitcast<u32>((((_e5955 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5955) * 277803737u);
                                    let _e5966 = ((_e5963 >> bitcast<u32>(22i)) ^ _e5963);
                                    let _e5971 = (_e2594 == 0u);
                                    if _e5971 {
                                        phi_30275_ = vec3<f32>();
                                        phi_30276_ = true;
                                    } else {
                                        if (_e2594 == 2u) {
                                            let _e5977 = (((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z));
                                            if (_e5977 != _e5977) {
                                                phi_30316_ = true;
                                            } else {
                                                phi_30316_ = (0f >= _e5977);
                                            }
                                            let _e5981 = phi_30316_;
                                            let _e5982 = select(_e5977, 0f, _e5981);
                                            let _e5983 = (1f / _e2584);
                                            let _e5987 = ((_e5983 * _e5983) * (1f - (_e5982 * _e5982)));
                                            if (_e5987 >= 1f) {
                                                phi_30335_ = 1f;
                                            } else {
                                                let _e5990 = sqrt((1f - _e5987));
                                                if (_e5982 != _e5982) {
                                                    phi_30346_ = true;
                                                } else {
                                                    phi_30346_ = (_e5990 <= _e5982);
                                                }
                                                let _e5994 = phi_30346_;
                                                let _e5999 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                phi_30335_ = (_e5999 + ((1f - _e5999) * pow((1f - select(_e5982, _e5990, _e5994)), 5f)));
                                            }
                                            let _e6006 = phi_30335_;
                                            phi_30266_ = select(true, false, ((f32((_e5966 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6006));
                                        } else {
                                            phi_30266_ = true;
                                        }
                                        let _e6010 = phi_30266_;
                                        if _e6010 {
                                            let _e6015 = (((_e1674 * _e1750) + (_e1675 * _e1751)) + (_e1676 * _e1753));
                                            let _e6020 = (((_e1674 * _e1746) + (_e1675 * _e1756)) + (_e1676 * _e1757));
                                            let _e6025 = (((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z));
                                            if (_e6025 <= 0f) {
                                                phi_30500_ = vec3<f32>();
                                                phi_30501_ = true;
                                            } else {
                                                let _e6028 = (_e2592 * _e6015);
                                                let _e6029 = (_e2592 * _e6020);
                                                let _e6036 = (1f / sqrt((((_e6028 * _e6028) + (_e6029 * _e6029)) + (_e6025 * _e6025))));
                                                let _e6037 = (_e6028 * _e6036);
                                                let _e6038 = (_e6029 * _e6036);
                                                let _e6039 = (_e6025 * _e6036);
                                                let _e6042 = ((_e6037 * _e6037) + (_e6038 * _e6038));
                                                if (_e6042 > 0f) {
                                                    let _e6045 = sqrt(_e6042);
                                                    phi_30402_ = vec3<f32>((-(_e6038) / _e6045), (_e6037 / _e6045), 0f);
                                                } else {
                                                    phi_30402_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e6050 = phi_30402_;
                                                let _e6063 = sqrt((_e5936 * 0.000000059604645f));
                                                let _e6064 = (_e5953 * 0.00000037450704f);
                                                let _e6066 = (_e6063 * cos(_e6064));
                                                let _e6068 = (0.5f * (1f + _e6039));
                                                let _e6071 = (1f - (_e6066 * _e6066));
                                                if (_e6071 != _e6071) {
                                                    phi_30558_ = true;
                                                } else {
                                                    phi_30558_ = (0f >= _e6071);
                                                }
                                                let _e6075 = phi_30558_;
                                                let _e6082 = (((1f - _e6068) * sqrt(select(_e6071, 0f, _e6075))) + ((_e6068 * _e6063) * sin(_e6064)));
                                                let _e6093 = (_e6071 - (_e6082 * _e6082));
                                                if (_e6093 != _e6093) {
                                                    phi_30573_ = true;
                                                } else {
                                                    phi_30573_ = (0f >= _e6093);
                                                }
                                                let _e6097 = phi_30573_;
                                                let _e6099 = sqrt(select(_e6093, 0f, _e6097));
                                                let _e6105 = (((_e6050.z * _e6066) + (((_e6037 * _e6050.y) - (_e6050.x * _e6038)) * _e6082)) + (_e6039 * _e6099));
                                                let _e6106 = (_e2592 * (((_e6050.x * _e6066) + (((_e6038 * _e6050.z) - (_e6050.y * _e6039)) * _e6082)) + (_e6037 * _e6099)));
                                                let _e6107 = (_e2592 * (((_e6050.y * _e6066) + (((_e6039 * _e6050.x) - (_e6050.z * _e6037)) * _e6082)) + (_e6038 * _e6099)));
                                                if (_e6105 != _e6105) {
                                                    phi_30588_ = true;
                                                } else {
                                                    phi_30588_ = (0f >= _e6105);
                                                }
                                                let _e6111 = phi_30588_;
                                                let _e6112 = select(_e6105, 0f, _e6111);
                                                let _e6119 = (1f / sqrt((((_e6106 * _e6106) + (_e6107 * _e6107)) + (_e6112 * _e6112))));
                                                let _e6120 = (_e6106 * _e6119);
                                                let _e6121 = (_e6107 * _e6119);
                                                let _e6122 = (_e6112 * _e6119);
                                                let _e6128 = (2f * (((_e6015 * _e6120) + (_e6020 * _e6121)) + (_e6025 * _e6122)));
                                                let _e6131 = (_e6122 * _e6128);
                                                let _e6134 = (vec3<f32>((_e6120 * _e6128), (_e6121 * _e6128), _e6131) - vec3<f32>(_e6015, _e6020, _e6025));
                                                let _e6135 = ((_e6131 - _e6025) <= 0f);
                                                if _e6135 {
                                                    phi_30497_ = vec3<f32>();
                                                } else {
                                                    phi_30497_ = vec3<f32>((((_e1750 * _e6134.x) + (_e1746 * _e6134.y)) + (_e1702.x * _e6134.z)), (((_e1751 * _e6134.x) + (_e1756 * _e6134.y)) + (_e1702.y * _e6134.z)), (((_e1753 * _e6134.x) + (_e1757 * _e6134.y)) + (_e1702.z * _e6134.z)));
                                                }
                                                let _e6156 = phi_30497_;
                                                phi_30500_ = _e6156;
                                                phi_30501_ = _e6135;
                                            }
                                            let _e6158 = phi_30500_;
                                            let _e6160 = phi_30501_;
                                            phi_30271_ = select(_e6158, vec3<f32>(0f, 0f, 0f), vec3(_e6160));
                                        } else {
                                            phi_30271_ = vec3<f32>();
                                        }
                                        let _e6164 = phi_30271_;
                                        phi_30275_ = _e6164;
                                        phi_30276_ = select(true, false, _e6010);
                                    }
                                    let _e6167 = phi_30275_;
                                    let _e6169 = phi_30276_;
                                    if _e6169 {
                                        let _e6170 = (_e5936 * 0.00000037450704f);
                                        let _e6172 = ((_e5953 * 0.00000011920929f) - 1f);
                                        let _e6174 = (1f - (_e6172 * _e6172));
                                        if (_e6174 != _e6174) {
                                            phi_30688_ = true;
                                        } else {
                                            phi_30688_ = (0f >= _e6174);
                                        }
                                        let _e6178 = phi_30688_;
                                        let _e6180 = sqrt(select(_e6174, 0f, _e6178));
                                        let _e6185 = (_e1702.x + (_e6180 * cos(_e6170)));
                                        let _e6186 = (_e1702.y + (_e6180 * sin(_e6170)));
                                        let _e6187 = (_e1702.z + _e6172);
                                        let _e6193 = sqrt((((_e6185 * _e6185) + (_e6186 * _e6186)) + (_e6187 * _e6187)));
                                        if (_e6193 < 0.000001f) {
                                            phi_30662_ = _e1702;
                                        } else {
                                            let _e6195 = (1f / _e6193);
                                            phi_30662_ = vec3<f32>((_e6185 * _e6195), (_e6186 * _e6195), (_e6187 * _e6195));
                                        }
                                        let _e6201 = phi_30662_;
                                        phi_30283_ = _e6201;
                                    } else {
                                        phi_30283_ = _e6167;
                                    }
                                    let _e6203 = phi_30283_;
                                    let _e6211 = (((_e6203.x * _e1702.x) + (_e6203.y * _e1702.y)) + (_e6203.z * _e1702.z));
                                    if (_e6211 != _e6211) {
                                        phi_30703_ = true;
                                    } else {
                                        phi_30703_ = (0f >= _e6211);
                                    }
                                    let _e6215 = phi_30703_;
                                    let _e6217 = (select(_e6211, 0f, _e6215) * 0.31830987f);
                                    if _e5971 {
                                        if (_e6211 <= 0f) {
                                            phi_31500_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_31500_ = (_e2588 * (_e6211 * 0.31830987f));
                                        }
                                        let _e6692 = phi_31500_;
                                        phi_19316_ = type_56(_e6692, _e6217);
                                        phi_19317_ = type_56();
                                        phi_19318_ = true;
                                    } else {
                                        let _e6222 = (((_e1674 * _e1750) + (_e1675 * _e1751)) + (_e1676 * _e1753));
                                        let _e6227 = (((_e1674 * _e1746) + (_e1675 * _e1756)) + (_e1676 * _e1757));
                                        let _e6232 = (((_e1674 * _e1702.x) + (_e1675 * _e1702.y)) + (_e1676 * _e1702.z));
                                        let _e6237 = (((_e6203.x * _e1750) + (_e6203.y * _e1751)) + (_e6203.z * _e1753));
                                        let _e6242 = (((_e6203.x * _e1746) + (_e6203.y * _e1756)) + (_e6203.z * _e1757));
                                        if (_e6232 <= 0f) {
                                            phi_19280_ = type_56();
                                            phi_19281_ = true;
                                        } else {
                                            let _e6244 = (_e6211 <= 0f);
                                            if _e6244 {
                                                phi_19279_ = type_56();
                                            } else {
                                                let _e6245 = (_e6222 + _e6237);
                                                let _e6246 = (_e6227 + _e6242);
                                                let _e6247 = (_e6232 + _e6211);
                                                let _e6254 = (1f / sqrt((((_e6245 * _e6245) + (_e6246 * _e6246)) + (_e6247 * _e6247))));
                                                let _e6257 = (_e6247 * _e6254);
                                                if (_e6257 <= 0f) {
                                                    phi_30813_ = 0f;
                                                } else {
                                                    let _e6259 = (_e2592 * _e2592);
                                                    let _e6263 = ((((_e6259 - 1f) * _e6257) * _e6257) + 1f);
                                                    phi_30813_ = (_e6259 / ((3.1415927f * _e6263) * _e6263));
                                                }
                                                let _e6268 = phi_30813_;
                                                let _e6269 = (_e6232 * _e6232);
                                                if (_e6269 >= 1f) {
                                                    phi_30841_ = 0f;
                                                } else {
                                                    phi_30841_ = ((sqrt((1f + (((_e2592 * _e2592) * ((_e6222 * _e6222) + (_e6227 * _e6227))) / _e6269))) - 1f) * 0.5f);
                                                }
                                                let _e6282 = phi_30841_;
                                                let _e6283 = (1f + _e6282);
                                                let _e6284 = (_e6211 * _e6211);
                                                if (_e6284 >= 1f) {
                                                    phi_30869_ = 0f;
                                                } else {
                                                    phi_30869_ = ((sqrt((1f + (((_e2592 * _e2592) * ((_e6237 * _e6237) + (_e6242 * _e6242))) / _e6284))) - 1f) * 0.5f);
                                                }
                                                let _e6297 = phi_30869_;
                                                let _e6304 = (((_e6222 * (_e6245 * _e6254)) + (_e6227 * (_e6246 * _e6254))) + (_e6232 * _e6257));
                                                if (_e2590 == 0u) {
                                                    let _e6599 = (1f - _e6304);
                                                    if (_e6599 != _e6599) {
                                                        phi_31372_ = true;
                                                    } else {
                                                        phi_31372_ = (0f >= _e6599);
                                                    }
                                                    let _e6603 = phi_31372_;
                                                    let _e6605 = pow(select(_e6599, 0f, _e6603), 5f);
                                                    let _e6606 = (1f - _e6605);
                                                    phi_19267_ = vec3<f32>(((_e2588.x * _e6606) + _e6605), ((_e2588.y * _e6606) + _e6605), ((_e2588.z * _e6606) + _e6605));
                                                } else {
                                                    if (_e2590 == 1u) {
                                                        let _e6572 = (1f / _e2584);
                                                        let _e6576 = ((_e6572 * _e6572) * (1f - (_e6304 * _e6304)));
                                                        if (_e6576 >= 1f) {
                                                            phi_31333_ = 1f;
                                                        } else {
                                                            let _e6579 = sqrt((1f - _e6576));
                                                            if (_e6304 != _e6304) {
                                                                phi_31344_ = true;
                                                            } else {
                                                                phi_31344_ = (_e6579 <= _e6304);
                                                            }
                                                            let _e6583 = phi_31344_;
                                                            let _e6588 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                            phi_31333_ = (_e6588 + ((1f - _e6588) * pow((1f - select(_e6304, _e6579, _e6583)), 5f)));
                                                        }
                                                        let _e6595 = phi_31333_;
                                                        phi_19266_ = vec3(_e6595);
                                                    } else {
                                                        let _e6308 = select(_e6304, 0f, (_e6304 < 0f));
                                                        let _e6310 = select(_e6308, 1f, (_e6308 > 1f));
                                                        let _e6312 = (1f - (_e6310 * _e6310));
                                                        let _e6325 = ((_e2588.x * _e2588.x) - (_e2586.x * _e2586.x));
                                                        let _e6326 = ((_e2588.y * _e2588.y) - (_e2586.y * _e2586.y));
                                                        let _e6327 = ((_e2588.z * _e2588.z) - (_e2586.z * _e2586.z));
                                                        let _e6329 = (_e2588.x * (_e2586.x + _e2586.x));
                                                        let _e6331 = (_e2588.y * (_e2586.y + _e2586.y));
                                                        let _e6333 = (_e2588.z * (_e2586.z + _e2586.z));
                                                        let _e6340 = ((_e6325 * _e6325) + (_e6329 * _e6329));
                                                        let _e6341 = ((_e6326 * _e6326) + (_e6331 * _e6331));
                                                        let _e6342 = ((_e6327 * _e6327) + (_e6333 * _e6333));
                                                        let _e6347 = ((_e6312 * _e6325) / _e6340);
                                                        let _e6348 = ((_e6312 * _e6326) / _e6341);
                                                        let _e6349 = ((_e6312 * _e6327) / _e6342);
                                                        let _e6350 = (1f - _e6347);
                                                        let _e6351 = (1f - _e6348);
                                                        let _e6352 = (1f - _e6349);
                                                        let _e6354 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e6347, _e6348, _e6349));
                                                        let _e6355 = (_e6312 * _e6329);
                                                        let _e6356 = (_e6312 * _e6331);
                                                        let _e6357 = (_e6312 * _e6333);
                                                        let _e6362 = (vec3<f32>(_e6355, _e6356, _e6357) / vec3<f32>(_e6340, _e6341, _e6342));
                                                        let _e6375 = ((_e6354.x * _e6354.x) + (_e6362.x * _e6362.x));
                                                        let _e6376 = ((_e6354.y * _e6354.y) + (_e6362.y * _e6362.y));
                                                        let _e6377 = ((_e6354.z * _e6354.z) + (_e6362.z * _e6362.z));
                                                        if (_e6375 != _e6375) {
                                                            phi_31003_ = true;
                                                        } else {
                                                            phi_31003_ = (0f >= _e6375);
                                                        }
                                                        let _e6381 = phi_31003_;
                                                        let _e6383 = sqrt(select(_e6375, 0f, _e6381));
                                                        if (_e6376 != _e6376) {
                                                            phi_31018_ = true;
                                                        } else {
                                                            phi_31018_ = (0f >= _e6376);
                                                        }
                                                        let _e6387 = phi_31018_;
                                                        let _e6389 = sqrt(select(_e6376, 0f, _e6387));
                                                        if (_e6377 != _e6377) {
                                                            phi_31033_ = true;
                                                        } else {
                                                            phi_31033_ = (0f >= _e6377);
                                                        }
                                                        let _e6393 = phi_31033_;
                                                        let _e6395 = sqrt(select(_e6377, 0f, _e6393));
                                                        let _e6399 = ((_e6383 + _e6350) * 0.5f);
                                                        let _e6400 = ((_e6389 + _e6351) * 0.5f);
                                                        let _e6401 = ((_e6395 + _e6352) * 0.5f);
                                                        if (_e6399 != _e6399) {
                                                            phi_31060_ = true;
                                                        } else {
                                                            phi_31060_ = (0f >= _e6399);
                                                        }
                                                        let _e6405 = phi_31060_;
                                                        let _e6407 = sqrt(select(_e6399, 0f, _e6405));
                                                        if (_e6400 != _e6400) {
                                                            phi_31075_ = true;
                                                        } else {
                                                            phi_31075_ = (0f >= _e6400);
                                                        }
                                                        let _e6411 = phi_31075_;
                                                        let _e6413 = sqrt(select(_e6400, 0f, _e6411));
                                                        if (_e6401 != _e6401) {
                                                            phi_31090_ = true;
                                                        } else {
                                                            phi_31090_ = (0f >= _e6401);
                                                        }
                                                        let _e6417 = phi_31090_;
                                                        let _e6419 = sqrt(select(_e6401, 0f, _e6417));
                                                        let _e6420 = vec3<f32>(_e6407, _e6413, _e6419);
                                                        let _e6424 = ((_e6383 - _e6350) * 0.5f);
                                                        let _e6425 = ((_e6389 - _e6351) * 0.5f);
                                                        let _e6426 = ((_e6395 - _e6352) * 0.5f);
                                                        if (_e6424 != _e6424) {
                                                            phi_31117_ = true;
                                                        } else {
                                                            phi_31117_ = (0f >= _e6424);
                                                        }
                                                        let _e6430 = phi_31117_;
                                                        let _e6432 = sqrt(select(_e6424, 0f, _e6430));
                                                        if (_e6425 != _e6425) {
                                                            phi_31132_ = true;
                                                        } else {
                                                            phi_31132_ = (0f >= _e6425);
                                                        }
                                                        let _e6436 = phi_31132_;
                                                        let _e6438 = sqrt(select(_e6425, 0f, _e6436));
                                                        if (_e6426 != _e6426) {
                                                            phi_31147_ = true;
                                                        } else {
                                                            phi_31147_ = (0f >= _e6426);
                                                        }
                                                        let _e6442 = phi_31147_;
                                                        let _e6444 = sqrt(select(_e6426, 0f, _e6442));
                                                        if ((_e6355 / _e6340) < 0f) {
                                                            phi_19143_ = -(_e6432);
                                                        } else {
                                                            phi_19143_ = _e6432;
                                                        }
                                                        let _e6448 = phi_19143_;
                                                        if ((_e6356 / _e6341) < 0f) {
                                                            phi_19151_ = -(_e6438);
                                                        } else {
                                                            phi_19151_ = _e6438;
                                                        }
                                                        let _e6452 = phi_19151_;
                                                        if ((_e6357 / _e6342) < 0f) {
                                                            phi_19159_ = -(_e6444);
                                                        } else {
                                                            phi_19159_ = _e6444;
                                                        }
                                                        let _e6456 = phi_19159_;
                                                        let _e6460 = (_e2586.x * _e6310);
                                                        let _e6461 = (_e2586.y * _e6310);
                                                        let _e6462 = (_e2586.z * _e6310);
                                                        let _e6472 = vec3<f32>(((_e2588.x * _e6407) - (_e2586.x * _e6448)), ((_e2588.y * _e6413) - (_e2586.y * _e6452)), ((_e2588.z * _e6419) - (_e2586.z * _e6456)));
                                                        let _e6479 = ((_e2588.x * _e6448) + (_e2586.x * _e6407));
                                                        let _e6480 = ((_e2588.y * _e6452) + (_e2586.y * _e6413));
                                                        let _e6481 = ((_e2588.z * _e6456) + (_e2586.z * _e6419));
                                                        let _e6483 = vec3<f32>((_e2588.x * _e6310), (_e2588.y * _e6310), (_e2588.z * _e6310));
                                                        let _e6484 = (_e6483 - _e6420);
                                                        let _e6485 = (_e6460 - _e6448);
                                                        let _e6486 = (_e6461 - _e6452);
                                                        let _e6487 = (_e6462 - _e6456);
                                                        let _e6500 = (_e6483 + _e6420);
                                                        let _e6501 = (_e6460 + _e6448);
                                                        let _e6502 = (_e6461 + _e6452);
                                                        let _e6503 = (_e6462 + _e6456);
                                                        let _e6513 = ((_e6500.x * _e6500.x) + (_e6501 * _e6501));
                                                        let _e6514 = ((_e6500.y * _e6500.y) + (_e6502 * _e6502));
                                                        let _e6515 = ((_e6500.z * _e6500.z) + (_e6503 * _e6503));
                                                        let _e6525 = vec3(_e6310);
                                                        let _e6526 = (_e6525 - _e6472);
                                                        let _e6527 = -(vec3<f32>(_e6479, _e6480, _e6481));
                                                        let _e6543 = (_e6525 + _e6472);
                                                        let _e6553 = ((_e6543.x * _e6543.x) + (_e6479 * _e6479));
                                                        let _e6554 = ((_e6543.y * _e6543.y) + (_e6480 * _e6480));
                                                        let _e6555 = ((_e6543.z * _e6543.z) + (_e6481 * _e6481));
                                                        phi_19266_ = vec3<f32>((((((_e6484.x * _e6484.x) + (_e6485 * _e6485)) / select(0.000000000000000000000000000001f, _e6513, (_e6513 > 0.000000000000000000000000000001f))) + (((_e6526.x * _e6526.x) + (_e6527.x * _e6527.x)) / select(0.000000000000000000000000000001f, _e6553, (_e6553 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6484.y * _e6484.y) + (_e6486 * _e6486)) / select(0.000000000000000000000000000001f, _e6514, (_e6514 > 0.000000000000000000000000000001f))) + (((_e6526.y * _e6526.y) + (_e6527.y * _e6527.y)) / select(0.000000000000000000000000000001f, _e6554, (_e6554 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6484.z * _e6484.z) + (_e6487 * _e6487)) / select(0.000000000000000000000000000001f, _e6515, (_e6515 > 0.000000000000000000000000000001f))) + (((_e6526.z * _e6526.z) + (_e6527.z * _e6527.z)) / select(0.000000000000000000000000000001f, _e6555, (_e6555 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6598 = phi_19266_;
                                                    phi_19267_ = _e6598;
                                                }
                                                let _e6618 = phi_19267_;
                                                let _e6620 = (4f * _e6232);
                                                phi_19279_ = type_56((_e6618 * (((1f / (_e6283 + _e6297)) * _e6268) / _e6620)), ((_e6268 / _e6283) / _e6620));
                                            }
                                            let _e6627 = phi_19279_;
                                            phi_19280_ = _e6627;
                                            phi_19281_ = _e6244;
                                        }
                                        let _e6629 = phi_19280_;
                                        let _e6631 = phi_19281_;
                                        if _e6631 {
                                            phi_19286_ = type_56(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19286_ = _e6629;
                                        }
                                        let _e6633 = phi_19286_;
                                        let _e6636 = (_e2594 == 1u);
                                        if _e6636 {
                                            phi_19314_ = _e6633;
                                            phi_19315_ = type_56();
                                        } else {
                                            if (_e6232 != _e6232) {
                                                phi_31409_ = true;
                                            } else {
                                                phi_31409_ = (0f >= _e6232);
                                            }
                                            let _e6640 = phi_31409_;
                                            let _e6641 = select(_e6232, 0f, _e6640);
                                            let _e6642 = (1f / _e2584);
                                            let _e6646 = ((_e6642 * _e6642) * (1f - (_e6641 * _e6641)));
                                            if (_e6646 >= 1f) {
                                                phi_31428_ = 1f;
                                            } else {
                                                let _e6649 = sqrt((1f - _e6646));
                                                if (_e6641 != _e6641) {
                                                    phi_31439_ = true;
                                                } else {
                                                    phi_31439_ = (_e6649 <= _e6641);
                                                }
                                                let _e6653 = phi_31439_;
                                                let _e6658 = pow(((1f - _e2584) / (1f + _e2584)), 2f);
                                                phi_31428_ = (_e6658 + ((1f - _e6658) * pow((1f - select(_e6641, _e6649, _e6653)), 5f)));
                                            }
                                            let _e6665 = phi_31428_;
                                            if (_e6211 <= 0f) {
                                                phi_31478_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_31478_ = (_e2582 * (_e6211 * 0.31830987f));
                                            }
                                            let _e6670 = phi_31478_;
                                            let _e6671 = (1f - _e6665);
                                            phi_19314_ = type_56();
                                            phi_19315_ = type_56((_e6633.member + vec3<f32>((_e6670.x * _e6671), (_e6670.y * _e6671), (_e6670.z * _e6671))), ((_e6665 * _e6633.member_1) + (_e6671 * _e6217)));
                                        }
                                        let _e6685 = phi_19314_;
                                        let _e6687 = phi_19315_;
                                        phi_19316_ = _e6685;
                                        phi_19317_ = _e6687;
                                        phi_19318_ = _e6636;
                                    }
                                    let _e6695 = phi_19316_;
                                    let _e6697 = phi_19317_;
                                    let _e6699 = phi_19318_;
                                    if _e6699 {
                                        phi_19322_ = _e6695;
                                    } else {
                                        phi_19322_ = _e6697;
                                    }
                                    let _e6701 = phi_19322_;
                                    if ((((_e6203.x * _e6203.x) + (_e6203.y * _e6203.y)) + (_e6203.z * _e6203.z)) == 0f) {
                                        phi_37415_ = _e5966;
                                        phi_19461_ = type_71();
                                        phi_19462_ = type_71();
                                        phi_19463_ = true;
                                        phi_19464_ = false;
                                    } else {
                                        let _e6710 = (_e6701.member_1 <= 0f);
                                        if _e6710 {
                                            phi_37416_ = _e5966;
                                            phi_19458_ = type_71();
                                            phi_19459_ = type_71();
                                            phi_19460_ = false;
                                        } else {
                                            let _e6712 = (_e6701.member.x / _e6701.member_1);
                                            let _e6714 = (_e6701.member.y / _e6701.member_1);
                                            let _e6716 = (_e6701.member.z / _e6701.member_1);
                                            let _e6725 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6712 * _e6712) + (_e6714 * _e6714)) + (_e6716 * _e6716))) <= _e6725) {
                                                phi_37409_ = _e5966;
                                                phi_19409_ = _e1597;
                                                phi_19410_ = false;
                                            } else {
                                                let _e6729 = global_1.member.member_7[1u];
                                                if (_e6729 == 1u) {
                                                    let _e6734 = global_1.member.member_4[3u];
                                                    if (f32(_e345) >= _e6734) {
                                                        let _e6737 = select(_e6714, _e6712, (_e6712 > _e6714));
                                                        let _e6739 = select(_e6716, _e6737, (_e6737 > _e6716));
                                                        if (_e6739 != _e6739) {
                                                            phi_31525_ = true;
                                                        } else {
                                                            phi_31525_ = (1f <= _e6739);
                                                        }
                                                        let _e6743 = phi_31525_;
                                                        let _e6744 = select(_e6739, 1f, _e6743);
                                                        let _e6746 = ((_e5966 * 747796405u) + 2891336453u);
                                                        let _e6754 = (((_e6746 >> bitcast<u32>((((_e6746 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6746) * 277803737u);
                                                        let _e6757 = ((_e6754 >> bitcast<u32>(22i)) ^ _e6754);
                                                        let _e6762 = ((f32((_e6757 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6744);
                                                        if _e6762 {
                                                            phi_19398_ = vec3<f32>();
                                                        } else {
                                                            phi_19398_ = vec3<f32>((_e1597.x / _e6744), (_e1597.y / _e6744), (_e1597.z / _e6744));
                                                        }
                                                        let _e6771 = phi_19398_;
                                                        phi_37412_ = _e6757;
                                                        phi_19400_ = _e6771;
                                                        phi_19401_ = select(true, false, _e6762);
                                                    } else {
                                                        phi_37412_ = _e5966;
                                                        phi_19400_ = _e1597;
                                                        phi_19401_ = true;
                                                    }
                                                    let _e6774 = phi_37412_;
                                                    let _e6776 = phi_19400_;
                                                    let _e6778 = phi_19401_;
                                                    phi_37411_ = _e6774;
                                                    phi_19402_ = _e6776;
                                                    phi_19403_ = _e6778;
                                                } else {
                                                    phi_37411_ = _e5966;
                                                    phi_19402_ = _e1597;
                                                    phi_19403_ = true;
                                                }
                                                let _e6780 = phi_37411_;
                                                let _e6782 = phi_19402_;
                                                let _e6784 = phi_19403_;
                                                phi_37409_ = _e6780;
                                                phi_19409_ = select(_e1597, _e6782, vec3(_e6784));
                                                phi_19410_ = select(false, true, _e6784);
                                            }
                                            let _e6789 = phi_37409_;
                                            let _e6791 = phi_19409_;
                                            let _e6793 = phi_19410_;
                                            if _e6793 {
                                                if ((((_e6203.x * _e1689.x) + (_e6203.y * _e1689.y)) + (_e6203.z * _e1689.z)) < 0f) {
                                                    phi_19439_ = -(_e1689);
                                                } else {
                                                    phi_19439_ = _e1689;
                                                }
                                                let _e6832 = phi_19439_;
                                                let _e6849 = local_92;
                                                let _e6852 = local_93;
                                                let _e6855 = local_94;
                                                let _e6858 = local_95;
                                                let _e6864 = local_96;
                                                phi_19455_ = type_71(type_35(vec4<f32>((_e1604 + (_e6832.x * _e1720)), (_e1605 + (_e6832.y * _e1720)), (_e1606 + (_e6832.z * _e1720)), _e318.member.w), vec4<f32>(_e6203.x, _e6203.y, _e6203.z, _e6701.member_1), vec4<f32>((_e6791.x * _e6712), (_e6791.y * _e6714), (_e6791.z * _e6716), f32((_e345 + 1u))), vec4<f32>(_e6852.x, _e6855.y, _e6858.z, f32(_e6849)), vec4<u32>(_e6789, _e2602, 0u, 0u)), _e1599, _e6864);
                                                phi_19456_ = type_71();
                                            } else {
                                                let _e6801 = local_87;
                                                let _e6804 = local_88;
                                                let _e6807 = local_89;
                                                let _e6810 = local_90;
                                                let _e6816 = local_91;
                                                phi_19455_ = type_71();
                                                phi_19456_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e6791.x, _e6791.y, _e6791.z, f32(_e345)), vec4<f32>(_e6804.x, _e6807.y, _e6810.z, f32(_e6801)), vec4<u32>(_e6789, (_e2602 & 4294967294u), 0u, 0u)), _e1599, _e6816);
                                            }
                                            let _e6867 = phi_19455_;
                                            let _e6869 = phi_19456_;
                                            phi_37416_ = _e6789;
                                            phi_19458_ = _e6867;
                                            phi_19459_ = _e6869;
                                            phi_19460_ = select(true, false, _e6793);
                                        }
                                        let _e6872 = phi_37416_;
                                        let _e6874 = phi_19458_;
                                        let _e6876 = phi_19459_;
                                        let _e6878 = phi_19460_;
                                        phi_37415_ = _e6872;
                                        phi_19461_ = _e6874;
                                        phi_19462_ = _e6876;
                                        phi_19463_ = _e6710;
                                        phi_19464_ = _e6878;
                                    }
                                    let _e6880 = phi_37415_;
                                    let _e6882 = phi_19461_;
                                    let _e6884 = phi_19462_;
                                    let _e6886 = phi_19463_;
                                    let _e6888 = phi_19464_;
                                    if _e6886 {
                                        let _e6896 = local_97;
                                        let _e6899 = local_98;
                                        let _e6902 = local_99;
                                        let _e6905 = local_100;
                                        let _e6911 = local_101;
                                        phi_19470_ = type_71(type_35(_e318.member, _e318.member_1, vec4<f32>(_e1597.x, _e1597.y, _e1597.z, f32(_e345)), vec4<f32>(_e6899.x, _e6902.y, _e6905.z, f32(_e6896)), vec4<u32>(_e6880, (_e2602 & 4294967294u), 0u, 0u)), _e1599, _e6911);
                                    } else {
                                        phi_19470_ = _e6884;
                                    }
                                    let _e6914 = phi_19470_;
                                    phi_19475_ = _e6914;
                                    phi_19476_ = _e6882;
                                    phi_19477_ = select(_e6888, true, _e6886);
                                }
                                let _e6941 = phi_19475_;
                                let _e6943 = phi_19476_;
                                let _e6945 = phi_19477_;
                                phi_19481_ = _e6941;
                                phi_19482_ = _e6943;
                                phi_19483_ = _e6945;
                            } else {
                                phi_19481_ = _e2598;
                                phi_19482_ = type_71();
                                phi_19483_ = true;
                            }
                            let _e6947 = phi_19481_;
                            let _e6949 = phi_19482_;
                            let _e6951 = phi_19483_;
                            phi_19487_ = _e6947;
                            phi_19488_ = _e6949;
                            phi_19489_ = _e6951;
                        }
                        let _e7163 = phi_19487_;
                        let _e7165 = phi_19488_;
                        let _e7167 = phi_19489_;
                        phi_19493_ = _e7163;
                        phi_19494_ = _e7165;
                        phi_19495_ = _e7167;
                    }
                    let _e7675 = phi_19493_;
                    let _e7677 = phi_19494_;
                    let _e7679 = phi_19495_;
                    if _e7679 {
                        phi_19499_ = _e7675;
                    } else {
                        phi_19499_ = _e7677;
                    }
                    let _e7681 = phi_19499_;
                    phi_19503_ = vec4<f32>(0f, 0f, 0f, 0f);
                    phi_19504_ = _e7681.member_2;
                    phi_19505_ = _e7681.member_1;
                    phi_19506_ = _e7681.member;
                }
                let _e7731 = phi_19503_;
                let _e7733 = phi_19504_;
                let _e7735 = phi_19505_;
                let _e7737 = phi_19506_;
                if _e315 {
                } else {
                    break;
                }
                global_7.member[_e310.member_1] = _e7737;
                let _e7738 = (2u * _e310.member_1);
                if (_e7738 < _e290) {
                } else {
                    break;
                }
                global_8.member[_e7738] = _e7735;
                let _e7742 = (_e7738 + 1u);
                if (_e7742 < _e290) {
                } else {
                    break;
                }
                global_8.member[_e7742] = _e7733;
                if (_e7731.w > 0f) {
                    if (_e310.member_1 < arrayLength((&global_9.member))) {
                    } else {
                        break;
                    }
                    let _e7753 = global_9.member[_e310.member_1][0u];
                    global_9.member[_e310.member_1][0u] = (_e7753 + _e7731.x);
                    let _e7759 = global_9.member[_e310.member_1][1u];
                    global_9.member[_e310.member_1][1u] = (_e7759 + _e7731.y);
                    let _e7765 = global_9.member[_e310.member_1][2u];
                    global_9.member[_e310.member_1][2u] = (_e7765 + _e7731.z);
                    let _e7770 = global_9.member[_e310.member_1][3u];
                    global_9.member[_e310.member_1][3u] = (_e7770 + _e7731.w);
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
