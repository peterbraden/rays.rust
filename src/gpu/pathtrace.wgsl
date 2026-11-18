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
    member_5: vec4<f32>,
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

struct type_70 {
    member: type_35,
    member_1: type_42,
    member_2: type_42,
}

struct type_74 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

struct type_76 {
    member: u32,
    member_1: vec4<f32>,
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
    var phi_492_: type_49;
    var phi_32411_: bool;
    var phi_512_: vec3<f32>;
    var phi_515_: u32;
    var phi_20676_: f32;
    var phi_20688_: type_57;
    var phi_20699_: type_58;
    var phi_20700_: type_58;
    var phi_20704_: type_58;
    var phi_20705_: type_58;
    var phi_20706_: bool;
    var phi_20710_: type_58;
    var phi_20597_: type_58;
    var phi_20748_: bool;
    var phi_20828_: f32;
    var phi_20840_: type_57;
    var phi_20851_: type_58;
    var phi_20852_: type_58;
    var phi_20856_: type_58;
    var phi_20857_: type_58;
    var phi_20858_: bool;
    var phi_20862_: type_58;
    var phi_20796_: f32;
    var phi_20900_: bool;
    var phi_20625_: f32;
    var phi_20627_: f32;
    var phi_20636_: type_58;
    var phi_20640_: type_58;
    var phi_20644_: type_58;
    var phi_20915_: bool;
    var phi_9515_: vec3<f32>;
    var phi_9518_: vec3<f32>;
    var phi_9520_: vec3<f32>;
    var phi_9522_: vec3<f32>;
    var phi_9524_: u32;
    var phi_9608_: u32;
    var phi_9623_: type_57;
    var phi_9624_: type_57;
    var phi_9625_: type_57;
    var phi_21115_: bool;
    var phi_21242_: bool;
    var phi_21257_: bool;
    var phi_9953_: vec3<f32>;
    var phi_10026_: vec3<f32>;
    var phi_10027_: vec3<f32>;
    var phi_10028_: bool;
    var phi_10033_: vec3<f32>;
    var phi_10034_: bool;
    var phi_21381_: bool;
    var phi_21396_: bool;
    var phi_10325_: vec3<f32>;
    var phi_9516_: vec3<f32>;
    var phi_9519_: vec3<f32>;
    var phi_9521_: vec3<f32>;
    var phi_9523_: vec3<f32>;
    var phi_9525_: u32;
    var phi_32394_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_32878_: bool;
    var phi_10382_: type_61;
    var phi_32877_: bool;
    var phi_10384_: type_61;
    var phi_10385_: bool;
    var phi_10390_: type_61;
    var phi_891_: vec3<f32>;
    var phi_893_: vec3<f32>;
    var phi_894_: bool;
    var phi_32486_: bool;
    var phi_904_: vec3<f32>;
    var phi_907_: i32;
    var phi_909_: u32;
    var phi_21665_: bool;
    var phi_1331_: u32;
    var phi_1361_: u32;
    var phi_1332_: u32;
    var phi_1362_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1380_: u32;
    var phi_2685_: type_56;
    var phi_2693_: type_53;
    var phi_2695_: type_53;
    var phi_2696_: bool;
    var phi_2697_: type_53;
    var phi_2698_: bool;
    var phi_2699_: bool;
    var phi_21776_: f32;
    var phi_21788_: type_57;
    var phi_21799_: type_58;
    var phi_21800_: type_58;
    var phi_21804_: type_58;
    var phi_21805_: type_58;
    var phi_21806_: bool;
    var phi_21810_: type_58;
    var phi_21697_: type_58;
    var phi_21848_: bool;
    var phi_21928_: f32;
    var phi_21940_: type_57;
    var phi_21951_: type_58;
    var phi_21952_: type_58;
    var phi_21956_: type_58;
    var phi_21957_: type_58;
    var phi_21958_: bool;
    var phi_21962_: type_58;
    var phi_21896_: f32;
    var phi_22000_: bool;
    var phi_21725_: f32;
    var phi_21727_: f32;
    var phi_21736_: type_58;
    var phi_21740_: type_58;
    var phi_21744_: type_58;
    var phi_2717_: vec3<f32>;
    var phi_2720_: vec3<f32>;
    var phi_2722_: vec3<f32>;
    var phi_2724_: vec3<f32>;
    var phi_2726_: u32;
    var phi_2810_: u32;
    var phi_2825_: type_57;
    var phi_2826_: type_57;
    var phi_2827_: type_57;
    var phi_22199_: bool;
    var phi_22326_: bool;
    var phi_22341_: bool;
    var phi_3155_: vec3<f32>;
    var phi_22463_: f32;
    var phi_22475_: type_57;
    var phi_22486_: type_58;
    var phi_22487_: type_58;
    var phi_22491_: type_58;
    var phi_22492_: type_58;
    var phi_22493_: bool;
    var phi_22497_: type_58;
    var phi_22431_: f32;
    var phi_22573_: f32;
    var phi_22585_: type_57;
    var phi_22596_: type_58;
    var phi_22597_: type_58;
    var phi_22601_: type_58;
    var phi_22602_: type_58;
    var phi_22603_: bool;
    var phi_22607_: type_58;
    var phi_22541_: f32;
    var phi_22368_: vec3<f32>;
    var phi_22369_: u32;
    var phi_22739_: bool;
    var phi_22402_: vec3<f32>;
    var phi_22403_: u32;
    var local_11: vec3<f32>;
    var phi_22410_: type_59;
    var phi_22414_: type_59;
    var phi_3214_: vec3<f32>;
    var phi_3215_: vec3<f32>;
    var phi_2718_: vec3<f32>;
    var phi_2721_: vec3<f32>;
    var phi_2723_: vec3<f32>;
    var phi_2725_: vec3<f32>;
    var phi_2727_: u32;
    var phi_32454_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_32548_: bool;
    var phi_3235_: type_53;
    var phi_32547_: bool;
    var phi_3236_: type_53;
    var phi_32546_: bool;
    var phi_3259_: vec3<f32>;
    var phi_3260_: i32;
    var phi_3261_: u32;
    var phi_32545_: bool;
    var phi_905_: vec3<f32>;
    var phi_908_: i32;
    var phi_910_: u32;
    var phi_3262_: bool;
    var phi_32544_: bool;
    var phi_3596_: vec3<f32>;
    var phi_3599_: u32;
    var phi_22867_: f32;
    var phi_22882_: f32;
    var phi_22897_: f32;
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
    var phi_32549_: bool;
    var local_15: vec3<f32>;
    var phi_32597_: bool;
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
    var phi_23013_: f32;
    var phi_23025_: type_57;
    var phi_23036_: type_58;
    var phi_23037_: type_58;
    var phi_23041_: type_58;
    var phi_23042_: type_58;
    var phi_23043_: bool;
    var phi_23047_: type_58;
    var phi_22981_: f32;
    var phi_4613_: bool;
    var phi_23173_: f32;
    var phi_23185_: type_57;
    var phi_23196_: type_58;
    var phi_23197_: type_58;
    var phi_23201_: type_58;
    var phi_23202_: type_58;
    var phi_23203_: bool;
    var phi_23207_: type_58;
    var phi_23094_: type_58;
    var phi_23245_: bool;
    var phi_23325_: f32;
    var phi_23337_: type_57;
    var phi_23348_: type_58;
    var phi_23349_: type_58;
    var phi_23353_: type_58;
    var phi_23354_: type_58;
    var phi_23355_: bool;
    var phi_23359_: type_58;
    var phi_23293_: f32;
    var phi_23397_: bool;
    var phi_23122_: f32;
    var phi_23124_: f32;
    var phi_23133_: type_58;
    var phi_23137_: type_58;
    var phi_23141_: type_58;
    var phi_4627_: vec3<f32>;
    var phi_4630_: u32;
    var phi_4714_: u32;
    var phi_4729_: type_57;
    var phi_4730_: type_57;
    var phi_4731_: type_57;
    var phi_23596_: bool;
    var phi_4628_: vec3<f32>;
    var phi_4631_: u32;
    var phi_32576_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_32643_: bool;
    var phi_4756_: vec3<f32>;
    var phi_32642_: bool;
    var phi_4758_: vec3<f32>;
    var phi_4760_: bool;
    var phi_32638_: bool;
    var phi_4771_: vec3<f32>;
    var phi_4772_: bool;
    var phi_32637_: bool;
    var phi_4774_: vec3<f32>;
    var phi_4775_: bool;
    var phi_4776_: bool;
    var phi_32636_: bool;
    var phi_4780_: vec3<f32>;
    var phi_4781_: vec3<f32>;
    var phi_4782_: bool;
    var phi_4783_: vec3<f32>;
    var phi_4784_: bool;
    var phi_5104_: vec3<f32>;
    var phi_5107_: u32;
    var phi_23667_: f32;
    var phi_23682_: f32;
    var phi_23697_: f32;
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
    var phi_32631_: bool;
    var phi_32673_: bool;
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
    var phi_23712_: vec3<f32>;
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
    var phi_23767_: f32;
    var phi_23773_: f32;
    var phi_23801_: f32;
    var phi_23807_: f32;
    var phi_23835_: f32;
    var phi_23841_: f32;
    var phi_23869_: f32;
    var phi_23875_: f32;
    var phi_23903_: f32;
    var phi_23909_: f32;
    var phi_23937_: f32;
    var phi_23943_: f32;
    var phi_23971_: f32;
    var phi_23977_: f32;
    var phi_24005_: f32;
    var phi_24011_: f32;
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
    var phi_24087_: bool;
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
    var phi_24176_: f32;
    var phi_24182_: f32;
    var phi_24210_: f32;
    var phi_24216_: f32;
    var phi_24244_: f32;
    var phi_24250_: f32;
    var phi_24278_: f32;
    var phi_24284_: f32;
    var phi_24312_: f32;
    var phi_24318_: f32;
    var phi_24346_: f32;
    var phi_24352_: f32;
    var phi_24380_: f32;
    var phi_24386_: f32;
    var phi_24414_: f32;
    var phi_24420_: f32;
    var phi_7184_: u32;
    var phi_7187_: f32;
    var phi_7189_: f32;
    var phi_7191_: f32;
    var phi_7193_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7606_: f32;
    var phi_24466_: bool;
    var phi_24481_: bool;
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
    var phi_24500_: vec3<f32>;
    var phi_8147_: u32;
    var phi_8148_: vec3<f32>;
    var phi_8149_: u32;
    var phi_8150_: vec3<f32>;
    var phi_8285_: u32;
    var phi_8288_: f32;
    var phi_8290_: f32;
    var phi_8292_: f32;
    var phi_8294_: vec3<f32>;
    var phi_24555_: f32;
    var phi_24561_: f32;
    var phi_24589_: f32;
    var phi_24595_: f32;
    var phi_24623_: f32;
    var phi_24629_: f32;
    var phi_24657_: f32;
    var phi_24663_: f32;
    var phi_24691_: f32;
    var phi_24697_: f32;
    var phi_24725_: f32;
    var phi_24731_: f32;
    var phi_24759_: f32;
    var phi_24765_: f32;
    var phi_24793_: f32;
    var phi_24799_: f32;
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
    var phi_9040_: type_60;
    var local_42: f32;
    var phi_9041_: type_60;
    var local_43: f32;
    var local_44: f32;
    var phi_9042_: type_60;
    var phi_9043_: type_60;
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
    var phi_32881_: bool;
    var phi_9119_: vec3<f32>;
    var phi_32880_: bool;
    var phi_9120_: vec3<f32>;
    var phi_32879_: bool;
    var phi_9121_: vec3<f32>;
    var phi_32875_: bool;
    var phi_10394_: vec3<f32>;
    var phi_32874_: bool;
    var phi_10395_: vec3<f32>;
    var phi_32873_: bool;
    var phi_513_: vec3<f32>;
    var phi_516_: u32;
    var phi_32872_: bool;
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
                phi_492_ = type_49(0u, type_49().member_1);
            } else {
                let _e300 = global_1.member.member[0u];
                let _e305 = global_1.member.member[1u];
                let _e311 = atomicLoad((&global_8.member[((8u + ((_e286 * _e300) * _e305)) + _e290)]));
                phi_492_ = type_49(1u, _e311);
            }
            let _e317 = phi_492_;
            if (bitcast<i32>(_e317.member) != 0i) {
                phi_32411_ = false;
                phi_512_ = vec3<f32>(0f, 0f, 0f);
                phi_515_ = 0u;
                loop {
                    let _e323 = phi_32411_;
                    let _e325 = phi_512_;
                    let _e327 = phi_515_;
                    local_55 = _e325;
                    local_56 = _e325;
                    local_57 = _e325;
                    let _e328 = (_e327 < 2u);
                    if _e328 {
                        let _e330 = ((2u * _e317.member_1) + _e327);
                        if (_e330 < arrayLength((&global_9.member))) {
                        } else {
                            phi_32872_ = true;
                            break;
                        }
                        let _e334 = global_9.member[_e330];
                        if ((((_e334.member_2.x * _e334.member_2.x) + (_e334.member_2.y * _e334.member_2.y)) + (_e334.member_2.z * _e334.member_2.z)) == 0f) {
                            phi_32874_ = _e323;
                            phi_10395_ = vec3<f32>(0f, 0f, 0f);
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
                                            phi_32872_ = true;
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
                                            phi_32872_ = true;
                                            break;
                                        }
                                        let _e1340 = (bitcast<u32>((select(0i, select(select(i32(_e1325), i32(-2147483648), (_e1325 < -2147483600f)), 2147483647i, (_e1325 > 2147483500f)), (_e1325 == _e1325)) + bitcast<i32>(_e1297))) % _e1297);
                                        let _e1345 = select(select(u32(_e1326), 0u, (_e1326 < 0f)), 4294967295u, (_e1326 > 4294967000f));
                                        let _e1346 = (_e1345 + 1u);
                                        let _e1351 = global_1.member.member_2[2u];
                                        let _e1352 = (_e1345 * _e1297);
                                        if _e1339 {
                                            phi_32872_ = true;
                                            break;
                                        }
                                        let _e1353 = (_e1340 % _e1297);
                                        let _e1356 = (_e1351 + (3u * (_e1352 + _e1353)));
                                        let _e1358 = global_6.member[_e1356];
                                        let _e1361 = global_6.member[(_e1356 + 1u)];
                                        let _e1364 = global_6.member[(_e1356 + 2u)];
                                        if _e1339 {
                                            phi_32872_ = true;
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
                                            phi_32872_ = true;
                                            break;
                                        }
                                        let _e1390 = (_e1351 + (3u * (_e1387 + _e1353)));
                                        let _e1392 = global_6.member[_e1390];
                                        let _e1395 = global_6.member[(_e1390 + 1u)];
                                        let _e1398 = global_6.member[(_e1390 + 2u)];
                                        if _e1339 {
                                            phi_32872_ = true;
                                            break;
                                        }
                                        let _e1402 = (_e1351 + (3u * (_e1387 + _e1367)));
                                        let _e1404 = global_6.member[_e1402];
                                        let _e1407 = global_6.member[(_e1402 + 1u)];
                                        let _e1410 = global_6.member[(_e1402 + 2u)];
                                        let _e1418 = (vec3<f32>(_e1392, _e1395, _e1398) + vec3<f32>(((_e1404 - _e1392) * _e1327), ((_e1407 - _e1395) * _e1327), ((_e1410 - _e1398) * _e1327)));
                                        phi_891_ = (_e1386 + vec3<f32>(((_e1418.x - _e1386.x) * _e1328), ((_e1418.y - _e1386.y) * _e1328), ((_e1418.z - _e1386.z) * _e1328)));
                                    } else {
                                        phi_891_ = vec3<f32>();
                                    }
                                    let _e1434 = phi_891_;
                                    phi_893_ = _e1434;
                                    phi_894_ = select(true, false, _e1294);
                                } else {
                                    phi_893_ = vec3<f32>();
                                    phi_894_ = true;
                                }
                                let _e1437 = phi_893_;
                                let _e1439 = phi_894_;
                                if _e1439 {
                                    let _e1440 = (_e1292 != 4294967295u);
                                    if (_e1289 != 4294967295u) {
                                        phi_32486_ = _e323;
                                        phi_904_ = vec3<f32>(0f, 0f, 0f);
                                        phi_907_ = 0i;
                                        phi_909_ = _e1289;
                                        loop {
                                            let _e1443 = phi_32486_;
                                            let _e1445 = phi_904_;
                                            let _e1447 = phi_907_;
                                            let _e1449 = phi_909_;
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
                                                        phi_2697_ = type_53();
                                                        phi_2698_ = true;
                                                        phi_2699_ = false;
                                                    } else {
                                                        let _e1528 = (_e1522 == 4294967295u);
                                                        if _e1528 {
                                                            phi_2695_ = type_53();
                                                            phi_2696_ = false;
                                                        } else {
                                                            let _e1542 = (_e334.member.x - _e1453);
                                                            let _e1543 = (_e334.member.y - _e1456);
                                                            let _e1544 = (_e334.member.z - _e1459);
                                                            let _e1552 = ((((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e1453) + (_e334.member.y * _e1456)) + (_e334.member.z * _e1459)))) + _e1468) / (sqrt((((_e1542 * _e1542) + (_e1543 * _e1543)) + (_e1544 * _e1544))) + _e1462));
                                                            let _e1553 = (_e1449 + 21u);
                                                            let _e1556 = global_6.member[(_e1449 + 28u)];
                                                            if (_e1552 > _e1556) {
                                                                phi_2685_ = type_56(0u, type_56().member_1);
                                                            } else {
                                                                let _e1559 = global_6.member[_e1553];
                                                                if (_e1552 != _e1552) {
                                                                    phi_21665_ = true;
                                                                } else {
                                                                    phi_21665_ = (_e1559 >= _e1552);
                                                                }
                                                                let _e1563 = phi_21665_;
                                                                let _e1564 = select(_e1552, _e1559, _e1563);
                                                                phi_1331_ = 0u;
                                                                loop {
                                                                    let _e1566 = phi_1331_;
                                                                    local_8 = _e1566;
                                                                    local_9 = _e1566;
                                                                    local_10 = _e1566;
                                                                    if (_e1566 < 8u) {
                                                                        let _e1570 = global_6.member[(_e1553 + _e1566)];
                                                                        let _e1571 = (_e1570 <= _e1564);
                                                                        if _e1571 {
                                                                            phi_1361_ = (_e1566 + 1u);
                                                                        } else {
                                                                            phi_1361_ = u32();
                                                                        }
                                                                        let _e1574 = phi_1361_;
                                                                        phi_1332_ = _e1574;
                                                                        phi_1362_ = _e1571;
                                                                    } else {
                                                                        phi_1332_ = u32();
                                                                        phi_1362_ = false;
                                                                    }
                                                                    let _e1576 = phi_1332_;
                                                                    let _e1578 = phi_1362_;
                                                                    phi_1331_ = _e1576;
                                                                    if !(_e1578) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1581 = local_8;
                                                                if (_e1581 < 1u) {
                                                                    phi_1380_ = 1u;
                                                                } else {
                                                                    let _e1584 = local_9;
                                                                    let _e1587 = local_10;
                                                                    phi_1380_ = select(_e1587, 7u, (_e1584 > 7u));
                                                                }
                                                                let _e1590 = phi_1380_;
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
                                                                phi_2685_ = type_56(1u, type_53(vec3<f32>(((((((_e1657 * _e1652) + (_e1668 * _e1653)) + (_e1682 * _e1654)) + (_e1696 * _e1655)) * _e1996) + (((((_e1830 * _e1652) + (_e1841 * _e1653)) + (_e1855 * _e1654)) + (_e1869 * _e1655)) * _e1600)), ((((((_e1660 * _e1652) + (_e1671 * _e1653)) + (_e1685 * _e1654)) + (_e1699 * _e1655)) * _e1996) + (((((_e1833 * _e1652) + (_e1844 * _e1653)) + (_e1858 * _e1654)) + (_e1872 * _e1655)) * _e1600)), ((((((_e1663 * _e1652) + (_e1674 * _e1653)) + (_e1688 * _e1654)) + (_e1702 * _e1655)) * _e1996) + (((((_e1836 * _e1652) + (_e1847 * _e1653)) + (_e1861 * _e1654)) + (_e1875 * _e1655)) * _e1600))), vec3<f32>(((((((_e1711 * _e1652) + (_e1723 * _e1653)) + (_e1738 * _e1654)) + (_e1753 * _e1655)) * _e1996) + (((((_e1884 * _e1652) + (_e1896 * _e1653)) + (_e1911 * _e1654)) + (_e1926 * _e1655)) * _e1600)), ((((((_e1714 * _e1652) + (_e1726 * _e1653)) + (_e1741 * _e1654)) + (_e1756 * _e1655)) * _e1996) + (((((_e1887 * _e1652) + (_e1899 * _e1653)) + (_e1914 * _e1654)) + (_e1929 * _e1655)) * _e1600)), ((((((_e1717 * _e1652) + (_e1729 * _e1653)) + (_e1744 * _e1654)) + (_e1759 * _e1655)) * _e1996) + (((((_e1890 * _e1652) + (_e1902 * _e1653)) + (_e1917 * _e1654)) + (_e1932 * _e1655)) * _e1600))), vec3<f32>(((((((_e1768 * _e1652) + (_e1780 * _e1653)) + (_e1795 * _e1654)) + (_e1810 * _e1655)) * _e1996) + (((((_e1941 * _e1652) + (_e1953 * _e1653)) + (_e1968 * _e1654)) + (_e1983 * _e1655)) * _e1600)), ((((((_e1771 * _e1652) + (_e1783 * _e1653)) + (_e1798 * _e1654)) + (_e1813 * _e1655)) * _e1996) + (((((_e1944 * _e1652) + (_e1956 * _e1653)) + (_e1971 * _e1654)) + (_e1986 * _e1655)) * _e1600)), ((((((_e1774 * _e1652) + (_e1786 * _e1653)) + (_e1801 * _e1654)) + (_e1816 * _e1655)) * _e1996) + (((((_e1947 * _e1652) + (_e1959 * _e1653)) + (_e1974 * _e1654)) + (_e1989 * _e1655)) * _e1600)))));
                                                            }
                                                            let _e2033 = phi_2685_;
                                                            let _e2036 = (bitcast<i32>(_e2033.member) != 0i);
                                                            if _e2036 {
                                                                phi_2693_ = _e2033.member_1;
                                                            } else {
                                                                phi_2693_ = type_53();
                                                            }
                                                            let _e2039 = phi_2693_;
                                                            phi_2695_ = _e2039;
                                                            phi_2696_ = select(true, false, _e2036);
                                                        }
                                                        let _e2042 = phi_2695_;
                                                        let _e2044 = phi_2696_;
                                                        phi_2697_ = _e2042;
                                                        phi_2698_ = _e1528;
                                                        phi_2699_ = _e2044;
                                                    }
                                                    let _e2046 = phi_2697_;
                                                    let _e2048 = phi_2698_;
                                                    let _e2050 = phi_2699_;
                                                    if select(_e2050, true, _e2048) {
                                                        let _e2059 = ((((_e334.member.x - _e1453) * _e334.member_1.x) + ((_e334.member.y - _e1456) * _e334.member_1.y)) + ((_e334.member.z - _e1459) * _e334.member_1.z));
                                                        let _e2071 = ((((_e334.member.x * _e334.member.x) + (_e334.member.y * _e334.member.y)) + (_e334.member.z * _e334.member.z)) - (2f * (((_e334.member.x * _e1453) + (_e334.member.y * _e1456)) + (_e334.member.z * _e1459))));
                                                        let _e2072 = (_e2071 + _e1471);
                                                        let _e2073 = (_e2059 * _e2059);
                                                        let _e2074 = (_e2073 - _e2072);
                                                        if (_e2074 < 0f) {
                                                            phi_21804_ = type_58(0u, type_58().member_1);
                                                            phi_21805_ = type_58();
                                                            phi_21806_ = true;
                                                        } else {
                                                            if (_e2059 < 0f) {
                                                                phi_21776_ = -(sqrt(_e2074));
                                                            } else {
                                                                phi_21776_ = sqrt(_e2074);
                                                            }
                                                            let _e2081 = phi_21776_;
                                                            let _e2083 = -((_e2059 + _e2081));
                                                            let _e2084 = (_e2083 == 0f);
                                                            if _e2084 {
                                                                phi_21799_ = type_58(1u, type_57(0f, 0f));
                                                                phi_21800_ = type_58();
                                                            } else {
                                                                let _e2085 = (_e2072 / _e2083);
                                                                if (_e2083 < _e2085) {
                                                                    phi_21788_ = type_57(_e2083, _e2085);
                                                                } else {
                                                                    phi_21788_ = type_57(_e2085, _e2083);
                                                                }
                                                                let _e2090 = phi_21788_;
                                                                phi_21799_ = type_58();
                                                                phi_21800_ = type_58(1u, _e2090);
                                                            }
                                                            let _e2093 = phi_21799_;
                                                            let _e2095 = phi_21800_;
                                                            phi_21804_ = _e2093;
                                                            phi_21805_ = _e2095;
                                                            phi_21806_ = _e2084;
                                                        }
                                                        let _e2100 = phi_21804_;
                                                        let _e2102 = phi_21805_;
                                                        let _e2104 = phi_21806_;
                                                        if _e2104 {
                                                            phi_21810_ = _e2100;
                                                        } else {
                                                            phi_21810_ = _e2102;
                                                        }
                                                        let _e2106 = phi_21810_;
                                                        if (bitcast<i32>(_e2106.member) != 0i) {
                                                            phi_21697_ = type_58(0u, _e2106.member_1);
                                                        } else {
                                                            phi_21697_ = type_58(1u, type_58().member_1);
                                                        }
                                                        let _e2116 = phi_21697_;
                                                        if (bitcast<i32>(_e2116.member) != 0i) {
                                                            phi_21744_ = type_58(0u, type_58().member_1);
                                                        } else {
                                                            if (_e2116.member_1.member_1 <= 0f) {
                                                                phi_21740_ = type_58(0u, type_58().member_1);
                                                            } else {
                                                                if (_e2116.member_1.member != _e2116.member_1.member) {
                                                                    phi_21848_ = true;
                                                                } else {
                                                                    phi_21848_ = (0f >= _e2116.member_1.member);
                                                                }
                                                                let _e2128 = phi_21848_;
                                                                let _e2129 = select(_e2116.member_1.member, 0f, _e2128);
                                                                let _e2130 = (_e2071 + _e1468);
                                                                if (_e2130 > 0f) {
                                                                    let _e2132 = (_e2073 - _e2130);
                                                                    if (_e2132 < 0f) {
                                                                        phi_21956_ = type_58(0u, type_58().member_1);
                                                                        phi_21957_ = type_58();
                                                                        phi_21958_ = true;
                                                                    } else {
                                                                        if (_e2059 < 0f) {
                                                                            phi_21928_ = -(sqrt(_e2132));
                                                                        } else {
                                                                            phi_21928_ = sqrt(_e2132);
                                                                        }
                                                                        let _e2139 = phi_21928_;
                                                                        let _e2141 = -((_e2059 + _e2139));
                                                                        let _e2142 = (_e2141 == 0f);
                                                                        if _e2142 {
                                                                            phi_21951_ = type_58(1u, type_57(0f, 0f));
                                                                            phi_21952_ = type_58();
                                                                        } else {
                                                                            let _e2143 = (_e2130 / _e2141);
                                                                            if (_e2141 < _e2143) {
                                                                                phi_21940_ = type_57(_e2141, _e2143);
                                                                            } else {
                                                                                phi_21940_ = type_57(_e2143, _e2141);
                                                                            }
                                                                            let _e2148 = phi_21940_;
                                                                            phi_21951_ = type_58();
                                                                            phi_21952_ = type_58(1u, _e2148);
                                                                        }
                                                                        let _e2151 = phi_21951_;
                                                                        let _e2153 = phi_21952_;
                                                                        phi_21956_ = _e2151;
                                                                        phi_21957_ = _e2153;
                                                                        phi_21958_ = _e2142;
                                                                    }
                                                                    let _e2158 = phi_21956_;
                                                                    let _e2160 = phi_21957_;
                                                                    let _e2162 = phi_21958_;
                                                                    if _e2162 {
                                                                        phi_21962_ = _e2158;
                                                                    } else {
                                                                        phi_21962_ = _e2160;
                                                                    }
                                                                    let _e2164 = phi_21962_;
                                                                    if (bitcast<i32>(_e2164.member) != 0i) {
                                                                        phi_21896_ = select(_e2164.member_1.member_1, _e2164.member_1.member, (_e2164.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_21896_ = -1f;
                                                                    }
                                                                    let _e2175 = phi_21896_;
                                                                    if (_e2175 >= 0f) {
                                                                        if (_e2116.member_1.member_1 != _e2116.member_1.member_1) {
                                                                            phi_22000_ = true;
                                                                        } else {
                                                                            phi_22000_ = (_e2175 <= _e2116.member_1.member_1);
                                                                        }
                                                                        let _e2180 = phi_22000_;
                                                                        phi_21725_ = select(_e2116.member_1.member_1, _e2175, _e2180);
                                                                    } else {
                                                                        phi_21725_ = _e2116.member_1.member_1;
                                                                    }
                                                                    let _e2183 = phi_21725_;
                                                                    phi_21727_ = _e2183;
                                                                } else {
                                                                    phi_21727_ = _e2116.member_1.member_1;
                                                                }
                                                                let _e2185 = phi_21727_;
                                                                if (_e2185 <= _e2129) {
                                                                    phi_21736_ = type_58(0u, type_58().member_1);
                                                                } else {
                                                                    phi_21736_ = type_58(1u, type_57(_e2129, _e2185));
                                                                }
                                                                let _e2193 = phi_21736_;
                                                                phi_21740_ = _e2193;
                                                            }
                                                            let _e2198 = phi_21740_;
                                                            phi_21744_ = _e2198;
                                                        }
                                                        let _e2203 = phi_21744_;
                                                        if (bitcast<i32>(_e2203.member) != 0i) {
                                                            phi_2717_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2720_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2722_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2724_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2726_ = 0u;
                                                            loop {
                                                                let _e2212 = phi_2717_;
                                                                let _e2214 = phi_2720_;
                                                                let _e2216 = phi_2722_;
                                                                let _e2218 = phi_2724_;
                                                                let _e2220 = phi_2726_;
                                                                local_12 = _e2218;
                                                                local_13 = _e2216;
                                                                local_14 = _e2214;
                                                                let _e2221 = (_e2220 < 32u);
                                                                if _e2221 {
                                                                    let _e2222 = -(_e2059);
                                                                    if (_e2203.member_1.member <= _e2203.member_1.member_1) {
                                                                    } else {
                                                                        phi_32454_ = true;
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
                                                                        phi_2827_ = type_57((_e2203.member_1.member + ((_e2230 * _e2311) * _e2311)), (_e2230 * ((_e2309 * _e2309) - (_e2306 * _e2306))));
                                                                    } else {
                                                                        if (_e2229 <= _e2231) {
                                                                            let _e2289 = (f32((31u - _e2220)) * 0.03125f);
                                                                            let _e2292 = (f32((32u - _e2220)) * 0.03125f);
                                                                            let _e2294 = ((_e2289 + _e2292) * 0.5f);
                                                                            phi_2826_ = type_57((_e2203.member_1.member_1 - ((_e2230 * _e2294) * _e2294)), (_e2230 * ((_e2292 * _e2292) - (_e2289 * _e2289))));
                                                                        } else {
                                                                            let _e2237 = round(((32f * _e2228) / (_e2228 + _e2229)));
                                                                            let _e2242 = select(select(u32(_e2237), 0u, (_e2237 < 0f)), 4294967295u, (_e2237 > 4294967000f));
                                                                            if (_e2242 < 1u) {
                                                                                phi_2810_ = 1u;
                                                                            } else {
                                                                                phi_2810_ = select(_e2242, 31u, (_e2242 > 31u));
                                                                            }
                                                                            let _e2247 = phi_2810_;
                                                                            if (_e2220 < _e2247) {
                                                                                let _e2268 = ((_e2247 - 1u) - _e2220);
                                                                                let _e2270 = f32(_e2247);
                                                                                let _e2271 = (f32(_e2268) / _e2270);
                                                                                let _e2274 = (f32((_e2268 + 1u)) / _e2270);
                                                                                let _e2276 = ((_e2271 + _e2274) * 0.5f);
                                                                                phi_2825_ = type_57((_e2227 - ((_e2228 * _e2276) * _e2276)), (_e2228 * ((_e2274 * _e2274) - (_e2271 * _e2271))));
                                                                            } else {
                                                                                let _e2250 = (_e2220 - _e2247);
                                                                                let _e2252 = f32((32u - _e2247));
                                                                                let _e2253 = (f32(_e2250) / _e2252);
                                                                                let _e2256 = (f32((_e2250 + 1u)) / _e2252);
                                                                                let _e2258 = ((_e2253 + _e2256) * 0.5f);
                                                                                phi_2825_ = type_57((_e2227 + ((_e2229 * _e2258) * _e2258)), (_e2229 * ((_e2256 * _e2256) - (_e2253 * _e2253))));
                                                                            }
                                                                            let _e2286 = phi_2825_;
                                                                            phi_2826_ = _e2286;
                                                                        }
                                                                        let _e2304 = phi_2826_;
                                                                        phi_2827_ = _e2304;
                                                                    }
                                                                    let _e2321 = phi_2827_;
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
                                                                        phi_22199_ = true;
                                                                    } else {
                                                                        phi_22199_ = (0f >= _e2358);
                                                                    }
                                                                    let _e2362 = phi_22199_;
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
                                                                        phi_3155_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2413 = (1f / _e2352);
                                                                        let _e2421 = ((((_e2344 * _e2413) * _e1474) + ((_e2345 * _e2413) * _e1477)) + ((_e2346 * _e2413) * _e1480));
                                                                        if (_e2354 != _e2354) {
                                                                            phi_22326_ = true;
                                                                        } else {
                                                                            phi_22326_ = (0f >= _e2354);
                                                                        }
                                                                        let _e2425 = phi_22326_;
                                                                        let _e2429 = sqrt((select(_e2354, 0f, _e2425) / (_e1465 - _e1462)));
                                                                        if (_e2429 != _e2429) {
                                                                            phi_22341_ = true;
                                                                        } else {
                                                                            phi_22341_ = (1f <= _e2429);
                                                                        }
                                                                        let _e2433 = phi_22341_;
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
                                                                        phi_3155_ = vec3<f32>(((((_e2470 * _e2477) + (_e2483 * _e2459)) * _e2496) + (((_e2501 * _e2477) + (_e2513 * _e2459)) * _e2457)), ((((_e2473 * _e2477) + (_e2486 * _e2459)) * _e2496) + (((_e2504 * _e2477) + (_e2516 * _e2459)) * _e2457)), ((((_e2476 * _e2477) + (_e2489 * _e2459)) * _e2496) + (((_e2507 * _e2477) + (_e2519 * _e2459)) * _e2457)));
                                                                    }
                                                                    let _e2534 = phi_3155_;
                                                                    let _e2552 = (((_e2344 * _e1474) + (_e2345 * _e1477)) + (_e2346 * _e1480));
                                                                    let _e2553 = (_e2552 * _e2552);
                                                                    let _e2554 = (_e2553 - _e2343);
                                                                    if (_e2554 < 0f) {
                                                                        phi_22491_ = type_58(0u, type_58().member_1);
                                                                        phi_22492_ = type_58();
                                                                        phi_22493_ = true;
                                                                    } else {
                                                                        if (_e2552 < 0f) {
                                                                            phi_22463_ = -(sqrt(_e2554));
                                                                        } else {
                                                                            phi_22463_ = sqrt(_e2554);
                                                                        }
                                                                        let _e2561 = phi_22463_;
                                                                        let _e2563 = -((_e2552 + _e2561));
                                                                        let _e2564 = (_e2563 == 0f);
                                                                        if _e2564 {
                                                                            phi_22486_ = type_58(1u, type_57(0f, 0f));
                                                                            phi_22487_ = type_58();
                                                                        } else {
                                                                            let _e2565 = (_e2343 / _e2563);
                                                                            if (_e2563 < _e2565) {
                                                                                phi_22475_ = type_57(_e2563, _e2565);
                                                                            } else {
                                                                                phi_22475_ = type_57(_e2565, _e2563);
                                                                            }
                                                                            let _e2570 = phi_22475_;
                                                                            phi_22486_ = type_58();
                                                                            phi_22487_ = type_58(1u, _e2570);
                                                                        }
                                                                        let _e2573 = phi_22486_;
                                                                        let _e2575 = phi_22487_;
                                                                        phi_22491_ = _e2573;
                                                                        phi_22492_ = _e2575;
                                                                        phi_22493_ = _e2564;
                                                                    }
                                                                    let _e2580 = phi_22491_;
                                                                    let _e2582 = phi_22492_;
                                                                    let _e2584 = phi_22493_;
                                                                    if _e2584 {
                                                                        phi_22497_ = _e2580;
                                                                    } else {
                                                                        phi_22497_ = _e2582;
                                                                    }
                                                                    let _e2586 = phi_22497_;
                                                                    if (bitcast<i32>(_e2586.member) != 0i) {
                                                                        phi_22431_ = select(_e2586.member_1.member_1, _e2586.member_1.member, (_e2586.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22431_ = -1f;
                                                                    }
                                                                    let _e2597 = phi_22431_;
                                                                    if (_e2597 >= 0f) {
                                                                        phi_22414_ = type_59(0u, type_59().member_1);
                                                                    } else {
                                                                        let _e2599 = (_e2342 + _e1471);
                                                                        let _e2600 = (_e2553 - _e2599);
                                                                        if (_e2600 < 0f) {
                                                                            phi_22601_ = type_58(0u, type_58().member_1);
                                                                            phi_22602_ = type_58();
                                                                            phi_22603_ = true;
                                                                        } else {
                                                                            if (_e2552 < 0f) {
                                                                                phi_22573_ = -(sqrt(_e2600));
                                                                            } else {
                                                                                phi_22573_ = sqrt(_e2600);
                                                                            }
                                                                            let _e2607 = phi_22573_;
                                                                            let _e2609 = -((_e2552 + _e2607));
                                                                            let _e2610 = (_e2609 == 0f);
                                                                            if _e2610 {
                                                                                phi_22596_ = type_58(1u, type_57(0f, 0f));
                                                                                phi_22597_ = type_58();
                                                                            } else {
                                                                                let _e2611 = (_e2599 / _e2609);
                                                                                if (_e2609 < _e2611) {
                                                                                    phi_22585_ = type_57(_e2609, _e2611);
                                                                                } else {
                                                                                    phi_22585_ = type_57(_e2611, _e2609);
                                                                                }
                                                                                let _e2616 = phi_22585_;
                                                                                phi_22596_ = type_58();
                                                                                phi_22597_ = type_58(1u, _e2616);
                                                                            }
                                                                            let _e2619 = phi_22596_;
                                                                            let _e2621 = phi_22597_;
                                                                            phi_22601_ = _e2619;
                                                                            phi_22602_ = _e2621;
                                                                            phi_22603_ = _e2610;
                                                                        }
                                                                        let _e2626 = phi_22601_;
                                                                        let _e2628 = phi_22602_;
                                                                        let _e2630 = phi_22603_;
                                                                        if _e2630 {
                                                                            phi_22607_ = _e2626;
                                                                        } else {
                                                                            phi_22607_ = _e2628;
                                                                        }
                                                                        let _e2632 = phi_22607_;
                                                                        if (bitcast<i32>(_e2632.member) != 0i) {
                                                                            phi_22541_ = select(_e2632.member_1.member_1, _e2632.member_1.member, (_e2632.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22541_ = -1f;
                                                                        }
                                                                        let _e2643 = phi_22541_;
                                                                        if (_e2643 < 0f) {
                                                                            phi_22410_ = type_59(0u, type_59().member_1);
                                                                        } else {
                                                                            phi_22368_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_22369_ = 0u;
                                                                            loop {
                                                                                let _e2646 = phi_22368_;
                                                                                let _e2648 = phi_22369_;
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
                                                                                        phi_22739_ = true;
                                                                                    } else {
                                                                                        phi_22739_ = (0f >= _e2698);
                                                                                    }
                                                                                    let _e2702 = phi_22739_;
                                                                                    let _e2704 = -(_e2694);
                                                                                    phi_22402_ = (_e2646 + vec3<f32>((exp((_e2704 / _e1483)) * _e2662), (exp((_e2704 / _e1486)) * _e2662), (select(_e2698, 0f, _e2702) * _e2662)));
                                                                                    phi_22403_ = _e2652;
                                                                                } else {
                                                                                    phi_22402_ = vec3<f32>();
                                                                                    phi_22403_ = u32();
                                                                                }
                                                                                let _e2715 = phi_22402_;
                                                                                let _e2717 = phi_22403_;
                                                                                phi_22368_ = _e2715;
                                                                                phi_22369_ = _e2717;
                                                                                if !(_e2649) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2720 = local_11;
                                                                            phi_22410_ = type_59(1u, _e2720);
                                                                        }
                                                                        let _e2726 = phi_22410_;
                                                                        phi_22414_ = _e2726;
                                                                    }
                                                                    let _e2731 = phi_22414_;
                                                                    if (bitcast<i32>(_e2731.member) != 0i) {
                                                                        let _e2736 = (vec3<f32>(_e2376, _e2378, _e2380) + _e2731.member_1);
                                                                        let _e2742 = (_e2385 * _e2736.y);
                                                                        let _e2754 = exp(-((((_e1492 * _e2736.x) + _e2742) + (_e1508 * _e2736.z))));
                                                                        let _e2756 = exp(-((((_e1495 * _e2736.x) + _e2742) + (_e1511 * _e2736.z))));
                                                                        let _e2758 = exp(-((((_e1498 * _e2736.x) + _e2742) + (_e1514 * _e2736.z))));
                                                                        phi_3214_ = vec3<f32>((_e2216.x + (_e2754 * _e2370)), (_e2216.y + (_e2756 * _e2370)), (_e2216.z + (_e2758 * _e2370)));
                                                                        phi_3215_ = vec3<f32>((_e2218.x + (_e2754 * _e2367)), (_e2218.y + (_e2756 * _e2367)), (_e2218.z + (_e2758 * _e2367)));
                                                                    } else {
                                                                        phi_3214_ = _e2216;
                                                                        phi_3215_ = _e2218;
                                                                    }
                                                                    let _e2780 = phi_3214_;
                                                                    let _e2782 = phi_3215_;
                                                                    phi_2718_ = vec3<f32>((_e2376 + _e2372), (_e2378 + _e2373), (_e2380 + _e2374));
                                                                    phi_2721_ = vec3<f32>((_e2214.x + ((exp(-((((_e1492 * _e2376) + _e2386) + (_e1508 * _e2380)))) * ((_e1492 * _e2367) + _e2405)) * _e2534.x)), (_e2214.y + ((exp(-((((_e1495 * _e2376) + _e2386) + (_e1511 * _e2380)))) * ((_e1495 * _e2367) + _e2405)) * _e2534.y)), (_e2214.z + ((exp(-((((_e1498 * _e2376) + _e2386) + (_e1514 * _e2380)))) * ((_e1498 * _e2367) + _e2405)) * _e2534.z)));
                                                                    phi_2723_ = _e2780;
                                                                    phi_2725_ = _e2782;
                                                                    phi_2727_ = (_e2220 + 1u);
                                                                } else {
                                                                    phi_2718_ = vec3<f32>();
                                                                    phi_2721_ = vec3<f32>();
                                                                    phi_2723_ = vec3<f32>();
                                                                    phi_2725_ = vec3<f32>();
                                                                    phi_2727_ = u32();
                                                                }
                                                                let _e2789 = phi_2718_;
                                                                let _e2791 = phi_2721_;
                                                                let _e2793 = phi_2723_;
                                                                let _e2795 = phi_2725_;
                                                                let _e2797 = phi_2727_;
                                                                phi_2717_ = _e2789;
                                                                phi_2720_ = _e2791;
                                                                phi_2722_ = _e2793;
                                                                phi_2724_ = _e2795;
                                                                phi_2726_ = _e2797;
                                                                phi_32454_ = _e1443;
                                                                if !(_e2221) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2800 = phi_32454_;
                                                            phi_32544_ = _e2800;
                                                            if _e2800 {
                                                                break;
                                                            }
                                                            let _e2802 = local_12;
                                                            let _e2805 = local_13;
                                                            let _e2808 = local_14;
                                                            phi_32548_ = _e2800;
                                                            phi_3235_ = type_53((_e2802 * vec3<f32>(_e1492, _e1495, _e1498)), (_e2805 * _e1502), _e2808);
                                                        } else {
                                                            phi_32548_ = _e1443;
                                                            phi_3235_ = type_53(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2811 = phi_32548_;
                                                        let _e2813 = phi_3235_;
                                                        phi_32547_ = _e2811;
                                                        phi_3236_ = _e2813;
                                                    } else {
                                                        phi_32547_ = _e1443;
                                                        phi_3236_ = _e2046;
                                                    }
                                                    let _e2815 = phi_32547_;
                                                    let _e2817 = phi_3236_;
                                                    let _e2825 = (((_e334.member_1.x * _e1474) + (_e334.member_1.y * _e1477)) + (_e334.member_1.z * _e1480));
                                                    let _e2827 = (1f + (_e2825 * _e2825));
                                                    let _e2828 = (0.059683103f * _e2827);
                                                    let _e2835 = (_e1505 * _e1505);
                                                    let _e2846 = ((0.119366206f * ((1f - _e2835) * _e2827)) / ((2f + _e2835) * pow(((1f + _e2835) - ((2f * _e1505) * _e2825)), 1.5f)));
                                                    phi_32546_ = _e2815;
                                                    phi_3259_ = vec3<f32>((_e1445.x + (((((_e2817.member.x * _e2828) + (_e2817.member_1.x * _e2846)) + _e2817.member_2.x) * _e1489) * 0.38317f)), (_e1445.y + (((((_e2817.member.y * _e2828) + (_e2817.member_1.y * _e2846)) + _e2817.member_2.y) * _e1489) * 0.38317f)), (_e1445.z + (((((_e2817.member.z * _e2828) + (_e2817.member_1.z * _e2846)) + _e2817.member_2.z) * _e1489) * 0.38317f)));
                                                    phi_3260_ = (_e1447 + 1i);
                                                    phi_3261_ = bitcast<u32>(_e1525);
                                                } else {
                                                    phi_32546_ = _e1443;
                                                    phi_3259_ = vec3<f32>();
                                                    phi_3260_ = i32();
                                                    phi_3261_ = u32();
                                                }
                                                let _e2877 = phi_32546_;
                                                let _e2879 = phi_3259_;
                                                let _e2881 = phi_3260_;
                                                let _e2883 = phi_3261_;
                                                phi_32545_ = _e2877;
                                                phi_905_ = _e2879;
                                                phi_908_ = _e2881;
                                                phi_910_ = _e2883;
                                                phi_3262_ = _e1451;
                                            } else {
                                                phi_32545_ = _e1443;
                                                phi_905_ = vec3<f32>();
                                                phi_908_ = i32();
                                                phi_910_ = u32();
                                                phi_3262_ = false;
                                            }
                                            let _e2885 = phi_32545_;
                                            let _e2887 = phi_905_;
                                            let _e2889 = phi_908_;
                                            let _e2891 = phi_910_;
                                            let _e2893 = phi_3262_;
                                            phi_32486_ = _e2885;
                                            phi_904_ = _e2887;
                                            phi_907_ = _e2889;
                                            phi_909_ = _e2891;
                                            phi_32544_ = _e2885;
                                            if !(_e2893) {
                                                break;
                                            }
                                        }
                                        let _e2896 = phi_32544_;
                                        phi_32872_ = _e2896;
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
                                                phi_3596_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3599_ = 0u;
                                                loop {
                                                    let _e2986 = phi_3596_;
                                                    let _e2988 = phi_3599_;
                                                    local_15 = _e2986;
                                                    let _e2989 = (_e2988 < 6u);
                                                    if _e2989 {
                                                        let _e2990 = (_e2988 / 2u);
                                                        if (_e2990 == 0u) {
                                                            phi_22867_ = _e334.member_1.x;
                                                        } else {
                                                            phi_22867_ = select(_e334.member_1.z, _e334.member_1.y, (_e2990 == 1u));
                                                        }
                                                        let _e2998 = phi_22867_;
                                                        let _e2999 = (_e2998 * select(-1f, 1f, ((_e2988 % 2u) == 0u)));
                                                        if (_e2999 > 0f) {
                                                            let _e3003 = ((_e2990 + 1u) % 3u);
                                                            if (_e3003 == 0u) {
                                                                phi_22882_ = _e334.member_1.x;
                                                            } else {
                                                                phi_22882_ = select(_e334.member_1.z, _e334.member_1.y, (_e3003 == 1u));
                                                            }
                                                            let _e3008 = phi_22882_;
                                                            let _e3009 = (_e3008 / _e2999);
                                                            let _e3011 = ((_e2990 + 2u) % 3u);
                                                            if (_e3011 == 0u) {
                                                                phi_22897_ = _e334.member_1.x;
                                                            } else {
                                                                phi_22897_ = select(_e334.member_1.z, _e334.member_1.y, (_e3011 == 1u));
                                                            }
                                                            let _e3016 = phi_22897_;
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
                                                                        phi_32549_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3034 < 0i) {
                                                                        phi_3670_ = 0i;
                                                                    } else {
                                                                        phi_3670_ = select(_e3034, _e3036, (_e3034 > _e3036));
                                                                    }
                                                                    let _e3042 = phi_3670_;
                                                                    let _e3046 = floor((((_e3017 + 1f) * 0.5f) * _e3025));
                                                                    let _e3053 = select(0i, select(select(i32(_e3046), i32(-2147483648), (_e3046 < -2147483600f)), 2147483647i, (_e3046 > 2147483500f)), (_e3046 == _e3046));
                                                                    if _e3037 {
                                                                    } else {
                                                                        phi_32549_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3053 < 0i) {
                                                                        phi_3700_ = 0i;
                                                                    } else {
                                                                        phi_3700_ = select(_e3053, _e3036, (_e3053 > _e3036));
                                                                    }
                                                                    let _e3058 = phi_3700_;
                                                                    let _e3059 = (_e3042 - 1i);
                                                                    phi_3708_ = _e2986;
                                                                    phi_3711_ = select(0i, _e3059, (0i < _e3059));
                                                                    loop {
                                                                        let _e3063 = phi_3708_;
                                                                        let _e3065 = phi_3711_;
                                                                        let _e3066 = (_e3042 + 1i);
                                                                        local_59 = _e3063;
                                                                        let _e3069 = (_e3065 <= select(_e3066, _e3036, (_e3036 < _e3066)));
                                                                        if _e3069 {
                                                                            let _e3070 = (_e3058 - 1i);
                                                                            phi_3734_ = _e3063;
                                                                            phi_3737_ = select(0i, _e3070, (0i < _e3070));
                                                                            loop {
                                                                                let _e3074 = phi_3734_;
                                                                                let _e3076 = phi_3737_;
                                                                                let _e3077 = (_e3058 + 1i);
                                                                                local_58 = _e3074;
                                                                                let _e3080 = (_e3076 <= select(_e3077, _e3036, (_e3036 < _e3077)));
                                                                                if _e3080 {
                                                                                    let _e3089 = global_6.member[(bitcast<u32>(_e2980) + ((((_e2988 * _e2977) + bitcast<u32>(_e3065)) * _e2977) + bitcast<u32>(_e3076)))];
                                                                                    phi_3735_ = vec3<f32>((_e3074.x + _e3089), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3738_ = (_e3076 + 1i);
                                                                                } else {
                                                                                    phi_3735_ = vec3<f32>();
                                                                                    phi_3738_ = i32();
                                                                                }
                                                                                let _e3098 = phi_3735_;
                                                                                let _e3100 = phi_3738_;
                                                                                phi_3734_ = _e3098;
                                                                                phi_3737_ = _e3100;
                                                                                if !(_e3080) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6935 = local_58;
                                                                            phi_3709_ = _e6935;
                                                                            phi_3712_ = (_e3065 + 1i);
                                                                        } else {
                                                                            phi_3709_ = vec3<f32>();
                                                                            phi_3712_ = i32();
                                                                        }
                                                                        let _e3104 = phi_3709_;
                                                                        let _e3106 = phi_3712_;
                                                                        phi_3708_ = _e3104;
                                                                        phi_3711_ = _e3106;
                                                                        if !(_e3069) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6939 = local_59;
                                                                    phi_3936_ = _e6939;
                                                                } else {
                                                                    phi_3936_ = _e2986;
                                                                }
                                                                let _e3109 = phi_3936_;
                                                                phi_3937_ = _e3109;
                                                            } else {
                                                                phi_3937_ = _e2986;
                                                            }
                                                            let _e3111 = phi_3937_;
                                                            phi_3938_ = _e3111;
                                                        } else {
                                                            phi_3938_ = _e2986;
                                                        }
                                                        let _e3113 = phi_3938_;
                                                        phi_3597_ = _e3113;
                                                        phi_3600_ = (_e2988 + 1u);
                                                    } else {
                                                        phi_3597_ = vec3<f32>();
                                                        phi_3600_ = u32();
                                                    }
                                                    let _e3116 = phi_3597_;
                                                    let _e3118 = phi_3600_;
                                                    phi_3596_ = _e3116;
                                                    phi_3599_ = _e3118;
                                                    phi_32549_ = _e2896;
                                                    if !(_e2989) {
                                                        break;
                                                    }
                                                }
                                                let _e3121 = phi_32549_;
                                                phi_32872_ = _e3121;
                                                if _e3121 {
                                                    break;
                                                }
                                                let _e3123 = local_15;
                                                phi_32597_ = _e3121;
                                                phi_3949_ = (_e3123.x * (1f / ((6.2831855f * _e2973) * _e2973)));
                                            } else {
                                                phi_32597_ = _e2896;
                                                phi_3949_ = 0f;
                                            }
                                            let _e3127 = phi_32597_;
                                            let _e3129 = phi_3949_;
                                            if (_e2907 != 4294967295u) {
                                                let _e3146 = (_e2959 * 1.3333334f);
                                                let _e3153 = global_6.member[_e2907];
                                                phi_3973_ = (_e3129 + ((_e3153 * ((exp((-(abs(_e2959)) / (0.06f + (abs(_e2970) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2970 * -0.71428573f) * (_e2970 * 0.71428573f)))))) + (1.3f * exp((((_e2970 * _e2970) + (_e3146 * _e3146)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3973_ = _e3129;
                                            }
                                            let _e3159 = phi_3973_;
                                            phi_3975_ = 0u;
                                            phi_3978_ = _e3159;
                                            loop {
                                                let _e3161 = phi_3975_;
                                                let _e3163 = phi_3978_;
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
                                                            phi_4082_ = 1.8f;
                                                        } else {
                                                            if (_e3191 == 1u) {
                                                                phi_4081_ = 1f;
                                                            } else {
                                                                phi_4081_ = select(0f, 4.7f, (_e3191 == 2u));
                                                            }
                                                            let _e3197 = phi_4081_;
                                                            phi_4082_ = _e3197;
                                                        }
                                                        let _e3199 = phi_4082_;
                                                        let _e3202 = global_6.member[(_e3166 + 6u)];
                                                        phi_4098_ = (_e3163 + (_e3202 * _e3199));
                                                    } else {
                                                        phi_4098_ = _e3163;
                                                    }
                                                    let _e3206 = phi_4098_;
                                                    phi_3976_ = (_e3161 + 1u);
                                                    phi_3979_ = _e3206;
                                                } else {
                                                    phi_3976_ = u32();
                                                    phi_3979_ = f32();
                                                }
                                                let _e3209 = phi_3976_;
                                                let _e3211 = phi_3979_;
                                                phi_3975_ = _e3209;
                                                phi_3978_ = _e3211;
                                                if !(_e3164) {
                                                    break;
                                                }
                                            }
                                            phi_4101_ = 0u;
                                            let _e6959 = local_60;
                                            phi_4104_ = _e6959;
                                            loop {
                                                let _e3214 = phi_4101_;
                                                let _e3216 = phi_4104_;
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
                                                        phi_4212_ = (_e3216 + (_e3243 * select(1f, 3.3808f, (_e3239 < 0.5f))));
                                                    } else {
                                                        phi_4212_ = _e3216;
                                                    }
                                                    let _e3248 = phi_4212_;
                                                    phi_4102_ = (_e3214 + 1u);
                                                    phi_4105_ = _e3248;
                                                } else {
                                                    phi_4102_ = u32();
                                                    phi_4105_ = f32();
                                                }
                                                let _e3251 = phi_4102_;
                                                let _e3253 = phi_4105_;
                                                phi_4101_ = _e3251;
                                                phi_4104_ = _e3253;
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
                                                phi_32638_ = _e3127;
                                                phi_4771_ = vec3<f32>();
                                                phi_4772_ = bool();
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
                                                        phi_23041_ = type_58(0u, type_58().member_1);
                                                        phi_23042_ = type_58();
                                                        phi_23043_ = true;
                                                    } else {
                                                        if (_e3342 < 0f) {
                                                            phi_23013_ = -(sqrt(_e3344));
                                                        } else {
                                                            phi_23013_ = sqrt(_e3344);
                                                        }
                                                        let _e3351 = phi_23013_;
                                                        let _e3353 = -((_e3342 + _e3351));
                                                        let _e3354 = (_e3353 == 0f);
                                                        if _e3354 {
                                                            phi_23036_ = type_58(1u, type_57(0f, 0f));
                                                            phi_23037_ = type_58();
                                                        } else {
                                                            let _e3355 = (_e3333 / _e3353);
                                                            if (_e3353 < _e3355) {
                                                                phi_23025_ = type_57(_e3353, _e3355);
                                                            } else {
                                                                phi_23025_ = type_57(_e3355, _e3353);
                                                            }
                                                            let _e3360 = phi_23025_;
                                                            phi_23036_ = type_58();
                                                            phi_23037_ = type_58(1u, _e3360);
                                                        }
                                                        let _e3363 = phi_23036_;
                                                        let _e3365 = phi_23037_;
                                                        phi_23041_ = _e3363;
                                                        phi_23042_ = _e3365;
                                                        phi_23043_ = _e3354;
                                                    }
                                                    let _e3370 = phi_23041_;
                                                    let _e3372 = phi_23042_;
                                                    let _e3374 = phi_23043_;
                                                    if _e3374 {
                                                        phi_23047_ = _e3370;
                                                    } else {
                                                        phi_23047_ = _e3372;
                                                    }
                                                    let _e3376 = phi_23047_;
                                                    if (bitcast<i32>(_e3376.member) != 0i) {
                                                        phi_22981_ = select(_e3376.member_1.member_1, _e3376.member_1.member, (_e3376.member_1.member >= 0f));
                                                    } else {
                                                        phi_22981_ = -1f;
                                                    }
                                                    let _e3387 = phi_22981_;
                                                    phi_4613_ = select(true, false, (_e3387 >= 0f));
                                                } else {
                                                    phi_4613_ = true;
                                                }
                                                let _e3391 = phi_4613_;
                                                if _e3391 {
                                                    let _e3399 = ((((_e334.member.x - _e3278) * _e334.member_1.x) + ((_e334.member.y - _e3281) * _e334.member_1.y)) + ((_e334.member.z - _e3284) * _e334.member_1.z));
                                                    let _e3400 = (_e3332 + _e3293);
                                                    let _e3401 = (_e3399 * _e3399);
                                                    let _e3402 = (_e3401 - _e3400);
                                                    if (_e3402 < 0f) {
                                                        phi_23201_ = type_58(0u, type_58().member_1);
                                                        phi_23202_ = type_58();
                                                        phi_23203_ = true;
                                                    } else {
                                                        if (_e3399 < 0f) {
                                                            phi_23173_ = -(sqrt(_e3402));
                                                        } else {
                                                            phi_23173_ = sqrt(_e3402);
                                                        }
                                                        let _e3409 = phi_23173_;
                                                        let _e3411 = -((_e3399 + _e3409));
                                                        let _e3412 = (_e3411 == 0f);
                                                        if _e3412 {
                                                            phi_23196_ = type_58(1u, type_57(0f, 0f));
                                                            phi_23197_ = type_58();
                                                        } else {
                                                            let _e3413 = (_e3400 / _e3411);
                                                            if (_e3411 < _e3413) {
                                                                phi_23185_ = type_57(_e3411, _e3413);
                                                            } else {
                                                                phi_23185_ = type_57(_e3413, _e3411);
                                                            }
                                                            let _e3418 = phi_23185_;
                                                            phi_23196_ = type_58();
                                                            phi_23197_ = type_58(1u, _e3418);
                                                        }
                                                        let _e3421 = phi_23196_;
                                                        let _e3423 = phi_23197_;
                                                        phi_23201_ = _e3421;
                                                        phi_23202_ = _e3423;
                                                        phi_23203_ = _e3412;
                                                    }
                                                    let _e3428 = phi_23201_;
                                                    let _e3430 = phi_23202_;
                                                    let _e3432 = phi_23203_;
                                                    if _e3432 {
                                                        phi_23207_ = _e3428;
                                                    } else {
                                                        phi_23207_ = _e3430;
                                                    }
                                                    let _e3434 = phi_23207_;
                                                    if (bitcast<i32>(_e3434.member) != 0i) {
                                                        phi_23094_ = type_58(0u, _e3434.member_1);
                                                    } else {
                                                        phi_23094_ = type_58(1u, type_58().member_1);
                                                    }
                                                    let _e3444 = phi_23094_;
                                                    if (bitcast<i32>(_e3444.member) != 0i) {
                                                        phi_23141_ = type_58(0u, type_58().member_1);
                                                    } else {
                                                        if (_e3444.member_1.member_1 <= 0f) {
                                                            phi_23137_ = type_58(0u, type_58().member_1);
                                                        } else {
                                                            if (_e3444.member_1.member != _e3444.member_1.member) {
                                                                phi_23245_ = true;
                                                            } else {
                                                                phi_23245_ = (0f >= _e3444.member_1.member);
                                                            }
                                                            let _e3456 = phi_23245_;
                                                            let _e3457 = select(_e3444.member_1.member, 0f, _e3456);
                                                            if _e3334 {
                                                                let _e3458 = (_e3401 - _e3333);
                                                                if (_e3458 < 0f) {
                                                                    phi_23353_ = type_58(0u, type_58().member_1);
                                                                    phi_23354_ = type_58();
                                                                    phi_23355_ = true;
                                                                } else {
                                                                    if (_e3399 < 0f) {
                                                                        phi_23325_ = -(sqrt(_e3458));
                                                                    } else {
                                                                        phi_23325_ = sqrt(_e3458);
                                                                    }
                                                                    let _e3465 = phi_23325_;
                                                                    let _e3467 = -((_e3399 + _e3465));
                                                                    let _e3468 = (_e3467 == 0f);
                                                                    if _e3468 {
                                                                        phi_23348_ = type_58(1u, type_57(0f, 0f));
                                                                        phi_23349_ = type_58();
                                                                    } else {
                                                                        let _e3469 = (_e3333 / _e3467);
                                                                        if (_e3467 < _e3469) {
                                                                            phi_23337_ = type_57(_e3467, _e3469);
                                                                        } else {
                                                                            phi_23337_ = type_57(_e3469, _e3467);
                                                                        }
                                                                        let _e3474 = phi_23337_;
                                                                        phi_23348_ = type_58();
                                                                        phi_23349_ = type_58(1u, _e3474);
                                                                    }
                                                                    let _e3477 = phi_23348_;
                                                                    let _e3479 = phi_23349_;
                                                                    phi_23353_ = _e3477;
                                                                    phi_23354_ = _e3479;
                                                                    phi_23355_ = _e3468;
                                                                }
                                                                let _e3484 = phi_23353_;
                                                                let _e3486 = phi_23354_;
                                                                let _e3488 = phi_23355_;
                                                                if _e3488 {
                                                                    phi_23359_ = _e3484;
                                                                } else {
                                                                    phi_23359_ = _e3486;
                                                                }
                                                                let _e3490 = phi_23359_;
                                                                if (bitcast<i32>(_e3490.member) != 0i) {
                                                                    phi_23293_ = select(_e3490.member_1.member_1, _e3490.member_1.member, (_e3490.member_1.member >= 0f));
                                                                } else {
                                                                    phi_23293_ = -1f;
                                                                }
                                                                let _e3501 = phi_23293_;
                                                                if (_e3501 >= 0f) {
                                                                    if (_e3444.member_1.member_1 != _e3444.member_1.member_1) {
                                                                        phi_23397_ = true;
                                                                    } else {
                                                                        phi_23397_ = (_e3501 <= _e3444.member_1.member_1);
                                                                    }
                                                                    let _e3506 = phi_23397_;
                                                                    phi_23122_ = select(_e3444.member_1.member_1, _e3501, _e3506);
                                                                } else {
                                                                    phi_23122_ = _e3444.member_1.member_1;
                                                                }
                                                                let _e3509 = phi_23122_;
                                                                phi_23124_ = _e3509;
                                                            } else {
                                                                phi_23124_ = _e3444.member_1.member_1;
                                                            }
                                                            let _e3511 = phi_23124_;
                                                            if (_e3511 <= _e3457) {
                                                                phi_23133_ = type_58(0u, type_58().member_1);
                                                            } else {
                                                                phi_23133_ = type_58(1u, type_57(_e3457, _e3511));
                                                            }
                                                            let _e3519 = phi_23133_;
                                                            phi_23137_ = _e3519;
                                                        }
                                                        let _e3524 = phi_23137_;
                                                        phi_23141_ = _e3524;
                                                    }
                                                    let _e3529 = phi_23141_;
                                                    let _e3532 = (bitcast<i32>(_e3529.member) != 0i);
                                                    if _e3532 {
                                                        phi_4627_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4630_ = 0u;
                                                        loop {
                                                            let _e3538 = phi_4627_;
                                                            let _e3540 = phi_4630_;
                                                            local_20 = _e3538;
                                                            local_21 = _e3538;
                                                            local_22 = _e3538;
                                                            let _e3541 = (_e3540 < 32u);
                                                            if _e3541 {
                                                                let _e3542 = -(_e3399);
                                                                if (_e3529.member_1.member <= _e3529.member_1.member_1) {
                                                                } else {
                                                                    phi_32576_ = true;
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
                                                                    phi_4731_ = type_57((_e3529.member_1.member + ((_e3550 * _e3631) * _e3631)), (_e3550 * ((_e3629 * _e3629) - (_e3626 * _e3626))));
                                                                } else {
                                                                    if (_e3549 <= _e3551) {
                                                                        let _e3609 = (f32((31u - _e3540)) * 0.03125f);
                                                                        let _e3612 = (f32((32u - _e3540)) * 0.03125f);
                                                                        let _e3614 = ((_e3609 + _e3612) * 0.5f);
                                                                        phi_4730_ = type_57((_e3529.member_1.member_1 - ((_e3550 * _e3614) * _e3614)), (_e3550 * ((_e3612 * _e3612) - (_e3609 * _e3609))));
                                                                    } else {
                                                                        let _e3557 = round(((32f * _e3548) / (_e3548 + _e3549)));
                                                                        let _e3562 = select(select(u32(_e3557), 0u, (_e3557 < 0f)), 4294967295u, (_e3557 > 4294967000f));
                                                                        if (_e3562 < 1u) {
                                                                            phi_4714_ = 1u;
                                                                        } else {
                                                                            phi_4714_ = select(_e3562, 31u, (_e3562 > 31u));
                                                                        }
                                                                        let _e3567 = phi_4714_;
                                                                        if (_e3540 < _e3567) {
                                                                            let _e3588 = ((_e3567 - 1u) - _e3540);
                                                                            let _e3590 = f32(_e3567);
                                                                            let _e3591 = (f32(_e3588) / _e3590);
                                                                            let _e3594 = (f32((_e3588 + 1u)) / _e3590);
                                                                            let _e3596 = ((_e3591 + _e3594) * 0.5f);
                                                                            phi_4729_ = type_57((_e3547 - ((_e3548 * _e3596) * _e3596)), (_e3548 * ((_e3594 * _e3594) - (_e3591 * _e3591))));
                                                                        } else {
                                                                            let _e3570 = (_e3540 - _e3567);
                                                                            let _e3572 = f32((32u - _e3567));
                                                                            let _e3573 = (f32(_e3570) / _e3572);
                                                                            let _e3576 = (f32((_e3570 + 1u)) / _e3572);
                                                                            let _e3578 = ((_e3573 + _e3576) * 0.5f);
                                                                            phi_4729_ = type_57((_e3547 + ((_e3549 * _e3578) * _e3578)), (_e3549 * ((_e3576 * _e3576) - (_e3573 * _e3573))));
                                                                        }
                                                                        let _e3606 = phi_4729_;
                                                                        phi_4730_ = _e3606;
                                                                    }
                                                                    let _e3624 = phi_4730_;
                                                                    phi_4731_ = _e3624;
                                                                }
                                                                let _e3641 = phi_4731_;
                                                                let _e3647 = (_e334.member.x + (_e334.member_1.x * _e3641.member));
                                                                let _e3648 = (_e334.member.y + (_e334.member_1.y * _e3641.member));
                                                                let _e3649 = (_e334.member.z + (_e334.member_1.z * _e3641.member));
                                                                let _e3663 = (_e3647 - _e3278);
                                                                let _e3664 = (_e3648 - _e3281);
                                                                let _e3665 = (_e3649 - _e3284);
                                                                let _e3673 = ((((((_e3647 * _e3647) + (_e3648 * _e3648)) + (_e3649 * _e3649)) - (2f * (((_e3647 * _e3278) + (_e3648 * _e3281)) + (_e3649 * _e3284)))) + _e3290) / (sqrt((((_e3663 * _e3663) + (_e3664 * _e3664)) + (_e3665 * _e3665))) + _e3287));
                                                                let _e3677 = (1f - (abs((_e3673 - 25000f)) * 0.00006666667f));
                                                                if (_e3677 != _e3677) {
                                                                    phi_23596_ = true;
                                                                } else {
                                                                    phi_23596_ = (0f >= _e3677);
                                                                }
                                                                let _e3681 = phi_23596_;
                                                                let _e3683 = -(_e3673);
                                                                phi_4628_ = (_e3538 + vec3<f32>((exp((_e3683 / _e3296)) * _e3641.member_1), (exp((_e3683 / _e3299)) * _e3641.member_1), (select(_e3677, 0f, _e3681) * _e3641.member_1)));
                                                                phi_4631_ = (_e3540 + 1u);
                                                            } else {
                                                                phi_4628_ = vec3<f32>();
                                                                phi_4631_ = u32();
                                                            }
                                                            let _e3695 = phi_4628_;
                                                            let _e3697 = phi_4631_;
                                                            phi_4627_ = _e3695;
                                                            phi_4630_ = _e3697;
                                                            phi_32576_ = _e3127;
                                                            if !(_e3541) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3700 = phi_32576_;
                                                        phi_32872_ = _e3700;
                                                        if _e3700 {
                                                            break;
                                                        }
                                                        let _e3702 = local_20;
                                                        let _e3709 = local_21;
                                                        let _e3711 = ((_e3311 * 1.1f) * _e3709.y);
                                                        let _e3716 = local_22;
                                                        phi_32643_ = _e3700;
                                                        phi_4756_ = vec3<f32>(exp(-((((_e3302 * _e3702.x) + _e3711) + (_e3314 * _e3716.z)))), exp(-((((_e3305 * _e3702.x) + _e3711) + (_e3317 * _e3716.z)))), exp(-((((_e3308 * _e3702.x) + _e3711) + (_e3320 * _e3716.z)))));
                                                    } else {
                                                        phi_32643_ = _e3127;
                                                        phi_4756_ = vec3<f32>();
                                                    }
                                                    let _e3732 = phi_32643_;
                                                    let _e3734 = phi_4756_;
                                                    phi_32642_ = _e3732;
                                                    phi_4758_ = _e3734;
                                                    phi_4760_ = select(true, false, _e3532);
                                                } else {
                                                    phi_32642_ = _e3127;
                                                    phi_4758_ = vec3<f32>();
                                                    phi_4760_ = true;
                                                }
                                                let _e3737 = phi_32642_;
                                                let _e3739 = phi_4758_;
                                                let _e3741 = phi_4760_;
                                                let _e3745 = select(_e3739, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3391)), vec3(_e3741));
                                                let _e3749 = select(_e3745.y, _e3745.x, (_e3745.x > _e3745.y));
                                                phi_32638_ = _e3737;
                                                phi_4771_ = _e3745;
                                                phi_4772_ = (select(_e3745.z, _e3749, (_e3749 > _e3745.z)) > 0f);
                                            }
                                            let _e3755 = phi_32638_;
                                            let _e3757 = phi_4771_;
                                            let _e3759 = phi_4772_;
                                            phi_32637_ = _e3755;
                                            phi_4774_ = _e3757;
                                            phi_4775_ = _e3759;
                                            phi_4776_ = select(true, false, _e3276);
                                        } else {
                                            phi_32637_ = _e2896;
                                            phi_4774_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4775_ = _e1440;
                                            phi_4776_ = true;
                                        }
                                        let _e3762 = phi_32637_;
                                        let _e3764 = phi_4774_;
                                        let _e3766 = phi_4775_;
                                        let _e3768 = phi_4776_;
                                        phi_32636_ = _e3762;
                                        let _e7030 = local_61;
                                        phi_4780_ = _e7030;
                                        phi_4781_ = _e3764;
                                        phi_4782_ = _e3766;
                                        let _e7035 = local_62;
                                        phi_4783_ = _e7035;
                                        phi_4784_ = _e3768;
                                    } else {
                                        phi_32636_ = _e323;
                                        phi_4780_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4781_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4782_ = _e1440;
                                        phi_4783_ = vec3<f32>();
                                        phi_4784_ = true;
                                    }
                                    let _e3770 = phi_32636_;
                                    let _e3772 = phi_4780_;
                                    let _e3774 = phi_4781_;
                                    let _e3776 = phi_4782_;
                                    let _e3778 = phi_4783_;
                                    let _e3780 = phi_4784_;
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
                                                phi_5104_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5107_ = 0u;
                                                loop {
                                                    let _e3873 = phi_5104_;
                                                    let _e3875 = phi_5107_;
                                                    local_66 = _e3873;
                                                    let _e3876 = (_e3875 < 6u);
                                                    if _e3876 {
                                                        let _e3877 = (_e3875 / 2u);
                                                        if (_e3877 == 0u) {
                                                            phi_23667_ = _e334.member_1.x;
                                                        } else {
                                                            phi_23667_ = select(_e334.member_1.z, _e334.member_1.y, (_e3877 == 1u));
                                                        }
                                                        let _e3885 = phi_23667_;
                                                        let _e3886 = (_e3885 * select(-1f, 1f, ((_e3875 % 2u) == 0u)));
                                                        if (_e3886 > 0f) {
                                                            let _e3890 = ((_e3877 + 1u) % 3u);
                                                            if (_e3890 == 0u) {
                                                                phi_23682_ = _e334.member_1.x;
                                                            } else {
                                                                phi_23682_ = select(_e334.member_1.z, _e334.member_1.y, (_e3890 == 1u));
                                                            }
                                                            let _e3895 = phi_23682_;
                                                            let _e3896 = (_e3895 / _e3886);
                                                            let _e3898 = ((_e3877 + 2u) % 3u);
                                                            if (_e3898 == 0u) {
                                                                phi_23697_ = _e334.member_1.x;
                                                            } else {
                                                                phi_23697_ = select(_e334.member_1.z, _e334.member_1.y, (_e3898 == 1u));
                                                            }
                                                            let _e3903 = phi_23697_;
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
                                                                        phi_32631_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3921 < 0i) {
                                                                        phi_5178_ = 0i;
                                                                    } else {
                                                                        phi_5178_ = select(_e3921, _e3923, (_e3921 > _e3923));
                                                                    }
                                                                    let _e3929 = phi_5178_;
                                                                    let _e3933 = floor((((_e3904 + 1f) * 0.5f) * _e3912));
                                                                    let _e3940 = select(0i, select(select(i32(_e3933), i32(-2147483648), (_e3933 < -2147483600f)), 2147483647i, (_e3933 > 2147483500f)), (_e3933 == _e3933));
                                                                    if _e3924 {
                                                                    } else {
                                                                        phi_32631_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3940 < 0i) {
                                                                        phi_5208_ = 0i;
                                                                    } else {
                                                                        phi_5208_ = select(_e3940, _e3923, (_e3940 > _e3923));
                                                                    }
                                                                    let _e3945 = phi_5208_;
                                                                    let _e3946 = (_e3929 - 1i);
                                                                    phi_5216_ = _e3873;
                                                                    phi_5219_ = select(0i, _e3946, (0i < _e3946));
                                                                    loop {
                                                                        let _e3950 = phi_5216_;
                                                                        let _e3952 = phi_5219_;
                                                                        let _e3953 = (_e3929 + 1i);
                                                                        local_65 = _e3950;
                                                                        let _e3956 = (_e3952 <= select(_e3953, _e3923, (_e3923 < _e3953)));
                                                                        if _e3956 {
                                                                            let _e3957 = (_e3945 - 1i);
                                                                            phi_5242_ = _e3950;
                                                                            phi_5245_ = select(0i, _e3957, (0i < _e3957));
                                                                            loop {
                                                                                let _e3961 = phi_5242_;
                                                                                let _e3963 = phi_5245_;
                                                                                let _e3964 = (_e3945 + 1i);
                                                                                local_64 = _e3961;
                                                                                let _e3967 = (_e3963 <= select(_e3964, _e3923, (_e3923 < _e3964)));
                                                                                if _e3967 {
                                                                                    let _e3974 = (bitcast<u32>(_e3862) + ((((_e3875 * _e3859) + bitcast<u32>(_e3952)) * _e3859) + bitcast<u32>(_e3963)));
                                                                                    let _e3976 = global_6.member[_e3974];
                                                                                    let _e3980 = global_6.member[(_e3974 + 1u)];
                                                                                    phi_5316_ = _e3961;
                                                                                    phi_5319_ = bitcast<u32>(_e3976);
                                                                                    loop {
                                                                                        let _e3983 = phi_5316_;
                                                                                        let _e3985 = phi_5319_;
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
                                                                                                phi_5439_ = vec3<f32>((_e3983.x + (_e3999 * _e4021)), (_e3983.y + (_e4002 * _e4021)), (_e3983.z + (_e4005 * _e4021)));
                                                                                            } else {
                                                                                                phi_5439_ = _e3983;
                                                                                            }
                                                                                            let _e4033 = phi_5439_;
                                                                                            phi_5317_ = _e4033;
                                                                                            phi_5320_ = (_e3985 + 1u);
                                                                                        } else {
                                                                                            phi_5317_ = vec3<f32>();
                                                                                            phi_5320_ = u32();
                                                                                        }
                                                                                        let _e4036 = phi_5317_;
                                                                                        let _e4038 = phi_5320_;
                                                                                        phi_5316_ = _e4036;
                                                                                        phi_5319_ = _e4038;
                                                                                        if !(_e3986) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7055 = local_63;
                                                                                    phi_5243_ = _e7055;
                                                                                    phi_5246_ = (_e3963 + 1i);
                                                                                } else {
                                                                                    phi_5243_ = vec3<f32>();
                                                                                    phi_5246_ = i32();
                                                                                }
                                                                                let _e4042 = phi_5243_;
                                                                                let _e4044 = phi_5246_;
                                                                                phi_5242_ = _e4042;
                                                                                phi_5245_ = _e4044;
                                                                                if !(_e3967) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7059 = local_64;
                                                                            phi_5217_ = _e7059;
                                                                            phi_5220_ = (_e3952 + 1i);
                                                                        } else {
                                                                            phi_5217_ = vec3<f32>();
                                                                            phi_5220_ = i32();
                                                                        }
                                                                        let _e4048 = phi_5217_;
                                                                        let _e4050 = phi_5220_;
                                                                        phi_5216_ = _e4048;
                                                                        phi_5219_ = _e4050;
                                                                        if !(_e3956) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7063 = local_65;
                                                                    phi_5444_ = _e7063;
                                                                } else {
                                                                    phi_5444_ = _e3873;
                                                                }
                                                                let _e4053 = phi_5444_;
                                                                phi_5445_ = _e4053;
                                                            } else {
                                                                phi_5445_ = _e3873;
                                                            }
                                                            let _e4055 = phi_5445_;
                                                            phi_5446_ = _e4055;
                                                        } else {
                                                            phi_5446_ = _e3873;
                                                        }
                                                        let _e4057 = phi_5446_;
                                                        phi_5105_ = _e4057;
                                                        phi_5108_ = (_e3875 + 1u);
                                                    } else {
                                                        phi_5105_ = vec3<f32>();
                                                        phi_5108_ = u32();
                                                    }
                                                    let _e4060 = phi_5105_;
                                                    let _e4062 = phi_5108_;
                                                    phi_5104_ = _e4060;
                                                    phi_5107_ = _e4062;
                                                    phi_32631_ = _e3770;
                                                    if !(_e3876) {
                                                        break;
                                                    }
                                                }
                                                let _e4065 = phi_32631_;
                                                phi_32872_ = _e4065;
                                                if _e4065 {
                                                    break;
                                                }
                                                phi_32673_ = _e4065;
                                                let _e7072 = local_66;
                                                phi_5466_ = _e7072;
                                            } else {
                                                phi_32673_ = _e3770;
                                                phi_5466_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4067 = phi_32673_;
                                            let _e4069 = phi_5466_;
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
                                                    phi_5679_ = _e4119;
                                                    phi_5680_ = _e4118;
                                                    phi_5683_ = _e4119;
                                                    phi_5684_ = _e4118;
                                                    phi_5685_ = select(true, false, _e4118);
                                                } else {
                                                    phi_5679_ = u32();
                                                    phi_5680_ = bool();
                                                    phi_5683_ = select(2u, 0u, _e4114);
                                                    phi_5684_ = false;
                                                    phi_5685_ = true;
                                                }
                                                let _e4122 = phi_5679_;
                                                let _e4124 = phi_5680_;
                                                let _e4126 = phi_5683_;
                                                let _e4128 = phi_5684_;
                                                let _e4130 = phi_5685_;
                                                let _e4132 = select(select(f32(), 0f, _e4115), 0f, _e4130);
                                                phi_5695_ = _e4132;
                                                phi_5698_ = _e4132;
                                                phi_5700_ = select(_e4122, _e4126, _e4130);
                                                phi_5702_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5704_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5706_ = 0f;
                                                phi_5708_ = 1f;
                                                loop {
                                                    let _e4137 = phi_5695_;
                                                    let _e4139 = phi_5698_;
                                                    let _e4141 = phi_5700_;
                                                    let _e4143 = phi_5702_;
                                                    let _e4145 = phi_5704_;
                                                    let _e4147 = phi_5706_;
                                                    let _e4149 = phi_5708_;
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
                                                            phi_5786_ = 4u;
                                                            phi_5788_ = vec3<f32>((_e334.member_1.x * _e4207), (_e334.member_1.y * _e4207), (_e334.member_1.z * _e4207));
                                                        } else {
                                                            if (_e4141 == 2u) {
                                                                phi_5784_ = 3u;
                                                                phi_5785_ = vec3<f32>(((_e334.member_1.x * 1.5f) + 5f), ((_e334.member_1.y * 1.5f) + 5f), ((_e334.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4153 = (_e4141 >= 7u);
                                                                if _e4153 {
                                                                    phi_5746_ = (_e4141 - 7u);
                                                                } else {
                                                                    phi_5746_ = (_e4141 - 3u);
                                                                }
                                                                let _e4157 = phi_5746_;
                                                                if _e4153 {
                                                                    phi_5761_ = vec3<f32>(((_e334.member_1.x * 11f) + -3f), ((_e334.member_1.y * 11f) + 1f), ((_e334.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5761_ = vec3<f32>((_e334.member_1.x * 7f), (_e334.member_1.y * 7f), (_e334.member_1.z * 7f));
                                                                }
                                                                let _e4170 = phi_5761_;
                                                                if (_e4157 < 3u) {
                                                                    if (_e4157 == 0u) {
                                                                        phi_23712_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_23712_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4157 == 1u)));
                                                                    }
                                                                    let _e4189 = phi_23712_;
                                                                    phi_5782_ = 4u;
                                                                    phi_5783_ = (_e4170 + _e4189);
                                                                } else {
                                                                    if _e4153 {
                                                                        phi_5781_ = (_e4170 + _e4143);
                                                                    } else {
                                                                        phi_5781_ = (_e4170 + vec3<f32>((_e4145.x * 1.1f), (_e4145.y * 1.1f), (_e4145.z * 1.1f)));
                                                                    }
                                                                    let _e4182 = phi_5781_;
                                                                    phi_5782_ = select(7u, 5u, _e4153);
                                                                    phi_5783_ = _e4182;
                                                                }
                                                                let _e4192 = phi_5782_;
                                                                let _e4194 = phi_5783_;
                                                                phi_5784_ = _e4192;
                                                                phi_5785_ = _e4194;
                                                            }
                                                            let _e4203 = phi_5784_;
                                                            let _e4205 = phi_5785_;
                                                            phi_5786_ = _e4203;
                                                            phi_5788_ = _e4205;
                                                        }
                                                        let _e4213 = phi_5786_;
                                                        let _e4215 = phi_5788_;
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
                                                        phi_5923_ = 0u;
                                                        phi_5926_ = 0f;
                                                        phi_5928_ = 1f;
                                                        phi_5930_ = 0f;
                                                        phi_5932_ = _e4215;
                                                        loop {
                                                            let _e4249 = phi_5923_;
                                                            let _e4251 = phi_5926_;
                                                            let _e4253 = phi_5928_;
                                                            let _e4255 = phi_5930_;
                                                            let _e4257 = phi_5932_;
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
                                                                    phi_23773_ = ((_e4390 * _e4298) + (_e4393 * _e4299));
                                                                } else {
                                                                    if (_e4387 < 8u) {
                                                                        phi_23767_ = ((_e4390 * _e4298) + (_e4393 * _e4300));
                                                                    } else {
                                                                        phi_23767_ = ((_e4390 * _e4299) + (_e4393 * _e4300));
                                                                    }
                                                                    let _e4403 = phi_23767_;
                                                                    phi_23773_ = _e4403;
                                                                }
                                                                let _e4408 = phi_23773_;
                                                                let _e4411 = global_6.member[(_e4220 + _e4368)];
                                                                let _e4417 = (_e4298 - 1f);
                                                                let _e4418 = (select(select(u32(_e4411), 0u, (_e4411 < 0f)), 4294967295u, (_e4411 > 4294967000f)) % 12u);
                                                                let _e4421 = select(-1f, 1f, ((_e4418 & 1u) == 0u));
                                                                let _e4424 = select(-1f, 1f, ((_e4418 & 2u) == 0u));
                                                                if (_e4418 < 4u) {
                                                                    phi_23807_ = ((_e4421 * _e4417) + (_e4424 * _e4299));
                                                                } else {
                                                                    if (_e4418 < 8u) {
                                                                        phi_23801_ = ((_e4421 * _e4417) + (_e4424 * _e4300));
                                                                    } else {
                                                                        phi_23801_ = ((_e4421 * _e4299) + (_e4424 * _e4300));
                                                                    }
                                                                    let _e4434 = phi_23801_;
                                                                    phi_23807_ = _e4434;
                                                                }
                                                                let _e4439 = phi_23807_;
                                                                let _e4442 = global_6.member[(_e4220 + _e4349)];
                                                                let _e4448 = (_e4299 - 1f);
                                                                let _e4449 = (select(select(u32(_e4442), 0u, (_e4442 < 0f)), 4294967295u, (_e4442 > 4294967000f)) % 12u);
                                                                let _e4452 = select(-1f, 1f, ((_e4449 & 1u) == 0u));
                                                                let _e4455 = select(-1f, 1f, ((_e4449 & 2u) == 0u));
                                                                if (_e4449 < 4u) {
                                                                    phi_23841_ = ((_e4452 * _e4298) + (_e4455 * _e4448));
                                                                } else {
                                                                    if (_e4449 < 8u) {
                                                                        phi_23835_ = ((_e4452 * _e4298) + (_e4455 * _e4300));
                                                                    } else {
                                                                        phi_23835_ = ((_e4452 * _e4448) + (_e4455 * _e4300));
                                                                    }
                                                                    let _e4465 = phi_23835_;
                                                                    phi_23841_ = _e4465;
                                                                }
                                                                let _e4470 = phi_23841_;
                                                                let _e4473 = global_6.member[(_e4220 + _e4378)];
                                                                let _e4479 = (select(select(u32(_e4473), 0u, (_e4473 < 0f)), 4294967295u, (_e4473 > 4294967000f)) % 12u);
                                                                let _e4482 = select(-1f, 1f, ((_e4479 & 1u) == 0u));
                                                                let _e4485 = select(-1f, 1f, ((_e4479 & 2u) == 0u));
                                                                if (_e4479 < 4u) {
                                                                    phi_23875_ = ((_e4482 * _e4417) + (_e4485 * _e4448));
                                                                } else {
                                                                    if (_e4479 < 8u) {
                                                                        phi_23869_ = ((_e4482 * _e4417) + (_e4485 * _e4300));
                                                                    } else {
                                                                        phi_23869_ = ((_e4482 * _e4448) + (_e4485 * _e4300));
                                                                    }
                                                                    let _e4495 = phi_23869_;
                                                                    phi_23875_ = _e4495;
                                                                }
                                                                let _e4500 = phi_23875_;
                                                                let _e4504 = global_6.member[(_e4220 + (_e4339 + 1u))];
                                                                let _e4510 = (_e4300 - 1f);
                                                                let _e4511 = (select(select(u32(_e4504), 0u, (_e4504 < 0f)), 4294967295u, (_e4504 > 4294967000f)) % 12u);
                                                                let _e4514 = select(-1f, 1f, ((_e4511 & 1u) == 0u));
                                                                let _e4517 = select(-1f, 1f, ((_e4511 & 2u) == 0u));
                                                                if (_e4511 < 4u) {
                                                                    phi_23909_ = ((_e4514 * _e4298) + (_e4517 * _e4299));
                                                                } else {
                                                                    if (_e4511 < 8u) {
                                                                        phi_23903_ = ((_e4514 * _e4298) + (_e4517 * _e4510));
                                                                    } else {
                                                                        phi_23903_ = ((_e4514 * _e4299) + (_e4517 * _e4510));
                                                                    }
                                                                    let _e4527 = phi_23903_;
                                                                    phi_23909_ = _e4527;
                                                                }
                                                                let _e4532 = phi_23909_;
                                                                let _e4536 = global_6.member[(_e4220 + (_e4368 + 1u))];
                                                                let _e4542 = (select(select(u32(_e4536), 0u, (_e4536 < 0f)), 4294967295u, (_e4536 > 4294967000f)) % 12u);
                                                                let _e4545 = select(-1f, 1f, ((_e4542 & 1u) == 0u));
                                                                let _e4548 = select(-1f, 1f, ((_e4542 & 2u) == 0u));
                                                                if (_e4542 < 4u) {
                                                                    phi_23943_ = ((_e4545 * _e4417) + (_e4548 * _e4299));
                                                                } else {
                                                                    if (_e4542 < 8u) {
                                                                        phi_23937_ = ((_e4545 * _e4417) + (_e4548 * _e4510));
                                                                    } else {
                                                                        phi_23937_ = ((_e4545 * _e4299) + (_e4548 * _e4510));
                                                                    }
                                                                    let _e4558 = phi_23937_;
                                                                    phi_23943_ = _e4558;
                                                                }
                                                                let _e4563 = phi_23943_;
                                                                let _e4567 = global_6.member[(_e4220 + (_e4349 + 1u))];
                                                                let _e4573 = (select(select(u32(_e4567), 0u, (_e4567 < 0f)), 4294967295u, (_e4567 > 4294967000f)) % 12u);
                                                                let _e4576 = select(-1f, 1f, ((_e4573 & 1u) == 0u));
                                                                let _e4579 = select(-1f, 1f, ((_e4573 & 2u) == 0u));
                                                                if (_e4573 < 4u) {
                                                                    phi_23977_ = ((_e4576 * _e4298) + (_e4579 * _e4448));
                                                                } else {
                                                                    if (_e4573 < 8u) {
                                                                        phi_23971_ = ((_e4576 * _e4298) + (_e4579 * _e4510));
                                                                    } else {
                                                                        phi_23971_ = ((_e4576 * _e4448) + (_e4579 * _e4510));
                                                                    }
                                                                    let _e4589 = phi_23971_;
                                                                    phi_23977_ = _e4589;
                                                                }
                                                                let _e4594 = phi_23977_;
                                                                let _e4598 = global_6.member[(_e4220 + (_e4378 + 1u))];
                                                                let _e4604 = (select(select(u32(_e4598), 0u, (_e4598 < 0f)), 4294967295u, (_e4598 > 4294967000f)) % 12u);
                                                                let _e4607 = select(-1f, 1f, ((_e4604 & 1u) == 0u));
                                                                let _e4610 = select(-1f, 1f, ((_e4604 & 2u) == 0u));
                                                                if (_e4604 < 4u) {
                                                                    phi_24011_ = ((_e4607 * _e4417) + (_e4610 * _e4448));
                                                                } else {
                                                                    if (_e4604 < 8u) {
                                                                        phi_24005_ = ((_e4607 * _e4417) + (_e4610 * _e4510));
                                                                    } else {
                                                                        phi_24005_ = ((_e4607 * _e4448) + (_e4610 * _e4510));
                                                                    }
                                                                    let _e4620 = phi_24005_;
                                                                    phi_24011_ = _e4620;
                                                                }
                                                                let _e4625 = phi_24011_;
                                                                let _e4628 = (_e4408 + (_e4307 * (_e4439 - _e4408)));
                                                                let _e4634 = (_e4532 + (_e4307 * (_e4563 - _e4532)));
                                                                let _e4640 = (_e4628 + (_e4314 * ((_e4470 + (_e4307 * (_e4500 - _e4470))) - _e4628)));
                                                                phi_5924_ = (_e4249 + 1u);
                                                                phi_5927_ = (_e4251 + _e4253);
                                                                phi_5929_ = (_e4253 * 0.5f);
                                                                phi_5931_ = (_e4255 + ((_e4640 + ((((_e4300 * _e4300) * _e4300) * ((_e4300 * ((_e4300 * 6f) - 15f)) + 10f)) * ((_e4634 + (_e4314 * ((_e4594 + (_e4307 * (_e4625 - _e4594))) - _e4634))) - _e4640))) * _e4253));
                                                                phi_5933_ = vec3<f32>(((((_e4223 * _e4257.x) + (_e4226 * _e4257.y)) + (_e4229 * _e4257.z)) * 2.03f), ((((_e4232 * _e4257.x) + (_e4235 * _e4257.y)) + (_e4238 * _e4257.z)) * 2.03f), ((((_e4241 * _e4257.x) + (_e4244 * _e4257.y)) + (_e4247 * _e4257.z)) * 2.03f));
                                                            } else {
                                                                phi_5924_ = u32();
                                                                phi_5927_ = f32();
                                                                phi_5929_ = f32();
                                                                phi_5931_ = f32();
                                                                phi_5933_ = vec3<f32>();
                                                            }
                                                            let _e4672 = phi_5924_;
                                                            let _e4674 = phi_5927_;
                                                            let _e4676 = phi_5929_;
                                                            let _e4678 = phi_5931_;
                                                            let _e4680 = phi_5933_;
                                                            phi_5923_ = _e4672;
                                                            phi_5926_ = _e4674;
                                                            phi_5928_ = _e4676;
                                                            phi_5930_ = _e4678;
                                                            phi_5932_ = _e4680;
                                                            if !(_e4258) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4683 = local_23;
                                                        if (_e4683 > 0f) {
                                                            let _e4686 = local_24;
                                                            let _e4688 = local_25;
                                                            phi_6349_ = (_e4686 / _e4688);
                                                        } else {
                                                            phi_6349_ = 0f;
                                                        }
                                                        let _e4691 = phi_6349_;
                                                        if (_e4141 == 0u) {
                                                            phi_6432_ = _e4137;
                                                            phi_6433_ = _e4139;
                                                            phi_6434_ = _e4143;
                                                            phi_6435_ = _e4145;
                                                            phi_6436_ = _e4147;
                                                            phi_6437_ = (_e4149 + (1.5f * _e4691));
                                                        } else {
                                                            if (_e4141 == 1u) {
                                                                phi_6426_ = _e4137;
                                                                phi_6427_ = _e4139;
                                                                phi_6428_ = _e4143;
                                                                phi_6429_ = _e4145;
                                                                phi_6430_ = _e4147;
                                                                phi_6431_ = (_e4149 + (0.8f * _e4691));
                                                            } else {
                                                                let _e4694 = (_e4141 == 2u);
                                                                if _e4694 {
                                                                    phi_6421_ = _e4137;
                                                                    phi_6422_ = _e4139;
                                                                    phi_6423_ = _e4143;
                                                                    phi_6424_ = _e4145;
                                                                } else {
                                                                    if (_e4141 == 3u) {
                                                                        phi_6417_ = _e4137;
                                                                        phi_6418_ = _e4139;
                                                                        phi_6419_ = _e4143;
                                                                        phi_6420_ = vec3<f32>(_e4691, _e4145.y, _e4145.z);
                                                                    } else {
                                                                        if (_e4141 == 4u) {
                                                                            phi_6413_ = _e4137;
                                                                            phi_6414_ = _e4139;
                                                                            phi_6415_ = _e4143;
                                                                            phi_6416_ = vec3<f32>(_e4145.x, _e4691, _e4145.z);
                                                                        } else {
                                                                            if (_e4141 == 5u) {
                                                                                phi_6409_ = _e4137;
                                                                                phi_6410_ = _e4139;
                                                                                phi_6411_ = _e4143;
                                                                                phi_6412_ = vec3<f32>(_e4145.x, _e4145.y, _e4691);
                                                                            } else {
                                                                                let _e4698 = (_e4141 == 6u);
                                                                                if _e4698 {
                                                                                    phi_6406_ = _e4137;
                                                                                    phi_6408_ = _e4143;
                                                                                } else {
                                                                                    if (_e4141 == 7u) {
                                                                                        phi_6404_ = _e4137;
                                                                                        phi_6405_ = vec3<f32>(_e4691, _e4143.y, _e4143.z);
                                                                                    } else {
                                                                                        if (_e4141 == 8u) {
                                                                                            phi_6402_ = _e4137;
                                                                                            phi_6403_ = vec3<f32>(_e4143.x, _e4691, _e4143.z);
                                                                                        } else {
                                                                                            let _e4701 = (_e4141 == 9u);
                                                                                            if _e4701 {
                                                                                                phi_6401_ = vec3<f32>(_e4143.x, _e4143.y, _e4691);
                                                                                            } else {
                                                                                                phi_6401_ = _e4143;
                                                                                            }
                                                                                            let _e4707 = phi_6401_;
                                                                                            phi_6402_ = select(_e4691, _e4137, _e4701);
                                                                                            phi_6403_ = _e4707;
                                                                                        }
                                                                                        let _e4714 = phi_6402_;
                                                                                        let _e4716 = phi_6403_;
                                                                                        phi_6404_ = _e4714;
                                                                                        phi_6405_ = _e4716;
                                                                                    }
                                                                                    let _e4722 = phi_6404_;
                                                                                    let _e4724 = phi_6405_;
                                                                                    phi_6406_ = _e4722;
                                                                                    phi_6408_ = _e4724;
                                                                                }
                                                                                let _e4726 = phi_6406_;
                                                                                let _e4728 = phi_6408_;
                                                                                phi_6409_ = _e4726;
                                                                                phi_6410_ = select(_e4139, _e4691, _e4698);
                                                                                phi_6411_ = _e4728;
                                                                                phi_6412_ = _e4145;
                                                                            }
                                                                            let _e4735 = phi_6409_;
                                                                            let _e4737 = phi_6410_;
                                                                            let _e4739 = phi_6411_;
                                                                            let _e4741 = phi_6412_;
                                                                            phi_6413_ = _e4735;
                                                                            phi_6414_ = _e4737;
                                                                            phi_6415_ = _e4739;
                                                                            phi_6416_ = _e4741;
                                                                        }
                                                                        let _e4747 = phi_6413_;
                                                                        let _e4749 = phi_6414_;
                                                                        let _e4751 = phi_6415_;
                                                                        let _e4753 = phi_6416_;
                                                                        phi_6417_ = _e4747;
                                                                        phi_6418_ = _e4749;
                                                                        phi_6419_ = _e4751;
                                                                        phi_6420_ = _e4753;
                                                                    }
                                                                    let _e4759 = phi_6417_;
                                                                    let _e4761 = phi_6418_;
                                                                    let _e4763 = phi_6419_;
                                                                    let _e4765 = phi_6420_;
                                                                    phi_6421_ = _e4759;
                                                                    phi_6422_ = _e4761;
                                                                    phi_6423_ = _e4763;
                                                                    phi_6424_ = _e4765;
                                                                }
                                                                let _e4767 = phi_6421_;
                                                                let _e4769 = phi_6422_;
                                                                let _e4771 = phi_6423_;
                                                                let _e4773 = phi_6424_;
                                                                phi_6426_ = _e4767;
                                                                phi_6427_ = _e4769;
                                                                phi_6428_ = _e4771;
                                                                phi_6429_ = _e4773;
                                                                phi_6430_ = select(_e4147, _e4691, _e4694);
                                                                phi_6431_ = _e4149;
                                                            }
                                                            let _e4778 = phi_6426_;
                                                            let _e4780 = phi_6427_;
                                                            let _e4782 = phi_6428_;
                                                            let _e4784 = phi_6429_;
                                                            let _e4786 = phi_6430_;
                                                            let _e4788 = phi_6431_;
                                                            phi_6432_ = _e4778;
                                                            phi_6433_ = _e4780;
                                                            phi_6434_ = _e4782;
                                                            phi_6435_ = _e4784;
                                                            phi_6436_ = _e4786;
                                                            phi_6437_ = _e4788;
                                                        }
                                                        let _e4792 = phi_6432_;
                                                        let _e4794 = phi_6433_;
                                                        let _e4796 = phi_6434_;
                                                        let _e4798 = phi_6435_;
                                                        let _e4800 = phi_6436_;
                                                        let _e4802 = phi_6437_;
                                                        phi_5696_ = _e4792;
                                                        phi_5699_ = _e4794;
                                                        phi_5701_ = (_e4141 + 1u);
                                                        phi_5703_ = _e4796;
                                                        phi_5705_ = _e4798;
                                                        phi_5707_ = _e4800;
                                                        phi_5709_ = _e4802;
                                                    } else {
                                                        phi_5696_ = f32();
                                                        phi_5699_ = f32();
                                                        phi_5701_ = u32();
                                                        phi_5703_ = vec3<f32>();
                                                        phi_5705_ = vec3<f32>();
                                                        phi_5707_ = f32();
                                                        phi_5709_ = f32();
                                                    }
                                                    let _e4805 = phi_5696_;
                                                    let _e4807 = phi_5699_;
                                                    let _e4809 = phi_5701_;
                                                    let _e4811 = phi_5703_;
                                                    let _e4813 = phi_5705_;
                                                    let _e4815 = phi_5707_;
                                                    let _e4817 = phi_5709_;
                                                    phi_5695_ = _e4805;
                                                    phi_5698_ = _e4807;
                                                    phi_5700_ = _e4809;
                                                    phi_5702_ = _e4811;
                                                    phi_5704_ = _e4813;
                                                    phi_5706_ = _e4815;
                                                    phi_5708_ = _e4817;
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
                                                        phi_24087_ = true;
                                                    } else {
                                                        let _e4855 = local_28;
                                                        phi_24087_ = (0.1f >= _e4855);
                                                    }
                                                    let _e4858 = phi_24087_;
                                                    let _e4860 = local_29;
                                                    let _e4863 = (pow(select(_e4860, 0.1f, _e4858), 1.3f) * _e4072);
                                                    phi_6469_ = vec3<f32>((_e4069.x + (((_e4078 * _e4832) + (_e4087 * _e4839)) * _e4863)), (_e4069.y + (((_e4081 * _e4832) + (_e4090 * _e4839)) * _e4863)), (_e4069.z + (((_e4084 * _e4832) + (_e4093 * _e4839)) * _e4863)));
                                                } else {
                                                    phi_6469_ = _e4069;
                                                }
                                                let _e4875 = phi_6469_;
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
                                                    phi_6548_ = (_e4075 * (((exp((-(_e4883) * _e4883)) * (0.15f + (3f * ((_e4893 * _e4893) * (3f - (2f * _e4893)))))) * (0.5f + (0.9f * exp(((_e3852 * -0.6666667f) * (_e3852 * 0.6666667f)))))) + (1.2f * (exp(((_e3841 * -3.3333333f) * (_e3841 * 3.3333333f))) * ((_e4915 * _e4915) * (3f - (2f * _e4915)))))));
                                                } else {
                                                    phi_6548_ = 0f;
                                                }
                                                let _e4929 = phi_6548_;
                                                phi_6549_ = _e4875;
                                                phi_6550_ = _e4929;
                                            } else {
                                                phi_6549_ = _e4069;
                                                phi_6550_ = 0f;
                                            }
                                            let _e4931 = phi_6549_;
                                            let _e4933 = phi_6550_;
                                            phi_6552_ = _e4931;
                                            phi_6555_ = 0u;
                                            loop {
                                                let _e4935 = phi_6552_;
                                                let _e4937 = phi_6555_;
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
                                                        phi_7701_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7702_ = vec3<f32>();
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
                                                            phi_7699_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4961 * exp((-2.5f * pow((sqrt(((_e5647 * _e5647) + (_e5653 * _e5653))) * 3.3333333f), 0.7f)))));
                                                            phi_7700_ = vec3<f32>();
                                                        } else {
                                                            let _e5047 = (((_e4970 * _e4946) + (_e4973 * _e4949)) + (_e4976 * _e4952));
                                                            if (abs(_e5047) < 0.12f) {
                                                                phi_7003_ = select(0.12f, -0.12f, (_e5047 < 0f));
                                                            } else {
                                                                phi_7003_ = _e5047;
                                                            }
                                                            let _e5053 = phi_7003_;
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
                                                            phi_7183_ = 0u;
                                                            phi_7186_ = 0f;
                                                            phi_7188_ = 1f;
                                                            phi_7190_ = 0f;
                                                            phi_7192_ = vec3<f32>((_e5070 * 9f), (_e5075 * 9f), 0f);
                                                            loop {
                                                                let _e5128 = phi_7183_;
                                                                let _e5130 = phi_7186_;
                                                                let _e5132 = phi_7188_;
                                                                let _e5134 = phi_7190_;
                                                                let _e5136 = phi_7192_;
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
                                                                        phi_24182_ = ((_e5266 * _e5174) + (_e5269 * _e5175));
                                                                    } else {
                                                                        if (_e5263 < 8u) {
                                                                            phi_24176_ = ((_e5266 * _e5174) + (_e5269 * _e5176));
                                                                        } else {
                                                                            phi_24176_ = ((_e5266 * _e5175) + (_e5269 * _e5176));
                                                                        }
                                                                        let _e5279 = phi_24176_;
                                                                        phi_24182_ = _e5279;
                                                                    }
                                                                    let _e5284 = phi_24182_;
                                                                    let _e5287 = global_6.member[(_e5099 + _e5244)];
                                                                    let _e5293 = (_e5174 - 1f);
                                                                    let _e5294 = (select(select(u32(_e5287), 0u, (_e5287 < 0f)), 4294967295u, (_e5287 > 4294967000f)) % 12u);
                                                                    let _e5297 = select(-1f, 1f, ((_e5294 & 1u) == 0u));
                                                                    let _e5300 = select(-1f, 1f, ((_e5294 & 2u) == 0u));
                                                                    if (_e5294 < 4u) {
                                                                        phi_24216_ = ((_e5297 * _e5293) + (_e5300 * _e5175));
                                                                    } else {
                                                                        if (_e5294 < 8u) {
                                                                            phi_24210_ = ((_e5297 * _e5293) + (_e5300 * _e5176));
                                                                        } else {
                                                                            phi_24210_ = ((_e5297 * _e5175) + (_e5300 * _e5176));
                                                                        }
                                                                        let _e5310 = phi_24210_;
                                                                        phi_24216_ = _e5310;
                                                                    }
                                                                    let _e5315 = phi_24216_;
                                                                    let _e5318 = global_6.member[(_e5099 + _e5225)];
                                                                    let _e5324 = (_e5175 - 1f);
                                                                    let _e5325 = (select(select(u32(_e5318), 0u, (_e5318 < 0f)), 4294967295u, (_e5318 > 4294967000f)) % 12u);
                                                                    let _e5328 = select(-1f, 1f, ((_e5325 & 1u) == 0u));
                                                                    let _e5331 = select(-1f, 1f, ((_e5325 & 2u) == 0u));
                                                                    if (_e5325 < 4u) {
                                                                        phi_24250_ = ((_e5328 * _e5174) + (_e5331 * _e5324));
                                                                    } else {
                                                                        if (_e5325 < 8u) {
                                                                            phi_24244_ = ((_e5328 * _e5174) + (_e5331 * _e5176));
                                                                        } else {
                                                                            phi_24244_ = ((_e5328 * _e5324) + (_e5331 * _e5176));
                                                                        }
                                                                        let _e5341 = phi_24244_;
                                                                        phi_24250_ = _e5341;
                                                                    }
                                                                    let _e5346 = phi_24250_;
                                                                    let _e5349 = global_6.member[(_e5099 + _e5254)];
                                                                    let _e5355 = (select(select(u32(_e5349), 0u, (_e5349 < 0f)), 4294967295u, (_e5349 > 4294967000f)) % 12u);
                                                                    let _e5358 = select(-1f, 1f, ((_e5355 & 1u) == 0u));
                                                                    let _e5361 = select(-1f, 1f, ((_e5355 & 2u) == 0u));
                                                                    if (_e5355 < 4u) {
                                                                        phi_24284_ = ((_e5358 * _e5293) + (_e5361 * _e5324));
                                                                    } else {
                                                                        if (_e5355 < 8u) {
                                                                            phi_24278_ = ((_e5358 * _e5293) + (_e5361 * _e5176));
                                                                        } else {
                                                                            phi_24278_ = ((_e5358 * _e5324) + (_e5361 * _e5176));
                                                                        }
                                                                        let _e5371 = phi_24278_;
                                                                        phi_24284_ = _e5371;
                                                                    }
                                                                    let _e5376 = phi_24284_;
                                                                    let _e5380 = global_6.member[(_e5099 + (_e5215 + 1u))];
                                                                    let _e5386 = (_e5176 - 1f);
                                                                    let _e5387 = (select(select(u32(_e5380), 0u, (_e5380 < 0f)), 4294967295u, (_e5380 > 4294967000f)) % 12u);
                                                                    let _e5390 = select(-1f, 1f, ((_e5387 & 1u) == 0u));
                                                                    let _e5393 = select(-1f, 1f, ((_e5387 & 2u) == 0u));
                                                                    if (_e5387 < 4u) {
                                                                        phi_24318_ = ((_e5390 * _e5174) + (_e5393 * _e5175));
                                                                    } else {
                                                                        if (_e5387 < 8u) {
                                                                            phi_24312_ = ((_e5390 * _e5174) + (_e5393 * _e5386));
                                                                        } else {
                                                                            phi_24312_ = ((_e5390 * _e5175) + (_e5393 * _e5386));
                                                                        }
                                                                        let _e5403 = phi_24312_;
                                                                        phi_24318_ = _e5403;
                                                                    }
                                                                    let _e5408 = phi_24318_;
                                                                    let _e5412 = global_6.member[(_e5099 + (_e5244 + 1u))];
                                                                    let _e5418 = (select(select(u32(_e5412), 0u, (_e5412 < 0f)), 4294967295u, (_e5412 > 4294967000f)) % 12u);
                                                                    let _e5421 = select(-1f, 1f, ((_e5418 & 1u) == 0u));
                                                                    let _e5424 = select(-1f, 1f, ((_e5418 & 2u) == 0u));
                                                                    if (_e5418 < 4u) {
                                                                        phi_24352_ = ((_e5421 * _e5293) + (_e5424 * _e5175));
                                                                    } else {
                                                                        if (_e5418 < 8u) {
                                                                            phi_24346_ = ((_e5421 * _e5293) + (_e5424 * _e5386));
                                                                        } else {
                                                                            phi_24346_ = ((_e5421 * _e5175) + (_e5424 * _e5386));
                                                                        }
                                                                        let _e5434 = phi_24346_;
                                                                        phi_24352_ = _e5434;
                                                                    }
                                                                    let _e5439 = phi_24352_;
                                                                    let _e5443 = global_6.member[(_e5099 + (_e5225 + 1u))];
                                                                    let _e5449 = (select(select(u32(_e5443), 0u, (_e5443 < 0f)), 4294967295u, (_e5443 > 4294967000f)) % 12u);
                                                                    let _e5452 = select(-1f, 1f, ((_e5449 & 1u) == 0u));
                                                                    let _e5455 = select(-1f, 1f, ((_e5449 & 2u) == 0u));
                                                                    if (_e5449 < 4u) {
                                                                        phi_24386_ = ((_e5452 * _e5174) + (_e5455 * _e5324));
                                                                    } else {
                                                                        if (_e5449 < 8u) {
                                                                            phi_24380_ = ((_e5452 * _e5174) + (_e5455 * _e5386));
                                                                        } else {
                                                                            phi_24380_ = ((_e5452 * _e5324) + (_e5455 * _e5386));
                                                                        }
                                                                        let _e5465 = phi_24380_;
                                                                        phi_24386_ = _e5465;
                                                                    }
                                                                    let _e5470 = phi_24386_;
                                                                    let _e5474 = global_6.member[(_e5099 + (_e5254 + 1u))];
                                                                    let _e5480 = (select(select(u32(_e5474), 0u, (_e5474 < 0f)), 4294967295u, (_e5474 > 4294967000f)) % 12u);
                                                                    let _e5483 = select(-1f, 1f, ((_e5480 & 1u) == 0u));
                                                                    let _e5486 = select(-1f, 1f, ((_e5480 & 2u) == 0u));
                                                                    if (_e5480 < 4u) {
                                                                        phi_24420_ = ((_e5483 * _e5293) + (_e5486 * _e5324));
                                                                    } else {
                                                                        if (_e5480 < 8u) {
                                                                            phi_24414_ = ((_e5483 * _e5293) + (_e5486 * _e5386));
                                                                        } else {
                                                                            phi_24414_ = ((_e5483 * _e5324) + (_e5486 * _e5386));
                                                                        }
                                                                        let _e5496 = phi_24414_;
                                                                        phi_24420_ = _e5496;
                                                                    }
                                                                    let _e5501 = phi_24420_;
                                                                    let _e5504 = (_e5284 + (_e5183 * (_e5315 - _e5284)));
                                                                    let _e5510 = (_e5408 + (_e5183 * (_e5439 - _e5408)));
                                                                    let _e5516 = (_e5504 + (_e5190 * ((_e5346 + (_e5183 * (_e5376 - _e5346))) - _e5504)));
                                                                    phi_7184_ = (_e5128 + 1u);
                                                                    phi_7187_ = (_e5130 + _e5132);
                                                                    phi_7189_ = (_e5132 * 0.5f);
                                                                    phi_7191_ = (_e5134 + ((_e5516 + ((((_e5176 * _e5176) * _e5176) * ((_e5176 * ((_e5176 * 6f) - 15f)) + 10f)) * ((_e5510 + (_e5190 * ((_e5470 + (_e5183 * (_e5501 - _e5470))) - _e5510))) - _e5516))) * _e5132));
                                                                    phi_7193_ = vec3<f32>(((((_e5102 * _e5136.x) + (_e5105 * _e5136.y)) + (_e5108 * _e5136.z)) * 2.03f), ((((_e5111 * _e5136.x) + (_e5114 * _e5136.y)) + (_e5117 * _e5136.z)) * 2.03f), ((((_e5120 * _e5136.x) + (_e5123 * _e5136.y)) + (_e5126 * _e5136.z)) * 2.03f));
                                                                } else {
                                                                    phi_7184_ = u32();
                                                                    phi_7187_ = f32();
                                                                    phi_7189_ = f32();
                                                                    phi_7191_ = f32();
                                                                    phi_7193_ = vec3<f32>();
                                                                }
                                                                let _e5548 = phi_7184_;
                                                                let _e5550 = phi_7187_;
                                                                let _e5552 = phi_7189_;
                                                                let _e5554 = phi_7191_;
                                                                let _e5556 = phi_7193_;
                                                                phi_7183_ = _e5548;
                                                                phi_7186_ = _e5550;
                                                                phi_7188_ = _e5552;
                                                                phi_7190_ = _e5554;
                                                                phi_7192_ = _e5556;
                                                                if !(_e5137) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5559 = local_33;
                                                            if (_e5559 > 0f) {
                                                                let _e5562 = local_34;
                                                                let _e5564 = local_35;
                                                                phi_7606_ = (_e5562 / _e5564);
                                                            } else {
                                                                phi_7606_ = 0f;
                                                            }
                                                            let _e5567 = phi_7606_;
                                                            let _e5569 = (1f + (1.2f * _e5567));
                                                            if (_e5569 != _e5569) {
                                                                phi_24466_ = true;
                                                            } else {
                                                                phi_24466_ = (0f >= _e5569);
                                                            }
                                                            let _e5573 = phi_24466_;
                                                            let _e5578 = ((_e5079 - 1.15f) * -2.8571432f);
                                                            let _e5580 = select(_e5578, 0f, (_e5578 < 0f));
                                                            let _e5582 = select(_e5580, 1f, (_e5580 > 1f));
                                                            let _e5588 = (1f / _e5091);
                                                            if (_e5588 != _e5588) {
                                                                phi_24481_ = true;
                                                            } else {
                                                                phi_24481_ = (4f <= _e5588);
                                                            }
                                                            let _e5592 = phi_24481_;
                                                            let _e5595 = (((exp((_e5079 * -3.5714285f)) * ((_e5582 * _e5582) * (3f - (2f * _e5582)))) * select(_e5588, 4f, _e5592)) * 0.12f);
                                                            let _e5601 = sqrt((((_e5040 * _e5040) + (_e5041 * _e5041)) + (_e5042 * _e5042)));
                                                            let _e5608 = (((_e5040 * _e5006) + (_e5041 * _e5009)) + (_e5042 * _e5012));
                                                            let _e5609 = (1f - _e5091);
                                                            let _e5625 = ((_e5595 * (0.6f + (0.4f * (1f - _e5093)))) + exp((_e5601 * -16.666668f)));
                                                            let _e5631 = (((_e5595 * _e5093) * 1.8f) * select(_e5569, 0f, _e5573));
                                                            let _e5638 = (_e4961 * (1f - ((((0.75f * _e5609) * _e5609) * exp(((_e5608 * -28.571428f) * (_e5608 * 28.571428f)))) * exp((_e5601 * -1.4285715f)))));
                                                            phi_7699_ = vec3<f32>();
                                                            phi_7700_ = vec3<f32>((((1.1633643f * _e5625) + (0.86689025f * _e5631)) * _e5638), (((0.97722596f * _e5625) + (1.0155001f * _e5631)) * _e5638), (((0.74455315f * _e5625) + (1.2384146f * _e5631)) * _e5638));
                                                        }
                                                        let _e5665 = phi_7699_;
                                                        let _e5667 = phi_7700_;
                                                        phi_7701_ = _e5665;
                                                        phi_7702_ = _e5667;
                                                    }
                                                    let _e5669 = phi_7701_;
                                                    let _e5671 = phi_7702_;
                                                    phi_6553_ = (_e4935 + select(_e5671, _e5669, vec3(select(_e4943, true, _e4967))));
                                                    phi_6556_ = (_e4937 + 1u);
                                                } else {
                                                    phi_6553_ = vec3<f32>();
                                                    phi_6556_ = u32();
                                                }
                                                let _e5678 = phi_6553_;
                                                let _e5680 = phi_6556_;
                                                phi_6552_ = _e5678;
                                                phi_6555_ = _e5680;
                                                if !(_e4938) {
                                                    break;
                                                }
                                            }
                                            phi_32872_ = _e4067;
                                            if _e4067 {
                                                break;
                                            }
                                            phi_7721_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7724_ = 0u;
                                            phi_7726_ = _e4933;
                                            loop {
                                                let _e5683 = phi_7721_;
                                                let _e5685 = phi_7724_;
                                                let _e5687 = phi_7726_;
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
                                                        phi_9043_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
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
                                                            phi_8028_ = 6u;
                                                            phi_8029_ = 1.2f;
                                                            phi_8030_ = _e5780;
                                                        } else {
                                                            let _e5782 = (_e5697 == 1u);
                                                            if _e5782 {
                                                                phi_8027_ = vec3<f32>((_e5767 * 1.54f), (_e5773 * 1.54f), 0f);
                                                            } else {
                                                                phi_8027_ = _e5780;
                                                            }
                                                            let _e5787 = phi_8027_;
                                                            phi_8028_ = select(6u, 5u, _e5782);
                                                            phi_8029_ = select(1f, 0.8f, _e5782);
                                                            phi_8030_ = _e5787;
                                                        }
                                                        let _e5791 = phi_8028_;
                                                        let _e5793 = phi_8029_;
                                                        let _e5795 = phi_8030_;
                                                        if _e5781 {
                                                            phi_8047_ = vec3<f32>(((_e5767 * 3.7400002f) + -7f), ((_e5773 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8047_ = vec3<f32>((_e5767 * 3.3000002f), (_e5773 * 3.3000002f), 0f);
                                                        }
                                                        let _e5805 = phi_8047_;
                                                        phi_8049_ = 0u;
                                                        phi_8052_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8054_ = 0f;
                                                        phi_8056_ = 0f;
                                                        phi_8058_ = 0f;
                                                        loop {
                                                            let _e5808 = phi_8049_;
                                                            let _e5810 = phi_8052_;
                                                            let _e5812 = phi_8054_;
                                                            let _e5814 = phi_8056_;
                                                            let _e5816 = phi_8058_;
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
                                                                    phi_8100_ = true;
                                                                    phi_8101_ = false;
                                                                    phi_8102_ = false;
                                                                } else {
                                                                    if (_e5808 <= 4u) {
                                                                        phi_8078_ = (_e5697 == 2u);
                                                                    } else {
                                                                        phi_8078_ = true;
                                                                    }
                                                                    let _e5822 = phi_8078_;
                                                                    if _e5822 {
                                                                        let _e5823 = (_e5808 == 5u);
                                                                        if _e5823 {
                                                                            if _e5781 {
                                                                                phi_8094_ = false;
                                                                            } else {
                                                                                phi_8094_ = (_e5697 == 2u);
                                                                            }
                                                                            let _e5826 = phi_8094_;
                                                                            phi_8096_ = _e5826;
                                                                        } else {
                                                                            phi_8096_ = false;
                                                                        }
                                                                        let _e5828 = phi_8096_;
                                                                        phi_8098_ = select(false, _e5781, _e5823);
                                                                        phi_8099_ = _e5828;
                                                                    } else {
                                                                        phi_8098_ = false;
                                                                        phi_8099_ = false;
                                                                    }
                                                                    let _e5831 = phi_8098_;
                                                                    let _e5833 = phi_8099_;
                                                                    phi_8100_ = select(true, false, _e5822);
                                                                    phi_8101_ = _e5831;
                                                                    phi_8102_ = _e5833;
                                                                }
                                                                let _e5836 = phi_8100_;
                                                                let _e5838 = phi_8101_;
                                                                let _e5840 = phi_8102_;
                                                                if select(select(_e5840, false, _e5836), true, select(_e5838, true, _e5836)) {
                                                                    if _e5818 {
                                                                        phi_8149_ = 3u;
                                                                        phi_8150_ = vec3<f32>(((_e5767 * 1.7600001f) + 3f), ((_e5773 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5808 < 4u) {
                                                                            let _e5857 = (_e5808 - 1u);
                                                                            if (_e5857 == 0u) {
                                                                                phi_24500_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_24500_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5857 == 1u)));
                                                                            }
                                                                            let _e5863 = phi_24500_;
                                                                            phi_8147_ = 4u;
                                                                            phi_8148_ = (_e5795 + _e5863);
                                                                        } else {
                                                                            let _e5845 = (_e5808 == 4u);
                                                                            if _e5845 {
                                                                                phi_8146_ = (_e5795 + vec3<f32>((_e5810.x * _e5793), (_e5810.y * _e5793), (_e5810.z * _e5793)));
                                                                            } else {
                                                                                phi_8146_ = _e5805;
                                                                            }
                                                                            let _e5855 = phi_8146_;
                                                                            phi_8147_ = select(select(4u, 5u, _e5781), _e5791, _e5845);
                                                                            phi_8148_ = _e5855;
                                                                        }
                                                                        let _e5866 = phi_8147_;
                                                                        let _e5868 = phi_8148_;
                                                                        phi_8149_ = _e5866;
                                                                        phi_8150_ = _e5868;
                                                                    }
                                                                    let _e5875 = phi_8149_;
                                                                    let _e5877 = phi_8150_;
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
                                                                    phi_8285_ = 0u;
                                                                    phi_8288_ = 0f;
                                                                    phi_8290_ = 1f;
                                                                    phi_8292_ = 0f;
                                                                    phi_8294_ = _e5877;
                                                                    loop {
                                                                        let _e5909 = phi_8285_;
                                                                        let _e5911 = phi_8288_;
                                                                        let _e5913 = phi_8290_;
                                                                        let _e5915 = phi_8292_;
                                                                        let _e5917 = phi_8294_;
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
                                                                                phi_24561_ = ((_e6047 * _e5955) + (_e6050 * _e5956));
                                                                            } else {
                                                                                if (_e6044 < 8u) {
                                                                                    phi_24555_ = ((_e6047 * _e5955) + (_e6050 * _e5957));
                                                                                } else {
                                                                                    phi_24555_ = ((_e6047 * _e5956) + (_e6050 * _e5957));
                                                                                }
                                                                                let _e6060 = phi_24555_;
                                                                                phi_24561_ = _e6060;
                                                                            }
                                                                            let _e6065 = phi_24561_;
                                                                            let _e6068 = global_6.member[(_e5880 + _e6025)];
                                                                            let _e6074 = (_e5955 - 1f);
                                                                            let _e6075 = (select(select(u32(_e6068), 0u, (_e6068 < 0f)), 4294967295u, (_e6068 > 4294967000f)) % 12u);
                                                                            let _e6078 = select(-1f, 1f, ((_e6075 & 1u) == 0u));
                                                                            let _e6081 = select(-1f, 1f, ((_e6075 & 2u) == 0u));
                                                                            if (_e6075 < 4u) {
                                                                                phi_24595_ = ((_e6078 * _e6074) + (_e6081 * _e5956));
                                                                            } else {
                                                                                if (_e6075 < 8u) {
                                                                                    phi_24589_ = ((_e6078 * _e6074) + (_e6081 * _e5957));
                                                                                } else {
                                                                                    phi_24589_ = ((_e6078 * _e5956) + (_e6081 * _e5957));
                                                                                }
                                                                                let _e6091 = phi_24589_;
                                                                                phi_24595_ = _e6091;
                                                                            }
                                                                            let _e6096 = phi_24595_;
                                                                            let _e6099 = global_6.member[(_e5880 + _e6006)];
                                                                            let _e6105 = (_e5956 - 1f);
                                                                            let _e6106 = (select(select(u32(_e6099), 0u, (_e6099 < 0f)), 4294967295u, (_e6099 > 4294967000f)) % 12u);
                                                                            let _e6109 = select(-1f, 1f, ((_e6106 & 1u) == 0u));
                                                                            let _e6112 = select(-1f, 1f, ((_e6106 & 2u) == 0u));
                                                                            if (_e6106 < 4u) {
                                                                                phi_24629_ = ((_e6109 * _e5955) + (_e6112 * _e6105));
                                                                            } else {
                                                                                if (_e6106 < 8u) {
                                                                                    phi_24623_ = ((_e6109 * _e5955) + (_e6112 * _e5957));
                                                                                } else {
                                                                                    phi_24623_ = ((_e6109 * _e6105) + (_e6112 * _e5957));
                                                                                }
                                                                                let _e6122 = phi_24623_;
                                                                                phi_24629_ = _e6122;
                                                                            }
                                                                            let _e6127 = phi_24629_;
                                                                            let _e6130 = global_6.member[(_e5880 + _e6035)];
                                                                            let _e6136 = (select(select(u32(_e6130), 0u, (_e6130 < 0f)), 4294967295u, (_e6130 > 4294967000f)) % 12u);
                                                                            let _e6139 = select(-1f, 1f, ((_e6136 & 1u) == 0u));
                                                                            let _e6142 = select(-1f, 1f, ((_e6136 & 2u) == 0u));
                                                                            if (_e6136 < 4u) {
                                                                                phi_24663_ = ((_e6139 * _e6074) + (_e6142 * _e6105));
                                                                            } else {
                                                                                if (_e6136 < 8u) {
                                                                                    phi_24657_ = ((_e6139 * _e6074) + (_e6142 * _e5957));
                                                                                } else {
                                                                                    phi_24657_ = ((_e6139 * _e6105) + (_e6142 * _e5957));
                                                                                }
                                                                                let _e6152 = phi_24657_;
                                                                                phi_24663_ = _e6152;
                                                                            }
                                                                            let _e6157 = phi_24663_;
                                                                            let _e6161 = global_6.member[(_e5880 + (_e5996 + 1u))];
                                                                            let _e6167 = (_e5957 - 1f);
                                                                            let _e6168 = (select(select(u32(_e6161), 0u, (_e6161 < 0f)), 4294967295u, (_e6161 > 4294967000f)) % 12u);
                                                                            let _e6171 = select(-1f, 1f, ((_e6168 & 1u) == 0u));
                                                                            let _e6174 = select(-1f, 1f, ((_e6168 & 2u) == 0u));
                                                                            if (_e6168 < 4u) {
                                                                                phi_24697_ = ((_e6171 * _e5955) + (_e6174 * _e5956));
                                                                            } else {
                                                                                if (_e6168 < 8u) {
                                                                                    phi_24691_ = ((_e6171 * _e5955) + (_e6174 * _e6167));
                                                                                } else {
                                                                                    phi_24691_ = ((_e6171 * _e5956) + (_e6174 * _e6167));
                                                                                }
                                                                                let _e6184 = phi_24691_;
                                                                                phi_24697_ = _e6184;
                                                                            }
                                                                            let _e6189 = phi_24697_;
                                                                            let _e6193 = global_6.member[(_e5880 + (_e6025 + 1u))];
                                                                            let _e6199 = (select(select(u32(_e6193), 0u, (_e6193 < 0f)), 4294967295u, (_e6193 > 4294967000f)) % 12u);
                                                                            let _e6202 = select(-1f, 1f, ((_e6199 & 1u) == 0u));
                                                                            let _e6205 = select(-1f, 1f, ((_e6199 & 2u) == 0u));
                                                                            if (_e6199 < 4u) {
                                                                                phi_24731_ = ((_e6202 * _e6074) + (_e6205 * _e5956));
                                                                            } else {
                                                                                if (_e6199 < 8u) {
                                                                                    phi_24725_ = ((_e6202 * _e6074) + (_e6205 * _e6167));
                                                                                } else {
                                                                                    phi_24725_ = ((_e6202 * _e5956) + (_e6205 * _e6167));
                                                                                }
                                                                                let _e6215 = phi_24725_;
                                                                                phi_24731_ = _e6215;
                                                                            }
                                                                            let _e6220 = phi_24731_;
                                                                            let _e6224 = global_6.member[(_e5880 + (_e6006 + 1u))];
                                                                            let _e6230 = (select(select(u32(_e6224), 0u, (_e6224 < 0f)), 4294967295u, (_e6224 > 4294967000f)) % 12u);
                                                                            let _e6233 = select(-1f, 1f, ((_e6230 & 1u) == 0u));
                                                                            let _e6236 = select(-1f, 1f, ((_e6230 & 2u) == 0u));
                                                                            if (_e6230 < 4u) {
                                                                                phi_24765_ = ((_e6233 * _e5955) + (_e6236 * _e6105));
                                                                            } else {
                                                                                if (_e6230 < 8u) {
                                                                                    phi_24759_ = ((_e6233 * _e5955) + (_e6236 * _e6167));
                                                                                } else {
                                                                                    phi_24759_ = ((_e6233 * _e6105) + (_e6236 * _e6167));
                                                                                }
                                                                                let _e6246 = phi_24759_;
                                                                                phi_24765_ = _e6246;
                                                                            }
                                                                            let _e6251 = phi_24765_;
                                                                            let _e6255 = global_6.member[(_e5880 + (_e6035 + 1u))];
                                                                            let _e6261 = (select(select(u32(_e6255), 0u, (_e6255 < 0f)), 4294967295u, (_e6255 > 4294967000f)) % 12u);
                                                                            let _e6264 = select(-1f, 1f, ((_e6261 & 1u) == 0u));
                                                                            let _e6267 = select(-1f, 1f, ((_e6261 & 2u) == 0u));
                                                                            if (_e6261 < 4u) {
                                                                                phi_24799_ = ((_e6264 * _e6074) + (_e6267 * _e6105));
                                                                            } else {
                                                                                if (_e6261 < 8u) {
                                                                                    phi_24793_ = ((_e6264 * _e6074) + (_e6267 * _e6167));
                                                                                } else {
                                                                                    phi_24793_ = ((_e6264 * _e6105) + (_e6267 * _e6167));
                                                                                }
                                                                                let _e6277 = phi_24793_;
                                                                                phi_24799_ = _e6277;
                                                                            }
                                                                            let _e6282 = phi_24799_;
                                                                            let _e6285 = (_e6065 + (_e5964 * (_e6096 - _e6065)));
                                                                            let _e6291 = (_e6189 + (_e5964 * (_e6220 - _e6189)));
                                                                            let _e6297 = (_e6285 + (_e5971 * ((_e6127 + (_e5964 * (_e6157 - _e6127))) - _e6285)));
                                                                            phi_8286_ = (_e5909 + 1u);
                                                                            phi_8289_ = (_e5911 + _e5913);
                                                                            phi_8291_ = (_e5913 * 0.5f);
                                                                            phi_8293_ = (_e5915 + ((_e6297 + ((((_e5957 * _e5957) * _e5957) * ((_e5957 * ((_e5957 * 6f) - 15f)) + 10f)) * ((_e6291 + (_e5971 * ((_e6251 + (_e5964 * (_e6282 - _e6251))) - _e6291))) - _e6297))) * _e5913));
                                                                            phi_8295_ = vec3<f32>(((((_e5883 * _e5917.x) + (_e5886 * _e5917.y)) + (_e5889 * _e5917.z)) * 2.03f), ((((_e5892 * _e5917.x) + (_e5895 * _e5917.y)) + (_e5898 * _e5917.z)) * 2.03f), ((((_e5901 * _e5917.x) + (_e5904 * _e5917.y)) + (_e5907 * _e5917.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8286_ = u32();
                                                                            phi_8289_ = f32();
                                                                            phi_8291_ = f32();
                                                                            phi_8293_ = f32();
                                                                            phi_8295_ = vec3<f32>();
                                                                        }
                                                                        let _e6329 = phi_8286_;
                                                                        let _e6331 = phi_8289_;
                                                                        let _e6333 = phi_8291_;
                                                                        let _e6335 = phi_8293_;
                                                                        let _e6337 = phi_8295_;
                                                                        phi_8285_ = _e6329;
                                                                        phi_8288_ = _e6331;
                                                                        phi_8290_ = _e6333;
                                                                        phi_8292_ = _e6335;
                                                                        phi_8294_ = _e6337;
                                                                        if !(_e5918) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6340 = local_36;
                                                                    if (_e6340 > 0f) {
                                                                        let _e6343 = local_37;
                                                                        let _e6345 = local_38;
                                                                        phi_8708_ = (_e6343 / _e6345);
                                                                    } else {
                                                                        phi_8708_ = 0f;
                                                                    }
                                                                    let _e6348 = phi_8708_;
                                                                    if _e5818 {
                                                                        phi_8743_ = _e5810;
                                                                        phi_8744_ = _e5812;
                                                                        phi_8745_ = _e5814;
                                                                    } else {
                                                                        if (_e5808 == 1u) {
                                                                            phi_8740_ = vec3<f32>(_e6348, _e5810.y, _e5810.z);
                                                                            phi_8741_ = _e5812;
                                                                            phi_8742_ = _e5814;
                                                                        } else {
                                                                            if (_e5808 == 2u) {
                                                                                phi_8737_ = vec3<f32>(_e5810.x, _e6348, _e5810.z);
                                                                                phi_8738_ = _e5812;
                                                                                phi_8739_ = _e5814;
                                                                            } else {
                                                                                if (_e5808 == 3u) {
                                                                                    phi_8734_ = vec3<f32>(_e5810.x, _e5810.y, _e6348);
                                                                                    phi_8735_ = _e5812;
                                                                                    phi_8736_ = _e5814;
                                                                                } else {
                                                                                    let _e6352 = (_e5808 == 4u);
                                                                                    phi_8734_ = _e5810;
                                                                                    phi_8735_ = select(_e6348, _e5812, _e6352);
                                                                                    phi_8736_ = select(_e5814, _e6348, _e6352);
                                                                                }
                                                                                let _e6360 = phi_8734_;
                                                                                let _e6362 = phi_8735_;
                                                                                let _e6364 = phi_8736_;
                                                                                phi_8737_ = _e6360;
                                                                                phi_8738_ = _e6362;
                                                                                phi_8739_ = _e6364;
                                                                            }
                                                                            let _e6370 = phi_8737_;
                                                                            let _e6372 = phi_8738_;
                                                                            let _e6374 = phi_8739_;
                                                                            phi_8740_ = _e6370;
                                                                            phi_8741_ = _e6372;
                                                                            phi_8742_ = _e6374;
                                                                        }
                                                                        let _e6380 = phi_8740_;
                                                                        let _e6382 = phi_8741_;
                                                                        let _e6384 = phi_8742_;
                                                                        phi_8743_ = _e6380;
                                                                        phi_8744_ = _e6382;
                                                                        phi_8745_ = _e6384;
                                                                    }
                                                                    let _e6386 = phi_8743_;
                                                                    let _e6388 = phi_8744_;
                                                                    let _e6390 = phi_8745_;
                                                                    phi_8747_ = _e6386;
                                                                    phi_8748_ = _e6388;
                                                                    phi_8749_ = _e6390;
                                                                    phi_8750_ = select(_e5816, _e6348, _e5818);
                                                                } else {
                                                                    phi_8747_ = _e5810;
                                                                    phi_8748_ = _e5812;
                                                                    phi_8749_ = _e5814;
                                                                    phi_8750_ = _e5816;
                                                                }
                                                                let _e6393 = phi_8747_;
                                                                let _e6395 = phi_8748_;
                                                                let _e6397 = phi_8749_;
                                                                let _e6399 = phi_8750_;
                                                                phi_8050_ = (_e5808 + 1u);
                                                                phi_8053_ = _e6393;
                                                                phi_8055_ = _e6395;
                                                                phi_8057_ = _e6397;
                                                                phi_8059_ = _e6399;
                                                            } else {
                                                                phi_8050_ = u32();
                                                                phi_8053_ = vec3<f32>();
                                                                phi_8055_ = f32();
                                                                phi_8057_ = f32();
                                                                phi_8059_ = f32();
                                                            }
                                                            let _e6402 = phi_8050_;
                                                            let _e6404 = phi_8053_;
                                                            let _e6406 = phi_8055_;
                                                            let _e6408 = phi_8057_;
                                                            let _e6410 = phi_8059_;
                                                            phi_8049_ = _e6402;
                                                            phi_8052_ = _e6404;
                                                            phi_8054_ = _e6406;
                                                            phi_8056_ = _e6408;
                                                            phi_8058_ = _e6410;
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
                                                            phi_9042_ = type_60(vec3<f32>((((3.1815925f * _e6540) + (0.14634217f * _e6569)) * _e6578), (((0.38179109f * _e6540) + (1.2439084f * _e6569)) * _e6578), (((0.69995034f * _e6540) + (1.0975662f * _e6569)) * _e6578)), (_e5718 * _e6564));
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
                                                                phi_9041_ = type_60((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5715 * _e6511)), (_e5718 * _e6511));
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
                                                                    phi_9040_ = type_60(vec3<f32>(((((3.1815925f * _e6453) + (_e6463 * 0.10243952f)) + _e6468) * _e5715), ((((0.38179109f * _e6453) + (_e6463 * 0.8707359f)) + _e6468) * _e5715), ((((0.69995034f * _e6453) + (_e6463 * 0.76829636f)) + _e6468) * _e5715)), 0f);
                                                                } else {
                                                                    let _e6420 = local_40;
                                                                    let _e6422 = ((_e6420 - -0.2f) * 1.6666666f);
                                                                    let _e6424 = select(_e6422, 0f, (_e6422 < 0f));
                                                                    let _e6426 = select(_e6424, 1f, (_e6424 > 1f));
                                                                    let _e6432 = ((_e6416 - 1f) * -1.4285715f);
                                                                    let _e6434 = select(_e6432, 0f, (_e6432 < 0f));
                                                                    let _e6436 = select(_e6434, 1f, (_e6434 > 1f));
                                                                    phi_9040_ = type_60(vec3<f32>(0f, 0f, 0f), (_e5718 * (((_e6426 * _e6426) * (3f - (2f * _e6426))) * ((_e6436 * _e6436) * (3f - (2f * _e6436))))));
                                                                }
                                                                let _e6487 = phi_9040_;
                                                                phi_9041_ = _e6487;
                                                            }
                                                            let _e6517 = phi_9041_;
                                                            phi_9042_ = _e6517;
                                                        }
                                                        let _e6586 = phi_9042_;
                                                        phi_9043_ = _e6586;
                                                    }
                                                    let _e6588 = phi_9043_;
                                                    phi_7722_ = (_e5683 + _e6588.member);
                                                    phi_7725_ = (_e5685 + 1u);
                                                    phi_7727_ = (_e5687 + _e6588.member_1);
                                                } else {
                                                    phi_7722_ = vec3<f32>();
                                                    phi_7725_ = u32();
                                                    phi_7727_ = f32();
                                                }
                                                let _e6595 = phi_7722_;
                                                let _e6597 = phi_7725_;
                                                let _e6599 = phi_7727_;
                                                phi_7721_ = _e6595;
                                                phi_7724_ = _e6597;
                                                phi_7726_ = _e6599;
                                                if !(_e5688) {
                                                    break;
                                                }
                                            }
                                            phi_32872_ = _e4067;
                                            if _e4067 {
                                                break;
                                            }
                                            let _e6602 = local_45;
                                            if (_e6602 <= 0f) {
                                                phi_9072_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6605 = local_46;
                                                let _e6608 = local_47;
                                                let _e6613 = local_48;
                                                phi_9072_ = vec3<f32>(exp((_e6608 * -0.81f)), exp(-(_e6605)), exp((_e6613 * -1.25f)));
                                            }
                                            let _e6618 = phi_9072_;
                                            let _e6620 = local_49;
                                            let _e6625 = local_50;
                                            let _e6630 = local_51;
                                            let _e6635 = local_52;
                                            let _e6639 = local_53;
                                            let _e6643 = local_54;
                                            let _e6648 = global_6.member[(_e1292 + 9u)];
                                            phi_32881_ = _e4067;
                                            phi_9119_ = vec3<f32>((_e3772.x + ((((_e6620.x * _e6618.x) + _e6635.x) * _e6648) * _e3774.x)), (_e3772.y + ((((_e6625.y * _e6618.y) + _e6639.y) * _e6648) * _e3774.y)), (_e3772.z + ((((_e6630.z * _e6618.z) + _e6643.z) * _e6648) * _e3774.z)));
                                        } else {
                                            phi_32881_ = _e3770;
                                            phi_9119_ = _e3772;
                                        }
                                        let _e6666 = phi_32881_;
                                        let _e6668 = phi_9119_;
                                        phi_32880_ = _e6666;
                                        phi_9120_ = _e6668;
                                    } else {
                                        phi_32880_ = _e3770;
                                        phi_9120_ = _e3778;
                                    }
                                    let _e6670 = phi_32880_;
                                    let _e6672 = phi_9120_;
                                    phi_32879_ = _e6670;
                                    phi_9121_ = _e6672;
                                } else {
                                    phi_32879_ = _e323;
                                    phi_9121_ = _e1437;
                                }
                                let _e6674 = phi_32879_;
                                let _e6676 = phi_9121_;
                                phi_32875_ = _e6674;
                                phi_10394_ = (vec3<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z) * _e6676);
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
                                    phi_20704_ = type_58(0u, type_58().member_1);
                                    phi_20705_ = type_58();
                                    phi_20706_ = true;
                                } else {
                                    if (_e437 < 0f) {
                                        phi_20676_ = -(sqrt(_e452));
                                    } else {
                                        phi_20676_ = sqrt(_e452);
                                    }
                                    let _e459 = phi_20676_;
                                    let _e461 = -((_e437 + _e459));
                                    let _e462 = (_e461 == 0f);
                                    if _e462 {
                                        phi_20699_ = type_58(1u, type_57(0f, 0f));
                                        phi_20700_ = type_58();
                                    } else {
                                        let _e463 = (_e450 / _e461);
                                        if (_e461 < _e463) {
                                            phi_20688_ = type_57(_e461, _e463);
                                        } else {
                                            phi_20688_ = type_57(_e463, _e461);
                                        }
                                        let _e468 = phi_20688_;
                                        phi_20699_ = type_58();
                                        phi_20700_ = type_58(1u, _e468);
                                    }
                                    let _e471 = phi_20699_;
                                    let _e473 = phi_20700_;
                                    phi_20704_ = _e471;
                                    phi_20705_ = _e473;
                                    phi_20706_ = _e462;
                                }
                                let _e478 = phi_20704_;
                                let _e480 = phi_20705_;
                                let _e482 = phi_20706_;
                                if _e482 {
                                    phi_20710_ = _e478;
                                } else {
                                    phi_20710_ = _e480;
                                }
                                let _e484 = phi_20710_;
                                if (bitcast<i32>(_e484.member) != 0i) {
                                    phi_20597_ = type_58(0u, _e484.member_1);
                                } else {
                                    phi_20597_ = type_58(1u, type_58().member_1);
                                }
                                let _e494 = phi_20597_;
                                if (bitcast<i32>(_e494.member) != 0i) {
                                    phi_20644_ = type_58(0u, type_58().member_1);
                                } else {
                                    if (_e494.member_1.member_1 <= 0f) {
                                        phi_20640_ = type_58(0u, type_58().member_1);
                                    } else {
                                        if (_e494.member_1.member != _e494.member_1.member) {
                                            phi_20748_ = true;
                                        } else {
                                            phi_20748_ = (0f >= _e494.member_1.member);
                                        }
                                        let _e506 = phi_20748_;
                                        let _e507 = select(_e494.member_1.member, 0f, _e506);
                                        let _e508 = (_e449 + _e375);
                                        if (_e508 > 0f) {
                                            let _e510 = (_e451 - _e508);
                                            if (_e510 < 0f) {
                                                phi_20856_ = type_58(0u, type_58().member_1);
                                                phi_20857_ = type_58();
                                                phi_20858_ = true;
                                            } else {
                                                if (_e437 < 0f) {
                                                    phi_20828_ = -(sqrt(_e510));
                                                } else {
                                                    phi_20828_ = sqrt(_e510);
                                                }
                                                let _e517 = phi_20828_;
                                                let _e519 = -((_e437 + _e517));
                                                let _e520 = (_e519 == 0f);
                                                if _e520 {
                                                    phi_20851_ = type_58(1u, type_57(0f, 0f));
                                                    phi_20852_ = type_58();
                                                } else {
                                                    let _e521 = (_e508 / _e519);
                                                    if (_e519 < _e521) {
                                                        phi_20840_ = type_57(_e519, _e521);
                                                    } else {
                                                        phi_20840_ = type_57(_e521, _e519);
                                                    }
                                                    let _e526 = phi_20840_;
                                                    phi_20851_ = type_58();
                                                    phi_20852_ = type_58(1u, _e526);
                                                }
                                                let _e529 = phi_20851_;
                                                let _e531 = phi_20852_;
                                                phi_20856_ = _e529;
                                                phi_20857_ = _e531;
                                                phi_20858_ = _e520;
                                            }
                                            let _e536 = phi_20856_;
                                            let _e538 = phi_20857_;
                                            let _e540 = phi_20858_;
                                            if _e540 {
                                                phi_20862_ = _e536;
                                            } else {
                                                phi_20862_ = _e538;
                                            }
                                            let _e542 = phi_20862_;
                                            if (bitcast<i32>(_e542.member) != 0i) {
                                                phi_20796_ = select(_e542.member_1.member_1, _e542.member_1.member, (_e542.member_1.member >= 0f));
                                            } else {
                                                phi_20796_ = -1f;
                                            }
                                            let _e553 = phi_20796_;
                                            if (_e553 >= 0f) {
                                                if (_e494.member_1.member_1 != _e494.member_1.member_1) {
                                                    phi_20900_ = true;
                                                } else {
                                                    phi_20900_ = (_e553 <= _e494.member_1.member_1);
                                                }
                                                let _e558 = phi_20900_;
                                                phi_20625_ = select(_e494.member_1.member_1, _e553, _e558);
                                            } else {
                                                phi_20625_ = _e494.member_1.member_1;
                                            }
                                            let _e561 = phi_20625_;
                                            phi_20627_ = _e561;
                                        } else {
                                            phi_20627_ = _e494.member_1.member_1;
                                        }
                                        let _e563 = phi_20627_;
                                        if (_e563 <= _e507) {
                                            phi_20636_ = type_58(0u, type_58().member_1);
                                        } else {
                                            phi_20636_ = type_58(1u, type_57(_e507, _e563));
                                        }
                                        let _e571 = phi_20636_;
                                        phi_20640_ = _e571;
                                    }
                                    let _e576 = phi_20640_;
                                    phi_20644_ = _e576;
                                }
                                let _e581 = phi_20644_;
                                if (bitcast<i32>(_e581.member) != 0i) {
                                    let _e587 = (_e581.member_1.member < _e334.member.w);
                                    if _e587 {
                                        if (_e581.member_1.member_1 != _e581.member_1.member_1) {
                                            phi_20915_ = true;
                                        } else {
                                            phi_20915_ = (_e334.member.w <= _e581.member_1.member_1);
                                        }
                                        let _e593 = phi_20915_;
                                        let _e594 = select(_e581.member_1.member_1, _e334.member.w, _e593);
                                        phi_9515_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9518_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9520_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9522_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9524_ = 0u;
                                        loop {
                                            let _e596 = phi_9515_;
                                            let _e598 = phi_9518_;
                                            let _e600 = phi_9520_;
                                            let _e602 = phi_9522_;
                                            let _e604 = phi_9524_;
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
                                                    phi_32394_ = true;
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
                                                    phi_9625_ = type_57((_e581.member_1.member + ((_e614 * _e695) * _e695)), (_e614 * ((_e693 * _e693) - (_e690 * _e690))));
                                                } else {
                                                    if (_e613 <= _e615) {
                                                        let _e673 = (f32((31u - _e604)) * 0.03125f);
                                                        let _e676 = (f32((32u - _e604)) * 0.03125f);
                                                        let _e678 = ((_e673 + _e676) * 0.5f);
                                                        phi_9624_ = type_57((_e594 - ((_e614 * _e678) * _e678)), (_e614 * ((_e676 * _e676) - (_e673 * _e673))));
                                                    } else {
                                                        let _e621 = round(((32f * _e612) / (_e612 + _e613)));
                                                        let _e626 = select(select(u32(_e621), 0u, (_e621 < 0f)), 4294967295u, (_e621 > 4294967000f));
                                                        if (_e626 < 1u) {
                                                            phi_9608_ = 1u;
                                                        } else {
                                                            phi_9608_ = select(_e626, 31u, (_e626 > 31u));
                                                        }
                                                        let _e631 = phi_9608_;
                                                        if (_e604 < _e631) {
                                                            let _e652 = ((_e631 - 1u) - _e604);
                                                            let _e654 = f32(_e631);
                                                            let _e655 = (f32(_e652) / _e654);
                                                            let _e658 = (f32((_e652 + 1u)) / _e654);
                                                            let _e660 = ((_e655 + _e658) * 0.5f);
                                                            phi_9623_ = type_57((_e611 - ((_e612 * _e660) * _e660)), (_e612 * ((_e658 * _e658) - (_e655 * _e655))));
                                                        } else {
                                                            let _e634 = (_e604 - _e631);
                                                            let _e636 = f32((32u - _e631));
                                                            let _e637 = (f32(_e634) / _e636);
                                                            let _e640 = (f32((_e634 + 1u)) / _e636);
                                                            let _e642 = ((_e637 + _e640) * 0.5f);
                                                            phi_9623_ = type_57((_e611 + ((_e613 * _e642) * _e642)), (_e613 * ((_e640 * _e640) - (_e637 * _e637))));
                                                        }
                                                        let _e670 = phi_9623_;
                                                        phi_9624_ = _e670;
                                                    }
                                                    let _e688 = phi_9624_;
                                                    phi_9625_ = _e688;
                                                }
                                                let _e705 = phi_9625_;
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
                                                    phi_21115_ = true;
                                                } else {
                                                    phi_21115_ = (0f >= _e742);
                                                }
                                                let _e746 = phi_21115_;
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
                                                    phi_9953_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e797 = (1f / _e736);
                                                    let _e805 = ((((_e728 * _e797) * _e381) + ((_e729 * _e797) * _e384)) + ((_e730 * _e797) * _e387));
                                                    if (_e738 != _e738) {
                                                        phi_21242_ = true;
                                                    } else {
                                                        phi_21242_ = (0f >= _e738);
                                                    }
                                                    let _e809 = phi_21242_;
                                                    let _e813 = sqrt((select(_e738, 0f, _e809) / (_e372 - _e369)));
                                                    if (_e813 != _e813) {
                                                        phi_21257_ = true;
                                                    } else {
                                                        phi_21257_ = (1f <= _e813);
                                                    }
                                                    let _e817 = phi_21257_;
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
                                                    phi_9953_ = vec3<f32>(((((_e854 * _e861) + (_e867 * _e843)) * _e880) + (((_e885 * _e861) + (_e897 * _e843)) * _e841)), ((((_e857 * _e861) + (_e870 * _e843)) * _e880) + (((_e888 * _e861) + (_e900 * _e843)) * _e841)), ((((_e860 * _e861) + (_e873 * _e843)) * _e880) + (((_e891 * _e861) + (_e903 * _e843)) * _e841)));
                                                }
                                                let _e918 = phi_9953_;
                                                let _e932 = (_e726 + _e378);
                                                if (_e932 > 0f) {
                                                    let _e938 = (((_e728 * _e381) + (_e729 * _e384)) + (_e730 * _e387));
                                                    let _e940 = ((_e938 * _e938) - _e932);
                                                    if (_e938 >= 0f) {
                                                        phi_10027_ = vec3<f32>();
                                                        phi_10028_ = true;
                                                    } else {
                                                        let _e942 = (_e940 <= 0f);
                                                        if _e942 {
                                                            phi_10026_ = vec3<f32>();
                                                        } else {
                                                            let _e945 = (-(_e938) - sqrt(_e940));
                                                            phi_10026_ = vec3<f32>((_e711 + (_e381 * _e945)), (_e712 + (_e384 * _e945)), (_e713 + (_e387 * _e945)));
                                                        }
                                                        let _e954 = phi_10026_;
                                                        phi_10027_ = _e954;
                                                        phi_10028_ = _e942;
                                                    }
                                                    let _e956 = phi_10027_;
                                                    let _e958 = phi_10028_;
                                                    phi_10033_ = _e956;
                                                    phi_10034_ = select(true, false, _e958);
                                                } else {
                                                    phi_10033_ = vec3<f32>(_e711, _e712, _e713);
                                                    phi_10034_ = true;
                                                }
                                                let _e961 = phi_10033_;
                                                let _e963 = phi_10034_;
                                                if _e963 {
                                                    let _e965 = (_e961.x - _e360);
                                                    let _e967 = (_e961.y - _e363);
                                                    let _e969 = (_e961.z - _e366);
                                                    let _e975 = sqrt((((_e965 * _e965) + (_e967 * _e967)) + (_e969 * _e969)));
                                                    let _e976 = (1f / _e975);
                                                    let _e994 = ((((((_e961.x * _e961.x) + (_e961.y * _e961.y)) + (_e961.z * _e961.z)) - (2f * (((_e961.x * _e360) + (_e961.y * _e363)) + (_e961.z * _e366)))) + _e375) / (_e975 + _e369));
                                                    if (_e994 != _e994) {
                                                        phi_21381_ = true;
                                                    } else {
                                                        phi_21381_ = (0f >= _e994);
                                                    }
                                                    let _e998 = phi_21381_;
                                                    let _e1002 = sqrt((select(_e994, 0f, _e998) / (_e372 - _e369)));
                                                    if (_e1002 != _e1002) {
                                                        phi_21396_ = true;
                                                    } else {
                                                        phi_21396_ = (1f <= _e1002);
                                                    }
                                                    let _e1006 = phi_21396_;
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
                                                    phi_10325_ = vec3<f32>(((((_e1048 * _e1055) + (_e1061 * _e1037)) * _e1074) + (((_e1079 * _e1055) + (_e1091 * _e1037)) * _e1035)), ((((_e1051 * _e1055) + (_e1064 * _e1037)) * _e1074) + (((_e1082 * _e1055) + (_e1094 * _e1037)) * _e1035)), ((((_e1054 * _e1055) + (_e1067 * _e1037)) * _e1074) + (((_e1085 * _e1055) + (_e1097 * _e1037)) * _e1035)));
                                                } else {
                                                    phi_10325_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1112 = phi_10325_;
                                                let _e1113 = (vec3<f32>(_e760, _e762, _e764) + _e1112);
                                                let _e1119 = (_e769 * _e1113.y);
                                                let _e1131 = exp(-((((_e399 * _e1113.x) + _e1119) + (_e415 * _e1113.z))));
                                                let _e1133 = exp(-((((_e402 * _e1113.x) + _e1119) + (_e418 * _e1113.z))));
                                                let _e1135 = exp(-((((_e405 * _e1113.x) + _e1119) + (_e421 * _e1113.z))));
                                                phi_9516_ = vec3<f32>((_e596.x + ((exp(-((((_e399 * _e760) + _e770) + (_e415 * _e764)))) * ((_e399 * _e751) + _e789)) * _e918.x)), (_e596.y + ((exp(-((((_e402 * _e760) + _e770) + (_e418 * _e764)))) * ((_e402 * _e751) + _e789)) * _e918.y)), (_e596.z + ((exp(-((((_e405 * _e760) + _e770) + (_e421 * _e764)))) * ((_e405 * _e751) + _e789)) * _e918.z)));
                                                phi_9519_ = vec3<f32>((_e598.x + (_e1131 * _e754)), (_e598.y + (_e1133 * _e754)), (_e598.z + (_e1135 * _e754)));
                                                phi_9521_ = vec3<f32>((_e600.x + (_e1131 * _e751)), (_e600.y + (_e1133 * _e751)), (_e600.z + (_e1135 * _e751)));
                                                phi_9523_ = vec3<f32>((_e760 + _e756), (_e762 + _e757), (_e764 + _e758));
                                                phi_9525_ = (_e604 + 1u);
                                            } else {
                                                phi_9516_ = vec3<f32>();
                                                phi_9519_ = vec3<f32>();
                                                phi_9521_ = vec3<f32>();
                                                phi_9523_ = vec3<f32>();
                                                phi_9525_ = u32();
                                            }
                                            let _e1162 = phi_9516_;
                                            let _e1164 = phi_9519_;
                                            let _e1166 = phi_9521_;
                                            let _e1168 = phi_9523_;
                                            let _e1170 = phi_9525_;
                                            phi_9515_ = _e1162;
                                            phi_9518_ = _e1164;
                                            phi_9520_ = _e1166;
                                            phi_9522_ = _e1168;
                                            phi_9524_ = _e1170;
                                            phi_32394_ = _e323;
                                            if !(_e605) {
                                                break;
                                            }
                                        }
                                        let _e1173 = phi_32394_;
                                        phi_32872_ = _e1173;
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
                                        phi_32878_ = _e1173;
                                        phi_10382_ = type_61(vec3<f32>(exp(-((((_e399 * _e1233.x) + _e1242) + (_e415 * _e1247.z)))), exp(-((((_e402 * _e1233.x) + _e1242) + (_e418 * _e1247.z)))), exp(-((((_e405 * _e1233.x) + _e1242) + (_e421 * _e1247.z))))), (vec3<f32>(((((_e1176.x * _e1187) + (_e1179.x * _e1205)) + _e1216.x) * _e396), ((((_e1176.y * _e1187) + (_e1179.y * _e1205)) + _e1220.y) * _e396), ((((_e1176.z * _e1187) + (_e1179.z * _e1205)) + _e1224.z) * _e396)) * 0.38317f));
                                    } else {
                                        phi_32878_ = _e323;
                                        phi_10382_ = type_61();
                                    }
                                    let _e1264 = phi_32878_;
                                    let _e1266 = phi_10382_;
                                    phi_32877_ = _e1264;
                                    phi_10384_ = _e1266;
                                    phi_10385_ = select(true, false, _e587);
                                } else {
                                    phi_32877_ = _e323;
                                    phi_10384_ = type_61();
                                    phi_10385_ = true;
                                }
                                let _e1269 = phi_32877_;
                                let _e1271 = phi_10384_;
                                let _e1273 = phi_10385_;
                                if _e1273 {
                                    phi_10390_ = type_61(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10390_ = _e1271;
                                }
                                let _e1275 = phi_10390_;
                                phi_32875_ = _e1269;
                                phi_10394_ = (vec3<f32>(_e334.member_2.x, _e334.member_2.y, _e334.member_2.z) * _e1275.member_1);
                            }
                            let _e6680 = phi_32875_;
                            let _e6682 = phi_10394_;
                            phi_32874_ = _e6680;
                            phi_10395_ = _e6682;
                        }
                        let _e6684 = phi_32874_;
                        let _e6686 = phi_10395_;
                        phi_32873_ = _e6684;
                        phi_513_ = (_e325 + _e6686);
                        phi_516_ = (_e327 + 1u);
                    } else {
                        phi_32873_ = _e323;
                        phi_513_ = vec3<f32>();
                        phi_516_ = u32();
                    }
                    let _e6690 = phi_32873_;
                    let _e6692 = phi_513_;
                    let _e6694 = phi_516_;
                    phi_32411_ = _e6690;
                    phi_512_ = _e6692;
                    phi_515_ = _e6694;
                    phi_32872_ = _e6690;
                    if !(_e328) {
                        break;
                    }
                }
                let _e6697 = phi_32872_;
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
    var phi_11195_: type_49;
    var phi_24994_: f32;
    var phi_25006_: type_57;
    var phi_25017_: type_58;
    var phi_25018_: type_58;
    var phi_25022_: type_58;
    var phi_25023_: type_58;
    var phi_25024_: bool;
    var phi_25028_: type_58;
    var phi_24915_: type_58;
    var phi_25066_: bool;
    var phi_25146_: f32;
    var phi_25158_: type_57;
    var phi_25169_: type_58;
    var phi_25170_: type_58;
    var phi_25174_: type_58;
    var phi_25175_: type_58;
    var phi_25176_: bool;
    var phi_25180_: type_58;
    var phi_25114_: f32;
    var phi_25218_: bool;
    var phi_24943_: f32;
    var phi_24945_: f32;
    var phi_24954_: type_58;
    var phi_24958_: type_58;
    var phi_24962_: type_58;
    var phi_25233_: bool;
    var phi_12714_: vec3<f32>;
    var phi_12717_: u32;
    var phi_12801_: u32;
    var phi_12816_: type_57;
    var phi_12817_: type_57;
    var phi_12818_: type_57;
    var phi_25432_: bool;
    var phi_12715_: vec3<f32>;
    var phi_12718_: u32;
    var phi_32883_: bool;
    var local_69: vec3<f32>;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var phi_35023_: bool;
    var phi_12843_: vec3<f32>;
    var phi_35022_: bool;
    var phi_12845_: vec3<f32>;
    var phi_12846_: bool;
    var phi_35020_: bool;
    var phi_12853_: vec3<f32>;
    var phi_12854_: type_42;
    var phi_35019_: bool;
    var phi_12855_: vec3<f32>;
    var phi_12856_: type_42;
    var phi_13017_: type_61;
    var phi_12983_: vec3<f32>;
    var phi_12984_: vec3<f32>;
    var phi_13018_: type_61;
    var phi_13035_: i32;
    var phi_13038_: u32;
    var phi_13092_: bool;
    var phi_13098_: bool;
    var phi_13105_: u32;
    var phi_13107_: i32;
    var phi_13108_: u32;
    var phi_13036_: i32;
    var phi_13039_: u32;
    var phi_13110_: bool;
    var local_72: u32;
    var phi_13385_: vec3<f32>;
    var phi_13399_: vec3<f32>;
    var phi_25499_: bool;
    var phi_13432_: f32;
    var phi_13540_: vec3<f32>;
    var phi_13546_: f32;
    var phi_25572_: bool;
    var phi_25561_: f32;
    var phi_13627_: vec3<f32>;
    var phi_13628_: vec3<f32>;
    var phi_13629_: vec3<f32>;
    var phi_33017_: u32;
    var phi_13631_: vec3<f32>;
    var phi_13632_: vec3<f32>;
    var phi_13633_: vec3<f32>;
    var phi_13634_: bool;
    var phi_13649_: vec3<f32>;
    var phi_13650_: vec3<f32>;
    var phi_33015_: u32;
    var phi_13652_: vec3<f32>;
    var phi_13653_: vec3<f32>;
    var phi_13654_: vec3<f32>;
    var phi_33014_: u32;
    var phi_13655_: vec3<f32>;
    var phi_13656_: vec3<f32>;
    var phi_13657_: vec3<f32>;
    var phi_13658_: bool;
    var phi_33013_: u32;
    var phi_13659_: vec3<f32>;
    var phi_13660_: vec3<f32>;
    var phi_13661_: vec3<f32>;
    var phi_13662_: bool;
    var phi_13663_: bool;
    var phi_25645_: bool;
    var phi_25807_: bool;
    var phi_25822_: bool;
    var phi_25837_: bool;
    var phi_25864_: bool;
    var phi_25879_: bool;
    var phi_25894_: bool;
    var phi_25921_: bool;
    var phi_25936_: bool;
    var phi_25951_: bool;
    var phi_13842_: f32;
    var phi_13850_: f32;
    var phi_13858_: f32;
    var phi_26148_: bool;
    var phi_26137_: f32;
    var phi_13965_: vec3<f32>;
    var phi_26176_: bool;
    var phi_13966_: vec3<f32>;
    var phi_13967_: vec3<f32>;
    var phi_13968_: vec3<f32>;
    var phi_33401_: vec3<f32>;
    var phi_33324_: vec3<f32>;
    var phi_33245_: u32;
    var phi_33163_: f32;
    var phi_13970_: vec3<f32>;
    var phi_13971_: vec3<f32>;
    var phi_13972_: bool;
    var phi_26191_: bool;
    var phi_26242_: bool;
    var phi_26231_: f32;
    var phi_33372_: vec3<f32>;
    var phi_13505_: vec3<f32>;
    var phi_33724_: vec3<f32>;
    var phi_33518_: f32;
    var phi_33371_: vec3<f32>;
    var phi_33059_: u32;
    var phi_13507_: vec3<f32>;
    var phi_13508_: bool;
    var phi_33670_: vec3<f32>;
    var phi_33478_: f32;
    var phi_33399_: vec3<f32>;
    var phi_33322_: vec3<f32>;
    var phi_33243_: u32;
    var phi_33161_: f32;
    var phi_33084_: u32;
    var phi_33010_: u32;
    var phi_13980_: vec3<f32>;
    var phi_13981_: vec3<f32>;
    var phi_13982_: vec3<f32>;
    var phi_13983_: bool;
    var phi_26308_: bool;
    var phi_14052_: vec3<f32>;
    var phi_33067_: u32;
    var phi_14054_: vec3<f32>;
    var phi_14055_: bool;
    var phi_33066_: u32;
    var phi_14056_: vec3<f32>;
    var phi_14057_: bool;
    var phi_33064_: u32;
    var phi_14063_: vec3<f32>;
    var phi_14064_: bool;
    var phi_14097_: type_70;
    var phi_33069_: u32;
    var phi_14099_: vec3<f32>;
    var phi_14100_: type_70;
    var phi_14101_: bool;
    var phi_14107_: type_70;
    var phi_33073_: u32;
    var phi_14108_: type_70;
    var phi_33667_: vec3<f32>;
    var phi_33475_: f32;
    var phi_33396_: vec3<f32>;
    var phi_33319_: vec3<f32>;
    var phi_33240_: u32;
    var phi_33158_: f32;
    var phi_33081_: u32;
    var phi_33072_: u32;
    var phi_14110_: type_70;
    var phi_14111_: bool;
    var phi_34996_: bool;
    var phi_33071_: u32;
    var phi_14122_: vec3<f32>;
    var phi_14125_: type_42;
    var phi_14127_: u32;
    var phi_14129_: u32;
    var phi_16818_: u32;
    var phi_16821_: u32;
    var phi_16819_: u32;
    var phi_16822_: u32;
    var local_73: u32;
    var phi_16859_: u32;
    var phi_16862_: u32;
    var phi_16860_: u32;
    var phi_16863_: u32;
    var local_74: u32;
    var local_75: u32;
    var phi_16939_: f32;
    var local_76: u32;
    var local_77: u32;
    var phi_16988_: f32;
    var phi_17144_: f32;
    var phi_17151_: vec3<f32>;
    var phi_35416_: u32;
    var phi_17153_: f32;
    var phi_17154_: vec3<f32>;
    var phi_17155_: vec3<f32>;
    var phi_16291_: u32;
    var phi_16294_: u32;
    var phi_16292_: u32;
    var phi_16295_: u32;
    var local_78: u32;
    var local_79: u32;
    var phi_16687_: f32;
    var phi_16690_: f32;
    var phi_26581_: bool;
    var phi_26562_: f32;
    var phi_16691_: f32;
    var phi_26596_: bool;
    var phi_26619_: bool;
    var phi_26634_: bool;
    var phi_16442_: f32;
    var phi_26649_: bool;
    var phi_26664_: bool;
    var phi_26679_: bool;
    var phi_26744_: bool;
    var phi_26832_: bool;
    var phi_26813_: f32;
    var phi_16507_: type_74;
    var phi_16693_: type_74;
    var phi_16771_: bool;
    var phi_16780_: vec3<f32>;
    var phi_16782_: vec3<f32>;
    var phi_16783_: bool;
    var phi_16788_: f32;
    var phi_16789_: vec3<f32>;
    var phi_35424_: u32;
    var phi_16790_: f32;
    var phi_16791_: vec3<f32>;
    var phi_16792_: vec3<f32>;
    var phi_16793_: f32;
    var phi_35423_: u32;
    var phi_16794_: f32;
    var phi_16795_: vec3<f32>;
    var phi_16796_: vec3<f32>;
    var phi_16797_: f32;
    var phi_35415_: u32;
    var phi_17156_: f32;
    var phi_17157_: vec3<f32>;
    var phi_17158_: vec3<f32>;
    var phi_17159_: f32;
    var phi_26847_: bool;
    var phi_26957_: f32;
    var phi_26985_: f32;
    var phi_27013_: f32;
    var phi_27147_: bool;
    var phi_27162_: bool;
    var phi_27177_: bool;
    var phi_27204_: bool;
    var phi_27219_: bool;
    var phi_27234_: bool;
    var phi_27261_: bool;
    var phi_27276_: bool;
    var phi_27291_: bool;
    var phi_14463_: f32;
    var phi_14471_: f32;
    var phi_14479_: f32;
    var phi_27488_: bool;
    var phi_27477_: f32;
    var phi_14586_: vec3<f32>;
    var phi_27516_: bool;
    var phi_14587_: vec3<f32>;
    var phi_14599_: type_60;
    var phi_14600_: type_60;
    var phi_14601_: bool;
    var phi_14606_: type_60;
    var phi_27553_: bool;
    var phi_27583_: bool;
    var phi_27572_: f32;
    var phi_27622_: vec3<f32>;
    var phi_14634_: type_60;
    var phi_14635_: type_60;
    var phi_27644_: vec3<f32>;
    var phi_14636_: type_60;
    var phi_14637_: type_60;
    var phi_14638_: bool;
    var phi_14642_: type_60;
    var phi_27676_: bool;
    var phi_27747_: bool;
    var phi_15462_: f32;
    var phi_27762_: bool;
    var phi_27777_: bool;
    var phi_15935_: vec3<f32>;
    var phi_15936_: vec3<f32>;
    var phi_15937_: bool;
    var phi_15942_: vec3<f32>;
    var phi_15943_: bool;
    var phi_27901_: bool;
    var phi_27916_: bool;
    var phi_16234_: vec3<f32>;
    var phi_16236_: vec3<f32>;
    var phi_16263_: vec3<f32>;
    var phi_16264_: vec3<f32>;
    var phi_15080_: vec3<f32>;
    var phi_15081_: vec3<f32>;
    var phi_15082_: bool;
    var phi_15087_: vec3<f32>;
    var phi_15088_: bool;
    var phi_28078_: bool;
    var phi_28093_: bool;
    var phi_15379_: vec3<f32>;
    var phi_15381_: vec3<f32>;
    var phi_35458_: u32;
    var phi_16265_: f32;
    var phi_16266_: vec3<f32>;
    var phi_16267_: vec3<f32>;
    var phi_35457_: u32;
    var phi_16268_: f32;
    var phi_16269_: vec3<f32>;
    var phi_16270_: vec3<f32>;
    var phi_35414_: u32;
    var phi_17160_: f32;
    var phi_17161_: vec3<f32>;
    var phi_17162_: vec3<f32>;
    var phi_17163_: f32;
    var phi_14225_: f32;
    var phi_14226_: f32;
    var phi_35413_: u32;
    var phi_17164_: f32;
    var phi_17165_: vec3<f32>;
    var phi_17166_: vec3<f32>;
    var phi_17167_: f32;
    var phi_28146_: bool;
    var phi_28256_: f32;
    var phi_28284_: f32;
    var phi_28312_: f32;
    var phi_28446_: bool;
    var phi_28461_: bool;
    var phi_28476_: bool;
    var phi_28503_: bool;
    var phi_28518_: bool;
    var phi_28533_: bool;
    var phi_28560_: bool;
    var phi_28575_: bool;
    var phi_28590_: bool;
    var phi_17390_: f32;
    var phi_17398_: f32;
    var phi_17406_: f32;
    var phi_28787_: bool;
    var phi_28776_: f32;
    var phi_17513_: vec3<f32>;
    var phi_28815_: bool;
    var phi_17514_: vec3<f32>;
    var phi_17526_: type_60;
    var phi_17527_: type_60;
    var phi_17528_: bool;
    var phi_17533_: type_60;
    var phi_28852_: bool;
    var phi_28882_: bool;
    var phi_28871_: f32;
    var phi_28921_: vec3<f32>;
    var phi_17561_: type_60;
    var phi_17562_: type_60;
    var phi_28943_: vec3<f32>;
    var phi_17563_: type_60;
    var phi_17564_: type_60;
    var phi_17565_: bool;
    var phi_17569_: type_60;
    var phi_28957_: f32;
    var phi_28960_: f32;
    var phi_17578_: f32;
    var phi_17611_: u32;
    var phi_17614_: u32;
    var phi_17616_: u32;
    var phi_17618_: u32;
    var phi_17620_: u32;
    var phi_17622_: vec4<f32>;
    var phi_17624_: f32;
    var phi_17993_: u32;
    var phi_17995_: u32;
    var phi_17996_: u32;
    var phi_17998_: bool;
    var phi_17999_: bool;
    var phi_18000_: u32;
    var phi_18001_: u32;
    var phi_18002_: u32;
    var phi_18003_: bool;
    var phi_18004_: bool;
    var phi_18005_: bool;
    var phi_29058_: bool;
    var phi_29088_: bool;
    var phi_29035_: type_57;
    var phi_29038_: type_57;
    var phi_29039_: bool;
    var phi_29044_: type_57;
    var phi_29145_: bool;
    var phi_29160_: bool;
    var phi_29175_: bool;
    var phi_29190_: bool;
    var phi_29122_: type_57;
    var phi_29125_: type_57;
    var phi_29126_: bool;
    var phi_29131_: type_57;
    var phi_29232_: bool;
    var phi_29247_: bool;
    var phi_29262_: bool;
    var phi_29277_: bool;
    var phi_29209_: type_57;
    var phi_29212_: type_57;
    var phi_29213_: bool;
    var phi_29218_: type_57;
    var phi_29013_: bool;
    var phi_29016_: bool;
    var phi_18034_: f32;
    var phi_29379_: bool;
    var phi_29409_: bool;
    var phi_29356_: type_57;
    var phi_29359_: type_57;
    var phi_29360_: bool;
    var phi_29365_: type_57;
    var phi_29466_: bool;
    var phi_29481_: bool;
    var phi_29496_: bool;
    var phi_29511_: bool;
    var phi_29443_: type_57;
    var phi_29446_: type_57;
    var phi_29447_: bool;
    var phi_29452_: type_57;
    var phi_29553_: bool;
    var phi_29568_: bool;
    var phi_29583_: bool;
    var phi_29598_: bool;
    var phi_29530_: type_57;
    var phi_29533_: type_57;
    var phi_29534_: bool;
    var phi_29539_: type_57;
    var phi_29334_: bool;
    var phi_29337_: bool;
    var phi_18043_: f32;
    var phi_18054_: u32;
    var phi_18065_: u32;
    var phi_18104_: u32;
    var phi_18105_: u32;
    var phi_18116_: u32;
    var phi_18117_: u32;
    var phi_18118_: u32;
    var phi_18119_: u32;
    var phi_18121_: u32;
    var phi_18122_: u32;
    var phi_18124_: u32;
    var phi_18125_: u32;
    var phi_18126_: bool;
    var phi_18127_: bool;
    var phi_17919_: vec4<f32>;
    var phi_17920_: vec4<f32>;
    var phi_17921_: bool;
    var phi_17925_: vec4<f32>;
    var phi_17926_: bool;
    var phi_17831_: bool;
    var phi_17868_: bool;
    var phi_17874_: f32;
    var phi_17884_: vec4<f32>;
    var phi_17885_: vec4<f32>;
    var phi_17886_: bool;
    var phi_17890_: vec4<f32>;
    var phi_17891_: bool;
    var phi_17895_: vec4<f32>;
    var phi_17896_: bool;
    var phi_17927_: vec4<f32>;
    var phi_17928_: bool;
    var phi_17760_: vec4<f32>;
    var phi_17761_: vec4<f32>;
    var phi_17762_: bool;
    var phi_17766_: vec4<f32>;
    var phi_17767_: bool;
    var phi_17768_: vec4<f32>;
    var phi_17769_: bool;
    var phi_17770_: bool;
    var phi_17778_: vec4<f32>;
    var phi_17779_: bool;
    var phi_17780_: vec4<f32>;
    var phi_17781_: bool;
    var phi_17782_: bool;
    var phi_17790_: vec4<f32>;
    var phi_17791_: bool;
    var phi_17937_: vec4<f32>;
    var phi_17938_: bool;
    var phi_17959_: f32;
    var phi_17965_: u32;
    var phi_17612_: u32;
    var phi_17615_: u32;
    var phi_17617_: u32;
    var phi_17619_: u32;
    var phi_17621_: u32;
    var phi_17623_: vec4<f32>;
    var phi_17625_: f32;
    var phi_18138_: vec4<f32>;
    var phi_18139_: bool;
    var phi_18140_: bool;
    var phi_34924_: bool;
    var phi_20566_: bool;
    var phi_20565_: vec4<f32>;
    var local_80: u32;
    var local_81: vec4<f32>;
    var phi_18145_: type_76;
    var local_82: u32;
    var phi_18151_: type_76;
    var phi_18203_: vec3<f32>;
    var phi_18204_: type_42;
    var phi_18205_: vec3<f32>;
    var phi_18206_: type_42;
    var phi_35366_: bool;
    var phi_18207_: vec3<f32>;
    var phi_18208_: type_42;
    var phi_18209_: u32;
    var phi_35365_: bool;
    var phi_18210_: vec3<f32>;
    var phi_18211_: type_42;
    var phi_18212_: u32;
    var phi_35651_: u32;
    var phi_35364_: bool;
    var phi_14123_: vec3<f32>;
    var phi_14126_: type_42;
    var phi_14128_: u32;
    var phi_14130_: u32;
    var phi_35371_: u32;
    var phi_35363_: bool;
    var phi_29749_: bool;
    var phi_29779_: bool;
    var phi_29768_: f32;
    var phi_29699_: bool;
    var phi_29835_: vec3<f32>;
    var phi_29991_: bool;
    var phi_30006_: bool;
    var phi_30021_: bool;
    var phi_29930_: vec3<f32>;
    var phi_29933_: vec3<f32>;
    var phi_29934_: bool;
    var phi_29704_: vec3<f32>;
    var phi_29708_: vec3<f32>;
    var phi_29709_: bool;
    var phi_30121_: bool;
    var phi_30095_: vec3<f32>;
    var phi_29716_: vec3<f32>;
    var phi_30136_: bool;
    var phi_30246_: f32;
    var phi_30274_: f32;
    var phi_30302_: f32;
    var phi_30436_: bool;
    var phi_30451_: bool;
    var phi_30466_: bool;
    var phi_30493_: bool;
    var phi_30508_: bool;
    var phi_30523_: bool;
    var phi_30550_: bool;
    var phi_30565_: bool;
    var phi_30580_: bool;
    var phi_18431_: f32;
    var phi_18439_: f32;
    var phi_18447_: f32;
    var phi_30777_: bool;
    var phi_30766_: f32;
    var phi_18554_: vec3<f32>;
    var phi_30805_: bool;
    var phi_18555_: vec3<f32>;
    var phi_18567_: type_60;
    var phi_18568_: type_60;
    var phi_18569_: bool;
    var phi_18574_: type_60;
    var phi_30842_: bool;
    var phi_30872_: bool;
    var phi_30861_: f32;
    var phi_30911_: vec3<f32>;
    var phi_18602_: type_60;
    var phi_18603_: type_60;
    var phi_30933_: vec3<f32>;
    var phi_18604_: type_60;
    var phi_18605_: type_60;
    var phi_18606_: bool;
    var phi_18610_: type_60;
    var phi_30958_: bool;
    var phi_18686_: vec3<f32>;
    var phi_37205_: u32;
    var phi_18688_: vec3<f32>;
    var phi_18689_: bool;
    var phi_37204_: u32;
    var phi_18690_: vec3<f32>;
    var phi_18691_: bool;
    var phi_37202_: u32;
    var phi_18697_: vec3<f32>;
    var phi_18698_: bool;
    var local_83: u32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: type_42;
    var phi_18727_: vec3<f32>;
    var local_88: u32;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: type_42;
    var phi_18743_: type_70;
    var phi_18744_: type_70;
    var phi_37209_: u32;
    var phi_18746_: type_70;
    var phi_18747_: type_70;
    var phi_18748_: bool;
    var phi_37208_: u32;
    var phi_18749_: type_70;
    var phi_18750_: type_70;
    var phi_18751_: bool;
    var phi_18752_: bool;
    var local_93: u32;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: vec3<f32>;
    var local_97: type_42;
    var phi_18758_: type_70;
    var local_98: u32;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: type_42;
    var phi_18763_: type_70;
    var phi_18764_: type_70;
    var phi_18765_: bool;
    var phi_18769_: type_70;
    var phi_18770_: type_70;
    var phi_18771_: bool;
    var phi_13169_: bool;
    var phi_13178_: vec3<f32>;
    var phi_13180_: vec3<f32>;
    var phi_13181_: bool;
    var phi_13340_: f32;
    var phi_13343_: f32;
    var phi_31179_: bool;
    var phi_31160_: f32;
    var phi_13344_: f32;
    var phi_31197_: f32;
    var phi_31200_: f32;
    var phi_13347_: f32;
    var phi_13348_: f32;
    var phi_13349_: f32;
    var phi_18775_: type_70;
    var phi_18776_: type_70;
    var phi_18777_: bool;
    var phi_11454_: f32;
    var phi_31247_: f32;
    var phi_31250_: f32;
    var phi_11456_: f32;
    var phi_11457_: f32;
    var phi_31276_: bool;
    var phi_11508_: f32;
    var phi_31294_: f32;
    var phi_31297_: f32;
    var phi_11510_: f32;
    var phi_31308_: bool;
    var phi_31323_: bool;
    var phi_11979_: vec3<f32>;
    var phi_11980_: vec3<f32>;
    var phi_11981_: bool;
    var phi_11986_: vec3<f32>;
    var phi_11987_: bool;
    var phi_31447_: bool;
    var phi_31462_: bool;
    var phi_12278_: vec3<f32>;
    var phi_12280_: vec3<f32>;
    var phi_12307_: vec3<f32>;
    var phi_12321_: vec3<f32>;
    var phi_18781_: type_70;
    var phi_18782_: type_70;
    var phi_18783_: bool;
    var phi_18787_: type_70;

    switch bitcast<i32>(0u) {
        default: {
            let _e278 = global;
            let _e288 = arrayLength((&global_9.member));
            let _e292 = global_1.member.member_7[2u];
            let _e296 = (_e278.x + (_e278.y * 4194240u));
            let _e301 = atomicLoad((&global_8.member[((_e292 * 4u) + 3u)]));
            if (_e296 >= _e301) {
                phi_11195_ = type_49(0u, type_49().member_1);
            } else {
                let _e306 = global_1.member.member[0u];
                let _e311 = global_1.member.member[1u];
                let _e317 = atomicLoad((&global_8.member[((8u + ((_e292 * _e306) * _e311)) + _e296)]));
                phi_11195_ = type_49(1u, _e317);
            }
            let _e323 = phi_11195_;
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
                        phi_11457_ = 1f;
                    } else {
                        if ((_e376 & 2u) == 0u) {
                            phi_11456_ = 1f;
                        } else {
                            let _e6525 = global_1.member.member_3[1u];
                            let _e6528 = global_1.member.member_3[2u];
                            let _e6531 = ((atan2(_e331.member_1.z, _e331.member_1.x) * 0.15915494f) + 0.5f);
                            let _e6535 = select(_e331.member_1.y, -1f, (_e331.member_1.y < -1f));
                            let _e6538 = acos(select(_e6535, 1f, (_e6535 > 1f)));
                            let _e6540 = f32(_e6525);
                            let _e6541 = ((_e6531 - floor(_e6531)) * _e6540);
                            let _e6546 = select(select(u32(_e6541), 0u, (_e6541 < 0f)), 4294967295u, (_e6541 > 4294967000f));
                            let _e6547 = (_e6525 - 1u);
                            let _e6550 = f32(_e6528);
                            let _e6551 = ((_e6538 * 0.31830987f) * _e6550);
                            let _e6556 = select(select(u32(_e6551), 0u, (_e6551 < 0f)), 4294967295u, (_e6551 > 4294967000f));
                            let _e6557 = (_e6528 - 1u);
                            let _e6559 = select(_e6556, _e6557, (_e6557 < _e6556));
                            let _e6560 = sin(_e6538);
                            if (_e6560 <= 0f) {
                                phi_11454_ = 0f;
                            } else {
                                let _e6564 = global_1.member.member_2[3u];
                                let _e6567 = global_1.member.member_3[0u];
                                let _e6571 = (_e6564 + _e6559);
                                let _e6574 = global_6.member[(_e6571 + 1u)];
                                let _e6576 = global_6.member[_e6571];
                                let _e6579 = ((_e6567 + (_e6559 * (_e6525 + 1u))) + select(_e6546, _e6547, (_e6547 < _e6546)));
                                let _e6582 = global_6.member[(_e6579 + 1u)];
                                let _e6584 = global_6.member[_e6579];
                                phi_11454_ = ((((_e6574 - _e6576) * _e6550) * ((_e6582 - _e6584) * _e6540)) / (19.73921f * _e6560));
                            }
                            let _e6591 = phi_11454_;
                            if (_e331.member_1.w >= 300000000000000000000000000000000000000f) {
                                phi_31250_ = 1f;
                            } else {
                                let _e6593 = (_e331.member_1.w * _e331.member_1.w);
                                let _e6595 = (_e6593 + (_e6591 * _e6591));
                                if (_e6595 == 0f) {
                                    phi_31247_ = 0f;
                                } else {
                                    phi_31247_ = (_e6593 / _e6595);
                                }
                                let _e6599 = phi_31247_;
                                phi_31250_ = _e6599;
                            }
                            let _e6601 = phi_31250_;
                            phi_11456_ = _e6601;
                        }
                        let _e6603 = phi_11456_;
                        phi_11457_ = _e6603;
                    }
                    let _e6605 = phi_11457_;
                    if ((_e376 & 4u) == 0u) {
                        phi_12321_ = _e360;
                    } else {
                        if _e370 {
                            phi_11510_ = 1f;
                        } else {
                            let _e6616 = global_1.member.member_5;
                            let _e6620 = (_e331.member_1.x - _e6616.x);
                            let _e6621 = (_e331.member_1.y - _e6616.y);
                            let _e6622 = (_e331.member_1.z - _e6616.z);
                            let _e6629 = (sqrt((((_e6620 * _e6620) + (_e6621 * _e6621)) + (_e6622 * _e6622))) * 0.5f);
                            if (_e6629 != _e6629) {
                                phi_31276_ = true;
                            } else {
                                phi_31276_ = (1f <= _e6629);
                            }
                            let _e6633 = phi_31276_;
                            let _e6639 = global_1.member.member_6[3u];
                            if ((2f * asin(select(_e6629, 1f, _e6633))) >= _e6639) {
                                phi_11508_ = 0f;
                            } else {
                                let _e6642 = sin((_e6639 * 0.5f));
                                phi_11508_ = (1f / ((12.566371f * _e6642) * _e6642));
                            }
                            let _e6647 = phi_11508_;
                            if (_e331.member_1.w >= 300000000000000000000000000000000000000f) {
                                phi_31297_ = 1f;
                            } else {
                                let _e6649 = (_e331.member_1.w * _e331.member_1.w);
                                let _e6651 = (_e6649 + (_e6647 * _e6647));
                                if (_e6651 == 0f) {
                                    phi_31294_ = 0f;
                                } else {
                                    phi_31294_ = (_e6649 / _e6651);
                                }
                                let _e6655 = phi_31294_;
                                phi_31297_ = _e6655;
                            }
                            let _e6657 = phi_31297_;
                            phi_11510_ = _e6657;
                        }
                        let _e6659 = phi_11510_;
                        let _e6661 = global_1.member.member_5;
                        let _e6665 = (_e331.member_1.x - _e6661.x);
                        let _e6666 = (_e331.member_1.y - _e6661.y);
                        let _e6667 = (_e331.member_1.z - _e6661.z);
                        let _e6674 = (sqrt((((_e6665 * _e6665) + (_e6666 * _e6666)) + (_e6667 * _e6667))) * 0.5f);
                        if (_e6674 != _e6674) {
                            phi_31308_ = true;
                        } else {
                            phi_31308_ = (1f <= _e6674);
                        }
                        let _e6678 = phi_31308_;
                        let _e6681 = (2f * asin(select(_e6674, 1f, _e6678)));
                        let _e6685 = global_1.member.member_6[3u];
                        if (_e6681 >= _e6685) {
                            phi_12307_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e6689 = (1f - pow((_e6681 / _e6685), 2f));
                            if (_e6689 != _e6689) {
                                phi_31323_ = true;
                            } else {
                                phi_31323_ = (0f >= _e6689);
                            }
                            let _e6693 = phi_31323_;
                            let _e6696 = global_1.member.member_6;
                            let _e6702 = global_1.member.member_8[2u];
                            if (_e6702 == 4294967295u) {
                                phi_12280_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e6705 = global_6.member[_e6702];
                                let _e6708 = global_6.member[(_e6702 + 1u)];
                                let _e6711 = global_6.member[(_e6702 + 2u)];
                                let _e6714 = global_6.member[(_e6702 + 3u)];
                                let _e6717 = global_6.member[(_e6702 + 4u)];
                                let _e6720 = global_6.member[(_e6702 + 5u)];
                                let _e6723 = global_6.member[(_e6702 + 6u)];
                                let _e6726 = global_6.member[(_e6702 + 7u)];
                                let _e6729 = global_6.member[(_e6702 + 8u)];
                                let _e6732 = global_6.member[(_e6702 + 9u)];
                                let _e6735 = global_6.member[(_e6702 + 13u)];
                                let _e6738 = global_6.member[(_e6702 + 14u)];
                                let _e6741 = global_6.member[(_e6702 + 15u)];
                                let _e6744 = global_6.member[(_e6702 + 16u)];
                                let _e6747 = global_6.member[(_e6702 + 18u)];
                                let _e6750 = global_6.member[(_e6702 + 19u)];
                                let _e6753 = global_6.member[(_e6702 + 20u)];
                                let _e6756 = global_6.member[(_e6702 + 29u)];
                                let _e6757 = bitcast<u32>(_e6756);
                                let _e6770 = (((((_e331.member.x * _e331.member.x) + (_e331.member.y * _e331.member.y)) + (_e331.member.z * _e331.member.z)) - (2f * (((_e331.member.x * _e6705) + (_e331.member.y * _e6708)) + (_e331.member.z * _e6711)))) + _e6723);
                                if (_e6770 > 0f) {
                                    let _e6779 = ((((_e331.member.x - _e6705) * _e6726) + ((_e331.member.y - _e6708) * _e6729)) + ((_e331.member.z - _e6711) * _e6732));
                                    let _e6781 = ((_e6779 * _e6779) - _e6770);
                                    if (_e6779 >= 0f) {
                                        phi_11980_ = vec3<f32>();
                                        phi_11981_ = true;
                                    } else {
                                        let _e6783 = (_e6781 <= 0f);
                                        if _e6783 {
                                            phi_11979_ = vec3<f32>();
                                        } else {
                                            let _e6786 = (-(_e6779) - sqrt(_e6781));
                                            phi_11979_ = vec3<f32>((_e331.member.x + (_e6726 * _e6786)), (_e331.member.y + (_e6729 * _e6786)), (_e331.member.z + (_e6732 * _e6786)));
                                        }
                                        let _e6795 = phi_11979_;
                                        phi_11980_ = _e6795;
                                        phi_11981_ = _e6783;
                                    }
                                    let _e6797 = phi_11980_;
                                    let _e6799 = phi_11981_;
                                    phi_11986_ = _e6797;
                                    phi_11987_ = select(true, false, _e6799);
                                } else {
                                    phi_11986_ = vec3<f32>(_e331.member.x, _e331.member.y, _e331.member.z);
                                    phi_11987_ = true;
                                }
                                let _e6802 = phi_11986_;
                                let _e6804 = phi_11987_;
                                if _e6804 {
                                    let _e6806 = (_e6802.x - _e6705);
                                    let _e6808 = (_e6802.y - _e6708);
                                    let _e6810 = (_e6802.z - _e6711);
                                    let _e6816 = sqrt((((_e6806 * _e6806) + (_e6808 * _e6808)) + (_e6810 * _e6810)));
                                    let _e6817 = (1f / _e6816);
                                    let _e6835 = ((((((_e6802.x * _e6802.x) + (_e6802.y * _e6802.y)) + (_e6802.z * _e6802.z)) - (2f * (((_e6802.x * _e6705) + (_e6802.y * _e6708)) + (_e6802.z * _e6711)))) + _e6720) / (_e6816 + _e6714));
                                    if (_e6835 != _e6835) {
                                        phi_31447_ = true;
                                    } else {
                                        phi_31447_ = (0f >= _e6835);
                                    }
                                    let _e6839 = phi_31447_;
                                    let _e6843 = sqrt((select(_e6835, 0f, _e6839) / (_e6717 - _e6714)));
                                    if (_e6843 != _e6843) {
                                        phi_31462_ = true;
                                    } else {
                                        phi_31462_ = (1f <= _e6843);
                                    }
                                    let _e6847 = phi_31462_;
                                    let _e6849 = (select(_e6843, 1f, _e6847) * 63f);
                                    let _e6854 = ((((_e6806 * _e6817) * _e6726) + ((_e6808 * _e6817) * _e6729)) + ((_e6810 * _e6817) * _e6732));
                                    let _e6856 = select(_e6854, -1f, (_e6854 < -1f));
                                    let _e6860 = ((select(_e6856, 1f, (_e6856 > 1f)) + 1f) * 127.5f);
                                    let _e6865 = select(select(u32(_e6849), 0u, (_e6849 < 0f)), 4294967295u, (_e6849 > 4294967000f));
                                    let _e6867 = select(_e6865, 62u, (62u < _e6865));
                                    let _e6872 = select(select(u32(_e6860), 0u, (_e6860 < 0f)), 4294967295u, (_e6860 > 4294967000f));
                                    let _e6874 = select(_e6872, 254u, (254u < _e6872));
                                    let _e6876 = (_e6849 - f32(_e6867));
                                    let _e6878 = (_e6860 - f32(_e6874));
                                    let _e6882 = (_e6757 + (3u * ((_e6867 * 256u) + _e6874)));
                                    let _e6887 = (_e6757 + (3u * (((_e6867 + 1u) * 256u) + _e6874)));
                                    let _e6889 = global_6.member[_e6882];
                                    let _e6892 = global_6.member[(_e6882 + 1u)];
                                    let _e6895 = global_6.member[(_e6882 + 2u)];
                                    let _e6896 = (1f - _e6878);
                                    let _e6902 = global_6.member[(_e6882 + 3u)];
                                    let _e6905 = global_6.member[(_e6882 + 4u)];
                                    let _e6908 = global_6.member[(_e6882 + 5u)];
                                    let _e6915 = (1f - _e6876);
                                    let _e6920 = global_6.member[_e6887];
                                    let _e6923 = global_6.member[(_e6887 + 1u)];
                                    let _e6926 = global_6.member[(_e6887 + 2u)];
                                    let _e6932 = global_6.member[(_e6887 + 3u)];
                                    let _e6935 = global_6.member[(_e6887 + 4u)];
                                    let _e6938 = global_6.member[(_e6887 + 5u)];
                                    phi_12278_ = vec3<f32>(((((_e6889 * _e6896) + (_e6902 * _e6878)) * _e6915) + (((_e6920 * _e6896) + (_e6932 * _e6878)) * _e6876)), ((((_e6892 * _e6896) + (_e6905 * _e6878)) * _e6915) + (((_e6923 * _e6896) + (_e6935 * _e6878)) * _e6876)), ((((_e6895 * _e6896) + (_e6908 * _e6878)) * _e6915) + (((_e6926 * _e6896) + (_e6938 * _e6878)) * _e6876)));
                                } else {
                                    phi_12278_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e6953 = phi_12278_;
                                let _e6960 = ((_e6744 * 1.1f) * _e6953.y);
                                phi_12280_ = vec3<f32>(exp(-((((_e6735 * _e6953.x) + _e6960) + (_e6747 * _e6953.z)))), exp(-((((_e6738 * _e6953.x) + _e6960) + (_e6750 * _e6953.z)))), exp(-((((_e6741 * _e6953.x) + _e6960) + (_e6753 * _e6953.z)))));
                            }
                            let _e6979 = phi_12280_;
                            let _e6991 = sin((_e6685 * 0.5f));
                            let _e6994 = (((1f - (0.6f * (1f - sqrt(select(_e6689, 0f, _e6693))))) * 1.25f) / ((12.566371f * _e6991) * _e6991));
                            phi_12307_ = vec3<f32>(((_e6696.x * _e6979.x) * _e6994), ((_e6696.y * _e6979.y) * _e6994), ((_e6696.z * _e6979.z) * _e6994));
                        }
                        let _e7000 = phi_12307_;
                        phi_12321_ = vec3<f32>((_e331.member_3.x + ((_e331.member_2.x * _e7000.x) * _e6659)), (_e331.member_3.y + ((_e331.member_2.y * _e7000.y) * _e6659)), (_e331.member_3.z + ((_e331.member_2.z * _e7000.z) * _e6659)));
                    }
                    let _e7015 = phi_12321_;
                    phi_18781_ = type_70(type_35(_e331.member, _e331.member_1, vec4<f32>(_e331.member_2.x, _e331.member_2.y, _e331.member_2.z, f32(_e356)), vec4<f32>(_e7015.x, _e7015.y, _e7015.z, f32(_e382)), vec4<u32>(_e331.member_4.x, (_e331.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), type_42(vec4<f32>(_e331.member.x, _e331.member.y, _e331.member.z, -1f), vec4<f32>(_e331.member_1.x, _e331.member_1.y, _e331.member_1.z, 0f), vec4<f32>((_e331.member_2.x * _e6605), (_e331.member_2.y * _e6605), (_e331.member_2.z * _e6605), 0f)), type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_18782_ = type_70();
                    phi_18783_ = true;
                } else {
                    if ((_e376 & 8u) == 0u) {
                        phi_35019_ = false;
                        phi_12855_ = _e350;
                        phi_12856_ = type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
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
                                phi_25022_ = type_58(0u, type_58().member_1);
                                phi_25023_ = type_58();
                                phi_25024_ = true;
                            } else {
                                if (_e445 < 0f) {
                                    phi_24994_ = -(sqrt(_e460));
                                } else {
                                    phi_24994_ = sqrt(_e460);
                                }
                                let _e467 = phi_24994_;
                                let _e469 = -((_e445 + _e467));
                                let _e470 = (_e469 == 0f);
                                if _e470 {
                                    phi_25017_ = type_58(1u, type_57(0f, 0f));
                                    phi_25018_ = type_58();
                                } else {
                                    let _e471 = (_e458 / _e469);
                                    if (_e469 < _e471) {
                                        phi_25006_ = type_57(_e469, _e471);
                                    } else {
                                        phi_25006_ = type_57(_e471, _e469);
                                    }
                                    let _e476 = phi_25006_;
                                    phi_25017_ = type_58();
                                    phi_25018_ = type_58(1u, _e476);
                                }
                                let _e479 = phi_25017_;
                                let _e481 = phi_25018_;
                                phi_25022_ = _e479;
                                phi_25023_ = _e481;
                                phi_25024_ = _e470;
                            }
                            let _e486 = phi_25022_;
                            let _e488 = phi_25023_;
                            let _e490 = phi_25024_;
                            if _e490 {
                                phi_25028_ = _e486;
                            } else {
                                phi_25028_ = _e488;
                            }
                            let _e492 = phi_25028_;
                            if (bitcast<i32>(_e492.member) != 0i) {
                                phi_24915_ = type_58(0u, _e492.member_1);
                            } else {
                                phi_24915_ = type_58(1u, type_58().member_1);
                            }
                            let _e502 = phi_24915_;
                            if (bitcast<i32>(_e502.member) != 0i) {
                                phi_24962_ = type_58(0u, type_58().member_1);
                            } else {
                                if (_e502.member_1.member_1 <= 0f) {
                                    phi_24958_ = type_58(0u, type_58().member_1);
                                } else {
                                    if (_e502.member_1.member != _e502.member_1.member) {
                                        phi_25066_ = true;
                                    } else {
                                        phi_25066_ = (0f >= _e502.member_1.member);
                                    }
                                    let _e514 = phi_25066_;
                                    let _e515 = select(_e502.member_1.member, 0f, _e514);
                                    let _e516 = (_e457 + _e407);
                                    if (_e516 > 0f) {
                                        let _e518 = (_e459 - _e516);
                                        if (_e518 < 0f) {
                                            phi_25174_ = type_58(0u, type_58().member_1);
                                            phi_25175_ = type_58();
                                            phi_25176_ = true;
                                        } else {
                                            if (_e445 < 0f) {
                                                phi_25146_ = -(sqrt(_e518));
                                            } else {
                                                phi_25146_ = sqrt(_e518);
                                            }
                                            let _e525 = phi_25146_;
                                            let _e527 = -((_e445 + _e525));
                                            let _e528 = (_e527 == 0f);
                                            if _e528 {
                                                phi_25169_ = type_58(1u, type_57(0f, 0f));
                                                phi_25170_ = type_58();
                                            } else {
                                                let _e529 = (_e516 / _e527);
                                                if (_e527 < _e529) {
                                                    phi_25158_ = type_57(_e527, _e529);
                                                } else {
                                                    phi_25158_ = type_57(_e529, _e527);
                                                }
                                                let _e534 = phi_25158_;
                                                phi_25169_ = type_58();
                                                phi_25170_ = type_58(1u, _e534);
                                            }
                                            let _e537 = phi_25169_;
                                            let _e539 = phi_25170_;
                                            phi_25174_ = _e537;
                                            phi_25175_ = _e539;
                                            phi_25176_ = _e528;
                                        }
                                        let _e544 = phi_25174_;
                                        let _e546 = phi_25175_;
                                        let _e548 = phi_25176_;
                                        if _e548 {
                                            phi_25180_ = _e544;
                                        } else {
                                            phi_25180_ = _e546;
                                        }
                                        let _e550 = phi_25180_;
                                        if (bitcast<i32>(_e550.member) != 0i) {
                                            phi_25114_ = select(_e550.member_1.member_1, _e550.member_1.member, (_e550.member_1.member >= 0f));
                                        } else {
                                            phi_25114_ = -1f;
                                        }
                                        let _e561 = phi_25114_;
                                        if (_e561 >= 0f) {
                                            if (_e502.member_1.member_1 != _e502.member_1.member_1) {
                                                phi_25218_ = true;
                                            } else {
                                                phi_25218_ = (_e561 <= _e502.member_1.member_1);
                                            }
                                            let _e566 = phi_25218_;
                                            phi_24943_ = select(_e502.member_1.member_1, _e561, _e566);
                                        } else {
                                            phi_24943_ = _e502.member_1.member_1;
                                        }
                                        let _e569 = phi_24943_;
                                        phi_24945_ = _e569;
                                    } else {
                                        phi_24945_ = _e502.member_1.member_1;
                                    }
                                    let _e571 = phi_24945_;
                                    if (_e571 <= _e515) {
                                        phi_24954_ = type_58(0u, type_58().member_1);
                                    } else {
                                        phi_24954_ = type_58(1u, type_57(_e515, _e571));
                                    }
                                    let _e579 = phi_24954_;
                                    phi_24958_ = _e579;
                                }
                                let _e584 = phi_24958_;
                                phi_24962_ = _e584;
                            }
                            let _e589 = phi_24962_;
                            if (bitcast<i32>(_e589.member) != 0i) {
                                let _e595 = (_e589.member_1.member < _e331.member_5.x);
                                if _e595 {
                                    if (_e589.member_1.member_1 != _e589.member_1.member_1) {
                                        phi_25233_ = true;
                                    } else {
                                        phi_25233_ = (_e331.member_5.x <= _e589.member_1.member_1);
                                    }
                                    let _e601 = phi_25233_;
                                    let _e602 = select(_e589.member_1.member_1, _e331.member_5.x, _e601);
                                    phi_12714_ = vec3<f32>(0f, 0f, 0f);
                                    phi_12717_ = 0u;
                                    loop {
                                        let _e604 = phi_12714_;
                                        let _e606 = phi_12717_;
                                        local_69 = _e604;
                                        local_70 = _e604;
                                        local_71 = _e604;
                                        let _e607 = (_e606 < 32u);
                                        if _e607 {
                                            let _e608 = -(_e445);
                                            if (_e589.member_1.member <= _e602) {
                                            } else {
                                                phi_32883_ = true;
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
                                                phi_12818_ = type_57((_e589.member_1.member + ((_e616 * _e697) * _e697)), (_e616 * ((_e695 * _e695) - (_e692 * _e692))));
                                            } else {
                                                if (_e615 <= _e617) {
                                                    let _e675 = (f32((31u - _e606)) * 0.03125f);
                                                    let _e678 = (f32((32u - _e606)) * 0.03125f);
                                                    let _e680 = ((_e675 + _e678) * 0.5f);
                                                    phi_12817_ = type_57((_e602 - ((_e616 * _e680) * _e680)), (_e616 * ((_e678 * _e678) - (_e675 * _e675))));
                                                } else {
                                                    let _e623 = round(((32f * _e614) / (_e614 + _e615)));
                                                    let _e628 = select(select(u32(_e623), 0u, (_e623 < 0f)), 4294967295u, (_e623 > 4294967000f));
                                                    if (_e628 < 1u) {
                                                        phi_12801_ = 1u;
                                                    } else {
                                                        phi_12801_ = select(_e628, 31u, (_e628 > 31u));
                                                    }
                                                    let _e633 = phi_12801_;
                                                    if (_e606 < _e633) {
                                                        let _e654 = ((_e633 - 1u) - _e606);
                                                        let _e656 = f32(_e633);
                                                        let _e657 = (f32(_e654) / _e656);
                                                        let _e660 = (f32((_e654 + 1u)) / _e656);
                                                        let _e662 = ((_e657 + _e660) * 0.5f);
                                                        phi_12816_ = type_57((_e613 - ((_e614 * _e662) * _e662)), (_e614 * ((_e660 * _e660) - (_e657 * _e657))));
                                                    } else {
                                                        let _e636 = (_e606 - _e633);
                                                        let _e638 = f32((32u - _e633));
                                                        let _e639 = (f32(_e636) / _e638);
                                                        let _e642 = (f32((_e636 + 1u)) / _e638);
                                                        let _e644 = ((_e639 + _e642) * 0.5f);
                                                        phi_12816_ = type_57((_e613 + ((_e615 * _e644) * _e644)), (_e615 * ((_e642 * _e642) - (_e639 * _e639))));
                                                    }
                                                    let _e672 = phi_12816_;
                                                    phi_12817_ = _e672;
                                                }
                                                let _e690 = phi_12817_;
                                                phi_12818_ = _e690;
                                            }
                                            let _e707 = phi_12818_;
                                            let _e713 = (_e331.member.x + (_e331.member_1.x * _e707.member));
                                            let _e714 = (_e331.member.y + (_e331.member_1.y * _e707.member));
                                            let _e715 = (_e331.member.z + (_e331.member_1.z * _e707.member));
                                            let _e729 = (_e713 - _e395);
                                            let _e730 = (_e714 - _e398);
                                            let _e731 = (_e715 - _e401);
                                            let _e739 = ((((((_e713 * _e713) + (_e714 * _e714)) + (_e715 * _e715)) - (2f * (((_e713 * _e395) + (_e714 * _e398)) + (_e715 * _e401)))) + _e407) / (sqrt((((_e729 * _e729) + (_e730 * _e730)) + (_e731 * _e731))) + _e404));
                                            let _e743 = (1f - (abs((_e739 - 25000f)) * 0.00006666667f));
                                            if (_e743 != _e743) {
                                                phi_25432_ = true;
                                            } else {
                                                phi_25432_ = (0f >= _e743);
                                            }
                                            let _e747 = phi_25432_;
                                            let _e749 = -(_e739);
                                            phi_12715_ = (_e604 + vec3<f32>((exp((_e749 / _e413)) * _e707.member_1), (exp((_e749 / _e416)) * _e707.member_1), (select(_e743, 0f, _e747) * _e707.member_1)));
                                            phi_12718_ = (_e606 + 1u);
                                        } else {
                                            phi_12715_ = vec3<f32>();
                                            phi_12718_ = u32();
                                        }
                                        let _e761 = phi_12715_;
                                        let _e763 = phi_12718_;
                                        phi_12714_ = _e761;
                                        phi_12717_ = _e763;
                                        phi_32883_ = false;
                                        if !(_e607) {
                                            break;
                                        }
                                    }
                                    let _e766 = phi_32883_;
                                    if _e766 {
                                        break;
                                    }
                                    let _e768 = local_69;
                                    let _e775 = local_70;
                                    let _e777 = ((_e428 * 1.1f) * _e775.y);
                                    let _e782 = local_71;
                                    phi_35023_ = _e766;
                                    phi_12843_ = vec3<f32>(exp(-((((_e419 * _e768.x) + _e777) + (_e431 * _e782.z)))), exp(-((((_e422 * _e768.x) + _e777) + (_e434 * _e782.z)))), exp(-((((_e425 * _e768.x) + _e777) + (_e437 * _e782.z)))));
                                } else {
                                    phi_35023_ = false;
                                    phi_12843_ = vec3<f32>();
                                }
                                let _e798 = phi_35023_;
                                let _e800 = phi_12843_;
                                phi_35022_ = _e798;
                                phi_12845_ = _e800;
                                phi_12846_ = select(true, false, _e595);
                            } else {
                                phi_35022_ = false;
                                phi_12845_ = vec3<f32>();
                                phi_12846_ = true;
                            }
                            let _e803 = phi_35022_;
                            let _e805 = phi_12845_;
                            let _e807 = phi_12846_;
                            phi_35020_ = _e803;
                            phi_12853_ = (_e350 * select(_e805, vec3<f32>(1f, 1f, 1f), vec3(_e807)));
                            phi_12854_ = type_42(vec4<f32>(_e331.member.x, _e331.member.y, _e331.member.z, _e331.member_5.x), vec4<f32>(_e331.member_1.x, _e331.member_1.y, _e331.member_1.z, 0f), vec4<f32>(_e331.member_2.x, _e331.member_2.y, _e331.member_2.z, 0f));
                        } else {
                            phi_35020_ = false;
                            phi_12853_ = _e350;
                            phi_12854_ = type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e812 = phi_35020_;
                        let _e814 = phi_12853_;
                        let _e816 = phi_12854_;
                        phi_35019_ = _e812;
                        phi_12855_ = _e814;
                        phi_12856_ = _e816;
                    }
                    let _e818 = phi_35019_;
                    let _e820 = phi_12855_;
                    let _e822 = phi_12856_;
                    let _e826 = (_e331.member.x + (_e331.member_1.x * _e331.member_5.x));
                    let _e827 = (_e331.member.y + (_e331.member_1.y * _e331.member_5.x));
                    let _e828 = (_e331.member.z + (_e331.member_1.z * _e331.member_5.x));
                    let _e830 = global_2.member[_e331.member_4.z];
                    let _e833 = (_e830.member_6.x == 0u);
                    if _e833 {
                        let _e863 = (_e830.member_1.x - _e830.member.x);
                        let _e864 = (_e830.member_1.y - _e830.member.y);
                        let _e865 = (_e830.member_1.z - _e830.member.z);
                        let _e870 = (_e830.member_2.x - _e830.member.x);
                        let _e871 = (_e830.member_2.y - _e830.member.y);
                        let _e872 = (_e830.member_2.z - _e830.member.z);
                        let _e875 = ((_e864 * _e872) - (_e871 * _e865));
                        let _e878 = ((_e865 * _e870) - (_e872 * _e863));
                        let _e881 = ((_e863 * _e871) - (_e870 * _e864));
                        let _e888 = (1f / sqrt((((_e875 * _e875) + (_e878 * _e878)) + (_e881 * _e881))));
                        let _e892 = vec3<f32>((_e875 * _e888), (_e878 * _e888), (_e881 * _e888));
                        if (_e830.member_6.w == 1u) {
                            let _e903 = ((1f - _e331.member_5.y) - _e331.member_5.z);
                            let _e924 = (((_e830.member_3.x * _e903) + (_e830.member_4.x * _e331.member_5.y)) + (_e830.member_5.x * _e331.member_5.z));
                            let _e925 = (((_e830.member_3.y * _e903) + (_e830.member_4.y * _e331.member_5.y)) + (_e830.member_5.y * _e331.member_5.z));
                            let _e926 = (((_e830.member_3.z * _e903) + (_e830.member_4.z * _e331.member_5.y)) + (_e830.member_5.z * _e331.member_5.z));
                            let _e931 = (((_e924 * _e924) + (_e925 * _e925)) + (_e926 * _e926));
                            if (_e931 > 0f) {
                                let _e934 = (1f / sqrt(_e931));
                                phi_12983_ = vec3<f32>((_e924 * _e934), (_e925 * _e934), (_e926 * _e934));
                            } else {
                                phi_12983_ = _e892;
                            }
                            let _e940 = phi_12983_;
                            phi_12984_ = _e940;
                        } else {
                            phi_12984_ = _e892;
                        }
                        let _e942 = phi_12984_;
                        phi_13018_ = type_61(_e892, _e942);
                    } else {
                        if (_e830.member_6.x == 1u) {
                            let _e851 = vec3<f32>(((_e826 - _e830.member.x) / _e830.member.w), ((_e827 - _e830.member.y) / _e830.member.w), ((_e828 - _e830.member.z) / _e830.member.w));
                            phi_13017_ = type_61(_e851, _e851);
                        } else {
                            let _e837 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e331.member_1.y > 0f)));
                            phi_13017_ = type_61(_e837, _e837);
                        }
                        let _e854 = phi_13017_;
                        phi_13018_ = _e854;
                    }
                    let _e945 = phi_13018_;
                    phi_13035_ = 0i;
                    phi_13038_ = _e830.member_6.y;
                    loop {
                        let _e951 = phi_13035_;
                        let _e953 = phi_13038_;
                        local_72 = _e953;
                        if (_e951 < 4i) {
                            let _e956 = global_4.member[_e953];
                            let _e958 = (_e956.member != 6u);
                            if _e958 {
                                phi_13107_ = i32();
                                phi_13108_ = u32();
                            } else {
                                let _e962 = (abs(_e826) / _e956.member_6.z);
                                let _e973 = (abs(_e828) / _e956.member_6.w);
                                if ((select(select(u32(_e962), 0u, (_e962 < 0f)), 4294967295u, (_e962 > 4294967000f)) % 2u) == 0u) {
                                    phi_13092_ = (_e826 > 0f);
                                } else {
                                    phi_13092_ = (_e826 <= 0f);
                                }
                                let _e984 = phi_13092_;
                                if ((select(select(u32(_e973), 0u, (_e973 < 0f)), 4294967295u, (_e973 > 4294967000f)) % 2u) == 0u) {
                                    phi_13098_ = (_e828 > 0f);
                                } else {
                                    phi_13098_ = (_e828 <= 0f);
                                }
                                let _e988 = phi_13098_;
                                if (_e984 != _e988) {
                                    phi_13105_ = _e956.member_1;
                                } else {
                                    phi_13105_ = _e956.member_2;
                                }
                                let _e993 = phi_13105_;
                                phi_13107_ = (_e951 + 1i);
                                phi_13108_ = _e993;
                            }
                            let _e996 = phi_13107_;
                            let _e998 = phi_13108_;
                            phi_13036_ = _e996;
                            phi_13039_ = _e998;
                            phi_13110_ = select(true, false, _e958);
                        } else {
                            phi_13036_ = i32();
                            phi_13039_ = u32();
                            phi_13110_ = false;
                        }
                        let _e1001 = phi_13036_;
                        let _e1003 = phi_13039_;
                        let _e1005 = phi_13110_;
                        phi_13035_ = _e1001;
                        phi_13038_ = _e1003;
                        if !(_e1005) {
                            break;
                        }
                    }
                    let _e1008 = local_72;
                    let _e1010 = global_4.member[_e1008];
                    let _e1012 = (_e1010.member == 4u);
                    if _e1012 {
                        if _e1012 {
                            if (_e1010.member_1 == 1u) {
                                phi_13169_ = true;
                            } else {
                                phi_13169_ = ((((-(_e331.member_1.x) * _e945.member_1.x) + (-(_e331.member_1.y) * _e945.member_1.y)) + (-(_e331.member_1.z) * _e945.member_1.z)) > 0f);
                            }
                            let _e6325 = phi_13169_;
                            if _e6325 {
                                phi_13178_ = vec3<f32>(_e1010.member_4.x, _e1010.member_4.y, _e1010.member_4.z);
                            } else {
                                phi_13178_ = vec3<f32>();
                            }
                            let _e6332 = phi_13178_;
                            phi_13180_ = _e6332;
                            phi_13181_ = select(true, false, _e6325);
                        } else {
                            phi_13180_ = vec3<f32>();
                            phi_13181_ = true;
                        }
                        let _e6335 = phi_13180_;
                        let _e6337 = phi_13181_;
                        let _e6339 = select(_e6335, vec3<f32>(0f, 0f, 0f), vec3(_e6337));
                        if _e370 {
                            phi_13349_ = 1f;
                        } else {
                            if ((_e376 & 1u) == 0u) {
                                phi_13348_ = 1f;
                            } else {
                                if (_e830.member_6.z > 0u) {
                                    let _e6348 = global_1.member.member_2[1u];
                                    let _e6349 = (_e6348 + (_e830.member_6.z - 1u));
                                    let _e6352 = global_6.member[(_e6349 + 1u)];
                                    let _e6354 = global_6.member[_e6349];
                                    if (_e830.member_6.x == 1u) {
                                        let _e6427 = (_e830.member.x - _e331.member.x);
                                        let _e6428 = (_e830.member.y - _e331.member.y);
                                        let _e6429 = (_e830.member.z - _e331.member.z);
                                        let _e6434 = (((_e6427 * _e6427) + (_e6428 * _e6428)) + (_e6429 * _e6429));
                                        let _e6435 = (_e830.member.w * _e830.member.w);
                                        if (_e6434 <= (_e6435 * 1.000001f)) {
                                            let _e6449 = (_e826 - _e331.member.x);
                                            let _e6450 = (_e827 - _e331.member.y);
                                            let _e6451 = (_e828 - _e331.member.z);
                                            let _e6456 = (((_e6449 * _e6449) + (_e6450 * _e6450)) + (_e6451 * _e6451));
                                            phi_31160_ = (_e6456 / (((abs((((_e945.member.x * _e6449) + (_e945.member.y * _e6450)) + (_e945.member.z * _e6451))) / sqrt(_e6456)) * 12.566371f) * _e6435));
                                        } else {
                                            let _e6438 = (_e6435 / _e6434);
                                            if (_e6438 != _e6438) {
                                                phi_31179_ = true;
                                            } else {
                                                phi_31179_ = (1f <= _e6438);
                                            }
                                            let _e6442 = phi_31179_;
                                            let _e6443 = select(_e6438, 1f, _e6442);
                                            phi_31160_ = (0.15915494f / (_e6443 / (1f + sqrt((1f - _e6443)))));
                                        }
                                        let _e6472 = phi_31160_;
                                        phi_13344_ = _e6472;
                                    } else {
                                        let _e6357 = (_e826 - _e331.member.x);
                                        let _e6358 = (_e827 - _e331.member.y);
                                        let _e6359 = (_e828 - _e331.member.z);
                                        let _e6364 = (((_e6357 * _e6357) + (_e6358 * _e6358)) + (_e6359 * _e6359));
                                        let _e6375 = (abs((((_e945.member.x * _e6357) + (_e945.member.y * _e6358)) + (_e945.member.z * _e6359))) / sqrt(_e6364));
                                        if (_e6375 <= 0f) {
                                            phi_13343_ = 0f;
                                        } else {
                                            if _e833 {
                                                let _e6389 = (_e830.member_1.x - _e830.member.x);
                                                let _e6390 = (_e830.member_1.y - _e830.member.y);
                                                let _e6391 = (_e830.member_1.z - _e830.member.z);
                                                let _e6396 = (_e830.member_2.x - _e830.member.x);
                                                let _e6397 = (_e830.member_2.y - _e830.member.y);
                                                let _e6398 = (_e830.member_2.z - _e830.member.z);
                                                let _e6401 = ((_e6390 * _e6398) - (_e6397 * _e6391));
                                                let _e6404 = ((_e6391 * _e6396) - (_e6398 * _e6389));
                                                let _e6407 = ((_e6389 * _e6397) - (_e6396 * _e6390));
                                                phi_13340_ = (sqrt((((_e6401 * _e6401) + (_e6404 * _e6404)) + (_e6407 * _e6407))) * 0.5f);
                                            } else {
                                                phi_13340_ = ((12.566371f * _e830.member.w) * _e830.member.w);
                                            }
                                            let _e6416 = phi_13340_;
                                            phi_13343_ = (_e6364 / (_e6375 * _e6416));
                                        }
                                        let _e6420 = phi_13343_;
                                        phi_13344_ = _e6420;
                                    }
                                    let _e6474 = phi_13344_;
                                    let _e6475 = ((_e6352 - _e6354) * _e6474);
                                    if (_e331.member_1.w >= 300000000000000000000000000000000000000f) {
                                        phi_31200_ = 1f;
                                    } else {
                                        let _e6477 = (_e331.member_1.w * _e331.member_1.w);
                                        let _e6479 = (_e6477 + (_e6475 * _e6475));
                                        if (_e6479 == 0f) {
                                            phi_31197_ = 0f;
                                        } else {
                                            phi_31197_ = (_e6477 / _e6479);
                                        }
                                        let _e6483 = phi_31197_;
                                        phi_31200_ = _e6483;
                                    }
                                    let _e6485 = phi_31200_;
                                    phi_13347_ = _e6485;
                                } else {
                                    phi_13347_ = 1f;
                                }
                                let _e6487 = phi_13347_;
                                phi_13348_ = _e6487;
                            }
                            let _e6489 = phi_13348_;
                            phi_13349_ = _e6489;
                        }
                        let _e6491 = phi_13349_;
                        phi_18775_ = type_70(type_35(_e331.member, _e331.member_1, vec4<f32>(_e820.x, _e820.y, _e820.z, f32(_e356)), vec4<f32>((_e331.member_3.x + ((_e820.x * _e6339.x) * _e6491)), (_e331.member_3.y + ((_e820.y * _e6339.y) * _e6491)), (_e331.member_3.z + ((_e820.z * _e6339.z) * _e6491)), f32(_e382)), vec4<u32>(_e331.member_4.x, (_e331.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_18776_ = type_70();
                        phi_18777_ = true;
                    } else {
                        let _e1013 = -(_e331.member_1.x);
                        let _e1014 = -(_e331.member_1.y);
                        let _e1015 = -(_e331.member_1.z);
                        if ((((_e945.member.x * _e1013) + (_e945.member.y * _e1014)) + (_e945.member.z * _e1015)) < 0f) {
                            phi_13385_ = -(_e945.member);
                        } else {
                            phi_13385_ = _e945.member;
                        }
                        let _e1027 = phi_13385_;
                        if ((((_e945.member_1.x * _e1013) + (_e945.member_1.y * _e1014)) + (_e945.member_1.z * _e1015)) < 0f) {
                            phi_13399_ = -(_e945.member_1);
                        } else {
                            phi_13399_ = _e945.member_1;
                        }
                        let _e1039 = phi_13399_;
                        let _e1042 = global_1.member.member_4[2u];
                        let _e1043 = abs(_e826);
                        let _e1044 = abs(_e827);
                        let _e1045 = abs(_e828);
                        let _e1047 = select(_e1044, _e1043, (_e1043 > _e1044));
                        let _e1051 = (0.00002f * (1f + select(_e1045, _e1047, (_e1047 > _e1045))));
                        if (_e1042 != _e1042) {
                            phi_25499_ = true;
                        } else {
                            phi_25499_ = (_e1051 >= _e1042);
                        }
                        let _e1055 = phi_25499_;
                        let _e1057 = (select(_e1042, _e1051, _e1055) * 2f);
                        let _e1064 = (_e826 + (_e1027.x * _e1057));
                        let _e1065 = (_e827 + (_e1027.y * _e1057));
                        let _e1066 = (_e828 + (_e1027.z * _e1057));
                        let _e1067 = vec3<f32>(_e1064, _e1065, _e1066);
                        if ((_e331.member_4.y & 4u) == 0u) {
                            phi_13432_ = 0f;
                        } else {
                            let _e1072 = global_1.member.member_4[0u];
                            phi_13432_ = _e1072;
                        }
                        let _e1074 = phi_13432_;
                        let _e1077 = select(-1f, 1f, (_e1039.z >= 0f));
                        let _e1079 = (-1f / (_e1077 + _e1039.z));
                        let _e1083 = ((_e1039.x * _e1039.y) * _e1079);
                        let _e1087 = (1f + (((_e1077 * _e1039.x) * _e1039.x) * _e1079));
                        let _e1088 = (_e1077 * _e1083);
                        let _e1090 = (-(_e1077) * _e1039.x);
                        let _e1093 = (_e1077 + ((_e1039.y * _e1039.y) * _e1079));
                        let _e1094 = -(_e1039.y);
                        if (_e1010.member == 0u) {
                            phi_33667_ = vec3<f32>(0f, 0f, 0f);
                            phi_33475_ = 1.5f;
                            phi_33396_ = vec3<f32>(0f, 0f, 0f);
                            phi_33319_ = vec3<f32>(_e1010.member_4.x, _e1010.member_4.y, _e1010.member_4.z);
                            phi_33240_ = 0u;
                            phi_33158_ = 1f;
                            phi_33081_ = 0u;
                            phi_33072_ = _e331.member_4.x;
                            phi_14110_ = type_70();
                            phi_14111_ = true;
                        } else {
                            if (_e1010.member == 1u) {
                                let _e1643 = (_e1010.member_6.y * _e1010.member_6.y);
                                if (_e1643 != _e1643) {
                                    phi_26191_ = true;
                                } else {
                                    phi_26191_ = (_e1074 >= _e1643);
                                }
                                let _e1647 = phi_26191_;
                                let _e1648 = select(_e1643, _e1074, _e1647);
                                let _e1649 = (_e1648 >= 0.001f);
                                if _e1649 {
                                    phi_33724_ = vec3<f32>(_e1010.member_4.x, _e1010.member_4.y, _e1010.member_4.z);
                                    phi_33518_ = _e1010.member_6.x;
                                    phi_33371_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33059_ = _e331.member_4.x;
                                    phi_13507_ = vec3<f32>();
                                    phi_13508_ = true;
                                } else {
                                    let _e1651 = ((_e331.member_4.x * 747796405u) + 2891336453u);
                                    let _e1659 = (((_e1651 >> bitcast<u32>((((_e1651 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1651) * 277803737u);
                                    let _e1662 = ((_e1659 >> bitcast<u32>(22i)) ^ _e1659);
                                    let _e1672 = abs((((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z)));
                                    let _e1675 = (1f / _e1010.member_6.x);
                                    let _e1679 = ((_e1675 * _e1675) * (1f - (_e1672 * _e1672)));
                                    if (_e1679 >= 1f) {
                                        phi_26231_ = 1f;
                                    } else {
                                        let _e1682 = sqrt((1f - _e1679));
                                        if (_e1672 != _e1672) {
                                            phi_26242_ = true;
                                        } else {
                                            phi_26242_ = (_e1682 <= _e1672);
                                        }
                                        let _e1686 = phi_26242_;
                                        let _e1691 = pow(((1f - _e1010.member_6.x) / (1f + _e1010.member_6.x)), 2f);
                                        phi_26231_ = (_e1691 + ((1f - _e1691) * pow((1f - select(_e1672, _e1682, _e1686)), 5f)));
                                    }
                                    let _e1698 = phi_26231_;
                                    let _e1699 = ((f32((_e1662 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1698);
                                    if _e1699 {
                                        let _e1712 = (((_e1039.x * _e331.member_1.x) + (_e1039.y * _e331.member_1.y)) + (_e1039.z * _e331.member_1.z));
                                        phi_33372_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13505_ = (_e345 - vec3<f32>(((_e1039.x * 2f) * _e1712), ((_e1039.y * 2f) * _e1712), ((_e1039.z * 2f) * _e1712)));
                                    } else {
                                        phi_33372_ = vec3<f32>(_e1010.member_4.x, _e1010.member_4.y, _e1010.member_4.z);
                                        phi_13505_ = vec3<f32>();
                                    }
                                    let _e1719 = phi_33372_;
                                    let _e1721 = phi_13505_;
                                    phi_33724_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33518_ = 1.5f;
                                    phi_33371_ = _e1719;
                                    phi_33059_ = _e1662;
                                    phi_13507_ = _e1721;
                                    phi_13508_ = select(true, false, _e1699);
                                }
                                let _e1731 = phi_33724_;
                                let _e1733 = phi_33518_;
                                let _e1735 = phi_33371_;
                                let _e1737 = phi_33059_;
                                let _e1739 = phi_13507_;
                                let _e1741 = phi_13508_;
                                phi_33670_ = _e1731;
                                phi_33478_ = _e1733;
                                phi_33399_ = vec3<f32>(0f, 0f, 0f);
                                phi_33322_ = _e1735;
                                phi_33243_ = select(0u, 1u, _e1649);
                                phi_33161_ = select(1f, _e1648, _e1649);
                                phi_33084_ = select(0u, 2u, _e1649);
                                phi_33010_ = _e1737;
                                phi_13980_ = _e1067;
                                phi_13981_ = _e1739;
                                phi_13982_ = vec3<f32>(1f, 1f, 1f);
                                phi_13983_ = _e1741;
                            } else {
                                if (_e1010.member == 3u) {
                                    phi_33013_ = _e331.member_4.x;
                                    phi_13659_ = vec3<f32>();
                                    phi_13660_ = vec3<f32>();
                                    phi_13661_ = vec3<f32>();
                                    phi_13662_ = true;
                                    phi_13663_ = false;
                                } else {
                                    let _e1098 = (_e1010.member == 5u);
                                    if _e1098 {
                                        phi_33014_ = _e331.member_4.x;
                                        phi_13655_ = vec3<f32>();
                                        phi_13656_ = vec3<f32>();
                                        phi_13657_ = vec3<f32>();
                                        phi_13658_ = false;
                                    } else {
                                        let _e1099 = (_e1010.member == 2u);
                                        if _e1099 {
                                            let _e1104 = (((_e331.member_1.x * _e945.member_1.x) + (_e331.member_1.y * _e945.member_1.y)) + (_e331.member_1.z * _e945.member_1.z));
                                            let _e1105 = (_e1104 > 0f);
                                            if _e1105 {
                                                phi_13540_ = -(_e945.member_1);
                                            } else {
                                                phi_13540_ = _e945.member_1;
                                            }
                                            let _e1108 = phi_13540_;
                                            if _e1105 {
                                                phi_13546_ = _e1010.member_6.x;
                                            } else {
                                                phi_13546_ = (1f / _e1010.member_6.x);
                                            }
                                            let _e1113 = phi_13546_;
                                            let _e1114 = abs(_e1104);
                                            let _e1115 = (_e1113 * _e1113);
                                            let _e1118 = (_e1115 * (1f - (_e1114 * _e1114)));
                                            if (_e1118 >= 1f) {
                                                phi_25561_ = 1f;
                                            } else {
                                                let _e1121 = sqrt((1f - _e1118));
                                                if (_e1114 != _e1114) {
                                                    phi_25572_ = true;
                                                } else {
                                                    phi_25572_ = (_e1121 <= _e1114);
                                                }
                                                let _e1125 = phi_25572_;
                                                let _e1130 = pow(((1f - _e1010.member_6.x) / (1f + _e1010.member_6.x)), 2f);
                                                phi_25561_ = (_e1130 + ((1f - _e1130) * pow((1f - select(_e1114, _e1121, _e1125)), 5f)));
                                            }
                                            let _e1137 = phi_25561_;
                                            let _e1145 = (((_e331.member_1.x * _e1108.x) + (_e331.member_1.y * _e1108.y)) + (_e331.member_1.z * _e1108.z));
                                            let _e1149 = (1f - (_e1115 * (1f - (_e1145 * _e1145))));
                                            if (_e1149 > 0f) {
                                                let _e1152 = ((_e331.member_4.x * 747796405u) + 2891336453u);
                                                let _e1160 = (((_e1152 >> bitcast<u32>((((_e1152 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1152) * 277803737u);
                                                let _e1163 = ((_e1160 >> bitcast<u32>(22i)) ^ _e1160);
                                                let _e1168 = ((f32((_e1163 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1137);
                                                if _e1168 {
                                                    let _e1178 = sqrt(_e1149);
                                                    let _e1182 = (((_e331.member_1.x - (_e1108.x * _e1145)) * _e1113) - (_e1108.x * _e1178));
                                                    let _e1183 = (((_e331.member_1.y - (_e1108.y * _e1145)) * _e1113) - (_e1108.y * _e1178));
                                                    let _e1184 = (((_e331.member_1.z - (_e1108.z * _e1145)) * _e1113) - (_e1108.z * _e1178));
                                                    let _e1191 = (1f / sqrt((((_e1182 * _e1182) + (_e1183 * _e1183)) + (_e1184 * _e1184))));
                                                    phi_13627_ = vec3<f32>((_e826 - (_e1108.x * _e1057)), (_e827 - (_e1108.y * _e1057)), (_e828 - (_e1108.z * _e1057)));
                                                    phi_13628_ = vec3<f32>((_e1182 * _e1191), (_e1183 * _e1191), (_e1184 * _e1191));
                                                    phi_13629_ = vec3<f32>(_e1010.member_4.x, _e1010.member_4.y, _e1010.member_4.z);
                                                } else {
                                                    phi_13627_ = vec3<f32>();
                                                    phi_13628_ = vec3<f32>();
                                                    phi_13629_ = vec3<f32>();
                                                }
                                                let _e1209 = phi_13627_;
                                                let _e1211 = phi_13628_;
                                                let _e1213 = phi_13629_;
                                                phi_33017_ = _e1163;
                                                phi_13631_ = _e1209;
                                                phi_13632_ = _e1211;
                                                phi_13633_ = _e1213;
                                                phi_13634_ = select(true, false, _e1168);
                                            } else {
                                                phi_33017_ = _e331.member_4.x;
                                                phi_13631_ = vec3<f32>();
                                                phi_13632_ = vec3<f32>();
                                                phi_13633_ = vec3<f32>();
                                                phi_13634_ = true;
                                            }
                                            let _e1216 = phi_33017_;
                                            let _e1218 = phi_13631_;
                                            let _e1220 = phi_13632_;
                                            let _e1222 = phi_13633_;
                                            let _e1224 = phi_13634_;
                                            if _e1224 {
                                                let _e1232 = (((_e945.member_1.x * _e331.member_1.x) + (_e945.member_1.y * _e331.member_1.y)) + (_e945.member_1.z * _e331.member_1.z));
                                                phi_13649_ = vec3<f32>((_e826 + (_e1108.x * _e1057)), (_e827 + (_e1108.y * _e1057)), (_e828 + (_e1108.z * _e1057)));
                                                phi_13650_ = (_e345 - vec3<f32>(((_e945.member_1.x * 2f) * _e1232), ((_e945.member_1.y * 2f) * _e1232), ((_e945.member_1.z * 2f) * _e1232)));
                                            } else {
                                                phi_13649_ = _e1218;
                                                phi_13650_ = _e1220;
                                            }
                                            let _e1246 = phi_13649_;
                                            let _e1248 = phi_13650_;
                                            phi_33015_ = _e1216;
                                            phi_13652_ = _e1246;
                                            phi_13653_ = _e1248;
                                            phi_13654_ = select(_e1222, vec3<f32>(1f, 1f, 1f), vec3(_e1224));
                                        } else {
                                            phi_33015_ = _e331.member_4.x;
                                            phi_13652_ = vec3<f32>();
                                            phi_13653_ = vec3<f32>();
                                            phi_13654_ = vec3<f32>();
                                        }
                                        let _e1252 = phi_33015_;
                                        let _e1254 = phi_13652_;
                                        let _e1256 = phi_13653_;
                                        let _e1258 = phi_13654_;
                                        phi_33014_ = _e1252;
                                        phi_13655_ = _e1254;
                                        phi_13656_ = _e1256;
                                        phi_13657_ = _e1258;
                                        phi_13658_ = _e1099;
                                    }
                                    let _e1260 = phi_33014_;
                                    let _e1262 = phi_13655_;
                                    let _e1264 = phi_13656_;
                                    let _e1266 = phi_13657_;
                                    let _e1268 = phi_13658_;
                                    phi_33013_ = _e1260;
                                    phi_13659_ = _e1262;
                                    phi_13660_ = _e1264;
                                    phi_13661_ = _e1266;
                                    phi_13662_ = _e1098;
                                    phi_13663_ = _e1268;
                                }
                                let _e1270 = phi_33013_;
                                let _e1272 = phi_13659_;
                                let _e1274 = phi_13660_;
                                let _e1276 = phi_13661_;
                                let _e1278 = phi_13662_;
                                let _e1280 = phi_13663_;
                                if _e1278 {
                                    let _e1283 = (_e1010.member_6.y * _e1010.member_6.y);
                                    if (_e1283 != _e1283) {
                                        phi_25645_ = true;
                                    } else {
                                        phi_25645_ = (_e1074 >= _e1283);
                                    }
                                    let _e1287 = phi_25645_;
                                    let _e1288 = select(_e1283, _e1074, _e1287);
                                    let _e1300 = select(2u, 0u, (_e1010.member == 5u));
                                    let _e1301 = (_e1288 < 0.001f);
                                    if _e1301 {
                                        let _e1309 = (((_e1039.x * _e331.member_1.x) + (_e1039.y * _e331.member_1.y)) + (_e1039.z * _e331.member_1.z));
                                        let _e1320 = abs((((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z)));
                                        if (_e1300 == 0u) {
                                            let _e1602 = (1f - _e1320);
                                            if (_e1602 != _e1602) {
                                                phi_26176_ = true;
                                            } else {
                                                phi_26176_ = (0f >= _e1602);
                                            }
                                            let _e1606 = phi_26176_;
                                            let _e1608 = pow(select(_e1602, 0f, _e1606), 5f);
                                            let _e1609 = (1f - _e1608);
                                            phi_13966_ = vec3<f32>(((_e1010.member_4.x * _e1609) + _e1608), ((_e1010.member_4.y * _e1609) + _e1608), ((_e1010.member_4.z * _e1609) + _e1608));
                                        } else {
                                            if (_e1300 == 1u) {
                                                let _e1584 = (0.44444448f * (1f - (_e1320 * _e1320)));
                                                if (_e1584 >= 1f) {
                                                    phi_26137_ = 1f;
                                                } else {
                                                    let _e1587 = sqrt((1f - _e1584));
                                                    if (_e1320 != _e1320) {
                                                        phi_26148_ = true;
                                                    } else {
                                                        phi_26148_ = (_e1587 <= _e1320);
                                                    }
                                                    let _e1591 = phi_26148_;
                                                    phi_26137_ = (0.040000003f + (0.96f * pow((1f - select(_e1320, _e1587, _e1591)), 5f)));
                                                }
                                                let _e1598 = phi_26137_;
                                                phi_13965_ = vec3(_e1598);
                                            } else {
                                                let _e1324 = select(_e1320, 0f, (_e1320 < 0f));
                                                let _e1326 = select(_e1324, 1f, (_e1324 > 1f));
                                                let _e1328 = (1f - (_e1326 * _e1326));
                                                let _e1335 = ((_e1010.member_4.x * _e1010.member_4.x) - (_e1010.member_5.x * _e1010.member_5.x));
                                                let _e1336 = ((_e1010.member_4.y * _e1010.member_4.y) - (_e1010.member_5.y * _e1010.member_5.y));
                                                let _e1337 = ((_e1010.member_4.z * _e1010.member_4.z) - (_e1010.member_5.z * _e1010.member_5.z));
                                                let _e1339 = (_e1010.member_4.x * (_e1010.member_5.x + _e1010.member_5.x));
                                                let _e1341 = (_e1010.member_4.y * (_e1010.member_5.y + _e1010.member_5.y));
                                                let _e1343 = (_e1010.member_4.z * (_e1010.member_5.z + _e1010.member_5.z));
                                                let _e1350 = ((_e1335 * _e1335) + (_e1339 * _e1339));
                                                let _e1351 = ((_e1336 * _e1336) + (_e1341 * _e1341));
                                                let _e1352 = ((_e1337 * _e1337) + (_e1343 * _e1343));
                                                let _e1357 = ((_e1328 * _e1335) / _e1350);
                                                let _e1358 = ((_e1328 * _e1336) / _e1351);
                                                let _e1359 = ((_e1328 * _e1337) / _e1352);
                                                let _e1360 = (1f - _e1357);
                                                let _e1361 = (1f - _e1358);
                                                let _e1362 = (1f - _e1359);
                                                let _e1364 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1357, _e1358, _e1359));
                                                let _e1365 = (_e1328 * _e1339);
                                                let _e1366 = (_e1328 * _e1341);
                                                let _e1367 = (_e1328 * _e1343);
                                                let _e1372 = (vec3<f32>(_e1365, _e1366, _e1367) / vec3<f32>(_e1350, _e1351, _e1352));
                                                let _e1385 = ((_e1364.x * _e1364.x) + (_e1372.x * _e1372.x));
                                                let _e1386 = ((_e1364.y * _e1364.y) + (_e1372.y * _e1372.y));
                                                let _e1387 = ((_e1364.z * _e1364.z) + (_e1372.z * _e1372.z));
                                                if (_e1385 != _e1385) {
                                                    phi_25807_ = true;
                                                } else {
                                                    phi_25807_ = (0f >= _e1385);
                                                }
                                                let _e1391 = phi_25807_;
                                                let _e1393 = sqrt(select(_e1385, 0f, _e1391));
                                                if (_e1386 != _e1386) {
                                                    phi_25822_ = true;
                                                } else {
                                                    phi_25822_ = (0f >= _e1386);
                                                }
                                                let _e1397 = phi_25822_;
                                                let _e1399 = sqrt(select(_e1386, 0f, _e1397));
                                                if (_e1387 != _e1387) {
                                                    phi_25837_ = true;
                                                } else {
                                                    phi_25837_ = (0f >= _e1387);
                                                }
                                                let _e1403 = phi_25837_;
                                                let _e1405 = sqrt(select(_e1387, 0f, _e1403));
                                                let _e1409 = ((_e1393 + _e1360) * 0.5f);
                                                let _e1410 = ((_e1399 + _e1361) * 0.5f);
                                                let _e1411 = ((_e1405 + _e1362) * 0.5f);
                                                if (_e1409 != _e1409) {
                                                    phi_25864_ = true;
                                                } else {
                                                    phi_25864_ = (0f >= _e1409);
                                                }
                                                let _e1415 = phi_25864_;
                                                let _e1417 = sqrt(select(_e1409, 0f, _e1415));
                                                if (_e1410 != _e1410) {
                                                    phi_25879_ = true;
                                                } else {
                                                    phi_25879_ = (0f >= _e1410);
                                                }
                                                let _e1421 = phi_25879_;
                                                let _e1423 = sqrt(select(_e1410, 0f, _e1421));
                                                if (_e1411 != _e1411) {
                                                    phi_25894_ = true;
                                                } else {
                                                    phi_25894_ = (0f >= _e1411);
                                                }
                                                let _e1427 = phi_25894_;
                                                let _e1429 = sqrt(select(_e1411, 0f, _e1427));
                                                let _e1430 = vec3<f32>(_e1417, _e1423, _e1429);
                                                let _e1434 = ((_e1393 - _e1360) * 0.5f);
                                                let _e1435 = ((_e1399 - _e1361) * 0.5f);
                                                let _e1436 = ((_e1405 - _e1362) * 0.5f);
                                                if (_e1434 != _e1434) {
                                                    phi_25921_ = true;
                                                } else {
                                                    phi_25921_ = (0f >= _e1434);
                                                }
                                                let _e1440 = phi_25921_;
                                                let _e1442 = sqrt(select(_e1434, 0f, _e1440));
                                                if (_e1435 != _e1435) {
                                                    phi_25936_ = true;
                                                } else {
                                                    phi_25936_ = (0f >= _e1435);
                                                }
                                                let _e1446 = phi_25936_;
                                                let _e1448 = sqrt(select(_e1435, 0f, _e1446));
                                                if (_e1436 != _e1436) {
                                                    phi_25951_ = true;
                                                } else {
                                                    phi_25951_ = (0f >= _e1436);
                                                }
                                                let _e1452 = phi_25951_;
                                                let _e1454 = sqrt(select(_e1436, 0f, _e1452));
                                                if ((_e1365 / _e1350) < 0f) {
                                                    phi_13842_ = -(_e1442);
                                                } else {
                                                    phi_13842_ = _e1442;
                                                }
                                                let _e1458 = phi_13842_;
                                                if ((_e1366 / _e1351) < 0f) {
                                                    phi_13850_ = -(_e1448);
                                                } else {
                                                    phi_13850_ = _e1448;
                                                }
                                                let _e1462 = phi_13850_;
                                                if ((_e1367 / _e1352) < 0f) {
                                                    phi_13858_ = -(_e1454);
                                                } else {
                                                    phi_13858_ = _e1454;
                                                }
                                                let _e1466 = phi_13858_;
                                                let _e1470 = (_e1010.member_5.x * _e1326);
                                                let _e1471 = (_e1010.member_5.y * _e1326);
                                                let _e1472 = (_e1010.member_5.z * _e1326);
                                                let _e1482 = vec3<f32>(((_e1010.member_4.x * _e1417) - (_e1010.member_5.x * _e1458)), ((_e1010.member_4.y * _e1423) - (_e1010.member_5.y * _e1462)), ((_e1010.member_4.z * _e1429) - (_e1010.member_5.z * _e1466)));
                                                let _e1489 = ((_e1010.member_4.x * _e1458) + (_e1010.member_5.x * _e1417));
                                                let _e1490 = ((_e1010.member_4.y * _e1462) + (_e1010.member_5.y * _e1423));
                                                let _e1491 = ((_e1010.member_4.z * _e1466) + (_e1010.member_5.z * _e1429));
                                                let _e1493 = vec3<f32>((_e1010.member_4.x * _e1326), (_e1010.member_4.y * _e1326), (_e1010.member_4.z * _e1326));
                                                let _e1494 = (_e1493 - _e1430);
                                                let _e1495 = (_e1470 - _e1458);
                                                let _e1496 = (_e1471 - _e1462);
                                                let _e1497 = (_e1472 - _e1466);
                                                let _e1510 = (_e1493 + _e1430);
                                                let _e1511 = (_e1470 + _e1458);
                                                let _e1512 = (_e1471 + _e1462);
                                                let _e1513 = (_e1472 + _e1466);
                                                let _e1523 = ((_e1510.x * _e1510.x) + (_e1511 * _e1511));
                                                let _e1524 = ((_e1510.y * _e1510.y) + (_e1512 * _e1512));
                                                let _e1525 = ((_e1510.z * _e1510.z) + (_e1513 * _e1513));
                                                let _e1535 = vec3(_e1326);
                                                let _e1536 = (_e1535 - _e1482);
                                                let _e1537 = -(vec3<f32>(_e1489, _e1490, _e1491));
                                                let _e1553 = (_e1535 + _e1482);
                                                let _e1563 = ((_e1553.x * _e1553.x) + (_e1489 * _e1489));
                                                let _e1564 = ((_e1553.y * _e1553.y) + (_e1490 * _e1490));
                                                let _e1565 = ((_e1553.z * _e1553.z) + (_e1491 * _e1491));
                                                phi_13965_ = vec3<f32>((((((_e1494.x * _e1494.x) + (_e1495 * _e1495)) / select(0.000000000000000000000000000001f, _e1523, (_e1523 > 0.000000000000000000000000000001f))) + (((_e1536.x * _e1536.x) + (_e1537.x * _e1537.x)) / select(0.000000000000000000000000000001f, _e1563, (_e1563 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1494.y * _e1494.y) + (_e1496 * _e1496)) / select(0.000000000000000000000000000001f, _e1524, (_e1524 > 0.000000000000000000000000000001f))) + (((_e1536.y * _e1536.y) + (_e1537.y * _e1537.y)) / select(0.000000000000000000000000000001f, _e1564, (_e1564 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1494.z * _e1494.z) + (_e1497 * _e1497)) / select(0.000000000000000000000000000001f, _e1525, (_e1525 > 0.000000000000000000000000000001f))) + (((_e1536.z * _e1536.z) + (_e1537.z * _e1537.z)) / select(0.000000000000000000000000000001f, _e1565, (_e1565 > 0.000000000000000000000000000001f)))) * 0.5f));
                                            }
                                            let _e1601 = phi_13965_;
                                            phi_13966_ = _e1601;
                                        }
                                        let _e1618 = phi_13966_;
                                        phi_13967_ = (_e345 - vec3<f32>(((_e1039.x * 2f) * _e1309), ((_e1039.y * 2f) * _e1309), ((_e1039.z * 2f) * _e1309)));
                                        phi_13968_ = _e1618;
                                    } else {
                                        phi_13967_ = vec3<f32>();
                                        phi_13968_ = vec3<f32>();
                                    }
                                    let _e1620 = phi_13967_;
                                    let _e1622 = phi_13968_;
                                    phi_33401_ = vec3<f32>(_e1010.member_5.x, _e1010.member_5.y, _e1010.member_5.z);
                                    phi_33324_ = vec3<f32>(_e1010.member_4.x, _e1010.member_4.y, _e1010.member_4.z);
                                    phi_33245_ = _e1300;
                                    phi_33163_ = _e1288;
                                    phi_13970_ = _e1620;
                                    phi_13971_ = _e1622;
                                    phi_13972_ = _e1301;
                                } else {
                                    phi_33401_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33324_ = vec3<f32>(0f, 0f, 0f);
                                    phi_33245_ = 0u;
                                    phi_33163_ = 1f;
                                    phi_13970_ = _e1274;
                                    phi_13971_ = _e1276;
                                    phi_13972_ = _e1280;
                                }
                                let _e1624 = phi_33401_;
                                let _e1626 = phi_33324_;
                                let _e1628 = phi_33245_;
                                let _e1630 = phi_33163_;
                                let _e1632 = phi_13970_;
                                let _e1634 = phi_13971_;
                                let _e1636 = phi_13972_;
                                phi_33670_ = vec3<f32>(0f, 0f, 0f);
                                phi_33478_ = 1.5f;
                                phi_33399_ = _e1624;
                                phi_33322_ = _e1626;
                                phi_33243_ = _e1628;
                                phi_33161_ = _e1630;
                                phi_33084_ = select(0u, 1u, _e1278);
                                phi_33010_ = _e1270;
                                phi_13980_ = select(_e1272, _e1067, vec3(_e1278));
                                phi_13981_ = _e1632;
                                phi_13982_ = _e1634;
                                phi_13983_ = select(true, false, _e1636);
                            }
                            let _e1746 = phi_33670_;
                            let _e1748 = phi_33478_;
                            let _e1750 = phi_33399_;
                            let _e1752 = phi_33322_;
                            let _e1754 = phi_33243_;
                            let _e1756 = phi_33161_;
                            let _e1758 = phi_33084_;
                            let _e1760 = phi_33010_;
                            let _e1762 = phi_13980_;
                            let _e1764 = phi_13981_;
                            let _e1766 = phi_13982_;
                            let _e1768 = phi_13983_;
                            let _e1769 = select(true, false, _e1768);
                            if _e1769 {
                                if (_e356 >= _e373) {
                                    phi_33069_ = _e1760;
                                    phi_14099_ = _e820;
                                    phi_14100_ = type_70();
                                    phi_14101_ = true;
                                } else {
                                    let _e1782 = global_1.member.member_4[1u];
                                    if (sqrt((((_e1766.x * _e1766.x) + (_e1766.y * _e1766.y)) + (_e1766.z * _e1766.z))) <= _e1782) {
                                        phi_33064_ = _e1760;
                                        phi_14063_ = _e820;
                                        phi_14064_ = false;
                                    } else {
                                        let _e1786 = global_1.member.member_7[1u];
                                        if (_e1786 == 1u) {
                                            let _e1791 = global_1.member.member_4[3u];
                                            if (f32(_e356) >= _e1791) {
                                                let _e1794 = select(_e1766.y, _e1766.x, (_e1766.x > _e1766.y));
                                                let _e1796 = select(_e1766.z, _e1794, (_e1794 > _e1766.z));
                                                if (_e1796 != _e1796) {
                                                    phi_26308_ = true;
                                                } else {
                                                    phi_26308_ = (1f <= _e1796);
                                                }
                                                let _e1800 = phi_26308_;
                                                let _e1801 = select(_e1796, 1f, _e1800);
                                                let _e1803 = ((_e1760 * 747796405u) + 2891336453u);
                                                let _e1811 = (((_e1803 >> bitcast<u32>((((_e1803 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1803) * 277803737u);
                                                let _e1814 = ((_e1811 >> bitcast<u32>(22i)) ^ _e1811);
                                                let _e1819 = ((f32((_e1814 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1801);
                                                if _e1819 {
                                                    phi_14052_ = vec3<f32>();
                                                } else {
                                                    phi_14052_ = vec3<f32>((_e820.x / _e1801), (_e820.y / _e1801), (_e820.z / _e1801));
                                                }
                                                let _e1828 = phi_14052_;
                                                phi_33067_ = _e1814;
                                                phi_14054_ = _e1828;
                                                phi_14055_ = select(true, false, _e1819);
                                            } else {
                                                phi_33067_ = _e1760;
                                                phi_14054_ = _e820;
                                                phi_14055_ = true;
                                            }
                                            let _e1831 = phi_33067_;
                                            let _e1833 = phi_14054_;
                                            let _e1835 = phi_14055_;
                                            phi_33066_ = _e1831;
                                            phi_14056_ = _e1833;
                                            phi_14057_ = _e1835;
                                        } else {
                                            phi_33066_ = _e1760;
                                            phi_14056_ = _e820;
                                            phi_14057_ = true;
                                        }
                                        let _e1837 = phi_33066_;
                                        let _e1839 = phi_14056_;
                                        let _e1841 = phi_14057_;
                                        phi_33064_ = _e1837;
                                        phi_14063_ = select(_e820, _e1839, vec3(_e1841));
                                        phi_14064_ = select(false, true, _e1841);
                                    }
                                    let _e1846 = phi_33064_;
                                    let _e1848 = phi_14063_;
                                    let _e1850 = phi_14064_;
                                    if _e1850 {
                                        let _e1851 = (_e1848 * _e1766);
                                        let _e1862 = (_e1764 * (1f / sqrt((((_e1764.x * _e1764.x) + (_e1764.y * _e1764.y)) + (_e1764.z * _e1764.z)))));
                                        phi_14097_ = type_70(type_35(vec4<f32>(_e1762.x, _e1762.y, _e1762.z, _e331.member.w), vec4<f32>(_e1862.x, _e1862.y, _e1862.z, _e331.member_1.w), vec4<f32>(_e1851.x, _e1851.y, _e1851.z, f32((_e356 + 1u))), vec4<f32>(_e331.member_3.x, _e331.member_3.y, _e331.member_3.z, f32(_e382)), vec4<u32>(_e1846, (_e331.member_4.y | 2u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14097_ = type_70();
                                    }
                                    let _e1885 = phi_14097_;
                                    phi_33069_ = _e1846;
                                    phi_14099_ = _e1848;
                                    phi_14100_ = _e1885;
                                    phi_14101_ = select(true, false, _e1850);
                                }
                                let _e1888 = phi_33069_;
                                let _e1890 = phi_14099_;
                                let _e1892 = phi_14100_;
                                let _e1894 = phi_14101_;
                                if _e1894 {
                                    phi_14107_ = type_70(type_35(_e331.member, _e331.member_1, vec4<f32>(_e1890.x, _e1890.y, _e1890.z, f32(_e356)), vec4<f32>(_e331.member_3.x, _e331.member_3.y, _e331.member_3.z, f32(_e382)), vec4<u32>(_e1888, (_e331.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                } else {
                                    phi_14107_ = _e1892;
                                }
                                let _e1907 = phi_14107_;
                                phi_33073_ = _e1888;
                                phi_14108_ = _e1907;
                            } else {
                                phi_33073_ = _e1760;
                                phi_14108_ = type_70();
                            }
                            let _e1909 = phi_33073_;
                            let _e1911 = phi_14108_;
                            phi_33667_ = _e1746;
                            phi_33475_ = _e1748;
                            phi_33396_ = _e1750;
                            phi_33319_ = _e1752;
                            phi_33240_ = _e1754;
                            phi_33158_ = _e1756;
                            phi_33081_ = _e1758;
                            phi_33072_ = _e1909;
                            phi_14110_ = _e1911;
                            phi_14111_ = select(true, false, _e1769);
                        }
                        let _e1919 = phi_33667_;
                        let _e1921 = phi_33475_;
                        let _e1923 = phi_33396_;
                        let _e1925 = phi_33319_;
                        let _e1927 = phi_33240_;
                        let _e1929 = phi_33158_;
                        let _e1931 = phi_33081_;
                        let _e1933 = phi_33072_;
                        let _e1935 = phi_14110_;
                        let _e1937 = phi_14111_;
                        if _e1937 {
                            let _e1939 = ((_e331.member_4.y | 4u) & 4294967293u);
                            let _e1942 = global_1.member.member_1[3u];
                            phi_34996_ = _e818;
                            phi_33071_ = _e1933;
                            phi_14122_ = _e360;
                            phi_14125_ = type_42(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            phi_14127_ = 0u;
                            phi_14129_ = _e382;
                            loop {
                                let _e1944 = phi_34996_;
                                let _e1946 = phi_33071_;
                                let _e1948 = phi_14122_;
                                let _e1950 = phi_14125_;
                                let _e1952 = phi_14127_;
                                let _e1954 = phi_14129_;
                                local_83 = _e1954;
                                local_84 = _e1948;
                                local_85 = _e1948;
                                local_86 = _e1948;
                                local_87 = _e1950;
                                local_88 = _e1954;
                                local_89 = _e1948;
                                local_90 = _e1948;
                                local_91 = _e1948;
                                local_92 = _e1950;
                                local_93 = _e1954;
                                local_94 = _e1948;
                                local_95 = _e1948;
                                local_96 = _e1948;
                                local_97 = _e1950;
                                local_98 = _e1954;
                                local_99 = _e1948;
                                local_100 = _e1948;
                                local_101 = _e1948;
                                local_102 = _e1950;
                                let _e1956 = (_e1952 < (_e1942 + 3u));
                                if _e1956 {
                                    if (_e1952 < _e1942) {
                                        let _e3941 = global_5.member[_e1952];
                                        let _e3946 = (_e3941.member.x - _e1064);
                                        let _e3947 = (_e3941.member.y - _e1065);
                                        let _e3948 = (_e3941.member.z - _e1066);
                                        let _e3953 = (((_e3946 * _e3946) + (_e3947 * _e3947)) + (_e3948 * _e3948));
                                        let _e3954 = sqrt(_e3953);
                                        let _e3955 = (_e3946 / _e3954);
                                        let _e3956 = (_e3947 / _e3954);
                                        let _e3957 = (_e3948 / _e3954);
                                        if (_e3941.member.w < 0.5f) {
                                            phi_14226_ = 1f;
                                        } else {
                                            let _e3981 = (((_e3941.member_2.x * -(_e3955)) + (_e3941.member_2.y * -(_e3956))) + (_e3941.member_2.z * -(_e3957)));
                                            if (_e3941.member_3.x == _e3941.member_2.w) {
                                                phi_14225_ = select(1f, 0f, (_e3981 < _e3941.member_3.x));
                                            } else {
                                                let _e3985 = ((_e3981 - _e3941.member_3.x) / (_e3941.member_2.w - _e3941.member_3.x));
                                                let _e3987 = select(_e3985, 0f, (_e3985 < 0f));
                                                let _e3989 = select(_e3987, 1f, (_e3987 > 1f));
                                                phi_14225_ = ((_e3989 * _e3989) * (3f - (2f * _e3989)));
                                            }
                                            let _e3997 = phi_14225_;
                                            phi_14226_ = _e3997;
                                        }
                                        let _e3999 = phi_14226_;
                                        phi_35413_ = _e1946;
                                        phi_17164_ = 0f;
                                        phi_17165_ = vec3<f32>(_e3955, _e3956, _e3957);
                                        phi_17166_ = vec3<f32>(((_e3941.member_1.x * _e3999) / _e3953), ((_e3941.member_1.y * _e3999) / _e3953), ((_e3941.member_1.z * _e3999) / _e3953));
                                        phi_17167_ = _e3954;
                                    } else {
                                        if (_e1952 == _e1942) {
                                            if ((_e376 & 4u) == 0u) {
                                                phi_35457_ = _e1946;
                                                phi_16268_ = 0f;
                                                phi_16269_ = vec3<f32>(0f, 0f, 0f);
                                                phi_16270_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                let _e2693 = global_1.member.member_5;
                                                let _e2701 = global_1.member.member_6[3u];
                                                let _e2703 = sin((_e2701 * 0.5f));
                                                let _e2705 = ((12.566371f * _e2703) * _e2703);
                                                let _e2710 = (((_e2693.x * _e1039.x) + (_e2693.y * _e1039.y)) + (_e2693.z * _e1039.z));
                                                if (_e2710 != _e2710) {
                                                    phi_26847_ = true;
                                                } else {
                                                    phi_26847_ = (0f >= _e2710);
                                                }
                                                let _e2714 = phi_26847_;
                                                let _e2716 = (select(_e2710, 0f, _e2714) * 0.31830987f);
                                                if (_e1931 == 0u) {
                                                    if (_e2710 <= 0f) {
                                                        phi_27644_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_27644_ = (_e1925 * (_e2710 * 0.31830987f));
                                                    }
                                                    let _e3192 = phi_27644_;
                                                    phi_14636_ = type_60(_e3192, _e2716);
                                                    phi_14637_ = type_60();
                                                    phi_14638_ = true;
                                                } else {
                                                    let _e2722 = (((_e1013 * _e1087) + (_e1014 * _e1088)) + (_e1015 * _e1090));
                                                    let _e2727 = (((_e1013 * _e1083) + (_e1014 * _e1093)) + (_e1015 * _e1094));
                                                    let _e2732 = (((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z));
                                                    let _e2737 = (((_e2693.x * _e1087) + (_e2693.y * _e1088)) + (_e2693.z * _e1090));
                                                    let _e2742 = (((_e2693.x * _e1083) + (_e2693.y * _e1093)) + (_e2693.z * _e1094));
                                                    if (_e2732 <= 0f) {
                                                        phi_14600_ = type_60();
                                                        phi_14601_ = true;
                                                    } else {
                                                        let _e2744 = (_e2710 <= 0f);
                                                        if _e2744 {
                                                            phi_14599_ = type_60();
                                                        } else {
                                                            let _e2745 = (_e2722 + _e2737);
                                                            let _e2746 = (_e2727 + _e2742);
                                                            let _e2747 = (_e2732 + _e2710);
                                                            let _e2754 = (1f / sqrt((((_e2745 * _e2745) + (_e2746 * _e2746)) + (_e2747 * _e2747))));
                                                            let _e2757 = (_e2747 * _e2754);
                                                            if (_e2757 <= 0f) {
                                                                phi_26957_ = 0f;
                                                            } else {
                                                                let _e2759 = (_e1929 * _e1929);
                                                                let _e2763 = ((((_e2759 - 1f) * _e2757) * _e2757) + 1f);
                                                                phi_26957_ = (_e2759 / ((3.1415927f * _e2763) * _e2763));
                                                            }
                                                            let _e2768 = phi_26957_;
                                                            let _e2769 = (_e2732 * _e2732);
                                                            if (_e2769 >= 1f) {
                                                                phi_26985_ = 0f;
                                                            } else {
                                                                phi_26985_ = ((sqrt((1f + (((_e1929 * _e1929) * ((_e2722 * _e2722) + (_e2727 * _e2727))) / _e2769))) - 1f) * 0.5f);
                                                            }
                                                            let _e2782 = phi_26985_;
                                                            let _e2783 = (1f + _e2782);
                                                            let _e2784 = (_e2710 * _e2710);
                                                            if (_e2784 >= 1f) {
                                                                phi_27013_ = 0f;
                                                            } else {
                                                                phi_27013_ = ((sqrt((1f + (((_e1929 * _e1929) * ((_e2737 * _e2737) + (_e2742 * _e2742))) / _e2784))) - 1f) * 0.5f);
                                                            }
                                                            let _e2797 = phi_27013_;
                                                            let _e2804 = (((_e2722 * (_e2745 * _e2754)) + (_e2727 * (_e2746 * _e2754))) + (_e2732 * _e2757));
                                                            if (_e1927 == 0u) {
                                                                let _e3099 = (1f - _e2804);
                                                                if (_e3099 != _e3099) {
                                                                    phi_27516_ = true;
                                                                } else {
                                                                    phi_27516_ = (0f >= _e3099);
                                                                }
                                                                let _e3103 = phi_27516_;
                                                                let _e3105 = pow(select(_e3099, 0f, _e3103), 5f);
                                                                let _e3106 = (1f - _e3105);
                                                                phi_14587_ = vec3<f32>(((_e1925.x * _e3106) + _e3105), ((_e1925.y * _e3106) + _e3105), ((_e1925.z * _e3106) + _e3105));
                                                            } else {
                                                                if (_e1927 == 1u) {
                                                                    let _e3072 = (1f / _e1921);
                                                                    let _e3076 = ((_e3072 * _e3072) * (1f - (_e2804 * _e2804)));
                                                                    if (_e3076 >= 1f) {
                                                                        phi_27477_ = 1f;
                                                                    } else {
                                                                        let _e3079 = sqrt((1f - _e3076));
                                                                        if (_e2804 != _e2804) {
                                                                            phi_27488_ = true;
                                                                        } else {
                                                                            phi_27488_ = (_e3079 <= _e2804);
                                                                        }
                                                                        let _e3083 = phi_27488_;
                                                                        let _e3088 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                                                        phi_27477_ = (_e3088 + ((1f - _e3088) * pow((1f - select(_e2804, _e3079, _e3083)), 5f)));
                                                                    }
                                                                    let _e3095 = phi_27477_;
                                                                    phi_14586_ = vec3(_e3095);
                                                                } else {
                                                                    let _e2808 = select(_e2804, 0f, (_e2804 < 0f));
                                                                    let _e2810 = select(_e2808, 1f, (_e2808 > 1f));
                                                                    let _e2812 = (1f - (_e2810 * _e2810));
                                                                    let _e2825 = ((_e1925.x * _e1925.x) - (_e1923.x * _e1923.x));
                                                                    let _e2826 = ((_e1925.y * _e1925.y) - (_e1923.y * _e1923.y));
                                                                    let _e2827 = ((_e1925.z * _e1925.z) - (_e1923.z * _e1923.z));
                                                                    let _e2829 = (_e1925.x * (_e1923.x + _e1923.x));
                                                                    let _e2831 = (_e1925.y * (_e1923.y + _e1923.y));
                                                                    let _e2833 = (_e1925.z * (_e1923.z + _e1923.z));
                                                                    let _e2840 = ((_e2825 * _e2825) + (_e2829 * _e2829));
                                                                    let _e2841 = ((_e2826 * _e2826) + (_e2831 * _e2831));
                                                                    let _e2842 = ((_e2827 * _e2827) + (_e2833 * _e2833));
                                                                    let _e2847 = ((_e2812 * _e2825) / _e2840);
                                                                    let _e2848 = ((_e2812 * _e2826) / _e2841);
                                                                    let _e2849 = ((_e2812 * _e2827) / _e2842);
                                                                    let _e2850 = (1f - _e2847);
                                                                    let _e2851 = (1f - _e2848);
                                                                    let _e2852 = (1f - _e2849);
                                                                    let _e2854 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2847, _e2848, _e2849));
                                                                    let _e2855 = (_e2812 * _e2829);
                                                                    let _e2856 = (_e2812 * _e2831);
                                                                    let _e2857 = (_e2812 * _e2833);
                                                                    let _e2862 = (vec3<f32>(_e2855, _e2856, _e2857) / vec3<f32>(_e2840, _e2841, _e2842));
                                                                    let _e2875 = ((_e2854.x * _e2854.x) + (_e2862.x * _e2862.x));
                                                                    let _e2876 = ((_e2854.y * _e2854.y) + (_e2862.y * _e2862.y));
                                                                    let _e2877 = ((_e2854.z * _e2854.z) + (_e2862.z * _e2862.z));
                                                                    if (_e2875 != _e2875) {
                                                                        phi_27147_ = true;
                                                                    } else {
                                                                        phi_27147_ = (0f >= _e2875);
                                                                    }
                                                                    let _e2881 = phi_27147_;
                                                                    let _e2883 = sqrt(select(_e2875, 0f, _e2881));
                                                                    if (_e2876 != _e2876) {
                                                                        phi_27162_ = true;
                                                                    } else {
                                                                        phi_27162_ = (0f >= _e2876);
                                                                    }
                                                                    let _e2887 = phi_27162_;
                                                                    let _e2889 = sqrt(select(_e2876, 0f, _e2887));
                                                                    if (_e2877 != _e2877) {
                                                                        phi_27177_ = true;
                                                                    } else {
                                                                        phi_27177_ = (0f >= _e2877);
                                                                    }
                                                                    let _e2893 = phi_27177_;
                                                                    let _e2895 = sqrt(select(_e2877, 0f, _e2893));
                                                                    let _e2899 = ((_e2883 + _e2850) * 0.5f);
                                                                    let _e2900 = ((_e2889 + _e2851) * 0.5f);
                                                                    let _e2901 = ((_e2895 + _e2852) * 0.5f);
                                                                    if (_e2899 != _e2899) {
                                                                        phi_27204_ = true;
                                                                    } else {
                                                                        phi_27204_ = (0f >= _e2899);
                                                                    }
                                                                    let _e2905 = phi_27204_;
                                                                    let _e2907 = sqrt(select(_e2899, 0f, _e2905));
                                                                    if (_e2900 != _e2900) {
                                                                        phi_27219_ = true;
                                                                    } else {
                                                                        phi_27219_ = (0f >= _e2900);
                                                                    }
                                                                    let _e2911 = phi_27219_;
                                                                    let _e2913 = sqrt(select(_e2900, 0f, _e2911));
                                                                    if (_e2901 != _e2901) {
                                                                        phi_27234_ = true;
                                                                    } else {
                                                                        phi_27234_ = (0f >= _e2901);
                                                                    }
                                                                    let _e2917 = phi_27234_;
                                                                    let _e2919 = sqrt(select(_e2901, 0f, _e2917));
                                                                    let _e2920 = vec3<f32>(_e2907, _e2913, _e2919);
                                                                    let _e2924 = ((_e2883 - _e2850) * 0.5f);
                                                                    let _e2925 = ((_e2889 - _e2851) * 0.5f);
                                                                    let _e2926 = ((_e2895 - _e2852) * 0.5f);
                                                                    if (_e2924 != _e2924) {
                                                                        phi_27261_ = true;
                                                                    } else {
                                                                        phi_27261_ = (0f >= _e2924);
                                                                    }
                                                                    let _e2930 = phi_27261_;
                                                                    let _e2932 = sqrt(select(_e2924, 0f, _e2930));
                                                                    if (_e2925 != _e2925) {
                                                                        phi_27276_ = true;
                                                                    } else {
                                                                        phi_27276_ = (0f >= _e2925);
                                                                    }
                                                                    let _e2936 = phi_27276_;
                                                                    let _e2938 = sqrt(select(_e2925, 0f, _e2936));
                                                                    if (_e2926 != _e2926) {
                                                                        phi_27291_ = true;
                                                                    } else {
                                                                        phi_27291_ = (0f >= _e2926);
                                                                    }
                                                                    let _e2942 = phi_27291_;
                                                                    let _e2944 = sqrt(select(_e2926, 0f, _e2942));
                                                                    if ((_e2855 / _e2840) < 0f) {
                                                                        phi_14463_ = -(_e2932);
                                                                    } else {
                                                                        phi_14463_ = _e2932;
                                                                    }
                                                                    let _e2948 = phi_14463_;
                                                                    if ((_e2856 / _e2841) < 0f) {
                                                                        phi_14471_ = -(_e2938);
                                                                    } else {
                                                                        phi_14471_ = _e2938;
                                                                    }
                                                                    let _e2952 = phi_14471_;
                                                                    if ((_e2857 / _e2842) < 0f) {
                                                                        phi_14479_ = -(_e2944);
                                                                    } else {
                                                                        phi_14479_ = _e2944;
                                                                    }
                                                                    let _e2956 = phi_14479_;
                                                                    let _e2960 = (_e1923.x * _e2810);
                                                                    let _e2961 = (_e1923.y * _e2810);
                                                                    let _e2962 = (_e1923.z * _e2810);
                                                                    let _e2972 = vec3<f32>(((_e1925.x * _e2907) - (_e1923.x * _e2948)), ((_e1925.y * _e2913) - (_e1923.y * _e2952)), ((_e1925.z * _e2919) - (_e1923.z * _e2956)));
                                                                    let _e2979 = ((_e1925.x * _e2948) + (_e1923.x * _e2907));
                                                                    let _e2980 = ((_e1925.y * _e2952) + (_e1923.y * _e2913));
                                                                    let _e2981 = ((_e1925.z * _e2956) + (_e1923.z * _e2919));
                                                                    let _e2983 = vec3<f32>((_e1925.x * _e2810), (_e1925.y * _e2810), (_e1925.z * _e2810));
                                                                    let _e2984 = (_e2983 - _e2920);
                                                                    let _e2985 = (_e2960 - _e2948);
                                                                    let _e2986 = (_e2961 - _e2952);
                                                                    let _e2987 = (_e2962 - _e2956);
                                                                    let _e3000 = (_e2983 + _e2920);
                                                                    let _e3001 = (_e2960 + _e2948);
                                                                    let _e3002 = (_e2961 + _e2952);
                                                                    let _e3003 = (_e2962 + _e2956);
                                                                    let _e3013 = ((_e3000.x * _e3000.x) + (_e3001 * _e3001));
                                                                    let _e3014 = ((_e3000.y * _e3000.y) + (_e3002 * _e3002));
                                                                    let _e3015 = ((_e3000.z * _e3000.z) + (_e3003 * _e3003));
                                                                    let _e3025 = vec3(_e2810);
                                                                    let _e3026 = (_e3025 - _e2972);
                                                                    let _e3027 = -(vec3<f32>(_e2979, _e2980, _e2981));
                                                                    let _e3043 = (_e3025 + _e2972);
                                                                    let _e3053 = ((_e3043.x * _e3043.x) + (_e2979 * _e2979));
                                                                    let _e3054 = ((_e3043.y * _e3043.y) + (_e2980 * _e2980));
                                                                    let _e3055 = ((_e3043.z * _e3043.z) + (_e2981 * _e2981));
                                                                    phi_14586_ = vec3<f32>((((((_e2984.x * _e2984.x) + (_e2985 * _e2985)) / select(0.000000000000000000000000000001f, _e3013, (_e3013 > 0.000000000000000000000000000001f))) + (((_e3026.x * _e3026.x) + (_e3027.x * _e3027.x)) / select(0.000000000000000000000000000001f, _e3053, (_e3053 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2984.y * _e2984.y) + (_e2986 * _e2986)) / select(0.000000000000000000000000000001f, _e3014, (_e3014 > 0.000000000000000000000000000001f))) + (((_e3026.y * _e3026.y) + (_e3027.y * _e3027.y)) / select(0.000000000000000000000000000001f, _e3054, (_e3054 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e2984.z * _e2984.z) + (_e2987 * _e2987)) / select(0.000000000000000000000000000001f, _e3015, (_e3015 > 0.000000000000000000000000000001f))) + (((_e3026.z * _e3026.z) + (_e3027.z * _e3027.z)) / select(0.000000000000000000000000000001f, _e3055, (_e3055 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                }
                                                                let _e3098 = phi_14586_;
                                                                phi_14587_ = _e3098;
                                                            }
                                                            let _e3118 = phi_14587_;
                                                            let _e3120 = (4f * _e2732);
                                                            phi_14599_ = type_60((_e3118 * (((1f / (_e2783 + _e2797)) * _e2768) / _e3120)), ((_e2768 / _e2783) / _e3120));
                                                        }
                                                        let _e3127 = phi_14599_;
                                                        phi_14600_ = _e3127;
                                                        phi_14601_ = _e2744;
                                                    }
                                                    let _e3129 = phi_14600_;
                                                    let _e3131 = phi_14601_;
                                                    if _e3131 {
                                                        phi_14606_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        phi_14606_ = _e3129;
                                                    }
                                                    let _e3133 = phi_14606_;
                                                    let _e3136 = (_e1931 == 1u);
                                                    if _e3136 {
                                                        phi_14634_ = _e3133;
                                                        phi_14635_ = type_60();
                                                    } else {
                                                        if (_e2732 != _e2732) {
                                                            phi_27553_ = true;
                                                        } else {
                                                            phi_27553_ = (0f >= _e2732);
                                                        }
                                                        let _e3140 = phi_27553_;
                                                        let _e3141 = select(_e2732, 0f, _e3140);
                                                        let _e3142 = (1f / _e1921);
                                                        let _e3146 = ((_e3142 * _e3142) * (1f - (_e3141 * _e3141)));
                                                        if (_e3146 >= 1f) {
                                                            phi_27572_ = 1f;
                                                        } else {
                                                            let _e3149 = sqrt((1f - _e3146));
                                                            if (_e3141 != _e3141) {
                                                                phi_27583_ = true;
                                                            } else {
                                                                phi_27583_ = (_e3149 <= _e3141);
                                                            }
                                                            let _e3153 = phi_27583_;
                                                            let _e3158 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                                            phi_27572_ = (_e3158 + ((1f - _e3158) * pow((1f - select(_e3141, _e3149, _e3153)), 5f)));
                                                        }
                                                        let _e3165 = phi_27572_;
                                                        if (_e2710 <= 0f) {
                                                            phi_27622_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_27622_ = (_e1919 * (_e2710 * 0.31830987f));
                                                        }
                                                        let _e3170 = phi_27622_;
                                                        let _e3171 = (1f - _e3165);
                                                        phi_14634_ = type_60();
                                                        phi_14635_ = type_60((_e3133.member + vec3<f32>((_e3170.x * _e3171), (_e3170.y * _e3171), (_e3170.z * _e3171))), ((_e3165 * _e3133.member_1) + (_e3171 * _e2716)));
                                                    }
                                                    let _e3185 = phi_14634_;
                                                    let _e3187 = phi_14635_;
                                                    phi_14636_ = _e3185;
                                                    phi_14637_ = _e3187;
                                                    phi_14638_ = _e3136;
                                                }
                                                let _e3195 = phi_14636_;
                                                let _e3197 = phi_14637_;
                                                let _e3199 = phi_14638_;
                                                if _e3199 {
                                                    phi_14642_ = _e3195;
                                                } else {
                                                    phi_14642_ = _e3197;
                                                }
                                                let _e3201 = phi_14642_;
                                                if ((_e3201.member_1 * _e2705) < 0.001f) {
                                                    let _e3628 = global_1.member.member_6;
                                                    let _e3634 = global_1.member.member_8[2u];
                                                    if (_e3634 == 4294967295u) {
                                                        phi_15381_ = vec3<f32>(1f, 1f, 1f);
                                                    } else {
                                                        let _e3637 = global_6.member[_e3634];
                                                        let _e3640 = global_6.member[(_e3634 + 1u)];
                                                        let _e3643 = global_6.member[(_e3634 + 2u)];
                                                        let _e3646 = global_6.member[(_e3634 + 3u)];
                                                        let _e3649 = global_6.member[(_e3634 + 4u)];
                                                        let _e3652 = global_6.member[(_e3634 + 5u)];
                                                        let _e3655 = global_6.member[(_e3634 + 6u)];
                                                        let _e3658 = global_6.member[(_e3634 + 7u)];
                                                        let _e3661 = global_6.member[(_e3634 + 8u)];
                                                        let _e3664 = global_6.member[(_e3634 + 9u)];
                                                        let _e3667 = global_6.member[(_e3634 + 13u)];
                                                        let _e3670 = global_6.member[(_e3634 + 14u)];
                                                        let _e3673 = global_6.member[(_e3634 + 15u)];
                                                        let _e3676 = global_6.member[(_e3634 + 16u)];
                                                        let _e3679 = global_6.member[(_e3634 + 18u)];
                                                        let _e3682 = global_6.member[(_e3634 + 19u)];
                                                        let _e3685 = global_6.member[(_e3634 + 20u)];
                                                        let _e3688 = global_6.member[(_e3634 + 29u)];
                                                        let _e3689 = bitcast<u32>(_e3688);
                                                        let _e3702 = (((((_e1064 * _e1064) + (_e1065 * _e1065)) + (_e1066 * _e1066)) - (2f * (((_e1064 * _e3637) + (_e1065 * _e3640)) + (_e1066 * _e3643)))) + _e3655);
                                                        if (_e3702 > 0f) {
                                                            let _e3711 = ((((_e1064 - _e3637) * _e3658) + ((_e1065 - _e3640) * _e3661)) + ((_e1066 - _e3643) * _e3664));
                                                            let _e3713 = ((_e3711 * _e3711) - _e3702);
                                                            if (_e3711 >= 0f) {
                                                                phi_15081_ = vec3<f32>();
                                                                phi_15082_ = true;
                                                            } else {
                                                                let _e3715 = (_e3713 <= 0f);
                                                                if _e3715 {
                                                                    phi_15080_ = vec3<f32>();
                                                                } else {
                                                                    let _e3718 = (-(_e3711) - sqrt(_e3713));
                                                                    phi_15080_ = vec3<f32>((_e1064 + (_e3658 * _e3718)), (_e1065 + (_e3661 * _e3718)), (_e1066 + (_e3664 * _e3718)));
                                                                }
                                                                let _e3727 = phi_15080_;
                                                                phi_15081_ = _e3727;
                                                                phi_15082_ = _e3715;
                                                            }
                                                            let _e3729 = phi_15081_;
                                                            let _e3731 = phi_15082_;
                                                            phi_15087_ = _e3729;
                                                            phi_15088_ = select(true, false, _e3731);
                                                        } else {
                                                            phi_15087_ = _e1067;
                                                            phi_15088_ = true;
                                                        }
                                                        let _e3734 = phi_15087_;
                                                        let _e3736 = phi_15088_;
                                                        if _e3736 {
                                                            let _e3738 = (_e3734.x - _e3637);
                                                            let _e3740 = (_e3734.y - _e3640);
                                                            let _e3742 = (_e3734.z - _e3643);
                                                            let _e3748 = sqrt((((_e3738 * _e3738) + (_e3740 * _e3740)) + (_e3742 * _e3742)));
                                                            let _e3749 = (1f / _e3748);
                                                            let _e3767 = ((((((_e3734.x * _e3734.x) + (_e3734.y * _e3734.y)) + (_e3734.z * _e3734.z)) - (2f * (((_e3734.x * _e3637) + (_e3734.y * _e3640)) + (_e3734.z * _e3643)))) + _e3652) / (_e3748 + _e3646));
                                                            if (_e3767 != _e3767) {
                                                                phi_28078_ = true;
                                                            } else {
                                                                phi_28078_ = (0f >= _e3767);
                                                            }
                                                            let _e3771 = phi_28078_;
                                                            let _e3775 = sqrt((select(_e3767, 0f, _e3771) / (_e3649 - _e3646)));
                                                            if (_e3775 != _e3775) {
                                                                phi_28093_ = true;
                                                            } else {
                                                                phi_28093_ = (1f <= _e3775);
                                                            }
                                                            let _e3779 = phi_28093_;
                                                            let _e3781 = (select(_e3775, 1f, _e3779) * 63f);
                                                            let _e3786 = ((((_e3738 * _e3749) * _e3658) + ((_e3740 * _e3749) * _e3661)) + ((_e3742 * _e3749) * _e3664));
                                                            let _e3788 = select(_e3786, -1f, (_e3786 < -1f));
                                                            let _e3792 = ((select(_e3788, 1f, (_e3788 > 1f)) + 1f) * 127.5f);
                                                            let _e3797 = select(select(u32(_e3781), 0u, (_e3781 < 0f)), 4294967295u, (_e3781 > 4294967000f));
                                                            let _e3799 = select(_e3797, 62u, (62u < _e3797));
                                                            let _e3804 = select(select(u32(_e3792), 0u, (_e3792 < 0f)), 4294967295u, (_e3792 > 4294967000f));
                                                            let _e3806 = select(_e3804, 254u, (254u < _e3804));
                                                            let _e3808 = (_e3781 - f32(_e3799));
                                                            let _e3810 = (_e3792 - f32(_e3806));
                                                            let _e3814 = (_e3689 + (3u * ((_e3799 * 256u) + _e3806)));
                                                            let _e3819 = (_e3689 + (3u * (((_e3799 + 1u) * 256u) + _e3806)));
                                                            let _e3821 = global_6.member[_e3814];
                                                            let _e3824 = global_6.member[(_e3814 + 1u)];
                                                            let _e3827 = global_6.member[(_e3814 + 2u)];
                                                            let _e3828 = (1f - _e3810);
                                                            let _e3834 = global_6.member[(_e3814 + 3u)];
                                                            let _e3837 = global_6.member[(_e3814 + 4u)];
                                                            let _e3840 = global_6.member[(_e3814 + 5u)];
                                                            let _e3847 = (1f - _e3808);
                                                            let _e3852 = global_6.member[_e3819];
                                                            let _e3855 = global_6.member[(_e3819 + 1u)];
                                                            let _e3858 = global_6.member[(_e3819 + 2u)];
                                                            let _e3864 = global_6.member[(_e3819 + 3u)];
                                                            let _e3867 = global_6.member[(_e3819 + 4u)];
                                                            let _e3870 = global_6.member[(_e3819 + 5u)];
                                                            phi_15379_ = vec3<f32>(((((_e3821 * _e3828) + (_e3834 * _e3810)) * _e3847) + (((_e3852 * _e3828) + (_e3864 * _e3810)) * _e3808)), ((((_e3824 * _e3828) + (_e3837 * _e3810)) * _e3847) + (((_e3855 * _e3828) + (_e3867 * _e3810)) * _e3808)), ((((_e3827 * _e3828) + (_e3840 * _e3810)) * _e3847) + (((_e3858 * _e3828) + (_e3870 * _e3810)) * _e3808)));
                                                        } else {
                                                            phi_15379_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3885 = phi_15379_;
                                                        let _e3892 = ((_e3676 * 1.1f) * _e3885.y);
                                                        phi_15381_ = vec3<f32>(exp(-((((_e3667 * _e3885.x) + _e3892) + (_e3679 * _e3885.z)))), exp(-((((_e3670 * _e3885.x) + _e3892) + (_e3682 * _e3885.z)))), exp(-((((_e3673 * _e3885.x) + _e3892) + (_e3685 * _e3885.z)))));
                                                    }
                                                    let _e3911 = phi_15381_;
                                                    phi_35458_ = _e1946;
                                                    phi_16265_ = 0f;
                                                    phi_16266_ = vec3<f32>(_e2693.x, _e2693.y, _e2693.z);
                                                    phi_16267_ = (vec3<f32>(_e3628.x, _e3628.y, _e3628.z) * _e3911);
                                                } else {
                                                    let _e3206 = ((_e1946 * 747796405u) + 2891336453u);
                                                    let _e3214 = (((_e3206 >> bitcast<u32>((((_e3206 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3206) * 277803737u);
                                                    let _e3217 = ((_e3214 >> bitcast<u32>(22i)) ^ _e3214);
                                                    let _e3223 = (((f32((_e3217 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2705) * 0.15915494f);
                                                    let _e3224 = (1f - _e3223);
                                                    let _e3226 = (_e3223 * (2f - _e3223));
                                                    if (_e3226 != _e3226) {
                                                        phi_27676_ = true;
                                                    } else {
                                                        phi_27676_ = (0f >= _e3226);
                                                    }
                                                    let _e3230 = phi_27676_;
                                                    let _e3232 = sqrt(select(_e3226, 0f, _e3230));
                                                    let _e3234 = ((_e3217 * 747796405u) + 2891336453u);
                                                    let _e3242 = (((_e3234 >> bitcast<u32>((((_e3234 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3234) * 277803737u);
                                                    let _e3245 = ((_e3242 >> bitcast<u32>(22i)) ^ _e3242);
                                                    let _e3249 = (f32((_e3245 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                    let _e3251 = select(-1f, 1f, (_e2693.z >= 0f));
                                                    let _e3253 = (-1f / (_e3251 + _e2693.z));
                                                    let _e3255 = ((_e2693.x * _e2693.y) * _e3253);
                                                    let _e3268 = (_e3232 * cos(_e3249));
                                                    let _e3273 = (_e3232 * sin(_e3249));
                                                    let _e3283 = ((((1f + (((_e3251 * _e2693.x) * _e2693.x) * _e3253)) * _e3268) + (_e3255 * _e3273)) + (_e2693.x * _e3224));
                                                    let _e3284 = ((((_e3251 * _e3255) * _e3268) + ((_e3251 + ((_e2693.y * _e2693.y) * _e3253)) * _e3273)) + (_e2693.y * _e3224));
                                                    let _e3285 = ((((-(_e3251) * _e2693.x) * _e3268) + (-(_e2693.y) * _e3273)) + (_e2693.z * _e3224));
                                                    let _e3287 = (_e3283 - _e2693.x);
                                                    let _e3288 = (_e3284 - _e2693.y);
                                                    let _e3289 = (_e3285 - _e2693.z);
                                                    let _e3296 = (sqrt((((_e3287 * _e3287) + (_e3288 * _e3288)) + (_e3289 * _e3289))) * 0.5f);
                                                    let _e3297 = (_e3296 != _e3296);
                                                    if _e3297 {
                                                        phi_27747_ = true;
                                                    } else {
                                                        phi_27747_ = (1f <= _e3296);
                                                    }
                                                    let _e3300 = phi_27747_;
                                                    if ((2f * asin(select(_e3296, 1f, _e3300))) >= _e2701) {
                                                        phi_15462_ = 0f;
                                                    } else {
                                                        phi_15462_ = (1f / _e2705);
                                                    }
                                                    let _e3307 = phi_15462_;
                                                    if (_e3307 > 0f) {
                                                        if _e3297 {
                                                            phi_27762_ = true;
                                                        } else {
                                                            phi_27762_ = (1f <= _e3296);
                                                        }
                                                        let _e3311 = phi_27762_;
                                                        let _e3314 = (2f * asin(select(_e3296, 1f, _e3311)));
                                                        if (_e3314 >= _e2701) {
                                                            phi_16263_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            let _e3318 = (1f - pow((_e3314 / _e2701), 2f));
                                                            if (_e3318 != _e3318) {
                                                                phi_27777_ = true;
                                                            } else {
                                                                phi_27777_ = (0f >= _e3318);
                                                            }
                                                            let _e3322 = phi_27777_;
                                                            let _e3325 = global_1.member.member_6;
                                                            let _e3331 = global_1.member.member_8[2u];
                                                            if (_e3331 == 4294967295u) {
                                                                phi_16236_ = vec3<f32>(1f, 1f, 1f);
                                                            } else {
                                                                let _e3334 = global_6.member[_e3331];
                                                                let _e3337 = global_6.member[(_e3331 + 1u)];
                                                                let _e3340 = global_6.member[(_e3331 + 2u)];
                                                                let _e3343 = global_6.member[(_e3331 + 3u)];
                                                                let _e3346 = global_6.member[(_e3331 + 4u)];
                                                                let _e3349 = global_6.member[(_e3331 + 5u)];
                                                                let _e3352 = global_6.member[(_e3331 + 6u)];
                                                                let _e3355 = global_6.member[(_e3331 + 7u)];
                                                                let _e3358 = global_6.member[(_e3331 + 8u)];
                                                                let _e3361 = global_6.member[(_e3331 + 9u)];
                                                                let _e3364 = global_6.member[(_e3331 + 13u)];
                                                                let _e3367 = global_6.member[(_e3331 + 14u)];
                                                                let _e3370 = global_6.member[(_e3331 + 15u)];
                                                                let _e3373 = global_6.member[(_e3331 + 16u)];
                                                                let _e3376 = global_6.member[(_e3331 + 18u)];
                                                                let _e3379 = global_6.member[(_e3331 + 19u)];
                                                                let _e3382 = global_6.member[(_e3331 + 20u)];
                                                                let _e3385 = global_6.member[(_e3331 + 29u)];
                                                                let _e3386 = bitcast<u32>(_e3385);
                                                                let _e3399 = (((((_e1064 * _e1064) + (_e1065 * _e1065)) + (_e1066 * _e1066)) - (2f * (((_e1064 * _e3334) + (_e1065 * _e3337)) + (_e1066 * _e3340)))) + _e3352);
                                                                if (_e3399 > 0f) {
                                                                    let _e3408 = ((((_e1064 - _e3334) * _e3355) + ((_e1065 - _e3337) * _e3358)) + ((_e1066 - _e3340) * _e3361));
                                                                    let _e3410 = ((_e3408 * _e3408) - _e3399);
                                                                    if (_e3408 >= 0f) {
                                                                        phi_15936_ = vec3<f32>();
                                                                        phi_15937_ = true;
                                                                    } else {
                                                                        let _e3412 = (_e3410 <= 0f);
                                                                        if _e3412 {
                                                                            phi_15935_ = vec3<f32>();
                                                                        } else {
                                                                            let _e3415 = (-(_e3408) - sqrt(_e3410));
                                                                            phi_15935_ = vec3<f32>((_e1064 + (_e3355 * _e3415)), (_e1065 + (_e3358 * _e3415)), (_e1066 + (_e3361 * _e3415)));
                                                                        }
                                                                        let _e3424 = phi_15935_;
                                                                        phi_15936_ = _e3424;
                                                                        phi_15937_ = _e3412;
                                                                    }
                                                                    let _e3426 = phi_15936_;
                                                                    let _e3428 = phi_15937_;
                                                                    phi_15942_ = _e3426;
                                                                    phi_15943_ = select(true, false, _e3428);
                                                                } else {
                                                                    phi_15942_ = _e1067;
                                                                    phi_15943_ = true;
                                                                }
                                                                let _e3431 = phi_15942_;
                                                                let _e3433 = phi_15943_;
                                                                if _e3433 {
                                                                    let _e3435 = (_e3431.x - _e3334);
                                                                    let _e3437 = (_e3431.y - _e3337);
                                                                    let _e3439 = (_e3431.z - _e3340);
                                                                    let _e3445 = sqrt((((_e3435 * _e3435) + (_e3437 * _e3437)) + (_e3439 * _e3439)));
                                                                    let _e3446 = (1f / _e3445);
                                                                    let _e3464 = ((((((_e3431.x * _e3431.x) + (_e3431.y * _e3431.y)) + (_e3431.z * _e3431.z)) - (2f * (((_e3431.x * _e3334) + (_e3431.y * _e3337)) + (_e3431.z * _e3340)))) + _e3349) / (_e3445 + _e3343));
                                                                    if (_e3464 != _e3464) {
                                                                        phi_27901_ = true;
                                                                    } else {
                                                                        phi_27901_ = (0f >= _e3464);
                                                                    }
                                                                    let _e3468 = phi_27901_;
                                                                    let _e3472 = sqrt((select(_e3464, 0f, _e3468) / (_e3346 - _e3343)));
                                                                    if (_e3472 != _e3472) {
                                                                        phi_27916_ = true;
                                                                    } else {
                                                                        phi_27916_ = (1f <= _e3472);
                                                                    }
                                                                    let _e3476 = phi_27916_;
                                                                    let _e3478 = (select(_e3472, 1f, _e3476) * 63f);
                                                                    let _e3483 = ((((_e3435 * _e3446) * _e3355) + ((_e3437 * _e3446) * _e3358)) + ((_e3439 * _e3446) * _e3361));
                                                                    let _e3485 = select(_e3483, -1f, (_e3483 < -1f));
                                                                    let _e3489 = ((select(_e3485, 1f, (_e3485 > 1f)) + 1f) * 127.5f);
                                                                    let _e3494 = select(select(u32(_e3478), 0u, (_e3478 < 0f)), 4294967295u, (_e3478 > 4294967000f));
                                                                    let _e3496 = select(_e3494, 62u, (62u < _e3494));
                                                                    let _e3501 = select(select(u32(_e3489), 0u, (_e3489 < 0f)), 4294967295u, (_e3489 > 4294967000f));
                                                                    let _e3503 = select(_e3501, 254u, (254u < _e3501));
                                                                    let _e3505 = (_e3478 - f32(_e3496));
                                                                    let _e3507 = (_e3489 - f32(_e3503));
                                                                    let _e3511 = (_e3386 + (3u * ((_e3496 * 256u) + _e3503)));
                                                                    let _e3516 = (_e3386 + (3u * (((_e3496 + 1u) * 256u) + _e3503)));
                                                                    let _e3518 = global_6.member[_e3511];
                                                                    let _e3521 = global_6.member[(_e3511 + 1u)];
                                                                    let _e3524 = global_6.member[(_e3511 + 2u)];
                                                                    let _e3525 = (1f - _e3507);
                                                                    let _e3531 = global_6.member[(_e3511 + 3u)];
                                                                    let _e3534 = global_6.member[(_e3511 + 4u)];
                                                                    let _e3537 = global_6.member[(_e3511 + 5u)];
                                                                    let _e3544 = (1f - _e3505);
                                                                    let _e3549 = global_6.member[_e3516];
                                                                    let _e3552 = global_6.member[(_e3516 + 1u)];
                                                                    let _e3555 = global_6.member[(_e3516 + 2u)];
                                                                    let _e3561 = global_6.member[(_e3516 + 3u)];
                                                                    let _e3564 = global_6.member[(_e3516 + 4u)];
                                                                    let _e3567 = global_6.member[(_e3516 + 5u)];
                                                                    phi_16234_ = vec3<f32>(((((_e3518 * _e3525) + (_e3531 * _e3507)) * _e3544) + (((_e3549 * _e3525) + (_e3561 * _e3507)) * _e3505)), ((((_e3521 * _e3525) + (_e3534 * _e3507)) * _e3544) + (((_e3552 * _e3525) + (_e3564 * _e3507)) * _e3505)), ((((_e3524 * _e3525) + (_e3537 * _e3507)) * _e3544) + (((_e3555 * _e3525) + (_e3567 * _e3507)) * _e3505)));
                                                                } else {
                                                                    phi_16234_ = vec3<f32>(0f, 0f, 0f);
                                                                }
                                                                let _e3582 = phi_16234_;
                                                                let _e3589 = ((_e3373 * 1.1f) * _e3582.y);
                                                                phi_16236_ = vec3<f32>(exp(-((((_e3364 * _e3582.x) + _e3589) + (_e3376 * _e3582.z)))), exp(-((((_e3367 * _e3582.x) + _e3589) + (_e3379 * _e3582.z)))), exp(-((((_e3370 * _e3582.x) + _e3589) + (_e3382 * _e3582.z)))));
                                                            }
                                                            let _e3608 = phi_16236_;
                                                            let _e3619 = (((1f - (0.6f * (1f - sqrt(select(_e3318, 0f, _e3322))))) * 1.25f) / _e2705);
                                                            phi_16263_ = vec3<f32>(((_e3325.x * _e3608.x) * _e3619), ((_e3325.y * _e3608.y) * _e3619), ((_e3325.z * _e3608.z) * _e3619));
                                                        }
                                                        let _e3625 = phi_16263_;
                                                        phi_16264_ = _e3625;
                                                    } else {
                                                        phi_16264_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e3627 = phi_16264_;
                                                    phi_35458_ = _e3245;
                                                    phi_16265_ = _e3307;
                                                    phi_16266_ = vec3<f32>(_e3283, _e3284, _e3285);
                                                    phi_16267_ = _e3627;
                                                }
                                                let _e3915 = phi_35458_;
                                                let _e3917 = phi_16265_;
                                                let _e3919 = phi_16266_;
                                                let _e3921 = phi_16267_;
                                                phi_35457_ = _e3915;
                                                phi_16268_ = _e3917;
                                                phi_16269_ = _e3919;
                                                phi_16270_ = _e3921;
                                            }
                                            let _e3923 = phi_35457_;
                                            let _e3925 = phi_16268_;
                                            let _e3927 = phi_16269_;
                                            let _e3929 = phi_16270_;
                                            phi_35414_ = _e3923;
                                            phi_17160_ = _e3925;
                                            phi_17161_ = _e3927;
                                            phi_17162_ = _e3929;
                                            phi_17163_ = 1000000000000000000000000000000f;
                                        } else {
                                            if (_e1952 == (_e1942 + 1u)) {
                                                if ((_e376 & 1u) == 0u) {
                                                    phi_35423_ = _e1946;
                                                    phi_16794_ = 0f;
                                                    phi_16795_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16796_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16797_ = 1000000000000000000000000000000f;
                                                } else {
                                                    if (_e379 > 0u) {
                                                        let _e2183 = global_1.member.member_2[1u];
                                                        let _e2185 = ((_e1946 * 747796405u) + 2891336453u);
                                                        let _e2193 = (((_e2185 >> bitcast<u32>((((_e2185 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2185) * 277803737u);
                                                        let _e2196 = ((_e2193 >> bitcast<u32>(22i)) ^ _e2193);
                                                        phi_16291_ = 0u;
                                                        phi_16294_ = _e379;
                                                        loop {
                                                            let _e2202 = phi_16291_;
                                                            let _e2204 = phi_16294_;
                                                            local_78 = _e2202;
                                                            local_79 = _e2202;
                                                            let _e2206 = ((_e2202 + 1u) < _e2204);
                                                            if _e2206 {
                                                                let _e2208 = ((_e2202 + _e2204) / 2u);
                                                                let _e2211 = global_6.member[(_e2183 + _e2208)];
                                                                let _e2212 = (_e2211 <= (f32((_e2196 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                phi_16292_ = select(_e2202, _e2208, _e2212);
                                                                phi_16295_ = select(_e2208, _e2204, _e2212);
                                                            } else {
                                                                phi_16292_ = u32();
                                                                phi_16295_ = u32();
                                                            }
                                                            let _e2216 = phi_16292_;
                                                            let _e2218 = phi_16295_;
                                                            phi_16291_ = _e2216;
                                                            phi_16294_ = _e2218;
                                                            if !(_e2206) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2221 = local_78;
                                                        let _e2222 = (_e2183 + _e2221);
                                                        let _e2225 = global_6.member[(_e2222 + 1u)];
                                                        let _e2227 = global_6.member[_e2222];
                                                        let _e2231 = global_1.member.member_2[0u];
                                                        let _e2233 = local_79;
                                                        let _e2236 = global_6.member[(_e2231 + _e2233)];
                                                        let _e2239 = ((_e2196 * 747796405u) + 2891336453u);
                                                        let _e2247 = (((_e2239 >> bitcast<u32>((((_e2239 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2239) * 277803737u);
                                                        let _e2250 = ((_e2247 >> bitcast<u32>(22i)) ^ _e2247);
                                                        let _e2253 = f32((_e2250 >> bitcast<u32>(8i)));
                                                        let _e2254 = (_e2253 * 0.000000059604645f);
                                                        let _e2256 = ((_e2250 * 747796405u) + 2891336453u);
                                                        let _e2264 = (((_e2256 >> bitcast<u32>((((_e2256 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2256) * 277803737u);
                                                        let _e2267 = ((_e2264 >> bitcast<u32>(22i)) ^ _e2264);
                                                        let _e2270 = f32((_e2267 >> bitcast<u32>(8i)));
                                                        let _e2273 = global_2.member[bitcast<u32>(_e2236)];
                                                        let _e2276 = (_e2273.member_6.x == 1u);
                                                        if _e2276 {
                                                            let _e2418 = (_e2273.member.x - _e1064);
                                                            let _e2419 = (_e2273.member.y - _e1065);
                                                            let _e2420 = (_e2273.member.z - _e1066);
                                                            let _e2425 = (((_e2418 * _e2418) + (_e2419 * _e2419)) + (_e2420 * _e2420));
                                                            let _e2426 = (_e2273.member.w * _e2273.member.w);
                                                            let _e2428 = (_e2425 <= (_e2426 * 1.000001f));
                                                            if _e2428 {
                                                                let _e2537 = (_e2253 * 0.00000037450704f);
                                                                let _e2539 = ((_e2270 * 0.00000011920929f) - 1f);
                                                                let _e2541 = (1f - (_e2539 * _e2539));
                                                                if (_e2541 != _e2541) {
                                                                    phi_26744_ = true;
                                                                } else {
                                                                    phi_26744_ = (0f >= _e2541);
                                                                }
                                                                let _e2545 = phi_26744_;
                                                                let _e2547 = sqrt(select(_e2541, 0f, _e2545));
                                                                let _e2549 = (_e2547 * cos(_e2537));
                                                                let _e2551 = (_e2547 * sin(_e2537));
                                                                let _e2556 = (_e2273.member.x + (_e2549 * _e2273.member.w));
                                                                let _e2557 = (_e2273.member.y + (_e2551 * _e2273.member.w));
                                                                let _e2558 = (_e2273.member.z + (_e2539 * _e2273.member.w));
                                                                if _e2428 {
                                                                    let _e2571 = (_e2556 - _e1064);
                                                                    let _e2572 = (_e2557 - _e1065);
                                                                    let _e2573 = (_e2558 - _e1066);
                                                                    let _e2578 = (((_e2571 * _e2571) + (_e2572 * _e2572)) + (_e2573 * _e2573));
                                                                    phi_26813_ = (_e2578 / (((abs((((_e2549 * _e2571) + (_e2551 * _e2572)) + (_e2539 * _e2573))) / sqrt(_e2578)) * 12.566371f) * _e2426));
                                                                } else {
                                                                    let _e2560 = (_e2426 / _e2425);
                                                                    if (_e2560 != _e2560) {
                                                                        phi_26832_ = true;
                                                                    } else {
                                                                        phi_26832_ = (1f <= _e2560);
                                                                    }
                                                                    let _e2564 = phi_26832_;
                                                                    let _e2565 = select(_e2560, 1f, _e2564);
                                                                    phi_26813_ = (0.15915494f / (_e2565 / (1f + sqrt((1f - _e2565)))));
                                                                }
                                                                let _e2591 = phi_26813_;
                                                                phi_16507_ = type_74(vec3<f32>(_e2556, _e2557, _e2558), vec3<f32>(_e2549, _e2551, _e2539), _e2591);
                                                            } else {
                                                                let _e2429 = (_e2426 / _e2425);
                                                                let _e2430 = (_e2429 != _e2429);
                                                                if _e2430 {
                                                                    phi_26596_ = true;
                                                                } else {
                                                                    phi_26596_ = (1f <= _e2429);
                                                                }
                                                                let _e2433 = phi_26596_;
                                                                let _e2434 = select(_e2429, 1f, _e2433);
                                                                if _e2430 {
                                                                    phi_26619_ = true;
                                                                } else {
                                                                    phi_26619_ = (1f <= _e2429);
                                                                }
                                                                let _e2438 = phi_26619_;
                                                                let _e2439 = select(_e2429, 1f, _e2438);
                                                                let _e2443 = (_e2439 / (1f + sqrt((1f - _e2439))));
                                                                if (_e2434 < 0.00068523f) {
                                                                    phi_16442_ = (_e2434 * _e2254);
                                                                } else {
                                                                    let _e2446 = (1f - (_e2254 * _e2443));
                                                                    let _e2448 = (1f - (_e2446 * _e2446));
                                                                    if (_e2448 != _e2448) {
                                                                        phi_26634_ = true;
                                                                    } else {
                                                                        phi_26634_ = (0f >= _e2448);
                                                                    }
                                                                    let _e2452 = phi_26634_;
                                                                    phi_16442_ = select(_e2448, 0f, _e2452);
                                                                }
                                                                let _e2456 = phi_16442_;
                                                                let _e2457 = (1f - _e2456);
                                                                if (_e2457 != _e2457) {
                                                                    phi_26649_ = true;
                                                                } else {
                                                                    phi_26649_ = (0f >= _e2457);
                                                                }
                                                                let _e2461 = phi_26649_;
                                                                let _e2466 = (1f - (_e2456 / _e2434));
                                                                if (_e2466 != _e2466) {
                                                                    phi_26664_ = true;
                                                                } else {
                                                                    phi_26664_ = (0f >= _e2466);
                                                                }
                                                                let _e2470 = phi_26664_;
                                                                let _e2474 = ((_e2456 / sqrt(_e2434)) + (sqrt(select(_e2457, 0f, _e2461)) * sqrt(select(_e2466, 0f, _e2470))));
                                                                let _e2476 = (1f - (_e2474 * _e2474));
                                                                if (_e2476 != _e2476) {
                                                                    phi_26679_ = true;
                                                                } else {
                                                                    phi_26679_ = (0f >= _e2476);
                                                                }
                                                                let _e2480 = phi_26679_;
                                                                let _e2482 = sqrt(select(_e2476, 0f, _e2480));
                                                                let _e2483 = (_e2270 * 0.00000037450704f);
                                                                let _e2484 = sqrt(_e2425);
                                                                let _e2485 = (_e2418 / _e2484);
                                                                let _e2486 = (_e2419 / _e2484);
                                                                let _e2487 = (_e2420 / _e2484);
                                                                let _e2489 = select(-1f, 1f, (_e2487 >= 0f));
                                                                let _e2491 = (-1f / (_e2489 + _e2487));
                                                                let _e2493 = ((_e2485 * _e2486) * _e2491);
                                                                let _e2506 = (_e2482 * cos(_e2483));
                                                                let _e2511 = (_e2482 * sin(_e2483));
                                                                let _e2524 = -(((((1f + (((_e2489 * _e2485) * _e2485) * _e2491)) * _e2506) + (_e2493 * _e2511)) + (_e2485 * _e2474)));
                                                                let _e2525 = -(((((_e2489 * _e2493) * _e2506) + ((_e2489 + ((_e2486 * _e2486) * _e2491)) * _e2511)) + (_e2486 * _e2474)));
                                                                let _e2526 = -(((((-(_e2489) * _e2485) * _e2506) + (-(_e2486) * _e2511)) + (_e2487 * _e2474)));
                                                                phi_16507_ = type_74(vec3<f32>((_e2273.member.x + (_e2524 * _e2273.member.w)), (_e2273.member.y + (_e2525 * _e2273.member.w)), (_e2273.member.z + (_e2526 * _e2273.member.w))), vec3<f32>(_e2524, _e2525, _e2526), (0.15915494f / _e2443));
                                                            }
                                                            let _e2594 = phi_16507_;
                                                            phi_16693_ = _e2594;
                                                        } else {
                                                            let _e2277 = sqrt(_e2254);
                                                            let _e2278 = (1f - _e2277);
                                                            let _e2279 = ((_e2270 * 0.000000059604645f) * _e2277);
                                                            let _e2301 = (_e2277 - _e2279);
                                                            let _e2305 = (((_e2273.member.x * _e2278) + (_e2273.member_1.x * _e2279)) + (_e2273.member_2.x * _e2301));
                                                            let _e2306 = (((_e2273.member.y * _e2278) + (_e2273.member_1.y * _e2279)) + (_e2273.member_2.y * _e2301));
                                                            let _e2307 = (((_e2273.member.z * _e2278) + (_e2273.member_1.z * _e2279)) + (_e2273.member_2.z * _e2301));
                                                            let _e2309 = (_e2273.member_1.x - _e2273.member.x);
                                                            let _e2310 = (_e2273.member_1.y - _e2273.member.y);
                                                            let _e2311 = (_e2273.member_1.z - _e2273.member.z);
                                                            let _e2312 = (_e2273.member_2.x - _e2273.member.x);
                                                            let _e2313 = (_e2273.member_2.y - _e2273.member.y);
                                                            let _e2314 = (_e2273.member_2.z - _e2273.member.z);
                                                            let _e2317 = ((_e2310 * _e2314) - (_e2313 * _e2311));
                                                            let _e2320 = ((_e2311 * _e2312) - (_e2314 * _e2309));
                                                            let _e2323 = ((_e2309 * _e2313) - (_e2312 * _e2310));
                                                            let _e2329 = sqrt((((_e2317 * _e2317) + (_e2320 * _e2320)) + (_e2323 * _e2323)));
                                                            let _e2330 = (1f / _e2329);
                                                            let _e2331 = (_e2317 * _e2330);
                                                            let _e2332 = (_e2320 * _e2330);
                                                            let _e2333 = (_e2323 * _e2330);
                                                            if _e2276 {
                                                                let _e2366 = (_e2273.member.x - _e1064);
                                                                let _e2367 = (_e2273.member.y - _e1065);
                                                                let _e2368 = (_e2273.member.z - _e1066);
                                                                let _e2373 = (((_e2366 * _e2366) + (_e2367 * _e2367)) + (_e2368 * _e2368));
                                                                let _e2374 = (_e2273.member.w * _e2273.member.w);
                                                                if (_e2373 <= (_e2374 * 1.000001f)) {
                                                                    let _e2388 = (_e2305 - _e1064);
                                                                    let _e2389 = (_e2306 - _e1065);
                                                                    let _e2390 = (_e2307 - _e1066);
                                                                    let _e2395 = (((_e2388 * _e2388) + (_e2389 * _e2389)) + (_e2390 * _e2390));
                                                                    phi_26562_ = (_e2395 / (((abs((((_e2331 * _e2388) + (_e2332 * _e2389)) + (_e2333 * _e2390))) / sqrt(_e2395)) * 12.566371f) * _e2374));
                                                                } else {
                                                                    let _e2377 = (_e2374 / _e2373);
                                                                    if (_e2377 != _e2377) {
                                                                        phi_26581_ = true;
                                                                    } else {
                                                                        phi_26581_ = (1f <= _e2377);
                                                                    }
                                                                    let _e2381 = phi_26581_;
                                                                    let _e2382 = select(_e2377, 1f, _e2381);
                                                                    phi_26562_ = (0.15915494f / (_e2382 / (1f + sqrt((1f - _e2382)))));
                                                                }
                                                                let _e2408 = phi_26562_;
                                                                phi_16691_ = _e2408;
                                                            } else {
                                                                let _e2335 = (_e2305 - _e1064);
                                                                let _e2336 = (_e2306 - _e1065);
                                                                let _e2337 = (_e2307 - _e1066);
                                                                let _e2342 = (((_e2335 * _e2335) + (_e2336 * _e2336)) + (_e2337 * _e2337));
                                                                let _e2350 = (abs((((_e2331 * _e2335) + (_e2332 * _e2336)) + (_e2333 * _e2337))) / sqrt(_e2342));
                                                                if (_e2350 <= 0f) {
                                                                    phi_16690_ = 0f;
                                                                } else {
                                                                    if (_e2273.member_6.x == 0u) {
                                                                        phi_16687_ = (_e2329 * 0.5f);
                                                                    } else {
                                                                        phi_16687_ = ((12.566371f * _e2273.member.w) * _e2273.member.w);
                                                                    }
                                                                    let _e2359 = phi_16687_;
                                                                    phi_16690_ = (_e2342 / (_e2350 * _e2359));
                                                                }
                                                                let _e2363 = phi_16690_;
                                                                phi_16691_ = _e2363;
                                                            }
                                                            let _e2410 = phi_16691_;
                                                            phi_16693_ = type_74(vec3<f32>(_e2305, _e2306, _e2307), vec3<f32>(_e2331, _e2332, _e2333), _e2410);
                                                        }
                                                        let _e2596 = phi_16693_;
                                                        let _e2601 = (_e2596.member.x - _e1064);
                                                        let _e2603 = (_e2596.member.y - _e1065);
                                                        let _e2605 = (_e2596.member.z - _e1066);
                                                        let _e2611 = sqrt((((_e2601 * _e2601) + (_e2603 * _e2603)) + (_e2605 * _e2605)));
                                                        let _e2612 = (_e2601 / _e2611);
                                                        let _e2613 = (_e2603 / _e2611);
                                                        let _e2614 = (_e2605 / _e2611);
                                                        if (_e2596.member_2 > 0f) {
                                                            let _e2626 = global_4.member[_e2273.member_6.y];
                                                            if (_e2626.member == 4u) {
                                                                if (_e2626.member_1 == 1u) {
                                                                    phi_16771_ = true;
                                                                } else {
                                                                    phi_16771_ = ((((-(_e2612) * _e2596.member_1.x) + (-(_e2613) * _e2596.member_1.y)) + (-(_e2614) * _e2596.member_1.z)) > 0f);
                                                                }
                                                                let _e2641 = phi_16771_;
                                                                if _e2641 {
                                                                    phi_16780_ = vec3<f32>(_e2626.member_4.x, _e2626.member_4.y, _e2626.member_4.z);
                                                                } else {
                                                                    phi_16780_ = vec3<f32>();
                                                                }
                                                                let _e2648 = phi_16780_;
                                                                phi_16782_ = _e2648;
                                                                phi_16783_ = select(true, false, _e2641);
                                                            } else {
                                                                phi_16782_ = vec3<f32>();
                                                                phi_16783_ = true;
                                                            }
                                                            let _e2651 = phi_16782_;
                                                            let _e2653 = phi_16783_;
                                                            phi_16788_ = ((_e2225 - _e2227) * _e2596.member_2);
                                                            phi_16789_ = select(_e2651, vec3<f32>(0f, 0f, 0f), vec3(_e2653));
                                                        } else {
                                                            phi_16788_ = 0f;
                                                            phi_16789_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2657 = phi_16788_;
                                                        let _e2659 = phi_16789_;
                                                        phi_35424_ = _e2267;
                                                        phi_16790_ = _e2657;
                                                        phi_16791_ = vec3<f32>(_e2612, _e2613, _e2614);
                                                        phi_16792_ = _e2659;
                                                        phi_16793_ = ((_e2611 * 0.9999f) - _e1057);
                                                    } else {
                                                        phi_35424_ = _e1946;
                                                        phi_16790_ = 0f;
                                                        phi_16791_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_16792_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_16793_ = 1000000000000000000000000000000f;
                                                    }
                                                    let _e2661 = phi_35424_;
                                                    let _e2663 = phi_16790_;
                                                    let _e2665 = phi_16791_;
                                                    let _e2667 = phi_16792_;
                                                    let _e2669 = phi_16793_;
                                                    phi_35423_ = _e2661;
                                                    phi_16794_ = _e2663;
                                                    phi_16795_ = _e2665;
                                                    phi_16796_ = _e2667;
                                                    phi_16797_ = _e2669;
                                                }
                                                let _e2671 = phi_35423_;
                                                let _e2673 = phi_16794_;
                                                let _e2675 = phi_16795_;
                                                let _e2677 = phi_16796_;
                                                let _e2679 = phi_16797_;
                                                phi_35415_ = _e2671;
                                                phi_17156_ = _e2673;
                                                phi_17157_ = _e2675;
                                                phi_17158_ = _e2677;
                                                phi_17159_ = _e2679;
                                            } else {
                                                if ((_e376 & 2u) == 0u) {
                                                    phi_35416_ = _e1946;
                                                    phi_17153_ = 0f;
                                                    phi_17154_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17155_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e1964 = ((_e1946 * 747796405u) + 2891336453u);
                                                    let _e1972 = (((_e1964 >> bitcast<u32>((((_e1964 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1964) * 277803737u);
                                                    let _e1975 = ((_e1972 >> bitcast<u32>(22i)) ^ _e1972);
                                                    let _e1979 = (f32((_e1975 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                    let _e1981 = ((_e1975 * 747796405u) + 2891336453u);
                                                    let _e1989 = (((_e1981 >> bitcast<u32>((((_e1981 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1981) * 277803737u);
                                                    let _e1992 = ((_e1989 >> bitcast<u32>(22i)) ^ _e1989);
                                                    let _e1996 = (f32((_e1992 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                    let _e1999 = global_1.member.member_3[1u];
                                                    let _e2002 = global_1.member.member_3[2u];
                                                    let _e2005 = global_1.member.member_2[3u];
                                                    phi_16818_ = 0u;
                                                    phi_16821_ = _e2002;
                                                    loop {
                                                        let _e2007 = phi_16818_;
                                                        let _e2009 = phi_16821_;
                                                        local_73 = _e2007;
                                                        local_76 = _e2007;
                                                        local_77 = _e2007;
                                                        let _e2011 = ((_e2007 + 1u) < _e2009);
                                                        if _e2011 {
                                                            let _e2013 = ((_e2007 + _e2009) / 2u);
                                                            let _e2016 = global_6.member[(_e2005 + _e2013)];
                                                            let _e2017 = (_e2016 <= _e1979);
                                                            phi_16819_ = select(_e2007, _e2013, _e2017);
                                                            phi_16822_ = select(_e2013, _e2009, _e2017);
                                                        } else {
                                                            phi_16819_ = u32();
                                                            phi_16822_ = u32();
                                                        }
                                                        let _e2021 = phi_16819_;
                                                        let _e2023 = phi_16822_;
                                                        phi_16818_ = _e2021;
                                                        phi_16821_ = _e2023;
                                                        if !(_e2011) {
                                                            break;
                                                        }
                                                    }
                                                    let _e2027 = global_1.member.member_3[0u];
                                                    let _e2028 = (_e1999 + 1u);
                                                    let _e2030 = local_73;
                                                    let _e2032 = (_e2027 + (_e2030 * _e2028));
                                                    phi_16859_ = 0u;
                                                    phi_16862_ = _e1999;
                                                    loop {
                                                        let _e2034 = phi_16859_;
                                                        let _e2036 = phi_16862_;
                                                        local_74 = _e2034;
                                                        local_75 = _e2034;
                                                        let _e2038 = ((_e2034 + 1u) < _e2036);
                                                        if _e2038 {
                                                            let _e2040 = ((_e2034 + _e2036) / 2u);
                                                            let _e2043 = global_6.member[(_e2032 + _e2040)];
                                                            let _e2044 = (_e2043 <= _e1996);
                                                            phi_16860_ = select(_e2034, _e2040, _e2044);
                                                            phi_16863_ = select(_e2040, _e2036, _e2044);
                                                        } else {
                                                            phi_16860_ = u32();
                                                            phi_16863_ = u32();
                                                        }
                                                        let _e2048 = phi_16860_;
                                                        let _e2050 = phi_16863_;
                                                        phi_16859_ = _e2048;
                                                        phi_16862_ = _e2050;
                                                        if !(_e2038) {
                                                            break;
                                                        }
                                                    }
                                                    let _e2053 = local_74;
                                                    let _e2056 = local_75;
                                                    let _e2057 = (_e2032 + _e2056);
                                                    let _e2059 = global_6.member[_e2057];
                                                    let _e2062 = global_6.member[(_e2057 + 1u)];
                                                    if (_e2062 > _e2059) {
                                                        let _e2066 = ((_e1996 - _e2059) / (_e2062 - _e2059));
                                                        let _e2068 = select(_e2066, 0f, (_e2066 < 0f));
                                                        phi_16939_ = select(_e2068, 0.99999f, (_e2068 > 0.99999f));
                                                    } else {
                                                        phi_16939_ = 0.5f;
                                                    }
                                                    let _e2072 = phi_16939_;
                                                    let _e2074 = f32(_e1999);
                                                    let _e2077 = local_76;
                                                    let _e2080 = local_77;
                                                    let _e2081 = (_e2005 + _e2080);
                                                    let _e2083 = global_6.member[_e2081];
                                                    let _e2086 = global_6.member[(_e2081 + 1u)];
                                                    if (_e2086 > _e2083) {
                                                        let _e2090 = ((_e1979 - _e2083) / (_e2086 - _e2083));
                                                        let _e2092 = select(_e2090, 0f, (_e2090 < 0f));
                                                        phi_16988_ = select(_e2092, 0.99999f, (_e2092 > 0.99999f));
                                                    } else {
                                                        phi_16988_ = 0.5f;
                                                    }
                                                    let _e2096 = phi_16988_;
                                                    let _e2098 = f32(_e2002);
                                                    let _e2101 = ((((f32(_e2053) + _e2072) / _e2074) - 0.5f) * 6.2831855f);
                                                    let _e2102 = (((f32(_e2077) + _e2096) / _e2098) * 3.1415927f);
                                                    let _e2103 = sin(_e2102);
                                                    let _e2105 = (_e2103 * cos(_e2101));
                                                    let _e2106 = cos(_e2102);
                                                    let _e2108 = (_e2103 * sin(_e2101));
                                                    let _e2111 = ((atan2(_e2108, _e2105) * 0.15915494f) + 0.5f);
                                                    let _e2115 = select(_e2106, -1f, (_e2106 < -1f));
                                                    let _e2118 = acos(select(_e2115, 1f, (_e2115 > 1f)));
                                                    let _e2120 = ((_e2111 - floor(_e2111)) * _e2074);
                                                    let _e2125 = select(select(u32(_e2120), 0u, (_e2120 < 0f)), 4294967295u, (_e2120 > 4294967000f));
                                                    let _e2126 = (_e1999 - 1u);
                                                    let _e2129 = ((_e2118 * 0.31830987f) * _e2098);
                                                    let _e2134 = select(select(u32(_e2129), 0u, (_e2129 < 0f)), 4294967295u, (_e2129 > 4294967000f));
                                                    let _e2135 = (_e2002 - 1u);
                                                    let _e2137 = select(_e2134, _e2135, (_e2135 < _e2134));
                                                    let _e2138 = sin(_e2118);
                                                    if (_e2138 <= 0f) {
                                                        phi_17144_ = 0f;
                                                    } else {
                                                        let _e2142 = (_e2005 + _e2137);
                                                        let _e2145 = global_6.member[(_e2142 + 1u)];
                                                        let _e2147 = global_6.member[_e2142];
                                                        let _e2150 = ((_e2027 + (_e2137 * _e2028)) + select(_e2125, _e2126, (_e2126 < _e2125)));
                                                        let _e2153 = global_6.member[(_e2150 + 1u)];
                                                        let _e2155 = global_6.member[_e2150];
                                                        phi_17144_ = ((((_e2145 - _e2147) * _e2098) * ((_e2153 - _e2155) * _e2074)) / (19.73921f * _e2138));
                                                    }
                                                    let _e2162 = phi_17144_;
                                                    let _e2163 = (_e2162 > 0f);
                                                    if _e2163 {
                                                        phi_17151_ = vec3<f32>(_e2105, _e2106, _e2108);
                                                    } else {
                                                        phi_17151_ = vec3<f32>(0f, 0f, 0f);
                                                    }
                                                    let _e2166 = phi_17151_;
                                                    phi_35416_ = _e1992;
                                                    phi_17153_ = select(0f, _e2162, _e2163);
                                                    phi_17154_ = _e2166;
                                                    phi_17155_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2163));
                                                }
                                                let _e2171 = phi_35416_;
                                                let _e2173 = phi_17153_;
                                                let _e2175 = phi_17154_;
                                                let _e2177 = phi_17155_;
                                                phi_35415_ = _e2171;
                                                phi_17156_ = _e2173;
                                                phi_17157_ = _e2175;
                                                phi_17158_ = _e2177;
                                                phi_17159_ = 1000000000000000000000000000000f;
                                            }
                                            let _e2681 = phi_35415_;
                                            let _e2683 = phi_17156_;
                                            let _e2685 = phi_17157_;
                                            let _e2687 = phi_17158_;
                                            let _e2689 = phi_17159_;
                                            phi_35414_ = _e2681;
                                            phi_17160_ = _e2683;
                                            phi_17161_ = _e2685;
                                            phi_17162_ = _e2687;
                                            phi_17163_ = _e2689;
                                        }
                                        let _e3931 = phi_35414_;
                                        let _e3933 = phi_17160_;
                                        let _e3935 = phi_17161_;
                                        let _e3937 = phi_17162_;
                                        let _e3939 = phi_17163_;
                                        phi_35413_ = _e3931;
                                        phi_17164_ = _e3933;
                                        phi_17165_ = _e3935;
                                        phi_17166_ = _e3937;
                                        phi_17167_ = _e3939;
                                    }
                                    let _e4008 = phi_35413_;
                                    let _e4010 = phi_17164_;
                                    let _e4012 = phi_17165_;
                                    let _e4014 = phi_17166_;
                                    let _e4016 = phi_17167_;
                                    if ((((_e4014.x * _e4014.x) + (_e4014.y * _e4014.y)) + (_e4014.z * _e4014.z)) > 0f) {
                                        let _e4033 = (((_e4012.x * _e1039.x) + (_e4012.y * _e1039.y)) + (_e4012.z * _e1039.z));
                                        if (_e4033 != _e4033) {
                                            phi_28146_ = true;
                                        } else {
                                            phi_28146_ = (0f >= _e4033);
                                        }
                                        let _e4037 = phi_28146_;
                                        let _e4039 = (select(_e4033, 0f, _e4037) * 0.31830987f);
                                        if (_e1931 == 0u) {
                                            if (_e4033 <= 0f) {
                                                phi_28943_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_28943_ = (_e1925 * (_e4033 * 0.31830987f));
                                            }
                                            let _e4515 = phi_28943_;
                                            phi_17563_ = type_60(_e4515, _e4039);
                                            phi_17564_ = type_60();
                                            phi_17565_ = true;
                                        } else {
                                            let _e4045 = (((_e1013 * _e1087) + (_e1014 * _e1088)) + (_e1015 * _e1090));
                                            let _e4050 = (((_e1013 * _e1083) + (_e1014 * _e1093)) + (_e1015 * _e1094));
                                            let _e4055 = (((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z));
                                            let _e4060 = (((_e4012.x * _e1087) + (_e4012.y * _e1088)) + (_e4012.z * _e1090));
                                            let _e4065 = (((_e4012.x * _e1083) + (_e4012.y * _e1093)) + (_e4012.z * _e1094));
                                            if (_e4055 <= 0f) {
                                                phi_17527_ = type_60();
                                                phi_17528_ = true;
                                            } else {
                                                let _e4067 = (_e4033 <= 0f);
                                                if _e4067 {
                                                    phi_17526_ = type_60();
                                                } else {
                                                    let _e4068 = (_e4045 + _e4060);
                                                    let _e4069 = (_e4050 + _e4065);
                                                    let _e4070 = (_e4055 + _e4033);
                                                    let _e4077 = (1f / sqrt((((_e4068 * _e4068) + (_e4069 * _e4069)) + (_e4070 * _e4070))));
                                                    let _e4080 = (_e4070 * _e4077);
                                                    if (_e4080 <= 0f) {
                                                        phi_28256_ = 0f;
                                                    } else {
                                                        let _e4082 = (_e1929 * _e1929);
                                                        let _e4086 = ((((_e4082 - 1f) * _e4080) * _e4080) + 1f);
                                                        phi_28256_ = (_e4082 / ((3.1415927f * _e4086) * _e4086));
                                                    }
                                                    let _e4091 = phi_28256_;
                                                    let _e4092 = (_e4055 * _e4055);
                                                    if (_e4092 >= 1f) {
                                                        phi_28284_ = 0f;
                                                    } else {
                                                        phi_28284_ = ((sqrt((1f + (((_e1929 * _e1929) * ((_e4045 * _e4045) + (_e4050 * _e4050))) / _e4092))) - 1f) * 0.5f);
                                                    }
                                                    let _e4105 = phi_28284_;
                                                    let _e4106 = (1f + _e4105);
                                                    let _e4107 = (_e4033 * _e4033);
                                                    if (_e4107 >= 1f) {
                                                        phi_28312_ = 0f;
                                                    } else {
                                                        phi_28312_ = ((sqrt((1f + (((_e1929 * _e1929) * ((_e4060 * _e4060) + (_e4065 * _e4065))) / _e4107))) - 1f) * 0.5f);
                                                    }
                                                    let _e4120 = phi_28312_;
                                                    let _e4127 = (((_e4045 * (_e4068 * _e4077)) + (_e4050 * (_e4069 * _e4077))) + (_e4055 * _e4080));
                                                    if (_e1927 == 0u) {
                                                        let _e4422 = (1f - _e4127);
                                                        if (_e4422 != _e4422) {
                                                            phi_28815_ = true;
                                                        } else {
                                                            phi_28815_ = (0f >= _e4422);
                                                        }
                                                        let _e4426 = phi_28815_;
                                                        let _e4428 = pow(select(_e4422, 0f, _e4426), 5f);
                                                        let _e4429 = (1f - _e4428);
                                                        phi_17514_ = vec3<f32>(((_e1925.x * _e4429) + _e4428), ((_e1925.y * _e4429) + _e4428), ((_e1925.z * _e4429) + _e4428));
                                                    } else {
                                                        if (_e1927 == 1u) {
                                                            let _e4395 = (1f / _e1921);
                                                            let _e4399 = ((_e4395 * _e4395) * (1f - (_e4127 * _e4127)));
                                                            if (_e4399 >= 1f) {
                                                                phi_28776_ = 1f;
                                                            } else {
                                                                let _e4402 = sqrt((1f - _e4399));
                                                                if (_e4127 != _e4127) {
                                                                    phi_28787_ = true;
                                                                } else {
                                                                    phi_28787_ = (_e4402 <= _e4127);
                                                                }
                                                                let _e4406 = phi_28787_;
                                                                let _e4411 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                                                phi_28776_ = (_e4411 + ((1f - _e4411) * pow((1f - select(_e4127, _e4402, _e4406)), 5f)));
                                                            }
                                                            let _e4418 = phi_28776_;
                                                            phi_17513_ = vec3(_e4418);
                                                        } else {
                                                            let _e4131 = select(_e4127, 0f, (_e4127 < 0f));
                                                            let _e4133 = select(_e4131, 1f, (_e4131 > 1f));
                                                            let _e4135 = (1f - (_e4133 * _e4133));
                                                            let _e4148 = ((_e1925.x * _e1925.x) - (_e1923.x * _e1923.x));
                                                            let _e4149 = ((_e1925.y * _e1925.y) - (_e1923.y * _e1923.y));
                                                            let _e4150 = ((_e1925.z * _e1925.z) - (_e1923.z * _e1923.z));
                                                            let _e4152 = (_e1925.x * (_e1923.x + _e1923.x));
                                                            let _e4154 = (_e1925.y * (_e1923.y + _e1923.y));
                                                            let _e4156 = (_e1925.z * (_e1923.z + _e1923.z));
                                                            let _e4163 = ((_e4148 * _e4148) + (_e4152 * _e4152));
                                                            let _e4164 = ((_e4149 * _e4149) + (_e4154 * _e4154));
                                                            let _e4165 = ((_e4150 * _e4150) + (_e4156 * _e4156));
                                                            let _e4170 = ((_e4135 * _e4148) / _e4163);
                                                            let _e4171 = ((_e4135 * _e4149) / _e4164);
                                                            let _e4172 = ((_e4135 * _e4150) / _e4165);
                                                            let _e4173 = (1f - _e4170);
                                                            let _e4174 = (1f - _e4171);
                                                            let _e4175 = (1f - _e4172);
                                                            let _e4177 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4170, _e4171, _e4172));
                                                            let _e4178 = (_e4135 * _e4152);
                                                            let _e4179 = (_e4135 * _e4154);
                                                            let _e4180 = (_e4135 * _e4156);
                                                            let _e4185 = (vec3<f32>(_e4178, _e4179, _e4180) / vec3<f32>(_e4163, _e4164, _e4165));
                                                            let _e4198 = ((_e4177.x * _e4177.x) + (_e4185.x * _e4185.x));
                                                            let _e4199 = ((_e4177.y * _e4177.y) + (_e4185.y * _e4185.y));
                                                            let _e4200 = ((_e4177.z * _e4177.z) + (_e4185.z * _e4185.z));
                                                            if (_e4198 != _e4198) {
                                                                phi_28446_ = true;
                                                            } else {
                                                                phi_28446_ = (0f >= _e4198);
                                                            }
                                                            let _e4204 = phi_28446_;
                                                            let _e4206 = sqrt(select(_e4198, 0f, _e4204));
                                                            if (_e4199 != _e4199) {
                                                                phi_28461_ = true;
                                                            } else {
                                                                phi_28461_ = (0f >= _e4199);
                                                            }
                                                            let _e4210 = phi_28461_;
                                                            let _e4212 = sqrt(select(_e4199, 0f, _e4210));
                                                            if (_e4200 != _e4200) {
                                                                phi_28476_ = true;
                                                            } else {
                                                                phi_28476_ = (0f >= _e4200);
                                                            }
                                                            let _e4216 = phi_28476_;
                                                            let _e4218 = sqrt(select(_e4200, 0f, _e4216));
                                                            let _e4222 = ((_e4206 + _e4173) * 0.5f);
                                                            let _e4223 = ((_e4212 + _e4174) * 0.5f);
                                                            let _e4224 = ((_e4218 + _e4175) * 0.5f);
                                                            if (_e4222 != _e4222) {
                                                                phi_28503_ = true;
                                                            } else {
                                                                phi_28503_ = (0f >= _e4222);
                                                            }
                                                            let _e4228 = phi_28503_;
                                                            let _e4230 = sqrt(select(_e4222, 0f, _e4228));
                                                            if (_e4223 != _e4223) {
                                                                phi_28518_ = true;
                                                            } else {
                                                                phi_28518_ = (0f >= _e4223);
                                                            }
                                                            let _e4234 = phi_28518_;
                                                            let _e4236 = sqrt(select(_e4223, 0f, _e4234));
                                                            if (_e4224 != _e4224) {
                                                                phi_28533_ = true;
                                                            } else {
                                                                phi_28533_ = (0f >= _e4224);
                                                            }
                                                            let _e4240 = phi_28533_;
                                                            let _e4242 = sqrt(select(_e4224, 0f, _e4240));
                                                            let _e4243 = vec3<f32>(_e4230, _e4236, _e4242);
                                                            let _e4247 = ((_e4206 - _e4173) * 0.5f);
                                                            let _e4248 = ((_e4212 - _e4174) * 0.5f);
                                                            let _e4249 = ((_e4218 - _e4175) * 0.5f);
                                                            if (_e4247 != _e4247) {
                                                                phi_28560_ = true;
                                                            } else {
                                                                phi_28560_ = (0f >= _e4247);
                                                            }
                                                            let _e4253 = phi_28560_;
                                                            let _e4255 = sqrt(select(_e4247, 0f, _e4253));
                                                            if (_e4248 != _e4248) {
                                                                phi_28575_ = true;
                                                            } else {
                                                                phi_28575_ = (0f >= _e4248);
                                                            }
                                                            let _e4259 = phi_28575_;
                                                            let _e4261 = sqrt(select(_e4248, 0f, _e4259));
                                                            if (_e4249 != _e4249) {
                                                                phi_28590_ = true;
                                                            } else {
                                                                phi_28590_ = (0f >= _e4249);
                                                            }
                                                            let _e4265 = phi_28590_;
                                                            let _e4267 = sqrt(select(_e4249, 0f, _e4265));
                                                            if ((_e4178 / _e4163) < 0f) {
                                                                phi_17390_ = -(_e4255);
                                                            } else {
                                                                phi_17390_ = _e4255;
                                                            }
                                                            let _e4271 = phi_17390_;
                                                            if ((_e4179 / _e4164) < 0f) {
                                                                phi_17398_ = -(_e4261);
                                                            } else {
                                                                phi_17398_ = _e4261;
                                                            }
                                                            let _e4275 = phi_17398_;
                                                            if ((_e4180 / _e4165) < 0f) {
                                                                phi_17406_ = -(_e4267);
                                                            } else {
                                                                phi_17406_ = _e4267;
                                                            }
                                                            let _e4279 = phi_17406_;
                                                            let _e4283 = (_e1923.x * _e4133);
                                                            let _e4284 = (_e1923.y * _e4133);
                                                            let _e4285 = (_e1923.z * _e4133);
                                                            let _e4295 = vec3<f32>(((_e1925.x * _e4230) - (_e1923.x * _e4271)), ((_e1925.y * _e4236) - (_e1923.y * _e4275)), ((_e1925.z * _e4242) - (_e1923.z * _e4279)));
                                                            let _e4302 = ((_e1925.x * _e4271) + (_e1923.x * _e4230));
                                                            let _e4303 = ((_e1925.y * _e4275) + (_e1923.y * _e4236));
                                                            let _e4304 = ((_e1925.z * _e4279) + (_e1923.z * _e4242));
                                                            let _e4306 = vec3<f32>((_e1925.x * _e4133), (_e1925.y * _e4133), (_e1925.z * _e4133));
                                                            let _e4307 = (_e4306 - _e4243);
                                                            let _e4308 = (_e4283 - _e4271);
                                                            let _e4309 = (_e4284 - _e4275);
                                                            let _e4310 = (_e4285 - _e4279);
                                                            let _e4323 = (_e4306 + _e4243);
                                                            let _e4324 = (_e4283 + _e4271);
                                                            let _e4325 = (_e4284 + _e4275);
                                                            let _e4326 = (_e4285 + _e4279);
                                                            let _e4336 = ((_e4323.x * _e4323.x) + (_e4324 * _e4324));
                                                            let _e4337 = ((_e4323.y * _e4323.y) + (_e4325 * _e4325));
                                                            let _e4338 = ((_e4323.z * _e4323.z) + (_e4326 * _e4326));
                                                            let _e4348 = vec3(_e4133);
                                                            let _e4349 = (_e4348 - _e4295);
                                                            let _e4350 = -(vec3<f32>(_e4302, _e4303, _e4304));
                                                            let _e4366 = (_e4348 + _e4295);
                                                            let _e4376 = ((_e4366.x * _e4366.x) + (_e4302 * _e4302));
                                                            let _e4377 = ((_e4366.y * _e4366.y) + (_e4303 * _e4303));
                                                            let _e4378 = ((_e4366.z * _e4366.z) + (_e4304 * _e4304));
                                                            phi_17513_ = vec3<f32>((((((_e4307.x * _e4307.x) + (_e4308 * _e4308)) / select(0.000000000000000000000000000001f, _e4336, (_e4336 > 0.000000000000000000000000000001f))) + (((_e4349.x * _e4349.x) + (_e4350.x * _e4350.x)) / select(0.000000000000000000000000000001f, _e4376, (_e4376 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4307.y * _e4307.y) + (_e4309 * _e4309)) / select(0.000000000000000000000000000001f, _e4337, (_e4337 > 0.000000000000000000000000000001f))) + (((_e4349.y * _e4349.y) + (_e4350.y * _e4350.y)) / select(0.000000000000000000000000000001f, _e4377, (_e4377 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4307.z * _e4307.z) + (_e4310 * _e4310)) / select(0.000000000000000000000000000001f, _e4338, (_e4338 > 0.000000000000000000000000000001f))) + (((_e4349.z * _e4349.z) + (_e4350.z * _e4350.z)) / select(0.000000000000000000000000000001f, _e4378, (_e4378 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                        }
                                                        let _e4421 = phi_17513_;
                                                        phi_17514_ = _e4421;
                                                    }
                                                    let _e4441 = phi_17514_;
                                                    let _e4443 = (4f * _e4055);
                                                    phi_17526_ = type_60((_e4441 * (((1f / (_e4106 + _e4120)) * _e4091) / _e4443)), ((_e4091 / _e4106) / _e4443));
                                                }
                                                let _e4450 = phi_17526_;
                                                phi_17527_ = _e4450;
                                                phi_17528_ = _e4067;
                                            }
                                            let _e4452 = phi_17527_;
                                            let _e4454 = phi_17528_;
                                            if _e4454 {
                                                phi_17533_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
                                            } else {
                                                phi_17533_ = _e4452;
                                            }
                                            let _e4456 = phi_17533_;
                                            let _e4459 = (_e1931 == 1u);
                                            if _e4459 {
                                                phi_17561_ = _e4456;
                                                phi_17562_ = type_60();
                                            } else {
                                                if (_e4055 != _e4055) {
                                                    phi_28852_ = true;
                                                } else {
                                                    phi_28852_ = (0f >= _e4055);
                                                }
                                                let _e4463 = phi_28852_;
                                                let _e4464 = select(_e4055, 0f, _e4463);
                                                let _e4465 = (1f / _e1921);
                                                let _e4469 = ((_e4465 * _e4465) * (1f - (_e4464 * _e4464)));
                                                if (_e4469 >= 1f) {
                                                    phi_28871_ = 1f;
                                                } else {
                                                    let _e4472 = sqrt((1f - _e4469));
                                                    if (_e4464 != _e4464) {
                                                        phi_28882_ = true;
                                                    } else {
                                                        phi_28882_ = (_e4472 <= _e4464);
                                                    }
                                                    let _e4476 = phi_28882_;
                                                    let _e4481 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                                    phi_28871_ = (_e4481 + ((1f - _e4481) * pow((1f - select(_e4464, _e4472, _e4476)), 5f)));
                                                }
                                                let _e4488 = phi_28871_;
                                                if (_e4033 <= 0f) {
                                                    phi_28921_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_28921_ = (_e1919 * (_e4033 * 0.31830987f));
                                                }
                                                let _e4493 = phi_28921_;
                                                let _e4494 = (1f - _e4488);
                                                phi_17561_ = type_60();
                                                phi_17562_ = type_60((_e4456.member + vec3<f32>((_e4493.x * _e4494), (_e4493.y * _e4494), (_e4493.z * _e4494))), ((_e4488 * _e4456.member_1) + (_e4494 * _e4039)));
                                            }
                                            let _e4508 = phi_17561_;
                                            let _e4510 = phi_17562_;
                                            phi_17563_ = _e4508;
                                            phi_17564_ = _e4510;
                                            phi_17565_ = _e4459;
                                        }
                                        let _e4518 = phi_17563_;
                                        let _e4520 = phi_17564_;
                                        let _e4522 = phi_17565_;
                                        if _e4522 {
                                            phi_17569_ = _e4518;
                                        } else {
                                            phi_17569_ = _e4520;
                                        }
                                        let _e4524 = phi_17569_;
                                        if (_e4010 > 0f) {
                                            if (_e4010 >= 300000000000000000000000000000000000000f) {
                                                phi_28960_ = 1f;
                                            } else {
                                                let _e4529 = (_e4010 * _e4010);
                                                let _e4531 = (_e4529 + (_e4524.member_1 * _e4524.member_1));
                                                if (_e4531 == 0f) {
                                                    phi_28957_ = 0f;
                                                } else {
                                                    phi_28957_ = (_e4529 / _e4531);
                                                }
                                                let _e4535 = phi_28957_;
                                                phi_28960_ = _e4535;
                                            }
                                            let _e4537 = phi_28960_;
                                            phi_17578_ = (_e4537 / _e4010);
                                        } else {
                                            phi_17578_ = 1f;
                                        }
                                        let _e4540 = phi_17578_;
                                        if ((((_e4524.member.x * _e4524.member.x) + (_e4524.member.y * _e4524.member.y)) + (_e4524.member.z * _e4524.member.z)) > 0f) {
                                            let _e4553 = global_1.member.member_1[0u];
                                            let _e4556 = global_1.member.member_1[1u];
                                            let _e4558 = (vec3<f32>(1f, 1f, 1f) / _e4012);
                                            local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                            local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                            phi_17611_ = 4294967295u;
                                            phi_17614_ = select(4294967295u, 0u, (_e4553 > 0u));
                                            phi_17616_ = 0u;
                                            phi_17618_ = (_e4553 + _e4556);
                                            phi_17620_ = _e4553;
                                            phi_17622_ = vec4<f32>(0f, 0f, 0f, 0f);
                                            phi_17624_ = _e4016;
                                            loop {
                                                let _e4562 = phi_17611_;
                                                let _e4564 = phi_17614_;
                                                let _e4566 = phi_17616_;
                                                let _e4568 = phi_17618_;
                                                let _e4570 = phi_17620_;
                                                let _e4572 = phi_17622_;
                                                let _e4574 = phi_17624_;
                                                local_80 = _e4562;
                                                local_81 = _e4572;
                                                local_82 = _e4570;
                                                if (_e4570 < _e4568) {
                                                    let _e4938 = global_2.member[_e4570];
                                                    if (_e4938.member_6.x == 0u) {
                                                        let _e5044 = (_e4938.member_1.x - _e4938.member.x);
                                                        let _e5045 = (_e4938.member_1.y - _e4938.member.y);
                                                        let _e5046 = (_e4938.member_1.z - _e4938.member.z);
                                                        let _e5051 = (_e4938.member_2.x - _e4938.member.x);
                                                        let _e5052 = (_e4938.member_2.y - _e4938.member.y);
                                                        let _e5053 = (_e4938.member_2.z - _e4938.member.z);
                                                        let _e5056 = ((_e4012.y * _e5053) - (_e5052 * _e4012.z));
                                                        let _e5059 = ((_e4012.z * _e5051) - (_e5053 * _e4012.x));
                                                        let _e5062 = ((_e4012.x * _e5052) - (_e5051 * _e4012.y));
                                                        let _e5067 = (((_e5044 * _e5056) + (_e5045 * _e5059)) + (_e5046 * _e5062));
                                                        if (abs(_e5067) < 0.000000000001f) {
                                                            phi_17790_ = vec4<f32>();
                                                            phi_17791_ = true;
                                                        } else {
                                                            let _e5070 = (1f / _e5067);
                                                            let _e5071 = (_e1064 - _e4938.member.x);
                                                            let _e5072 = (_e1065 - _e4938.member.y);
                                                            let _e5073 = (_e1066 - _e4938.member.z);
                                                            let _e5079 = ((((_e5071 * _e5056) + (_e5072 * _e5059)) + (_e5073 * _e5062)) * _e5070);
                                                            if (_e5079 < 0f) {
                                                                phi_17780_ = vec4<f32>();
                                                                phi_17781_ = true;
                                                                phi_17782_ = false;
                                                            } else {
                                                                let _e5081 = (_e5079 > 1f);
                                                                if _e5081 {
                                                                    phi_17778_ = vec4<f32>();
                                                                    phi_17779_ = false;
                                                                } else {
                                                                    let _e5084 = ((_e5072 * _e5046) - (_e5045 * _e5073));
                                                                    let _e5087 = ((_e5073 * _e5044) - (_e5046 * _e5071));
                                                                    let _e5090 = ((_e5071 * _e5045) - (_e5044 * _e5072));
                                                                    let _e5096 = ((((_e4012.x * _e5084) + (_e4012.y * _e5087)) + (_e4012.z * _e5090)) * _e5070);
                                                                    if (_e5096 < 0f) {
                                                                        phi_17768_ = vec4<f32>();
                                                                        phi_17769_ = true;
                                                                        phi_17770_ = false;
                                                                    } else {
                                                                        let _e5099 = ((_e5079 + _e5096) > 1f);
                                                                        if _e5099 {
                                                                            phi_17766_ = vec4<f32>();
                                                                            phi_17767_ = false;
                                                                        } else {
                                                                            let _e5105 = ((((_e5051 * _e5084) + (_e5052 * _e5087)) + (_e5053 * _e5090)) * _e5070);
                                                                            if (_e5105 <= 0f) {
                                                                                phi_17761_ = vec4<f32>();
                                                                                phi_17762_ = true;
                                                                            } else {
                                                                                let _e5107 = (_e5105 >= _e4574);
                                                                                if _e5107 {
                                                                                    phi_17760_ = vec4<f32>();
                                                                                } else {
                                                                                    phi_17760_ = vec4<f32>(_e5105, _e5079, _e5096, 1f);
                                                                                }
                                                                                let _e5110 = phi_17760_;
                                                                                phi_17761_ = _e5110;
                                                                                phi_17762_ = _e5107;
                                                                            }
                                                                            let _e5112 = phi_17761_;
                                                                            let _e5114 = phi_17762_;
                                                                            phi_17766_ = _e5112;
                                                                            phi_17767_ = _e5114;
                                                                        }
                                                                        let _e5116 = phi_17766_;
                                                                        let _e5118 = phi_17767_;
                                                                        phi_17768_ = _e5116;
                                                                        phi_17769_ = _e5099;
                                                                        phi_17770_ = _e5118;
                                                                    }
                                                                    let _e5120 = phi_17768_;
                                                                    let _e5122 = phi_17769_;
                                                                    let _e5124 = phi_17770_;
                                                                    phi_17778_ = _e5120;
                                                                    phi_17779_ = select(_e5124, true, _e5122);
                                                                }
                                                                let _e5127 = phi_17778_;
                                                                let _e5129 = phi_17779_;
                                                                phi_17780_ = _e5127;
                                                                phi_17781_ = _e5081;
                                                                phi_17782_ = _e5129;
                                                            }
                                                            let _e5131 = phi_17780_;
                                                            let _e5133 = phi_17781_;
                                                            let _e5135 = phi_17782_;
                                                            phi_17790_ = _e5131;
                                                            phi_17791_ = select(_e5135, true, _e5133);
                                                        }
                                                        let _e5138 = phi_17790_;
                                                        let _e5140 = phi_17791_;
                                                        phi_17937_ = _e5138;
                                                        phi_17938_ = _e5140;
                                                    } else {
                                                        if (_e4938.member_6.x == 1u) {
                                                            let _e4966 = (_e1064 - _e4938.member.x);
                                                            let _e4967 = (_e1065 - _e4938.member.y);
                                                            let _e4968 = (_e1066 - _e4938.member.z);
                                                            let _e4973 = (((_e4966 * _e4012.x) + (_e4967 * _e4012.y)) + (_e4968 * _e4012.z));
                                                            if (_e4973 > 0f) {
                                                                phi_17831_ = select(true, false, ((((_e4966 * _e4966) + (_e4967 * _e4967)) + (_e4968 * _e4968)) > (_e4938.member.w * _e4938.member.w)));
                                                            } else {
                                                                phi_17831_ = true;
                                                            }
                                                            let _e4986 = phi_17831_;
                                                            if _e4986 {
                                                                let _e4990 = (_e4966 - (_e4012.x * _e4973));
                                                                let _e4991 = (_e4967 - (_e4012.y * _e4973));
                                                                let _e4992 = (_e4968 - (_e4012.z * _e4973));
                                                                let _e5001 = ((_e4938.member.w * _e4938.member.w) - (((_e4990 * _e4990) + (_e4991 * _e4991)) + (_e4992 * _e4992)));
                                                                if (_e5001 < 0f) {
                                                                    phi_17890_ = vec4<f32>();
                                                                    phi_17891_ = true;
                                                                } else {
                                                                    let _e5003 = sqrt(_e5001);
                                                                    let _e5004 = -(_e4973);
                                                                    let _e5005 = (_e5004 - _e5003);
                                                                    if (_e5005 <= 0f) {
                                                                        phi_17868_ = true;
                                                                    } else {
                                                                        phi_17868_ = (_e5005 >= _e4574);
                                                                    }
                                                                    let _e5009 = phi_17868_;
                                                                    if _e5009 {
                                                                        phi_17874_ = (_e5004 + _e5003);
                                                                    } else {
                                                                        phi_17874_ = _e5005;
                                                                    }
                                                                    let _e5012 = phi_17874_;
                                                                    if (_e5012 <= 0f) {
                                                                        phi_17885_ = vec4<f32>();
                                                                        phi_17886_ = true;
                                                                    } else {
                                                                        let _e5014 = (_e5012 >= _e4574);
                                                                        if _e5014 {
                                                                            phi_17884_ = vec4<f32>();
                                                                        } else {
                                                                            phi_17884_ = vec4<f32>(_e5012, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5017 = phi_17884_;
                                                                        phi_17885_ = _e5017;
                                                                        phi_17886_ = _e5014;
                                                                    }
                                                                    let _e5019 = phi_17885_;
                                                                    let _e5021 = phi_17886_;
                                                                    phi_17890_ = _e5019;
                                                                    phi_17891_ = _e5021;
                                                                }
                                                                let _e5023 = phi_17890_;
                                                                let _e5025 = phi_17891_;
                                                                phi_17895_ = _e5023;
                                                                phi_17896_ = _e5025;
                                                            } else {
                                                                phi_17895_ = vec4<f32>();
                                                                phi_17896_ = true;
                                                            }
                                                            let _e5027 = phi_17895_;
                                                            let _e5029 = phi_17896_;
                                                            phi_17927_ = _e5027;
                                                            phi_17928_ = _e5029;
                                                        } else {
                                                            if (abs(_e4012.y) < 0.000000000001f) {
                                                                phi_17925_ = vec4<f32>();
                                                                phi_17926_ = true;
                                                            } else {
                                                                let _e4948 = ((_e4938.member.x - _e1065) / _e4012.y);
                                                                if (_e4948 <= 0f) {
                                                                    phi_17920_ = vec4<f32>();
                                                                    phi_17921_ = true;
                                                                } else {
                                                                    let _e4950 = (_e4948 >= _e4574);
                                                                    if _e4950 {
                                                                        phi_17919_ = vec4<f32>();
                                                                    } else {
                                                                        phi_17919_ = vec4<f32>(_e4948, 0f, 0f, 1f);
                                                                    }
                                                                    let _e4953 = phi_17919_;
                                                                    phi_17920_ = _e4953;
                                                                    phi_17921_ = _e4950;
                                                                }
                                                                let _e4955 = phi_17920_;
                                                                let _e4957 = phi_17921_;
                                                                phi_17925_ = _e4955;
                                                                phi_17926_ = _e4957;
                                                            }
                                                            let _e4959 = phi_17925_;
                                                            let _e4961 = phi_17926_;
                                                            phi_17927_ = _e4959;
                                                            phi_17928_ = _e4961;
                                                        }
                                                        let _e5031 = phi_17927_;
                                                        let _e5033 = phi_17928_;
                                                        phi_17937_ = _e5031;
                                                        phi_17938_ = select(true, false, select(true, false, _e5033));
                                                    }
                                                    let _e5142 = phi_17937_;
                                                    let _e5144 = phi_17938_;
                                                    let _e5147 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5142, vec4(select(true, false, _e5144)));
                                                    let _e5149 = (_e5147.w > 0f);
                                                    if _e5149 {
                                                        phi_17959_ = _e5147.x;
                                                    } else {
                                                        phi_17959_ = _e4574;
                                                    }
                                                    let _e5152 = phi_17959_;
                                                    let _e5156 = select(true, false, _e5149);
                                                    if _e5156 {
                                                        phi_17965_ = (_e4570 + 1u);
                                                    } else {
                                                        phi_17965_ = u32();
                                                    }
                                                    let _e5159 = phi_17965_;
                                                    phi_17612_ = select(_e4562, _e4570, _e5149);
                                                    phi_17615_ = _e4564;
                                                    phi_17617_ = _e4566;
                                                    phi_17619_ = _e4568;
                                                    phi_17621_ = _e5159;
                                                    phi_17623_ = select(_e4572, _e5147, vec4(_e5149));
                                                    phi_17625_ = _e5152;
                                                    phi_18138_ = _e5147;
                                                    phi_18139_ = _e5156;
                                                    phi_18140_ = false;
                                                } else {
                                                    if (_e4564 == 4294967295u) {
                                                        let _e4577 = (_e4566 == 0u);
                                                        if _e4577 {
                                                            phi_17995_ = u32();
                                                            phi_17996_ = u32();
                                                            phi_17998_ = false;
                                                            phi_17999_ = false;
                                                        } else {
                                                            let _e4578 = (_e4566 - 1u);
                                                            let _e4579 = (_e4578 < 64u);
                                                            if _e4579 {
                                                            } else {
                                                                phi_34924_ = true;
                                                                phi_20566_ = bool();
                                                                phi_20565_ = vec4<f32>();
                                                                break;
                                                            }
                                                            let _e4581 = local_68[_e4578];
                                                            let _e4582 = (_e4581 >= _e4574);
                                                            if _e4582 {
                                                                phi_17993_ = u32();
                                                            } else {
                                                                if _e4579 {
                                                                } else {
                                                                    phi_34924_ = true;
                                                                    phi_20566_ = bool();
                                                                    phi_20565_ = vec4<f32>();
                                                                    break;
                                                                }
                                                                let _e4584 = local_67[_e4578];
                                                                phi_17993_ = _e4584;
                                                            }
                                                            let _e4586 = phi_17993_;
                                                            phi_17995_ = _e4578;
                                                            phi_17996_ = _e4586;
                                                            phi_17998_ = select(true, false, _e4582);
                                                            phi_17999_ = _e4582;
                                                        }
                                                        let _e4589 = phi_17995_;
                                                        let _e4591 = phi_17996_;
                                                        let _e4593 = phi_17998_;
                                                        let _e4595 = phi_17999_;
                                                        phi_18000_ = _e4589;
                                                        phi_18001_ = _e4591;
                                                        phi_18002_ = _e4589;
                                                        phi_18003_ = _e4593;
                                                        phi_18004_ = _e4595;
                                                        phi_18005_ = _e4577;
                                                    } else {
                                                        phi_18000_ = u32();
                                                        phi_18001_ = _e4564;
                                                        phi_18002_ = _e4566;
                                                        phi_18003_ = true;
                                                        phi_18004_ = false;
                                                        phi_18005_ = false;
                                                    }
                                                    let _e4597 = phi_18000_;
                                                    let _e4599 = phi_18001_;
                                                    let _e4601 = phi_18002_;
                                                    let _e4603 = phi_18003_;
                                                    let _e4605 = phi_18004_;
                                                    let _e4607 = phi_18005_;
                                                    if _e4603 {
                                                        let _e4609 = ((_e4599 & 2147483648u) == 0u);
                                                        if _e4609 {
                                                            let _e4617 = global_3.member[_e4599];
                                                            if (_e4617.member_1 == 4294967295u) {
                                                                phi_18034_ = 300000000000000000000000000000000000000f;
                                                            } else {
                                                                let _e4629 = ((_e4617.member.x - _e1064) * _e4558.x);
                                                                let _e4631 = ((_e4617.member.y - _e1065) * _e4558.y);
                                                                let _e4633 = ((_e4617.member.z - _e1066) * _e4558.z);
                                                                let _e4640 = ((_e4617.member_2.x - _e1064) * _e4558.x);
                                                                let _e4641 = ((_e4617.member_2.y - _e1065) * _e4558.y);
                                                                let _e4642 = ((_e4617.member_2.z - _e1066) * _e4558.z);
                                                                if (_e4629 == _e4629) {
                                                                    let _e4644 = (_e4640 == _e4640);
                                                                    if _e4644 {
                                                                        let _e4645 = (_e4629 != _e4629);
                                                                        if _e4645 {
                                                                            phi_29058_ = true;
                                                                        } else {
                                                                            phi_29058_ = (_e4640 <= _e4629);
                                                                        }
                                                                        let _e4648 = phi_29058_;
                                                                        let _e4649 = select(_e4629, _e4640, _e4648);
                                                                        if _e4645 {
                                                                            phi_29088_ = true;
                                                                        } else {
                                                                            phi_29088_ = (_e4640 >= _e4629);
                                                                        }
                                                                        let _e4654 = phi_29088_;
                                                                        let _e4655 = select(_e4629, _e4640, _e4654);
                                                                        phi_29035_ = type_57(select(-300000000000000000000000000000000000000f, _e4649, (_e4649 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4655, (_e4655 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29035_ = type_57();
                                                                    }
                                                                    let _e4660 = phi_29035_;
                                                                    phi_29038_ = _e4660;
                                                                    phi_29039_ = select(true, false, _e4644);
                                                                } else {
                                                                    phi_29038_ = type_57();
                                                                    phi_29039_ = true;
                                                                }
                                                                let _e4663 = phi_29038_;
                                                                let _e4665 = phi_29039_;
                                                                if _e4665 {
                                                                    phi_29044_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29044_ = _e4663;
                                                                }
                                                                let _e4667 = phi_29044_;
                                                                if (_e4631 == _e4631) {
                                                                    let _e4671 = (_e4641 == _e4641);
                                                                    if _e4671 {
                                                                        let _e4672 = (_e4631 != _e4631);
                                                                        if _e4672 {
                                                                            phi_29145_ = true;
                                                                        } else {
                                                                            phi_29145_ = (_e4641 <= _e4631);
                                                                        }
                                                                        let _e4675 = phi_29145_;
                                                                        let _e4676 = select(_e4631, _e4641, _e4675);
                                                                        if (_e4667.member != _e4667.member) {
                                                                            phi_29160_ = true;
                                                                        } else {
                                                                            phi_29160_ = (_e4676 >= _e4667.member);
                                                                        }
                                                                        let _e4680 = phi_29160_;
                                                                        if _e4672 {
                                                                            phi_29175_ = true;
                                                                        } else {
                                                                            phi_29175_ = (_e4641 >= _e4631);
                                                                        }
                                                                        let _e4684 = phi_29175_;
                                                                        let _e4685 = select(_e4631, _e4641, _e4684);
                                                                        if (_e4667.member_1 != _e4667.member_1) {
                                                                            phi_29190_ = true;
                                                                        } else {
                                                                            phi_29190_ = (_e4685 <= _e4667.member_1);
                                                                        }
                                                                        let _e4689 = phi_29190_;
                                                                        phi_29122_ = type_57(select(_e4667.member, _e4676, _e4680), select(_e4667.member_1, _e4685, _e4689));
                                                                    } else {
                                                                        phi_29122_ = type_57();
                                                                    }
                                                                    let _e4693 = phi_29122_;
                                                                    phi_29125_ = _e4693;
                                                                    phi_29126_ = select(true, false, _e4671);
                                                                } else {
                                                                    phi_29125_ = type_57();
                                                                    phi_29126_ = true;
                                                                }
                                                                let _e4696 = phi_29125_;
                                                                let _e4698 = phi_29126_;
                                                                if _e4698 {
                                                                    phi_29131_ = _e4667;
                                                                } else {
                                                                    phi_29131_ = _e4696;
                                                                }
                                                                let _e4700 = phi_29131_;
                                                                if (_e4633 == _e4633) {
                                                                    let _e4704 = (_e4642 == _e4642);
                                                                    if _e4704 {
                                                                        let _e4705 = (_e4633 != _e4633);
                                                                        if _e4705 {
                                                                            phi_29232_ = true;
                                                                        } else {
                                                                            phi_29232_ = (_e4642 <= _e4633);
                                                                        }
                                                                        let _e4708 = phi_29232_;
                                                                        let _e4709 = select(_e4633, _e4642, _e4708);
                                                                        if (_e4700.member != _e4700.member) {
                                                                            phi_29247_ = true;
                                                                        } else {
                                                                            phi_29247_ = (_e4709 >= _e4700.member);
                                                                        }
                                                                        let _e4713 = phi_29247_;
                                                                        if _e4705 {
                                                                            phi_29262_ = true;
                                                                        } else {
                                                                            phi_29262_ = (_e4642 >= _e4633);
                                                                        }
                                                                        let _e4717 = phi_29262_;
                                                                        let _e4718 = select(_e4633, _e4642, _e4717);
                                                                        if (_e4700.member_1 != _e4700.member_1) {
                                                                            phi_29277_ = true;
                                                                        } else {
                                                                            phi_29277_ = (_e4718 <= _e4700.member_1);
                                                                        }
                                                                        let _e4722 = phi_29277_;
                                                                        phi_29209_ = type_57(select(_e4700.member, _e4709, _e4713), select(_e4700.member_1, _e4718, _e4722));
                                                                    } else {
                                                                        phi_29209_ = type_57();
                                                                    }
                                                                    let _e4726 = phi_29209_;
                                                                    phi_29212_ = _e4726;
                                                                    phi_29213_ = select(true, false, _e4704);
                                                                } else {
                                                                    phi_29212_ = type_57();
                                                                    phi_29213_ = true;
                                                                }
                                                                let _e4729 = phi_29212_;
                                                                let _e4731 = phi_29213_;
                                                                if _e4731 {
                                                                    phi_29218_ = _e4700;
                                                                } else {
                                                                    phi_29218_ = _e4729;
                                                                }
                                                                let _e4733 = phi_29218_;
                                                                if (_e4733.member_1 < 0f) {
                                                                    phi_29016_ = true;
                                                                } else {
                                                                    if (_e4733.member > _e4733.member_1) {
                                                                        phi_29013_ = true;
                                                                    } else {
                                                                        phi_29013_ = (_e4733.member > _e4574);
                                                                    }
                                                                    let _e4740 = phi_29013_;
                                                                    phi_29016_ = _e4740;
                                                                }
                                                                let _e4742 = phi_29016_;
                                                                phi_18034_ = select(_e4733.member, 300000000000000000000000000000000000000f, _e4742);
                                                            }
                                                            let _e4745 = phi_18034_;
                                                            if (_e4617.member_5 == 4294967295u) {
                                                                phi_18043_ = 300000000000000000000000000000000000000f;
                                                            } else {
                                                                let _e4757 = ((_e4617.member_4.x - _e1064) * _e4558.x);
                                                                let _e4759 = ((_e4617.member_4.y - _e1065) * _e4558.y);
                                                                let _e4761 = ((_e4617.member_4.z - _e1066) * _e4558.z);
                                                                let _e4768 = ((_e4617.member_6.x - _e1064) * _e4558.x);
                                                                let _e4769 = ((_e4617.member_6.y - _e1065) * _e4558.y);
                                                                let _e4770 = ((_e4617.member_6.z - _e1066) * _e4558.z);
                                                                if (_e4757 == _e4757) {
                                                                    let _e4772 = (_e4768 == _e4768);
                                                                    if _e4772 {
                                                                        let _e4773 = (_e4757 != _e4757);
                                                                        if _e4773 {
                                                                            phi_29379_ = true;
                                                                        } else {
                                                                            phi_29379_ = (_e4768 <= _e4757);
                                                                        }
                                                                        let _e4776 = phi_29379_;
                                                                        let _e4777 = select(_e4757, _e4768, _e4776);
                                                                        if _e4773 {
                                                                            phi_29409_ = true;
                                                                        } else {
                                                                            phi_29409_ = (_e4768 >= _e4757);
                                                                        }
                                                                        let _e4782 = phi_29409_;
                                                                        let _e4783 = select(_e4757, _e4768, _e4782);
                                                                        phi_29356_ = type_57(select(-300000000000000000000000000000000000000f, _e4777, (_e4777 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4783, (_e4783 <= 300000000000000000000000000000000000000f)));
                                                                    } else {
                                                                        phi_29356_ = type_57();
                                                                    }
                                                                    let _e4788 = phi_29356_;
                                                                    phi_29359_ = _e4788;
                                                                    phi_29360_ = select(true, false, _e4772);
                                                                } else {
                                                                    phi_29359_ = type_57();
                                                                    phi_29360_ = true;
                                                                }
                                                                let _e4791 = phi_29359_;
                                                                let _e4793 = phi_29360_;
                                                                if _e4793 {
                                                                    phi_29365_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                } else {
                                                                    phi_29365_ = _e4791;
                                                                }
                                                                let _e4795 = phi_29365_;
                                                                if (_e4759 == _e4759) {
                                                                    let _e4799 = (_e4769 == _e4769);
                                                                    if _e4799 {
                                                                        let _e4800 = (_e4759 != _e4759);
                                                                        if _e4800 {
                                                                            phi_29466_ = true;
                                                                        } else {
                                                                            phi_29466_ = (_e4769 <= _e4759);
                                                                        }
                                                                        let _e4803 = phi_29466_;
                                                                        let _e4804 = select(_e4759, _e4769, _e4803);
                                                                        if (_e4795.member != _e4795.member) {
                                                                            phi_29481_ = true;
                                                                        } else {
                                                                            phi_29481_ = (_e4804 >= _e4795.member);
                                                                        }
                                                                        let _e4808 = phi_29481_;
                                                                        if _e4800 {
                                                                            phi_29496_ = true;
                                                                        } else {
                                                                            phi_29496_ = (_e4769 >= _e4759);
                                                                        }
                                                                        let _e4812 = phi_29496_;
                                                                        let _e4813 = select(_e4759, _e4769, _e4812);
                                                                        if (_e4795.member_1 != _e4795.member_1) {
                                                                            phi_29511_ = true;
                                                                        } else {
                                                                            phi_29511_ = (_e4813 <= _e4795.member_1);
                                                                        }
                                                                        let _e4817 = phi_29511_;
                                                                        phi_29443_ = type_57(select(_e4795.member, _e4804, _e4808), select(_e4795.member_1, _e4813, _e4817));
                                                                    } else {
                                                                        phi_29443_ = type_57();
                                                                    }
                                                                    let _e4821 = phi_29443_;
                                                                    phi_29446_ = _e4821;
                                                                    phi_29447_ = select(true, false, _e4799);
                                                                } else {
                                                                    phi_29446_ = type_57();
                                                                    phi_29447_ = true;
                                                                }
                                                                let _e4824 = phi_29446_;
                                                                let _e4826 = phi_29447_;
                                                                if _e4826 {
                                                                    phi_29452_ = _e4795;
                                                                } else {
                                                                    phi_29452_ = _e4824;
                                                                }
                                                                let _e4828 = phi_29452_;
                                                                if (_e4761 == _e4761) {
                                                                    let _e4832 = (_e4770 == _e4770);
                                                                    if _e4832 {
                                                                        let _e4833 = (_e4761 != _e4761);
                                                                        if _e4833 {
                                                                            phi_29553_ = true;
                                                                        } else {
                                                                            phi_29553_ = (_e4770 <= _e4761);
                                                                        }
                                                                        let _e4836 = phi_29553_;
                                                                        let _e4837 = select(_e4761, _e4770, _e4836);
                                                                        if (_e4828.member != _e4828.member) {
                                                                            phi_29568_ = true;
                                                                        } else {
                                                                            phi_29568_ = (_e4837 >= _e4828.member);
                                                                        }
                                                                        let _e4841 = phi_29568_;
                                                                        if _e4833 {
                                                                            phi_29583_ = true;
                                                                        } else {
                                                                            phi_29583_ = (_e4770 >= _e4761);
                                                                        }
                                                                        let _e4845 = phi_29583_;
                                                                        let _e4846 = select(_e4761, _e4770, _e4845);
                                                                        if (_e4828.member_1 != _e4828.member_1) {
                                                                            phi_29598_ = true;
                                                                        } else {
                                                                            phi_29598_ = (_e4846 <= _e4828.member_1);
                                                                        }
                                                                        let _e4850 = phi_29598_;
                                                                        phi_29530_ = type_57(select(_e4828.member, _e4837, _e4841), select(_e4828.member_1, _e4846, _e4850));
                                                                    } else {
                                                                        phi_29530_ = type_57();
                                                                    }
                                                                    let _e4854 = phi_29530_;
                                                                    phi_29533_ = _e4854;
                                                                    phi_29534_ = select(true, false, _e4832);
                                                                } else {
                                                                    phi_29533_ = type_57();
                                                                    phi_29534_ = true;
                                                                }
                                                                let _e4857 = phi_29533_;
                                                                let _e4859 = phi_29534_;
                                                                if _e4859 {
                                                                    phi_29539_ = _e4828;
                                                                } else {
                                                                    phi_29539_ = _e4857;
                                                                }
                                                                let _e4861 = phi_29539_;
                                                                if (_e4861.member_1 < 0f) {
                                                                    phi_29337_ = true;
                                                                } else {
                                                                    if (_e4861.member > _e4861.member_1) {
                                                                        phi_29334_ = true;
                                                                    } else {
                                                                        phi_29334_ = (_e4861.member > _e4574);
                                                                    }
                                                                    let _e4868 = phi_29334_;
                                                                    phi_29337_ = _e4868;
                                                                }
                                                                let _e4870 = phi_29337_;
                                                                phi_18043_ = select(_e4861.member, 300000000000000000000000000000000000000f, _e4870);
                                                            }
                                                            let _e4873 = phi_18043_;
                                                            if (_e4617.member_3 > 0u) {
                                                                phi_18054_ = ((2147483648u | ((_e4617.member_3 - 1u) << bitcast<u32>(28u))) | _e4617.member_1);
                                                            } else {
                                                                phi_18054_ = _e4617.member_1;
                                                            }
                                                            let _e4882 = phi_18054_;
                                                            if (_e4617.member_7 > 0u) {
                                                                phi_18065_ = ((2147483648u | ((_e4617.member_7 - 1u) << bitcast<u32>(28u))) | _e4617.member_5);
                                                            } else {
                                                                phi_18065_ = _e4617.member_5;
                                                            }
                                                            let _e4891 = phi_18065_;
                                                            let _e4892 = (_e4745 <= _e4873);
                                                            let _e4896 = select(_e4745, _e4873, _e4892);
                                                            if (_e4896 < 300000000000000000000000000000000000000f) {
                                                                let _e4898 = (_e4601 < 64u);
                                                                if _e4898 {
                                                                    if _e4898 {
                                                                    } else {
                                                                        phi_34924_ = true;
                                                                        phi_20566_ = bool();
                                                                        phi_20565_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    local_67[_e4601] = select(_e4882, _e4891, _e4892);
                                                                    if _e4898 {
                                                                    } else {
                                                                        phi_34924_ = true;
                                                                        phi_20566_ = bool();
                                                                        phi_20565_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    local_68[_e4601] = _e4896;
                                                                    phi_18104_ = (_e4601 + 1u);
                                                                } else {
                                                                    phi_18104_ = _e4601;
                                                                }
                                                                let _e4903 = phi_18104_;
                                                                phi_18105_ = _e4903;
                                                            } else {
                                                                phi_18105_ = _e4601;
                                                            }
                                                            let _e4905 = phi_18105_;
                                                            phi_18116_ = select(4294967295u, select(_e4891, _e4882, _e4892), (select(_e4873, _e4745, _e4892) < 300000000000000000000000000000000000000f));
                                                            phi_18117_ = _e4905;
                                                            phi_18118_ = u32();
                                                            phi_18119_ = u32();
                                                        } else {
                                                            let _e4610 = (_e4599 & 268435455u);
                                                            phi_18116_ = u32();
                                                            phi_18117_ = u32();
                                                            phi_18118_ = ((_e4610 + ((_e4599 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                            phi_18119_ = _e4610;
                                                        }
                                                        let _e4909 = phi_18116_;
                                                        let _e4911 = phi_18117_;
                                                        let _e4913 = phi_18118_;
                                                        let _e4915 = phi_18119_;
                                                        phi_18121_ = _e4909;
                                                        phi_18122_ = _e4911;
                                                        phi_18124_ = _e4913;
                                                        phi_18125_ = _e4915;
                                                        phi_18126_ = select(true, false, _e4609);
                                                        phi_18127_ = _e4609;
                                                    } else {
                                                        phi_18121_ = u32();
                                                        phi_18122_ = u32();
                                                        phi_18124_ = _e4568;
                                                        phi_18125_ = _e4570;
                                                        phi_18126_ = _e4605;
                                                        phi_18127_ = false;
                                                    }
                                                    let _e4918 = phi_18121_;
                                                    let _e4920 = phi_18122_;
                                                    let _e4922 = phi_18124_;
                                                    let _e4924 = phi_18125_;
                                                    let _e4926 = phi_18126_;
                                                    let _e4928 = phi_18127_;
                                                    phi_17612_ = _e4562;
                                                    phi_17615_ = select(_e4918, 4294967295u, _e4926);
                                                    phi_17617_ = select(_e4920, select(_e4597, _e4601, _e4603), _e4926);
                                                    phi_17619_ = select(_e4568, _e4922, _e4926);
                                                    phi_17621_ = select(_e4570, _e4924, _e4926);
                                                    phi_17623_ = _e4572;
                                                    phi_17625_ = _e4574;
                                                    phi_18138_ = vec4<f32>();
                                                    phi_18139_ = select(_e4928, true, _e4926);
                                                    phi_18140_ = select(select(_e4607, false, _e4603), false, _e4926);
                                                }
                                                let _e5161 = phi_17612_;
                                                let _e5163 = phi_17615_;
                                                let _e5165 = phi_17617_;
                                                let _e5167 = phi_17619_;
                                                let _e5169 = phi_17621_;
                                                let _e5171 = phi_17623_;
                                                let _e5173 = phi_17625_;
                                                let _e5175 = phi_18138_;
                                                let _e5177 = phi_18139_;
                                                let _e5179 = phi_18140_;
                                                phi_17611_ = _e5161;
                                                phi_17614_ = _e5163;
                                                phi_17616_ = _e5165;
                                                phi_17618_ = _e5167;
                                                phi_17620_ = _e5169;
                                                phi_17622_ = _e5171;
                                                phi_17624_ = _e5173;
                                                phi_34924_ = _e1944;
                                                phi_20566_ = _e5179;
                                                phi_20565_ = _e5175;
                                                if !(_e5177) {
                                                    break;
                                                }
                                            }
                                            let _e5182 = phi_34924_;
                                            let _e5184 = phi_20566_;
                                            let _e5186 = phi_20565_;
                                            phi_35371_ = _e4008;
                                            phi_35363_ = _e5182;
                                            if _e5182 {
                                                break;
                                            }
                                            if _e5184 {
                                                let _e5188 = local_80;
                                                let _e5190 = local_81;
                                                phi_18145_ = type_76(_e5188, _e5190);
                                            } else {
                                                phi_18145_ = type_76();
                                            }
                                            let _e5193 = phi_18145_;
                                            if select(true, false, _e5184) {
                                                let _e5196 = local_82;
                                                phi_18151_ = type_76(_e5196, _e5186);
                                            } else {
                                                phi_18151_ = _e5193;
                                            }
                                            let _e5199 = phi_18151_;
                                            if (_e5199.member != 4294967295u) {
                                                phi_18205_ = _e1948;
                                                phi_18206_ = _e1950;
                                            } else {
                                                if (_e1952 == (_e1942 + 2u)) {
                                                    phi_18203_ = _e1948;
                                                    phi_18204_ = type_42(vec4<f32>(_e1064, _e1065, _e1066, -1f), vec4<f32>(_e4012.x, _e4012.y, _e4012.z, 0f), vec4<f32>(((_e820.x * _e4524.member.x) * _e4540), ((_e820.y * _e4524.member.y) * _e4540), ((_e820.z * _e4524.member.z) * _e4540), 0f));
                                                } else {
                                                    phi_18203_ = vec3<f32>((_e1948.x + (((_e820.x * _e4524.member.x) * _e4014.x) * _e4540)), (_e1948.y + (((_e820.y * _e4524.member.y) * _e4014.y) * _e4540)), (_e1948.z + (((_e820.z * _e4524.member.z) * _e4014.z) * _e4540)));
                                                    phi_18204_ = _e1950;
                                                }
                                                let _e5237 = phi_18203_;
                                                let _e5239 = phi_18204_;
                                                phi_18205_ = _e5237;
                                                phi_18206_ = _e5239;
                                            }
                                            let _e5241 = phi_18205_;
                                            let _e5243 = phi_18206_;
                                            phi_35366_ = _e5182;
                                            phi_18207_ = _e5241;
                                            phi_18208_ = _e5243;
                                            phi_18209_ = (_e1954 + 1u);
                                        } else {
                                            phi_35366_ = _e1944;
                                            phi_18207_ = _e1948;
                                            phi_18208_ = _e1950;
                                            phi_18209_ = _e1954;
                                        }
                                        let _e5245 = phi_35366_;
                                        let _e5247 = phi_18207_;
                                        let _e5249 = phi_18208_;
                                        let _e5251 = phi_18209_;
                                        phi_35365_ = _e5245;
                                        phi_18210_ = _e5247;
                                        phi_18211_ = _e5249;
                                        phi_18212_ = _e5251;
                                    } else {
                                        phi_35365_ = _e1944;
                                        phi_18210_ = _e1948;
                                        phi_18211_ = _e1950;
                                        phi_18212_ = _e1954;
                                    }
                                    let _e5253 = phi_35365_;
                                    let _e5255 = phi_18210_;
                                    let _e5257 = phi_18211_;
                                    let _e5259 = phi_18212_;
                                    phi_35651_ = _e4008;
                                    phi_35364_ = _e5253;
                                    phi_14123_ = _e5255;
                                    phi_14126_ = _e5257;
                                    phi_14128_ = (_e1952 + 1u);
                                    phi_14130_ = _e5259;
                                } else {
                                    phi_35651_ = _e1946;
                                    phi_35364_ = _e1944;
                                    phi_14123_ = vec3<f32>();
                                    phi_14126_ = type_42();
                                    phi_14128_ = u32();
                                    phi_14130_ = u32();
                                }
                                let _e5262 = phi_35651_;
                                let _e5264 = phi_35364_;
                                let _e5266 = phi_14123_;
                                let _e5268 = phi_14126_;
                                let _e5270 = phi_14128_;
                                let _e5272 = phi_14130_;
                                phi_34996_ = _e5264;
                                phi_33071_ = _e5262;
                                phi_14122_ = _e5266;
                                phi_14125_ = _e5268;
                                phi_14127_ = _e5270;
                                phi_14129_ = _e5272;
                                phi_35371_ = _e5262;
                                phi_35363_ = _e5264;
                                if !(_e1956) {
                                    break;
                                }
                            }
                            let _e5275 = phi_35371_;
                            let _e5277 = phi_35363_;
                            if _e5277 {
                                break;
                            }
                            if (_e356 >= _e373) {
                                let _e6281 = local_98;
                                let _e6284 = local_99;
                                let _e6287 = local_100;
                                let _e6290 = local_101;
                                let _e6296 = local_102;
                                phi_18763_ = type_70(type_35(_e331.member, _e331.member_1, vec4<f32>(_e820.x, _e820.y, _e820.z, f32(_e356)), vec4<f32>(_e6284.x, _e6287.y, _e6290.z, f32(_e6281)), vec4<u32>(_e5275, (_e1939 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6296);
                                phi_18764_ = type_70();
                                phi_18765_ = true;
                            } else {
                                let _e5280 = ((_e5275 * 747796405u) + 2891336453u);
                                let _e5288 = (((_e5280 >> bitcast<u32>((((_e5280 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5280) * 277803737u);
                                let _e5291 = ((_e5288 >> bitcast<u32>(22i)) ^ _e5288);
                                let _e5294 = f32((_e5291 >> bitcast<u32>(8i)));
                                let _e5297 = ((_e5291 * 747796405u) + 2891336453u);
                                let _e5305 = (((_e5297 >> bitcast<u32>((((_e5297 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5297) * 277803737u);
                                let _e5308 = ((_e5305 >> bitcast<u32>(22i)) ^ _e5305);
                                let _e5311 = f32((_e5308 >> bitcast<u32>(8i)));
                                let _e5313 = ((_e5308 * 747796405u) + 2891336453u);
                                let _e5321 = (((_e5313 >> bitcast<u32>((((_e5313 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5313) * 277803737u);
                                let _e5324 = ((_e5321 >> bitcast<u32>(22i)) ^ _e5321);
                                let _e5329 = (_e1931 == 0u);
                                if _e5329 {
                                    phi_29708_ = vec3<f32>();
                                    phi_29709_ = true;
                                } else {
                                    if (_e1931 == 2u) {
                                        let _e5335 = (((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z));
                                        if (_e5335 != _e5335) {
                                            phi_29749_ = true;
                                        } else {
                                            phi_29749_ = (0f >= _e5335);
                                        }
                                        let _e5339 = phi_29749_;
                                        let _e5340 = select(_e5335, 0f, _e5339);
                                        let _e5341 = (1f / _e1921);
                                        let _e5345 = ((_e5341 * _e5341) * (1f - (_e5340 * _e5340)));
                                        if (_e5345 >= 1f) {
                                            phi_29768_ = 1f;
                                        } else {
                                            let _e5348 = sqrt((1f - _e5345));
                                            if (_e5340 != _e5340) {
                                                phi_29779_ = true;
                                            } else {
                                                phi_29779_ = (_e5348 <= _e5340);
                                            }
                                            let _e5352 = phi_29779_;
                                            let _e5357 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                            phi_29768_ = (_e5357 + ((1f - _e5357) * pow((1f - select(_e5340, _e5348, _e5352)), 5f)));
                                        }
                                        let _e5364 = phi_29768_;
                                        phi_29699_ = select(true, false, ((f32((_e5324 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5364));
                                    } else {
                                        phi_29699_ = true;
                                    }
                                    let _e5368 = phi_29699_;
                                    if _e5368 {
                                        let _e5373 = (((_e1013 * _e1087) + (_e1014 * _e1088)) + (_e1015 * _e1090));
                                        let _e5378 = (((_e1013 * _e1083) + (_e1014 * _e1093)) + (_e1015 * _e1094));
                                        let _e5383 = (((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z));
                                        if (_e5383 <= 0f) {
                                            phi_29933_ = vec3<f32>();
                                            phi_29934_ = true;
                                        } else {
                                            let _e5386 = (_e1929 * _e5373);
                                            let _e5387 = (_e1929 * _e5378);
                                            let _e5394 = (1f / sqrt((((_e5386 * _e5386) + (_e5387 * _e5387)) + (_e5383 * _e5383))));
                                            let _e5395 = (_e5386 * _e5394);
                                            let _e5396 = (_e5387 * _e5394);
                                            let _e5397 = (_e5383 * _e5394);
                                            let _e5400 = ((_e5395 * _e5395) + (_e5396 * _e5396));
                                            if (_e5400 > 0f) {
                                                let _e5403 = sqrt(_e5400);
                                                phi_29835_ = vec3<f32>((-(_e5396) / _e5403), (_e5395 / _e5403), 0f);
                                            } else {
                                                phi_29835_ = vec3<f32>(1f, 0f, 0f);
                                            }
                                            let _e5408 = phi_29835_;
                                            let _e5421 = sqrt((_e5294 * 0.000000059604645f));
                                            let _e5422 = (_e5311 * 0.00000037450704f);
                                            let _e5424 = (_e5421 * cos(_e5422));
                                            let _e5426 = (0.5f * (1f + _e5397));
                                            let _e5429 = (1f - (_e5424 * _e5424));
                                            if (_e5429 != _e5429) {
                                                phi_29991_ = true;
                                            } else {
                                                phi_29991_ = (0f >= _e5429);
                                            }
                                            let _e5433 = phi_29991_;
                                            let _e5440 = (((1f - _e5426) * sqrt(select(_e5429, 0f, _e5433))) + ((_e5426 * _e5421) * sin(_e5422)));
                                            let _e5451 = (_e5429 - (_e5440 * _e5440));
                                            if (_e5451 != _e5451) {
                                                phi_30006_ = true;
                                            } else {
                                                phi_30006_ = (0f >= _e5451);
                                            }
                                            let _e5455 = phi_30006_;
                                            let _e5457 = sqrt(select(_e5451, 0f, _e5455));
                                            let _e5463 = (((_e5408.z * _e5424) + (((_e5395 * _e5408.y) - (_e5408.x * _e5396)) * _e5440)) + (_e5397 * _e5457));
                                            let _e5464 = (_e1929 * (((_e5408.x * _e5424) + (((_e5396 * _e5408.z) - (_e5408.y * _e5397)) * _e5440)) + (_e5395 * _e5457)));
                                            let _e5465 = (_e1929 * (((_e5408.y * _e5424) + (((_e5397 * _e5408.x) - (_e5408.z * _e5395)) * _e5440)) + (_e5396 * _e5457)));
                                            if (_e5463 != _e5463) {
                                                phi_30021_ = true;
                                            } else {
                                                phi_30021_ = (0f >= _e5463);
                                            }
                                            let _e5469 = phi_30021_;
                                            let _e5470 = select(_e5463, 0f, _e5469);
                                            let _e5477 = (1f / sqrt((((_e5464 * _e5464) + (_e5465 * _e5465)) + (_e5470 * _e5470))));
                                            let _e5478 = (_e5464 * _e5477);
                                            let _e5479 = (_e5465 * _e5477);
                                            let _e5480 = (_e5470 * _e5477);
                                            let _e5486 = (2f * (((_e5373 * _e5478) + (_e5378 * _e5479)) + (_e5383 * _e5480)));
                                            let _e5489 = (_e5480 * _e5486);
                                            let _e5492 = (vec3<f32>((_e5478 * _e5486), (_e5479 * _e5486), _e5489) - vec3<f32>(_e5373, _e5378, _e5383));
                                            let _e5493 = ((_e5489 - _e5383) <= 0f);
                                            if _e5493 {
                                                phi_29930_ = vec3<f32>();
                                            } else {
                                                phi_29930_ = vec3<f32>((((_e1087 * _e5492.x) + (_e1083 * _e5492.y)) + (_e1039.x * _e5492.z)), (((_e1088 * _e5492.x) + (_e1093 * _e5492.y)) + (_e1039.y * _e5492.z)), (((_e1090 * _e5492.x) + (_e1094 * _e5492.y)) + (_e1039.z * _e5492.z)));
                                            }
                                            let _e5514 = phi_29930_;
                                            phi_29933_ = _e5514;
                                            phi_29934_ = _e5493;
                                        }
                                        let _e5516 = phi_29933_;
                                        let _e5518 = phi_29934_;
                                        phi_29704_ = select(_e5516, vec3<f32>(0f, 0f, 0f), vec3(_e5518));
                                    } else {
                                        phi_29704_ = vec3<f32>();
                                    }
                                    let _e5522 = phi_29704_;
                                    phi_29708_ = _e5522;
                                    phi_29709_ = select(true, false, _e5368);
                                }
                                let _e5525 = phi_29708_;
                                let _e5527 = phi_29709_;
                                if _e5527 {
                                    let _e5528 = (_e5294 * 0.00000037450704f);
                                    let _e5530 = ((_e5311 * 0.00000011920929f) - 1f);
                                    let _e5532 = (1f - (_e5530 * _e5530));
                                    if (_e5532 != _e5532) {
                                        phi_30121_ = true;
                                    } else {
                                        phi_30121_ = (0f >= _e5532);
                                    }
                                    let _e5536 = phi_30121_;
                                    let _e5538 = sqrt(select(_e5532, 0f, _e5536));
                                    let _e5543 = (_e1039.x + (_e5538 * cos(_e5528)));
                                    let _e5544 = (_e1039.y + (_e5538 * sin(_e5528)));
                                    let _e5545 = (_e1039.z + _e5530);
                                    let _e5551 = sqrt((((_e5543 * _e5543) + (_e5544 * _e5544)) + (_e5545 * _e5545)));
                                    if (_e5551 < 0.000001f) {
                                        phi_30095_ = _e1039;
                                    } else {
                                        let _e5553 = (1f / _e5551);
                                        phi_30095_ = vec3<f32>((_e5543 * _e5553), (_e5544 * _e5553), (_e5545 * _e5553));
                                    }
                                    let _e5559 = phi_30095_;
                                    phi_29716_ = _e5559;
                                } else {
                                    phi_29716_ = _e5525;
                                }
                                let _e5561 = phi_29716_;
                                let _e5569 = (((_e5561.x * _e1039.x) + (_e5561.y * _e1039.y)) + (_e5561.z * _e1039.z));
                                if (_e5569 != _e5569) {
                                    phi_30136_ = true;
                                } else {
                                    phi_30136_ = (0f >= _e5569);
                                }
                                let _e5573 = phi_30136_;
                                let _e5575 = (select(_e5569, 0f, _e5573) * 0.31830987f);
                                if _e5329 {
                                    if (_e5569 <= 0f) {
                                        phi_30933_ = vec3<f32>(0f, 0f, 0f);
                                    } else {
                                        phi_30933_ = (_e1925 * (_e5569 * 0.31830987f));
                                    }
                                    let _e6050 = phi_30933_;
                                    phi_18604_ = type_60(_e6050, _e5575);
                                    phi_18605_ = type_60();
                                    phi_18606_ = true;
                                } else {
                                    let _e5580 = (((_e1013 * _e1087) + (_e1014 * _e1088)) + (_e1015 * _e1090));
                                    let _e5585 = (((_e1013 * _e1083) + (_e1014 * _e1093)) + (_e1015 * _e1094));
                                    let _e5590 = (((_e1013 * _e1039.x) + (_e1014 * _e1039.y)) + (_e1015 * _e1039.z));
                                    let _e5595 = (((_e5561.x * _e1087) + (_e5561.y * _e1088)) + (_e5561.z * _e1090));
                                    let _e5600 = (((_e5561.x * _e1083) + (_e5561.y * _e1093)) + (_e5561.z * _e1094));
                                    if (_e5590 <= 0f) {
                                        phi_18568_ = type_60();
                                        phi_18569_ = true;
                                    } else {
                                        let _e5602 = (_e5569 <= 0f);
                                        if _e5602 {
                                            phi_18567_ = type_60();
                                        } else {
                                            let _e5603 = (_e5580 + _e5595);
                                            let _e5604 = (_e5585 + _e5600);
                                            let _e5605 = (_e5590 + _e5569);
                                            let _e5612 = (1f / sqrt((((_e5603 * _e5603) + (_e5604 * _e5604)) + (_e5605 * _e5605))));
                                            let _e5615 = (_e5605 * _e5612);
                                            if (_e5615 <= 0f) {
                                                phi_30246_ = 0f;
                                            } else {
                                                let _e5617 = (_e1929 * _e1929);
                                                let _e5621 = ((((_e5617 - 1f) * _e5615) * _e5615) + 1f);
                                                phi_30246_ = (_e5617 / ((3.1415927f * _e5621) * _e5621));
                                            }
                                            let _e5626 = phi_30246_;
                                            let _e5627 = (_e5590 * _e5590);
                                            if (_e5627 >= 1f) {
                                                phi_30274_ = 0f;
                                            } else {
                                                phi_30274_ = ((sqrt((1f + (((_e1929 * _e1929) * ((_e5580 * _e5580) + (_e5585 * _e5585))) / _e5627))) - 1f) * 0.5f);
                                            }
                                            let _e5640 = phi_30274_;
                                            let _e5641 = (1f + _e5640);
                                            let _e5642 = (_e5569 * _e5569);
                                            if (_e5642 >= 1f) {
                                                phi_30302_ = 0f;
                                            } else {
                                                phi_30302_ = ((sqrt((1f + (((_e1929 * _e1929) * ((_e5595 * _e5595) + (_e5600 * _e5600))) / _e5642))) - 1f) * 0.5f);
                                            }
                                            let _e5655 = phi_30302_;
                                            let _e5662 = (((_e5580 * (_e5603 * _e5612)) + (_e5585 * (_e5604 * _e5612))) + (_e5590 * _e5615));
                                            if (_e1927 == 0u) {
                                                let _e5957 = (1f - _e5662);
                                                if (_e5957 != _e5957) {
                                                    phi_30805_ = true;
                                                } else {
                                                    phi_30805_ = (0f >= _e5957);
                                                }
                                                let _e5961 = phi_30805_;
                                                let _e5963 = pow(select(_e5957, 0f, _e5961), 5f);
                                                let _e5964 = (1f - _e5963);
                                                phi_18555_ = vec3<f32>(((_e1925.x * _e5964) + _e5963), ((_e1925.y * _e5964) + _e5963), ((_e1925.z * _e5964) + _e5963));
                                            } else {
                                                if (_e1927 == 1u) {
                                                    let _e5930 = (1f / _e1921);
                                                    let _e5934 = ((_e5930 * _e5930) * (1f - (_e5662 * _e5662)));
                                                    if (_e5934 >= 1f) {
                                                        phi_30766_ = 1f;
                                                    } else {
                                                        let _e5937 = sqrt((1f - _e5934));
                                                        if (_e5662 != _e5662) {
                                                            phi_30777_ = true;
                                                        } else {
                                                            phi_30777_ = (_e5937 <= _e5662);
                                                        }
                                                        let _e5941 = phi_30777_;
                                                        let _e5946 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                                        phi_30766_ = (_e5946 + ((1f - _e5946) * pow((1f - select(_e5662, _e5937, _e5941)), 5f)));
                                                    }
                                                    let _e5953 = phi_30766_;
                                                    phi_18554_ = vec3(_e5953);
                                                } else {
                                                    let _e5666 = select(_e5662, 0f, (_e5662 < 0f));
                                                    let _e5668 = select(_e5666, 1f, (_e5666 > 1f));
                                                    let _e5670 = (1f - (_e5668 * _e5668));
                                                    let _e5683 = ((_e1925.x * _e1925.x) - (_e1923.x * _e1923.x));
                                                    let _e5684 = ((_e1925.y * _e1925.y) - (_e1923.y * _e1923.y));
                                                    let _e5685 = ((_e1925.z * _e1925.z) - (_e1923.z * _e1923.z));
                                                    let _e5687 = (_e1925.x * (_e1923.x + _e1923.x));
                                                    let _e5689 = (_e1925.y * (_e1923.y + _e1923.y));
                                                    let _e5691 = (_e1925.z * (_e1923.z + _e1923.z));
                                                    let _e5698 = ((_e5683 * _e5683) + (_e5687 * _e5687));
                                                    let _e5699 = ((_e5684 * _e5684) + (_e5689 * _e5689));
                                                    let _e5700 = ((_e5685 * _e5685) + (_e5691 * _e5691));
                                                    let _e5705 = ((_e5670 * _e5683) / _e5698);
                                                    let _e5706 = ((_e5670 * _e5684) / _e5699);
                                                    let _e5707 = ((_e5670 * _e5685) / _e5700);
                                                    let _e5708 = (1f - _e5705);
                                                    let _e5709 = (1f - _e5706);
                                                    let _e5710 = (1f - _e5707);
                                                    let _e5712 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5705, _e5706, _e5707));
                                                    let _e5713 = (_e5670 * _e5687);
                                                    let _e5714 = (_e5670 * _e5689);
                                                    let _e5715 = (_e5670 * _e5691);
                                                    let _e5720 = (vec3<f32>(_e5713, _e5714, _e5715) / vec3<f32>(_e5698, _e5699, _e5700));
                                                    let _e5733 = ((_e5712.x * _e5712.x) + (_e5720.x * _e5720.x));
                                                    let _e5734 = ((_e5712.y * _e5712.y) + (_e5720.y * _e5720.y));
                                                    let _e5735 = ((_e5712.z * _e5712.z) + (_e5720.z * _e5720.z));
                                                    if (_e5733 != _e5733) {
                                                        phi_30436_ = true;
                                                    } else {
                                                        phi_30436_ = (0f >= _e5733);
                                                    }
                                                    let _e5739 = phi_30436_;
                                                    let _e5741 = sqrt(select(_e5733, 0f, _e5739));
                                                    if (_e5734 != _e5734) {
                                                        phi_30451_ = true;
                                                    } else {
                                                        phi_30451_ = (0f >= _e5734);
                                                    }
                                                    let _e5745 = phi_30451_;
                                                    let _e5747 = sqrt(select(_e5734, 0f, _e5745));
                                                    if (_e5735 != _e5735) {
                                                        phi_30466_ = true;
                                                    } else {
                                                        phi_30466_ = (0f >= _e5735);
                                                    }
                                                    let _e5751 = phi_30466_;
                                                    let _e5753 = sqrt(select(_e5735, 0f, _e5751));
                                                    let _e5757 = ((_e5741 + _e5708) * 0.5f);
                                                    let _e5758 = ((_e5747 + _e5709) * 0.5f);
                                                    let _e5759 = ((_e5753 + _e5710) * 0.5f);
                                                    if (_e5757 != _e5757) {
                                                        phi_30493_ = true;
                                                    } else {
                                                        phi_30493_ = (0f >= _e5757);
                                                    }
                                                    let _e5763 = phi_30493_;
                                                    let _e5765 = sqrt(select(_e5757, 0f, _e5763));
                                                    if (_e5758 != _e5758) {
                                                        phi_30508_ = true;
                                                    } else {
                                                        phi_30508_ = (0f >= _e5758);
                                                    }
                                                    let _e5769 = phi_30508_;
                                                    let _e5771 = sqrt(select(_e5758, 0f, _e5769));
                                                    if (_e5759 != _e5759) {
                                                        phi_30523_ = true;
                                                    } else {
                                                        phi_30523_ = (0f >= _e5759);
                                                    }
                                                    let _e5775 = phi_30523_;
                                                    let _e5777 = sqrt(select(_e5759, 0f, _e5775));
                                                    let _e5778 = vec3<f32>(_e5765, _e5771, _e5777);
                                                    let _e5782 = ((_e5741 - _e5708) * 0.5f);
                                                    let _e5783 = ((_e5747 - _e5709) * 0.5f);
                                                    let _e5784 = ((_e5753 - _e5710) * 0.5f);
                                                    if (_e5782 != _e5782) {
                                                        phi_30550_ = true;
                                                    } else {
                                                        phi_30550_ = (0f >= _e5782);
                                                    }
                                                    let _e5788 = phi_30550_;
                                                    let _e5790 = sqrt(select(_e5782, 0f, _e5788));
                                                    if (_e5783 != _e5783) {
                                                        phi_30565_ = true;
                                                    } else {
                                                        phi_30565_ = (0f >= _e5783);
                                                    }
                                                    let _e5794 = phi_30565_;
                                                    let _e5796 = sqrt(select(_e5783, 0f, _e5794));
                                                    if (_e5784 != _e5784) {
                                                        phi_30580_ = true;
                                                    } else {
                                                        phi_30580_ = (0f >= _e5784);
                                                    }
                                                    let _e5800 = phi_30580_;
                                                    let _e5802 = sqrt(select(_e5784, 0f, _e5800));
                                                    if ((_e5713 / _e5698) < 0f) {
                                                        phi_18431_ = -(_e5790);
                                                    } else {
                                                        phi_18431_ = _e5790;
                                                    }
                                                    let _e5806 = phi_18431_;
                                                    if ((_e5714 / _e5699) < 0f) {
                                                        phi_18439_ = -(_e5796);
                                                    } else {
                                                        phi_18439_ = _e5796;
                                                    }
                                                    let _e5810 = phi_18439_;
                                                    if ((_e5715 / _e5700) < 0f) {
                                                        phi_18447_ = -(_e5802);
                                                    } else {
                                                        phi_18447_ = _e5802;
                                                    }
                                                    let _e5814 = phi_18447_;
                                                    let _e5818 = (_e1923.x * _e5668);
                                                    let _e5819 = (_e1923.y * _e5668);
                                                    let _e5820 = (_e1923.z * _e5668);
                                                    let _e5830 = vec3<f32>(((_e1925.x * _e5765) - (_e1923.x * _e5806)), ((_e1925.y * _e5771) - (_e1923.y * _e5810)), ((_e1925.z * _e5777) - (_e1923.z * _e5814)));
                                                    let _e5837 = ((_e1925.x * _e5806) + (_e1923.x * _e5765));
                                                    let _e5838 = ((_e1925.y * _e5810) + (_e1923.y * _e5771));
                                                    let _e5839 = ((_e1925.z * _e5814) + (_e1923.z * _e5777));
                                                    let _e5841 = vec3<f32>((_e1925.x * _e5668), (_e1925.y * _e5668), (_e1925.z * _e5668));
                                                    let _e5842 = (_e5841 - _e5778);
                                                    let _e5843 = (_e5818 - _e5806);
                                                    let _e5844 = (_e5819 - _e5810);
                                                    let _e5845 = (_e5820 - _e5814);
                                                    let _e5858 = (_e5841 + _e5778);
                                                    let _e5859 = (_e5818 + _e5806);
                                                    let _e5860 = (_e5819 + _e5810);
                                                    let _e5861 = (_e5820 + _e5814);
                                                    let _e5871 = ((_e5858.x * _e5858.x) + (_e5859 * _e5859));
                                                    let _e5872 = ((_e5858.y * _e5858.y) + (_e5860 * _e5860));
                                                    let _e5873 = ((_e5858.z * _e5858.z) + (_e5861 * _e5861));
                                                    let _e5883 = vec3(_e5668);
                                                    let _e5884 = (_e5883 - _e5830);
                                                    let _e5885 = -(vec3<f32>(_e5837, _e5838, _e5839));
                                                    let _e5901 = (_e5883 + _e5830);
                                                    let _e5911 = ((_e5901.x * _e5901.x) + (_e5837 * _e5837));
                                                    let _e5912 = ((_e5901.y * _e5901.y) + (_e5838 * _e5838));
                                                    let _e5913 = ((_e5901.z * _e5901.z) + (_e5839 * _e5839));
                                                    phi_18554_ = vec3<f32>((((((_e5842.x * _e5842.x) + (_e5843 * _e5843)) / select(0.000000000000000000000000000001f, _e5871, (_e5871 > 0.000000000000000000000000000001f))) + (((_e5884.x * _e5884.x) + (_e5885.x * _e5885.x)) / select(0.000000000000000000000000000001f, _e5911, (_e5911 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5842.y * _e5842.y) + (_e5844 * _e5844)) / select(0.000000000000000000000000000001f, _e5872, (_e5872 > 0.000000000000000000000000000001f))) + (((_e5884.y * _e5884.y) + (_e5885.y * _e5885.y)) / select(0.000000000000000000000000000001f, _e5912, (_e5912 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5842.z * _e5842.z) + (_e5845 * _e5845)) / select(0.000000000000000000000000000001f, _e5873, (_e5873 > 0.000000000000000000000000000001f))) + (((_e5884.z * _e5884.z) + (_e5885.z * _e5885.z)) / select(0.000000000000000000000000000001f, _e5913, (_e5913 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e5956 = phi_18554_;
                                                phi_18555_ = _e5956;
                                            }
                                            let _e5976 = phi_18555_;
                                            let _e5978 = (4f * _e5590);
                                            phi_18567_ = type_60((_e5976 * (((1f / (_e5641 + _e5655)) * _e5626) / _e5978)), ((_e5626 / _e5641) / _e5978));
                                        }
                                        let _e5985 = phi_18567_;
                                        phi_18568_ = _e5985;
                                        phi_18569_ = _e5602;
                                    }
                                    let _e5987 = phi_18568_;
                                    let _e5989 = phi_18569_;
                                    if _e5989 {
                                        phi_18574_ = type_60(vec3<f32>(0f, 0f, 0f), 0f);
                                    } else {
                                        phi_18574_ = _e5987;
                                    }
                                    let _e5991 = phi_18574_;
                                    let _e5994 = (_e1931 == 1u);
                                    if _e5994 {
                                        phi_18602_ = _e5991;
                                        phi_18603_ = type_60();
                                    } else {
                                        if (_e5590 != _e5590) {
                                            phi_30842_ = true;
                                        } else {
                                            phi_30842_ = (0f >= _e5590);
                                        }
                                        let _e5998 = phi_30842_;
                                        let _e5999 = select(_e5590, 0f, _e5998);
                                        let _e6000 = (1f / _e1921);
                                        let _e6004 = ((_e6000 * _e6000) * (1f - (_e5999 * _e5999)));
                                        if (_e6004 >= 1f) {
                                            phi_30861_ = 1f;
                                        } else {
                                            let _e6007 = sqrt((1f - _e6004));
                                            if (_e5999 != _e5999) {
                                                phi_30872_ = true;
                                            } else {
                                                phi_30872_ = (_e6007 <= _e5999);
                                            }
                                            let _e6011 = phi_30872_;
                                            let _e6016 = pow(((1f - _e1921) / (1f + _e1921)), 2f);
                                            phi_30861_ = (_e6016 + ((1f - _e6016) * pow((1f - select(_e5999, _e6007, _e6011)), 5f)));
                                        }
                                        let _e6023 = phi_30861_;
                                        if (_e5569 <= 0f) {
                                            phi_30911_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_30911_ = (_e1919 * (_e5569 * 0.31830987f));
                                        }
                                        let _e6028 = phi_30911_;
                                        let _e6029 = (1f - _e6023);
                                        phi_18602_ = type_60();
                                        phi_18603_ = type_60((_e5991.member + vec3<f32>((_e6028.x * _e6029), (_e6028.y * _e6029), (_e6028.z * _e6029))), ((_e6023 * _e5991.member_1) + (_e6029 * _e5575)));
                                    }
                                    let _e6043 = phi_18602_;
                                    let _e6045 = phi_18603_;
                                    phi_18604_ = _e6043;
                                    phi_18605_ = _e6045;
                                    phi_18606_ = _e5994;
                                }
                                let _e6053 = phi_18604_;
                                let _e6055 = phi_18605_;
                                let _e6057 = phi_18606_;
                                if _e6057 {
                                    phi_18610_ = _e6053;
                                } else {
                                    phi_18610_ = _e6055;
                                }
                                let _e6059 = phi_18610_;
                                if ((((_e5561.x * _e5561.x) + (_e5561.y * _e5561.y)) + (_e5561.z * _e5561.z)) == 0f) {
                                    phi_37208_ = _e5324;
                                    phi_18749_ = type_70();
                                    phi_18750_ = type_70();
                                    phi_18751_ = true;
                                    phi_18752_ = false;
                                } else {
                                    let _e6068 = (_e6059.member_1 <= 0f);
                                    if _e6068 {
                                        phi_37209_ = _e5324;
                                        phi_18746_ = type_70();
                                        phi_18747_ = type_70();
                                        phi_18748_ = false;
                                    } else {
                                        let _e6070 = (_e6059.member.x / _e6059.member_1);
                                        let _e6072 = (_e6059.member.y / _e6059.member_1);
                                        let _e6074 = (_e6059.member.z / _e6059.member_1);
                                        let _e6083 = global_1.member.member_4[1u];
                                        if (sqrt((((_e6070 * _e6070) + (_e6072 * _e6072)) + (_e6074 * _e6074))) <= _e6083) {
                                            phi_37202_ = _e5324;
                                            phi_18697_ = _e820;
                                            phi_18698_ = false;
                                        } else {
                                            let _e6087 = global_1.member.member_7[1u];
                                            if (_e6087 == 1u) {
                                                let _e6092 = global_1.member.member_4[3u];
                                                if (f32(_e356) >= _e6092) {
                                                    let _e6095 = select(_e6072, _e6070, (_e6070 > _e6072));
                                                    let _e6097 = select(_e6074, _e6095, (_e6095 > _e6074));
                                                    if (_e6097 != _e6097) {
                                                        phi_30958_ = true;
                                                    } else {
                                                        phi_30958_ = (1f <= _e6097);
                                                    }
                                                    let _e6101 = phi_30958_;
                                                    let _e6102 = select(_e6097, 1f, _e6101);
                                                    let _e6104 = ((_e5324 * 747796405u) + 2891336453u);
                                                    let _e6112 = (((_e6104 >> bitcast<u32>((((_e6104 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6104) * 277803737u);
                                                    let _e6115 = ((_e6112 >> bitcast<u32>(22i)) ^ _e6112);
                                                    let _e6120 = ((f32((_e6115 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6102);
                                                    if _e6120 {
                                                        phi_18686_ = vec3<f32>();
                                                    } else {
                                                        phi_18686_ = vec3<f32>((_e820.x / _e6102), (_e820.y / _e6102), (_e820.z / _e6102));
                                                    }
                                                    let _e6129 = phi_18686_;
                                                    phi_37205_ = _e6115;
                                                    phi_18688_ = _e6129;
                                                    phi_18689_ = select(true, false, _e6120);
                                                } else {
                                                    phi_37205_ = _e5324;
                                                    phi_18688_ = _e820;
                                                    phi_18689_ = true;
                                                }
                                                let _e6132 = phi_37205_;
                                                let _e6134 = phi_18688_;
                                                let _e6136 = phi_18689_;
                                                phi_37204_ = _e6132;
                                                phi_18690_ = _e6134;
                                                phi_18691_ = _e6136;
                                            } else {
                                                phi_37204_ = _e5324;
                                                phi_18690_ = _e820;
                                                phi_18691_ = true;
                                            }
                                            let _e6138 = phi_37204_;
                                            let _e6140 = phi_18690_;
                                            let _e6142 = phi_18691_;
                                            phi_37202_ = _e6138;
                                            phi_18697_ = select(_e820, _e6140, vec3(_e6142));
                                            phi_18698_ = select(false, true, _e6142);
                                        }
                                        let _e6147 = phi_37202_;
                                        let _e6149 = phi_18697_;
                                        let _e6151 = phi_18698_;
                                        if _e6151 {
                                            if ((((_e5561.x * _e1027.x) + (_e5561.y * _e1027.y)) + (_e5561.z * _e1027.z)) < 0f) {
                                                phi_18727_ = -(_e1027);
                                            } else {
                                                phi_18727_ = _e1027;
                                            }
                                            let _e6190 = phi_18727_;
                                            let _e6207 = local_88;
                                            let _e6210 = local_89;
                                            let _e6213 = local_90;
                                            let _e6216 = local_91;
                                            let _e6222 = local_92;
                                            phi_18743_ = type_70(type_35(vec4<f32>((_e826 + (_e6190.x * _e1057)), (_e827 + (_e6190.y * _e1057)), (_e828 + (_e6190.z * _e1057)), _e331.member.w), vec4<f32>(_e5561.x, _e5561.y, _e5561.z, _e6059.member_1), vec4<f32>((_e6149.x * _e6070), (_e6149.y * _e6072), (_e6149.z * _e6074), f32((_e356 + 1u))), vec4<f32>(_e6210.x, _e6213.y, _e6216.z, f32(_e6207)), vec4<u32>(_e6147, _e1939, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6222);
                                            phi_18744_ = type_70();
                                        } else {
                                            let _e6159 = local_83;
                                            let _e6162 = local_84;
                                            let _e6165 = local_85;
                                            let _e6168 = local_86;
                                            let _e6174 = local_87;
                                            phi_18743_ = type_70();
                                            phi_18744_ = type_70(type_35(_e331.member, _e331.member_1, vec4<f32>(_e6149.x, _e6149.y, _e6149.z, f32(_e356)), vec4<f32>(_e6162.x, _e6165.y, _e6168.z, f32(_e6159)), vec4<u32>(_e6147, (_e1939 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6174);
                                        }
                                        let _e6225 = phi_18743_;
                                        let _e6227 = phi_18744_;
                                        phi_37209_ = _e6147;
                                        phi_18746_ = _e6225;
                                        phi_18747_ = _e6227;
                                        phi_18748_ = select(true, false, _e6151);
                                    }
                                    let _e6230 = phi_37209_;
                                    let _e6232 = phi_18746_;
                                    let _e6234 = phi_18747_;
                                    let _e6236 = phi_18748_;
                                    phi_37208_ = _e6230;
                                    phi_18749_ = _e6232;
                                    phi_18750_ = _e6234;
                                    phi_18751_ = _e6068;
                                    phi_18752_ = _e6236;
                                }
                                let _e6238 = phi_37208_;
                                let _e6240 = phi_18749_;
                                let _e6242 = phi_18750_;
                                let _e6244 = phi_18751_;
                                let _e6246 = phi_18752_;
                                if _e6244 {
                                    let _e6254 = local_93;
                                    let _e6257 = local_94;
                                    let _e6260 = local_95;
                                    let _e6263 = local_96;
                                    let _e6269 = local_97;
                                    phi_18758_ = type_70(type_35(_e331.member, _e331.member_1, vec4<f32>(_e820.x, _e820.y, _e820.z, f32(_e356)), vec4<f32>(_e6257.x, _e6260.y, _e6263.z, f32(_e6254)), vec4<u32>(_e6238, (_e1939 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e822, _e6269);
                                } else {
                                    phi_18758_ = _e6242;
                                }
                                let _e6272 = phi_18758_;
                                phi_18763_ = _e6272;
                                phi_18764_ = _e6240;
                                phi_18765_ = select(_e6246, true, _e6244);
                            }
                            let _e6299 = phi_18763_;
                            let _e6301 = phi_18764_;
                            let _e6303 = phi_18765_;
                            phi_18769_ = _e6299;
                            phi_18770_ = _e6301;
                            phi_18771_ = _e6303;
                        } else {
                            phi_18769_ = _e1935;
                            phi_18770_ = type_70();
                            phi_18771_ = true;
                        }
                        let _e6305 = phi_18769_;
                        let _e6307 = phi_18770_;
                        let _e6309 = phi_18771_;
                        phi_18775_ = _e6305;
                        phi_18776_ = _e6307;
                        phi_18777_ = _e6309;
                    }
                    let _e6516 = phi_18775_;
                    let _e6518 = phi_18776_;
                    let _e6520 = phi_18777_;
                    phi_18781_ = _e6516;
                    phi_18782_ = _e6518;
                    phi_18783_ = _e6520;
                }
                let _e7028 = phi_18781_;
                let _e7030 = phi_18782_;
                let _e7032 = phi_18783_;
                if _e7032 {
                    phi_18787_ = _e7028;
                } else {
                    phi_18787_ = _e7030;
                }
                let _e7034 = phi_18787_;
                if _e328 {
                } else {
                    break;
                }
                global_7.member[_e323.member_1] = _e7034.member;
                let _e7038 = (2u * _e323.member_1);
                if (_e7038 < _e288) {
                } else {
                    break;
                }
                global_9.member[_e7038] = _e7034.member_1;
                let _e7042 = (_e7038 + 1u);
                if (_e7042 < _e288) {
                } else {
                    break;
                }
                global_9.member[_e7042] = _e7034.member_2;
                if ((_e7034.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7050 = (1u - _e292);
                    let _e7051 = (_e7050 * 4u);
                    let _e7055 = atomicAdd((&global_8.member[(_e7051 + 3u)]), 1u);
                    let _e7059 = global_1.member.member[0u];
                    let _e7064 = global_1.member.member[1u];
                    atomicStore((&global_8.member[((8u + ((_e7050 * _e7059) * _e7064)) + _e7055)]), _e323.member_1);
                    if ((_e7055 % 64u) == 0u) {
                        let _e7072 = (_e7055 / 64u);
                        if (_e7072 < 65535u) {
                            let _e7076 = atomicAdd((&global_8.member[_e7051]), 1u);
                        }
                        if ((_e7072 % 65535u) == 0u) {
                            let _e7082 = atomicAdd((&global_8.member[(_e7051 + 1u)]), 1u);
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
    var phi_19828_: type_49;
    var phi_19881_: u32;
    var phi_19884_: u32;
    var phi_19886_: u32;
    var phi_19888_: u32;
    var phi_19890_: u32;
    var phi_19892_: vec4<f32>;
    var phi_19894_: f32;
    var phi_20246_: u32;
    var phi_20248_: u32;
    var phi_20249_: u32;
    var phi_20251_: bool;
    var phi_20252_: bool;
    var phi_20253_: u32;
    var phi_20254_: u32;
    var phi_20255_: u32;
    var phi_20256_: bool;
    var phi_20257_: bool;
    var phi_20258_: bool;
    var phi_31631_: bool;
    var phi_31661_: bool;
    var phi_31608_: type_57;
    var phi_31611_: type_57;
    var phi_31612_: bool;
    var phi_31617_: type_57;
    var phi_31718_: bool;
    var phi_31733_: bool;
    var phi_31748_: bool;
    var phi_31763_: bool;
    var phi_31695_: type_57;
    var phi_31698_: type_57;
    var phi_31699_: bool;
    var phi_31704_: type_57;
    var phi_31805_: bool;
    var phi_31820_: bool;
    var phi_31835_: bool;
    var phi_31850_: bool;
    var phi_31782_: type_57;
    var phi_31785_: type_57;
    var phi_31786_: bool;
    var phi_31791_: type_57;
    var phi_31586_: bool;
    var phi_31589_: bool;
    var phi_20287_: f32;
    var phi_31952_: bool;
    var phi_31982_: bool;
    var phi_31929_: type_57;
    var phi_31932_: type_57;
    var phi_31933_: bool;
    var phi_31938_: type_57;
    var phi_32039_: bool;
    var phi_32054_: bool;
    var phi_32069_: bool;
    var phi_32084_: bool;
    var phi_32016_: type_57;
    var phi_32019_: type_57;
    var phi_32020_: bool;
    var phi_32025_: type_57;
    var phi_32126_: bool;
    var phi_32141_: bool;
    var phi_32156_: bool;
    var phi_32171_: bool;
    var phi_32103_: type_57;
    var phi_32106_: type_57;
    var phi_32107_: bool;
    var phi_32112_: type_57;
    var phi_31907_: bool;
    var phi_31910_: bool;
    var phi_20296_: f32;
    var phi_20307_: u32;
    var phi_20318_: u32;
    var phi_20357_: u32;
    var phi_20358_: u32;
    var phi_20369_: u32;
    var phi_20370_: u32;
    var phi_20371_: u32;
    var phi_20372_: u32;
    var phi_20374_: u32;
    var phi_20375_: u32;
    var phi_20377_: u32;
    var phi_20378_: u32;
    var phi_20379_: bool;
    var phi_20380_: bool;
    var phi_20172_: vec4<f32>;
    var phi_20173_: vec4<f32>;
    var phi_20174_: bool;
    var phi_20178_: vec4<f32>;
    var phi_20179_: bool;
    var phi_20089_: bool;
    var phi_20123_: bool;
    var phi_20129_: f32;
    var phi_20139_: vec4<f32>;
    var phi_20140_: vec4<f32>;
    var phi_20141_: bool;
    var phi_20145_: vec4<f32>;
    var phi_20146_: bool;
    var phi_20150_: vec4<f32>;
    var phi_20151_: bool;
    var phi_20180_: vec4<f32>;
    var phi_20181_: bool;
    var phi_20021_: vec4<f32>;
    var phi_20022_: vec4<f32>;
    var phi_20023_: bool;
    var phi_20027_: vec4<f32>;
    var phi_20028_: bool;
    var phi_20029_: vec4<f32>;
    var phi_20030_: bool;
    var phi_20031_: bool;
    var phi_20039_: vec4<f32>;
    var phi_20040_: bool;
    var phi_20041_: vec4<f32>;
    var phi_20042_: bool;
    var phi_20043_: bool;
    var phi_20051_: vec4<f32>;
    var phi_20052_: bool;
    var phi_20190_: vec4<f32>;
    var phi_20191_: bool;
    var phi_20212_: f32;
    var phi_19882_: u32;
    var phi_19885_: u32;
    var phi_19887_: u32;
    var phi_19889_: u32;
    var phi_19891_: u32;
    var phi_19893_: vec4<f32>;
    var phi_19895_: f32;
    var phi_20391_: vec4<f32>;
    var phi_20392_: bool;
    var phi_20393_: bool;
    var phi_37261_: bool;
    var phi_20573_: bool;
    var phi_20572_: vec4<f32>;
    var local_105: u32;
    var local_106: vec4<f32>;
    var phi_20398_: type_76;
    var local_107: u32;
    var phi_20404_: type_76;

    switch bitcast<i32>(0u) {
        default: {
            let _e278 = global;
            let _e287 = global_1.member.member_7[2u];
            let _e291 = (_e278.x + (_e278.y * 4194240u));
            let _e296 = atomicLoad((&global_8.member[((_e287 * 4u) + 3u)]));
            if (_e291 >= _e296) {
                phi_19828_ = type_49(0u, type_49().member_1);
            } else {
                let _e301 = global_1.member.member[0u];
                let _e306 = global_1.member.member[1u];
                let _e312 = atomicLoad((&global_8.member[((8u + ((_e287 * _e301) * _e306)) + _e291)]));
                phi_19828_ = type_49(1u, _e312);
            }
            let _e318 = phi_19828_;
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
                phi_19881_ = 4294967295u;
                phi_19884_ = select(4294967295u, 0u, (_e342 > 0u));
                phi_19886_ = 0u;
                phi_19888_ = (_e342 + _e345);
                phi_19890_ = _e342;
                phi_19892_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_19894_ = 300000000000000000000000000000000000000f;
                loop {
                    let _e352 = phi_19881_;
                    let _e354 = phi_19884_;
                    let _e356 = phi_19886_;
                    let _e358 = phi_19888_;
                    let _e360 = phi_19890_;
                    let _e362 = phi_19892_;
                    let _e364 = phi_19894_;
                    local_105 = _e352;
                    local_106 = _e362;
                    local_107 = _e360;
                    if (_e360 < _e358) {
                        let _e728 = global_2.member[_e360];
                        if (_e728.member_6.x == 0u) {
                            let _e834 = (_e728.member_1.x - _e728.member.x);
                            let _e835 = (_e728.member_1.y - _e728.member.y);
                            let _e836 = (_e728.member_1.z - _e728.member.z);
                            let _e841 = (_e728.member_2.x - _e728.member.x);
                            let _e842 = (_e728.member_2.y - _e728.member.y);
                            let _e843 = (_e728.member_2.z - _e728.member.z);
                            let _e846 = ((_e336.y * _e843) - (_e842 * _e336.z));
                            let _e849 = ((_e336.z * _e841) - (_e843 * _e336.x));
                            let _e852 = ((_e336.x * _e842) - (_e841 * _e336.y));
                            let _e857 = (((_e834 * _e846) + (_e835 * _e849)) + (_e836 * _e852));
                            if (abs(_e857) < 0.000000000001f) {
                                phi_20051_ = vec4<f32>();
                                phi_20052_ = true;
                            } else {
                                let _e860 = (1f / _e857);
                                let _e861 = (_e329.x - _e728.member.x);
                                let _e862 = (_e329.y - _e728.member.y);
                                let _e863 = (_e329.z - _e728.member.z);
                                let _e869 = ((((_e861 * _e846) + (_e862 * _e849)) + (_e863 * _e852)) * _e860);
                                if (_e869 < 0f) {
                                    phi_20041_ = vec4<f32>();
                                    phi_20042_ = true;
                                    phi_20043_ = false;
                                } else {
                                    let _e871 = (_e869 > 1f);
                                    if _e871 {
                                        phi_20039_ = vec4<f32>();
                                        phi_20040_ = false;
                                    } else {
                                        let _e874 = ((_e862 * _e836) - (_e835 * _e863));
                                        let _e877 = ((_e863 * _e834) - (_e836 * _e861));
                                        let _e880 = ((_e861 * _e835) - (_e834 * _e862));
                                        let _e886 = ((((_e336.x * _e874) + (_e336.y * _e877)) + (_e336.z * _e880)) * _e860);
                                        if (_e886 < 0f) {
                                            phi_20029_ = vec4<f32>();
                                            phi_20030_ = true;
                                            phi_20031_ = false;
                                        } else {
                                            let _e889 = ((_e869 + _e886) > 1f);
                                            if _e889 {
                                                phi_20027_ = vec4<f32>();
                                                phi_20028_ = false;
                                            } else {
                                                let _e895 = ((((_e841 * _e874) + (_e842 * _e877)) + (_e843 * _e880)) * _e860);
                                                if (_e895 <= 0f) {
                                                    phi_20022_ = vec4<f32>();
                                                    phi_20023_ = true;
                                                } else {
                                                    let _e897 = (_e895 >= _e364);
                                                    if _e897 {
                                                        phi_20021_ = vec4<f32>();
                                                    } else {
                                                        phi_20021_ = vec4<f32>(_e895, _e869, _e886, 1f);
                                                    }
                                                    let _e900 = phi_20021_;
                                                    phi_20022_ = _e900;
                                                    phi_20023_ = _e897;
                                                }
                                                let _e902 = phi_20022_;
                                                let _e904 = phi_20023_;
                                                phi_20027_ = _e902;
                                                phi_20028_ = _e904;
                                            }
                                            let _e906 = phi_20027_;
                                            let _e908 = phi_20028_;
                                            phi_20029_ = _e906;
                                            phi_20030_ = _e889;
                                            phi_20031_ = _e908;
                                        }
                                        let _e910 = phi_20029_;
                                        let _e912 = phi_20030_;
                                        let _e914 = phi_20031_;
                                        phi_20039_ = _e910;
                                        phi_20040_ = select(_e914, true, _e912);
                                    }
                                    let _e917 = phi_20039_;
                                    let _e919 = phi_20040_;
                                    phi_20041_ = _e917;
                                    phi_20042_ = _e871;
                                    phi_20043_ = _e919;
                                }
                                let _e921 = phi_20041_;
                                let _e923 = phi_20042_;
                                let _e925 = phi_20043_;
                                phi_20051_ = _e921;
                                phi_20052_ = select(_e925, true, _e923);
                            }
                            let _e928 = phi_20051_;
                            let _e930 = phi_20052_;
                            phi_20190_ = _e928;
                            phi_20191_ = _e930;
                        } else {
                            if (_e728.member_6.x == 1u) {
                                let _e756 = (_e329.x - _e728.member.x);
                                let _e757 = (_e329.y - _e728.member.y);
                                let _e758 = (_e329.z - _e728.member.z);
                                let _e763 = (((_e756 * _e336.x) + (_e757 * _e336.y)) + (_e758 * _e336.z));
                                if (_e763 > 0f) {
                                    phi_20089_ = select(true, false, ((((_e756 * _e756) + (_e757 * _e757)) + (_e758 * _e758)) > (_e728.member.w * _e728.member.w)));
                                } else {
                                    phi_20089_ = true;
                                }
                                let _e776 = phi_20089_;
                                if _e776 {
                                    let _e780 = (_e756 - (_e336.x * _e763));
                                    let _e781 = (_e757 - (_e336.y * _e763));
                                    let _e782 = (_e758 - (_e336.z * _e763));
                                    let _e791 = ((_e728.member.w * _e728.member.w) - (((_e780 * _e780) + (_e781 * _e781)) + (_e782 * _e782)));
                                    if (_e791 < 0f) {
                                        phi_20145_ = vec4<f32>();
                                        phi_20146_ = true;
                                    } else {
                                        let _e793 = sqrt(_e791);
                                        let _e794 = -(_e763);
                                        let _e795 = (_e794 - _e793);
                                        if (_e795 <= 0f) {
                                            phi_20123_ = true;
                                        } else {
                                            phi_20123_ = (_e795 >= _e364);
                                        }
                                        let _e799 = phi_20123_;
                                        if _e799 {
                                            phi_20129_ = (_e794 + _e793);
                                        } else {
                                            phi_20129_ = _e795;
                                        }
                                        let _e802 = phi_20129_;
                                        if (_e802 <= 0f) {
                                            phi_20140_ = vec4<f32>();
                                            phi_20141_ = true;
                                        } else {
                                            let _e804 = (_e802 >= _e364);
                                            if _e804 {
                                                phi_20139_ = vec4<f32>();
                                            } else {
                                                phi_20139_ = vec4<f32>(_e802, 0f, 0f, 1f);
                                            }
                                            let _e807 = phi_20139_;
                                            phi_20140_ = _e807;
                                            phi_20141_ = _e804;
                                        }
                                        let _e809 = phi_20140_;
                                        let _e811 = phi_20141_;
                                        phi_20145_ = _e809;
                                        phi_20146_ = _e811;
                                    }
                                    let _e813 = phi_20145_;
                                    let _e815 = phi_20146_;
                                    phi_20150_ = _e813;
                                    phi_20151_ = _e815;
                                } else {
                                    phi_20150_ = vec4<f32>();
                                    phi_20151_ = true;
                                }
                                let _e817 = phi_20150_;
                                let _e819 = phi_20151_;
                                phi_20180_ = _e817;
                                phi_20181_ = _e819;
                            } else {
                                if (abs(_e336.y) < 0.000000000001f) {
                                    phi_20178_ = vec4<f32>();
                                    phi_20179_ = true;
                                } else {
                                    let _e738 = ((_e728.member.x - _e329.y) / _e336.y);
                                    if (_e738 <= 0f) {
                                        phi_20173_ = vec4<f32>();
                                        phi_20174_ = true;
                                    } else {
                                        let _e740 = (_e738 >= _e364);
                                        if _e740 {
                                            phi_20172_ = vec4<f32>();
                                        } else {
                                            phi_20172_ = vec4<f32>(_e738, 0f, 0f, 1f);
                                        }
                                        let _e743 = phi_20172_;
                                        phi_20173_ = _e743;
                                        phi_20174_ = _e740;
                                    }
                                    let _e745 = phi_20173_;
                                    let _e747 = phi_20174_;
                                    phi_20178_ = _e745;
                                    phi_20179_ = _e747;
                                }
                                let _e749 = phi_20178_;
                                let _e751 = phi_20179_;
                                phi_20180_ = _e749;
                                phi_20181_ = _e751;
                            }
                            let _e821 = phi_20180_;
                            let _e823 = phi_20181_;
                            phi_20190_ = _e821;
                            phi_20191_ = select(true, false, select(true, false, _e823));
                        }
                        let _e932 = phi_20190_;
                        let _e934 = phi_20191_;
                        let _e937 = select(vec4<f32>(0f, 0f, 0f, 0f), _e932, vec4(select(true, false, _e934)));
                        let _e939 = (_e937.w > 0f);
                        if _e939 {
                            phi_20212_ = _e937.x;
                        } else {
                            phi_20212_ = _e364;
                        }
                        let _e942 = phi_20212_;
                        phi_19882_ = select(_e352, _e360, _e939);
                        phi_19885_ = _e354;
                        phi_19887_ = _e356;
                        phi_19889_ = _e358;
                        phi_19891_ = (_e360 + 1u);
                        phi_19893_ = select(_e362, _e937, vec4(_e939));
                        phi_19895_ = _e942;
                        phi_20391_ = _e937;
                        phi_20392_ = true;
                        phi_20393_ = false;
                    } else {
                        if (_e354 == 4294967295u) {
                            let _e367 = (_e356 == 0u);
                            if _e367 {
                                phi_20248_ = u32();
                                phi_20249_ = u32();
                                phi_20251_ = false;
                                phi_20252_ = false;
                            } else {
                                let _e368 = (_e356 - 1u);
                                let _e369 = (_e368 < 64u);
                                if _e369 {
                                } else {
                                    phi_37261_ = true;
                                    phi_20573_ = bool();
                                    phi_20572_ = vec4<f32>();
                                    break;
                                }
                                let _e371 = local_104[_e368];
                                let _e372 = (_e371 >= _e364);
                                if _e372 {
                                    phi_20246_ = u32();
                                } else {
                                    if _e369 {
                                    } else {
                                        phi_37261_ = true;
                                        phi_20573_ = bool();
                                        phi_20572_ = vec4<f32>();
                                        break;
                                    }
                                    let _e374 = local_103[_e368];
                                    phi_20246_ = _e374;
                                }
                                let _e376 = phi_20246_;
                                phi_20248_ = _e368;
                                phi_20249_ = _e376;
                                phi_20251_ = select(true, false, _e372);
                                phi_20252_ = _e372;
                            }
                            let _e379 = phi_20248_;
                            let _e381 = phi_20249_;
                            let _e383 = phi_20251_;
                            let _e385 = phi_20252_;
                            phi_20253_ = _e379;
                            phi_20254_ = _e381;
                            phi_20255_ = _e379;
                            phi_20256_ = _e383;
                            phi_20257_ = _e385;
                            phi_20258_ = _e367;
                        } else {
                            phi_20253_ = u32();
                            phi_20254_ = _e354;
                            phi_20255_ = _e356;
                            phi_20256_ = true;
                            phi_20257_ = false;
                            phi_20258_ = false;
                        }
                        let _e387 = phi_20253_;
                        let _e389 = phi_20254_;
                        let _e391 = phi_20255_;
                        let _e393 = phi_20256_;
                        let _e395 = phi_20257_;
                        let _e397 = phi_20258_;
                        if _e393 {
                            let _e399 = ((_e389 & 2147483648u) == 0u);
                            if _e399 {
                                let _e407 = global_3.member[_e389];
                                if (_e407.member_1 == 4294967295u) {
                                    phi_20287_ = 300000000000000000000000000000000000000f;
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
                                                phi_31631_ = true;
                                            } else {
                                                phi_31631_ = (_e430 <= _e419);
                                            }
                                            let _e438 = phi_31631_;
                                            let _e439 = select(_e419, _e430, _e438);
                                            if _e435 {
                                                phi_31661_ = true;
                                            } else {
                                                phi_31661_ = (_e430 >= _e419);
                                            }
                                            let _e444 = phi_31661_;
                                            let _e445 = select(_e419, _e430, _e444);
                                            phi_31608_ = type_57(select(-300000000000000000000000000000000000000f, _e439, (_e439 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e445, (_e445 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_31608_ = type_57();
                                        }
                                        let _e450 = phi_31608_;
                                        phi_31611_ = _e450;
                                        phi_31612_ = select(true, false, _e434);
                                    } else {
                                        phi_31611_ = type_57();
                                        phi_31612_ = true;
                                    }
                                    let _e453 = phi_31611_;
                                    let _e455 = phi_31612_;
                                    if _e455 {
                                        phi_31617_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_31617_ = _e453;
                                    }
                                    let _e457 = phi_31617_;
                                    if (_e421 == _e421) {
                                        let _e461 = (_e431 == _e431);
                                        if _e461 {
                                            let _e462 = (_e421 != _e421);
                                            if _e462 {
                                                phi_31718_ = true;
                                            } else {
                                                phi_31718_ = (_e431 <= _e421);
                                            }
                                            let _e465 = phi_31718_;
                                            let _e466 = select(_e421, _e431, _e465);
                                            if (_e457.member != _e457.member) {
                                                phi_31733_ = true;
                                            } else {
                                                phi_31733_ = (_e466 >= _e457.member);
                                            }
                                            let _e470 = phi_31733_;
                                            if _e462 {
                                                phi_31748_ = true;
                                            } else {
                                                phi_31748_ = (_e431 >= _e421);
                                            }
                                            let _e474 = phi_31748_;
                                            let _e475 = select(_e421, _e431, _e474);
                                            if (_e457.member_1 != _e457.member_1) {
                                                phi_31763_ = true;
                                            } else {
                                                phi_31763_ = (_e475 <= _e457.member_1);
                                            }
                                            let _e479 = phi_31763_;
                                            phi_31695_ = type_57(select(_e457.member, _e466, _e470), select(_e457.member_1, _e475, _e479));
                                        } else {
                                            phi_31695_ = type_57();
                                        }
                                        let _e483 = phi_31695_;
                                        phi_31698_ = _e483;
                                        phi_31699_ = select(true, false, _e461);
                                    } else {
                                        phi_31698_ = type_57();
                                        phi_31699_ = true;
                                    }
                                    let _e486 = phi_31698_;
                                    let _e488 = phi_31699_;
                                    if _e488 {
                                        phi_31704_ = _e457;
                                    } else {
                                        phi_31704_ = _e486;
                                    }
                                    let _e490 = phi_31704_;
                                    if (_e423 == _e423) {
                                        let _e494 = (_e432 == _e432);
                                        if _e494 {
                                            let _e495 = (_e423 != _e423);
                                            if _e495 {
                                                phi_31805_ = true;
                                            } else {
                                                phi_31805_ = (_e432 <= _e423);
                                            }
                                            let _e498 = phi_31805_;
                                            let _e499 = select(_e423, _e432, _e498);
                                            if (_e490.member != _e490.member) {
                                                phi_31820_ = true;
                                            } else {
                                                phi_31820_ = (_e499 >= _e490.member);
                                            }
                                            let _e503 = phi_31820_;
                                            if _e495 {
                                                phi_31835_ = true;
                                            } else {
                                                phi_31835_ = (_e432 >= _e423);
                                            }
                                            let _e507 = phi_31835_;
                                            let _e508 = select(_e423, _e432, _e507);
                                            if (_e490.member_1 != _e490.member_1) {
                                                phi_31850_ = true;
                                            } else {
                                                phi_31850_ = (_e508 <= _e490.member_1);
                                            }
                                            let _e512 = phi_31850_;
                                            phi_31782_ = type_57(select(_e490.member, _e499, _e503), select(_e490.member_1, _e508, _e512));
                                        } else {
                                            phi_31782_ = type_57();
                                        }
                                        let _e516 = phi_31782_;
                                        phi_31785_ = _e516;
                                        phi_31786_ = select(true, false, _e494);
                                    } else {
                                        phi_31785_ = type_57();
                                        phi_31786_ = true;
                                    }
                                    let _e519 = phi_31785_;
                                    let _e521 = phi_31786_;
                                    if _e521 {
                                        phi_31791_ = _e490;
                                    } else {
                                        phi_31791_ = _e519;
                                    }
                                    let _e523 = phi_31791_;
                                    if (_e523.member_1 < 0f) {
                                        phi_31589_ = true;
                                    } else {
                                        if (_e523.member > _e523.member_1) {
                                            phi_31586_ = true;
                                        } else {
                                            phi_31586_ = (_e523.member > _e364);
                                        }
                                        let _e530 = phi_31586_;
                                        phi_31589_ = _e530;
                                    }
                                    let _e532 = phi_31589_;
                                    phi_20287_ = select(_e523.member, 300000000000000000000000000000000000000f, _e532);
                                }
                                let _e535 = phi_20287_;
                                if (_e407.member_5 == 4294967295u) {
                                    phi_20296_ = 300000000000000000000000000000000000000f;
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
                                                phi_31952_ = true;
                                            } else {
                                                phi_31952_ = (_e558 <= _e547);
                                            }
                                            let _e566 = phi_31952_;
                                            let _e567 = select(_e547, _e558, _e566);
                                            if _e563 {
                                                phi_31982_ = true;
                                            } else {
                                                phi_31982_ = (_e558 >= _e547);
                                            }
                                            let _e572 = phi_31982_;
                                            let _e573 = select(_e547, _e558, _e572);
                                            phi_31929_ = type_57(select(-300000000000000000000000000000000000000f, _e567, (_e567 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e573, (_e573 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_31929_ = type_57();
                                        }
                                        let _e578 = phi_31929_;
                                        phi_31932_ = _e578;
                                        phi_31933_ = select(true, false, _e562);
                                    } else {
                                        phi_31932_ = type_57();
                                        phi_31933_ = true;
                                    }
                                    let _e581 = phi_31932_;
                                    let _e583 = phi_31933_;
                                    if _e583 {
                                        phi_31938_ = type_57(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_31938_ = _e581;
                                    }
                                    let _e585 = phi_31938_;
                                    if (_e549 == _e549) {
                                        let _e589 = (_e559 == _e559);
                                        if _e589 {
                                            let _e590 = (_e549 != _e549);
                                            if _e590 {
                                                phi_32039_ = true;
                                            } else {
                                                phi_32039_ = (_e559 <= _e549);
                                            }
                                            let _e593 = phi_32039_;
                                            let _e594 = select(_e549, _e559, _e593);
                                            if (_e585.member != _e585.member) {
                                                phi_32054_ = true;
                                            } else {
                                                phi_32054_ = (_e594 >= _e585.member);
                                            }
                                            let _e598 = phi_32054_;
                                            if _e590 {
                                                phi_32069_ = true;
                                            } else {
                                                phi_32069_ = (_e559 >= _e549);
                                            }
                                            let _e602 = phi_32069_;
                                            let _e603 = select(_e549, _e559, _e602);
                                            if (_e585.member_1 != _e585.member_1) {
                                                phi_32084_ = true;
                                            } else {
                                                phi_32084_ = (_e603 <= _e585.member_1);
                                            }
                                            let _e607 = phi_32084_;
                                            phi_32016_ = type_57(select(_e585.member, _e594, _e598), select(_e585.member_1, _e603, _e607));
                                        } else {
                                            phi_32016_ = type_57();
                                        }
                                        let _e611 = phi_32016_;
                                        phi_32019_ = _e611;
                                        phi_32020_ = select(true, false, _e589);
                                    } else {
                                        phi_32019_ = type_57();
                                        phi_32020_ = true;
                                    }
                                    let _e614 = phi_32019_;
                                    let _e616 = phi_32020_;
                                    if _e616 {
                                        phi_32025_ = _e585;
                                    } else {
                                        phi_32025_ = _e614;
                                    }
                                    let _e618 = phi_32025_;
                                    if (_e551 == _e551) {
                                        let _e622 = (_e560 == _e560);
                                        if _e622 {
                                            let _e623 = (_e551 != _e551);
                                            if _e623 {
                                                phi_32126_ = true;
                                            } else {
                                                phi_32126_ = (_e560 <= _e551);
                                            }
                                            let _e626 = phi_32126_;
                                            let _e627 = select(_e551, _e560, _e626);
                                            if (_e618.member != _e618.member) {
                                                phi_32141_ = true;
                                            } else {
                                                phi_32141_ = (_e627 >= _e618.member);
                                            }
                                            let _e631 = phi_32141_;
                                            if _e623 {
                                                phi_32156_ = true;
                                            } else {
                                                phi_32156_ = (_e560 >= _e551);
                                            }
                                            let _e635 = phi_32156_;
                                            let _e636 = select(_e551, _e560, _e635);
                                            if (_e618.member_1 != _e618.member_1) {
                                                phi_32171_ = true;
                                            } else {
                                                phi_32171_ = (_e636 <= _e618.member_1);
                                            }
                                            let _e640 = phi_32171_;
                                            phi_32103_ = type_57(select(_e618.member, _e627, _e631), select(_e618.member_1, _e636, _e640));
                                        } else {
                                            phi_32103_ = type_57();
                                        }
                                        let _e644 = phi_32103_;
                                        phi_32106_ = _e644;
                                        phi_32107_ = select(true, false, _e622);
                                    } else {
                                        phi_32106_ = type_57();
                                        phi_32107_ = true;
                                    }
                                    let _e647 = phi_32106_;
                                    let _e649 = phi_32107_;
                                    if _e649 {
                                        phi_32112_ = _e618;
                                    } else {
                                        phi_32112_ = _e647;
                                    }
                                    let _e651 = phi_32112_;
                                    if (_e651.member_1 < 0f) {
                                        phi_31910_ = true;
                                    } else {
                                        if (_e651.member > _e651.member_1) {
                                            phi_31907_ = true;
                                        } else {
                                            phi_31907_ = (_e651.member > _e364);
                                        }
                                        let _e658 = phi_31907_;
                                        phi_31910_ = _e658;
                                    }
                                    let _e660 = phi_31910_;
                                    phi_20296_ = select(_e651.member, 300000000000000000000000000000000000000f, _e660);
                                }
                                let _e663 = phi_20296_;
                                if (_e407.member_3 > 0u) {
                                    phi_20307_ = ((2147483648u | ((_e407.member_3 - 1u) << bitcast<u32>(28u))) | _e407.member_1);
                                } else {
                                    phi_20307_ = _e407.member_1;
                                }
                                let _e672 = phi_20307_;
                                if (_e407.member_7 > 0u) {
                                    phi_20318_ = ((2147483648u | ((_e407.member_7 - 1u) << bitcast<u32>(28u))) | _e407.member_5);
                                } else {
                                    phi_20318_ = _e407.member_5;
                                }
                                let _e681 = phi_20318_;
                                let _e682 = (_e535 <= _e663);
                                let _e686 = select(_e535, _e663, _e682);
                                if (_e686 < 300000000000000000000000000000000000000f) {
                                    let _e688 = (_e391 < 64u);
                                    if _e688 {
                                        if _e688 {
                                        } else {
                                            phi_37261_ = true;
                                            phi_20573_ = bool();
                                            phi_20572_ = vec4<f32>();
                                            break;
                                        }
                                        local_103[_e391] = select(_e672, _e681, _e682);
                                        if _e688 {
                                        } else {
                                            phi_37261_ = true;
                                            phi_20573_ = bool();
                                            phi_20572_ = vec4<f32>();
                                            break;
                                        }
                                        local_104[_e391] = _e686;
                                        phi_20357_ = (_e391 + 1u);
                                    } else {
                                        phi_20357_ = _e391;
                                    }
                                    let _e693 = phi_20357_;
                                    phi_20358_ = _e693;
                                } else {
                                    phi_20358_ = _e391;
                                }
                                let _e695 = phi_20358_;
                                phi_20369_ = select(4294967295u, select(_e681, _e672, _e682), (select(_e663, _e535, _e682) < 300000000000000000000000000000000000000f));
                                phi_20370_ = _e695;
                                phi_20371_ = u32();
                                phi_20372_ = u32();
                            } else {
                                let _e400 = (_e389 & 268435455u);
                                phi_20369_ = u32();
                                phi_20370_ = u32();
                                phi_20371_ = ((_e400 + ((_e389 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_20372_ = _e400;
                            }
                            let _e699 = phi_20369_;
                            let _e701 = phi_20370_;
                            let _e703 = phi_20371_;
                            let _e705 = phi_20372_;
                            phi_20374_ = _e699;
                            phi_20375_ = _e701;
                            phi_20377_ = _e703;
                            phi_20378_ = _e705;
                            phi_20379_ = select(true, false, _e399);
                            phi_20380_ = _e399;
                        } else {
                            phi_20374_ = u32();
                            phi_20375_ = u32();
                            phi_20377_ = _e358;
                            phi_20378_ = _e360;
                            phi_20379_ = _e395;
                            phi_20380_ = false;
                        }
                        let _e708 = phi_20374_;
                        let _e710 = phi_20375_;
                        let _e712 = phi_20377_;
                        let _e714 = phi_20378_;
                        let _e716 = phi_20379_;
                        let _e718 = phi_20380_;
                        phi_19882_ = _e352;
                        phi_19885_ = select(_e708, 4294967295u, _e716);
                        phi_19887_ = select(_e710, select(_e387, _e391, _e393), _e716);
                        phi_19889_ = select(_e358, _e712, _e716);
                        phi_19891_ = select(_e360, _e714, _e716);
                        phi_19893_ = _e362;
                        phi_19895_ = _e364;
                        phi_20391_ = vec4<f32>();
                        phi_20392_ = select(_e718, true, _e716);
                        phi_20393_ = select(select(_e397, false, _e393), false, _e716);
                    }
                    let _e948 = phi_19882_;
                    let _e950 = phi_19885_;
                    let _e952 = phi_19887_;
                    let _e954 = phi_19889_;
                    let _e956 = phi_19891_;
                    let _e958 = phi_19893_;
                    let _e960 = phi_19895_;
                    let _e962 = phi_20391_;
                    let _e964 = phi_20392_;
                    let _e966 = phi_20393_;
                    phi_19881_ = _e948;
                    phi_19884_ = _e950;
                    phi_19886_ = _e952;
                    phi_19888_ = _e954;
                    phi_19890_ = _e956;
                    phi_19892_ = _e958;
                    phi_19894_ = _e960;
                    phi_37261_ = false;
                    phi_20573_ = _e966;
                    phi_20572_ = _e962;
                    if !(_e964) {
                        break;
                    }
                }
                let _e969 = phi_37261_;
                let _e971 = phi_20573_;
                let _e973 = phi_20572_;
                if _e969 {
                    break;
                }
                if _e971 {
                    let _e975 = local_105;
                    let _e977 = local_106;
                    phi_20398_ = type_76(_e975, _e977);
                } else {
                    phi_20398_ = type_76();
                }
                let _e980 = phi_20398_;
                if select(true, false, _e971) {
                    let _e983 = local_107;
                    phi_20404_ = type_76(_e983, _e973);
                } else {
                    phi_20404_ = _e980;
                }
                let _e986 = phi_20404_;
                let _e989 = global_7.member[_e318.member_1];
                let _e1001 = type_35(_e989.member, _e989.member_1, _e989.member_2, _e989.member_3, vec4<u32>(_e989.member_4.x, _e989.member_4.y, _e986.member, _e989.member_4.w), _e989.member_5);
                if _e323 {
                } else {
                    break;
                }
                global_7.member[_e318.member_1] = type_35(_e1001.member, _e1001.member_1, _e1001.member_2, _e1001.member_3, _e1001.member_4, _e986.member_1);
            }
            break;
        }
    }
    return;
}

fn function_3() {
    var phi_20438_: type_49;
    var phi_20485_: bool;
    var phi_20486_: bool;
    var phi_20487_: bool;
    var phi_20503_: vec4<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e276 = global;
            let _e284 = (_e276.x + (_e276.y * 4194240u));
            let _e288 = global_1.member.member[0u];
            let _e292 = global_1.member.member[1u];
            if (_e284 >= (_e288 * _e292)) {
                phi_20438_ = type_49(0u, type_49().member_1);
            } else {
                phi_20438_ = type_49(1u, _e284);
            }
            let _e300 = phi_20438_;
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
                            phi_20485_ = select(true, false, (_e327 == _e327));
                        } else {
                            phi_20485_ = false;
                        }
                        let _e334 = phi_20485_;
                        phi_20486_ = select(true, false, _e330);
                        phi_20487_ = _e334;
                    } else {
                        phi_20486_ = true;
                        phi_20487_ = false;
                    }
                    let _e337 = phi_20486_;
                    let _e339 = phi_20487_;
                    let _e342 = select(vec3<f32>(_e325, _e326, _e327), vec3<f32>(0f, 0f, 0f), vec3(select(_e339, true, _e337)));
                    let _e347 = global_10.member[_e300.member_1].member_3[3u];
                    phi_20503_ = vec4<f32>(_e342.x, _e342.y, _e342.z, _e347);
                } else {
                    phi_20503_ = vec4<f32>(0f, 0f, 0f, 0f);
                }
                let _e353 = phi_20503_;
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
