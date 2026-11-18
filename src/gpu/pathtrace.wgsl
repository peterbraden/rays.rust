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
    var phi_492_: type_48;
    var phi_33555_: bool;
    var phi_512_: vec3<f32>;
    var phi_515_: u32;
    var phi_21789_: f32;
    var phi_21801_: type_56;
    var phi_21812_: type_57;
    var phi_21813_: type_57;
    var phi_21817_: type_57;
    var phi_21818_: type_57;
    var phi_21819_: bool;
    var phi_21823_: type_57;
    var phi_21710_: type_57;
    var phi_21861_: bool;
    var phi_21941_: f32;
    var phi_21953_: type_56;
    var phi_21964_: type_57;
    var phi_21965_: type_57;
    var phi_21969_: type_57;
    var phi_21970_: type_57;
    var phi_21971_: bool;
    var phi_21975_: type_57;
    var phi_21909_: f32;
    var phi_22013_: bool;
    var phi_21738_: f32;
    var phi_21740_: f32;
    var phi_21749_: type_57;
    var phi_21753_: type_57;
    var phi_21757_: type_57;
    var phi_22028_: bool;
    var phi_9515_: vec3<f32>;
    var phi_9518_: vec3<f32>;
    var phi_9520_: vec3<f32>;
    var phi_9522_: vec3<f32>;
    var phi_9524_: u32;
    var phi_9608_: u32;
    var phi_9623_: type_56;
    var phi_9624_: type_56;
    var phi_9625_: type_56;
    var phi_22228_: bool;
    var phi_22355_: bool;
    var phi_22370_: bool;
    var phi_9953_: vec3<f32>;
    var phi_10026_: vec3<f32>;
    var phi_10027_: vec3<f32>;
    var phi_10028_: bool;
    var phi_10033_: vec3<f32>;
    var phi_10034_: bool;
    var phi_22494_: bool;
    var phi_22509_: bool;
    var phi_10325_: vec3<f32>;
    var phi_9516_: vec3<f32>;
    var phi_9519_: vec3<f32>;
    var phi_9521_: vec3<f32>;
    var phi_9523_: vec3<f32>;
    var phi_9525_: u32;
    var phi_33538_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_34022_: bool;
    var phi_10382_: type_60;
    var phi_34021_: bool;
    var phi_10384_: type_60;
    var phi_10385_: bool;
    var phi_10390_: type_60;
    var phi_891_: vec3<f32>;
    var phi_893_: vec3<f32>;
    var phi_894_: bool;
    var phi_33630_: bool;
    var phi_904_: vec3<f32>;
    var phi_907_: i32;
    var phi_909_: u32;
    var phi_22778_: bool;
    var phi_1331_: u32;
    var phi_1361_: u32;
    var phi_1332_: u32;
    var phi_1362_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1380_: u32;
    var phi_2685_: type_55;
    var phi_2693_: type_52;
    var phi_2695_: type_52;
    var phi_2696_: bool;
    var phi_2697_: type_52;
    var phi_2698_: bool;
    var phi_2699_: bool;
    var phi_22889_: f32;
    var phi_22901_: type_56;
    var phi_22912_: type_57;
    var phi_22913_: type_57;
    var phi_22917_: type_57;
    var phi_22918_: type_57;
    var phi_22919_: bool;
    var phi_22923_: type_57;
    var phi_22810_: type_57;
    var phi_22961_: bool;
    var phi_23041_: f32;
    var phi_23053_: type_56;
    var phi_23064_: type_57;
    var phi_23065_: type_57;
    var phi_23069_: type_57;
    var phi_23070_: type_57;
    var phi_23071_: bool;
    var phi_23075_: type_57;
    var phi_23009_: f32;
    var phi_23113_: bool;
    var phi_22838_: f32;
    var phi_22840_: f32;
    var phi_22849_: type_57;
    var phi_22853_: type_57;
    var phi_22857_: type_57;
    var phi_2717_: vec3<f32>;
    var phi_2720_: vec3<f32>;
    var phi_2722_: vec3<f32>;
    var phi_2724_: vec3<f32>;
    var phi_2726_: u32;
    var phi_2810_: u32;
    var phi_2825_: type_56;
    var phi_2826_: type_56;
    var phi_2827_: type_56;
    var phi_23312_: bool;
    var phi_23439_: bool;
    var phi_23454_: bool;
    var phi_3155_: vec3<f32>;
    var phi_23576_: f32;
    var phi_23588_: type_56;
    var phi_23599_: type_57;
    var phi_23600_: type_57;
    var phi_23604_: type_57;
    var phi_23605_: type_57;
    var phi_23606_: bool;
    var phi_23610_: type_57;
    var phi_23544_: f32;
    var phi_23686_: f32;
    var phi_23698_: type_56;
    var phi_23709_: type_57;
    var phi_23710_: type_57;
    var phi_23714_: type_57;
    var phi_23715_: type_57;
    var phi_23716_: bool;
    var phi_23720_: type_57;
    var phi_23654_: f32;
    var phi_23481_: vec3<f32>;
    var phi_23482_: u32;
    var phi_23852_: bool;
    var phi_23515_: vec3<f32>;
    var phi_23516_: u32;
    var local_11: vec3<f32>;
    var phi_23523_: type_58;
    var phi_23527_: type_58;
    var phi_3214_: vec3<f32>;
    var phi_3215_: vec3<f32>;
    var phi_2718_: vec3<f32>;
    var phi_2721_: vec3<f32>;
    var phi_2723_: vec3<f32>;
    var phi_2725_: vec3<f32>;
    var phi_2727_: u32;
    var phi_33598_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_33692_: bool;
    var phi_3235_: type_52;
    var phi_33691_: bool;
    var phi_3236_: type_52;
    var phi_33690_: bool;
    var phi_3259_: vec3<f32>;
    var phi_3260_: i32;
    var phi_3261_: u32;
    var phi_33689_: bool;
    var phi_905_: vec3<f32>;
    var phi_908_: i32;
    var phi_910_: u32;
    var phi_3262_: bool;
    var phi_33688_: bool;
    var phi_3596_: vec3<f32>;
    var phi_3599_: u32;
    var phi_23980_: f32;
    var phi_23995_: f32;
    var phi_24010_: f32;
    var phi_3670_: i32;
    var phi_3700_: i32;
    var phi_3708_: vec3<f32>;
    var phi_3711_: i32;
    var phi_3734_: vec3<f32>;
    var phi_3737_: i32;
    var phi_3735_: vec3<f32>;
    var phi_3738_: i32;
    var phi_3709_: vec3<f32>;
    var phi_3712_: i32;
    var phi_3936_: vec3<f32>;
    var phi_3937_: vec3<f32>;
    var phi_3938_: vec3<f32>;
    var phi_3597_: vec3<f32>;
    var phi_3600_: u32;
    var phi_33693_: bool;
    var local_15: vec3<f32>;
    var phi_33741_: bool;
    var phi_3949_: f32;
    var phi_3973_: f32;
    var phi_3975_: u32;
    var phi_3978_: f32;
    var phi_4081_: f32;
    var phi_4082_: f32;
    var phi_4098_: f32;
    var phi_3976_: u32;
    var phi_3979_: f32;
    var phi_4101_: u32;
    var phi_4104_: f32;
    var phi_4212_: f32;
    var phi_4102_: u32;
    var phi_4105_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_24126_: f32;
    var phi_24138_: type_56;
    var phi_24149_: type_57;
    var phi_24150_: type_57;
    var phi_24154_: type_57;
    var phi_24155_: type_57;
    var phi_24156_: bool;
    var phi_24160_: type_57;
    var phi_24094_: f32;
    var phi_4613_: bool;
    var phi_24286_: f32;
    var phi_24298_: type_56;
    var phi_24309_: type_57;
    var phi_24310_: type_57;
    var phi_24314_: type_57;
    var phi_24315_: type_57;
    var phi_24316_: bool;
    var phi_24320_: type_57;
    var phi_24207_: type_57;
    var phi_24358_: bool;
    var phi_24438_: f32;
    var phi_24450_: type_56;
    var phi_24461_: type_57;
    var phi_24462_: type_57;
    var phi_24466_: type_57;
    var phi_24467_: type_57;
    var phi_24468_: bool;
    var phi_24472_: type_57;
    var phi_24406_: f32;
    var phi_24510_: bool;
    var phi_24235_: f32;
    var phi_24237_: f32;
    var phi_24246_: type_57;
    var phi_24250_: type_57;
    var phi_24254_: type_57;
    var phi_4627_: vec3<f32>;
    var phi_4630_: u32;
    var phi_4714_: u32;
    var phi_4729_: type_56;
    var phi_4730_: type_56;
    var phi_4731_: type_56;
    var phi_24709_: bool;
    var phi_4628_: vec3<f32>;
    var phi_4631_: u32;
    var phi_33720_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_33787_: bool;
    var phi_4756_: vec3<f32>;
    var phi_33786_: bool;
    var phi_4758_: vec3<f32>;
    var phi_4760_: bool;
    var phi_33782_: bool;
    var phi_4771_: vec3<f32>;
    var phi_4772_: bool;
    var phi_33781_: bool;
    var phi_4774_: vec3<f32>;
    var phi_4775_: bool;
    var phi_4776_: bool;
    var phi_33780_: bool;
    var phi_4780_: vec3<f32>;
    var phi_4781_: vec3<f32>;
    var phi_4782_: bool;
    var phi_4783_: vec3<f32>;
    var phi_4784_: bool;
    var phi_5104_: vec3<f32>;
    var phi_5107_: u32;
    var phi_24780_: f32;
    var phi_24795_: f32;
    var phi_24810_: f32;
    var phi_5178_: i32;
    var phi_5208_: i32;
    var phi_5216_: vec3<f32>;
    var phi_5219_: i32;
    var phi_5242_: vec3<f32>;
    var phi_5245_: i32;
    var phi_5316_: vec3<f32>;
    var phi_5319_: u32;
    var phi_5439_: vec3<f32>;
    var phi_5317_: vec3<f32>;
    var phi_5320_: u32;
    var phi_5243_: vec3<f32>;
    var phi_5246_: i32;
    var phi_5217_: vec3<f32>;
    var phi_5220_: i32;
    var phi_5444_: vec3<f32>;
    var phi_5445_: vec3<f32>;
    var phi_5446_: vec3<f32>;
    var phi_5105_: vec3<f32>;
    var phi_5108_: u32;
    var phi_33775_: bool;
    var phi_33817_: bool;
    var phi_5466_: vec3<f32>;
    var phi_5679_: u32;
    var phi_5680_: bool;
    var phi_5683_: u32;
    var phi_5684_: bool;
    var phi_5685_: bool;
    var phi_5695_: f32;
    var phi_5698_: f32;
    var phi_5700_: u32;
    var phi_5702_: vec3<f32>;
    var phi_5704_: vec3<f32>;
    var phi_5706_: f32;
    var phi_5708_: f32;
    var phi_5746_: u32;
    var phi_5761_: vec3<f32>;
    var phi_5781_: vec3<f32>;
    var phi_24825_: vec3<f32>;
    var phi_5782_: u32;
    var phi_5783_: vec3<f32>;
    var phi_5784_: u32;
    var phi_5785_: vec3<f32>;
    var phi_5786_: u32;
    var phi_5788_: vec3<f32>;
    var phi_5923_: u32;
    var phi_5926_: f32;
    var phi_5928_: f32;
    var phi_5930_: f32;
    var phi_5932_: vec3<f32>;
    var phi_24880_: f32;
    var phi_24886_: f32;
    var phi_24914_: f32;
    var phi_24920_: f32;
    var phi_24948_: f32;
    var phi_24954_: f32;
    var phi_24982_: f32;
    var phi_24988_: f32;
    var phi_25016_: f32;
    var phi_25022_: f32;
    var phi_25050_: f32;
    var phi_25056_: f32;
    var phi_25084_: f32;
    var phi_25090_: f32;
    var phi_25118_: f32;
    var phi_25124_: f32;
    var phi_5924_: u32;
    var phi_5927_: f32;
    var phi_5929_: f32;
    var phi_5931_: f32;
    var phi_5933_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6349_: f32;
    var phi_6401_: vec3<f32>;
    var phi_6402_: f32;
    var phi_6403_: vec3<f32>;
    var phi_6404_: f32;
    var phi_6405_: vec3<f32>;
    var phi_6406_: f32;
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
    var phi_6421_: f32;
    var phi_6422_: f32;
    var phi_6423_: vec3<f32>;
    var phi_6424_: vec3<f32>;
    var phi_6426_: f32;
    var phi_6427_: f32;
    var phi_6428_: vec3<f32>;
    var phi_6429_: vec3<f32>;
    var phi_6430_: f32;
    var phi_6431_: f32;
    var phi_6432_: f32;
    var phi_6433_: f32;
    var phi_6434_: vec3<f32>;
    var phi_6435_: vec3<f32>;
    var phi_6436_: f32;
    var phi_6437_: f32;
    var phi_5696_: f32;
    var phi_5699_: f32;
    var phi_5701_: u32;
    var phi_5703_: vec3<f32>;
    var phi_5705_: vec3<f32>;
    var phi_5707_: f32;
    var phi_5709_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_25200_: bool;
    var local_29: f32;
    var phi_6469_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6548_: f32;
    var phi_6549_: vec3<f32>;
    var phi_6550_: f32;
    var phi_6552_: vec3<f32>;
    var phi_6555_: u32;
    var phi_7003_: f32;
    var phi_7183_: u32;
    var phi_7186_: f32;
    var phi_7188_: f32;
    var phi_7190_: f32;
    var phi_7192_: vec3<f32>;
    var phi_25289_: f32;
    var phi_25295_: f32;
    var phi_25323_: f32;
    var phi_25329_: f32;
    var phi_25357_: f32;
    var phi_25363_: f32;
    var phi_25391_: f32;
    var phi_25397_: f32;
    var phi_25425_: f32;
    var phi_25431_: f32;
    var phi_25459_: f32;
    var phi_25465_: f32;
    var phi_25493_: f32;
    var phi_25499_: f32;
    var phi_25527_: f32;
    var phi_25533_: f32;
    var phi_7184_: u32;
    var phi_7187_: f32;
    var phi_7189_: f32;
    var phi_7191_: f32;
    var phi_7193_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7606_: f32;
    var phi_25579_: bool;
    var phi_25594_: bool;
    var phi_7699_: vec3<f32>;
    var phi_7700_: vec3<f32>;
    var phi_7701_: vec3<f32>;
    var phi_7702_: vec3<f32>;
    var phi_6553_: vec3<f32>;
    var phi_6556_: u32;
    var phi_7721_: vec3<f32>;
    var phi_7724_: u32;
    var phi_7726_: f32;
    var phi_8027_: vec3<f32>;
    var phi_8028_: u32;
    var phi_8029_: f32;
    var phi_8030_: vec3<f32>;
    var phi_8047_: vec3<f32>;
    var phi_8049_: u32;
    var phi_8052_: vec3<f32>;
    var phi_8054_: f32;
    var phi_8056_: f32;
    var phi_8058_: f32;
    var phi_8078_: bool;
    var phi_8094_: bool;
    var phi_8096_: bool;
    var phi_8098_: bool;
    var phi_8099_: bool;
    var phi_8100_: bool;
    var phi_8101_: bool;
    var phi_8102_: bool;
    var phi_8146_: vec3<f32>;
    var phi_25613_: vec3<f32>;
    var phi_8147_: u32;
    var phi_8148_: vec3<f32>;
    var phi_8149_: u32;
    var phi_8150_: vec3<f32>;
    var phi_8285_: u32;
    var phi_8288_: f32;
    var phi_8290_: f32;
    var phi_8292_: f32;
    var phi_8294_: vec3<f32>;
    var phi_25668_: f32;
    var phi_25674_: f32;
    var phi_25702_: f32;
    var phi_25708_: f32;
    var phi_25736_: f32;
    var phi_25742_: f32;
    var phi_25770_: f32;
    var phi_25776_: f32;
    var phi_25804_: f32;
    var phi_25810_: f32;
    var phi_25838_: f32;
    var phi_25844_: f32;
    var phi_25872_: f32;
    var phi_25878_: f32;
    var phi_25906_: f32;
    var phi_25912_: f32;
    var phi_8286_: u32;
    var phi_8289_: f32;
    var phi_8291_: f32;
    var phi_8293_: f32;
    var phi_8295_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8708_: f32;
    var phi_8734_: vec3<f32>;
    var phi_8735_: f32;
    var phi_8736_: f32;
    var phi_8737_: vec3<f32>;
    var phi_8738_: f32;
    var phi_8739_: f32;
    var phi_8740_: vec3<f32>;
    var phi_8741_: f32;
    var phi_8742_: f32;
    var phi_8743_: vec3<f32>;
    var phi_8744_: f32;
    var phi_8745_: f32;
    var phi_8747_: vec3<f32>;
    var phi_8748_: f32;
    var phi_8749_: f32;
    var phi_8750_: f32;
    var phi_8050_: u32;
    var phi_8053_: vec3<f32>;
    var phi_8055_: f32;
    var phi_8057_: f32;
    var phi_8059_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9040_: type_59;
    var local_42: f32;
    var phi_9041_: type_59;
    var local_43: f32;
    var local_44: f32;
    var phi_9042_: type_59;
    var phi_9043_: type_59;
    var phi_7722_: vec3<f32>;
    var phi_7725_: u32;
    var phi_7727_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9072_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_34025_: bool;
    var phi_9119_: vec3<f32>;
    var phi_34024_: bool;
    var phi_9120_: vec3<f32>;
    var phi_34023_: bool;
    var phi_9121_: vec3<f32>;
    var phi_34019_: bool;
    var phi_10394_: vec3<f32>;
    var phi_34018_: bool;
    var phi_10395_: vec3<f32>;
    var phi_34017_: bool;
    var phi_513_: vec3<f32>;
    var phi_516_: u32;
    var phi_34016_: bool;
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
                phi_492_ = type_48(0u, type_48().member_1);
            } else {
                let _e301 = global_1.member.member[0u];
                let _e305 = global_1.member.member[1u];
                let _e310 = global_1.member.member_7[3u];
                let _e317 = atomicLoad((&global_8.member[((12u + (_e287 * ((_e301 * _e305) * _e310))) + _e291)]));
                phi_492_ = type_48(1u, _e317);
            }
            let _e323 = phi_492_;
            if (bitcast<i32>(_e323.member) != 0i) {
                phi_33555_ = false;
                phi_512_ = vec3<f32>(0f, 0f, 0f);
                phi_515_ = 0u;
                loop {
                    let _e329 = phi_33555_;
                    let _e331 = phi_512_;
                    let _e333 = phi_515_;
                    local_55 = _e331;
                    local_56 = _e331;
                    local_57 = _e331;
                    let _e334 = (_e333 < 2u);
                    if _e334 {
                        let _e336 = ((2u * _e323.member_1) + _e333);
                        if (_e336 < arrayLength((&global_9.member))) {
                        } else {
                            phi_34016_ = true;
                            break;
                        }
                        let _e340 = global_9.member[_e336];
                        if ((((_e340.member_2.x * _e340.member_2.x) + (_e340.member_2.y * _e340.member_2.y)) + (_e340.member_2.z * _e340.member_2.z)) == 0f) {
                            phi_34018_ = _e329;
                            phi_10395_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e340.member.w < 0f) {
                                let _e1295 = global_1.member.member_8[0u];
                                let _e1298 = global_1.member.member_8[3u];
                                if (_e1295 == 4294967295u) {
                                    let _e1300 = (_e1298 == 4294967295u);
                                    if _e1300 {
                                        let _e1303 = global_1.member.member_3[1u];
                                        let _e1306 = global_1.member.member_3[2u];
                                        let _e1309 = ((atan2(_e340.member_1.z, _e340.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1314 = (((_e1309 - floor(_e1309)) * f32(_e1303)) - 0.5f);
                                        let _e1316 = select(_e340.member_1.y, -1f, (_e340.member_1.y < -1f));
                                        let _e1323 = (((acos(select(_e1316, 1f, (_e1316 > 1f))) * 0.31830987f) * f32(_e1306)) - 0.5f);
                                        let _e1324 = (_e1306 - 1u);
                                        let _e1325 = f32(_e1324);
                                        if (0f <= _e1325) {
                                        } else {
                                            phi_34016_ = true;
                                            break;
                                        }
                                        let _e1328 = select(_e1323, 0f, (_e1323 < 0f));
                                        let _e1330 = select(_e1328, _e1325, (_e1328 > _e1325));
                                        let _e1331 = floor(_e1314);
                                        let _e1332 = floor(_e1330);
                                        let _e1333 = (_e1314 - _e1331);
                                        let _e1334 = (_e1330 - _e1332);
                                        let _e1345 = (_e1303 == 0u);
                                        if _e1345 {
                                            phi_34016_ = true;
                                            break;
                                        }
                                        let _e1346 = (bitcast<u32>((select(0i, select(select(i32(_e1331), i32(-2147483648), (_e1331 < -2147483600f)), 2147483647i, (_e1331 > 2147483500f)), (_e1331 == _e1331)) + bitcast<i32>(_e1303))) % _e1303);
                                        let _e1351 = select(select(u32(_e1332), 0u, (_e1332 < 0f)), 4294967295u, (_e1332 > 4294967000f));
                                        let _e1352 = (_e1351 + 1u);
                                        let _e1357 = global_1.member.member_2[2u];
                                        let _e1358 = (_e1351 * _e1303);
                                        if _e1345 {
                                            phi_34016_ = true;
                                            break;
                                        }
                                        let _e1359 = (_e1346 % _e1303);
                                        let _e1362 = (_e1357 + (3u * (_e1358 + _e1359)));
                                        let _e1364 = global_6.member[_e1362];
                                        let _e1367 = global_6.member[(_e1362 + 1u)];
                                        let _e1370 = global_6.member[(_e1362 + 2u)];
                                        if _e1345 {
                                            phi_34016_ = true;
                                            break;
                                        }
                                        let _e1373 = ((_e1346 + 1u) % _e1303);
                                        let _e1376 = (_e1357 + (3u * (_e1358 + _e1373)));
                                        let _e1378 = global_6.member[_e1376];
                                        let _e1381 = global_6.member[(_e1376 + 1u)];
                                        let _e1384 = global_6.member[(_e1376 + 2u)];
                                        let _e1392 = (vec3<f32>(_e1364, _e1367, _e1370) + vec3<f32>(((_e1378 - _e1364) * _e1333), ((_e1381 - _e1367) * _e1333), ((_e1384 - _e1370) * _e1333)));
                                        let _e1393 = (select(_e1352, _e1324, (_e1324 < _e1352)) * _e1303);
                                        if _e1345 {
                                            phi_34016_ = true;
                                            break;
                                        }
                                        let _e1396 = (_e1357 + (3u * (_e1393 + _e1359)));
                                        let _e1398 = global_6.member[_e1396];
                                        let _e1401 = global_6.member[(_e1396 + 1u)];
                                        let _e1404 = global_6.member[(_e1396 + 2u)];
                                        if _e1345 {
                                            phi_34016_ = true;
                                            break;
                                        }
                                        let _e1408 = (_e1357 + (3u * (_e1393 + _e1373)));
                                        let _e1410 = global_6.member[_e1408];
                                        let _e1413 = global_6.member[(_e1408 + 1u)];
                                        let _e1416 = global_6.member[(_e1408 + 2u)];
                                        let _e1424 = (vec3<f32>(_e1398, _e1401, _e1404) + vec3<f32>(((_e1410 - _e1398) * _e1333), ((_e1413 - _e1401) * _e1333), ((_e1416 - _e1404) * _e1333)));
                                        phi_891_ = (_e1392 + vec3<f32>(((_e1424.x - _e1392.x) * _e1334), ((_e1424.y - _e1392.y) * _e1334), ((_e1424.z - _e1392.z) * _e1334)));
                                    } else {
                                        phi_891_ = vec3<f32>();
                                    }
                                    let _e1440 = phi_891_;
                                    phi_893_ = _e1440;
                                    phi_894_ = select(true, false, _e1300);
                                } else {
                                    phi_893_ = vec3<f32>();
                                    phi_894_ = true;
                                }
                                let _e1443 = phi_893_;
                                let _e1445 = phi_894_;
                                if _e1445 {
                                    let _e1446 = (_e1298 != 4294967295u);
                                    if (_e1295 != 4294967295u) {
                                        phi_33630_ = _e329;
                                        phi_904_ = vec3<f32>(0f, 0f, 0f);
                                        phi_907_ = 0i;
                                        phi_909_ = _e1295;
                                        loop {
                                            let _e1449 = phi_33630_;
                                            let _e1451 = phi_904_;
                                            let _e1453 = phi_907_;
                                            let _e1455 = phi_909_;
                                            local_17 = _e1451;
                                            local_18 = _e1451;
                                            local_19 = _e1451;
                                            local_61 = _e1451;
                                            local_62 = _e1451;
                                            if (_e1455 != 4294967295u) {
                                                let _e1457 = (_e1453 < 2i);
                                                if _e1457 {
                                                    let _e1459 = global_6.member[_e1455];
                                                    let _e1462 = global_6.member[(_e1455 + 1u)];
                                                    let _e1465 = global_6.member[(_e1455 + 2u)];
                                                    let _e1468 = global_6.member[(_e1455 + 3u)];
                                                    let _e1471 = global_6.member[(_e1455 + 4u)];
                                                    let _e1474 = global_6.member[(_e1455 + 5u)];
                                                    let _e1477 = global_6.member[(_e1455 + 6u)];
                                                    let _e1480 = global_6.member[(_e1455 + 7u)];
                                                    let _e1483 = global_6.member[(_e1455 + 8u)];
                                                    let _e1486 = global_6.member[(_e1455 + 9u)];
                                                    let _e1489 = global_6.member[(_e1455 + 10u)];
                                                    let _e1492 = global_6.member[(_e1455 + 11u)];
                                                    let _e1495 = global_6.member[(_e1455 + 12u)];
                                                    let _e1498 = global_6.member[(_e1455 + 13u)];
                                                    let _e1501 = global_6.member[(_e1455 + 14u)];
                                                    let _e1504 = global_6.member[(_e1455 + 15u)];
                                                    let _e1508 = global_6.member[(_e1455 + 16u)];
                                                    let _e1511 = global_6.member[(_e1455 + 17u)];
                                                    let _e1514 = global_6.member[(_e1455 + 18u)];
                                                    let _e1517 = global_6.member[(_e1455 + 19u)];
                                                    let _e1520 = global_6.member[(_e1455 + 20u)];
                                                    let _e1523 = global_6.member[(_e1455 + 30u)];
                                                    let _e1524 = bitcast<u32>(_e1523);
                                                    let _e1527 = global_6.member[(_e1455 + 31u)];
                                                    let _e1528 = bitcast<u32>(_e1527);
                                                    let _e1531 = global_6.member[(_e1455 + 32u)];
                                                    let _e1533 = (_e340.member_1.y < 0f);
                                                    if _e1533 {
                                                        phi_2697_ = type_52();
                                                        phi_2698_ = true;
                                                        phi_2699_ = false;
                                                    } else {
                                                        let _e1534 = (_e1528 == 4294967295u);
                                                        if _e1534 {
                                                            phi_2695_ = type_52();
                                                            phi_2696_ = false;
                                                        } else {
                                                            let _e1548 = (_e340.member.x - _e1459);
                                                            let _e1549 = (_e340.member.y - _e1462);
                                                            let _e1550 = (_e340.member.z - _e1465);
                                                            let _e1558 = ((((((_e340.member.x * _e340.member.x) + (_e340.member.y * _e340.member.y)) + (_e340.member.z * _e340.member.z)) - (2f * (((_e340.member.x * _e1459) + (_e340.member.y * _e1462)) + (_e340.member.z * _e1465)))) + _e1474) / (sqrt((((_e1548 * _e1548) + (_e1549 * _e1549)) + (_e1550 * _e1550))) + _e1468));
                                                            let _e1559 = (_e1455 + 21u);
                                                            let _e1562 = global_6.member[(_e1455 + 28u)];
                                                            if (_e1558 > _e1562) {
                                                                phi_2685_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                let _e1565 = global_6.member[_e1559];
                                                                if (_e1558 != _e1558) {
                                                                    phi_22778_ = true;
                                                                } else {
                                                                    phi_22778_ = (_e1565 >= _e1558);
                                                                }
                                                                let _e1569 = phi_22778_;
                                                                let _e1570 = select(_e1558, _e1565, _e1569);
                                                                phi_1331_ = 0u;
                                                                loop {
                                                                    let _e1572 = phi_1331_;
                                                                    local_8 = _e1572;
                                                                    local_9 = _e1572;
                                                                    local_10 = _e1572;
                                                                    if (_e1572 < 8u) {
                                                                        let _e1576 = global_6.member[(_e1559 + _e1572)];
                                                                        let _e1577 = (_e1576 <= _e1570);
                                                                        if _e1577 {
                                                                            phi_1361_ = (_e1572 + 1u);
                                                                        } else {
                                                                            phi_1361_ = u32();
                                                                        }
                                                                        let _e1580 = phi_1361_;
                                                                        phi_1332_ = _e1580;
                                                                        phi_1362_ = _e1577;
                                                                    } else {
                                                                        phi_1332_ = u32();
                                                                        phi_1362_ = false;
                                                                    }
                                                                    let _e1582 = phi_1332_;
                                                                    let _e1584 = phi_1362_;
                                                                    phi_1331_ = _e1582;
                                                                    if !(_e1584) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1587 = local_8;
                                                                if (_e1587 < 1u) {
                                                                    phi_1380_ = 1u;
                                                                } else {
                                                                    let _e1590 = local_9;
                                                                    let _e1593 = local_10;
                                                                    phi_1380_ = select(_e1593, 7u, (_e1590 > 7u));
                                                                }
                                                                let _e1596 = phi_1380_;
                                                                let _e1597 = (_e1596 - 1u);
                                                                let _e1600 = global_6.member[(_e1559 + _e1597)];
                                                                let _e1603 = global_6.member[(_e1559 + _e1596)];
                                                                let _e1606 = ((_e1570 - _e1600) / (_e1603 - _e1600));
                                                                let _e1607 = select(_e340.member_1.y, 0f, _e1533);
                                                                let _e1613 = (sqrt((asin(select(_e1607, 1f, (_e1607 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1615 = (atan2(_e340.member_1.z, _e340.member_1.x) * 0.15915494f);
                                                                let _e1618 = ((_e1615 - floor(_e1615)) * 256f);
                                                                let _e1619 = floor(_e1618);
                                                                let _e1625 = (select(select(u32(_e1619), 0u, (_e1619 < 0f)), 4294967295u, (_e1619 > 4294967000f)) % 256u);
                                                                let _e1626 = floor(_e1613);
                                                                let _e1631 = select(select(u32(_e1626), 0u, (_e1626 < 0f)), 4294967295u, (_e1626 > 4294967000f));
                                                                let _e1633 = select(_e1631, 126u, (126u < _e1631));
                                                                let _e1635 = ((_e1625 + 1u) % 256u);
                                                                let _e1637 = (_e1618 - _e1619);
                                                                let _e1639 = (_e1613 - f32(_e1633));
                                                                let _e1641 = (_e1528 + (_e1597 * 294912u));
                                                                let _e1642 = (_e1633 * 256u);
                                                                let _e1644 = (9u * (_e1642 + _e1625));
                                                                let _e1645 = (_e1641 + _e1644);
                                                                let _e1647 = (9u * (_e1642 + _e1635));
                                                                let _e1648 = (_e1641 + _e1647);
                                                                let _e1649 = ((_e1633 + 1u) * 256u);
                                                                let _e1651 = (9u * (_e1649 + _e1625));
                                                                let _e1652 = (_e1641 + _e1651);
                                                                let _e1654 = (9u * (_e1649 + _e1635));
                                                                let _e1655 = (_e1641 + _e1654);
                                                                let _e1656 = (1f - _e1637);
                                                                let _e1657 = (1f - _e1639);
                                                                let _e1658 = (_e1656 * _e1657);
                                                                let _e1659 = (_e1637 * _e1657);
                                                                let _e1660 = (_e1656 * _e1639);
                                                                let _e1661 = (_e1637 * _e1639);
                                                                let _e1663 = global_6.member[_e1645];
                                                                let _e1666 = global_6.member[(_e1645 + 1u)];
                                                                let _e1669 = global_6.member[(_e1645 + 2u)];
                                                                let _e1674 = global_6.member[_e1648];
                                                                let _e1677 = global_6.member[(_e1648 + 1u)];
                                                                let _e1680 = global_6.member[(_e1648 + 2u)];
                                                                let _e1688 = global_6.member[_e1652];
                                                                let _e1691 = global_6.member[(_e1652 + 1u)];
                                                                let _e1694 = global_6.member[(_e1652 + 2u)];
                                                                let _e1702 = global_6.member[_e1655];
                                                                let _e1705 = global_6.member[(_e1655 + 1u)];
                                                                let _e1708 = global_6.member[(_e1655 + 2u)];
                                                                let _e1717 = global_6.member[(_e1645 + 3u)];
                                                                let _e1720 = global_6.member[(_e1645 + 4u)];
                                                                let _e1723 = global_6.member[(_e1645 + 5u)];
                                                                let _e1729 = global_6.member[(_e1648 + 3u)];
                                                                let _e1732 = global_6.member[(_e1648 + 4u)];
                                                                let _e1735 = global_6.member[(_e1648 + 5u)];
                                                                let _e1744 = global_6.member[(_e1652 + 3u)];
                                                                let _e1747 = global_6.member[(_e1652 + 4u)];
                                                                let _e1750 = global_6.member[(_e1652 + 5u)];
                                                                let _e1759 = global_6.member[(_e1655 + 3u)];
                                                                let _e1762 = global_6.member[(_e1655 + 4u)];
                                                                let _e1765 = global_6.member[(_e1655 + 5u)];
                                                                let _e1774 = global_6.member[(_e1645 + 6u)];
                                                                let _e1777 = global_6.member[(_e1645 + 7u)];
                                                                let _e1780 = global_6.member[(_e1645 + 8u)];
                                                                let _e1786 = global_6.member[(_e1648 + 6u)];
                                                                let _e1789 = global_6.member[(_e1648 + 7u)];
                                                                let _e1792 = global_6.member[(_e1648 + 8u)];
                                                                let _e1801 = global_6.member[(_e1652 + 6u)];
                                                                let _e1804 = global_6.member[(_e1652 + 7u)];
                                                                let _e1807 = global_6.member[(_e1652 + 8u)];
                                                                let _e1816 = global_6.member[(_e1655 + 6u)];
                                                                let _e1819 = global_6.member[(_e1655 + 7u)];
                                                                let _e1822 = global_6.member[(_e1655 + 8u)];
                                                                let _e1830 = (_e1528 + (_e1596 * 294912u));
                                                                let _e1831 = (_e1830 + _e1644);
                                                                let _e1832 = (_e1830 + _e1647);
                                                                let _e1833 = (_e1830 + _e1651);
                                                                let _e1834 = (_e1830 + _e1654);
                                                                let _e1836 = global_6.member[_e1831];
                                                                let _e1839 = global_6.member[(_e1831 + 1u)];
                                                                let _e1842 = global_6.member[(_e1831 + 2u)];
                                                                let _e1847 = global_6.member[_e1832];
                                                                let _e1850 = global_6.member[(_e1832 + 1u)];
                                                                let _e1853 = global_6.member[(_e1832 + 2u)];
                                                                let _e1861 = global_6.member[_e1833];
                                                                let _e1864 = global_6.member[(_e1833 + 1u)];
                                                                let _e1867 = global_6.member[(_e1833 + 2u)];
                                                                let _e1875 = global_6.member[_e1834];
                                                                let _e1878 = global_6.member[(_e1834 + 1u)];
                                                                let _e1881 = global_6.member[(_e1834 + 2u)];
                                                                let _e1890 = global_6.member[(_e1831 + 3u)];
                                                                let _e1893 = global_6.member[(_e1831 + 4u)];
                                                                let _e1896 = global_6.member[(_e1831 + 5u)];
                                                                let _e1902 = global_6.member[(_e1832 + 3u)];
                                                                let _e1905 = global_6.member[(_e1832 + 4u)];
                                                                let _e1908 = global_6.member[(_e1832 + 5u)];
                                                                let _e1917 = global_6.member[(_e1833 + 3u)];
                                                                let _e1920 = global_6.member[(_e1833 + 4u)];
                                                                let _e1923 = global_6.member[(_e1833 + 5u)];
                                                                let _e1932 = global_6.member[(_e1834 + 3u)];
                                                                let _e1935 = global_6.member[(_e1834 + 4u)];
                                                                let _e1938 = global_6.member[(_e1834 + 5u)];
                                                                let _e1947 = global_6.member[(_e1831 + 6u)];
                                                                let _e1950 = global_6.member[(_e1831 + 7u)];
                                                                let _e1953 = global_6.member[(_e1831 + 8u)];
                                                                let _e1959 = global_6.member[(_e1832 + 6u)];
                                                                let _e1962 = global_6.member[(_e1832 + 7u)];
                                                                let _e1965 = global_6.member[(_e1832 + 8u)];
                                                                let _e1974 = global_6.member[(_e1833 + 6u)];
                                                                let _e1977 = global_6.member[(_e1833 + 7u)];
                                                                let _e1980 = global_6.member[(_e1833 + 8u)];
                                                                let _e1989 = global_6.member[(_e1834 + 6u)];
                                                                let _e1992 = global_6.member[(_e1834 + 7u)];
                                                                let _e1995 = global_6.member[(_e1834 + 8u)];
                                                                let _e2002 = (1f - _e1606);
                                                                phi_2685_ = type_55(1u, type_52(vec3<f32>(((((((_e1663 * _e1658) + (_e1674 * _e1659)) + (_e1688 * _e1660)) + (_e1702 * _e1661)) * _e2002) + (((((_e1836 * _e1658) + (_e1847 * _e1659)) + (_e1861 * _e1660)) + (_e1875 * _e1661)) * _e1606)), ((((((_e1666 * _e1658) + (_e1677 * _e1659)) + (_e1691 * _e1660)) + (_e1705 * _e1661)) * _e2002) + (((((_e1839 * _e1658) + (_e1850 * _e1659)) + (_e1864 * _e1660)) + (_e1878 * _e1661)) * _e1606)), ((((((_e1669 * _e1658) + (_e1680 * _e1659)) + (_e1694 * _e1660)) + (_e1708 * _e1661)) * _e2002) + (((((_e1842 * _e1658) + (_e1853 * _e1659)) + (_e1867 * _e1660)) + (_e1881 * _e1661)) * _e1606))), vec3<f32>(((((((_e1717 * _e1658) + (_e1729 * _e1659)) + (_e1744 * _e1660)) + (_e1759 * _e1661)) * _e2002) + (((((_e1890 * _e1658) + (_e1902 * _e1659)) + (_e1917 * _e1660)) + (_e1932 * _e1661)) * _e1606)), ((((((_e1720 * _e1658) + (_e1732 * _e1659)) + (_e1747 * _e1660)) + (_e1762 * _e1661)) * _e2002) + (((((_e1893 * _e1658) + (_e1905 * _e1659)) + (_e1920 * _e1660)) + (_e1935 * _e1661)) * _e1606)), ((((((_e1723 * _e1658) + (_e1735 * _e1659)) + (_e1750 * _e1660)) + (_e1765 * _e1661)) * _e2002) + (((((_e1896 * _e1658) + (_e1908 * _e1659)) + (_e1923 * _e1660)) + (_e1938 * _e1661)) * _e1606))), vec3<f32>(((((((_e1774 * _e1658) + (_e1786 * _e1659)) + (_e1801 * _e1660)) + (_e1816 * _e1661)) * _e2002) + (((((_e1947 * _e1658) + (_e1959 * _e1659)) + (_e1974 * _e1660)) + (_e1989 * _e1661)) * _e1606)), ((((((_e1777 * _e1658) + (_e1789 * _e1659)) + (_e1804 * _e1660)) + (_e1819 * _e1661)) * _e2002) + (((((_e1950 * _e1658) + (_e1962 * _e1659)) + (_e1977 * _e1660)) + (_e1992 * _e1661)) * _e1606)), ((((((_e1780 * _e1658) + (_e1792 * _e1659)) + (_e1807 * _e1660)) + (_e1822 * _e1661)) * _e2002) + (((((_e1953 * _e1658) + (_e1965 * _e1659)) + (_e1980 * _e1660)) + (_e1995 * _e1661)) * _e1606)))));
                                                            }
                                                            let _e2039 = phi_2685_;
                                                            let _e2042 = (bitcast<i32>(_e2039.member) != 0i);
                                                            if _e2042 {
                                                                phi_2693_ = _e2039.member_1;
                                                            } else {
                                                                phi_2693_ = type_52();
                                                            }
                                                            let _e2045 = phi_2693_;
                                                            phi_2695_ = _e2045;
                                                            phi_2696_ = select(true, false, _e2042);
                                                        }
                                                        let _e2048 = phi_2695_;
                                                        let _e2050 = phi_2696_;
                                                        phi_2697_ = _e2048;
                                                        phi_2698_ = _e1534;
                                                        phi_2699_ = _e2050;
                                                    }
                                                    let _e2052 = phi_2697_;
                                                    let _e2054 = phi_2698_;
                                                    let _e2056 = phi_2699_;
                                                    if select(_e2056, true, _e2054) {
                                                        let _e2065 = ((((_e340.member.x - _e1459) * _e340.member_1.x) + ((_e340.member.y - _e1462) * _e340.member_1.y)) + ((_e340.member.z - _e1465) * _e340.member_1.z));
                                                        let _e2077 = ((((_e340.member.x * _e340.member.x) + (_e340.member.y * _e340.member.y)) + (_e340.member.z * _e340.member.z)) - (2f * (((_e340.member.x * _e1459) + (_e340.member.y * _e1462)) + (_e340.member.z * _e1465))));
                                                        let _e2078 = (_e2077 + _e1477);
                                                        let _e2079 = (_e2065 * _e2065);
                                                        let _e2080 = (_e2079 - _e2078);
                                                        if (_e2080 < 0f) {
                                                            phi_22917_ = type_57(0u, type_57().member_1);
                                                            phi_22918_ = type_57();
                                                            phi_22919_ = true;
                                                        } else {
                                                            if (_e2065 < 0f) {
                                                                phi_22889_ = -(sqrt(_e2080));
                                                            } else {
                                                                phi_22889_ = sqrt(_e2080);
                                                            }
                                                            let _e2087 = phi_22889_;
                                                            let _e2089 = -((_e2065 + _e2087));
                                                            let _e2090 = (_e2089 == 0f);
                                                            if _e2090 {
                                                                phi_22912_ = type_57(1u, type_56(0f, 0f));
                                                                phi_22913_ = type_57();
                                                            } else {
                                                                let _e2091 = (_e2078 / _e2089);
                                                                if (_e2089 < _e2091) {
                                                                    phi_22901_ = type_56(_e2089, _e2091);
                                                                } else {
                                                                    phi_22901_ = type_56(_e2091, _e2089);
                                                                }
                                                                let _e2096 = phi_22901_;
                                                                phi_22912_ = type_57();
                                                                phi_22913_ = type_57(1u, _e2096);
                                                            }
                                                            let _e2099 = phi_22912_;
                                                            let _e2101 = phi_22913_;
                                                            phi_22917_ = _e2099;
                                                            phi_22918_ = _e2101;
                                                            phi_22919_ = _e2090;
                                                        }
                                                        let _e2106 = phi_22917_;
                                                        let _e2108 = phi_22918_;
                                                        let _e2110 = phi_22919_;
                                                        if _e2110 {
                                                            phi_22923_ = _e2106;
                                                        } else {
                                                            phi_22923_ = _e2108;
                                                        }
                                                        let _e2112 = phi_22923_;
                                                        if (bitcast<i32>(_e2112.member) != 0i) {
                                                            phi_22810_ = type_57(0u, _e2112.member_1);
                                                        } else {
                                                            phi_22810_ = type_57(1u, type_57().member_1);
                                                        }
                                                        let _e2122 = phi_22810_;
                                                        if (bitcast<i32>(_e2122.member) != 0i) {
                                                            phi_22857_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e2122.member_1.member_1 <= 0f) {
                                                                phi_22853_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e2122.member_1.member != _e2122.member_1.member) {
                                                                    phi_22961_ = true;
                                                                } else {
                                                                    phi_22961_ = (0f >= _e2122.member_1.member);
                                                                }
                                                                let _e2134 = phi_22961_;
                                                                let _e2135 = select(_e2122.member_1.member, 0f, _e2134);
                                                                let _e2136 = (_e2077 + _e1474);
                                                                if (_e2136 > 0f) {
                                                                    let _e2138 = (_e2079 - _e2136);
                                                                    if (_e2138 < 0f) {
                                                                        phi_23069_ = type_57(0u, type_57().member_1);
                                                                        phi_23070_ = type_57();
                                                                        phi_23071_ = true;
                                                                    } else {
                                                                        if (_e2065 < 0f) {
                                                                            phi_23041_ = -(sqrt(_e2138));
                                                                        } else {
                                                                            phi_23041_ = sqrt(_e2138);
                                                                        }
                                                                        let _e2145 = phi_23041_;
                                                                        let _e2147 = -((_e2065 + _e2145));
                                                                        let _e2148 = (_e2147 == 0f);
                                                                        if _e2148 {
                                                                            phi_23064_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23065_ = type_57();
                                                                        } else {
                                                                            let _e2149 = (_e2136 / _e2147);
                                                                            if (_e2147 < _e2149) {
                                                                                phi_23053_ = type_56(_e2147, _e2149);
                                                                            } else {
                                                                                phi_23053_ = type_56(_e2149, _e2147);
                                                                            }
                                                                            let _e2154 = phi_23053_;
                                                                            phi_23064_ = type_57();
                                                                            phi_23065_ = type_57(1u, _e2154);
                                                                        }
                                                                        let _e2157 = phi_23064_;
                                                                        let _e2159 = phi_23065_;
                                                                        phi_23069_ = _e2157;
                                                                        phi_23070_ = _e2159;
                                                                        phi_23071_ = _e2148;
                                                                    }
                                                                    let _e2164 = phi_23069_;
                                                                    let _e2166 = phi_23070_;
                                                                    let _e2168 = phi_23071_;
                                                                    if _e2168 {
                                                                        phi_23075_ = _e2164;
                                                                    } else {
                                                                        phi_23075_ = _e2166;
                                                                    }
                                                                    let _e2170 = phi_23075_;
                                                                    if (bitcast<i32>(_e2170.member) != 0i) {
                                                                        phi_23009_ = select(_e2170.member_1.member_1, _e2170.member_1.member, (_e2170.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23009_ = -1f;
                                                                    }
                                                                    let _e2181 = phi_23009_;
                                                                    if (_e2181 >= 0f) {
                                                                        if (_e2122.member_1.member_1 != _e2122.member_1.member_1) {
                                                                            phi_23113_ = true;
                                                                        } else {
                                                                            phi_23113_ = (_e2181 <= _e2122.member_1.member_1);
                                                                        }
                                                                        let _e2186 = phi_23113_;
                                                                        phi_22838_ = select(_e2122.member_1.member_1, _e2181, _e2186);
                                                                    } else {
                                                                        phi_22838_ = _e2122.member_1.member_1;
                                                                    }
                                                                    let _e2189 = phi_22838_;
                                                                    phi_22840_ = _e2189;
                                                                } else {
                                                                    phi_22840_ = _e2122.member_1.member_1;
                                                                }
                                                                let _e2191 = phi_22840_;
                                                                if (_e2191 <= _e2135) {
                                                                    phi_22849_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    phi_22849_ = type_57(1u, type_56(_e2135, _e2191));
                                                                }
                                                                let _e2199 = phi_22849_;
                                                                phi_22853_ = _e2199;
                                                            }
                                                            let _e2204 = phi_22853_;
                                                            phi_22857_ = _e2204;
                                                        }
                                                        let _e2209 = phi_22857_;
                                                        if (bitcast<i32>(_e2209.member) != 0i) {
                                                            phi_2717_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2720_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2722_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2724_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2726_ = 0u;
                                                            loop {
                                                                let _e2218 = phi_2717_;
                                                                let _e2220 = phi_2720_;
                                                                let _e2222 = phi_2722_;
                                                                let _e2224 = phi_2724_;
                                                                let _e2226 = phi_2726_;
                                                                local_12 = _e2224;
                                                                local_13 = _e2222;
                                                                local_14 = _e2220;
                                                                let _e2227 = (_e2226 < 32u);
                                                                if _e2227 {
                                                                    let _e2228 = -(_e2065);
                                                                    if (_e2209.member_1.member <= _e2209.member_1.member_1) {
                                                                    } else {
                                                                        phi_33598_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2231 = select(_e2228, _e2209.member_1.member, (_e2228 < _e2209.member_1.member));
                                                                    let _e2233 = select(_e2231, _e2209.member_1.member_1, (_e2231 > _e2209.member_1.member_1));
                                                                    let _e2234 = (_e2233 - _e2209.member_1.member);
                                                                    let _e2235 = (_e2209.member_1.member_1 - _e2233);
                                                                    let _e2236 = (_e2209.member_1.member_1 - _e2209.member_1.member);
                                                                    let _e2237 = (0.000000001f * _e2236);
                                                                    if (_e2234 <= _e2237) {
                                                                        let _e2312 = (f32(_e2226) * 0.03125f);
                                                                        let _e2315 = (f32((_e2226 + 1u)) * 0.03125f);
                                                                        let _e2317 = ((_e2312 + _e2315) * 0.5f);
                                                                        phi_2827_ = type_56((_e2209.member_1.member + ((_e2236 * _e2317) * _e2317)), (_e2236 * ((_e2315 * _e2315) - (_e2312 * _e2312))));
                                                                    } else {
                                                                        if (_e2235 <= _e2237) {
                                                                            let _e2295 = (f32((31u - _e2226)) * 0.03125f);
                                                                            let _e2298 = (f32((32u - _e2226)) * 0.03125f);
                                                                            let _e2300 = ((_e2295 + _e2298) * 0.5f);
                                                                            phi_2826_ = type_56((_e2209.member_1.member_1 - ((_e2236 * _e2300) * _e2300)), (_e2236 * ((_e2298 * _e2298) - (_e2295 * _e2295))));
                                                                        } else {
                                                                            let _e2243 = round(((32f * _e2234) / (_e2234 + _e2235)));
                                                                            let _e2248 = select(select(u32(_e2243), 0u, (_e2243 < 0f)), 4294967295u, (_e2243 > 4294967000f));
                                                                            if (_e2248 < 1u) {
                                                                                phi_2810_ = 1u;
                                                                            } else {
                                                                                phi_2810_ = select(_e2248, 31u, (_e2248 > 31u));
                                                                            }
                                                                            let _e2253 = phi_2810_;
                                                                            if (_e2226 < _e2253) {
                                                                                let _e2274 = ((_e2253 - 1u) - _e2226);
                                                                                let _e2276 = f32(_e2253);
                                                                                let _e2277 = (f32(_e2274) / _e2276);
                                                                                let _e2280 = (f32((_e2274 + 1u)) / _e2276);
                                                                                let _e2282 = ((_e2277 + _e2280) * 0.5f);
                                                                                phi_2825_ = type_56((_e2233 - ((_e2234 * _e2282) * _e2282)), (_e2234 * ((_e2280 * _e2280) - (_e2277 * _e2277))));
                                                                            } else {
                                                                                let _e2256 = (_e2226 - _e2253);
                                                                                let _e2258 = f32((32u - _e2253));
                                                                                let _e2259 = (f32(_e2256) / _e2258);
                                                                                let _e2262 = (f32((_e2256 + 1u)) / _e2258);
                                                                                let _e2264 = ((_e2259 + _e2262) * 0.5f);
                                                                                phi_2825_ = type_56((_e2233 + ((_e2235 * _e2264) * _e2264)), (_e2235 * ((_e2262 * _e2262) - (_e2259 * _e2259))));
                                                                            }
                                                                            let _e2292 = phi_2825_;
                                                                            phi_2826_ = _e2292;
                                                                        }
                                                                        let _e2310 = phi_2826_;
                                                                        phi_2827_ = _e2310;
                                                                    }
                                                                    let _e2327 = phi_2827_;
                                                                    let _e2333 = (_e340.member.x + (_e340.member_1.x * _e2327.member));
                                                                    let _e2334 = (_e340.member.y + (_e340.member_1.y * _e2327.member));
                                                                    let _e2335 = (_e340.member.z + (_e340.member_1.z * _e2327.member));
                                                                    let _e2348 = ((((_e2333 * _e2333) + (_e2334 * _e2334)) + (_e2335 * _e2335)) - (2f * (((_e2333 * _e1459) + (_e2334 * _e1462)) + (_e2335 * _e1465))));
                                                                    let _e2349 = (_e2348 + _e1474);
                                                                    let _e2350 = (_e2333 - _e1459);
                                                                    let _e2351 = (_e2334 - _e1462);
                                                                    let _e2352 = (_e2335 - _e1465);
                                                                    let _e2358 = sqrt((((_e2350 * _e2350) + (_e2351 * _e2351)) + (_e2352 * _e2352)));
                                                                    let _e2360 = (_e2349 / (_e2358 + _e1468));
                                                                    let _e2364 = (1f - (abs((_e2360 - 25000f)) * 0.00006666667f));
                                                                    if (_e2364 != _e2364) {
                                                                        phi_23312_ = true;
                                                                    } else {
                                                                        phi_23312_ = (0f >= _e2364);
                                                                    }
                                                                    let _e2368 = phi_23312_;
                                                                    let _e2370 = -(_e2360);
                                                                    let _e2373 = (exp((_e2370 / _e1489)) * _e2327.member_1);
                                                                    let _e2376 = (exp((_e2370 / _e1492)) * _e2327.member_1);
                                                                    let _e2378 = (_e2373 * 0.5f);
                                                                    let _e2379 = (_e2376 * 0.5f);
                                                                    let _e2380 = ((select(_e2364, 0f, _e2368) * _e2327.member_1) * 0.5f);
                                                                    let _e2382 = (_e2218.x + _e2378);
                                                                    let _e2384 = (_e2218.y + _e2379);
                                                                    let _e2386 = (_e2218.z + _e2380);
                                                                    let _e2391 = (_e1508 * 1.1f);
                                                                    let _e2392 = (_e2391 * _e2384);
                                                                    let _e2411 = (_e1508 * _e2376);
                                                                    if (_e1524 == 4294967295u) {
                                                                        phi_3155_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2419 = (1f / _e2358);
                                                                        let _e2427 = ((((_e2350 * _e2419) * _e1480) + ((_e2351 * _e2419) * _e1483)) + ((_e2352 * _e2419) * _e1486));
                                                                        if (_e2360 != _e2360) {
                                                                            phi_23439_ = true;
                                                                        } else {
                                                                            phi_23439_ = (0f >= _e2360);
                                                                        }
                                                                        let _e2431 = phi_23439_;
                                                                        let _e2435 = sqrt((select(_e2360, 0f, _e2431) / (_e1471 - _e1468)));
                                                                        if (_e2435 != _e2435) {
                                                                            phi_23454_ = true;
                                                                        } else {
                                                                            phi_23454_ = (1f <= _e2435);
                                                                        }
                                                                        let _e2439 = phi_23454_;
                                                                        let _e2441 = (select(_e2435, 1f, _e2439) * 31f);
                                                                        let _e2443 = select(_e2427, -1f, (_e2427 < -1f));
                                                                        let _e2447 = ((select(_e2443, 1f, (_e2443 > 1f)) + 1f) * 15.5f);
                                                                        let _e2452 = select(select(u32(_e2441), 0u, (_e2441 < 0f)), 4294967295u, (_e2441 > 4294967000f));
                                                                        let _e2454 = select(_e2452, 30u, (30u < _e2452));
                                                                        let _e2459 = select(select(u32(_e2447), 0u, (_e2447 < 0f)), 4294967295u, (_e2447 > 4294967000f));
                                                                        let _e2461 = select(_e2459, 30u, (30u < _e2459));
                                                                        let _e2463 = (_e2441 - f32(_e2454));
                                                                        let _e2465 = (_e2447 - f32(_e2461));
                                                                        let _e2469 = (_e1524 + (3u * ((_e2454 * 32u) + _e2461)));
                                                                        let _e2474 = (_e1524 + (3u * (((_e2454 + 1u) * 32u) + _e2461)));
                                                                        let _e2476 = global_6.member[_e2469];
                                                                        let _e2479 = global_6.member[(_e2469 + 1u)];
                                                                        let _e2482 = global_6.member[(_e2469 + 2u)];
                                                                        let _e2483 = (1f - _e2465);
                                                                        let _e2489 = global_6.member[(_e2469 + 3u)];
                                                                        let _e2492 = global_6.member[(_e2469 + 4u)];
                                                                        let _e2495 = global_6.member[(_e2469 + 5u)];
                                                                        let _e2502 = (1f - _e2463);
                                                                        let _e2507 = global_6.member[_e2474];
                                                                        let _e2510 = global_6.member[(_e2474 + 1u)];
                                                                        let _e2513 = global_6.member[(_e2474 + 2u)];
                                                                        let _e2519 = global_6.member[(_e2474 + 3u)];
                                                                        let _e2522 = global_6.member[(_e2474 + 4u)];
                                                                        let _e2525 = global_6.member[(_e2474 + 5u)];
                                                                        phi_3155_ = vec3<f32>(((((_e2476 * _e2483) + (_e2489 * _e2465)) * _e2502) + (((_e2507 * _e2483) + (_e2519 * _e2465)) * _e2463)), ((((_e2479 * _e2483) + (_e2492 * _e2465)) * _e2502) + (((_e2510 * _e2483) + (_e2522 * _e2465)) * _e2463)), ((((_e2482 * _e2483) + (_e2495 * _e2465)) * _e2502) + (((_e2513 * _e2483) + (_e2525 * _e2465)) * _e2463)));
                                                                    }
                                                                    let _e2540 = phi_3155_;
                                                                    let _e2558 = (((_e2350 * _e1480) + (_e2351 * _e1483)) + (_e2352 * _e1486));
                                                                    let _e2559 = (_e2558 * _e2558);
                                                                    let _e2560 = (_e2559 - _e2349);
                                                                    if (_e2560 < 0f) {
                                                                        phi_23604_ = type_57(0u, type_57().member_1);
                                                                        phi_23605_ = type_57();
                                                                        phi_23606_ = true;
                                                                    } else {
                                                                        if (_e2558 < 0f) {
                                                                            phi_23576_ = -(sqrt(_e2560));
                                                                        } else {
                                                                            phi_23576_ = sqrt(_e2560);
                                                                        }
                                                                        let _e2567 = phi_23576_;
                                                                        let _e2569 = -((_e2558 + _e2567));
                                                                        let _e2570 = (_e2569 == 0f);
                                                                        if _e2570 {
                                                                            phi_23599_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23600_ = type_57();
                                                                        } else {
                                                                            let _e2571 = (_e2349 / _e2569);
                                                                            if (_e2569 < _e2571) {
                                                                                phi_23588_ = type_56(_e2569, _e2571);
                                                                            } else {
                                                                                phi_23588_ = type_56(_e2571, _e2569);
                                                                            }
                                                                            let _e2576 = phi_23588_;
                                                                            phi_23599_ = type_57();
                                                                            phi_23600_ = type_57(1u, _e2576);
                                                                        }
                                                                        let _e2579 = phi_23599_;
                                                                        let _e2581 = phi_23600_;
                                                                        phi_23604_ = _e2579;
                                                                        phi_23605_ = _e2581;
                                                                        phi_23606_ = _e2570;
                                                                    }
                                                                    let _e2586 = phi_23604_;
                                                                    let _e2588 = phi_23605_;
                                                                    let _e2590 = phi_23606_;
                                                                    if _e2590 {
                                                                        phi_23610_ = _e2586;
                                                                    } else {
                                                                        phi_23610_ = _e2588;
                                                                    }
                                                                    let _e2592 = phi_23610_;
                                                                    if (bitcast<i32>(_e2592.member) != 0i) {
                                                                        phi_23544_ = select(_e2592.member_1.member_1, _e2592.member_1.member, (_e2592.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23544_ = -1f;
                                                                    }
                                                                    let _e2603 = phi_23544_;
                                                                    if (_e2603 >= 0f) {
                                                                        phi_23527_ = type_58(0u, type_58().member_1);
                                                                    } else {
                                                                        let _e2605 = (_e2348 + _e1477);
                                                                        let _e2606 = (_e2559 - _e2605);
                                                                        if (_e2606 < 0f) {
                                                                            phi_23714_ = type_57(0u, type_57().member_1);
                                                                            phi_23715_ = type_57();
                                                                            phi_23716_ = true;
                                                                        } else {
                                                                            if (_e2558 < 0f) {
                                                                                phi_23686_ = -(sqrt(_e2606));
                                                                            } else {
                                                                                phi_23686_ = sqrt(_e2606);
                                                                            }
                                                                            let _e2613 = phi_23686_;
                                                                            let _e2615 = -((_e2558 + _e2613));
                                                                            let _e2616 = (_e2615 == 0f);
                                                                            if _e2616 {
                                                                                phi_23709_ = type_57(1u, type_56(0f, 0f));
                                                                                phi_23710_ = type_57();
                                                                            } else {
                                                                                let _e2617 = (_e2605 / _e2615);
                                                                                if (_e2615 < _e2617) {
                                                                                    phi_23698_ = type_56(_e2615, _e2617);
                                                                                } else {
                                                                                    phi_23698_ = type_56(_e2617, _e2615);
                                                                                }
                                                                                let _e2622 = phi_23698_;
                                                                                phi_23709_ = type_57();
                                                                                phi_23710_ = type_57(1u, _e2622);
                                                                            }
                                                                            let _e2625 = phi_23709_;
                                                                            let _e2627 = phi_23710_;
                                                                            phi_23714_ = _e2625;
                                                                            phi_23715_ = _e2627;
                                                                            phi_23716_ = _e2616;
                                                                        }
                                                                        let _e2632 = phi_23714_;
                                                                        let _e2634 = phi_23715_;
                                                                        let _e2636 = phi_23716_;
                                                                        if _e2636 {
                                                                            phi_23720_ = _e2632;
                                                                        } else {
                                                                            phi_23720_ = _e2634;
                                                                        }
                                                                        let _e2638 = phi_23720_;
                                                                        if (bitcast<i32>(_e2638.member) != 0i) {
                                                                            phi_23654_ = select(_e2638.member_1.member_1, _e2638.member_1.member, (_e2638.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_23654_ = -1f;
                                                                        }
                                                                        let _e2649 = phi_23654_;
                                                                        if (_e2649 < 0f) {
                                                                            phi_23523_ = type_58(0u, type_58().member_1);
                                                                        } else {
                                                                            phi_23481_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_23482_ = 0u;
                                                                            loop {
                                                                                let _e2652 = phi_23481_;
                                                                                let _e2654 = phi_23482_;
                                                                                local_11 = _e2652;
                                                                                let _e2655 = (_e2654 < 16u);
                                                                                if _e2655 {
                                                                                    let _e2657 = (f32(_e2654) * 0.0625f);
                                                                                    let _e2658 = (_e2654 + 1u);
                                                                                    let _e2660 = (f32(_e2658) * 0.0625f);
                                                                                    let _e2662 = ((_e2657 + _e2660) * 0.5f);
                                                                                    let _e2664 = ((_e2649 * _e2662) * _e2662);
                                                                                    let _e2668 = (_e2649 * ((_e2660 * _e2660) - (_e2657 * _e2657)));
                                                                                    let _e2673 = (vec3<f32>(_e2333, _e2334, _e2335) + vec3<f32>((_e1480 * _e2664), (_e1483 * _e2664), (_e1486 * _e2664)));
                                                                                    let _e2690 = (_e2673.x - _e1459);
                                                                                    let _e2691 = (_e2673.y - _e1462);
                                                                                    let _e2692 = (_e2673.z - _e1465);
                                                                                    let _e2700 = ((((((_e2673.x * _e2673.x) + (_e2673.y * _e2673.y)) + (_e2673.z * _e2673.z)) - (2f * (((_e2673.x * _e1459) + (_e2673.y * _e1462)) + (_e2673.z * _e1465)))) + _e1474) / (sqrt((((_e2690 * _e2690) + (_e2691 * _e2691)) + (_e2692 * _e2692))) + _e1468));
                                                                                    let _e2704 = (1f - (abs((_e2700 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2704 != _e2704) {
                                                                                        phi_23852_ = true;
                                                                                    } else {
                                                                                        phi_23852_ = (0f >= _e2704);
                                                                                    }
                                                                                    let _e2708 = phi_23852_;
                                                                                    let _e2710 = -(_e2700);
                                                                                    phi_23515_ = (_e2652 + vec3<f32>((exp((_e2710 / _e1489)) * _e2668), (exp((_e2710 / _e1492)) * _e2668), (select(_e2704, 0f, _e2708) * _e2668)));
                                                                                    phi_23516_ = _e2658;
                                                                                } else {
                                                                                    phi_23515_ = vec3<f32>();
                                                                                    phi_23516_ = u32();
                                                                                }
                                                                                let _e2721 = phi_23515_;
                                                                                let _e2723 = phi_23516_;
                                                                                phi_23481_ = _e2721;
                                                                                phi_23482_ = _e2723;
                                                                                if !(_e2655) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2726 = local_11;
                                                                            phi_23523_ = type_58(1u, _e2726);
                                                                        }
                                                                        let _e2732 = phi_23523_;
                                                                        phi_23527_ = _e2732;
                                                                    }
                                                                    let _e2737 = phi_23527_;
                                                                    if (bitcast<i32>(_e2737.member) != 0i) {
                                                                        let _e2742 = (vec3<f32>(_e2382, _e2384, _e2386) + _e2737.member_1);
                                                                        let _e2748 = (_e2391 * _e2742.y);
                                                                        let _e2760 = exp(-((((_e1498 * _e2742.x) + _e2748) + (_e1514 * _e2742.z))));
                                                                        let _e2762 = exp(-((((_e1501 * _e2742.x) + _e2748) + (_e1517 * _e2742.z))));
                                                                        let _e2764 = exp(-((((_e1504 * _e2742.x) + _e2748) + (_e1520 * _e2742.z))));
                                                                        phi_3214_ = vec3<f32>((_e2222.x + (_e2760 * _e2376)), (_e2222.y + (_e2762 * _e2376)), (_e2222.z + (_e2764 * _e2376)));
                                                                        phi_3215_ = vec3<f32>((_e2224.x + (_e2760 * _e2373)), (_e2224.y + (_e2762 * _e2373)), (_e2224.z + (_e2764 * _e2373)));
                                                                    } else {
                                                                        phi_3214_ = _e2222;
                                                                        phi_3215_ = _e2224;
                                                                    }
                                                                    let _e2786 = phi_3214_;
                                                                    let _e2788 = phi_3215_;
                                                                    phi_2718_ = vec3<f32>((_e2382 + _e2378), (_e2384 + _e2379), (_e2386 + _e2380));
                                                                    phi_2721_ = vec3<f32>((_e2220.x + ((exp(-((((_e1498 * _e2382) + _e2392) + (_e1514 * _e2386)))) * ((_e1498 * _e2373) + _e2411)) * _e2540.x)), (_e2220.y + ((exp(-((((_e1501 * _e2382) + _e2392) + (_e1517 * _e2386)))) * ((_e1501 * _e2373) + _e2411)) * _e2540.y)), (_e2220.z + ((exp(-((((_e1504 * _e2382) + _e2392) + (_e1520 * _e2386)))) * ((_e1504 * _e2373) + _e2411)) * _e2540.z)));
                                                                    phi_2723_ = _e2786;
                                                                    phi_2725_ = _e2788;
                                                                    phi_2727_ = (_e2226 + 1u);
                                                                } else {
                                                                    phi_2718_ = vec3<f32>();
                                                                    phi_2721_ = vec3<f32>();
                                                                    phi_2723_ = vec3<f32>();
                                                                    phi_2725_ = vec3<f32>();
                                                                    phi_2727_ = u32();
                                                                }
                                                                let _e2795 = phi_2718_;
                                                                let _e2797 = phi_2721_;
                                                                let _e2799 = phi_2723_;
                                                                let _e2801 = phi_2725_;
                                                                let _e2803 = phi_2727_;
                                                                phi_2717_ = _e2795;
                                                                phi_2720_ = _e2797;
                                                                phi_2722_ = _e2799;
                                                                phi_2724_ = _e2801;
                                                                phi_2726_ = _e2803;
                                                                phi_33598_ = _e1449;
                                                                if !(_e2227) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2806 = phi_33598_;
                                                            phi_33688_ = _e2806;
                                                            if _e2806 {
                                                                break;
                                                            }
                                                            let _e2808 = local_12;
                                                            let _e2811 = local_13;
                                                            let _e2814 = local_14;
                                                            phi_33692_ = _e2806;
                                                            phi_3235_ = type_52((_e2808 * vec3<f32>(_e1498, _e1501, _e1504)), (_e2811 * _e1508), _e2814);
                                                        } else {
                                                            phi_33692_ = _e1449;
                                                            phi_3235_ = type_52(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2817 = phi_33692_;
                                                        let _e2819 = phi_3235_;
                                                        phi_33691_ = _e2817;
                                                        phi_3236_ = _e2819;
                                                    } else {
                                                        phi_33691_ = _e1449;
                                                        phi_3236_ = _e2052;
                                                    }
                                                    let _e2821 = phi_33691_;
                                                    let _e2823 = phi_3236_;
                                                    let _e2831 = (((_e340.member_1.x * _e1480) + (_e340.member_1.y * _e1483)) + (_e340.member_1.z * _e1486));
                                                    let _e2833 = (1f + (_e2831 * _e2831));
                                                    let _e2834 = (0.059683103f * _e2833);
                                                    let _e2841 = (_e1511 * _e1511);
                                                    let _e2852 = ((0.119366206f * ((1f - _e2841) * _e2833)) / ((2f + _e2841) * pow(((1f + _e2841) - ((2f * _e1511) * _e2831)), 1.5f)));
                                                    phi_33690_ = _e2821;
                                                    phi_3259_ = vec3<f32>((_e1451.x + (((((_e2823.member.x * _e2834) + (_e2823.member_1.x * _e2852)) + _e2823.member_2.x) * _e1495) * 0.38317f)), (_e1451.y + (((((_e2823.member.y * _e2834) + (_e2823.member_1.y * _e2852)) + _e2823.member_2.y) * _e1495) * 0.38317f)), (_e1451.z + (((((_e2823.member.z * _e2834) + (_e2823.member_1.z * _e2852)) + _e2823.member_2.z) * _e1495) * 0.38317f)));
                                                    phi_3260_ = (_e1453 + 1i);
                                                    phi_3261_ = bitcast<u32>(_e1531);
                                                } else {
                                                    phi_33690_ = _e1449;
                                                    phi_3259_ = vec3<f32>();
                                                    phi_3260_ = i32();
                                                    phi_3261_ = u32();
                                                }
                                                let _e2883 = phi_33690_;
                                                let _e2885 = phi_3259_;
                                                let _e2887 = phi_3260_;
                                                let _e2889 = phi_3261_;
                                                phi_33689_ = _e2883;
                                                phi_905_ = _e2885;
                                                phi_908_ = _e2887;
                                                phi_910_ = _e2889;
                                                phi_3262_ = _e1457;
                                            } else {
                                                phi_33689_ = _e1449;
                                                phi_905_ = vec3<f32>();
                                                phi_908_ = i32();
                                                phi_910_ = u32();
                                                phi_3262_ = false;
                                            }
                                            let _e2891 = phi_33689_;
                                            let _e2893 = phi_905_;
                                            let _e2895 = phi_908_;
                                            let _e2897 = phi_910_;
                                            let _e2899 = phi_3262_;
                                            phi_33630_ = _e2891;
                                            phi_904_ = _e2893;
                                            phi_907_ = _e2895;
                                            phi_909_ = _e2897;
                                            phi_33688_ = _e2891;
                                            if !(_e2899) {
                                                break;
                                            }
                                        }
                                        let _e2902 = phi_33688_;
                                        phi_34016_ = _e2902;
                                        if _e2902 {
                                            break;
                                        }
                                        if _e1446 {
                                            let _e2905 = global_6.member[(_e1298 + 10u)];
                                            let _e2908 = global_6.member[(_e1298 + 21u)];
                                            let _e2909 = bitcast<u32>(_e2908);
                                            let _e2912 = global_6.member[(_e1298 + 22u)];
                                            let _e2913 = bitcast<u32>(_e2912);
                                            let _e2916 = global_6.member[(_e1298 + 23u)];
                                            let _e2920 = global_6.member[(_e1298 + 24u)];
                                            let _e2924 = global_6.member[(_e1298 + 25u)];
                                            let _e2928 = global_6.member[(_e1298 + 26u)];
                                            let _e2931 = global_6.member[_e1298];
                                            let _e2934 = global_6.member[(_e1298 + 1u)];
                                            let _e2937 = global_6.member[(_e1298 + 2u)];
                                            let _e2940 = global_6.member[(_e1298 + 3u)];
                                            let _e2943 = global_6.member[(_e1298 + 4u)];
                                            let _e2946 = global_6.member[(_e1298 + 5u)];
                                            let _e2949 = global_6.member[(_e1298 + 6u)];
                                            let _e2952 = global_6.member[(_e1298 + 7u)];
                                            let _e2955 = global_6.member[(_e1298 + 8u)];
                                            let _e2960 = (((_e340.member_1.x * _e2940) + (_e340.member_1.y * _e2943)) + (_e340.member_1.z * _e2946));
                                            let _e2962 = select(_e2960, -1f, (_e2960 < -1f));
                                            let _e2965 = asin(select(_e2962, 1f, (_e2962 > 1f)));
                                            let _e2976 = atan2((((_e340.member_1.x * _e2949) + (_e340.member_1.y * _e2952)) + (_e340.member_1.z * _e2955)), (((_e340.member_1.x * _e2931) + (_e340.member_1.y * _e2934)) + (_e340.member_1.z * _e2937)));
                                            if (_e2909 != 4294967295u) {
                                                let _e2979 = global_6.member[_e2909];
                                                let _e2982 = global_6.member[(_e2909 + 1u)];
                                                let _e2983 = bitcast<u32>(_e2982);
                                                let _e2986 = global_6.member[(_e2909 + 4u)];
                                                phi_3596_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3599_ = 0u;
                                                loop {
                                                    let _e2992 = phi_3596_;
                                                    let _e2994 = phi_3599_;
                                                    local_15 = _e2992;
                                                    let _e2995 = (_e2994 < 6u);
                                                    if _e2995 {
                                                        let _e2996 = (_e2994 / 2u);
                                                        if (_e2996 == 0u) {
                                                            phi_23980_ = _e340.member_1.x;
                                                        } else {
                                                            phi_23980_ = select(_e340.member_1.z, _e340.member_1.y, (_e2996 == 1u));
                                                        }
                                                        let _e3004 = phi_23980_;
                                                        let _e3005 = (_e3004 * select(-1f, 1f, ((_e2994 % 2u) == 0u)));
                                                        if (_e3005 > 0f) {
                                                            let _e3009 = ((_e2996 + 1u) % 3u);
                                                            if (_e3009 == 0u) {
                                                                phi_23995_ = _e340.member_1.x;
                                                            } else {
                                                                phi_23995_ = select(_e340.member_1.z, _e340.member_1.y, (_e3009 == 1u));
                                                            }
                                                            let _e3014 = phi_23995_;
                                                            let _e3015 = (_e3014 / _e3005);
                                                            let _e3017 = ((_e2996 + 2u) % 3u);
                                                            if (_e3017 == 0u) {
                                                                phi_24010_ = _e340.member_1.x;
                                                            } else {
                                                                phi_24010_ = select(_e340.member_1.z, _e340.member_1.y, (_e3017 == 1u));
                                                            }
                                                            let _e3022 = phi_24010_;
                                                            let _e3023 = (_e3022 / _e3005);
                                                            let _e3025 = (1f + (_e2979 * 12f));
                                                            if (abs(_e3015) <= _e3025) {
                                                                if (abs(_e3023) <= _e3025) {
                                                                    let _e3031 = f32(_e2983);
                                                                    let _e3033 = floor((((_e3015 + 1f) * 0.5f) * _e3031));
                                                                    let _e3040 = select(0i, select(select(i32(_e3033), i32(-2147483648), (_e3033 < -2147483600f)), 2147483647i, (_e3033 > 2147483500f)), (_e3033 == _e3033));
                                                                    let _e3042 = (bitcast<i32>(_e2983) - 1i);
                                                                    let _e3043 = (0i <= _e3042);
                                                                    if _e3043 {
                                                                    } else {
                                                                        phi_33693_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3040 < 0i) {
                                                                        phi_3670_ = 0i;
                                                                    } else {
                                                                        phi_3670_ = select(_e3040, _e3042, (_e3040 > _e3042));
                                                                    }
                                                                    let _e3048 = phi_3670_;
                                                                    let _e3052 = floor((((_e3023 + 1f) * 0.5f) * _e3031));
                                                                    let _e3059 = select(0i, select(select(i32(_e3052), i32(-2147483648), (_e3052 < -2147483600f)), 2147483647i, (_e3052 > 2147483500f)), (_e3052 == _e3052));
                                                                    if _e3043 {
                                                                    } else {
                                                                        phi_33693_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3059 < 0i) {
                                                                        phi_3700_ = 0i;
                                                                    } else {
                                                                        phi_3700_ = select(_e3059, _e3042, (_e3059 > _e3042));
                                                                    }
                                                                    let _e3064 = phi_3700_;
                                                                    let _e3065 = (_e3048 - 1i);
                                                                    phi_3708_ = _e2992;
                                                                    phi_3711_ = select(0i, _e3065, (0i < _e3065));
                                                                    loop {
                                                                        let _e3069 = phi_3708_;
                                                                        let _e3071 = phi_3711_;
                                                                        let _e3072 = (_e3048 + 1i);
                                                                        local_59 = _e3069;
                                                                        let _e3075 = (_e3071 <= select(_e3072, _e3042, (_e3042 < _e3072)));
                                                                        if _e3075 {
                                                                            let _e3076 = (_e3064 - 1i);
                                                                            phi_3734_ = _e3069;
                                                                            phi_3737_ = select(0i, _e3076, (0i < _e3076));
                                                                            loop {
                                                                                let _e3080 = phi_3734_;
                                                                                let _e3082 = phi_3737_;
                                                                                let _e3083 = (_e3064 + 1i);
                                                                                local_58 = _e3080;
                                                                                let _e3086 = (_e3082 <= select(_e3083, _e3042, (_e3042 < _e3083)));
                                                                                if _e3086 {
                                                                                    let _e3095 = global_6.member[(bitcast<u32>(_e2986) + ((((_e2994 * _e2983) + bitcast<u32>(_e3071)) * _e2983) + bitcast<u32>(_e3082)))];
                                                                                    phi_3735_ = vec3<f32>((_e3080.x + _e3095), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3738_ = (_e3082 + 1i);
                                                                                } else {
                                                                                    phi_3735_ = vec3<f32>();
                                                                                    phi_3738_ = i32();
                                                                                }
                                                                                let _e3104 = phi_3735_;
                                                                                let _e3106 = phi_3738_;
                                                                                phi_3734_ = _e3104;
                                                                                phi_3737_ = _e3106;
                                                                                if !(_e3086) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6941 = local_58;
                                                                            phi_3709_ = _e6941;
                                                                            phi_3712_ = (_e3071 + 1i);
                                                                        } else {
                                                                            phi_3709_ = vec3<f32>();
                                                                            phi_3712_ = i32();
                                                                        }
                                                                        let _e3110 = phi_3709_;
                                                                        let _e3112 = phi_3712_;
                                                                        phi_3708_ = _e3110;
                                                                        phi_3711_ = _e3112;
                                                                        if !(_e3075) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6945 = local_59;
                                                                    phi_3936_ = _e6945;
                                                                } else {
                                                                    phi_3936_ = _e2992;
                                                                }
                                                                let _e3115 = phi_3936_;
                                                                phi_3937_ = _e3115;
                                                            } else {
                                                                phi_3937_ = _e2992;
                                                            }
                                                            let _e3117 = phi_3937_;
                                                            phi_3938_ = _e3117;
                                                        } else {
                                                            phi_3938_ = _e2992;
                                                        }
                                                        let _e3119 = phi_3938_;
                                                        phi_3597_ = _e3119;
                                                        phi_3600_ = (_e2994 + 1u);
                                                    } else {
                                                        phi_3597_ = vec3<f32>();
                                                        phi_3600_ = u32();
                                                    }
                                                    let _e3122 = phi_3597_;
                                                    let _e3124 = phi_3600_;
                                                    phi_3596_ = _e3122;
                                                    phi_3599_ = _e3124;
                                                    phi_33693_ = _e2902;
                                                    if !(_e2995) {
                                                        break;
                                                    }
                                                }
                                                let _e3127 = phi_33693_;
                                                phi_34016_ = _e3127;
                                                if _e3127 {
                                                    break;
                                                }
                                                let _e3129 = local_15;
                                                phi_33741_ = _e3127;
                                                phi_3949_ = (_e3129.x * (1f / ((6.2831855f * _e2979) * _e2979)));
                                            } else {
                                                phi_33741_ = _e2902;
                                                phi_3949_ = 0f;
                                            }
                                            let _e3133 = phi_33741_;
                                            let _e3135 = phi_3949_;
                                            if (_e2913 != 4294967295u) {
                                                let _e3152 = (_e2965 * 1.3333334f);
                                                let _e3159 = global_6.member[_e2913];
                                                phi_3973_ = (_e3135 + ((_e3159 * ((exp((-(abs(_e2965)) / (0.06f + (abs(_e2976) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2976 * -0.71428573f) * (_e2976 * 0.71428573f)))))) + (1.3f * exp((((_e2976 * _e2976) + (_e3152 * _e3152)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3973_ = _e3135;
                                            }
                                            let _e3165 = phi_3973_;
                                            phi_3975_ = 0u;
                                            phi_3978_ = _e3165;
                                            loop {
                                                let _e3167 = phi_3975_;
                                                let _e3169 = phi_3978_;
                                                local_60 = _e3169;
                                                let _e3170 = (_e3167 < bitcast<u32>(_e2920));
                                                if _e3170 {
                                                    let _e3172 = (bitcast<u32>(_e2916) + (_e3167 * 18u));
                                                    let _e3175 = global_6.member[(_e3172 + 1u)];
                                                    let _e3178 = global_6.member[(_e3172 + 2u)];
                                                    let _e3181 = global_6.member[(_e3172 + 3u)];
                                                    let _e3189 = global_6.member[(_e3172 + 5u)];
                                                    if ((((_e340.member_1.x * _e3175) + (_e340.member_1.y * _e3178)) + (_e340.member_1.z * _e3181)) >= _e3189) {
                                                        let _e3192 = global_6.member[_e3172];
                                                        let _e3197 = select(select(u32(_e3192), 0u, (_e3192 < 0f)), 4294967295u, (_e3192 > 4294967000f));
                                                        if (_e3197 == 0u) {
                                                            phi_4082_ = 1.8f;
                                                        } else {
                                                            if (_e3197 == 1u) {
                                                                phi_4081_ = 1f;
                                                            } else {
                                                                phi_4081_ = select(0f, 4.7f, (_e3197 == 2u));
                                                            }
                                                            let _e3203 = phi_4081_;
                                                            phi_4082_ = _e3203;
                                                        }
                                                        let _e3205 = phi_4082_;
                                                        let _e3208 = global_6.member[(_e3172 + 6u)];
                                                        phi_4098_ = (_e3169 + (_e3208 * _e3205));
                                                    } else {
                                                        phi_4098_ = _e3169;
                                                    }
                                                    let _e3212 = phi_4098_;
                                                    phi_3976_ = (_e3167 + 1u);
                                                    phi_3979_ = _e3212;
                                                } else {
                                                    phi_3976_ = u32();
                                                    phi_3979_ = f32();
                                                }
                                                let _e3215 = phi_3976_;
                                                let _e3217 = phi_3979_;
                                                phi_3975_ = _e3215;
                                                phi_3978_ = _e3217;
                                                if !(_e3170) {
                                                    break;
                                                }
                                            }
                                            phi_4101_ = 0u;
                                            let _e6965 = local_60;
                                            phi_4104_ = _e6965;
                                            loop {
                                                let _e3220 = phi_4101_;
                                                let _e3222 = phi_4104_;
                                                local_16 = _e3222;
                                                let _e3223 = (_e3220 < bitcast<u32>(_e2928));
                                                if _e3223 {
                                                    let _e3225 = (bitcast<u32>(_e2924) + (_e3220 * 29u));
                                                    let _e3228 = global_6.member[(_e3225 + 1u)];
                                                    let _e3231 = global_6.member[(_e3225 + 2u)];
                                                    let _e3234 = global_6.member[(_e3225 + 3u)];
                                                    let _e3242 = global_6.member[(_e3225 + 5u)];
                                                    if ((((_e340.member_1.x * _e3228) + (_e340.member_1.y * _e3231)) + (_e340.member_1.z * _e3234)) >= _e3242) {
                                                        let _e3245 = global_6.member[_e3225];
                                                        let _e3249 = global_6.member[(_e3225 + 6u)];
                                                        phi_4212_ = (_e3222 + (_e3249 * select(1f, 3.3808f, (_e3245 < 0.5f))));
                                                    } else {
                                                        phi_4212_ = _e3222;
                                                    }
                                                    let _e3254 = phi_4212_;
                                                    phi_4102_ = (_e3220 + 1u);
                                                    phi_4105_ = _e3254;
                                                } else {
                                                    phi_4102_ = u32();
                                                    phi_4105_ = f32();
                                                }
                                                let _e3257 = phi_4102_;
                                                let _e3259 = phi_4105_;
                                                phi_4101_ = _e3257;
                                                phi_4104_ = _e3259;
                                                if !(_e3223) {
                                                    break;
                                                }
                                            }
                                            let _e3263 = global_6.member[(_e1298 + 9u)];
                                            let _e3265 = local_16;
                                            let _e3268 = local_17;
                                            let _e3272 = local_18;
                                            let _e3277 = local_19;
                                            let _e3282 = ((_e3265 * _e3263) <= (_e2905 * (((0.2126f * _e3268.x) + (0.7152f * _e3272.y)) + (0.0722f * _e3277.z))));
                                            if _e3282 {
                                                phi_33782_ = _e3133;
                                                phi_4771_ = vec3<f32>();
                                                phi_4772_ = bool();
                                            } else {
                                                let _e3284 = global_6.member[_e1295];
                                                let _e3287 = global_6.member[(_e1295 + 1u)];
                                                let _e3290 = global_6.member[(_e1295 + 2u)];
                                                let _e3293 = global_6.member[(_e1295 + 3u)];
                                                let _e3296 = global_6.member[(_e1295 + 5u)];
                                                let _e3299 = global_6.member[(_e1295 + 6u)];
                                                let _e3302 = global_6.member[(_e1295 + 10u)];
                                                let _e3305 = global_6.member[(_e1295 + 11u)];
                                                let _e3308 = global_6.member[(_e1295 + 13u)];
                                                let _e3311 = global_6.member[(_e1295 + 14u)];
                                                let _e3314 = global_6.member[(_e1295 + 15u)];
                                                let _e3317 = global_6.member[(_e1295 + 16u)];
                                                let _e3320 = global_6.member[(_e1295 + 18u)];
                                                let _e3323 = global_6.member[(_e1295 + 19u)];
                                                let _e3326 = global_6.member[(_e1295 + 20u)];
                                                let _e3338 = ((((_e340.member.x * _e340.member.x) + (_e340.member.y * _e340.member.y)) + (_e340.member.z * _e340.member.z)) - (2f * (((_e340.member.x * _e3284) + (_e340.member.y * _e3287)) + (_e340.member.z * _e3290))));
                                                let _e3339 = (_e3338 + _e3296);
                                                let _e3340 = (_e3339 > 0f);
                                                if _e3340 {
                                                    let _e3348 = ((((_e340.member.x - _e3284) * _e340.member_1.x) + ((_e340.member.y - _e3287) * _e340.member_1.y)) + ((_e340.member.z - _e3290) * _e340.member_1.z));
                                                    let _e3350 = ((_e3348 * _e3348) - _e3339);
                                                    if (_e3350 < 0f) {
                                                        phi_24154_ = type_57(0u, type_57().member_1);
                                                        phi_24155_ = type_57();
                                                        phi_24156_ = true;
                                                    } else {
                                                        if (_e3348 < 0f) {
                                                            phi_24126_ = -(sqrt(_e3350));
                                                        } else {
                                                            phi_24126_ = sqrt(_e3350);
                                                        }
                                                        let _e3357 = phi_24126_;
                                                        let _e3359 = -((_e3348 + _e3357));
                                                        let _e3360 = (_e3359 == 0f);
                                                        if _e3360 {
                                                            phi_24149_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24150_ = type_57();
                                                        } else {
                                                            let _e3361 = (_e3339 / _e3359);
                                                            if (_e3359 < _e3361) {
                                                                phi_24138_ = type_56(_e3359, _e3361);
                                                            } else {
                                                                phi_24138_ = type_56(_e3361, _e3359);
                                                            }
                                                            let _e3366 = phi_24138_;
                                                            phi_24149_ = type_57();
                                                            phi_24150_ = type_57(1u, _e3366);
                                                        }
                                                        let _e3369 = phi_24149_;
                                                        let _e3371 = phi_24150_;
                                                        phi_24154_ = _e3369;
                                                        phi_24155_ = _e3371;
                                                        phi_24156_ = _e3360;
                                                    }
                                                    let _e3376 = phi_24154_;
                                                    let _e3378 = phi_24155_;
                                                    let _e3380 = phi_24156_;
                                                    if _e3380 {
                                                        phi_24160_ = _e3376;
                                                    } else {
                                                        phi_24160_ = _e3378;
                                                    }
                                                    let _e3382 = phi_24160_;
                                                    if (bitcast<i32>(_e3382.member) != 0i) {
                                                        phi_24094_ = select(_e3382.member_1.member_1, _e3382.member_1.member, (_e3382.member_1.member >= 0f));
                                                    } else {
                                                        phi_24094_ = -1f;
                                                    }
                                                    let _e3393 = phi_24094_;
                                                    phi_4613_ = select(true, false, (_e3393 >= 0f));
                                                } else {
                                                    phi_4613_ = true;
                                                }
                                                let _e3397 = phi_4613_;
                                                if _e3397 {
                                                    let _e3405 = ((((_e340.member.x - _e3284) * _e340.member_1.x) + ((_e340.member.y - _e3287) * _e340.member_1.y)) + ((_e340.member.z - _e3290) * _e340.member_1.z));
                                                    let _e3406 = (_e3338 + _e3299);
                                                    let _e3407 = (_e3405 * _e3405);
                                                    let _e3408 = (_e3407 - _e3406);
                                                    if (_e3408 < 0f) {
                                                        phi_24314_ = type_57(0u, type_57().member_1);
                                                        phi_24315_ = type_57();
                                                        phi_24316_ = true;
                                                    } else {
                                                        if (_e3405 < 0f) {
                                                            phi_24286_ = -(sqrt(_e3408));
                                                        } else {
                                                            phi_24286_ = sqrt(_e3408);
                                                        }
                                                        let _e3415 = phi_24286_;
                                                        let _e3417 = -((_e3405 + _e3415));
                                                        let _e3418 = (_e3417 == 0f);
                                                        if _e3418 {
                                                            phi_24309_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24310_ = type_57();
                                                        } else {
                                                            let _e3419 = (_e3406 / _e3417);
                                                            if (_e3417 < _e3419) {
                                                                phi_24298_ = type_56(_e3417, _e3419);
                                                            } else {
                                                                phi_24298_ = type_56(_e3419, _e3417);
                                                            }
                                                            let _e3424 = phi_24298_;
                                                            phi_24309_ = type_57();
                                                            phi_24310_ = type_57(1u, _e3424);
                                                        }
                                                        let _e3427 = phi_24309_;
                                                        let _e3429 = phi_24310_;
                                                        phi_24314_ = _e3427;
                                                        phi_24315_ = _e3429;
                                                        phi_24316_ = _e3418;
                                                    }
                                                    let _e3434 = phi_24314_;
                                                    let _e3436 = phi_24315_;
                                                    let _e3438 = phi_24316_;
                                                    if _e3438 {
                                                        phi_24320_ = _e3434;
                                                    } else {
                                                        phi_24320_ = _e3436;
                                                    }
                                                    let _e3440 = phi_24320_;
                                                    if (bitcast<i32>(_e3440.member) != 0i) {
                                                        phi_24207_ = type_57(0u, _e3440.member_1);
                                                    } else {
                                                        phi_24207_ = type_57(1u, type_57().member_1);
                                                    }
                                                    let _e3450 = phi_24207_;
                                                    if (bitcast<i32>(_e3450.member) != 0i) {
                                                        phi_24254_ = type_57(0u, type_57().member_1);
                                                    } else {
                                                        if (_e3450.member_1.member_1 <= 0f) {
                                                            phi_24250_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e3450.member_1.member != _e3450.member_1.member) {
                                                                phi_24358_ = true;
                                                            } else {
                                                                phi_24358_ = (0f >= _e3450.member_1.member);
                                                            }
                                                            let _e3462 = phi_24358_;
                                                            let _e3463 = select(_e3450.member_1.member, 0f, _e3462);
                                                            if _e3340 {
                                                                let _e3464 = (_e3407 - _e3339);
                                                                if (_e3464 < 0f) {
                                                                    phi_24466_ = type_57(0u, type_57().member_1);
                                                                    phi_24467_ = type_57();
                                                                    phi_24468_ = true;
                                                                } else {
                                                                    if (_e3405 < 0f) {
                                                                        phi_24438_ = -(sqrt(_e3464));
                                                                    } else {
                                                                        phi_24438_ = sqrt(_e3464);
                                                                    }
                                                                    let _e3471 = phi_24438_;
                                                                    let _e3473 = -((_e3405 + _e3471));
                                                                    let _e3474 = (_e3473 == 0f);
                                                                    if _e3474 {
                                                                        phi_24461_ = type_57(1u, type_56(0f, 0f));
                                                                        phi_24462_ = type_57();
                                                                    } else {
                                                                        let _e3475 = (_e3339 / _e3473);
                                                                        if (_e3473 < _e3475) {
                                                                            phi_24450_ = type_56(_e3473, _e3475);
                                                                        } else {
                                                                            phi_24450_ = type_56(_e3475, _e3473);
                                                                        }
                                                                        let _e3480 = phi_24450_;
                                                                        phi_24461_ = type_57();
                                                                        phi_24462_ = type_57(1u, _e3480);
                                                                    }
                                                                    let _e3483 = phi_24461_;
                                                                    let _e3485 = phi_24462_;
                                                                    phi_24466_ = _e3483;
                                                                    phi_24467_ = _e3485;
                                                                    phi_24468_ = _e3474;
                                                                }
                                                                let _e3490 = phi_24466_;
                                                                let _e3492 = phi_24467_;
                                                                let _e3494 = phi_24468_;
                                                                if _e3494 {
                                                                    phi_24472_ = _e3490;
                                                                } else {
                                                                    phi_24472_ = _e3492;
                                                                }
                                                                let _e3496 = phi_24472_;
                                                                if (bitcast<i32>(_e3496.member) != 0i) {
                                                                    phi_24406_ = select(_e3496.member_1.member_1, _e3496.member_1.member, (_e3496.member_1.member >= 0f));
                                                                } else {
                                                                    phi_24406_ = -1f;
                                                                }
                                                                let _e3507 = phi_24406_;
                                                                if (_e3507 >= 0f) {
                                                                    if (_e3450.member_1.member_1 != _e3450.member_1.member_1) {
                                                                        phi_24510_ = true;
                                                                    } else {
                                                                        phi_24510_ = (_e3507 <= _e3450.member_1.member_1);
                                                                    }
                                                                    let _e3512 = phi_24510_;
                                                                    phi_24235_ = select(_e3450.member_1.member_1, _e3507, _e3512);
                                                                } else {
                                                                    phi_24235_ = _e3450.member_1.member_1;
                                                                }
                                                                let _e3515 = phi_24235_;
                                                                phi_24237_ = _e3515;
                                                            } else {
                                                                phi_24237_ = _e3450.member_1.member_1;
                                                            }
                                                            let _e3517 = phi_24237_;
                                                            if (_e3517 <= _e3463) {
                                                                phi_24246_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                phi_24246_ = type_57(1u, type_56(_e3463, _e3517));
                                                            }
                                                            let _e3525 = phi_24246_;
                                                            phi_24250_ = _e3525;
                                                        }
                                                        let _e3530 = phi_24250_;
                                                        phi_24254_ = _e3530;
                                                    }
                                                    let _e3535 = phi_24254_;
                                                    let _e3538 = (bitcast<i32>(_e3535.member) != 0i);
                                                    if _e3538 {
                                                        phi_4627_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4630_ = 0u;
                                                        loop {
                                                            let _e3544 = phi_4627_;
                                                            let _e3546 = phi_4630_;
                                                            local_20 = _e3544;
                                                            local_21 = _e3544;
                                                            local_22 = _e3544;
                                                            let _e3547 = (_e3546 < 32u);
                                                            if _e3547 {
                                                                let _e3548 = -(_e3405);
                                                                if (_e3535.member_1.member <= _e3535.member_1.member_1) {
                                                                } else {
                                                                    phi_33720_ = true;
                                                                    break;
                                                                }
                                                                let _e3551 = select(_e3548, _e3535.member_1.member, (_e3548 < _e3535.member_1.member));
                                                                let _e3553 = select(_e3551, _e3535.member_1.member_1, (_e3551 > _e3535.member_1.member_1));
                                                                let _e3554 = (_e3553 - _e3535.member_1.member);
                                                                let _e3555 = (_e3535.member_1.member_1 - _e3553);
                                                                let _e3556 = (_e3535.member_1.member_1 - _e3535.member_1.member);
                                                                let _e3557 = (0.000000001f * _e3556);
                                                                if (_e3554 <= _e3557) {
                                                                    let _e3632 = (f32(_e3546) * 0.03125f);
                                                                    let _e3635 = (f32((_e3546 + 1u)) * 0.03125f);
                                                                    let _e3637 = ((_e3632 + _e3635) * 0.5f);
                                                                    phi_4731_ = type_56((_e3535.member_1.member + ((_e3556 * _e3637) * _e3637)), (_e3556 * ((_e3635 * _e3635) - (_e3632 * _e3632))));
                                                                } else {
                                                                    if (_e3555 <= _e3557) {
                                                                        let _e3615 = (f32((31u - _e3546)) * 0.03125f);
                                                                        let _e3618 = (f32((32u - _e3546)) * 0.03125f);
                                                                        let _e3620 = ((_e3615 + _e3618) * 0.5f);
                                                                        phi_4730_ = type_56((_e3535.member_1.member_1 - ((_e3556 * _e3620) * _e3620)), (_e3556 * ((_e3618 * _e3618) - (_e3615 * _e3615))));
                                                                    } else {
                                                                        let _e3563 = round(((32f * _e3554) / (_e3554 + _e3555)));
                                                                        let _e3568 = select(select(u32(_e3563), 0u, (_e3563 < 0f)), 4294967295u, (_e3563 > 4294967000f));
                                                                        if (_e3568 < 1u) {
                                                                            phi_4714_ = 1u;
                                                                        } else {
                                                                            phi_4714_ = select(_e3568, 31u, (_e3568 > 31u));
                                                                        }
                                                                        let _e3573 = phi_4714_;
                                                                        if (_e3546 < _e3573) {
                                                                            let _e3594 = ((_e3573 - 1u) - _e3546);
                                                                            let _e3596 = f32(_e3573);
                                                                            let _e3597 = (f32(_e3594) / _e3596);
                                                                            let _e3600 = (f32((_e3594 + 1u)) / _e3596);
                                                                            let _e3602 = ((_e3597 + _e3600) * 0.5f);
                                                                            phi_4729_ = type_56((_e3553 - ((_e3554 * _e3602) * _e3602)), (_e3554 * ((_e3600 * _e3600) - (_e3597 * _e3597))));
                                                                        } else {
                                                                            let _e3576 = (_e3546 - _e3573);
                                                                            let _e3578 = f32((32u - _e3573));
                                                                            let _e3579 = (f32(_e3576) / _e3578);
                                                                            let _e3582 = (f32((_e3576 + 1u)) / _e3578);
                                                                            let _e3584 = ((_e3579 + _e3582) * 0.5f);
                                                                            phi_4729_ = type_56((_e3553 + ((_e3555 * _e3584) * _e3584)), (_e3555 * ((_e3582 * _e3582) - (_e3579 * _e3579))));
                                                                        }
                                                                        let _e3612 = phi_4729_;
                                                                        phi_4730_ = _e3612;
                                                                    }
                                                                    let _e3630 = phi_4730_;
                                                                    phi_4731_ = _e3630;
                                                                }
                                                                let _e3647 = phi_4731_;
                                                                let _e3653 = (_e340.member.x + (_e340.member_1.x * _e3647.member));
                                                                let _e3654 = (_e340.member.y + (_e340.member_1.y * _e3647.member));
                                                                let _e3655 = (_e340.member.z + (_e340.member_1.z * _e3647.member));
                                                                let _e3669 = (_e3653 - _e3284);
                                                                let _e3670 = (_e3654 - _e3287);
                                                                let _e3671 = (_e3655 - _e3290);
                                                                let _e3679 = ((((((_e3653 * _e3653) + (_e3654 * _e3654)) + (_e3655 * _e3655)) - (2f * (((_e3653 * _e3284) + (_e3654 * _e3287)) + (_e3655 * _e3290)))) + _e3296) / (sqrt((((_e3669 * _e3669) + (_e3670 * _e3670)) + (_e3671 * _e3671))) + _e3293));
                                                                let _e3683 = (1f - (abs((_e3679 - 25000f)) * 0.00006666667f));
                                                                if (_e3683 != _e3683) {
                                                                    phi_24709_ = true;
                                                                } else {
                                                                    phi_24709_ = (0f >= _e3683);
                                                                }
                                                                let _e3687 = phi_24709_;
                                                                let _e3689 = -(_e3679);
                                                                phi_4628_ = (_e3544 + vec3<f32>((exp((_e3689 / _e3302)) * _e3647.member_1), (exp((_e3689 / _e3305)) * _e3647.member_1), (select(_e3683, 0f, _e3687) * _e3647.member_1)));
                                                                phi_4631_ = (_e3546 + 1u);
                                                            } else {
                                                                phi_4628_ = vec3<f32>();
                                                                phi_4631_ = u32();
                                                            }
                                                            let _e3701 = phi_4628_;
                                                            let _e3703 = phi_4631_;
                                                            phi_4627_ = _e3701;
                                                            phi_4630_ = _e3703;
                                                            phi_33720_ = _e3133;
                                                            if !(_e3547) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3706 = phi_33720_;
                                                        phi_34016_ = _e3706;
                                                        if _e3706 {
                                                            break;
                                                        }
                                                        let _e3708 = local_20;
                                                        let _e3715 = local_21;
                                                        let _e3717 = ((_e3317 * 1.1f) * _e3715.y);
                                                        let _e3722 = local_22;
                                                        phi_33787_ = _e3706;
                                                        phi_4756_ = vec3<f32>(exp(-((((_e3308 * _e3708.x) + _e3717) + (_e3320 * _e3722.z)))), exp(-((((_e3311 * _e3708.x) + _e3717) + (_e3323 * _e3722.z)))), exp(-((((_e3314 * _e3708.x) + _e3717) + (_e3326 * _e3722.z)))));
                                                    } else {
                                                        phi_33787_ = _e3133;
                                                        phi_4756_ = vec3<f32>();
                                                    }
                                                    let _e3738 = phi_33787_;
                                                    let _e3740 = phi_4756_;
                                                    phi_33786_ = _e3738;
                                                    phi_4758_ = _e3740;
                                                    phi_4760_ = select(true, false, _e3538);
                                                } else {
                                                    phi_33786_ = _e3133;
                                                    phi_4758_ = vec3<f32>();
                                                    phi_4760_ = true;
                                                }
                                                let _e3743 = phi_33786_;
                                                let _e3745 = phi_4758_;
                                                let _e3747 = phi_4760_;
                                                let _e3751 = select(_e3745, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3397)), vec3(_e3747));
                                                let _e3755 = select(_e3751.y, _e3751.x, (_e3751.x > _e3751.y));
                                                phi_33782_ = _e3743;
                                                phi_4771_ = _e3751;
                                                phi_4772_ = (select(_e3751.z, _e3755, (_e3755 > _e3751.z)) > 0f);
                                            }
                                            let _e3761 = phi_33782_;
                                            let _e3763 = phi_4771_;
                                            let _e3765 = phi_4772_;
                                            phi_33781_ = _e3761;
                                            phi_4774_ = _e3763;
                                            phi_4775_ = _e3765;
                                            phi_4776_ = select(true, false, _e3282);
                                        } else {
                                            phi_33781_ = _e2902;
                                            phi_4774_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4775_ = _e1446;
                                            phi_4776_ = true;
                                        }
                                        let _e3768 = phi_33781_;
                                        let _e3770 = phi_4774_;
                                        let _e3772 = phi_4775_;
                                        let _e3774 = phi_4776_;
                                        phi_33780_ = _e3768;
                                        let _e7036 = local_61;
                                        phi_4780_ = _e7036;
                                        phi_4781_ = _e3770;
                                        phi_4782_ = _e3772;
                                        let _e7041 = local_62;
                                        phi_4783_ = _e7041;
                                        phi_4784_ = _e3774;
                                    } else {
                                        phi_33780_ = _e329;
                                        phi_4780_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4781_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4782_ = _e1446;
                                        phi_4783_ = vec3<f32>();
                                        phi_4784_ = true;
                                    }
                                    let _e3776 = phi_33780_;
                                    let _e3778 = phi_4780_;
                                    let _e3780 = phi_4781_;
                                    let _e3782 = phi_4782_;
                                    let _e3784 = phi_4783_;
                                    let _e3786 = phi_4784_;
                                    if _e3786 {
                                        if _e3782 {
                                            let _e3787 = (_e1298 + 20u);
                                            let _e3790 = global_6.member[(_e1298 + 21u)];
                                            let _e3791 = bitcast<u32>(_e3790);
                                            let _e3794 = global_6.member[(_e1298 + 22u)];
                                            let _e3795 = bitcast<u32>(_e3794);
                                            let _e3798 = global_6.member[(_e1298 + 23u)];
                                            let _e3802 = global_6.member[(_e1298 + 24u)];
                                            let _e3806 = global_6.member[(_e1298 + 25u)];
                                            let _e3810 = global_6.member[(_e1298 + 26u)];
                                            let _e3813 = global_6.member[_e1298];
                                            let _e3816 = global_6.member[(_e1298 + 1u)];
                                            let _e3819 = global_6.member[(_e1298 + 2u)];
                                            let _e3822 = global_6.member[(_e1298 + 3u)];
                                            let _e3825 = global_6.member[(_e1298 + 4u)];
                                            let _e3828 = global_6.member[(_e1298 + 5u)];
                                            let _e3831 = global_6.member[(_e1298 + 6u)];
                                            let _e3834 = global_6.member[(_e1298 + 7u)];
                                            let _e3837 = global_6.member[(_e1298 + 8u)];
                                            let _e3842 = (((_e340.member_1.x * _e3822) + (_e340.member_1.y * _e3825)) + (_e340.member_1.z * _e3828));
                                            let _e3844 = select(_e3842, -1f, (_e3842 < -1f));
                                            let _e3847 = asin(select(_e3844, 1f, (_e3844 > 1f)));
                                            let _e3858 = atan2((((_e340.member_1.x * _e3831) + (_e340.member_1.y * _e3834)) + (_e340.member_1.z * _e3837)), (((_e340.member_1.x * _e3813) + (_e340.member_1.y * _e3816)) + (_e340.member_1.z * _e3819)));
                                            if (_e3791 != 4294967295u) {
                                                let _e3861 = global_6.member[_e3791];
                                                let _e3864 = global_6.member[(_e3791 + 1u)];
                                                let _e3865 = bitcast<u32>(_e3864);
                                                let _e3868 = global_6.member[(_e3791 + 2u)];
                                                let _e3872 = global_6.member[(_e3791 + 3u)];
                                                let _e3874 = (4f * _e3861);
                                                phi_5104_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5107_ = 0u;
                                                loop {
                                                    let _e3879 = phi_5104_;
                                                    let _e3881 = phi_5107_;
                                                    local_66 = _e3879;
                                                    let _e3882 = (_e3881 < 6u);
                                                    if _e3882 {
                                                        let _e3883 = (_e3881 / 2u);
                                                        if (_e3883 == 0u) {
                                                            phi_24780_ = _e340.member_1.x;
                                                        } else {
                                                            phi_24780_ = select(_e340.member_1.z, _e340.member_1.y, (_e3883 == 1u));
                                                        }
                                                        let _e3891 = phi_24780_;
                                                        let _e3892 = (_e3891 * select(-1f, 1f, ((_e3881 % 2u) == 0u)));
                                                        if (_e3892 > 0f) {
                                                            let _e3896 = ((_e3883 + 1u) % 3u);
                                                            if (_e3896 == 0u) {
                                                                phi_24795_ = _e340.member_1.x;
                                                            } else {
                                                                phi_24795_ = select(_e340.member_1.z, _e340.member_1.y, (_e3896 == 1u));
                                                            }
                                                            let _e3901 = phi_24795_;
                                                            let _e3902 = (_e3901 / _e3892);
                                                            let _e3904 = ((_e3883 + 2u) % 3u);
                                                            if (_e3904 == 0u) {
                                                                phi_24810_ = _e340.member_1.x;
                                                            } else {
                                                                phi_24810_ = select(_e340.member_1.z, _e340.member_1.y, (_e3904 == 1u));
                                                            }
                                                            let _e3909 = phi_24810_;
                                                            let _e3910 = (_e3909 / _e3892);
                                                            let _e3912 = (1f + (_e3861 * 12f));
                                                            if (abs(_e3902) <= _e3912) {
                                                                if (abs(_e3910) <= _e3912) {
                                                                    let _e3918 = f32(_e3865);
                                                                    let _e3920 = floor((((_e3902 + 1f) * 0.5f) * _e3918));
                                                                    let _e3927 = select(0i, select(select(i32(_e3920), i32(-2147483648), (_e3920 < -2147483600f)), 2147483647i, (_e3920 > 2147483500f)), (_e3920 == _e3920));
                                                                    let _e3929 = (bitcast<i32>(_e3865) - 1i);
                                                                    let _e3930 = (0i <= _e3929);
                                                                    if _e3930 {
                                                                    } else {
                                                                        phi_33775_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3927 < 0i) {
                                                                        phi_5178_ = 0i;
                                                                    } else {
                                                                        phi_5178_ = select(_e3927, _e3929, (_e3927 > _e3929));
                                                                    }
                                                                    let _e3935 = phi_5178_;
                                                                    let _e3939 = floor((((_e3910 + 1f) * 0.5f) * _e3918));
                                                                    let _e3946 = select(0i, select(select(i32(_e3939), i32(-2147483648), (_e3939 < -2147483600f)), 2147483647i, (_e3939 > 2147483500f)), (_e3939 == _e3939));
                                                                    if _e3930 {
                                                                    } else {
                                                                        phi_33775_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3946 < 0i) {
                                                                        phi_5208_ = 0i;
                                                                    } else {
                                                                        phi_5208_ = select(_e3946, _e3929, (_e3946 > _e3929));
                                                                    }
                                                                    let _e3951 = phi_5208_;
                                                                    let _e3952 = (_e3935 - 1i);
                                                                    phi_5216_ = _e3879;
                                                                    phi_5219_ = select(0i, _e3952, (0i < _e3952));
                                                                    loop {
                                                                        let _e3956 = phi_5216_;
                                                                        let _e3958 = phi_5219_;
                                                                        let _e3959 = (_e3935 + 1i);
                                                                        local_65 = _e3956;
                                                                        let _e3962 = (_e3958 <= select(_e3959, _e3929, (_e3929 < _e3959)));
                                                                        if _e3962 {
                                                                            let _e3963 = (_e3951 - 1i);
                                                                            phi_5242_ = _e3956;
                                                                            phi_5245_ = select(0i, _e3963, (0i < _e3963));
                                                                            loop {
                                                                                let _e3967 = phi_5242_;
                                                                                let _e3969 = phi_5245_;
                                                                                let _e3970 = (_e3951 + 1i);
                                                                                local_64 = _e3967;
                                                                                let _e3973 = (_e3969 <= select(_e3970, _e3929, (_e3929 < _e3970)));
                                                                                if _e3973 {
                                                                                    let _e3980 = (bitcast<u32>(_e3868) + ((((_e3881 * _e3865) + bitcast<u32>(_e3958)) * _e3865) + bitcast<u32>(_e3969)));
                                                                                    let _e3982 = global_6.member[_e3980];
                                                                                    let _e3986 = global_6.member[(_e3980 + 1u)];
                                                                                    phi_5316_ = _e3967;
                                                                                    phi_5319_ = bitcast<u32>(_e3982);
                                                                                    loop {
                                                                                        let _e3989 = phi_5316_;
                                                                                        let _e3991 = phi_5319_;
                                                                                        local_63 = _e3989;
                                                                                        let _e3992 = (_e3991 < bitcast<u32>(_e3986));
                                                                                        if _e3992 {
                                                                                            let _e3994 = (bitcast<u32>(_e3872) + (6u * _e3991));
                                                                                            let _e3996 = global_6.member[_e3994];
                                                                                            let _e3999 = global_6.member[(_e3994 + 1u)];
                                                                                            let _e4002 = global_6.member[(_e3994 + 2u)];
                                                                                            let _e4005 = global_6.member[(_e3994 + 3u)];
                                                                                            let _e4008 = global_6.member[(_e3994 + 4u)];
                                                                                            let _e4011 = global_6.member[(_e3994 + 5u)];
                                                                                            let _e4012 = (_e340.member_1.x - _e3996);
                                                                                            let _e4013 = (_e340.member_1.y - _e3999);
                                                                                            let _e4014 = (_e340.member_1.z - _e4002);
                                                                                            let _e4019 = (((_e4012 * _e4012) + (_e4013 * _e4013)) + (_e4014 * _e4014));
                                                                                            if (_e4019 < (_e3874 * _e3874)) {
                                                                                                let _e4027 = ((1f / ((6.2831855f * _e3861) * _e3861)) * exp((-(_e4019) / ((2f * _e3861) * _e3861))));
                                                                                                phi_5439_ = vec3<f32>((_e3989.x + (_e4005 * _e4027)), (_e3989.y + (_e4008 * _e4027)), (_e3989.z + (_e4011 * _e4027)));
                                                                                            } else {
                                                                                                phi_5439_ = _e3989;
                                                                                            }
                                                                                            let _e4039 = phi_5439_;
                                                                                            phi_5317_ = _e4039;
                                                                                            phi_5320_ = (_e3991 + 1u);
                                                                                        } else {
                                                                                            phi_5317_ = vec3<f32>();
                                                                                            phi_5320_ = u32();
                                                                                        }
                                                                                        let _e4042 = phi_5317_;
                                                                                        let _e4044 = phi_5320_;
                                                                                        phi_5316_ = _e4042;
                                                                                        phi_5319_ = _e4044;
                                                                                        if !(_e3992) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7061 = local_63;
                                                                                    phi_5243_ = _e7061;
                                                                                    phi_5246_ = (_e3969 + 1i);
                                                                                } else {
                                                                                    phi_5243_ = vec3<f32>();
                                                                                    phi_5246_ = i32();
                                                                                }
                                                                                let _e4048 = phi_5243_;
                                                                                let _e4050 = phi_5246_;
                                                                                phi_5242_ = _e4048;
                                                                                phi_5245_ = _e4050;
                                                                                if !(_e3973) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7065 = local_64;
                                                                            phi_5217_ = _e7065;
                                                                            phi_5220_ = (_e3958 + 1i);
                                                                        } else {
                                                                            phi_5217_ = vec3<f32>();
                                                                            phi_5220_ = i32();
                                                                        }
                                                                        let _e4054 = phi_5217_;
                                                                        let _e4056 = phi_5220_;
                                                                        phi_5216_ = _e4054;
                                                                        phi_5219_ = _e4056;
                                                                        if !(_e3962) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7069 = local_65;
                                                                    phi_5444_ = _e7069;
                                                                } else {
                                                                    phi_5444_ = _e3879;
                                                                }
                                                                let _e4059 = phi_5444_;
                                                                phi_5445_ = _e4059;
                                                            } else {
                                                                phi_5445_ = _e3879;
                                                            }
                                                            let _e4061 = phi_5445_;
                                                            phi_5446_ = _e4061;
                                                        } else {
                                                            phi_5446_ = _e3879;
                                                        }
                                                        let _e4063 = phi_5446_;
                                                        phi_5105_ = _e4063;
                                                        phi_5108_ = (_e3881 + 1u);
                                                    } else {
                                                        phi_5105_ = vec3<f32>();
                                                        phi_5108_ = u32();
                                                    }
                                                    let _e4066 = phi_5105_;
                                                    let _e4068 = phi_5108_;
                                                    phi_5104_ = _e4066;
                                                    phi_5107_ = _e4068;
                                                    phi_33775_ = _e3776;
                                                    if !(_e3882) {
                                                        break;
                                                    }
                                                }
                                                let _e4071 = phi_33775_;
                                                phi_34016_ = _e4071;
                                                if _e4071 {
                                                    break;
                                                }
                                                phi_33817_ = _e4071;
                                                let _e7078 = local_66;
                                                phi_5466_ = _e7078;
                                            } else {
                                                phi_33817_ = _e3776;
                                                phi_5466_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4073 = phi_33817_;
                                            let _e4075 = phi_5466_;
                                            if (_e3795 != 4294967295u) {
                                                let _e4078 = global_6.member[_e3795];
                                                let _e4081 = global_6.member[(_e3795 + 1u)];
                                                let _e4084 = global_6.member[(_e3795 + 2u)];
                                                let _e4087 = global_6.member[(_e3795 + 3u)];
                                                let _e4090 = global_6.member[(_e3795 + 4u)];
                                                let _e4093 = global_6.member[(_e3795 + 5u)];
                                                let _e4096 = global_6.member[(_e3795 + 6u)];
                                                let _e4099 = global_6.member[(_e3795 + 7u)];
                                                let _e4102 = global_6.member[(_e3795 + 8u)];
                                                let _e4105 = global_6.member[(_e3795 + 9u)];
                                                let _e4108 = global_6.member[(_e3795 + 10u)];
                                                let _e4112 = global_6.member[(_e3795 + 11u)];
                                                let _e4115 = global_6.member[(_e3795 + 12u)];
                                                let _e4118 = global_6.member[(_e3795 + 13u)];
                                                let _e4120 = (_e4078 > 0f);
                                                let _e4121 = (_e4081 > 0f);
                                                if _e4121 {
                                                    let _e4124 = (abs(_e3847) <= 0.8f);
                                                    let _e4125 = select(2u, 0u, _e4120);
                                                    phi_5679_ = _e4125;
                                                    phi_5680_ = _e4124;
                                                    phi_5683_ = _e4125;
                                                    phi_5684_ = _e4124;
                                                    phi_5685_ = select(true, false, _e4124);
                                                } else {
                                                    phi_5679_ = u32();
                                                    phi_5680_ = bool();
                                                    phi_5683_ = select(2u, 0u, _e4120);
                                                    phi_5684_ = false;
                                                    phi_5685_ = true;
                                                }
                                                let _e4128 = phi_5679_;
                                                let _e4130 = phi_5680_;
                                                let _e4132 = phi_5683_;
                                                let _e4134 = phi_5684_;
                                                let _e4136 = phi_5685_;
                                                let _e4138 = select(select(f32(), 0f, _e4121), 0f, _e4136);
                                                phi_5695_ = _e4138;
                                                phi_5698_ = _e4138;
                                                phi_5700_ = select(_e4128, _e4132, _e4136);
                                                phi_5702_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5704_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5706_ = 0f;
                                                phi_5708_ = 1f;
                                                loop {
                                                    let _e4143 = phi_5695_;
                                                    let _e4145 = phi_5698_;
                                                    let _e4147 = phi_5700_;
                                                    let _e4149 = phi_5702_;
                                                    let _e4151 = phi_5704_;
                                                    let _e4153 = phi_5706_;
                                                    let _e4155 = phi_5708_;
                                                    local_26 = _e4155;
                                                    local_27 = _e4155;
                                                    local_28 = _e4155;
                                                    local_29 = _e4155;
                                                    local_30 = _e4153;
                                                    local_31 = _e4145;
                                                    local_32 = _e4143;
                                                    let _e4156 = (_e4147 < select(11u, 2u, _e4136));
                                                    if _e4156 {
                                                        let _e4157 = (_e4147 < 2u);
                                                        if _e4157 {
                                                            let _e4213 = select(23f, 4f, (_e4147 == 0u));
                                                            phi_5786_ = 4u;
                                                            phi_5788_ = vec3<f32>((_e340.member_1.x * _e4213), (_e340.member_1.y * _e4213), (_e340.member_1.z * _e4213));
                                                        } else {
                                                            if (_e4147 == 2u) {
                                                                phi_5784_ = 3u;
                                                                phi_5785_ = vec3<f32>(((_e340.member_1.x * 1.5f) + 5f), ((_e340.member_1.y * 1.5f) + 5f), ((_e340.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4159 = (_e4147 >= 7u);
                                                                if _e4159 {
                                                                    phi_5746_ = (_e4147 - 7u);
                                                                } else {
                                                                    phi_5746_ = (_e4147 - 3u);
                                                                }
                                                                let _e4163 = phi_5746_;
                                                                if _e4159 {
                                                                    phi_5761_ = vec3<f32>(((_e340.member_1.x * 11f) + -3f), ((_e340.member_1.y * 11f) + 1f), ((_e340.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5761_ = vec3<f32>((_e340.member_1.x * 7f), (_e340.member_1.y * 7f), (_e340.member_1.z * 7f));
                                                                }
                                                                let _e4176 = phi_5761_;
                                                                if (_e4163 < 3u) {
                                                                    if (_e4163 == 0u) {
                                                                        phi_24825_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_24825_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4163 == 1u)));
                                                                    }
                                                                    let _e4195 = phi_24825_;
                                                                    phi_5782_ = 4u;
                                                                    phi_5783_ = (_e4176 + _e4195);
                                                                } else {
                                                                    if _e4159 {
                                                                        phi_5781_ = (_e4176 + _e4149);
                                                                    } else {
                                                                        phi_5781_ = (_e4176 + vec3<f32>((_e4151.x * 1.1f), (_e4151.y * 1.1f), (_e4151.z * 1.1f)));
                                                                    }
                                                                    let _e4188 = phi_5781_;
                                                                    phi_5782_ = select(7u, 5u, _e4159);
                                                                    phi_5783_ = _e4188;
                                                                }
                                                                let _e4198 = phi_5782_;
                                                                let _e4200 = phi_5783_;
                                                                phi_5784_ = _e4198;
                                                                phi_5785_ = _e4200;
                                                            }
                                                            let _e4209 = phi_5784_;
                                                            let _e4211 = phi_5785_;
                                                            phi_5786_ = _e4209;
                                                            phi_5788_ = _e4211;
                                                        }
                                                        let _e4219 = phi_5786_;
                                                        let _e4221 = phi_5788_;
                                                        let _e4223 = select(vec3<f32>(_e4112, _e4115, _e4118), vec3<f32>(_e4102, _e4105, _e4108), vec3(_e4157));
                                                        let _e4225 = global_6.member[_e3787];
                                                        let _e4226 = bitcast<u32>(_e4225);
                                                        let _e4229 = global_6.member[(_e1298 + 11u)];
                                                        let _e4232 = global_6.member[(_e1298 + 12u)];
                                                        let _e4235 = global_6.member[(_e1298 + 13u)];
                                                        let _e4238 = global_6.member[(_e1298 + 14u)];
                                                        let _e4241 = global_6.member[(_e1298 + 15u)];
                                                        let _e4244 = global_6.member[(_e1298 + 16u)];
                                                        let _e4247 = global_6.member[(_e1298 + 17u)];
                                                        let _e4250 = global_6.member[(_e1298 + 18u)];
                                                        let _e4253 = global_6.member[(_e1298 + 19u)];
                                                        phi_5923_ = 0u;
                                                        phi_5926_ = 0f;
                                                        phi_5928_ = 1f;
                                                        phi_5930_ = 0f;
                                                        phi_5932_ = _e4221;
                                                        loop {
                                                            let _e4255 = phi_5923_;
                                                            let _e4257 = phi_5926_;
                                                            let _e4259 = phi_5928_;
                                                            let _e4261 = phi_5930_;
                                                            let _e4263 = phi_5932_;
                                                            local_23 = _e4257;
                                                            local_24 = _e4261;
                                                            local_25 = _e4257;
                                                            let _e4264 = (_e4255 < _e4219);
                                                            if _e4264 {
                                                                let _e4267 = (_e4263.x + _e4223.x);
                                                                let _e4270 = (_e4263.y + _e4223.y);
                                                                let _e4273 = (_e4263.z + _e4223.z);
                                                                let _e4274 = floor(_e4267);
                                                                let _e4275 = floor(_e4270);
                                                                let _e4276 = floor(_e4273);
                                                                let _e4285 = bitcast<u32>((select(0i, select(select(i32(_e4274), i32(-2147483648), (_e4274 < -2147483600f)), 2147483647i, (_e4274 > 2147483500f)), (_e4274 == _e4274)) & 255i));
                                                                let _e4294 = bitcast<u32>((select(0i, select(select(i32(_e4275), i32(-2147483648), (_e4275 < -2147483600f)), 2147483647i, (_e4275 > 2147483500f)), (_e4275 == _e4275)) & 255i));
                                                                let _e4303 = bitcast<u32>((select(0i, select(select(i32(_e4276), i32(-2147483648), (_e4276 < -2147483600f)), 2147483647i, (_e4276 > 2147483500f)), (_e4276 == _e4276)) & 255i));
                                                                let _e4304 = (_e4267 - _e4274);
                                                                let _e4305 = (_e4270 - _e4275);
                                                                let _e4306 = (_e4273 - _e4276);
                                                                let _e4313 = (((_e4304 * _e4304) * _e4304) * ((_e4304 * ((_e4304 * 6f) - 15f)) + 10f));
                                                                let _e4320 = (((_e4305 * _e4305) * _e4305) * ((_e4305 * ((_e4305 * 6f) - 15f)) + 10f));
                                                                let _e4330 = global_6.member[(_e4226 + _e4285)];
                                                                let _e4336 = (select(select(u32(_e4330), 0u, (_e4330 < 0f)), 4294967295u, (_e4330 > 4294967000f)) + _e4294);
                                                                let _e4339 = global_6.member[(_e4226 + _e4336)];
                                                                let _e4345 = (select(select(u32(_e4339), 0u, (_e4339 < 0f)), 4294967295u, (_e4339 > 4294967000f)) + _e4303);
                                                                let _e4349 = global_6.member[(_e4226 + (_e4336 + 1u))];
                                                                let _e4355 = (select(select(u32(_e4349), 0u, (_e4349 < 0f)), 4294967295u, (_e4349 > 4294967000f)) + _e4303);
                                                                let _e4359 = global_6.member[(_e4226 + (_e4285 + 1u))];
                                                                let _e4365 = (select(select(u32(_e4359), 0u, (_e4359 < 0f)), 4294967295u, (_e4359 > 4294967000f)) + _e4294);
                                                                let _e4368 = global_6.member[(_e4226 + _e4365)];
                                                                let _e4374 = (select(select(u32(_e4368), 0u, (_e4368 < 0f)), 4294967295u, (_e4368 > 4294967000f)) + _e4303);
                                                                let _e4378 = global_6.member[(_e4226 + (_e4365 + 1u))];
                                                                let _e4384 = (select(select(u32(_e4378), 0u, (_e4378 < 0f)), 4294967295u, (_e4378 > 4294967000f)) + _e4303);
                                                                let _e4387 = global_6.member[(_e4226 + _e4345)];
                                                                let _e4393 = (select(select(u32(_e4387), 0u, (_e4387 < 0f)), 4294967295u, (_e4387 > 4294967000f)) % 12u);
                                                                let _e4396 = select(-1f, 1f, ((_e4393 & 1u) == 0u));
                                                                let _e4399 = select(-1f, 1f, ((_e4393 & 2u) == 0u));
                                                                if (_e4393 < 4u) {
                                                                    phi_24886_ = ((_e4396 * _e4304) + (_e4399 * _e4305));
                                                                } else {
                                                                    if (_e4393 < 8u) {
                                                                        phi_24880_ = ((_e4396 * _e4304) + (_e4399 * _e4306));
                                                                    } else {
                                                                        phi_24880_ = ((_e4396 * _e4305) + (_e4399 * _e4306));
                                                                    }
                                                                    let _e4409 = phi_24880_;
                                                                    phi_24886_ = _e4409;
                                                                }
                                                                let _e4414 = phi_24886_;
                                                                let _e4417 = global_6.member[(_e4226 + _e4374)];
                                                                let _e4423 = (_e4304 - 1f);
                                                                let _e4424 = (select(select(u32(_e4417), 0u, (_e4417 < 0f)), 4294967295u, (_e4417 > 4294967000f)) % 12u);
                                                                let _e4427 = select(-1f, 1f, ((_e4424 & 1u) == 0u));
                                                                let _e4430 = select(-1f, 1f, ((_e4424 & 2u) == 0u));
                                                                if (_e4424 < 4u) {
                                                                    phi_24920_ = ((_e4427 * _e4423) + (_e4430 * _e4305));
                                                                } else {
                                                                    if (_e4424 < 8u) {
                                                                        phi_24914_ = ((_e4427 * _e4423) + (_e4430 * _e4306));
                                                                    } else {
                                                                        phi_24914_ = ((_e4427 * _e4305) + (_e4430 * _e4306));
                                                                    }
                                                                    let _e4440 = phi_24914_;
                                                                    phi_24920_ = _e4440;
                                                                }
                                                                let _e4445 = phi_24920_;
                                                                let _e4448 = global_6.member[(_e4226 + _e4355)];
                                                                let _e4454 = (_e4305 - 1f);
                                                                let _e4455 = (select(select(u32(_e4448), 0u, (_e4448 < 0f)), 4294967295u, (_e4448 > 4294967000f)) % 12u);
                                                                let _e4458 = select(-1f, 1f, ((_e4455 & 1u) == 0u));
                                                                let _e4461 = select(-1f, 1f, ((_e4455 & 2u) == 0u));
                                                                if (_e4455 < 4u) {
                                                                    phi_24954_ = ((_e4458 * _e4304) + (_e4461 * _e4454));
                                                                } else {
                                                                    if (_e4455 < 8u) {
                                                                        phi_24948_ = ((_e4458 * _e4304) + (_e4461 * _e4306));
                                                                    } else {
                                                                        phi_24948_ = ((_e4458 * _e4454) + (_e4461 * _e4306));
                                                                    }
                                                                    let _e4471 = phi_24948_;
                                                                    phi_24954_ = _e4471;
                                                                }
                                                                let _e4476 = phi_24954_;
                                                                let _e4479 = global_6.member[(_e4226 + _e4384)];
                                                                let _e4485 = (select(select(u32(_e4479), 0u, (_e4479 < 0f)), 4294967295u, (_e4479 > 4294967000f)) % 12u);
                                                                let _e4488 = select(-1f, 1f, ((_e4485 & 1u) == 0u));
                                                                let _e4491 = select(-1f, 1f, ((_e4485 & 2u) == 0u));
                                                                if (_e4485 < 4u) {
                                                                    phi_24988_ = ((_e4488 * _e4423) + (_e4491 * _e4454));
                                                                } else {
                                                                    if (_e4485 < 8u) {
                                                                        phi_24982_ = ((_e4488 * _e4423) + (_e4491 * _e4306));
                                                                    } else {
                                                                        phi_24982_ = ((_e4488 * _e4454) + (_e4491 * _e4306));
                                                                    }
                                                                    let _e4501 = phi_24982_;
                                                                    phi_24988_ = _e4501;
                                                                }
                                                                let _e4506 = phi_24988_;
                                                                let _e4510 = global_6.member[(_e4226 + (_e4345 + 1u))];
                                                                let _e4516 = (_e4306 - 1f);
                                                                let _e4517 = (select(select(u32(_e4510), 0u, (_e4510 < 0f)), 4294967295u, (_e4510 > 4294967000f)) % 12u);
                                                                let _e4520 = select(-1f, 1f, ((_e4517 & 1u) == 0u));
                                                                let _e4523 = select(-1f, 1f, ((_e4517 & 2u) == 0u));
                                                                if (_e4517 < 4u) {
                                                                    phi_25022_ = ((_e4520 * _e4304) + (_e4523 * _e4305));
                                                                } else {
                                                                    if (_e4517 < 8u) {
                                                                        phi_25016_ = ((_e4520 * _e4304) + (_e4523 * _e4516));
                                                                    } else {
                                                                        phi_25016_ = ((_e4520 * _e4305) + (_e4523 * _e4516));
                                                                    }
                                                                    let _e4533 = phi_25016_;
                                                                    phi_25022_ = _e4533;
                                                                }
                                                                let _e4538 = phi_25022_;
                                                                let _e4542 = global_6.member[(_e4226 + (_e4374 + 1u))];
                                                                let _e4548 = (select(select(u32(_e4542), 0u, (_e4542 < 0f)), 4294967295u, (_e4542 > 4294967000f)) % 12u);
                                                                let _e4551 = select(-1f, 1f, ((_e4548 & 1u) == 0u));
                                                                let _e4554 = select(-1f, 1f, ((_e4548 & 2u) == 0u));
                                                                if (_e4548 < 4u) {
                                                                    phi_25056_ = ((_e4551 * _e4423) + (_e4554 * _e4305));
                                                                } else {
                                                                    if (_e4548 < 8u) {
                                                                        phi_25050_ = ((_e4551 * _e4423) + (_e4554 * _e4516));
                                                                    } else {
                                                                        phi_25050_ = ((_e4551 * _e4305) + (_e4554 * _e4516));
                                                                    }
                                                                    let _e4564 = phi_25050_;
                                                                    phi_25056_ = _e4564;
                                                                }
                                                                let _e4569 = phi_25056_;
                                                                let _e4573 = global_6.member[(_e4226 + (_e4355 + 1u))];
                                                                let _e4579 = (select(select(u32(_e4573), 0u, (_e4573 < 0f)), 4294967295u, (_e4573 > 4294967000f)) % 12u);
                                                                let _e4582 = select(-1f, 1f, ((_e4579 & 1u) == 0u));
                                                                let _e4585 = select(-1f, 1f, ((_e4579 & 2u) == 0u));
                                                                if (_e4579 < 4u) {
                                                                    phi_25090_ = ((_e4582 * _e4304) + (_e4585 * _e4454));
                                                                } else {
                                                                    if (_e4579 < 8u) {
                                                                        phi_25084_ = ((_e4582 * _e4304) + (_e4585 * _e4516));
                                                                    } else {
                                                                        phi_25084_ = ((_e4582 * _e4454) + (_e4585 * _e4516));
                                                                    }
                                                                    let _e4595 = phi_25084_;
                                                                    phi_25090_ = _e4595;
                                                                }
                                                                let _e4600 = phi_25090_;
                                                                let _e4604 = global_6.member[(_e4226 + (_e4384 + 1u))];
                                                                let _e4610 = (select(select(u32(_e4604), 0u, (_e4604 < 0f)), 4294967295u, (_e4604 > 4294967000f)) % 12u);
                                                                let _e4613 = select(-1f, 1f, ((_e4610 & 1u) == 0u));
                                                                let _e4616 = select(-1f, 1f, ((_e4610 & 2u) == 0u));
                                                                if (_e4610 < 4u) {
                                                                    phi_25124_ = ((_e4613 * _e4423) + (_e4616 * _e4454));
                                                                } else {
                                                                    if (_e4610 < 8u) {
                                                                        phi_25118_ = ((_e4613 * _e4423) + (_e4616 * _e4516));
                                                                    } else {
                                                                        phi_25118_ = ((_e4613 * _e4454) + (_e4616 * _e4516));
                                                                    }
                                                                    let _e4626 = phi_25118_;
                                                                    phi_25124_ = _e4626;
                                                                }
                                                                let _e4631 = phi_25124_;
                                                                let _e4634 = (_e4414 + (_e4313 * (_e4445 - _e4414)));
                                                                let _e4640 = (_e4538 + (_e4313 * (_e4569 - _e4538)));
                                                                let _e4646 = (_e4634 + (_e4320 * ((_e4476 + (_e4313 * (_e4506 - _e4476))) - _e4634)));
                                                                phi_5924_ = (_e4255 + 1u);
                                                                phi_5927_ = (_e4257 + _e4259);
                                                                phi_5929_ = (_e4259 * 0.5f);
                                                                phi_5931_ = (_e4261 + ((_e4646 + ((((_e4306 * _e4306) * _e4306) * ((_e4306 * ((_e4306 * 6f) - 15f)) + 10f)) * ((_e4640 + (_e4320 * ((_e4600 + (_e4313 * (_e4631 - _e4600))) - _e4640))) - _e4646))) * _e4259));
                                                                phi_5933_ = vec3<f32>(((((_e4229 * _e4263.x) + (_e4232 * _e4263.y)) + (_e4235 * _e4263.z)) * 2.03f), ((((_e4238 * _e4263.x) + (_e4241 * _e4263.y)) + (_e4244 * _e4263.z)) * 2.03f), ((((_e4247 * _e4263.x) + (_e4250 * _e4263.y)) + (_e4253 * _e4263.z)) * 2.03f));
                                                            } else {
                                                                phi_5924_ = u32();
                                                                phi_5927_ = f32();
                                                                phi_5929_ = f32();
                                                                phi_5931_ = f32();
                                                                phi_5933_ = vec3<f32>();
                                                            }
                                                            let _e4678 = phi_5924_;
                                                            let _e4680 = phi_5927_;
                                                            let _e4682 = phi_5929_;
                                                            let _e4684 = phi_5931_;
                                                            let _e4686 = phi_5933_;
                                                            phi_5923_ = _e4678;
                                                            phi_5926_ = _e4680;
                                                            phi_5928_ = _e4682;
                                                            phi_5930_ = _e4684;
                                                            phi_5932_ = _e4686;
                                                            if !(_e4264) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4689 = local_23;
                                                        if (_e4689 > 0f) {
                                                            let _e4692 = local_24;
                                                            let _e4694 = local_25;
                                                            phi_6349_ = (_e4692 / _e4694);
                                                        } else {
                                                            phi_6349_ = 0f;
                                                        }
                                                        let _e4697 = phi_6349_;
                                                        if (_e4147 == 0u) {
                                                            phi_6432_ = _e4143;
                                                            phi_6433_ = _e4145;
                                                            phi_6434_ = _e4149;
                                                            phi_6435_ = _e4151;
                                                            phi_6436_ = _e4153;
                                                            phi_6437_ = (_e4155 + (1.5f * _e4697));
                                                        } else {
                                                            if (_e4147 == 1u) {
                                                                phi_6426_ = _e4143;
                                                                phi_6427_ = _e4145;
                                                                phi_6428_ = _e4149;
                                                                phi_6429_ = _e4151;
                                                                phi_6430_ = _e4153;
                                                                phi_6431_ = (_e4155 + (0.8f * _e4697));
                                                            } else {
                                                                let _e4700 = (_e4147 == 2u);
                                                                if _e4700 {
                                                                    phi_6421_ = _e4143;
                                                                    phi_6422_ = _e4145;
                                                                    phi_6423_ = _e4149;
                                                                    phi_6424_ = _e4151;
                                                                } else {
                                                                    if (_e4147 == 3u) {
                                                                        phi_6417_ = _e4143;
                                                                        phi_6418_ = _e4145;
                                                                        phi_6419_ = _e4149;
                                                                        phi_6420_ = vec3<f32>(_e4697, _e4151.y, _e4151.z);
                                                                    } else {
                                                                        if (_e4147 == 4u) {
                                                                            phi_6413_ = _e4143;
                                                                            phi_6414_ = _e4145;
                                                                            phi_6415_ = _e4149;
                                                                            phi_6416_ = vec3<f32>(_e4151.x, _e4697, _e4151.z);
                                                                        } else {
                                                                            if (_e4147 == 5u) {
                                                                                phi_6409_ = _e4143;
                                                                                phi_6410_ = _e4145;
                                                                                phi_6411_ = _e4149;
                                                                                phi_6412_ = vec3<f32>(_e4151.x, _e4151.y, _e4697);
                                                                            } else {
                                                                                let _e4704 = (_e4147 == 6u);
                                                                                if _e4704 {
                                                                                    phi_6406_ = _e4143;
                                                                                    phi_6408_ = _e4149;
                                                                                } else {
                                                                                    if (_e4147 == 7u) {
                                                                                        phi_6404_ = _e4143;
                                                                                        phi_6405_ = vec3<f32>(_e4697, _e4149.y, _e4149.z);
                                                                                    } else {
                                                                                        if (_e4147 == 8u) {
                                                                                            phi_6402_ = _e4143;
                                                                                            phi_6403_ = vec3<f32>(_e4149.x, _e4697, _e4149.z);
                                                                                        } else {
                                                                                            let _e4707 = (_e4147 == 9u);
                                                                                            if _e4707 {
                                                                                                phi_6401_ = vec3<f32>(_e4149.x, _e4149.y, _e4697);
                                                                                            } else {
                                                                                                phi_6401_ = _e4149;
                                                                                            }
                                                                                            let _e4713 = phi_6401_;
                                                                                            phi_6402_ = select(_e4697, _e4143, _e4707);
                                                                                            phi_6403_ = _e4713;
                                                                                        }
                                                                                        let _e4720 = phi_6402_;
                                                                                        let _e4722 = phi_6403_;
                                                                                        phi_6404_ = _e4720;
                                                                                        phi_6405_ = _e4722;
                                                                                    }
                                                                                    let _e4728 = phi_6404_;
                                                                                    let _e4730 = phi_6405_;
                                                                                    phi_6406_ = _e4728;
                                                                                    phi_6408_ = _e4730;
                                                                                }
                                                                                let _e4732 = phi_6406_;
                                                                                let _e4734 = phi_6408_;
                                                                                phi_6409_ = _e4732;
                                                                                phi_6410_ = select(_e4145, _e4697, _e4704);
                                                                                phi_6411_ = _e4734;
                                                                                phi_6412_ = _e4151;
                                                                            }
                                                                            let _e4741 = phi_6409_;
                                                                            let _e4743 = phi_6410_;
                                                                            let _e4745 = phi_6411_;
                                                                            let _e4747 = phi_6412_;
                                                                            phi_6413_ = _e4741;
                                                                            phi_6414_ = _e4743;
                                                                            phi_6415_ = _e4745;
                                                                            phi_6416_ = _e4747;
                                                                        }
                                                                        let _e4753 = phi_6413_;
                                                                        let _e4755 = phi_6414_;
                                                                        let _e4757 = phi_6415_;
                                                                        let _e4759 = phi_6416_;
                                                                        phi_6417_ = _e4753;
                                                                        phi_6418_ = _e4755;
                                                                        phi_6419_ = _e4757;
                                                                        phi_6420_ = _e4759;
                                                                    }
                                                                    let _e4765 = phi_6417_;
                                                                    let _e4767 = phi_6418_;
                                                                    let _e4769 = phi_6419_;
                                                                    let _e4771 = phi_6420_;
                                                                    phi_6421_ = _e4765;
                                                                    phi_6422_ = _e4767;
                                                                    phi_6423_ = _e4769;
                                                                    phi_6424_ = _e4771;
                                                                }
                                                                let _e4773 = phi_6421_;
                                                                let _e4775 = phi_6422_;
                                                                let _e4777 = phi_6423_;
                                                                let _e4779 = phi_6424_;
                                                                phi_6426_ = _e4773;
                                                                phi_6427_ = _e4775;
                                                                phi_6428_ = _e4777;
                                                                phi_6429_ = _e4779;
                                                                phi_6430_ = select(_e4153, _e4697, _e4700);
                                                                phi_6431_ = _e4155;
                                                            }
                                                            let _e4784 = phi_6426_;
                                                            let _e4786 = phi_6427_;
                                                            let _e4788 = phi_6428_;
                                                            let _e4790 = phi_6429_;
                                                            let _e4792 = phi_6430_;
                                                            let _e4794 = phi_6431_;
                                                            phi_6432_ = _e4784;
                                                            phi_6433_ = _e4786;
                                                            phi_6434_ = _e4788;
                                                            phi_6435_ = _e4790;
                                                            phi_6436_ = _e4792;
                                                            phi_6437_ = _e4794;
                                                        }
                                                        let _e4798 = phi_6432_;
                                                        let _e4800 = phi_6433_;
                                                        let _e4802 = phi_6434_;
                                                        let _e4804 = phi_6435_;
                                                        let _e4806 = phi_6436_;
                                                        let _e4808 = phi_6437_;
                                                        phi_5696_ = _e4798;
                                                        phi_5699_ = _e4800;
                                                        phi_5701_ = (_e4147 + 1u);
                                                        phi_5703_ = _e4802;
                                                        phi_5705_ = _e4804;
                                                        phi_5707_ = _e4806;
                                                        phi_5709_ = _e4808;
                                                    } else {
                                                        phi_5696_ = f32();
                                                        phi_5699_ = f32();
                                                        phi_5701_ = u32();
                                                        phi_5703_ = vec3<f32>();
                                                        phi_5705_ = vec3<f32>();
                                                        phi_5707_ = f32();
                                                        phi_5709_ = f32();
                                                    }
                                                    let _e4811 = phi_5696_;
                                                    let _e4813 = phi_5699_;
                                                    let _e4815 = phi_5701_;
                                                    let _e4817 = phi_5703_;
                                                    let _e4819 = phi_5705_;
                                                    let _e4821 = phi_5707_;
                                                    let _e4823 = phi_5709_;
                                                    phi_5695_ = _e4811;
                                                    phi_5698_ = _e4813;
                                                    phi_5700_ = _e4815;
                                                    phi_5702_ = _e4817;
                                                    phi_5704_ = _e4819;
                                                    phi_5706_ = _e4821;
                                                    phi_5708_ = _e4823;
                                                    if !(_e4156) {
                                                        break;
                                                    }
                                                }
                                                if _e4120 {
                                                    let _e4838 = (exp((-(abs(_e3847)) / (0.06f + (abs(_e3858) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3858 * -0.71428573f) * (_e3858 * 0.71428573f))))));
                                                    let _e4840 = (_e3847 * 1.3333334f);
                                                    let _e4845 = (1.3f * exp((((_e3858 * _e3858) + (_e4840 * _e4840)) * -12.499999f)));
                                                    let _e4856 = local_26;
                                                    let _e4858 = local_27;
                                                    if (_e4856 != _e4858) {
                                                        phi_25200_ = true;
                                                    } else {
                                                        let _e4861 = local_28;
                                                        phi_25200_ = (0.1f >= _e4861);
                                                    }
                                                    let _e4864 = phi_25200_;
                                                    let _e4866 = local_29;
                                                    let _e4869 = (pow(select(_e4866, 0.1f, _e4864), 1.3f) * _e4078);
                                                    phi_6469_ = vec3<f32>((_e4075.x + (((_e4084 * _e4838) + (_e4093 * _e4845)) * _e4869)), (_e4075.y + (((_e4087 * _e4838) + (_e4096 * _e4845)) * _e4869)), (_e4075.z + (((_e4090 * _e4838) + (_e4099 * _e4845)) * _e4869)));
                                                } else {
                                                    phi_6469_ = _e4075;
                                                }
                                                let _e4881 = phi_6469_;
                                                if select(_e4130, _e4134, _e4136) {
                                                    let _e4883 = local_30;
                                                    let _e4889 = ((_e3847 - (0.025f * _e4883)) / (0.045f + (abs(_e3858) * 0.0095492955f)));
                                                    let _e4894 = local_31;
                                                    let _e4895 = (_e4894 * 3.3333333f);
                                                    let _e4897 = select(_e4895, 0f, (_e4895 < 0f));
                                                    let _e4899 = select(_e4897, 1f, (_e4897 > 1f));
                                                    let _e4915 = local_32;
                                                    let _e4917 = ((_e4915 - 0.25f) * 5.0000005f);
                                                    let _e4919 = select(_e4917, 0f, (_e4917 < 0f));
                                                    let _e4921 = select(_e4919, 1f, (_e4919 > 1f));
                                                    phi_6548_ = (_e4081 * (((exp((-(_e4889) * _e4889)) * (0.15f + (3f * ((_e4899 * _e4899) * (3f - (2f * _e4899)))))) * (0.5f + (0.9f * exp(((_e3858 * -0.6666667f) * (_e3858 * 0.6666667f)))))) + (1.2f * (exp(((_e3847 * -3.3333333f) * (_e3847 * 3.3333333f))) * ((_e4921 * _e4921) * (3f - (2f * _e4921)))))));
                                                } else {
                                                    phi_6548_ = 0f;
                                                }
                                                let _e4935 = phi_6548_;
                                                phi_6549_ = _e4881;
                                                phi_6550_ = _e4935;
                                            } else {
                                                phi_6549_ = _e4075;
                                                phi_6550_ = 0f;
                                            }
                                            let _e4937 = phi_6549_;
                                            let _e4939 = phi_6550_;
                                            phi_6552_ = _e4937;
                                            phi_6555_ = 0u;
                                            loop {
                                                let _e4941 = phi_6552_;
                                                let _e4943 = phi_6555_;
                                                local_49 = _e4941;
                                                local_50 = _e4941;
                                                local_51 = _e4941;
                                                let _e4944 = (_e4943 < bitcast<u32>(_e3810));
                                                if _e4944 {
                                                    let _e4946 = (bitcast<u32>(_e3806) + (_e4943 * 29u));
                                                    let _e4948 = global_6.member[_e4946];
                                                    let _e4949 = (_e4948 > 0.5f);
                                                    let _e4952 = global_6.member[(_e4946 + 1u)];
                                                    let _e4955 = global_6.member[(_e4946 + 2u)];
                                                    let _e4958 = global_6.member[(_e4946 + 3u)];
                                                    let _e4961 = global_6.member[(_e4946 + 4u)];
                                                    let _e4964 = global_6.member[(_e4946 + 5u)];
                                                    let _e4967 = global_6.member[(_e4946 + 6u)];
                                                    let _e4972 = (((_e340.member_1.x * _e4952) + (_e340.member_1.y * _e4955)) + (_e340.member_1.z * _e4958));
                                                    let _e4973 = (_e4972 < _e4964);
                                                    if _e4973 {
                                                        phi_7701_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7702_ = vec3<f32>();
                                                    } else {
                                                        let _e4976 = global_6.member[(_e4946 + 7u)];
                                                        let _e4979 = global_6.member[(_e4946 + 8u)];
                                                        let _e4982 = global_6.member[(_e4946 + 9u)];
                                                        let _e4985 = global_6.member[(_e4946 + 10u)];
                                                        let _e4988 = global_6.member[(_e4946 + 11u)];
                                                        let _e4991 = global_6.member[(_e4946 + 12u)];
                                                        let _e4994 = global_6.member[(_e4946 + 13u)];
                                                        let _e4997 = global_6.member[(_e4946 + 14u)];
                                                        let _e5000 = global_6.member[(_e4946 + 15u)];
                                                        let _e5003 = global_6.member[(_e4946 + 16u)];
                                                        let _e5006 = global_6.member[(_e4946 + 17u)];
                                                        let _e5009 = global_6.member[(_e4946 + 18u)];
                                                        let _e5012 = global_6.member[(_e4946 + 19u)];
                                                        let _e5015 = global_6.member[(_e4946 + 20u)];
                                                        let _e5018 = global_6.member[(_e4946 + 21u)];
                                                        let _e5021 = global_6.member[(_e4946 + 22u)];
                                                        let _e5024 = global_6.member[(_e4946 + 23u)];
                                                        let _e5027 = global_6.member[(_e4946 + 24u)];
                                                        let _e5030 = global_6.member[(_e4946 + 25u)];
                                                        let _e5033 = global_6.member[(_e4946 + 26u)];
                                                        let _e5036 = global_6.member[(_e4946 + 27u)];
                                                        let _e5039 = global_6.member[(_e4946 + 28u)];
                                                        let _e5046 = (((_e340.member_1.x / _e4972) - _e4952) / _e4961);
                                                        let _e5047 = (((_e340.member_1.y / _e4972) - _e4955) / _e4961);
                                                        let _e5048 = (((_e340.member_1.z / _e4972) - _e4958) / _e4961);
                                                        if _e4949 {
                                                            let _e5653 = (((_e5046 * _e5003) + (_e5047 * _e5006)) + (_e5048 * _e5009));
                                                            let _e5659 = ((((_e5046 * _e5012) + (_e5047 * _e5015)) + (_e5048 * _e5018)) / _e5021);
                                                            phi_7699_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4967 * exp((-2.5f * pow((sqrt(((_e5653 * _e5653) + (_e5659 * _e5659))) * 3.3333333f), 0.7f)))));
                                                            phi_7700_ = vec3<f32>();
                                                        } else {
                                                            let _e5053 = (((_e4976 * _e4952) + (_e4979 * _e4955)) + (_e4982 * _e4958));
                                                            if (abs(_e5053) < 0.12f) {
                                                                phi_7003_ = select(0.12f, -0.12f, (_e5053 < 0f));
                                                            } else {
                                                                phi_7003_ = _e5053;
                                                            }
                                                            let _e5059 = phi_7003_;
                                                            let _e5065 = ((((_e4976 * _e5046) + (_e4979 * _e5047)) + (_e4982 * _e5048)) / _e5059);
                                                            let _e5069 = (_e5046 - (_e4952 * _e5065));
                                                            let _e5070 = (_e5047 - (_e4955 * _e5065));
                                                            let _e5071 = (_e5048 - (_e4958 * _e5065));
                                                            let _e5076 = (((_e5069 * _e4985) + (_e5070 * _e4988)) + (_e5071 * _e4991));
                                                            let _e5081 = (((_e5069 * _e4994) + (_e5070 * _e4997)) + (_e5071 * _e5000));
                                                            let _e5085 = sqrt(((_e5076 * _e5076) + (_e5081 * _e5081)));
                                                            let _e5097 = abs(_e5059);
                                                            let _e5099 = (pow((0.5f + (0.5f * cos(((_e5024 * (atan2(_e5081, _e5076) - (log((_e5085 + 0.03f)) / _e5027))) + _e5030)))), 3f) * sqrt(_e5097));
                                                            let _e5104 = global_6.member[_e3787];
                                                            let _e5105 = bitcast<u32>(_e5104);
                                                            let _e5108 = global_6.member[(_e1298 + 11u)];
                                                            let _e5111 = global_6.member[(_e1298 + 12u)];
                                                            let _e5114 = global_6.member[(_e1298 + 13u)];
                                                            let _e5117 = global_6.member[(_e1298 + 14u)];
                                                            let _e5120 = global_6.member[(_e1298 + 15u)];
                                                            let _e5123 = global_6.member[(_e1298 + 16u)];
                                                            let _e5126 = global_6.member[(_e1298 + 17u)];
                                                            let _e5129 = global_6.member[(_e1298 + 18u)];
                                                            let _e5132 = global_6.member[(_e1298 + 19u)];
                                                            phi_7183_ = 0u;
                                                            phi_7186_ = 0f;
                                                            phi_7188_ = 1f;
                                                            phi_7190_ = 0f;
                                                            phi_7192_ = vec3<f32>((_e5076 * 9f), (_e5081 * 9f), 0f);
                                                            loop {
                                                                let _e5134 = phi_7183_;
                                                                let _e5136 = phi_7186_;
                                                                let _e5138 = phi_7188_;
                                                                let _e5140 = phi_7190_;
                                                                let _e5142 = phi_7192_;
                                                                local_33 = _e5136;
                                                                local_34 = _e5140;
                                                                local_35 = _e5136;
                                                                let _e5143 = (_e5134 < 4u);
                                                                if _e5143 {
                                                                    let _e5145 = (_e5142.x + _e5033);
                                                                    let _e5147 = (_e5142.y + _e5036);
                                                                    let _e5149 = (_e5142.z + _e5039);
                                                                    let _e5150 = floor(_e5145);
                                                                    let _e5151 = floor(_e5147);
                                                                    let _e5152 = floor(_e5149);
                                                                    let _e5161 = bitcast<u32>((select(0i, select(select(i32(_e5150), i32(-2147483648), (_e5150 < -2147483600f)), 2147483647i, (_e5150 > 2147483500f)), (_e5150 == _e5150)) & 255i));
                                                                    let _e5170 = bitcast<u32>((select(0i, select(select(i32(_e5151), i32(-2147483648), (_e5151 < -2147483600f)), 2147483647i, (_e5151 > 2147483500f)), (_e5151 == _e5151)) & 255i));
                                                                    let _e5179 = bitcast<u32>((select(0i, select(select(i32(_e5152), i32(-2147483648), (_e5152 < -2147483600f)), 2147483647i, (_e5152 > 2147483500f)), (_e5152 == _e5152)) & 255i));
                                                                    let _e5180 = (_e5145 - _e5150);
                                                                    let _e5181 = (_e5147 - _e5151);
                                                                    let _e5182 = (_e5149 - _e5152);
                                                                    let _e5189 = (((_e5180 * _e5180) * _e5180) * ((_e5180 * ((_e5180 * 6f) - 15f)) + 10f));
                                                                    let _e5196 = (((_e5181 * _e5181) * _e5181) * ((_e5181 * ((_e5181 * 6f) - 15f)) + 10f));
                                                                    let _e5206 = global_6.member[(_e5105 + _e5161)];
                                                                    let _e5212 = (select(select(u32(_e5206), 0u, (_e5206 < 0f)), 4294967295u, (_e5206 > 4294967000f)) + _e5170);
                                                                    let _e5215 = global_6.member[(_e5105 + _e5212)];
                                                                    let _e5221 = (select(select(u32(_e5215), 0u, (_e5215 < 0f)), 4294967295u, (_e5215 > 4294967000f)) + _e5179);
                                                                    let _e5225 = global_6.member[(_e5105 + (_e5212 + 1u))];
                                                                    let _e5231 = (select(select(u32(_e5225), 0u, (_e5225 < 0f)), 4294967295u, (_e5225 > 4294967000f)) + _e5179);
                                                                    let _e5235 = global_6.member[(_e5105 + (_e5161 + 1u))];
                                                                    let _e5241 = (select(select(u32(_e5235), 0u, (_e5235 < 0f)), 4294967295u, (_e5235 > 4294967000f)) + _e5170);
                                                                    let _e5244 = global_6.member[(_e5105 + _e5241)];
                                                                    let _e5250 = (select(select(u32(_e5244), 0u, (_e5244 < 0f)), 4294967295u, (_e5244 > 4294967000f)) + _e5179);
                                                                    let _e5254 = global_6.member[(_e5105 + (_e5241 + 1u))];
                                                                    let _e5260 = (select(select(u32(_e5254), 0u, (_e5254 < 0f)), 4294967295u, (_e5254 > 4294967000f)) + _e5179);
                                                                    let _e5263 = global_6.member[(_e5105 + _e5221)];
                                                                    let _e5269 = (select(select(u32(_e5263), 0u, (_e5263 < 0f)), 4294967295u, (_e5263 > 4294967000f)) % 12u);
                                                                    let _e5272 = select(-1f, 1f, ((_e5269 & 1u) == 0u));
                                                                    let _e5275 = select(-1f, 1f, ((_e5269 & 2u) == 0u));
                                                                    if (_e5269 < 4u) {
                                                                        phi_25295_ = ((_e5272 * _e5180) + (_e5275 * _e5181));
                                                                    } else {
                                                                        if (_e5269 < 8u) {
                                                                            phi_25289_ = ((_e5272 * _e5180) + (_e5275 * _e5182));
                                                                        } else {
                                                                            phi_25289_ = ((_e5272 * _e5181) + (_e5275 * _e5182));
                                                                        }
                                                                        let _e5285 = phi_25289_;
                                                                        phi_25295_ = _e5285;
                                                                    }
                                                                    let _e5290 = phi_25295_;
                                                                    let _e5293 = global_6.member[(_e5105 + _e5250)];
                                                                    let _e5299 = (_e5180 - 1f);
                                                                    let _e5300 = (select(select(u32(_e5293), 0u, (_e5293 < 0f)), 4294967295u, (_e5293 > 4294967000f)) % 12u);
                                                                    let _e5303 = select(-1f, 1f, ((_e5300 & 1u) == 0u));
                                                                    let _e5306 = select(-1f, 1f, ((_e5300 & 2u) == 0u));
                                                                    if (_e5300 < 4u) {
                                                                        phi_25329_ = ((_e5303 * _e5299) + (_e5306 * _e5181));
                                                                    } else {
                                                                        if (_e5300 < 8u) {
                                                                            phi_25323_ = ((_e5303 * _e5299) + (_e5306 * _e5182));
                                                                        } else {
                                                                            phi_25323_ = ((_e5303 * _e5181) + (_e5306 * _e5182));
                                                                        }
                                                                        let _e5316 = phi_25323_;
                                                                        phi_25329_ = _e5316;
                                                                    }
                                                                    let _e5321 = phi_25329_;
                                                                    let _e5324 = global_6.member[(_e5105 + _e5231)];
                                                                    let _e5330 = (_e5181 - 1f);
                                                                    let _e5331 = (select(select(u32(_e5324), 0u, (_e5324 < 0f)), 4294967295u, (_e5324 > 4294967000f)) % 12u);
                                                                    let _e5334 = select(-1f, 1f, ((_e5331 & 1u) == 0u));
                                                                    let _e5337 = select(-1f, 1f, ((_e5331 & 2u) == 0u));
                                                                    if (_e5331 < 4u) {
                                                                        phi_25363_ = ((_e5334 * _e5180) + (_e5337 * _e5330));
                                                                    } else {
                                                                        if (_e5331 < 8u) {
                                                                            phi_25357_ = ((_e5334 * _e5180) + (_e5337 * _e5182));
                                                                        } else {
                                                                            phi_25357_ = ((_e5334 * _e5330) + (_e5337 * _e5182));
                                                                        }
                                                                        let _e5347 = phi_25357_;
                                                                        phi_25363_ = _e5347;
                                                                    }
                                                                    let _e5352 = phi_25363_;
                                                                    let _e5355 = global_6.member[(_e5105 + _e5260)];
                                                                    let _e5361 = (select(select(u32(_e5355), 0u, (_e5355 < 0f)), 4294967295u, (_e5355 > 4294967000f)) % 12u);
                                                                    let _e5364 = select(-1f, 1f, ((_e5361 & 1u) == 0u));
                                                                    let _e5367 = select(-1f, 1f, ((_e5361 & 2u) == 0u));
                                                                    if (_e5361 < 4u) {
                                                                        phi_25397_ = ((_e5364 * _e5299) + (_e5367 * _e5330));
                                                                    } else {
                                                                        if (_e5361 < 8u) {
                                                                            phi_25391_ = ((_e5364 * _e5299) + (_e5367 * _e5182));
                                                                        } else {
                                                                            phi_25391_ = ((_e5364 * _e5330) + (_e5367 * _e5182));
                                                                        }
                                                                        let _e5377 = phi_25391_;
                                                                        phi_25397_ = _e5377;
                                                                    }
                                                                    let _e5382 = phi_25397_;
                                                                    let _e5386 = global_6.member[(_e5105 + (_e5221 + 1u))];
                                                                    let _e5392 = (_e5182 - 1f);
                                                                    let _e5393 = (select(select(u32(_e5386), 0u, (_e5386 < 0f)), 4294967295u, (_e5386 > 4294967000f)) % 12u);
                                                                    let _e5396 = select(-1f, 1f, ((_e5393 & 1u) == 0u));
                                                                    let _e5399 = select(-1f, 1f, ((_e5393 & 2u) == 0u));
                                                                    if (_e5393 < 4u) {
                                                                        phi_25431_ = ((_e5396 * _e5180) + (_e5399 * _e5181));
                                                                    } else {
                                                                        if (_e5393 < 8u) {
                                                                            phi_25425_ = ((_e5396 * _e5180) + (_e5399 * _e5392));
                                                                        } else {
                                                                            phi_25425_ = ((_e5396 * _e5181) + (_e5399 * _e5392));
                                                                        }
                                                                        let _e5409 = phi_25425_;
                                                                        phi_25431_ = _e5409;
                                                                    }
                                                                    let _e5414 = phi_25431_;
                                                                    let _e5418 = global_6.member[(_e5105 + (_e5250 + 1u))];
                                                                    let _e5424 = (select(select(u32(_e5418), 0u, (_e5418 < 0f)), 4294967295u, (_e5418 > 4294967000f)) % 12u);
                                                                    let _e5427 = select(-1f, 1f, ((_e5424 & 1u) == 0u));
                                                                    let _e5430 = select(-1f, 1f, ((_e5424 & 2u) == 0u));
                                                                    if (_e5424 < 4u) {
                                                                        phi_25465_ = ((_e5427 * _e5299) + (_e5430 * _e5181));
                                                                    } else {
                                                                        if (_e5424 < 8u) {
                                                                            phi_25459_ = ((_e5427 * _e5299) + (_e5430 * _e5392));
                                                                        } else {
                                                                            phi_25459_ = ((_e5427 * _e5181) + (_e5430 * _e5392));
                                                                        }
                                                                        let _e5440 = phi_25459_;
                                                                        phi_25465_ = _e5440;
                                                                    }
                                                                    let _e5445 = phi_25465_;
                                                                    let _e5449 = global_6.member[(_e5105 + (_e5231 + 1u))];
                                                                    let _e5455 = (select(select(u32(_e5449), 0u, (_e5449 < 0f)), 4294967295u, (_e5449 > 4294967000f)) % 12u);
                                                                    let _e5458 = select(-1f, 1f, ((_e5455 & 1u) == 0u));
                                                                    let _e5461 = select(-1f, 1f, ((_e5455 & 2u) == 0u));
                                                                    if (_e5455 < 4u) {
                                                                        phi_25499_ = ((_e5458 * _e5180) + (_e5461 * _e5330));
                                                                    } else {
                                                                        if (_e5455 < 8u) {
                                                                            phi_25493_ = ((_e5458 * _e5180) + (_e5461 * _e5392));
                                                                        } else {
                                                                            phi_25493_ = ((_e5458 * _e5330) + (_e5461 * _e5392));
                                                                        }
                                                                        let _e5471 = phi_25493_;
                                                                        phi_25499_ = _e5471;
                                                                    }
                                                                    let _e5476 = phi_25499_;
                                                                    let _e5480 = global_6.member[(_e5105 + (_e5260 + 1u))];
                                                                    let _e5486 = (select(select(u32(_e5480), 0u, (_e5480 < 0f)), 4294967295u, (_e5480 > 4294967000f)) % 12u);
                                                                    let _e5489 = select(-1f, 1f, ((_e5486 & 1u) == 0u));
                                                                    let _e5492 = select(-1f, 1f, ((_e5486 & 2u) == 0u));
                                                                    if (_e5486 < 4u) {
                                                                        phi_25533_ = ((_e5489 * _e5299) + (_e5492 * _e5330));
                                                                    } else {
                                                                        if (_e5486 < 8u) {
                                                                            phi_25527_ = ((_e5489 * _e5299) + (_e5492 * _e5392));
                                                                        } else {
                                                                            phi_25527_ = ((_e5489 * _e5330) + (_e5492 * _e5392));
                                                                        }
                                                                        let _e5502 = phi_25527_;
                                                                        phi_25533_ = _e5502;
                                                                    }
                                                                    let _e5507 = phi_25533_;
                                                                    let _e5510 = (_e5290 + (_e5189 * (_e5321 - _e5290)));
                                                                    let _e5516 = (_e5414 + (_e5189 * (_e5445 - _e5414)));
                                                                    let _e5522 = (_e5510 + (_e5196 * ((_e5352 + (_e5189 * (_e5382 - _e5352))) - _e5510)));
                                                                    phi_7184_ = (_e5134 + 1u);
                                                                    phi_7187_ = (_e5136 + _e5138);
                                                                    phi_7189_ = (_e5138 * 0.5f);
                                                                    phi_7191_ = (_e5140 + ((_e5522 + ((((_e5182 * _e5182) * _e5182) * ((_e5182 * ((_e5182 * 6f) - 15f)) + 10f)) * ((_e5516 + (_e5196 * ((_e5476 + (_e5189 * (_e5507 - _e5476))) - _e5516))) - _e5522))) * _e5138));
                                                                    phi_7193_ = vec3<f32>(((((_e5108 * _e5142.x) + (_e5111 * _e5142.y)) + (_e5114 * _e5142.z)) * 2.03f), ((((_e5117 * _e5142.x) + (_e5120 * _e5142.y)) + (_e5123 * _e5142.z)) * 2.03f), ((((_e5126 * _e5142.x) + (_e5129 * _e5142.y)) + (_e5132 * _e5142.z)) * 2.03f));
                                                                } else {
                                                                    phi_7184_ = u32();
                                                                    phi_7187_ = f32();
                                                                    phi_7189_ = f32();
                                                                    phi_7191_ = f32();
                                                                    phi_7193_ = vec3<f32>();
                                                                }
                                                                let _e5554 = phi_7184_;
                                                                let _e5556 = phi_7187_;
                                                                let _e5558 = phi_7189_;
                                                                let _e5560 = phi_7191_;
                                                                let _e5562 = phi_7193_;
                                                                phi_7183_ = _e5554;
                                                                phi_7186_ = _e5556;
                                                                phi_7188_ = _e5558;
                                                                phi_7190_ = _e5560;
                                                                phi_7192_ = _e5562;
                                                                if !(_e5143) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5565 = local_33;
                                                            if (_e5565 > 0f) {
                                                                let _e5568 = local_34;
                                                                let _e5570 = local_35;
                                                                phi_7606_ = (_e5568 / _e5570);
                                                            } else {
                                                                phi_7606_ = 0f;
                                                            }
                                                            let _e5573 = phi_7606_;
                                                            let _e5575 = (1f + (1.2f * _e5573));
                                                            if (_e5575 != _e5575) {
                                                                phi_25579_ = true;
                                                            } else {
                                                                phi_25579_ = (0f >= _e5575);
                                                            }
                                                            let _e5579 = phi_25579_;
                                                            let _e5584 = ((_e5085 - 1.15f) * -2.8571432f);
                                                            let _e5586 = select(_e5584, 0f, (_e5584 < 0f));
                                                            let _e5588 = select(_e5586, 1f, (_e5586 > 1f));
                                                            let _e5594 = (1f / _e5097);
                                                            if (_e5594 != _e5594) {
                                                                phi_25594_ = true;
                                                            } else {
                                                                phi_25594_ = (4f <= _e5594);
                                                            }
                                                            let _e5598 = phi_25594_;
                                                            let _e5601 = (((exp((_e5085 * -3.5714285f)) * ((_e5588 * _e5588) * (3f - (2f * _e5588)))) * select(_e5594, 4f, _e5598)) * 0.12f);
                                                            let _e5607 = sqrt((((_e5046 * _e5046) + (_e5047 * _e5047)) + (_e5048 * _e5048)));
                                                            let _e5614 = (((_e5046 * _e5012) + (_e5047 * _e5015)) + (_e5048 * _e5018));
                                                            let _e5615 = (1f - _e5097);
                                                            let _e5631 = ((_e5601 * (0.6f + (0.4f * (1f - _e5099)))) + exp((_e5607 * -16.666668f)));
                                                            let _e5637 = (((_e5601 * _e5099) * 1.8f) * select(_e5575, 0f, _e5579));
                                                            let _e5644 = (_e4967 * (1f - ((((0.75f * _e5615) * _e5615) * exp(((_e5614 * -28.571428f) * (_e5614 * 28.571428f)))) * exp((_e5607 * -1.4285715f)))));
                                                            phi_7699_ = vec3<f32>();
                                                            phi_7700_ = vec3<f32>((((1.1633643f * _e5631) + (0.86689025f * _e5637)) * _e5644), (((0.97722596f * _e5631) + (1.0155001f * _e5637)) * _e5644), (((0.74455315f * _e5631) + (1.2384146f * _e5637)) * _e5644));
                                                        }
                                                        let _e5671 = phi_7699_;
                                                        let _e5673 = phi_7700_;
                                                        phi_7701_ = _e5671;
                                                        phi_7702_ = _e5673;
                                                    }
                                                    let _e5675 = phi_7701_;
                                                    let _e5677 = phi_7702_;
                                                    phi_6553_ = (_e4941 + select(_e5677, _e5675, vec3(select(_e4949, true, _e4973))));
                                                    phi_6556_ = (_e4943 + 1u);
                                                } else {
                                                    phi_6553_ = vec3<f32>();
                                                    phi_6556_ = u32();
                                                }
                                                let _e5684 = phi_6553_;
                                                let _e5686 = phi_6556_;
                                                phi_6552_ = _e5684;
                                                phi_6555_ = _e5686;
                                                if !(_e4944) {
                                                    break;
                                                }
                                            }
                                            phi_34016_ = _e4073;
                                            if _e4073 {
                                                break;
                                            }
                                            phi_7721_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7724_ = 0u;
                                            phi_7726_ = _e4939;
                                            loop {
                                                let _e5689 = phi_7721_;
                                                let _e5691 = phi_7724_;
                                                let _e5693 = phi_7726_;
                                                local_45 = _e5693;
                                                local_46 = _e5693;
                                                local_47 = _e5693;
                                                local_48 = _e5693;
                                                local_52 = _e5689;
                                                local_53 = _e5689;
                                                local_54 = _e5689;
                                                let _e5694 = (_e5691 < bitcast<u32>(_e3802));
                                                if _e5694 {
                                                    let _e5696 = (bitcast<u32>(_e3798) + (_e5691 * 18u));
                                                    let _e5698 = global_6.member[_e5696];
                                                    let _e5703 = select(select(u32(_e5698), 0u, (_e5698 < 0f)), 4294967295u, (_e5698 > 4294967000f));
                                                    let _e5706 = global_6.member[(_e5696 + 1u)];
                                                    let _e5709 = global_6.member[(_e5696 + 2u)];
                                                    let _e5712 = global_6.member[(_e5696 + 3u)];
                                                    let _e5715 = global_6.member[(_e5696 + 4u)];
                                                    let _e5718 = global_6.member[(_e5696 + 5u)];
                                                    let _e5721 = global_6.member[(_e5696 + 6u)];
                                                    let _e5724 = global_6.member[(_e5696 + 7u)];
                                                    let _e5727 = global_6.member[(_e5696 + 8u)];
                                                    let _e5730 = global_6.member[(_e5696 + 9u)];
                                                    let _e5733 = global_6.member[(_e5696 + 10u)];
                                                    let _e5736 = global_6.member[(_e5696 + 11u)];
                                                    let _e5739 = global_6.member[(_e5696 + 12u)];
                                                    let _e5742 = global_6.member[(_e5696 + 13u)];
                                                    let _e5745 = global_6.member[(_e5696 + 14u)];
                                                    let _e5748 = global_6.member[(_e5696 + 15u)];
                                                    let _e5751 = global_6.member[(_e5696 + 16u)];
                                                    let _e5754 = global_6.member[(_e5696 + 17u)];
                                                    let _e5759 = (((_e340.member_1.x * _e5706) + (_e340.member_1.y * _e5709)) + (_e340.member_1.z * _e5712));
                                                    if (_e5759 < _e5718) {
                                                        phi_9043_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5764 = ((_e340.member_1.x / _e5759) - _e5706);
                                                        let _e5765 = ((_e340.member_1.y / _e5759) - _e5709);
                                                        let _e5766 = ((_e340.member_1.z / _e5759) - _e5712);
                                                        let _e5773 = ((((_e5764 * _e5727) + (_e5765 * _e5730)) + (_e5766 * _e5733)) / (_e5715 * _e5745));
                                                        let _e5779 = ((((_e5764 * _e5736) + (_e5765 * _e5739)) + (_e5766 * _e5742)) / _e5715);
                                                        let _e5783 = sqrt(((_e5773 * _e5773) + (_e5779 * _e5779)));
                                                        let _e5786 = vec3<f32>((_e5773 * 2.2f), (_e5779 * 2.2f), 0f);
                                                        let _e5787 = (_e5703 == 0u);
                                                        if _e5787 {
                                                            phi_8028_ = 6u;
                                                            phi_8029_ = 1.2f;
                                                            phi_8030_ = _e5786;
                                                        } else {
                                                            let _e5788 = (_e5703 == 1u);
                                                            if _e5788 {
                                                                phi_8027_ = vec3<f32>((_e5773 * 1.54f), (_e5779 * 1.54f), 0f);
                                                            } else {
                                                                phi_8027_ = _e5786;
                                                            }
                                                            let _e5793 = phi_8027_;
                                                            phi_8028_ = select(6u, 5u, _e5788);
                                                            phi_8029_ = select(1f, 0.8f, _e5788);
                                                            phi_8030_ = _e5793;
                                                        }
                                                        let _e5797 = phi_8028_;
                                                        let _e5799 = phi_8029_;
                                                        let _e5801 = phi_8030_;
                                                        if _e5787 {
                                                            phi_8047_ = vec3<f32>(((_e5773 * 3.7400002f) + -7f), ((_e5779 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8047_ = vec3<f32>((_e5773 * 3.3000002f), (_e5779 * 3.3000002f), 0f);
                                                        }
                                                        let _e5811 = phi_8047_;
                                                        phi_8049_ = 0u;
                                                        phi_8052_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8054_ = 0f;
                                                        phi_8056_ = 0f;
                                                        phi_8058_ = 0f;
                                                        loop {
                                                            let _e5814 = phi_8049_;
                                                            let _e5816 = phi_8052_;
                                                            let _e5818 = phi_8054_;
                                                            let _e5820 = phi_8056_;
                                                            let _e5822 = phi_8058_;
                                                            local_39 = _e5822;
                                                            local_40 = _e5820;
                                                            local_41 = _e5818;
                                                            local_42 = _e5820;
                                                            local_43 = _e5820;
                                                            local_44 = _e5818;
                                                            let _e5823 = (_e5814 < 6u);
                                                            if _e5823 {
                                                                let _e5824 = (_e5814 == 0u);
                                                                if _e5824 {
                                                                    phi_8100_ = true;
                                                                    phi_8101_ = false;
                                                                    phi_8102_ = false;
                                                                } else {
                                                                    if (_e5814 <= 4u) {
                                                                        phi_8078_ = (_e5703 == 2u);
                                                                    } else {
                                                                        phi_8078_ = true;
                                                                    }
                                                                    let _e5828 = phi_8078_;
                                                                    if _e5828 {
                                                                        let _e5829 = (_e5814 == 5u);
                                                                        if _e5829 {
                                                                            if _e5787 {
                                                                                phi_8094_ = false;
                                                                            } else {
                                                                                phi_8094_ = (_e5703 == 2u);
                                                                            }
                                                                            let _e5832 = phi_8094_;
                                                                            phi_8096_ = _e5832;
                                                                        } else {
                                                                            phi_8096_ = false;
                                                                        }
                                                                        let _e5834 = phi_8096_;
                                                                        phi_8098_ = select(false, _e5787, _e5829);
                                                                        phi_8099_ = _e5834;
                                                                    } else {
                                                                        phi_8098_ = false;
                                                                        phi_8099_ = false;
                                                                    }
                                                                    let _e5837 = phi_8098_;
                                                                    let _e5839 = phi_8099_;
                                                                    phi_8100_ = select(true, false, _e5828);
                                                                    phi_8101_ = _e5837;
                                                                    phi_8102_ = _e5839;
                                                                }
                                                                let _e5842 = phi_8100_;
                                                                let _e5844 = phi_8101_;
                                                                let _e5846 = phi_8102_;
                                                                if select(select(_e5846, false, _e5842), true, select(_e5844, true, _e5842)) {
                                                                    if _e5824 {
                                                                        phi_8149_ = 3u;
                                                                        phi_8150_ = vec3<f32>(((_e5773 * 1.7600001f) + 3f), ((_e5779 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5814 < 4u) {
                                                                            let _e5863 = (_e5814 - 1u);
                                                                            if (_e5863 == 0u) {
                                                                                phi_25613_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_25613_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5863 == 1u)));
                                                                            }
                                                                            let _e5869 = phi_25613_;
                                                                            phi_8147_ = 4u;
                                                                            phi_8148_ = (_e5801 + _e5869);
                                                                        } else {
                                                                            let _e5851 = (_e5814 == 4u);
                                                                            if _e5851 {
                                                                                phi_8146_ = (_e5801 + vec3<f32>((_e5816.x * _e5799), (_e5816.y * _e5799), (_e5816.z * _e5799)));
                                                                            } else {
                                                                                phi_8146_ = _e5811;
                                                                            }
                                                                            let _e5861 = phi_8146_;
                                                                            phi_8147_ = select(select(4u, 5u, _e5787), _e5797, _e5851);
                                                                            phi_8148_ = _e5861;
                                                                        }
                                                                        let _e5872 = phi_8147_;
                                                                        let _e5874 = phi_8148_;
                                                                        phi_8149_ = _e5872;
                                                                        phi_8150_ = _e5874;
                                                                    }
                                                                    let _e5881 = phi_8149_;
                                                                    let _e5883 = phi_8150_;
                                                                    let _e5885 = global_6.member[_e3787];
                                                                    let _e5886 = bitcast<u32>(_e5885);
                                                                    let _e5889 = global_6.member[(_e1298 + 11u)];
                                                                    let _e5892 = global_6.member[(_e1298 + 12u)];
                                                                    let _e5895 = global_6.member[(_e1298 + 13u)];
                                                                    let _e5898 = global_6.member[(_e1298 + 14u)];
                                                                    let _e5901 = global_6.member[(_e1298 + 15u)];
                                                                    let _e5904 = global_6.member[(_e1298 + 16u)];
                                                                    let _e5907 = global_6.member[(_e1298 + 17u)];
                                                                    let _e5910 = global_6.member[(_e1298 + 18u)];
                                                                    let _e5913 = global_6.member[(_e1298 + 19u)];
                                                                    phi_8285_ = 0u;
                                                                    phi_8288_ = 0f;
                                                                    phi_8290_ = 1f;
                                                                    phi_8292_ = 0f;
                                                                    phi_8294_ = _e5883;
                                                                    loop {
                                                                        let _e5915 = phi_8285_;
                                                                        let _e5917 = phi_8288_;
                                                                        let _e5919 = phi_8290_;
                                                                        let _e5921 = phi_8292_;
                                                                        let _e5923 = phi_8294_;
                                                                        local_36 = _e5917;
                                                                        local_37 = _e5921;
                                                                        local_38 = _e5917;
                                                                        let _e5924 = (_e5915 < _e5881);
                                                                        if _e5924 {
                                                                            let _e5926 = (_e5923.x + _e5748);
                                                                            let _e5928 = (_e5923.y + _e5751);
                                                                            let _e5930 = (_e5923.z + _e5754);
                                                                            let _e5931 = floor(_e5926);
                                                                            let _e5932 = floor(_e5928);
                                                                            let _e5933 = floor(_e5930);
                                                                            let _e5942 = bitcast<u32>((select(0i, select(select(i32(_e5931), i32(-2147483648), (_e5931 < -2147483600f)), 2147483647i, (_e5931 > 2147483500f)), (_e5931 == _e5931)) & 255i));
                                                                            let _e5951 = bitcast<u32>((select(0i, select(select(i32(_e5932), i32(-2147483648), (_e5932 < -2147483600f)), 2147483647i, (_e5932 > 2147483500f)), (_e5932 == _e5932)) & 255i));
                                                                            let _e5960 = bitcast<u32>((select(0i, select(select(i32(_e5933), i32(-2147483648), (_e5933 < -2147483600f)), 2147483647i, (_e5933 > 2147483500f)), (_e5933 == _e5933)) & 255i));
                                                                            let _e5961 = (_e5926 - _e5931);
                                                                            let _e5962 = (_e5928 - _e5932);
                                                                            let _e5963 = (_e5930 - _e5933);
                                                                            let _e5970 = (((_e5961 * _e5961) * _e5961) * ((_e5961 * ((_e5961 * 6f) - 15f)) + 10f));
                                                                            let _e5977 = (((_e5962 * _e5962) * _e5962) * ((_e5962 * ((_e5962 * 6f) - 15f)) + 10f));
                                                                            let _e5987 = global_6.member[(_e5886 + _e5942)];
                                                                            let _e5993 = (select(select(u32(_e5987), 0u, (_e5987 < 0f)), 4294967295u, (_e5987 > 4294967000f)) + _e5951);
                                                                            let _e5996 = global_6.member[(_e5886 + _e5993)];
                                                                            let _e6002 = (select(select(u32(_e5996), 0u, (_e5996 < 0f)), 4294967295u, (_e5996 > 4294967000f)) + _e5960);
                                                                            let _e6006 = global_6.member[(_e5886 + (_e5993 + 1u))];
                                                                            let _e6012 = (select(select(u32(_e6006), 0u, (_e6006 < 0f)), 4294967295u, (_e6006 > 4294967000f)) + _e5960);
                                                                            let _e6016 = global_6.member[(_e5886 + (_e5942 + 1u))];
                                                                            let _e6022 = (select(select(u32(_e6016), 0u, (_e6016 < 0f)), 4294967295u, (_e6016 > 4294967000f)) + _e5951);
                                                                            let _e6025 = global_6.member[(_e5886 + _e6022)];
                                                                            let _e6031 = (select(select(u32(_e6025), 0u, (_e6025 < 0f)), 4294967295u, (_e6025 > 4294967000f)) + _e5960);
                                                                            let _e6035 = global_6.member[(_e5886 + (_e6022 + 1u))];
                                                                            let _e6041 = (select(select(u32(_e6035), 0u, (_e6035 < 0f)), 4294967295u, (_e6035 > 4294967000f)) + _e5960);
                                                                            let _e6044 = global_6.member[(_e5886 + _e6002)];
                                                                            let _e6050 = (select(select(u32(_e6044), 0u, (_e6044 < 0f)), 4294967295u, (_e6044 > 4294967000f)) % 12u);
                                                                            let _e6053 = select(-1f, 1f, ((_e6050 & 1u) == 0u));
                                                                            let _e6056 = select(-1f, 1f, ((_e6050 & 2u) == 0u));
                                                                            if (_e6050 < 4u) {
                                                                                phi_25674_ = ((_e6053 * _e5961) + (_e6056 * _e5962));
                                                                            } else {
                                                                                if (_e6050 < 8u) {
                                                                                    phi_25668_ = ((_e6053 * _e5961) + (_e6056 * _e5963));
                                                                                } else {
                                                                                    phi_25668_ = ((_e6053 * _e5962) + (_e6056 * _e5963));
                                                                                }
                                                                                let _e6066 = phi_25668_;
                                                                                phi_25674_ = _e6066;
                                                                            }
                                                                            let _e6071 = phi_25674_;
                                                                            let _e6074 = global_6.member[(_e5886 + _e6031)];
                                                                            let _e6080 = (_e5961 - 1f);
                                                                            let _e6081 = (select(select(u32(_e6074), 0u, (_e6074 < 0f)), 4294967295u, (_e6074 > 4294967000f)) % 12u);
                                                                            let _e6084 = select(-1f, 1f, ((_e6081 & 1u) == 0u));
                                                                            let _e6087 = select(-1f, 1f, ((_e6081 & 2u) == 0u));
                                                                            if (_e6081 < 4u) {
                                                                                phi_25708_ = ((_e6084 * _e6080) + (_e6087 * _e5962));
                                                                            } else {
                                                                                if (_e6081 < 8u) {
                                                                                    phi_25702_ = ((_e6084 * _e6080) + (_e6087 * _e5963));
                                                                                } else {
                                                                                    phi_25702_ = ((_e6084 * _e5962) + (_e6087 * _e5963));
                                                                                }
                                                                                let _e6097 = phi_25702_;
                                                                                phi_25708_ = _e6097;
                                                                            }
                                                                            let _e6102 = phi_25708_;
                                                                            let _e6105 = global_6.member[(_e5886 + _e6012)];
                                                                            let _e6111 = (_e5962 - 1f);
                                                                            let _e6112 = (select(select(u32(_e6105), 0u, (_e6105 < 0f)), 4294967295u, (_e6105 > 4294967000f)) % 12u);
                                                                            let _e6115 = select(-1f, 1f, ((_e6112 & 1u) == 0u));
                                                                            let _e6118 = select(-1f, 1f, ((_e6112 & 2u) == 0u));
                                                                            if (_e6112 < 4u) {
                                                                                phi_25742_ = ((_e6115 * _e5961) + (_e6118 * _e6111));
                                                                            } else {
                                                                                if (_e6112 < 8u) {
                                                                                    phi_25736_ = ((_e6115 * _e5961) + (_e6118 * _e5963));
                                                                                } else {
                                                                                    phi_25736_ = ((_e6115 * _e6111) + (_e6118 * _e5963));
                                                                                }
                                                                                let _e6128 = phi_25736_;
                                                                                phi_25742_ = _e6128;
                                                                            }
                                                                            let _e6133 = phi_25742_;
                                                                            let _e6136 = global_6.member[(_e5886 + _e6041)];
                                                                            let _e6142 = (select(select(u32(_e6136), 0u, (_e6136 < 0f)), 4294967295u, (_e6136 > 4294967000f)) % 12u);
                                                                            let _e6145 = select(-1f, 1f, ((_e6142 & 1u) == 0u));
                                                                            let _e6148 = select(-1f, 1f, ((_e6142 & 2u) == 0u));
                                                                            if (_e6142 < 4u) {
                                                                                phi_25776_ = ((_e6145 * _e6080) + (_e6148 * _e6111));
                                                                            } else {
                                                                                if (_e6142 < 8u) {
                                                                                    phi_25770_ = ((_e6145 * _e6080) + (_e6148 * _e5963));
                                                                                } else {
                                                                                    phi_25770_ = ((_e6145 * _e6111) + (_e6148 * _e5963));
                                                                                }
                                                                                let _e6158 = phi_25770_;
                                                                                phi_25776_ = _e6158;
                                                                            }
                                                                            let _e6163 = phi_25776_;
                                                                            let _e6167 = global_6.member[(_e5886 + (_e6002 + 1u))];
                                                                            let _e6173 = (_e5963 - 1f);
                                                                            let _e6174 = (select(select(u32(_e6167), 0u, (_e6167 < 0f)), 4294967295u, (_e6167 > 4294967000f)) % 12u);
                                                                            let _e6177 = select(-1f, 1f, ((_e6174 & 1u) == 0u));
                                                                            let _e6180 = select(-1f, 1f, ((_e6174 & 2u) == 0u));
                                                                            if (_e6174 < 4u) {
                                                                                phi_25810_ = ((_e6177 * _e5961) + (_e6180 * _e5962));
                                                                            } else {
                                                                                if (_e6174 < 8u) {
                                                                                    phi_25804_ = ((_e6177 * _e5961) + (_e6180 * _e6173));
                                                                                } else {
                                                                                    phi_25804_ = ((_e6177 * _e5962) + (_e6180 * _e6173));
                                                                                }
                                                                                let _e6190 = phi_25804_;
                                                                                phi_25810_ = _e6190;
                                                                            }
                                                                            let _e6195 = phi_25810_;
                                                                            let _e6199 = global_6.member[(_e5886 + (_e6031 + 1u))];
                                                                            let _e6205 = (select(select(u32(_e6199), 0u, (_e6199 < 0f)), 4294967295u, (_e6199 > 4294967000f)) % 12u);
                                                                            let _e6208 = select(-1f, 1f, ((_e6205 & 1u) == 0u));
                                                                            let _e6211 = select(-1f, 1f, ((_e6205 & 2u) == 0u));
                                                                            if (_e6205 < 4u) {
                                                                                phi_25844_ = ((_e6208 * _e6080) + (_e6211 * _e5962));
                                                                            } else {
                                                                                if (_e6205 < 8u) {
                                                                                    phi_25838_ = ((_e6208 * _e6080) + (_e6211 * _e6173));
                                                                                } else {
                                                                                    phi_25838_ = ((_e6208 * _e5962) + (_e6211 * _e6173));
                                                                                }
                                                                                let _e6221 = phi_25838_;
                                                                                phi_25844_ = _e6221;
                                                                            }
                                                                            let _e6226 = phi_25844_;
                                                                            let _e6230 = global_6.member[(_e5886 + (_e6012 + 1u))];
                                                                            let _e6236 = (select(select(u32(_e6230), 0u, (_e6230 < 0f)), 4294967295u, (_e6230 > 4294967000f)) % 12u);
                                                                            let _e6239 = select(-1f, 1f, ((_e6236 & 1u) == 0u));
                                                                            let _e6242 = select(-1f, 1f, ((_e6236 & 2u) == 0u));
                                                                            if (_e6236 < 4u) {
                                                                                phi_25878_ = ((_e6239 * _e5961) + (_e6242 * _e6111));
                                                                            } else {
                                                                                if (_e6236 < 8u) {
                                                                                    phi_25872_ = ((_e6239 * _e5961) + (_e6242 * _e6173));
                                                                                } else {
                                                                                    phi_25872_ = ((_e6239 * _e6111) + (_e6242 * _e6173));
                                                                                }
                                                                                let _e6252 = phi_25872_;
                                                                                phi_25878_ = _e6252;
                                                                            }
                                                                            let _e6257 = phi_25878_;
                                                                            let _e6261 = global_6.member[(_e5886 + (_e6041 + 1u))];
                                                                            let _e6267 = (select(select(u32(_e6261), 0u, (_e6261 < 0f)), 4294967295u, (_e6261 > 4294967000f)) % 12u);
                                                                            let _e6270 = select(-1f, 1f, ((_e6267 & 1u) == 0u));
                                                                            let _e6273 = select(-1f, 1f, ((_e6267 & 2u) == 0u));
                                                                            if (_e6267 < 4u) {
                                                                                phi_25912_ = ((_e6270 * _e6080) + (_e6273 * _e6111));
                                                                            } else {
                                                                                if (_e6267 < 8u) {
                                                                                    phi_25906_ = ((_e6270 * _e6080) + (_e6273 * _e6173));
                                                                                } else {
                                                                                    phi_25906_ = ((_e6270 * _e6111) + (_e6273 * _e6173));
                                                                                }
                                                                                let _e6283 = phi_25906_;
                                                                                phi_25912_ = _e6283;
                                                                            }
                                                                            let _e6288 = phi_25912_;
                                                                            let _e6291 = (_e6071 + (_e5970 * (_e6102 - _e6071)));
                                                                            let _e6297 = (_e6195 + (_e5970 * (_e6226 - _e6195)));
                                                                            let _e6303 = (_e6291 + (_e5977 * ((_e6133 + (_e5970 * (_e6163 - _e6133))) - _e6291)));
                                                                            phi_8286_ = (_e5915 + 1u);
                                                                            phi_8289_ = (_e5917 + _e5919);
                                                                            phi_8291_ = (_e5919 * 0.5f);
                                                                            phi_8293_ = (_e5921 + ((_e6303 + ((((_e5963 * _e5963) * _e5963) * ((_e5963 * ((_e5963 * 6f) - 15f)) + 10f)) * ((_e6297 + (_e5977 * ((_e6257 + (_e5970 * (_e6288 - _e6257))) - _e6297))) - _e6303))) * _e5919));
                                                                            phi_8295_ = vec3<f32>(((((_e5889 * _e5923.x) + (_e5892 * _e5923.y)) + (_e5895 * _e5923.z)) * 2.03f), ((((_e5898 * _e5923.x) + (_e5901 * _e5923.y)) + (_e5904 * _e5923.z)) * 2.03f), ((((_e5907 * _e5923.x) + (_e5910 * _e5923.y)) + (_e5913 * _e5923.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8286_ = u32();
                                                                            phi_8289_ = f32();
                                                                            phi_8291_ = f32();
                                                                            phi_8293_ = f32();
                                                                            phi_8295_ = vec3<f32>();
                                                                        }
                                                                        let _e6335 = phi_8286_;
                                                                        let _e6337 = phi_8289_;
                                                                        let _e6339 = phi_8291_;
                                                                        let _e6341 = phi_8293_;
                                                                        let _e6343 = phi_8295_;
                                                                        phi_8285_ = _e6335;
                                                                        phi_8288_ = _e6337;
                                                                        phi_8290_ = _e6339;
                                                                        phi_8292_ = _e6341;
                                                                        phi_8294_ = _e6343;
                                                                        if !(_e5924) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6346 = local_36;
                                                                    if (_e6346 > 0f) {
                                                                        let _e6349 = local_37;
                                                                        let _e6351 = local_38;
                                                                        phi_8708_ = (_e6349 / _e6351);
                                                                    } else {
                                                                        phi_8708_ = 0f;
                                                                    }
                                                                    let _e6354 = phi_8708_;
                                                                    if _e5824 {
                                                                        phi_8743_ = _e5816;
                                                                        phi_8744_ = _e5818;
                                                                        phi_8745_ = _e5820;
                                                                    } else {
                                                                        if (_e5814 == 1u) {
                                                                            phi_8740_ = vec3<f32>(_e6354, _e5816.y, _e5816.z);
                                                                            phi_8741_ = _e5818;
                                                                            phi_8742_ = _e5820;
                                                                        } else {
                                                                            if (_e5814 == 2u) {
                                                                                phi_8737_ = vec3<f32>(_e5816.x, _e6354, _e5816.z);
                                                                                phi_8738_ = _e5818;
                                                                                phi_8739_ = _e5820;
                                                                            } else {
                                                                                if (_e5814 == 3u) {
                                                                                    phi_8734_ = vec3<f32>(_e5816.x, _e5816.y, _e6354);
                                                                                    phi_8735_ = _e5818;
                                                                                    phi_8736_ = _e5820;
                                                                                } else {
                                                                                    let _e6358 = (_e5814 == 4u);
                                                                                    phi_8734_ = _e5816;
                                                                                    phi_8735_ = select(_e6354, _e5818, _e6358);
                                                                                    phi_8736_ = select(_e5820, _e6354, _e6358);
                                                                                }
                                                                                let _e6366 = phi_8734_;
                                                                                let _e6368 = phi_8735_;
                                                                                let _e6370 = phi_8736_;
                                                                                phi_8737_ = _e6366;
                                                                                phi_8738_ = _e6368;
                                                                                phi_8739_ = _e6370;
                                                                            }
                                                                            let _e6376 = phi_8737_;
                                                                            let _e6378 = phi_8738_;
                                                                            let _e6380 = phi_8739_;
                                                                            phi_8740_ = _e6376;
                                                                            phi_8741_ = _e6378;
                                                                            phi_8742_ = _e6380;
                                                                        }
                                                                        let _e6386 = phi_8740_;
                                                                        let _e6388 = phi_8741_;
                                                                        let _e6390 = phi_8742_;
                                                                        phi_8743_ = _e6386;
                                                                        phi_8744_ = _e6388;
                                                                        phi_8745_ = _e6390;
                                                                    }
                                                                    let _e6392 = phi_8743_;
                                                                    let _e6394 = phi_8744_;
                                                                    let _e6396 = phi_8745_;
                                                                    phi_8747_ = _e6392;
                                                                    phi_8748_ = _e6394;
                                                                    phi_8749_ = _e6396;
                                                                    phi_8750_ = select(_e5822, _e6354, _e5824);
                                                                } else {
                                                                    phi_8747_ = _e5816;
                                                                    phi_8748_ = _e5818;
                                                                    phi_8749_ = _e5820;
                                                                    phi_8750_ = _e5822;
                                                                }
                                                                let _e6399 = phi_8747_;
                                                                let _e6401 = phi_8748_;
                                                                let _e6403 = phi_8749_;
                                                                let _e6405 = phi_8750_;
                                                                phi_8050_ = (_e5814 + 1u);
                                                                phi_8053_ = _e6399;
                                                                phi_8055_ = _e6401;
                                                                phi_8057_ = _e6403;
                                                                phi_8059_ = _e6405;
                                                            } else {
                                                                phi_8050_ = u32();
                                                                phi_8053_ = vec3<f32>();
                                                                phi_8055_ = f32();
                                                                phi_8057_ = f32();
                                                                phi_8059_ = f32();
                                                            }
                                                            let _e6408 = phi_8050_;
                                                            let _e6410 = phi_8053_;
                                                            let _e6412 = phi_8055_;
                                                            let _e6414 = phi_8057_;
                                                            let _e6416 = phi_8059_;
                                                            phi_8049_ = _e6408;
                                                            phi_8052_ = _e6410;
                                                            phi_8054_ = _e6412;
                                                            phi_8056_ = _e6414;
                                                            phi_8058_ = _e6416;
                                                            if !(_e5823) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6419 = local_39;
                                                        let _e6422 = (_e5783 * (1f + (0.45f * _e6419)));
                                                        if _e5787 {
                                                            let _e6525 = local_43;
                                                            let _e6527 = ((_e6525 - -0.25f) * 1.4285715f);
                                                            let _e6529 = select(_e6527, 0f, (_e6527 < 0f));
                                                            let _e6531 = select(_e6529, 1f, (_e6529 > 1f));
                                                            let _e6535 = ((_e6531 * _e6531) * (3f - (2f * _e6531)));
                                                            let _e6537 = ((_e6422 - 1.05f) * -1.1764706f);
                                                            let _e6539 = select(_e6537, 0f, (_e6537 < 0f));
                                                            let _e6541 = select(_e6539, 1f, (_e6539 > 1f));
                                                            let _e6545 = ((_e6541 * _e6541) * (3f - (2f * _e6541)));
                                                            let _e6546 = (_e6535 * _e6545);
                                                            let _e6548 = ((_e6422 - 0.65f) * -1.5384616f);
                                                            let _e6550 = select(_e6548, 0f, (_e6548 < 0f));
                                                            let _e6552 = select(_e6550, 1f, (_e6550 > 1f));
                                                            let _e6557 = (((_e6552 * _e6552) * (3f - (2f * _e6552))) * _e6535);
                                                            let _e6559 = local_44;
                                                            let _e6561 = ((_e6559 - 0.1f) * 2.857143f);
                                                            let _e6563 = select(_e6561, 0f, (_e6561 < 0f));
                                                            let _e6565 = select(_e6563, 1f, (_e6563 > 1f));
                                                            let _e6570 = (((_e6565 * _e6565) * (3f - (2f * _e6565))) * _e6545);
                                                            let _e6575 = ((0.8f * _e6557) * _e6557);
                                                            let _e6584 = (_e5721 * exp((-2f * _e6570)));
                                                            phi_9042_ = type_59(vec3<f32>((((3.1815925f * _e6546) + (0.14634217f * _e6575)) * _e6584), (((0.38179109f * _e6546) + (1.2439084f * _e6575)) * _e6584), (((0.69995034f * _e6546) + (1.0975662f * _e6575)) * _e6584)), (_e5724 * _e6570));
                                                        } else {
                                                            if (_e5703 == 1u) {
                                                                let _e6495 = local_42;
                                                                let _e6497 = ((_e6495 - -0.3f) * 1.25f);
                                                                let _e6499 = select(_e6497, 0f, (_e6497 < 0f));
                                                                let _e6501 = select(_e6499, 1f, (_e6499 > 1f));
                                                                let _e6507 = ((_e6422 - 1f) * -1f);
                                                                let _e6509 = select(_e6507, 0f, (_e6507 < 0f));
                                                                let _e6511 = select(_e6509, 1f, (_e6509 > 1f));
                                                                let _e6517 = (((_e6501 * _e6501) * (3f - (2f * _e6501))) * pow(((_e6511 * _e6511) * (3f - (2f * _e6511))), 1.5f));
                                                                phi_9041_ = type_59((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5721 * _e6517)), (_e5724 * _e6517));
                                                            } else {
                                                                if (_e5703 == 2u) {
                                                                    let _e6451 = local_41;
                                                                    let _e6454 = (_e5783 * (1f + (0.15f * _e6451)));
                                                                    let _e6455 = (_e6454 - 0.65f);
                                                                    let _e6459 = exp(((_e6455 * -5.5555553f) * (_e6455 * 5.5555553f)));
                                                                    let _e6461 = ((_e6454 - 0.75f) * -1.8181818f);
                                                                    let _e6463 = select(_e6461, 0f, (_e6461 < 0f));
                                                                    let _e6465 = select(_e6463, 1f, (_e6463 > 1f));
                                                                    let _e6469 = ((_e6465 * _e6465) * (3f - (2f * _e6465)));
                                                                    let _e6474 = (exp(((_e5783 * -25f) * (_e5783 * 25f))) * 3f);
                                                                    phi_9040_ = type_59(vec3<f32>(((((3.1815925f * _e6459) + (_e6469 * 0.10243952f)) + _e6474) * _e5721), ((((0.38179109f * _e6459) + (_e6469 * 0.8707359f)) + _e6474) * _e5721), ((((0.69995034f * _e6459) + (_e6469 * 0.76829636f)) + _e6474) * _e5721)), 0f);
                                                                } else {
                                                                    let _e6426 = local_40;
                                                                    let _e6428 = ((_e6426 - -0.2f) * 1.6666666f);
                                                                    let _e6430 = select(_e6428, 0f, (_e6428 < 0f));
                                                                    let _e6432 = select(_e6430, 1f, (_e6430 > 1f));
                                                                    let _e6438 = ((_e6422 - 1f) * -1.4285715f);
                                                                    let _e6440 = select(_e6438, 0f, (_e6438 < 0f));
                                                                    let _e6442 = select(_e6440, 1f, (_e6440 > 1f));
                                                                    phi_9040_ = type_59(vec3<f32>(0f, 0f, 0f), (_e5724 * (((_e6432 * _e6432) * (3f - (2f * _e6432))) * ((_e6442 * _e6442) * (3f - (2f * _e6442))))));
                                                                }
                                                                let _e6493 = phi_9040_;
                                                                phi_9041_ = _e6493;
                                                            }
                                                            let _e6523 = phi_9041_;
                                                            phi_9042_ = _e6523;
                                                        }
                                                        let _e6592 = phi_9042_;
                                                        phi_9043_ = _e6592;
                                                    }
                                                    let _e6594 = phi_9043_;
                                                    phi_7722_ = (_e5689 + _e6594.member);
                                                    phi_7725_ = (_e5691 + 1u);
                                                    phi_7727_ = (_e5693 + _e6594.member_1);
                                                } else {
                                                    phi_7722_ = vec3<f32>();
                                                    phi_7725_ = u32();
                                                    phi_7727_ = f32();
                                                }
                                                let _e6601 = phi_7722_;
                                                let _e6603 = phi_7725_;
                                                let _e6605 = phi_7727_;
                                                phi_7721_ = _e6601;
                                                phi_7724_ = _e6603;
                                                phi_7726_ = _e6605;
                                                if !(_e5694) {
                                                    break;
                                                }
                                            }
                                            phi_34016_ = _e4073;
                                            if _e4073 {
                                                break;
                                            }
                                            let _e6608 = local_45;
                                            if (_e6608 <= 0f) {
                                                phi_9072_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6611 = local_46;
                                                let _e6614 = local_47;
                                                let _e6619 = local_48;
                                                phi_9072_ = vec3<f32>(exp((_e6614 * -0.81f)), exp(-(_e6611)), exp((_e6619 * -1.25f)));
                                            }
                                            let _e6624 = phi_9072_;
                                            let _e6626 = local_49;
                                            let _e6631 = local_50;
                                            let _e6636 = local_51;
                                            let _e6641 = local_52;
                                            let _e6645 = local_53;
                                            let _e6649 = local_54;
                                            let _e6654 = global_6.member[(_e1298 + 9u)];
                                            phi_34025_ = _e4073;
                                            phi_9119_ = vec3<f32>((_e3778.x + ((((_e6626.x * _e6624.x) + _e6641.x) * _e6654) * _e3780.x)), (_e3778.y + ((((_e6631.y * _e6624.y) + _e6645.y) * _e6654) * _e3780.y)), (_e3778.z + ((((_e6636.z * _e6624.z) + _e6649.z) * _e6654) * _e3780.z)));
                                        } else {
                                            phi_34025_ = _e3776;
                                            phi_9119_ = _e3778;
                                        }
                                        let _e6672 = phi_34025_;
                                        let _e6674 = phi_9119_;
                                        phi_34024_ = _e6672;
                                        phi_9120_ = _e6674;
                                    } else {
                                        phi_34024_ = _e3776;
                                        phi_9120_ = _e3784;
                                    }
                                    let _e6676 = phi_34024_;
                                    let _e6678 = phi_9120_;
                                    phi_34023_ = _e6676;
                                    phi_9121_ = _e6678;
                                } else {
                                    phi_34023_ = _e329;
                                    phi_9121_ = _e1443;
                                }
                                let _e6680 = phi_34023_;
                                let _e6682 = phi_9121_;
                                phi_34019_ = _e6680;
                                phi_10394_ = (vec3<f32>(_e340.member_2.x, _e340.member_2.y, _e340.member_2.z) * _e6682);
                            } else {
                                let _e356 = global_1.member.member_8[1u];
                                let _e366 = global_6.member[_e356];
                                let _e369 = global_6.member[(_e356 + 1u)];
                                let _e372 = global_6.member[(_e356 + 2u)];
                                let _e375 = global_6.member[(_e356 + 3u)];
                                let _e378 = global_6.member[(_e356 + 4u)];
                                let _e381 = global_6.member[(_e356 + 5u)];
                                let _e384 = global_6.member[(_e356 + 6u)];
                                let _e387 = global_6.member[(_e356 + 7u)];
                                let _e390 = global_6.member[(_e356 + 8u)];
                                let _e393 = global_6.member[(_e356 + 9u)];
                                let _e396 = global_6.member[(_e356 + 10u)];
                                let _e399 = global_6.member[(_e356 + 11u)];
                                let _e402 = global_6.member[(_e356 + 12u)];
                                let _e405 = global_6.member[(_e356 + 13u)];
                                let _e408 = global_6.member[(_e356 + 14u)];
                                let _e411 = global_6.member[(_e356 + 15u)];
                                let _e415 = global_6.member[(_e356 + 16u)];
                                let _e418 = global_6.member[(_e356 + 17u)];
                                let _e421 = global_6.member[(_e356 + 18u)];
                                let _e424 = global_6.member[(_e356 + 19u)];
                                let _e427 = global_6.member[(_e356 + 20u)];
                                let _e430 = global_6.member[(_e356 + 29u)];
                                let _e431 = bitcast<u32>(_e430);
                                let _e434 = global_6.member[(_e356 + 30u)];
                                let _e435 = bitcast<u32>(_e434);
                                let _e443 = ((((_e340.member.x - _e366) * _e340.member_1.x) + ((_e340.member.y - _e369) * _e340.member_1.y)) + ((_e340.member.z - _e372) * _e340.member_1.z));
                                let _e455 = ((((_e340.member.x * _e340.member.x) + (_e340.member.y * _e340.member.y)) + (_e340.member.z * _e340.member.z)) - (2f * (((_e340.member.x * _e366) + (_e340.member.y * _e369)) + (_e340.member.z * _e372))));
                                let _e456 = (_e455 + _e384);
                                let _e457 = (_e443 * _e443);
                                let _e458 = (_e457 - _e456);
                                if (_e458 < 0f) {
                                    phi_21817_ = type_57(0u, type_57().member_1);
                                    phi_21818_ = type_57();
                                    phi_21819_ = true;
                                } else {
                                    if (_e443 < 0f) {
                                        phi_21789_ = -(sqrt(_e458));
                                    } else {
                                        phi_21789_ = sqrt(_e458);
                                    }
                                    let _e465 = phi_21789_;
                                    let _e467 = -((_e443 + _e465));
                                    let _e468 = (_e467 == 0f);
                                    if _e468 {
                                        phi_21812_ = type_57(1u, type_56(0f, 0f));
                                        phi_21813_ = type_57();
                                    } else {
                                        let _e469 = (_e456 / _e467);
                                        if (_e467 < _e469) {
                                            phi_21801_ = type_56(_e467, _e469);
                                        } else {
                                            phi_21801_ = type_56(_e469, _e467);
                                        }
                                        let _e474 = phi_21801_;
                                        phi_21812_ = type_57();
                                        phi_21813_ = type_57(1u, _e474);
                                    }
                                    let _e477 = phi_21812_;
                                    let _e479 = phi_21813_;
                                    phi_21817_ = _e477;
                                    phi_21818_ = _e479;
                                    phi_21819_ = _e468;
                                }
                                let _e484 = phi_21817_;
                                let _e486 = phi_21818_;
                                let _e488 = phi_21819_;
                                if _e488 {
                                    phi_21823_ = _e484;
                                } else {
                                    phi_21823_ = _e486;
                                }
                                let _e490 = phi_21823_;
                                if (bitcast<i32>(_e490.member) != 0i) {
                                    phi_21710_ = type_57(0u, _e490.member_1);
                                } else {
                                    phi_21710_ = type_57(1u, type_57().member_1);
                                }
                                let _e500 = phi_21710_;
                                if (bitcast<i32>(_e500.member) != 0i) {
                                    phi_21757_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e500.member_1.member_1 <= 0f) {
                                        phi_21753_ = type_57(0u, type_57().member_1);
                                    } else {
                                        if (_e500.member_1.member != _e500.member_1.member) {
                                            phi_21861_ = true;
                                        } else {
                                            phi_21861_ = (0f >= _e500.member_1.member);
                                        }
                                        let _e512 = phi_21861_;
                                        let _e513 = select(_e500.member_1.member, 0f, _e512);
                                        let _e514 = (_e455 + _e381);
                                        if (_e514 > 0f) {
                                            let _e516 = (_e457 - _e514);
                                            if (_e516 < 0f) {
                                                phi_21969_ = type_57(0u, type_57().member_1);
                                                phi_21970_ = type_57();
                                                phi_21971_ = true;
                                            } else {
                                                if (_e443 < 0f) {
                                                    phi_21941_ = -(sqrt(_e516));
                                                } else {
                                                    phi_21941_ = sqrt(_e516);
                                                }
                                                let _e523 = phi_21941_;
                                                let _e525 = -((_e443 + _e523));
                                                let _e526 = (_e525 == 0f);
                                                if _e526 {
                                                    phi_21964_ = type_57(1u, type_56(0f, 0f));
                                                    phi_21965_ = type_57();
                                                } else {
                                                    let _e527 = (_e514 / _e525);
                                                    if (_e525 < _e527) {
                                                        phi_21953_ = type_56(_e525, _e527);
                                                    } else {
                                                        phi_21953_ = type_56(_e527, _e525);
                                                    }
                                                    let _e532 = phi_21953_;
                                                    phi_21964_ = type_57();
                                                    phi_21965_ = type_57(1u, _e532);
                                                }
                                                let _e535 = phi_21964_;
                                                let _e537 = phi_21965_;
                                                phi_21969_ = _e535;
                                                phi_21970_ = _e537;
                                                phi_21971_ = _e526;
                                            }
                                            let _e542 = phi_21969_;
                                            let _e544 = phi_21970_;
                                            let _e546 = phi_21971_;
                                            if _e546 {
                                                phi_21975_ = _e542;
                                            } else {
                                                phi_21975_ = _e544;
                                            }
                                            let _e548 = phi_21975_;
                                            if (bitcast<i32>(_e548.member) != 0i) {
                                                phi_21909_ = select(_e548.member_1.member_1, _e548.member_1.member, (_e548.member_1.member >= 0f));
                                            } else {
                                                phi_21909_ = -1f;
                                            }
                                            let _e559 = phi_21909_;
                                            if (_e559 >= 0f) {
                                                if (_e500.member_1.member_1 != _e500.member_1.member_1) {
                                                    phi_22013_ = true;
                                                } else {
                                                    phi_22013_ = (_e559 <= _e500.member_1.member_1);
                                                }
                                                let _e564 = phi_22013_;
                                                phi_21738_ = select(_e500.member_1.member_1, _e559, _e564);
                                            } else {
                                                phi_21738_ = _e500.member_1.member_1;
                                            }
                                            let _e567 = phi_21738_;
                                            phi_21740_ = _e567;
                                        } else {
                                            phi_21740_ = _e500.member_1.member_1;
                                        }
                                        let _e569 = phi_21740_;
                                        if (_e569 <= _e513) {
                                            phi_21749_ = type_57(0u, type_57().member_1);
                                        } else {
                                            phi_21749_ = type_57(1u, type_56(_e513, _e569));
                                        }
                                        let _e577 = phi_21749_;
                                        phi_21753_ = _e577;
                                    }
                                    let _e582 = phi_21753_;
                                    phi_21757_ = _e582;
                                }
                                let _e587 = phi_21757_;
                                if (bitcast<i32>(_e587.member) != 0i) {
                                    let _e593 = (_e587.member_1.member < _e340.member.w);
                                    if _e593 {
                                        if (_e587.member_1.member_1 != _e587.member_1.member_1) {
                                            phi_22028_ = true;
                                        } else {
                                            phi_22028_ = (_e340.member.w <= _e587.member_1.member_1);
                                        }
                                        let _e599 = phi_22028_;
                                        let _e600 = select(_e587.member_1.member_1, _e340.member.w, _e599);
                                        phi_9515_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9518_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9520_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9522_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9524_ = 0u;
                                        loop {
                                            let _e602 = phi_9515_;
                                            let _e604 = phi_9518_;
                                            let _e606 = phi_9520_;
                                            let _e608 = phi_9522_;
                                            let _e610 = phi_9524_;
                                            local = _e606;
                                            local_1 = _e604;
                                            local_2 = _e602;
                                            local_3 = _e602;
                                            local_4 = _e602;
                                            local_5 = _e608;
                                            local_6 = _e608;
                                            local_7 = _e608;
                                            let _e611 = (_e610 < 32u);
                                            if _e611 {
                                                let _e612 = -(_e443);
                                                if (_e587.member_1.member <= _e600) {
                                                } else {
                                                    phi_33538_ = true;
                                                    break;
                                                }
                                                let _e615 = select(_e612, _e587.member_1.member, (_e612 < _e587.member_1.member));
                                                let _e617 = select(_e615, _e600, (_e615 > _e600));
                                                let _e618 = (_e617 - _e587.member_1.member);
                                                let _e619 = (_e600 - _e617);
                                                let _e620 = (_e600 - _e587.member_1.member);
                                                let _e621 = (0.000000001f * _e620);
                                                if (_e618 <= _e621) {
                                                    let _e696 = (f32(_e610) * 0.03125f);
                                                    let _e699 = (f32((_e610 + 1u)) * 0.03125f);
                                                    let _e701 = ((_e696 + _e699) * 0.5f);
                                                    phi_9625_ = type_56((_e587.member_1.member + ((_e620 * _e701) * _e701)), (_e620 * ((_e699 * _e699) - (_e696 * _e696))));
                                                } else {
                                                    if (_e619 <= _e621) {
                                                        let _e679 = (f32((31u - _e610)) * 0.03125f);
                                                        let _e682 = (f32((32u - _e610)) * 0.03125f);
                                                        let _e684 = ((_e679 + _e682) * 0.5f);
                                                        phi_9624_ = type_56((_e600 - ((_e620 * _e684) * _e684)), (_e620 * ((_e682 * _e682) - (_e679 * _e679))));
                                                    } else {
                                                        let _e627 = round(((32f * _e618) / (_e618 + _e619)));
                                                        let _e632 = select(select(u32(_e627), 0u, (_e627 < 0f)), 4294967295u, (_e627 > 4294967000f));
                                                        if (_e632 < 1u) {
                                                            phi_9608_ = 1u;
                                                        } else {
                                                            phi_9608_ = select(_e632, 31u, (_e632 > 31u));
                                                        }
                                                        let _e637 = phi_9608_;
                                                        if (_e610 < _e637) {
                                                            let _e658 = ((_e637 - 1u) - _e610);
                                                            let _e660 = f32(_e637);
                                                            let _e661 = (f32(_e658) / _e660);
                                                            let _e664 = (f32((_e658 + 1u)) / _e660);
                                                            let _e666 = ((_e661 + _e664) * 0.5f);
                                                            phi_9623_ = type_56((_e617 - ((_e618 * _e666) * _e666)), (_e618 * ((_e664 * _e664) - (_e661 * _e661))));
                                                        } else {
                                                            let _e640 = (_e610 - _e637);
                                                            let _e642 = f32((32u - _e637));
                                                            let _e643 = (f32(_e640) / _e642);
                                                            let _e646 = (f32((_e640 + 1u)) / _e642);
                                                            let _e648 = ((_e643 + _e646) * 0.5f);
                                                            phi_9623_ = type_56((_e617 + ((_e619 * _e648) * _e648)), (_e619 * ((_e646 * _e646) - (_e643 * _e643))));
                                                        }
                                                        let _e676 = phi_9623_;
                                                        phi_9624_ = _e676;
                                                    }
                                                    let _e694 = phi_9624_;
                                                    phi_9625_ = _e694;
                                                }
                                                let _e711 = phi_9625_;
                                                let _e717 = (_e340.member.x + (_e340.member_1.x * _e711.member));
                                                let _e718 = (_e340.member.y + (_e340.member_1.y * _e711.member));
                                                let _e719 = (_e340.member.z + (_e340.member_1.z * _e711.member));
                                                let _e732 = ((((_e717 * _e717) + (_e718 * _e718)) + (_e719 * _e719)) - (2f * (((_e717 * _e366) + (_e718 * _e369)) + (_e719 * _e372))));
                                                let _e734 = (_e717 - _e366);
                                                let _e735 = (_e718 - _e369);
                                                let _e736 = (_e719 - _e372);
                                                let _e742 = sqrt((((_e734 * _e734) + (_e735 * _e735)) + (_e736 * _e736)));
                                                let _e744 = ((_e732 + _e381) / (_e742 + _e375));
                                                let _e748 = (1f - (abs((_e744 - 25000f)) * 0.00006666667f));
                                                if (_e748 != _e748) {
                                                    phi_22228_ = true;
                                                } else {
                                                    phi_22228_ = (0f >= _e748);
                                                }
                                                let _e752 = phi_22228_;
                                                let _e754 = -(_e744);
                                                let _e757 = (exp((_e754 / _e396)) * _e711.member_1);
                                                let _e760 = (exp((_e754 / _e399)) * _e711.member_1);
                                                let _e762 = (_e757 * 0.5f);
                                                let _e763 = (_e760 * 0.5f);
                                                let _e764 = ((select(_e748, 0f, _e752) * _e711.member_1) * 0.5f);
                                                let _e766 = (_e608.x + _e762);
                                                let _e768 = (_e608.y + _e763);
                                                let _e770 = (_e608.z + _e764);
                                                let _e775 = (_e415 * 1.1f);
                                                let _e776 = (_e775 * _e768);
                                                let _e795 = (_e415 * _e760);
                                                if (_e435 == 4294967295u) {
                                                    phi_9953_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e803 = (1f / _e742);
                                                    let _e811 = ((((_e734 * _e803) * _e387) + ((_e735 * _e803) * _e390)) + ((_e736 * _e803) * _e393));
                                                    if (_e744 != _e744) {
                                                        phi_22355_ = true;
                                                    } else {
                                                        phi_22355_ = (0f >= _e744);
                                                    }
                                                    let _e815 = phi_22355_;
                                                    let _e819 = sqrt((select(_e744, 0f, _e815) / (_e378 - _e375)));
                                                    if (_e819 != _e819) {
                                                        phi_22370_ = true;
                                                    } else {
                                                        phi_22370_ = (1f <= _e819);
                                                    }
                                                    let _e823 = phi_22370_;
                                                    let _e825 = (select(_e819, 1f, _e823) * 31f);
                                                    let _e827 = select(_e811, -1f, (_e811 < -1f));
                                                    let _e831 = ((select(_e827, 1f, (_e827 > 1f)) + 1f) * 15.5f);
                                                    let _e836 = select(select(u32(_e825), 0u, (_e825 < 0f)), 4294967295u, (_e825 > 4294967000f));
                                                    let _e838 = select(_e836, 30u, (30u < _e836));
                                                    let _e843 = select(select(u32(_e831), 0u, (_e831 < 0f)), 4294967295u, (_e831 > 4294967000f));
                                                    let _e845 = select(_e843, 30u, (30u < _e843));
                                                    let _e847 = (_e825 - f32(_e838));
                                                    let _e849 = (_e831 - f32(_e845));
                                                    let _e853 = (_e435 + (3u * ((_e838 * 32u) + _e845)));
                                                    let _e858 = (_e435 + (3u * (((_e838 + 1u) * 32u) + _e845)));
                                                    let _e860 = global_6.member[_e853];
                                                    let _e863 = global_6.member[(_e853 + 1u)];
                                                    let _e866 = global_6.member[(_e853 + 2u)];
                                                    let _e867 = (1f - _e849);
                                                    let _e873 = global_6.member[(_e853 + 3u)];
                                                    let _e876 = global_6.member[(_e853 + 4u)];
                                                    let _e879 = global_6.member[(_e853 + 5u)];
                                                    let _e886 = (1f - _e847);
                                                    let _e891 = global_6.member[_e858];
                                                    let _e894 = global_6.member[(_e858 + 1u)];
                                                    let _e897 = global_6.member[(_e858 + 2u)];
                                                    let _e903 = global_6.member[(_e858 + 3u)];
                                                    let _e906 = global_6.member[(_e858 + 4u)];
                                                    let _e909 = global_6.member[(_e858 + 5u)];
                                                    phi_9953_ = vec3<f32>(((((_e860 * _e867) + (_e873 * _e849)) * _e886) + (((_e891 * _e867) + (_e903 * _e849)) * _e847)), ((((_e863 * _e867) + (_e876 * _e849)) * _e886) + (((_e894 * _e867) + (_e906 * _e849)) * _e847)), ((((_e866 * _e867) + (_e879 * _e849)) * _e886) + (((_e897 * _e867) + (_e909 * _e849)) * _e847)));
                                                }
                                                let _e924 = phi_9953_;
                                                let _e938 = (_e732 + _e384);
                                                if (_e938 > 0f) {
                                                    let _e944 = (((_e734 * _e387) + (_e735 * _e390)) + (_e736 * _e393));
                                                    let _e946 = ((_e944 * _e944) - _e938);
                                                    if (_e944 >= 0f) {
                                                        phi_10027_ = vec3<f32>();
                                                        phi_10028_ = true;
                                                    } else {
                                                        let _e948 = (_e946 <= 0f);
                                                        if _e948 {
                                                            phi_10026_ = vec3<f32>();
                                                        } else {
                                                            let _e951 = (-(_e944) - sqrt(_e946));
                                                            phi_10026_ = vec3<f32>((_e717 + (_e387 * _e951)), (_e718 + (_e390 * _e951)), (_e719 + (_e393 * _e951)));
                                                        }
                                                        let _e960 = phi_10026_;
                                                        phi_10027_ = _e960;
                                                        phi_10028_ = _e948;
                                                    }
                                                    let _e962 = phi_10027_;
                                                    let _e964 = phi_10028_;
                                                    phi_10033_ = _e962;
                                                    phi_10034_ = select(true, false, _e964);
                                                } else {
                                                    phi_10033_ = vec3<f32>(_e717, _e718, _e719);
                                                    phi_10034_ = true;
                                                }
                                                let _e967 = phi_10033_;
                                                let _e969 = phi_10034_;
                                                if _e969 {
                                                    let _e971 = (_e967.x - _e366);
                                                    let _e973 = (_e967.y - _e369);
                                                    let _e975 = (_e967.z - _e372);
                                                    let _e981 = sqrt((((_e971 * _e971) + (_e973 * _e973)) + (_e975 * _e975)));
                                                    let _e982 = (1f / _e981);
                                                    let _e1000 = ((((((_e967.x * _e967.x) + (_e967.y * _e967.y)) + (_e967.z * _e967.z)) - (2f * (((_e967.x * _e366) + (_e967.y * _e369)) + (_e967.z * _e372)))) + _e381) / (_e981 + _e375));
                                                    if (_e1000 != _e1000) {
                                                        phi_22494_ = true;
                                                    } else {
                                                        phi_22494_ = (0f >= _e1000);
                                                    }
                                                    let _e1004 = phi_22494_;
                                                    let _e1008 = sqrt((select(_e1000, 0f, _e1004) / (_e378 - _e375)));
                                                    if (_e1008 != _e1008) {
                                                        phi_22509_ = true;
                                                    } else {
                                                        phi_22509_ = (1f <= _e1008);
                                                    }
                                                    let _e1012 = phi_22509_;
                                                    let _e1014 = (select(_e1008, 1f, _e1012) * 63f);
                                                    let _e1019 = ((((_e971 * _e982) * _e387) + ((_e973 * _e982) * _e390)) + ((_e975 * _e982) * _e393));
                                                    let _e1021 = select(_e1019, -1f, (_e1019 < -1f));
                                                    let _e1025 = ((select(_e1021, 1f, (_e1021 > 1f)) + 1f) * 127.5f);
                                                    let _e1030 = select(select(u32(_e1014), 0u, (_e1014 < 0f)), 4294967295u, (_e1014 > 4294967000f));
                                                    let _e1032 = select(_e1030, 62u, (62u < _e1030));
                                                    let _e1037 = select(select(u32(_e1025), 0u, (_e1025 < 0f)), 4294967295u, (_e1025 > 4294967000f));
                                                    let _e1039 = select(_e1037, 254u, (254u < _e1037));
                                                    let _e1041 = (_e1014 - f32(_e1032));
                                                    let _e1043 = (_e1025 - f32(_e1039));
                                                    let _e1047 = (_e431 + (3u * ((_e1032 * 256u) + _e1039)));
                                                    let _e1052 = (_e431 + (3u * (((_e1032 + 1u) * 256u) + _e1039)));
                                                    let _e1054 = global_6.member[_e1047];
                                                    let _e1057 = global_6.member[(_e1047 + 1u)];
                                                    let _e1060 = global_6.member[(_e1047 + 2u)];
                                                    let _e1061 = (1f - _e1043);
                                                    let _e1067 = global_6.member[(_e1047 + 3u)];
                                                    let _e1070 = global_6.member[(_e1047 + 4u)];
                                                    let _e1073 = global_6.member[(_e1047 + 5u)];
                                                    let _e1080 = (1f - _e1041);
                                                    let _e1085 = global_6.member[_e1052];
                                                    let _e1088 = global_6.member[(_e1052 + 1u)];
                                                    let _e1091 = global_6.member[(_e1052 + 2u)];
                                                    let _e1097 = global_6.member[(_e1052 + 3u)];
                                                    let _e1100 = global_6.member[(_e1052 + 4u)];
                                                    let _e1103 = global_6.member[(_e1052 + 5u)];
                                                    phi_10325_ = vec3<f32>(((((_e1054 * _e1061) + (_e1067 * _e1043)) * _e1080) + (((_e1085 * _e1061) + (_e1097 * _e1043)) * _e1041)), ((((_e1057 * _e1061) + (_e1070 * _e1043)) * _e1080) + (((_e1088 * _e1061) + (_e1100 * _e1043)) * _e1041)), ((((_e1060 * _e1061) + (_e1073 * _e1043)) * _e1080) + (((_e1091 * _e1061) + (_e1103 * _e1043)) * _e1041)));
                                                } else {
                                                    phi_10325_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1118 = phi_10325_;
                                                let _e1119 = (vec3<f32>(_e766, _e768, _e770) + _e1118);
                                                let _e1125 = (_e775 * _e1119.y);
                                                let _e1137 = exp(-((((_e405 * _e1119.x) + _e1125) + (_e421 * _e1119.z))));
                                                let _e1139 = exp(-((((_e408 * _e1119.x) + _e1125) + (_e424 * _e1119.z))));
                                                let _e1141 = exp(-((((_e411 * _e1119.x) + _e1125) + (_e427 * _e1119.z))));
                                                phi_9516_ = vec3<f32>((_e602.x + ((exp(-((((_e405 * _e766) + _e776) + (_e421 * _e770)))) * ((_e405 * _e757) + _e795)) * _e924.x)), (_e602.y + ((exp(-((((_e408 * _e766) + _e776) + (_e424 * _e770)))) * ((_e408 * _e757) + _e795)) * _e924.y)), (_e602.z + ((exp(-((((_e411 * _e766) + _e776) + (_e427 * _e770)))) * ((_e411 * _e757) + _e795)) * _e924.z)));
                                                phi_9519_ = vec3<f32>((_e604.x + (_e1137 * _e760)), (_e604.y + (_e1139 * _e760)), (_e604.z + (_e1141 * _e760)));
                                                phi_9521_ = vec3<f32>((_e606.x + (_e1137 * _e757)), (_e606.y + (_e1139 * _e757)), (_e606.z + (_e1141 * _e757)));
                                                phi_9523_ = vec3<f32>((_e766 + _e762), (_e768 + _e763), (_e770 + _e764));
                                                phi_9525_ = (_e610 + 1u);
                                            } else {
                                                phi_9516_ = vec3<f32>();
                                                phi_9519_ = vec3<f32>();
                                                phi_9521_ = vec3<f32>();
                                                phi_9523_ = vec3<f32>();
                                                phi_9525_ = u32();
                                            }
                                            let _e1168 = phi_9516_;
                                            let _e1170 = phi_9519_;
                                            let _e1172 = phi_9521_;
                                            let _e1174 = phi_9523_;
                                            let _e1176 = phi_9525_;
                                            phi_9515_ = _e1168;
                                            phi_9518_ = _e1170;
                                            phi_9520_ = _e1172;
                                            phi_9522_ = _e1174;
                                            phi_9524_ = _e1176;
                                            phi_33538_ = _e329;
                                            if !(_e611) {
                                                break;
                                            }
                                        }
                                        let _e1179 = phi_33538_;
                                        phi_34016_ = _e1179;
                                        if _e1179 {
                                            break;
                                        }
                                        let _e1181 = local;
                                        let _e1182 = (_e1181 * vec3<f32>(_e405, _e408, _e411));
                                        let _e1184 = local_1;
                                        let _e1185 = (_e1184 * _e415);
                                        let _e1190 = (((_e340.member_1.x * _e387) + (_e340.member_1.y * _e390)) + (_e340.member_1.z * _e393));
                                        let _e1192 = (1f + (_e1190 * _e1190));
                                        let _e1193 = (0.059683103f * _e1192);
                                        let _e1200 = (_e418 * _e418);
                                        let _e1211 = ((0.119366206f * ((1f - _e1200) * _e1192)) / ((2f + _e1200) * pow(((1f + _e1200) - ((2f * _e418) * _e1190)), 1.5f)));
                                        let _e1222 = local_2;
                                        let _e1226 = local_3;
                                        let _e1230 = local_4;
                                        let _e1239 = local_5;
                                        let _e1246 = local_6;
                                        let _e1248 = ((_e415 * 1.1f) * _e1246.y);
                                        let _e1253 = local_7;
                                        phi_34022_ = _e1179;
                                        phi_10382_ = type_60(vec3<f32>(exp(-((((_e405 * _e1239.x) + _e1248) + (_e421 * _e1253.z)))), exp(-((((_e408 * _e1239.x) + _e1248) + (_e424 * _e1253.z)))), exp(-((((_e411 * _e1239.x) + _e1248) + (_e427 * _e1253.z))))), (vec3<f32>(((((_e1182.x * _e1193) + (_e1185.x * _e1211)) + _e1222.x) * _e402), ((((_e1182.y * _e1193) + (_e1185.y * _e1211)) + _e1226.y) * _e402), ((((_e1182.z * _e1193) + (_e1185.z * _e1211)) + _e1230.z) * _e402)) * 0.38317f));
                                    } else {
                                        phi_34022_ = _e329;
                                        phi_10382_ = type_60();
                                    }
                                    let _e1270 = phi_34022_;
                                    let _e1272 = phi_10382_;
                                    phi_34021_ = _e1270;
                                    phi_10384_ = _e1272;
                                    phi_10385_ = select(true, false, _e593);
                                } else {
                                    phi_34021_ = _e329;
                                    phi_10384_ = type_60();
                                    phi_10385_ = true;
                                }
                                let _e1275 = phi_34021_;
                                let _e1277 = phi_10384_;
                                let _e1279 = phi_10385_;
                                if _e1279 {
                                    phi_10390_ = type_60(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10390_ = _e1277;
                                }
                                let _e1281 = phi_10390_;
                                phi_34019_ = _e1275;
                                phi_10394_ = (vec3<f32>(_e340.member_2.x, _e340.member_2.y, _e340.member_2.z) * _e1281.member_1);
                            }
                            let _e6686 = phi_34019_;
                            let _e6688 = phi_10394_;
                            phi_34018_ = _e6686;
                            phi_10395_ = _e6688;
                        }
                        let _e6690 = phi_34018_;
                        let _e6692 = phi_10395_;
                        phi_34017_ = _e6690;
                        phi_513_ = (_e331 + _e6692);
                        phi_516_ = (_e333 + 1u);
                    } else {
                        phi_34017_ = _e329;
                        phi_513_ = vec3<f32>();
                        phi_516_ = u32();
                    }
                    let _e6696 = phi_34017_;
                    let _e6698 = phi_513_;
                    let _e6700 = phi_516_;
                    phi_33555_ = _e6696;
                    phi_512_ = _e6698;
                    phi_515_ = _e6700;
                    phi_34016_ = _e6696;
                    if !(_e334) {
                        break;
                    }
                }
                let _e6703 = phi_34016_;
                if _e6703 {
                    break;
                }
                let _e6705 = local_55;
                let _e6709 = local_56;
                let _e6714 = local_57;
                if ((((_e6705.x * _e6705.x) + (_e6709.y * _e6709.y)) + (_e6714.z * _e6714.z)) > 0f) {
                    if (_e323.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6724 = global_7.member[_e323.member_1].member_3[0u];
                    global_7.member[_e323.member_1].member_3[0u] = (_e6724 + _e6705.x);
                    let _e6730 = global_7.member[_e323.member_1].member_3[1u];
                    global_7.member[_e323.member_1].member_3[1u] = (_e6730 + _e6709.y);
                    let _e6736 = global_7.member[_e323.member_1].member_3[2u];
                    global_7.member[_e323.member_1].member_3[2u] = (_e6736 + _e6714.z);
                    let _e6742 = global_7.member[_e323.member_1].member_3[3u];
                    global_7.member[_e323.member_1].member_3[3u] = _e6742;
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
    var phi_11199_: type_48;
    var phi_26107_: f32;
    var phi_26119_: type_56;
    var phi_26130_: type_57;
    var phi_26131_: type_57;
    var phi_26135_: type_57;
    var phi_26136_: type_57;
    var phi_26137_: bool;
    var phi_26141_: type_57;
    var phi_26028_: type_57;
    var phi_26179_: bool;
    var phi_26259_: f32;
    var phi_26271_: type_56;
    var phi_26282_: type_57;
    var phi_26283_: type_57;
    var phi_26287_: type_57;
    var phi_26288_: type_57;
    var phi_26289_: bool;
    var phi_26293_: type_57;
    var phi_26227_: f32;
    var phi_26331_: bool;
    var phi_26056_: f32;
    var phi_26058_: f32;
    var phi_26067_: type_57;
    var phi_26071_: type_57;
    var phi_26075_: type_57;
    var phi_26346_: bool;
    var phi_12728_: vec3<f32>;
    var phi_12731_: u32;
    var phi_12815_: u32;
    var phi_12830_: type_56;
    var phi_12831_: type_56;
    var phi_12832_: type_56;
    var phi_26545_: bool;
    var phi_12729_: vec3<f32>;
    var phi_12732_: u32;
    var phi_34027_: bool;
    var local_69: vec3<f32>;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var phi_36167_: bool;
    var phi_12857_: vec3<f32>;
    var phi_36166_: bool;
    var phi_12859_: vec3<f32>;
    var phi_12860_: bool;
    var phi_36164_: bool;
    var phi_12867_: vec3<f32>;
    var phi_12868_: type_41;
    var phi_36163_: bool;
    var phi_12869_: vec3<f32>;
    var phi_12870_: type_41;
    var phi_13149_: type_60;
    var phi_13116_: vec3<f32>;
    var phi_13117_: vec3<f32>;
    var phi_13150_: type_60;
    var phi_13273_: i32;
    var phi_13276_: u32;
    var phi_13330_: bool;
    var phi_13336_: bool;
    var phi_13343_: u32;
    var phi_13345_: i32;
    var phi_13346_: u32;
    var phi_13274_: i32;
    var phi_13277_: u32;
    var phi_13348_: bool;
    var local_72: u32;
    var phi_13902_: vec3<f32>;
    var phi_13916_: vec3<f32>;
    var phi_26612_: bool;
    var phi_13949_: f32;
    var phi_14057_: vec3<f32>;
    var phi_14063_: f32;
    var phi_26685_: bool;
    var phi_26674_: f32;
    var phi_14144_: vec3<f32>;
    var phi_14145_: vec3<f32>;
    var phi_14146_: vec3<f32>;
    var phi_34161_: u32;
    var phi_14148_: vec3<f32>;
    var phi_14149_: vec3<f32>;
    var phi_14150_: vec3<f32>;
    var phi_14151_: bool;
    var phi_14166_: vec3<f32>;
    var phi_14167_: vec3<f32>;
    var phi_34159_: u32;
    var phi_14169_: vec3<f32>;
    var phi_14170_: vec3<f32>;
    var phi_14171_: vec3<f32>;
    var phi_34158_: u32;
    var phi_14172_: vec3<f32>;
    var phi_14173_: vec3<f32>;
    var phi_14174_: vec3<f32>;
    var phi_14175_: bool;
    var phi_34157_: u32;
    var phi_14176_: vec3<f32>;
    var phi_14177_: vec3<f32>;
    var phi_14178_: vec3<f32>;
    var phi_14179_: bool;
    var phi_14180_: bool;
    var phi_26758_: bool;
    var phi_26920_: bool;
    var phi_26935_: bool;
    var phi_26950_: bool;
    var phi_26977_: bool;
    var phi_26992_: bool;
    var phi_27007_: bool;
    var phi_27034_: bool;
    var phi_27049_: bool;
    var phi_27064_: bool;
    var phi_14359_: f32;
    var phi_14367_: f32;
    var phi_14375_: f32;
    var phi_27261_: bool;
    var phi_27250_: f32;
    var phi_14482_: vec3<f32>;
    var phi_27289_: bool;
    var phi_14483_: vec3<f32>;
    var phi_14484_: vec3<f32>;
    var phi_14485_: vec3<f32>;
    var phi_34545_: vec3<f32>;
    var phi_34468_: vec3<f32>;
    var phi_34389_: u32;
    var phi_34307_: f32;
    var phi_14487_: vec3<f32>;
    var phi_14488_: vec3<f32>;
    var phi_14489_: bool;
    var phi_27304_: bool;
    var phi_27355_: bool;
    var phi_27344_: f32;
    var phi_34516_: vec3<f32>;
    var phi_14022_: vec3<f32>;
    var phi_34868_: vec3<f32>;
    var phi_34662_: f32;
    var phi_34515_: vec3<f32>;
    var phi_34203_: u32;
    var phi_14024_: vec3<f32>;
    var phi_14025_: bool;
    var phi_34814_: vec3<f32>;
    var phi_34622_: f32;
    var phi_34543_: vec3<f32>;
    var phi_34466_: vec3<f32>;
    var phi_34387_: u32;
    var phi_34305_: f32;
    var phi_34228_: u32;
    var phi_34154_: u32;
    var phi_14497_: vec3<f32>;
    var phi_14498_: vec3<f32>;
    var phi_14499_: vec3<f32>;
    var phi_14500_: bool;
    var phi_27421_: bool;
    var phi_14569_: vec3<f32>;
    var phi_34211_: u32;
    var phi_14571_: vec3<f32>;
    var phi_14572_: bool;
    var phi_34210_: u32;
    var phi_14573_: vec3<f32>;
    var phi_14574_: bool;
    var phi_34208_: u32;
    var phi_14580_: vec3<f32>;
    var phi_14581_: bool;
    var phi_14615_: type_69;
    var phi_34213_: u32;
    var phi_14617_: vec3<f32>;
    var phi_14618_: type_69;
    var phi_14619_: bool;
    var phi_14625_: type_69;
    var phi_34217_: u32;
    var phi_14626_: type_69;
    var phi_34811_: vec3<f32>;
    var phi_34619_: f32;
    var phi_34540_: vec3<f32>;
    var phi_34463_: vec3<f32>;
    var phi_34384_: u32;
    var phi_34302_: f32;
    var phi_34225_: u32;
    var phi_34216_: u32;
    var phi_14628_: type_69;
    var phi_14629_: bool;
    var phi_36140_: bool;
    var phi_34215_: u32;
    var phi_14640_: vec3<f32>;
    var phi_14643_: type_41;
    var phi_14645_: u32;
    var phi_14647_: u32;
    var phi_17811_: u32;
    var phi_17814_: u32;
    var phi_17812_: u32;
    var phi_17815_: u32;
    var local_73: u32;
    var phi_17852_: u32;
    var phi_17855_: u32;
    var phi_17853_: u32;
    var phi_17856_: u32;
    var local_74: u32;
    var local_75: u32;
    var phi_17932_: f32;
    var local_76: u32;
    var local_77: u32;
    var phi_17981_: f32;
    var phi_18137_: f32;
    var phi_18144_: vec3<f32>;
    var phi_36560_: u32;
    var phi_18146_: f32;
    var phi_18147_: vec3<f32>;
    var phi_18148_: vec3<f32>;
    var phi_16809_: u32;
    var phi_16812_: u32;
    var phi_16810_: u32;
    var phi_16813_: u32;
    var local_78: u32;
    var local_79: u32;
    var phi_17582_: f32;
    var phi_17585_: f32;
    var phi_27694_: bool;
    var phi_27675_: f32;
    var phi_17586_: f32;
    var phi_27709_: bool;
    var phi_27732_: bool;
    var phi_27747_: bool;
    var phi_17095_: f32;
    var phi_27762_: bool;
    var phi_27777_: bool;
    var phi_27792_: bool;
    var phi_27857_: bool;
    var phi_27945_: bool;
    var phi_27926_: f32;
    var phi_17160_: type_72;
    var phi_17588_: type_72;
    var phi_17764_: bool;
    var phi_17773_: vec3<f32>;
    var phi_17775_: vec3<f32>;
    var phi_17776_: bool;
    var phi_17781_: f32;
    var phi_17782_: vec3<f32>;
    var phi_36568_: u32;
    var phi_17783_: f32;
    var phi_17784_: vec3<f32>;
    var phi_17785_: vec3<f32>;
    var phi_17786_: f32;
    var phi_36567_: u32;
    var phi_17787_: f32;
    var phi_17788_: vec3<f32>;
    var phi_17789_: vec3<f32>;
    var phi_17790_: f32;
    var phi_36559_: u32;
    var phi_18149_: f32;
    var phi_18150_: vec3<f32>;
    var phi_18151_: vec3<f32>;
    var phi_18152_: f32;
    var phi_27960_: bool;
    var phi_28070_: f32;
    var phi_28098_: f32;
    var phi_28126_: f32;
    var phi_28260_: bool;
    var phi_28275_: bool;
    var phi_28290_: bool;
    var phi_28317_: bool;
    var phi_28332_: bool;
    var phi_28347_: bool;
    var phi_28374_: bool;
    var phi_28389_: bool;
    var phi_28404_: bool;
    var phi_14981_: f32;
    var phi_14989_: f32;
    var phi_14997_: f32;
    var phi_28601_: bool;
    var phi_28590_: f32;
    var phi_15104_: vec3<f32>;
    var phi_28629_: bool;
    var phi_15105_: vec3<f32>;
    var phi_15117_: type_59;
    var phi_15118_: type_59;
    var phi_15119_: bool;
    var phi_15124_: type_59;
    var phi_28666_: bool;
    var phi_28696_: bool;
    var phi_28685_: f32;
    var phi_28735_: vec3<f32>;
    var phi_15152_: type_59;
    var phi_15153_: type_59;
    var phi_28757_: vec3<f32>;
    var phi_15154_: type_59;
    var phi_15155_: type_59;
    var phi_15156_: bool;
    var phi_15160_: type_59;
    var phi_28789_: bool;
    var phi_28860_: bool;
    var phi_15980_: f32;
    var phi_28875_: bool;
    var phi_28890_: bool;
    var phi_16453_: vec3<f32>;
    var phi_16454_: vec3<f32>;
    var phi_16455_: bool;
    var phi_16460_: vec3<f32>;
    var phi_16461_: bool;
    var phi_29014_: bool;
    var phi_29029_: bool;
    var phi_16752_: vec3<f32>;
    var phi_16754_: vec3<f32>;
    var phi_16781_: vec3<f32>;
    var phi_16782_: vec3<f32>;
    var phi_15598_: vec3<f32>;
    var phi_15599_: vec3<f32>;
    var phi_15600_: bool;
    var phi_15605_: vec3<f32>;
    var phi_15606_: bool;
    var phi_29191_: bool;
    var phi_29206_: bool;
    var phi_15897_: vec3<f32>;
    var phi_15899_: vec3<f32>;
    var phi_36602_: u32;
    var phi_16783_: f32;
    var phi_16784_: vec3<f32>;
    var phi_16785_: vec3<f32>;
    var phi_36601_: u32;
    var phi_16786_: f32;
    var phi_16787_: vec3<f32>;
    var phi_16788_: vec3<f32>;
    var phi_36558_: u32;
    var phi_18153_: f32;
    var phi_18154_: vec3<f32>;
    var phi_18155_: vec3<f32>;
    var phi_18156_: f32;
    var phi_14743_: f32;
    var phi_14744_: f32;
    var phi_36557_: u32;
    var phi_18157_: f32;
    var phi_18158_: vec3<f32>;
    var phi_18159_: vec3<f32>;
    var phi_18160_: f32;
    var phi_29259_: bool;
    var phi_29369_: f32;
    var phi_29397_: f32;
    var phi_29425_: f32;
    var phi_29559_: bool;
    var phi_29574_: bool;
    var phi_29589_: bool;
    var phi_29616_: bool;
    var phi_29631_: bool;
    var phi_29646_: bool;
    var phi_29673_: bool;
    var phi_29688_: bool;
    var phi_29703_: bool;
    var phi_18383_: f32;
    var phi_18391_: f32;
    var phi_18399_: f32;
    var phi_29900_: bool;
    var phi_29889_: f32;
    var phi_18506_: vec3<f32>;
    var phi_29928_: bool;
    var phi_18507_: vec3<f32>;
    var phi_18519_: type_59;
    var phi_18520_: type_59;
    var phi_18521_: bool;
    var phi_18526_: type_59;
    var phi_29965_: bool;
    var phi_29995_: bool;
    var phi_29984_: f32;
    var phi_30034_: vec3<f32>;
    var phi_18554_: type_59;
    var phi_18555_: type_59;
    var phi_30056_: vec3<f32>;
    var phi_18556_: type_59;
    var phi_18557_: type_59;
    var phi_18558_: bool;
    var phi_18562_: type_59;
    var phi_30070_: f32;
    var phi_30073_: f32;
    var phi_18571_: f32;
    var phi_18604_: u32;
    var phi_18607_: u32;
    var phi_18609_: u32;
    var phi_18611_: u32;
    var phi_18613_: u32;
    var phi_18615_: vec4<f32>;
    var phi_18617_: f32;
    var phi_19004_: u32;
    var phi_19006_: u32;
    var phi_19007_: u32;
    var phi_19009_: bool;
    var phi_19010_: bool;
    var phi_19011_: u32;
    var phi_19012_: u32;
    var phi_19013_: u32;
    var phi_19014_: bool;
    var phi_19015_: bool;
    var phi_19016_: bool;
    var phi_30171_: bool;
    var phi_30201_: bool;
    var phi_30148_: type_56;
    var phi_30151_: type_56;
    var phi_30152_: bool;
    var phi_30157_: type_56;
    var phi_30258_: bool;
    var phi_30273_: bool;
    var phi_30288_: bool;
    var phi_30303_: bool;
    var phi_30235_: type_56;
    var phi_30238_: type_56;
    var phi_30239_: bool;
    var phi_30244_: type_56;
    var phi_30345_: bool;
    var phi_30360_: bool;
    var phi_30375_: bool;
    var phi_30390_: bool;
    var phi_30322_: type_56;
    var phi_30325_: type_56;
    var phi_30326_: bool;
    var phi_30331_: type_56;
    var phi_30126_: bool;
    var phi_30129_: bool;
    var phi_19045_: f32;
    var phi_30492_: bool;
    var phi_30522_: bool;
    var phi_30469_: type_56;
    var phi_30472_: type_56;
    var phi_30473_: bool;
    var phi_30478_: type_56;
    var phi_30579_: bool;
    var phi_30594_: bool;
    var phi_30609_: bool;
    var phi_30624_: bool;
    var phi_30556_: type_56;
    var phi_30559_: type_56;
    var phi_30560_: bool;
    var phi_30565_: type_56;
    var phi_30666_: bool;
    var phi_30681_: bool;
    var phi_30696_: bool;
    var phi_30711_: bool;
    var phi_30643_: type_56;
    var phi_30646_: type_56;
    var phi_30647_: bool;
    var phi_30652_: type_56;
    var phi_30447_: bool;
    var phi_30450_: bool;
    var phi_19054_: f32;
    var phi_19065_: u32;
    var phi_19076_: u32;
    var phi_19115_: u32;
    var phi_19116_: u32;
    var phi_19127_: u32;
    var phi_19128_: u32;
    var phi_19129_: u32;
    var phi_19130_: u32;
    var phi_19132_: u32;
    var phi_19133_: u32;
    var phi_19135_: u32;
    var phi_19136_: u32;
    var phi_19137_: bool;
    var phi_19138_: bool;
    var phi_18930_: vec4<f32>;
    var phi_18931_: vec4<f32>;
    var phi_18932_: bool;
    var phi_18936_: vec4<f32>;
    var phi_18937_: bool;
    var phi_18842_: bool;
    var phi_18879_: bool;
    var phi_18885_: f32;
    var phi_18895_: vec4<f32>;
    var phi_18896_: vec4<f32>;
    var phi_18897_: bool;
    var phi_18901_: vec4<f32>;
    var phi_18902_: bool;
    var phi_18906_: vec4<f32>;
    var phi_18907_: bool;
    var phi_18938_: vec4<f32>;
    var phi_18939_: bool;
    var phi_18772_: vec4<f32>;
    var phi_18773_: vec4<f32>;
    var phi_18774_: bool;
    var phi_18778_: vec4<f32>;
    var phi_18779_: bool;
    var phi_18780_: vec4<f32>;
    var phi_18781_: bool;
    var phi_18782_: bool;
    var phi_18790_: vec4<f32>;
    var phi_18791_: bool;
    var phi_18792_: vec4<f32>;
    var phi_18793_: bool;
    var phi_18794_: bool;
    var phi_18802_: vec4<f32>;
    var phi_18803_: bool;
    var phi_18948_: vec4<f32>;
    var phi_18949_: bool;
    var phi_18970_: f32;
    var phi_18976_: u32;
    var phi_18605_: u32;
    var phi_18608_: u32;
    var phi_18610_: u32;
    var phi_18612_: u32;
    var phi_18614_: u32;
    var phi_18616_: vec4<f32>;
    var phi_18618_: f32;
    var phi_19149_: vec4<f32>;
    var phi_19150_: bool;
    var phi_19151_: bool;
    var phi_36068_: bool;
    var phi_21677_: bool;
    var phi_21676_: vec4<f32>;
    var local_80: u32;
    var local_81: vec4<f32>;
    var phi_19156_: type_74;
    var local_82: u32;
    var phi_19162_: type_74;
    var phi_19214_: vec3<f32>;
    var phi_19215_: type_41;
    var phi_19216_: vec3<f32>;
    var phi_19217_: type_41;
    var phi_36510_: bool;
    var phi_19218_: vec3<f32>;
    var phi_19219_: type_41;
    var phi_19220_: u32;
    var phi_36509_: bool;
    var phi_19221_: vec3<f32>;
    var phi_19222_: type_41;
    var phi_19223_: u32;
    var phi_36795_: u32;
    var phi_36508_: bool;
    var phi_14641_: vec3<f32>;
    var phi_14644_: type_41;
    var phi_14646_: u32;
    var phi_14648_: u32;
    var phi_36515_: u32;
    var phi_36507_: bool;
    var phi_30862_: bool;
    var phi_30892_: bool;
    var phi_30881_: f32;
    var phi_30812_: bool;
    var phi_30948_: vec3<f32>;
    var phi_31104_: bool;
    var phi_31119_: bool;
    var phi_31134_: bool;
    var phi_31043_: vec3<f32>;
    var phi_31046_: vec3<f32>;
    var phi_31047_: bool;
    var phi_30817_: vec3<f32>;
    var phi_30821_: vec3<f32>;
    var phi_30822_: bool;
    var phi_31234_: bool;
    var phi_31208_: vec3<f32>;
    var phi_30829_: vec3<f32>;
    var phi_31249_: bool;
    var phi_31359_: f32;
    var phi_31387_: f32;
    var phi_31415_: f32;
    var phi_31549_: bool;
    var phi_31564_: bool;
    var phi_31579_: bool;
    var phi_31606_: bool;
    var phi_31621_: bool;
    var phi_31636_: bool;
    var phi_31663_: bool;
    var phi_31678_: bool;
    var phi_31693_: bool;
    var phi_19442_: f32;
    var phi_19450_: f32;
    var phi_19458_: f32;
    var phi_31890_: bool;
    var phi_31879_: f32;
    var phi_19565_: vec3<f32>;
    var phi_31918_: bool;
    var phi_19566_: vec3<f32>;
    var phi_19578_: type_59;
    var phi_19579_: type_59;
    var phi_19580_: bool;
    var phi_19585_: type_59;
    var phi_31955_: bool;
    var phi_31985_: bool;
    var phi_31974_: f32;
    var phi_32024_: vec3<f32>;
    var phi_19613_: type_59;
    var phi_19614_: type_59;
    var phi_32046_: vec3<f32>;
    var phi_19615_: type_59;
    var phi_19616_: type_59;
    var phi_19617_: bool;
    var phi_19621_: type_59;
    var phi_32071_: bool;
    var phi_19697_: vec3<f32>;
    var phi_38349_: u32;
    var phi_19699_: vec3<f32>;
    var phi_19700_: bool;
    var phi_38348_: u32;
    var phi_19701_: vec3<f32>;
    var phi_19702_: bool;
    var phi_38346_: u32;
    var phi_19708_: vec3<f32>;
    var phi_19709_: bool;
    var local_83: u32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: type_41;
    var phi_19738_: vec3<f32>;
    var local_88: u32;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: type_41;
    var phi_19754_: type_69;
    var phi_19755_: type_69;
    var phi_38353_: u32;
    var phi_19757_: type_69;
    var phi_19758_: type_69;
    var phi_19759_: bool;
    var phi_38352_: u32;
    var phi_19760_: type_69;
    var phi_19761_: type_69;
    var phi_19762_: bool;
    var phi_19763_: bool;
    var local_93: u32;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: vec3<f32>;
    var local_97: type_41;
    var phi_19769_: type_69;
    var local_98: u32;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: type_41;
    var phi_19774_: type_69;
    var phi_19775_: type_69;
    var phi_19776_: bool;
    var phi_19780_: type_69;
    var phi_19781_: type_69;
    var phi_19782_: bool;
    var phi_19786_: type_69;
    var phi_19787_: type_69;
    var phi_19788_: bool;
    var phi_13407_: bool;
    var phi_13416_: vec3<f32>;
    var phi_13418_: vec3<f32>;
    var phi_13419_: bool;
    var phi_13837_: f32;
    var phi_13840_: f32;
    var phi_32321_: bool;
    var phi_32302_: f32;
    var phi_13841_: f32;
    var phi_32339_: f32;
    var phi_32342_: f32;
    var phi_13847_: f32;
    var phi_13848_: f32;
    var phi_13849_: f32;
    var phi_13850_: f32;
    var phi_19789_: type_69;
    var phi_19790_: type_69;
    var phi_19791_: bool;
    var phi_11463_: f32;
    var phi_32389_: f32;
    var phi_32392_: f32;
    var phi_11465_: f32;
    var phi_11466_: f32;
    var phi_11467_: f32;
    var phi_32418_: bool;
    var phi_11521_: f32;
    var phi_32436_: f32;
    var phi_32439_: f32;
    var phi_11523_: f32;
    var phi_11524_: f32;
    var phi_32450_: bool;
    var phi_32465_: bool;
    var phi_11993_: vec3<f32>;
    var phi_11994_: vec3<f32>;
    var phi_11995_: bool;
    var phi_12000_: vec3<f32>;
    var phi_12001_: bool;
    var phi_32589_: bool;
    var phi_32604_: bool;
    var phi_12292_: vec3<f32>;
    var phi_12294_: vec3<f32>;
    var phi_12321_: vec3<f32>;
    var phi_12335_: vec3<f32>;
    var phi_19795_: type_69;
    var phi_19796_: type_69;
    var phi_19797_: bool;
    var phi_19801_: type_69;

    switch bitcast<i32>(0u) {
        default: {
            let _e279 = global;
            let _e289 = arrayLength((&global_9.member));
            let _e293 = global_1.member.member_7[2u];
            let _e297 = (_e279.x + (_e279.y * 4194240u));
            let _e302 = atomicLoad((&global_8.member[((_e293 * 4u) + 3u)]));
            if (_e297 >= _e302) {
                phi_11199_ = type_48(0u, type_48().member_1);
            } else {
                let _e307 = global_1.member.member[0u];
                let _e311 = global_1.member.member[1u];
                let _e316 = global_1.member.member_7[3u];
                let _e323 = atomicLoad((&global_8.member[((12u + (_e293 * ((_e307 * _e311) * _e316))) + _e297)]));
                phi_11199_ = type_48(1u, _e323);
            }
            let _e329 = phi_11199_;
            if (bitcast<i32>(_e329.member) != 0i) {
                let _e334 = (_e329.member_1 < arrayLength((&global_7.member)));
                if _e334 {
                } else {
                    break;
                }
                let _e337 = global_7.member[_e329.member_1];
                let _e351 = vec3<f32>(_e337.member_1.x, _e337.member_1.y, _e337.member_1.z);
                let _e356 = vec3<f32>(_e337.member_2.x, _e337.member_2.y, _e337.member_2.z);
                let _e362 = select(select(u32(_e337.member_2.w), 0u, (_e337.member_2.w < 0f)), 4294967295u, (_e337.member_2.w > 4294967000f));
                let _e366 = vec3<f32>(_e337.member_3.x, _e337.member_3.y, _e337.member_3.z);
                let _e376 = ((_e337.member_4.y & 2u) != 0u);
                let _e378 = ((_e337.member_4.y & 32u) != 0u);
                let _e381 = global_1.member.member[3u];
                let _e384 = global_1.member.member_3[3u];
                let _e387 = global_1.member.member_1[2u];
                let _e390 = (select(select(u32(_e337.member_3.w), 0u, (_e337.member_3.w < 0f)), 4294967295u, (_e337.member_3.w > 4294967000f)) + 1u);
                if (_e337.member_4.z == 4294967295u) {
                    if _e376 {
                        phi_11467_ = 1f;
                    } else {
                        if _e378 {
                            phi_11466_ = 1f;
                        } else {
                            if ((_e384 & 2u) == 0u) {
                                phi_11465_ = 1f;
                            } else {
                                let _e6601 = global_1.member.member_3[1u];
                                let _e6604 = global_1.member.member_3[2u];
                                let _e6607 = ((atan2(_e337.member_1.z, _e337.member_1.x) * 0.15915494f) + 0.5f);
                                let _e6611 = select(_e337.member_1.y, -1f, (_e337.member_1.y < -1f));
                                let _e6614 = acos(select(_e6611, 1f, (_e6611 > 1f)));
                                let _e6616 = f32(_e6601);
                                let _e6617 = ((_e6607 - floor(_e6607)) * _e6616);
                                let _e6622 = select(select(u32(_e6617), 0u, (_e6617 < 0f)), 4294967295u, (_e6617 > 4294967000f));
                                let _e6623 = (_e6601 - 1u);
                                let _e6626 = f32(_e6604);
                                let _e6627 = ((_e6614 * 0.31830987f) * _e6626);
                                let _e6632 = select(select(u32(_e6627), 0u, (_e6627 < 0f)), 4294967295u, (_e6627 > 4294967000f));
                                let _e6633 = (_e6604 - 1u);
                                let _e6635 = select(_e6632, _e6633, (_e6633 < _e6632));
                                let _e6636 = sin(_e6614);
                                if (_e6636 <= 0f) {
                                    phi_11463_ = 0f;
                                } else {
                                    let _e6640 = global_1.member.member_2[3u];
                                    let _e6643 = global_1.member.member_3[0u];
                                    let _e6647 = (_e6640 + _e6635);
                                    let _e6650 = global_6.member[(_e6647 + 1u)];
                                    let _e6652 = global_6.member[_e6647];
                                    let _e6655 = ((_e6643 + (_e6635 * (_e6601 + 1u))) + select(_e6622, _e6623, (_e6623 < _e6622)));
                                    let _e6658 = global_6.member[(_e6655 + 1u)];
                                    let _e6660 = global_6.member[_e6655];
                                    phi_11463_ = ((((_e6650 - _e6652) * _e6626) * ((_e6658 - _e6660) * _e6616)) / (19.73921f * _e6636));
                                }
                                let _e6667 = phi_11463_;
                                if (_e337.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32392_ = 1f;
                                } else {
                                    let _e6669 = (_e337.member_1.w * _e337.member_1.w);
                                    let _e6671 = (_e6669 + (_e6667 * _e6667));
                                    if (_e6671 == 0f) {
                                        phi_32389_ = 0f;
                                    } else {
                                        phi_32389_ = (_e6669 / _e6671);
                                    }
                                    let _e6675 = phi_32389_;
                                    phi_32392_ = _e6675;
                                }
                                let _e6677 = phi_32392_;
                                phi_11465_ = _e6677;
                            }
                            let _e6679 = phi_11465_;
                            phi_11466_ = _e6679;
                        }
                        let _e6681 = phi_11466_;
                        phi_11467_ = _e6681;
                    }
                    let _e6683 = phi_11467_;
                    if ((_e384 & 4u) == 0u) {
                        phi_12335_ = _e366;
                    } else {
                        if _e376 {
                            phi_11524_ = 1f;
                        } else {
                            if _e378 {
                                phi_11523_ = 0f;
                            } else {
                                let _e6694 = global_1.member.member_5;
                                let _e6698 = (_e337.member_1.x - _e6694.x);
                                let _e6699 = (_e337.member_1.y - _e6694.y);
                                let _e6700 = (_e337.member_1.z - _e6694.z);
                                let _e6707 = (sqrt((((_e6698 * _e6698) + (_e6699 * _e6699)) + (_e6700 * _e6700))) * 0.5f);
                                if (_e6707 != _e6707) {
                                    phi_32418_ = true;
                                } else {
                                    phi_32418_ = (1f <= _e6707);
                                }
                                let _e6711 = phi_32418_;
                                let _e6717 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e6707, 1f, _e6711))) >= _e6717) {
                                    phi_11521_ = 0f;
                                } else {
                                    let _e6720 = sin((_e6717 * 0.5f));
                                    phi_11521_ = (1f / ((12.566371f * _e6720) * _e6720));
                                }
                                let _e6725 = phi_11521_;
                                if (_e337.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32439_ = 1f;
                                } else {
                                    let _e6727 = (_e337.member_1.w * _e337.member_1.w);
                                    let _e6729 = (_e6727 + (_e6725 * _e6725));
                                    if (_e6729 == 0f) {
                                        phi_32436_ = 0f;
                                    } else {
                                        phi_32436_ = (_e6727 / _e6729);
                                    }
                                    let _e6733 = phi_32436_;
                                    phi_32439_ = _e6733;
                                }
                                let _e6735 = phi_32439_;
                                phi_11523_ = _e6735;
                            }
                            let _e6737 = phi_11523_;
                            phi_11524_ = _e6737;
                        }
                        let _e6739 = phi_11524_;
                        let _e6741 = global_1.member.member_5;
                        let _e6745 = (_e337.member_1.x - _e6741.x);
                        let _e6746 = (_e337.member_1.y - _e6741.y);
                        let _e6747 = (_e337.member_1.z - _e6741.z);
                        let _e6754 = (sqrt((((_e6745 * _e6745) + (_e6746 * _e6746)) + (_e6747 * _e6747))) * 0.5f);
                        if (_e6754 != _e6754) {
                            phi_32450_ = true;
                        } else {
                            phi_32450_ = (1f <= _e6754);
                        }
                        let _e6758 = phi_32450_;
                        let _e6761 = (2f * asin(select(_e6754, 1f, _e6758)));
                        let _e6765 = global_1.member.member_6[3u];
                        if (_e6761 >= _e6765) {
                            phi_12321_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e6769 = (1f - pow((_e6761 / _e6765), 2f));
                            if (_e6769 != _e6769) {
                                phi_32465_ = true;
                            } else {
                                phi_32465_ = (0f >= _e6769);
                            }
                            let _e6773 = phi_32465_;
                            let _e6776 = global_1.member.member_6;
                            let _e6782 = global_1.member.member_8[2u];
                            if (_e6782 == 4294967295u) {
                                phi_12294_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e6785 = global_6.member[_e6782];
                                let _e6788 = global_6.member[(_e6782 + 1u)];
                                let _e6791 = global_6.member[(_e6782 + 2u)];
                                let _e6794 = global_6.member[(_e6782 + 3u)];
                                let _e6797 = global_6.member[(_e6782 + 4u)];
                                let _e6800 = global_6.member[(_e6782 + 5u)];
                                let _e6803 = global_6.member[(_e6782 + 6u)];
                                let _e6806 = global_6.member[(_e6782 + 7u)];
                                let _e6809 = global_6.member[(_e6782 + 8u)];
                                let _e6812 = global_6.member[(_e6782 + 9u)];
                                let _e6815 = global_6.member[(_e6782 + 13u)];
                                let _e6818 = global_6.member[(_e6782 + 14u)];
                                let _e6821 = global_6.member[(_e6782 + 15u)];
                                let _e6824 = global_6.member[(_e6782 + 16u)];
                                let _e6827 = global_6.member[(_e6782 + 18u)];
                                let _e6830 = global_6.member[(_e6782 + 19u)];
                                let _e6833 = global_6.member[(_e6782 + 20u)];
                                let _e6836 = global_6.member[(_e6782 + 29u)];
                                let _e6837 = bitcast<u32>(_e6836);
                                let _e6850 = (((((_e337.member.x * _e337.member.x) + (_e337.member.y * _e337.member.y)) + (_e337.member.z * _e337.member.z)) - (2f * (((_e337.member.x * _e6785) + (_e337.member.y * _e6788)) + (_e337.member.z * _e6791)))) + _e6803);
                                if (_e6850 > 0f) {
                                    let _e6859 = ((((_e337.member.x - _e6785) * _e6806) + ((_e337.member.y - _e6788) * _e6809)) + ((_e337.member.z - _e6791) * _e6812));
                                    let _e6861 = ((_e6859 * _e6859) - _e6850);
                                    if (_e6859 >= 0f) {
                                        phi_11994_ = vec3<f32>();
                                        phi_11995_ = true;
                                    } else {
                                        let _e6863 = (_e6861 <= 0f);
                                        if _e6863 {
                                            phi_11993_ = vec3<f32>();
                                        } else {
                                            let _e6866 = (-(_e6859) - sqrt(_e6861));
                                            phi_11993_ = vec3<f32>((_e337.member.x + (_e6806 * _e6866)), (_e337.member.y + (_e6809 * _e6866)), (_e337.member.z + (_e6812 * _e6866)));
                                        }
                                        let _e6875 = phi_11993_;
                                        phi_11994_ = _e6875;
                                        phi_11995_ = _e6863;
                                    }
                                    let _e6877 = phi_11994_;
                                    let _e6879 = phi_11995_;
                                    phi_12000_ = _e6877;
                                    phi_12001_ = select(true, false, _e6879);
                                } else {
                                    phi_12000_ = vec3<f32>(_e337.member.x, _e337.member.y, _e337.member.z);
                                    phi_12001_ = true;
                                }
                                let _e6882 = phi_12000_;
                                let _e6884 = phi_12001_;
                                if _e6884 {
                                    let _e6886 = (_e6882.x - _e6785);
                                    let _e6888 = (_e6882.y - _e6788);
                                    let _e6890 = (_e6882.z - _e6791);
                                    let _e6896 = sqrt((((_e6886 * _e6886) + (_e6888 * _e6888)) + (_e6890 * _e6890)));
                                    let _e6897 = (1f / _e6896);
                                    let _e6915 = ((((((_e6882.x * _e6882.x) + (_e6882.y * _e6882.y)) + (_e6882.z * _e6882.z)) - (2f * (((_e6882.x * _e6785) + (_e6882.y * _e6788)) + (_e6882.z * _e6791)))) + _e6800) / (_e6896 + _e6794));
                                    if (_e6915 != _e6915) {
                                        phi_32589_ = true;
                                    } else {
                                        phi_32589_ = (0f >= _e6915);
                                    }
                                    let _e6919 = phi_32589_;
                                    let _e6923 = sqrt((select(_e6915, 0f, _e6919) / (_e6797 - _e6794)));
                                    if (_e6923 != _e6923) {
                                        phi_32604_ = true;
                                    } else {
                                        phi_32604_ = (1f <= _e6923);
                                    }
                                    let _e6927 = phi_32604_;
                                    let _e6929 = (select(_e6923, 1f, _e6927) * 63f);
                                    let _e6934 = ((((_e6886 * _e6897) * _e6806) + ((_e6888 * _e6897) * _e6809)) + ((_e6890 * _e6897) * _e6812));
                                    let _e6936 = select(_e6934, -1f, (_e6934 < -1f));
                                    let _e6940 = ((select(_e6936, 1f, (_e6936 > 1f)) + 1f) * 127.5f);
                                    let _e6945 = select(select(u32(_e6929), 0u, (_e6929 < 0f)), 4294967295u, (_e6929 > 4294967000f));
                                    let _e6947 = select(_e6945, 62u, (62u < _e6945));
                                    let _e6952 = select(select(u32(_e6940), 0u, (_e6940 < 0f)), 4294967295u, (_e6940 > 4294967000f));
                                    let _e6954 = select(_e6952, 254u, (254u < _e6952));
                                    let _e6956 = (_e6929 - f32(_e6947));
                                    let _e6958 = (_e6940 - f32(_e6954));
                                    let _e6962 = (_e6837 + (3u * ((_e6947 * 256u) + _e6954)));
                                    let _e6967 = (_e6837 + (3u * (((_e6947 + 1u) * 256u) + _e6954)));
                                    let _e6969 = global_6.member[_e6962];
                                    let _e6972 = global_6.member[(_e6962 + 1u)];
                                    let _e6975 = global_6.member[(_e6962 + 2u)];
                                    let _e6976 = (1f - _e6958);
                                    let _e6982 = global_6.member[(_e6962 + 3u)];
                                    let _e6985 = global_6.member[(_e6962 + 4u)];
                                    let _e6988 = global_6.member[(_e6962 + 5u)];
                                    let _e6995 = (1f - _e6956);
                                    let _e7000 = global_6.member[_e6967];
                                    let _e7003 = global_6.member[(_e6967 + 1u)];
                                    let _e7006 = global_6.member[(_e6967 + 2u)];
                                    let _e7012 = global_6.member[(_e6967 + 3u)];
                                    let _e7015 = global_6.member[(_e6967 + 4u)];
                                    let _e7018 = global_6.member[(_e6967 + 5u)];
                                    phi_12292_ = vec3<f32>(((((_e6969 * _e6976) + (_e6982 * _e6958)) * _e6995) + (((_e7000 * _e6976) + (_e7012 * _e6958)) * _e6956)), ((((_e6972 * _e6976) + (_e6985 * _e6958)) * _e6995) + (((_e7003 * _e6976) + (_e7015 * _e6958)) * _e6956)), ((((_e6975 * _e6976) + (_e6988 * _e6958)) * _e6995) + (((_e7006 * _e6976) + (_e7018 * _e6958)) * _e6956)));
                                } else {
                                    phi_12292_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e7033 = phi_12292_;
                                let _e7040 = ((_e6824 * 1.1f) * _e7033.y);
                                phi_12294_ = vec3<f32>(exp(-((((_e6815 * _e7033.x) + _e7040) + (_e6827 * _e7033.z)))), exp(-((((_e6818 * _e7033.x) + _e7040) + (_e6830 * _e7033.z)))), exp(-((((_e6821 * _e7033.x) + _e7040) + (_e6833 * _e7033.z)))));
                            }
                            let _e7059 = phi_12294_;
                            let _e7071 = sin((_e6765 * 0.5f));
                            let _e7074 = (((1f - (0.6f * (1f - sqrt(select(_e6769, 0f, _e6773))))) * 1.25f) / ((12.566371f * _e7071) * _e7071));
                            phi_12321_ = vec3<f32>(((_e6776.x * _e7059.x) * _e7074), ((_e6776.y * _e7059.y) * _e7074), ((_e6776.z * _e7059.z) * _e7074));
                        }
                        let _e7080 = phi_12321_;
                        phi_12335_ = vec3<f32>((_e337.member_3.x + ((_e337.member_2.x * _e7080.x) * _e6739)), (_e337.member_3.y + ((_e337.member_2.y * _e7080.y) * _e6739)), (_e337.member_3.z + ((_e337.member_2.z * _e7080.z) * _e6739)));
                    }
                    let _e7095 = phi_12335_;
                    phi_19795_ = type_69(type_34(_e337.member, _e337.member_1, vec4<f32>(_e337.member_2.x, _e337.member_2.y, _e337.member_2.z, f32(_e362)), vec4<f32>(_e7095.x, _e7095.y, _e7095.z, f32(_e390)), vec4<u32>(_e337.member_4.x, (_e337.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), type_41(vec4<f32>(_e337.member.x, _e337.member.y, _e337.member.z, -1f), vec4<f32>(_e337.member_1.x, _e337.member_1.y, _e337.member_1.z, 0f), vec4<f32>((_e337.member_2.x * _e6683), (_e337.member_2.y * _e6683), (_e337.member_2.z * _e6683), 0f)), type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_19796_ = type_69();
                    phi_19797_ = true;
                } else {
                    if ((_e384 & 8u) == 0u) {
                        phi_36163_ = false;
                        phi_12869_ = _e356;
                        phi_12870_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    } else {
                        if (_e337.member_5.x > 100f) {
                            let _e401 = global_1.member.member_8[1u];
                            let _e403 = global_6.member[_e401];
                            let _e406 = global_6.member[(_e401 + 1u)];
                            let _e409 = global_6.member[(_e401 + 2u)];
                            let _e412 = global_6.member[(_e401 + 3u)];
                            let _e415 = global_6.member[(_e401 + 5u)];
                            let _e418 = global_6.member[(_e401 + 6u)];
                            let _e421 = global_6.member[(_e401 + 10u)];
                            let _e424 = global_6.member[(_e401 + 11u)];
                            let _e427 = global_6.member[(_e401 + 13u)];
                            let _e430 = global_6.member[(_e401 + 14u)];
                            let _e433 = global_6.member[(_e401 + 15u)];
                            let _e436 = global_6.member[(_e401 + 16u)];
                            let _e439 = global_6.member[(_e401 + 18u)];
                            let _e442 = global_6.member[(_e401 + 19u)];
                            let _e445 = global_6.member[(_e401 + 20u)];
                            let _e453 = ((((_e337.member.x - _e403) * _e337.member_1.x) + ((_e337.member.y - _e406) * _e337.member_1.y)) + ((_e337.member.z - _e409) * _e337.member_1.z));
                            let _e465 = ((((_e337.member.x * _e337.member.x) + (_e337.member.y * _e337.member.y)) + (_e337.member.z * _e337.member.z)) - (2f * (((_e337.member.x * _e403) + (_e337.member.y * _e406)) + (_e337.member.z * _e409))));
                            let _e466 = (_e465 + _e418);
                            let _e467 = (_e453 * _e453);
                            let _e468 = (_e467 - _e466);
                            if (_e468 < 0f) {
                                phi_26135_ = type_57(0u, type_57().member_1);
                                phi_26136_ = type_57();
                                phi_26137_ = true;
                            } else {
                                if (_e453 < 0f) {
                                    phi_26107_ = -(sqrt(_e468));
                                } else {
                                    phi_26107_ = sqrt(_e468);
                                }
                                let _e475 = phi_26107_;
                                let _e477 = -((_e453 + _e475));
                                let _e478 = (_e477 == 0f);
                                if _e478 {
                                    phi_26130_ = type_57(1u, type_56(0f, 0f));
                                    phi_26131_ = type_57();
                                } else {
                                    let _e479 = (_e466 / _e477);
                                    if (_e477 < _e479) {
                                        phi_26119_ = type_56(_e477, _e479);
                                    } else {
                                        phi_26119_ = type_56(_e479, _e477);
                                    }
                                    let _e484 = phi_26119_;
                                    phi_26130_ = type_57();
                                    phi_26131_ = type_57(1u, _e484);
                                }
                                let _e487 = phi_26130_;
                                let _e489 = phi_26131_;
                                phi_26135_ = _e487;
                                phi_26136_ = _e489;
                                phi_26137_ = _e478;
                            }
                            let _e494 = phi_26135_;
                            let _e496 = phi_26136_;
                            let _e498 = phi_26137_;
                            if _e498 {
                                phi_26141_ = _e494;
                            } else {
                                phi_26141_ = _e496;
                            }
                            let _e500 = phi_26141_;
                            if (bitcast<i32>(_e500.member) != 0i) {
                                phi_26028_ = type_57(0u, _e500.member_1);
                            } else {
                                phi_26028_ = type_57(1u, type_57().member_1);
                            }
                            let _e510 = phi_26028_;
                            if (bitcast<i32>(_e510.member) != 0i) {
                                phi_26075_ = type_57(0u, type_57().member_1);
                            } else {
                                if (_e510.member_1.member_1 <= 0f) {
                                    phi_26071_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e510.member_1.member != _e510.member_1.member) {
                                        phi_26179_ = true;
                                    } else {
                                        phi_26179_ = (0f >= _e510.member_1.member);
                                    }
                                    let _e522 = phi_26179_;
                                    let _e523 = select(_e510.member_1.member, 0f, _e522);
                                    let _e524 = (_e465 + _e415);
                                    if (_e524 > 0f) {
                                        let _e526 = (_e467 - _e524);
                                        if (_e526 < 0f) {
                                            phi_26287_ = type_57(0u, type_57().member_1);
                                            phi_26288_ = type_57();
                                            phi_26289_ = true;
                                        } else {
                                            if (_e453 < 0f) {
                                                phi_26259_ = -(sqrt(_e526));
                                            } else {
                                                phi_26259_ = sqrt(_e526);
                                            }
                                            let _e533 = phi_26259_;
                                            let _e535 = -((_e453 + _e533));
                                            let _e536 = (_e535 == 0f);
                                            if _e536 {
                                                phi_26282_ = type_57(1u, type_56(0f, 0f));
                                                phi_26283_ = type_57();
                                            } else {
                                                let _e537 = (_e524 / _e535);
                                                if (_e535 < _e537) {
                                                    phi_26271_ = type_56(_e535, _e537);
                                                } else {
                                                    phi_26271_ = type_56(_e537, _e535);
                                                }
                                                let _e542 = phi_26271_;
                                                phi_26282_ = type_57();
                                                phi_26283_ = type_57(1u, _e542);
                                            }
                                            let _e545 = phi_26282_;
                                            let _e547 = phi_26283_;
                                            phi_26287_ = _e545;
                                            phi_26288_ = _e547;
                                            phi_26289_ = _e536;
                                        }
                                        let _e552 = phi_26287_;
                                        let _e554 = phi_26288_;
                                        let _e556 = phi_26289_;
                                        if _e556 {
                                            phi_26293_ = _e552;
                                        } else {
                                            phi_26293_ = _e554;
                                        }
                                        let _e558 = phi_26293_;
                                        if (bitcast<i32>(_e558.member) != 0i) {
                                            phi_26227_ = select(_e558.member_1.member_1, _e558.member_1.member, (_e558.member_1.member >= 0f));
                                        } else {
                                            phi_26227_ = -1f;
                                        }
                                        let _e569 = phi_26227_;
                                        if (_e569 >= 0f) {
                                            if (_e510.member_1.member_1 != _e510.member_1.member_1) {
                                                phi_26331_ = true;
                                            } else {
                                                phi_26331_ = (_e569 <= _e510.member_1.member_1);
                                            }
                                            let _e574 = phi_26331_;
                                            phi_26056_ = select(_e510.member_1.member_1, _e569, _e574);
                                        } else {
                                            phi_26056_ = _e510.member_1.member_1;
                                        }
                                        let _e577 = phi_26056_;
                                        phi_26058_ = _e577;
                                    } else {
                                        phi_26058_ = _e510.member_1.member_1;
                                    }
                                    let _e579 = phi_26058_;
                                    if (_e579 <= _e523) {
                                        phi_26067_ = type_57(0u, type_57().member_1);
                                    } else {
                                        phi_26067_ = type_57(1u, type_56(_e523, _e579));
                                    }
                                    let _e587 = phi_26067_;
                                    phi_26071_ = _e587;
                                }
                                let _e592 = phi_26071_;
                                phi_26075_ = _e592;
                            }
                            let _e597 = phi_26075_;
                            if (bitcast<i32>(_e597.member) != 0i) {
                                let _e603 = (_e597.member_1.member < _e337.member_5.x);
                                if _e603 {
                                    if (_e597.member_1.member_1 != _e597.member_1.member_1) {
                                        phi_26346_ = true;
                                    } else {
                                        phi_26346_ = (_e337.member_5.x <= _e597.member_1.member_1);
                                    }
                                    let _e609 = phi_26346_;
                                    let _e610 = select(_e597.member_1.member_1, _e337.member_5.x, _e609);
                                    phi_12728_ = vec3<f32>(0f, 0f, 0f);
                                    phi_12731_ = 0u;
                                    loop {
                                        let _e612 = phi_12728_;
                                        let _e614 = phi_12731_;
                                        local_69 = _e612;
                                        local_70 = _e612;
                                        local_71 = _e612;
                                        let _e615 = (_e614 < 32u);
                                        if _e615 {
                                            let _e616 = -(_e453);
                                            if (_e597.member_1.member <= _e610) {
                                            } else {
                                                phi_34027_ = true;
                                                break;
                                            }
                                            let _e619 = select(_e616, _e597.member_1.member, (_e616 < _e597.member_1.member));
                                            let _e621 = select(_e619, _e610, (_e619 > _e610));
                                            let _e622 = (_e621 - _e597.member_1.member);
                                            let _e623 = (_e610 - _e621);
                                            let _e624 = (_e610 - _e597.member_1.member);
                                            let _e625 = (0.000000001f * _e624);
                                            if (_e622 <= _e625) {
                                                let _e700 = (f32(_e614) * 0.03125f);
                                                let _e703 = (f32((_e614 + 1u)) * 0.03125f);
                                                let _e705 = ((_e700 + _e703) * 0.5f);
                                                phi_12832_ = type_56((_e597.member_1.member + ((_e624 * _e705) * _e705)), (_e624 * ((_e703 * _e703) - (_e700 * _e700))));
                                            } else {
                                                if (_e623 <= _e625) {
                                                    let _e683 = (f32((31u - _e614)) * 0.03125f);
                                                    let _e686 = (f32((32u - _e614)) * 0.03125f);
                                                    let _e688 = ((_e683 + _e686) * 0.5f);
                                                    phi_12831_ = type_56((_e610 - ((_e624 * _e688) * _e688)), (_e624 * ((_e686 * _e686) - (_e683 * _e683))));
                                                } else {
                                                    let _e631 = round(((32f * _e622) / (_e622 + _e623)));
                                                    let _e636 = select(select(u32(_e631), 0u, (_e631 < 0f)), 4294967295u, (_e631 > 4294967000f));
                                                    if (_e636 < 1u) {
                                                        phi_12815_ = 1u;
                                                    } else {
                                                        phi_12815_ = select(_e636, 31u, (_e636 > 31u));
                                                    }
                                                    let _e641 = phi_12815_;
                                                    if (_e614 < _e641) {
                                                        let _e662 = ((_e641 - 1u) - _e614);
                                                        let _e664 = f32(_e641);
                                                        let _e665 = (f32(_e662) / _e664);
                                                        let _e668 = (f32((_e662 + 1u)) / _e664);
                                                        let _e670 = ((_e665 + _e668) * 0.5f);
                                                        phi_12830_ = type_56((_e621 - ((_e622 * _e670) * _e670)), (_e622 * ((_e668 * _e668) - (_e665 * _e665))));
                                                    } else {
                                                        let _e644 = (_e614 - _e641);
                                                        let _e646 = f32((32u - _e641));
                                                        let _e647 = (f32(_e644) / _e646);
                                                        let _e650 = (f32((_e644 + 1u)) / _e646);
                                                        let _e652 = ((_e647 + _e650) * 0.5f);
                                                        phi_12830_ = type_56((_e621 + ((_e623 * _e652) * _e652)), (_e623 * ((_e650 * _e650) - (_e647 * _e647))));
                                                    }
                                                    let _e680 = phi_12830_;
                                                    phi_12831_ = _e680;
                                                }
                                                let _e698 = phi_12831_;
                                                phi_12832_ = _e698;
                                            }
                                            let _e715 = phi_12832_;
                                            let _e721 = (_e337.member.x + (_e337.member_1.x * _e715.member));
                                            let _e722 = (_e337.member.y + (_e337.member_1.y * _e715.member));
                                            let _e723 = (_e337.member.z + (_e337.member_1.z * _e715.member));
                                            let _e737 = (_e721 - _e403);
                                            let _e738 = (_e722 - _e406);
                                            let _e739 = (_e723 - _e409);
                                            let _e747 = ((((((_e721 * _e721) + (_e722 * _e722)) + (_e723 * _e723)) - (2f * (((_e721 * _e403) + (_e722 * _e406)) + (_e723 * _e409)))) + _e415) / (sqrt((((_e737 * _e737) + (_e738 * _e738)) + (_e739 * _e739))) + _e412));
                                            let _e751 = (1f - (abs((_e747 - 25000f)) * 0.00006666667f));
                                            if (_e751 != _e751) {
                                                phi_26545_ = true;
                                            } else {
                                                phi_26545_ = (0f >= _e751);
                                            }
                                            let _e755 = phi_26545_;
                                            let _e757 = -(_e747);
                                            phi_12729_ = (_e612 + vec3<f32>((exp((_e757 / _e421)) * _e715.member_1), (exp((_e757 / _e424)) * _e715.member_1), (select(_e751, 0f, _e755) * _e715.member_1)));
                                            phi_12732_ = (_e614 + 1u);
                                        } else {
                                            phi_12729_ = vec3<f32>();
                                            phi_12732_ = u32();
                                        }
                                        let _e769 = phi_12729_;
                                        let _e771 = phi_12732_;
                                        phi_12728_ = _e769;
                                        phi_12731_ = _e771;
                                        phi_34027_ = false;
                                        if !(_e615) {
                                            break;
                                        }
                                    }
                                    let _e774 = phi_34027_;
                                    if _e774 {
                                        break;
                                    }
                                    let _e776 = local_69;
                                    let _e783 = local_70;
                                    let _e785 = ((_e436 * 1.1f) * _e783.y);
                                    let _e790 = local_71;
                                    phi_36167_ = _e774;
                                    phi_12857_ = vec3<f32>(exp(-((((_e427 * _e776.x) + _e785) + (_e439 * _e790.z)))), exp(-((((_e430 * _e776.x) + _e785) + (_e442 * _e790.z)))), exp(-((((_e433 * _e776.x) + _e785) + (_e445 * _e790.z)))));
                                } else {
                                    phi_36167_ = false;
                                    phi_12857_ = vec3<f32>();
                                }
                                let _e806 = phi_36167_;
                                let _e808 = phi_12857_;
                                phi_36166_ = _e806;
                                phi_12859_ = _e808;
                                phi_12860_ = select(true, false, _e603);
                            } else {
                                phi_36166_ = false;
                                phi_12859_ = vec3<f32>();
                                phi_12860_ = true;
                            }
                            let _e811 = phi_36166_;
                            let _e813 = phi_12859_;
                            let _e815 = phi_12860_;
                            phi_36164_ = _e811;
                            phi_12867_ = (_e356 * select(_e813, vec3<f32>(1f, 1f, 1f), vec3(_e815)));
                            phi_12868_ = type_41(vec4<f32>(_e337.member.x, _e337.member.y, _e337.member.z, _e337.member_5.x), vec4<f32>(_e337.member_1.x, _e337.member_1.y, _e337.member_1.z, 0f), vec4<f32>(_e337.member_2.x, _e337.member_2.y, _e337.member_2.z, 0f));
                        } else {
                            phi_36164_ = false;
                            phi_12867_ = _e356;
                            phi_12868_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e820 = phi_36164_;
                        let _e822 = phi_12867_;
                        let _e824 = phi_12868_;
                        phi_36163_ = _e820;
                        phi_12869_ = _e822;
                        phi_12870_ = _e824;
                    }
                    let _e826 = phi_36163_;
                    let _e828 = phi_12869_;
                    let _e830 = phi_12870_;
                    let _e834 = (_e337.member.x + (_e337.member_1.x * _e337.member_5.x));
                    let _e835 = (_e337.member.y + (_e337.member_1.y * _e337.member_5.x));
                    let _e836 = (_e337.member.z + (_e337.member_1.z * _e337.member_5.x));
                    let _e839 = global_1.member.member_1[0u];
                    let _e842 = global_1.member.member_1[1u];
                    let _e843 = (_e839 + _e842);
                    let _e844 = (3u * _e337.member_4.z);
                    let _e846 = global_2.member[_e844];
                    let _e849 = global_2.member[(_e844 + 1u)];
                    let _e855 = select(select(u32(_e849.w), 0u, (_e849.w < 0f)), 4294967295u, (_e849.w > 4294967000f));
                    let _e856 = (3u * _e843);
                    let _e858 = (_e856 + (5u * _e337.member_4.z));
                    let _e860 = global_2.member[_e858];
                    let _e863 = global_2.member[(_e858 + 1u)];
                    let _e866 = global_2.member[(_e858 + 2u)];
                    let _e878 = global_2.member[(_e858 + 3u)];
                    let _e881 = global_2.member[(_e858 + 4u)];
                    let _e888 = (_e855 == 0u);
                    if _e888 {
                        let _e911 = (_e860.x - _e846.x);
                        let _e912 = (_e860.y - _e846.y);
                        let _e913 = (_e860.z - _e846.z);
                        let _e914 = (_e863.x - _e846.x);
                        let _e915 = (_e863.y - _e846.y);
                        let _e916 = (_e863.z - _e846.z);
                        let _e919 = ((_e912 * _e916) - (_e915 * _e913));
                        let _e922 = ((_e913 * _e914) - (_e916 * _e911));
                        let _e925 = ((_e911 * _e915) - (_e914 * _e912));
                        let _e932 = (1f / sqrt((((_e919 * _e919) + (_e922 * _e922)) + (_e925 * _e925))));
                        let _e936 = vec3<f32>((_e919 * _e932), (_e922 * _e932), (_e925 * _e932));
                        if (select(select(u32(_e866.w), 0u, (_e866.w < 0f)), 4294967295u, (_e866.w > 4294967000f)) == 1u) {
                            let _e941 = ((1f - _e337.member_5.y) - _e337.member_5.z);
                            let _e960 = (((_e866.x * _e941) + (_e878.x * _e337.member_5.y)) + (_e881.x * _e337.member_5.z));
                            let _e961 = (((_e866.y * _e941) + (_e878.y * _e337.member_5.y)) + (_e881.y * _e337.member_5.z));
                            let _e962 = (((_e866.z * _e941) + (_e878.z * _e337.member_5.y)) + (_e881.z * _e337.member_5.z));
                            let _e967 = (((_e960 * _e960) + (_e961 * _e961)) + (_e962 * _e962));
                            if (_e967 > 0f) {
                                let _e970 = (1f / sqrt(_e967));
                                phi_13116_ = vec3<f32>((_e960 * _e970), (_e961 * _e970), (_e962 * _e970));
                            } else {
                                phi_13116_ = _e936;
                            }
                            let _e976 = phi_13116_;
                            phi_13117_ = _e976;
                        } else {
                            phi_13117_ = _e936;
                        }
                        let _e978 = phi_13117_;
                        phi_13150_ = type_60(_e936, _e978);
                    } else {
                        if (_e855 == 1u) {
                            let _e904 = vec3<f32>(((_e834 - _e846.x) / _e846.w), ((_e835 - _e846.y) / _e846.w), ((_e836 - _e846.z) / _e846.w));
                            phi_13149_ = type_60(_e904, _e904);
                        } else {
                            let _e892 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e337.member_1.y > 0f)));
                            phi_13149_ = type_60(_e892, _e892);
                        }
                        let _e907 = phi_13149_;
                        phi_13150_ = _e907;
                    }
                    let _e981 = phi_13150_;
                    let _e995 = select(select(u32(_e863.w), 0u, (_e863.w < 0f)), 4294967295u, (_e863.w > 4294967000f));
                    phi_13273_ = 0i;
                    phi_13276_ = select(select(u32(_e860.w), 0u, (_e860.w < 0f)), 4294967295u, (_e860.w > 4294967000f));
                    loop {
                        let _e997 = phi_13273_;
                        let _e999 = phi_13276_;
                        local_72 = _e999;
                        if (_e997 < 4i) {
                            let _e1002 = global_4.member[_e999];
                            let _e1004 = (_e1002.member != 6u);
                            if _e1004 {
                                phi_13345_ = i32();
                                phi_13346_ = u32();
                            } else {
                                let _e1008 = (abs(_e834) / _e1002.member_6.z);
                                let _e1019 = (abs(_e836) / _e1002.member_6.w);
                                if ((select(select(u32(_e1008), 0u, (_e1008 < 0f)), 4294967295u, (_e1008 > 4294967000f)) % 2u) == 0u) {
                                    phi_13330_ = (_e834 > 0f);
                                } else {
                                    phi_13330_ = (_e834 <= 0f);
                                }
                                let _e1030 = phi_13330_;
                                if ((select(select(u32(_e1019), 0u, (_e1019 < 0f)), 4294967295u, (_e1019 > 4294967000f)) % 2u) == 0u) {
                                    phi_13336_ = (_e836 > 0f);
                                } else {
                                    phi_13336_ = (_e836 <= 0f);
                                }
                                let _e1034 = phi_13336_;
                                if (_e1030 != _e1034) {
                                    phi_13343_ = _e1002.member_1;
                                } else {
                                    phi_13343_ = _e1002.member_2;
                                }
                                let _e1039 = phi_13343_;
                                phi_13345_ = (_e997 + 1i);
                                phi_13346_ = _e1039;
                            }
                            let _e1042 = phi_13345_;
                            let _e1044 = phi_13346_;
                            phi_13274_ = _e1042;
                            phi_13277_ = _e1044;
                            phi_13348_ = select(true, false, _e1004);
                        } else {
                            phi_13274_ = i32();
                            phi_13277_ = u32();
                            phi_13348_ = false;
                        }
                        let _e1047 = phi_13274_;
                        let _e1049 = phi_13277_;
                        let _e1051 = phi_13348_;
                        phi_13273_ = _e1047;
                        phi_13276_ = _e1049;
                        if !(_e1051) {
                            break;
                        }
                    }
                    let _e1054 = local_72;
                    let _e1056 = global_4.member[_e1054];
                    let _e1058 = (_e1056.member == 4u);
                    if _e1058 {
                        if _e1058 {
                            if (_e1056.member_1 == 1u) {
                                phi_13407_ = true;
                            } else {
                                phi_13407_ = ((((-(_e337.member_1.x) * _e981.member_1.x) + (-(_e337.member_1.y) * _e981.member_1.y)) + (-(_e337.member_1.z) * _e981.member_1.z)) > 0f);
                            }
                            let _e6413 = phi_13407_;
                            if _e6413 {
                                phi_13416_ = vec3<f32>(_e1056.member_4.x, _e1056.member_4.y, _e1056.member_4.z);
                            } else {
                                phi_13416_ = vec3<f32>();
                            }
                            let _e6420 = phi_13416_;
                            phi_13418_ = _e6420;
                            phi_13419_ = select(true, false, _e6413);
                        } else {
                            phi_13418_ = vec3<f32>();
                            phi_13419_ = true;
                        }
                        let _e6423 = phi_13418_;
                        let _e6425 = phi_13419_;
                        let _e6427 = select(_e6423, vec3<f32>(0f, 0f, 0f), vec3(_e6425));
                        if _e376 {
                            phi_13850_ = 1f;
                        } else {
                            if ((_e384 & 1u) == 0u) {
                                phi_13849_ = 1f;
                            } else {
                                if (_e995 > 0u) {
                                    let _e6434 = global_1.member.member_2[1u];
                                    let _e6435 = (_e6434 + (_e995 - 1u));
                                    let _e6438 = global_6.member[(_e6435 + 1u)];
                                    let _e6440 = global_6.member[_e6435];
                                    if (_e855 == 1u) {
                                        let _e6501 = (_e846.x - _e337.member.x);
                                        let _e6502 = (_e846.y - _e337.member.y);
                                        let _e6503 = (_e846.z - _e337.member.z);
                                        let _e6508 = (((_e6501 * _e6501) + (_e6502 * _e6502)) + (_e6503 * _e6503));
                                        let _e6509 = (_e846.w * _e846.w);
                                        if (_e6508 <= (_e6509 * 1.000001f)) {
                                            let _e6523 = (_e834 - _e337.member.x);
                                            let _e6524 = (_e835 - _e337.member.y);
                                            let _e6525 = (_e836 - _e337.member.z);
                                            let _e6530 = (((_e6523 * _e6523) + (_e6524 * _e6524)) + (_e6525 * _e6525));
                                            phi_32302_ = (_e6530 / (((abs((((_e981.member.x * _e6523) + (_e981.member.y * _e6524)) + (_e981.member.z * _e6525))) / sqrt(_e6530)) * 12.566371f) * _e6509));
                                        } else {
                                            let _e6512 = (_e6509 / _e6508);
                                            if (_e6512 != _e6512) {
                                                phi_32321_ = true;
                                            } else {
                                                phi_32321_ = (1f <= _e6512);
                                            }
                                            let _e6516 = phi_32321_;
                                            let _e6517 = select(_e6512, 1f, _e6516);
                                            phi_32302_ = (0.15915494f / (_e6517 / (1f + sqrt((1f - _e6517)))));
                                        }
                                        let _e6546 = phi_32302_;
                                        phi_13841_ = _e6546;
                                    } else {
                                        let _e6443 = (_e834 - _e337.member.x);
                                        let _e6444 = (_e835 - _e337.member.y);
                                        let _e6445 = (_e836 - _e337.member.z);
                                        let _e6450 = (((_e6443 * _e6443) + (_e6444 * _e6444)) + (_e6445 * _e6445));
                                        let _e6461 = (abs((((_e981.member.x * _e6443) + (_e981.member.y * _e6444)) + (_e981.member.z * _e6445))) / sqrt(_e6450));
                                        if (_e6461 <= 0f) {
                                            phi_13840_ = 0f;
                                        } else {
                                            if _e888 {
                                                let _e6469 = (_e860.x - _e846.x);
                                                let _e6470 = (_e860.y - _e846.y);
                                                let _e6471 = (_e860.z - _e846.z);
                                                let _e6472 = (_e863.x - _e846.x);
                                                let _e6473 = (_e863.y - _e846.y);
                                                let _e6474 = (_e863.z - _e846.z);
                                                let _e6477 = ((_e6470 * _e6474) - (_e6473 * _e6471));
                                                let _e6480 = ((_e6471 * _e6472) - (_e6474 * _e6469));
                                                let _e6483 = ((_e6469 * _e6473) - (_e6472 * _e6470));
                                                phi_13837_ = (sqrt((((_e6477 * _e6477) + (_e6480 * _e6480)) + (_e6483 * _e6483))) * 0.5f);
                                            } else {
                                                phi_13837_ = ((12.566371f * _e846.w) * _e846.w);
                                            }
                                            let _e6492 = phi_13837_;
                                            phi_13840_ = (_e6450 / (_e6461 * _e6492));
                                        }
                                        let _e6496 = phi_13840_;
                                        phi_13841_ = _e6496;
                                    }
                                    let _e6548 = phi_13841_;
                                    let _e6549 = ((_e6438 - _e6440) * _e6548);
                                    if _e378 {
                                        phi_13847_ = 0f;
                                    } else {
                                        if (_e337.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_32342_ = 1f;
                                        } else {
                                            let _e6551 = (_e337.member_1.w * _e337.member_1.w);
                                            let _e6553 = (_e6551 + (_e6549 * _e6549));
                                            if (_e6553 == 0f) {
                                                phi_32339_ = 0f;
                                            } else {
                                                phi_32339_ = (_e6551 / _e6553);
                                            }
                                            let _e6557 = phi_32339_;
                                            phi_32342_ = _e6557;
                                        }
                                        let _e6559 = phi_32342_;
                                        phi_13847_ = _e6559;
                                    }
                                    let _e6561 = phi_13847_;
                                    phi_13848_ = _e6561;
                                } else {
                                    phi_13848_ = 1f;
                                }
                                let _e6563 = phi_13848_;
                                phi_13849_ = _e6563;
                            }
                            let _e6565 = phi_13849_;
                            phi_13850_ = _e6565;
                        }
                        let _e6567 = phi_13850_;
                        phi_19789_ = type_69(type_34(_e337.member, _e337.member_1, vec4<f32>(_e828.x, _e828.y, _e828.z, f32(_e362)), vec4<f32>((_e337.member_3.x + ((_e828.x * _e6427.x) * _e6567)), (_e337.member_3.y + ((_e828.y * _e6427.y) * _e6567)), (_e337.member_3.z + ((_e828.z * _e6427.z) * _e6567)), f32(_e390)), vec4<u32>(_e337.member_4.x, (_e337.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19790_ = type_69();
                        phi_19791_ = true;
                    } else {
                        if (_e1056.member == 7u) {
                            let _e6371 = type_34(_e337.member, _e337.member_1, vec4<f32>(_e828.x, _e828.y, _e828.z, f32(_e362)), vec4<f32>(_e337.member_3.x, _e337.member_3.y, _e337.member_3.z, f32(_e390)), vec4<u32>(_e337.member_4.x, ((_e337.member_4.y | 16u) & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f));
                            let _e6383 = type_34(_e6371.member, _e6371.member_1, _e6371.member_2, _e6371.member_3, vec4<u32>(_e6371.member_4.x, _e6371.member_4.y, _e337.member_4.z, _e6371.member_4.w), _e6371.member_5);
                            phi_19786_ = type_69(type_34(_e6383.member, _e6383.member_1, _e6383.member_2, _e6383.member_3, _e6383.member_4, _e337.member_5), _e830, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19787_ = type_69();
                            phi_19788_ = true;
                        } else {
                            let _e1060 = -(_e337.member_1.x);
                            let _e1061 = -(_e337.member_1.y);
                            let _e1062 = -(_e337.member_1.z);
                            if ((((_e981.member.x * _e1060) + (_e981.member.y * _e1061)) + (_e981.member.z * _e1062)) < 0f) {
                                phi_13902_ = -(_e981.member);
                            } else {
                                phi_13902_ = _e981.member;
                            }
                            let _e1074 = phi_13902_;
                            if ((((_e981.member_1.x * _e1060) + (_e981.member_1.y * _e1061)) + (_e981.member_1.z * _e1062)) < 0f) {
                                phi_13916_ = -(_e981.member_1);
                            } else {
                                phi_13916_ = _e981.member_1;
                            }
                            let _e1086 = phi_13916_;
                            let _e1089 = global_1.member.member_4[2u];
                            let _e1090 = abs(_e834);
                            let _e1091 = abs(_e835);
                            let _e1092 = abs(_e836);
                            let _e1094 = select(_e1091, _e1090, (_e1090 > _e1091));
                            let _e1098 = (0.00002f * (1f + select(_e1092, _e1094, (_e1094 > _e1092))));
                            if (_e1089 != _e1089) {
                                phi_26612_ = true;
                            } else {
                                phi_26612_ = (_e1098 >= _e1089);
                            }
                            let _e1102 = phi_26612_;
                            let _e1104 = (select(_e1089, _e1098, _e1102) * 2f);
                            let _e1111 = (_e834 + (_e1074.x * _e1104));
                            let _e1112 = (_e835 + (_e1074.y * _e1104));
                            let _e1113 = (_e836 + (_e1074.z * _e1104));
                            let _e1114 = vec3<f32>(_e1111, _e1112, _e1113);
                            if ((_e337.member_4.y & 4u) == 0u) {
                                phi_13949_ = 0f;
                            } else {
                                let _e1119 = global_1.member.member_4[0u];
                                phi_13949_ = _e1119;
                            }
                            let _e1121 = phi_13949_;
                            let _e1124 = select(-1f, 1f, (_e1086.z >= 0f));
                            let _e1126 = (-1f / (_e1124 + _e1086.z));
                            let _e1130 = ((_e1086.x * _e1086.y) * _e1126);
                            let _e1134 = (1f + (((_e1124 * _e1086.x) * _e1086.x) * _e1126));
                            let _e1135 = (_e1124 * _e1130);
                            let _e1137 = (-(_e1124) * _e1086.x);
                            let _e1140 = (_e1124 + ((_e1086.y * _e1086.y) * _e1126));
                            let _e1141 = -(_e1086.y);
                            if (_e1056.member == 0u) {
                                phi_34811_ = vec3<f32>(0f, 0f, 0f);
                                phi_34619_ = 1.5f;
                                phi_34540_ = vec3<f32>(0f, 0f, 0f);
                                phi_34463_ = vec3<f32>(_e1056.member_4.x, _e1056.member_4.y, _e1056.member_4.z);
                                phi_34384_ = 0u;
                                phi_34302_ = 1f;
                                phi_34225_ = 0u;
                                phi_34216_ = _e337.member_4.x;
                                phi_14628_ = type_69();
                                phi_14629_ = true;
                            } else {
                                if (_e1056.member == 1u) {
                                    let _e1690 = (_e1056.member_6.y * _e1056.member_6.y);
                                    if (_e1690 != _e1690) {
                                        phi_27304_ = true;
                                    } else {
                                        phi_27304_ = (_e1121 >= _e1690);
                                    }
                                    let _e1694 = phi_27304_;
                                    let _e1695 = select(_e1690, _e1121, _e1694);
                                    let _e1696 = (_e1695 >= 0.001f);
                                    if _e1696 {
                                        phi_34868_ = vec3<f32>(_e1056.member_4.x, _e1056.member_4.y, _e1056.member_4.z);
                                        phi_34662_ = _e1056.member_6.x;
                                        phi_34515_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34203_ = _e337.member_4.x;
                                        phi_14024_ = vec3<f32>();
                                        phi_14025_ = true;
                                    } else {
                                        let _e1698 = ((_e337.member_4.x * 747796405u) + 2891336453u);
                                        let _e1706 = (((_e1698 >> bitcast<u32>((((_e1698 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1698) * 277803737u);
                                        let _e1709 = ((_e1706 >> bitcast<u32>(22i)) ^ _e1706);
                                        let _e1719 = abs((((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z)));
                                        let _e1722 = (1f / _e1056.member_6.x);
                                        let _e1726 = ((_e1722 * _e1722) * (1f - (_e1719 * _e1719)));
                                        if (_e1726 >= 1f) {
                                            phi_27344_ = 1f;
                                        } else {
                                            let _e1729 = sqrt((1f - _e1726));
                                            if (_e1719 != _e1719) {
                                                phi_27355_ = true;
                                            } else {
                                                phi_27355_ = (_e1729 <= _e1719);
                                            }
                                            let _e1733 = phi_27355_;
                                            let _e1738 = pow(((1f - _e1056.member_6.x) / (1f + _e1056.member_6.x)), 2f);
                                            phi_27344_ = (_e1738 + ((1f - _e1738) * pow((1f - select(_e1719, _e1729, _e1733)), 5f)));
                                        }
                                        let _e1745 = phi_27344_;
                                        let _e1746 = ((f32((_e1709 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1745);
                                        if _e1746 {
                                            let _e1759 = (((_e1086.x * _e337.member_1.x) + (_e1086.y * _e337.member_1.y)) + (_e1086.z * _e337.member_1.z));
                                            phi_34516_ = vec3<f32>(0f, 0f, 0f);
                                            phi_14022_ = (_e351 - vec3<f32>(((_e1086.x * 2f) * _e1759), ((_e1086.y * 2f) * _e1759), ((_e1086.z * 2f) * _e1759)));
                                        } else {
                                            phi_34516_ = vec3<f32>(_e1056.member_4.x, _e1056.member_4.y, _e1056.member_4.z);
                                            phi_14022_ = vec3<f32>();
                                        }
                                        let _e1766 = phi_34516_;
                                        let _e1768 = phi_14022_;
                                        phi_34868_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34662_ = 1.5f;
                                        phi_34515_ = _e1766;
                                        phi_34203_ = _e1709;
                                        phi_14024_ = _e1768;
                                        phi_14025_ = select(true, false, _e1746);
                                    }
                                    let _e1778 = phi_34868_;
                                    let _e1780 = phi_34662_;
                                    let _e1782 = phi_34515_;
                                    let _e1784 = phi_34203_;
                                    let _e1786 = phi_14024_;
                                    let _e1788 = phi_14025_;
                                    phi_34814_ = _e1778;
                                    phi_34622_ = _e1780;
                                    phi_34543_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34466_ = _e1782;
                                    phi_34387_ = select(0u, 1u, _e1696);
                                    phi_34305_ = select(1f, _e1695, _e1696);
                                    phi_34228_ = select(0u, 2u, _e1696);
                                    phi_34154_ = _e1784;
                                    phi_14497_ = _e1114;
                                    phi_14498_ = _e1786;
                                    phi_14499_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14500_ = _e1788;
                                } else {
                                    if (_e1056.member == 3u) {
                                        phi_34157_ = _e337.member_4.x;
                                        phi_14176_ = vec3<f32>();
                                        phi_14177_ = vec3<f32>();
                                        phi_14178_ = vec3<f32>();
                                        phi_14179_ = true;
                                        phi_14180_ = false;
                                    } else {
                                        let _e1145 = (_e1056.member == 5u);
                                        if _e1145 {
                                            phi_34158_ = _e337.member_4.x;
                                            phi_14172_ = vec3<f32>();
                                            phi_14173_ = vec3<f32>();
                                            phi_14174_ = vec3<f32>();
                                            phi_14175_ = false;
                                        } else {
                                            let _e1146 = (_e1056.member == 2u);
                                            if _e1146 {
                                                let _e1151 = (((_e337.member_1.x * _e981.member_1.x) + (_e337.member_1.y * _e981.member_1.y)) + (_e337.member_1.z * _e981.member_1.z));
                                                let _e1152 = (_e1151 > 0f);
                                                if _e1152 {
                                                    phi_14057_ = -(_e981.member_1);
                                                } else {
                                                    phi_14057_ = _e981.member_1;
                                                }
                                                let _e1155 = phi_14057_;
                                                if _e1152 {
                                                    phi_14063_ = _e1056.member_6.x;
                                                } else {
                                                    phi_14063_ = (1f / _e1056.member_6.x);
                                                }
                                                let _e1160 = phi_14063_;
                                                let _e1161 = abs(_e1151);
                                                let _e1162 = (_e1160 * _e1160);
                                                let _e1165 = (_e1162 * (1f - (_e1161 * _e1161)));
                                                if (_e1165 >= 1f) {
                                                    phi_26674_ = 1f;
                                                } else {
                                                    let _e1168 = sqrt((1f - _e1165));
                                                    if (_e1161 != _e1161) {
                                                        phi_26685_ = true;
                                                    } else {
                                                        phi_26685_ = (_e1168 <= _e1161);
                                                    }
                                                    let _e1172 = phi_26685_;
                                                    let _e1177 = pow(((1f - _e1056.member_6.x) / (1f + _e1056.member_6.x)), 2f);
                                                    phi_26674_ = (_e1177 + ((1f - _e1177) * pow((1f - select(_e1161, _e1168, _e1172)), 5f)));
                                                }
                                                let _e1184 = phi_26674_;
                                                let _e1192 = (((_e337.member_1.x * _e1155.x) + (_e337.member_1.y * _e1155.y)) + (_e337.member_1.z * _e1155.z));
                                                let _e1196 = (1f - (_e1162 * (1f - (_e1192 * _e1192))));
                                                if (_e1196 > 0f) {
                                                    let _e1199 = ((_e337.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1207 = (((_e1199 >> bitcast<u32>((((_e1199 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1199) * 277803737u);
                                                    let _e1210 = ((_e1207 >> bitcast<u32>(22i)) ^ _e1207);
                                                    let _e1215 = ((f32((_e1210 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1184);
                                                    if _e1215 {
                                                        let _e1225 = sqrt(_e1196);
                                                        let _e1229 = (((_e337.member_1.x - (_e1155.x * _e1192)) * _e1160) - (_e1155.x * _e1225));
                                                        let _e1230 = (((_e337.member_1.y - (_e1155.y * _e1192)) * _e1160) - (_e1155.y * _e1225));
                                                        let _e1231 = (((_e337.member_1.z - (_e1155.z * _e1192)) * _e1160) - (_e1155.z * _e1225));
                                                        let _e1238 = (1f / sqrt((((_e1229 * _e1229) + (_e1230 * _e1230)) + (_e1231 * _e1231))));
                                                        phi_14144_ = vec3<f32>((_e834 - (_e1155.x * _e1104)), (_e835 - (_e1155.y * _e1104)), (_e836 - (_e1155.z * _e1104)));
                                                        phi_14145_ = vec3<f32>((_e1229 * _e1238), (_e1230 * _e1238), (_e1231 * _e1238));
                                                        phi_14146_ = vec3<f32>(_e1056.member_4.x, _e1056.member_4.y, _e1056.member_4.z);
                                                    } else {
                                                        phi_14144_ = vec3<f32>();
                                                        phi_14145_ = vec3<f32>();
                                                        phi_14146_ = vec3<f32>();
                                                    }
                                                    let _e1256 = phi_14144_;
                                                    let _e1258 = phi_14145_;
                                                    let _e1260 = phi_14146_;
                                                    phi_34161_ = _e1210;
                                                    phi_14148_ = _e1256;
                                                    phi_14149_ = _e1258;
                                                    phi_14150_ = _e1260;
                                                    phi_14151_ = select(true, false, _e1215);
                                                } else {
                                                    phi_34161_ = _e337.member_4.x;
                                                    phi_14148_ = vec3<f32>();
                                                    phi_14149_ = vec3<f32>();
                                                    phi_14150_ = vec3<f32>();
                                                    phi_14151_ = true;
                                                }
                                                let _e1263 = phi_34161_;
                                                let _e1265 = phi_14148_;
                                                let _e1267 = phi_14149_;
                                                let _e1269 = phi_14150_;
                                                let _e1271 = phi_14151_;
                                                if _e1271 {
                                                    let _e1279 = (((_e981.member_1.x * _e337.member_1.x) + (_e981.member_1.y * _e337.member_1.y)) + (_e981.member_1.z * _e337.member_1.z));
                                                    phi_14166_ = vec3<f32>((_e834 + (_e1155.x * _e1104)), (_e835 + (_e1155.y * _e1104)), (_e836 + (_e1155.z * _e1104)));
                                                    phi_14167_ = (_e351 - vec3<f32>(((_e981.member_1.x * 2f) * _e1279), ((_e981.member_1.y * 2f) * _e1279), ((_e981.member_1.z * 2f) * _e1279)));
                                                } else {
                                                    phi_14166_ = _e1265;
                                                    phi_14167_ = _e1267;
                                                }
                                                let _e1293 = phi_14166_;
                                                let _e1295 = phi_14167_;
                                                phi_34159_ = _e1263;
                                                phi_14169_ = _e1293;
                                                phi_14170_ = _e1295;
                                                phi_14171_ = select(_e1269, vec3<f32>(1f, 1f, 1f), vec3(_e1271));
                                            } else {
                                                phi_34159_ = _e337.member_4.x;
                                                phi_14169_ = vec3<f32>();
                                                phi_14170_ = vec3<f32>();
                                                phi_14171_ = vec3<f32>();
                                            }
                                            let _e1299 = phi_34159_;
                                            let _e1301 = phi_14169_;
                                            let _e1303 = phi_14170_;
                                            let _e1305 = phi_14171_;
                                            phi_34158_ = _e1299;
                                            phi_14172_ = _e1301;
                                            phi_14173_ = _e1303;
                                            phi_14174_ = _e1305;
                                            phi_14175_ = _e1146;
                                        }
                                        let _e1307 = phi_34158_;
                                        let _e1309 = phi_14172_;
                                        let _e1311 = phi_14173_;
                                        let _e1313 = phi_14174_;
                                        let _e1315 = phi_14175_;
                                        phi_34157_ = _e1307;
                                        phi_14176_ = _e1309;
                                        phi_14177_ = _e1311;
                                        phi_14178_ = _e1313;
                                        phi_14179_ = _e1145;
                                        phi_14180_ = _e1315;
                                    }
                                    let _e1317 = phi_34157_;
                                    let _e1319 = phi_14176_;
                                    let _e1321 = phi_14177_;
                                    let _e1323 = phi_14178_;
                                    let _e1325 = phi_14179_;
                                    let _e1327 = phi_14180_;
                                    if _e1325 {
                                        let _e1330 = (_e1056.member_6.y * _e1056.member_6.y);
                                        if (_e1330 != _e1330) {
                                            phi_26758_ = true;
                                        } else {
                                            phi_26758_ = (_e1121 >= _e1330);
                                        }
                                        let _e1334 = phi_26758_;
                                        let _e1335 = select(_e1330, _e1121, _e1334);
                                        let _e1347 = select(2u, 0u, (_e1056.member == 5u));
                                        let _e1348 = (_e1335 < 0.001f);
                                        if _e1348 {
                                            let _e1356 = (((_e1086.x * _e337.member_1.x) + (_e1086.y * _e337.member_1.y)) + (_e1086.z * _e337.member_1.z));
                                            let _e1367 = abs((((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z)));
                                            if (_e1347 == 0u) {
                                                let _e1649 = (1f - _e1367);
                                                if (_e1649 != _e1649) {
                                                    phi_27289_ = true;
                                                } else {
                                                    phi_27289_ = (0f >= _e1649);
                                                }
                                                let _e1653 = phi_27289_;
                                                let _e1655 = pow(select(_e1649, 0f, _e1653), 5f);
                                                let _e1656 = (1f - _e1655);
                                                phi_14483_ = vec3<f32>(((_e1056.member_4.x * _e1656) + _e1655), ((_e1056.member_4.y * _e1656) + _e1655), ((_e1056.member_4.z * _e1656) + _e1655));
                                            } else {
                                                if (_e1347 == 1u) {
                                                    let _e1631 = (0.44444448f * (1f - (_e1367 * _e1367)));
                                                    if (_e1631 >= 1f) {
                                                        phi_27250_ = 1f;
                                                    } else {
                                                        let _e1634 = sqrt((1f - _e1631));
                                                        if (_e1367 != _e1367) {
                                                            phi_27261_ = true;
                                                        } else {
                                                            phi_27261_ = (_e1634 <= _e1367);
                                                        }
                                                        let _e1638 = phi_27261_;
                                                        phi_27250_ = (0.040000003f + (0.96f * pow((1f - select(_e1367, _e1634, _e1638)), 5f)));
                                                    }
                                                    let _e1645 = phi_27250_;
                                                    phi_14482_ = vec3(_e1645);
                                                } else {
                                                    let _e1371 = select(_e1367, 0f, (_e1367 < 0f));
                                                    let _e1373 = select(_e1371, 1f, (_e1371 > 1f));
                                                    let _e1375 = (1f - (_e1373 * _e1373));
                                                    let _e1382 = ((_e1056.member_4.x * _e1056.member_4.x) - (_e1056.member_5.x * _e1056.member_5.x));
                                                    let _e1383 = ((_e1056.member_4.y * _e1056.member_4.y) - (_e1056.member_5.y * _e1056.member_5.y));
                                                    let _e1384 = ((_e1056.member_4.z * _e1056.member_4.z) - (_e1056.member_5.z * _e1056.member_5.z));
                                                    let _e1386 = (_e1056.member_4.x * (_e1056.member_5.x + _e1056.member_5.x));
                                                    let _e1388 = (_e1056.member_4.y * (_e1056.member_5.y + _e1056.member_5.y));
                                                    let _e1390 = (_e1056.member_4.z * (_e1056.member_5.z + _e1056.member_5.z));
                                                    let _e1397 = ((_e1382 * _e1382) + (_e1386 * _e1386));
                                                    let _e1398 = ((_e1383 * _e1383) + (_e1388 * _e1388));
                                                    let _e1399 = ((_e1384 * _e1384) + (_e1390 * _e1390));
                                                    let _e1404 = ((_e1375 * _e1382) / _e1397);
                                                    let _e1405 = ((_e1375 * _e1383) / _e1398);
                                                    let _e1406 = ((_e1375 * _e1384) / _e1399);
                                                    let _e1407 = (1f - _e1404);
                                                    let _e1408 = (1f - _e1405);
                                                    let _e1409 = (1f - _e1406);
                                                    let _e1411 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1404, _e1405, _e1406));
                                                    let _e1412 = (_e1375 * _e1386);
                                                    let _e1413 = (_e1375 * _e1388);
                                                    let _e1414 = (_e1375 * _e1390);
                                                    let _e1419 = (vec3<f32>(_e1412, _e1413, _e1414) / vec3<f32>(_e1397, _e1398, _e1399));
                                                    let _e1432 = ((_e1411.x * _e1411.x) + (_e1419.x * _e1419.x));
                                                    let _e1433 = ((_e1411.y * _e1411.y) + (_e1419.y * _e1419.y));
                                                    let _e1434 = ((_e1411.z * _e1411.z) + (_e1419.z * _e1419.z));
                                                    if (_e1432 != _e1432) {
                                                        phi_26920_ = true;
                                                    } else {
                                                        phi_26920_ = (0f >= _e1432);
                                                    }
                                                    let _e1438 = phi_26920_;
                                                    let _e1440 = sqrt(select(_e1432, 0f, _e1438));
                                                    if (_e1433 != _e1433) {
                                                        phi_26935_ = true;
                                                    } else {
                                                        phi_26935_ = (0f >= _e1433);
                                                    }
                                                    let _e1444 = phi_26935_;
                                                    let _e1446 = sqrt(select(_e1433, 0f, _e1444));
                                                    if (_e1434 != _e1434) {
                                                        phi_26950_ = true;
                                                    } else {
                                                        phi_26950_ = (0f >= _e1434);
                                                    }
                                                    let _e1450 = phi_26950_;
                                                    let _e1452 = sqrt(select(_e1434, 0f, _e1450));
                                                    let _e1456 = ((_e1440 + _e1407) * 0.5f);
                                                    let _e1457 = ((_e1446 + _e1408) * 0.5f);
                                                    let _e1458 = ((_e1452 + _e1409) * 0.5f);
                                                    if (_e1456 != _e1456) {
                                                        phi_26977_ = true;
                                                    } else {
                                                        phi_26977_ = (0f >= _e1456);
                                                    }
                                                    let _e1462 = phi_26977_;
                                                    let _e1464 = sqrt(select(_e1456, 0f, _e1462));
                                                    if (_e1457 != _e1457) {
                                                        phi_26992_ = true;
                                                    } else {
                                                        phi_26992_ = (0f >= _e1457);
                                                    }
                                                    let _e1468 = phi_26992_;
                                                    let _e1470 = sqrt(select(_e1457, 0f, _e1468));
                                                    if (_e1458 != _e1458) {
                                                        phi_27007_ = true;
                                                    } else {
                                                        phi_27007_ = (0f >= _e1458);
                                                    }
                                                    let _e1474 = phi_27007_;
                                                    let _e1476 = sqrt(select(_e1458, 0f, _e1474));
                                                    let _e1477 = vec3<f32>(_e1464, _e1470, _e1476);
                                                    let _e1481 = ((_e1440 - _e1407) * 0.5f);
                                                    let _e1482 = ((_e1446 - _e1408) * 0.5f);
                                                    let _e1483 = ((_e1452 - _e1409) * 0.5f);
                                                    if (_e1481 != _e1481) {
                                                        phi_27034_ = true;
                                                    } else {
                                                        phi_27034_ = (0f >= _e1481);
                                                    }
                                                    let _e1487 = phi_27034_;
                                                    let _e1489 = sqrt(select(_e1481, 0f, _e1487));
                                                    if (_e1482 != _e1482) {
                                                        phi_27049_ = true;
                                                    } else {
                                                        phi_27049_ = (0f >= _e1482);
                                                    }
                                                    let _e1493 = phi_27049_;
                                                    let _e1495 = sqrt(select(_e1482, 0f, _e1493));
                                                    if (_e1483 != _e1483) {
                                                        phi_27064_ = true;
                                                    } else {
                                                        phi_27064_ = (0f >= _e1483);
                                                    }
                                                    let _e1499 = phi_27064_;
                                                    let _e1501 = sqrt(select(_e1483, 0f, _e1499));
                                                    if ((_e1412 / _e1397) < 0f) {
                                                        phi_14359_ = -(_e1489);
                                                    } else {
                                                        phi_14359_ = _e1489;
                                                    }
                                                    let _e1505 = phi_14359_;
                                                    if ((_e1413 / _e1398) < 0f) {
                                                        phi_14367_ = -(_e1495);
                                                    } else {
                                                        phi_14367_ = _e1495;
                                                    }
                                                    let _e1509 = phi_14367_;
                                                    if ((_e1414 / _e1399) < 0f) {
                                                        phi_14375_ = -(_e1501);
                                                    } else {
                                                        phi_14375_ = _e1501;
                                                    }
                                                    let _e1513 = phi_14375_;
                                                    let _e1517 = (_e1056.member_5.x * _e1373);
                                                    let _e1518 = (_e1056.member_5.y * _e1373);
                                                    let _e1519 = (_e1056.member_5.z * _e1373);
                                                    let _e1529 = vec3<f32>(((_e1056.member_4.x * _e1464) - (_e1056.member_5.x * _e1505)), ((_e1056.member_4.y * _e1470) - (_e1056.member_5.y * _e1509)), ((_e1056.member_4.z * _e1476) - (_e1056.member_5.z * _e1513)));
                                                    let _e1536 = ((_e1056.member_4.x * _e1505) + (_e1056.member_5.x * _e1464));
                                                    let _e1537 = ((_e1056.member_4.y * _e1509) + (_e1056.member_5.y * _e1470));
                                                    let _e1538 = ((_e1056.member_4.z * _e1513) + (_e1056.member_5.z * _e1476));
                                                    let _e1540 = vec3<f32>((_e1056.member_4.x * _e1373), (_e1056.member_4.y * _e1373), (_e1056.member_4.z * _e1373));
                                                    let _e1541 = (_e1540 - _e1477);
                                                    let _e1542 = (_e1517 - _e1505);
                                                    let _e1543 = (_e1518 - _e1509);
                                                    let _e1544 = (_e1519 - _e1513);
                                                    let _e1557 = (_e1540 + _e1477);
                                                    let _e1558 = (_e1517 + _e1505);
                                                    let _e1559 = (_e1518 + _e1509);
                                                    let _e1560 = (_e1519 + _e1513);
                                                    let _e1570 = ((_e1557.x * _e1557.x) + (_e1558 * _e1558));
                                                    let _e1571 = ((_e1557.y * _e1557.y) + (_e1559 * _e1559));
                                                    let _e1572 = ((_e1557.z * _e1557.z) + (_e1560 * _e1560));
                                                    let _e1582 = vec3(_e1373);
                                                    let _e1583 = (_e1582 - _e1529);
                                                    let _e1584 = -(vec3<f32>(_e1536, _e1537, _e1538));
                                                    let _e1600 = (_e1582 + _e1529);
                                                    let _e1610 = ((_e1600.x * _e1600.x) + (_e1536 * _e1536));
                                                    let _e1611 = ((_e1600.y * _e1600.y) + (_e1537 * _e1537));
                                                    let _e1612 = ((_e1600.z * _e1600.z) + (_e1538 * _e1538));
                                                    phi_14482_ = vec3<f32>((((((_e1541.x * _e1541.x) + (_e1542 * _e1542)) / select(0.000000000000000000000000000001f, _e1570, (_e1570 > 0.000000000000000000000000000001f))) + (((_e1583.x * _e1583.x) + (_e1584.x * _e1584.x)) / select(0.000000000000000000000000000001f, _e1610, (_e1610 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1541.y * _e1541.y) + (_e1543 * _e1543)) / select(0.000000000000000000000000000001f, _e1571, (_e1571 > 0.000000000000000000000000000001f))) + (((_e1583.y * _e1583.y) + (_e1584.y * _e1584.y)) / select(0.000000000000000000000000000001f, _e1611, (_e1611 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1541.z * _e1541.z) + (_e1544 * _e1544)) / select(0.000000000000000000000000000001f, _e1572, (_e1572 > 0.000000000000000000000000000001f))) + (((_e1583.z * _e1583.z) + (_e1584.z * _e1584.z)) / select(0.000000000000000000000000000001f, _e1612, (_e1612 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e1648 = phi_14482_;
                                                phi_14483_ = _e1648;
                                            }
                                            let _e1665 = phi_14483_;
                                            phi_14484_ = (_e351 - vec3<f32>(((_e1086.x * 2f) * _e1356), ((_e1086.y * 2f) * _e1356), ((_e1086.z * 2f) * _e1356)));
                                            phi_14485_ = _e1665;
                                        } else {
                                            phi_14484_ = vec3<f32>();
                                            phi_14485_ = vec3<f32>();
                                        }
                                        let _e1667 = phi_14484_;
                                        let _e1669 = phi_14485_;
                                        phi_34545_ = vec3<f32>(_e1056.member_5.x, _e1056.member_5.y, _e1056.member_5.z);
                                        phi_34468_ = vec3<f32>(_e1056.member_4.x, _e1056.member_4.y, _e1056.member_4.z);
                                        phi_34389_ = _e1347;
                                        phi_34307_ = _e1335;
                                        phi_14487_ = _e1667;
                                        phi_14488_ = _e1669;
                                        phi_14489_ = _e1348;
                                    } else {
                                        phi_34545_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34468_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34389_ = 0u;
                                        phi_34307_ = 1f;
                                        phi_14487_ = _e1321;
                                        phi_14488_ = _e1323;
                                        phi_14489_ = _e1327;
                                    }
                                    let _e1671 = phi_34545_;
                                    let _e1673 = phi_34468_;
                                    let _e1675 = phi_34389_;
                                    let _e1677 = phi_34307_;
                                    let _e1679 = phi_14487_;
                                    let _e1681 = phi_14488_;
                                    let _e1683 = phi_14489_;
                                    phi_34814_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34622_ = 1.5f;
                                    phi_34543_ = _e1671;
                                    phi_34466_ = _e1673;
                                    phi_34387_ = _e1675;
                                    phi_34305_ = _e1677;
                                    phi_34228_ = select(0u, 1u, _e1325);
                                    phi_34154_ = _e1317;
                                    phi_14497_ = select(_e1319, _e1114, vec3(_e1325));
                                    phi_14498_ = _e1679;
                                    phi_14499_ = _e1681;
                                    phi_14500_ = select(true, false, _e1683);
                                }
                                let _e1793 = phi_34814_;
                                let _e1795 = phi_34622_;
                                let _e1797 = phi_34543_;
                                let _e1799 = phi_34466_;
                                let _e1801 = phi_34387_;
                                let _e1803 = phi_34305_;
                                let _e1805 = phi_34228_;
                                let _e1807 = phi_34154_;
                                let _e1809 = phi_14497_;
                                let _e1811 = phi_14498_;
                                let _e1813 = phi_14499_;
                                let _e1815 = phi_14500_;
                                let _e1816 = select(true, false, _e1815);
                                if _e1816 {
                                    if (_e362 >= _e381) {
                                        phi_34213_ = _e1807;
                                        phi_14617_ = _e828;
                                        phi_14618_ = type_69();
                                        phi_14619_ = true;
                                    } else {
                                        let _e1829 = global_1.member.member_4[1u];
                                        if (sqrt((((_e1813.x * _e1813.x) + (_e1813.y * _e1813.y)) + (_e1813.z * _e1813.z))) <= _e1829) {
                                            phi_34208_ = _e1807;
                                            phi_14580_ = _e828;
                                            phi_14581_ = false;
                                        } else {
                                            let _e1833 = global_1.member.member_7[1u];
                                            if (_e1833 == 1u) {
                                                let _e1838 = global_1.member.member_4[3u];
                                                if (f32(_e362) >= _e1838) {
                                                    let _e1841 = select(_e1813.y, _e1813.x, (_e1813.x > _e1813.y));
                                                    let _e1843 = select(_e1813.z, _e1841, (_e1841 > _e1813.z));
                                                    if (_e1843 != _e1843) {
                                                        phi_27421_ = true;
                                                    } else {
                                                        phi_27421_ = (1f <= _e1843);
                                                    }
                                                    let _e1847 = phi_27421_;
                                                    let _e1848 = select(_e1843, 1f, _e1847);
                                                    let _e1850 = ((_e1807 * 747796405u) + 2891336453u);
                                                    let _e1858 = (((_e1850 >> bitcast<u32>((((_e1850 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1850) * 277803737u);
                                                    let _e1861 = ((_e1858 >> bitcast<u32>(22i)) ^ _e1858);
                                                    let _e1866 = ((f32((_e1861 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1848);
                                                    if _e1866 {
                                                        phi_14569_ = vec3<f32>();
                                                    } else {
                                                        phi_14569_ = vec3<f32>((_e828.x / _e1848), (_e828.y / _e1848), (_e828.z / _e1848));
                                                    }
                                                    let _e1875 = phi_14569_;
                                                    phi_34211_ = _e1861;
                                                    phi_14571_ = _e1875;
                                                    phi_14572_ = select(true, false, _e1866);
                                                } else {
                                                    phi_34211_ = _e1807;
                                                    phi_14571_ = _e828;
                                                    phi_14572_ = true;
                                                }
                                                let _e1878 = phi_34211_;
                                                let _e1880 = phi_14571_;
                                                let _e1882 = phi_14572_;
                                                phi_34210_ = _e1878;
                                                phi_14573_ = _e1880;
                                                phi_14574_ = _e1882;
                                            } else {
                                                phi_34210_ = _e1807;
                                                phi_14573_ = _e828;
                                                phi_14574_ = true;
                                            }
                                            let _e1884 = phi_34210_;
                                            let _e1886 = phi_14573_;
                                            let _e1888 = phi_14574_;
                                            phi_34208_ = _e1884;
                                            phi_14580_ = select(_e828, _e1886, vec3(_e1888));
                                            phi_14581_ = select(false, true, _e1888);
                                        }
                                        let _e1893 = phi_34208_;
                                        let _e1895 = phi_14580_;
                                        let _e1897 = phi_14581_;
                                        if _e1897 {
                                            let _e1898 = (_e1895 * _e1813);
                                            let _e1909 = (_e1811 * (1f / sqrt((((_e1811.x * _e1811.x) + (_e1811.y * _e1811.y)) + (_e1811.z * _e1811.z)))));
                                            phi_14615_ = type_69(type_34(vec4<f32>(_e1809.x, _e1809.y, _e1809.z, _e337.member.w), vec4<f32>(_e1909.x, _e1909.y, _e1909.z, _e337.member_1.w), vec4<f32>(_e1898.x, _e1898.y, _e1898.z, f32((_e362 + 1u))), vec4<f32>(_e337.member_3.x, _e337.member_3.y, _e337.member_3.z, f32(_e390)), vec4<u32>(_e1893, ((_e337.member_4.y | 2u) & 4294967263u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14615_ = type_69();
                                        }
                                        let _e1933 = phi_14615_;
                                        phi_34213_ = _e1893;
                                        phi_14617_ = _e1895;
                                        phi_14618_ = _e1933;
                                        phi_14619_ = select(true, false, _e1897);
                                    }
                                    let _e1936 = phi_34213_;
                                    let _e1938 = phi_14617_;
                                    let _e1940 = phi_14618_;
                                    let _e1942 = phi_14619_;
                                    if _e1942 {
                                        phi_14625_ = type_69(type_34(_e337.member, _e337.member_1, vec4<f32>(_e1938.x, _e1938.y, _e1938.z, f32(_e362)), vec4<f32>(_e337.member_3.x, _e337.member_3.y, _e337.member_3.z, f32(_e390)), vec4<u32>(_e1936, (_e337.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14625_ = _e1940;
                                    }
                                    let _e1955 = phi_14625_;
                                    phi_34217_ = _e1936;
                                    phi_14626_ = _e1955;
                                } else {
                                    phi_34217_ = _e1807;
                                    phi_14626_ = type_69();
                                }
                                let _e1957 = phi_34217_;
                                let _e1959 = phi_14626_;
                                phi_34811_ = _e1793;
                                phi_34619_ = _e1795;
                                phi_34540_ = _e1797;
                                phi_34463_ = _e1799;
                                phi_34384_ = _e1801;
                                phi_34302_ = _e1803;
                                phi_34225_ = _e1805;
                                phi_34216_ = _e1957;
                                phi_14628_ = _e1959;
                                phi_14629_ = select(true, false, _e1816);
                            }
                            let _e1967 = phi_34811_;
                            let _e1969 = phi_34619_;
                            let _e1971 = phi_34540_;
                            let _e1973 = phi_34463_;
                            let _e1975 = phi_34384_;
                            let _e1977 = phi_34302_;
                            let _e1979 = phi_34225_;
                            let _e1981 = phi_34216_;
                            let _e1983 = phi_14628_;
                            let _e1985 = phi_14629_;
                            if _e1985 {
                                let _e1987 = ((_e337.member_4.y | 4u) & 4294967261u);
                                let _e1990 = global_1.member.member_1[3u];
                                phi_36140_ = _e826;
                                phi_34215_ = _e1981;
                                phi_14640_ = _e366;
                                phi_14643_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14645_ = 0u;
                                phi_14647_ = _e390;
                                loop {
                                    let _e1992 = phi_36140_;
                                    let _e1994 = phi_34215_;
                                    let _e1996 = phi_14640_;
                                    let _e1998 = phi_14643_;
                                    let _e2000 = phi_14645_;
                                    let _e2002 = phi_14647_;
                                    local_83 = _e2002;
                                    local_84 = _e1996;
                                    local_85 = _e1996;
                                    local_86 = _e1996;
                                    local_87 = _e1998;
                                    local_88 = _e2002;
                                    local_89 = _e1996;
                                    local_90 = _e1996;
                                    local_91 = _e1996;
                                    local_92 = _e1998;
                                    local_93 = _e2002;
                                    local_94 = _e1996;
                                    local_95 = _e1996;
                                    local_96 = _e1996;
                                    local_97 = _e1998;
                                    local_98 = _e2002;
                                    local_99 = _e1996;
                                    local_100 = _e1996;
                                    local_101 = _e1996;
                                    local_102 = _e1998;
                                    let _e2004 = (_e2000 < (_e1990 + 3u));
                                    if _e2004 {
                                        if (_e2000 < _e1990) {
                                            let _e4001 = global_5.member[_e2000];
                                            let _e4006 = (_e4001.member.x - _e1111);
                                            let _e4007 = (_e4001.member.y - _e1112);
                                            let _e4008 = (_e4001.member.z - _e1113);
                                            let _e4013 = (((_e4006 * _e4006) + (_e4007 * _e4007)) + (_e4008 * _e4008));
                                            let _e4014 = sqrt(_e4013);
                                            let _e4015 = (_e4006 / _e4014);
                                            let _e4016 = (_e4007 / _e4014);
                                            let _e4017 = (_e4008 / _e4014);
                                            if (_e4001.member.w < 0.5f) {
                                                phi_14744_ = 1f;
                                            } else {
                                                let _e4041 = (((_e4001.member_2.x * -(_e4015)) + (_e4001.member_2.y * -(_e4016))) + (_e4001.member_2.z * -(_e4017)));
                                                if (_e4001.member_3.x == _e4001.member_2.w) {
                                                    phi_14743_ = select(1f, 0f, (_e4041 < _e4001.member_3.x));
                                                } else {
                                                    let _e4045 = ((_e4041 - _e4001.member_3.x) / (_e4001.member_2.w - _e4001.member_3.x));
                                                    let _e4047 = select(_e4045, 0f, (_e4045 < 0f));
                                                    let _e4049 = select(_e4047, 1f, (_e4047 > 1f));
                                                    phi_14743_ = ((_e4049 * _e4049) * (3f - (2f * _e4049)));
                                                }
                                                let _e4057 = phi_14743_;
                                                phi_14744_ = _e4057;
                                            }
                                            let _e4059 = phi_14744_;
                                            phi_36557_ = _e1994;
                                            phi_18157_ = 0f;
                                            phi_18158_ = vec3<f32>(_e4015, _e4016, _e4017);
                                            phi_18159_ = vec3<f32>(((_e4001.member_1.x * _e4059) / _e4013), ((_e4001.member_1.y * _e4059) / _e4013), ((_e4001.member_1.z * _e4059) / _e4013));
                                            phi_18160_ = _e4014;
                                        } else {
                                            if (_e2000 == _e1990) {
                                                if ((_e384 & 4u) == 0u) {
                                                    phi_36601_ = _e1994;
                                                    phi_16786_ = 0f;
                                                    phi_16787_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16788_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e2753 = global_1.member.member_5;
                                                    let _e2761 = global_1.member.member_6[3u];
                                                    let _e2763 = sin((_e2761 * 0.5f));
                                                    let _e2765 = ((12.566371f * _e2763) * _e2763);
                                                    let _e2770 = (((_e2753.x * _e1086.x) + (_e2753.y * _e1086.y)) + (_e2753.z * _e1086.z));
                                                    if (_e2770 != _e2770) {
                                                        phi_27960_ = true;
                                                    } else {
                                                        phi_27960_ = (0f >= _e2770);
                                                    }
                                                    let _e2774 = phi_27960_;
                                                    let _e2776 = (select(_e2770, 0f, _e2774) * 0.31830987f);
                                                    if (_e1979 == 0u) {
                                                        if (_e2770 <= 0f) {
                                                            phi_28757_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_28757_ = (_e1973 * (_e2770 * 0.31830987f));
                                                        }
                                                        let _e3252 = phi_28757_;
                                                        phi_15154_ = type_59(_e3252, _e2776);
                                                        phi_15155_ = type_59();
                                                        phi_15156_ = true;
                                                    } else {
                                                        let _e2782 = (((_e1060 * _e1134) + (_e1061 * _e1135)) + (_e1062 * _e1137));
                                                        let _e2787 = (((_e1060 * _e1130) + (_e1061 * _e1140)) + (_e1062 * _e1141));
                                                        let _e2792 = (((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z));
                                                        let _e2797 = (((_e2753.x * _e1134) + (_e2753.y * _e1135)) + (_e2753.z * _e1137));
                                                        let _e2802 = (((_e2753.x * _e1130) + (_e2753.y * _e1140)) + (_e2753.z * _e1141));
                                                        if (_e2792 <= 0f) {
                                                            phi_15118_ = type_59();
                                                            phi_15119_ = true;
                                                        } else {
                                                            let _e2804 = (_e2770 <= 0f);
                                                            if _e2804 {
                                                                phi_15117_ = type_59();
                                                            } else {
                                                                let _e2805 = (_e2782 + _e2797);
                                                                let _e2806 = (_e2787 + _e2802);
                                                                let _e2807 = (_e2792 + _e2770);
                                                                let _e2814 = (1f / sqrt((((_e2805 * _e2805) + (_e2806 * _e2806)) + (_e2807 * _e2807))));
                                                                let _e2817 = (_e2807 * _e2814);
                                                                if (_e2817 <= 0f) {
                                                                    phi_28070_ = 0f;
                                                                } else {
                                                                    let _e2819 = (_e1977 * _e1977);
                                                                    let _e2823 = ((((_e2819 - 1f) * _e2817) * _e2817) + 1f);
                                                                    phi_28070_ = (_e2819 / ((3.1415927f * _e2823) * _e2823));
                                                                }
                                                                let _e2828 = phi_28070_;
                                                                let _e2829 = (_e2792 * _e2792);
                                                                if (_e2829 >= 1f) {
                                                                    phi_28098_ = 0f;
                                                                } else {
                                                                    phi_28098_ = ((sqrt((1f + (((_e1977 * _e1977) * ((_e2782 * _e2782) + (_e2787 * _e2787))) / _e2829))) - 1f) * 0.5f);
                                                                }
                                                                let _e2842 = phi_28098_;
                                                                let _e2843 = (1f + _e2842);
                                                                let _e2844 = (_e2770 * _e2770);
                                                                if (_e2844 >= 1f) {
                                                                    phi_28126_ = 0f;
                                                                } else {
                                                                    phi_28126_ = ((sqrt((1f + (((_e1977 * _e1977) * ((_e2797 * _e2797) + (_e2802 * _e2802))) / _e2844))) - 1f) * 0.5f);
                                                                }
                                                                let _e2857 = phi_28126_;
                                                                let _e2864 = (((_e2782 * (_e2805 * _e2814)) + (_e2787 * (_e2806 * _e2814))) + (_e2792 * _e2817));
                                                                if (_e1975 == 0u) {
                                                                    let _e3159 = (1f - _e2864);
                                                                    if (_e3159 != _e3159) {
                                                                        phi_28629_ = true;
                                                                    } else {
                                                                        phi_28629_ = (0f >= _e3159);
                                                                    }
                                                                    let _e3163 = phi_28629_;
                                                                    let _e3165 = pow(select(_e3159, 0f, _e3163), 5f);
                                                                    let _e3166 = (1f - _e3165);
                                                                    phi_15105_ = vec3<f32>(((_e1973.x * _e3166) + _e3165), ((_e1973.y * _e3166) + _e3165), ((_e1973.z * _e3166) + _e3165));
                                                                } else {
                                                                    if (_e1975 == 1u) {
                                                                        let _e3132 = (1f / _e1969);
                                                                        let _e3136 = ((_e3132 * _e3132) * (1f - (_e2864 * _e2864)));
                                                                        if (_e3136 >= 1f) {
                                                                            phi_28590_ = 1f;
                                                                        } else {
                                                                            let _e3139 = sqrt((1f - _e3136));
                                                                            if (_e2864 != _e2864) {
                                                                                phi_28601_ = true;
                                                                            } else {
                                                                                phi_28601_ = (_e3139 <= _e2864);
                                                                            }
                                                                            let _e3143 = phi_28601_;
                                                                            let _e3148 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                                            phi_28590_ = (_e3148 + ((1f - _e3148) * pow((1f - select(_e2864, _e3139, _e3143)), 5f)));
                                                                        }
                                                                        let _e3155 = phi_28590_;
                                                                        phi_15104_ = vec3(_e3155);
                                                                    } else {
                                                                        let _e2868 = select(_e2864, 0f, (_e2864 < 0f));
                                                                        let _e2870 = select(_e2868, 1f, (_e2868 > 1f));
                                                                        let _e2872 = (1f - (_e2870 * _e2870));
                                                                        let _e2885 = ((_e1973.x * _e1973.x) - (_e1971.x * _e1971.x));
                                                                        let _e2886 = ((_e1973.y * _e1973.y) - (_e1971.y * _e1971.y));
                                                                        let _e2887 = ((_e1973.z * _e1973.z) - (_e1971.z * _e1971.z));
                                                                        let _e2889 = (_e1973.x * (_e1971.x + _e1971.x));
                                                                        let _e2891 = (_e1973.y * (_e1971.y + _e1971.y));
                                                                        let _e2893 = (_e1973.z * (_e1971.z + _e1971.z));
                                                                        let _e2900 = ((_e2885 * _e2885) + (_e2889 * _e2889));
                                                                        let _e2901 = ((_e2886 * _e2886) + (_e2891 * _e2891));
                                                                        let _e2902 = ((_e2887 * _e2887) + (_e2893 * _e2893));
                                                                        let _e2907 = ((_e2872 * _e2885) / _e2900);
                                                                        let _e2908 = ((_e2872 * _e2886) / _e2901);
                                                                        let _e2909 = ((_e2872 * _e2887) / _e2902);
                                                                        let _e2910 = (1f - _e2907);
                                                                        let _e2911 = (1f - _e2908);
                                                                        let _e2912 = (1f - _e2909);
                                                                        let _e2914 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2907, _e2908, _e2909));
                                                                        let _e2915 = (_e2872 * _e2889);
                                                                        let _e2916 = (_e2872 * _e2891);
                                                                        let _e2917 = (_e2872 * _e2893);
                                                                        let _e2922 = (vec3<f32>(_e2915, _e2916, _e2917) / vec3<f32>(_e2900, _e2901, _e2902));
                                                                        let _e2935 = ((_e2914.x * _e2914.x) + (_e2922.x * _e2922.x));
                                                                        let _e2936 = ((_e2914.y * _e2914.y) + (_e2922.y * _e2922.y));
                                                                        let _e2937 = ((_e2914.z * _e2914.z) + (_e2922.z * _e2922.z));
                                                                        if (_e2935 != _e2935) {
                                                                            phi_28260_ = true;
                                                                        } else {
                                                                            phi_28260_ = (0f >= _e2935);
                                                                        }
                                                                        let _e2941 = phi_28260_;
                                                                        let _e2943 = sqrt(select(_e2935, 0f, _e2941));
                                                                        if (_e2936 != _e2936) {
                                                                            phi_28275_ = true;
                                                                        } else {
                                                                            phi_28275_ = (0f >= _e2936);
                                                                        }
                                                                        let _e2947 = phi_28275_;
                                                                        let _e2949 = sqrt(select(_e2936, 0f, _e2947));
                                                                        if (_e2937 != _e2937) {
                                                                            phi_28290_ = true;
                                                                        } else {
                                                                            phi_28290_ = (0f >= _e2937);
                                                                        }
                                                                        let _e2953 = phi_28290_;
                                                                        let _e2955 = sqrt(select(_e2937, 0f, _e2953));
                                                                        let _e2959 = ((_e2943 + _e2910) * 0.5f);
                                                                        let _e2960 = ((_e2949 + _e2911) * 0.5f);
                                                                        let _e2961 = ((_e2955 + _e2912) * 0.5f);
                                                                        if (_e2959 != _e2959) {
                                                                            phi_28317_ = true;
                                                                        } else {
                                                                            phi_28317_ = (0f >= _e2959);
                                                                        }
                                                                        let _e2965 = phi_28317_;
                                                                        let _e2967 = sqrt(select(_e2959, 0f, _e2965));
                                                                        if (_e2960 != _e2960) {
                                                                            phi_28332_ = true;
                                                                        } else {
                                                                            phi_28332_ = (0f >= _e2960);
                                                                        }
                                                                        let _e2971 = phi_28332_;
                                                                        let _e2973 = sqrt(select(_e2960, 0f, _e2971));
                                                                        if (_e2961 != _e2961) {
                                                                            phi_28347_ = true;
                                                                        } else {
                                                                            phi_28347_ = (0f >= _e2961);
                                                                        }
                                                                        let _e2977 = phi_28347_;
                                                                        let _e2979 = sqrt(select(_e2961, 0f, _e2977));
                                                                        let _e2980 = vec3<f32>(_e2967, _e2973, _e2979);
                                                                        let _e2984 = ((_e2943 - _e2910) * 0.5f);
                                                                        let _e2985 = ((_e2949 - _e2911) * 0.5f);
                                                                        let _e2986 = ((_e2955 - _e2912) * 0.5f);
                                                                        if (_e2984 != _e2984) {
                                                                            phi_28374_ = true;
                                                                        } else {
                                                                            phi_28374_ = (0f >= _e2984);
                                                                        }
                                                                        let _e2990 = phi_28374_;
                                                                        let _e2992 = sqrt(select(_e2984, 0f, _e2990));
                                                                        if (_e2985 != _e2985) {
                                                                            phi_28389_ = true;
                                                                        } else {
                                                                            phi_28389_ = (0f >= _e2985);
                                                                        }
                                                                        let _e2996 = phi_28389_;
                                                                        let _e2998 = sqrt(select(_e2985, 0f, _e2996));
                                                                        if (_e2986 != _e2986) {
                                                                            phi_28404_ = true;
                                                                        } else {
                                                                            phi_28404_ = (0f >= _e2986);
                                                                        }
                                                                        let _e3002 = phi_28404_;
                                                                        let _e3004 = sqrt(select(_e2986, 0f, _e3002));
                                                                        if ((_e2915 / _e2900) < 0f) {
                                                                            phi_14981_ = -(_e2992);
                                                                        } else {
                                                                            phi_14981_ = _e2992;
                                                                        }
                                                                        let _e3008 = phi_14981_;
                                                                        if ((_e2916 / _e2901) < 0f) {
                                                                            phi_14989_ = -(_e2998);
                                                                        } else {
                                                                            phi_14989_ = _e2998;
                                                                        }
                                                                        let _e3012 = phi_14989_;
                                                                        if ((_e2917 / _e2902) < 0f) {
                                                                            phi_14997_ = -(_e3004);
                                                                        } else {
                                                                            phi_14997_ = _e3004;
                                                                        }
                                                                        let _e3016 = phi_14997_;
                                                                        let _e3020 = (_e1971.x * _e2870);
                                                                        let _e3021 = (_e1971.y * _e2870);
                                                                        let _e3022 = (_e1971.z * _e2870);
                                                                        let _e3032 = vec3<f32>(((_e1973.x * _e2967) - (_e1971.x * _e3008)), ((_e1973.y * _e2973) - (_e1971.y * _e3012)), ((_e1973.z * _e2979) - (_e1971.z * _e3016)));
                                                                        let _e3039 = ((_e1973.x * _e3008) + (_e1971.x * _e2967));
                                                                        let _e3040 = ((_e1973.y * _e3012) + (_e1971.y * _e2973));
                                                                        let _e3041 = ((_e1973.z * _e3016) + (_e1971.z * _e2979));
                                                                        let _e3043 = vec3<f32>((_e1973.x * _e2870), (_e1973.y * _e2870), (_e1973.z * _e2870));
                                                                        let _e3044 = (_e3043 - _e2980);
                                                                        let _e3045 = (_e3020 - _e3008);
                                                                        let _e3046 = (_e3021 - _e3012);
                                                                        let _e3047 = (_e3022 - _e3016);
                                                                        let _e3060 = (_e3043 + _e2980);
                                                                        let _e3061 = (_e3020 + _e3008);
                                                                        let _e3062 = (_e3021 + _e3012);
                                                                        let _e3063 = (_e3022 + _e3016);
                                                                        let _e3073 = ((_e3060.x * _e3060.x) + (_e3061 * _e3061));
                                                                        let _e3074 = ((_e3060.y * _e3060.y) + (_e3062 * _e3062));
                                                                        let _e3075 = ((_e3060.z * _e3060.z) + (_e3063 * _e3063));
                                                                        let _e3085 = vec3(_e2870);
                                                                        let _e3086 = (_e3085 - _e3032);
                                                                        let _e3087 = -(vec3<f32>(_e3039, _e3040, _e3041));
                                                                        let _e3103 = (_e3085 + _e3032);
                                                                        let _e3113 = ((_e3103.x * _e3103.x) + (_e3039 * _e3039));
                                                                        let _e3114 = ((_e3103.y * _e3103.y) + (_e3040 * _e3040));
                                                                        let _e3115 = ((_e3103.z * _e3103.z) + (_e3041 * _e3041));
                                                                        phi_15104_ = vec3<f32>((((((_e3044.x * _e3044.x) + (_e3045 * _e3045)) / select(0.000000000000000000000000000001f, _e3073, (_e3073 > 0.000000000000000000000000000001f))) + (((_e3086.x * _e3086.x) + (_e3087.x * _e3087.x)) / select(0.000000000000000000000000000001f, _e3113, (_e3113 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3044.y * _e3044.y) + (_e3046 * _e3046)) / select(0.000000000000000000000000000001f, _e3074, (_e3074 > 0.000000000000000000000000000001f))) + (((_e3086.y * _e3086.y) + (_e3087.y * _e3087.y)) / select(0.000000000000000000000000000001f, _e3114, (_e3114 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3044.z * _e3044.z) + (_e3047 * _e3047)) / select(0.000000000000000000000000000001f, _e3075, (_e3075 > 0.000000000000000000000000000001f))) + (((_e3086.z * _e3086.z) + (_e3087.z * _e3087.z)) / select(0.000000000000000000000000000001f, _e3115, (_e3115 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3158 = phi_15104_;
                                                                    phi_15105_ = _e3158;
                                                                }
                                                                let _e3178 = phi_15105_;
                                                                let _e3180 = (4f * _e2792);
                                                                phi_15117_ = type_59((_e3178 * (((1f / (_e2843 + _e2857)) * _e2828) / _e3180)), ((_e2828 / _e2843) / _e3180));
                                                            }
                                                            let _e3187 = phi_15117_;
                                                            phi_15118_ = _e3187;
                                                            phi_15119_ = _e2804;
                                                        }
                                                        let _e3189 = phi_15118_;
                                                        let _e3191 = phi_15119_;
                                                        if _e3191 {
                                                            phi_15124_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15124_ = _e3189;
                                                        }
                                                        let _e3193 = phi_15124_;
                                                        let _e3196 = (_e1979 == 1u);
                                                        if _e3196 {
                                                            phi_15152_ = _e3193;
                                                            phi_15153_ = type_59();
                                                        } else {
                                                            if (_e2792 != _e2792) {
                                                                phi_28666_ = true;
                                                            } else {
                                                                phi_28666_ = (0f >= _e2792);
                                                            }
                                                            let _e3200 = phi_28666_;
                                                            let _e3201 = select(_e2792, 0f, _e3200);
                                                            let _e3202 = (1f / _e1969);
                                                            let _e3206 = ((_e3202 * _e3202) * (1f - (_e3201 * _e3201)));
                                                            if (_e3206 >= 1f) {
                                                                phi_28685_ = 1f;
                                                            } else {
                                                                let _e3209 = sqrt((1f - _e3206));
                                                                if (_e3201 != _e3201) {
                                                                    phi_28696_ = true;
                                                                } else {
                                                                    phi_28696_ = (_e3209 <= _e3201);
                                                                }
                                                                let _e3213 = phi_28696_;
                                                                let _e3218 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                                phi_28685_ = (_e3218 + ((1f - _e3218) * pow((1f - select(_e3201, _e3209, _e3213)), 5f)));
                                                            }
                                                            let _e3225 = phi_28685_;
                                                            if (_e2770 <= 0f) {
                                                                phi_28735_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_28735_ = (_e1967 * (_e2770 * 0.31830987f));
                                                            }
                                                            let _e3230 = phi_28735_;
                                                            let _e3231 = (1f - _e3225);
                                                            phi_15152_ = type_59();
                                                            phi_15153_ = type_59((_e3193.member + vec3<f32>((_e3230.x * _e3231), (_e3230.y * _e3231), (_e3230.z * _e3231))), ((_e3225 * _e3193.member_1) + (_e3231 * _e2776)));
                                                        }
                                                        let _e3245 = phi_15152_;
                                                        let _e3247 = phi_15153_;
                                                        phi_15154_ = _e3245;
                                                        phi_15155_ = _e3247;
                                                        phi_15156_ = _e3196;
                                                    }
                                                    let _e3255 = phi_15154_;
                                                    let _e3257 = phi_15155_;
                                                    let _e3259 = phi_15156_;
                                                    if _e3259 {
                                                        phi_15160_ = _e3255;
                                                    } else {
                                                        phi_15160_ = _e3257;
                                                    }
                                                    let _e3261 = phi_15160_;
                                                    if ((_e3261.member_1 * _e2765) < 0.001f) {
                                                        let _e3688 = global_1.member.member_6;
                                                        let _e3694 = global_1.member.member_8[2u];
                                                        if (_e3694 == 4294967295u) {
                                                            phi_15899_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e3697 = global_6.member[_e3694];
                                                            let _e3700 = global_6.member[(_e3694 + 1u)];
                                                            let _e3703 = global_6.member[(_e3694 + 2u)];
                                                            let _e3706 = global_6.member[(_e3694 + 3u)];
                                                            let _e3709 = global_6.member[(_e3694 + 4u)];
                                                            let _e3712 = global_6.member[(_e3694 + 5u)];
                                                            let _e3715 = global_6.member[(_e3694 + 6u)];
                                                            let _e3718 = global_6.member[(_e3694 + 7u)];
                                                            let _e3721 = global_6.member[(_e3694 + 8u)];
                                                            let _e3724 = global_6.member[(_e3694 + 9u)];
                                                            let _e3727 = global_6.member[(_e3694 + 13u)];
                                                            let _e3730 = global_6.member[(_e3694 + 14u)];
                                                            let _e3733 = global_6.member[(_e3694 + 15u)];
                                                            let _e3736 = global_6.member[(_e3694 + 16u)];
                                                            let _e3739 = global_6.member[(_e3694 + 18u)];
                                                            let _e3742 = global_6.member[(_e3694 + 19u)];
                                                            let _e3745 = global_6.member[(_e3694 + 20u)];
                                                            let _e3748 = global_6.member[(_e3694 + 29u)];
                                                            let _e3749 = bitcast<u32>(_e3748);
                                                            let _e3762 = (((((_e1111 * _e1111) + (_e1112 * _e1112)) + (_e1113 * _e1113)) - (2f * (((_e1111 * _e3697) + (_e1112 * _e3700)) + (_e1113 * _e3703)))) + _e3715);
                                                            if (_e3762 > 0f) {
                                                                let _e3771 = ((((_e1111 - _e3697) * _e3718) + ((_e1112 - _e3700) * _e3721)) + ((_e1113 - _e3703) * _e3724));
                                                                let _e3773 = ((_e3771 * _e3771) - _e3762);
                                                                if (_e3771 >= 0f) {
                                                                    phi_15599_ = vec3<f32>();
                                                                    phi_15600_ = true;
                                                                } else {
                                                                    let _e3775 = (_e3773 <= 0f);
                                                                    if _e3775 {
                                                                        phi_15598_ = vec3<f32>();
                                                                    } else {
                                                                        let _e3778 = (-(_e3771) - sqrt(_e3773));
                                                                        phi_15598_ = vec3<f32>((_e1111 + (_e3718 * _e3778)), (_e1112 + (_e3721 * _e3778)), (_e1113 + (_e3724 * _e3778)));
                                                                    }
                                                                    let _e3787 = phi_15598_;
                                                                    phi_15599_ = _e3787;
                                                                    phi_15600_ = _e3775;
                                                                }
                                                                let _e3789 = phi_15599_;
                                                                let _e3791 = phi_15600_;
                                                                phi_15605_ = _e3789;
                                                                phi_15606_ = select(true, false, _e3791);
                                                            } else {
                                                                phi_15605_ = _e1114;
                                                                phi_15606_ = true;
                                                            }
                                                            let _e3794 = phi_15605_;
                                                            let _e3796 = phi_15606_;
                                                            if _e3796 {
                                                                let _e3798 = (_e3794.x - _e3697);
                                                                let _e3800 = (_e3794.y - _e3700);
                                                                let _e3802 = (_e3794.z - _e3703);
                                                                let _e3808 = sqrt((((_e3798 * _e3798) + (_e3800 * _e3800)) + (_e3802 * _e3802)));
                                                                let _e3809 = (1f / _e3808);
                                                                let _e3827 = ((((((_e3794.x * _e3794.x) + (_e3794.y * _e3794.y)) + (_e3794.z * _e3794.z)) - (2f * (((_e3794.x * _e3697) + (_e3794.y * _e3700)) + (_e3794.z * _e3703)))) + _e3712) / (_e3808 + _e3706));
                                                                if (_e3827 != _e3827) {
                                                                    phi_29191_ = true;
                                                                } else {
                                                                    phi_29191_ = (0f >= _e3827);
                                                                }
                                                                let _e3831 = phi_29191_;
                                                                let _e3835 = sqrt((select(_e3827, 0f, _e3831) / (_e3709 - _e3706)));
                                                                if (_e3835 != _e3835) {
                                                                    phi_29206_ = true;
                                                                } else {
                                                                    phi_29206_ = (1f <= _e3835);
                                                                }
                                                                let _e3839 = phi_29206_;
                                                                let _e3841 = (select(_e3835, 1f, _e3839) * 63f);
                                                                let _e3846 = ((((_e3798 * _e3809) * _e3718) + ((_e3800 * _e3809) * _e3721)) + ((_e3802 * _e3809) * _e3724));
                                                                let _e3848 = select(_e3846, -1f, (_e3846 < -1f));
                                                                let _e3852 = ((select(_e3848, 1f, (_e3848 > 1f)) + 1f) * 127.5f);
                                                                let _e3857 = select(select(u32(_e3841), 0u, (_e3841 < 0f)), 4294967295u, (_e3841 > 4294967000f));
                                                                let _e3859 = select(_e3857, 62u, (62u < _e3857));
                                                                let _e3864 = select(select(u32(_e3852), 0u, (_e3852 < 0f)), 4294967295u, (_e3852 > 4294967000f));
                                                                let _e3866 = select(_e3864, 254u, (254u < _e3864));
                                                                let _e3868 = (_e3841 - f32(_e3859));
                                                                let _e3870 = (_e3852 - f32(_e3866));
                                                                let _e3874 = (_e3749 + (3u * ((_e3859 * 256u) + _e3866)));
                                                                let _e3879 = (_e3749 + (3u * (((_e3859 + 1u) * 256u) + _e3866)));
                                                                let _e3881 = global_6.member[_e3874];
                                                                let _e3884 = global_6.member[(_e3874 + 1u)];
                                                                let _e3887 = global_6.member[(_e3874 + 2u)];
                                                                let _e3888 = (1f - _e3870);
                                                                let _e3894 = global_6.member[(_e3874 + 3u)];
                                                                let _e3897 = global_6.member[(_e3874 + 4u)];
                                                                let _e3900 = global_6.member[(_e3874 + 5u)];
                                                                let _e3907 = (1f - _e3868);
                                                                let _e3912 = global_6.member[_e3879];
                                                                let _e3915 = global_6.member[(_e3879 + 1u)];
                                                                let _e3918 = global_6.member[(_e3879 + 2u)];
                                                                let _e3924 = global_6.member[(_e3879 + 3u)];
                                                                let _e3927 = global_6.member[(_e3879 + 4u)];
                                                                let _e3930 = global_6.member[(_e3879 + 5u)];
                                                                phi_15897_ = vec3<f32>(((((_e3881 * _e3888) + (_e3894 * _e3870)) * _e3907) + (((_e3912 * _e3888) + (_e3924 * _e3870)) * _e3868)), ((((_e3884 * _e3888) + (_e3897 * _e3870)) * _e3907) + (((_e3915 * _e3888) + (_e3927 * _e3870)) * _e3868)), ((((_e3887 * _e3888) + (_e3900 * _e3870)) * _e3907) + (((_e3918 * _e3888) + (_e3930 * _e3870)) * _e3868)));
                                                            } else {
                                                                phi_15897_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3945 = phi_15897_;
                                                            let _e3952 = ((_e3736 * 1.1f) * _e3945.y);
                                                            phi_15899_ = vec3<f32>(exp(-((((_e3727 * _e3945.x) + _e3952) + (_e3739 * _e3945.z)))), exp(-((((_e3730 * _e3945.x) + _e3952) + (_e3742 * _e3945.z)))), exp(-((((_e3733 * _e3945.x) + _e3952) + (_e3745 * _e3945.z)))));
                                                        }
                                                        let _e3971 = phi_15899_;
                                                        phi_36602_ = _e1994;
                                                        phi_16783_ = 0f;
                                                        phi_16784_ = vec3<f32>(_e2753.x, _e2753.y, _e2753.z);
                                                        phi_16785_ = (vec3<f32>(_e3688.x, _e3688.y, _e3688.z) * _e3971);
                                                    } else {
                                                        let _e3266 = ((_e1994 * 747796405u) + 2891336453u);
                                                        let _e3274 = (((_e3266 >> bitcast<u32>((((_e3266 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3266) * 277803737u);
                                                        let _e3277 = ((_e3274 >> bitcast<u32>(22i)) ^ _e3274);
                                                        let _e3283 = (((f32((_e3277 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2765) * 0.15915494f);
                                                        let _e3284 = (1f - _e3283);
                                                        let _e3286 = (_e3283 * (2f - _e3283));
                                                        if (_e3286 != _e3286) {
                                                            phi_28789_ = true;
                                                        } else {
                                                            phi_28789_ = (0f >= _e3286);
                                                        }
                                                        let _e3290 = phi_28789_;
                                                        let _e3292 = sqrt(select(_e3286, 0f, _e3290));
                                                        let _e3294 = ((_e3277 * 747796405u) + 2891336453u);
                                                        let _e3302 = (((_e3294 >> bitcast<u32>((((_e3294 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3294) * 277803737u);
                                                        let _e3305 = ((_e3302 >> bitcast<u32>(22i)) ^ _e3302);
                                                        let _e3309 = (f32((_e3305 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3311 = select(-1f, 1f, (_e2753.z >= 0f));
                                                        let _e3313 = (-1f / (_e3311 + _e2753.z));
                                                        let _e3315 = ((_e2753.x * _e2753.y) * _e3313);
                                                        let _e3328 = (_e3292 * cos(_e3309));
                                                        let _e3333 = (_e3292 * sin(_e3309));
                                                        let _e3343 = ((((1f + (((_e3311 * _e2753.x) * _e2753.x) * _e3313)) * _e3328) + (_e3315 * _e3333)) + (_e2753.x * _e3284));
                                                        let _e3344 = ((((_e3311 * _e3315) * _e3328) + ((_e3311 + ((_e2753.y * _e2753.y) * _e3313)) * _e3333)) + (_e2753.y * _e3284));
                                                        let _e3345 = ((((-(_e3311) * _e2753.x) * _e3328) + (-(_e2753.y) * _e3333)) + (_e2753.z * _e3284));
                                                        let _e3347 = (_e3343 - _e2753.x);
                                                        let _e3348 = (_e3344 - _e2753.y);
                                                        let _e3349 = (_e3345 - _e2753.z);
                                                        let _e3356 = (sqrt((((_e3347 * _e3347) + (_e3348 * _e3348)) + (_e3349 * _e3349))) * 0.5f);
                                                        let _e3357 = (_e3356 != _e3356);
                                                        if _e3357 {
                                                            phi_28860_ = true;
                                                        } else {
                                                            phi_28860_ = (1f <= _e3356);
                                                        }
                                                        let _e3360 = phi_28860_;
                                                        if ((2f * asin(select(_e3356, 1f, _e3360))) >= _e2761) {
                                                            phi_15980_ = 0f;
                                                        } else {
                                                            phi_15980_ = (1f / _e2765);
                                                        }
                                                        let _e3367 = phi_15980_;
                                                        if (_e3367 > 0f) {
                                                            if _e3357 {
                                                                phi_28875_ = true;
                                                            } else {
                                                                phi_28875_ = (1f <= _e3356);
                                                            }
                                                            let _e3371 = phi_28875_;
                                                            let _e3374 = (2f * asin(select(_e3356, 1f, _e3371)));
                                                            if (_e3374 >= _e2761) {
                                                                phi_16781_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3378 = (1f - pow((_e3374 / _e2761), 2f));
                                                                if (_e3378 != _e3378) {
                                                                    phi_28890_ = true;
                                                                } else {
                                                                    phi_28890_ = (0f >= _e3378);
                                                                }
                                                                let _e3382 = phi_28890_;
                                                                let _e3385 = global_1.member.member_6;
                                                                let _e3391 = global_1.member.member_8[2u];
                                                                if (_e3391 == 4294967295u) {
                                                                    phi_16754_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e3394 = global_6.member[_e3391];
                                                                    let _e3397 = global_6.member[(_e3391 + 1u)];
                                                                    let _e3400 = global_6.member[(_e3391 + 2u)];
                                                                    let _e3403 = global_6.member[(_e3391 + 3u)];
                                                                    let _e3406 = global_6.member[(_e3391 + 4u)];
                                                                    let _e3409 = global_6.member[(_e3391 + 5u)];
                                                                    let _e3412 = global_6.member[(_e3391 + 6u)];
                                                                    let _e3415 = global_6.member[(_e3391 + 7u)];
                                                                    let _e3418 = global_6.member[(_e3391 + 8u)];
                                                                    let _e3421 = global_6.member[(_e3391 + 9u)];
                                                                    let _e3424 = global_6.member[(_e3391 + 13u)];
                                                                    let _e3427 = global_6.member[(_e3391 + 14u)];
                                                                    let _e3430 = global_6.member[(_e3391 + 15u)];
                                                                    let _e3433 = global_6.member[(_e3391 + 16u)];
                                                                    let _e3436 = global_6.member[(_e3391 + 18u)];
                                                                    let _e3439 = global_6.member[(_e3391 + 19u)];
                                                                    let _e3442 = global_6.member[(_e3391 + 20u)];
                                                                    let _e3445 = global_6.member[(_e3391 + 29u)];
                                                                    let _e3446 = bitcast<u32>(_e3445);
                                                                    let _e3459 = (((((_e1111 * _e1111) + (_e1112 * _e1112)) + (_e1113 * _e1113)) - (2f * (((_e1111 * _e3394) + (_e1112 * _e3397)) + (_e1113 * _e3400)))) + _e3412);
                                                                    if (_e3459 > 0f) {
                                                                        let _e3468 = ((((_e1111 - _e3394) * _e3415) + ((_e1112 - _e3397) * _e3418)) + ((_e1113 - _e3400) * _e3421));
                                                                        let _e3470 = ((_e3468 * _e3468) - _e3459);
                                                                        if (_e3468 >= 0f) {
                                                                            phi_16454_ = vec3<f32>();
                                                                            phi_16455_ = true;
                                                                        } else {
                                                                            let _e3472 = (_e3470 <= 0f);
                                                                            if _e3472 {
                                                                                phi_16453_ = vec3<f32>();
                                                                            } else {
                                                                                let _e3475 = (-(_e3468) - sqrt(_e3470));
                                                                                phi_16453_ = vec3<f32>((_e1111 + (_e3415 * _e3475)), (_e1112 + (_e3418 * _e3475)), (_e1113 + (_e3421 * _e3475)));
                                                                            }
                                                                            let _e3484 = phi_16453_;
                                                                            phi_16454_ = _e3484;
                                                                            phi_16455_ = _e3472;
                                                                        }
                                                                        let _e3486 = phi_16454_;
                                                                        let _e3488 = phi_16455_;
                                                                        phi_16460_ = _e3486;
                                                                        phi_16461_ = select(true, false, _e3488);
                                                                    } else {
                                                                        phi_16460_ = _e1114;
                                                                        phi_16461_ = true;
                                                                    }
                                                                    let _e3491 = phi_16460_;
                                                                    let _e3493 = phi_16461_;
                                                                    if _e3493 {
                                                                        let _e3495 = (_e3491.x - _e3394);
                                                                        let _e3497 = (_e3491.y - _e3397);
                                                                        let _e3499 = (_e3491.z - _e3400);
                                                                        let _e3505 = sqrt((((_e3495 * _e3495) + (_e3497 * _e3497)) + (_e3499 * _e3499)));
                                                                        let _e3506 = (1f / _e3505);
                                                                        let _e3524 = ((((((_e3491.x * _e3491.x) + (_e3491.y * _e3491.y)) + (_e3491.z * _e3491.z)) - (2f * (((_e3491.x * _e3394) + (_e3491.y * _e3397)) + (_e3491.z * _e3400)))) + _e3409) / (_e3505 + _e3403));
                                                                        if (_e3524 != _e3524) {
                                                                            phi_29014_ = true;
                                                                        } else {
                                                                            phi_29014_ = (0f >= _e3524);
                                                                        }
                                                                        let _e3528 = phi_29014_;
                                                                        let _e3532 = sqrt((select(_e3524, 0f, _e3528) / (_e3406 - _e3403)));
                                                                        if (_e3532 != _e3532) {
                                                                            phi_29029_ = true;
                                                                        } else {
                                                                            phi_29029_ = (1f <= _e3532);
                                                                        }
                                                                        let _e3536 = phi_29029_;
                                                                        let _e3538 = (select(_e3532, 1f, _e3536) * 63f);
                                                                        let _e3543 = ((((_e3495 * _e3506) * _e3415) + ((_e3497 * _e3506) * _e3418)) + ((_e3499 * _e3506) * _e3421));
                                                                        let _e3545 = select(_e3543, -1f, (_e3543 < -1f));
                                                                        let _e3549 = ((select(_e3545, 1f, (_e3545 > 1f)) + 1f) * 127.5f);
                                                                        let _e3554 = select(select(u32(_e3538), 0u, (_e3538 < 0f)), 4294967295u, (_e3538 > 4294967000f));
                                                                        let _e3556 = select(_e3554, 62u, (62u < _e3554));
                                                                        let _e3561 = select(select(u32(_e3549), 0u, (_e3549 < 0f)), 4294967295u, (_e3549 > 4294967000f));
                                                                        let _e3563 = select(_e3561, 254u, (254u < _e3561));
                                                                        let _e3565 = (_e3538 - f32(_e3556));
                                                                        let _e3567 = (_e3549 - f32(_e3563));
                                                                        let _e3571 = (_e3446 + (3u * ((_e3556 * 256u) + _e3563)));
                                                                        let _e3576 = (_e3446 + (3u * (((_e3556 + 1u) * 256u) + _e3563)));
                                                                        let _e3578 = global_6.member[_e3571];
                                                                        let _e3581 = global_6.member[(_e3571 + 1u)];
                                                                        let _e3584 = global_6.member[(_e3571 + 2u)];
                                                                        let _e3585 = (1f - _e3567);
                                                                        let _e3591 = global_6.member[(_e3571 + 3u)];
                                                                        let _e3594 = global_6.member[(_e3571 + 4u)];
                                                                        let _e3597 = global_6.member[(_e3571 + 5u)];
                                                                        let _e3604 = (1f - _e3565);
                                                                        let _e3609 = global_6.member[_e3576];
                                                                        let _e3612 = global_6.member[(_e3576 + 1u)];
                                                                        let _e3615 = global_6.member[(_e3576 + 2u)];
                                                                        let _e3621 = global_6.member[(_e3576 + 3u)];
                                                                        let _e3624 = global_6.member[(_e3576 + 4u)];
                                                                        let _e3627 = global_6.member[(_e3576 + 5u)];
                                                                        phi_16752_ = vec3<f32>(((((_e3578 * _e3585) + (_e3591 * _e3567)) * _e3604) + (((_e3609 * _e3585) + (_e3621 * _e3567)) * _e3565)), ((((_e3581 * _e3585) + (_e3594 * _e3567)) * _e3604) + (((_e3612 * _e3585) + (_e3624 * _e3567)) * _e3565)), ((((_e3584 * _e3585) + (_e3597 * _e3567)) * _e3604) + (((_e3615 * _e3585) + (_e3627 * _e3567)) * _e3565)));
                                                                    } else {
                                                                        phi_16752_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e3642 = phi_16752_;
                                                                    let _e3649 = ((_e3433 * 1.1f) * _e3642.y);
                                                                    phi_16754_ = vec3<f32>(exp(-((((_e3424 * _e3642.x) + _e3649) + (_e3436 * _e3642.z)))), exp(-((((_e3427 * _e3642.x) + _e3649) + (_e3439 * _e3642.z)))), exp(-((((_e3430 * _e3642.x) + _e3649) + (_e3442 * _e3642.z)))));
                                                                }
                                                                let _e3668 = phi_16754_;
                                                                let _e3679 = (((1f - (0.6f * (1f - sqrt(select(_e3378, 0f, _e3382))))) * 1.25f) / _e2765);
                                                                phi_16781_ = vec3<f32>(((_e3385.x * _e3668.x) * _e3679), ((_e3385.y * _e3668.y) * _e3679), ((_e3385.z * _e3668.z) * _e3679));
                                                            }
                                                            let _e3685 = phi_16781_;
                                                            phi_16782_ = _e3685;
                                                        } else {
                                                            phi_16782_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3687 = phi_16782_;
                                                        phi_36602_ = _e3305;
                                                        phi_16783_ = _e3367;
                                                        phi_16784_ = vec3<f32>(_e3343, _e3344, _e3345);
                                                        phi_16785_ = _e3687;
                                                    }
                                                    let _e3975 = phi_36602_;
                                                    let _e3977 = phi_16783_;
                                                    let _e3979 = phi_16784_;
                                                    let _e3981 = phi_16785_;
                                                    phi_36601_ = _e3975;
                                                    phi_16786_ = _e3977;
                                                    phi_16787_ = _e3979;
                                                    phi_16788_ = _e3981;
                                                }
                                                let _e3983 = phi_36601_;
                                                let _e3985 = phi_16786_;
                                                let _e3987 = phi_16787_;
                                                let _e3989 = phi_16788_;
                                                phi_36558_ = _e3983;
                                                phi_18153_ = _e3985;
                                                phi_18154_ = _e3987;
                                                phi_18155_ = _e3989;
                                                phi_18156_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e2000 == (_e1990 + 1u)) {
                                                    if ((_e384 & 1u) == 0u) {
                                                        phi_36567_ = _e1994;
                                                        phi_17787_ = 0f;
                                                        phi_17788_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17789_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17790_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e387 > 0u) {
                                                            let _e2231 = global_1.member.member_2[1u];
                                                            let _e2233 = ((_e1994 * 747796405u) + 2891336453u);
                                                            let _e2241 = (((_e2233 >> bitcast<u32>((((_e2233 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2233) * 277803737u);
                                                            let _e2244 = ((_e2241 >> bitcast<u32>(22i)) ^ _e2241);
                                                            phi_16809_ = 0u;
                                                            phi_16812_ = _e387;
                                                            loop {
                                                                let _e2250 = phi_16809_;
                                                                let _e2252 = phi_16812_;
                                                                local_78 = _e2250;
                                                                local_79 = _e2250;
                                                                let _e2254 = ((_e2250 + 1u) < _e2252);
                                                                if _e2254 {
                                                                    let _e2256 = ((_e2250 + _e2252) / 2u);
                                                                    let _e2259 = global_6.member[(_e2231 + _e2256)];
                                                                    let _e2260 = (_e2259 <= (f32((_e2244 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_16810_ = select(_e2250, _e2256, _e2260);
                                                                    phi_16813_ = select(_e2256, _e2252, _e2260);
                                                                } else {
                                                                    phi_16810_ = u32();
                                                                    phi_16813_ = u32();
                                                                }
                                                                let _e2264 = phi_16810_;
                                                                let _e2266 = phi_16813_;
                                                                phi_16809_ = _e2264;
                                                                phi_16812_ = _e2266;
                                                                if !(_e2254) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2269 = local_78;
                                                            let _e2270 = (_e2231 + _e2269);
                                                            let _e2273 = global_6.member[(_e2270 + 1u)];
                                                            let _e2275 = global_6.member[_e2270];
                                                            let _e2279 = global_1.member.member_2[0u];
                                                            let _e2281 = local_79;
                                                            let _e2284 = global_6.member[(_e2279 + _e2281)];
                                                            let _e2285 = bitcast<u32>(_e2284);
                                                            let _e2287 = ((_e2244 * 747796405u) + 2891336453u);
                                                            let _e2295 = (((_e2287 >> bitcast<u32>((((_e2287 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2287) * 277803737u);
                                                            let _e2298 = ((_e2295 >> bitcast<u32>(22i)) ^ _e2295);
                                                            let _e2301 = f32((_e2298 >> bitcast<u32>(8i)));
                                                            let _e2302 = (_e2301 * 0.000000059604645f);
                                                            let _e2304 = ((_e2298 * 747796405u) + 2891336453u);
                                                            let _e2312 = (((_e2304 >> bitcast<u32>((((_e2304 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2304) * 277803737u);
                                                            let _e2315 = ((_e2312 >> bitcast<u32>(22i)) ^ _e2312);
                                                            let _e2318 = f32((_e2315 >> bitcast<u32>(8i)));
                                                            let _e2320 = (3u * _e2285);
                                                            let _e2322 = global_2.member[_e2320];
                                                            let _e2325 = global_2.member[(_e2320 + 1u)];
                                                            let _e2331 = select(select(u32(_e2325.w), 0u, (_e2325.w < 0f)), 4294967295u, (_e2325.w > 4294967000f));
                                                            let _e2333 = (_e856 + (5u * _e2285));
                                                            let _e2335 = global_2.member[_e2333];
                                                            let _e2338 = global_2.member[(_e2333 + 1u)];
                                                            let _e2345 = (_e2331 == 1u);
                                                            if _e2345 {
                                                                let _e2474 = (_e2322.x - _e1111);
                                                                let _e2475 = (_e2322.y - _e1112);
                                                                let _e2476 = (_e2322.z - _e1113);
                                                                let _e2481 = (((_e2474 * _e2474) + (_e2475 * _e2475)) + (_e2476 * _e2476));
                                                                let _e2482 = (_e2322.w * _e2322.w);
                                                                let _e2484 = (_e2481 <= (_e2482 * 1.000001f));
                                                                if _e2484 {
                                                                    let _e2593 = (_e2301 * 0.00000037450704f);
                                                                    let _e2595 = ((_e2318 * 0.00000011920929f) - 1f);
                                                                    let _e2597 = (1f - (_e2595 * _e2595));
                                                                    if (_e2597 != _e2597) {
                                                                        phi_27857_ = true;
                                                                    } else {
                                                                        phi_27857_ = (0f >= _e2597);
                                                                    }
                                                                    let _e2601 = phi_27857_;
                                                                    let _e2603 = sqrt(select(_e2597, 0f, _e2601));
                                                                    let _e2605 = (_e2603 * cos(_e2593));
                                                                    let _e2607 = (_e2603 * sin(_e2593));
                                                                    let _e2612 = (_e2322.x + (_e2605 * _e2322.w));
                                                                    let _e2613 = (_e2322.y + (_e2607 * _e2322.w));
                                                                    let _e2614 = (_e2322.z + (_e2595 * _e2322.w));
                                                                    if _e2484 {
                                                                        let _e2627 = (_e2612 - _e1111);
                                                                        let _e2628 = (_e2613 - _e1112);
                                                                        let _e2629 = (_e2614 - _e1113);
                                                                        let _e2634 = (((_e2627 * _e2627) + (_e2628 * _e2628)) + (_e2629 * _e2629));
                                                                        phi_27926_ = (_e2634 / (((abs((((_e2605 * _e2627) + (_e2607 * _e2628)) + (_e2595 * _e2629))) / sqrt(_e2634)) * 12.566371f) * _e2482));
                                                                    } else {
                                                                        let _e2616 = (_e2482 / _e2481);
                                                                        if (_e2616 != _e2616) {
                                                                            phi_27945_ = true;
                                                                        } else {
                                                                            phi_27945_ = (1f <= _e2616);
                                                                        }
                                                                        let _e2620 = phi_27945_;
                                                                        let _e2621 = select(_e2616, 1f, _e2620);
                                                                        phi_27926_ = (0.15915494f / (_e2621 / (1f + sqrt((1f - _e2621)))));
                                                                    }
                                                                    let _e2647 = phi_27926_;
                                                                    phi_17160_ = type_72(vec3<f32>(_e2612, _e2613, _e2614), vec3<f32>(_e2605, _e2607, _e2595), _e2647);
                                                                } else {
                                                                    let _e2485 = (_e2482 / _e2481);
                                                                    let _e2486 = (_e2485 != _e2485);
                                                                    if _e2486 {
                                                                        phi_27709_ = true;
                                                                    } else {
                                                                        phi_27709_ = (1f <= _e2485);
                                                                    }
                                                                    let _e2489 = phi_27709_;
                                                                    let _e2490 = select(_e2485, 1f, _e2489);
                                                                    if _e2486 {
                                                                        phi_27732_ = true;
                                                                    } else {
                                                                        phi_27732_ = (1f <= _e2485);
                                                                    }
                                                                    let _e2494 = phi_27732_;
                                                                    let _e2495 = select(_e2485, 1f, _e2494);
                                                                    let _e2499 = (_e2495 / (1f + sqrt((1f - _e2495))));
                                                                    if (_e2490 < 0.00068523f) {
                                                                        phi_17095_ = (_e2490 * _e2302);
                                                                    } else {
                                                                        let _e2502 = (1f - (_e2302 * _e2499));
                                                                        let _e2504 = (1f - (_e2502 * _e2502));
                                                                        if (_e2504 != _e2504) {
                                                                            phi_27747_ = true;
                                                                        } else {
                                                                            phi_27747_ = (0f >= _e2504);
                                                                        }
                                                                        let _e2508 = phi_27747_;
                                                                        phi_17095_ = select(_e2504, 0f, _e2508);
                                                                    }
                                                                    let _e2512 = phi_17095_;
                                                                    let _e2513 = (1f - _e2512);
                                                                    if (_e2513 != _e2513) {
                                                                        phi_27762_ = true;
                                                                    } else {
                                                                        phi_27762_ = (0f >= _e2513);
                                                                    }
                                                                    let _e2517 = phi_27762_;
                                                                    let _e2522 = (1f - (_e2512 / _e2490));
                                                                    if (_e2522 != _e2522) {
                                                                        phi_27777_ = true;
                                                                    } else {
                                                                        phi_27777_ = (0f >= _e2522);
                                                                    }
                                                                    let _e2526 = phi_27777_;
                                                                    let _e2530 = ((_e2512 / sqrt(_e2490)) + (sqrt(select(_e2513, 0f, _e2517)) * sqrt(select(_e2522, 0f, _e2526))));
                                                                    let _e2532 = (1f - (_e2530 * _e2530));
                                                                    if (_e2532 != _e2532) {
                                                                        phi_27792_ = true;
                                                                    } else {
                                                                        phi_27792_ = (0f >= _e2532);
                                                                    }
                                                                    let _e2536 = phi_27792_;
                                                                    let _e2538 = sqrt(select(_e2532, 0f, _e2536));
                                                                    let _e2539 = (_e2318 * 0.00000037450704f);
                                                                    let _e2540 = sqrt(_e2481);
                                                                    let _e2541 = (_e2474 / _e2540);
                                                                    let _e2542 = (_e2475 / _e2540);
                                                                    let _e2543 = (_e2476 / _e2540);
                                                                    let _e2545 = select(-1f, 1f, (_e2543 >= 0f));
                                                                    let _e2547 = (-1f / (_e2545 + _e2543));
                                                                    let _e2549 = ((_e2541 * _e2542) * _e2547);
                                                                    let _e2562 = (_e2538 * cos(_e2539));
                                                                    let _e2567 = (_e2538 * sin(_e2539));
                                                                    let _e2580 = -(((((1f + (((_e2545 * _e2541) * _e2541) * _e2547)) * _e2562) + (_e2549 * _e2567)) + (_e2541 * _e2530)));
                                                                    let _e2581 = -(((((_e2545 * _e2549) * _e2562) + ((_e2545 + ((_e2542 * _e2542) * _e2547)) * _e2567)) + (_e2542 * _e2530)));
                                                                    let _e2582 = -(((((-(_e2545) * _e2541) * _e2562) + (-(_e2542) * _e2567)) + (_e2543 * _e2530)));
                                                                    phi_17160_ = type_72(vec3<f32>((_e2322.x + (_e2580 * _e2322.w)), (_e2322.y + (_e2581 * _e2322.w)), (_e2322.z + (_e2582 * _e2322.w))), vec3<f32>(_e2580, _e2581, _e2582), (0.15915494f / _e2499));
                                                                }
                                                                let _e2650 = phi_17160_;
                                                                phi_17588_ = _e2650;
                                                            } else {
                                                                let _e2346 = sqrt(_e2302);
                                                                let _e2347 = (1f - _e2346);
                                                                let _e2348 = ((_e2318 * 0.000000059604645f) * _e2346);
                                                                let _e2361 = (_e2346 - _e2348);
                                                                let _e2365 = (((_e2322.x * _e2347) + (_e2335.x * _e2348)) + (_e2338.x * _e2361));
                                                                let _e2366 = (((_e2322.y * _e2347) + (_e2335.y * _e2348)) + (_e2338.y * _e2361));
                                                                let _e2367 = (((_e2322.z * _e2347) + (_e2335.z * _e2348)) + (_e2338.z * _e2361));
                                                                let _e2369 = (_e2335.x - _e2322.x);
                                                                let _e2370 = (_e2335.y - _e2322.y);
                                                                let _e2371 = (_e2335.z - _e2322.z);
                                                                let _e2372 = (_e2338.x - _e2322.x);
                                                                let _e2373 = (_e2338.y - _e2322.y);
                                                                let _e2374 = (_e2338.z - _e2322.z);
                                                                let _e2377 = ((_e2370 * _e2374) - (_e2373 * _e2371));
                                                                let _e2380 = ((_e2371 * _e2372) - (_e2374 * _e2369));
                                                                let _e2383 = ((_e2369 * _e2373) - (_e2372 * _e2370));
                                                                let _e2389 = sqrt((((_e2377 * _e2377) + (_e2380 * _e2380)) + (_e2383 * _e2383)));
                                                                let _e2390 = (1f / _e2389);
                                                                let _e2391 = (_e2377 * _e2390);
                                                                let _e2392 = (_e2380 * _e2390);
                                                                let _e2393 = (_e2383 * _e2390);
                                                                if _e2345 {
                                                                    let _e2424 = (_e2322.x - _e1111);
                                                                    let _e2425 = (_e2322.y - _e1112);
                                                                    let _e2426 = (_e2322.z - _e1113);
                                                                    let _e2431 = (((_e2424 * _e2424) + (_e2425 * _e2425)) + (_e2426 * _e2426));
                                                                    let _e2432 = (_e2322.w * _e2322.w);
                                                                    if (_e2431 <= (_e2432 * 1.000001f)) {
                                                                        let _e2446 = (_e2365 - _e1111);
                                                                        let _e2447 = (_e2366 - _e1112);
                                                                        let _e2448 = (_e2367 - _e1113);
                                                                        let _e2453 = (((_e2446 * _e2446) + (_e2447 * _e2447)) + (_e2448 * _e2448));
                                                                        phi_27675_ = (_e2453 / (((abs((((_e2391 * _e2446) + (_e2392 * _e2447)) + (_e2393 * _e2448))) / sqrt(_e2453)) * 12.566371f) * _e2432));
                                                                    } else {
                                                                        let _e2435 = (_e2432 / _e2431);
                                                                        if (_e2435 != _e2435) {
                                                                            phi_27694_ = true;
                                                                        } else {
                                                                            phi_27694_ = (1f <= _e2435);
                                                                        }
                                                                        let _e2439 = phi_27694_;
                                                                        let _e2440 = select(_e2435, 1f, _e2439);
                                                                        phi_27675_ = (0.15915494f / (_e2440 / (1f + sqrt((1f - _e2440)))));
                                                                    }
                                                                    let _e2466 = phi_27675_;
                                                                    phi_17586_ = _e2466;
                                                                } else {
                                                                    let _e2395 = (_e2365 - _e1111);
                                                                    let _e2396 = (_e2366 - _e1112);
                                                                    let _e2397 = (_e2367 - _e1113);
                                                                    let _e2402 = (((_e2395 * _e2395) + (_e2396 * _e2396)) + (_e2397 * _e2397));
                                                                    let _e2410 = (abs((((_e2391 * _e2395) + (_e2392 * _e2396)) + (_e2393 * _e2397))) / sqrt(_e2402));
                                                                    if (_e2410 <= 0f) {
                                                                        phi_17585_ = 0f;
                                                                    } else {
                                                                        if (_e2331 == 0u) {
                                                                            phi_17582_ = (_e2389 * 0.5f);
                                                                        } else {
                                                                            phi_17582_ = ((12.566371f * _e2322.w) * _e2322.w);
                                                                        }
                                                                        let _e2418 = phi_17582_;
                                                                        phi_17585_ = (_e2402 / (_e2410 * _e2418));
                                                                    }
                                                                    let _e2422 = phi_17585_;
                                                                    phi_17586_ = _e2422;
                                                                }
                                                                let _e2468 = phi_17586_;
                                                                phi_17588_ = type_72(vec3<f32>(_e2365, _e2366, _e2367), vec3<f32>(_e2391, _e2392, _e2393), _e2468);
                                                            }
                                                            let _e2652 = phi_17588_;
                                                            let _e2657 = (_e2652.member.x - _e1111);
                                                            let _e2659 = (_e2652.member.y - _e1112);
                                                            let _e2661 = (_e2652.member.z - _e1113);
                                                            let _e2667 = sqrt((((_e2657 * _e2657) + (_e2659 * _e2659)) + (_e2661 * _e2661)));
                                                            let _e2668 = (_e2657 / _e2667);
                                                            let _e2669 = (_e2659 / _e2667);
                                                            let _e2670 = (_e2661 / _e2667);
                                                            if (_e2652.member_2 > 0f) {
                                                                let _e2686 = global_4.member[select(select(u32(_e2335.w), 0u, (_e2335.w < 0f)), 4294967295u, (_e2335.w > 4294967000f))];
                                                                if (_e2686.member == 4u) {
                                                                    if (_e2686.member_1 == 1u) {
                                                                        phi_17764_ = true;
                                                                    } else {
                                                                        phi_17764_ = ((((-(_e2668) * _e2652.member_1.x) + (-(_e2669) * _e2652.member_1.y)) + (-(_e2670) * _e2652.member_1.z)) > 0f);
                                                                    }
                                                                    let _e2701 = phi_17764_;
                                                                    if _e2701 {
                                                                        phi_17773_ = vec3<f32>(_e2686.member_4.x, _e2686.member_4.y, _e2686.member_4.z);
                                                                    } else {
                                                                        phi_17773_ = vec3<f32>();
                                                                    }
                                                                    let _e2708 = phi_17773_;
                                                                    phi_17775_ = _e2708;
                                                                    phi_17776_ = select(true, false, _e2701);
                                                                } else {
                                                                    phi_17775_ = vec3<f32>();
                                                                    phi_17776_ = true;
                                                                }
                                                                let _e2711 = phi_17775_;
                                                                let _e2713 = phi_17776_;
                                                                phi_17781_ = ((_e2273 - _e2275) * _e2652.member_2);
                                                                phi_17782_ = select(_e2711, vec3<f32>(0f, 0f, 0f), vec3(_e2713));
                                                            } else {
                                                                phi_17781_ = 0f;
                                                                phi_17782_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2717 = phi_17781_;
                                                            let _e2719 = phi_17782_;
                                                            phi_36568_ = _e2315;
                                                            phi_17783_ = _e2717;
                                                            phi_17784_ = vec3<f32>(_e2668, _e2669, _e2670);
                                                            phi_17785_ = _e2719;
                                                            phi_17786_ = ((_e2667 * 0.9999f) - _e1104);
                                                        } else {
                                                            phi_36568_ = _e1994;
                                                            phi_17783_ = 0f;
                                                            phi_17784_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17785_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17786_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e2721 = phi_36568_;
                                                        let _e2723 = phi_17783_;
                                                        let _e2725 = phi_17784_;
                                                        let _e2727 = phi_17785_;
                                                        let _e2729 = phi_17786_;
                                                        phi_36567_ = _e2721;
                                                        phi_17787_ = _e2723;
                                                        phi_17788_ = _e2725;
                                                        phi_17789_ = _e2727;
                                                        phi_17790_ = _e2729;
                                                    }
                                                    let _e2731 = phi_36567_;
                                                    let _e2733 = phi_17787_;
                                                    let _e2735 = phi_17788_;
                                                    let _e2737 = phi_17789_;
                                                    let _e2739 = phi_17790_;
                                                    phi_36559_ = _e2731;
                                                    phi_18149_ = _e2733;
                                                    phi_18150_ = _e2735;
                                                    phi_18151_ = _e2737;
                                                    phi_18152_ = _e2739;
                                                } else {
                                                    if ((_e384 & 2u) == 0u) {
                                                        phi_36560_ = _e1994;
                                                        phi_18146_ = 0f;
                                                        phi_18147_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_18148_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2012 = ((_e1994 * 747796405u) + 2891336453u);
                                                        let _e2020 = (((_e2012 >> bitcast<u32>((((_e2012 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2012) * 277803737u);
                                                        let _e2023 = ((_e2020 >> bitcast<u32>(22i)) ^ _e2020);
                                                        let _e2027 = (f32((_e2023 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2029 = ((_e2023 * 747796405u) + 2891336453u);
                                                        let _e2037 = (((_e2029 >> bitcast<u32>((((_e2029 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2029) * 277803737u);
                                                        let _e2040 = ((_e2037 >> bitcast<u32>(22i)) ^ _e2037);
                                                        let _e2044 = (f32((_e2040 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2047 = global_1.member.member_3[1u];
                                                        let _e2050 = global_1.member.member_3[2u];
                                                        let _e2053 = global_1.member.member_2[3u];
                                                        phi_17811_ = 0u;
                                                        phi_17814_ = _e2050;
                                                        loop {
                                                            let _e2055 = phi_17811_;
                                                            let _e2057 = phi_17814_;
                                                            local_73 = _e2055;
                                                            local_76 = _e2055;
                                                            local_77 = _e2055;
                                                            let _e2059 = ((_e2055 + 1u) < _e2057);
                                                            if _e2059 {
                                                                let _e2061 = ((_e2055 + _e2057) / 2u);
                                                                let _e2064 = global_6.member[(_e2053 + _e2061)];
                                                                let _e2065 = (_e2064 <= _e2027);
                                                                phi_17812_ = select(_e2055, _e2061, _e2065);
                                                                phi_17815_ = select(_e2061, _e2057, _e2065);
                                                            } else {
                                                                phi_17812_ = u32();
                                                                phi_17815_ = u32();
                                                            }
                                                            let _e2069 = phi_17812_;
                                                            let _e2071 = phi_17815_;
                                                            phi_17811_ = _e2069;
                                                            phi_17814_ = _e2071;
                                                            if !(_e2059) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2075 = global_1.member.member_3[0u];
                                                        let _e2076 = (_e2047 + 1u);
                                                        let _e2078 = local_73;
                                                        let _e2080 = (_e2075 + (_e2078 * _e2076));
                                                        phi_17852_ = 0u;
                                                        phi_17855_ = _e2047;
                                                        loop {
                                                            let _e2082 = phi_17852_;
                                                            let _e2084 = phi_17855_;
                                                            local_74 = _e2082;
                                                            local_75 = _e2082;
                                                            let _e2086 = ((_e2082 + 1u) < _e2084);
                                                            if _e2086 {
                                                                let _e2088 = ((_e2082 + _e2084) / 2u);
                                                                let _e2091 = global_6.member[(_e2080 + _e2088)];
                                                                let _e2092 = (_e2091 <= _e2044);
                                                                phi_17853_ = select(_e2082, _e2088, _e2092);
                                                                phi_17856_ = select(_e2088, _e2084, _e2092);
                                                            } else {
                                                                phi_17853_ = u32();
                                                                phi_17856_ = u32();
                                                            }
                                                            let _e2096 = phi_17853_;
                                                            let _e2098 = phi_17856_;
                                                            phi_17852_ = _e2096;
                                                            phi_17855_ = _e2098;
                                                            if !(_e2086) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2101 = local_74;
                                                        let _e2104 = local_75;
                                                        let _e2105 = (_e2080 + _e2104);
                                                        let _e2107 = global_6.member[_e2105];
                                                        let _e2110 = global_6.member[(_e2105 + 1u)];
                                                        if (_e2110 > _e2107) {
                                                            let _e2114 = ((_e2044 - _e2107) / (_e2110 - _e2107));
                                                            let _e2116 = select(_e2114, 0f, (_e2114 < 0f));
                                                            phi_17932_ = select(_e2116, 0.99999f, (_e2116 > 0.99999f));
                                                        } else {
                                                            phi_17932_ = 0.5f;
                                                        }
                                                        let _e2120 = phi_17932_;
                                                        let _e2122 = f32(_e2047);
                                                        let _e2125 = local_76;
                                                        let _e2128 = local_77;
                                                        let _e2129 = (_e2053 + _e2128);
                                                        let _e2131 = global_6.member[_e2129];
                                                        let _e2134 = global_6.member[(_e2129 + 1u)];
                                                        if (_e2134 > _e2131) {
                                                            let _e2138 = ((_e2027 - _e2131) / (_e2134 - _e2131));
                                                            let _e2140 = select(_e2138, 0f, (_e2138 < 0f));
                                                            phi_17981_ = select(_e2140, 0.99999f, (_e2140 > 0.99999f));
                                                        } else {
                                                            phi_17981_ = 0.5f;
                                                        }
                                                        let _e2144 = phi_17981_;
                                                        let _e2146 = f32(_e2050);
                                                        let _e2149 = ((((f32(_e2101) + _e2120) / _e2122) - 0.5f) * 6.2831855f);
                                                        let _e2150 = (((f32(_e2125) + _e2144) / _e2146) * 3.1415927f);
                                                        let _e2151 = sin(_e2150);
                                                        let _e2153 = (_e2151 * cos(_e2149));
                                                        let _e2154 = cos(_e2150);
                                                        let _e2156 = (_e2151 * sin(_e2149));
                                                        let _e2159 = ((atan2(_e2156, _e2153) * 0.15915494f) + 0.5f);
                                                        let _e2163 = select(_e2154, -1f, (_e2154 < -1f));
                                                        let _e2166 = acos(select(_e2163, 1f, (_e2163 > 1f)));
                                                        let _e2168 = ((_e2159 - floor(_e2159)) * _e2122);
                                                        let _e2173 = select(select(u32(_e2168), 0u, (_e2168 < 0f)), 4294967295u, (_e2168 > 4294967000f));
                                                        let _e2174 = (_e2047 - 1u);
                                                        let _e2177 = ((_e2166 * 0.31830987f) * _e2146);
                                                        let _e2182 = select(select(u32(_e2177), 0u, (_e2177 < 0f)), 4294967295u, (_e2177 > 4294967000f));
                                                        let _e2183 = (_e2050 - 1u);
                                                        let _e2185 = select(_e2182, _e2183, (_e2183 < _e2182));
                                                        let _e2186 = sin(_e2166);
                                                        if (_e2186 <= 0f) {
                                                            phi_18137_ = 0f;
                                                        } else {
                                                            let _e2190 = (_e2053 + _e2185);
                                                            let _e2193 = global_6.member[(_e2190 + 1u)];
                                                            let _e2195 = global_6.member[_e2190];
                                                            let _e2198 = ((_e2075 + (_e2185 * _e2076)) + select(_e2173, _e2174, (_e2174 < _e2173)));
                                                            let _e2201 = global_6.member[(_e2198 + 1u)];
                                                            let _e2203 = global_6.member[_e2198];
                                                            phi_18137_ = ((((_e2193 - _e2195) * _e2146) * ((_e2201 - _e2203) * _e2122)) / (19.73921f * _e2186));
                                                        }
                                                        let _e2210 = phi_18137_;
                                                        let _e2211 = (_e2210 > 0f);
                                                        if _e2211 {
                                                            phi_18144_ = vec3<f32>(_e2153, _e2154, _e2156);
                                                        } else {
                                                            phi_18144_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2214 = phi_18144_;
                                                        phi_36560_ = _e2040;
                                                        phi_18146_ = select(0f, _e2210, _e2211);
                                                        phi_18147_ = _e2214;
                                                        phi_18148_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2211));
                                                    }
                                                    let _e2219 = phi_36560_;
                                                    let _e2221 = phi_18146_;
                                                    let _e2223 = phi_18147_;
                                                    let _e2225 = phi_18148_;
                                                    phi_36559_ = _e2219;
                                                    phi_18149_ = _e2221;
                                                    phi_18150_ = _e2223;
                                                    phi_18151_ = _e2225;
                                                    phi_18152_ = 1000000000000000000000000000000f;
                                                }
                                                let _e2741 = phi_36559_;
                                                let _e2743 = phi_18149_;
                                                let _e2745 = phi_18150_;
                                                let _e2747 = phi_18151_;
                                                let _e2749 = phi_18152_;
                                                phi_36558_ = _e2741;
                                                phi_18153_ = _e2743;
                                                phi_18154_ = _e2745;
                                                phi_18155_ = _e2747;
                                                phi_18156_ = _e2749;
                                            }
                                            let _e3991 = phi_36558_;
                                            let _e3993 = phi_18153_;
                                            let _e3995 = phi_18154_;
                                            let _e3997 = phi_18155_;
                                            let _e3999 = phi_18156_;
                                            phi_36557_ = _e3991;
                                            phi_18157_ = _e3993;
                                            phi_18158_ = _e3995;
                                            phi_18159_ = _e3997;
                                            phi_18160_ = _e3999;
                                        }
                                        let _e4068 = phi_36557_;
                                        let _e4070 = phi_18157_;
                                        let _e4072 = phi_18158_;
                                        let _e4074 = phi_18159_;
                                        let _e4076 = phi_18160_;
                                        if ((((_e4074.x * _e4074.x) + (_e4074.y * _e4074.y)) + (_e4074.z * _e4074.z)) > 0f) {
                                            let _e4093 = (((_e4072.x * _e1086.x) + (_e4072.y * _e1086.y)) + (_e4072.z * _e1086.z));
                                            if (_e4093 != _e4093) {
                                                phi_29259_ = true;
                                            } else {
                                                phi_29259_ = (0f >= _e4093);
                                            }
                                            let _e4097 = phi_29259_;
                                            let _e4099 = (select(_e4093, 0f, _e4097) * 0.31830987f);
                                            if (_e1979 == 0u) {
                                                if (_e4093 <= 0f) {
                                                    phi_30056_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_30056_ = (_e1973 * (_e4093 * 0.31830987f));
                                                }
                                                let _e4575 = phi_30056_;
                                                phi_18556_ = type_59(_e4575, _e4099);
                                                phi_18557_ = type_59();
                                                phi_18558_ = true;
                                            } else {
                                                let _e4105 = (((_e1060 * _e1134) + (_e1061 * _e1135)) + (_e1062 * _e1137));
                                                let _e4110 = (((_e1060 * _e1130) + (_e1061 * _e1140)) + (_e1062 * _e1141));
                                                let _e4115 = (((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z));
                                                let _e4120 = (((_e4072.x * _e1134) + (_e4072.y * _e1135)) + (_e4072.z * _e1137));
                                                let _e4125 = (((_e4072.x * _e1130) + (_e4072.y * _e1140)) + (_e4072.z * _e1141));
                                                if (_e4115 <= 0f) {
                                                    phi_18520_ = type_59();
                                                    phi_18521_ = true;
                                                } else {
                                                    let _e4127 = (_e4093 <= 0f);
                                                    if _e4127 {
                                                        phi_18519_ = type_59();
                                                    } else {
                                                        let _e4128 = (_e4105 + _e4120);
                                                        let _e4129 = (_e4110 + _e4125);
                                                        let _e4130 = (_e4115 + _e4093);
                                                        let _e4137 = (1f / sqrt((((_e4128 * _e4128) + (_e4129 * _e4129)) + (_e4130 * _e4130))));
                                                        let _e4140 = (_e4130 * _e4137);
                                                        if (_e4140 <= 0f) {
                                                            phi_29369_ = 0f;
                                                        } else {
                                                            let _e4142 = (_e1977 * _e1977);
                                                            let _e4146 = ((((_e4142 - 1f) * _e4140) * _e4140) + 1f);
                                                            phi_29369_ = (_e4142 / ((3.1415927f * _e4146) * _e4146));
                                                        }
                                                        let _e4151 = phi_29369_;
                                                        let _e4152 = (_e4115 * _e4115);
                                                        if (_e4152 >= 1f) {
                                                            phi_29397_ = 0f;
                                                        } else {
                                                            phi_29397_ = ((sqrt((1f + (((_e1977 * _e1977) * ((_e4105 * _e4105) + (_e4110 * _e4110))) / _e4152))) - 1f) * 0.5f);
                                                        }
                                                        let _e4165 = phi_29397_;
                                                        let _e4166 = (1f + _e4165);
                                                        let _e4167 = (_e4093 * _e4093);
                                                        if (_e4167 >= 1f) {
                                                            phi_29425_ = 0f;
                                                        } else {
                                                            phi_29425_ = ((sqrt((1f + (((_e1977 * _e1977) * ((_e4120 * _e4120) + (_e4125 * _e4125))) / _e4167))) - 1f) * 0.5f);
                                                        }
                                                        let _e4180 = phi_29425_;
                                                        let _e4187 = (((_e4105 * (_e4128 * _e4137)) + (_e4110 * (_e4129 * _e4137))) + (_e4115 * _e4140));
                                                        if (_e1975 == 0u) {
                                                            let _e4482 = (1f - _e4187);
                                                            if (_e4482 != _e4482) {
                                                                phi_29928_ = true;
                                                            } else {
                                                                phi_29928_ = (0f >= _e4482);
                                                            }
                                                            let _e4486 = phi_29928_;
                                                            let _e4488 = pow(select(_e4482, 0f, _e4486), 5f);
                                                            let _e4489 = (1f - _e4488);
                                                            phi_18507_ = vec3<f32>(((_e1973.x * _e4489) + _e4488), ((_e1973.y * _e4489) + _e4488), ((_e1973.z * _e4489) + _e4488));
                                                        } else {
                                                            if (_e1975 == 1u) {
                                                                let _e4455 = (1f / _e1969);
                                                                let _e4459 = ((_e4455 * _e4455) * (1f - (_e4187 * _e4187)));
                                                                if (_e4459 >= 1f) {
                                                                    phi_29889_ = 1f;
                                                                } else {
                                                                    let _e4462 = sqrt((1f - _e4459));
                                                                    if (_e4187 != _e4187) {
                                                                        phi_29900_ = true;
                                                                    } else {
                                                                        phi_29900_ = (_e4462 <= _e4187);
                                                                    }
                                                                    let _e4466 = phi_29900_;
                                                                    let _e4471 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                                    phi_29889_ = (_e4471 + ((1f - _e4471) * pow((1f - select(_e4187, _e4462, _e4466)), 5f)));
                                                                }
                                                                let _e4478 = phi_29889_;
                                                                phi_18506_ = vec3(_e4478);
                                                            } else {
                                                                let _e4191 = select(_e4187, 0f, (_e4187 < 0f));
                                                                let _e4193 = select(_e4191, 1f, (_e4191 > 1f));
                                                                let _e4195 = (1f - (_e4193 * _e4193));
                                                                let _e4208 = ((_e1973.x * _e1973.x) - (_e1971.x * _e1971.x));
                                                                let _e4209 = ((_e1973.y * _e1973.y) - (_e1971.y * _e1971.y));
                                                                let _e4210 = ((_e1973.z * _e1973.z) - (_e1971.z * _e1971.z));
                                                                let _e4212 = (_e1973.x * (_e1971.x + _e1971.x));
                                                                let _e4214 = (_e1973.y * (_e1971.y + _e1971.y));
                                                                let _e4216 = (_e1973.z * (_e1971.z + _e1971.z));
                                                                let _e4223 = ((_e4208 * _e4208) + (_e4212 * _e4212));
                                                                let _e4224 = ((_e4209 * _e4209) + (_e4214 * _e4214));
                                                                let _e4225 = ((_e4210 * _e4210) + (_e4216 * _e4216));
                                                                let _e4230 = ((_e4195 * _e4208) / _e4223);
                                                                let _e4231 = ((_e4195 * _e4209) / _e4224);
                                                                let _e4232 = ((_e4195 * _e4210) / _e4225);
                                                                let _e4233 = (1f - _e4230);
                                                                let _e4234 = (1f - _e4231);
                                                                let _e4235 = (1f - _e4232);
                                                                let _e4237 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4230, _e4231, _e4232));
                                                                let _e4238 = (_e4195 * _e4212);
                                                                let _e4239 = (_e4195 * _e4214);
                                                                let _e4240 = (_e4195 * _e4216);
                                                                let _e4245 = (vec3<f32>(_e4238, _e4239, _e4240) / vec3<f32>(_e4223, _e4224, _e4225));
                                                                let _e4258 = ((_e4237.x * _e4237.x) + (_e4245.x * _e4245.x));
                                                                let _e4259 = ((_e4237.y * _e4237.y) + (_e4245.y * _e4245.y));
                                                                let _e4260 = ((_e4237.z * _e4237.z) + (_e4245.z * _e4245.z));
                                                                if (_e4258 != _e4258) {
                                                                    phi_29559_ = true;
                                                                } else {
                                                                    phi_29559_ = (0f >= _e4258);
                                                                }
                                                                let _e4264 = phi_29559_;
                                                                let _e4266 = sqrt(select(_e4258, 0f, _e4264));
                                                                if (_e4259 != _e4259) {
                                                                    phi_29574_ = true;
                                                                } else {
                                                                    phi_29574_ = (0f >= _e4259);
                                                                }
                                                                let _e4270 = phi_29574_;
                                                                let _e4272 = sqrt(select(_e4259, 0f, _e4270));
                                                                if (_e4260 != _e4260) {
                                                                    phi_29589_ = true;
                                                                } else {
                                                                    phi_29589_ = (0f >= _e4260);
                                                                }
                                                                let _e4276 = phi_29589_;
                                                                let _e4278 = sqrt(select(_e4260, 0f, _e4276));
                                                                let _e4282 = ((_e4266 + _e4233) * 0.5f);
                                                                let _e4283 = ((_e4272 + _e4234) * 0.5f);
                                                                let _e4284 = ((_e4278 + _e4235) * 0.5f);
                                                                if (_e4282 != _e4282) {
                                                                    phi_29616_ = true;
                                                                } else {
                                                                    phi_29616_ = (0f >= _e4282);
                                                                }
                                                                let _e4288 = phi_29616_;
                                                                let _e4290 = sqrt(select(_e4282, 0f, _e4288));
                                                                if (_e4283 != _e4283) {
                                                                    phi_29631_ = true;
                                                                } else {
                                                                    phi_29631_ = (0f >= _e4283);
                                                                }
                                                                let _e4294 = phi_29631_;
                                                                let _e4296 = sqrt(select(_e4283, 0f, _e4294));
                                                                if (_e4284 != _e4284) {
                                                                    phi_29646_ = true;
                                                                } else {
                                                                    phi_29646_ = (0f >= _e4284);
                                                                }
                                                                let _e4300 = phi_29646_;
                                                                let _e4302 = sqrt(select(_e4284, 0f, _e4300));
                                                                let _e4303 = vec3<f32>(_e4290, _e4296, _e4302);
                                                                let _e4307 = ((_e4266 - _e4233) * 0.5f);
                                                                let _e4308 = ((_e4272 - _e4234) * 0.5f);
                                                                let _e4309 = ((_e4278 - _e4235) * 0.5f);
                                                                if (_e4307 != _e4307) {
                                                                    phi_29673_ = true;
                                                                } else {
                                                                    phi_29673_ = (0f >= _e4307);
                                                                }
                                                                let _e4313 = phi_29673_;
                                                                let _e4315 = sqrt(select(_e4307, 0f, _e4313));
                                                                if (_e4308 != _e4308) {
                                                                    phi_29688_ = true;
                                                                } else {
                                                                    phi_29688_ = (0f >= _e4308);
                                                                }
                                                                let _e4319 = phi_29688_;
                                                                let _e4321 = sqrt(select(_e4308, 0f, _e4319));
                                                                if (_e4309 != _e4309) {
                                                                    phi_29703_ = true;
                                                                } else {
                                                                    phi_29703_ = (0f >= _e4309);
                                                                }
                                                                let _e4325 = phi_29703_;
                                                                let _e4327 = sqrt(select(_e4309, 0f, _e4325));
                                                                if ((_e4238 / _e4223) < 0f) {
                                                                    phi_18383_ = -(_e4315);
                                                                } else {
                                                                    phi_18383_ = _e4315;
                                                                }
                                                                let _e4331 = phi_18383_;
                                                                if ((_e4239 / _e4224) < 0f) {
                                                                    phi_18391_ = -(_e4321);
                                                                } else {
                                                                    phi_18391_ = _e4321;
                                                                }
                                                                let _e4335 = phi_18391_;
                                                                if ((_e4240 / _e4225) < 0f) {
                                                                    phi_18399_ = -(_e4327);
                                                                } else {
                                                                    phi_18399_ = _e4327;
                                                                }
                                                                let _e4339 = phi_18399_;
                                                                let _e4343 = (_e1971.x * _e4193);
                                                                let _e4344 = (_e1971.y * _e4193);
                                                                let _e4345 = (_e1971.z * _e4193);
                                                                let _e4355 = vec3<f32>(((_e1973.x * _e4290) - (_e1971.x * _e4331)), ((_e1973.y * _e4296) - (_e1971.y * _e4335)), ((_e1973.z * _e4302) - (_e1971.z * _e4339)));
                                                                let _e4362 = ((_e1973.x * _e4331) + (_e1971.x * _e4290));
                                                                let _e4363 = ((_e1973.y * _e4335) + (_e1971.y * _e4296));
                                                                let _e4364 = ((_e1973.z * _e4339) + (_e1971.z * _e4302));
                                                                let _e4366 = vec3<f32>((_e1973.x * _e4193), (_e1973.y * _e4193), (_e1973.z * _e4193));
                                                                let _e4367 = (_e4366 - _e4303);
                                                                let _e4368 = (_e4343 - _e4331);
                                                                let _e4369 = (_e4344 - _e4335);
                                                                let _e4370 = (_e4345 - _e4339);
                                                                let _e4383 = (_e4366 + _e4303);
                                                                let _e4384 = (_e4343 + _e4331);
                                                                let _e4385 = (_e4344 + _e4335);
                                                                let _e4386 = (_e4345 + _e4339);
                                                                let _e4396 = ((_e4383.x * _e4383.x) + (_e4384 * _e4384));
                                                                let _e4397 = ((_e4383.y * _e4383.y) + (_e4385 * _e4385));
                                                                let _e4398 = ((_e4383.z * _e4383.z) + (_e4386 * _e4386));
                                                                let _e4408 = vec3(_e4193);
                                                                let _e4409 = (_e4408 - _e4355);
                                                                let _e4410 = -(vec3<f32>(_e4362, _e4363, _e4364));
                                                                let _e4426 = (_e4408 + _e4355);
                                                                let _e4436 = ((_e4426.x * _e4426.x) + (_e4362 * _e4362));
                                                                let _e4437 = ((_e4426.y * _e4426.y) + (_e4363 * _e4363));
                                                                let _e4438 = ((_e4426.z * _e4426.z) + (_e4364 * _e4364));
                                                                phi_18506_ = vec3<f32>((((((_e4367.x * _e4367.x) + (_e4368 * _e4368)) / select(0.000000000000000000000000000001f, _e4396, (_e4396 > 0.000000000000000000000000000001f))) + (((_e4409.x * _e4409.x) + (_e4410.x * _e4410.x)) / select(0.000000000000000000000000000001f, _e4436, (_e4436 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4367.y * _e4367.y) + (_e4369 * _e4369)) / select(0.000000000000000000000000000001f, _e4397, (_e4397 > 0.000000000000000000000000000001f))) + (((_e4409.y * _e4409.y) + (_e4410.y * _e4410.y)) / select(0.000000000000000000000000000001f, _e4437, (_e4437 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4367.z * _e4367.z) + (_e4370 * _e4370)) / select(0.000000000000000000000000000001f, _e4398, (_e4398 > 0.000000000000000000000000000001f))) + (((_e4409.z * _e4409.z) + (_e4410.z * _e4410.z)) / select(0.000000000000000000000000000001f, _e4438, (_e4438 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e4481 = phi_18506_;
                                                            phi_18507_ = _e4481;
                                                        }
                                                        let _e4501 = phi_18507_;
                                                        let _e4503 = (4f * _e4115);
                                                        phi_18519_ = type_59((_e4501 * (((1f / (_e4166 + _e4180)) * _e4151) / _e4503)), ((_e4151 / _e4166) / _e4503));
                                                    }
                                                    let _e4510 = phi_18519_;
                                                    phi_18520_ = _e4510;
                                                    phi_18521_ = _e4127;
                                                }
                                                let _e4512 = phi_18520_;
                                                let _e4514 = phi_18521_;
                                                if _e4514 {
                                                    phi_18526_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_18526_ = _e4512;
                                                }
                                                let _e4516 = phi_18526_;
                                                let _e4519 = (_e1979 == 1u);
                                                if _e4519 {
                                                    phi_18554_ = _e4516;
                                                    phi_18555_ = type_59();
                                                } else {
                                                    if (_e4115 != _e4115) {
                                                        phi_29965_ = true;
                                                    } else {
                                                        phi_29965_ = (0f >= _e4115);
                                                    }
                                                    let _e4523 = phi_29965_;
                                                    let _e4524 = select(_e4115, 0f, _e4523);
                                                    let _e4525 = (1f / _e1969);
                                                    let _e4529 = ((_e4525 * _e4525) * (1f - (_e4524 * _e4524)));
                                                    if (_e4529 >= 1f) {
                                                        phi_29984_ = 1f;
                                                    } else {
                                                        let _e4532 = sqrt((1f - _e4529));
                                                        if (_e4524 != _e4524) {
                                                            phi_29995_ = true;
                                                        } else {
                                                            phi_29995_ = (_e4532 <= _e4524);
                                                        }
                                                        let _e4536 = phi_29995_;
                                                        let _e4541 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                        phi_29984_ = (_e4541 + ((1f - _e4541) * pow((1f - select(_e4524, _e4532, _e4536)), 5f)));
                                                    }
                                                    let _e4548 = phi_29984_;
                                                    if (_e4093 <= 0f) {
                                                        phi_30034_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_30034_ = (_e1967 * (_e4093 * 0.31830987f));
                                                    }
                                                    let _e4553 = phi_30034_;
                                                    let _e4554 = (1f - _e4548);
                                                    phi_18554_ = type_59();
                                                    phi_18555_ = type_59((_e4516.member + vec3<f32>((_e4553.x * _e4554), (_e4553.y * _e4554), (_e4553.z * _e4554))), ((_e4548 * _e4516.member_1) + (_e4554 * _e4099)));
                                                }
                                                let _e4568 = phi_18554_;
                                                let _e4570 = phi_18555_;
                                                phi_18556_ = _e4568;
                                                phi_18557_ = _e4570;
                                                phi_18558_ = _e4519;
                                            }
                                            let _e4578 = phi_18556_;
                                            let _e4580 = phi_18557_;
                                            let _e4582 = phi_18558_;
                                            if _e4582 {
                                                phi_18562_ = _e4578;
                                            } else {
                                                phi_18562_ = _e4580;
                                            }
                                            let _e4584 = phi_18562_;
                                            if (_e4070 > 0f) {
                                                if (_e4070 >= 300000000000000000000000000000000000000f) {
                                                    phi_30073_ = 1f;
                                                } else {
                                                    let _e4589 = (_e4070 * _e4070);
                                                    let _e4591 = (_e4589 + (_e4584.member_1 * _e4584.member_1));
                                                    if (_e4591 == 0f) {
                                                        phi_30070_ = 0f;
                                                    } else {
                                                        phi_30070_ = (_e4589 / _e4591);
                                                    }
                                                    let _e4595 = phi_30070_;
                                                    phi_30073_ = _e4595;
                                                }
                                                let _e4597 = phi_30073_;
                                                phi_18571_ = (_e4597 / _e4070);
                                            } else {
                                                phi_18571_ = 1f;
                                            }
                                            let _e4600 = phi_18571_;
                                            if ((((_e4584.member.x * _e4584.member.x) + (_e4584.member.y * _e4584.member.y)) + (_e4584.member.z * _e4584.member.z)) > 0f) {
                                                let _e4611 = (vec3<f32>(1f, 1f, 1f) / _e4072);
                                                local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                                phi_18604_ = 4294967295u;
                                                phi_18607_ = select(4294967295u, 0u, (_e839 > 0u));
                                                phi_18609_ = 0u;
                                                phi_18611_ = _e843;
                                                phi_18613_ = _e839;
                                                phi_18615_ = vec4<f32>(0f, 0f, 0f, 0f);
                                                phi_18617_ = _e4076;
                                                loop {
                                                    let _e4615 = phi_18604_;
                                                    let _e4617 = phi_18607_;
                                                    let _e4619 = phi_18609_;
                                                    let _e4621 = phi_18611_;
                                                    let _e4623 = phi_18613_;
                                                    let _e4625 = phi_18615_;
                                                    let _e4627 = phi_18617_;
                                                    local_80 = _e4615;
                                                    local_81 = _e4625;
                                                    local_82 = _e4623;
                                                    if (_e4623 < _e4621) {
                                                        let _e4990 = (3u * _e4623);
                                                        let _e4992 = global_2.member[_e4990];
                                                        let _e4995 = global_2.member[(_e4990 + 1u)];
                                                        let _e5001 = select(select(u32(_e4995.w), 0u, (_e4995.w < 0f)), 4294967295u, (_e4995.w > 4294967000f));
                                                        if (_e5001 == 0u) {
                                                            let _e5095 = global_2.member[(_e4990 + 2u)];
                                                            let _e5104 = ((_e4072.y * _e5095.z) - (_e5095.y * _e4072.z));
                                                            let _e5107 = ((_e4072.z * _e5095.x) - (_e5095.z * _e4072.x));
                                                            let _e5110 = ((_e4072.x * _e5095.y) - (_e5095.x * _e4072.y));
                                                            let _e5115 = (((_e4995.x * _e5104) + (_e4995.y * _e5107)) + (_e4995.z * _e5110));
                                                            if (abs(_e5115) < 0.000000000001f) {
                                                                phi_18802_ = vec4<f32>();
                                                                phi_18803_ = true;
                                                            } else {
                                                                let _e5118 = (1f / _e5115);
                                                                let _e5122 = (_e1111 - _e4992.x);
                                                                let _e5123 = (_e1112 - _e4992.y);
                                                                let _e5124 = (_e1113 - _e4992.z);
                                                                let _e5130 = ((((_e5122 * _e5104) + (_e5123 * _e5107)) + (_e5124 * _e5110)) * _e5118);
                                                                if (_e5130 < 0f) {
                                                                    phi_18792_ = vec4<f32>();
                                                                    phi_18793_ = true;
                                                                    phi_18794_ = false;
                                                                } else {
                                                                    let _e5132 = (_e5130 > 1f);
                                                                    if _e5132 {
                                                                        phi_18790_ = vec4<f32>();
                                                                        phi_18791_ = false;
                                                                    } else {
                                                                        let _e5135 = ((_e5123 * _e4995.z) - (_e4995.y * _e5124));
                                                                        let _e5138 = ((_e5124 * _e4995.x) - (_e4995.z * _e5122));
                                                                        let _e5141 = ((_e5122 * _e4995.y) - (_e4995.x * _e5123));
                                                                        let _e5147 = ((((_e4072.x * _e5135) + (_e4072.y * _e5138)) + (_e4072.z * _e5141)) * _e5118);
                                                                        if (_e5147 < 0f) {
                                                                            phi_18780_ = vec4<f32>();
                                                                            phi_18781_ = true;
                                                                            phi_18782_ = false;
                                                                        } else {
                                                                            let _e5150 = ((_e5130 + _e5147) > 1f);
                                                                            if _e5150 {
                                                                                phi_18778_ = vec4<f32>();
                                                                                phi_18779_ = false;
                                                                            } else {
                                                                                let _e5156 = ((((_e5095.x * _e5135) + (_e5095.y * _e5138)) + (_e5095.z * _e5141)) * _e5118);
                                                                                if (_e5156 <= 0f) {
                                                                                    phi_18773_ = vec4<f32>();
                                                                                    phi_18774_ = true;
                                                                                } else {
                                                                                    let _e5158 = (_e5156 >= _e4627);
                                                                                    if _e5158 {
                                                                                        phi_18772_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_18772_ = vec4<f32>(_e5156, _e5130, _e5147, 1f);
                                                                                    }
                                                                                    let _e5161 = phi_18772_;
                                                                                    phi_18773_ = _e5161;
                                                                                    phi_18774_ = _e5158;
                                                                                }
                                                                                let _e5163 = phi_18773_;
                                                                                let _e5165 = phi_18774_;
                                                                                phi_18778_ = _e5163;
                                                                                phi_18779_ = _e5165;
                                                                            }
                                                                            let _e5167 = phi_18778_;
                                                                            let _e5169 = phi_18779_;
                                                                            phi_18780_ = _e5167;
                                                                            phi_18781_ = _e5150;
                                                                            phi_18782_ = _e5169;
                                                                        }
                                                                        let _e5171 = phi_18780_;
                                                                        let _e5173 = phi_18781_;
                                                                        let _e5175 = phi_18782_;
                                                                        phi_18790_ = _e5171;
                                                                        phi_18791_ = select(_e5175, true, _e5173);
                                                                    }
                                                                    let _e5178 = phi_18790_;
                                                                    let _e5180 = phi_18791_;
                                                                    phi_18792_ = _e5178;
                                                                    phi_18793_ = _e5132;
                                                                    phi_18794_ = _e5180;
                                                                }
                                                                let _e5182 = phi_18792_;
                                                                let _e5184 = phi_18793_;
                                                                let _e5186 = phi_18794_;
                                                                phi_18802_ = _e5182;
                                                                phi_18803_ = select(_e5186, true, _e5184);
                                                            }
                                                            let _e5189 = phi_18802_;
                                                            let _e5191 = phi_18803_;
                                                            phi_18948_ = _e5189;
                                                            phi_18949_ = _e5191;
                                                        } else {
                                                            if (_e5001 == 1u) {
                                                                let _e5025 = (_e1111 - _e4992.x);
                                                                let _e5026 = (_e1112 - _e4992.y);
                                                                let _e5027 = (_e1113 - _e4992.z);
                                                                let _e5032 = (((_e5025 * _e4072.x) + (_e5026 * _e4072.y)) + (_e5027 * _e4072.z));
                                                                if (_e5032 > 0f) {
                                                                    phi_18842_ = select(true, false, ((((_e5025 * _e5025) + (_e5026 * _e5026)) + (_e5027 * _e5027)) > (_e4992.w * _e4992.w)));
                                                                } else {
                                                                    phi_18842_ = true;
                                                                }
                                                                let _e5044 = phi_18842_;
                                                                if _e5044 {
                                                                    let _e5048 = (_e5025 - (_e4072.x * _e5032));
                                                                    let _e5049 = (_e5026 - (_e4072.y * _e5032));
                                                                    let _e5050 = (_e5027 - (_e4072.z * _e5032));
                                                                    let _e5058 = ((_e4992.w * _e4992.w) - (((_e5048 * _e5048) + (_e5049 * _e5049)) + (_e5050 * _e5050)));
                                                                    if (_e5058 < 0f) {
                                                                        phi_18901_ = vec4<f32>();
                                                                        phi_18902_ = true;
                                                                    } else {
                                                                        let _e5060 = sqrt(_e5058);
                                                                        let _e5061 = -(_e5032);
                                                                        let _e5062 = (_e5061 - _e5060);
                                                                        if (_e5062 <= 0f) {
                                                                            phi_18879_ = true;
                                                                        } else {
                                                                            phi_18879_ = (_e5062 >= _e4627);
                                                                        }
                                                                        let _e5066 = phi_18879_;
                                                                        if _e5066 {
                                                                            phi_18885_ = (_e5061 + _e5060);
                                                                        } else {
                                                                            phi_18885_ = _e5062;
                                                                        }
                                                                        let _e5069 = phi_18885_;
                                                                        if (_e5069 <= 0f) {
                                                                            phi_18896_ = vec4<f32>();
                                                                            phi_18897_ = true;
                                                                        } else {
                                                                            let _e5071 = (_e5069 >= _e4627);
                                                                            if _e5071 {
                                                                                phi_18895_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18895_ = vec4<f32>(_e5069, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5074 = phi_18895_;
                                                                            phi_18896_ = _e5074;
                                                                            phi_18897_ = _e5071;
                                                                        }
                                                                        let _e5076 = phi_18896_;
                                                                        let _e5078 = phi_18897_;
                                                                        phi_18901_ = _e5076;
                                                                        phi_18902_ = _e5078;
                                                                    }
                                                                    let _e5080 = phi_18901_;
                                                                    let _e5082 = phi_18902_;
                                                                    phi_18906_ = _e5080;
                                                                    phi_18907_ = _e5082;
                                                                } else {
                                                                    phi_18906_ = vec4<f32>();
                                                                    phi_18907_ = true;
                                                                }
                                                                let _e5084 = phi_18906_;
                                                                let _e5086 = phi_18907_;
                                                                phi_18938_ = _e5084;
                                                                phi_18939_ = _e5086;
                                                            } else {
                                                                if (abs(_e4072.y) < 0.000000000001f) {
                                                                    phi_18936_ = vec4<f32>();
                                                                    phi_18937_ = true;
                                                                } else {
                                                                    let _e5008 = ((_e4992.x - _e1112) / _e4072.y);
                                                                    if (_e5008 <= 0f) {
                                                                        phi_18931_ = vec4<f32>();
                                                                        phi_18932_ = true;
                                                                    } else {
                                                                        let _e5010 = (_e5008 >= _e4627);
                                                                        if _e5010 {
                                                                            phi_18930_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18930_ = vec4<f32>(_e5008, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5013 = phi_18930_;
                                                                        phi_18931_ = _e5013;
                                                                        phi_18932_ = _e5010;
                                                                    }
                                                                    let _e5015 = phi_18931_;
                                                                    let _e5017 = phi_18932_;
                                                                    phi_18936_ = _e5015;
                                                                    phi_18937_ = _e5017;
                                                                }
                                                                let _e5019 = phi_18936_;
                                                                let _e5021 = phi_18937_;
                                                                phi_18938_ = _e5019;
                                                                phi_18939_ = _e5021;
                                                            }
                                                            let _e5088 = phi_18938_;
                                                            let _e5090 = phi_18939_;
                                                            phi_18948_ = _e5088;
                                                            phi_18949_ = select(true, false, select(true, false, _e5090));
                                                        }
                                                        let _e5193 = phi_18948_;
                                                        let _e5195 = phi_18949_;
                                                        let _e5198 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5193, vec4(select(true, false, _e5195)));
                                                        let _e5200 = (_e5198.w > 0f);
                                                        if _e5200 {
                                                            phi_18970_ = _e5198.x;
                                                        } else {
                                                            phi_18970_ = _e4627;
                                                        }
                                                        let _e5203 = phi_18970_;
                                                        let _e5207 = select(true, false, _e5200);
                                                        if _e5207 {
                                                            phi_18976_ = (_e4623 + 1u);
                                                        } else {
                                                            phi_18976_ = u32();
                                                        }
                                                        let _e5210 = phi_18976_;
                                                        phi_18605_ = select(_e4615, _e4623, _e5200);
                                                        phi_18608_ = _e4617;
                                                        phi_18610_ = _e4619;
                                                        phi_18612_ = _e4621;
                                                        phi_18614_ = _e5210;
                                                        phi_18616_ = select(_e4625, _e5198, vec4(_e5200));
                                                        phi_18618_ = _e5203;
                                                        phi_19149_ = _e5198;
                                                        phi_19150_ = _e5207;
                                                        phi_19151_ = false;
                                                    } else {
                                                        if (_e4617 == 4294967295u) {
                                                            let _e4630 = (_e4619 == 0u);
                                                            if _e4630 {
                                                                phi_19006_ = u32();
                                                                phi_19007_ = u32();
                                                                phi_19009_ = false;
                                                                phi_19010_ = false;
                                                            } else {
                                                                let _e4631 = (_e4619 - 1u);
                                                                let _e4632 = (_e4631 < 64u);
                                                                if _e4632 {
                                                                } else {
                                                                    phi_36068_ = true;
                                                                    phi_21677_ = bool();
                                                                    phi_21676_ = vec4<f32>();
                                                                    break;
                                                                }
                                                                let _e4634 = local_68[_e4631];
                                                                let _e4635 = (_e4634 >= _e4627);
                                                                if _e4635 {
                                                                    phi_19004_ = u32();
                                                                } else {
                                                                    if _e4632 {
                                                                    } else {
                                                                        phi_36068_ = true;
                                                                        phi_21677_ = bool();
                                                                        phi_21676_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    let _e4637 = local_67[_e4631];
                                                                    phi_19004_ = _e4637;
                                                                }
                                                                let _e4639 = phi_19004_;
                                                                phi_19006_ = _e4631;
                                                                phi_19007_ = _e4639;
                                                                phi_19009_ = select(true, false, _e4635);
                                                                phi_19010_ = _e4635;
                                                            }
                                                            let _e4642 = phi_19006_;
                                                            let _e4644 = phi_19007_;
                                                            let _e4646 = phi_19009_;
                                                            let _e4648 = phi_19010_;
                                                            phi_19011_ = _e4642;
                                                            phi_19012_ = _e4644;
                                                            phi_19013_ = _e4642;
                                                            phi_19014_ = _e4646;
                                                            phi_19015_ = _e4648;
                                                            phi_19016_ = _e4630;
                                                        } else {
                                                            phi_19011_ = u32();
                                                            phi_19012_ = _e4617;
                                                            phi_19013_ = _e4619;
                                                            phi_19014_ = true;
                                                            phi_19015_ = false;
                                                            phi_19016_ = false;
                                                        }
                                                        let _e4650 = phi_19011_;
                                                        let _e4652 = phi_19012_;
                                                        let _e4654 = phi_19013_;
                                                        let _e4656 = phi_19014_;
                                                        let _e4658 = phi_19015_;
                                                        let _e4660 = phi_19016_;
                                                        if _e4656 {
                                                            let _e4662 = ((_e4652 & 2147483648u) == 0u);
                                                            if _e4662 {
                                                                let _e4670 = global_3.member[_e4652];
                                                                if (_e4670.member_1 == 4294967295u) {
                                                                    phi_19045_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4682 = ((_e4670.member.x - _e1111) * _e4611.x);
                                                                    let _e4684 = ((_e4670.member.y - _e1112) * _e4611.y);
                                                                    let _e4686 = ((_e4670.member.z - _e1113) * _e4611.z);
                                                                    let _e4693 = ((_e4670.member_2.x - _e1111) * _e4611.x);
                                                                    let _e4694 = ((_e4670.member_2.y - _e1112) * _e4611.y);
                                                                    let _e4695 = ((_e4670.member_2.z - _e1113) * _e4611.z);
                                                                    if (_e4682 == _e4682) {
                                                                        let _e4697 = (_e4693 == _e4693);
                                                                        if _e4697 {
                                                                            let _e4698 = (_e4682 != _e4682);
                                                                            if _e4698 {
                                                                                phi_30171_ = true;
                                                                            } else {
                                                                                phi_30171_ = (_e4693 <= _e4682);
                                                                            }
                                                                            let _e4701 = phi_30171_;
                                                                            let _e4702 = select(_e4682, _e4693, _e4701);
                                                                            if _e4698 {
                                                                                phi_30201_ = true;
                                                                            } else {
                                                                                phi_30201_ = (_e4693 >= _e4682);
                                                                            }
                                                                            let _e4707 = phi_30201_;
                                                                            let _e4708 = select(_e4682, _e4693, _e4707);
                                                                            phi_30148_ = type_56(select(-300000000000000000000000000000000000000f, _e4702, (_e4702 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4708, (_e4708 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30148_ = type_56();
                                                                        }
                                                                        let _e4713 = phi_30148_;
                                                                        phi_30151_ = _e4713;
                                                                        phi_30152_ = select(true, false, _e4697);
                                                                    } else {
                                                                        phi_30151_ = type_56();
                                                                        phi_30152_ = true;
                                                                    }
                                                                    let _e4716 = phi_30151_;
                                                                    let _e4718 = phi_30152_;
                                                                    if _e4718 {
                                                                        phi_30157_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30157_ = _e4716;
                                                                    }
                                                                    let _e4720 = phi_30157_;
                                                                    if (_e4684 == _e4684) {
                                                                        let _e4724 = (_e4694 == _e4694);
                                                                        if _e4724 {
                                                                            let _e4725 = (_e4684 != _e4684);
                                                                            if _e4725 {
                                                                                phi_30258_ = true;
                                                                            } else {
                                                                                phi_30258_ = (_e4694 <= _e4684);
                                                                            }
                                                                            let _e4728 = phi_30258_;
                                                                            let _e4729 = select(_e4684, _e4694, _e4728);
                                                                            if (_e4720.member != _e4720.member) {
                                                                                phi_30273_ = true;
                                                                            } else {
                                                                                phi_30273_ = (_e4729 >= _e4720.member);
                                                                            }
                                                                            let _e4733 = phi_30273_;
                                                                            if _e4725 {
                                                                                phi_30288_ = true;
                                                                            } else {
                                                                                phi_30288_ = (_e4694 >= _e4684);
                                                                            }
                                                                            let _e4737 = phi_30288_;
                                                                            let _e4738 = select(_e4684, _e4694, _e4737);
                                                                            if (_e4720.member_1 != _e4720.member_1) {
                                                                                phi_30303_ = true;
                                                                            } else {
                                                                                phi_30303_ = (_e4738 <= _e4720.member_1);
                                                                            }
                                                                            let _e4742 = phi_30303_;
                                                                            phi_30235_ = type_56(select(_e4720.member, _e4729, _e4733), select(_e4720.member_1, _e4738, _e4742));
                                                                        } else {
                                                                            phi_30235_ = type_56();
                                                                        }
                                                                        let _e4746 = phi_30235_;
                                                                        phi_30238_ = _e4746;
                                                                        phi_30239_ = select(true, false, _e4724);
                                                                    } else {
                                                                        phi_30238_ = type_56();
                                                                        phi_30239_ = true;
                                                                    }
                                                                    let _e4749 = phi_30238_;
                                                                    let _e4751 = phi_30239_;
                                                                    if _e4751 {
                                                                        phi_30244_ = _e4720;
                                                                    } else {
                                                                        phi_30244_ = _e4749;
                                                                    }
                                                                    let _e4753 = phi_30244_;
                                                                    if (_e4686 == _e4686) {
                                                                        let _e4757 = (_e4695 == _e4695);
                                                                        if _e4757 {
                                                                            let _e4758 = (_e4686 != _e4686);
                                                                            if _e4758 {
                                                                                phi_30345_ = true;
                                                                            } else {
                                                                                phi_30345_ = (_e4695 <= _e4686);
                                                                            }
                                                                            let _e4761 = phi_30345_;
                                                                            let _e4762 = select(_e4686, _e4695, _e4761);
                                                                            if (_e4753.member != _e4753.member) {
                                                                                phi_30360_ = true;
                                                                            } else {
                                                                                phi_30360_ = (_e4762 >= _e4753.member);
                                                                            }
                                                                            let _e4766 = phi_30360_;
                                                                            if _e4758 {
                                                                                phi_30375_ = true;
                                                                            } else {
                                                                                phi_30375_ = (_e4695 >= _e4686);
                                                                            }
                                                                            let _e4770 = phi_30375_;
                                                                            let _e4771 = select(_e4686, _e4695, _e4770);
                                                                            if (_e4753.member_1 != _e4753.member_1) {
                                                                                phi_30390_ = true;
                                                                            } else {
                                                                                phi_30390_ = (_e4771 <= _e4753.member_1);
                                                                            }
                                                                            let _e4775 = phi_30390_;
                                                                            phi_30322_ = type_56(select(_e4753.member, _e4762, _e4766), select(_e4753.member_1, _e4771, _e4775));
                                                                        } else {
                                                                            phi_30322_ = type_56();
                                                                        }
                                                                        let _e4779 = phi_30322_;
                                                                        phi_30325_ = _e4779;
                                                                        phi_30326_ = select(true, false, _e4757);
                                                                    } else {
                                                                        phi_30325_ = type_56();
                                                                        phi_30326_ = true;
                                                                    }
                                                                    let _e4782 = phi_30325_;
                                                                    let _e4784 = phi_30326_;
                                                                    if _e4784 {
                                                                        phi_30331_ = _e4753;
                                                                    } else {
                                                                        phi_30331_ = _e4782;
                                                                    }
                                                                    let _e4786 = phi_30331_;
                                                                    if (_e4786.member_1 < 0f) {
                                                                        phi_30129_ = true;
                                                                    } else {
                                                                        if (_e4786.member > _e4786.member_1) {
                                                                            phi_30126_ = true;
                                                                        } else {
                                                                            phi_30126_ = (_e4786.member > _e4627);
                                                                        }
                                                                        let _e4793 = phi_30126_;
                                                                        phi_30129_ = _e4793;
                                                                    }
                                                                    let _e4795 = phi_30129_;
                                                                    phi_19045_ = select(_e4786.member, 300000000000000000000000000000000000000f, _e4795);
                                                                }
                                                                let _e4798 = phi_19045_;
                                                                if (_e4670.member_5 == 4294967295u) {
                                                                    phi_19054_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4810 = ((_e4670.member_4.x - _e1111) * _e4611.x);
                                                                    let _e4812 = ((_e4670.member_4.y - _e1112) * _e4611.y);
                                                                    let _e4814 = ((_e4670.member_4.z - _e1113) * _e4611.z);
                                                                    let _e4821 = ((_e4670.member_6.x - _e1111) * _e4611.x);
                                                                    let _e4822 = ((_e4670.member_6.y - _e1112) * _e4611.y);
                                                                    let _e4823 = ((_e4670.member_6.z - _e1113) * _e4611.z);
                                                                    if (_e4810 == _e4810) {
                                                                        let _e4825 = (_e4821 == _e4821);
                                                                        if _e4825 {
                                                                            let _e4826 = (_e4810 != _e4810);
                                                                            if _e4826 {
                                                                                phi_30492_ = true;
                                                                            } else {
                                                                                phi_30492_ = (_e4821 <= _e4810);
                                                                            }
                                                                            let _e4829 = phi_30492_;
                                                                            let _e4830 = select(_e4810, _e4821, _e4829);
                                                                            if _e4826 {
                                                                                phi_30522_ = true;
                                                                            } else {
                                                                                phi_30522_ = (_e4821 >= _e4810);
                                                                            }
                                                                            let _e4835 = phi_30522_;
                                                                            let _e4836 = select(_e4810, _e4821, _e4835);
                                                                            phi_30469_ = type_56(select(-300000000000000000000000000000000000000f, _e4830, (_e4830 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4836, (_e4836 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30469_ = type_56();
                                                                        }
                                                                        let _e4841 = phi_30469_;
                                                                        phi_30472_ = _e4841;
                                                                        phi_30473_ = select(true, false, _e4825);
                                                                    } else {
                                                                        phi_30472_ = type_56();
                                                                        phi_30473_ = true;
                                                                    }
                                                                    let _e4844 = phi_30472_;
                                                                    let _e4846 = phi_30473_;
                                                                    if _e4846 {
                                                                        phi_30478_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30478_ = _e4844;
                                                                    }
                                                                    let _e4848 = phi_30478_;
                                                                    if (_e4812 == _e4812) {
                                                                        let _e4852 = (_e4822 == _e4822);
                                                                        if _e4852 {
                                                                            let _e4853 = (_e4812 != _e4812);
                                                                            if _e4853 {
                                                                                phi_30579_ = true;
                                                                            } else {
                                                                                phi_30579_ = (_e4822 <= _e4812);
                                                                            }
                                                                            let _e4856 = phi_30579_;
                                                                            let _e4857 = select(_e4812, _e4822, _e4856);
                                                                            if (_e4848.member != _e4848.member) {
                                                                                phi_30594_ = true;
                                                                            } else {
                                                                                phi_30594_ = (_e4857 >= _e4848.member);
                                                                            }
                                                                            let _e4861 = phi_30594_;
                                                                            if _e4853 {
                                                                                phi_30609_ = true;
                                                                            } else {
                                                                                phi_30609_ = (_e4822 >= _e4812);
                                                                            }
                                                                            let _e4865 = phi_30609_;
                                                                            let _e4866 = select(_e4812, _e4822, _e4865);
                                                                            if (_e4848.member_1 != _e4848.member_1) {
                                                                                phi_30624_ = true;
                                                                            } else {
                                                                                phi_30624_ = (_e4866 <= _e4848.member_1);
                                                                            }
                                                                            let _e4870 = phi_30624_;
                                                                            phi_30556_ = type_56(select(_e4848.member, _e4857, _e4861), select(_e4848.member_1, _e4866, _e4870));
                                                                        } else {
                                                                            phi_30556_ = type_56();
                                                                        }
                                                                        let _e4874 = phi_30556_;
                                                                        phi_30559_ = _e4874;
                                                                        phi_30560_ = select(true, false, _e4852);
                                                                    } else {
                                                                        phi_30559_ = type_56();
                                                                        phi_30560_ = true;
                                                                    }
                                                                    let _e4877 = phi_30559_;
                                                                    let _e4879 = phi_30560_;
                                                                    if _e4879 {
                                                                        phi_30565_ = _e4848;
                                                                    } else {
                                                                        phi_30565_ = _e4877;
                                                                    }
                                                                    let _e4881 = phi_30565_;
                                                                    if (_e4814 == _e4814) {
                                                                        let _e4885 = (_e4823 == _e4823);
                                                                        if _e4885 {
                                                                            let _e4886 = (_e4814 != _e4814);
                                                                            if _e4886 {
                                                                                phi_30666_ = true;
                                                                            } else {
                                                                                phi_30666_ = (_e4823 <= _e4814);
                                                                            }
                                                                            let _e4889 = phi_30666_;
                                                                            let _e4890 = select(_e4814, _e4823, _e4889);
                                                                            if (_e4881.member != _e4881.member) {
                                                                                phi_30681_ = true;
                                                                            } else {
                                                                                phi_30681_ = (_e4890 >= _e4881.member);
                                                                            }
                                                                            let _e4894 = phi_30681_;
                                                                            if _e4886 {
                                                                                phi_30696_ = true;
                                                                            } else {
                                                                                phi_30696_ = (_e4823 >= _e4814);
                                                                            }
                                                                            let _e4898 = phi_30696_;
                                                                            let _e4899 = select(_e4814, _e4823, _e4898);
                                                                            if (_e4881.member_1 != _e4881.member_1) {
                                                                                phi_30711_ = true;
                                                                            } else {
                                                                                phi_30711_ = (_e4899 <= _e4881.member_1);
                                                                            }
                                                                            let _e4903 = phi_30711_;
                                                                            phi_30643_ = type_56(select(_e4881.member, _e4890, _e4894), select(_e4881.member_1, _e4899, _e4903));
                                                                        } else {
                                                                            phi_30643_ = type_56();
                                                                        }
                                                                        let _e4907 = phi_30643_;
                                                                        phi_30646_ = _e4907;
                                                                        phi_30647_ = select(true, false, _e4885);
                                                                    } else {
                                                                        phi_30646_ = type_56();
                                                                        phi_30647_ = true;
                                                                    }
                                                                    let _e4910 = phi_30646_;
                                                                    let _e4912 = phi_30647_;
                                                                    if _e4912 {
                                                                        phi_30652_ = _e4881;
                                                                    } else {
                                                                        phi_30652_ = _e4910;
                                                                    }
                                                                    let _e4914 = phi_30652_;
                                                                    if (_e4914.member_1 < 0f) {
                                                                        phi_30450_ = true;
                                                                    } else {
                                                                        if (_e4914.member > _e4914.member_1) {
                                                                            phi_30447_ = true;
                                                                        } else {
                                                                            phi_30447_ = (_e4914.member > _e4627);
                                                                        }
                                                                        let _e4921 = phi_30447_;
                                                                        phi_30450_ = _e4921;
                                                                    }
                                                                    let _e4923 = phi_30450_;
                                                                    phi_19054_ = select(_e4914.member, 300000000000000000000000000000000000000f, _e4923);
                                                                }
                                                                let _e4926 = phi_19054_;
                                                                if (_e4670.member_3 > 0u) {
                                                                    phi_19065_ = ((2147483648u | ((_e4670.member_3 - 1u) << bitcast<u32>(28u))) | _e4670.member_1);
                                                                } else {
                                                                    phi_19065_ = _e4670.member_1;
                                                                }
                                                                let _e4935 = phi_19065_;
                                                                if (_e4670.member_7 > 0u) {
                                                                    phi_19076_ = ((2147483648u | ((_e4670.member_7 - 1u) << bitcast<u32>(28u))) | _e4670.member_5);
                                                                } else {
                                                                    phi_19076_ = _e4670.member_5;
                                                                }
                                                                let _e4944 = phi_19076_;
                                                                let _e4945 = (_e4798 <= _e4926);
                                                                let _e4949 = select(_e4798, _e4926, _e4945);
                                                                if (_e4949 < 300000000000000000000000000000000000000f) {
                                                                    let _e4951 = (_e4654 < 64u);
                                                                    if _e4951 {
                                                                        if _e4951 {
                                                                        } else {
                                                                            phi_36068_ = true;
                                                                            phi_21677_ = bool();
                                                                            phi_21676_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_67[_e4654] = select(_e4935, _e4944, _e4945);
                                                                        if _e4951 {
                                                                        } else {
                                                                            phi_36068_ = true;
                                                                            phi_21677_ = bool();
                                                                            phi_21676_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_68[_e4654] = _e4949;
                                                                        phi_19115_ = (_e4654 + 1u);
                                                                    } else {
                                                                        phi_19115_ = _e4654;
                                                                    }
                                                                    let _e4956 = phi_19115_;
                                                                    phi_19116_ = _e4956;
                                                                } else {
                                                                    phi_19116_ = _e4654;
                                                                }
                                                                let _e4958 = phi_19116_;
                                                                phi_19127_ = select(4294967295u, select(_e4944, _e4935, _e4945), (select(_e4926, _e4798, _e4945) < 300000000000000000000000000000000000000f));
                                                                phi_19128_ = _e4958;
                                                                phi_19129_ = u32();
                                                                phi_19130_ = u32();
                                                            } else {
                                                                let _e4663 = (_e4652 & 268435455u);
                                                                phi_19127_ = u32();
                                                                phi_19128_ = u32();
                                                                phi_19129_ = ((_e4663 + ((_e4652 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                                phi_19130_ = _e4663;
                                                            }
                                                            let _e4962 = phi_19127_;
                                                            let _e4964 = phi_19128_;
                                                            let _e4966 = phi_19129_;
                                                            let _e4968 = phi_19130_;
                                                            phi_19132_ = _e4962;
                                                            phi_19133_ = _e4964;
                                                            phi_19135_ = _e4966;
                                                            phi_19136_ = _e4968;
                                                            phi_19137_ = select(true, false, _e4662);
                                                            phi_19138_ = _e4662;
                                                        } else {
                                                            phi_19132_ = u32();
                                                            phi_19133_ = u32();
                                                            phi_19135_ = _e4621;
                                                            phi_19136_ = _e4623;
                                                            phi_19137_ = _e4658;
                                                            phi_19138_ = false;
                                                        }
                                                        let _e4971 = phi_19132_;
                                                        let _e4973 = phi_19133_;
                                                        let _e4975 = phi_19135_;
                                                        let _e4977 = phi_19136_;
                                                        let _e4979 = phi_19137_;
                                                        let _e4981 = phi_19138_;
                                                        phi_18605_ = _e4615;
                                                        phi_18608_ = select(_e4971, 4294967295u, _e4979);
                                                        phi_18610_ = select(_e4973, select(_e4650, _e4654, _e4656), _e4979);
                                                        phi_18612_ = select(_e4621, _e4975, _e4979);
                                                        phi_18614_ = select(_e4623, _e4977, _e4979);
                                                        phi_18616_ = _e4625;
                                                        phi_18618_ = _e4627;
                                                        phi_19149_ = vec4<f32>();
                                                        phi_19150_ = select(_e4981, true, _e4979);
                                                        phi_19151_ = select(select(_e4660, false, _e4656), false, _e4979);
                                                    }
                                                    let _e5212 = phi_18605_;
                                                    let _e5214 = phi_18608_;
                                                    let _e5216 = phi_18610_;
                                                    let _e5218 = phi_18612_;
                                                    let _e5220 = phi_18614_;
                                                    let _e5222 = phi_18616_;
                                                    let _e5224 = phi_18618_;
                                                    let _e5226 = phi_19149_;
                                                    let _e5228 = phi_19150_;
                                                    let _e5230 = phi_19151_;
                                                    phi_18604_ = _e5212;
                                                    phi_18607_ = _e5214;
                                                    phi_18609_ = _e5216;
                                                    phi_18611_ = _e5218;
                                                    phi_18613_ = _e5220;
                                                    phi_18615_ = _e5222;
                                                    phi_18617_ = _e5224;
                                                    phi_36068_ = _e1992;
                                                    phi_21677_ = _e5230;
                                                    phi_21676_ = _e5226;
                                                    if !(_e5228) {
                                                        break;
                                                    }
                                                }
                                                let _e5233 = phi_36068_;
                                                let _e5235 = phi_21677_;
                                                let _e5237 = phi_21676_;
                                                phi_36515_ = _e4068;
                                                phi_36507_ = _e5233;
                                                if _e5233 {
                                                    break;
                                                }
                                                if _e5235 {
                                                    let _e5239 = local_80;
                                                    let _e5241 = local_81;
                                                    phi_19156_ = type_74(_e5239, _e5241);
                                                } else {
                                                    phi_19156_ = type_74();
                                                }
                                                let _e5244 = phi_19156_;
                                                if select(true, false, _e5235) {
                                                    let _e5247 = local_82;
                                                    phi_19162_ = type_74(_e5247, _e5237);
                                                } else {
                                                    phi_19162_ = _e5244;
                                                }
                                                let _e5250 = phi_19162_;
                                                if (_e5250.member != 4294967295u) {
                                                    phi_19216_ = _e1996;
                                                    phi_19217_ = _e1998;
                                                } else {
                                                    if (_e2000 == (_e1990 + 2u)) {
                                                        phi_19214_ = _e1996;
                                                        phi_19215_ = type_41(vec4<f32>(_e1111, _e1112, _e1113, -1f), vec4<f32>(_e4072.x, _e4072.y, _e4072.z, 0f), vec4<f32>(((_e828.x * _e4584.member.x) * _e4600), ((_e828.y * _e4584.member.y) * _e4600), ((_e828.z * _e4584.member.z) * _e4600), 0f));
                                                    } else {
                                                        phi_19214_ = vec3<f32>((_e1996.x + (((_e828.x * _e4584.member.x) * _e4074.x) * _e4600)), (_e1996.y + (((_e828.y * _e4584.member.y) * _e4074.y) * _e4600)), (_e1996.z + (((_e828.z * _e4584.member.z) * _e4074.z) * _e4600)));
                                                        phi_19215_ = _e1998;
                                                    }
                                                    let _e5288 = phi_19214_;
                                                    let _e5290 = phi_19215_;
                                                    phi_19216_ = _e5288;
                                                    phi_19217_ = _e5290;
                                                }
                                                let _e5292 = phi_19216_;
                                                let _e5294 = phi_19217_;
                                                phi_36510_ = _e5233;
                                                phi_19218_ = _e5292;
                                                phi_19219_ = _e5294;
                                                phi_19220_ = (_e2002 + 1u);
                                            } else {
                                                phi_36510_ = _e1992;
                                                phi_19218_ = _e1996;
                                                phi_19219_ = _e1998;
                                                phi_19220_ = _e2002;
                                            }
                                            let _e5296 = phi_36510_;
                                            let _e5298 = phi_19218_;
                                            let _e5300 = phi_19219_;
                                            let _e5302 = phi_19220_;
                                            phi_36509_ = _e5296;
                                            phi_19221_ = _e5298;
                                            phi_19222_ = _e5300;
                                            phi_19223_ = _e5302;
                                        } else {
                                            phi_36509_ = _e1992;
                                            phi_19221_ = _e1996;
                                            phi_19222_ = _e1998;
                                            phi_19223_ = _e2002;
                                        }
                                        let _e5304 = phi_36509_;
                                        let _e5306 = phi_19221_;
                                        let _e5308 = phi_19222_;
                                        let _e5310 = phi_19223_;
                                        phi_36795_ = _e4068;
                                        phi_36508_ = _e5304;
                                        phi_14641_ = _e5306;
                                        phi_14644_ = _e5308;
                                        phi_14646_ = (_e2000 + 1u);
                                        phi_14648_ = _e5310;
                                    } else {
                                        phi_36795_ = _e1994;
                                        phi_36508_ = _e1992;
                                        phi_14641_ = vec3<f32>();
                                        phi_14644_ = type_41();
                                        phi_14646_ = u32();
                                        phi_14648_ = u32();
                                    }
                                    let _e5313 = phi_36795_;
                                    let _e5315 = phi_36508_;
                                    let _e5317 = phi_14641_;
                                    let _e5319 = phi_14644_;
                                    let _e5321 = phi_14646_;
                                    let _e5323 = phi_14648_;
                                    phi_36140_ = _e5315;
                                    phi_34215_ = _e5313;
                                    phi_14640_ = _e5317;
                                    phi_14643_ = _e5319;
                                    phi_14645_ = _e5321;
                                    phi_14647_ = _e5323;
                                    phi_36515_ = _e5313;
                                    phi_36507_ = _e5315;
                                    if !(_e2004) {
                                        break;
                                    }
                                }
                                let _e5326 = phi_36515_;
                                let _e5328 = phi_36507_;
                                if _e5328 {
                                    break;
                                }
                                if (_e362 >= _e381) {
                                    let _e6332 = local_98;
                                    let _e6335 = local_99;
                                    let _e6338 = local_100;
                                    let _e6341 = local_101;
                                    let _e6347 = local_102;
                                    phi_19774_ = type_69(type_34(_e337.member, _e337.member_1, vec4<f32>(_e828.x, _e828.y, _e828.z, f32(_e362)), vec4<f32>(_e6335.x, _e6338.y, _e6341.z, f32(_e6332)), vec4<u32>(_e5326, (_e1987 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, _e6347);
                                    phi_19775_ = type_69();
                                    phi_19776_ = true;
                                } else {
                                    let _e5331 = ((_e5326 * 747796405u) + 2891336453u);
                                    let _e5339 = (((_e5331 >> bitcast<u32>((((_e5331 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5331) * 277803737u);
                                    let _e5342 = ((_e5339 >> bitcast<u32>(22i)) ^ _e5339);
                                    let _e5345 = f32((_e5342 >> bitcast<u32>(8i)));
                                    let _e5348 = ((_e5342 * 747796405u) + 2891336453u);
                                    let _e5356 = (((_e5348 >> bitcast<u32>((((_e5348 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5348) * 277803737u);
                                    let _e5359 = ((_e5356 >> bitcast<u32>(22i)) ^ _e5356);
                                    let _e5362 = f32((_e5359 >> bitcast<u32>(8i)));
                                    let _e5364 = ((_e5359 * 747796405u) + 2891336453u);
                                    let _e5372 = (((_e5364 >> bitcast<u32>((((_e5364 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5364) * 277803737u);
                                    let _e5375 = ((_e5372 >> bitcast<u32>(22i)) ^ _e5372);
                                    let _e5380 = (_e1979 == 0u);
                                    if _e5380 {
                                        phi_30821_ = vec3<f32>();
                                        phi_30822_ = true;
                                    } else {
                                        if (_e1979 == 2u) {
                                            let _e5386 = (((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z));
                                            if (_e5386 != _e5386) {
                                                phi_30862_ = true;
                                            } else {
                                                phi_30862_ = (0f >= _e5386);
                                            }
                                            let _e5390 = phi_30862_;
                                            let _e5391 = select(_e5386, 0f, _e5390);
                                            let _e5392 = (1f / _e1969);
                                            let _e5396 = ((_e5392 * _e5392) * (1f - (_e5391 * _e5391)));
                                            if (_e5396 >= 1f) {
                                                phi_30881_ = 1f;
                                            } else {
                                                let _e5399 = sqrt((1f - _e5396));
                                                if (_e5391 != _e5391) {
                                                    phi_30892_ = true;
                                                } else {
                                                    phi_30892_ = (_e5399 <= _e5391);
                                                }
                                                let _e5403 = phi_30892_;
                                                let _e5408 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                phi_30881_ = (_e5408 + ((1f - _e5408) * pow((1f - select(_e5391, _e5399, _e5403)), 5f)));
                                            }
                                            let _e5415 = phi_30881_;
                                            phi_30812_ = select(true, false, ((f32((_e5375 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5415));
                                        } else {
                                            phi_30812_ = true;
                                        }
                                        let _e5419 = phi_30812_;
                                        if _e5419 {
                                            let _e5424 = (((_e1060 * _e1134) + (_e1061 * _e1135)) + (_e1062 * _e1137));
                                            let _e5429 = (((_e1060 * _e1130) + (_e1061 * _e1140)) + (_e1062 * _e1141));
                                            let _e5434 = (((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z));
                                            if (_e5434 <= 0f) {
                                                phi_31046_ = vec3<f32>();
                                                phi_31047_ = true;
                                            } else {
                                                let _e5437 = (_e1977 * _e5424);
                                                let _e5438 = (_e1977 * _e5429);
                                                let _e5445 = (1f / sqrt((((_e5437 * _e5437) + (_e5438 * _e5438)) + (_e5434 * _e5434))));
                                                let _e5446 = (_e5437 * _e5445);
                                                let _e5447 = (_e5438 * _e5445);
                                                let _e5448 = (_e5434 * _e5445);
                                                let _e5451 = ((_e5446 * _e5446) + (_e5447 * _e5447));
                                                if (_e5451 > 0f) {
                                                    let _e5454 = sqrt(_e5451);
                                                    phi_30948_ = vec3<f32>((-(_e5447) / _e5454), (_e5446 / _e5454), 0f);
                                                } else {
                                                    phi_30948_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e5459 = phi_30948_;
                                                let _e5472 = sqrt((_e5345 * 0.000000059604645f));
                                                let _e5473 = (_e5362 * 0.00000037450704f);
                                                let _e5475 = (_e5472 * cos(_e5473));
                                                let _e5477 = (0.5f * (1f + _e5448));
                                                let _e5480 = (1f - (_e5475 * _e5475));
                                                if (_e5480 != _e5480) {
                                                    phi_31104_ = true;
                                                } else {
                                                    phi_31104_ = (0f >= _e5480);
                                                }
                                                let _e5484 = phi_31104_;
                                                let _e5491 = (((1f - _e5477) * sqrt(select(_e5480, 0f, _e5484))) + ((_e5477 * _e5472) * sin(_e5473)));
                                                let _e5502 = (_e5480 - (_e5491 * _e5491));
                                                if (_e5502 != _e5502) {
                                                    phi_31119_ = true;
                                                } else {
                                                    phi_31119_ = (0f >= _e5502);
                                                }
                                                let _e5506 = phi_31119_;
                                                let _e5508 = sqrt(select(_e5502, 0f, _e5506));
                                                let _e5514 = (((_e5459.z * _e5475) + (((_e5446 * _e5459.y) - (_e5459.x * _e5447)) * _e5491)) + (_e5448 * _e5508));
                                                let _e5515 = (_e1977 * (((_e5459.x * _e5475) + (((_e5447 * _e5459.z) - (_e5459.y * _e5448)) * _e5491)) + (_e5446 * _e5508)));
                                                let _e5516 = (_e1977 * (((_e5459.y * _e5475) + (((_e5448 * _e5459.x) - (_e5459.z * _e5446)) * _e5491)) + (_e5447 * _e5508)));
                                                if (_e5514 != _e5514) {
                                                    phi_31134_ = true;
                                                } else {
                                                    phi_31134_ = (0f >= _e5514);
                                                }
                                                let _e5520 = phi_31134_;
                                                let _e5521 = select(_e5514, 0f, _e5520);
                                                let _e5528 = (1f / sqrt((((_e5515 * _e5515) + (_e5516 * _e5516)) + (_e5521 * _e5521))));
                                                let _e5529 = (_e5515 * _e5528);
                                                let _e5530 = (_e5516 * _e5528);
                                                let _e5531 = (_e5521 * _e5528);
                                                let _e5537 = (2f * (((_e5424 * _e5529) + (_e5429 * _e5530)) + (_e5434 * _e5531)));
                                                let _e5540 = (_e5531 * _e5537);
                                                let _e5543 = (vec3<f32>((_e5529 * _e5537), (_e5530 * _e5537), _e5540) - vec3<f32>(_e5424, _e5429, _e5434));
                                                let _e5544 = ((_e5540 - _e5434) <= 0f);
                                                if _e5544 {
                                                    phi_31043_ = vec3<f32>();
                                                } else {
                                                    phi_31043_ = vec3<f32>((((_e1134 * _e5543.x) + (_e1130 * _e5543.y)) + (_e1086.x * _e5543.z)), (((_e1135 * _e5543.x) + (_e1140 * _e5543.y)) + (_e1086.y * _e5543.z)), (((_e1137 * _e5543.x) + (_e1141 * _e5543.y)) + (_e1086.z * _e5543.z)));
                                                }
                                                let _e5565 = phi_31043_;
                                                phi_31046_ = _e5565;
                                                phi_31047_ = _e5544;
                                            }
                                            let _e5567 = phi_31046_;
                                            let _e5569 = phi_31047_;
                                            phi_30817_ = select(_e5567, vec3<f32>(0f, 0f, 0f), vec3(_e5569));
                                        } else {
                                            phi_30817_ = vec3<f32>();
                                        }
                                        let _e5573 = phi_30817_;
                                        phi_30821_ = _e5573;
                                        phi_30822_ = select(true, false, _e5419);
                                    }
                                    let _e5576 = phi_30821_;
                                    let _e5578 = phi_30822_;
                                    if _e5578 {
                                        let _e5579 = (_e5345 * 0.00000037450704f);
                                        let _e5581 = ((_e5362 * 0.00000011920929f) - 1f);
                                        let _e5583 = (1f - (_e5581 * _e5581));
                                        if (_e5583 != _e5583) {
                                            phi_31234_ = true;
                                        } else {
                                            phi_31234_ = (0f >= _e5583);
                                        }
                                        let _e5587 = phi_31234_;
                                        let _e5589 = sqrt(select(_e5583, 0f, _e5587));
                                        let _e5594 = (_e1086.x + (_e5589 * cos(_e5579)));
                                        let _e5595 = (_e1086.y + (_e5589 * sin(_e5579)));
                                        let _e5596 = (_e1086.z + _e5581);
                                        let _e5602 = sqrt((((_e5594 * _e5594) + (_e5595 * _e5595)) + (_e5596 * _e5596)));
                                        if (_e5602 < 0.000001f) {
                                            phi_31208_ = _e1086;
                                        } else {
                                            let _e5604 = (1f / _e5602);
                                            phi_31208_ = vec3<f32>((_e5594 * _e5604), (_e5595 * _e5604), (_e5596 * _e5604));
                                        }
                                        let _e5610 = phi_31208_;
                                        phi_30829_ = _e5610;
                                    } else {
                                        phi_30829_ = _e5576;
                                    }
                                    let _e5612 = phi_30829_;
                                    let _e5620 = (((_e5612.x * _e1086.x) + (_e5612.y * _e1086.y)) + (_e5612.z * _e1086.z));
                                    if (_e5620 != _e5620) {
                                        phi_31249_ = true;
                                    } else {
                                        phi_31249_ = (0f >= _e5620);
                                    }
                                    let _e5624 = phi_31249_;
                                    let _e5626 = (select(_e5620, 0f, _e5624) * 0.31830987f);
                                    if _e5380 {
                                        if (_e5620 <= 0f) {
                                            phi_32046_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_32046_ = (_e1973 * (_e5620 * 0.31830987f));
                                        }
                                        let _e6101 = phi_32046_;
                                        phi_19615_ = type_59(_e6101, _e5626);
                                        phi_19616_ = type_59();
                                        phi_19617_ = true;
                                    } else {
                                        let _e5631 = (((_e1060 * _e1134) + (_e1061 * _e1135)) + (_e1062 * _e1137));
                                        let _e5636 = (((_e1060 * _e1130) + (_e1061 * _e1140)) + (_e1062 * _e1141));
                                        let _e5641 = (((_e1060 * _e1086.x) + (_e1061 * _e1086.y)) + (_e1062 * _e1086.z));
                                        let _e5646 = (((_e5612.x * _e1134) + (_e5612.y * _e1135)) + (_e5612.z * _e1137));
                                        let _e5651 = (((_e5612.x * _e1130) + (_e5612.y * _e1140)) + (_e5612.z * _e1141));
                                        if (_e5641 <= 0f) {
                                            phi_19579_ = type_59();
                                            phi_19580_ = true;
                                        } else {
                                            let _e5653 = (_e5620 <= 0f);
                                            if _e5653 {
                                                phi_19578_ = type_59();
                                            } else {
                                                let _e5654 = (_e5631 + _e5646);
                                                let _e5655 = (_e5636 + _e5651);
                                                let _e5656 = (_e5641 + _e5620);
                                                let _e5663 = (1f / sqrt((((_e5654 * _e5654) + (_e5655 * _e5655)) + (_e5656 * _e5656))));
                                                let _e5666 = (_e5656 * _e5663);
                                                if (_e5666 <= 0f) {
                                                    phi_31359_ = 0f;
                                                } else {
                                                    let _e5668 = (_e1977 * _e1977);
                                                    let _e5672 = ((((_e5668 - 1f) * _e5666) * _e5666) + 1f);
                                                    phi_31359_ = (_e5668 / ((3.1415927f * _e5672) * _e5672));
                                                }
                                                let _e5677 = phi_31359_;
                                                let _e5678 = (_e5641 * _e5641);
                                                if (_e5678 >= 1f) {
                                                    phi_31387_ = 0f;
                                                } else {
                                                    phi_31387_ = ((sqrt((1f + (((_e1977 * _e1977) * ((_e5631 * _e5631) + (_e5636 * _e5636))) / _e5678))) - 1f) * 0.5f);
                                                }
                                                let _e5691 = phi_31387_;
                                                let _e5692 = (1f + _e5691);
                                                let _e5693 = (_e5620 * _e5620);
                                                if (_e5693 >= 1f) {
                                                    phi_31415_ = 0f;
                                                } else {
                                                    phi_31415_ = ((sqrt((1f + (((_e1977 * _e1977) * ((_e5646 * _e5646) + (_e5651 * _e5651))) / _e5693))) - 1f) * 0.5f);
                                                }
                                                let _e5706 = phi_31415_;
                                                let _e5713 = (((_e5631 * (_e5654 * _e5663)) + (_e5636 * (_e5655 * _e5663))) + (_e5641 * _e5666));
                                                if (_e1975 == 0u) {
                                                    let _e6008 = (1f - _e5713);
                                                    if (_e6008 != _e6008) {
                                                        phi_31918_ = true;
                                                    } else {
                                                        phi_31918_ = (0f >= _e6008);
                                                    }
                                                    let _e6012 = phi_31918_;
                                                    let _e6014 = pow(select(_e6008, 0f, _e6012), 5f);
                                                    let _e6015 = (1f - _e6014);
                                                    phi_19566_ = vec3<f32>(((_e1973.x * _e6015) + _e6014), ((_e1973.y * _e6015) + _e6014), ((_e1973.z * _e6015) + _e6014));
                                                } else {
                                                    if (_e1975 == 1u) {
                                                        let _e5981 = (1f / _e1969);
                                                        let _e5985 = ((_e5981 * _e5981) * (1f - (_e5713 * _e5713)));
                                                        if (_e5985 >= 1f) {
                                                            phi_31879_ = 1f;
                                                        } else {
                                                            let _e5988 = sqrt((1f - _e5985));
                                                            if (_e5713 != _e5713) {
                                                                phi_31890_ = true;
                                                            } else {
                                                                phi_31890_ = (_e5988 <= _e5713);
                                                            }
                                                            let _e5992 = phi_31890_;
                                                            let _e5997 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                            phi_31879_ = (_e5997 + ((1f - _e5997) * pow((1f - select(_e5713, _e5988, _e5992)), 5f)));
                                                        }
                                                        let _e6004 = phi_31879_;
                                                        phi_19565_ = vec3(_e6004);
                                                    } else {
                                                        let _e5717 = select(_e5713, 0f, (_e5713 < 0f));
                                                        let _e5719 = select(_e5717, 1f, (_e5717 > 1f));
                                                        let _e5721 = (1f - (_e5719 * _e5719));
                                                        let _e5734 = ((_e1973.x * _e1973.x) - (_e1971.x * _e1971.x));
                                                        let _e5735 = ((_e1973.y * _e1973.y) - (_e1971.y * _e1971.y));
                                                        let _e5736 = ((_e1973.z * _e1973.z) - (_e1971.z * _e1971.z));
                                                        let _e5738 = (_e1973.x * (_e1971.x + _e1971.x));
                                                        let _e5740 = (_e1973.y * (_e1971.y + _e1971.y));
                                                        let _e5742 = (_e1973.z * (_e1971.z + _e1971.z));
                                                        let _e5749 = ((_e5734 * _e5734) + (_e5738 * _e5738));
                                                        let _e5750 = ((_e5735 * _e5735) + (_e5740 * _e5740));
                                                        let _e5751 = ((_e5736 * _e5736) + (_e5742 * _e5742));
                                                        let _e5756 = ((_e5721 * _e5734) / _e5749);
                                                        let _e5757 = ((_e5721 * _e5735) / _e5750);
                                                        let _e5758 = ((_e5721 * _e5736) / _e5751);
                                                        let _e5759 = (1f - _e5756);
                                                        let _e5760 = (1f - _e5757);
                                                        let _e5761 = (1f - _e5758);
                                                        let _e5763 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5756, _e5757, _e5758));
                                                        let _e5764 = (_e5721 * _e5738);
                                                        let _e5765 = (_e5721 * _e5740);
                                                        let _e5766 = (_e5721 * _e5742);
                                                        let _e5771 = (vec3<f32>(_e5764, _e5765, _e5766) / vec3<f32>(_e5749, _e5750, _e5751));
                                                        let _e5784 = ((_e5763.x * _e5763.x) + (_e5771.x * _e5771.x));
                                                        let _e5785 = ((_e5763.y * _e5763.y) + (_e5771.y * _e5771.y));
                                                        let _e5786 = ((_e5763.z * _e5763.z) + (_e5771.z * _e5771.z));
                                                        if (_e5784 != _e5784) {
                                                            phi_31549_ = true;
                                                        } else {
                                                            phi_31549_ = (0f >= _e5784);
                                                        }
                                                        let _e5790 = phi_31549_;
                                                        let _e5792 = sqrt(select(_e5784, 0f, _e5790));
                                                        if (_e5785 != _e5785) {
                                                            phi_31564_ = true;
                                                        } else {
                                                            phi_31564_ = (0f >= _e5785);
                                                        }
                                                        let _e5796 = phi_31564_;
                                                        let _e5798 = sqrt(select(_e5785, 0f, _e5796));
                                                        if (_e5786 != _e5786) {
                                                            phi_31579_ = true;
                                                        } else {
                                                            phi_31579_ = (0f >= _e5786);
                                                        }
                                                        let _e5802 = phi_31579_;
                                                        let _e5804 = sqrt(select(_e5786, 0f, _e5802));
                                                        let _e5808 = ((_e5792 + _e5759) * 0.5f);
                                                        let _e5809 = ((_e5798 + _e5760) * 0.5f);
                                                        let _e5810 = ((_e5804 + _e5761) * 0.5f);
                                                        if (_e5808 != _e5808) {
                                                            phi_31606_ = true;
                                                        } else {
                                                            phi_31606_ = (0f >= _e5808);
                                                        }
                                                        let _e5814 = phi_31606_;
                                                        let _e5816 = sqrt(select(_e5808, 0f, _e5814));
                                                        if (_e5809 != _e5809) {
                                                            phi_31621_ = true;
                                                        } else {
                                                            phi_31621_ = (0f >= _e5809);
                                                        }
                                                        let _e5820 = phi_31621_;
                                                        let _e5822 = sqrt(select(_e5809, 0f, _e5820));
                                                        if (_e5810 != _e5810) {
                                                            phi_31636_ = true;
                                                        } else {
                                                            phi_31636_ = (0f >= _e5810);
                                                        }
                                                        let _e5826 = phi_31636_;
                                                        let _e5828 = sqrt(select(_e5810, 0f, _e5826));
                                                        let _e5829 = vec3<f32>(_e5816, _e5822, _e5828);
                                                        let _e5833 = ((_e5792 - _e5759) * 0.5f);
                                                        let _e5834 = ((_e5798 - _e5760) * 0.5f);
                                                        let _e5835 = ((_e5804 - _e5761) * 0.5f);
                                                        if (_e5833 != _e5833) {
                                                            phi_31663_ = true;
                                                        } else {
                                                            phi_31663_ = (0f >= _e5833);
                                                        }
                                                        let _e5839 = phi_31663_;
                                                        let _e5841 = sqrt(select(_e5833, 0f, _e5839));
                                                        if (_e5834 != _e5834) {
                                                            phi_31678_ = true;
                                                        } else {
                                                            phi_31678_ = (0f >= _e5834);
                                                        }
                                                        let _e5845 = phi_31678_;
                                                        let _e5847 = sqrt(select(_e5834, 0f, _e5845));
                                                        if (_e5835 != _e5835) {
                                                            phi_31693_ = true;
                                                        } else {
                                                            phi_31693_ = (0f >= _e5835);
                                                        }
                                                        let _e5851 = phi_31693_;
                                                        let _e5853 = sqrt(select(_e5835, 0f, _e5851));
                                                        if ((_e5764 / _e5749) < 0f) {
                                                            phi_19442_ = -(_e5841);
                                                        } else {
                                                            phi_19442_ = _e5841;
                                                        }
                                                        let _e5857 = phi_19442_;
                                                        if ((_e5765 / _e5750) < 0f) {
                                                            phi_19450_ = -(_e5847);
                                                        } else {
                                                            phi_19450_ = _e5847;
                                                        }
                                                        let _e5861 = phi_19450_;
                                                        if ((_e5766 / _e5751) < 0f) {
                                                            phi_19458_ = -(_e5853);
                                                        } else {
                                                            phi_19458_ = _e5853;
                                                        }
                                                        let _e5865 = phi_19458_;
                                                        let _e5869 = (_e1971.x * _e5719);
                                                        let _e5870 = (_e1971.y * _e5719);
                                                        let _e5871 = (_e1971.z * _e5719);
                                                        let _e5881 = vec3<f32>(((_e1973.x * _e5816) - (_e1971.x * _e5857)), ((_e1973.y * _e5822) - (_e1971.y * _e5861)), ((_e1973.z * _e5828) - (_e1971.z * _e5865)));
                                                        let _e5888 = ((_e1973.x * _e5857) + (_e1971.x * _e5816));
                                                        let _e5889 = ((_e1973.y * _e5861) + (_e1971.y * _e5822));
                                                        let _e5890 = ((_e1973.z * _e5865) + (_e1971.z * _e5828));
                                                        let _e5892 = vec3<f32>((_e1973.x * _e5719), (_e1973.y * _e5719), (_e1973.z * _e5719));
                                                        let _e5893 = (_e5892 - _e5829);
                                                        let _e5894 = (_e5869 - _e5857);
                                                        let _e5895 = (_e5870 - _e5861);
                                                        let _e5896 = (_e5871 - _e5865);
                                                        let _e5909 = (_e5892 + _e5829);
                                                        let _e5910 = (_e5869 + _e5857);
                                                        let _e5911 = (_e5870 + _e5861);
                                                        let _e5912 = (_e5871 + _e5865);
                                                        let _e5922 = ((_e5909.x * _e5909.x) + (_e5910 * _e5910));
                                                        let _e5923 = ((_e5909.y * _e5909.y) + (_e5911 * _e5911));
                                                        let _e5924 = ((_e5909.z * _e5909.z) + (_e5912 * _e5912));
                                                        let _e5934 = vec3(_e5719);
                                                        let _e5935 = (_e5934 - _e5881);
                                                        let _e5936 = -(vec3<f32>(_e5888, _e5889, _e5890));
                                                        let _e5952 = (_e5934 + _e5881);
                                                        let _e5962 = ((_e5952.x * _e5952.x) + (_e5888 * _e5888));
                                                        let _e5963 = ((_e5952.y * _e5952.y) + (_e5889 * _e5889));
                                                        let _e5964 = ((_e5952.z * _e5952.z) + (_e5890 * _e5890));
                                                        phi_19565_ = vec3<f32>((((((_e5893.x * _e5893.x) + (_e5894 * _e5894)) / select(0.000000000000000000000000000001f, _e5922, (_e5922 > 0.000000000000000000000000000001f))) + (((_e5935.x * _e5935.x) + (_e5936.x * _e5936.x)) / select(0.000000000000000000000000000001f, _e5962, (_e5962 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5893.y * _e5893.y) + (_e5895 * _e5895)) / select(0.000000000000000000000000000001f, _e5923, (_e5923 > 0.000000000000000000000000000001f))) + (((_e5935.y * _e5935.y) + (_e5936.y * _e5936.y)) / select(0.000000000000000000000000000001f, _e5963, (_e5963 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5893.z * _e5893.z) + (_e5896 * _e5896)) / select(0.000000000000000000000000000001f, _e5924, (_e5924 > 0.000000000000000000000000000001f))) + (((_e5935.z * _e5935.z) + (_e5936.z * _e5936.z)) / select(0.000000000000000000000000000001f, _e5964, (_e5964 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6007 = phi_19565_;
                                                    phi_19566_ = _e6007;
                                                }
                                                let _e6027 = phi_19566_;
                                                let _e6029 = (4f * _e5641);
                                                phi_19578_ = type_59((_e6027 * (((1f / (_e5692 + _e5706)) * _e5677) / _e6029)), ((_e5677 / _e5692) / _e6029));
                                            }
                                            let _e6036 = phi_19578_;
                                            phi_19579_ = _e6036;
                                            phi_19580_ = _e5653;
                                        }
                                        let _e6038 = phi_19579_;
                                        let _e6040 = phi_19580_;
                                        if _e6040 {
                                            phi_19585_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19585_ = _e6038;
                                        }
                                        let _e6042 = phi_19585_;
                                        let _e6045 = (_e1979 == 1u);
                                        if _e6045 {
                                            phi_19613_ = _e6042;
                                            phi_19614_ = type_59();
                                        } else {
                                            if (_e5641 != _e5641) {
                                                phi_31955_ = true;
                                            } else {
                                                phi_31955_ = (0f >= _e5641);
                                            }
                                            let _e6049 = phi_31955_;
                                            let _e6050 = select(_e5641, 0f, _e6049);
                                            let _e6051 = (1f / _e1969);
                                            let _e6055 = ((_e6051 * _e6051) * (1f - (_e6050 * _e6050)));
                                            if (_e6055 >= 1f) {
                                                phi_31974_ = 1f;
                                            } else {
                                                let _e6058 = sqrt((1f - _e6055));
                                                if (_e6050 != _e6050) {
                                                    phi_31985_ = true;
                                                } else {
                                                    phi_31985_ = (_e6058 <= _e6050);
                                                }
                                                let _e6062 = phi_31985_;
                                                let _e6067 = pow(((1f - _e1969) / (1f + _e1969)), 2f);
                                                phi_31974_ = (_e6067 + ((1f - _e6067) * pow((1f - select(_e6050, _e6058, _e6062)), 5f)));
                                            }
                                            let _e6074 = phi_31974_;
                                            if (_e5620 <= 0f) {
                                                phi_32024_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_32024_ = (_e1967 * (_e5620 * 0.31830987f));
                                            }
                                            let _e6079 = phi_32024_;
                                            let _e6080 = (1f - _e6074);
                                            phi_19613_ = type_59();
                                            phi_19614_ = type_59((_e6042.member + vec3<f32>((_e6079.x * _e6080), (_e6079.y * _e6080), (_e6079.z * _e6080))), ((_e6074 * _e6042.member_1) + (_e6080 * _e5626)));
                                        }
                                        let _e6094 = phi_19613_;
                                        let _e6096 = phi_19614_;
                                        phi_19615_ = _e6094;
                                        phi_19616_ = _e6096;
                                        phi_19617_ = _e6045;
                                    }
                                    let _e6104 = phi_19615_;
                                    let _e6106 = phi_19616_;
                                    let _e6108 = phi_19617_;
                                    if _e6108 {
                                        phi_19621_ = _e6104;
                                    } else {
                                        phi_19621_ = _e6106;
                                    }
                                    let _e6110 = phi_19621_;
                                    if ((((_e5612.x * _e5612.x) + (_e5612.y * _e5612.y)) + (_e5612.z * _e5612.z)) == 0f) {
                                        phi_38352_ = _e5375;
                                        phi_19760_ = type_69();
                                        phi_19761_ = type_69();
                                        phi_19762_ = true;
                                        phi_19763_ = false;
                                    } else {
                                        let _e6119 = (_e6110.member_1 <= 0f);
                                        if _e6119 {
                                            phi_38353_ = _e5375;
                                            phi_19757_ = type_69();
                                            phi_19758_ = type_69();
                                            phi_19759_ = false;
                                        } else {
                                            let _e6121 = (_e6110.member.x / _e6110.member_1);
                                            let _e6123 = (_e6110.member.y / _e6110.member_1);
                                            let _e6125 = (_e6110.member.z / _e6110.member_1);
                                            let _e6134 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6121 * _e6121) + (_e6123 * _e6123)) + (_e6125 * _e6125))) <= _e6134) {
                                                phi_38346_ = _e5375;
                                                phi_19708_ = _e828;
                                                phi_19709_ = false;
                                            } else {
                                                let _e6138 = global_1.member.member_7[1u];
                                                if (_e6138 == 1u) {
                                                    let _e6143 = global_1.member.member_4[3u];
                                                    if (f32(_e362) >= _e6143) {
                                                        let _e6146 = select(_e6123, _e6121, (_e6121 > _e6123));
                                                        let _e6148 = select(_e6125, _e6146, (_e6146 > _e6125));
                                                        if (_e6148 != _e6148) {
                                                            phi_32071_ = true;
                                                        } else {
                                                            phi_32071_ = (1f <= _e6148);
                                                        }
                                                        let _e6152 = phi_32071_;
                                                        let _e6153 = select(_e6148, 1f, _e6152);
                                                        let _e6155 = ((_e5375 * 747796405u) + 2891336453u);
                                                        let _e6163 = (((_e6155 >> bitcast<u32>((((_e6155 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6155) * 277803737u);
                                                        let _e6166 = ((_e6163 >> bitcast<u32>(22i)) ^ _e6163);
                                                        let _e6171 = ((f32((_e6166 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6153);
                                                        if _e6171 {
                                                            phi_19697_ = vec3<f32>();
                                                        } else {
                                                            phi_19697_ = vec3<f32>((_e828.x / _e6153), (_e828.y / _e6153), (_e828.z / _e6153));
                                                        }
                                                        let _e6180 = phi_19697_;
                                                        phi_38349_ = _e6166;
                                                        phi_19699_ = _e6180;
                                                        phi_19700_ = select(true, false, _e6171);
                                                    } else {
                                                        phi_38349_ = _e5375;
                                                        phi_19699_ = _e828;
                                                        phi_19700_ = true;
                                                    }
                                                    let _e6183 = phi_38349_;
                                                    let _e6185 = phi_19699_;
                                                    let _e6187 = phi_19700_;
                                                    phi_38348_ = _e6183;
                                                    phi_19701_ = _e6185;
                                                    phi_19702_ = _e6187;
                                                } else {
                                                    phi_38348_ = _e5375;
                                                    phi_19701_ = _e828;
                                                    phi_19702_ = true;
                                                }
                                                let _e6189 = phi_38348_;
                                                let _e6191 = phi_19701_;
                                                let _e6193 = phi_19702_;
                                                phi_38346_ = _e6189;
                                                phi_19708_ = select(_e828, _e6191, vec3(_e6193));
                                                phi_19709_ = select(false, true, _e6193);
                                            }
                                            let _e6198 = phi_38346_;
                                            let _e6200 = phi_19708_;
                                            let _e6202 = phi_19709_;
                                            if _e6202 {
                                                if ((((_e5612.x * _e1074.x) + (_e5612.y * _e1074.y)) + (_e5612.z * _e1074.z)) < 0f) {
                                                    phi_19738_ = -(_e1074);
                                                } else {
                                                    phi_19738_ = _e1074;
                                                }
                                                let _e6241 = phi_19738_;
                                                let _e6258 = local_88;
                                                let _e6261 = local_89;
                                                let _e6264 = local_90;
                                                let _e6267 = local_91;
                                                let _e6273 = local_92;
                                                phi_19754_ = type_69(type_34(vec4<f32>((_e834 + (_e6241.x * _e1104)), (_e835 + (_e6241.y * _e1104)), (_e836 + (_e6241.z * _e1104)), _e337.member.w), vec4<f32>(_e5612.x, _e5612.y, _e5612.z, _e6110.member_1), vec4<f32>((_e6200.x * _e6121), (_e6200.y * _e6123), (_e6200.z * _e6125), f32((_e362 + 1u))), vec4<f32>(_e6261.x, _e6264.y, _e6267.z, f32(_e6258)), vec4<u32>(_e6198, _e1987, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, _e6273);
                                                phi_19755_ = type_69();
                                            } else {
                                                let _e6210 = local_83;
                                                let _e6213 = local_84;
                                                let _e6216 = local_85;
                                                let _e6219 = local_86;
                                                let _e6225 = local_87;
                                                phi_19754_ = type_69();
                                                phi_19755_ = type_69(type_34(_e337.member, _e337.member_1, vec4<f32>(_e6200.x, _e6200.y, _e6200.z, f32(_e362)), vec4<f32>(_e6213.x, _e6216.y, _e6219.z, f32(_e6210)), vec4<u32>(_e6198, (_e1987 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, _e6225);
                                            }
                                            let _e6276 = phi_19754_;
                                            let _e6278 = phi_19755_;
                                            phi_38353_ = _e6198;
                                            phi_19757_ = _e6276;
                                            phi_19758_ = _e6278;
                                            phi_19759_ = select(true, false, _e6202);
                                        }
                                        let _e6281 = phi_38353_;
                                        let _e6283 = phi_19757_;
                                        let _e6285 = phi_19758_;
                                        let _e6287 = phi_19759_;
                                        phi_38352_ = _e6281;
                                        phi_19760_ = _e6283;
                                        phi_19761_ = _e6285;
                                        phi_19762_ = _e6119;
                                        phi_19763_ = _e6287;
                                    }
                                    let _e6289 = phi_38352_;
                                    let _e6291 = phi_19760_;
                                    let _e6293 = phi_19761_;
                                    let _e6295 = phi_19762_;
                                    let _e6297 = phi_19763_;
                                    if _e6295 {
                                        let _e6305 = local_93;
                                        let _e6308 = local_94;
                                        let _e6311 = local_95;
                                        let _e6314 = local_96;
                                        let _e6320 = local_97;
                                        phi_19769_ = type_69(type_34(_e337.member, _e337.member_1, vec4<f32>(_e828.x, _e828.y, _e828.z, f32(_e362)), vec4<f32>(_e6308.x, _e6311.y, _e6314.z, f32(_e6305)), vec4<u32>(_e6289, (_e1987 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e830, _e6320);
                                    } else {
                                        phi_19769_ = _e6293;
                                    }
                                    let _e6323 = phi_19769_;
                                    phi_19774_ = _e6323;
                                    phi_19775_ = _e6291;
                                    phi_19776_ = select(_e6297, true, _e6295);
                                }
                                let _e6350 = phi_19774_;
                                let _e6352 = phi_19775_;
                                let _e6354 = phi_19776_;
                                phi_19780_ = _e6350;
                                phi_19781_ = _e6352;
                                phi_19782_ = _e6354;
                            } else {
                                phi_19780_ = _e1983;
                                phi_19781_ = type_69();
                                phi_19782_ = true;
                            }
                            let _e6356 = phi_19780_;
                            let _e6358 = phi_19781_;
                            let _e6360 = phi_19782_;
                            phi_19786_ = _e6356;
                            phi_19787_ = _e6358;
                            phi_19788_ = _e6360;
                        }
                        let _e6393 = phi_19786_;
                        let _e6395 = phi_19787_;
                        let _e6397 = phi_19788_;
                        phi_19789_ = _e6393;
                        phi_19790_ = _e6395;
                        phi_19791_ = _e6397;
                    }
                    let _e6592 = phi_19789_;
                    let _e6594 = phi_19790_;
                    let _e6596 = phi_19791_;
                    phi_19795_ = _e6592;
                    phi_19796_ = _e6594;
                    phi_19797_ = _e6596;
                }
                let _e7108 = phi_19795_;
                let _e7110 = phi_19796_;
                let _e7112 = phi_19797_;
                if _e7112 {
                    phi_19801_ = _e7108;
                } else {
                    phi_19801_ = _e7110;
                }
                let _e7114 = phi_19801_;
                if _e334 {
                } else {
                    break;
                }
                global_7.member[_e329.member_1] = _e7114.member;
                let _e7118 = (2u * _e329.member_1);
                if (_e7118 < _e289) {
                } else {
                    break;
                }
                global_9.member[_e7118] = _e7114.member_1;
                let _e7122 = (_e7118 + 1u);
                if (_e7122 < _e289) {
                } else {
                    break;
                }
                global_9.member[_e7122] = _e7114.member_2;
                if ((_e7114.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7130 = (1u - _e293);
                    let _e7131 = (_e7130 * 4u);
                    let _e7135 = atomicAdd((&global_8.member[(_e7131 + 3u)]), 1u);
                    let _e7139 = global_1.member.member[0u];
                    let _e7143 = global_1.member.member[1u];
                    let _e7148 = global_1.member.member_7[3u];
                    atomicStore((&global_8.member[((12u + (_e7130 * ((_e7139 * _e7143) * _e7148))) + _e7135)]), _e329.member_1);
                    if ((_e7135 % 64u) == 0u) {
                        let _e7157 = (_e7135 / 64u);
                        if (_e7157 < 65535u) {
                            let _e7161 = atomicAdd((&global_8.member[_e7131]), 1u);
                        }
                        if ((_e7157 % 65535u) == 0u) {
                            let _e7167 = atomicAdd((&global_8.member[(_e7131 + 1u)]), 1u);
                        }
                    }
                }
                if ((_e7114.member.member_4.y & 16u) == 0u) {
                } else {
                    let _e7172 = atomicAdd((&global_8.member[11u]), 1u);
                    let _e7176 = global_1.member.member[0u];
                    let _e7180 = global_1.member.member[1u];
                    let _e7185 = global_1.member.member_7[3u];
                    atomicStore((&global_8.member[((12u + (2u * ((_e7176 * _e7180) * _e7185))) + _e7172)]), _e329.member_1);
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
    var phi_20873_: type_48;
    var phi_20926_: u32;
    var phi_20929_: u32;
    var phi_20931_: u32;
    var phi_20933_: u32;
    var phi_20935_: u32;
    var phi_20937_: vec4<f32>;
    var phi_20939_: f32;
    var phi_21309_: u32;
    var phi_21311_: u32;
    var phi_21312_: u32;
    var phi_21314_: bool;
    var phi_21315_: bool;
    var phi_21316_: u32;
    var phi_21317_: u32;
    var phi_21318_: u32;
    var phi_21319_: bool;
    var phi_21320_: bool;
    var phi_21321_: bool;
    var phi_32773_: bool;
    var phi_32803_: bool;
    var phi_32750_: type_56;
    var phi_32753_: type_56;
    var phi_32754_: bool;
    var phi_32759_: type_56;
    var phi_32860_: bool;
    var phi_32875_: bool;
    var phi_32890_: bool;
    var phi_32905_: bool;
    var phi_32837_: type_56;
    var phi_32840_: type_56;
    var phi_32841_: bool;
    var phi_32846_: type_56;
    var phi_32947_: bool;
    var phi_32962_: bool;
    var phi_32977_: bool;
    var phi_32992_: bool;
    var phi_32924_: type_56;
    var phi_32927_: type_56;
    var phi_32928_: bool;
    var phi_32933_: type_56;
    var phi_32728_: bool;
    var phi_32731_: bool;
    var phi_21350_: f32;
    var phi_33094_: bool;
    var phi_33124_: bool;
    var phi_33071_: type_56;
    var phi_33074_: type_56;
    var phi_33075_: bool;
    var phi_33080_: type_56;
    var phi_33181_: bool;
    var phi_33196_: bool;
    var phi_33211_: bool;
    var phi_33226_: bool;
    var phi_33158_: type_56;
    var phi_33161_: type_56;
    var phi_33162_: bool;
    var phi_33167_: type_56;
    var phi_33268_: bool;
    var phi_33283_: bool;
    var phi_33298_: bool;
    var phi_33313_: bool;
    var phi_33245_: type_56;
    var phi_33248_: type_56;
    var phi_33249_: bool;
    var phi_33254_: type_56;
    var phi_33049_: bool;
    var phi_33052_: bool;
    var phi_21359_: f32;
    var phi_21370_: u32;
    var phi_21381_: u32;
    var phi_21420_: u32;
    var phi_21421_: u32;
    var phi_21432_: u32;
    var phi_21433_: u32;
    var phi_21434_: u32;
    var phi_21435_: u32;
    var phi_21437_: u32;
    var phi_21438_: u32;
    var phi_21440_: u32;
    var phi_21441_: u32;
    var phi_21442_: bool;
    var phi_21443_: bool;
    var phi_21235_: vec4<f32>;
    var phi_21236_: vec4<f32>;
    var phi_21237_: bool;
    var phi_21241_: vec4<f32>;
    var phi_21242_: bool;
    var phi_21152_: bool;
    var phi_21186_: bool;
    var phi_21192_: f32;
    var phi_21202_: vec4<f32>;
    var phi_21203_: vec4<f32>;
    var phi_21204_: bool;
    var phi_21208_: vec4<f32>;
    var phi_21209_: bool;
    var phi_21213_: vec4<f32>;
    var phi_21214_: bool;
    var phi_21243_: vec4<f32>;
    var phi_21244_: bool;
    var phi_21085_: vec4<f32>;
    var phi_21086_: vec4<f32>;
    var phi_21087_: bool;
    var phi_21091_: vec4<f32>;
    var phi_21092_: bool;
    var phi_21093_: vec4<f32>;
    var phi_21094_: bool;
    var phi_21095_: bool;
    var phi_21103_: vec4<f32>;
    var phi_21104_: bool;
    var phi_21105_: vec4<f32>;
    var phi_21106_: bool;
    var phi_21107_: bool;
    var phi_21115_: vec4<f32>;
    var phi_21116_: bool;
    var phi_21253_: vec4<f32>;
    var phi_21254_: bool;
    var phi_21275_: f32;
    var phi_20927_: u32;
    var phi_20930_: u32;
    var phi_20932_: u32;
    var phi_20934_: u32;
    var phi_20936_: u32;
    var phi_20938_: vec4<f32>;
    var phi_20940_: f32;
    var phi_21454_: vec4<f32>;
    var phi_21455_: bool;
    var phi_21456_: bool;
    var phi_38408_: bool;
    var phi_21684_: bool;
    var phi_21683_: vec4<f32>;
    var local_105: u32;
    var local_106: vec4<f32>;
    var phi_21461_: type_74;
    var local_107: u32;
    var phi_21467_: type_74;

    switch bitcast<i32>(0u) {
        default: {
            let _e279 = global;
            let _e288 = global_1.member.member_7[2u];
            let _e292 = (_e279.x + (_e279.y * 4194240u));
            let _e297 = atomicLoad((&global_8.member[((_e288 * 4u) + 3u)]));
            if (_e292 >= _e297) {
                phi_20873_ = type_48(0u, type_48().member_1);
            } else {
                let _e302 = global_1.member.member[0u];
                let _e306 = global_1.member.member[1u];
                let _e311 = global_1.member.member_7[3u];
                let _e318 = atomicLoad((&global_8.member[((12u + (_e288 * ((_e302 * _e306) * _e311))) + _e292)]));
                phi_20873_ = type_48(1u, _e318);
            }
            let _e324 = phi_20873_;
            if (bitcast<i32>(_e324.member) != 0i) {
                let _e329 = (_e324.member_1 < arrayLength((&global_7.member)));
                if _e329 {
                } else {
                    break;
                }
                let _e335 = global_7.member[_e324.member_1].member;
                let _e342 = global_7.member[_e324.member_1].member_1;
                let _e348 = global_1.member.member_1[0u];
                let _e351 = global_1.member.member_1[1u];
                let _e354 = (vec3<f32>(1f, 1f, 1f) / vec3<f32>(_e342.x, _e342.y, _e342.z));
                local_103 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                local_104 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                phi_20926_ = 4294967295u;
                phi_20929_ = select(4294967295u, 0u, (_e348 > 0u));
                phi_20931_ = 0u;
                phi_20933_ = (_e348 + _e351);
                phi_20935_ = _e348;
                phi_20937_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_20939_ = 300000000000000000000000000000000000000f;
                loop {
                    let _e358 = phi_20926_;
                    let _e360 = phi_20929_;
                    let _e362 = phi_20931_;
                    let _e364 = phi_20933_;
                    let _e366 = phi_20935_;
                    let _e368 = phi_20937_;
                    let _e370 = phi_20939_;
                    local_105 = _e358;
                    local_106 = _e368;
                    local_107 = _e366;
                    if (_e366 < _e364) {
                        let _e733 = (3u * _e366);
                        let _e735 = global_2.member[_e733];
                        let _e738 = global_2.member[(_e733 + 1u)];
                        let _e744 = select(select(u32(_e738.w), 0u, (_e738.w < 0f)), 4294967295u, (_e738.w > 4294967000f));
                        if (_e744 == 0u) {
                            let _e838 = global_2.member[(_e733 + 2u)];
                            let _e847 = ((_e342.y * _e838.z) - (_e838.y * _e342.z));
                            let _e850 = ((_e342.z * _e838.x) - (_e838.z * _e342.x));
                            let _e853 = ((_e342.x * _e838.y) - (_e838.x * _e342.y));
                            let _e858 = (((_e738.x * _e847) + (_e738.y * _e850)) + (_e738.z * _e853));
                            if (abs(_e858) < 0.000000000001f) {
                                phi_21115_ = vec4<f32>();
                                phi_21116_ = true;
                            } else {
                                let _e861 = (1f / _e858);
                                let _e865 = (_e335.x - _e735.x);
                                let _e866 = (_e335.y - _e735.y);
                                let _e867 = (_e335.z - _e735.z);
                                let _e873 = ((((_e865 * _e847) + (_e866 * _e850)) + (_e867 * _e853)) * _e861);
                                if (_e873 < 0f) {
                                    phi_21105_ = vec4<f32>();
                                    phi_21106_ = true;
                                    phi_21107_ = false;
                                } else {
                                    let _e875 = (_e873 > 1f);
                                    if _e875 {
                                        phi_21103_ = vec4<f32>();
                                        phi_21104_ = false;
                                    } else {
                                        let _e878 = ((_e866 * _e738.z) - (_e738.y * _e867));
                                        let _e881 = ((_e867 * _e738.x) - (_e738.z * _e865));
                                        let _e884 = ((_e865 * _e738.y) - (_e738.x * _e866));
                                        let _e890 = ((((_e342.x * _e878) + (_e342.y * _e881)) + (_e342.z * _e884)) * _e861);
                                        if (_e890 < 0f) {
                                            phi_21093_ = vec4<f32>();
                                            phi_21094_ = true;
                                            phi_21095_ = false;
                                        } else {
                                            let _e893 = ((_e873 + _e890) > 1f);
                                            if _e893 {
                                                phi_21091_ = vec4<f32>();
                                                phi_21092_ = false;
                                            } else {
                                                let _e899 = ((((_e838.x * _e878) + (_e838.y * _e881)) + (_e838.z * _e884)) * _e861);
                                                if (_e899 <= 0f) {
                                                    phi_21086_ = vec4<f32>();
                                                    phi_21087_ = true;
                                                } else {
                                                    let _e901 = (_e899 >= _e370);
                                                    if _e901 {
                                                        phi_21085_ = vec4<f32>();
                                                    } else {
                                                        phi_21085_ = vec4<f32>(_e899, _e873, _e890, 1f);
                                                    }
                                                    let _e904 = phi_21085_;
                                                    phi_21086_ = _e904;
                                                    phi_21087_ = _e901;
                                                }
                                                let _e906 = phi_21086_;
                                                let _e908 = phi_21087_;
                                                phi_21091_ = _e906;
                                                phi_21092_ = _e908;
                                            }
                                            let _e910 = phi_21091_;
                                            let _e912 = phi_21092_;
                                            phi_21093_ = _e910;
                                            phi_21094_ = _e893;
                                            phi_21095_ = _e912;
                                        }
                                        let _e914 = phi_21093_;
                                        let _e916 = phi_21094_;
                                        let _e918 = phi_21095_;
                                        phi_21103_ = _e914;
                                        phi_21104_ = select(_e918, true, _e916);
                                    }
                                    let _e921 = phi_21103_;
                                    let _e923 = phi_21104_;
                                    phi_21105_ = _e921;
                                    phi_21106_ = _e875;
                                    phi_21107_ = _e923;
                                }
                                let _e925 = phi_21105_;
                                let _e927 = phi_21106_;
                                let _e929 = phi_21107_;
                                phi_21115_ = _e925;
                                phi_21116_ = select(_e929, true, _e927);
                            }
                            let _e932 = phi_21115_;
                            let _e934 = phi_21116_;
                            phi_21253_ = _e932;
                            phi_21254_ = _e934;
                        } else {
                            if (_e744 == 1u) {
                                let _e768 = (_e335.x - _e735.x);
                                let _e769 = (_e335.y - _e735.y);
                                let _e770 = (_e335.z - _e735.z);
                                let _e775 = (((_e768 * _e342.x) + (_e769 * _e342.y)) + (_e770 * _e342.z));
                                if (_e775 > 0f) {
                                    phi_21152_ = select(true, false, ((((_e768 * _e768) + (_e769 * _e769)) + (_e770 * _e770)) > (_e735.w * _e735.w)));
                                } else {
                                    phi_21152_ = true;
                                }
                                let _e787 = phi_21152_;
                                if _e787 {
                                    let _e791 = (_e768 - (_e342.x * _e775));
                                    let _e792 = (_e769 - (_e342.y * _e775));
                                    let _e793 = (_e770 - (_e342.z * _e775));
                                    let _e801 = ((_e735.w * _e735.w) - (((_e791 * _e791) + (_e792 * _e792)) + (_e793 * _e793)));
                                    if (_e801 < 0f) {
                                        phi_21208_ = vec4<f32>();
                                        phi_21209_ = true;
                                    } else {
                                        let _e803 = sqrt(_e801);
                                        let _e804 = -(_e775);
                                        let _e805 = (_e804 - _e803);
                                        if (_e805 <= 0f) {
                                            phi_21186_ = true;
                                        } else {
                                            phi_21186_ = (_e805 >= _e370);
                                        }
                                        let _e809 = phi_21186_;
                                        if _e809 {
                                            phi_21192_ = (_e804 + _e803);
                                        } else {
                                            phi_21192_ = _e805;
                                        }
                                        let _e812 = phi_21192_;
                                        if (_e812 <= 0f) {
                                            phi_21203_ = vec4<f32>();
                                            phi_21204_ = true;
                                        } else {
                                            let _e814 = (_e812 >= _e370);
                                            if _e814 {
                                                phi_21202_ = vec4<f32>();
                                            } else {
                                                phi_21202_ = vec4<f32>(_e812, 0f, 0f, 1f);
                                            }
                                            let _e817 = phi_21202_;
                                            phi_21203_ = _e817;
                                            phi_21204_ = _e814;
                                        }
                                        let _e819 = phi_21203_;
                                        let _e821 = phi_21204_;
                                        phi_21208_ = _e819;
                                        phi_21209_ = _e821;
                                    }
                                    let _e823 = phi_21208_;
                                    let _e825 = phi_21209_;
                                    phi_21213_ = _e823;
                                    phi_21214_ = _e825;
                                } else {
                                    phi_21213_ = vec4<f32>();
                                    phi_21214_ = true;
                                }
                                let _e827 = phi_21213_;
                                let _e829 = phi_21214_;
                                phi_21243_ = _e827;
                                phi_21244_ = _e829;
                            } else {
                                if (abs(_e342.y) < 0.000000000001f) {
                                    phi_21241_ = vec4<f32>();
                                    phi_21242_ = true;
                                } else {
                                    let _e751 = ((_e735.x - _e335.y) / _e342.y);
                                    if (_e751 <= 0f) {
                                        phi_21236_ = vec4<f32>();
                                        phi_21237_ = true;
                                    } else {
                                        let _e753 = (_e751 >= _e370);
                                        if _e753 {
                                            phi_21235_ = vec4<f32>();
                                        } else {
                                            phi_21235_ = vec4<f32>(_e751, 0f, 0f, 1f);
                                        }
                                        let _e756 = phi_21235_;
                                        phi_21236_ = _e756;
                                        phi_21237_ = _e753;
                                    }
                                    let _e758 = phi_21236_;
                                    let _e760 = phi_21237_;
                                    phi_21241_ = _e758;
                                    phi_21242_ = _e760;
                                }
                                let _e762 = phi_21241_;
                                let _e764 = phi_21242_;
                                phi_21243_ = _e762;
                                phi_21244_ = _e764;
                            }
                            let _e831 = phi_21243_;
                            let _e833 = phi_21244_;
                            phi_21253_ = _e831;
                            phi_21254_ = select(true, false, select(true, false, _e833));
                        }
                        let _e936 = phi_21253_;
                        let _e938 = phi_21254_;
                        let _e941 = select(vec4<f32>(0f, 0f, 0f, 0f), _e936, vec4(select(true, false, _e938)));
                        let _e943 = (_e941.w > 0f);
                        if _e943 {
                            phi_21275_ = _e941.x;
                        } else {
                            phi_21275_ = _e370;
                        }
                        let _e946 = phi_21275_;
                        phi_20927_ = select(_e358, _e366, _e943);
                        phi_20930_ = _e360;
                        phi_20932_ = _e362;
                        phi_20934_ = _e364;
                        phi_20936_ = (_e366 + 1u);
                        phi_20938_ = select(_e368, _e941, vec4(_e943));
                        phi_20940_ = _e946;
                        phi_21454_ = _e941;
                        phi_21455_ = true;
                        phi_21456_ = false;
                    } else {
                        if (_e360 == 4294967295u) {
                            let _e373 = (_e362 == 0u);
                            if _e373 {
                                phi_21311_ = u32();
                                phi_21312_ = u32();
                                phi_21314_ = false;
                                phi_21315_ = false;
                            } else {
                                let _e374 = (_e362 - 1u);
                                let _e375 = (_e374 < 64u);
                                if _e375 {
                                } else {
                                    phi_38408_ = true;
                                    phi_21684_ = bool();
                                    phi_21683_ = vec4<f32>();
                                    break;
                                }
                                let _e377 = local_104[_e374];
                                let _e378 = (_e377 >= _e370);
                                if _e378 {
                                    phi_21309_ = u32();
                                } else {
                                    if _e375 {
                                    } else {
                                        phi_38408_ = true;
                                        phi_21684_ = bool();
                                        phi_21683_ = vec4<f32>();
                                        break;
                                    }
                                    let _e380 = local_103[_e374];
                                    phi_21309_ = _e380;
                                }
                                let _e382 = phi_21309_;
                                phi_21311_ = _e374;
                                phi_21312_ = _e382;
                                phi_21314_ = select(true, false, _e378);
                                phi_21315_ = _e378;
                            }
                            let _e385 = phi_21311_;
                            let _e387 = phi_21312_;
                            let _e389 = phi_21314_;
                            let _e391 = phi_21315_;
                            phi_21316_ = _e385;
                            phi_21317_ = _e387;
                            phi_21318_ = _e385;
                            phi_21319_ = _e389;
                            phi_21320_ = _e391;
                            phi_21321_ = _e373;
                        } else {
                            phi_21316_ = u32();
                            phi_21317_ = _e360;
                            phi_21318_ = _e362;
                            phi_21319_ = true;
                            phi_21320_ = false;
                            phi_21321_ = false;
                        }
                        let _e393 = phi_21316_;
                        let _e395 = phi_21317_;
                        let _e397 = phi_21318_;
                        let _e399 = phi_21319_;
                        let _e401 = phi_21320_;
                        let _e403 = phi_21321_;
                        if _e399 {
                            let _e405 = ((_e395 & 2147483648u) == 0u);
                            if _e405 {
                                let _e413 = global_3.member[_e395];
                                if (_e413.member_1 == 4294967295u) {
                                    phi_21350_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e425 = ((_e413.member.x - _e335.x) * _e354.x);
                                    let _e427 = ((_e413.member.y - _e335.y) * _e354.y);
                                    let _e429 = ((_e413.member.z - _e335.z) * _e354.z);
                                    let _e436 = ((_e413.member_2.x - _e335.x) * _e354.x);
                                    let _e437 = ((_e413.member_2.y - _e335.y) * _e354.y);
                                    let _e438 = ((_e413.member_2.z - _e335.z) * _e354.z);
                                    if (_e425 == _e425) {
                                        let _e440 = (_e436 == _e436);
                                        if _e440 {
                                            let _e441 = (_e425 != _e425);
                                            if _e441 {
                                                phi_32773_ = true;
                                            } else {
                                                phi_32773_ = (_e436 <= _e425);
                                            }
                                            let _e444 = phi_32773_;
                                            let _e445 = select(_e425, _e436, _e444);
                                            if _e441 {
                                                phi_32803_ = true;
                                            } else {
                                                phi_32803_ = (_e436 >= _e425);
                                            }
                                            let _e450 = phi_32803_;
                                            let _e451 = select(_e425, _e436, _e450);
                                            phi_32750_ = type_56(select(-300000000000000000000000000000000000000f, _e445, (_e445 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e451, (_e451 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_32750_ = type_56();
                                        }
                                        let _e456 = phi_32750_;
                                        phi_32753_ = _e456;
                                        phi_32754_ = select(true, false, _e440);
                                    } else {
                                        phi_32753_ = type_56();
                                        phi_32754_ = true;
                                    }
                                    let _e459 = phi_32753_;
                                    let _e461 = phi_32754_;
                                    if _e461 {
                                        phi_32759_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_32759_ = _e459;
                                    }
                                    let _e463 = phi_32759_;
                                    if (_e427 == _e427) {
                                        let _e467 = (_e437 == _e437);
                                        if _e467 {
                                            let _e468 = (_e427 != _e427);
                                            if _e468 {
                                                phi_32860_ = true;
                                            } else {
                                                phi_32860_ = (_e437 <= _e427);
                                            }
                                            let _e471 = phi_32860_;
                                            let _e472 = select(_e427, _e437, _e471);
                                            if (_e463.member != _e463.member) {
                                                phi_32875_ = true;
                                            } else {
                                                phi_32875_ = (_e472 >= _e463.member);
                                            }
                                            let _e476 = phi_32875_;
                                            if _e468 {
                                                phi_32890_ = true;
                                            } else {
                                                phi_32890_ = (_e437 >= _e427);
                                            }
                                            let _e480 = phi_32890_;
                                            let _e481 = select(_e427, _e437, _e480);
                                            if (_e463.member_1 != _e463.member_1) {
                                                phi_32905_ = true;
                                            } else {
                                                phi_32905_ = (_e481 <= _e463.member_1);
                                            }
                                            let _e485 = phi_32905_;
                                            phi_32837_ = type_56(select(_e463.member, _e472, _e476), select(_e463.member_1, _e481, _e485));
                                        } else {
                                            phi_32837_ = type_56();
                                        }
                                        let _e489 = phi_32837_;
                                        phi_32840_ = _e489;
                                        phi_32841_ = select(true, false, _e467);
                                    } else {
                                        phi_32840_ = type_56();
                                        phi_32841_ = true;
                                    }
                                    let _e492 = phi_32840_;
                                    let _e494 = phi_32841_;
                                    if _e494 {
                                        phi_32846_ = _e463;
                                    } else {
                                        phi_32846_ = _e492;
                                    }
                                    let _e496 = phi_32846_;
                                    if (_e429 == _e429) {
                                        let _e500 = (_e438 == _e438);
                                        if _e500 {
                                            let _e501 = (_e429 != _e429);
                                            if _e501 {
                                                phi_32947_ = true;
                                            } else {
                                                phi_32947_ = (_e438 <= _e429);
                                            }
                                            let _e504 = phi_32947_;
                                            let _e505 = select(_e429, _e438, _e504);
                                            if (_e496.member != _e496.member) {
                                                phi_32962_ = true;
                                            } else {
                                                phi_32962_ = (_e505 >= _e496.member);
                                            }
                                            let _e509 = phi_32962_;
                                            if _e501 {
                                                phi_32977_ = true;
                                            } else {
                                                phi_32977_ = (_e438 >= _e429);
                                            }
                                            let _e513 = phi_32977_;
                                            let _e514 = select(_e429, _e438, _e513);
                                            if (_e496.member_1 != _e496.member_1) {
                                                phi_32992_ = true;
                                            } else {
                                                phi_32992_ = (_e514 <= _e496.member_1);
                                            }
                                            let _e518 = phi_32992_;
                                            phi_32924_ = type_56(select(_e496.member, _e505, _e509), select(_e496.member_1, _e514, _e518));
                                        } else {
                                            phi_32924_ = type_56();
                                        }
                                        let _e522 = phi_32924_;
                                        phi_32927_ = _e522;
                                        phi_32928_ = select(true, false, _e500);
                                    } else {
                                        phi_32927_ = type_56();
                                        phi_32928_ = true;
                                    }
                                    let _e525 = phi_32927_;
                                    let _e527 = phi_32928_;
                                    if _e527 {
                                        phi_32933_ = _e496;
                                    } else {
                                        phi_32933_ = _e525;
                                    }
                                    let _e529 = phi_32933_;
                                    if (_e529.member_1 < 0f) {
                                        phi_32731_ = true;
                                    } else {
                                        if (_e529.member > _e529.member_1) {
                                            phi_32728_ = true;
                                        } else {
                                            phi_32728_ = (_e529.member > _e370);
                                        }
                                        let _e536 = phi_32728_;
                                        phi_32731_ = _e536;
                                    }
                                    let _e538 = phi_32731_;
                                    phi_21350_ = select(_e529.member, 300000000000000000000000000000000000000f, _e538);
                                }
                                let _e541 = phi_21350_;
                                if (_e413.member_5 == 4294967295u) {
                                    phi_21359_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e553 = ((_e413.member_4.x - _e335.x) * _e354.x);
                                    let _e555 = ((_e413.member_4.y - _e335.y) * _e354.y);
                                    let _e557 = ((_e413.member_4.z - _e335.z) * _e354.z);
                                    let _e564 = ((_e413.member_6.x - _e335.x) * _e354.x);
                                    let _e565 = ((_e413.member_6.y - _e335.y) * _e354.y);
                                    let _e566 = ((_e413.member_6.z - _e335.z) * _e354.z);
                                    if (_e553 == _e553) {
                                        let _e568 = (_e564 == _e564);
                                        if _e568 {
                                            let _e569 = (_e553 != _e553);
                                            if _e569 {
                                                phi_33094_ = true;
                                            } else {
                                                phi_33094_ = (_e564 <= _e553);
                                            }
                                            let _e572 = phi_33094_;
                                            let _e573 = select(_e553, _e564, _e572);
                                            if _e569 {
                                                phi_33124_ = true;
                                            } else {
                                                phi_33124_ = (_e564 >= _e553);
                                            }
                                            let _e578 = phi_33124_;
                                            let _e579 = select(_e553, _e564, _e578);
                                            phi_33071_ = type_56(select(-300000000000000000000000000000000000000f, _e573, (_e573 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e579, (_e579 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_33071_ = type_56();
                                        }
                                        let _e584 = phi_33071_;
                                        phi_33074_ = _e584;
                                        phi_33075_ = select(true, false, _e568);
                                    } else {
                                        phi_33074_ = type_56();
                                        phi_33075_ = true;
                                    }
                                    let _e587 = phi_33074_;
                                    let _e589 = phi_33075_;
                                    if _e589 {
                                        phi_33080_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_33080_ = _e587;
                                    }
                                    let _e591 = phi_33080_;
                                    if (_e555 == _e555) {
                                        let _e595 = (_e565 == _e565);
                                        if _e595 {
                                            let _e596 = (_e555 != _e555);
                                            if _e596 {
                                                phi_33181_ = true;
                                            } else {
                                                phi_33181_ = (_e565 <= _e555);
                                            }
                                            let _e599 = phi_33181_;
                                            let _e600 = select(_e555, _e565, _e599);
                                            if (_e591.member != _e591.member) {
                                                phi_33196_ = true;
                                            } else {
                                                phi_33196_ = (_e600 >= _e591.member);
                                            }
                                            let _e604 = phi_33196_;
                                            if _e596 {
                                                phi_33211_ = true;
                                            } else {
                                                phi_33211_ = (_e565 >= _e555);
                                            }
                                            let _e608 = phi_33211_;
                                            let _e609 = select(_e555, _e565, _e608);
                                            if (_e591.member_1 != _e591.member_1) {
                                                phi_33226_ = true;
                                            } else {
                                                phi_33226_ = (_e609 <= _e591.member_1);
                                            }
                                            let _e613 = phi_33226_;
                                            phi_33158_ = type_56(select(_e591.member, _e600, _e604), select(_e591.member_1, _e609, _e613));
                                        } else {
                                            phi_33158_ = type_56();
                                        }
                                        let _e617 = phi_33158_;
                                        phi_33161_ = _e617;
                                        phi_33162_ = select(true, false, _e595);
                                    } else {
                                        phi_33161_ = type_56();
                                        phi_33162_ = true;
                                    }
                                    let _e620 = phi_33161_;
                                    let _e622 = phi_33162_;
                                    if _e622 {
                                        phi_33167_ = _e591;
                                    } else {
                                        phi_33167_ = _e620;
                                    }
                                    let _e624 = phi_33167_;
                                    if (_e557 == _e557) {
                                        let _e628 = (_e566 == _e566);
                                        if _e628 {
                                            let _e629 = (_e557 != _e557);
                                            if _e629 {
                                                phi_33268_ = true;
                                            } else {
                                                phi_33268_ = (_e566 <= _e557);
                                            }
                                            let _e632 = phi_33268_;
                                            let _e633 = select(_e557, _e566, _e632);
                                            if (_e624.member != _e624.member) {
                                                phi_33283_ = true;
                                            } else {
                                                phi_33283_ = (_e633 >= _e624.member);
                                            }
                                            let _e637 = phi_33283_;
                                            if _e629 {
                                                phi_33298_ = true;
                                            } else {
                                                phi_33298_ = (_e566 >= _e557);
                                            }
                                            let _e641 = phi_33298_;
                                            let _e642 = select(_e557, _e566, _e641);
                                            if (_e624.member_1 != _e624.member_1) {
                                                phi_33313_ = true;
                                            } else {
                                                phi_33313_ = (_e642 <= _e624.member_1);
                                            }
                                            let _e646 = phi_33313_;
                                            phi_33245_ = type_56(select(_e624.member, _e633, _e637), select(_e624.member_1, _e642, _e646));
                                        } else {
                                            phi_33245_ = type_56();
                                        }
                                        let _e650 = phi_33245_;
                                        phi_33248_ = _e650;
                                        phi_33249_ = select(true, false, _e628);
                                    } else {
                                        phi_33248_ = type_56();
                                        phi_33249_ = true;
                                    }
                                    let _e653 = phi_33248_;
                                    let _e655 = phi_33249_;
                                    if _e655 {
                                        phi_33254_ = _e624;
                                    } else {
                                        phi_33254_ = _e653;
                                    }
                                    let _e657 = phi_33254_;
                                    if (_e657.member_1 < 0f) {
                                        phi_33052_ = true;
                                    } else {
                                        if (_e657.member > _e657.member_1) {
                                            phi_33049_ = true;
                                        } else {
                                            phi_33049_ = (_e657.member > _e370);
                                        }
                                        let _e664 = phi_33049_;
                                        phi_33052_ = _e664;
                                    }
                                    let _e666 = phi_33052_;
                                    phi_21359_ = select(_e657.member, 300000000000000000000000000000000000000f, _e666);
                                }
                                let _e669 = phi_21359_;
                                if (_e413.member_3 > 0u) {
                                    phi_21370_ = ((2147483648u | ((_e413.member_3 - 1u) << bitcast<u32>(28u))) | _e413.member_1);
                                } else {
                                    phi_21370_ = _e413.member_1;
                                }
                                let _e678 = phi_21370_;
                                if (_e413.member_7 > 0u) {
                                    phi_21381_ = ((2147483648u | ((_e413.member_7 - 1u) << bitcast<u32>(28u))) | _e413.member_5);
                                } else {
                                    phi_21381_ = _e413.member_5;
                                }
                                let _e687 = phi_21381_;
                                let _e688 = (_e541 <= _e669);
                                let _e692 = select(_e541, _e669, _e688);
                                if (_e692 < 300000000000000000000000000000000000000f) {
                                    let _e694 = (_e397 < 64u);
                                    if _e694 {
                                        if _e694 {
                                        } else {
                                            phi_38408_ = true;
                                            phi_21684_ = bool();
                                            phi_21683_ = vec4<f32>();
                                            break;
                                        }
                                        local_103[_e397] = select(_e678, _e687, _e688);
                                        if _e694 {
                                        } else {
                                            phi_38408_ = true;
                                            phi_21684_ = bool();
                                            phi_21683_ = vec4<f32>();
                                            break;
                                        }
                                        local_104[_e397] = _e692;
                                        phi_21420_ = (_e397 + 1u);
                                    } else {
                                        phi_21420_ = _e397;
                                    }
                                    let _e699 = phi_21420_;
                                    phi_21421_ = _e699;
                                } else {
                                    phi_21421_ = _e397;
                                }
                                let _e701 = phi_21421_;
                                phi_21432_ = select(4294967295u, select(_e687, _e678, _e688), (select(_e669, _e541, _e688) < 300000000000000000000000000000000000000f));
                                phi_21433_ = _e701;
                                phi_21434_ = u32();
                                phi_21435_ = u32();
                            } else {
                                let _e406 = (_e395 & 268435455u);
                                phi_21432_ = u32();
                                phi_21433_ = u32();
                                phi_21434_ = ((_e406 + ((_e395 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_21435_ = _e406;
                            }
                            let _e705 = phi_21432_;
                            let _e707 = phi_21433_;
                            let _e709 = phi_21434_;
                            let _e711 = phi_21435_;
                            phi_21437_ = _e705;
                            phi_21438_ = _e707;
                            phi_21440_ = _e709;
                            phi_21441_ = _e711;
                            phi_21442_ = select(true, false, _e405);
                            phi_21443_ = _e405;
                        } else {
                            phi_21437_ = u32();
                            phi_21438_ = u32();
                            phi_21440_ = _e364;
                            phi_21441_ = _e366;
                            phi_21442_ = _e401;
                            phi_21443_ = false;
                        }
                        let _e714 = phi_21437_;
                        let _e716 = phi_21438_;
                        let _e718 = phi_21440_;
                        let _e720 = phi_21441_;
                        let _e722 = phi_21442_;
                        let _e724 = phi_21443_;
                        phi_20927_ = _e358;
                        phi_20930_ = select(_e714, 4294967295u, _e722);
                        phi_20932_ = select(_e716, select(_e393, _e397, _e399), _e722);
                        phi_20934_ = select(_e364, _e718, _e722);
                        phi_20936_ = select(_e366, _e720, _e722);
                        phi_20938_ = _e368;
                        phi_20940_ = _e370;
                        phi_21454_ = vec4<f32>();
                        phi_21455_ = select(_e724, true, _e722);
                        phi_21456_ = select(select(_e403, false, _e399), false, _e722);
                    }
                    let _e952 = phi_20927_;
                    let _e954 = phi_20930_;
                    let _e956 = phi_20932_;
                    let _e958 = phi_20934_;
                    let _e960 = phi_20936_;
                    let _e962 = phi_20938_;
                    let _e964 = phi_20940_;
                    let _e966 = phi_21454_;
                    let _e968 = phi_21455_;
                    let _e970 = phi_21456_;
                    phi_20926_ = _e952;
                    phi_20929_ = _e954;
                    phi_20931_ = _e956;
                    phi_20933_ = _e958;
                    phi_20935_ = _e960;
                    phi_20937_ = _e962;
                    phi_20939_ = _e964;
                    phi_38408_ = false;
                    phi_21684_ = _e970;
                    phi_21683_ = _e966;
                    if !(_e968) {
                        break;
                    }
                }
                let _e973 = phi_38408_;
                let _e975 = phi_21684_;
                let _e977 = phi_21683_;
                if _e973 {
                    break;
                }
                if _e975 {
                    let _e979 = local_105;
                    let _e981 = local_106;
                    phi_21461_ = type_74(_e979, _e981);
                } else {
                    phi_21461_ = type_74();
                }
                let _e984 = phi_21461_;
                if select(true, false, _e975) {
                    let _e987 = local_107;
                    phi_21467_ = type_74(_e987, _e977);
                } else {
                    phi_21467_ = _e984;
                }
                let _e990 = phi_21467_;
                let _e993 = global_7.member[_e324.member_1];
                let _e1005 = type_34(_e993.member, _e993.member_1, _e993.member_2, _e993.member_3, vec4<u32>(_e993.member_4.x, _e993.member_4.y, _e990.member, _e993.member_4.w), _e993.member_5);
                if _e329 {
                } else {
                    break;
                }
                global_7.member[_e324.member_1] = type_34(_e1005.member, _e1005.member_1, _e1005.member_2, _e1005.member_3, _e1005.member_4, _e990.member_1);
            }
            break;
        }
    }
    return;
}

fn function_3() {
    var phi_21501_: type_48;
    var phi_21515_: u32;
    var phi_21568_: bool;
    var phi_21569_: bool;
    var phi_21570_: bool;
    var phi_21586_: vec4<f32>;
    var phi_21516_: u32;
    var phi_38530_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e277 = global;
            let _e281 = arrayLength((&global_11.member));
            let _e285 = (_e277.x + (_e277.y * 4194240u));
            let _e289 = global_1.member.member[0u];
            let _e293 = global_1.member.member[1u];
            let _e294 = (_e289 * _e293);
            if (_e285 >= _e294) {
                phi_21501_ = type_48(0u, type_48().member_1);
            } else {
                phi_21501_ = type_48(1u, _e285);
            }
            let _e301 = phi_21501_;
            if (bitcast<i32>(_e301.member) != 0i) {
                phi_21515_ = 0u;
                loop {
                    let _e307 = phi_21515_;
                    let _e311 = global_1.member.member_7[3u];
                    let _e312 = (_e307 < _e311);
                    if _e312 {
                        let _e314 = ((_e307 * _e294) + _e301.member_1);
                        if (_e314 < arrayLength((&global_10.member))) {
                        } else {
                            phi_38530_ = true;
                            break;
                        }
                        let _e320 = global_10.member[_e314].member_4[1u];
                        if ((_e320 & 8u) == 0u) {
                            let _e326 = global_10.member[_e314].member_3;
                            let _e334 = global_10.member[_e314].member[3u];
                            let _e335 = (_e326.x * _e334);
                            let _e336 = (_e326.y * _e334);
                            let _e337 = (_e326.z * _e334);
                            if (_e335 == _e335) {
                                let _e340 = (_e336 == _e336);
                                if _e340 {
                                    phi_21568_ = select(true, false, (_e337 == _e337));
                                } else {
                                    phi_21568_ = false;
                                }
                                let _e344 = phi_21568_;
                                phi_21569_ = select(true, false, _e340);
                                phi_21570_ = _e344;
                            } else {
                                phi_21569_ = true;
                                phi_21570_ = false;
                            }
                            let _e347 = phi_21569_;
                            let _e349 = phi_21570_;
                            let _e352 = select(vec3<f32>(_e335, _e336, _e337), vec3<f32>(0f, 0f, 0f), vec3(select(_e349, true, _e347)));
                            let _e357 = global_10.member[_e314].member_3[3u];
                            phi_21586_ = vec4<f32>(_e352.x, _e352.y, _e352.z, _e357);
                        } else {
                            phi_21586_ = vec4<f32>(0f, 0f, 0f, 0f);
                        }
                        let _e363 = phi_21586_;
                        if (_e363.w > 0f) {
                            let _e369 = (2u * _e301.member_1);
                            if (_e369 < _e281) {
                            } else {
                                phi_38530_ = true;
                                break;
                            }
                            let _e374 = global_11.member[_e369][0u];
                            global_11.member[_e369][0u] = (_e374 + _e363.x);
                            let _e379 = global_11.member[_e369][1u];
                            global_11.member[_e369][1u] = (_e379 + _e363.y);
                            let _e384 = global_11.member[_e369][2u];
                            global_11.member[_e369][2u] = (_e384 + _e363.z);
                            let _e389 = global_11.member[_e369][3u];
                            global_11.member[_e369][3u] = (_e389 + _e363.w);
                            let _e391 = (_e369 + 1u);
                            if (_e391 < _e281) {
                            } else {
                                phi_38530_ = true;
                                break;
                            }
                            let _e399 = global_11.member[_e391][0u];
                            global_11.member[_e391][0u] = (_e399 + (_e363.x * _e363.x));
                            let _e404 = global_11.member[_e391][1u];
                            global_11.member[_e391][1u] = (_e404 + (_e363.y * _e363.y));
                            let _e409 = global_11.member[_e391][2u];
                            global_11.member[_e391][2u] = (_e409 + (_e363.z * _e363.z));
                            let _e414 = global_11.member[_e391][3u];
                            global_11.member[_e391][3u] = (_e414 + 1f);
                        }
                        phi_21516_ = (_e307 + 1u);
                    } else {
                        phi_21516_ = u32();
                    }
                    let _e418 = phi_21516_;
                    phi_21515_ = _e418;
                    phi_38530_ = false;
                    if !(_e312) {
                        break;
                    }
                }
                let _e421 = phi_38530_;
                if _e421 {
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
