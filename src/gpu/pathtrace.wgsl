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

struct type_49 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: vec3<f32>,
}

struct type_51 {
    member: u32,
    member_1: type_49,
}

struct type_52 {
    member: f32,
    member_1: f32,
}

struct type_53 {
    member: u32,
    member_1: type_52,
}

struct type_54 {
    member: u32,
    member_1: vec3<f32>,
}

struct type_55 {
    member: vec3<f32>,
    member_1: f32,
}

struct type_56 {
    member: vec3<f32>,
    member_1: vec3<f32>,
}

struct type_61 {
    member: array<vec4<f32>>,
}

struct type_63 {
    member: f32,
    member_1: u32,
    member_2: vec3<f32>,
    member_3: vec3<f32>,
}

struct type_68 {
    member: type_35,
    member_1: type_39,
    member_2: type_39,
}

struct type_71 {
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
var<storage, read_write> global_9: type_61;

fn function_() {
    var phi_444_: type_45;
    var phi_31499_: bool;
    var phi_464_: vec3<f32>;
    var phi_467_: u32;
    var phi_20406_: f32;
    var phi_20418_: type_52;
    var phi_20429_: type_53;
    var phi_20430_: type_53;
    var phi_20434_: type_53;
    var phi_20435_: type_53;
    var phi_20436_: bool;
    var phi_20440_: type_53;
    var phi_20327_: type_53;
    var phi_20478_: bool;
    var phi_20558_: f32;
    var phi_20570_: type_52;
    var phi_20581_: type_53;
    var phi_20582_: type_53;
    var phi_20586_: type_53;
    var phi_20587_: type_53;
    var phi_20588_: bool;
    var phi_20592_: type_53;
    var phi_20526_: f32;
    var phi_20630_: bool;
    var phi_20355_: f32;
    var phi_20357_: f32;
    var phi_20366_: type_53;
    var phi_20370_: type_53;
    var phi_20374_: type_53;
    var phi_20645_: bool;
    var phi_9467_: vec3<f32>;
    var phi_9470_: vec3<f32>;
    var phi_9472_: vec3<f32>;
    var phi_9474_: vec3<f32>;
    var phi_9476_: u32;
    var phi_9560_: u32;
    var phi_9575_: type_52;
    var phi_9576_: type_52;
    var phi_9577_: type_52;
    var phi_20845_: bool;
    var phi_20972_: bool;
    var phi_20987_: bool;
    var phi_9905_: vec3<f32>;
    var phi_9978_: vec3<f32>;
    var phi_9979_: vec3<f32>;
    var phi_9980_: bool;
    var phi_9985_: vec3<f32>;
    var phi_9986_: bool;
    var phi_21111_: bool;
    var phi_21126_: bool;
    var phi_10277_: vec3<f32>;
    var phi_9468_: vec3<f32>;
    var phi_9471_: vec3<f32>;
    var phi_9473_: vec3<f32>;
    var phi_9475_: vec3<f32>;
    var phi_9477_: u32;
    var phi_31482_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_31966_: bool;
    var phi_10334_: type_56;
    var phi_31965_: bool;
    var phi_10336_: type_56;
    var phi_10337_: bool;
    var phi_10342_: type_56;
    var phi_843_: vec3<f32>;
    var phi_845_: vec3<f32>;
    var phi_846_: bool;
    var phi_31574_: bool;
    var phi_856_: vec3<f32>;
    var phi_859_: i32;
    var phi_861_: u32;
    var phi_21395_: bool;
    var phi_1283_: u32;
    var phi_1313_: u32;
    var phi_1284_: u32;
    var phi_1314_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1332_: u32;
    var phi_2637_: type_51;
    var phi_2645_: type_49;
    var phi_2647_: type_49;
    var phi_2648_: bool;
    var phi_2649_: type_49;
    var phi_2650_: bool;
    var phi_2651_: bool;
    var phi_21506_: f32;
    var phi_21518_: type_52;
    var phi_21529_: type_53;
    var phi_21530_: type_53;
    var phi_21534_: type_53;
    var phi_21535_: type_53;
    var phi_21536_: bool;
    var phi_21540_: type_53;
    var phi_21427_: type_53;
    var phi_21578_: bool;
    var phi_21658_: f32;
    var phi_21670_: type_52;
    var phi_21681_: type_53;
    var phi_21682_: type_53;
    var phi_21686_: type_53;
    var phi_21687_: type_53;
    var phi_21688_: bool;
    var phi_21692_: type_53;
    var phi_21626_: f32;
    var phi_21730_: bool;
    var phi_21455_: f32;
    var phi_21457_: f32;
    var phi_21466_: type_53;
    var phi_21470_: type_53;
    var phi_21474_: type_53;
    var phi_2669_: vec3<f32>;
    var phi_2672_: vec3<f32>;
    var phi_2674_: vec3<f32>;
    var phi_2676_: vec3<f32>;
    var phi_2678_: u32;
    var phi_2762_: u32;
    var phi_2777_: type_52;
    var phi_2778_: type_52;
    var phi_2779_: type_52;
    var phi_21929_: bool;
    var phi_22056_: bool;
    var phi_22071_: bool;
    var phi_3107_: vec3<f32>;
    var phi_22193_: f32;
    var phi_22205_: type_52;
    var phi_22216_: type_53;
    var phi_22217_: type_53;
    var phi_22221_: type_53;
    var phi_22222_: type_53;
    var phi_22223_: bool;
    var phi_22227_: type_53;
    var phi_22161_: f32;
    var phi_22303_: f32;
    var phi_22315_: type_52;
    var phi_22326_: type_53;
    var phi_22327_: type_53;
    var phi_22331_: type_53;
    var phi_22332_: type_53;
    var phi_22333_: bool;
    var phi_22337_: type_53;
    var phi_22271_: f32;
    var phi_22098_: vec3<f32>;
    var phi_22099_: u32;
    var phi_22469_: bool;
    var phi_22132_: vec3<f32>;
    var phi_22133_: u32;
    var local_11: vec3<f32>;
    var phi_22140_: type_54;
    var phi_22144_: type_54;
    var phi_3166_: vec3<f32>;
    var phi_3167_: vec3<f32>;
    var phi_2670_: vec3<f32>;
    var phi_2673_: vec3<f32>;
    var phi_2675_: vec3<f32>;
    var phi_2677_: vec3<f32>;
    var phi_2679_: u32;
    var phi_31542_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_31636_: bool;
    var phi_3187_: type_49;
    var phi_31635_: bool;
    var phi_3188_: type_49;
    var phi_31634_: bool;
    var phi_3211_: vec3<f32>;
    var phi_3212_: i32;
    var phi_3213_: u32;
    var phi_31633_: bool;
    var phi_857_: vec3<f32>;
    var phi_860_: i32;
    var phi_862_: u32;
    var phi_3214_: bool;
    var phi_31632_: bool;
    var phi_3548_: vec3<f32>;
    var phi_3551_: u32;
    var phi_22597_: f32;
    var phi_22612_: f32;
    var phi_22627_: f32;
    var phi_3622_: i32;
    var phi_3652_: i32;
    var phi_3660_: vec3<f32>;
    var phi_3663_: i32;
    var phi_3686_: vec3<f32>;
    var phi_3689_: i32;
    var phi_3687_: vec3<f32>;
    var phi_3690_: i32;
    var phi_3661_: vec3<f32>;
    var phi_3664_: i32;
    var phi_3888_: vec3<f32>;
    var phi_3889_: vec3<f32>;
    var phi_3890_: vec3<f32>;
    var phi_3549_: vec3<f32>;
    var phi_3552_: u32;
    var phi_31637_: bool;
    var local_15: vec3<f32>;
    var phi_31685_: bool;
    var phi_3901_: f32;
    var phi_3925_: f32;
    var phi_3927_: u32;
    var phi_3930_: f32;
    var phi_4033_: f32;
    var phi_4034_: f32;
    var phi_4050_: f32;
    var phi_3928_: u32;
    var phi_3931_: f32;
    var phi_4053_: u32;
    var phi_4056_: f32;
    var phi_4164_: f32;
    var phi_4054_: u32;
    var phi_4057_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_22743_: f32;
    var phi_22755_: type_52;
    var phi_22766_: type_53;
    var phi_22767_: type_53;
    var phi_22771_: type_53;
    var phi_22772_: type_53;
    var phi_22773_: bool;
    var phi_22777_: type_53;
    var phi_22711_: f32;
    var phi_4565_: bool;
    var phi_22903_: f32;
    var phi_22915_: type_52;
    var phi_22926_: type_53;
    var phi_22927_: type_53;
    var phi_22931_: type_53;
    var phi_22932_: type_53;
    var phi_22933_: bool;
    var phi_22937_: type_53;
    var phi_22824_: type_53;
    var phi_22975_: bool;
    var phi_23055_: f32;
    var phi_23067_: type_52;
    var phi_23078_: type_53;
    var phi_23079_: type_53;
    var phi_23083_: type_53;
    var phi_23084_: type_53;
    var phi_23085_: bool;
    var phi_23089_: type_53;
    var phi_23023_: f32;
    var phi_23127_: bool;
    var phi_22852_: f32;
    var phi_22854_: f32;
    var phi_22863_: type_53;
    var phi_22867_: type_53;
    var phi_22871_: type_53;
    var phi_4579_: vec3<f32>;
    var phi_4582_: u32;
    var phi_4666_: u32;
    var phi_4681_: type_52;
    var phi_4682_: type_52;
    var phi_4683_: type_52;
    var phi_23326_: bool;
    var phi_4580_: vec3<f32>;
    var phi_4583_: u32;
    var phi_31664_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_31731_: bool;
    var phi_4708_: vec3<f32>;
    var phi_31730_: bool;
    var phi_4710_: vec3<f32>;
    var phi_4712_: bool;
    var phi_31726_: bool;
    var phi_4723_: vec3<f32>;
    var phi_4724_: bool;
    var phi_31725_: bool;
    var phi_4726_: vec3<f32>;
    var phi_4727_: bool;
    var phi_4728_: bool;
    var phi_31724_: bool;
    var phi_4732_: vec3<f32>;
    var phi_4733_: vec3<f32>;
    var phi_4734_: bool;
    var phi_4735_: vec3<f32>;
    var phi_4736_: bool;
    var phi_5056_: vec3<f32>;
    var phi_5059_: u32;
    var phi_23397_: f32;
    var phi_23412_: f32;
    var phi_23427_: f32;
    var phi_5130_: i32;
    var phi_5160_: i32;
    var phi_5168_: vec3<f32>;
    var phi_5171_: i32;
    var phi_5194_: vec3<f32>;
    var phi_5197_: i32;
    var phi_5268_: vec3<f32>;
    var phi_5271_: u32;
    var phi_5391_: vec3<f32>;
    var phi_5269_: vec3<f32>;
    var phi_5272_: u32;
    var phi_5195_: vec3<f32>;
    var phi_5198_: i32;
    var phi_5169_: vec3<f32>;
    var phi_5172_: i32;
    var phi_5396_: vec3<f32>;
    var phi_5397_: vec3<f32>;
    var phi_5398_: vec3<f32>;
    var phi_5057_: vec3<f32>;
    var phi_5060_: u32;
    var phi_31719_: bool;
    var phi_31761_: bool;
    var phi_5418_: vec3<f32>;
    var phi_5631_: u32;
    var phi_5632_: bool;
    var phi_5635_: u32;
    var phi_5636_: bool;
    var phi_5637_: bool;
    var phi_5647_: f32;
    var phi_5650_: f32;
    var phi_5652_: u32;
    var phi_5654_: vec3<f32>;
    var phi_5656_: vec3<f32>;
    var phi_5658_: f32;
    var phi_5660_: f32;
    var phi_5698_: u32;
    var phi_5713_: vec3<f32>;
    var phi_5733_: vec3<f32>;
    var phi_23442_: vec3<f32>;
    var phi_5734_: u32;
    var phi_5735_: vec3<f32>;
    var phi_5736_: u32;
    var phi_5737_: vec3<f32>;
    var phi_5738_: u32;
    var phi_5740_: vec3<f32>;
    var phi_5875_: u32;
    var phi_5878_: f32;
    var phi_5880_: f32;
    var phi_5882_: f32;
    var phi_5884_: vec3<f32>;
    var phi_23497_: f32;
    var phi_23503_: f32;
    var phi_23531_: f32;
    var phi_23537_: f32;
    var phi_23565_: f32;
    var phi_23571_: f32;
    var phi_23599_: f32;
    var phi_23605_: f32;
    var phi_23633_: f32;
    var phi_23639_: f32;
    var phi_23667_: f32;
    var phi_23673_: f32;
    var phi_23701_: f32;
    var phi_23707_: f32;
    var phi_23735_: f32;
    var phi_23741_: f32;
    var phi_5876_: u32;
    var phi_5879_: f32;
    var phi_5881_: f32;
    var phi_5883_: f32;
    var phi_5885_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6301_: f32;
    var phi_6353_: vec3<f32>;
    var phi_6354_: f32;
    var phi_6355_: vec3<f32>;
    var phi_6356_: f32;
    var phi_6357_: vec3<f32>;
    var phi_6358_: f32;
    var phi_6360_: vec3<f32>;
    var phi_6361_: f32;
    var phi_6362_: f32;
    var phi_6363_: vec3<f32>;
    var phi_6364_: vec3<f32>;
    var phi_6365_: f32;
    var phi_6366_: f32;
    var phi_6367_: vec3<f32>;
    var phi_6368_: vec3<f32>;
    var phi_6369_: f32;
    var phi_6370_: f32;
    var phi_6371_: vec3<f32>;
    var phi_6372_: vec3<f32>;
    var phi_6373_: f32;
    var phi_6374_: f32;
    var phi_6375_: vec3<f32>;
    var phi_6376_: vec3<f32>;
    var phi_6378_: f32;
    var phi_6379_: f32;
    var phi_6380_: vec3<f32>;
    var phi_6381_: vec3<f32>;
    var phi_6382_: f32;
    var phi_6383_: f32;
    var phi_6384_: f32;
    var phi_6385_: f32;
    var phi_6386_: vec3<f32>;
    var phi_6387_: vec3<f32>;
    var phi_6388_: f32;
    var phi_6389_: f32;
    var phi_5648_: f32;
    var phi_5651_: f32;
    var phi_5653_: u32;
    var phi_5655_: vec3<f32>;
    var phi_5657_: vec3<f32>;
    var phi_5659_: f32;
    var phi_5661_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_23817_: bool;
    var local_29: f32;
    var phi_6421_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6500_: f32;
    var phi_6501_: vec3<f32>;
    var phi_6502_: f32;
    var phi_6504_: vec3<f32>;
    var phi_6507_: u32;
    var phi_6955_: f32;
    var phi_7135_: u32;
    var phi_7138_: f32;
    var phi_7140_: f32;
    var phi_7142_: f32;
    var phi_7144_: vec3<f32>;
    var phi_23906_: f32;
    var phi_23912_: f32;
    var phi_23940_: f32;
    var phi_23946_: f32;
    var phi_23974_: f32;
    var phi_23980_: f32;
    var phi_24008_: f32;
    var phi_24014_: f32;
    var phi_24042_: f32;
    var phi_24048_: f32;
    var phi_24076_: f32;
    var phi_24082_: f32;
    var phi_24110_: f32;
    var phi_24116_: f32;
    var phi_24144_: f32;
    var phi_24150_: f32;
    var phi_7136_: u32;
    var phi_7139_: f32;
    var phi_7141_: f32;
    var phi_7143_: f32;
    var phi_7145_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7558_: f32;
    var phi_24196_: bool;
    var phi_24211_: bool;
    var phi_7651_: vec3<f32>;
    var phi_7652_: vec3<f32>;
    var phi_7653_: vec3<f32>;
    var phi_7654_: vec3<f32>;
    var phi_6505_: vec3<f32>;
    var phi_6508_: u32;
    var phi_7673_: vec3<f32>;
    var phi_7676_: u32;
    var phi_7678_: f32;
    var phi_7979_: vec3<f32>;
    var phi_7980_: u32;
    var phi_7981_: f32;
    var phi_7982_: vec3<f32>;
    var phi_7999_: vec3<f32>;
    var phi_8001_: u32;
    var phi_8004_: vec3<f32>;
    var phi_8006_: f32;
    var phi_8008_: f32;
    var phi_8010_: f32;
    var phi_8030_: bool;
    var phi_8046_: bool;
    var phi_8048_: bool;
    var phi_8050_: bool;
    var phi_8051_: bool;
    var phi_8052_: bool;
    var phi_8053_: bool;
    var phi_8054_: bool;
    var phi_8098_: vec3<f32>;
    var phi_24230_: vec3<f32>;
    var phi_8099_: u32;
    var phi_8100_: vec3<f32>;
    var phi_8101_: u32;
    var phi_8102_: vec3<f32>;
    var phi_8237_: u32;
    var phi_8240_: f32;
    var phi_8242_: f32;
    var phi_8244_: f32;
    var phi_8246_: vec3<f32>;
    var phi_24285_: f32;
    var phi_24291_: f32;
    var phi_24319_: f32;
    var phi_24325_: f32;
    var phi_24353_: f32;
    var phi_24359_: f32;
    var phi_24387_: f32;
    var phi_24393_: f32;
    var phi_24421_: f32;
    var phi_24427_: f32;
    var phi_24455_: f32;
    var phi_24461_: f32;
    var phi_24489_: f32;
    var phi_24495_: f32;
    var phi_24523_: f32;
    var phi_24529_: f32;
    var phi_8238_: u32;
    var phi_8241_: f32;
    var phi_8243_: f32;
    var phi_8245_: f32;
    var phi_8247_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8660_: f32;
    var phi_8686_: vec3<f32>;
    var phi_8687_: f32;
    var phi_8688_: f32;
    var phi_8689_: vec3<f32>;
    var phi_8690_: f32;
    var phi_8691_: f32;
    var phi_8692_: vec3<f32>;
    var phi_8693_: f32;
    var phi_8694_: f32;
    var phi_8695_: vec3<f32>;
    var phi_8696_: f32;
    var phi_8697_: f32;
    var phi_8699_: vec3<f32>;
    var phi_8700_: f32;
    var phi_8701_: f32;
    var phi_8702_: f32;
    var phi_8002_: u32;
    var phi_8005_: vec3<f32>;
    var phi_8007_: f32;
    var phi_8009_: f32;
    var phi_8011_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_8992_: type_55;
    var local_42: f32;
    var phi_8993_: type_55;
    var local_43: f32;
    var local_44: f32;
    var phi_8994_: type_55;
    var phi_8995_: type_55;
    var phi_7674_: vec3<f32>;
    var phi_7677_: u32;
    var phi_7679_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9024_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_31969_: bool;
    var phi_9071_: vec3<f32>;
    var phi_31968_: bool;
    var phi_9072_: vec3<f32>;
    var phi_31967_: bool;
    var phi_9073_: vec3<f32>;
    var phi_31963_: bool;
    var phi_10346_: vec3<f32>;
    var phi_31962_: bool;
    var phi_10347_: vec3<f32>;
    var phi_31961_: bool;
    var phi_465_: vec3<f32>;
    var phi_468_: u32;
    var phi_31960_: bool;
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
                phi_444_ = type_45(0u, type_45().member_1);
            } else {
                phi_444_ = type_45(1u, _e280);
            }
            let _e296 = phi_444_;
            if (bitcast<i32>(_e296.member) != 0i) {
                phi_31499_ = false;
                phi_464_ = vec3<f32>(0f, 0f, 0f);
                phi_467_ = 0u;
                loop {
                    let _e302 = phi_31499_;
                    let _e304 = phi_464_;
                    let _e306 = phi_467_;
                    local_55 = _e304;
                    local_56 = _e304;
                    local_57 = _e304;
                    let _e307 = (_e306 < 2u);
                    if _e307 {
                        let _e309 = ((2u * _e296.member_1) + _e306);
                        if (_e309 < arrayLength((&global_8.member))) {
                        } else {
                            phi_31960_ = true;
                            break;
                        }
                        let _e313 = global_8.member[_e309];
                        if ((((_e313.member_2.x * _e313.member_2.x) + (_e313.member_2.y * _e313.member_2.y)) + (_e313.member_2.z * _e313.member_2.z)) == 0f) {
                            phi_31962_ = _e302;
                            phi_10347_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e313.member.w < 0f) {
                                let _e1268 = global_1.member.member_8[0u];
                                let _e1271 = global_1.member.member_8[3u];
                                if (_e1268 == 4294967295u) {
                                    let _e1273 = (_e1271 == 4294967295u);
                                    if _e1273 {
                                        let _e1276 = global_1.member.member_3[1u];
                                        let _e1279 = global_1.member.member_3[2u];
                                        let _e1282 = ((atan2(_e313.member_1.z, _e313.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1287 = (((_e1282 - floor(_e1282)) * f32(_e1276)) - 0.5f);
                                        let _e1289 = select(_e313.member_1.y, -1f, (_e313.member_1.y < -1f));
                                        let _e1296 = (((acos(select(_e1289, 1f, (_e1289 > 1f))) * 0.31830987f) * f32(_e1279)) - 0.5f);
                                        let _e1297 = (_e1279 - 1u);
                                        let _e1298 = f32(_e1297);
                                        if (0f <= _e1298) {
                                        } else {
                                            phi_31960_ = true;
                                            break;
                                        }
                                        let _e1301 = select(_e1296, 0f, (_e1296 < 0f));
                                        let _e1303 = select(_e1301, _e1298, (_e1301 > _e1298));
                                        let _e1304 = floor(_e1287);
                                        let _e1305 = floor(_e1303);
                                        let _e1306 = (_e1287 - _e1304);
                                        let _e1307 = (_e1303 - _e1305);
                                        let _e1318 = (_e1276 == 0u);
                                        if _e1318 {
                                            phi_31960_ = true;
                                            break;
                                        }
                                        let _e1319 = (bitcast<u32>((select(0i, select(select(i32(_e1304), i32(-2147483648), (_e1304 < -2147483600f)), 2147483647i, (_e1304 > 2147483500f)), (_e1304 == _e1304)) + bitcast<i32>(_e1276))) % _e1276);
                                        let _e1324 = select(select(u32(_e1305), 0u, (_e1305 < 0f)), 4294967295u, (_e1305 > 4294967000f));
                                        let _e1325 = (_e1324 + 1u);
                                        let _e1330 = global_1.member.member_2[2u];
                                        let _e1331 = (_e1324 * _e1276);
                                        if _e1318 {
                                            phi_31960_ = true;
                                            break;
                                        }
                                        let _e1332 = (_e1319 % _e1276);
                                        let _e1335 = (_e1330 + (3u * (_e1331 + _e1332)));
                                        let _e1337 = global_6.member[_e1335];
                                        let _e1340 = global_6.member[(_e1335 + 1u)];
                                        let _e1343 = global_6.member[(_e1335 + 2u)];
                                        if _e1318 {
                                            phi_31960_ = true;
                                            break;
                                        }
                                        let _e1346 = ((_e1319 + 1u) % _e1276);
                                        let _e1349 = (_e1330 + (3u * (_e1331 + _e1346)));
                                        let _e1351 = global_6.member[_e1349];
                                        let _e1354 = global_6.member[(_e1349 + 1u)];
                                        let _e1357 = global_6.member[(_e1349 + 2u)];
                                        let _e1365 = (vec3<f32>(_e1337, _e1340, _e1343) + vec3<f32>(((_e1351 - _e1337) * _e1306), ((_e1354 - _e1340) * _e1306), ((_e1357 - _e1343) * _e1306)));
                                        let _e1366 = (select(_e1325, _e1297, (_e1297 < _e1325)) * _e1276);
                                        if _e1318 {
                                            phi_31960_ = true;
                                            break;
                                        }
                                        let _e1369 = (_e1330 + (3u * (_e1366 + _e1332)));
                                        let _e1371 = global_6.member[_e1369];
                                        let _e1374 = global_6.member[(_e1369 + 1u)];
                                        let _e1377 = global_6.member[(_e1369 + 2u)];
                                        if _e1318 {
                                            phi_31960_ = true;
                                            break;
                                        }
                                        let _e1381 = (_e1330 + (3u * (_e1366 + _e1346)));
                                        let _e1383 = global_6.member[_e1381];
                                        let _e1386 = global_6.member[(_e1381 + 1u)];
                                        let _e1389 = global_6.member[(_e1381 + 2u)];
                                        let _e1397 = (vec3<f32>(_e1371, _e1374, _e1377) + vec3<f32>(((_e1383 - _e1371) * _e1306), ((_e1386 - _e1374) * _e1306), ((_e1389 - _e1377) * _e1306)));
                                        phi_843_ = (_e1365 + vec3<f32>(((_e1397.x - _e1365.x) * _e1307), ((_e1397.y - _e1365.y) * _e1307), ((_e1397.z - _e1365.z) * _e1307)));
                                    } else {
                                        phi_843_ = vec3<f32>();
                                    }
                                    let _e1413 = phi_843_;
                                    phi_845_ = _e1413;
                                    phi_846_ = select(true, false, _e1273);
                                } else {
                                    phi_845_ = vec3<f32>();
                                    phi_846_ = true;
                                }
                                let _e1416 = phi_845_;
                                let _e1418 = phi_846_;
                                if _e1418 {
                                    let _e1419 = (_e1271 != 4294967295u);
                                    if (_e1268 != 4294967295u) {
                                        phi_31574_ = _e302;
                                        phi_856_ = vec3<f32>(0f, 0f, 0f);
                                        phi_859_ = 0i;
                                        phi_861_ = _e1268;
                                        loop {
                                            let _e1422 = phi_31574_;
                                            let _e1424 = phi_856_;
                                            let _e1426 = phi_859_;
                                            let _e1428 = phi_861_;
                                            local_17 = _e1424;
                                            local_18 = _e1424;
                                            local_19 = _e1424;
                                            local_61 = _e1424;
                                            local_62 = _e1424;
                                            if (_e1428 != 4294967295u) {
                                                let _e1430 = (_e1426 < 2i);
                                                if _e1430 {
                                                    let _e1432 = global_6.member[_e1428];
                                                    let _e1435 = global_6.member[(_e1428 + 1u)];
                                                    let _e1438 = global_6.member[(_e1428 + 2u)];
                                                    let _e1441 = global_6.member[(_e1428 + 3u)];
                                                    let _e1444 = global_6.member[(_e1428 + 4u)];
                                                    let _e1447 = global_6.member[(_e1428 + 5u)];
                                                    let _e1450 = global_6.member[(_e1428 + 6u)];
                                                    let _e1453 = global_6.member[(_e1428 + 7u)];
                                                    let _e1456 = global_6.member[(_e1428 + 8u)];
                                                    let _e1459 = global_6.member[(_e1428 + 9u)];
                                                    let _e1462 = global_6.member[(_e1428 + 10u)];
                                                    let _e1465 = global_6.member[(_e1428 + 11u)];
                                                    let _e1468 = global_6.member[(_e1428 + 12u)];
                                                    let _e1471 = global_6.member[(_e1428 + 13u)];
                                                    let _e1474 = global_6.member[(_e1428 + 14u)];
                                                    let _e1477 = global_6.member[(_e1428 + 15u)];
                                                    let _e1481 = global_6.member[(_e1428 + 16u)];
                                                    let _e1484 = global_6.member[(_e1428 + 17u)];
                                                    let _e1487 = global_6.member[(_e1428 + 18u)];
                                                    let _e1490 = global_6.member[(_e1428 + 19u)];
                                                    let _e1493 = global_6.member[(_e1428 + 20u)];
                                                    let _e1496 = global_6.member[(_e1428 + 30u)];
                                                    let _e1497 = bitcast<u32>(_e1496);
                                                    let _e1500 = global_6.member[(_e1428 + 31u)];
                                                    let _e1501 = bitcast<u32>(_e1500);
                                                    let _e1504 = global_6.member[(_e1428 + 32u)];
                                                    let _e1506 = (_e313.member_1.y < 0f);
                                                    if _e1506 {
                                                        phi_2649_ = type_49();
                                                        phi_2650_ = true;
                                                        phi_2651_ = false;
                                                    } else {
                                                        let _e1507 = (_e1501 == 4294967295u);
                                                        if _e1507 {
                                                            phi_2647_ = type_49();
                                                            phi_2648_ = false;
                                                        } else {
                                                            let _e1521 = (_e313.member.x - _e1432);
                                                            let _e1522 = (_e313.member.y - _e1435);
                                                            let _e1523 = (_e313.member.z - _e1438);
                                                            let _e1531 = ((((((_e313.member.x * _e313.member.x) + (_e313.member.y * _e313.member.y)) + (_e313.member.z * _e313.member.z)) - (2f * (((_e313.member.x * _e1432) + (_e313.member.y * _e1435)) + (_e313.member.z * _e1438)))) + _e1447) / (sqrt((((_e1521 * _e1521) + (_e1522 * _e1522)) + (_e1523 * _e1523))) + _e1441));
                                                            let _e1532 = (_e1428 + 21u);
                                                            let _e1535 = global_6.member[(_e1428 + 28u)];
                                                            if (_e1531 > _e1535) {
                                                                phi_2637_ = type_51(0u, type_51().member_1);
                                                            } else {
                                                                let _e1538 = global_6.member[_e1532];
                                                                if (_e1531 != _e1531) {
                                                                    phi_21395_ = true;
                                                                } else {
                                                                    phi_21395_ = (_e1538 >= _e1531);
                                                                }
                                                                let _e1542 = phi_21395_;
                                                                let _e1543 = select(_e1531, _e1538, _e1542);
                                                                phi_1283_ = 0u;
                                                                loop {
                                                                    let _e1545 = phi_1283_;
                                                                    local_8 = _e1545;
                                                                    local_9 = _e1545;
                                                                    local_10 = _e1545;
                                                                    if (_e1545 < 8u) {
                                                                        let _e1549 = global_6.member[(_e1532 + _e1545)];
                                                                        let _e1550 = (_e1549 <= _e1543);
                                                                        if _e1550 {
                                                                            phi_1313_ = (_e1545 + 1u);
                                                                        } else {
                                                                            phi_1313_ = u32();
                                                                        }
                                                                        let _e1553 = phi_1313_;
                                                                        phi_1284_ = _e1553;
                                                                        phi_1314_ = _e1550;
                                                                    } else {
                                                                        phi_1284_ = u32();
                                                                        phi_1314_ = false;
                                                                    }
                                                                    let _e1555 = phi_1284_;
                                                                    let _e1557 = phi_1314_;
                                                                    phi_1283_ = _e1555;
                                                                    if !(_e1557) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1560 = local_8;
                                                                if (_e1560 < 1u) {
                                                                    phi_1332_ = 1u;
                                                                } else {
                                                                    let _e1563 = local_9;
                                                                    let _e1566 = local_10;
                                                                    phi_1332_ = select(_e1566, 7u, (_e1563 > 7u));
                                                                }
                                                                let _e1569 = phi_1332_;
                                                                let _e1570 = (_e1569 - 1u);
                                                                let _e1573 = global_6.member[(_e1532 + _e1570)];
                                                                let _e1576 = global_6.member[(_e1532 + _e1569)];
                                                                let _e1579 = ((_e1543 - _e1573) / (_e1576 - _e1573));
                                                                let _e1580 = select(_e313.member_1.y, 0f, _e1506);
                                                                let _e1586 = (sqrt((asin(select(_e1580, 1f, (_e1580 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1588 = (atan2(_e313.member_1.z, _e313.member_1.x) * 0.15915494f);
                                                                let _e1591 = ((_e1588 - floor(_e1588)) * 256f);
                                                                let _e1592 = floor(_e1591);
                                                                let _e1598 = (select(select(u32(_e1592), 0u, (_e1592 < 0f)), 4294967295u, (_e1592 > 4294967000f)) % 256u);
                                                                let _e1599 = floor(_e1586);
                                                                let _e1604 = select(select(u32(_e1599), 0u, (_e1599 < 0f)), 4294967295u, (_e1599 > 4294967000f));
                                                                let _e1606 = select(_e1604, 126u, (126u < _e1604));
                                                                let _e1608 = ((_e1598 + 1u) % 256u);
                                                                let _e1610 = (_e1591 - _e1592);
                                                                let _e1612 = (_e1586 - f32(_e1606));
                                                                let _e1614 = (_e1501 + (_e1570 * 294912u));
                                                                let _e1615 = (_e1606 * 256u);
                                                                let _e1617 = (9u * (_e1615 + _e1598));
                                                                let _e1618 = (_e1614 + _e1617);
                                                                let _e1620 = (9u * (_e1615 + _e1608));
                                                                let _e1621 = (_e1614 + _e1620);
                                                                let _e1622 = ((_e1606 + 1u) * 256u);
                                                                let _e1624 = (9u * (_e1622 + _e1598));
                                                                let _e1625 = (_e1614 + _e1624);
                                                                let _e1627 = (9u * (_e1622 + _e1608));
                                                                let _e1628 = (_e1614 + _e1627);
                                                                let _e1629 = (1f - _e1610);
                                                                let _e1630 = (1f - _e1612);
                                                                let _e1631 = (_e1629 * _e1630);
                                                                let _e1632 = (_e1610 * _e1630);
                                                                let _e1633 = (_e1629 * _e1612);
                                                                let _e1634 = (_e1610 * _e1612);
                                                                let _e1636 = global_6.member[_e1618];
                                                                let _e1639 = global_6.member[(_e1618 + 1u)];
                                                                let _e1642 = global_6.member[(_e1618 + 2u)];
                                                                let _e1647 = global_6.member[_e1621];
                                                                let _e1650 = global_6.member[(_e1621 + 1u)];
                                                                let _e1653 = global_6.member[(_e1621 + 2u)];
                                                                let _e1661 = global_6.member[_e1625];
                                                                let _e1664 = global_6.member[(_e1625 + 1u)];
                                                                let _e1667 = global_6.member[(_e1625 + 2u)];
                                                                let _e1675 = global_6.member[_e1628];
                                                                let _e1678 = global_6.member[(_e1628 + 1u)];
                                                                let _e1681 = global_6.member[(_e1628 + 2u)];
                                                                let _e1690 = global_6.member[(_e1618 + 3u)];
                                                                let _e1693 = global_6.member[(_e1618 + 4u)];
                                                                let _e1696 = global_6.member[(_e1618 + 5u)];
                                                                let _e1702 = global_6.member[(_e1621 + 3u)];
                                                                let _e1705 = global_6.member[(_e1621 + 4u)];
                                                                let _e1708 = global_6.member[(_e1621 + 5u)];
                                                                let _e1717 = global_6.member[(_e1625 + 3u)];
                                                                let _e1720 = global_6.member[(_e1625 + 4u)];
                                                                let _e1723 = global_6.member[(_e1625 + 5u)];
                                                                let _e1732 = global_6.member[(_e1628 + 3u)];
                                                                let _e1735 = global_6.member[(_e1628 + 4u)];
                                                                let _e1738 = global_6.member[(_e1628 + 5u)];
                                                                let _e1747 = global_6.member[(_e1618 + 6u)];
                                                                let _e1750 = global_6.member[(_e1618 + 7u)];
                                                                let _e1753 = global_6.member[(_e1618 + 8u)];
                                                                let _e1759 = global_6.member[(_e1621 + 6u)];
                                                                let _e1762 = global_6.member[(_e1621 + 7u)];
                                                                let _e1765 = global_6.member[(_e1621 + 8u)];
                                                                let _e1774 = global_6.member[(_e1625 + 6u)];
                                                                let _e1777 = global_6.member[(_e1625 + 7u)];
                                                                let _e1780 = global_6.member[(_e1625 + 8u)];
                                                                let _e1789 = global_6.member[(_e1628 + 6u)];
                                                                let _e1792 = global_6.member[(_e1628 + 7u)];
                                                                let _e1795 = global_6.member[(_e1628 + 8u)];
                                                                let _e1803 = (_e1501 + (_e1569 * 294912u));
                                                                let _e1804 = (_e1803 + _e1617);
                                                                let _e1805 = (_e1803 + _e1620);
                                                                let _e1806 = (_e1803 + _e1624);
                                                                let _e1807 = (_e1803 + _e1627);
                                                                let _e1809 = global_6.member[_e1804];
                                                                let _e1812 = global_6.member[(_e1804 + 1u)];
                                                                let _e1815 = global_6.member[(_e1804 + 2u)];
                                                                let _e1820 = global_6.member[_e1805];
                                                                let _e1823 = global_6.member[(_e1805 + 1u)];
                                                                let _e1826 = global_6.member[(_e1805 + 2u)];
                                                                let _e1834 = global_6.member[_e1806];
                                                                let _e1837 = global_6.member[(_e1806 + 1u)];
                                                                let _e1840 = global_6.member[(_e1806 + 2u)];
                                                                let _e1848 = global_6.member[_e1807];
                                                                let _e1851 = global_6.member[(_e1807 + 1u)];
                                                                let _e1854 = global_6.member[(_e1807 + 2u)];
                                                                let _e1863 = global_6.member[(_e1804 + 3u)];
                                                                let _e1866 = global_6.member[(_e1804 + 4u)];
                                                                let _e1869 = global_6.member[(_e1804 + 5u)];
                                                                let _e1875 = global_6.member[(_e1805 + 3u)];
                                                                let _e1878 = global_6.member[(_e1805 + 4u)];
                                                                let _e1881 = global_6.member[(_e1805 + 5u)];
                                                                let _e1890 = global_6.member[(_e1806 + 3u)];
                                                                let _e1893 = global_6.member[(_e1806 + 4u)];
                                                                let _e1896 = global_6.member[(_e1806 + 5u)];
                                                                let _e1905 = global_6.member[(_e1807 + 3u)];
                                                                let _e1908 = global_6.member[(_e1807 + 4u)];
                                                                let _e1911 = global_6.member[(_e1807 + 5u)];
                                                                let _e1920 = global_6.member[(_e1804 + 6u)];
                                                                let _e1923 = global_6.member[(_e1804 + 7u)];
                                                                let _e1926 = global_6.member[(_e1804 + 8u)];
                                                                let _e1932 = global_6.member[(_e1805 + 6u)];
                                                                let _e1935 = global_6.member[(_e1805 + 7u)];
                                                                let _e1938 = global_6.member[(_e1805 + 8u)];
                                                                let _e1947 = global_6.member[(_e1806 + 6u)];
                                                                let _e1950 = global_6.member[(_e1806 + 7u)];
                                                                let _e1953 = global_6.member[(_e1806 + 8u)];
                                                                let _e1962 = global_6.member[(_e1807 + 6u)];
                                                                let _e1965 = global_6.member[(_e1807 + 7u)];
                                                                let _e1968 = global_6.member[(_e1807 + 8u)];
                                                                let _e1975 = (1f - _e1579);
                                                                phi_2637_ = type_51(1u, type_49(vec3<f32>(((((((_e1636 * _e1631) + (_e1647 * _e1632)) + (_e1661 * _e1633)) + (_e1675 * _e1634)) * _e1975) + (((((_e1809 * _e1631) + (_e1820 * _e1632)) + (_e1834 * _e1633)) + (_e1848 * _e1634)) * _e1579)), ((((((_e1639 * _e1631) + (_e1650 * _e1632)) + (_e1664 * _e1633)) + (_e1678 * _e1634)) * _e1975) + (((((_e1812 * _e1631) + (_e1823 * _e1632)) + (_e1837 * _e1633)) + (_e1851 * _e1634)) * _e1579)), ((((((_e1642 * _e1631) + (_e1653 * _e1632)) + (_e1667 * _e1633)) + (_e1681 * _e1634)) * _e1975) + (((((_e1815 * _e1631) + (_e1826 * _e1632)) + (_e1840 * _e1633)) + (_e1854 * _e1634)) * _e1579))), vec3<f32>(((((((_e1690 * _e1631) + (_e1702 * _e1632)) + (_e1717 * _e1633)) + (_e1732 * _e1634)) * _e1975) + (((((_e1863 * _e1631) + (_e1875 * _e1632)) + (_e1890 * _e1633)) + (_e1905 * _e1634)) * _e1579)), ((((((_e1693 * _e1631) + (_e1705 * _e1632)) + (_e1720 * _e1633)) + (_e1735 * _e1634)) * _e1975) + (((((_e1866 * _e1631) + (_e1878 * _e1632)) + (_e1893 * _e1633)) + (_e1908 * _e1634)) * _e1579)), ((((((_e1696 * _e1631) + (_e1708 * _e1632)) + (_e1723 * _e1633)) + (_e1738 * _e1634)) * _e1975) + (((((_e1869 * _e1631) + (_e1881 * _e1632)) + (_e1896 * _e1633)) + (_e1911 * _e1634)) * _e1579))), vec3<f32>(((((((_e1747 * _e1631) + (_e1759 * _e1632)) + (_e1774 * _e1633)) + (_e1789 * _e1634)) * _e1975) + (((((_e1920 * _e1631) + (_e1932 * _e1632)) + (_e1947 * _e1633)) + (_e1962 * _e1634)) * _e1579)), ((((((_e1750 * _e1631) + (_e1762 * _e1632)) + (_e1777 * _e1633)) + (_e1792 * _e1634)) * _e1975) + (((((_e1923 * _e1631) + (_e1935 * _e1632)) + (_e1950 * _e1633)) + (_e1965 * _e1634)) * _e1579)), ((((((_e1753 * _e1631) + (_e1765 * _e1632)) + (_e1780 * _e1633)) + (_e1795 * _e1634)) * _e1975) + (((((_e1926 * _e1631) + (_e1938 * _e1632)) + (_e1953 * _e1633)) + (_e1968 * _e1634)) * _e1579)))));
                                                            }
                                                            let _e2012 = phi_2637_;
                                                            let _e2015 = (bitcast<i32>(_e2012.member) != 0i);
                                                            if _e2015 {
                                                                phi_2645_ = _e2012.member_1;
                                                            } else {
                                                                phi_2645_ = type_49();
                                                            }
                                                            let _e2018 = phi_2645_;
                                                            phi_2647_ = _e2018;
                                                            phi_2648_ = select(true, false, _e2015);
                                                        }
                                                        let _e2021 = phi_2647_;
                                                        let _e2023 = phi_2648_;
                                                        phi_2649_ = _e2021;
                                                        phi_2650_ = _e1507;
                                                        phi_2651_ = _e2023;
                                                    }
                                                    let _e2025 = phi_2649_;
                                                    let _e2027 = phi_2650_;
                                                    let _e2029 = phi_2651_;
                                                    if select(_e2029, true, _e2027) {
                                                        let _e2038 = ((((_e313.member.x - _e1432) * _e313.member_1.x) + ((_e313.member.y - _e1435) * _e313.member_1.y)) + ((_e313.member.z - _e1438) * _e313.member_1.z));
                                                        let _e2050 = ((((_e313.member.x * _e313.member.x) + (_e313.member.y * _e313.member.y)) + (_e313.member.z * _e313.member.z)) - (2f * (((_e313.member.x * _e1432) + (_e313.member.y * _e1435)) + (_e313.member.z * _e1438))));
                                                        let _e2051 = (_e2050 + _e1450);
                                                        let _e2052 = (_e2038 * _e2038);
                                                        let _e2053 = (_e2052 - _e2051);
                                                        if (_e2053 < 0f) {
                                                            phi_21534_ = type_53(0u, type_53().member_1);
                                                            phi_21535_ = type_53();
                                                            phi_21536_ = true;
                                                        } else {
                                                            if (_e2038 < 0f) {
                                                                phi_21506_ = -(sqrt(_e2053));
                                                            } else {
                                                                phi_21506_ = sqrt(_e2053);
                                                            }
                                                            let _e2060 = phi_21506_;
                                                            let _e2062 = -((_e2038 + _e2060));
                                                            let _e2063 = (_e2062 == 0f);
                                                            if _e2063 {
                                                                phi_21529_ = type_53(1u, type_52(0f, 0f));
                                                                phi_21530_ = type_53();
                                                            } else {
                                                                let _e2064 = (_e2051 / _e2062);
                                                                if (_e2062 < _e2064) {
                                                                    phi_21518_ = type_52(_e2062, _e2064);
                                                                } else {
                                                                    phi_21518_ = type_52(_e2064, _e2062);
                                                                }
                                                                let _e2069 = phi_21518_;
                                                                phi_21529_ = type_53();
                                                                phi_21530_ = type_53(1u, _e2069);
                                                            }
                                                            let _e2072 = phi_21529_;
                                                            let _e2074 = phi_21530_;
                                                            phi_21534_ = _e2072;
                                                            phi_21535_ = _e2074;
                                                            phi_21536_ = _e2063;
                                                        }
                                                        let _e2079 = phi_21534_;
                                                        let _e2081 = phi_21535_;
                                                        let _e2083 = phi_21536_;
                                                        if _e2083 {
                                                            phi_21540_ = _e2079;
                                                        } else {
                                                            phi_21540_ = _e2081;
                                                        }
                                                        let _e2085 = phi_21540_;
                                                        if (bitcast<i32>(_e2085.member) != 0i) {
                                                            phi_21427_ = type_53(0u, _e2085.member_1);
                                                        } else {
                                                            phi_21427_ = type_53(1u, type_53().member_1);
                                                        }
                                                        let _e2095 = phi_21427_;
                                                        if (bitcast<i32>(_e2095.member) != 0i) {
                                                            phi_21474_ = type_53(0u, type_53().member_1);
                                                        } else {
                                                            if (_e2095.member_1.member_1 <= 0f) {
                                                                phi_21470_ = type_53(0u, type_53().member_1);
                                                            } else {
                                                                if (_e2095.member_1.member != _e2095.member_1.member) {
                                                                    phi_21578_ = true;
                                                                } else {
                                                                    phi_21578_ = (0f >= _e2095.member_1.member);
                                                                }
                                                                let _e2107 = phi_21578_;
                                                                let _e2108 = select(_e2095.member_1.member, 0f, _e2107);
                                                                let _e2109 = (_e2050 + _e1447);
                                                                if (_e2109 > 0f) {
                                                                    let _e2111 = (_e2052 - _e2109);
                                                                    if (_e2111 < 0f) {
                                                                        phi_21686_ = type_53(0u, type_53().member_1);
                                                                        phi_21687_ = type_53();
                                                                        phi_21688_ = true;
                                                                    } else {
                                                                        if (_e2038 < 0f) {
                                                                            phi_21658_ = -(sqrt(_e2111));
                                                                        } else {
                                                                            phi_21658_ = sqrt(_e2111);
                                                                        }
                                                                        let _e2118 = phi_21658_;
                                                                        let _e2120 = -((_e2038 + _e2118));
                                                                        let _e2121 = (_e2120 == 0f);
                                                                        if _e2121 {
                                                                            phi_21681_ = type_53(1u, type_52(0f, 0f));
                                                                            phi_21682_ = type_53();
                                                                        } else {
                                                                            let _e2122 = (_e2109 / _e2120);
                                                                            if (_e2120 < _e2122) {
                                                                                phi_21670_ = type_52(_e2120, _e2122);
                                                                            } else {
                                                                                phi_21670_ = type_52(_e2122, _e2120);
                                                                            }
                                                                            let _e2127 = phi_21670_;
                                                                            phi_21681_ = type_53();
                                                                            phi_21682_ = type_53(1u, _e2127);
                                                                        }
                                                                        let _e2130 = phi_21681_;
                                                                        let _e2132 = phi_21682_;
                                                                        phi_21686_ = _e2130;
                                                                        phi_21687_ = _e2132;
                                                                        phi_21688_ = _e2121;
                                                                    }
                                                                    let _e2137 = phi_21686_;
                                                                    let _e2139 = phi_21687_;
                                                                    let _e2141 = phi_21688_;
                                                                    if _e2141 {
                                                                        phi_21692_ = _e2137;
                                                                    } else {
                                                                        phi_21692_ = _e2139;
                                                                    }
                                                                    let _e2143 = phi_21692_;
                                                                    if (bitcast<i32>(_e2143.member) != 0i) {
                                                                        phi_21626_ = select(_e2143.member_1.member_1, _e2143.member_1.member, (_e2143.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_21626_ = -1f;
                                                                    }
                                                                    let _e2154 = phi_21626_;
                                                                    if (_e2154 >= 0f) {
                                                                        if (_e2095.member_1.member_1 != _e2095.member_1.member_1) {
                                                                            phi_21730_ = true;
                                                                        } else {
                                                                            phi_21730_ = (_e2154 <= _e2095.member_1.member_1);
                                                                        }
                                                                        let _e2159 = phi_21730_;
                                                                        phi_21455_ = select(_e2095.member_1.member_1, _e2154, _e2159);
                                                                    } else {
                                                                        phi_21455_ = _e2095.member_1.member_1;
                                                                    }
                                                                    let _e2162 = phi_21455_;
                                                                    phi_21457_ = _e2162;
                                                                } else {
                                                                    phi_21457_ = _e2095.member_1.member_1;
                                                                }
                                                                let _e2164 = phi_21457_;
                                                                if (_e2164 <= _e2108) {
                                                                    phi_21466_ = type_53(0u, type_53().member_1);
                                                                } else {
                                                                    phi_21466_ = type_53(1u, type_52(_e2108, _e2164));
                                                                }
                                                                let _e2172 = phi_21466_;
                                                                phi_21470_ = _e2172;
                                                            }
                                                            let _e2177 = phi_21470_;
                                                            phi_21474_ = _e2177;
                                                        }
                                                        let _e2182 = phi_21474_;
                                                        if (bitcast<i32>(_e2182.member) != 0i) {
                                                            phi_2669_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2672_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2674_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2676_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2678_ = 0u;
                                                            loop {
                                                                let _e2191 = phi_2669_;
                                                                let _e2193 = phi_2672_;
                                                                let _e2195 = phi_2674_;
                                                                let _e2197 = phi_2676_;
                                                                let _e2199 = phi_2678_;
                                                                local_12 = _e2197;
                                                                local_13 = _e2195;
                                                                local_14 = _e2193;
                                                                let _e2200 = (_e2199 < 32u);
                                                                if _e2200 {
                                                                    let _e2201 = -(_e2038);
                                                                    if (_e2182.member_1.member <= _e2182.member_1.member_1) {
                                                                    } else {
                                                                        phi_31542_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2204 = select(_e2201, _e2182.member_1.member, (_e2201 < _e2182.member_1.member));
                                                                    let _e2206 = select(_e2204, _e2182.member_1.member_1, (_e2204 > _e2182.member_1.member_1));
                                                                    let _e2207 = (_e2206 - _e2182.member_1.member);
                                                                    let _e2208 = (_e2182.member_1.member_1 - _e2206);
                                                                    let _e2209 = (_e2182.member_1.member_1 - _e2182.member_1.member);
                                                                    let _e2210 = (0.000000001f * _e2209);
                                                                    if (_e2207 <= _e2210) {
                                                                        let _e2285 = (f32(_e2199) * 0.03125f);
                                                                        let _e2288 = (f32((_e2199 + 1u)) * 0.03125f);
                                                                        let _e2290 = ((_e2285 + _e2288) * 0.5f);
                                                                        phi_2779_ = type_52((_e2182.member_1.member + ((_e2209 * _e2290) * _e2290)), (_e2209 * ((_e2288 * _e2288) - (_e2285 * _e2285))));
                                                                    } else {
                                                                        if (_e2208 <= _e2210) {
                                                                            let _e2268 = (f32((31u - _e2199)) * 0.03125f);
                                                                            let _e2271 = (f32((32u - _e2199)) * 0.03125f);
                                                                            let _e2273 = ((_e2268 + _e2271) * 0.5f);
                                                                            phi_2778_ = type_52((_e2182.member_1.member_1 - ((_e2209 * _e2273) * _e2273)), (_e2209 * ((_e2271 * _e2271) - (_e2268 * _e2268))));
                                                                        } else {
                                                                            let _e2216 = round(((32f * _e2207) / (_e2207 + _e2208)));
                                                                            let _e2221 = select(select(u32(_e2216), 0u, (_e2216 < 0f)), 4294967295u, (_e2216 > 4294967000f));
                                                                            if (_e2221 < 1u) {
                                                                                phi_2762_ = 1u;
                                                                            } else {
                                                                                phi_2762_ = select(_e2221, 31u, (_e2221 > 31u));
                                                                            }
                                                                            let _e2226 = phi_2762_;
                                                                            if (_e2199 < _e2226) {
                                                                                let _e2247 = ((_e2226 - 1u) - _e2199);
                                                                                let _e2249 = f32(_e2226);
                                                                                let _e2250 = (f32(_e2247) / _e2249);
                                                                                let _e2253 = (f32((_e2247 + 1u)) / _e2249);
                                                                                let _e2255 = ((_e2250 + _e2253) * 0.5f);
                                                                                phi_2777_ = type_52((_e2206 - ((_e2207 * _e2255) * _e2255)), (_e2207 * ((_e2253 * _e2253) - (_e2250 * _e2250))));
                                                                            } else {
                                                                                let _e2229 = (_e2199 - _e2226);
                                                                                let _e2231 = f32((32u - _e2226));
                                                                                let _e2232 = (f32(_e2229) / _e2231);
                                                                                let _e2235 = (f32((_e2229 + 1u)) / _e2231);
                                                                                let _e2237 = ((_e2232 + _e2235) * 0.5f);
                                                                                phi_2777_ = type_52((_e2206 + ((_e2208 * _e2237) * _e2237)), (_e2208 * ((_e2235 * _e2235) - (_e2232 * _e2232))));
                                                                            }
                                                                            let _e2265 = phi_2777_;
                                                                            phi_2778_ = _e2265;
                                                                        }
                                                                        let _e2283 = phi_2778_;
                                                                        phi_2779_ = _e2283;
                                                                    }
                                                                    let _e2300 = phi_2779_;
                                                                    let _e2306 = (_e313.member.x + (_e313.member_1.x * _e2300.member));
                                                                    let _e2307 = (_e313.member.y + (_e313.member_1.y * _e2300.member));
                                                                    let _e2308 = (_e313.member.z + (_e313.member_1.z * _e2300.member));
                                                                    let _e2321 = ((((_e2306 * _e2306) + (_e2307 * _e2307)) + (_e2308 * _e2308)) - (2f * (((_e2306 * _e1432) + (_e2307 * _e1435)) + (_e2308 * _e1438))));
                                                                    let _e2322 = (_e2321 + _e1447);
                                                                    let _e2323 = (_e2306 - _e1432);
                                                                    let _e2324 = (_e2307 - _e1435);
                                                                    let _e2325 = (_e2308 - _e1438);
                                                                    let _e2331 = sqrt((((_e2323 * _e2323) + (_e2324 * _e2324)) + (_e2325 * _e2325)));
                                                                    let _e2333 = (_e2322 / (_e2331 + _e1441));
                                                                    let _e2337 = (1f - (abs((_e2333 - 25000f)) * 0.00006666667f));
                                                                    if (_e2337 != _e2337) {
                                                                        phi_21929_ = true;
                                                                    } else {
                                                                        phi_21929_ = (0f >= _e2337);
                                                                    }
                                                                    let _e2341 = phi_21929_;
                                                                    let _e2343 = -(_e2333);
                                                                    let _e2346 = (exp((_e2343 / _e1462)) * _e2300.member_1);
                                                                    let _e2349 = (exp((_e2343 / _e1465)) * _e2300.member_1);
                                                                    let _e2351 = (_e2346 * 0.5f);
                                                                    let _e2352 = (_e2349 * 0.5f);
                                                                    let _e2353 = ((select(_e2337, 0f, _e2341) * _e2300.member_1) * 0.5f);
                                                                    let _e2355 = (_e2191.x + _e2351);
                                                                    let _e2357 = (_e2191.y + _e2352);
                                                                    let _e2359 = (_e2191.z + _e2353);
                                                                    let _e2364 = (_e1481 * 1.1f);
                                                                    let _e2365 = (_e2364 * _e2357);
                                                                    let _e2384 = (_e1481 * _e2349);
                                                                    if (_e1497 == 4294967295u) {
                                                                        phi_3107_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2392 = (1f / _e2331);
                                                                        let _e2400 = ((((_e2323 * _e2392) * _e1453) + ((_e2324 * _e2392) * _e1456)) + ((_e2325 * _e2392) * _e1459));
                                                                        if (_e2333 != _e2333) {
                                                                            phi_22056_ = true;
                                                                        } else {
                                                                            phi_22056_ = (0f >= _e2333);
                                                                        }
                                                                        let _e2404 = phi_22056_;
                                                                        let _e2408 = sqrt((select(_e2333, 0f, _e2404) / (_e1444 - _e1441)));
                                                                        if (_e2408 != _e2408) {
                                                                            phi_22071_ = true;
                                                                        } else {
                                                                            phi_22071_ = (1f <= _e2408);
                                                                        }
                                                                        let _e2412 = phi_22071_;
                                                                        let _e2414 = (select(_e2408, 1f, _e2412) * 31f);
                                                                        let _e2416 = select(_e2400, -1f, (_e2400 < -1f));
                                                                        let _e2420 = ((select(_e2416, 1f, (_e2416 > 1f)) + 1f) * 15.5f);
                                                                        let _e2425 = select(select(u32(_e2414), 0u, (_e2414 < 0f)), 4294967295u, (_e2414 > 4294967000f));
                                                                        let _e2427 = select(_e2425, 30u, (30u < _e2425));
                                                                        let _e2432 = select(select(u32(_e2420), 0u, (_e2420 < 0f)), 4294967295u, (_e2420 > 4294967000f));
                                                                        let _e2434 = select(_e2432, 30u, (30u < _e2432));
                                                                        let _e2436 = (_e2414 - f32(_e2427));
                                                                        let _e2438 = (_e2420 - f32(_e2434));
                                                                        let _e2442 = (_e1497 + (3u * ((_e2427 * 32u) + _e2434)));
                                                                        let _e2447 = (_e1497 + (3u * (((_e2427 + 1u) * 32u) + _e2434)));
                                                                        let _e2449 = global_6.member[_e2442];
                                                                        let _e2452 = global_6.member[(_e2442 + 1u)];
                                                                        let _e2455 = global_6.member[(_e2442 + 2u)];
                                                                        let _e2456 = (1f - _e2438);
                                                                        let _e2462 = global_6.member[(_e2442 + 3u)];
                                                                        let _e2465 = global_6.member[(_e2442 + 4u)];
                                                                        let _e2468 = global_6.member[(_e2442 + 5u)];
                                                                        let _e2475 = (1f - _e2436);
                                                                        let _e2480 = global_6.member[_e2447];
                                                                        let _e2483 = global_6.member[(_e2447 + 1u)];
                                                                        let _e2486 = global_6.member[(_e2447 + 2u)];
                                                                        let _e2492 = global_6.member[(_e2447 + 3u)];
                                                                        let _e2495 = global_6.member[(_e2447 + 4u)];
                                                                        let _e2498 = global_6.member[(_e2447 + 5u)];
                                                                        phi_3107_ = vec3<f32>(((((_e2449 * _e2456) + (_e2462 * _e2438)) * _e2475) + (((_e2480 * _e2456) + (_e2492 * _e2438)) * _e2436)), ((((_e2452 * _e2456) + (_e2465 * _e2438)) * _e2475) + (((_e2483 * _e2456) + (_e2495 * _e2438)) * _e2436)), ((((_e2455 * _e2456) + (_e2468 * _e2438)) * _e2475) + (((_e2486 * _e2456) + (_e2498 * _e2438)) * _e2436)));
                                                                    }
                                                                    let _e2513 = phi_3107_;
                                                                    let _e2531 = (((_e2323 * _e1453) + (_e2324 * _e1456)) + (_e2325 * _e1459));
                                                                    let _e2532 = (_e2531 * _e2531);
                                                                    let _e2533 = (_e2532 - _e2322);
                                                                    if (_e2533 < 0f) {
                                                                        phi_22221_ = type_53(0u, type_53().member_1);
                                                                        phi_22222_ = type_53();
                                                                        phi_22223_ = true;
                                                                    } else {
                                                                        if (_e2531 < 0f) {
                                                                            phi_22193_ = -(sqrt(_e2533));
                                                                        } else {
                                                                            phi_22193_ = sqrt(_e2533);
                                                                        }
                                                                        let _e2540 = phi_22193_;
                                                                        let _e2542 = -((_e2531 + _e2540));
                                                                        let _e2543 = (_e2542 == 0f);
                                                                        if _e2543 {
                                                                            phi_22216_ = type_53(1u, type_52(0f, 0f));
                                                                            phi_22217_ = type_53();
                                                                        } else {
                                                                            let _e2544 = (_e2322 / _e2542);
                                                                            if (_e2542 < _e2544) {
                                                                                phi_22205_ = type_52(_e2542, _e2544);
                                                                            } else {
                                                                                phi_22205_ = type_52(_e2544, _e2542);
                                                                            }
                                                                            let _e2549 = phi_22205_;
                                                                            phi_22216_ = type_53();
                                                                            phi_22217_ = type_53(1u, _e2549);
                                                                        }
                                                                        let _e2552 = phi_22216_;
                                                                        let _e2554 = phi_22217_;
                                                                        phi_22221_ = _e2552;
                                                                        phi_22222_ = _e2554;
                                                                        phi_22223_ = _e2543;
                                                                    }
                                                                    let _e2559 = phi_22221_;
                                                                    let _e2561 = phi_22222_;
                                                                    let _e2563 = phi_22223_;
                                                                    if _e2563 {
                                                                        phi_22227_ = _e2559;
                                                                    } else {
                                                                        phi_22227_ = _e2561;
                                                                    }
                                                                    let _e2565 = phi_22227_;
                                                                    if (bitcast<i32>(_e2565.member) != 0i) {
                                                                        phi_22161_ = select(_e2565.member_1.member_1, _e2565.member_1.member, (_e2565.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_22161_ = -1f;
                                                                    }
                                                                    let _e2576 = phi_22161_;
                                                                    if (_e2576 >= 0f) {
                                                                        phi_22144_ = type_54(0u, type_54().member_1);
                                                                    } else {
                                                                        let _e2578 = (_e2321 + _e1450);
                                                                        let _e2579 = (_e2532 - _e2578);
                                                                        if (_e2579 < 0f) {
                                                                            phi_22331_ = type_53(0u, type_53().member_1);
                                                                            phi_22332_ = type_53();
                                                                            phi_22333_ = true;
                                                                        } else {
                                                                            if (_e2531 < 0f) {
                                                                                phi_22303_ = -(sqrt(_e2579));
                                                                            } else {
                                                                                phi_22303_ = sqrt(_e2579);
                                                                            }
                                                                            let _e2586 = phi_22303_;
                                                                            let _e2588 = -((_e2531 + _e2586));
                                                                            let _e2589 = (_e2588 == 0f);
                                                                            if _e2589 {
                                                                                phi_22326_ = type_53(1u, type_52(0f, 0f));
                                                                                phi_22327_ = type_53();
                                                                            } else {
                                                                                let _e2590 = (_e2578 / _e2588);
                                                                                if (_e2588 < _e2590) {
                                                                                    phi_22315_ = type_52(_e2588, _e2590);
                                                                                } else {
                                                                                    phi_22315_ = type_52(_e2590, _e2588);
                                                                                }
                                                                                let _e2595 = phi_22315_;
                                                                                phi_22326_ = type_53();
                                                                                phi_22327_ = type_53(1u, _e2595);
                                                                            }
                                                                            let _e2598 = phi_22326_;
                                                                            let _e2600 = phi_22327_;
                                                                            phi_22331_ = _e2598;
                                                                            phi_22332_ = _e2600;
                                                                            phi_22333_ = _e2589;
                                                                        }
                                                                        let _e2605 = phi_22331_;
                                                                        let _e2607 = phi_22332_;
                                                                        let _e2609 = phi_22333_;
                                                                        if _e2609 {
                                                                            phi_22337_ = _e2605;
                                                                        } else {
                                                                            phi_22337_ = _e2607;
                                                                        }
                                                                        let _e2611 = phi_22337_;
                                                                        if (bitcast<i32>(_e2611.member) != 0i) {
                                                                            phi_22271_ = select(_e2611.member_1.member_1, _e2611.member_1.member, (_e2611.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_22271_ = -1f;
                                                                        }
                                                                        let _e2622 = phi_22271_;
                                                                        if (_e2622 < 0f) {
                                                                            phi_22140_ = type_54(0u, type_54().member_1);
                                                                        } else {
                                                                            phi_22098_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_22099_ = 0u;
                                                                            loop {
                                                                                let _e2625 = phi_22098_;
                                                                                let _e2627 = phi_22099_;
                                                                                local_11 = _e2625;
                                                                                let _e2628 = (_e2627 < 16u);
                                                                                if _e2628 {
                                                                                    let _e2630 = (f32(_e2627) * 0.0625f);
                                                                                    let _e2631 = (_e2627 + 1u);
                                                                                    let _e2633 = (f32(_e2631) * 0.0625f);
                                                                                    let _e2635 = ((_e2630 + _e2633) * 0.5f);
                                                                                    let _e2637 = ((_e2622 * _e2635) * _e2635);
                                                                                    let _e2641 = (_e2622 * ((_e2633 * _e2633) - (_e2630 * _e2630)));
                                                                                    let _e2646 = (vec3<f32>(_e2306, _e2307, _e2308) + vec3<f32>((_e1453 * _e2637), (_e1456 * _e2637), (_e1459 * _e2637)));
                                                                                    let _e2663 = (_e2646.x - _e1432);
                                                                                    let _e2664 = (_e2646.y - _e1435);
                                                                                    let _e2665 = (_e2646.z - _e1438);
                                                                                    let _e2673 = ((((((_e2646.x * _e2646.x) + (_e2646.y * _e2646.y)) + (_e2646.z * _e2646.z)) - (2f * (((_e2646.x * _e1432) + (_e2646.y * _e1435)) + (_e2646.z * _e1438)))) + _e1447) / (sqrt((((_e2663 * _e2663) + (_e2664 * _e2664)) + (_e2665 * _e2665))) + _e1441));
                                                                                    let _e2677 = (1f - (abs((_e2673 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2677 != _e2677) {
                                                                                        phi_22469_ = true;
                                                                                    } else {
                                                                                        phi_22469_ = (0f >= _e2677);
                                                                                    }
                                                                                    let _e2681 = phi_22469_;
                                                                                    let _e2683 = -(_e2673);
                                                                                    phi_22132_ = (_e2625 + vec3<f32>((exp((_e2683 / _e1462)) * _e2641), (exp((_e2683 / _e1465)) * _e2641), (select(_e2677, 0f, _e2681) * _e2641)));
                                                                                    phi_22133_ = _e2631;
                                                                                } else {
                                                                                    phi_22132_ = vec3<f32>();
                                                                                    phi_22133_ = u32();
                                                                                }
                                                                                let _e2694 = phi_22132_;
                                                                                let _e2696 = phi_22133_;
                                                                                phi_22098_ = _e2694;
                                                                                phi_22099_ = _e2696;
                                                                                if !(_e2628) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2699 = local_11;
                                                                            phi_22140_ = type_54(1u, _e2699);
                                                                        }
                                                                        let _e2705 = phi_22140_;
                                                                        phi_22144_ = _e2705;
                                                                    }
                                                                    let _e2710 = phi_22144_;
                                                                    if (bitcast<i32>(_e2710.member) != 0i) {
                                                                        let _e2715 = (vec3<f32>(_e2355, _e2357, _e2359) + _e2710.member_1);
                                                                        let _e2721 = (_e2364 * _e2715.y);
                                                                        let _e2733 = exp(-((((_e1471 * _e2715.x) + _e2721) + (_e1487 * _e2715.z))));
                                                                        let _e2735 = exp(-((((_e1474 * _e2715.x) + _e2721) + (_e1490 * _e2715.z))));
                                                                        let _e2737 = exp(-((((_e1477 * _e2715.x) + _e2721) + (_e1493 * _e2715.z))));
                                                                        phi_3166_ = vec3<f32>((_e2195.x + (_e2733 * _e2349)), (_e2195.y + (_e2735 * _e2349)), (_e2195.z + (_e2737 * _e2349)));
                                                                        phi_3167_ = vec3<f32>((_e2197.x + (_e2733 * _e2346)), (_e2197.y + (_e2735 * _e2346)), (_e2197.z + (_e2737 * _e2346)));
                                                                    } else {
                                                                        phi_3166_ = _e2195;
                                                                        phi_3167_ = _e2197;
                                                                    }
                                                                    let _e2759 = phi_3166_;
                                                                    let _e2761 = phi_3167_;
                                                                    phi_2670_ = vec3<f32>((_e2355 + _e2351), (_e2357 + _e2352), (_e2359 + _e2353));
                                                                    phi_2673_ = vec3<f32>((_e2193.x + ((exp(-((((_e1471 * _e2355) + _e2365) + (_e1487 * _e2359)))) * ((_e1471 * _e2346) + _e2384)) * _e2513.x)), (_e2193.y + ((exp(-((((_e1474 * _e2355) + _e2365) + (_e1490 * _e2359)))) * ((_e1474 * _e2346) + _e2384)) * _e2513.y)), (_e2193.z + ((exp(-((((_e1477 * _e2355) + _e2365) + (_e1493 * _e2359)))) * ((_e1477 * _e2346) + _e2384)) * _e2513.z)));
                                                                    phi_2675_ = _e2759;
                                                                    phi_2677_ = _e2761;
                                                                    phi_2679_ = (_e2199 + 1u);
                                                                } else {
                                                                    phi_2670_ = vec3<f32>();
                                                                    phi_2673_ = vec3<f32>();
                                                                    phi_2675_ = vec3<f32>();
                                                                    phi_2677_ = vec3<f32>();
                                                                    phi_2679_ = u32();
                                                                }
                                                                let _e2768 = phi_2670_;
                                                                let _e2770 = phi_2673_;
                                                                let _e2772 = phi_2675_;
                                                                let _e2774 = phi_2677_;
                                                                let _e2776 = phi_2679_;
                                                                phi_2669_ = _e2768;
                                                                phi_2672_ = _e2770;
                                                                phi_2674_ = _e2772;
                                                                phi_2676_ = _e2774;
                                                                phi_2678_ = _e2776;
                                                                phi_31542_ = _e1422;
                                                                if !(_e2200) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2779 = phi_31542_;
                                                            phi_31632_ = _e2779;
                                                            if _e2779 {
                                                                break;
                                                            }
                                                            let _e2781 = local_12;
                                                            let _e2784 = local_13;
                                                            let _e2787 = local_14;
                                                            phi_31636_ = _e2779;
                                                            phi_3187_ = type_49((_e2781 * vec3<f32>(_e1471, _e1474, _e1477)), (_e2784 * _e1481), _e2787);
                                                        } else {
                                                            phi_31636_ = _e1422;
                                                            phi_3187_ = type_49(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2790 = phi_31636_;
                                                        let _e2792 = phi_3187_;
                                                        phi_31635_ = _e2790;
                                                        phi_3188_ = _e2792;
                                                    } else {
                                                        phi_31635_ = _e1422;
                                                        phi_3188_ = _e2025;
                                                    }
                                                    let _e2794 = phi_31635_;
                                                    let _e2796 = phi_3188_;
                                                    let _e2804 = (((_e313.member_1.x * _e1453) + (_e313.member_1.y * _e1456)) + (_e313.member_1.z * _e1459));
                                                    let _e2806 = (1f + (_e2804 * _e2804));
                                                    let _e2807 = (0.059683103f * _e2806);
                                                    let _e2814 = (_e1484 * _e1484);
                                                    let _e2825 = ((0.119366206f * ((1f - _e2814) * _e2806)) / ((2f + _e2814) * pow(((1f + _e2814) - ((2f * _e1484) * _e2804)), 1.5f)));
                                                    phi_31634_ = _e2794;
                                                    phi_3211_ = vec3<f32>((_e1424.x + (((((_e2796.member.x * _e2807) + (_e2796.member_1.x * _e2825)) + _e2796.member_2.x) * _e1468) * 0.38317f)), (_e1424.y + (((((_e2796.member.y * _e2807) + (_e2796.member_1.y * _e2825)) + _e2796.member_2.y) * _e1468) * 0.38317f)), (_e1424.z + (((((_e2796.member.z * _e2807) + (_e2796.member_1.z * _e2825)) + _e2796.member_2.z) * _e1468) * 0.38317f)));
                                                    phi_3212_ = (_e1426 + 1i);
                                                    phi_3213_ = bitcast<u32>(_e1504);
                                                } else {
                                                    phi_31634_ = _e1422;
                                                    phi_3211_ = vec3<f32>();
                                                    phi_3212_ = i32();
                                                    phi_3213_ = u32();
                                                }
                                                let _e2856 = phi_31634_;
                                                let _e2858 = phi_3211_;
                                                let _e2860 = phi_3212_;
                                                let _e2862 = phi_3213_;
                                                phi_31633_ = _e2856;
                                                phi_857_ = _e2858;
                                                phi_860_ = _e2860;
                                                phi_862_ = _e2862;
                                                phi_3214_ = _e1430;
                                            } else {
                                                phi_31633_ = _e1422;
                                                phi_857_ = vec3<f32>();
                                                phi_860_ = i32();
                                                phi_862_ = u32();
                                                phi_3214_ = false;
                                            }
                                            let _e2864 = phi_31633_;
                                            let _e2866 = phi_857_;
                                            let _e2868 = phi_860_;
                                            let _e2870 = phi_862_;
                                            let _e2872 = phi_3214_;
                                            phi_31574_ = _e2864;
                                            phi_856_ = _e2866;
                                            phi_859_ = _e2868;
                                            phi_861_ = _e2870;
                                            phi_31632_ = _e2864;
                                            if !(_e2872) {
                                                break;
                                            }
                                        }
                                        let _e2875 = phi_31632_;
                                        phi_31960_ = _e2875;
                                        if _e2875 {
                                            break;
                                        }
                                        if _e1419 {
                                            let _e2878 = global_6.member[(_e1271 + 10u)];
                                            let _e2881 = global_6.member[(_e1271 + 21u)];
                                            let _e2882 = bitcast<u32>(_e2881);
                                            let _e2885 = global_6.member[(_e1271 + 22u)];
                                            let _e2886 = bitcast<u32>(_e2885);
                                            let _e2889 = global_6.member[(_e1271 + 23u)];
                                            let _e2893 = global_6.member[(_e1271 + 24u)];
                                            let _e2897 = global_6.member[(_e1271 + 25u)];
                                            let _e2901 = global_6.member[(_e1271 + 26u)];
                                            let _e2904 = global_6.member[_e1271];
                                            let _e2907 = global_6.member[(_e1271 + 1u)];
                                            let _e2910 = global_6.member[(_e1271 + 2u)];
                                            let _e2913 = global_6.member[(_e1271 + 3u)];
                                            let _e2916 = global_6.member[(_e1271 + 4u)];
                                            let _e2919 = global_6.member[(_e1271 + 5u)];
                                            let _e2922 = global_6.member[(_e1271 + 6u)];
                                            let _e2925 = global_6.member[(_e1271 + 7u)];
                                            let _e2928 = global_6.member[(_e1271 + 8u)];
                                            let _e2933 = (((_e313.member_1.x * _e2913) + (_e313.member_1.y * _e2916)) + (_e313.member_1.z * _e2919));
                                            let _e2935 = select(_e2933, -1f, (_e2933 < -1f));
                                            let _e2938 = asin(select(_e2935, 1f, (_e2935 > 1f)));
                                            let _e2949 = atan2((((_e313.member_1.x * _e2922) + (_e313.member_1.y * _e2925)) + (_e313.member_1.z * _e2928)), (((_e313.member_1.x * _e2904) + (_e313.member_1.y * _e2907)) + (_e313.member_1.z * _e2910)));
                                            if (_e2882 != 4294967295u) {
                                                let _e2952 = global_6.member[_e2882];
                                                let _e2955 = global_6.member[(_e2882 + 1u)];
                                                let _e2956 = bitcast<u32>(_e2955);
                                                let _e2959 = global_6.member[(_e2882 + 4u)];
                                                phi_3548_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3551_ = 0u;
                                                loop {
                                                    let _e2965 = phi_3548_;
                                                    let _e2967 = phi_3551_;
                                                    local_15 = _e2965;
                                                    let _e2968 = (_e2967 < 6u);
                                                    if _e2968 {
                                                        let _e2969 = (_e2967 / 2u);
                                                        if (_e2969 == 0u) {
                                                            phi_22597_ = _e313.member_1.x;
                                                        } else {
                                                            phi_22597_ = select(_e313.member_1.z, _e313.member_1.y, (_e2969 == 1u));
                                                        }
                                                        let _e2977 = phi_22597_;
                                                        let _e2978 = (_e2977 * select(-1f, 1f, ((_e2967 % 2u) == 0u)));
                                                        if (_e2978 > 0f) {
                                                            let _e2982 = ((_e2969 + 1u) % 3u);
                                                            if (_e2982 == 0u) {
                                                                phi_22612_ = _e313.member_1.x;
                                                            } else {
                                                                phi_22612_ = select(_e313.member_1.z, _e313.member_1.y, (_e2982 == 1u));
                                                            }
                                                            let _e2987 = phi_22612_;
                                                            let _e2988 = (_e2987 / _e2978);
                                                            let _e2990 = ((_e2969 + 2u) % 3u);
                                                            if (_e2990 == 0u) {
                                                                phi_22627_ = _e313.member_1.x;
                                                            } else {
                                                                phi_22627_ = select(_e313.member_1.z, _e313.member_1.y, (_e2990 == 1u));
                                                            }
                                                            let _e2995 = phi_22627_;
                                                            let _e2996 = (_e2995 / _e2978);
                                                            let _e2998 = (1f + (_e2952 * 12f));
                                                            if (abs(_e2988) <= _e2998) {
                                                                if (abs(_e2996) <= _e2998) {
                                                                    let _e3004 = f32(_e2956);
                                                                    let _e3006 = floor((((_e2988 + 1f) * 0.5f) * _e3004));
                                                                    let _e3013 = select(0i, select(select(i32(_e3006), i32(-2147483648), (_e3006 < -2147483600f)), 2147483647i, (_e3006 > 2147483500f)), (_e3006 == _e3006));
                                                                    let _e3015 = (bitcast<i32>(_e2956) - 1i);
                                                                    let _e3016 = (0i <= _e3015);
                                                                    if _e3016 {
                                                                    } else {
                                                                        phi_31637_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3013 < 0i) {
                                                                        phi_3622_ = 0i;
                                                                    } else {
                                                                        phi_3622_ = select(_e3013, _e3015, (_e3013 > _e3015));
                                                                    }
                                                                    let _e3021 = phi_3622_;
                                                                    let _e3025 = floor((((_e2996 + 1f) * 0.5f) * _e3004));
                                                                    let _e3032 = select(0i, select(select(i32(_e3025), i32(-2147483648), (_e3025 < -2147483600f)), 2147483647i, (_e3025 > 2147483500f)), (_e3025 == _e3025));
                                                                    if _e3016 {
                                                                    } else {
                                                                        phi_31637_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3032 < 0i) {
                                                                        phi_3652_ = 0i;
                                                                    } else {
                                                                        phi_3652_ = select(_e3032, _e3015, (_e3032 > _e3015));
                                                                    }
                                                                    let _e3037 = phi_3652_;
                                                                    let _e3038 = (_e3021 - 1i);
                                                                    phi_3660_ = _e2965;
                                                                    phi_3663_ = select(0i, _e3038, (0i < _e3038));
                                                                    loop {
                                                                        let _e3042 = phi_3660_;
                                                                        let _e3044 = phi_3663_;
                                                                        let _e3045 = (_e3021 + 1i);
                                                                        local_59 = _e3042;
                                                                        let _e3048 = (_e3044 <= select(_e3045, _e3015, (_e3015 < _e3045)));
                                                                        if _e3048 {
                                                                            let _e3049 = (_e3037 - 1i);
                                                                            phi_3686_ = _e3042;
                                                                            phi_3689_ = select(0i, _e3049, (0i < _e3049));
                                                                            loop {
                                                                                let _e3053 = phi_3686_;
                                                                                let _e3055 = phi_3689_;
                                                                                let _e3056 = (_e3037 + 1i);
                                                                                local_58 = _e3053;
                                                                                let _e3059 = (_e3055 <= select(_e3056, _e3015, (_e3015 < _e3056)));
                                                                                if _e3059 {
                                                                                    let _e3068 = global_6.member[(bitcast<u32>(_e2959) + ((((_e2967 * _e2956) + bitcast<u32>(_e3044)) * _e2956) + bitcast<u32>(_e3055)))];
                                                                                    phi_3687_ = vec3<f32>((_e3053.x + _e3068), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3690_ = (_e3055 + 1i);
                                                                                } else {
                                                                                    phi_3687_ = vec3<f32>();
                                                                                    phi_3690_ = i32();
                                                                                }
                                                                                let _e3077 = phi_3687_;
                                                                                let _e3079 = phi_3690_;
                                                                                phi_3686_ = _e3077;
                                                                                phi_3689_ = _e3079;
                                                                                if !(_e3059) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6914 = local_58;
                                                                            phi_3661_ = _e6914;
                                                                            phi_3664_ = (_e3044 + 1i);
                                                                        } else {
                                                                            phi_3661_ = vec3<f32>();
                                                                            phi_3664_ = i32();
                                                                        }
                                                                        let _e3083 = phi_3661_;
                                                                        let _e3085 = phi_3664_;
                                                                        phi_3660_ = _e3083;
                                                                        phi_3663_ = _e3085;
                                                                        if !(_e3048) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6918 = local_59;
                                                                    phi_3888_ = _e6918;
                                                                } else {
                                                                    phi_3888_ = _e2965;
                                                                }
                                                                let _e3088 = phi_3888_;
                                                                phi_3889_ = _e3088;
                                                            } else {
                                                                phi_3889_ = _e2965;
                                                            }
                                                            let _e3090 = phi_3889_;
                                                            phi_3890_ = _e3090;
                                                        } else {
                                                            phi_3890_ = _e2965;
                                                        }
                                                        let _e3092 = phi_3890_;
                                                        phi_3549_ = _e3092;
                                                        phi_3552_ = (_e2967 + 1u);
                                                    } else {
                                                        phi_3549_ = vec3<f32>();
                                                        phi_3552_ = u32();
                                                    }
                                                    let _e3095 = phi_3549_;
                                                    let _e3097 = phi_3552_;
                                                    phi_3548_ = _e3095;
                                                    phi_3551_ = _e3097;
                                                    phi_31637_ = _e2875;
                                                    if !(_e2968) {
                                                        break;
                                                    }
                                                }
                                                let _e3100 = phi_31637_;
                                                phi_31960_ = _e3100;
                                                if _e3100 {
                                                    break;
                                                }
                                                let _e3102 = local_15;
                                                phi_31685_ = _e3100;
                                                phi_3901_ = (_e3102.x * (1f / ((6.2831855f * _e2952) * _e2952)));
                                            } else {
                                                phi_31685_ = _e2875;
                                                phi_3901_ = 0f;
                                            }
                                            let _e3106 = phi_31685_;
                                            let _e3108 = phi_3901_;
                                            if (_e2886 != 4294967295u) {
                                                let _e3125 = (_e2938 * 1.3333334f);
                                                let _e3132 = global_6.member[_e2886];
                                                phi_3925_ = (_e3108 + ((_e3132 * ((exp((-(abs(_e2938)) / (0.06f + (abs(_e2949) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2949 * -0.71428573f) * (_e2949 * 0.71428573f)))))) + (1.3f * exp((((_e2949 * _e2949) + (_e3125 * _e3125)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3925_ = _e3108;
                                            }
                                            let _e3138 = phi_3925_;
                                            phi_3927_ = 0u;
                                            phi_3930_ = _e3138;
                                            loop {
                                                let _e3140 = phi_3927_;
                                                let _e3142 = phi_3930_;
                                                local_60 = _e3142;
                                                let _e3143 = (_e3140 < bitcast<u32>(_e2893));
                                                if _e3143 {
                                                    let _e3145 = (bitcast<u32>(_e2889) + (_e3140 * 18u));
                                                    let _e3148 = global_6.member[(_e3145 + 1u)];
                                                    let _e3151 = global_6.member[(_e3145 + 2u)];
                                                    let _e3154 = global_6.member[(_e3145 + 3u)];
                                                    let _e3162 = global_6.member[(_e3145 + 5u)];
                                                    if ((((_e313.member_1.x * _e3148) + (_e313.member_1.y * _e3151)) + (_e313.member_1.z * _e3154)) >= _e3162) {
                                                        let _e3165 = global_6.member[_e3145];
                                                        let _e3170 = select(select(u32(_e3165), 0u, (_e3165 < 0f)), 4294967295u, (_e3165 > 4294967000f));
                                                        if (_e3170 == 0u) {
                                                            phi_4034_ = 1.8f;
                                                        } else {
                                                            if (_e3170 == 1u) {
                                                                phi_4033_ = 1f;
                                                            } else {
                                                                phi_4033_ = select(0f, 4.7f, (_e3170 == 2u));
                                                            }
                                                            let _e3176 = phi_4033_;
                                                            phi_4034_ = _e3176;
                                                        }
                                                        let _e3178 = phi_4034_;
                                                        let _e3181 = global_6.member[(_e3145 + 6u)];
                                                        phi_4050_ = (_e3142 + (_e3181 * _e3178));
                                                    } else {
                                                        phi_4050_ = _e3142;
                                                    }
                                                    let _e3185 = phi_4050_;
                                                    phi_3928_ = (_e3140 + 1u);
                                                    phi_3931_ = _e3185;
                                                } else {
                                                    phi_3928_ = u32();
                                                    phi_3931_ = f32();
                                                }
                                                let _e3188 = phi_3928_;
                                                let _e3190 = phi_3931_;
                                                phi_3927_ = _e3188;
                                                phi_3930_ = _e3190;
                                                if !(_e3143) {
                                                    break;
                                                }
                                            }
                                            phi_4053_ = 0u;
                                            let _e6938 = local_60;
                                            phi_4056_ = _e6938;
                                            loop {
                                                let _e3193 = phi_4053_;
                                                let _e3195 = phi_4056_;
                                                local_16 = _e3195;
                                                let _e3196 = (_e3193 < bitcast<u32>(_e2901));
                                                if _e3196 {
                                                    let _e3198 = (bitcast<u32>(_e2897) + (_e3193 * 29u));
                                                    let _e3201 = global_6.member[(_e3198 + 1u)];
                                                    let _e3204 = global_6.member[(_e3198 + 2u)];
                                                    let _e3207 = global_6.member[(_e3198 + 3u)];
                                                    let _e3215 = global_6.member[(_e3198 + 5u)];
                                                    if ((((_e313.member_1.x * _e3201) + (_e313.member_1.y * _e3204)) + (_e313.member_1.z * _e3207)) >= _e3215) {
                                                        let _e3218 = global_6.member[_e3198];
                                                        let _e3222 = global_6.member[(_e3198 + 6u)];
                                                        phi_4164_ = (_e3195 + (_e3222 * select(1f, 3.3808f, (_e3218 < 0.5f))));
                                                    } else {
                                                        phi_4164_ = _e3195;
                                                    }
                                                    let _e3227 = phi_4164_;
                                                    phi_4054_ = (_e3193 + 1u);
                                                    phi_4057_ = _e3227;
                                                } else {
                                                    phi_4054_ = u32();
                                                    phi_4057_ = f32();
                                                }
                                                let _e3230 = phi_4054_;
                                                let _e3232 = phi_4057_;
                                                phi_4053_ = _e3230;
                                                phi_4056_ = _e3232;
                                                if !(_e3196) {
                                                    break;
                                                }
                                            }
                                            let _e3236 = global_6.member[(_e1271 + 9u)];
                                            let _e3238 = local_16;
                                            let _e3241 = local_17;
                                            let _e3245 = local_18;
                                            let _e3250 = local_19;
                                            let _e3255 = ((_e3238 * _e3236) <= (_e2878 * (((0.2126f * _e3241.x) + (0.7152f * _e3245.y)) + (0.0722f * _e3250.z))));
                                            if _e3255 {
                                                phi_31726_ = _e3106;
                                                phi_4723_ = vec3<f32>();
                                                phi_4724_ = bool();
                                            } else {
                                                let _e3257 = global_6.member[_e1268];
                                                let _e3260 = global_6.member[(_e1268 + 1u)];
                                                let _e3263 = global_6.member[(_e1268 + 2u)];
                                                let _e3266 = global_6.member[(_e1268 + 3u)];
                                                let _e3269 = global_6.member[(_e1268 + 5u)];
                                                let _e3272 = global_6.member[(_e1268 + 6u)];
                                                let _e3275 = global_6.member[(_e1268 + 10u)];
                                                let _e3278 = global_6.member[(_e1268 + 11u)];
                                                let _e3281 = global_6.member[(_e1268 + 13u)];
                                                let _e3284 = global_6.member[(_e1268 + 14u)];
                                                let _e3287 = global_6.member[(_e1268 + 15u)];
                                                let _e3290 = global_6.member[(_e1268 + 16u)];
                                                let _e3293 = global_6.member[(_e1268 + 18u)];
                                                let _e3296 = global_6.member[(_e1268 + 19u)];
                                                let _e3299 = global_6.member[(_e1268 + 20u)];
                                                let _e3311 = ((((_e313.member.x * _e313.member.x) + (_e313.member.y * _e313.member.y)) + (_e313.member.z * _e313.member.z)) - (2f * (((_e313.member.x * _e3257) + (_e313.member.y * _e3260)) + (_e313.member.z * _e3263))));
                                                let _e3312 = (_e3311 + _e3269);
                                                let _e3313 = (_e3312 > 0f);
                                                if _e3313 {
                                                    let _e3321 = ((((_e313.member.x - _e3257) * _e313.member_1.x) + ((_e313.member.y - _e3260) * _e313.member_1.y)) + ((_e313.member.z - _e3263) * _e313.member_1.z));
                                                    let _e3323 = ((_e3321 * _e3321) - _e3312);
                                                    if (_e3323 < 0f) {
                                                        phi_22771_ = type_53(0u, type_53().member_1);
                                                        phi_22772_ = type_53();
                                                        phi_22773_ = true;
                                                    } else {
                                                        if (_e3321 < 0f) {
                                                            phi_22743_ = -(sqrt(_e3323));
                                                        } else {
                                                            phi_22743_ = sqrt(_e3323);
                                                        }
                                                        let _e3330 = phi_22743_;
                                                        let _e3332 = -((_e3321 + _e3330));
                                                        let _e3333 = (_e3332 == 0f);
                                                        if _e3333 {
                                                            phi_22766_ = type_53(1u, type_52(0f, 0f));
                                                            phi_22767_ = type_53();
                                                        } else {
                                                            let _e3334 = (_e3312 / _e3332);
                                                            if (_e3332 < _e3334) {
                                                                phi_22755_ = type_52(_e3332, _e3334);
                                                            } else {
                                                                phi_22755_ = type_52(_e3334, _e3332);
                                                            }
                                                            let _e3339 = phi_22755_;
                                                            phi_22766_ = type_53();
                                                            phi_22767_ = type_53(1u, _e3339);
                                                        }
                                                        let _e3342 = phi_22766_;
                                                        let _e3344 = phi_22767_;
                                                        phi_22771_ = _e3342;
                                                        phi_22772_ = _e3344;
                                                        phi_22773_ = _e3333;
                                                    }
                                                    let _e3349 = phi_22771_;
                                                    let _e3351 = phi_22772_;
                                                    let _e3353 = phi_22773_;
                                                    if _e3353 {
                                                        phi_22777_ = _e3349;
                                                    } else {
                                                        phi_22777_ = _e3351;
                                                    }
                                                    let _e3355 = phi_22777_;
                                                    if (bitcast<i32>(_e3355.member) != 0i) {
                                                        phi_22711_ = select(_e3355.member_1.member_1, _e3355.member_1.member, (_e3355.member_1.member >= 0f));
                                                    } else {
                                                        phi_22711_ = -1f;
                                                    }
                                                    let _e3366 = phi_22711_;
                                                    phi_4565_ = select(true, false, (_e3366 >= 0f));
                                                } else {
                                                    phi_4565_ = true;
                                                }
                                                let _e3370 = phi_4565_;
                                                if _e3370 {
                                                    let _e3378 = ((((_e313.member.x - _e3257) * _e313.member_1.x) + ((_e313.member.y - _e3260) * _e313.member_1.y)) + ((_e313.member.z - _e3263) * _e313.member_1.z));
                                                    let _e3379 = (_e3311 + _e3272);
                                                    let _e3380 = (_e3378 * _e3378);
                                                    let _e3381 = (_e3380 - _e3379);
                                                    if (_e3381 < 0f) {
                                                        phi_22931_ = type_53(0u, type_53().member_1);
                                                        phi_22932_ = type_53();
                                                        phi_22933_ = true;
                                                    } else {
                                                        if (_e3378 < 0f) {
                                                            phi_22903_ = -(sqrt(_e3381));
                                                        } else {
                                                            phi_22903_ = sqrt(_e3381);
                                                        }
                                                        let _e3388 = phi_22903_;
                                                        let _e3390 = -((_e3378 + _e3388));
                                                        let _e3391 = (_e3390 == 0f);
                                                        if _e3391 {
                                                            phi_22926_ = type_53(1u, type_52(0f, 0f));
                                                            phi_22927_ = type_53();
                                                        } else {
                                                            let _e3392 = (_e3379 / _e3390);
                                                            if (_e3390 < _e3392) {
                                                                phi_22915_ = type_52(_e3390, _e3392);
                                                            } else {
                                                                phi_22915_ = type_52(_e3392, _e3390);
                                                            }
                                                            let _e3397 = phi_22915_;
                                                            phi_22926_ = type_53();
                                                            phi_22927_ = type_53(1u, _e3397);
                                                        }
                                                        let _e3400 = phi_22926_;
                                                        let _e3402 = phi_22927_;
                                                        phi_22931_ = _e3400;
                                                        phi_22932_ = _e3402;
                                                        phi_22933_ = _e3391;
                                                    }
                                                    let _e3407 = phi_22931_;
                                                    let _e3409 = phi_22932_;
                                                    let _e3411 = phi_22933_;
                                                    if _e3411 {
                                                        phi_22937_ = _e3407;
                                                    } else {
                                                        phi_22937_ = _e3409;
                                                    }
                                                    let _e3413 = phi_22937_;
                                                    if (bitcast<i32>(_e3413.member) != 0i) {
                                                        phi_22824_ = type_53(0u, _e3413.member_1);
                                                    } else {
                                                        phi_22824_ = type_53(1u, type_53().member_1);
                                                    }
                                                    let _e3423 = phi_22824_;
                                                    if (bitcast<i32>(_e3423.member) != 0i) {
                                                        phi_22871_ = type_53(0u, type_53().member_1);
                                                    } else {
                                                        if (_e3423.member_1.member_1 <= 0f) {
                                                            phi_22867_ = type_53(0u, type_53().member_1);
                                                        } else {
                                                            if (_e3423.member_1.member != _e3423.member_1.member) {
                                                                phi_22975_ = true;
                                                            } else {
                                                                phi_22975_ = (0f >= _e3423.member_1.member);
                                                            }
                                                            let _e3435 = phi_22975_;
                                                            let _e3436 = select(_e3423.member_1.member, 0f, _e3435);
                                                            if _e3313 {
                                                                let _e3437 = (_e3380 - _e3312);
                                                                if (_e3437 < 0f) {
                                                                    phi_23083_ = type_53(0u, type_53().member_1);
                                                                    phi_23084_ = type_53();
                                                                    phi_23085_ = true;
                                                                } else {
                                                                    if (_e3378 < 0f) {
                                                                        phi_23055_ = -(sqrt(_e3437));
                                                                    } else {
                                                                        phi_23055_ = sqrt(_e3437);
                                                                    }
                                                                    let _e3444 = phi_23055_;
                                                                    let _e3446 = -((_e3378 + _e3444));
                                                                    let _e3447 = (_e3446 == 0f);
                                                                    if _e3447 {
                                                                        phi_23078_ = type_53(1u, type_52(0f, 0f));
                                                                        phi_23079_ = type_53();
                                                                    } else {
                                                                        let _e3448 = (_e3312 / _e3446);
                                                                        if (_e3446 < _e3448) {
                                                                            phi_23067_ = type_52(_e3446, _e3448);
                                                                        } else {
                                                                            phi_23067_ = type_52(_e3448, _e3446);
                                                                        }
                                                                        let _e3453 = phi_23067_;
                                                                        phi_23078_ = type_53();
                                                                        phi_23079_ = type_53(1u, _e3453);
                                                                    }
                                                                    let _e3456 = phi_23078_;
                                                                    let _e3458 = phi_23079_;
                                                                    phi_23083_ = _e3456;
                                                                    phi_23084_ = _e3458;
                                                                    phi_23085_ = _e3447;
                                                                }
                                                                let _e3463 = phi_23083_;
                                                                let _e3465 = phi_23084_;
                                                                let _e3467 = phi_23085_;
                                                                if _e3467 {
                                                                    phi_23089_ = _e3463;
                                                                } else {
                                                                    phi_23089_ = _e3465;
                                                                }
                                                                let _e3469 = phi_23089_;
                                                                if (bitcast<i32>(_e3469.member) != 0i) {
                                                                    phi_23023_ = select(_e3469.member_1.member_1, _e3469.member_1.member, (_e3469.member_1.member >= 0f));
                                                                } else {
                                                                    phi_23023_ = -1f;
                                                                }
                                                                let _e3480 = phi_23023_;
                                                                if (_e3480 >= 0f) {
                                                                    if (_e3423.member_1.member_1 != _e3423.member_1.member_1) {
                                                                        phi_23127_ = true;
                                                                    } else {
                                                                        phi_23127_ = (_e3480 <= _e3423.member_1.member_1);
                                                                    }
                                                                    let _e3485 = phi_23127_;
                                                                    phi_22852_ = select(_e3423.member_1.member_1, _e3480, _e3485);
                                                                } else {
                                                                    phi_22852_ = _e3423.member_1.member_1;
                                                                }
                                                                let _e3488 = phi_22852_;
                                                                phi_22854_ = _e3488;
                                                            } else {
                                                                phi_22854_ = _e3423.member_1.member_1;
                                                            }
                                                            let _e3490 = phi_22854_;
                                                            if (_e3490 <= _e3436) {
                                                                phi_22863_ = type_53(0u, type_53().member_1);
                                                            } else {
                                                                phi_22863_ = type_53(1u, type_52(_e3436, _e3490));
                                                            }
                                                            let _e3498 = phi_22863_;
                                                            phi_22867_ = _e3498;
                                                        }
                                                        let _e3503 = phi_22867_;
                                                        phi_22871_ = _e3503;
                                                    }
                                                    let _e3508 = phi_22871_;
                                                    let _e3511 = (bitcast<i32>(_e3508.member) != 0i);
                                                    if _e3511 {
                                                        phi_4579_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4582_ = 0u;
                                                        loop {
                                                            let _e3517 = phi_4579_;
                                                            let _e3519 = phi_4582_;
                                                            local_20 = _e3517;
                                                            local_21 = _e3517;
                                                            local_22 = _e3517;
                                                            let _e3520 = (_e3519 < 32u);
                                                            if _e3520 {
                                                                let _e3521 = -(_e3378);
                                                                if (_e3508.member_1.member <= _e3508.member_1.member_1) {
                                                                } else {
                                                                    phi_31664_ = true;
                                                                    break;
                                                                }
                                                                let _e3524 = select(_e3521, _e3508.member_1.member, (_e3521 < _e3508.member_1.member));
                                                                let _e3526 = select(_e3524, _e3508.member_1.member_1, (_e3524 > _e3508.member_1.member_1));
                                                                let _e3527 = (_e3526 - _e3508.member_1.member);
                                                                let _e3528 = (_e3508.member_1.member_1 - _e3526);
                                                                let _e3529 = (_e3508.member_1.member_1 - _e3508.member_1.member);
                                                                let _e3530 = (0.000000001f * _e3529);
                                                                if (_e3527 <= _e3530) {
                                                                    let _e3605 = (f32(_e3519) * 0.03125f);
                                                                    let _e3608 = (f32((_e3519 + 1u)) * 0.03125f);
                                                                    let _e3610 = ((_e3605 + _e3608) * 0.5f);
                                                                    phi_4683_ = type_52((_e3508.member_1.member + ((_e3529 * _e3610) * _e3610)), (_e3529 * ((_e3608 * _e3608) - (_e3605 * _e3605))));
                                                                } else {
                                                                    if (_e3528 <= _e3530) {
                                                                        let _e3588 = (f32((31u - _e3519)) * 0.03125f);
                                                                        let _e3591 = (f32((32u - _e3519)) * 0.03125f);
                                                                        let _e3593 = ((_e3588 + _e3591) * 0.5f);
                                                                        phi_4682_ = type_52((_e3508.member_1.member_1 - ((_e3529 * _e3593) * _e3593)), (_e3529 * ((_e3591 * _e3591) - (_e3588 * _e3588))));
                                                                    } else {
                                                                        let _e3536 = round(((32f * _e3527) / (_e3527 + _e3528)));
                                                                        let _e3541 = select(select(u32(_e3536), 0u, (_e3536 < 0f)), 4294967295u, (_e3536 > 4294967000f));
                                                                        if (_e3541 < 1u) {
                                                                            phi_4666_ = 1u;
                                                                        } else {
                                                                            phi_4666_ = select(_e3541, 31u, (_e3541 > 31u));
                                                                        }
                                                                        let _e3546 = phi_4666_;
                                                                        if (_e3519 < _e3546) {
                                                                            let _e3567 = ((_e3546 - 1u) - _e3519);
                                                                            let _e3569 = f32(_e3546);
                                                                            let _e3570 = (f32(_e3567) / _e3569);
                                                                            let _e3573 = (f32((_e3567 + 1u)) / _e3569);
                                                                            let _e3575 = ((_e3570 + _e3573) * 0.5f);
                                                                            phi_4681_ = type_52((_e3526 - ((_e3527 * _e3575) * _e3575)), (_e3527 * ((_e3573 * _e3573) - (_e3570 * _e3570))));
                                                                        } else {
                                                                            let _e3549 = (_e3519 - _e3546);
                                                                            let _e3551 = f32((32u - _e3546));
                                                                            let _e3552 = (f32(_e3549) / _e3551);
                                                                            let _e3555 = (f32((_e3549 + 1u)) / _e3551);
                                                                            let _e3557 = ((_e3552 + _e3555) * 0.5f);
                                                                            phi_4681_ = type_52((_e3526 + ((_e3528 * _e3557) * _e3557)), (_e3528 * ((_e3555 * _e3555) - (_e3552 * _e3552))));
                                                                        }
                                                                        let _e3585 = phi_4681_;
                                                                        phi_4682_ = _e3585;
                                                                    }
                                                                    let _e3603 = phi_4682_;
                                                                    phi_4683_ = _e3603;
                                                                }
                                                                let _e3620 = phi_4683_;
                                                                let _e3626 = (_e313.member.x + (_e313.member_1.x * _e3620.member));
                                                                let _e3627 = (_e313.member.y + (_e313.member_1.y * _e3620.member));
                                                                let _e3628 = (_e313.member.z + (_e313.member_1.z * _e3620.member));
                                                                let _e3642 = (_e3626 - _e3257);
                                                                let _e3643 = (_e3627 - _e3260);
                                                                let _e3644 = (_e3628 - _e3263);
                                                                let _e3652 = ((((((_e3626 * _e3626) + (_e3627 * _e3627)) + (_e3628 * _e3628)) - (2f * (((_e3626 * _e3257) + (_e3627 * _e3260)) + (_e3628 * _e3263)))) + _e3269) / (sqrt((((_e3642 * _e3642) + (_e3643 * _e3643)) + (_e3644 * _e3644))) + _e3266));
                                                                let _e3656 = (1f - (abs((_e3652 - 25000f)) * 0.00006666667f));
                                                                if (_e3656 != _e3656) {
                                                                    phi_23326_ = true;
                                                                } else {
                                                                    phi_23326_ = (0f >= _e3656);
                                                                }
                                                                let _e3660 = phi_23326_;
                                                                let _e3662 = -(_e3652);
                                                                phi_4580_ = (_e3517 + vec3<f32>((exp((_e3662 / _e3275)) * _e3620.member_1), (exp((_e3662 / _e3278)) * _e3620.member_1), (select(_e3656, 0f, _e3660) * _e3620.member_1)));
                                                                phi_4583_ = (_e3519 + 1u);
                                                            } else {
                                                                phi_4580_ = vec3<f32>();
                                                                phi_4583_ = u32();
                                                            }
                                                            let _e3674 = phi_4580_;
                                                            let _e3676 = phi_4583_;
                                                            phi_4579_ = _e3674;
                                                            phi_4582_ = _e3676;
                                                            phi_31664_ = _e3106;
                                                            if !(_e3520) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3679 = phi_31664_;
                                                        phi_31960_ = _e3679;
                                                        if _e3679 {
                                                            break;
                                                        }
                                                        let _e3681 = local_20;
                                                        let _e3688 = local_21;
                                                        let _e3690 = ((_e3290 * 1.1f) * _e3688.y);
                                                        let _e3695 = local_22;
                                                        phi_31731_ = _e3679;
                                                        phi_4708_ = vec3<f32>(exp(-((((_e3281 * _e3681.x) + _e3690) + (_e3293 * _e3695.z)))), exp(-((((_e3284 * _e3681.x) + _e3690) + (_e3296 * _e3695.z)))), exp(-((((_e3287 * _e3681.x) + _e3690) + (_e3299 * _e3695.z)))));
                                                    } else {
                                                        phi_31731_ = _e3106;
                                                        phi_4708_ = vec3<f32>();
                                                    }
                                                    let _e3711 = phi_31731_;
                                                    let _e3713 = phi_4708_;
                                                    phi_31730_ = _e3711;
                                                    phi_4710_ = _e3713;
                                                    phi_4712_ = select(true, false, _e3511);
                                                } else {
                                                    phi_31730_ = _e3106;
                                                    phi_4710_ = vec3<f32>();
                                                    phi_4712_ = true;
                                                }
                                                let _e3716 = phi_31730_;
                                                let _e3718 = phi_4710_;
                                                let _e3720 = phi_4712_;
                                                let _e3724 = select(_e3718, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3370)), vec3(_e3720));
                                                let _e3728 = select(_e3724.y, _e3724.x, (_e3724.x > _e3724.y));
                                                phi_31726_ = _e3716;
                                                phi_4723_ = _e3724;
                                                phi_4724_ = (select(_e3724.z, _e3728, (_e3728 > _e3724.z)) > 0f);
                                            }
                                            let _e3734 = phi_31726_;
                                            let _e3736 = phi_4723_;
                                            let _e3738 = phi_4724_;
                                            phi_31725_ = _e3734;
                                            phi_4726_ = _e3736;
                                            phi_4727_ = _e3738;
                                            phi_4728_ = select(true, false, _e3255);
                                        } else {
                                            phi_31725_ = _e2875;
                                            phi_4726_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4727_ = _e1419;
                                            phi_4728_ = true;
                                        }
                                        let _e3741 = phi_31725_;
                                        let _e3743 = phi_4726_;
                                        let _e3745 = phi_4727_;
                                        let _e3747 = phi_4728_;
                                        phi_31724_ = _e3741;
                                        let _e7009 = local_61;
                                        phi_4732_ = _e7009;
                                        phi_4733_ = _e3743;
                                        phi_4734_ = _e3745;
                                        let _e7014 = local_62;
                                        phi_4735_ = _e7014;
                                        phi_4736_ = _e3747;
                                    } else {
                                        phi_31724_ = _e302;
                                        phi_4732_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4733_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4734_ = _e1419;
                                        phi_4735_ = vec3<f32>();
                                        phi_4736_ = true;
                                    }
                                    let _e3749 = phi_31724_;
                                    let _e3751 = phi_4732_;
                                    let _e3753 = phi_4733_;
                                    let _e3755 = phi_4734_;
                                    let _e3757 = phi_4735_;
                                    let _e3759 = phi_4736_;
                                    if _e3759 {
                                        if _e3755 {
                                            let _e3760 = (_e1271 + 20u);
                                            let _e3763 = global_6.member[(_e1271 + 21u)];
                                            let _e3764 = bitcast<u32>(_e3763);
                                            let _e3767 = global_6.member[(_e1271 + 22u)];
                                            let _e3768 = bitcast<u32>(_e3767);
                                            let _e3771 = global_6.member[(_e1271 + 23u)];
                                            let _e3775 = global_6.member[(_e1271 + 24u)];
                                            let _e3779 = global_6.member[(_e1271 + 25u)];
                                            let _e3783 = global_6.member[(_e1271 + 26u)];
                                            let _e3786 = global_6.member[_e1271];
                                            let _e3789 = global_6.member[(_e1271 + 1u)];
                                            let _e3792 = global_6.member[(_e1271 + 2u)];
                                            let _e3795 = global_6.member[(_e1271 + 3u)];
                                            let _e3798 = global_6.member[(_e1271 + 4u)];
                                            let _e3801 = global_6.member[(_e1271 + 5u)];
                                            let _e3804 = global_6.member[(_e1271 + 6u)];
                                            let _e3807 = global_6.member[(_e1271 + 7u)];
                                            let _e3810 = global_6.member[(_e1271 + 8u)];
                                            let _e3815 = (((_e313.member_1.x * _e3795) + (_e313.member_1.y * _e3798)) + (_e313.member_1.z * _e3801));
                                            let _e3817 = select(_e3815, -1f, (_e3815 < -1f));
                                            let _e3820 = asin(select(_e3817, 1f, (_e3817 > 1f)));
                                            let _e3831 = atan2((((_e313.member_1.x * _e3804) + (_e313.member_1.y * _e3807)) + (_e313.member_1.z * _e3810)), (((_e313.member_1.x * _e3786) + (_e313.member_1.y * _e3789)) + (_e313.member_1.z * _e3792)));
                                            if (_e3764 != 4294967295u) {
                                                let _e3834 = global_6.member[_e3764];
                                                let _e3837 = global_6.member[(_e3764 + 1u)];
                                                let _e3838 = bitcast<u32>(_e3837);
                                                let _e3841 = global_6.member[(_e3764 + 2u)];
                                                let _e3845 = global_6.member[(_e3764 + 3u)];
                                                let _e3847 = (4f * _e3834);
                                                phi_5056_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5059_ = 0u;
                                                loop {
                                                    let _e3852 = phi_5056_;
                                                    let _e3854 = phi_5059_;
                                                    local_66 = _e3852;
                                                    let _e3855 = (_e3854 < 6u);
                                                    if _e3855 {
                                                        let _e3856 = (_e3854 / 2u);
                                                        if (_e3856 == 0u) {
                                                            phi_23397_ = _e313.member_1.x;
                                                        } else {
                                                            phi_23397_ = select(_e313.member_1.z, _e313.member_1.y, (_e3856 == 1u));
                                                        }
                                                        let _e3864 = phi_23397_;
                                                        let _e3865 = (_e3864 * select(-1f, 1f, ((_e3854 % 2u) == 0u)));
                                                        if (_e3865 > 0f) {
                                                            let _e3869 = ((_e3856 + 1u) % 3u);
                                                            if (_e3869 == 0u) {
                                                                phi_23412_ = _e313.member_1.x;
                                                            } else {
                                                                phi_23412_ = select(_e313.member_1.z, _e313.member_1.y, (_e3869 == 1u));
                                                            }
                                                            let _e3874 = phi_23412_;
                                                            let _e3875 = (_e3874 / _e3865);
                                                            let _e3877 = ((_e3856 + 2u) % 3u);
                                                            if (_e3877 == 0u) {
                                                                phi_23427_ = _e313.member_1.x;
                                                            } else {
                                                                phi_23427_ = select(_e313.member_1.z, _e313.member_1.y, (_e3877 == 1u));
                                                            }
                                                            let _e3882 = phi_23427_;
                                                            let _e3883 = (_e3882 / _e3865);
                                                            let _e3885 = (1f + (_e3834 * 12f));
                                                            if (abs(_e3875) <= _e3885) {
                                                                if (abs(_e3883) <= _e3885) {
                                                                    let _e3891 = f32(_e3838);
                                                                    let _e3893 = floor((((_e3875 + 1f) * 0.5f) * _e3891));
                                                                    let _e3900 = select(0i, select(select(i32(_e3893), i32(-2147483648), (_e3893 < -2147483600f)), 2147483647i, (_e3893 > 2147483500f)), (_e3893 == _e3893));
                                                                    let _e3902 = (bitcast<i32>(_e3838) - 1i);
                                                                    let _e3903 = (0i <= _e3902);
                                                                    if _e3903 {
                                                                    } else {
                                                                        phi_31719_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3900 < 0i) {
                                                                        phi_5130_ = 0i;
                                                                    } else {
                                                                        phi_5130_ = select(_e3900, _e3902, (_e3900 > _e3902));
                                                                    }
                                                                    let _e3908 = phi_5130_;
                                                                    let _e3912 = floor((((_e3883 + 1f) * 0.5f) * _e3891));
                                                                    let _e3919 = select(0i, select(select(i32(_e3912), i32(-2147483648), (_e3912 < -2147483600f)), 2147483647i, (_e3912 > 2147483500f)), (_e3912 == _e3912));
                                                                    if _e3903 {
                                                                    } else {
                                                                        phi_31719_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3919 < 0i) {
                                                                        phi_5160_ = 0i;
                                                                    } else {
                                                                        phi_5160_ = select(_e3919, _e3902, (_e3919 > _e3902));
                                                                    }
                                                                    let _e3924 = phi_5160_;
                                                                    let _e3925 = (_e3908 - 1i);
                                                                    phi_5168_ = _e3852;
                                                                    phi_5171_ = select(0i, _e3925, (0i < _e3925));
                                                                    loop {
                                                                        let _e3929 = phi_5168_;
                                                                        let _e3931 = phi_5171_;
                                                                        let _e3932 = (_e3908 + 1i);
                                                                        local_65 = _e3929;
                                                                        let _e3935 = (_e3931 <= select(_e3932, _e3902, (_e3902 < _e3932)));
                                                                        if _e3935 {
                                                                            let _e3936 = (_e3924 - 1i);
                                                                            phi_5194_ = _e3929;
                                                                            phi_5197_ = select(0i, _e3936, (0i < _e3936));
                                                                            loop {
                                                                                let _e3940 = phi_5194_;
                                                                                let _e3942 = phi_5197_;
                                                                                let _e3943 = (_e3924 + 1i);
                                                                                local_64 = _e3940;
                                                                                let _e3946 = (_e3942 <= select(_e3943, _e3902, (_e3902 < _e3943)));
                                                                                if _e3946 {
                                                                                    let _e3953 = (bitcast<u32>(_e3841) + ((((_e3854 * _e3838) + bitcast<u32>(_e3931)) * _e3838) + bitcast<u32>(_e3942)));
                                                                                    let _e3955 = global_6.member[_e3953];
                                                                                    let _e3959 = global_6.member[(_e3953 + 1u)];
                                                                                    phi_5268_ = _e3940;
                                                                                    phi_5271_ = bitcast<u32>(_e3955);
                                                                                    loop {
                                                                                        let _e3962 = phi_5268_;
                                                                                        let _e3964 = phi_5271_;
                                                                                        local_63 = _e3962;
                                                                                        let _e3965 = (_e3964 < bitcast<u32>(_e3959));
                                                                                        if _e3965 {
                                                                                            let _e3967 = (bitcast<u32>(_e3845) + (6u * _e3964));
                                                                                            let _e3969 = global_6.member[_e3967];
                                                                                            let _e3972 = global_6.member[(_e3967 + 1u)];
                                                                                            let _e3975 = global_6.member[(_e3967 + 2u)];
                                                                                            let _e3978 = global_6.member[(_e3967 + 3u)];
                                                                                            let _e3981 = global_6.member[(_e3967 + 4u)];
                                                                                            let _e3984 = global_6.member[(_e3967 + 5u)];
                                                                                            let _e3985 = (_e313.member_1.x - _e3969);
                                                                                            let _e3986 = (_e313.member_1.y - _e3972);
                                                                                            let _e3987 = (_e313.member_1.z - _e3975);
                                                                                            let _e3992 = (((_e3985 * _e3985) + (_e3986 * _e3986)) + (_e3987 * _e3987));
                                                                                            if (_e3992 < (_e3847 * _e3847)) {
                                                                                                let _e4000 = ((1f / ((6.2831855f * _e3834) * _e3834)) * exp((-(_e3992) / ((2f * _e3834) * _e3834))));
                                                                                                phi_5391_ = vec3<f32>((_e3962.x + (_e3978 * _e4000)), (_e3962.y + (_e3981 * _e4000)), (_e3962.z + (_e3984 * _e4000)));
                                                                                            } else {
                                                                                                phi_5391_ = _e3962;
                                                                                            }
                                                                                            let _e4012 = phi_5391_;
                                                                                            phi_5269_ = _e4012;
                                                                                            phi_5272_ = (_e3964 + 1u);
                                                                                        } else {
                                                                                            phi_5269_ = vec3<f32>();
                                                                                            phi_5272_ = u32();
                                                                                        }
                                                                                        let _e4015 = phi_5269_;
                                                                                        let _e4017 = phi_5272_;
                                                                                        phi_5268_ = _e4015;
                                                                                        phi_5271_ = _e4017;
                                                                                        if !(_e3965) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7034 = local_63;
                                                                                    phi_5195_ = _e7034;
                                                                                    phi_5198_ = (_e3942 + 1i);
                                                                                } else {
                                                                                    phi_5195_ = vec3<f32>();
                                                                                    phi_5198_ = i32();
                                                                                }
                                                                                let _e4021 = phi_5195_;
                                                                                let _e4023 = phi_5198_;
                                                                                phi_5194_ = _e4021;
                                                                                phi_5197_ = _e4023;
                                                                                if !(_e3946) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7038 = local_64;
                                                                            phi_5169_ = _e7038;
                                                                            phi_5172_ = (_e3931 + 1i);
                                                                        } else {
                                                                            phi_5169_ = vec3<f32>();
                                                                            phi_5172_ = i32();
                                                                        }
                                                                        let _e4027 = phi_5169_;
                                                                        let _e4029 = phi_5172_;
                                                                        phi_5168_ = _e4027;
                                                                        phi_5171_ = _e4029;
                                                                        if !(_e3935) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7042 = local_65;
                                                                    phi_5396_ = _e7042;
                                                                } else {
                                                                    phi_5396_ = _e3852;
                                                                }
                                                                let _e4032 = phi_5396_;
                                                                phi_5397_ = _e4032;
                                                            } else {
                                                                phi_5397_ = _e3852;
                                                            }
                                                            let _e4034 = phi_5397_;
                                                            phi_5398_ = _e4034;
                                                        } else {
                                                            phi_5398_ = _e3852;
                                                        }
                                                        let _e4036 = phi_5398_;
                                                        phi_5057_ = _e4036;
                                                        phi_5060_ = (_e3854 + 1u);
                                                    } else {
                                                        phi_5057_ = vec3<f32>();
                                                        phi_5060_ = u32();
                                                    }
                                                    let _e4039 = phi_5057_;
                                                    let _e4041 = phi_5060_;
                                                    phi_5056_ = _e4039;
                                                    phi_5059_ = _e4041;
                                                    phi_31719_ = _e3749;
                                                    if !(_e3855) {
                                                        break;
                                                    }
                                                }
                                                let _e4044 = phi_31719_;
                                                phi_31960_ = _e4044;
                                                if _e4044 {
                                                    break;
                                                }
                                                phi_31761_ = _e4044;
                                                let _e7051 = local_66;
                                                phi_5418_ = _e7051;
                                            } else {
                                                phi_31761_ = _e3749;
                                                phi_5418_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4046 = phi_31761_;
                                            let _e4048 = phi_5418_;
                                            if (_e3768 != 4294967295u) {
                                                let _e4051 = global_6.member[_e3768];
                                                let _e4054 = global_6.member[(_e3768 + 1u)];
                                                let _e4057 = global_6.member[(_e3768 + 2u)];
                                                let _e4060 = global_6.member[(_e3768 + 3u)];
                                                let _e4063 = global_6.member[(_e3768 + 4u)];
                                                let _e4066 = global_6.member[(_e3768 + 5u)];
                                                let _e4069 = global_6.member[(_e3768 + 6u)];
                                                let _e4072 = global_6.member[(_e3768 + 7u)];
                                                let _e4075 = global_6.member[(_e3768 + 8u)];
                                                let _e4078 = global_6.member[(_e3768 + 9u)];
                                                let _e4081 = global_6.member[(_e3768 + 10u)];
                                                let _e4085 = global_6.member[(_e3768 + 11u)];
                                                let _e4088 = global_6.member[(_e3768 + 12u)];
                                                let _e4091 = global_6.member[(_e3768 + 13u)];
                                                let _e4093 = (_e4051 > 0f);
                                                let _e4094 = (_e4054 > 0f);
                                                if _e4094 {
                                                    let _e4097 = (abs(_e3820) <= 0.8f);
                                                    let _e4098 = select(2u, 0u, _e4093);
                                                    phi_5631_ = _e4098;
                                                    phi_5632_ = _e4097;
                                                    phi_5635_ = _e4098;
                                                    phi_5636_ = _e4097;
                                                    phi_5637_ = select(true, false, _e4097);
                                                } else {
                                                    phi_5631_ = u32();
                                                    phi_5632_ = bool();
                                                    phi_5635_ = select(2u, 0u, _e4093);
                                                    phi_5636_ = false;
                                                    phi_5637_ = true;
                                                }
                                                let _e4101 = phi_5631_;
                                                let _e4103 = phi_5632_;
                                                let _e4105 = phi_5635_;
                                                let _e4107 = phi_5636_;
                                                let _e4109 = phi_5637_;
                                                let _e4111 = select(select(f32(), 0f, _e4094), 0f, _e4109);
                                                phi_5647_ = _e4111;
                                                phi_5650_ = _e4111;
                                                phi_5652_ = select(_e4101, _e4105, _e4109);
                                                phi_5654_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5656_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5658_ = 0f;
                                                phi_5660_ = 1f;
                                                loop {
                                                    let _e4116 = phi_5647_;
                                                    let _e4118 = phi_5650_;
                                                    let _e4120 = phi_5652_;
                                                    let _e4122 = phi_5654_;
                                                    let _e4124 = phi_5656_;
                                                    let _e4126 = phi_5658_;
                                                    let _e4128 = phi_5660_;
                                                    local_26 = _e4128;
                                                    local_27 = _e4128;
                                                    local_28 = _e4128;
                                                    local_29 = _e4128;
                                                    local_30 = _e4126;
                                                    local_31 = _e4118;
                                                    local_32 = _e4116;
                                                    let _e4129 = (_e4120 < select(11u, 2u, _e4109));
                                                    if _e4129 {
                                                        let _e4130 = (_e4120 < 2u);
                                                        if _e4130 {
                                                            let _e4186 = select(23f, 4f, (_e4120 == 0u));
                                                            phi_5738_ = 4u;
                                                            phi_5740_ = vec3<f32>((_e313.member_1.x * _e4186), (_e313.member_1.y * _e4186), (_e313.member_1.z * _e4186));
                                                        } else {
                                                            if (_e4120 == 2u) {
                                                                phi_5736_ = 3u;
                                                                phi_5737_ = vec3<f32>(((_e313.member_1.x * 1.5f) + 5f), ((_e313.member_1.y * 1.5f) + 5f), ((_e313.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4132 = (_e4120 >= 7u);
                                                                if _e4132 {
                                                                    phi_5698_ = (_e4120 - 7u);
                                                                } else {
                                                                    phi_5698_ = (_e4120 - 3u);
                                                                }
                                                                let _e4136 = phi_5698_;
                                                                if _e4132 {
                                                                    phi_5713_ = vec3<f32>(((_e313.member_1.x * 11f) + -3f), ((_e313.member_1.y * 11f) + 1f), ((_e313.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5713_ = vec3<f32>((_e313.member_1.x * 7f), (_e313.member_1.y * 7f), (_e313.member_1.z * 7f));
                                                                }
                                                                let _e4149 = phi_5713_;
                                                                if (_e4136 < 3u) {
                                                                    if (_e4136 == 0u) {
                                                                        phi_23442_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_23442_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4136 == 1u)));
                                                                    }
                                                                    let _e4168 = phi_23442_;
                                                                    phi_5734_ = 4u;
                                                                    phi_5735_ = (_e4149 + _e4168);
                                                                } else {
                                                                    if _e4132 {
                                                                        phi_5733_ = (_e4149 + _e4122);
                                                                    } else {
                                                                        phi_5733_ = (_e4149 + vec3<f32>((_e4124.x * 1.1f), (_e4124.y * 1.1f), (_e4124.z * 1.1f)));
                                                                    }
                                                                    let _e4161 = phi_5733_;
                                                                    phi_5734_ = select(7u, 5u, _e4132);
                                                                    phi_5735_ = _e4161;
                                                                }
                                                                let _e4171 = phi_5734_;
                                                                let _e4173 = phi_5735_;
                                                                phi_5736_ = _e4171;
                                                                phi_5737_ = _e4173;
                                                            }
                                                            let _e4182 = phi_5736_;
                                                            let _e4184 = phi_5737_;
                                                            phi_5738_ = _e4182;
                                                            phi_5740_ = _e4184;
                                                        }
                                                        let _e4192 = phi_5738_;
                                                        let _e4194 = phi_5740_;
                                                        let _e4196 = select(vec3<f32>(_e4085, _e4088, _e4091), vec3<f32>(_e4075, _e4078, _e4081), vec3(_e4130));
                                                        let _e4198 = global_6.member[_e3760];
                                                        let _e4199 = bitcast<u32>(_e4198);
                                                        let _e4202 = global_6.member[(_e1271 + 11u)];
                                                        let _e4205 = global_6.member[(_e1271 + 12u)];
                                                        let _e4208 = global_6.member[(_e1271 + 13u)];
                                                        let _e4211 = global_6.member[(_e1271 + 14u)];
                                                        let _e4214 = global_6.member[(_e1271 + 15u)];
                                                        let _e4217 = global_6.member[(_e1271 + 16u)];
                                                        let _e4220 = global_6.member[(_e1271 + 17u)];
                                                        let _e4223 = global_6.member[(_e1271 + 18u)];
                                                        let _e4226 = global_6.member[(_e1271 + 19u)];
                                                        phi_5875_ = 0u;
                                                        phi_5878_ = 0f;
                                                        phi_5880_ = 1f;
                                                        phi_5882_ = 0f;
                                                        phi_5884_ = _e4194;
                                                        loop {
                                                            let _e4228 = phi_5875_;
                                                            let _e4230 = phi_5878_;
                                                            let _e4232 = phi_5880_;
                                                            let _e4234 = phi_5882_;
                                                            let _e4236 = phi_5884_;
                                                            local_23 = _e4230;
                                                            local_24 = _e4234;
                                                            local_25 = _e4230;
                                                            let _e4237 = (_e4228 < _e4192);
                                                            if _e4237 {
                                                                let _e4240 = (_e4236.x + _e4196.x);
                                                                let _e4243 = (_e4236.y + _e4196.y);
                                                                let _e4246 = (_e4236.z + _e4196.z);
                                                                let _e4247 = floor(_e4240);
                                                                let _e4248 = floor(_e4243);
                                                                let _e4249 = floor(_e4246);
                                                                let _e4258 = bitcast<u32>((select(0i, select(select(i32(_e4247), i32(-2147483648), (_e4247 < -2147483600f)), 2147483647i, (_e4247 > 2147483500f)), (_e4247 == _e4247)) & 255i));
                                                                let _e4267 = bitcast<u32>((select(0i, select(select(i32(_e4248), i32(-2147483648), (_e4248 < -2147483600f)), 2147483647i, (_e4248 > 2147483500f)), (_e4248 == _e4248)) & 255i));
                                                                let _e4276 = bitcast<u32>((select(0i, select(select(i32(_e4249), i32(-2147483648), (_e4249 < -2147483600f)), 2147483647i, (_e4249 > 2147483500f)), (_e4249 == _e4249)) & 255i));
                                                                let _e4277 = (_e4240 - _e4247);
                                                                let _e4278 = (_e4243 - _e4248);
                                                                let _e4279 = (_e4246 - _e4249);
                                                                let _e4286 = (((_e4277 * _e4277) * _e4277) * ((_e4277 * ((_e4277 * 6f) - 15f)) + 10f));
                                                                let _e4293 = (((_e4278 * _e4278) * _e4278) * ((_e4278 * ((_e4278 * 6f) - 15f)) + 10f));
                                                                let _e4303 = global_6.member[(_e4199 + _e4258)];
                                                                let _e4309 = (select(select(u32(_e4303), 0u, (_e4303 < 0f)), 4294967295u, (_e4303 > 4294967000f)) + _e4267);
                                                                let _e4312 = global_6.member[(_e4199 + _e4309)];
                                                                let _e4318 = (select(select(u32(_e4312), 0u, (_e4312 < 0f)), 4294967295u, (_e4312 > 4294967000f)) + _e4276);
                                                                let _e4322 = global_6.member[(_e4199 + (_e4309 + 1u))];
                                                                let _e4328 = (select(select(u32(_e4322), 0u, (_e4322 < 0f)), 4294967295u, (_e4322 > 4294967000f)) + _e4276);
                                                                let _e4332 = global_6.member[(_e4199 + (_e4258 + 1u))];
                                                                let _e4338 = (select(select(u32(_e4332), 0u, (_e4332 < 0f)), 4294967295u, (_e4332 > 4294967000f)) + _e4267);
                                                                let _e4341 = global_6.member[(_e4199 + _e4338)];
                                                                let _e4347 = (select(select(u32(_e4341), 0u, (_e4341 < 0f)), 4294967295u, (_e4341 > 4294967000f)) + _e4276);
                                                                let _e4351 = global_6.member[(_e4199 + (_e4338 + 1u))];
                                                                let _e4357 = (select(select(u32(_e4351), 0u, (_e4351 < 0f)), 4294967295u, (_e4351 > 4294967000f)) + _e4276);
                                                                let _e4360 = global_6.member[(_e4199 + _e4318)];
                                                                let _e4366 = (select(select(u32(_e4360), 0u, (_e4360 < 0f)), 4294967295u, (_e4360 > 4294967000f)) % 12u);
                                                                let _e4369 = select(-1f, 1f, ((_e4366 & 1u) == 0u));
                                                                let _e4372 = select(-1f, 1f, ((_e4366 & 2u) == 0u));
                                                                if (_e4366 < 4u) {
                                                                    phi_23503_ = ((_e4369 * _e4277) + (_e4372 * _e4278));
                                                                } else {
                                                                    if (_e4366 < 8u) {
                                                                        phi_23497_ = ((_e4369 * _e4277) + (_e4372 * _e4279));
                                                                    } else {
                                                                        phi_23497_ = ((_e4369 * _e4278) + (_e4372 * _e4279));
                                                                    }
                                                                    let _e4382 = phi_23497_;
                                                                    phi_23503_ = _e4382;
                                                                }
                                                                let _e4387 = phi_23503_;
                                                                let _e4390 = global_6.member[(_e4199 + _e4347)];
                                                                let _e4396 = (_e4277 - 1f);
                                                                let _e4397 = (select(select(u32(_e4390), 0u, (_e4390 < 0f)), 4294967295u, (_e4390 > 4294967000f)) % 12u);
                                                                let _e4400 = select(-1f, 1f, ((_e4397 & 1u) == 0u));
                                                                let _e4403 = select(-1f, 1f, ((_e4397 & 2u) == 0u));
                                                                if (_e4397 < 4u) {
                                                                    phi_23537_ = ((_e4400 * _e4396) + (_e4403 * _e4278));
                                                                } else {
                                                                    if (_e4397 < 8u) {
                                                                        phi_23531_ = ((_e4400 * _e4396) + (_e4403 * _e4279));
                                                                    } else {
                                                                        phi_23531_ = ((_e4400 * _e4278) + (_e4403 * _e4279));
                                                                    }
                                                                    let _e4413 = phi_23531_;
                                                                    phi_23537_ = _e4413;
                                                                }
                                                                let _e4418 = phi_23537_;
                                                                let _e4421 = global_6.member[(_e4199 + _e4328)];
                                                                let _e4427 = (_e4278 - 1f);
                                                                let _e4428 = (select(select(u32(_e4421), 0u, (_e4421 < 0f)), 4294967295u, (_e4421 > 4294967000f)) % 12u);
                                                                let _e4431 = select(-1f, 1f, ((_e4428 & 1u) == 0u));
                                                                let _e4434 = select(-1f, 1f, ((_e4428 & 2u) == 0u));
                                                                if (_e4428 < 4u) {
                                                                    phi_23571_ = ((_e4431 * _e4277) + (_e4434 * _e4427));
                                                                } else {
                                                                    if (_e4428 < 8u) {
                                                                        phi_23565_ = ((_e4431 * _e4277) + (_e4434 * _e4279));
                                                                    } else {
                                                                        phi_23565_ = ((_e4431 * _e4427) + (_e4434 * _e4279));
                                                                    }
                                                                    let _e4444 = phi_23565_;
                                                                    phi_23571_ = _e4444;
                                                                }
                                                                let _e4449 = phi_23571_;
                                                                let _e4452 = global_6.member[(_e4199 + _e4357)];
                                                                let _e4458 = (select(select(u32(_e4452), 0u, (_e4452 < 0f)), 4294967295u, (_e4452 > 4294967000f)) % 12u);
                                                                let _e4461 = select(-1f, 1f, ((_e4458 & 1u) == 0u));
                                                                let _e4464 = select(-1f, 1f, ((_e4458 & 2u) == 0u));
                                                                if (_e4458 < 4u) {
                                                                    phi_23605_ = ((_e4461 * _e4396) + (_e4464 * _e4427));
                                                                } else {
                                                                    if (_e4458 < 8u) {
                                                                        phi_23599_ = ((_e4461 * _e4396) + (_e4464 * _e4279));
                                                                    } else {
                                                                        phi_23599_ = ((_e4461 * _e4427) + (_e4464 * _e4279));
                                                                    }
                                                                    let _e4474 = phi_23599_;
                                                                    phi_23605_ = _e4474;
                                                                }
                                                                let _e4479 = phi_23605_;
                                                                let _e4483 = global_6.member[(_e4199 + (_e4318 + 1u))];
                                                                let _e4489 = (_e4279 - 1f);
                                                                let _e4490 = (select(select(u32(_e4483), 0u, (_e4483 < 0f)), 4294967295u, (_e4483 > 4294967000f)) % 12u);
                                                                let _e4493 = select(-1f, 1f, ((_e4490 & 1u) == 0u));
                                                                let _e4496 = select(-1f, 1f, ((_e4490 & 2u) == 0u));
                                                                if (_e4490 < 4u) {
                                                                    phi_23639_ = ((_e4493 * _e4277) + (_e4496 * _e4278));
                                                                } else {
                                                                    if (_e4490 < 8u) {
                                                                        phi_23633_ = ((_e4493 * _e4277) + (_e4496 * _e4489));
                                                                    } else {
                                                                        phi_23633_ = ((_e4493 * _e4278) + (_e4496 * _e4489));
                                                                    }
                                                                    let _e4506 = phi_23633_;
                                                                    phi_23639_ = _e4506;
                                                                }
                                                                let _e4511 = phi_23639_;
                                                                let _e4515 = global_6.member[(_e4199 + (_e4347 + 1u))];
                                                                let _e4521 = (select(select(u32(_e4515), 0u, (_e4515 < 0f)), 4294967295u, (_e4515 > 4294967000f)) % 12u);
                                                                let _e4524 = select(-1f, 1f, ((_e4521 & 1u) == 0u));
                                                                let _e4527 = select(-1f, 1f, ((_e4521 & 2u) == 0u));
                                                                if (_e4521 < 4u) {
                                                                    phi_23673_ = ((_e4524 * _e4396) + (_e4527 * _e4278));
                                                                } else {
                                                                    if (_e4521 < 8u) {
                                                                        phi_23667_ = ((_e4524 * _e4396) + (_e4527 * _e4489));
                                                                    } else {
                                                                        phi_23667_ = ((_e4524 * _e4278) + (_e4527 * _e4489));
                                                                    }
                                                                    let _e4537 = phi_23667_;
                                                                    phi_23673_ = _e4537;
                                                                }
                                                                let _e4542 = phi_23673_;
                                                                let _e4546 = global_6.member[(_e4199 + (_e4328 + 1u))];
                                                                let _e4552 = (select(select(u32(_e4546), 0u, (_e4546 < 0f)), 4294967295u, (_e4546 > 4294967000f)) % 12u);
                                                                let _e4555 = select(-1f, 1f, ((_e4552 & 1u) == 0u));
                                                                let _e4558 = select(-1f, 1f, ((_e4552 & 2u) == 0u));
                                                                if (_e4552 < 4u) {
                                                                    phi_23707_ = ((_e4555 * _e4277) + (_e4558 * _e4427));
                                                                } else {
                                                                    if (_e4552 < 8u) {
                                                                        phi_23701_ = ((_e4555 * _e4277) + (_e4558 * _e4489));
                                                                    } else {
                                                                        phi_23701_ = ((_e4555 * _e4427) + (_e4558 * _e4489));
                                                                    }
                                                                    let _e4568 = phi_23701_;
                                                                    phi_23707_ = _e4568;
                                                                }
                                                                let _e4573 = phi_23707_;
                                                                let _e4577 = global_6.member[(_e4199 + (_e4357 + 1u))];
                                                                let _e4583 = (select(select(u32(_e4577), 0u, (_e4577 < 0f)), 4294967295u, (_e4577 > 4294967000f)) % 12u);
                                                                let _e4586 = select(-1f, 1f, ((_e4583 & 1u) == 0u));
                                                                let _e4589 = select(-1f, 1f, ((_e4583 & 2u) == 0u));
                                                                if (_e4583 < 4u) {
                                                                    phi_23741_ = ((_e4586 * _e4396) + (_e4589 * _e4427));
                                                                } else {
                                                                    if (_e4583 < 8u) {
                                                                        phi_23735_ = ((_e4586 * _e4396) + (_e4589 * _e4489));
                                                                    } else {
                                                                        phi_23735_ = ((_e4586 * _e4427) + (_e4589 * _e4489));
                                                                    }
                                                                    let _e4599 = phi_23735_;
                                                                    phi_23741_ = _e4599;
                                                                }
                                                                let _e4604 = phi_23741_;
                                                                let _e4607 = (_e4387 + (_e4286 * (_e4418 - _e4387)));
                                                                let _e4613 = (_e4511 + (_e4286 * (_e4542 - _e4511)));
                                                                let _e4619 = (_e4607 + (_e4293 * ((_e4449 + (_e4286 * (_e4479 - _e4449))) - _e4607)));
                                                                phi_5876_ = (_e4228 + 1u);
                                                                phi_5879_ = (_e4230 + _e4232);
                                                                phi_5881_ = (_e4232 * 0.5f);
                                                                phi_5883_ = (_e4234 + ((_e4619 + ((((_e4279 * _e4279) * _e4279) * ((_e4279 * ((_e4279 * 6f) - 15f)) + 10f)) * ((_e4613 + (_e4293 * ((_e4573 + (_e4286 * (_e4604 - _e4573))) - _e4613))) - _e4619))) * _e4232));
                                                                phi_5885_ = vec3<f32>(((((_e4202 * _e4236.x) + (_e4205 * _e4236.y)) + (_e4208 * _e4236.z)) * 2.03f), ((((_e4211 * _e4236.x) + (_e4214 * _e4236.y)) + (_e4217 * _e4236.z)) * 2.03f), ((((_e4220 * _e4236.x) + (_e4223 * _e4236.y)) + (_e4226 * _e4236.z)) * 2.03f));
                                                            } else {
                                                                phi_5876_ = u32();
                                                                phi_5879_ = f32();
                                                                phi_5881_ = f32();
                                                                phi_5883_ = f32();
                                                                phi_5885_ = vec3<f32>();
                                                            }
                                                            let _e4651 = phi_5876_;
                                                            let _e4653 = phi_5879_;
                                                            let _e4655 = phi_5881_;
                                                            let _e4657 = phi_5883_;
                                                            let _e4659 = phi_5885_;
                                                            phi_5875_ = _e4651;
                                                            phi_5878_ = _e4653;
                                                            phi_5880_ = _e4655;
                                                            phi_5882_ = _e4657;
                                                            phi_5884_ = _e4659;
                                                            if !(_e4237) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4662 = local_23;
                                                        if (_e4662 > 0f) {
                                                            let _e4665 = local_24;
                                                            let _e4667 = local_25;
                                                            phi_6301_ = (_e4665 / _e4667);
                                                        } else {
                                                            phi_6301_ = 0f;
                                                        }
                                                        let _e4670 = phi_6301_;
                                                        if (_e4120 == 0u) {
                                                            phi_6384_ = _e4116;
                                                            phi_6385_ = _e4118;
                                                            phi_6386_ = _e4122;
                                                            phi_6387_ = _e4124;
                                                            phi_6388_ = _e4126;
                                                            phi_6389_ = (_e4128 + (1.5f * _e4670));
                                                        } else {
                                                            if (_e4120 == 1u) {
                                                                phi_6378_ = _e4116;
                                                                phi_6379_ = _e4118;
                                                                phi_6380_ = _e4122;
                                                                phi_6381_ = _e4124;
                                                                phi_6382_ = _e4126;
                                                                phi_6383_ = (_e4128 + (0.8f * _e4670));
                                                            } else {
                                                                let _e4673 = (_e4120 == 2u);
                                                                if _e4673 {
                                                                    phi_6373_ = _e4116;
                                                                    phi_6374_ = _e4118;
                                                                    phi_6375_ = _e4122;
                                                                    phi_6376_ = _e4124;
                                                                } else {
                                                                    if (_e4120 == 3u) {
                                                                        phi_6369_ = _e4116;
                                                                        phi_6370_ = _e4118;
                                                                        phi_6371_ = _e4122;
                                                                        phi_6372_ = vec3<f32>(_e4670, _e4124.y, _e4124.z);
                                                                    } else {
                                                                        if (_e4120 == 4u) {
                                                                            phi_6365_ = _e4116;
                                                                            phi_6366_ = _e4118;
                                                                            phi_6367_ = _e4122;
                                                                            phi_6368_ = vec3<f32>(_e4124.x, _e4670, _e4124.z);
                                                                        } else {
                                                                            if (_e4120 == 5u) {
                                                                                phi_6361_ = _e4116;
                                                                                phi_6362_ = _e4118;
                                                                                phi_6363_ = _e4122;
                                                                                phi_6364_ = vec3<f32>(_e4124.x, _e4124.y, _e4670);
                                                                            } else {
                                                                                let _e4677 = (_e4120 == 6u);
                                                                                if _e4677 {
                                                                                    phi_6358_ = _e4116;
                                                                                    phi_6360_ = _e4122;
                                                                                } else {
                                                                                    if (_e4120 == 7u) {
                                                                                        phi_6356_ = _e4116;
                                                                                        phi_6357_ = vec3<f32>(_e4670, _e4122.y, _e4122.z);
                                                                                    } else {
                                                                                        if (_e4120 == 8u) {
                                                                                            phi_6354_ = _e4116;
                                                                                            phi_6355_ = vec3<f32>(_e4122.x, _e4670, _e4122.z);
                                                                                        } else {
                                                                                            let _e4680 = (_e4120 == 9u);
                                                                                            if _e4680 {
                                                                                                phi_6353_ = vec3<f32>(_e4122.x, _e4122.y, _e4670);
                                                                                            } else {
                                                                                                phi_6353_ = _e4122;
                                                                                            }
                                                                                            let _e4686 = phi_6353_;
                                                                                            phi_6354_ = select(_e4670, _e4116, _e4680);
                                                                                            phi_6355_ = _e4686;
                                                                                        }
                                                                                        let _e4693 = phi_6354_;
                                                                                        let _e4695 = phi_6355_;
                                                                                        phi_6356_ = _e4693;
                                                                                        phi_6357_ = _e4695;
                                                                                    }
                                                                                    let _e4701 = phi_6356_;
                                                                                    let _e4703 = phi_6357_;
                                                                                    phi_6358_ = _e4701;
                                                                                    phi_6360_ = _e4703;
                                                                                }
                                                                                let _e4705 = phi_6358_;
                                                                                let _e4707 = phi_6360_;
                                                                                phi_6361_ = _e4705;
                                                                                phi_6362_ = select(_e4118, _e4670, _e4677);
                                                                                phi_6363_ = _e4707;
                                                                                phi_6364_ = _e4124;
                                                                            }
                                                                            let _e4714 = phi_6361_;
                                                                            let _e4716 = phi_6362_;
                                                                            let _e4718 = phi_6363_;
                                                                            let _e4720 = phi_6364_;
                                                                            phi_6365_ = _e4714;
                                                                            phi_6366_ = _e4716;
                                                                            phi_6367_ = _e4718;
                                                                            phi_6368_ = _e4720;
                                                                        }
                                                                        let _e4726 = phi_6365_;
                                                                        let _e4728 = phi_6366_;
                                                                        let _e4730 = phi_6367_;
                                                                        let _e4732 = phi_6368_;
                                                                        phi_6369_ = _e4726;
                                                                        phi_6370_ = _e4728;
                                                                        phi_6371_ = _e4730;
                                                                        phi_6372_ = _e4732;
                                                                    }
                                                                    let _e4738 = phi_6369_;
                                                                    let _e4740 = phi_6370_;
                                                                    let _e4742 = phi_6371_;
                                                                    let _e4744 = phi_6372_;
                                                                    phi_6373_ = _e4738;
                                                                    phi_6374_ = _e4740;
                                                                    phi_6375_ = _e4742;
                                                                    phi_6376_ = _e4744;
                                                                }
                                                                let _e4746 = phi_6373_;
                                                                let _e4748 = phi_6374_;
                                                                let _e4750 = phi_6375_;
                                                                let _e4752 = phi_6376_;
                                                                phi_6378_ = _e4746;
                                                                phi_6379_ = _e4748;
                                                                phi_6380_ = _e4750;
                                                                phi_6381_ = _e4752;
                                                                phi_6382_ = select(_e4126, _e4670, _e4673);
                                                                phi_6383_ = _e4128;
                                                            }
                                                            let _e4757 = phi_6378_;
                                                            let _e4759 = phi_6379_;
                                                            let _e4761 = phi_6380_;
                                                            let _e4763 = phi_6381_;
                                                            let _e4765 = phi_6382_;
                                                            let _e4767 = phi_6383_;
                                                            phi_6384_ = _e4757;
                                                            phi_6385_ = _e4759;
                                                            phi_6386_ = _e4761;
                                                            phi_6387_ = _e4763;
                                                            phi_6388_ = _e4765;
                                                            phi_6389_ = _e4767;
                                                        }
                                                        let _e4771 = phi_6384_;
                                                        let _e4773 = phi_6385_;
                                                        let _e4775 = phi_6386_;
                                                        let _e4777 = phi_6387_;
                                                        let _e4779 = phi_6388_;
                                                        let _e4781 = phi_6389_;
                                                        phi_5648_ = _e4771;
                                                        phi_5651_ = _e4773;
                                                        phi_5653_ = (_e4120 + 1u);
                                                        phi_5655_ = _e4775;
                                                        phi_5657_ = _e4777;
                                                        phi_5659_ = _e4779;
                                                        phi_5661_ = _e4781;
                                                    } else {
                                                        phi_5648_ = f32();
                                                        phi_5651_ = f32();
                                                        phi_5653_ = u32();
                                                        phi_5655_ = vec3<f32>();
                                                        phi_5657_ = vec3<f32>();
                                                        phi_5659_ = f32();
                                                        phi_5661_ = f32();
                                                    }
                                                    let _e4784 = phi_5648_;
                                                    let _e4786 = phi_5651_;
                                                    let _e4788 = phi_5653_;
                                                    let _e4790 = phi_5655_;
                                                    let _e4792 = phi_5657_;
                                                    let _e4794 = phi_5659_;
                                                    let _e4796 = phi_5661_;
                                                    phi_5647_ = _e4784;
                                                    phi_5650_ = _e4786;
                                                    phi_5652_ = _e4788;
                                                    phi_5654_ = _e4790;
                                                    phi_5656_ = _e4792;
                                                    phi_5658_ = _e4794;
                                                    phi_5660_ = _e4796;
                                                    if !(_e4129) {
                                                        break;
                                                    }
                                                }
                                                if _e4093 {
                                                    let _e4811 = (exp((-(abs(_e3820)) / (0.06f + (abs(_e3831) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3831 * -0.71428573f) * (_e3831 * 0.71428573f))))));
                                                    let _e4813 = (_e3820 * 1.3333334f);
                                                    let _e4818 = (1.3f * exp((((_e3831 * _e3831) + (_e4813 * _e4813)) * -12.499999f)));
                                                    let _e4829 = local_26;
                                                    let _e4831 = local_27;
                                                    if (_e4829 != _e4831) {
                                                        phi_23817_ = true;
                                                    } else {
                                                        let _e4834 = local_28;
                                                        phi_23817_ = (0.1f >= _e4834);
                                                    }
                                                    let _e4837 = phi_23817_;
                                                    let _e4839 = local_29;
                                                    let _e4842 = (pow(select(_e4839, 0.1f, _e4837), 1.3f) * _e4051);
                                                    phi_6421_ = vec3<f32>((_e4048.x + (((_e4057 * _e4811) + (_e4066 * _e4818)) * _e4842)), (_e4048.y + (((_e4060 * _e4811) + (_e4069 * _e4818)) * _e4842)), (_e4048.z + (((_e4063 * _e4811) + (_e4072 * _e4818)) * _e4842)));
                                                } else {
                                                    phi_6421_ = _e4048;
                                                }
                                                let _e4854 = phi_6421_;
                                                if select(_e4103, _e4107, _e4109) {
                                                    let _e4856 = local_30;
                                                    let _e4862 = ((_e3820 - (0.025f * _e4856)) / (0.045f + (abs(_e3831) * 0.0095492955f)));
                                                    let _e4867 = local_31;
                                                    let _e4868 = (_e4867 * 3.3333333f);
                                                    let _e4870 = select(_e4868, 0f, (_e4868 < 0f));
                                                    let _e4872 = select(_e4870, 1f, (_e4870 > 1f));
                                                    let _e4888 = local_32;
                                                    let _e4890 = ((_e4888 - 0.25f) * 5.0000005f);
                                                    let _e4892 = select(_e4890, 0f, (_e4890 < 0f));
                                                    let _e4894 = select(_e4892, 1f, (_e4892 > 1f));
                                                    phi_6500_ = (_e4054 * (((exp((-(_e4862) * _e4862)) * (0.15f + (3f * ((_e4872 * _e4872) * (3f - (2f * _e4872)))))) * (0.5f + (0.9f * exp(((_e3831 * -0.6666667f) * (_e3831 * 0.6666667f)))))) + (1.2f * (exp(((_e3820 * -3.3333333f) * (_e3820 * 3.3333333f))) * ((_e4894 * _e4894) * (3f - (2f * _e4894)))))));
                                                } else {
                                                    phi_6500_ = 0f;
                                                }
                                                let _e4908 = phi_6500_;
                                                phi_6501_ = _e4854;
                                                phi_6502_ = _e4908;
                                            } else {
                                                phi_6501_ = _e4048;
                                                phi_6502_ = 0f;
                                            }
                                            let _e4910 = phi_6501_;
                                            let _e4912 = phi_6502_;
                                            phi_6504_ = _e4910;
                                            phi_6507_ = 0u;
                                            loop {
                                                let _e4914 = phi_6504_;
                                                let _e4916 = phi_6507_;
                                                local_49 = _e4914;
                                                local_50 = _e4914;
                                                local_51 = _e4914;
                                                let _e4917 = (_e4916 < bitcast<u32>(_e3783));
                                                if _e4917 {
                                                    let _e4919 = (bitcast<u32>(_e3779) + (_e4916 * 29u));
                                                    let _e4921 = global_6.member[_e4919];
                                                    let _e4922 = (_e4921 > 0.5f);
                                                    let _e4925 = global_6.member[(_e4919 + 1u)];
                                                    let _e4928 = global_6.member[(_e4919 + 2u)];
                                                    let _e4931 = global_6.member[(_e4919 + 3u)];
                                                    let _e4934 = global_6.member[(_e4919 + 4u)];
                                                    let _e4937 = global_6.member[(_e4919 + 5u)];
                                                    let _e4940 = global_6.member[(_e4919 + 6u)];
                                                    let _e4945 = (((_e313.member_1.x * _e4925) + (_e313.member_1.y * _e4928)) + (_e313.member_1.z * _e4931));
                                                    let _e4946 = (_e4945 < _e4937);
                                                    if _e4946 {
                                                        phi_7653_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7654_ = vec3<f32>();
                                                    } else {
                                                        let _e4949 = global_6.member[(_e4919 + 7u)];
                                                        let _e4952 = global_6.member[(_e4919 + 8u)];
                                                        let _e4955 = global_6.member[(_e4919 + 9u)];
                                                        let _e4958 = global_6.member[(_e4919 + 10u)];
                                                        let _e4961 = global_6.member[(_e4919 + 11u)];
                                                        let _e4964 = global_6.member[(_e4919 + 12u)];
                                                        let _e4967 = global_6.member[(_e4919 + 13u)];
                                                        let _e4970 = global_6.member[(_e4919 + 14u)];
                                                        let _e4973 = global_6.member[(_e4919 + 15u)];
                                                        let _e4976 = global_6.member[(_e4919 + 16u)];
                                                        let _e4979 = global_6.member[(_e4919 + 17u)];
                                                        let _e4982 = global_6.member[(_e4919 + 18u)];
                                                        let _e4985 = global_6.member[(_e4919 + 19u)];
                                                        let _e4988 = global_6.member[(_e4919 + 20u)];
                                                        let _e4991 = global_6.member[(_e4919 + 21u)];
                                                        let _e4994 = global_6.member[(_e4919 + 22u)];
                                                        let _e4997 = global_6.member[(_e4919 + 23u)];
                                                        let _e5000 = global_6.member[(_e4919 + 24u)];
                                                        let _e5003 = global_6.member[(_e4919 + 25u)];
                                                        let _e5006 = global_6.member[(_e4919 + 26u)];
                                                        let _e5009 = global_6.member[(_e4919 + 27u)];
                                                        let _e5012 = global_6.member[(_e4919 + 28u)];
                                                        let _e5019 = (((_e313.member_1.x / _e4945) - _e4925) / _e4934);
                                                        let _e5020 = (((_e313.member_1.y / _e4945) - _e4928) / _e4934);
                                                        let _e5021 = (((_e313.member_1.z / _e4945) - _e4931) / _e4934);
                                                        if _e4922 {
                                                            let _e5626 = (((_e5019 * _e4976) + (_e5020 * _e4979)) + (_e5021 * _e4982));
                                                            let _e5632 = ((((_e5019 * _e4985) + (_e5020 * _e4988)) + (_e5021 * _e4991)) / _e4994);
                                                            phi_7651_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4940 * exp((-2.5f * pow((sqrt(((_e5626 * _e5626) + (_e5632 * _e5632))) * 3.3333333f), 0.7f)))));
                                                            phi_7652_ = vec3<f32>();
                                                        } else {
                                                            let _e5026 = (((_e4949 * _e4925) + (_e4952 * _e4928)) + (_e4955 * _e4931));
                                                            if (abs(_e5026) < 0.12f) {
                                                                phi_6955_ = select(0.12f, -0.12f, (_e5026 < 0f));
                                                            } else {
                                                                phi_6955_ = _e5026;
                                                            }
                                                            let _e5032 = phi_6955_;
                                                            let _e5038 = ((((_e4949 * _e5019) + (_e4952 * _e5020)) + (_e4955 * _e5021)) / _e5032);
                                                            let _e5042 = (_e5019 - (_e4925 * _e5038));
                                                            let _e5043 = (_e5020 - (_e4928 * _e5038));
                                                            let _e5044 = (_e5021 - (_e4931 * _e5038));
                                                            let _e5049 = (((_e5042 * _e4958) + (_e5043 * _e4961)) + (_e5044 * _e4964));
                                                            let _e5054 = (((_e5042 * _e4967) + (_e5043 * _e4970)) + (_e5044 * _e4973));
                                                            let _e5058 = sqrt(((_e5049 * _e5049) + (_e5054 * _e5054)));
                                                            let _e5070 = abs(_e5032);
                                                            let _e5072 = (pow((0.5f + (0.5f * cos(((_e4997 * (atan2(_e5054, _e5049) - (log((_e5058 + 0.03f)) / _e5000))) + _e5003)))), 3f) * sqrt(_e5070));
                                                            let _e5077 = global_6.member[_e3760];
                                                            let _e5078 = bitcast<u32>(_e5077);
                                                            let _e5081 = global_6.member[(_e1271 + 11u)];
                                                            let _e5084 = global_6.member[(_e1271 + 12u)];
                                                            let _e5087 = global_6.member[(_e1271 + 13u)];
                                                            let _e5090 = global_6.member[(_e1271 + 14u)];
                                                            let _e5093 = global_6.member[(_e1271 + 15u)];
                                                            let _e5096 = global_6.member[(_e1271 + 16u)];
                                                            let _e5099 = global_6.member[(_e1271 + 17u)];
                                                            let _e5102 = global_6.member[(_e1271 + 18u)];
                                                            let _e5105 = global_6.member[(_e1271 + 19u)];
                                                            phi_7135_ = 0u;
                                                            phi_7138_ = 0f;
                                                            phi_7140_ = 1f;
                                                            phi_7142_ = 0f;
                                                            phi_7144_ = vec3<f32>((_e5049 * 9f), (_e5054 * 9f), 0f);
                                                            loop {
                                                                let _e5107 = phi_7135_;
                                                                let _e5109 = phi_7138_;
                                                                let _e5111 = phi_7140_;
                                                                let _e5113 = phi_7142_;
                                                                let _e5115 = phi_7144_;
                                                                local_33 = _e5109;
                                                                local_34 = _e5113;
                                                                local_35 = _e5109;
                                                                let _e5116 = (_e5107 < 4u);
                                                                if _e5116 {
                                                                    let _e5118 = (_e5115.x + _e5006);
                                                                    let _e5120 = (_e5115.y + _e5009);
                                                                    let _e5122 = (_e5115.z + _e5012);
                                                                    let _e5123 = floor(_e5118);
                                                                    let _e5124 = floor(_e5120);
                                                                    let _e5125 = floor(_e5122);
                                                                    let _e5134 = bitcast<u32>((select(0i, select(select(i32(_e5123), i32(-2147483648), (_e5123 < -2147483600f)), 2147483647i, (_e5123 > 2147483500f)), (_e5123 == _e5123)) & 255i));
                                                                    let _e5143 = bitcast<u32>((select(0i, select(select(i32(_e5124), i32(-2147483648), (_e5124 < -2147483600f)), 2147483647i, (_e5124 > 2147483500f)), (_e5124 == _e5124)) & 255i));
                                                                    let _e5152 = bitcast<u32>((select(0i, select(select(i32(_e5125), i32(-2147483648), (_e5125 < -2147483600f)), 2147483647i, (_e5125 > 2147483500f)), (_e5125 == _e5125)) & 255i));
                                                                    let _e5153 = (_e5118 - _e5123);
                                                                    let _e5154 = (_e5120 - _e5124);
                                                                    let _e5155 = (_e5122 - _e5125);
                                                                    let _e5162 = (((_e5153 * _e5153) * _e5153) * ((_e5153 * ((_e5153 * 6f) - 15f)) + 10f));
                                                                    let _e5169 = (((_e5154 * _e5154) * _e5154) * ((_e5154 * ((_e5154 * 6f) - 15f)) + 10f));
                                                                    let _e5179 = global_6.member[(_e5078 + _e5134)];
                                                                    let _e5185 = (select(select(u32(_e5179), 0u, (_e5179 < 0f)), 4294967295u, (_e5179 > 4294967000f)) + _e5143);
                                                                    let _e5188 = global_6.member[(_e5078 + _e5185)];
                                                                    let _e5194 = (select(select(u32(_e5188), 0u, (_e5188 < 0f)), 4294967295u, (_e5188 > 4294967000f)) + _e5152);
                                                                    let _e5198 = global_6.member[(_e5078 + (_e5185 + 1u))];
                                                                    let _e5204 = (select(select(u32(_e5198), 0u, (_e5198 < 0f)), 4294967295u, (_e5198 > 4294967000f)) + _e5152);
                                                                    let _e5208 = global_6.member[(_e5078 + (_e5134 + 1u))];
                                                                    let _e5214 = (select(select(u32(_e5208), 0u, (_e5208 < 0f)), 4294967295u, (_e5208 > 4294967000f)) + _e5143);
                                                                    let _e5217 = global_6.member[(_e5078 + _e5214)];
                                                                    let _e5223 = (select(select(u32(_e5217), 0u, (_e5217 < 0f)), 4294967295u, (_e5217 > 4294967000f)) + _e5152);
                                                                    let _e5227 = global_6.member[(_e5078 + (_e5214 + 1u))];
                                                                    let _e5233 = (select(select(u32(_e5227), 0u, (_e5227 < 0f)), 4294967295u, (_e5227 > 4294967000f)) + _e5152);
                                                                    let _e5236 = global_6.member[(_e5078 + _e5194)];
                                                                    let _e5242 = (select(select(u32(_e5236), 0u, (_e5236 < 0f)), 4294967295u, (_e5236 > 4294967000f)) % 12u);
                                                                    let _e5245 = select(-1f, 1f, ((_e5242 & 1u) == 0u));
                                                                    let _e5248 = select(-1f, 1f, ((_e5242 & 2u) == 0u));
                                                                    if (_e5242 < 4u) {
                                                                        phi_23912_ = ((_e5245 * _e5153) + (_e5248 * _e5154));
                                                                    } else {
                                                                        if (_e5242 < 8u) {
                                                                            phi_23906_ = ((_e5245 * _e5153) + (_e5248 * _e5155));
                                                                        } else {
                                                                            phi_23906_ = ((_e5245 * _e5154) + (_e5248 * _e5155));
                                                                        }
                                                                        let _e5258 = phi_23906_;
                                                                        phi_23912_ = _e5258;
                                                                    }
                                                                    let _e5263 = phi_23912_;
                                                                    let _e5266 = global_6.member[(_e5078 + _e5223)];
                                                                    let _e5272 = (_e5153 - 1f);
                                                                    let _e5273 = (select(select(u32(_e5266), 0u, (_e5266 < 0f)), 4294967295u, (_e5266 > 4294967000f)) % 12u);
                                                                    let _e5276 = select(-1f, 1f, ((_e5273 & 1u) == 0u));
                                                                    let _e5279 = select(-1f, 1f, ((_e5273 & 2u) == 0u));
                                                                    if (_e5273 < 4u) {
                                                                        phi_23946_ = ((_e5276 * _e5272) + (_e5279 * _e5154));
                                                                    } else {
                                                                        if (_e5273 < 8u) {
                                                                            phi_23940_ = ((_e5276 * _e5272) + (_e5279 * _e5155));
                                                                        } else {
                                                                            phi_23940_ = ((_e5276 * _e5154) + (_e5279 * _e5155));
                                                                        }
                                                                        let _e5289 = phi_23940_;
                                                                        phi_23946_ = _e5289;
                                                                    }
                                                                    let _e5294 = phi_23946_;
                                                                    let _e5297 = global_6.member[(_e5078 + _e5204)];
                                                                    let _e5303 = (_e5154 - 1f);
                                                                    let _e5304 = (select(select(u32(_e5297), 0u, (_e5297 < 0f)), 4294967295u, (_e5297 > 4294967000f)) % 12u);
                                                                    let _e5307 = select(-1f, 1f, ((_e5304 & 1u) == 0u));
                                                                    let _e5310 = select(-1f, 1f, ((_e5304 & 2u) == 0u));
                                                                    if (_e5304 < 4u) {
                                                                        phi_23980_ = ((_e5307 * _e5153) + (_e5310 * _e5303));
                                                                    } else {
                                                                        if (_e5304 < 8u) {
                                                                            phi_23974_ = ((_e5307 * _e5153) + (_e5310 * _e5155));
                                                                        } else {
                                                                            phi_23974_ = ((_e5307 * _e5303) + (_e5310 * _e5155));
                                                                        }
                                                                        let _e5320 = phi_23974_;
                                                                        phi_23980_ = _e5320;
                                                                    }
                                                                    let _e5325 = phi_23980_;
                                                                    let _e5328 = global_6.member[(_e5078 + _e5233)];
                                                                    let _e5334 = (select(select(u32(_e5328), 0u, (_e5328 < 0f)), 4294967295u, (_e5328 > 4294967000f)) % 12u);
                                                                    let _e5337 = select(-1f, 1f, ((_e5334 & 1u) == 0u));
                                                                    let _e5340 = select(-1f, 1f, ((_e5334 & 2u) == 0u));
                                                                    if (_e5334 < 4u) {
                                                                        phi_24014_ = ((_e5337 * _e5272) + (_e5340 * _e5303));
                                                                    } else {
                                                                        if (_e5334 < 8u) {
                                                                            phi_24008_ = ((_e5337 * _e5272) + (_e5340 * _e5155));
                                                                        } else {
                                                                            phi_24008_ = ((_e5337 * _e5303) + (_e5340 * _e5155));
                                                                        }
                                                                        let _e5350 = phi_24008_;
                                                                        phi_24014_ = _e5350;
                                                                    }
                                                                    let _e5355 = phi_24014_;
                                                                    let _e5359 = global_6.member[(_e5078 + (_e5194 + 1u))];
                                                                    let _e5365 = (_e5155 - 1f);
                                                                    let _e5366 = (select(select(u32(_e5359), 0u, (_e5359 < 0f)), 4294967295u, (_e5359 > 4294967000f)) % 12u);
                                                                    let _e5369 = select(-1f, 1f, ((_e5366 & 1u) == 0u));
                                                                    let _e5372 = select(-1f, 1f, ((_e5366 & 2u) == 0u));
                                                                    if (_e5366 < 4u) {
                                                                        phi_24048_ = ((_e5369 * _e5153) + (_e5372 * _e5154));
                                                                    } else {
                                                                        if (_e5366 < 8u) {
                                                                            phi_24042_ = ((_e5369 * _e5153) + (_e5372 * _e5365));
                                                                        } else {
                                                                            phi_24042_ = ((_e5369 * _e5154) + (_e5372 * _e5365));
                                                                        }
                                                                        let _e5382 = phi_24042_;
                                                                        phi_24048_ = _e5382;
                                                                    }
                                                                    let _e5387 = phi_24048_;
                                                                    let _e5391 = global_6.member[(_e5078 + (_e5223 + 1u))];
                                                                    let _e5397 = (select(select(u32(_e5391), 0u, (_e5391 < 0f)), 4294967295u, (_e5391 > 4294967000f)) % 12u);
                                                                    let _e5400 = select(-1f, 1f, ((_e5397 & 1u) == 0u));
                                                                    let _e5403 = select(-1f, 1f, ((_e5397 & 2u) == 0u));
                                                                    if (_e5397 < 4u) {
                                                                        phi_24082_ = ((_e5400 * _e5272) + (_e5403 * _e5154));
                                                                    } else {
                                                                        if (_e5397 < 8u) {
                                                                            phi_24076_ = ((_e5400 * _e5272) + (_e5403 * _e5365));
                                                                        } else {
                                                                            phi_24076_ = ((_e5400 * _e5154) + (_e5403 * _e5365));
                                                                        }
                                                                        let _e5413 = phi_24076_;
                                                                        phi_24082_ = _e5413;
                                                                    }
                                                                    let _e5418 = phi_24082_;
                                                                    let _e5422 = global_6.member[(_e5078 + (_e5204 + 1u))];
                                                                    let _e5428 = (select(select(u32(_e5422), 0u, (_e5422 < 0f)), 4294967295u, (_e5422 > 4294967000f)) % 12u);
                                                                    let _e5431 = select(-1f, 1f, ((_e5428 & 1u) == 0u));
                                                                    let _e5434 = select(-1f, 1f, ((_e5428 & 2u) == 0u));
                                                                    if (_e5428 < 4u) {
                                                                        phi_24116_ = ((_e5431 * _e5153) + (_e5434 * _e5303));
                                                                    } else {
                                                                        if (_e5428 < 8u) {
                                                                            phi_24110_ = ((_e5431 * _e5153) + (_e5434 * _e5365));
                                                                        } else {
                                                                            phi_24110_ = ((_e5431 * _e5303) + (_e5434 * _e5365));
                                                                        }
                                                                        let _e5444 = phi_24110_;
                                                                        phi_24116_ = _e5444;
                                                                    }
                                                                    let _e5449 = phi_24116_;
                                                                    let _e5453 = global_6.member[(_e5078 + (_e5233 + 1u))];
                                                                    let _e5459 = (select(select(u32(_e5453), 0u, (_e5453 < 0f)), 4294967295u, (_e5453 > 4294967000f)) % 12u);
                                                                    let _e5462 = select(-1f, 1f, ((_e5459 & 1u) == 0u));
                                                                    let _e5465 = select(-1f, 1f, ((_e5459 & 2u) == 0u));
                                                                    if (_e5459 < 4u) {
                                                                        phi_24150_ = ((_e5462 * _e5272) + (_e5465 * _e5303));
                                                                    } else {
                                                                        if (_e5459 < 8u) {
                                                                            phi_24144_ = ((_e5462 * _e5272) + (_e5465 * _e5365));
                                                                        } else {
                                                                            phi_24144_ = ((_e5462 * _e5303) + (_e5465 * _e5365));
                                                                        }
                                                                        let _e5475 = phi_24144_;
                                                                        phi_24150_ = _e5475;
                                                                    }
                                                                    let _e5480 = phi_24150_;
                                                                    let _e5483 = (_e5263 + (_e5162 * (_e5294 - _e5263)));
                                                                    let _e5489 = (_e5387 + (_e5162 * (_e5418 - _e5387)));
                                                                    let _e5495 = (_e5483 + (_e5169 * ((_e5325 + (_e5162 * (_e5355 - _e5325))) - _e5483)));
                                                                    phi_7136_ = (_e5107 + 1u);
                                                                    phi_7139_ = (_e5109 + _e5111);
                                                                    phi_7141_ = (_e5111 * 0.5f);
                                                                    phi_7143_ = (_e5113 + ((_e5495 + ((((_e5155 * _e5155) * _e5155) * ((_e5155 * ((_e5155 * 6f) - 15f)) + 10f)) * ((_e5489 + (_e5169 * ((_e5449 + (_e5162 * (_e5480 - _e5449))) - _e5489))) - _e5495))) * _e5111));
                                                                    phi_7145_ = vec3<f32>(((((_e5081 * _e5115.x) + (_e5084 * _e5115.y)) + (_e5087 * _e5115.z)) * 2.03f), ((((_e5090 * _e5115.x) + (_e5093 * _e5115.y)) + (_e5096 * _e5115.z)) * 2.03f), ((((_e5099 * _e5115.x) + (_e5102 * _e5115.y)) + (_e5105 * _e5115.z)) * 2.03f));
                                                                } else {
                                                                    phi_7136_ = u32();
                                                                    phi_7139_ = f32();
                                                                    phi_7141_ = f32();
                                                                    phi_7143_ = f32();
                                                                    phi_7145_ = vec3<f32>();
                                                                }
                                                                let _e5527 = phi_7136_;
                                                                let _e5529 = phi_7139_;
                                                                let _e5531 = phi_7141_;
                                                                let _e5533 = phi_7143_;
                                                                let _e5535 = phi_7145_;
                                                                phi_7135_ = _e5527;
                                                                phi_7138_ = _e5529;
                                                                phi_7140_ = _e5531;
                                                                phi_7142_ = _e5533;
                                                                phi_7144_ = _e5535;
                                                                if !(_e5116) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5538 = local_33;
                                                            if (_e5538 > 0f) {
                                                                let _e5541 = local_34;
                                                                let _e5543 = local_35;
                                                                phi_7558_ = (_e5541 / _e5543);
                                                            } else {
                                                                phi_7558_ = 0f;
                                                            }
                                                            let _e5546 = phi_7558_;
                                                            let _e5548 = (1f + (1.2f * _e5546));
                                                            if (_e5548 != _e5548) {
                                                                phi_24196_ = true;
                                                            } else {
                                                                phi_24196_ = (0f >= _e5548);
                                                            }
                                                            let _e5552 = phi_24196_;
                                                            let _e5557 = ((_e5058 - 1.15f) * -2.8571432f);
                                                            let _e5559 = select(_e5557, 0f, (_e5557 < 0f));
                                                            let _e5561 = select(_e5559, 1f, (_e5559 > 1f));
                                                            let _e5567 = (1f / _e5070);
                                                            if (_e5567 != _e5567) {
                                                                phi_24211_ = true;
                                                            } else {
                                                                phi_24211_ = (4f <= _e5567);
                                                            }
                                                            let _e5571 = phi_24211_;
                                                            let _e5574 = (((exp((_e5058 * -3.5714285f)) * ((_e5561 * _e5561) * (3f - (2f * _e5561)))) * select(_e5567, 4f, _e5571)) * 0.12f);
                                                            let _e5580 = sqrt((((_e5019 * _e5019) + (_e5020 * _e5020)) + (_e5021 * _e5021)));
                                                            let _e5587 = (((_e5019 * _e4985) + (_e5020 * _e4988)) + (_e5021 * _e4991));
                                                            let _e5588 = (1f - _e5070);
                                                            let _e5604 = ((_e5574 * (0.6f + (0.4f * (1f - _e5072)))) + exp((_e5580 * -16.666668f)));
                                                            let _e5610 = (((_e5574 * _e5072) * 1.8f) * select(_e5548, 0f, _e5552));
                                                            let _e5617 = (_e4940 * (1f - ((((0.75f * _e5588) * _e5588) * exp(((_e5587 * -28.571428f) * (_e5587 * 28.571428f)))) * exp((_e5580 * -1.4285715f)))));
                                                            phi_7651_ = vec3<f32>();
                                                            phi_7652_ = vec3<f32>((((1.1633643f * _e5604) + (0.86689025f * _e5610)) * _e5617), (((0.97722596f * _e5604) + (1.0155001f * _e5610)) * _e5617), (((0.74455315f * _e5604) + (1.2384146f * _e5610)) * _e5617));
                                                        }
                                                        let _e5644 = phi_7651_;
                                                        let _e5646 = phi_7652_;
                                                        phi_7653_ = _e5644;
                                                        phi_7654_ = _e5646;
                                                    }
                                                    let _e5648 = phi_7653_;
                                                    let _e5650 = phi_7654_;
                                                    phi_6505_ = (_e4914 + select(_e5650, _e5648, vec3(select(_e4922, true, _e4946))));
                                                    phi_6508_ = (_e4916 + 1u);
                                                } else {
                                                    phi_6505_ = vec3<f32>();
                                                    phi_6508_ = u32();
                                                }
                                                let _e5657 = phi_6505_;
                                                let _e5659 = phi_6508_;
                                                phi_6504_ = _e5657;
                                                phi_6507_ = _e5659;
                                                if !(_e4917) {
                                                    break;
                                                }
                                            }
                                            phi_31960_ = _e4046;
                                            if _e4046 {
                                                break;
                                            }
                                            phi_7673_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7676_ = 0u;
                                            phi_7678_ = _e4912;
                                            loop {
                                                let _e5662 = phi_7673_;
                                                let _e5664 = phi_7676_;
                                                let _e5666 = phi_7678_;
                                                local_45 = _e5666;
                                                local_46 = _e5666;
                                                local_47 = _e5666;
                                                local_48 = _e5666;
                                                local_52 = _e5662;
                                                local_53 = _e5662;
                                                local_54 = _e5662;
                                                let _e5667 = (_e5664 < bitcast<u32>(_e3775));
                                                if _e5667 {
                                                    let _e5669 = (bitcast<u32>(_e3771) + (_e5664 * 18u));
                                                    let _e5671 = global_6.member[_e5669];
                                                    let _e5676 = select(select(u32(_e5671), 0u, (_e5671 < 0f)), 4294967295u, (_e5671 > 4294967000f));
                                                    let _e5679 = global_6.member[(_e5669 + 1u)];
                                                    let _e5682 = global_6.member[(_e5669 + 2u)];
                                                    let _e5685 = global_6.member[(_e5669 + 3u)];
                                                    let _e5688 = global_6.member[(_e5669 + 4u)];
                                                    let _e5691 = global_6.member[(_e5669 + 5u)];
                                                    let _e5694 = global_6.member[(_e5669 + 6u)];
                                                    let _e5697 = global_6.member[(_e5669 + 7u)];
                                                    let _e5700 = global_6.member[(_e5669 + 8u)];
                                                    let _e5703 = global_6.member[(_e5669 + 9u)];
                                                    let _e5706 = global_6.member[(_e5669 + 10u)];
                                                    let _e5709 = global_6.member[(_e5669 + 11u)];
                                                    let _e5712 = global_6.member[(_e5669 + 12u)];
                                                    let _e5715 = global_6.member[(_e5669 + 13u)];
                                                    let _e5718 = global_6.member[(_e5669 + 14u)];
                                                    let _e5721 = global_6.member[(_e5669 + 15u)];
                                                    let _e5724 = global_6.member[(_e5669 + 16u)];
                                                    let _e5727 = global_6.member[(_e5669 + 17u)];
                                                    let _e5732 = (((_e313.member_1.x * _e5679) + (_e313.member_1.y * _e5682)) + (_e313.member_1.z * _e5685));
                                                    if (_e5732 < _e5691) {
                                                        phi_8995_ = type_55(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5737 = ((_e313.member_1.x / _e5732) - _e5679);
                                                        let _e5738 = ((_e313.member_1.y / _e5732) - _e5682);
                                                        let _e5739 = ((_e313.member_1.z / _e5732) - _e5685);
                                                        let _e5746 = ((((_e5737 * _e5700) + (_e5738 * _e5703)) + (_e5739 * _e5706)) / (_e5688 * _e5718));
                                                        let _e5752 = ((((_e5737 * _e5709) + (_e5738 * _e5712)) + (_e5739 * _e5715)) / _e5688);
                                                        let _e5756 = sqrt(((_e5746 * _e5746) + (_e5752 * _e5752)));
                                                        let _e5759 = vec3<f32>((_e5746 * 2.2f), (_e5752 * 2.2f), 0f);
                                                        let _e5760 = (_e5676 == 0u);
                                                        if _e5760 {
                                                            phi_7980_ = 6u;
                                                            phi_7981_ = 1.2f;
                                                            phi_7982_ = _e5759;
                                                        } else {
                                                            let _e5761 = (_e5676 == 1u);
                                                            if _e5761 {
                                                                phi_7979_ = vec3<f32>((_e5746 * 1.54f), (_e5752 * 1.54f), 0f);
                                                            } else {
                                                                phi_7979_ = _e5759;
                                                            }
                                                            let _e5766 = phi_7979_;
                                                            phi_7980_ = select(6u, 5u, _e5761);
                                                            phi_7981_ = select(1f, 0.8f, _e5761);
                                                            phi_7982_ = _e5766;
                                                        }
                                                        let _e5770 = phi_7980_;
                                                        let _e5772 = phi_7981_;
                                                        let _e5774 = phi_7982_;
                                                        if _e5760 {
                                                            phi_7999_ = vec3<f32>(((_e5746 * 3.7400002f) + -7f), ((_e5752 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_7999_ = vec3<f32>((_e5746 * 3.3000002f), (_e5752 * 3.3000002f), 0f);
                                                        }
                                                        let _e5784 = phi_7999_;
                                                        phi_8001_ = 0u;
                                                        phi_8004_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8006_ = 0f;
                                                        phi_8008_ = 0f;
                                                        phi_8010_ = 0f;
                                                        loop {
                                                            let _e5787 = phi_8001_;
                                                            let _e5789 = phi_8004_;
                                                            let _e5791 = phi_8006_;
                                                            let _e5793 = phi_8008_;
                                                            let _e5795 = phi_8010_;
                                                            local_39 = _e5795;
                                                            local_40 = _e5793;
                                                            local_41 = _e5791;
                                                            local_42 = _e5793;
                                                            local_43 = _e5793;
                                                            local_44 = _e5791;
                                                            let _e5796 = (_e5787 < 6u);
                                                            if _e5796 {
                                                                let _e5797 = (_e5787 == 0u);
                                                                if _e5797 {
                                                                    phi_8052_ = true;
                                                                    phi_8053_ = false;
                                                                    phi_8054_ = false;
                                                                } else {
                                                                    if (_e5787 <= 4u) {
                                                                        phi_8030_ = (_e5676 == 2u);
                                                                    } else {
                                                                        phi_8030_ = true;
                                                                    }
                                                                    let _e5801 = phi_8030_;
                                                                    if _e5801 {
                                                                        let _e5802 = (_e5787 == 5u);
                                                                        if _e5802 {
                                                                            if _e5760 {
                                                                                phi_8046_ = false;
                                                                            } else {
                                                                                phi_8046_ = (_e5676 == 2u);
                                                                            }
                                                                            let _e5805 = phi_8046_;
                                                                            phi_8048_ = _e5805;
                                                                        } else {
                                                                            phi_8048_ = false;
                                                                        }
                                                                        let _e5807 = phi_8048_;
                                                                        phi_8050_ = select(false, _e5760, _e5802);
                                                                        phi_8051_ = _e5807;
                                                                    } else {
                                                                        phi_8050_ = false;
                                                                        phi_8051_ = false;
                                                                    }
                                                                    let _e5810 = phi_8050_;
                                                                    let _e5812 = phi_8051_;
                                                                    phi_8052_ = select(true, false, _e5801);
                                                                    phi_8053_ = _e5810;
                                                                    phi_8054_ = _e5812;
                                                                }
                                                                let _e5815 = phi_8052_;
                                                                let _e5817 = phi_8053_;
                                                                let _e5819 = phi_8054_;
                                                                if select(select(_e5819, false, _e5815), true, select(_e5817, true, _e5815)) {
                                                                    if _e5797 {
                                                                        phi_8101_ = 3u;
                                                                        phi_8102_ = vec3<f32>(((_e5746 * 1.7600001f) + 3f), ((_e5752 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5787 < 4u) {
                                                                            let _e5836 = (_e5787 - 1u);
                                                                            if (_e5836 == 0u) {
                                                                                phi_24230_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_24230_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5836 == 1u)));
                                                                            }
                                                                            let _e5842 = phi_24230_;
                                                                            phi_8099_ = 4u;
                                                                            phi_8100_ = (_e5774 + _e5842);
                                                                        } else {
                                                                            let _e5824 = (_e5787 == 4u);
                                                                            if _e5824 {
                                                                                phi_8098_ = (_e5774 + vec3<f32>((_e5789.x * _e5772), (_e5789.y * _e5772), (_e5789.z * _e5772)));
                                                                            } else {
                                                                                phi_8098_ = _e5784;
                                                                            }
                                                                            let _e5834 = phi_8098_;
                                                                            phi_8099_ = select(select(4u, 5u, _e5760), _e5770, _e5824);
                                                                            phi_8100_ = _e5834;
                                                                        }
                                                                        let _e5845 = phi_8099_;
                                                                        let _e5847 = phi_8100_;
                                                                        phi_8101_ = _e5845;
                                                                        phi_8102_ = _e5847;
                                                                    }
                                                                    let _e5854 = phi_8101_;
                                                                    let _e5856 = phi_8102_;
                                                                    let _e5858 = global_6.member[_e3760];
                                                                    let _e5859 = bitcast<u32>(_e5858);
                                                                    let _e5862 = global_6.member[(_e1271 + 11u)];
                                                                    let _e5865 = global_6.member[(_e1271 + 12u)];
                                                                    let _e5868 = global_6.member[(_e1271 + 13u)];
                                                                    let _e5871 = global_6.member[(_e1271 + 14u)];
                                                                    let _e5874 = global_6.member[(_e1271 + 15u)];
                                                                    let _e5877 = global_6.member[(_e1271 + 16u)];
                                                                    let _e5880 = global_6.member[(_e1271 + 17u)];
                                                                    let _e5883 = global_6.member[(_e1271 + 18u)];
                                                                    let _e5886 = global_6.member[(_e1271 + 19u)];
                                                                    phi_8237_ = 0u;
                                                                    phi_8240_ = 0f;
                                                                    phi_8242_ = 1f;
                                                                    phi_8244_ = 0f;
                                                                    phi_8246_ = _e5856;
                                                                    loop {
                                                                        let _e5888 = phi_8237_;
                                                                        let _e5890 = phi_8240_;
                                                                        let _e5892 = phi_8242_;
                                                                        let _e5894 = phi_8244_;
                                                                        let _e5896 = phi_8246_;
                                                                        local_36 = _e5890;
                                                                        local_37 = _e5894;
                                                                        local_38 = _e5890;
                                                                        let _e5897 = (_e5888 < _e5854);
                                                                        if _e5897 {
                                                                            let _e5899 = (_e5896.x + _e5721);
                                                                            let _e5901 = (_e5896.y + _e5724);
                                                                            let _e5903 = (_e5896.z + _e5727);
                                                                            let _e5904 = floor(_e5899);
                                                                            let _e5905 = floor(_e5901);
                                                                            let _e5906 = floor(_e5903);
                                                                            let _e5915 = bitcast<u32>((select(0i, select(select(i32(_e5904), i32(-2147483648), (_e5904 < -2147483600f)), 2147483647i, (_e5904 > 2147483500f)), (_e5904 == _e5904)) & 255i));
                                                                            let _e5924 = bitcast<u32>((select(0i, select(select(i32(_e5905), i32(-2147483648), (_e5905 < -2147483600f)), 2147483647i, (_e5905 > 2147483500f)), (_e5905 == _e5905)) & 255i));
                                                                            let _e5933 = bitcast<u32>((select(0i, select(select(i32(_e5906), i32(-2147483648), (_e5906 < -2147483600f)), 2147483647i, (_e5906 > 2147483500f)), (_e5906 == _e5906)) & 255i));
                                                                            let _e5934 = (_e5899 - _e5904);
                                                                            let _e5935 = (_e5901 - _e5905);
                                                                            let _e5936 = (_e5903 - _e5906);
                                                                            let _e5943 = (((_e5934 * _e5934) * _e5934) * ((_e5934 * ((_e5934 * 6f) - 15f)) + 10f));
                                                                            let _e5950 = (((_e5935 * _e5935) * _e5935) * ((_e5935 * ((_e5935 * 6f) - 15f)) + 10f));
                                                                            let _e5960 = global_6.member[(_e5859 + _e5915)];
                                                                            let _e5966 = (select(select(u32(_e5960), 0u, (_e5960 < 0f)), 4294967295u, (_e5960 > 4294967000f)) + _e5924);
                                                                            let _e5969 = global_6.member[(_e5859 + _e5966)];
                                                                            let _e5975 = (select(select(u32(_e5969), 0u, (_e5969 < 0f)), 4294967295u, (_e5969 > 4294967000f)) + _e5933);
                                                                            let _e5979 = global_6.member[(_e5859 + (_e5966 + 1u))];
                                                                            let _e5985 = (select(select(u32(_e5979), 0u, (_e5979 < 0f)), 4294967295u, (_e5979 > 4294967000f)) + _e5933);
                                                                            let _e5989 = global_6.member[(_e5859 + (_e5915 + 1u))];
                                                                            let _e5995 = (select(select(u32(_e5989), 0u, (_e5989 < 0f)), 4294967295u, (_e5989 > 4294967000f)) + _e5924);
                                                                            let _e5998 = global_6.member[(_e5859 + _e5995)];
                                                                            let _e6004 = (select(select(u32(_e5998), 0u, (_e5998 < 0f)), 4294967295u, (_e5998 > 4294967000f)) + _e5933);
                                                                            let _e6008 = global_6.member[(_e5859 + (_e5995 + 1u))];
                                                                            let _e6014 = (select(select(u32(_e6008), 0u, (_e6008 < 0f)), 4294967295u, (_e6008 > 4294967000f)) + _e5933);
                                                                            let _e6017 = global_6.member[(_e5859 + _e5975)];
                                                                            let _e6023 = (select(select(u32(_e6017), 0u, (_e6017 < 0f)), 4294967295u, (_e6017 > 4294967000f)) % 12u);
                                                                            let _e6026 = select(-1f, 1f, ((_e6023 & 1u) == 0u));
                                                                            let _e6029 = select(-1f, 1f, ((_e6023 & 2u) == 0u));
                                                                            if (_e6023 < 4u) {
                                                                                phi_24291_ = ((_e6026 * _e5934) + (_e6029 * _e5935));
                                                                            } else {
                                                                                if (_e6023 < 8u) {
                                                                                    phi_24285_ = ((_e6026 * _e5934) + (_e6029 * _e5936));
                                                                                } else {
                                                                                    phi_24285_ = ((_e6026 * _e5935) + (_e6029 * _e5936));
                                                                                }
                                                                                let _e6039 = phi_24285_;
                                                                                phi_24291_ = _e6039;
                                                                            }
                                                                            let _e6044 = phi_24291_;
                                                                            let _e6047 = global_6.member[(_e5859 + _e6004)];
                                                                            let _e6053 = (_e5934 - 1f);
                                                                            let _e6054 = (select(select(u32(_e6047), 0u, (_e6047 < 0f)), 4294967295u, (_e6047 > 4294967000f)) % 12u);
                                                                            let _e6057 = select(-1f, 1f, ((_e6054 & 1u) == 0u));
                                                                            let _e6060 = select(-1f, 1f, ((_e6054 & 2u) == 0u));
                                                                            if (_e6054 < 4u) {
                                                                                phi_24325_ = ((_e6057 * _e6053) + (_e6060 * _e5935));
                                                                            } else {
                                                                                if (_e6054 < 8u) {
                                                                                    phi_24319_ = ((_e6057 * _e6053) + (_e6060 * _e5936));
                                                                                } else {
                                                                                    phi_24319_ = ((_e6057 * _e5935) + (_e6060 * _e5936));
                                                                                }
                                                                                let _e6070 = phi_24319_;
                                                                                phi_24325_ = _e6070;
                                                                            }
                                                                            let _e6075 = phi_24325_;
                                                                            let _e6078 = global_6.member[(_e5859 + _e5985)];
                                                                            let _e6084 = (_e5935 - 1f);
                                                                            let _e6085 = (select(select(u32(_e6078), 0u, (_e6078 < 0f)), 4294967295u, (_e6078 > 4294967000f)) % 12u);
                                                                            let _e6088 = select(-1f, 1f, ((_e6085 & 1u) == 0u));
                                                                            let _e6091 = select(-1f, 1f, ((_e6085 & 2u) == 0u));
                                                                            if (_e6085 < 4u) {
                                                                                phi_24359_ = ((_e6088 * _e5934) + (_e6091 * _e6084));
                                                                            } else {
                                                                                if (_e6085 < 8u) {
                                                                                    phi_24353_ = ((_e6088 * _e5934) + (_e6091 * _e5936));
                                                                                } else {
                                                                                    phi_24353_ = ((_e6088 * _e6084) + (_e6091 * _e5936));
                                                                                }
                                                                                let _e6101 = phi_24353_;
                                                                                phi_24359_ = _e6101;
                                                                            }
                                                                            let _e6106 = phi_24359_;
                                                                            let _e6109 = global_6.member[(_e5859 + _e6014)];
                                                                            let _e6115 = (select(select(u32(_e6109), 0u, (_e6109 < 0f)), 4294967295u, (_e6109 > 4294967000f)) % 12u);
                                                                            let _e6118 = select(-1f, 1f, ((_e6115 & 1u) == 0u));
                                                                            let _e6121 = select(-1f, 1f, ((_e6115 & 2u) == 0u));
                                                                            if (_e6115 < 4u) {
                                                                                phi_24393_ = ((_e6118 * _e6053) + (_e6121 * _e6084));
                                                                            } else {
                                                                                if (_e6115 < 8u) {
                                                                                    phi_24387_ = ((_e6118 * _e6053) + (_e6121 * _e5936));
                                                                                } else {
                                                                                    phi_24387_ = ((_e6118 * _e6084) + (_e6121 * _e5936));
                                                                                }
                                                                                let _e6131 = phi_24387_;
                                                                                phi_24393_ = _e6131;
                                                                            }
                                                                            let _e6136 = phi_24393_;
                                                                            let _e6140 = global_6.member[(_e5859 + (_e5975 + 1u))];
                                                                            let _e6146 = (_e5936 - 1f);
                                                                            let _e6147 = (select(select(u32(_e6140), 0u, (_e6140 < 0f)), 4294967295u, (_e6140 > 4294967000f)) % 12u);
                                                                            let _e6150 = select(-1f, 1f, ((_e6147 & 1u) == 0u));
                                                                            let _e6153 = select(-1f, 1f, ((_e6147 & 2u) == 0u));
                                                                            if (_e6147 < 4u) {
                                                                                phi_24427_ = ((_e6150 * _e5934) + (_e6153 * _e5935));
                                                                            } else {
                                                                                if (_e6147 < 8u) {
                                                                                    phi_24421_ = ((_e6150 * _e5934) + (_e6153 * _e6146));
                                                                                } else {
                                                                                    phi_24421_ = ((_e6150 * _e5935) + (_e6153 * _e6146));
                                                                                }
                                                                                let _e6163 = phi_24421_;
                                                                                phi_24427_ = _e6163;
                                                                            }
                                                                            let _e6168 = phi_24427_;
                                                                            let _e6172 = global_6.member[(_e5859 + (_e6004 + 1u))];
                                                                            let _e6178 = (select(select(u32(_e6172), 0u, (_e6172 < 0f)), 4294967295u, (_e6172 > 4294967000f)) % 12u);
                                                                            let _e6181 = select(-1f, 1f, ((_e6178 & 1u) == 0u));
                                                                            let _e6184 = select(-1f, 1f, ((_e6178 & 2u) == 0u));
                                                                            if (_e6178 < 4u) {
                                                                                phi_24461_ = ((_e6181 * _e6053) + (_e6184 * _e5935));
                                                                            } else {
                                                                                if (_e6178 < 8u) {
                                                                                    phi_24455_ = ((_e6181 * _e6053) + (_e6184 * _e6146));
                                                                                } else {
                                                                                    phi_24455_ = ((_e6181 * _e5935) + (_e6184 * _e6146));
                                                                                }
                                                                                let _e6194 = phi_24455_;
                                                                                phi_24461_ = _e6194;
                                                                            }
                                                                            let _e6199 = phi_24461_;
                                                                            let _e6203 = global_6.member[(_e5859 + (_e5985 + 1u))];
                                                                            let _e6209 = (select(select(u32(_e6203), 0u, (_e6203 < 0f)), 4294967295u, (_e6203 > 4294967000f)) % 12u);
                                                                            let _e6212 = select(-1f, 1f, ((_e6209 & 1u) == 0u));
                                                                            let _e6215 = select(-1f, 1f, ((_e6209 & 2u) == 0u));
                                                                            if (_e6209 < 4u) {
                                                                                phi_24495_ = ((_e6212 * _e5934) + (_e6215 * _e6084));
                                                                            } else {
                                                                                if (_e6209 < 8u) {
                                                                                    phi_24489_ = ((_e6212 * _e5934) + (_e6215 * _e6146));
                                                                                } else {
                                                                                    phi_24489_ = ((_e6212 * _e6084) + (_e6215 * _e6146));
                                                                                }
                                                                                let _e6225 = phi_24489_;
                                                                                phi_24495_ = _e6225;
                                                                            }
                                                                            let _e6230 = phi_24495_;
                                                                            let _e6234 = global_6.member[(_e5859 + (_e6014 + 1u))];
                                                                            let _e6240 = (select(select(u32(_e6234), 0u, (_e6234 < 0f)), 4294967295u, (_e6234 > 4294967000f)) % 12u);
                                                                            let _e6243 = select(-1f, 1f, ((_e6240 & 1u) == 0u));
                                                                            let _e6246 = select(-1f, 1f, ((_e6240 & 2u) == 0u));
                                                                            if (_e6240 < 4u) {
                                                                                phi_24529_ = ((_e6243 * _e6053) + (_e6246 * _e6084));
                                                                            } else {
                                                                                if (_e6240 < 8u) {
                                                                                    phi_24523_ = ((_e6243 * _e6053) + (_e6246 * _e6146));
                                                                                } else {
                                                                                    phi_24523_ = ((_e6243 * _e6084) + (_e6246 * _e6146));
                                                                                }
                                                                                let _e6256 = phi_24523_;
                                                                                phi_24529_ = _e6256;
                                                                            }
                                                                            let _e6261 = phi_24529_;
                                                                            let _e6264 = (_e6044 + (_e5943 * (_e6075 - _e6044)));
                                                                            let _e6270 = (_e6168 + (_e5943 * (_e6199 - _e6168)));
                                                                            let _e6276 = (_e6264 + (_e5950 * ((_e6106 + (_e5943 * (_e6136 - _e6106))) - _e6264)));
                                                                            phi_8238_ = (_e5888 + 1u);
                                                                            phi_8241_ = (_e5890 + _e5892);
                                                                            phi_8243_ = (_e5892 * 0.5f);
                                                                            phi_8245_ = (_e5894 + ((_e6276 + ((((_e5936 * _e5936) * _e5936) * ((_e5936 * ((_e5936 * 6f) - 15f)) + 10f)) * ((_e6270 + (_e5950 * ((_e6230 + (_e5943 * (_e6261 - _e6230))) - _e6270))) - _e6276))) * _e5892));
                                                                            phi_8247_ = vec3<f32>(((((_e5862 * _e5896.x) + (_e5865 * _e5896.y)) + (_e5868 * _e5896.z)) * 2.03f), ((((_e5871 * _e5896.x) + (_e5874 * _e5896.y)) + (_e5877 * _e5896.z)) * 2.03f), ((((_e5880 * _e5896.x) + (_e5883 * _e5896.y)) + (_e5886 * _e5896.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8238_ = u32();
                                                                            phi_8241_ = f32();
                                                                            phi_8243_ = f32();
                                                                            phi_8245_ = f32();
                                                                            phi_8247_ = vec3<f32>();
                                                                        }
                                                                        let _e6308 = phi_8238_;
                                                                        let _e6310 = phi_8241_;
                                                                        let _e6312 = phi_8243_;
                                                                        let _e6314 = phi_8245_;
                                                                        let _e6316 = phi_8247_;
                                                                        phi_8237_ = _e6308;
                                                                        phi_8240_ = _e6310;
                                                                        phi_8242_ = _e6312;
                                                                        phi_8244_ = _e6314;
                                                                        phi_8246_ = _e6316;
                                                                        if !(_e5897) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6319 = local_36;
                                                                    if (_e6319 > 0f) {
                                                                        let _e6322 = local_37;
                                                                        let _e6324 = local_38;
                                                                        phi_8660_ = (_e6322 / _e6324);
                                                                    } else {
                                                                        phi_8660_ = 0f;
                                                                    }
                                                                    let _e6327 = phi_8660_;
                                                                    if _e5797 {
                                                                        phi_8695_ = _e5789;
                                                                        phi_8696_ = _e5791;
                                                                        phi_8697_ = _e5793;
                                                                    } else {
                                                                        if (_e5787 == 1u) {
                                                                            phi_8692_ = vec3<f32>(_e6327, _e5789.y, _e5789.z);
                                                                            phi_8693_ = _e5791;
                                                                            phi_8694_ = _e5793;
                                                                        } else {
                                                                            if (_e5787 == 2u) {
                                                                                phi_8689_ = vec3<f32>(_e5789.x, _e6327, _e5789.z);
                                                                                phi_8690_ = _e5791;
                                                                                phi_8691_ = _e5793;
                                                                            } else {
                                                                                if (_e5787 == 3u) {
                                                                                    phi_8686_ = vec3<f32>(_e5789.x, _e5789.y, _e6327);
                                                                                    phi_8687_ = _e5791;
                                                                                    phi_8688_ = _e5793;
                                                                                } else {
                                                                                    let _e6331 = (_e5787 == 4u);
                                                                                    phi_8686_ = _e5789;
                                                                                    phi_8687_ = select(_e6327, _e5791, _e6331);
                                                                                    phi_8688_ = select(_e5793, _e6327, _e6331);
                                                                                }
                                                                                let _e6339 = phi_8686_;
                                                                                let _e6341 = phi_8687_;
                                                                                let _e6343 = phi_8688_;
                                                                                phi_8689_ = _e6339;
                                                                                phi_8690_ = _e6341;
                                                                                phi_8691_ = _e6343;
                                                                            }
                                                                            let _e6349 = phi_8689_;
                                                                            let _e6351 = phi_8690_;
                                                                            let _e6353 = phi_8691_;
                                                                            phi_8692_ = _e6349;
                                                                            phi_8693_ = _e6351;
                                                                            phi_8694_ = _e6353;
                                                                        }
                                                                        let _e6359 = phi_8692_;
                                                                        let _e6361 = phi_8693_;
                                                                        let _e6363 = phi_8694_;
                                                                        phi_8695_ = _e6359;
                                                                        phi_8696_ = _e6361;
                                                                        phi_8697_ = _e6363;
                                                                    }
                                                                    let _e6365 = phi_8695_;
                                                                    let _e6367 = phi_8696_;
                                                                    let _e6369 = phi_8697_;
                                                                    phi_8699_ = _e6365;
                                                                    phi_8700_ = _e6367;
                                                                    phi_8701_ = _e6369;
                                                                    phi_8702_ = select(_e5795, _e6327, _e5797);
                                                                } else {
                                                                    phi_8699_ = _e5789;
                                                                    phi_8700_ = _e5791;
                                                                    phi_8701_ = _e5793;
                                                                    phi_8702_ = _e5795;
                                                                }
                                                                let _e6372 = phi_8699_;
                                                                let _e6374 = phi_8700_;
                                                                let _e6376 = phi_8701_;
                                                                let _e6378 = phi_8702_;
                                                                phi_8002_ = (_e5787 + 1u);
                                                                phi_8005_ = _e6372;
                                                                phi_8007_ = _e6374;
                                                                phi_8009_ = _e6376;
                                                                phi_8011_ = _e6378;
                                                            } else {
                                                                phi_8002_ = u32();
                                                                phi_8005_ = vec3<f32>();
                                                                phi_8007_ = f32();
                                                                phi_8009_ = f32();
                                                                phi_8011_ = f32();
                                                            }
                                                            let _e6381 = phi_8002_;
                                                            let _e6383 = phi_8005_;
                                                            let _e6385 = phi_8007_;
                                                            let _e6387 = phi_8009_;
                                                            let _e6389 = phi_8011_;
                                                            phi_8001_ = _e6381;
                                                            phi_8004_ = _e6383;
                                                            phi_8006_ = _e6385;
                                                            phi_8008_ = _e6387;
                                                            phi_8010_ = _e6389;
                                                            if !(_e5796) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6392 = local_39;
                                                        let _e6395 = (_e5756 * (1f + (0.45f * _e6392)));
                                                        if _e5760 {
                                                            let _e6498 = local_43;
                                                            let _e6500 = ((_e6498 - -0.25f) * 1.4285715f);
                                                            let _e6502 = select(_e6500, 0f, (_e6500 < 0f));
                                                            let _e6504 = select(_e6502, 1f, (_e6502 > 1f));
                                                            let _e6508 = ((_e6504 * _e6504) * (3f - (2f * _e6504)));
                                                            let _e6510 = ((_e6395 - 1.05f) * -1.1764706f);
                                                            let _e6512 = select(_e6510, 0f, (_e6510 < 0f));
                                                            let _e6514 = select(_e6512, 1f, (_e6512 > 1f));
                                                            let _e6518 = ((_e6514 * _e6514) * (3f - (2f * _e6514)));
                                                            let _e6519 = (_e6508 * _e6518);
                                                            let _e6521 = ((_e6395 - 0.65f) * -1.5384616f);
                                                            let _e6523 = select(_e6521, 0f, (_e6521 < 0f));
                                                            let _e6525 = select(_e6523, 1f, (_e6523 > 1f));
                                                            let _e6530 = (((_e6525 * _e6525) * (3f - (2f * _e6525))) * _e6508);
                                                            let _e6532 = local_44;
                                                            let _e6534 = ((_e6532 - 0.1f) * 2.857143f);
                                                            let _e6536 = select(_e6534, 0f, (_e6534 < 0f));
                                                            let _e6538 = select(_e6536, 1f, (_e6536 > 1f));
                                                            let _e6543 = (((_e6538 * _e6538) * (3f - (2f * _e6538))) * _e6518);
                                                            let _e6548 = ((0.8f * _e6530) * _e6530);
                                                            let _e6557 = (_e5694 * exp((-2f * _e6543)));
                                                            phi_8994_ = type_55(vec3<f32>((((3.1815925f * _e6519) + (0.14634217f * _e6548)) * _e6557), (((0.38179109f * _e6519) + (1.2439084f * _e6548)) * _e6557), (((0.69995034f * _e6519) + (1.0975662f * _e6548)) * _e6557)), (_e5697 * _e6543));
                                                        } else {
                                                            if (_e5676 == 1u) {
                                                                let _e6468 = local_42;
                                                                let _e6470 = ((_e6468 - -0.3f) * 1.25f);
                                                                let _e6472 = select(_e6470, 0f, (_e6470 < 0f));
                                                                let _e6474 = select(_e6472, 1f, (_e6472 > 1f));
                                                                let _e6480 = ((_e6395 - 1f) * -1f);
                                                                let _e6482 = select(_e6480, 0f, (_e6480 < 0f));
                                                                let _e6484 = select(_e6482, 1f, (_e6482 > 1f));
                                                                let _e6490 = (((_e6474 * _e6474) * (3f - (2f * _e6474))) * pow(((_e6484 * _e6484) * (3f - (2f * _e6484))), 1.5f));
                                                                phi_8993_ = type_55((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5694 * _e6490)), (_e5697 * _e6490));
                                                            } else {
                                                                if (_e5676 == 2u) {
                                                                    let _e6424 = local_41;
                                                                    let _e6427 = (_e5756 * (1f + (0.15f * _e6424)));
                                                                    let _e6428 = (_e6427 - 0.65f);
                                                                    let _e6432 = exp(((_e6428 * -5.5555553f) * (_e6428 * 5.5555553f)));
                                                                    let _e6434 = ((_e6427 - 0.75f) * -1.8181818f);
                                                                    let _e6436 = select(_e6434, 0f, (_e6434 < 0f));
                                                                    let _e6438 = select(_e6436, 1f, (_e6436 > 1f));
                                                                    let _e6442 = ((_e6438 * _e6438) * (3f - (2f * _e6438)));
                                                                    let _e6447 = (exp(((_e5756 * -25f) * (_e5756 * 25f))) * 3f);
                                                                    phi_8992_ = type_55(vec3<f32>(((((3.1815925f * _e6432) + (_e6442 * 0.10243952f)) + _e6447) * _e5694), ((((0.38179109f * _e6432) + (_e6442 * 0.8707359f)) + _e6447) * _e5694), ((((0.69995034f * _e6432) + (_e6442 * 0.76829636f)) + _e6447) * _e5694)), 0f);
                                                                } else {
                                                                    let _e6399 = local_40;
                                                                    let _e6401 = ((_e6399 - -0.2f) * 1.6666666f);
                                                                    let _e6403 = select(_e6401, 0f, (_e6401 < 0f));
                                                                    let _e6405 = select(_e6403, 1f, (_e6403 > 1f));
                                                                    let _e6411 = ((_e6395 - 1f) * -1.4285715f);
                                                                    let _e6413 = select(_e6411, 0f, (_e6411 < 0f));
                                                                    let _e6415 = select(_e6413, 1f, (_e6413 > 1f));
                                                                    phi_8992_ = type_55(vec3<f32>(0f, 0f, 0f), (_e5697 * (((_e6405 * _e6405) * (3f - (2f * _e6405))) * ((_e6415 * _e6415) * (3f - (2f * _e6415))))));
                                                                }
                                                                let _e6466 = phi_8992_;
                                                                phi_8993_ = _e6466;
                                                            }
                                                            let _e6496 = phi_8993_;
                                                            phi_8994_ = _e6496;
                                                        }
                                                        let _e6565 = phi_8994_;
                                                        phi_8995_ = _e6565;
                                                    }
                                                    let _e6567 = phi_8995_;
                                                    phi_7674_ = (_e5662 + _e6567.member);
                                                    phi_7677_ = (_e5664 + 1u);
                                                    phi_7679_ = (_e5666 + _e6567.member_1);
                                                } else {
                                                    phi_7674_ = vec3<f32>();
                                                    phi_7677_ = u32();
                                                    phi_7679_ = f32();
                                                }
                                                let _e6574 = phi_7674_;
                                                let _e6576 = phi_7677_;
                                                let _e6578 = phi_7679_;
                                                phi_7673_ = _e6574;
                                                phi_7676_ = _e6576;
                                                phi_7678_ = _e6578;
                                                if !(_e5667) {
                                                    break;
                                                }
                                            }
                                            phi_31960_ = _e4046;
                                            if _e4046 {
                                                break;
                                            }
                                            let _e6581 = local_45;
                                            if (_e6581 <= 0f) {
                                                phi_9024_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6584 = local_46;
                                                let _e6587 = local_47;
                                                let _e6592 = local_48;
                                                phi_9024_ = vec3<f32>(exp((_e6587 * -0.81f)), exp(-(_e6584)), exp((_e6592 * -1.25f)));
                                            }
                                            let _e6597 = phi_9024_;
                                            let _e6599 = local_49;
                                            let _e6604 = local_50;
                                            let _e6609 = local_51;
                                            let _e6614 = local_52;
                                            let _e6618 = local_53;
                                            let _e6622 = local_54;
                                            let _e6627 = global_6.member[(_e1271 + 9u)];
                                            phi_31969_ = _e4046;
                                            phi_9071_ = vec3<f32>((_e3751.x + ((((_e6599.x * _e6597.x) + _e6614.x) * _e6627) * _e3753.x)), (_e3751.y + ((((_e6604.y * _e6597.y) + _e6618.y) * _e6627) * _e3753.y)), (_e3751.z + ((((_e6609.z * _e6597.z) + _e6622.z) * _e6627) * _e3753.z)));
                                        } else {
                                            phi_31969_ = _e3749;
                                            phi_9071_ = _e3751;
                                        }
                                        let _e6645 = phi_31969_;
                                        let _e6647 = phi_9071_;
                                        phi_31968_ = _e6645;
                                        phi_9072_ = _e6647;
                                    } else {
                                        phi_31968_ = _e3749;
                                        phi_9072_ = _e3757;
                                    }
                                    let _e6649 = phi_31968_;
                                    let _e6651 = phi_9072_;
                                    phi_31967_ = _e6649;
                                    phi_9073_ = _e6651;
                                } else {
                                    phi_31967_ = _e302;
                                    phi_9073_ = _e1416;
                                }
                                let _e6653 = phi_31967_;
                                let _e6655 = phi_9073_;
                                phi_31963_ = _e6653;
                                phi_10346_ = (vec3<f32>(_e313.member_2.x, _e313.member_2.y, _e313.member_2.z) * _e6655);
                            } else {
                                let _e329 = global_1.member.member_8[1u];
                                let _e339 = global_6.member[_e329];
                                let _e342 = global_6.member[(_e329 + 1u)];
                                let _e345 = global_6.member[(_e329 + 2u)];
                                let _e348 = global_6.member[(_e329 + 3u)];
                                let _e351 = global_6.member[(_e329 + 4u)];
                                let _e354 = global_6.member[(_e329 + 5u)];
                                let _e357 = global_6.member[(_e329 + 6u)];
                                let _e360 = global_6.member[(_e329 + 7u)];
                                let _e363 = global_6.member[(_e329 + 8u)];
                                let _e366 = global_6.member[(_e329 + 9u)];
                                let _e369 = global_6.member[(_e329 + 10u)];
                                let _e372 = global_6.member[(_e329 + 11u)];
                                let _e375 = global_6.member[(_e329 + 12u)];
                                let _e378 = global_6.member[(_e329 + 13u)];
                                let _e381 = global_6.member[(_e329 + 14u)];
                                let _e384 = global_6.member[(_e329 + 15u)];
                                let _e388 = global_6.member[(_e329 + 16u)];
                                let _e391 = global_6.member[(_e329 + 17u)];
                                let _e394 = global_6.member[(_e329 + 18u)];
                                let _e397 = global_6.member[(_e329 + 19u)];
                                let _e400 = global_6.member[(_e329 + 20u)];
                                let _e403 = global_6.member[(_e329 + 29u)];
                                let _e404 = bitcast<u32>(_e403);
                                let _e407 = global_6.member[(_e329 + 30u)];
                                let _e408 = bitcast<u32>(_e407);
                                let _e416 = ((((_e313.member.x - _e339) * _e313.member_1.x) + ((_e313.member.y - _e342) * _e313.member_1.y)) + ((_e313.member.z - _e345) * _e313.member_1.z));
                                let _e428 = ((((_e313.member.x * _e313.member.x) + (_e313.member.y * _e313.member.y)) + (_e313.member.z * _e313.member.z)) - (2f * (((_e313.member.x * _e339) + (_e313.member.y * _e342)) + (_e313.member.z * _e345))));
                                let _e429 = (_e428 + _e357);
                                let _e430 = (_e416 * _e416);
                                let _e431 = (_e430 - _e429);
                                if (_e431 < 0f) {
                                    phi_20434_ = type_53(0u, type_53().member_1);
                                    phi_20435_ = type_53();
                                    phi_20436_ = true;
                                } else {
                                    if (_e416 < 0f) {
                                        phi_20406_ = -(sqrt(_e431));
                                    } else {
                                        phi_20406_ = sqrt(_e431);
                                    }
                                    let _e438 = phi_20406_;
                                    let _e440 = -((_e416 + _e438));
                                    let _e441 = (_e440 == 0f);
                                    if _e441 {
                                        phi_20429_ = type_53(1u, type_52(0f, 0f));
                                        phi_20430_ = type_53();
                                    } else {
                                        let _e442 = (_e429 / _e440);
                                        if (_e440 < _e442) {
                                            phi_20418_ = type_52(_e440, _e442);
                                        } else {
                                            phi_20418_ = type_52(_e442, _e440);
                                        }
                                        let _e447 = phi_20418_;
                                        phi_20429_ = type_53();
                                        phi_20430_ = type_53(1u, _e447);
                                    }
                                    let _e450 = phi_20429_;
                                    let _e452 = phi_20430_;
                                    phi_20434_ = _e450;
                                    phi_20435_ = _e452;
                                    phi_20436_ = _e441;
                                }
                                let _e457 = phi_20434_;
                                let _e459 = phi_20435_;
                                let _e461 = phi_20436_;
                                if _e461 {
                                    phi_20440_ = _e457;
                                } else {
                                    phi_20440_ = _e459;
                                }
                                let _e463 = phi_20440_;
                                if (bitcast<i32>(_e463.member) != 0i) {
                                    phi_20327_ = type_53(0u, _e463.member_1);
                                } else {
                                    phi_20327_ = type_53(1u, type_53().member_1);
                                }
                                let _e473 = phi_20327_;
                                if (bitcast<i32>(_e473.member) != 0i) {
                                    phi_20374_ = type_53(0u, type_53().member_1);
                                } else {
                                    if (_e473.member_1.member_1 <= 0f) {
                                        phi_20370_ = type_53(0u, type_53().member_1);
                                    } else {
                                        if (_e473.member_1.member != _e473.member_1.member) {
                                            phi_20478_ = true;
                                        } else {
                                            phi_20478_ = (0f >= _e473.member_1.member);
                                        }
                                        let _e485 = phi_20478_;
                                        let _e486 = select(_e473.member_1.member, 0f, _e485);
                                        let _e487 = (_e428 + _e354);
                                        if (_e487 > 0f) {
                                            let _e489 = (_e430 - _e487);
                                            if (_e489 < 0f) {
                                                phi_20586_ = type_53(0u, type_53().member_1);
                                                phi_20587_ = type_53();
                                                phi_20588_ = true;
                                            } else {
                                                if (_e416 < 0f) {
                                                    phi_20558_ = -(sqrt(_e489));
                                                } else {
                                                    phi_20558_ = sqrt(_e489);
                                                }
                                                let _e496 = phi_20558_;
                                                let _e498 = -((_e416 + _e496));
                                                let _e499 = (_e498 == 0f);
                                                if _e499 {
                                                    phi_20581_ = type_53(1u, type_52(0f, 0f));
                                                    phi_20582_ = type_53();
                                                } else {
                                                    let _e500 = (_e487 / _e498);
                                                    if (_e498 < _e500) {
                                                        phi_20570_ = type_52(_e498, _e500);
                                                    } else {
                                                        phi_20570_ = type_52(_e500, _e498);
                                                    }
                                                    let _e505 = phi_20570_;
                                                    phi_20581_ = type_53();
                                                    phi_20582_ = type_53(1u, _e505);
                                                }
                                                let _e508 = phi_20581_;
                                                let _e510 = phi_20582_;
                                                phi_20586_ = _e508;
                                                phi_20587_ = _e510;
                                                phi_20588_ = _e499;
                                            }
                                            let _e515 = phi_20586_;
                                            let _e517 = phi_20587_;
                                            let _e519 = phi_20588_;
                                            if _e519 {
                                                phi_20592_ = _e515;
                                            } else {
                                                phi_20592_ = _e517;
                                            }
                                            let _e521 = phi_20592_;
                                            if (bitcast<i32>(_e521.member) != 0i) {
                                                phi_20526_ = select(_e521.member_1.member_1, _e521.member_1.member, (_e521.member_1.member >= 0f));
                                            } else {
                                                phi_20526_ = -1f;
                                            }
                                            let _e532 = phi_20526_;
                                            if (_e532 >= 0f) {
                                                if (_e473.member_1.member_1 != _e473.member_1.member_1) {
                                                    phi_20630_ = true;
                                                } else {
                                                    phi_20630_ = (_e532 <= _e473.member_1.member_1);
                                                }
                                                let _e537 = phi_20630_;
                                                phi_20355_ = select(_e473.member_1.member_1, _e532, _e537);
                                            } else {
                                                phi_20355_ = _e473.member_1.member_1;
                                            }
                                            let _e540 = phi_20355_;
                                            phi_20357_ = _e540;
                                        } else {
                                            phi_20357_ = _e473.member_1.member_1;
                                        }
                                        let _e542 = phi_20357_;
                                        if (_e542 <= _e486) {
                                            phi_20366_ = type_53(0u, type_53().member_1);
                                        } else {
                                            phi_20366_ = type_53(1u, type_52(_e486, _e542));
                                        }
                                        let _e550 = phi_20366_;
                                        phi_20370_ = _e550;
                                    }
                                    let _e555 = phi_20370_;
                                    phi_20374_ = _e555;
                                }
                                let _e560 = phi_20374_;
                                if (bitcast<i32>(_e560.member) != 0i) {
                                    let _e566 = (_e560.member_1.member < _e313.member.w);
                                    if _e566 {
                                        if (_e560.member_1.member_1 != _e560.member_1.member_1) {
                                            phi_20645_ = true;
                                        } else {
                                            phi_20645_ = (_e313.member.w <= _e560.member_1.member_1);
                                        }
                                        let _e572 = phi_20645_;
                                        let _e573 = select(_e560.member_1.member_1, _e313.member.w, _e572);
                                        phi_9467_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9470_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9472_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9474_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9476_ = 0u;
                                        loop {
                                            let _e575 = phi_9467_;
                                            let _e577 = phi_9470_;
                                            let _e579 = phi_9472_;
                                            let _e581 = phi_9474_;
                                            let _e583 = phi_9476_;
                                            local = _e579;
                                            local_1 = _e577;
                                            local_2 = _e575;
                                            local_3 = _e575;
                                            local_4 = _e575;
                                            local_5 = _e581;
                                            local_6 = _e581;
                                            local_7 = _e581;
                                            let _e584 = (_e583 < 32u);
                                            if _e584 {
                                                let _e585 = -(_e416);
                                                if (_e560.member_1.member <= _e573) {
                                                } else {
                                                    phi_31482_ = true;
                                                    break;
                                                }
                                                let _e588 = select(_e585, _e560.member_1.member, (_e585 < _e560.member_1.member));
                                                let _e590 = select(_e588, _e573, (_e588 > _e573));
                                                let _e591 = (_e590 - _e560.member_1.member);
                                                let _e592 = (_e573 - _e590);
                                                let _e593 = (_e573 - _e560.member_1.member);
                                                let _e594 = (0.000000001f * _e593);
                                                if (_e591 <= _e594) {
                                                    let _e669 = (f32(_e583) * 0.03125f);
                                                    let _e672 = (f32((_e583 + 1u)) * 0.03125f);
                                                    let _e674 = ((_e669 + _e672) * 0.5f);
                                                    phi_9577_ = type_52((_e560.member_1.member + ((_e593 * _e674) * _e674)), (_e593 * ((_e672 * _e672) - (_e669 * _e669))));
                                                } else {
                                                    if (_e592 <= _e594) {
                                                        let _e652 = (f32((31u - _e583)) * 0.03125f);
                                                        let _e655 = (f32((32u - _e583)) * 0.03125f);
                                                        let _e657 = ((_e652 + _e655) * 0.5f);
                                                        phi_9576_ = type_52((_e573 - ((_e593 * _e657) * _e657)), (_e593 * ((_e655 * _e655) - (_e652 * _e652))));
                                                    } else {
                                                        let _e600 = round(((32f * _e591) / (_e591 + _e592)));
                                                        let _e605 = select(select(u32(_e600), 0u, (_e600 < 0f)), 4294967295u, (_e600 > 4294967000f));
                                                        if (_e605 < 1u) {
                                                            phi_9560_ = 1u;
                                                        } else {
                                                            phi_9560_ = select(_e605, 31u, (_e605 > 31u));
                                                        }
                                                        let _e610 = phi_9560_;
                                                        if (_e583 < _e610) {
                                                            let _e631 = ((_e610 - 1u) - _e583);
                                                            let _e633 = f32(_e610);
                                                            let _e634 = (f32(_e631) / _e633);
                                                            let _e637 = (f32((_e631 + 1u)) / _e633);
                                                            let _e639 = ((_e634 + _e637) * 0.5f);
                                                            phi_9575_ = type_52((_e590 - ((_e591 * _e639) * _e639)), (_e591 * ((_e637 * _e637) - (_e634 * _e634))));
                                                        } else {
                                                            let _e613 = (_e583 - _e610);
                                                            let _e615 = f32((32u - _e610));
                                                            let _e616 = (f32(_e613) / _e615);
                                                            let _e619 = (f32((_e613 + 1u)) / _e615);
                                                            let _e621 = ((_e616 + _e619) * 0.5f);
                                                            phi_9575_ = type_52((_e590 + ((_e592 * _e621) * _e621)), (_e592 * ((_e619 * _e619) - (_e616 * _e616))));
                                                        }
                                                        let _e649 = phi_9575_;
                                                        phi_9576_ = _e649;
                                                    }
                                                    let _e667 = phi_9576_;
                                                    phi_9577_ = _e667;
                                                }
                                                let _e684 = phi_9577_;
                                                let _e690 = (_e313.member.x + (_e313.member_1.x * _e684.member));
                                                let _e691 = (_e313.member.y + (_e313.member_1.y * _e684.member));
                                                let _e692 = (_e313.member.z + (_e313.member_1.z * _e684.member));
                                                let _e705 = ((((_e690 * _e690) + (_e691 * _e691)) + (_e692 * _e692)) - (2f * (((_e690 * _e339) + (_e691 * _e342)) + (_e692 * _e345))));
                                                let _e707 = (_e690 - _e339);
                                                let _e708 = (_e691 - _e342);
                                                let _e709 = (_e692 - _e345);
                                                let _e715 = sqrt((((_e707 * _e707) + (_e708 * _e708)) + (_e709 * _e709)));
                                                let _e717 = ((_e705 + _e354) / (_e715 + _e348));
                                                let _e721 = (1f - (abs((_e717 - 25000f)) * 0.00006666667f));
                                                if (_e721 != _e721) {
                                                    phi_20845_ = true;
                                                } else {
                                                    phi_20845_ = (0f >= _e721);
                                                }
                                                let _e725 = phi_20845_;
                                                let _e727 = -(_e717);
                                                let _e730 = (exp((_e727 / _e369)) * _e684.member_1);
                                                let _e733 = (exp((_e727 / _e372)) * _e684.member_1);
                                                let _e735 = (_e730 * 0.5f);
                                                let _e736 = (_e733 * 0.5f);
                                                let _e737 = ((select(_e721, 0f, _e725) * _e684.member_1) * 0.5f);
                                                let _e739 = (_e581.x + _e735);
                                                let _e741 = (_e581.y + _e736);
                                                let _e743 = (_e581.z + _e737);
                                                let _e748 = (_e388 * 1.1f);
                                                let _e749 = (_e748 * _e741);
                                                let _e768 = (_e388 * _e733);
                                                if (_e408 == 4294967295u) {
                                                    phi_9905_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e776 = (1f / _e715);
                                                    let _e784 = ((((_e707 * _e776) * _e360) + ((_e708 * _e776) * _e363)) + ((_e709 * _e776) * _e366));
                                                    if (_e717 != _e717) {
                                                        phi_20972_ = true;
                                                    } else {
                                                        phi_20972_ = (0f >= _e717);
                                                    }
                                                    let _e788 = phi_20972_;
                                                    let _e792 = sqrt((select(_e717, 0f, _e788) / (_e351 - _e348)));
                                                    if (_e792 != _e792) {
                                                        phi_20987_ = true;
                                                    } else {
                                                        phi_20987_ = (1f <= _e792);
                                                    }
                                                    let _e796 = phi_20987_;
                                                    let _e798 = (select(_e792, 1f, _e796) * 31f);
                                                    let _e800 = select(_e784, -1f, (_e784 < -1f));
                                                    let _e804 = ((select(_e800, 1f, (_e800 > 1f)) + 1f) * 15.5f);
                                                    let _e809 = select(select(u32(_e798), 0u, (_e798 < 0f)), 4294967295u, (_e798 > 4294967000f));
                                                    let _e811 = select(_e809, 30u, (30u < _e809));
                                                    let _e816 = select(select(u32(_e804), 0u, (_e804 < 0f)), 4294967295u, (_e804 > 4294967000f));
                                                    let _e818 = select(_e816, 30u, (30u < _e816));
                                                    let _e820 = (_e798 - f32(_e811));
                                                    let _e822 = (_e804 - f32(_e818));
                                                    let _e826 = (_e408 + (3u * ((_e811 * 32u) + _e818)));
                                                    let _e831 = (_e408 + (3u * (((_e811 + 1u) * 32u) + _e818)));
                                                    let _e833 = global_6.member[_e826];
                                                    let _e836 = global_6.member[(_e826 + 1u)];
                                                    let _e839 = global_6.member[(_e826 + 2u)];
                                                    let _e840 = (1f - _e822);
                                                    let _e846 = global_6.member[(_e826 + 3u)];
                                                    let _e849 = global_6.member[(_e826 + 4u)];
                                                    let _e852 = global_6.member[(_e826 + 5u)];
                                                    let _e859 = (1f - _e820);
                                                    let _e864 = global_6.member[_e831];
                                                    let _e867 = global_6.member[(_e831 + 1u)];
                                                    let _e870 = global_6.member[(_e831 + 2u)];
                                                    let _e876 = global_6.member[(_e831 + 3u)];
                                                    let _e879 = global_6.member[(_e831 + 4u)];
                                                    let _e882 = global_6.member[(_e831 + 5u)];
                                                    phi_9905_ = vec3<f32>(((((_e833 * _e840) + (_e846 * _e822)) * _e859) + (((_e864 * _e840) + (_e876 * _e822)) * _e820)), ((((_e836 * _e840) + (_e849 * _e822)) * _e859) + (((_e867 * _e840) + (_e879 * _e822)) * _e820)), ((((_e839 * _e840) + (_e852 * _e822)) * _e859) + (((_e870 * _e840) + (_e882 * _e822)) * _e820)));
                                                }
                                                let _e897 = phi_9905_;
                                                let _e911 = (_e705 + _e357);
                                                if (_e911 > 0f) {
                                                    let _e917 = (((_e707 * _e360) + (_e708 * _e363)) + (_e709 * _e366));
                                                    let _e919 = ((_e917 * _e917) - _e911);
                                                    if (_e917 >= 0f) {
                                                        phi_9979_ = vec3<f32>();
                                                        phi_9980_ = true;
                                                    } else {
                                                        let _e921 = (_e919 <= 0f);
                                                        if _e921 {
                                                            phi_9978_ = vec3<f32>();
                                                        } else {
                                                            let _e924 = (-(_e917) - sqrt(_e919));
                                                            phi_9978_ = vec3<f32>((_e690 + (_e360 * _e924)), (_e691 + (_e363 * _e924)), (_e692 + (_e366 * _e924)));
                                                        }
                                                        let _e933 = phi_9978_;
                                                        phi_9979_ = _e933;
                                                        phi_9980_ = _e921;
                                                    }
                                                    let _e935 = phi_9979_;
                                                    let _e937 = phi_9980_;
                                                    phi_9985_ = _e935;
                                                    phi_9986_ = select(true, false, _e937);
                                                } else {
                                                    phi_9985_ = vec3<f32>(_e690, _e691, _e692);
                                                    phi_9986_ = true;
                                                }
                                                let _e940 = phi_9985_;
                                                let _e942 = phi_9986_;
                                                if _e942 {
                                                    let _e944 = (_e940.x - _e339);
                                                    let _e946 = (_e940.y - _e342);
                                                    let _e948 = (_e940.z - _e345);
                                                    let _e954 = sqrt((((_e944 * _e944) + (_e946 * _e946)) + (_e948 * _e948)));
                                                    let _e955 = (1f / _e954);
                                                    let _e973 = ((((((_e940.x * _e940.x) + (_e940.y * _e940.y)) + (_e940.z * _e940.z)) - (2f * (((_e940.x * _e339) + (_e940.y * _e342)) + (_e940.z * _e345)))) + _e354) / (_e954 + _e348));
                                                    if (_e973 != _e973) {
                                                        phi_21111_ = true;
                                                    } else {
                                                        phi_21111_ = (0f >= _e973);
                                                    }
                                                    let _e977 = phi_21111_;
                                                    let _e981 = sqrt((select(_e973, 0f, _e977) / (_e351 - _e348)));
                                                    if (_e981 != _e981) {
                                                        phi_21126_ = true;
                                                    } else {
                                                        phi_21126_ = (1f <= _e981);
                                                    }
                                                    let _e985 = phi_21126_;
                                                    let _e987 = (select(_e981, 1f, _e985) * 63f);
                                                    let _e992 = ((((_e944 * _e955) * _e360) + ((_e946 * _e955) * _e363)) + ((_e948 * _e955) * _e366));
                                                    let _e994 = select(_e992, -1f, (_e992 < -1f));
                                                    let _e998 = ((select(_e994, 1f, (_e994 > 1f)) + 1f) * 127.5f);
                                                    let _e1003 = select(select(u32(_e987), 0u, (_e987 < 0f)), 4294967295u, (_e987 > 4294967000f));
                                                    let _e1005 = select(_e1003, 62u, (62u < _e1003));
                                                    let _e1010 = select(select(u32(_e998), 0u, (_e998 < 0f)), 4294967295u, (_e998 > 4294967000f));
                                                    let _e1012 = select(_e1010, 254u, (254u < _e1010));
                                                    let _e1014 = (_e987 - f32(_e1005));
                                                    let _e1016 = (_e998 - f32(_e1012));
                                                    let _e1020 = (_e404 + (3u * ((_e1005 * 256u) + _e1012)));
                                                    let _e1025 = (_e404 + (3u * (((_e1005 + 1u) * 256u) + _e1012)));
                                                    let _e1027 = global_6.member[_e1020];
                                                    let _e1030 = global_6.member[(_e1020 + 1u)];
                                                    let _e1033 = global_6.member[(_e1020 + 2u)];
                                                    let _e1034 = (1f - _e1016);
                                                    let _e1040 = global_6.member[(_e1020 + 3u)];
                                                    let _e1043 = global_6.member[(_e1020 + 4u)];
                                                    let _e1046 = global_6.member[(_e1020 + 5u)];
                                                    let _e1053 = (1f - _e1014);
                                                    let _e1058 = global_6.member[_e1025];
                                                    let _e1061 = global_6.member[(_e1025 + 1u)];
                                                    let _e1064 = global_6.member[(_e1025 + 2u)];
                                                    let _e1070 = global_6.member[(_e1025 + 3u)];
                                                    let _e1073 = global_6.member[(_e1025 + 4u)];
                                                    let _e1076 = global_6.member[(_e1025 + 5u)];
                                                    phi_10277_ = vec3<f32>(((((_e1027 * _e1034) + (_e1040 * _e1016)) * _e1053) + (((_e1058 * _e1034) + (_e1070 * _e1016)) * _e1014)), ((((_e1030 * _e1034) + (_e1043 * _e1016)) * _e1053) + (((_e1061 * _e1034) + (_e1073 * _e1016)) * _e1014)), ((((_e1033 * _e1034) + (_e1046 * _e1016)) * _e1053) + (((_e1064 * _e1034) + (_e1076 * _e1016)) * _e1014)));
                                                } else {
                                                    phi_10277_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1091 = phi_10277_;
                                                let _e1092 = (vec3<f32>(_e739, _e741, _e743) + _e1091);
                                                let _e1098 = (_e748 * _e1092.y);
                                                let _e1110 = exp(-((((_e378 * _e1092.x) + _e1098) + (_e394 * _e1092.z))));
                                                let _e1112 = exp(-((((_e381 * _e1092.x) + _e1098) + (_e397 * _e1092.z))));
                                                let _e1114 = exp(-((((_e384 * _e1092.x) + _e1098) + (_e400 * _e1092.z))));
                                                phi_9468_ = vec3<f32>((_e575.x + ((exp(-((((_e378 * _e739) + _e749) + (_e394 * _e743)))) * ((_e378 * _e730) + _e768)) * _e897.x)), (_e575.y + ((exp(-((((_e381 * _e739) + _e749) + (_e397 * _e743)))) * ((_e381 * _e730) + _e768)) * _e897.y)), (_e575.z + ((exp(-((((_e384 * _e739) + _e749) + (_e400 * _e743)))) * ((_e384 * _e730) + _e768)) * _e897.z)));
                                                phi_9471_ = vec3<f32>((_e577.x + (_e1110 * _e733)), (_e577.y + (_e1112 * _e733)), (_e577.z + (_e1114 * _e733)));
                                                phi_9473_ = vec3<f32>((_e579.x + (_e1110 * _e730)), (_e579.y + (_e1112 * _e730)), (_e579.z + (_e1114 * _e730)));
                                                phi_9475_ = vec3<f32>((_e739 + _e735), (_e741 + _e736), (_e743 + _e737));
                                                phi_9477_ = (_e583 + 1u);
                                            } else {
                                                phi_9468_ = vec3<f32>();
                                                phi_9471_ = vec3<f32>();
                                                phi_9473_ = vec3<f32>();
                                                phi_9475_ = vec3<f32>();
                                                phi_9477_ = u32();
                                            }
                                            let _e1141 = phi_9468_;
                                            let _e1143 = phi_9471_;
                                            let _e1145 = phi_9473_;
                                            let _e1147 = phi_9475_;
                                            let _e1149 = phi_9477_;
                                            phi_9467_ = _e1141;
                                            phi_9470_ = _e1143;
                                            phi_9472_ = _e1145;
                                            phi_9474_ = _e1147;
                                            phi_9476_ = _e1149;
                                            phi_31482_ = _e302;
                                            if !(_e584) {
                                                break;
                                            }
                                        }
                                        let _e1152 = phi_31482_;
                                        phi_31960_ = _e1152;
                                        if _e1152 {
                                            break;
                                        }
                                        let _e1154 = local;
                                        let _e1155 = (_e1154 * vec3<f32>(_e378, _e381, _e384));
                                        let _e1157 = local_1;
                                        let _e1158 = (_e1157 * _e388);
                                        let _e1163 = (((_e313.member_1.x * _e360) + (_e313.member_1.y * _e363)) + (_e313.member_1.z * _e366));
                                        let _e1165 = (1f + (_e1163 * _e1163));
                                        let _e1166 = (0.059683103f * _e1165);
                                        let _e1173 = (_e391 * _e391);
                                        let _e1184 = ((0.119366206f * ((1f - _e1173) * _e1165)) / ((2f + _e1173) * pow(((1f + _e1173) - ((2f * _e391) * _e1163)), 1.5f)));
                                        let _e1195 = local_2;
                                        let _e1199 = local_3;
                                        let _e1203 = local_4;
                                        let _e1212 = local_5;
                                        let _e1219 = local_6;
                                        let _e1221 = ((_e388 * 1.1f) * _e1219.y);
                                        let _e1226 = local_7;
                                        phi_31966_ = _e1152;
                                        phi_10334_ = type_56(vec3<f32>(exp(-((((_e378 * _e1212.x) + _e1221) + (_e394 * _e1226.z)))), exp(-((((_e381 * _e1212.x) + _e1221) + (_e397 * _e1226.z)))), exp(-((((_e384 * _e1212.x) + _e1221) + (_e400 * _e1226.z))))), (vec3<f32>(((((_e1155.x * _e1166) + (_e1158.x * _e1184)) + _e1195.x) * _e375), ((((_e1155.y * _e1166) + (_e1158.y * _e1184)) + _e1199.y) * _e375), ((((_e1155.z * _e1166) + (_e1158.z * _e1184)) + _e1203.z) * _e375)) * 0.38317f));
                                    } else {
                                        phi_31966_ = _e302;
                                        phi_10334_ = type_56();
                                    }
                                    let _e1243 = phi_31966_;
                                    let _e1245 = phi_10334_;
                                    phi_31965_ = _e1243;
                                    phi_10336_ = _e1245;
                                    phi_10337_ = select(true, false, _e566);
                                } else {
                                    phi_31965_ = _e302;
                                    phi_10336_ = type_56();
                                    phi_10337_ = true;
                                }
                                let _e1248 = phi_31965_;
                                let _e1250 = phi_10336_;
                                let _e1252 = phi_10337_;
                                if _e1252 {
                                    phi_10342_ = type_56(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10342_ = _e1250;
                                }
                                let _e1254 = phi_10342_;
                                phi_31963_ = _e1248;
                                phi_10346_ = (vec3<f32>(_e313.member_2.x, _e313.member_2.y, _e313.member_2.z) * _e1254.member_1);
                            }
                            let _e6659 = phi_31963_;
                            let _e6661 = phi_10346_;
                            phi_31962_ = _e6659;
                            phi_10347_ = _e6661;
                        }
                        let _e6663 = phi_31962_;
                        let _e6665 = phi_10347_;
                        phi_31961_ = _e6663;
                        phi_465_ = (_e304 + _e6665);
                        phi_468_ = (_e306 + 1u);
                    } else {
                        phi_31961_ = _e302;
                        phi_465_ = vec3<f32>();
                        phi_468_ = u32();
                    }
                    let _e6669 = phi_31961_;
                    let _e6671 = phi_465_;
                    let _e6673 = phi_468_;
                    phi_31499_ = _e6669;
                    phi_464_ = _e6671;
                    phi_467_ = _e6673;
                    phi_31960_ = _e6669;
                    if !(_e307) {
                        break;
                    }
                }
                let _e6676 = phi_31960_;
                if _e6676 {
                    break;
                }
                let _e6678 = local_55;
                let _e6682 = local_56;
                let _e6687 = local_57;
                if ((((_e6678.x * _e6678.x) + (_e6682.y * _e6682.y)) + (_e6687.z * _e6687.z)) > 0f) {
                    if (_e296.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6697 = global_7.member[_e296.member_1].member_3[0u];
                    global_7.member[_e296.member_1].member_3[0u] = (_e6697 + _e6678.x);
                    let _e6703 = global_7.member[_e296.member_1].member_3[1u];
                    global_7.member[_e296.member_1].member_3[1u] = (_e6703 + _e6682.y);
                    let _e6709 = global_7.member[_e296.member_1].member_3[2u];
                    global_7.member[_e296.member_1].member_3[2u] = (_e6709 + _e6687.z);
                    let _e6715 = global_7.member[_e296.member_1].member_3[3u];
                    global_7.member[_e296.member_1].member_3[3u] = _e6715;
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
    var phi_11119_: type_45;
    var phi_11308_: u32;
    var phi_11311_: u32;
    var phi_11313_: u32;
    var phi_11315_: vec4<f32>;
    var phi_11317_: type_63;
    var phi_24706_: bool;
    var phi_24736_: bool;
    var phi_24683_: type_52;
    var phi_24686_: type_52;
    var phi_24687_: bool;
    var phi_24692_: type_52;
    var phi_24793_: bool;
    var phi_24808_: bool;
    var phi_24823_: bool;
    var phi_24838_: bool;
    var phi_24770_: type_52;
    var phi_24773_: type_52;
    var phi_24774_: bool;
    var phi_24779_: type_52;
    var phi_24880_: bool;
    var phi_24895_: bool;
    var phi_24910_: bool;
    var phi_24925_: bool;
    var phi_24857_: type_52;
    var phi_24860_: type_52;
    var phi_24861_: bool;
    var phi_24866_: type_52;
    var phi_24659_: bool;
    var phi_24662_: bool;
    var phi_24663_: bool;
    var phi_11707_: u32;
    var phi_11708_: u32;
    var phi_11709_: u32;
    var phi_11710_: u32;
    var phi_11711_: u32;
    var phi_11712_: u32;
    var phi_11713_: u32;
    var phi_11714_: u32;
    var phi_11715_: u32;
    var phi_11596_: vec4<f32>;
    var phi_11597_: vec4<f32>;
    var phi_11598_: bool;
    var phi_11602_: vec4<f32>;
    var phi_11603_: bool;
    var phi_11513_: bool;
    var phi_11547_: bool;
    var phi_11553_: f32;
    var phi_11563_: vec4<f32>;
    var phi_11564_: vec4<f32>;
    var phi_11565_: bool;
    var phi_11569_: vec4<f32>;
    var phi_11570_: bool;
    var phi_11574_: vec4<f32>;
    var phi_11575_: bool;
    var phi_11604_: vec4<f32>;
    var phi_11605_: bool;
    var phi_11445_: vec4<f32>;
    var phi_11446_: vec4<f32>;
    var phi_11447_: bool;
    var phi_11451_: vec4<f32>;
    var phi_11452_: bool;
    var phi_11453_: vec4<f32>;
    var phi_11454_: bool;
    var phi_11455_: bool;
    var phi_11463_: vec4<f32>;
    var phi_11464_: bool;
    var phi_11465_: vec4<f32>;
    var phi_11466_: bool;
    var phi_11467_: bool;
    var phi_11475_: vec4<f32>;
    var phi_11476_: bool;
    var phi_11614_: vec4<f32>;
    var phi_11615_: bool;
    var phi_11636_: type_63;
    var phi_11638_: type_63;
    var phi_11309_: u32;
    var phi_11312_: u32;
    var phi_11314_: u32;
    var phi_11316_: vec4<f32>;
    var phi_11318_: type_63;
    var phi_11717_: type_63;
    var phi_11718_: bool;
    var phi_11719_: bool;
    var phi_31971_: bool;
    var phi_20304_: bool;
    var phi_20303_: type_63;
    var local_69: type_63;
    var local_70: vec4<f32>;
    var phi_11880_: type_56;
    var local_71: vec4<f32>;
    var local_72: vec4<f32>;
    var phi_11846_: vec3<f32>;
    var phi_11847_: vec3<f32>;
    var phi_11881_: type_56;
    var local_73: type_63;
    var phi_11886_: type_63;
    var phi_11887_: type_63;
    var phi_11892_: type_63;
    var phi_25044_: f32;
    var phi_25056_: type_52;
    var phi_25067_: type_53;
    var phi_25068_: type_53;
    var phi_25072_: type_53;
    var phi_25073_: type_53;
    var phi_25074_: bool;
    var phi_25078_: type_53;
    var phi_24965_: type_53;
    var phi_25116_: bool;
    var phi_25196_: f32;
    var phi_25208_: type_52;
    var phi_25219_: type_53;
    var phi_25220_: type_53;
    var phi_25224_: type_53;
    var phi_25225_: type_53;
    var phi_25226_: bool;
    var phi_25230_: type_53;
    var phi_25164_: f32;
    var phi_25268_: bool;
    var phi_24993_: f32;
    var phi_24995_: f32;
    var phi_25004_: type_53;
    var phi_25008_: type_53;
    var phi_25012_: type_53;
    var phi_25283_: bool;
    var phi_13311_: vec3<f32>;
    var phi_13314_: u32;
    var phi_13398_: u32;
    var phi_13413_: type_52;
    var phi_13414_: type_52;
    var phi_13415_: type_52;
    var phi_25482_: bool;
    var phi_13312_: vec3<f32>;
    var phi_13315_: u32;
    var phi_32044_: bool;
    var local_74: vec3<f32>;
    var local_75: vec3<f32>;
    var local_76: vec3<f32>;
    var phi_34271_: bool;
    var phi_13440_: vec3<f32>;
    var phi_34270_: bool;
    var phi_13442_: vec3<f32>;
    var phi_13443_: bool;
    var phi_34268_: bool;
    var phi_13450_: vec3<f32>;
    var phi_13451_: type_39;
    var phi_34267_: bool;
    var phi_13452_: vec3<f32>;
    var phi_13453_: type_39;
    var phi_13477_: i32;
    var phi_13480_: u32;
    var phi_13534_: bool;
    var phi_13540_: bool;
    var phi_13547_: u32;
    var phi_13549_: i32;
    var phi_13550_: u32;
    var phi_13478_: i32;
    var phi_13481_: u32;
    var phi_13552_: bool;
    var local_77: u32;
    var phi_13833_: vec3<f32>;
    var phi_13850_: vec3<f32>;
    var phi_25549_: bool;
    var phi_13883_: f32;
    var phi_13992_: vec3<f32>;
    var phi_13998_: f32;
    var phi_25622_: bool;
    var phi_25611_: f32;
    var phi_14079_: vec3<f32>;
    var phi_14080_: vec3<f32>;
    var phi_14081_: vec3<f32>;
    var phi_32260_: u32;
    var phi_14083_: vec3<f32>;
    var phi_14084_: vec3<f32>;
    var phi_14085_: vec3<f32>;
    var phi_14086_: bool;
    var phi_14101_: vec3<f32>;
    var phi_14102_: vec3<f32>;
    var phi_32258_: u32;
    var phi_14104_: vec3<f32>;
    var phi_14105_: vec3<f32>;
    var phi_14106_: vec3<f32>;
    var phi_32257_: u32;
    var phi_14107_: vec3<f32>;
    var phi_14108_: vec3<f32>;
    var phi_14109_: vec3<f32>;
    var phi_14110_: bool;
    var phi_32256_: u32;
    var phi_14111_: vec3<f32>;
    var phi_14112_: vec3<f32>;
    var phi_14113_: vec3<f32>;
    var phi_14114_: bool;
    var phi_14115_: bool;
    var phi_25695_: bool;
    var phi_25857_: bool;
    var phi_25872_: bool;
    var phi_25887_: bool;
    var phi_25914_: bool;
    var phi_25929_: bool;
    var phi_25944_: bool;
    var phi_25971_: bool;
    var phi_25986_: bool;
    var phi_26001_: bool;
    var phi_14294_: f32;
    var phi_14302_: f32;
    var phi_14310_: f32;
    var phi_26198_: bool;
    var phi_26187_: f32;
    var phi_14417_: vec3<f32>;
    var phi_26226_: bool;
    var phi_14418_: vec3<f32>;
    var phi_14419_: vec3<f32>;
    var phi_14420_: vec3<f32>;
    var phi_32644_: vec3<f32>;
    var phi_32567_: vec3<f32>;
    var phi_32488_: u32;
    var phi_32406_: f32;
    var phi_14422_: vec3<f32>;
    var phi_14423_: vec3<f32>;
    var phi_14424_: bool;
    var phi_26241_: bool;
    var phi_26292_: bool;
    var phi_26281_: f32;
    var phi_32615_: vec3<f32>;
    var phi_13956_: vec3<f32>;
    var phi_32967_: vec3<f32>;
    var phi_32761_: f32;
    var phi_32614_: vec3<f32>;
    var phi_32302_: u32;
    var phi_13958_: vec3<f32>;
    var phi_13959_: bool;
    var phi_32913_: vec3<f32>;
    var phi_32721_: f32;
    var phi_32642_: vec3<f32>;
    var phi_32565_: vec3<f32>;
    var phi_32486_: u32;
    var phi_32404_: f32;
    var phi_32327_: u32;
    var phi_32253_: u32;
    var phi_14432_: vec3<f32>;
    var phi_14433_: vec3<f32>;
    var phi_14434_: vec3<f32>;
    var phi_14435_: bool;
    var phi_26358_: bool;
    var phi_14504_: vec3<f32>;
    var phi_32310_: u32;
    var phi_14506_: vec3<f32>;
    var phi_14507_: bool;
    var phi_32309_: u32;
    var phi_14508_: vec3<f32>;
    var phi_14509_: bool;
    var phi_32307_: u32;
    var phi_14515_: vec3<f32>;
    var phi_14516_: bool;
    var phi_14549_: type_68;
    var phi_32312_: u32;
    var phi_14551_: vec3<f32>;
    var phi_14552_: type_68;
    var phi_14553_: bool;
    var phi_14559_: type_68;
    var phi_32316_: u32;
    var phi_14560_: type_68;
    var phi_32910_: vec3<f32>;
    var phi_32718_: f32;
    var phi_32639_: vec3<f32>;
    var phi_32562_: vec3<f32>;
    var phi_32483_: u32;
    var phi_32401_: f32;
    var phi_32324_: u32;
    var phi_32315_: u32;
    var phi_14562_: type_68;
    var phi_14563_: bool;
    var phi_34247_: bool;
    var phi_32314_: u32;
    var phi_14574_: vec3<f32>;
    var phi_14577_: type_39;
    var phi_14579_: u32;
    var phi_14581_: u32;
    var phi_17270_: u32;
    var phi_17273_: u32;
    var phi_17271_: u32;
    var phi_17274_: u32;
    var local_78: u32;
    var phi_17311_: u32;
    var phi_17314_: u32;
    var phi_17312_: u32;
    var phi_17315_: u32;
    var local_79: u32;
    var local_80: u32;
    var phi_17391_: f32;
    var local_81: u32;
    var local_82: u32;
    var phi_17440_: f32;
    var phi_17596_: f32;
    var phi_17603_: vec3<f32>;
    var phi_34611_: u32;
    var phi_17605_: f32;
    var phi_17606_: vec3<f32>;
    var phi_17607_: vec3<f32>;
    var phi_16743_: u32;
    var phi_16746_: u32;
    var phi_16744_: u32;
    var phi_16747_: u32;
    var local_83: u32;
    var local_84: u32;
    var phi_17139_: f32;
    var phi_17142_: f32;
    var phi_26631_: bool;
    var phi_26612_: f32;
    var phi_17143_: f32;
    var phi_26646_: bool;
    var phi_26669_: bool;
    var phi_26684_: bool;
    var phi_16894_: f32;
    var phi_26699_: bool;
    var phi_26714_: bool;
    var phi_26729_: bool;
    var phi_26794_: bool;
    var phi_26882_: bool;
    var phi_26863_: f32;
    var phi_16959_: type_71;
    var phi_17145_: type_71;
    var phi_17223_: bool;
    var phi_17232_: vec3<f32>;
    var phi_17234_: vec3<f32>;
    var phi_17235_: bool;
    var phi_17240_: f32;
    var phi_17241_: vec3<f32>;
    var phi_34619_: u32;
    var phi_17242_: f32;
    var phi_17243_: vec3<f32>;
    var phi_17244_: vec3<f32>;
    var phi_17245_: f32;
    var phi_34618_: u32;
    var phi_17246_: f32;
    var phi_17247_: vec3<f32>;
    var phi_17248_: vec3<f32>;
    var phi_17249_: f32;
    var phi_34610_: u32;
    var phi_17608_: f32;
    var phi_17609_: vec3<f32>;
    var phi_17610_: vec3<f32>;
    var phi_17611_: f32;
    var phi_26897_: bool;
    var phi_27007_: f32;
    var phi_27035_: f32;
    var phi_27063_: f32;
    var phi_27197_: bool;
    var phi_27212_: bool;
    var phi_27227_: bool;
    var phi_27254_: bool;
    var phi_27269_: bool;
    var phi_27284_: bool;
    var phi_27311_: bool;
    var phi_27326_: bool;
    var phi_27341_: bool;
    var phi_14915_: f32;
    var phi_14923_: f32;
    var phi_14931_: f32;
    var phi_27538_: bool;
    var phi_27527_: f32;
    var phi_15038_: vec3<f32>;
    var phi_27566_: bool;
    var phi_15039_: vec3<f32>;
    var phi_15051_: type_55;
    var phi_15052_: type_55;
    var phi_15053_: bool;
    var phi_15058_: type_55;
    var phi_27603_: bool;
    var phi_27633_: bool;
    var phi_27622_: f32;
    var phi_27672_: vec3<f32>;
    var phi_15086_: type_55;
    var phi_15087_: type_55;
    var phi_27694_: vec3<f32>;
    var phi_15088_: type_55;
    var phi_15089_: type_55;
    var phi_15090_: bool;
    var phi_15094_: type_55;
    var phi_27726_: bool;
    var phi_27797_: bool;
    var phi_15914_: f32;
    var phi_27812_: bool;
    var phi_27827_: bool;
    var phi_16387_: vec3<f32>;
    var phi_16388_: vec3<f32>;
    var phi_16389_: bool;
    var phi_16394_: vec3<f32>;
    var phi_16395_: bool;
    var phi_27951_: bool;
    var phi_27966_: bool;
    var phi_16686_: vec3<f32>;
    var phi_16688_: vec3<f32>;
    var phi_16715_: vec3<f32>;
    var phi_16716_: vec3<f32>;
    var phi_15532_: vec3<f32>;
    var phi_15533_: vec3<f32>;
    var phi_15534_: bool;
    var phi_15539_: vec3<f32>;
    var phi_15540_: bool;
    var phi_28128_: bool;
    var phi_28143_: bool;
    var phi_15831_: vec3<f32>;
    var phi_15833_: vec3<f32>;
    var phi_34653_: u32;
    var phi_16717_: f32;
    var phi_16718_: vec3<f32>;
    var phi_16719_: vec3<f32>;
    var phi_34652_: u32;
    var phi_16720_: f32;
    var phi_16721_: vec3<f32>;
    var phi_16722_: vec3<f32>;
    var phi_34609_: u32;
    var phi_17612_: f32;
    var phi_17613_: vec3<f32>;
    var phi_17614_: vec3<f32>;
    var phi_17615_: f32;
    var phi_14677_: f32;
    var phi_14678_: f32;
    var phi_34608_: u32;
    var phi_17616_: f32;
    var phi_17617_: vec3<f32>;
    var phi_17618_: vec3<f32>;
    var phi_17619_: f32;
    var phi_28196_: bool;
    var phi_28306_: f32;
    var phi_28334_: f32;
    var phi_28362_: f32;
    var phi_28496_: bool;
    var phi_28511_: bool;
    var phi_28526_: bool;
    var phi_28553_: bool;
    var phi_28568_: bool;
    var phi_28583_: bool;
    var phi_28610_: bool;
    var phi_28625_: bool;
    var phi_28640_: bool;
    var phi_17842_: f32;
    var phi_17850_: f32;
    var phi_17858_: f32;
    var phi_28837_: bool;
    var phi_28826_: f32;
    var phi_17965_: vec3<f32>;
    var phi_28865_: bool;
    var phi_17966_: vec3<f32>;
    var phi_17978_: type_55;
    var phi_17979_: type_55;
    var phi_17980_: bool;
    var phi_17985_: type_55;
    var phi_28902_: bool;
    var phi_28932_: bool;
    var phi_28921_: f32;
    var phi_28971_: vec3<f32>;
    var phi_18013_: type_55;
    var phi_18014_: type_55;
    var phi_28993_: vec3<f32>;
    var phi_18015_: type_55;
    var phi_18016_: type_55;
    var phi_18017_: bool;
    var phi_18021_: type_55;
    var phi_29007_: f32;
    var phi_29010_: f32;
    var phi_18030_: f32;
    var phi_18064_: u32;
    var phi_18067_: u32;
    var phi_18069_: u32;
    var phi_18073_: type_63;
    var phi_29107_: bool;
    var phi_29137_: bool;
    var phi_29084_: type_52;
    var phi_29087_: type_52;
    var phi_29088_: bool;
    var phi_29093_: type_52;
    var phi_29194_: bool;
    var phi_29209_: bool;
    var phi_29224_: bool;
    var phi_29239_: bool;
    var phi_29171_: type_52;
    var phi_29174_: type_52;
    var phi_29175_: bool;
    var phi_29180_: type_52;
    var phi_29281_: bool;
    var phi_29296_: bool;
    var phi_29311_: bool;
    var phi_29326_: bool;
    var phi_29258_: type_52;
    var phi_29261_: type_52;
    var phi_29262_: bool;
    var phi_29267_: type_52;
    var phi_29060_: bool;
    var phi_29063_: bool;
    var phi_29064_: bool;
    var phi_18480_: u32;
    var phi_18481_: u32;
    var phi_18482_: u32;
    var phi_18483_: u32;
    var phi_18484_: u32;
    var phi_18485_: u32;
    var phi_18486_: u32;
    var phi_18487_: u32;
    var phi_18488_: u32;
    var phi_18369_: vec4<f32>;
    var phi_18370_: vec4<f32>;
    var phi_18371_: bool;
    var phi_18375_: vec4<f32>;
    var phi_18376_: bool;
    var phi_18281_: bool;
    var phi_18318_: bool;
    var phi_18324_: f32;
    var phi_18334_: vec4<f32>;
    var phi_18335_: vec4<f32>;
    var phi_18336_: bool;
    var phi_18340_: vec4<f32>;
    var phi_18341_: bool;
    var phi_18345_: vec4<f32>;
    var phi_18346_: bool;
    var phi_18377_: vec4<f32>;
    var phi_18378_: bool;
    var phi_18210_: vec4<f32>;
    var phi_18211_: vec4<f32>;
    var phi_18212_: bool;
    var phi_18216_: vec4<f32>;
    var phi_18217_: bool;
    var phi_18218_: vec4<f32>;
    var phi_18219_: bool;
    var phi_18220_: bool;
    var phi_18228_: vec4<f32>;
    var phi_18229_: bool;
    var phi_18230_: vec4<f32>;
    var phi_18231_: bool;
    var phi_18232_: bool;
    var phi_18240_: vec4<f32>;
    var phi_18241_: bool;
    var phi_18387_: vec4<f32>;
    var phi_18388_: bool;
    var phi_18409_: type_63;
    var phi_18411_: type_63;
    var phi_18417_: u32;
    var phi_18065_: u32;
    var phi_18068_: u32;
    var phi_18070_: u32;
    var phi_18074_: type_63;
    var phi_18490_: type_63;
    var phi_18491_: bool;
    var phi_18492_: bool;
    var phi_34167_: bool;
    var phi_20306_: bool;
    var phi_20305_: type_63;
    var phi_18664_: type_63;
    var phi_18669_: type_63;
    var phi_18721_: vec3<f32>;
    var phi_18722_: type_39;
    var phi_18723_: vec3<f32>;
    var phi_18724_: type_39;
    var phi_34555_: bool;
    var phi_18725_: vec3<f32>;
    var phi_18726_: type_39;
    var phi_18727_: u32;
    var phi_34554_: bool;
    var phi_18728_: vec3<f32>;
    var phi_18729_: type_39;
    var phi_18730_: u32;
    var phi_34797_: u32;
    var phi_34553_: bool;
    var phi_14575_: vec3<f32>;
    var phi_14578_: type_39;
    var phi_14580_: u32;
    var phi_14582_: u32;
    var phi_34566_: u32;
    var phi_34552_: bool;
    var phi_29477_: bool;
    var phi_29507_: bool;
    var phi_29496_: f32;
    var phi_29427_: bool;
    var phi_29563_: vec3<f32>;
    var phi_29719_: bool;
    var phi_29734_: bool;
    var phi_29749_: bool;
    var phi_29658_: vec3<f32>;
    var phi_29661_: vec3<f32>;
    var phi_29662_: bool;
    var phi_29432_: vec3<f32>;
    var phi_29436_: vec3<f32>;
    var phi_29437_: bool;
    var phi_29849_: bool;
    var phi_29823_: vec3<f32>;
    var phi_29444_: vec3<f32>;
    var phi_29864_: bool;
    var phi_29974_: f32;
    var phi_30002_: f32;
    var phi_30030_: f32;
    var phi_30164_: bool;
    var phi_30179_: bool;
    var phi_30194_: bool;
    var phi_30221_: bool;
    var phi_30236_: bool;
    var phi_30251_: bool;
    var phi_30278_: bool;
    var phi_30293_: bool;
    var phi_30308_: bool;
    var phi_18949_: f32;
    var phi_18957_: f32;
    var phi_18965_: f32;
    var phi_30505_: bool;
    var phi_30494_: f32;
    var phi_19072_: vec3<f32>;
    var phi_30533_: bool;
    var phi_19073_: vec3<f32>;
    var phi_19085_: type_55;
    var phi_19086_: type_55;
    var phi_19087_: bool;
    var phi_19092_: type_55;
    var phi_30570_: bool;
    var phi_30600_: bool;
    var phi_30589_: f32;
    var phi_30639_: vec3<f32>;
    var phi_19120_: type_55;
    var phi_19121_: type_55;
    var phi_30661_: vec3<f32>;
    var phi_19122_: type_55;
    var phi_19123_: type_55;
    var phi_19124_: bool;
    var phi_19128_: type_55;
    var phi_30686_: bool;
    var phi_19204_: vec3<f32>;
    var phi_36056_: u32;
    var phi_19206_: vec3<f32>;
    var phi_19207_: bool;
    var phi_36055_: u32;
    var phi_19208_: vec3<f32>;
    var phi_19209_: bool;
    var phi_36053_: u32;
    var phi_19215_: vec3<f32>;
    var phi_19216_: bool;
    var local_85: u32;
    var local_86: vec3<f32>;
    var local_87: vec3<f32>;
    var local_88: vec3<f32>;
    var local_89: type_39;
    var phi_19245_: vec3<f32>;
    var local_90: u32;
    var local_91: vec3<f32>;
    var local_92: vec3<f32>;
    var local_93: vec3<f32>;
    var local_94: type_39;
    var phi_19261_: type_68;
    var phi_19262_: type_68;
    var phi_36060_: u32;
    var phi_19264_: type_68;
    var phi_19265_: type_68;
    var phi_19266_: bool;
    var phi_36059_: u32;
    var phi_19267_: type_68;
    var phi_19268_: type_68;
    var phi_19269_: bool;
    var phi_19270_: bool;
    var local_95: u32;
    var local_96: vec3<f32>;
    var local_97: vec3<f32>;
    var local_98: vec3<f32>;
    var local_99: type_39;
    var phi_19276_: type_68;
    var local_100: u32;
    var local_101: vec3<f32>;
    var local_102: vec3<f32>;
    var local_103: vec3<f32>;
    var local_104: type_39;
    var phi_19281_: type_68;
    var phi_19282_: type_68;
    var phi_19283_: bool;
    var phi_19287_: type_68;
    var phi_19288_: type_68;
    var phi_19289_: bool;
    var phi_13612_: bool;
    var phi_13621_: vec3<f32>;
    var phi_13623_: vec3<f32>;
    var phi_13624_: bool;
    var phi_13785_: f32;
    var phi_13788_: f32;
    var phi_30907_: bool;
    var phi_30888_: f32;
    var phi_13789_: f32;
    var phi_30925_: f32;
    var phi_30928_: f32;
    var phi_13792_: f32;
    var phi_13793_: f32;
    var phi_13794_: f32;
    var phi_19293_: type_68;
    var phi_19294_: type_68;
    var phi_19295_: bool;
    var phi_12048_: f32;
    var phi_30975_: f32;
    var phi_30978_: f32;
    var phi_12050_: f32;
    var phi_12051_: f32;
    var phi_31004_: bool;
    var phi_12102_: f32;
    var phi_31022_: f32;
    var phi_31025_: f32;
    var phi_12104_: f32;
    var phi_31036_: bool;
    var phi_31051_: bool;
    var phi_12573_: vec3<f32>;
    var phi_12574_: vec3<f32>;
    var phi_12575_: bool;
    var phi_12580_: vec3<f32>;
    var phi_12581_: bool;
    var phi_31175_: bool;
    var phi_31190_: bool;
    var phi_12872_: vec3<f32>;
    var phi_12874_: vec3<f32>;
    var phi_12901_: vec3<f32>;
    var phi_12915_: vec3<f32>;
    var phi_19299_: type_68;
    var phi_19300_: type_68;
    var phi_19301_: bool;
    var phi_19305_: type_68;
    var phi_11188_: bool;
    var phi_11189_: bool;
    var phi_11190_: bool;
    var phi_11211_: vec4<f32>;
    var phi_11212_: type_35;
    var phi_19309_: vec4<f32>;
    var phi_19310_: type_39;
    var phi_19311_: type_39;
    var phi_19312_: type_35;
    var local_105: type_63;
    var local_106: type_63;

    switch bitcast<i32>(0u) {
        default: {
            let _e272 = global;
            let _e284 = arrayLength((&global_8.member));
            let _e288 = (_e272.x + (_e272.y * 4194240u));
            let _e292 = global_1.member.member[0u];
            let _e296 = global_1.member.member[1u];
            if (_e288 >= (_e292 * _e296)) {
                phi_11119_ = type_45(0u, type_45().member_1);
            } else {
                phi_11119_ = type_45(1u, _e288);
            }
            let _e304 = phi_11119_;
            if (bitcast<i32>(_e304.member) != 0i) {
                let _e309 = (_e304.member_1 < arrayLength((&global_7.member)));
                if _e309 {
                } else {
                    break;
                }
                let _e312 = global_7.member[_e304.member_1];
                if ((_e312.member_4.y & 1u) == 0u) {
                    if ((_e312.member_4.y & 8u) == 0u) {
                        let _e7299 = (_e312.member_3.x * _e312.member.w);
                        let _e7300 = (_e312.member_3.y * _e312.member.w);
                        let _e7301 = (_e312.member_3.z * _e312.member.w);
                        if (_e7299 == _e7299) {
                            let _e7304 = (_e7300 == _e7300);
                            if _e7304 {
                                phi_11188_ = select(true, false, (_e7301 == _e7301));
                            } else {
                                phi_11188_ = false;
                            }
                            let _e7308 = phi_11188_;
                            phi_11189_ = select(true, false, _e7304);
                            phi_11190_ = _e7308;
                        } else {
                            phi_11189_ = true;
                            phi_11190_ = false;
                        }
                        let _e7311 = phi_11189_;
                        let _e7313 = phi_11190_;
                        let _e7316 = select(vec3<f32>(_e7299, _e7300, _e7301), vec3<f32>(0f, 0f, 0f), vec3(select(_e7313, true, _e7311)));
                        phi_11211_ = vec4<f32>(_e7316.x, _e7316.y, _e7316.z, _e312.member_3.w);
                        phi_11212_ = type_35(_e312.member, _e312.member_1, _e312.member_2, _e312.member_3, vec4<u32>(_e312.member_4.x, (_e312.member_4.y | 8u), _e312.member_4.z, _e312.member_4.w));
                    } else {
                        phi_11211_ = vec4<f32>(0f, 0f, 0f, 0f);
                        phi_11212_ = _e312;
                    }
                    let _e7335 = phi_11211_;
                    let _e7337 = phi_11212_;
                    phi_19309_ = _e7335;
                    phi_19310_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_19311_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    phi_19312_ = _e7337;
                } else {
                    let _e328 = vec3<f32>(_e312.member_1.x, _e312.member_1.y, _e312.member_1.z);
                    let _e333 = vec3<f32>(_e312.member_2.x, _e312.member_2.y, _e312.member_2.z);
                    let _e339 = select(select(u32(_e312.member_2.w), 0u, (_e312.member_2.w < 0f)), 4294967295u, (_e312.member_2.w > 4294967000f));
                    let _e343 = vec3<f32>(_e312.member_3.x, _e312.member_3.y, _e312.member_3.z);
                    let _e352 = ((_e312.member_4.y & 2u) != 0u);
                    let _e355 = global_1.member.member[3u];
                    let _e358 = global_1.member.member_3[3u];
                    let _e361 = global_1.member.member_1[2u];
                    let _e364 = global_1.member.member_1[0u];
                    let _e367 = global_1.member.member_1[1u];
                    let _e368 = (_e364 + _e367);
                    let _e369 = (vec3<f32>(1f, 1f, 1f) / _e328);
                    local_68 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                    let _e371 = select(0u, 1u, (_e364 > 0u));
                    phi_11308_ = _e364;
                    phi_11311_ = _e371;
                    phi_11313_ = _e368;
                    phi_11315_ = vec4<f32>(0f, 0f, 0f, 0f);
                    phi_11317_ = type_63(300000000000000000000000000000000000000f, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                    loop {
                        let _e373 = phi_11308_;
                        let _e375 = phi_11311_;
                        let _e377 = phi_11313_;
                        let _e379 = phi_11315_;
                        let _e381 = phi_11317_;
                        local_69 = _e381;
                        local_70 = _e379;
                        local_71 = _e379;
                        local_72 = _e379;
                        local_73 = _e381;
                        local_105 = _e381;
                        if (_e373 < _e377) {
                            let _e550 = global_2.member[_e373];
                            if (_e550.member_6.x == 0u) {
                                let _e656 = (_e550.member_1.x - _e550.member.x);
                                let _e657 = (_e550.member_1.y - _e550.member.y);
                                let _e658 = (_e550.member_1.z - _e550.member.z);
                                let _e663 = (_e550.member_2.x - _e550.member.x);
                                let _e664 = (_e550.member_2.y - _e550.member.y);
                                let _e665 = (_e550.member_2.z - _e550.member.z);
                                let _e668 = ((_e312.member_1.y * _e665) - (_e664 * _e312.member_1.z));
                                let _e671 = ((_e312.member_1.z * _e663) - (_e665 * _e312.member_1.x));
                                let _e674 = ((_e312.member_1.x * _e664) - (_e663 * _e312.member_1.y));
                                let _e679 = (((_e656 * _e668) + (_e657 * _e671)) + (_e658 * _e674));
                                if (abs(_e679) < 0.000000000001f) {
                                    phi_11475_ = vec4<f32>();
                                    phi_11476_ = true;
                                } else {
                                    let _e682 = (1f / _e679);
                                    let _e683 = (_e312.member.x - _e550.member.x);
                                    let _e684 = (_e312.member.y - _e550.member.y);
                                    let _e685 = (_e312.member.z - _e550.member.z);
                                    let _e691 = ((((_e683 * _e668) + (_e684 * _e671)) + (_e685 * _e674)) * _e682);
                                    if (_e691 < 0f) {
                                        phi_11465_ = vec4<f32>();
                                        phi_11466_ = true;
                                        phi_11467_ = false;
                                    } else {
                                        let _e693 = (_e691 > 1f);
                                        if _e693 {
                                            phi_11463_ = vec4<f32>();
                                            phi_11464_ = false;
                                        } else {
                                            let _e696 = ((_e684 * _e658) - (_e657 * _e685));
                                            let _e699 = ((_e685 * _e656) - (_e658 * _e683));
                                            let _e702 = ((_e683 * _e657) - (_e656 * _e684));
                                            let _e708 = ((((_e312.member_1.x * _e696) + (_e312.member_1.y * _e699)) + (_e312.member_1.z * _e702)) * _e682);
                                            if (_e708 < 0f) {
                                                phi_11453_ = vec4<f32>();
                                                phi_11454_ = true;
                                                phi_11455_ = false;
                                            } else {
                                                let _e711 = ((_e691 + _e708) > 1f);
                                                if _e711 {
                                                    phi_11451_ = vec4<f32>();
                                                    phi_11452_ = false;
                                                } else {
                                                    let _e717 = ((((_e663 * _e696) + (_e664 * _e699)) + (_e665 * _e702)) * _e682);
                                                    if (_e717 <= 0f) {
                                                        phi_11446_ = vec4<f32>();
                                                        phi_11447_ = true;
                                                    } else {
                                                        let _e719 = (_e717 >= _e381.member);
                                                        if _e719 {
                                                            phi_11445_ = vec4<f32>();
                                                        } else {
                                                            phi_11445_ = vec4<f32>(_e717, _e691, _e708, 1f);
                                                        }
                                                        let _e722 = phi_11445_;
                                                        phi_11446_ = _e722;
                                                        phi_11447_ = _e719;
                                                    }
                                                    let _e724 = phi_11446_;
                                                    let _e726 = phi_11447_;
                                                    phi_11451_ = _e724;
                                                    phi_11452_ = _e726;
                                                }
                                                let _e728 = phi_11451_;
                                                let _e730 = phi_11452_;
                                                phi_11453_ = _e728;
                                                phi_11454_ = _e711;
                                                phi_11455_ = _e730;
                                            }
                                            let _e732 = phi_11453_;
                                            let _e734 = phi_11454_;
                                            let _e736 = phi_11455_;
                                            phi_11463_ = _e732;
                                            phi_11464_ = select(_e736, true, _e734);
                                        }
                                        let _e739 = phi_11463_;
                                        let _e741 = phi_11464_;
                                        phi_11465_ = _e739;
                                        phi_11466_ = _e693;
                                        phi_11467_ = _e741;
                                    }
                                    let _e743 = phi_11465_;
                                    let _e745 = phi_11466_;
                                    let _e747 = phi_11467_;
                                    phi_11475_ = _e743;
                                    phi_11476_ = select(_e747, true, _e745);
                                }
                                let _e750 = phi_11475_;
                                let _e752 = phi_11476_;
                                phi_11614_ = _e750;
                                phi_11615_ = _e752;
                            } else {
                                if (_e550.member_6.x == 1u) {
                                    let _e578 = (_e312.member.x - _e550.member.x);
                                    let _e579 = (_e312.member.y - _e550.member.y);
                                    let _e580 = (_e312.member.z - _e550.member.z);
                                    let _e585 = (((_e578 * _e312.member_1.x) + (_e579 * _e312.member_1.y)) + (_e580 * _e312.member_1.z));
                                    if (_e585 > 0f) {
                                        phi_11513_ = select(true, false, ((((_e578 * _e578) + (_e579 * _e579)) + (_e580 * _e580)) > (_e550.member.w * _e550.member.w)));
                                    } else {
                                        phi_11513_ = true;
                                    }
                                    let _e598 = phi_11513_;
                                    if _e598 {
                                        let _e602 = (_e578 - (_e312.member_1.x * _e585));
                                        let _e603 = (_e579 - (_e312.member_1.y * _e585));
                                        let _e604 = (_e580 - (_e312.member_1.z * _e585));
                                        let _e613 = ((_e550.member.w * _e550.member.w) - (((_e602 * _e602) + (_e603 * _e603)) + (_e604 * _e604)));
                                        if (_e613 < 0f) {
                                            phi_11569_ = vec4<f32>();
                                            phi_11570_ = true;
                                        } else {
                                            let _e615 = sqrt(_e613);
                                            let _e616 = -(_e585);
                                            let _e617 = (_e616 - _e615);
                                            if (_e617 <= 0f) {
                                                phi_11547_ = true;
                                            } else {
                                                phi_11547_ = (_e617 >= _e381.member);
                                            }
                                            let _e621 = phi_11547_;
                                            if _e621 {
                                                phi_11553_ = (_e616 + _e615);
                                            } else {
                                                phi_11553_ = _e617;
                                            }
                                            let _e624 = phi_11553_;
                                            if (_e624 <= 0f) {
                                                phi_11564_ = vec4<f32>();
                                                phi_11565_ = true;
                                            } else {
                                                let _e626 = (_e624 >= _e381.member);
                                                if _e626 {
                                                    phi_11563_ = vec4<f32>();
                                                } else {
                                                    phi_11563_ = vec4<f32>(_e624, 0f, 0f, 1f);
                                                }
                                                let _e629 = phi_11563_;
                                                phi_11564_ = _e629;
                                                phi_11565_ = _e626;
                                            }
                                            let _e631 = phi_11564_;
                                            let _e633 = phi_11565_;
                                            phi_11569_ = _e631;
                                            phi_11570_ = _e633;
                                        }
                                        let _e635 = phi_11569_;
                                        let _e637 = phi_11570_;
                                        phi_11574_ = _e635;
                                        phi_11575_ = _e637;
                                    } else {
                                        phi_11574_ = vec4<f32>();
                                        phi_11575_ = true;
                                    }
                                    let _e639 = phi_11574_;
                                    let _e641 = phi_11575_;
                                    phi_11604_ = _e639;
                                    phi_11605_ = _e641;
                                } else {
                                    if (abs(_e312.member_1.y) < 0.000000000001f) {
                                        phi_11602_ = vec4<f32>();
                                        phi_11603_ = true;
                                    } else {
                                        let _e560 = ((_e550.member.x - _e312.member.y) / _e312.member_1.y);
                                        if (_e560 <= 0f) {
                                            phi_11597_ = vec4<f32>();
                                            phi_11598_ = true;
                                        } else {
                                            let _e562 = (_e560 >= _e381.member);
                                            if _e562 {
                                                phi_11596_ = vec4<f32>();
                                            } else {
                                                phi_11596_ = vec4<f32>(_e560, 0f, 0f, 1f);
                                            }
                                            let _e565 = phi_11596_;
                                            phi_11597_ = _e565;
                                            phi_11598_ = _e562;
                                        }
                                        let _e567 = phi_11597_;
                                        let _e569 = phi_11598_;
                                        phi_11602_ = _e567;
                                        phi_11603_ = _e569;
                                    }
                                    let _e571 = phi_11602_;
                                    let _e573 = phi_11603_;
                                    phi_11604_ = _e571;
                                    phi_11605_ = _e573;
                                }
                                let _e643 = phi_11604_;
                                let _e645 = phi_11605_;
                                phi_11614_ = _e643;
                                phi_11615_ = select(true, false, select(true, false, _e645));
                            }
                            let _e754 = phi_11614_;
                            let _e756 = phi_11615_;
                            let _e759 = select(vec4<f32>(0f, 0f, 0f, 0f), _e754, vec4(select(true, false, _e756)));
                            let _e761 = (_e759.w > 0f);
                            if _e761 {
                                let _e767 = type_63(_e759.x, _e381.member_1, _e381.member_2, _e381.member_3);
                                let _e772 = type_63(_e767.member, _e373, _e767.member_2, _e767.member_3);
                                phi_11636_ = _e772;
                                phi_11638_ = _e772;
                            } else {
                                phi_11636_ = type_63();
                                phi_11638_ = _e381;
                            }
                            let _e774 = phi_11636_;
                            let _e776 = phi_11638_;
                            phi_11309_ = (_e373 + 1u);
                            phi_11312_ = _e375;
                            phi_11314_ = _e377;
                            phi_11316_ = select(_e379, _e759, vec4(_e761));
                            phi_11318_ = _e776;
                            phi_11717_ = _e774;
                            phi_11718_ = true;
                            phi_11719_ = false;
                        } else {
                            let _e383 = (_e375 == 0u);
                            if _e383 {
                                phi_11713_ = u32();
                                phi_11714_ = u32();
                                phi_11715_ = u32();
                            } else {
                                let _e384 = (_e375 - 1u);
                                let _e385 = (_e384 < 100u);
                                if _e385 {
                                } else {
                                    phi_31971_ = true;
                                    phi_20304_ = bool();
                                    phi_20303_ = type_63();
                                    break;
                                }
                                let _e387 = local_68[_e384];
                                let _e389 = global_3.member[_e387];
                                let _e400 = ((_e389.member.x - _e312.member.x) * _e369.x);
                                let _e402 = ((_e389.member.y - _e312.member.y) * _e369.y);
                                let _e404 = ((_e389.member.z - _e312.member.z) * _e369.z);
                                let _e411 = ((_e389.member_2.x - _e312.member.x) * _e369.x);
                                let _e412 = ((_e389.member_2.y - _e312.member.y) * _e369.y);
                                let _e413 = ((_e389.member_2.z - _e312.member.z) * _e369.z);
                                if (_e400 == _e400) {
                                    let _e415 = (_e411 == _e411);
                                    if _e415 {
                                        let _e416 = (_e400 != _e400);
                                        if _e416 {
                                            phi_24706_ = true;
                                        } else {
                                            phi_24706_ = (_e411 <= _e400);
                                        }
                                        let _e419 = phi_24706_;
                                        let _e420 = select(_e400, _e411, _e419);
                                        if _e416 {
                                            phi_24736_ = true;
                                        } else {
                                            phi_24736_ = (_e411 >= _e400);
                                        }
                                        let _e425 = phi_24736_;
                                        let _e426 = select(_e400, _e411, _e425);
                                        phi_24683_ = type_52(select(-300000000000000000000000000000000000000f, _e420, (_e420 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e426, (_e426 <= 300000000000000000000000000000000000000f)));
                                    } else {
                                        phi_24683_ = type_52();
                                    }
                                    let _e431 = phi_24683_;
                                    phi_24686_ = _e431;
                                    phi_24687_ = select(true, false, _e415);
                                } else {
                                    phi_24686_ = type_52();
                                    phi_24687_ = true;
                                }
                                let _e434 = phi_24686_;
                                let _e436 = phi_24687_;
                                if _e436 {
                                    phi_24692_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                } else {
                                    phi_24692_ = _e434;
                                }
                                let _e438 = phi_24692_;
                                if (_e402 == _e402) {
                                    let _e442 = (_e412 == _e412);
                                    if _e442 {
                                        let _e443 = (_e402 != _e402);
                                        if _e443 {
                                            phi_24793_ = true;
                                        } else {
                                            phi_24793_ = (_e412 <= _e402);
                                        }
                                        let _e446 = phi_24793_;
                                        let _e447 = select(_e402, _e412, _e446);
                                        if (_e438.member != _e438.member) {
                                            phi_24808_ = true;
                                        } else {
                                            phi_24808_ = (_e447 >= _e438.member);
                                        }
                                        let _e451 = phi_24808_;
                                        if _e443 {
                                            phi_24823_ = true;
                                        } else {
                                            phi_24823_ = (_e412 >= _e402);
                                        }
                                        let _e455 = phi_24823_;
                                        let _e456 = select(_e402, _e412, _e455);
                                        if (_e438.member_1 != _e438.member_1) {
                                            phi_24838_ = true;
                                        } else {
                                            phi_24838_ = (_e456 <= _e438.member_1);
                                        }
                                        let _e460 = phi_24838_;
                                        phi_24770_ = type_52(select(_e438.member, _e447, _e451), select(_e438.member_1, _e456, _e460));
                                    } else {
                                        phi_24770_ = type_52();
                                    }
                                    let _e464 = phi_24770_;
                                    phi_24773_ = _e464;
                                    phi_24774_ = select(true, false, _e442);
                                } else {
                                    phi_24773_ = type_52();
                                    phi_24774_ = true;
                                }
                                let _e467 = phi_24773_;
                                let _e469 = phi_24774_;
                                if _e469 {
                                    phi_24779_ = _e438;
                                } else {
                                    phi_24779_ = _e467;
                                }
                                let _e471 = phi_24779_;
                                if (_e404 == _e404) {
                                    let _e475 = (_e413 == _e413);
                                    if _e475 {
                                        let _e476 = (_e404 != _e404);
                                        if _e476 {
                                            phi_24880_ = true;
                                        } else {
                                            phi_24880_ = (_e413 <= _e404);
                                        }
                                        let _e479 = phi_24880_;
                                        let _e480 = select(_e404, _e413, _e479);
                                        if (_e471.member != _e471.member) {
                                            phi_24895_ = true;
                                        } else {
                                            phi_24895_ = (_e480 >= _e471.member);
                                        }
                                        let _e484 = phi_24895_;
                                        if _e476 {
                                            phi_24910_ = true;
                                        } else {
                                            phi_24910_ = (_e413 >= _e404);
                                        }
                                        let _e488 = phi_24910_;
                                        let _e489 = select(_e404, _e413, _e488);
                                        if (_e471.member_1 != _e471.member_1) {
                                            phi_24925_ = true;
                                        } else {
                                            phi_24925_ = (_e489 <= _e471.member_1);
                                        }
                                        let _e493 = phi_24925_;
                                        phi_24857_ = type_52(select(_e471.member, _e480, _e484), select(_e471.member_1, _e489, _e493));
                                    } else {
                                        phi_24857_ = type_52();
                                    }
                                    let _e497 = phi_24857_;
                                    phi_24860_ = _e497;
                                    phi_24861_ = select(true, false, _e475);
                                } else {
                                    phi_24860_ = type_52();
                                    phi_24861_ = true;
                                }
                                let _e500 = phi_24860_;
                                let _e502 = phi_24861_;
                                if _e502 {
                                    phi_24866_ = _e471;
                                } else {
                                    phi_24866_ = _e500;
                                }
                                let _e504 = phi_24866_;
                                if (_e504.member_1 < 0f) {
                                    phi_24662_ = bool();
                                    phi_24663_ = true;
                                } else {
                                    let _e508 = (_e504.member > _e504.member_1);
                                    if _e508 {
                                        phi_24659_ = bool();
                                    } else {
                                        phi_24659_ = (_e504.member > _e381.member);
                                    }
                                    let _e511 = phi_24659_;
                                    phi_24662_ = _e511;
                                    phi_24663_ = _e508;
                                }
                                let _e513 = phi_24662_;
                                let _e515 = phi_24663_;
                                if !(select(_e513, true, _e515)) {
                                    if (_e389.member_3 > 0u) {
                                        phi_11707_ = _e389.member_1;
                                        phi_11708_ = _e384;
                                        phi_11709_ = (_e389.member_1 + _e389.member_3);
                                    } else {
                                        let _e520 = (_e375 + 1u);
                                        let _e521 = (_e520 <= 100u);
                                        if _e521 {
                                            if _e385 {
                                            } else {
                                                phi_31971_ = true;
                                                phi_20304_ = bool();
                                                phi_20303_ = type_63();
                                                break;
                                            }
                                            local_68[_e384] = _e389.member_1;
                                            if (_e375 < 100u) {
                                            } else {
                                                phi_31971_ = true;
                                                phi_20304_ = bool();
                                                phi_20303_ = type_63();
                                                break;
                                            }
                                            local_68[_e375] = (_e387 + 1u);
                                        }
                                        phi_11707_ = _e373;
                                        phi_11708_ = select(_e384, _e520, _e521);
                                        phi_11709_ = _e377;
                                    }
                                    let _e530 = phi_11707_;
                                    let _e532 = phi_11708_;
                                    let _e534 = phi_11709_;
                                    phi_11710_ = _e530;
                                    phi_11711_ = _e532;
                                    phi_11712_ = _e534;
                                } else {
                                    phi_11710_ = _e373;
                                    phi_11711_ = _e384;
                                    phi_11712_ = _e377;
                                }
                                let _e536 = phi_11710_;
                                let _e538 = phi_11711_;
                                let _e540 = phi_11712_;
                                phi_11713_ = _e536;
                                phi_11714_ = _e538;
                                phi_11715_ = _e540;
                            }
                            let _e542 = phi_11713_;
                            let _e544 = phi_11714_;
                            let _e546 = phi_11715_;
                            phi_11309_ = _e542;
                            phi_11312_ = _e544;
                            phi_11314_ = _e546;
                            phi_11316_ = _e379;
                            phi_11318_ = _e381;
                            phi_11717_ = type_63();
                            phi_11718_ = select(true, false, _e383);
                            phi_11719_ = _e383;
                        }
                        let _e781 = phi_11309_;
                        let _e783 = phi_11312_;
                        let _e785 = phi_11314_;
                        let _e787 = phi_11316_;
                        let _e789 = phi_11318_;
                        let _e791 = phi_11717_;
                        let _e793 = phi_11718_;
                        let _e795 = phi_11719_;
                        phi_11308_ = _e781;
                        phi_11311_ = _e783;
                        phi_11313_ = _e785;
                        phi_11315_ = _e787;
                        phi_11317_ = _e789;
                        phi_31971_ = false;
                        phi_20304_ = _e795;
                        phi_20303_ = _e791;
                        if !(_e793) {
                            break;
                        }
                    }
                    let _e798 = phi_31971_;
                    let _e800 = phi_20304_;
                    let _e802 = phi_20303_;
                    if _e798 {
                        break;
                    }
                    if _e800 {
                        let _e804 = local_69;
                        if (_e804.member_1 != 4294967295u) {
                            let _e808 = global_2.member[_e804.member_1];
                            if (_e808.member_6.x == 0u) {
                                let _e850 = (_e808.member_1.x - _e808.member.x);
                                let _e851 = (_e808.member_1.y - _e808.member.y);
                                let _e852 = (_e808.member_1.z - _e808.member.z);
                                let _e857 = (_e808.member_2.x - _e808.member.x);
                                let _e858 = (_e808.member_2.y - _e808.member.y);
                                let _e859 = (_e808.member_2.z - _e808.member.z);
                                let _e862 = ((_e851 * _e859) - (_e858 * _e852));
                                let _e865 = ((_e852 * _e857) - (_e859 * _e850));
                                let _e868 = ((_e850 * _e858) - (_e857 * _e851));
                                let _e875 = (1f / sqrt((((_e862 * _e862) + (_e865 * _e865)) + (_e868 * _e868))));
                                let _e879 = vec3<f32>((_e862 * _e875), (_e865 * _e875), (_e868 * _e875));
                                if (_e808.member_6.w == 1u) {
                                    let _e888 = local_71;
                                    let _e892 = local_72;
                                    let _e894 = ((1f - _e888.y) - _e892.z);
                                    let _e915 = (((_e808.member_3.x * _e894) + (_e808.member_4.x * _e888.y)) + (_e808.member_5.x * _e892.z));
                                    let _e916 = (((_e808.member_3.y * _e894) + (_e808.member_4.y * _e888.y)) + (_e808.member_5.y * _e892.z));
                                    let _e917 = (((_e808.member_3.z * _e894) + (_e808.member_4.z * _e888.y)) + (_e808.member_5.z * _e892.z));
                                    let _e922 = (((_e915 * _e915) + (_e916 * _e916)) + (_e917 * _e917));
                                    if (_e922 > 0f) {
                                        let _e925 = (1f / sqrt(_e922));
                                        phi_11846_ = vec3<f32>((_e915 * _e925), (_e916 * _e925), (_e917 * _e925));
                                    } else {
                                        phi_11846_ = _e879;
                                    }
                                    let _e931 = phi_11846_;
                                    phi_11847_ = _e931;
                                } else {
                                    phi_11847_ = _e879;
                                }
                                let _e933 = phi_11847_;
                                phi_11881_ = type_56(_e879, _e933);
                            } else {
                                if (_e808.member_6.x == 1u) {
                                    let _e818 = local_70;
                                    let _e838 = vec3<f32>((((_e312.member.x + (_e312.member_1.x * _e818.x)) - _e808.member.x) / _e808.member.w), (((_e312.member.y + (_e312.member_1.y * _e818.x)) - _e808.member.y) / _e808.member.w), (((_e312.member.z + (_e312.member_1.z * _e818.x)) - _e808.member.z) / _e808.member.w));
                                    phi_11880_ = type_56(_e838, _e838);
                                } else {
                                    let _e815 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e312.member_1.y > 0f)));
                                    phi_11880_ = type_56(_e815, _e815);
                                }
                                let _e841 = phi_11880_;
                                phi_11881_ = _e841;
                            }
                            let _e936 = phi_11881_;
                            let _e940 = local_73;
                            let _e945 = type_63(_e940.member, _e940.member_1, _e936.member, _e940.member_3);
                            phi_11886_ = type_63(_e945.member, _e945.member_1, _e945.member_2, _e936.member_1);
                        } else {
                            let _e7478 = local_105;
                            phi_11886_ = _e7478;
                        }
                        let _e952 = phi_11886_;
                        phi_11887_ = _e952;
                    } else {
                        phi_11887_ = type_63();
                    }
                    let _e954 = phi_11887_;
                    if select(true, false, _e800) {
                        phi_11892_ = _e802;
                    } else {
                        phi_11892_ = _e954;
                    }
                    let _e957 = phi_11892_;
                    let _e958 = (select(select(u32(_e312.member_3.w), 0u, (_e312.member_3.w < 0f)), 4294967295u, (_e312.member_3.w > 4294967000f)) + 1u);
                    if (_e957.member_1 == 4294967295u) {
                        if _e352 {
                            phi_12051_ = 1f;
                        } else {
                            if ((_e358 & 2u) == 0u) {
                                phi_12050_ = 1f;
                            } else {
                                let _e6780 = global_1.member.member_3[1u];
                                let _e6783 = global_1.member.member_3[2u];
                                let _e6786 = ((atan2(_e312.member_1.z, _e312.member_1.x) * 0.15915494f) + 0.5f);
                                let _e6790 = select(_e312.member_1.y, -1f, (_e312.member_1.y < -1f));
                                let _e6793 = acos(select(_e6790, 1f, (_e6790 > 1f)));
                                let _e6795 = f32(_e6780);
                                let _e6796 = ((_e6786 - floor(_e6786)) * _e6795);
                                let _e6801 = select(select(u32(_e6796), 0u, (_e6796 < 0f)), 4294967295u, (_e6796 > 4294967000f));
                                let _e6802 = (_e6780 - 1u);
                                let _e6805 = f32(_e6783);
                                let _e6806 = ((_e6793 * 0.31830987f) * _e6805);
                                let _e6811 = select(select(u32(_e6806), 0u, (_e6806 < 0f)), 4294967295u, (_e6806 > 4294967000f));
                                let _e6812 = (_e6783 - 1u);
                                let _e6814 = select(_e6811, _e6812, (_e6812 < _e6811));
                                let _e6815 = sin(_e6793);
                                if (_e6815 <= 0f) {
                                    phi_12048_ = 0f;
                                } else {
                                    let _e6819 = global_1.member.member_2[3u];
                                    let _e6822 = global_1.member.member_3[0u];
                                    let _e6826 = (_e6819 + _e6814);
                                    let _e6829 = global_6.member[(_e6826 + 1u)];
                                    let _e6831 = global_6.member[_e6826];
                                    let _e6834 = ((_e6822 + (_e6814 * (_e6780 + 1u))) + select(_e6801, _e6802, (_e6802 < _e6801)));
                                    let _e6837 = global_6.member[(_e6834 + 1u)];
                                    let _e6839 = global_6.member[_e6834];
                                    phi_12048_ = ((((_e6829 - _e6831) * _e6805) * ((_e6837 - _e6839) * _e6795)) / (19.73921f * _e6815));
                                }
                                let _e6846 = phi_12048_;
                                if (_e312.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_30978_ = 1f;
                                } else {
                                    let _e6848 = (_e312.member_1.w * _e312.member_1.w);
                                    let _e6850 = (_e6848 + (_e6846 * _e6846));
                                    if (_e6850 == 0f) {
                                        phi_30975_ = 0f;
                                    } else {
                                        phi_30975_ = (_e6848 / _e6850);
                                    }
                                    let _e6854 = phi_30975_;
                                    phi_30978_ = _e6854;
                                }
                                let _e6856 = phi_30978_;
                                phi_12050_ = _e6856;
                            }
                            let _e6858 = phi_12050_;
                            phi_12051_ = _e6858;
                        }
                        let _e6860 = phi_12051_;
                        if ((_e358 & 4u) == 0u) {
                            phi_12915_ = _e343;
                        } else {
                            if _e352 {
                                phi_12104_ = 1f;
                            } else {
                                let _e6871 = global_1.member.member_5;
                                let _e6875 = (_e312.member_1.x - _e6871.x);
                                let _e6876 = (_e312.member_1.y - _e6871.y);
                                let _e6877 = (_e312.member_1.z - _e6871.z);
                                let _e6884 = (sqrt((((_e6875 * _e6875) + (_e6876 * _e6876)) + (_e6877 * _e6877))) * 0.5f);
                                if (_e6884 != _e6884) {
                                    phi_31004_ = true;
                                } else {
                                    phi_31004_ = (1f <= _e6884);
                                }
                                let _e6888 = phi_31004_;
                                let _e6894 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e6884, 1f, _e6888))) >= _e6894) {
                                    phi_12102_ = 0f;
                                } else {
                                    let _e6897 = sin((_e6894 * 0.5f));
                                    phi_12102_ = (1f / ((12.566371f * _e6897) * _e6897));
                                }
                                let _e6902 = phi_12102_;
                                if (_e312.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_31025_ = 1f;
                                } else {
                                    let _e6904 = (_e312.member_1.w * _e312.member_1.w);
                                    let _e6906 = (_e6904 + (_e6902 * _e6902));
                                    if (_e6906 == 0f) {
                                        phi_31022_ = 0f;
                                    } else {
                                        phi_31022_ = (_e6904 / _e6906);
                                    }
                                    let _e6910 = phi_31022_;
                                    phi_31025_ = _e6910;
                                }
                                let _e6912 = phi_31025_;
                                phi_12104_ = _e6912;
                            }
                            let _e6914 = phi_12104_;
                            let _e6916 = global_1.member.member_5;
                            let _e6920 = (_e312.member_1.x - _e6916.x);
                            let _e6921 = (_e312.member_1.y - _e6916.y);
                            let _e6922 = (_e312.member_1.z - _e6916.z);
                            let _e6929 = (sqrt((((_e6920 * _e6920) + (_e6921 * _e6921)) + (_e6922 * _e6922))) * 0.5f);
                            if (_e6929 != _e6929) {
                                phi_31036_ = true;
                            } else {
                                phi_31036_ = (1f <= _e6929);
                            }
                            let _e6933 = phi_31036_;
                            let _e6936 = (2f * asin(select(_e6929, 1f, _e6933)));
                            let _e6940 = global_1.member.member_6[3u];
                            if (_e6936 >= _e6940) {
                                phi_12901_ = vec3<f32>(0f, 0f, 0f);
                            } else {
                                let _e6944 = (1f - pow((_e6936 / _e6940), 2f));
                                if (_e6944 != _e6944) {
                                    phi_31051_ = true;
                                } else {
                                    phi_31051_ = (0f >= _e6944);
                                }
                                let _e6948 = phi_31051_;
                                let _e6951 = global_1.member.member_6;
                                let _e6957 = global_1.member.member_8[2u];
                                if (_e6957 == 4294967295u) {
                                    phi_12874_ = vec3<f32>(1f, 1f, 1f);
                                } else {
                                    let _e6960 = global_6.member[_e6957];
                                    let _e6963 = global_6.member[(_e6957 + 1u)];
                                    let _e6966 = global_6.member[(_e6957 + 2u)];
                                    let _e6969 = global_6.member[(_e6957 + 3u)];
                                    let _e6972 = global_6.member[(_e6957 + 4u)];
                                    let _e6975 = global_6.member[(_e6957 + 5u)];
                                    let _e6978 = global_6.member[(_e6957 + 6u)];
                                    let _e6981 = global_6.member[(_e6957 + 7u)];
                                    let _e6984 = global_6.member[(_e6957 + 8u)];
                                    let _e6987 = global_6.member[(_e6957 + 9u)];
                                    let _e6990 = global_6.member[(_e6957 + 13u)];
                                    let _e6993 = global_6.member[(_e6957 + 14u)];
                                    let _e6996 = global_6.member[(_e6957 + 15u)];
                                    let _e6999 = global_6.member[(_e6957 + 16u)];
                                    let _e7002 = global_6.member[(_e6957 + 18u)];
                                    let _e7005 = global_6.member[(_e6957 + 19u)];
                                    let _e7008 = global_6.member[(_e6957 + 20u)];
                                    let _e7011 = global_6.member[(_e6957 + 29u)];
                                    let _e7012 = bitcast<u32>(_e7011);
                                    let _e7025 = (((((_e312.member.x * _e312.member.x) + (_e312.member.y * _e312.member.y)) + (_e312.member.z * _e312.member.z)) - (2f * (((_e312.member.x * _e6960) + (_e312.member.y * _e6963)) + (_e312.member.z * _e6966)))) + _e6978);
                                    if (_e7025 > 0f) {
                                        let _e7034 = ((((_e312.member.x - _e6960) * _e6981) + ((_e312.member.y - _e6963) * _e6984)) + ((_e312.member.z - _e6966) * _e6987));
                                        let _e7036 = ((_e7034 * _e7034) - _e7025);
                                        if (_e7034 >= 0f) {
                                            phi_12574_ = vec3<f32>();
                                            phi_12575_ = true;
                                        } else {
                                            let _e7038 = (_e7036 <= 0f);
                                            if _e7038 {
                                                phi_12573_ = vec3<f32>();
                                            } else {
                                                let _e7041 = (-(_e7034) - sqrt(_e7036));
                                                phi_12573_ = vec3<f32>((_e312.member.x + (_e6981 * _e7041)), (_e312.member.y + (_e6984 * _e7041)), (_e312.member.z + (_e6987 * _e7041)));
                                            }
                                            let _e7050 = phi_12573_;
                                            phi_12574_ = _e7050;
                                            phi_12575_ = _e7038;
                                        }
                                        let _e7052 = phi_12574_;
                                        let _e7054 = phi_12575_;
                                        phi_12580_ = _e7052;
                                        phi_12581_ = select(true, false, _e7054);
                                    } else {
                                        phi_12580_ = vec3<f32>(_e312.member.x, _e312.member.y, _e312.member.z);
                                        phi_12581_ = true;
                                    }
                                    let _e7057 = phi_12580_;
                                    let _e7059 = phi_12581_;
                                    if _e7059 {
                                        let _e7061 = (_e7057.x - _e6960);
                                        let _e7063 = (_e7057.y - _e6963);
                                        let _e7065 = (_e7057.z - _e6966);
                                        let _e7071 = sqrt((((_e7061 * _e7061) + (_e7063 * _e7063)) + (_e7065 * _e7065)));
                                        let _e7072 = (1f / _e7071);
                                        let _e7090 = ((((((_e7057.x * _e7057.x) + (_e7057.y * _e7057.y)) + (_e7057.z * _e7057.z)) - (2f * (((_e7057.x * _e6960) + (_e7057.y * _e6963)) + (_e7057.z * _e6966)))) + _e6975) / (_e7071 + _e6969));
                                        if (_e7090 != _e7090) {
                                            phi_31175_ = true;
                                        } else {
                                            phi_31175_ = (0f >= _e7090);
                                        }
                                        let _e7094 = phi_31175_;
                                        let _e7098 = sqrt((select(_e7090, 0f, _e7094) / (_e6972 - _e6969)));
                                        if (_e7098 != _e7098) {
                                            phi_31190_ = true;
                                        } else {
                                            phi_31190_ = (1f <= _e7098);
                                        }
                                        let _e7102 = phi_31190_;
                                        let _e7104 = (select(_e7098, 1f, _e7102) * 63f);
                                        let _e7109 = ((((_e7061 * _e7072) * _e6981) + ((_e7063 * _e7072) * _e6984)) + ((_e7065 * _e7072) * _e6987));
                                        let _e7111 = select(_e7109, -1f, (_e7109 < -1f));
                                        let _e7115 = ((select(_e7111, 1f, (_e7111 > 1f)) + 1f) * 127.5f);
                                        let _e7120 = select(select(u32(_e7104), 0u, (_e7104 < 0f)), 4294967295u, (_e7104 > 4294967000f));
                                        let _e7122 = select(_e7120, 62u, (62u < _e7120));
                                        let _e7127 = select(select(u32(_e7115), 0u, (_e7115 < 0f)), 4294967295u, (_e7115 > 4294967000f));
                                        let _e7129 = select(_e7127, 254u, (254u < _e7127));
                                        let _e7131 = (_e7104 - f32(_e7122));
                                        let _e7133 = (_e7115 - f32(_e7129));
                                        let _e7137 = (_e7012 + (3u * ((_e7122 * 256u) + _e7129)));
                                        let _e7142 = (_e7012 + (3u * (((_e7122 + 1u) * 256u) + _e7129)));
                                        let _e7144 = global_6.member[_e7137];
                                        let _e7147 = global_6.member[(_e7137 + 1u)];
                                        let _e7150 = global_6.member[(_e7137 + 2u)];
                                        let _e7151 = (1f - _e7133);
                                        let _e7157 = global_6.member[(_e7137 + 3u)];
                                        let _e7160 = global_6.member[(_e7137 + 4u)];
                                        let _e7163 = global_6.member[(_e7137 + 5u)];
                                        let _e7170 = (1f - _e7131);
                                        let _e7175 = global_6.member[_e7142];
                                        let _e7178 = global_6.member[(_e7142 + 1u)];
                                        let _e7181 = global_6.member[(_e7142 + 2u)];
                                        let _e7187 = global_6.member[(_e7142 + 3u)];
                                        let _e7190 = global_6.member[(_e7142 + 4u)];
                                        let _e7193 = global_6.member[(_e7142 + 5u)];
                                        phi_12872_ = vec3<f32>(((((_e7144 * _e7151) + (_e7157 * _e7133)) * _e7170) + (((_e7175 * _e7151) + (_e7187 * _e7133)) * _e7131)), ((((_e7147 * _e7151) + (_e7160 * _e7133)) * _e7170) + (((_e7178 * _e7151) + (_e7190 * _e7133)) * _e7131)), ((((_e7150 * _e7151) + (_e7163 * _e7133)) * _e7170) + (((_e7181 * _e7151) + (_e7193 * _e7133)) * _e7131)));
                                    } else {
                                        phi_12872_ = vec3<f32>(0f, 0f, 0f);
                                    }
                                    let _e7208 = phi_12872_;
                                    let _e7215 = ((_e6999 * 1.1f) * _e7208.y);
                                    phi_12874_ = vec3<f32>(exp(-((((_e6990 * _e7208.x) + _e7215) + (_e7002 * _e7208.z)))), exp(-((((_e6993 * _e7208.x) + _e7215) + (_e7005 * _e7208.z)))), exp(-((((_e6996 * _e7208.x) + _e7215) + (_e7008 * _e7208.z)))));
                                }
                                let _e7234 = phi_12874_;
                                let _e7246 = sin((_e6940 * 0.5f));
                                let _e7249 = (((1f - (0.6f * (1f - sqrt(select(_e6944, 0f, _e6948))))) * 1.25f) / ((12.566371f * _e7246) * _e7246));
                                phi_12901_ = vec3<f32>(((_e6951.x * _e7234.x) * _e7249), ((_e6951.y * _e7234.y) * _e7249), ((_e6951.z * _e7234.z) * _e7249));
                            }
                            let _e7255 = phi_12901_;
                            phi_12915_ = vec3<f32>((_e312.member_3.x + ((_e312.member_2.x * _e7255.x) * _e6914)), (_e312.member_3.y + ((_e312.member_2.y * _e7255.y) * _e6914)), (_e312.member_3.z + ((_e312.member_2.z * _e7255.z) * _e6914)));
                        }
                        let _e7270 = phi_12915_;
                        phi_19299_ = type_68(type_35(_e312.member, _e312.member_1, vec4<f32>(_e312.member_2.x, _e312.member_2.y, _e312.member_2.z, f32(_e339)), vec4<f32>(_e7270.x, _e7270.y, _e7270.z, f32(_e958)), vec4<u32>(_e312.member_4.x, (_e312.member_4.y & 4294967294u), 0u, 0u)), type_39(vec4<f32>(_e312.member.x, _e312.member.y, _e312.member.z, -1f), vec4<f32>(_e312.member_1.x, _e312.member_1.y, _e312.member_1.z, 0f), vec4<f32>((_e312.member_2.x * _e6860), (_e312.member_2.y * _e6860), (_e312.member_2.z * _e6860), 0f)), type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_19300_ = type_68();
                        phi_19301_ = true;
                    } else {
                        if ((_e358 & 8u) == 0u) {
                            phi_34267_ = _e798;
                            phi_13452_ = _e333;
                            phi_13453_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        } else {
                            if (_e957.member > 100f) {
                                let _e971 = global_1.member.member_8[1u];
                                let _e973 = global_6.member[_e971];
                                let _e976 = global_6.member[(_e971 + 1u)];
                                let _e979 = global_6.member[(_e971 + 2u)];
                                let _e982 = global_6.member[(_e971 + 3u)];
                                let _e985 = global_6.member[(_e971 + 5u)];
                                let _e988 = global_6.member[(_e971 + 6u)];
                                let _e991 = global_6.member[(_e971 + 10u)];
                                let _e994 = global_6.member[(_e971 + 11u)];
                                let _e997 = global_6.member[(_e971 + 13u)];
                                let _e1000 = global_6.member[(_e971 + 14u)];
                                let _e1003 = global_6.member[(_e971 + 15u)];
                                let _e1006 = global_6.member[(_e971 + 16u)];
                                let _e1009 = global_6.member[(_e971 + 18u)];
                                let _e1012 = global_6.member[(_e971 + 19u)];
                                let _e1015 = global_6.member[(_e971 + 20u)];
                                let _e1023 = ((((_e312.member.x - _e973) * _e312.member_1.x) + ((_e312.member.y - _e976) * _e312.member_1.y)) + ((_e312.member.z - _e979) * _e312.member_1.z));
                                let _e1035 = ((((_e312.member.x * _e312.member.x) + (_e312.member.y * _e312.member.y)) + (_e312.member.z * _e312.member.z)) - (2f * (((_e312.member.x * _e973) + (_e312.member.y * _e976)) + (_e312.member.z * _e979))));
                                let _e1036 = (_e1035 + _e988);
                                let _e1037 = (_e1023 * _e1023);
                                let _e1038 = (_e1037 - _e1036);
                                if (_e1038 < 0f) {
                                    phi_25072_ = type_53(0u, type_53().member_1);
                                    phi_25073_ = type_53();
                                    phi_25074_ = true;
                                } else {
                                    if (_e1023 < 0f) {
                                        phi_25044_ = -(sqrt(_e1038));
                                    } else {
                                        phi_25044_ = sqrt(_e1038);
                                    }
                                    let _e1045 = phi_25044_;
                                    let _e1047 = -((_e1023 + _e1045));
                                    let _e1048 = (_e1047 == 0f);
                                    if _e1048 {
                                        phi_25067_ = type_53(1u, type_52(0f, 0f));
                                        phi_25068_ = type_53();
                                    } else {
                                        let _e1049 = (_e1036 / _e1047);
                                        if (_e1047 < _e1049) {
                                            phi_25056_ = type_52(_e1047, _e1049);
                                        } else {
                                            phi_25056_ = type_52(_e1049, _e1047);
                                        }
                                        let _e1054 = phi_25056_;
                                        phi_25067_ = type_53();
                                        phi_25068_ = type_53(1u, _e1054);
                                    }
                                    let _e1057 = phi_25067_;
                                    let _e1059 = phi_25068_;
                                    phi_25072_ = _e1057;
                                    phi_25073_ = _e1059;
                                    phi_25074_ = _e1048;
                                }
                                let _e1064 = phi_25072_;
                                let _e1066 = phi_25073_;
                                let _e1068 = phi_25074_;
                                if _e1068 {
                                    phi_25078_ = _e1064;
                                } else {
                                    phi_25078_ = _e1066;
                                }
                                let _e1070 = phi_25078_;
                                if (bitcast<i32>(_e1070.member) != 0i) {
                                    phi_24965_ = type_53(0u, _e1070.member_1);
                                } else {
                                    phi_24965_ = type_53(1u, type_53().member_1);
                                }
                                let _e1080 = phi_24965_;
                                if (bitcast<i32>(_e1080.member) != 0i) {
                                    phi_25012_ = type_53(0u, type_53().member_1);
                                } else {
                                    if (_e1080.member_1.member_1 <= 0f) {
                                        phi_25008_ = type_53(0u, type_53().member_1);
                                    } else {
                                        if (_e1080.member_1.member != _e1080.member_1.member) {
                                            phi_25116_ = true;
                                        } else {
                                            phi_25116_ = (0f >= _e1080.member_1.member);
                                        }
                                        let _e1092 = phi_25116_;
                                        let _e1093 = select(_e1080.member_1.member, 0f, _e1092);
                                        let _e1094 = (_e1035 + _e985);
                                        if (_e1094 > 0f) {
                                            let _e1096 = (_e1037 - _e1094);
                                            if (_e1096 < 0f) {
                                                phi_25224_ = type_53(0u, type_53().member_1);
                                                phi_25225_ = type_53();
                                                phi_25226_ = true;
                                            } else {
                                                if (_e1023 < 0f) {
                                                    phi_25196_ = -(sqrt(_e1096));
                                                } else {
                                                    phi_25196_ = sqrt(_e1096);
                                                }
                                                let _e1103 = phi_25196_;
                                                let _e1105 = -((_e1023 + _e1103));
                                                let _e1106 = (_e1105 == 0f);
                                                if _e1106 {
                                                    phi_25219_ = type_53(1u, type_52(0f, 0f));
                                                    phi_25220_ = type_53();
                                                } else {
                                                    let _e1107 = (_e1094 / _e1105);
                                                    if (_e1105 < _e1107) {
                                                        phi_25208_ = type_52(_e1105, _e1107);
                                                    } else {
                                                        phi_25208_ = type_52(_e1107, _e1105);
                                                    }
                                                    let _e1112 = phi_25208_;
                                                    phi_25219_ = type_53();
                                                    phi_25220_ = type_53(1u, _e1112);
                                                }
                                                let _e1115 = phi_25219_;
                                                let _e1117 = phi_25220_;
                                                phi_25224_ = _e1115;
                                                phi_25225_ = _e1117;
                                                phi_25226_ = _e1106;
                                            }
                                            let _e1122 = phi_25224_;
                                            let _e1124 = phi_25225_;
                                            let _e1126 = phi_25226_;
                                            if _e1126 {
                                                phi_25230_ = _e1122;
                                            } else {
                                                phi_25230_ = _e1124;
                                            }
                                            let _e1128 = phi_25230_;
                                            if (bitcast<i32>(_e1128.member) != 0i) {
                                                phi_25164_ = select(_e1128.member_1.member_1, _e1128.member_1.member, (_e1128.member_1.member >= 0f));
                                            } else {
                                                phi_25164_ = -1f;
                                            }
                                            let _e1139 = phi_25164_;
                                            if (_e1139 >= 0f) {
                                                if (_e1080.member_1.member_1 != _e1080.member_1.member_1) {
                                                    phi_25268_ = true;
                                                } else {
                                                    phi_25268_ = (_e1139 <= _e1080.member_1.member_1);
                                                }
                                                let _e1144 = phi_25268_;
                                                phi_24993_ = select(_e1080.member_1.member_1, _e1139, _e1144);
                                            } else {
                                                phi_24993_ = _e1080.member_1.member_1;
                                            }
                                            let _e1147 = phi_24993_;
                                            phi_24995_ = _e1147;
                                        } else {
                                            phi_24995_ = _e1080.member_1.member_1;
                                        }
                                        let _e1149 = phi_24995_;
                                        if (_e1149 <= _e1093) {
                                            phi_25004_ = type_53(0u, type_53().member_1);
                                        } else {
                                            phi_25004_ = type_53(1u, type_52(_e1093, _e1149));
                                        }
                                        let _e1157 = phi_25004_;
                                        phi_25008_ = _e1157;
                                    }
                                    let _e1162 = phi_25008_;
                                    phi_25012_ = _e1162;
                                }
                                let _e1167 = phi_25012_;
                                if (bitcast<i32>(_e1167.member) != 0i) {
                                    let _e1173 = (_e1167.member_1.member < _e957.member);
                                    if _e1173 {
                                        if (_e1167.member_1.member_1 != _e1167.member_1.member_1) {
                                            phi_25283_ = true;
                                        } else {
                                            phi_25283_ = (_e957.member <= _e1167.member_1.member_1);
                                        }
                                        let _e1179 = phi_25283_;
                                        let _e1180 = select(_e1167.member_1.member_1, _e957.member, _e1179);
                                        phi_13311_ = vec3<f32>(0f, 0f, 0f);
                                        phi_13314_ = 0u;
                                        loop {
                                            let _e1182 = phi_13311_;
                                            let _e1184 = phi_13314_;
                                            local_74 = _e1182;
                                            local_75 = _e1182;
                                            local_76 = _e1182;
                                            let _e1185 = (_e1184 < 32u);
                                            if _e1185 {
                                                let _e1186 = -(_e1023);
                                                if (_e1167.member_1.member <= _e1180) {
                                                } else {
                                                    phi_32044_ = true;
                                                    break;
                                                }
                                                let _e1189 = select(_e1186, _e1167.member_1.member, (_e1186 < _e1167.member_1.member));
                                                let _e1191 = select(_e1189, _e1180, (_e1189 > _e1180));
                                                let _e1192 = (_e1191 - _e1167.member_1.member);
                                                let _e1193 = (_e1180 - _e1191);
                                                let _e1194 = (_e1180 - _e1167.member_1.member);
                                                let _e1195 = (0.000000001f * _e1194);
                                                if (_e1192 <= _e1195) {
                                                    let _e1270 = (f32(_e1184) * 0.03125f);
                                                    let _e1273 = (f32((_e1184 + 1u)) * 0.03125f);
                                                    let _e1275 = ((_e1270 + _e1273) * 0.5f);
                                                    phi_13415_ = type_52((_e1167.member_1.member + ((_e1194 * _e1275) * _e1275)), (_e1194 * ((_e1273 * _e1273) - (_e1270 * _e1270))));
                                                } else {
                                                    if (_e1193 <= _e1195) {
                                                        let _e1253 = (f32((31u - _e1184)) * 0.03125f);
                                                        let _e1256 = (f32((32u - _e1184)) * 0.03125f);
                                                        let _e1258 = ((_e1253 + _e1256) * 0.5f);
                                                        phi_13414_ = type_52((_e1180 - ((_e1194 * _e1258) * _e1258)), (_e1194 * ((_e1256 * _e1256) - (_e1253 * _e1253))));
                                                    } else {
                                                        let _e1201 = round(((32f * _e1192) / (_e1192 + _e1193)));
                                                        let _e1206 = select(select(u32(_e1201), 0u, (_e1201 < 0f)), 4294967295u, (_e1201 > 4294967000f));
                                                        if (_e1206 < 1u) {
                                                            phi_13398_ = 1u;
                                                        } else {
                                                            phi_13398_ = select(_e1206, 31u, (_e1206 > 31u));
                                                        }
                                                        let _e1211 = phi_13398_;
                                                        if (_e1184 < _e1211) {
                                                            let _e1232 = ((_e1211 - 1u) - _e1184);
                                                            let _e1234 = f32(_e1211);
                                                            let _e1235 = (f32(_e1232) / _e1234);
                                                            let _e1238 = (f32((_e1232 + 1u)) / _e1234);
                                                            let _e1240 = ((_e1235 + _e1238) * 0.5f);
                                                            phi_13413_ = type_52((_e1191 - ((_e1192 * _e1240) * _e1240)), (_e1192 * ((_e1238 * _e1238) - (_e1235 * _e1235))));
                                                        } else {
                                                            let _e1214 = (_e1184 - _e1211);
                                                            let _e1216 = f32((32u - _e1211));
                                                            let _e1217 = (f32(_e1214) / _e1216);
                                                            let _e1220 = (f32((_e1214 + 1u)) / _e1216);
                                                            let _e1222 = ((_e1217 + _e1220) * 0.5f);
                                                            phi_13413_ = type_52((_e1191 + ((_e1193 * _e1222) * _e1222)), (_e1193 * ((_e1220 * _e1220) - (_e1217 * _e1217))));
                                                        }
                                                        let _e1250 = phi_13413_;
                                                        phi_13414_ = _e1250;
                                                    }
                                                    let _e1268 = phi_13414_;
                                                    phi_13415_ = _e1268;
                                                }
                                                let _e1285 = phi_13415_;
                                                let _e1291 = (_e312.member.x + (_e312.member_1.x * _e1285.member));
                                                let _e1292 = (_e312.member.y + (_e312.member_1.y * _e1285.member));
                                                let _e1293 = (_e312.member.z + (_e312.member_1.z * _e1285.member));
                                                let _e1307 = (_e1291 - _e973);
                                                let _e1308 = (_e1292 - _e976);
                                                let _e1309 = (_e1293 - _e979);
                                                let _e1317 = ((((((_e1291 * _e1291) + (_e1292 * _e1292)) + (_e1293 * _e1293)) - (2f * (((_e1291 * _e973) + (_e1292 * _e976)) + (_e1293 * _e979)))) + _e985) / (sqrt((((_e1307 * _e1307) + (_e1308 * _e1308)) + (_e1309 * _e1309))) + _e982));
                                                let _e1321 = (1f - (abs((_e1317 - 25000f)) * 0.00006666667f));
                                                if (_e1321 != _e1321) {
                                                    phi_25482_ = true;
                                                } else {
                                                    phi_25482_ = (0f >= _e1321);
                                                }
                                                let _e1325 = phi_25482_;
                                                let _e1327 = -(_e1317);
                                                phi_13312_ = (_e1182 + vec3<f32>((exp((_e1327 / _e991)) * _e1285.member_1), (exp((_e1327 / _e994)) * _e1285.member_1), (select(_e1321, 0f, _e1325) * _e1285.member_1)));
                                                phi_13315_ = (_e1184 + 1u);
                                            } else {
                                                phi_13312_ = vec3<f32>();
                                                phi_13315_ = u32();
                                            }
                                            let _e1339 = phi_13312_;
                                            let _e1341 = phi_13315_;
                                            phi_13311_ = _e1339;
                                            phi_13314_ = _e1341;
                                            phi_32044_ = _e798;
                                            if !(_e1185) {
                                                break;
                                            }
                                        }
                                        let _e1344 = phi_32044_;
                                        if _e1344 {
                                            break;
                                        }
                                        let _e1346 = local_74;
                                        let _e1353 = local_75;
                                        let _e1355 = ((_e1006 * 1.1f) * _e1353.y);
                                        let _e1360 = local_76;
                                        phi_34271_ = _e1344;
                                        phi_13440_ = vec3<f32>(exp(-((((_e997 * _e1346.x) + _e1355) + (_e1009 * _e1360.z)))), exp(-((((_e1000 * _e1346.x) + _e1355) + (_e1012 * _e1360.z)))), exp(-((((_e1003 * _e1346.x) + _e1355) + (_e1015 * _e1360.z)))));
                                    } else {
                                        phi_34271_ = _e798;
                                        phi_13440_ = vec3<f32>();
                                    }
                                    let _e1376 = phi_34271_;
                                    let _e1378 = phi_13440_;
                                    phi_34270_ = _e1376;
                                    phi_13442_ = _e1378;
                                    phi_13443_ = select(true, false, _e1173);
                                } else {
                                    phi_34270_ = _e798;
                                    phi_13442_ = vec3<f32>();
                                    phi_13443_ = true;
                                }
                                let _e1381 = phi_34270_;
                                let _e1383 = phi_13442_;
                                let _e1385 = phi_13443_;
                                phi_34268_ = _e1381;
                                phi_13450_ = (_e333 * select(_e1383, vec3<f32>(1f, 1f, 1f), vec3(_e1385)));
                                phi_13451_ = type_39(vec4<f32>(_e312.member.x, _e312.member.y, _e312.member.z, _e957.member), vec4<f32>(_e312.member_1.x, _e312.member_1.y, _e312.member_1.z, 0f), vec4<f32>(_e312.member_2.x, _e312.member_2.y, _e312.member_2.z, 0f));
                            } else {
                                phi_34268_ = _e798;
                                phi_13450_ = _e333;
                                phi_13451_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                            }
                            let _e1390 = phi_34268_;
                            let _e1392 = phi_13450_;
                            let _e1394 = phi_13451_;
                            phi_34267_ = _e1390;
                            phi_13452_ = _e1392;
                            phi_13453_ = _e1394;
                        }
                        let _e1396 = phi_34267_;
                        let _e1398 = phi_13452_;
                        let _e1400 = phi_13453_;
                        let _e1405 = (_e312.member.x + (_e312.member_1.x * _e957.member));
                        let _e1406 = (_e312.member.y + (_e312.member_1.y * _e957.member));
                        let _e1407 = (_e312.member.z + (_e312.member_1.z * _e957.member));
                        let _e1409 = global_2.member[_e957.member_1];
                        phi_13477_ = 0i;
                        phi_13480_ = _e1409.member_6.y;
                        loop {
                            let _e1413 = phi_13477_;
                            let _e1415 = phi_13480_;
                            local_77 = _e1415;
                            if (_e1413 < 4i) {
                                let _e1418 = global_4.member[_e1415];
                                let _e1420 = (_e1418.member != 6u);
                                if _e1420 {
                                    phi_13549_ = i32();
                                    phi_13550_ = u32();
                                } else {
                                    let _e1424 = (abs(_e1405) / _e1418.member_6.z);
                                    let _e1435 = (abs(_e1407) / _e1418.member_6.w);
                                    if ((select(select(u32(_e1424), 0u, (_e1424 < 0f)), 4294967295u, (_e1424 > 4294967000f)) % 2u) == 0u) {
                                        phi_13534_ = (_e1405 > 0f);
                                    } else {
                                        phi_13534_ = (_e1405 <= 0f);
                                    }
                                    let _e1446 = phi_13534_;
                                    if ((select(select(u32(_e1435), 0u, (_e1435 < 0f)), 4294967295u, (_e1435 > 4294967000f)) % 2u) == 0u) {
                                        phi_13540_ = (_e1407 > 0f);
                                    } else {
                                        phi_13540_ = (_e1407 <= 0f);
                                    }
                                    let _e1450 = phi_13540_;
                                    if (_e1446 != _e1450) {
                                        phi_13547_ = _e1418.member_1;
                                    } else {
                                        phi_13547_ = _e1418.member_2;
                                    }
                                    let _e1455 = phi_13547_;
                                    phi_13549_ = (_e1413 + 1i);
                                    phi_13550_ = _e1455;
                                }
                                let _e1458 = phi_13549_;
                                let _e1460 = phi_13550_;
                                phi_13478_ = _e1458;
                                phi_13481_ = _e1460;
                                phi_13552_ = select(true, false, _e1420);
                            } else {
                                phi_13478_ = i32();
                                phi_13481_ = u32();
                                phi_13552_ = false;
                            }
                            let _e1463 = phi_13478_;
                            let _e1465 = phi_13481_;
                            let _e1467 = phi_13552_;
                            phi_13477_ = _e1463;
                            phi_13480_ = _e1465;
                            if !(_e1467) {
                                break;
                            }
                        }
                        let _e1470 = local_77;
                        let _e1472 = global_4.member[_e1470];
                        let _e1474 = (_e1472.member == 4u);
                        if _e1474 {
                            if _e1474 {
                                if (_e1472.member_1 == 1u) {
                                    phi_13612_ = true;
                                } else {
                                    phi_13612_ = ((((-(_e312.member_1.x) * _e957.member_3.x) + (-(_e312.member_1.y) * _e957.member_3.y)) + (-(_e312.member_1.z) * _e957.member_3.z)) > 0f);
                                }
                                let _e6576 = phi_13612_;
                                if _e6576 {
                                    phi_13621_ = vec3<f32>(_e1472.member_4.x, _e1472.member_4.y, _e1472.member_4.z);
                                } else {
                                    phi_13621_ = vec3<f32>();
                                }
                                let _e6583 = phi_13621_;
                                phi_13623_ = _e6583;
                                phi_13624_ = select(true, false, _e6576);
                            } else {
                                phi_13623_ = vec3<f32>();
                                phi_13624_ = true;
                            }
                            let _e6586 = phi_13623_;
                            let _e6588 = phi_13624_;
                            let _e6590 = select(_e6586, vec3<f32>(0f, 0f, 0f), vec3(_e6588));
                            if _e352 {
                                phi_13794_ = 1f;
                            } else {
                                if ((_e358 & 1u) == 0u) {
                                    phi_13793_ = 1f;
                                } else {
                                    if (_e1409.member_6.z > 0u) {
                                        let _e6599 = global_1.member.member_2[1u];
                                        let _e6600 = (_e6599 + (_e1409.member_6.z - 1u));
                                        let _e6603 = global_6.member[(_e6600 + 1u)];
                                        let _e6605 = global_6.member[_e6600];
                                        if (_e1409.member_6.x == 1u) {
                                            let _e6682 = (_e1409.member.x - _e312.member.x);
                                            let _e6683 = (_e1409.member.y - _e312.member.y);
                                            let _e6684 = (_e1409.member.z - _e312.member.z);
                                            let _e6689 = (((_e6682 * _e6682) + (_e6683 * _e6683)) + (_e6684 * _e6684));
                                            let _e6690 = (_e1409.member.w * _e1409.member.w);
                                            if (_e6689 <= (_e6690 * 1.000001f)) {
                                                let _e6704 = (_e1405 - _e312.member.x);
                                                let _e6705 = (_e1406 - _e312.member.y);
                                                let _e6706 = (_e1407 - _e312.member.z);
                                                let _e6711 = (((_e6704 * _e6704) + (_e6705 * _e6705)) + (_e6706 * _e6706));
                                                phi_30888_ = (_e6711 / (((abs((((_e957.member_2.x * _e6704) + (_e957.member_2.y * _e6705)) + (_e957.member_2.z * _e6706))) / sqrt(_e6711)) * 12.566371f) * _e6690));
                                            } else {
                                                let _e6693 = (_e6690 / _e6689);
                                                if (_e6693 != _e6693) {
                                                    phi_30907_ = true;
                                                } else {
                                                    phi_30907_ = (1f <= _e6693);
                                                }
                                                let _e6697 = phi_30907_;
                                                let _e6698 = select(_e6693, 1f, _e6697);
                                                phi_30888_ = (0.15915494f / (_e6698 / (1f + sqrt((1f - _e6698)))));
                                            }
                                            let _e6727 = phi_30888_;
                                            phi_13789_ = _e6727;
                                        } else {
                                            let _e6611 = (_e1405 - _e312.member.x);
                                            let _e6612 = (_e1406 - _e312.member.y);
                                            let _e6613 = (_e1407 - _e312.member.z);
                                            let _e6618 = (((_e6611 * _e6611) + (_e6612 * _e6612)) + (_e6613 * _e6613));
                                            let _e6629 = (abs((((_e957.member_2.x * _e6611) + (_e957.member_2.y * _e6612)) + (_e957.member_2.z * _e6613))) / sqrt(_e6618));
                                            if (_e6629 <= 0f) {
                                                phi_13788_ = 0f;
                                            } else {
                                                if (_e1409.member_6.x == 0u) {
                                                    let _e6644 = (_e1409.member_1.x - _e1409.member.x);
                                                    let _e6645 = (_e1409.member_1.y - _e1409.member.y);
                                                    let _e6646 = (_e1409.member_1.z - _e1409.member.z);
                                                    let _e6651 = (_e1409.member_2.x - _e1409.member.x);
                                                    let _e6652 = (_e1409.member_2.y - _e1409.member.y);
                                                    let _e6653 = (_e1409.member_2.z - _e1409.member.z);
                                                    let _e6656 = ((_e6645 * _e6653) - (_e6652 * _e6646));
                                                    let _e6659 = ((_e6646 * _e6651) - (_e6653 * _e6644));
                                                    let _e6662 = ((_e6644 * _e6652) - (_e6651 * _e6645));
                                                    phi_13785_ = (sqrt((((_e6656 * _e6656) + (_e6659 * _e6659)) + (_e6662 * _e6662))) * 0.5f);
                                                } else {
                                                    phi_13785_ = ((12.566371f * _e1409.member.w) * _e1409.member.w);
                                                }
                                                let _e6671 = phi_13785_;
                                                phi_13788_ = (_e6618 / (_e6629 * _e6671));
                                            }
                                            let _e6675 = phi_13788_;
                                            phi_13789_ = _e6675;
                                        }
                                        let _e6729 = phi_13789_;
                                        let _e6730 = ((_e6603 - _e6605) * _e6729);
                                        if (_e312.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_30928_ = 1f;
                                        } else {
                                            let _e6732 = (_e312.member_1.w * _e312.member_1.w);
                                            let _e6734 = (_e6732 + (_e6730 * _e6730));
                                            if (_e6734 == 0f) {
                                                phi_30925_ = 0f;
                                            } else {
                                                phi_30925_ = (_e6732 / _e6734);
                                            }
                                            let _e6738 = phi_30925_;
                                            phi_30928_ = _e6738;
                                        }
                                        let _e6740 = phi_30928_;
                                        phi_13792_ = _e6740;
                                    } else {
                                        phi_13792_ = 1f;
                                    }
                                    let _e6742 = phi_13792_;
                                    phi_13793_ = _e6742;
                                }
                                let _e6744 = phi_13793_;
                                phi_13794_ = _e6744;
                            }
                            let _e6746 = phi_13794_;
                            phi_19293_ = type_68(type_35(_e312.member, _e312.member_1, vec4<f32>(_e1398.x, _e1398.y, _e1398.z, f32(_e339)), vec4<f32>((_e312.member_3.x + ((_e1398.x * _e6590.x) * _e6746)), (_e312.member_3.y + ((_e1398.y * _e6590.y) * _e6746)), (_e312.member_3.z + ((_e1398.z * _e6590.z) * _e6746)), f32(_e958)), vec4<u32>(_e312.member_4.x, (_e312.member_4.y & 4294967294u), 0u, 0u)), _e1400, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_19294_ = type_68();
                            phi_19295_ = true;
                        } else {
                            let _e1475 = -(_e312.member_1.x);
                            let _e1476 = -(_e312.member_1.y);
                            let _e1477 = -(_e312.member_1.z);
                            if ((((_e957.member_2.x * _e1475) + (_e957.member_2.y * _e1476)) + (_e957.member_2.z * _e1477)) < 0f) {
                                phi_13833_ = -(_e957.member_2);
                            } else {
                                phi_13833_ = _e957.member_2;
                            }
                            let _e1490 = phi_13833_;
                            if ((((_e957.member_3.x * _e1475) + (_e957.member_3.y * _e1476)) + (_e957.member_3.z * _e1477)) < 0f) {
                                phi_13850_ = -(_e957.member_3);
                            } else {
                                phi_13850_ = _e957.member_3;
                            }
                            let _e1503 = phi_13850_;
                            let _e1506 = global_1.member.member_4[2u];
                            let _e1507 = abs(_e1405);
                            let _e1508 = abs(_e1406);
                            let _e1509 = abs(_e1407);
                            let _e1511 = select(_e1508, _e1507, (_e1507 > _e1508));
                            let _e1515 = (0.00002f * (1f + select(_e1509, _e1511, (_e1511 > _e1509))));
                            if (_e1506 != _e1506) {
                                phi_25549_ = true;
                            } else {
                                phi_25549_ = (_e1515 >= _e1506);
                            }
                            let _e1519 = phi_25549_;
                            let _e1521 = (select(_e1506, _e1515, _e1519) * 2f);
                            let _e1528 = (_e1405 + (_e1490.x * _e1521));
                            let _e1529 = (_e1406 + (_e1490.y * _e1521));
                            let _e1530 = (_e1407 + (_e1490.z * _e1521));
                            let _e1531 = vec3<f32>(_e1528, _e1529, _e1530);
                            if ((_e312.member_4.y & 4u) == 0u) {
                                phi_13883_ = 0f;
                            } else {
                                let _e1536 = global_1.member.member_4[0u];
                                phi_13883_ = _e1536;
                            }
                            let _e1538 = phi_13883_;
                            let _e1541 = select(-1f, 1f, (_e1503.z >= 0f));
                            let _e1543 = (-1f / (_e1541 + _e1503.z));
                            let _e1547 = ((_e1503.x * _e1503.y) * _e1543);
                            let _e1551 = (1f + (((_e1541 * _e1503.x) * _e1503.x) * _e1543));
                            let _e1552 = (_e1541 * _e1547);
                            let _e1554 = (-(_e1541) * _e1503.x);
                            let _e1557 = (_e1541 + ((_e1503.y * _e1503.y) * _e1543));
                            let _e1558 = -(_e1503.y);
                            if (_e1472.member == 0u) {
                                phi_32910_ = vec3<f32>(0f, 0f, 0f);
                                phi_32718_ = 1.5f;
                                phi_32639_ = vec3<f32>(0f, 0f, 0f);
                                phi_32562_ = vec3<f32>(_e1472.member_4.x, _e1472.member_4.y, _e1472.member_4.z);
                                phi_32483_ = 0u;
                                phi_32401_ = 1f;
                                phi_32324_ = 0u;
                                phi_32315_ = _e312.member_4.x;
                                phi_14562_ = type_68();
                                phi_14563_ = true;
                            } else {
                                if (_e1472.member == 1u) {
                                    let _e2107 = (_e1472.member_6.y * _e1472.member_6.y);
                                    if (_e2107 != _e2107) {
                                        phi_26241_ = true;
                                    } else {
                                        phi_26241_ = (_e1538 >= _e2107);
                                    }
                                    let _e2111 = phi_26241_;
                                    let _e2112 = select(_e2107, _e1538, _e2111);
                                    let _e2113 = (_e2112 >= 0.001f);
                                    if _e2113 {
                                        phi_32967_ = vec3<f32>(_e1472.member_4.x, _e1472.member_4.y, _e1472.member_4.z);
                                        phi_32761_ = _e1472.member_6.x;
                                        phi_32614_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32302_ = _e312.member_4.x;
                                        phi_13958_ = vec3<f32>();
                                        phi_13959_ = true;
                                    } else {
                                        let _e2115 = ((_e312.member_4.x * 747796405u) + 2891336453u);
                                        let _e2123 = (((_e2115 >> bitcast<u32>((((_e2115 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2115) * 277803737u);
                                        let _e2126 = ((_e2123 >> bitcast<u32>(22i)) ^ _e2123);
                                        let _e2136 = abs((((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z)));
                                        let _e2139 = (1f / _e1472.member_6.x);
                                        let _e2143 = ((_e2139 * _e2139) * (1f - (_e2136 * _e2136)));
                                        if (_e2143 >= 1f) {
                                            phi_26281_ = 1f;
                                        } else {
                                            let _e2146 = sqrt((1f - _e2143));
                                            if (_e2136 != _e2136) {
                                                phi_26292_ = true;
                                            } else {
                                                phi_26292_ = (_e2146 <= _e2136);
                                            }
                                            let _e2150 = phi_26292_;
                                            let _e2155 = pow(((1f - _e1472.member_6.x) / (1f + _e1472.member_6.x)), 2f);
                                            phi_26281_ = (_e2155 + ((1f - _e2155) * pow((1f - select(_e2136, _e2146, _e2150)), 5f)));
                                        }
                                        let _e2162 = phi_26281_;
                                        let _e2163 = ((f32((_e2126 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e2162);
                                        if _e2163 {
                                            let _e2176 = (((_e1503.x * _e312.member_1.x) + (_e1503.y * _e312.member_1.y)) + (_e1503.z * _e312.member_1.z));
                                            phi_32615_ = vec3<f32>(0f, 0f, 0f);
                                            phi_13956_ = (_e328 - vec3<f32>(((_e1503.x * 2f) * _e2176), ((_e1503.y * 2f) * _e2176), ((_e1503.z * 2f) * _e2176)));
                                        } else {
                                            phi_32615_ = vec3<f32>(_e1472.member_4.x, _e1472.member_4.y, _e1472.member_4.z);
                                            phi_13956_ = vec3<f32>();
                                        }
                                        let _e2183 = phi_32615_;
                                        let _e2185 = phi_13956_;
                                        phi_32967_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32761_ = 1.5f;
                                        phi_32614_ = _e2183;
                                        phi_32302_ = _e2126;
                                        phi_13958_ = _e2185;
                                        phi_13959_ = select(true, false, _e2163);
                                    }
                                    let _e2195 = phi_32967_;
                                    let _e2197 = phi_32761_;
                                    let _e2199 = phi_32614_;
                                    let _e2201 = phi_32302_;
                                    let _e2203 = phi_13958_;
                                    let _e2205 = phi_13959_;
                                    phi_32913_ = _e2195;
                                    phi_32721_ = _e2197;
                                    phi_32642_ = vec3<f32>(0f, 0f, 0f);
                                    phi_32565_ = _e2199;
                                    phi_32486_ = select(0u, 1u, _e2113);
                                    phi_32404_ = select(1f, _e2112, _e2113);
                                    phi_32327_ = select(0u, 2u, _e2113);
                                    phi_32253_ = _e2201;
                                    phi_14432_ = _e1531;
                                    phi_14433_ = _e2203;
                                    phi_14434_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14435_ = _e2205;
                                } else {
                                    if (_e1472.member == 3u) {
                                        phi_32256_ = _e312.member_4.x;
                                        phi_14111_ = vec3<f32>();
                                        phi_14112_ = vec3<f32>();
                                        phi_14113_ = vec3<f32>();
                                        phi_14114_ = true;
                                        phi_14115_ = false;
                                    } else {
                                        let _e1562 = (_e1472.member == 5u);
                                        if _e1562 {
                                            phi_32257_ = _e312.member_4.x;
                                            phi_14107_ = vec3<f32>();
                                            phi_14108_ = vec3<f32>();
                                            phi_14109_ = vec3<f32>();
                                            phi_14110_ = false;
                                        } else {
                                            let _e1563 = (_e1472.member == 2u);
                                            if _e1563 {
                                                let _e1568 = (((_e312.member_1.x * _e957.member_3.x) + (_e312.member_1.y * _e957.member_3.y)) + (_e312.member_1.z * _e957.member_3.z));
                                                let _e1569 = (_e1568 > 0f);
                                                if _e1569 {
                                                    phi_13992_ = -(_e957.member_3);
                                                } else {
                                                    phi_13992_ = _e957.member_3;
                                                }
                                                let _e1572 = phi_13992_;
                                                if _e1569 {
                                                    phi_13998_ = _e1472.member_6.x;
                                                } else {
                                                    phi_13998_ = (1f / _e1472.member_6.x);
                                                }
                                                let _e1577 = phi_13998_;
                                                let _e1578 = abs(_e1568);
                                                let _e1579 = (_e1577 * _e1577);
                                                let _e1582 = (_e1579 * (1f - (_e1578 * _e1578)));
                                                if (_e1582 >= 1f) {
                                                    phi_25611_ = 1f;
                                                } else {
                                                    let _e1585 = sqrt((1f - _e1582));
                                                    if (_e1578 != _e1578) {
                                                        phi_25622_ = true;
                                                    } else {
                                                        phi_25622_ = (_e1585 <= _e1578);
                                                    }
                                                    let _e1589 = phi_25622_;
                                                    let _e1594 = pow(((1f - _e1472.member_6.x) / (1f + _e1472.member_6.x)), 2f);
                                                    phi_25611_ = (_e1594 + ((1f - _e1594) * pow((1f - select(_e1578, _e1585, _e1589)), 5f)));
                                                }
                                                let _e1601 = phi_25611_;
                                                let _e1609 = (((_e312.member_1.x * _e1572.x) + (_e312.member_1.y * _e1572.y)) + (_e312.member_1.z * _e1572.z));
                                                let _e1613 = (1f - (_e1579 * (1f - (_e1609 * _e1609))));
                                                if (_e1613 > 0f) {
                                                    let _e1616 = ((_e312.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1624 = (((_e1616 >> bitcast<u32>((((_e1616 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1616) * 277803737u);
                                                    let _e1627 = ((_e1624 >> bitcast<u32>(22i)) ^ _e1624);
                                                    let _e1632 = ((f32((_e1627 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1601);
                                                    if _e1632 {
                                                        let _e1642 = sqrt(_e1613);
                                                        let _e1646 = (((_e312.member_1.x - (_e1572.x * _e1609)) * _e1577) - (_e1572.x * _e1642));
                                                        let _e1647 = (((_e312.member_1.y - (_e1572.y * _e1609)) * _e1577) - (_e1572.y * _e1642));
                                                        let _e1648 = (((_e312.member_1.z - (_e1572.z * _e1609)) * _e1577) - (_e1572.z * _e1642));
                                                        let _e1655 = (1f / sqrt((((_e1646 * _e1646) + (_e1647 * _e1647)) + (_e1648 * _e1648))));
                                                        phi_14079_ = vec3<f32>((_e1405 - (_e1572.x * _e1521)), (_e1406 - (_e1572.y * _e1521)), (_e1407 - (_e1572.z * _e1521)));
                                                        phi_14080_ = vec3<f32>((_e1646 * _e1655), (_e1647 * _e1655), (_e1648 * _e1655));
                                                        phi_14081_ = vec3<f32>(_e1472.member_4.x, _e1472.member_4.y, _e1472.member_4.z);
                                                    } else {
                                                        phi_14079_ = vec3<f32>();
                                                        phi_14080_ = vec3<f32>();
                                                        phi_14081_ = vec3<f32>();
                                                    }
                                                    let _e1673 = phi_14079_;
                                                    let _e1675 = phi_14080_;
                                                    let _e1677 = phi_14081_;
                                                    phi_32260_ = _e1627;
                                                    phi_14083_ = _e1673;
                                                    phi_14084_ = _e1675;
                                                    phi_14085_ = _e1677;
                                                    phi_14086_ = select(true, false, _e1632);
                                                } else {
                                                    phi_32260_ = _e312.member_4.x;
                                                    phi_14083_ = vec3<f32>();
                                                    phi_14084_ = vec3<f32>();
                                                    phi_14085_ = vec3<f32>();
                                                    phi_14086_ = true;
                                                }
                                                let _e1680 = phi_32260_;
                                                let _e1682 = phi_14083_;
                                                let _e1684 = phi_14084_;
                                                let _e1686 = phi_14085_;
                                                let _e1688 = phi_14086_;
                                                if _e1688 {
                                                    let _e1696 = (((_e957.member_3.x * _e312.member_1.x) + (_e957.member_3.y * _e312.member_1.y)) + (_e957.member_3.z * _e312.member_1.z));
                                                    phi_14101_ = vec3<f32>((_e1405 + (_e1572.x * _e1521)), (_e1406 + (_e1572.y * _e1521)), (_e1407 + (_e1572.z * _e1521)));
                                                    phi_14102_ = (_e328 - vec3<f32>(((_e957.member_3.x * 2f) * _e1696), ((_e957.member_3.y * 2f) * _e1696), ((_e957.member_3.z * 2f) * _e1696)));
                                                } else {
                                                    phi_14101_ = _e1682;
                                                    phi_14102_ = _e1684;
                                                }
                                                let _e1710 = phi_14101_;
                                                let _e1712 = phi_14102_;
                                                phi_32258_ = _e1680;
                                                phi_14104_ = _e1710;
                                                phi_14105_ = _e1712;
                                                phi_14106_ = select(_e1686, vec3<f32>(1f, 1f, 1f), vec3(_e1688));
                                            } else {
                                                phi_32258_ = _e312.member_4.x;
                                                phi_14104_ = vec3<f32>();
                                                phi_14105_ = vec3<f32>();
                                                phi_14106_ = vec3<f32>();
                                            }
                                            let _e1716 = phi_32258_;
                                            let _e1718 = phi_14104_;
                                            let _e1720 = phi_14105_;
                                            let _e1722 = phi_14106_;
                                            phi_32257_ = _e1716;
                                            phi_14107_ = _e1718;
                                            phi_14108_ = _e1720;
                                            phi_14109_ = _e1722;
                                            phi_14110_ = _e1563;
                                        }
                                        let _e1724 = phi_32257_;
                                        let _e1726 = phi_14107_;
                                        let _e1728 = phi_14108_;
                                        let _e1730 = phi_14109_;
                                        let _e1732 = phi_14110_;
                                        phi_32256_ = _e1724;
                                        phi_14111_ = _e1726;
                                        phi_14112_ = _e1728;
                                        phi_14113_ = _e1730;
                                        phi_14114_ = _e1562;
                                        phi_14115_ = _e1732;
                                    }
                                    let _e1734 = phi_32256_;
                                    let _e1736 = phi_14111_;
                                    let _e1738 = phi_14112_;
                                    let _e1740 = phi_14113_;
                                    let _e1742 = phi_14114_;
                                    let _e1744 = phi_14115_;
                                    if _e1742 {
                                        let _e1747 = (_e1472.member_6.y * _e1472.member_6.y);
                                        if (_e1747 != _e1747) {
                                            phi_25695_ = true;
                                        } else {
                                            phi_25695_ = (_e1538 >= _e1747);
                                        }
                                        let _e1751 = phi_25695_;
                                        let _e1752 = select(_e1747, _e1538, _e1751);
                                        let _e1764 = select(2u, 0u, (_e1472.member == 5u));
                                        let _e1765 = (_e1752 < 0.001f);
                                        if _e1765 {
                                            let _e1773 = (((_e1503.x * _e312.member_1.x) + (_e1503.y * _e312.member_1.y)) + (_e1503.z * _e312.member_1.z));
                                            let _e1784 = abs((((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z)));
                                            if (_e1764 == 0u) {
                                                let _e2066 = (1f - _e1784);
                                                if (_e2066 != _e2066) {
                                                    phi_26226_ = true;
                                                } else {
                                                    phi_26226_ = (0f >= _e2066);
                                                }
                                                let _e2070 = phi_26226_;
                                                let _e2072 = pow(select(_e2066, 0f, _e2070), 5f);
                                                let _e2073 = (1f - _e2072);
                                                phi_14418_ = vec3<f32>(((_e1472.member_4.x * _e2073) + _e2072), ((_e1472.member_4.y * _e2073) + _e2072), ((_e1472.member_4.z * _e2073) + _e2072));
                                            } else {
                                                if (_e1764 == 1u) {
                                                    let _e2048 = (0.44444448f * (1f - (_e1784 * _e1784)));
                                                    if (_e2048 >= 1f) {
                                                        phi_26187_ = 1f;
                                                    } else {
                                                        let _e2051 = sqrt((1f - _e2048));
                                                        if (_e1784 != _e1784) {
                                                            phi_26198_ = true;
                                                        } else {
                                                            phi_26198_ = (_e2051 <= _e1784);
                                                        }
                                                        let _e2055 = phi_26198_;
                                                        phi_26187_ = (0.040000003f + (0.96f * pow((1f - select(_e1784, _e2051, _e2055)), 5f)));
                                                    }
                                                    let _e2062 = phi_26187_;
                                                    phi_14417_ = vec3(_e2062);
                                                } else {
                                                    let _e1788 = select(_e1784, 0f, (_e1784 < 0f));
                                                    let _e1790 = select(_e1788, 1f, (_e1788 > 1f));
                                                    let _e1792 = (1f - (_e1790 * _e1790));
                                                    let _e1799 = ((_e1472.member_4.x * _e1472.member_4.x) - (_e1472.member_5.x * _e1472.member_5.x));
                                                    let _e1800 = ((_e1472.member_4.y * _e1472.member_4.y) - (_e1472.member_5.y * _e1472.member_5.y));
                                                    let _e1801 = ((_e1472.member_4.z * _e1472.member_4.z) - (_e1472.member_5.z * _e1472.member_5.z));
                                                    let _e1803 = (_e1472.member_4.x * (_e1472.member_5.x + _e1472.member_5.x));
                                                    let _e1805 = (_e1472.member_4.y * (_e1472.member_5.y + _e1472.member_5.y));
                                                    let _e1807 = (_e1472.member_4.z * (_e1472.member_5.z + _e1472.member_5.z));
                                                    let _e1814 = ((_e1799 * _e1799) + (_e1803 * _e1803));
                                                    let _e1815 = ((_e1800 * _e1800) + (_e1805 * _e1805));
                                                    let _e1816 = ((_e1801 * _e1801) + (_e1807 * _e1807));
                                                    let _e1821 = ((_e1792 * _e1799) / _e1814);
                                                    let _e1822 = ((_e1792 * _e1800) / _e1815);
                                                    let _e1823 = ((_e1792 * _e1801) / _e1816);
                                                    let _e1824 = (1f - _e1821);
                                                    let _e1825 = (1f - _e1822);
                                                    let _e1826 = (1f - _e1823);
                                                    let _e1828 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1821, _e1822, _e1823));
                                                    let _e1829 = (_e1792 * _e1803);
                                                    let _e1830 = (_e1792 * _e1805);
                                                    let _e1831 = (_e1792 * _e1807);
                                                    let _e1836 = (vec3<f32>(_e1829, _e1830, _e1831) / vec3<f32>(_e1814, _e1815, _e1816));
                                                    let _e1849 = ((_e1828.x * _e1828.x) + (_e1836.x * _e1836.x));
                                                    let _e1850 = ((_e1828.y * _e1828.y) + (_e1836.y * _e1836.y));
                                                    let _e1851 = ((_e1828.z * _e1828.z) + (_e1836.z * _e1836.z));
                                                    if (_e1849 != _e1849) {
                                                        phi_25857_ = true;
                                                    } else {
                                                        phi_25857_ = (0f >= _e1849);
                                                    }
                                                    let _e1855 = phi_25857_;
                                                    let _e1857 = sqrt(select(_e1849, 0f, _e1855));
                                                    if (_e1850 != _e1850) {
                                                        phi_25872_ = true;
                                                    } else {
                                                        phi_25872_ = (0f >= _e1850);
                                                    }
                                                    let _e1861 = phi_25872_;
                                                    let _e1863 = sqrt(select(_e1850, 0f, _e1861));
                                                    if (_e1851 != _e1851) {
                                                        phi_25887_ = true;
                                                    } else {
                                                        phi_25887_ = (0f >= _e1851);
                                                    }
                                                    let _e1867 = phi_25887_;
                                                    let _e1869 = sqrt(select(_e1851, 0f, _e1867));
                                                    let _e1873 = ((_e1857 + _e1824) * 0.5f);
                                                    let _e1874 = ((_e1863 + _e1825) * 0.5f);
                                                    let _e1875 = ((_e1869 + _e1826) * 0.5f);
                                                    if (_e1873 != _e1873) {
                                                        phi_25914_ = true;
                                                    } else {
                                                        phi_25914_ = (0f >= _e1873);
                                                    }
                                                    let _e1879 = phi_25914_;
                                                    let _e1881 = sqrt(select(_e1873, 0f, _e1879));
                                                    if (_e1874 != _e1874) {
                                                        phi_25929_ = true;
                                                    } else {
                                                        phi_25929_ = (0f >= _e1874);
                                                    }
                                                    let _e1885 = phi_25929_;
                                                    let _e1887 = sqrt(select(_e1874, 0f, _e1885));
                                                    if (_e1875 != _e1875) {
                                                        phi_25944_ = true;
                                                    } else {
                                                        phi_25944_ = (0f >= _e1875);
                                                    }
                                                    let _e1891 = phi_25944_;
                                                    let _e1893 = sqrt(select(_e1875, 0f, _e1891));
                                                    let _e1894 = vec3<f32>(_e1881, _e1887, _e1893);
                                                    let _e1898 = ((_e1857 - _e1824) * 0.5f);
                                                    let _e1899 = ((_e1863 - _e1825) * 0.5f);
                                                    let _e1900 = ((_e1869 - _e1826) * 0.5f);
                                                    if (_e1898 != _e1898) {
                                                        phi_25971_ = true;
                                                    } else {
                                                        phi_25971_ = (0f >= _e1898);
                                                    }
                                                    let _e1904 = phi_25971_;
                                                    let _e1906 = sqrt(select(_e1898, 0f, _e1904));
                                                    if (_e1899 != _e1899) {
                                                        phi_25986_ = true;
                                                    } else {
                                                        phi_25986_ = (0f >= _e1899);
                                                    }
                                                    let _e1910 = phi_25986_;
                                                    let _e1912 = sqrt(select(_e1899, 0f, _e1910));
                                                    if (_e1900 != _e1900) {
                                                        phi_26001_ = true;
                                                    } else {
                                                        phi_26001_ = (0f >= _e1900);
                                                    }
                                                    let _e1916 = phi_26001_;
                                                    let _e1918 = sqrt(select(_e1900, 0f, _e1916));
                                                    if ((_e1829 / _e1814) < 0f) {
                                                        phi_14294_ = -(_e1906);
                                                    } else {
                                                        phi_14294_ = _e1906;
                                                    }
                                                    let _e1922 = phi_14294_;
                                                    if ((_e1830 / _e1815) < 0f) {
                                                        phi_14302_ = -(_e1912);
                                                    } else {
                                                        phi_14302_ = _e1912;
                                                    }
                                                    let _e1926 = phi_14302_;
                                                    if ((_e1831 / _e1816) < 0f) {
                                                        phi_14310_ = -(_e1918);
                                                    } else {
                                                        phi_14310_ = _e1918;
                                                    }
                                                    let _e1930 = phi_14310_;
                                                    let _e1934 = (_e1472.member_5.x * _e1790);
                                                    let _e1935 = (_e1472.member_5.y * _e1790);
                                                    let _e1936 = (_e1472.member_5.z * _e1790);
                                                    let _e1946 = vec3<f32>(((_e1472.member_4.x * _e1881) - (_e1472.member_5.x * _e1922)), ((_e1472.member_4.y * _e1887) - (_e1472.member_5.y * _e1926)), ((_e1472.member_4.z * _e1893) - (_e1472.member_5.z * _e1930)));
                                                    let _e1953 = ((_e1472.member_4.x * _e1922) + (_e1472.member_5.x * _e1881));
                                                    let _e1954 = ((_e1472.member_4.y * _e1926) + (_e1472.member_5.y * _e1887));
                                                    let _e1955 = ((_e1472.member_4.z * _e1930) + (_e1472.member_5.z * _e1893));
                                                    let _e1957 = vec3<f32>((_e1472.member_4.x * _e1790), (_e1472.member_4.y * _e1790), (_e1472.member_4.z * _e1790));
                                                    let _e1958 = (_e1957 - _e1894);
                                                    let _e1959 = (_e1934 - _e1922);
                                                    let _e1960 = (_e1935 - _e1926);
                                                    let _e1961 = (_e1936 - _e1930);
                                                    let _e1974 = (_e1957 + _e1894);
                                                    let _e1975 = (_e1934 + _e1922);
                                                    let _e1976 = (_e1935 + _e1926);
                                                    let _e1977 = (_e1936 + _e1930);
                                                    let _e1987 = ((_e1974.x * _e1974.x) + (_e1975 * _e1975));
                                                    let _e1988 = ((_e1974.y * _e1974.y) + (_e1976 * _e1976));
                                                    let _e1989 = ((_e1974.z * _e1974.z) + (_e1977 * _e1977));
                                                    let _e1999 = vec3(_e1790);
                                                    let _e2000 = (_e1999 - _e1946);
                                                    let _e2001 = -(vec3<f32>(_e1953, _e1954, _e1955));
                                                    let _e2017 = (_e1999 + _e1946);
                                                    let _e2027 = ((_e2017.x * _e2017.x) + (_e1953 * _e1953));
                                                    let _e2028 = ((_e2017.y * _e2017.y) + (_e1954 * _e1954));
                                                    let _e2029 = ((_e2017.z * _e2017.z) + (_e1955 * _e1955));
                                                    phi_14417_ = vec3<f32>((((((_e1958.x * _e1958.x) + (_e1959 * _e1959)) / select(0.000000000000000000000000000001f, _e1987, (_e1987 > 0.000000000000000000000000000001f))) + (((_e2000.x * _e2000.x) + (_e2001.x * _e2001.x)) / select(0.000000000000000000000000000001f, _e2027, (_e2027 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1958.y * _e1958.y) + (_e1960 * _e1960)) / select(0.000000000000000000000000000001f, _e1988, (_e1988 > 0.000000000000000000000000000001f))) + (((_e2000.y * _e2000.y) + (_e2001.y * _e2001.y)) / select(0.000000000000000000000000000001f, _e2028, (_e2028 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1958.z * _e1958.z) + (_e1961 * _e1961)) / select(0.000000000000000000000000000001f, _e1989, (_e1989 > 0.000000000000000000000000000001f))) + (((_e2000.z * _e2000.z) + (_e2001.z * _e2001.z)) / select(0.000000000000000000000000000001f, _e2029, (_e2029 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e2065 = phi_14417_;
                                                phi_14418_ = _e2065;
                                            }
                                            let _e2082 = phi_14418_;
                                            phi_14419_ = (_e328 - vec3<f32>(((_e1503.x * 2f) * _e1773), ((_e1503.y * 2f) * _e1773), ((_e1503.z * 2f) * _e1773)));
                                            phi_14420_ = _e2082;
                                        } else {
                                            phi_14419_ = vec3<f32>();
                                            phi_14420_ = vec3<f32>();
                                        }
                                        let _e2084 = phi_14419_;
                                        let _e2086 = phi_14420_;
                                        phi_32644_ = vec3<f32>(_e1472.member_5.x, _e1472.member_5.y, _e1472.member_5.z);
                                        phi_32567_ = vec3<f32>(_e1472.member_4.x, _e1472.member_4.y, _e1472.member_4.z);
                                        phi_32488_ = _e1764;
                                        phi_32406_ = _e1752;
                                        phi_14422_ = _e2084;
                                        phi_14423_ = _e2086;
                                        phi_14424_ = _e1765;
                                    } else {
                                        phi_32644_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32567_ = vec3<f32>(0f, 0f, 0f);
                                        phi_32488_ = 0u;
                                        phi_32406_ = 1f;
                                        phi_14422_ = _e1738;
                                        phi_14423_ = _e1740;
                                        phi_14424_ = _e1744;
                                    }
                                    let _e2088 = phi_32644_;
                                    let _e2090 = phi_32567_;
                                    let _e2092 = phi_32488_;
                                    let _e2094 = phi_32406_;
                                    let _e2096 = phi_14422_;
                                    let _e2098 = phi_14423_;
                                    let _e2100 = phi_14424_;
                                    phi_32913_ = vec3<f32>(0f, 0f, 0f);
                                    phi_32721_ = 1.5f;
                                    phi_32642_ = _e2088;
                                    phi_32565_ = _e2090;
                                    phi_32486_ = _e2092;
                                    phi_32404_ = _e2094;
                                    phi_32327_ = select(0u, 1u, _e1742);
                                    phi_32253_ = _e1734;
                                    phi_14432_ = select(_e1736, _e1531, vec3(_e1742));
                                    phi_14433_ = _e2096;
                                    phi_14434_ = _e2098;
                                    phi_14435_ = select(true, false, _e2100);
                                }
                                let _e2210 = phi_32913_;
                                let _e2212 = phi_32721_;
                                let _e2214 = phi_32642_;
                                let _e2216 = phi_32565_;
                                let _e2218 = phi_32486_;
                                let _e2220 = phi_32404_;
                                let _e2222 = phi_32327_;
                                let _e2224 = phi_32253_;
                                let _e2226 = phi_14432_;
                                let _e2228 = phi_14433_;
                                let _e2230 = phi_14434_;
                                let _e2232 = phi_14435_;
                                let _e2233 = select(true, false, _e2232);
                                if _e2233 {
                                    if (_e339 >= _e355) {
                                        phi_32312_ = _e2224;
                                        phi_14551_ = _e1398;
                                        phi_14552_ = type_68();
                                        phi_14553_ = true;
                                    } else {
                                        let _e2246 = global_1.member.member_4[1u];
                                        if (sqrt((((_e2230.x * _e2230.x) + (_e2230.y * _e2230.y)) + (_e2230.z * _e2230.z))) <= _e2246) {
                                            phi_32307_ = _e2224;
                                            phi_14515_ = _e1398;
                                            phi_14516_ = false;
                                        } else {
                                            let _e2250 = global_1.member.member_7[1u];
                                            if (_e2250 == 1u) {
                                                let _e2255 = global_1.member.member_4[3u];
                                                if (f32(_e339) >= _e2255) {
                                                    let _e2258 = select(_e2230.y, _e2230.x, (_e2230.x > _e2230.y));
                                                    let _e2260 = select(_e2230.z, _e2258, (_e2258 > _e2230.z));
                                                    if (_e2260 != _e2260) {
                                                        phi_26358_ = true;
                                                    } else {
                                                        phi_26358_ = (1f <= _e2260);
                                                    }
                                                    let _e2264 = phi_26358_;
                                                    let _e2265 = select(_e2260, 1f, _e2264);
                                                    let _e2267 = ((_e2224 * 747796405u) + 2891336453u);
                                                    let _e2275 = (((_e2267 >> bitcast<u32>((((_e2267 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2267) * 277803737u);
                                                    let _e2278 = ((_e2275 >> bitcast<u32>(22i)) ^ _e2275);
                                                    let _e2283 = ((f32((_e2278 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e2265);
                                                    if _e2283 {
                                                        phi_14504_ = vec3<f32>();
                                                    } else {
                                                        phi_14504_ = vec3<f32>((_e1398.x / _e2265), (_e1398.y / _e2265), (_e1398.z / _e2265));
                                                    }
                                                    let _e2292 = phi_14504_;
                                                    phi_32310_ = _e2278;
                                                    phi_14506_ = _e2292;
                                                    phi_14507_ = select(true, false, _e2283);
                                                } else {
                                                    phi_32310_ = _e2224;
                                                    phi_14506_ = _e1398;
                                                    phi_14507_ = true;
                                                }
                                                let _e2295 = phi_32310_;
                                                let _e2297 = phi_14506_;
                                                let _e2299 = phi_14507_;
                                                phi_32309_ = _e2295;
                                                phi_14508_ = _e2297;
                                                phi_14509_ = _e2299;
                                            } else {
                                                phi_32309_ = _e2224;
                                                phi_14508_ = _e1398;
                                                phi_14509_ = true;
                                            }
                                            let _e2301 = phi_32309_;
                                            let _e2303 = phi_14508_;
                                            let _e2305 = phi_14509_;
                                            phi_32307_ = _e2301;
                                            phi_14515_ = select(_e1398, _e2303, vec3(_e2305));
                                            phi_14516_ = select(false, true, _e2305);
                                        }
                                        let _e2310 = phi_32307_;
                                        let _e2312 = phi_14515_;
                                        let _e2314 = phi_14516_;
                                        if _e2314 {
                                            let _e2315 = (_e2312 * _e2230);
                                            let _e2326 = (_e2228 * (1f / sqrt((((_e2228.x * _e2228.x) + (_e2228.y * _e2228.y)) + (_e2228.z * _e2228.z)))));
                                            phi_14549_ = type_68(type_35(vec4<f32>(_e2226.x, _e2226.y, _e2226.z, _e312.member.w), vec4<f32>(_e2326.x, _e2326.y, _e2326.z, _e312.member_1.w), vec4<f32>(_e2315.x, _e2315.y, _e2315.z, f32((_e339 + 1u))), vec4<f32>(_e312.member_3.x, _e312.member_3.y, _e312.member_3.z, f32(_e958)), vec4<u32>(_e2310, (_e312.member_4.y | 2u), 0u, 0u)), _e1400, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14549_ = type_68();
                                        }
                                        let _e2349 = phi_14549_;
                                        phi_32312_ = _e2310;
                                        phi_14551_ = _e2312;
                                        phi_14552_ = _e2349;
                                        phi_14553_ = select(true, false, _e2314);
                                    }
                                    let _e2352 = phi_32312_;
                                    let _e2354 = phi_14551_;
                                    let _e2356 = phi_14552_;
                                    let _e2358 = phi_14553_;
                                    if _e2358 {
                                        phi_14559_ = type_68(type_35(_e312.member, _e312.member_1, vec4<f32>(_e2354.x, _e2354.y, _e2354.z, f32(_e339)), vec4<f32>(_e312.member_3.x, _e312.member_3.y, _e312.member_3.z, f32(_e958)), vec4<u32>(_e2352, (_e312.member_4.y & 4294967294u), 0u, 0u)), _e1400, type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14559_ = _e2356;
                                    }
                                    let _e2371 = phi_14559_;
                                    phi_32316_ = _e2352;
                                    phi_14560_ = _e2371;
                                } else {
                                    phi_32316_ = _e2224;
                                    phi_14560_ = type_68();
                                }
                                let _e2373 = phi_32316_;
                                let _e2375 = phi_14560_;
                                phi_32910_ = _e2210;
                                phi_32718_ = _e2212;
                                phi_32639_ = _e2214;
                                phi_32562_ = _e2216;
                                phi_32483_ = _e2218;
                                phi_32401_ = _e2220;
                                phi_32324_ = _e2222;
                                phi_32315_ = _e2373;
                                phi_14562_ = _e2375;
                                phi_14563_ = select(true, false, _e2233);
                            }
                            let _e2383 = phi_32910_;
                            let _e2385 = phi_32718_;
                            let _e2387 = phi_32639_;
                            let _e2389 = phi_32562_;
                            let _e2391 = phi_32483_;
                            let _e2393 = phi_32401_;
                            let _e2395 = phi_32324_;
                            let _e2397 = phi_32315_;
                            let _e2399 = phi_14562_;
                            let _e2401 = phi_14563_;
                            if _e2401 {
                                let _e2403 = ((_e312.member_4.y | 4u) & 4294967293u);
                                let _e2406 = global_1.member.member_1[3u];
                                phi_34247_ = _e1396;
                                phi_32314_ = _e2397;
                                phi_14574_ = _e343;
                                phi_14577_ = type_39(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14579_ = 0u;
                                phi_14581_ = _e958;
                                loop {
                                    let _e2408 = phi_34247_;
                                    let _e2410 = phi_32314_;
                                    let _e2412 = phi_14574_;
                                    let _e2414 = phi_14577_;
                                    let _e2416 = phi_14579_;
                                    let _e2418 = phi_14581_;
                                    local_85 = _e2418;
                                    local_86 = _e2412;
                                    local_87 = _e2412;
                                    local_88 = _e2412;
                                    local_89 = _e2414;
                                    local_90 = _e2418;
                                    local_91 = _e2412;
                                    local_92 = _e2412;
                                    local_93 = _e2412;
                                    local_94 = _e2414;
                                    local_95 = _e2418;
                                    local_96 = _e2412;
                                    local_97 = _e2412;
                                    local_98 = _e2412;
                                    local_99 = _e2414;
                                    local_100 = _e2418;
                                    local_101 = _e2412;
                                    local_102 = _e2412;
                                    local_103 = _e2412;
                                    local_104 = _e2414;
                                    let _e2420 = (_e2416 < (_e2406 + 3u));
                                    if _e2420 {
                                        if (_e2416 < _e2406) {
                                            let _e4405 = global_5.member[_e2416];
                                            let _e4410 = (_e4405.member.x - _e1528);
                                            let _e4411 = (_e4405.member.y - _e1529);
                                            let _e4412 = (_e4405.member.z - _e1530);
                                            let _e4417 = (((_e4410 * _e4410) + (_e4411 * _e4411)) + (_e4412 * _e4412));
                                            let _e4418 = sqrt(_e4417);
                                            let _e4419 = (_e4410 / _e4418);
                                            let _e4420 = (_e4411 / _e4418);
                                            let _e4421 = (_e4412 / _e4418);
                                            if (_e4405.member.w < 0.5f) {
                                                phi_14678_ = 1f;
                                            } else {
                                                let _e4445 = (((_e4405.member_2.x * -(_e4419)) + (_e4405.member_2.y * -(_e4420))) + (_e4405.member_2.z * -(_e4421)));
                                                if (_e4405.member_3.x == _e4405.member_2.w) {
                                                    phi_14677_ = select(1f, 0f, (_e4445 < _e4405.member_3.x));
                                                } else {
                                                    let _e4449 = ((_e4445 - _e4405.member_3.x) / (_e4405.member_2.w - _e4405.member_3.x));
                                                    let _e4451 = select(_e4449, 0f, (_e4449 < 0f));
                                                    let _e4453 = select(_e4451, 1f, (_e4451 > 1f));
                                                    phi_14677_ = ((_e4453 * _e4453) * (3f - (2f * _e4453)));
                                                }
                                                let _e4461 = phi_14677_;
                                                phi_14678_ = _e4461;
                                            }
                                            let _e4463 = phi_14678_;
                                            phi_34608_ = _e2410;
                                            phi_17616_ = 0f;
                                            phi_17617_ = vec3<f32>(_e4419, _e4420, _e4421);
                                            phi_17618_ = vec3<f32>(((_e4405.member_1.x * _e4463) / _e4417), ((_e4405.member_1.y * _e4463) / _e4417), ((_e4405.member_1.z * _e4463) / _e4417));
                                            phi_17619_ = _e4418;
                                        } else {
                                            if (_e2416 == _e2406) {
                                                if ((_e358 & 4u) == 0u) {
                                                    phi_34652_ = _e2410;
                                                    phi_16720_ = 0f;
                                                    phi_16721_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_16722_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e3157 = global_1.member.member_5;
                                                    let _e3165 = global_1.member.member_6[3u];
                                                    let _e3167 = sin((_e3165 * 0.5f));
                                                    let _e3169 = ((12.566371f * _e3167) * _e3167);
                                                    let _e3174 = (((_e3157.x * _e1503.x) + (_e3157.y * _e1503.y)) + (_e3157.z * _e1503.z));
                                                    if (_e3174 != _e3174) {
                                                        phi_26897_ = true;
                                                    } else {
                                                        phi_26897_ = (0f >= _e3174);
                                                    }
                                                    let _e3178 = phi_26897_;
                                                    let _e3180 = (select(_e3174, 0f, _e3178) * 0.31830987f);
                                                    if (_e2395 == 0u) {
                                                        if (_e3174 <= 0f) {
                                                            phi_27694_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_27694_ = (_e2389 * (_e3174 * 0.31830987f));
                                                        }
                                                        let _e3656 = phi_27694_;
                                                        phi_15088_ = type_55(_e3656, _e3180);
                                                        phi_15089_ = type_55();
                                                        phi_15090_ = true;
                                                    } else {
                                                        let _e3186 = (((_e1475 * _e1551) + (_e1476 * _e1552)) + (_e1477 * _e1554));
                                                        let _e3191 = (((_e1475 * _e1547) + (_e1476 * _e1557)) + (_e1477 * _e1558));
                                                        let _e3196 = (((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z));
                                                        let _e3201 = (((_e3157.x * _e1551) + (_e3157.y * _e1552)) + (_e3157.z * _e1554));
                                                        let _e3206 = (((_e3157.x * _e1547) + (_e3157.y * _e1557)) + (_e3157.z * _e1558));
                                                        if (_e3196 <= 0f) {
                                                            phi_15052_ = type_55();
                                                            phi_15053_ = true;
                                                        } else {
                                                            let _e3208 = (_e3174 <= 0f);
                                                            if _e3208 {
                                                                phi_15051_ = type_55();
                                                            } else {
                                                                let _e3209 = (_e3186 + _e3201);
                                                                let _e3210 = (_e3191 + _e3206);
                                                                let _e3211 = (_e3196 + _e3174);
                                                                let _e3218 = (1f / sqrt((((_e3209 * _e3209) + (_e3210 * _e3210)) + (_e3211 * _e3211))));
                                                                let _e3221 = (_e3211 * _e3218);
                                                                if (_e3221 <= 0f) {
                                                                    phi_27007_ = 0f;
                                                                } else {
                                                                    let _e3223 = (_e2393 * _e2393);
                                                                    let _e3227 = ((((_e3223 - 1f) * _e3221) * _e3221) + 1f);
                                                                    phi_27007_ = (_e3223 / ((3.1415927f * _e3227) * _e3227));
                                                                }
                                                                let _e3232 = phi_27007_;
                                                                let _e3233 = (_e3196 * _e3196);
                                                                if (_e3233 >= 1f) {
                                                                    phi_27035_ = 0f;
                                                                } else {
                                                                    phi_27035_ = ((sqrt((1f + (((_e2393 * _e2393) * ((_e3186 * _e3186) + (_e3191 * _e3191))) / _e3233))) - 1f) * 0.5f);
                                                                }
                                                                let _e3246 = phi_27035_;
                                                                let _e3247 = (1f + _e3246);
                                                                let _e3248 = (_e3174 * _e3174);
                                                                if (_e3248 >= 1f) {
                                                                    phi_27063_ = 0f;
                                                                } else {
                                                                    phi_27063_ = ((sqrt((1f + (((_e2393 * _e2393) * ((_e3201 * _e3201) + (_e3206 * _e3206))) / _e3248))) - 1f) * 0.5f);
                                                                }
                                                                let _e3261 = phi_27063_;
                                                                let _e3268 = (((_e3186 * (_e3209 * _e3218)) + (_e3191 * (_e3210 * _e3218))) + (_e3196 * _e3221));
                                                                if (_e2391 == 0u) {
                                                                    let _e3563 = (1f - _e3268);
                                                                    if (_e3563 != _e3563) {
                                                                        phi_27566_ = true;
                                                                    } else {
                                                                        phi_27566_ = (0f >= _e3563);
                                                                    }
                                                                    let _e3567 = phi_27566_;
                                                                    let _e3569 = pow(select(_e3563, 0f, _e3567), 5f);
                                                                    let _e3570 = (1f - _e3569);
                                                                    phi_15039_ = vec3<f32>(((_e2389.x * _e3570) + _e3569), ((_e2389.y * _e3570) + _e3569), ((_e2389.z * _e3570) + _e3569));
                                                                } else {
                                                                    if (_e2391 == 1u) {
                                                                        let _e3536 = (1f / _e2385);
                                                                        let _e3540 = ((_e3536 * _e3536) * (1f - (_e3268 * _e3268)));
                                                                        if (_e3540 >= 1f) {
                                                                            phi_27527_ = 1f;
                                                                        } else {
                                                                            let _e3543 = sqrt((1f - _e3540));
                                                                            if (_e3268 != _e3268) {
                                                                                phi_27538_ = true;
                                                                            } else {
                                                                                phi_27538_ = (_e3543 <= _e3268);
                                                                            }
                                                                            let _e3547 = phi_27538_;
                                                                            let _e3552 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                                            phi_27527_ = (_e3552 + ((1f - _e3552) * pow((1f - select(_e3268, _e3543, _e3547)), 5f)));
                                                                        }
                                                                        let _e3559 = phi_27527_;
                                                                        phi_15038_ = vec3(_e3559);
                                                                    } else {
                                                                        let _e3272 = select(_e3268, 0f, (_e3268 < 0f));
                                                                        let _e3274 = select(_e3272, 1f, (_e3272 > 1f));
                                                                        let _e3276 = (1f - (_e3274 * _e3274));
                                                                        let _e3289 = ((_e2389.x * _e2389.x) - (_e2387.x * _e2387.x));
                                                                        let _e3290 = ((_e2389.y * _e2389.y) - (_e2387.y * _e2387.y));
                                                                        let _e3291 = ((_e2389.z * _e2389.z) - (_e2387.z * _e2387.z));
                                                                        let _e3293 = (_e2389.x * (_e2387.x + _e2387.x));
                                                                        let _e3295 = (_e2389.y * (_e2387.y + _e2387.y));
                                                                        let _e3297 = (_e2389.z * (_e2387.z + _e2387.z));
                                                                        let _e3304 = ((_e3289 * _e3289) + (_e3293 * _e3293));
                                                                        let _e3305 = ((_e3290 * _e3290) + (_e3295 * _e3295));
                                                                        let _e3306 = ((_e3291 * _e3291) + (_e3297 * _e3297));
                                                                        let _e3311 = ((_e3276 * _e3289) / _e3304);
                                                                        let _e3312 = ((_e3276 * _e3290) / _e3305);
                                                                        let _e3313 = ((_e3276 * _e3291) / _e3306);
                                                                        let _e3314 = (1f - _e3311);
                                                                        let _e3315 = (1f - _e3312);
                                                                        let _e3316 = (1f - _e3313);
                                                                        let _e3318 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e3311, _e3312, _e3313));
                                                                        let _e3319 = (_e3276 * _e3293);
                                                                        let _e3320 = (_e3276 * _e3295);
                                                                        let _e3321 = (_e3276 * _e3297);
                                                                        let _e3326 = (vec3<f32>(_e3319, _e3320, _e3321) / vec3<f32>(_e3304, _e3305, _e3306));
                                                                        let _e3339 = ((_e3318.x * _e3318.x) + (_e3326.x * _e3326.x));
                                                                        let _e3340 = ((_e3318.y * _e3318.y) + (_e3326.y * _e3326.y));
                                                                        let _e3341 = ((_e3318.z * _e3318.z) + (_e3326.z * _e3326.z));
                                                                        if (_e3339 != _e3339) {
                                                                            phi_27197_ = true;
                                                                        } else {
                                                                            phi_27197_ = (0f >= _e3339);
                                                                        }
                                                                        let _e3345 = phi_27197_;
                                                                        let _e3347 = sqrt(select(_e3339, 0f, _e3345));
                                                                        if (_e3340 != _e3340) {
                                                                            phi_27212_ = true;
                                                                        } else {
                                                                            phi_27212_ = (0f >= _e3340);
                                                                        }
                                                                        let _e3351 = phi_27212_;
                                                                        let _e3353 = sqrt(select(_e3340, 0f, _e3351));
                                                                        if (_e3341 != _e3341) {
                                                                            phi_27227_ = true;
                                                                        } else {
                                                                            phi_27227_ = (0f >= _e3341);
                                                                        }
                                                                        let _e3357 = phi_27227_;
                                                                        let _e3359 = sqrt(select(_e3341, 0f, _e3357));
                                                                        let _e3363 = ((_e3347 + _e3314) * 0.5f);
                                                                        let _e3364 = ((_e3353 + _e3315) * 0.5f);
                                                                        let _e3365 = ((_e3359 + _e3316) * 0.5f);
                                                                        if (_e3363 != _e3363) {
                                                                            phi_27254_ = true;
                                                                        } else {
                                                                            phi_27254_ = (0f >= _e3363);
                                                                        }
                                                                        let _e3369 = phi_27254_;
                                                                        let _e3371 = sqrt(select(_e3363, 0f, _e3369));
                                                                        if (_e3364 != _e3364) {
                                                                            phi_27269_ = true;
                                                                        } else {
                                                                            phi_27269_ = (0f >= _e3364);
                                                                        }
                                                                        let _e3375 = phi_27269_;
                                                                        let _e3377 = sqrt(select(_e3364, 0f, _e3375));
                                                                        if (_e3365 != _e3365) {
                                                                            phi_27284_ = true;
                                                                        } else {
                                                                            phi_27284_ = (0f >= _e3365);
                                                                        }
                                                                        let _e3381 = phi_27284_;
                                                                        let _e3383 = sqrt(select(_e3365, 0f, _e3381));
                                                                        let _e3384 = vec3<f32>(_e3371, _e3377, _e3383);
                                                                        let _e3388 = ((_e3347 - _e3314) * 0.5f);
                                                                        let _e3389 = ((_e3353 - _e3315) * 0.5f);
                                                                        let _e3390 = ((_e3359 - _e3316) * 0.5f);
                                                                        if (_e3388 != _e3388) {
                                                                            phi_27311_ = true;
                                                                        } else {
                                                                            phi_27311_ = (0f >= _e3388);
                                                                        }
                                                                        let _e3394 = phi_27311_;
                                                                        let _e3396 = sqrt(select(_e3388, 0f, _e3394));
                                                                        if (_e3389 != _e3389) {
                                                                            phi_27326_ = true;
                                                                        } else {
                                                                            phi_27326_ = (0f >= _e3389);
                                                                        }
                                                                        let _e3400 = phi_27326_;
                                                                        let _e3402 = sqrt(select(_e3389, 0f, _e3400));
                                                                        if (_e3390 != _e3390) {
                                                                            phi_27341_ = true;
                                                                        } else {
                                                                            phi_27341_ = (0f >= _e3390);
                                                                        }
                                                                        let _e3406 = phi_27341_;
                                                                        let _e3408 = sqrt(select(_e3390, 0f, _e3406));
                                                                        if ((_e3319 / _e3304) < 0f) {
                                                                            phi_14915_ = -(_e3396);
                                                                        } else {
                                                                            phi_14915_ = _e3396;
                                                                        }
                                                                        let _e3412 = phi_14915_;
                                                                        if ((_e3320 / _e3305) < 0f) {
                                                                            phi_14923_ = -(_e3402);
                                                                        } else {
                                                                            phi_14923_ = _e3402;
                                                                        }
                                                                        let _e3416 = phi_14923_;
                                                                        if ((_e3321 / _e3306) < 0f) {
                                                                            phi_14931_ = -(_e3408);
                                                                        } else {
                                                                            phi_14931_ = _e3408;
                                                                        }
                                                                        let _e3420 = phi_14931_;
                                                                        let _e3424 = (_e2387.x * _e3274);
                                                                        let _e3425 = (_e2387.y * _e3274);
                                                                        let _e3426 = (_e2387.z * _e3274);
                                                                        let _e3436 = vec3<f32>(((_e2389.x * _e3371) - (_e2387.x * _e3412)), ((_e2389.y * _e3377) - (_e2387.y * _e3416)), ((_e2389.z * _e3383) - (_e2387.z * _e3420)));
                                                                        let _e3443 = ((_e2389.x * _e3412) + (_e2387.x * _e3371));
                                                                        let _e3444 = ((_e2389.y * _e3416) + (_e2387.y * _e3377));
                                                                        let _e3445 = ((_e2389.z * _e3420) + (_e2387.z * _e3383));
                                                                        let _e3447 = vec3<f32>((_e2389.x * _e3274), (_e2389.y * _e3274), (_e2389.z * _e3274));
                                                                        let _e3448 = (_e3447 - _e3384);
                                                                        let _e3449 = (_e3424 - _e3412);
                                                                        let _e3450 = (_e3425 - _e3416);
                                                                        let _e3451 = (_e3426 - _e3420);
                                                                        let _e3464 = (_e3447 + _e3384);
                                                                        let _e3465 = (_e3424 + _e3412);
                                                                        let _e3466 = (_e3425 + _e3416);
                                                                        let _e3467 = (_e3426 + _e3420);
                                                                        let _e3477 = ((_e3464.x * _e3464.x) + (_e3465 * _e3465));
                                                                        let _e3478 = ((_e3464.y * _e3464.y) + (_e3466 * _e3466));
                                                                        let _e3479 = ((_e3464.z * _e3464.z) + (_e3467 * _e3467));
                                                                        let _e3489 = vec3(_e3274);
                                                                        let _e3490 = (_e3489 - _e3436);
                                                                        let _e3491 = -(vec3<f32>(_e3443, _e3444, _e3445));
                                                                        let _e3507 = (_e3489 + _e3436);
                                                                        let _e3517 = ((_e3507.x * _e3507.x) + (_e3443 * _e3443));
                                                                        let _e3518 = ((_e3507.y * _e3507.y) + (_e3444 * _e3444));
                                                                        let _e3519 = ((_e3507.z * _e3507.z) + (_e3445 * _e3445));
                                                                        phi_15038_ = vec3<f32>((((((_e3448.x * _e3448.x) + (_e3449 * _e3449)) / select(0.000000000000000000000000000001f, _e3477, (_e3477 > 0.000000000000000000000000000001f))) + (((_e3490.x * _e3490.x) + (_e3491.x * _e3491.x)) / select(0.000000000000000000000000000001f, _e3517, (_e3517 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3448.y * _e3448.y) + (_e3450 * _e3450)) / select(0.000000000000000000000000000001f, _e3478, (_e3478 > 0.000000000000000000000000000001f))) + (((_e3490.y * _e3490.y) + (_e3491.y * _e3491.y)) / select(0.000000000000000000000000000001f, _e3518, (_e3518 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3448.z * _e3448.z) + (_e3451 * _e3451)) / select(0.000000000000000000000000000001f, _e3479, (_e3479 > 0.000000000000000000000000000001f))) + (((_e3490.z * _e3490.z) + (_e3491.z * _e3491.z)) / select(0.000000000000000000000000000001f, _e3519, (_e3519 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3562 = phi_15038_;
                                                                    phi_15039_ = _e3562;
                                                                }
                                                                let _e3582 = phi_15039_;
                                                                let _e3584 = (4f * _e3196);
                                                                phi_15051_ = type_55((_e3582 * (((1f / (_e3247 + _e3261)) * _e3232) / _e3584)), ((_e3232 / _e3247) / _e3584));
                                                            }
                                                            let _e3591 = phi_15051_;
                                                            phi_15052_ = _e3591;
                                                            phi_15053_ = _e3208;
                                                        }
                                                        let _e3593 = phi_15052_;
                                                        let _e3595 = phi_15053_;
                                                        if _e3595 {
                                                            phi_15058_ = type_55(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15058_ = _e3593;
                                                        }
                                                        let _e3597 = phi_15058_;
                                                        let _e3600 = (_e2395 == 1u);
                                                        if _e3600 {
                                                            phi_15086_ = _e3597;
                                                            phi_15087_ = type_55();
                                                        } else {
                                                            if (_e3196 != _e3196) {
                                                                phi_27603_ = true;
                                                            } else {
                                                                phi_27603_ = (0f >= _e3196);
                                                            }
                                                            let _e3604 = phi_27603_;
                                                            let _e3605 = select(_e3196, 0f, _e3604);
                                                            let _e3606 = (1f / _e2385);
                                                            let _e3610 = ((_e3606 * _e3606) * (1f - (_e3605 * _e3605)));
                                                            if (_e3610 >= 1f) {
                                                                phi_27622_ = 1f;
                                                            } else {
                                                                let _e3613 = sqrt((1f - _e3610));
                                                                if (_e3605 != _e3605) {
                                                                    phi_27633_ = true;
                                                                } else {
                                                                    phi_27633_ = (_e3613 <= _e3605);
                                                                }
                                                                let _e3617 = phi_27633_;
                                                                let _e3622 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                                phi_27622_ = (_e3622 + ((1f - _e3622) * pow((1f - select(_e3605, _e3613, _e3617)), 5f)));
                                                            }
                                                            let _e3629 = phi_27622_;
                                                            if (_e3174 <= 0f) {
                                                                phi_27672_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_27672_ = (_e2383 * (_e3174 * 0.31830987f));
                                                            }
                                                            let _e3634 = phi_27672_;
                                                            let _e3635 = (1f - _e3629);
                                                            phi_15086_ = type_55();
                                                            phi_15087_ = type_55((_e3597.member + vec3<f32>((_e3634.x * _e3635), (_e3634.y * _e3635), (_e3634.z * _e3635))), ((_e3629 * _e3597.member_1) + (_e3635 * _e3180)));
                                                        }
                                                        let _e3649 = phi_15086_;
                                                        let _e3651 = phi_15087_;
                                                        phi_15088_ = _e3649;
                                                        phi_15089_ = _e3651;
                                                        phi_15090_ = _e3600;
                                                    }
                                                    let _e3659 = phi_15088_;
                                                    let _e3661 = phi_15089_;
                                                    let _e3663 = phi_15090_;
                                                    if _e3663 {
                                                        phi_15094_ = _e3659;
                                                    } else {
                                                        phi_15094_ = _e3661;
                                                    }
                                                    let _e3665 = phi_15094_;
                                                    if ((_e3665.member_1 * _e3169) < 0.001f) {
                                                        let _e4092 = global_1.member.member_6;
                                                        let _e4098 = global_1.member.member_8[2u];
                                                        if (_e4098 == 4294967295u) {
                                                            phi_15833_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e4101 = global_6.member[_e4098];
                                                            let _e4104 = global_6.member[(_e4098 + 1u)];
                                                            let _e4107 = global_6.member[(_e4098 + 2u)];
                                                            let _e4110 = global_6.member[(_e4098 + 3u)];
                                                            let _e4113 = global_6.member[(_e4098 + 4u)];
                                                            let _e4116 = global_6.member[(_e4098 + 5u)];
                                                            let _e4119 = global_6.member[(_e4098 + 6u)];
                                                            let _e4122 = global_6.member[(_e4098 + 7u)];
                                                            let _e4125 = global_6.member[(_e4098 + 8u)];
                                                            let _e4128 = global_6.member[(_e4098 + 9u)];
                                                            let _e4131 = global_6.member[(_e4098 + 13u)];
                                                            let _e4134 = global_6.member[(_e4098 + 14u)];
                                                            let _e4137 = global_6.member[(_e4098 + 15u)];
                                                            let _e4140 = global_6.member[(_e4098 + 16u)];
                                                            let _e4143 = global_6.member[(_e4098 + 18u)];
                                                            let _e4146 = global_6.member[(_e4098 + 19u)];
                                                            let _e4149 = global_6.member[(_e4098 + 20u)];
                                                            let _e4152 = global_6.member[(_e4098 + 29u)];
                                                            let _e4153 = bitcast<u32>(_e4152);
                                                            let _e4166 = (((((_e1528 * _e1528) + (_e1529 * _e1529)) + (_e1530 * _e1530)) - (2f * (((_e1528 * _e4101) + (_e1529 * _e4104)) + (_e1530 * _e4107)))) + _e4119);
                                                            if (_e4166 > 0f) {
                                                                let _e4175 = ((((_e1528 - _e4101) * _e4122) + ((_e1529 - _e4104) * _e4125)) + ((_e1530 - _e4107) * _e4128));
                                                                let _e4177 = ((_e4175 * _e4175) - _e4166);
                                                                if (_e4175 >= 0f) {
                                                                    phi_15533_ = vec3<f32>();
                                                                    phi_15534_ = true;
                                                                } else {
                                                                    let _e4179 = (_e4177 <= 0f);
                                                                    if _e4179 {
                                                                        phi_15532_ = vec3<f32>();
                                                                    } else {
                                                                        let _e4182 = (-(_e4175) - sqrt(_e4177));
                                                                        phi_15532_ = vec3<f32>((_e1528 + (_e4122 * _e4182)), (_e1529 + (_e4125 * _e4182)), (_e1530 + (_e4128 * _e4182)));
                                                                    }
                                                                    let _e4191 = phi_15532_;
                                                                    phi_15533_ = _e4191;
                                                                    phi_15534_ = _e4179;
                                                                }
                                                                let _e4193 = phi_15533_;
                                                                let _e4195 = phi_15534_;
                                                                phi_15539_ = _e4193;
                                                                phi_15540_ = select(true, false, _e4195);
                                                            } else {
                                                                phi_15539_ = _e1531;
                                                                phi_15540_ = true;
                                                            }
                                                            let _e4198 = phi_15539_;
                                                            let _e4200 = phi_15540_;
                                                            if _e4200 {
                                                                let _e4202 = (_e4198.x - _e4101);
                                                                let _e4204 = (_e4198.y - _e4104);
                                                                let _e4206 = (_e4198.z - _e4107);
                                                                let _e4212 = sqrt((((_e4202 * _e4202) + (_e4204 * _e4204)) + (_e4206 * _e4206)));
                                                                let _e4213 = (1f / _e4212);
                                                                let _e4231 = ((((((_e4198.x * _e4198.x) + (_e4198.y * _e4198.y)) + (_e4198.z * _e4198.z)) - (2f * (((_e4198.x * _e4101) + (_e4198.y * _e4104)) + (_e4198.z * _e4107)))) + _e4116) / (_e4212 + _e4110));
                                                                if (_e4231 != _e4231) {
                                                                    phi_28128_ = true;
                                                                } else {
                                                                    phi_28128_ = (0f >= _e4231);
                                                                }
                                                                let _e4235 = phi_28128_;
                                                                let _e4239 = sqrt((select(_e4231, 0f, _e4235) / (_e4113 - _e4110)));
                                                                if (_e4239 != _e4239) {
                                                                    phi_28143_ = true;
                                                                } else {
                                                                    phi_28143_ = (1f <= _e4239);
                                                                }
                                                                let _e4243 = phi_28143_;
                                                                let _e4245 = (select(_e4239, 1f, _e4243) * 63f);
                                                                let _e4250 = ((((_e4202 * _e4213) * _e4122) + ((_e4204 * _e4213) * _e4125)) + ((_e4206 * _e4213) * _e4128));
                                                                let _e4252 = select(_e4250, -1f, (_e4250 < -1f));
                                                                let _e4256 = ((select(_e4252, 1f, (_e4252 > 1f)) + 1f) * 127.5f);
                                                                let _e4261 = select(select(u32(_e4245), 0u, (_e4245 < 0f)), 4294967295u, (_e4245 > 4294967000f));
                                                                let _e4263 = select(_e4261, 62u, (62u < _e4261));
                                                                let _e4268 = select(select(u32(_e4256), 0u, (_e4256 < 0f)), 4294967295u, (_e4256 > 4294967000f));
                                                                let _e4270 = select(_e4268, 254u, (254u < _e4268));
                                                                let _e4272 = (_e4245 - f32(_e4263));
                                                                let _e4274 = (_e4256 - f32(_e4270));
                                                                let _e4278 = (_e4153 + (3u * ((_e4263 * 256u) + _e4270)));
                                                                let _e4283 = (_e4153 + (3u * (((_e4263 + 1u) * 256u) + _e4270)));
                                                                let _e4285 = global_6.member[_e4278];
                                                                let _e4288 = global_6.member[(_e4278 + 1u)];
                                                                let _e4291 = global_6.member[(_e4278 + 2u)];
                                                                let _e4292 = (1f - _e4274);
                                                                let _e4298 = global_6.member[(_e4278 + 3u)];
                                                                let _e4301 = global_6.member[(_e4278 + 4u)];
                                                                let _e4304 = global_6.member[(_e4278 + 5u)];
                                                                let _e4311 = (1f - _e4272);
                                                                let _e4316 = global_6.member[_e4283];
                                                                let _e4319 = global_6.member[(_e4283 + 1u)];
                                                                let _e4322 = global_6.member[(_e4283 + 2u)];
                                                                let _e4328 = global_6.member[(_e4283 + 3u)];
                                                                let _e4331 = global_6.member[(_e4283 + 4u)];
                                                                let _e4334 = global_6.member[(_e4283 + 5u)];
                                                                phi_15831_ = vec3<f32>(((((_e4285 * _e4292) + (_e4298 * _e4274)) * _e4311) + (((_e4316 * _e4292) + (_e4328 * _e4274)) * _e4272)), ((((_e4288 * _e4292) + (_e4301 * _e4274)) * _e4311) + (((_e4319 * _e4292) + (_e4331 * _e4274)) * _e4272)), ((((_e4291 * _e4292) + (_e4304 * _e4274)) * _e4311) + (((_e4322 * _e4292) + (_e4334 * _e4274)) * _e4272)));
                                                            } else {
                                                                phi_15831_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e4349 = phi_15831_;
                                                            let _e4356 = ((_e4140 * 1.1f) * _e4349.y);
                                                            phi_15833_ = vec3<f32>(exp(-((((_e4131 * _e4349.x) + _e4356) + (_e4143 * _e4349.z)))), exp(-((((_e4134 * _e4349.x) + _e4356) + (_e4146 * _e4349.z)))), exp(-((((_e4137 * _e4349.x) + _e4356) + (_e4149 * _e4349.z)))));
                                                        }
                                                        let _e4375 = phi_15833_;
                                                        phi_34653_ = _e2410;
                                                        phi_16717_ = 0f;
                                                        phi_16718_ = vec3<f32>(_e3157.x, _e3157.y, _e3157.z);
                                                        phi_16719_ = (vec3<f32>(_e4092.x, _e4092.y, _e4092.z) * _e4375);
                                                    } else {
                                                        let _e3670 = ((_e2410 * 747796405u) + 2891336453u);
                                                        let _e3678 = (((_e3670 >> bitcast<u32>((((_e3670 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3670) * 277803737u);
                                                        let _e3681 = ((_e3678 >> bitcast<u32>(22i)) ^ _e3678);
                                                        let _e3687 = (((f32((_e3681 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e3169) * 0.15915494f);
                                                        let _e3688 = (1f - _e3687);
                                                        let _e3690 = (_e3687 * (2f - _e3687));
                                                        if (_e3690 != _e3690) {
                                                            phi_27726_ = true;
                                                        } else {
                                                            phi_27726_ = (0f >= _e3690);
                                                        }
                                                        let _e3694 = phi_27726_;
                                                        let _e3696 = sqrt(select(_e3690, 0f, _e3694));
                                                        let _e3698 = ((_e3681 * 747796405u) + 2891336453u);
                                                        let _e3706 = (((_e3698 >> bitcast<u32>((((_e3698 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3698) * 277803737u);
                                                        let _e3709 = ((_e3706 >> bitcast<u32>(22i)) ^ _e3706);
                                                        let _e3713 = (f32((_e3709 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3715 = select(-1f, 1f, (_e3157.z >= 0f));
                                                        let _e3717 = (-1f / (_e3715 + _e3157.z));
                                                        let _e3719 = ((_e3157.x * _e3157.y) * _e3717);
                                                        let _e3732 = (_e3696 * cos(_e3713));
                                                        let _e3737 = (_e3696 * sin(_e3713));
                                                        let _e3747 = ((((1f + (((_e3715 * _e3157.x) * _e3157.x) * _e3717)) * _e3732) + (_e3719 * _e3737)) + (_e3157.x * _e3688));
                                                        let _e3748 = ((((_e3715 * _e3719) * _e3732) + ((_e3715 + ((_e3157.y * _e3157.y) * _e3717)) * _e3737)) + (_e3157.y * _e3688));
                                                        let _e3749 = ((((-(_e3715) * _e3157.x) * _e3732) + (-(_e3157.y) * _e3737)) + (_e3157.z * _e3688));
                                                        let _e3751 = (_e3747 - _e3157.x);
                                                        let _e3752 = (_e3748 - _e3157.y);
                                                        let _e3753 = (_e3749 - _e3157.z);
                                                        let _e3760 = (sqrt((((_e3751 * _e3751) + (_e3752 * _e3752)) + (_e3753 * _e3753))) * 0.5f);
                                                        let _e3761 = (_e3760 != _e3760);
                                                        if _e3761 {
                                                            phi_27797_ = true;
                                                        } else {
                                                            phi_27797_ = (1f <= _e3760);
                                                        }
                                                        let _e3764 = phi_27797_;
                                                        if ((2f * asin(select(_e3760, 1f, _e3764))) >= _e3165) {
                                                            phi_15914_ = 0f;
                                                        } else {
                                                            phi_15914_ = (1f / _e3169);
                                                        }
                                                        let _e3771 = phi_15914_;
                                                        if (_e3771 > 0f) {
                                                            if _e3761 {
                                                                phi_27812_ = true;
                                                            } else {
                                                                phi_27812_ = (1f <= _e3760);
                                                            }
                                                            let _e3775 = phi_27812_;
                                                            let _e3778 = (2f * asin(select(_e3760, 1f, _e3775)));
                                                            if (_e3778 >= _e3165) {
                                                                phi_16715_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3782 = (1f - pow((_e3778 / _e3165), 2f));
                                                                if (_e3782 != _e3782) {
                                                                    phi_27827_ = true;
                                                                } else {
                                                                    phi_27827_ = (0f >= _e3782);
                                                                }
                                                                let _e3786 = phi_27827_;
                                                                let _e3789 = global_1.member.member_6;
                                                                let _e3795 = global_1.member.member_8[2u];
                                                                if (_e3795 == 4294967295u) {
                                                                    phi_16688_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e3798 = global_6.member[_e3795];
                                                                    let _e3801 = global_6.member[(_e3795 + 1u)];
                                                                    let _e3804 = global_6.member[(_e3795 + 2u)];
                                                                    let _e3807 = global_6.member[(_e3795 + 3u)];
                                                                    let _e3810 = global_6.member[(_e3795 + 4u)];
                                                                    let _e3813 = global_6.member[(_e3795 + 5u)];
                                                                    let _e3816 = global_6.member[(_e3795 + 6u)];
                                                                    let _e3819 = global_6.member[(_e3795 + 7u)];
                                                                    let _e3822 = global_6.member[(_e3795 + 8u)];
                                                                    let _e3825 = global_6.member[(_e3795 + 9u)];
                                                                    let _e3828 = global_6.member[(_e3795 + 13u)];
                                                                    let _e3831 = global_6.member[(_e3795 + 14u)];
                                                                    let _e3834 = global_6.member[(_e3795 + 15u)];
                                                                    let _e3837 = global_6.member[(_e3795 + 16u)];
                                                                    let _e3840 = global_6.member[(_e3795 + 18u)];
                                                                    let _e3843 = global_6.member[(_e3795 + 19u)];
                                                                    let _e3846 = global_6.member[(_e3795 + 20u)];
                                                                    let _e3849 = global_6.member[(_e3795 + 29u)];
                                                                    let _e3850 = bitcast<u32>(_e3849);
                                                                    let _e3863 = (((((_e1528 * _e1528) + (_e1529 * _e1529)) + (_e1530 * _e1530)) - (2f * (((_e1528 * _e3798) + (_e1529 * _e3801)) + (_e1530 * _e3804)))) + _e3816);
                                                                    if (_e3863 > 0f) {
                                                                        let _e3872 = ((((_e1528 - _e3798) * _e3819) + ((_e1529 - _e3801) * _e3822)) + ((_e1530 - _e3804) * _e3825));
                                                                        let _e3874 = ((_e3872 * _e3872) - _e3863);
                                                                        if (_e3872 >= 0f) {
                                                                            phi_16388_ = vec3<f32>();
                                                                            phi_16389_ = true;
                                                                        } else {
                                                                            let _e3876 = (_e3874 <= 0f);
                                                                            if _e3876 {
                                                                                phi_16387_ = vec3<f32>();
                                                                            } else {
                                                                                let _e3879 = (-(_e3872) - sqrt(_e3874));
                                                                                phi_16387_ = vec3<f32>((_e1528 + (_e3819 * _e3879)), (_e1529 + (_e3822 * _e3879)), (_e1530 + (_e3825 * _e3879)));
                                                                            }
                                                                            let _e3888 = phi_16387_;
                                                                            phi_16388_ = _e3888;
                                                                            phi_16389_ = _e3876;
                                                                        }
                                                                        let _e3890 = phi_16388_;
                                                                        let _e3892 = phi_16389_;
                                                                        phi_16394_ = _e3890;
                                                                        phi_16395_ = select(true, false, _e3892);
                                                                    } else {
                                                                        phi_16394_ = _e1531;
                                                                        phi_16395_ = true;
                                                                    }
                                                                    let _e3895 = phi_16394_;
                                                                    let _e3897 = phi_16395_;
                                                                    if _e3897 {
                                                                        let _e3899 = (_e3895.x - _e3798);
                                                                        let _e3901 = (_e3895.y - _e3801);
                                                                        let _e3903 = (_e3895.z - _e3804);
                                                                        let _e3909 = sqrt((((_e3899 * _e3899) + (_e3901 * _e3901)) + (_e3903 * _e3903)));
                                                                        let _e3910 = (1f / _e3909);
                                                                        let _e3928 = ((((((_e3895.x * _e3895.x) + (_e3895.y * _e3895.y)) + (_e3895.z * _e3895.z)) - (2f * (((_e3895.x * _e3798) + (_e3895.y * _e3801)) + (_e3895.z * _e3804)))) + _e3813) / (_e3909 + _e3807));
                                                                        if (_e3928 != _e3928) {
                                                                            phi_27951_ = true;
                                                                        } else {
                                                                            phi_27951_ = (0f >= _e3928);
                                                                        }
                                                                        let _e3932 = phi_27951_;
                                                                        let _e3936 = sqrt((select(_e3928, 0f, _e3932) / (_e3810 - _e3807)));
                                                                        if (_e3936 != _e3936) {
                                                                            phi_27966_ = true;
                                                                        } else {
                                                                            phi_27966_ = (1f <= _e3936);
                                                                        }
                                                                        let _e3940 = phi_27966_;
                                                                        let _e3942 = (select(_e3936, 1f, _e3940) * 63f);
                                                                        let _e3947 = ((((_e3899 * _e3910) * _e3819) + ((_e3901 * _e3910) * _e3822)) + ((_e3903 * _e3910) * _e3825));
                                                                        let _e3949 = select(_e3947, -1f, (_e3947 < -1f));
                                                                        let _e3953 = ((select(_e3949, 1f, (_e3949 > 1f)) + 1f) * 127.5f);
                                                                        let _e3958 = select(select(u32(_e3942), 0u, (_e3942 < 0f)), 4294967295u, (_e3942 > 4294967000f));
                                                                        let _e3960 = select(_e3958, 62u, (62u < _e3958));
                                                                        let _e3965 = select(select(u32(_e3953), 0u, (_e3953 < 0f)), 4294967295u, (_e3953 > 4294967000f));
                                                                        let _e3967 = select(_e3965, 254u, (254u < _e3965));
                                                                        let _e3969 = (_e3942 - f32(_e3960));
                                                                        let _e3971 = (_e3953 - f32(_e3967));
                                                                        let _e3975 = (_e3850 + (3u * ((_e3960 * 256u) + _e3967)));
                                                                        let _e3980 = (_e3850 + (3u * (((_e3960 + 1u) * 256u) + _e3967)));
                                                                        let _e3982 = global_6.member[_e3975];
                                                                        let _e3985 = global_6.member[(_e3975 + 1u)];
                                                                        let _e3988 = global_6.member[(_e3975 + 2u)];
                                                                        let _e3989 = (1f - _e3971);
                                                                        let _e3995 = global_6.member[(_e3975 + 3u)];
                                                                        let _e3998 = global_6.member[(_e3975 + 4u)];
                                                                        let _e4001 = global_6.member[(_e3975 + 5u)];
                                                                        let _e4008 = (1f - _e3969);
                                                                        let _e4013 = global_6.member[_e3980];
                                                                        let _e4016 = global_6.member[(_e3980 + 1u)];
                                                                        let _e4019 = global_6.member[(_e3980 + 2u)];
                                                                        let _e4025 = global_6.member[(_e3980 + 3u)];
                                                                        let _e4028 = global_6.member[(_e3980 + 4u)];
                                                                        let _e4031 = global_6.member[(_e3980 + 5u)];
                                                                        phi_16686_ = vec3<f32>(((((_e3982 * _e3989) + (_e3995 * _e3971)) * _e4008) + (((_e4013 * _e3989) + (_e4025 * _e3971)) * _e3969)), ((((_e3985 * _e3989) + (_e3998 * _e3971)) * _e4008) + (((_e4016 * _e3989) + (_e4028 * _e3971)) * _e3969)), ((((_e3988 * _e3989) + (_e4001 * _e3971)) * _e4008) + (((_e4019 * _e3989) + (_e4031 * _e3971)) * _e3969)));
                                                                    } else {
                                                                        phi_16686_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e4046 = phi_16686_;
                                                                    let _e4053 = ((_e3837 * 1.1f) * _e4046.y);
                                                                    phi_16688_ = vec3<f32>(exp(-((((_e3828 * _e4046.x) + _e4053) + (_e3840 * _e4046.z)))), exp(-((((_e3831 * _e4046.x) + _e4053) + (_e3843 * _e4046.z)))), exp(-((((_e3834 * _e4046.x) + _e4053) + (_e3846 * _e4046.z)))));
                                                                }
                                                                let _e4072 = phi_16688_;
                                                                let _e4083 = (((1f - (0.6f * (1f - sqrt(select(_e3782, 0f, _e3786))))) * 1.25f) / _e3169);
                                                                phi_16715_ = vec3<f32>(((_e3789.x * _e4072.x) * _e4083), ((_e3789.y * _e4072.y) * _e4083), ((_e3789.z * _e4072.z) * _e4083));
                                                            }
                                                            let _e4089 = phi_16715_;
                                                            phi_16716_ = _e4089;
                                                        } else {
                                                            phi_16716_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e4091 = phi_16716_;
                                                        phi_34653_ = _e3709;
                                                        phi_16717_ = _e3771;
                                                        phi_16718_ = vec3<f32>(_e3747, _e3748, _e3749);
                                                        phi_16719_ = _e4091;
                                                    }
                                                    let _e4379 = phi_34653_;
                                                    let _e4381 = phi_16717_;
                                                    let _e4383 = phi_16718_;
                                                    let _e4385 = phi_16719_;
                                                    phi_34652_ = _e4379;
                                                    phi_16720_ = _e4381;
                                                    phi_16721_ = _e4383;
                                                    phi_16722_ = _e4385;
                                                }
                                                let _e4387 = phi_34652_;
                                                let _e4389 = phi_16720_;
                                                let _e4391 = phi_16721_;
                                                let _e4393 = phi_16722_;
                                                phi_34609_ = _e4387;
                                                phi_17612_ = _e4389;
                                                phi_17613_ = _e4391;
                                                phi_17614_ = _e4393;
                                                phi_17615_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e2416 == (_e2406 + 1u)) {
                                                    if ((_e358 & 1u) == 0u) {
                                                        phi_34618_ = _e2410;
                                                        phi_17246_ = 0f;
                                                        phi_17247_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17248_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17249_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e361 > 0u) {
                                                            let _e2647 = global_1.member.member_2[1u];
                                                            let _e2649 = ((_e2410 * 747796405u) + 2891336453u);
                                                            let _e2657 = (((_e2649 >> bitcast<u32>((((_e2649 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2649) * 277803737u);
                                                            let _e2660 = ((_e2657 >> bitcast<u32>(22i)) ^ _e2657);
                                                            phi_16743_ = 0u;
                                                            phi_16746_ = _e361;
                                                            loop {
                                                                let _e2666 = phi_16743_;
                                                                let _e2668 = phi_16746_;
                                                                local_83 = _e2666;
                                                                local_84 = _e2666;
                                                                let _e2670 = ((_e2666 + 1u) < _e2668);
                                                                if _e2670 {
                                                                    let _e2672 = ((_e2666 + _e2668) / 2u);
                                                                    let _e2675 = global_6.member[(_e2647 + _e2672)];
                                                                    let _e2676 = (_e2675 <= (f32((_e2660 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_16744_ = select(_e2666, _e2672, _e2676);
                                                                    phi_16747_ = select(_e2672, _e2668, _e2676);
                                                                } else {
                                                                    phi_16744_ = u32();
                                                                    phi_16747_ = u32();
                                                                }
                                                                let _e2680 = phi_16744_;
                                                                let _e2682 = phi_16747_;
                                                                phi_16743_ = _e2680;
                                                                phi_16746_ = _e2682;
                                                                if !(_e2670) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2685 = local_83;
                                                            let _e2686 = (_e2647 + _e2685);
                                                            let _e2689 = global_6.member[(_e2686 + 1u)];
                                                            let _e2691 = global_6.member[_e2686];
                                                            let _e2695 = global_1.member.member_2[0u];
                                                            let _e2697 = local_84;
                                                            let _e2700 = global_6.member[(_e2695 + _e2697)];
                                                            let _e2703 = ((_e2660 * 747796405u) + 2891336453u);
                                                            let _e2711 = (((_e2703 >> bitcast<u32>((((_e2703 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2703) * 277803737u);
                                                            let _e2714 = ((_e2711 >> bitcast<u32>(22i)) ^ _e2711);
                                                            let _e2717 = f32((_e2714 >> bitcast<u32>(8i)));
                                                            let _e2718 = (_e2717 * 0.000000059604645f);
                                                            let _e2720 = ((_e2714 * 747796405u) + 2891336453u);
                                                            let _e2728 = (((_e2720 >> bitcast<u32>((((_e2720 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2720) * 277803737u);
                                                            let _e2731 = ((_e2728 >> bitcast<u32>(22i)) ^ _e2728);
                                                            let _e2734 = f32((_e2731 >> bitcast<u32>(8i)));
                                                            let _e2737 = global_2.member[bitcast<u32>(_e2700)];
                                                            let _e2740 = (_e2737.member_6.x == 1u);
                                                            if _e2740 {
                                                                let _e2882 = (_e2737.member.x - _e1528);
                                                                let _e2883 = (_e2737.member.y - _e1529);
                                                                let _e2884 = (_e2737.member.z - _e1530);
                                                                let _e2889 = (((_e2882 * _e2882) + (_e2883 * _e2883)) + (_e2884 * _e2884));
                                                                let _e2890 = (_e2737.member.w * _e2737.member.w);
                                                                let _e2892 = (_e2889 <= (_e2890 * 1.000001f));
                                                                if _e2892 {
                                                                    let _e3001 = (_e2717 * 0.00000037450704f);
                                                                    let _e3003 = ((_e2734 * 0.00000011920929f) - 1f);
                                                                    let _e3005 = (1f - (_e3003 * _e3003));
                                                                    if (_e3005 != _e3005) {
                                                                        phi_26794_ = true;
                                                                    } else {
                                                                        phi_26794_ = (0f >= _e3005);
                                                                    }
                                                                    let _e3009 = phi_26794_;
                                                                    let _e3011 = sqrt(select(_e3005, 0f, _e3009));
                                                                    let _e3013 = (_e3011 * cos(_e3001));
                                                                    let _e3015 = (_e3011 * sin(_e3001));
                                                                    let _e3020 = (_e2737.member.x + (_e3013 * _e2737.member.w));
                                                                    let _e3021 = (_e2737.member.y + (_e3015 * _e2737.member.w));
                                                                    let _e3022 = (_e2737.member.z + (_e3003 * _e2737.member.w));
                                                                    if _e2892 {
                                                                        let _e3035 = (_e3020 - _e1528);
                                                                        let _e3036 = (_e3021 - _e1529);
                                                                        let _e3037 = (_e3022 - _e1530);
                                                                        let _e3042 = (((_e3035 * _e3035) + (_e3036 * _e3036)) + (_e3037 * _e3037));
                                                                        phi_26863_ = (_e3042 / (((abs((((_e3013 * _e3035) + (_e3015 * _e3036)) + (_e3003 * _e3037))) / sqrt(_e3042)) * 12.566371f) * _e2890));
                                                                    } else {
                                                                        let _e3024 = (_e2890 / _e2889);
                                                                        if (_e3024 != _e3024) {
                                                                            phi_26882_ = true;
                                                                        } else {
                                                                            phi_26882_ = (1f <= _e3024);
                                                                        }
                                                                        let _e3028 = phi_26882_;
                                                                        let _e3029 = select(_e3024, 1f, _e3028);
                                                                        phi_26863_ = (0.15915494f / (_e3029 / (1f + sqrt((1f - _e3029)))));
                                                                    }
                                                                    let _e3055 = phi_26863_;
                                                                    phi_16959_ = type_71(vec3<f32>(_e3020, _e3021, _e3022), vec3<f32>(_e3013, _e3015, _e3003), _e3055);
                                                                } else {
                                                                    let _e2893 = (_e2890 / _e2889);
                                                                    let _e2894 = (_e2893 != _e2893);
                                                                    if _e2894 {
                                                                        phi_26646_ = true;
                                                                    } else {
                                                                        phi_26646_ = (1f <= _e2893);
                                                                    }
                                                                    let _e2897 = phi_26646_;
                                                                    let _e2898 = select(_e2893, 1f, _e2897);
                                                                    if _e2894 {
                                                                        phi_26669_ = true;
                                                                    } else {
                                                                        phi_26669_ = (1f <= _e2893);
                                                                    }
                                                                    let _e2902 = phi_26669_;
                                                                    let _e2903 = select(_e2893, 1f, _e2902);
                                                                    let _e2907 = (_e2903 / (1f + sqrt((1f - _e2903))));
                                                                    if (_e2898 < 0.00068523f) {
                                                                        phi_16894_ = (_e2898 * _e2718);
                                                                    } else {
                                                                        let _e2910 = (1f - (_e2718 * _e2907));
                                                                        let _e2912 = (1f - (_e2910 * _e2910));
                                                                        if (_e2912 != _e2912) {
                                                                            phi_26684_ = true;
                                                                        } else {
                                                                            phi_26684_ = (0f >= _e2912);
                                                                        }
                                                                        let _e2916 = phi_26684_;
                                                                        phi_16894_ = select(_e2912, 0f, _e2916);
                                                                    }
                                                                    let _e2920 = phi_16894_;
                                                                    let _e2921 = (1f - _e2920);
                                                                    if (_e2921 != _e2921) {
                                                                        phi_26699_ = true;
                                                                    } else {
                                                                        phi_26699_ = (0f >= _e2921);
                                                                    }
                                                                    let _e2925 = phi_26699_;
                                                                    let _e2930 = (1f - (_e2920 / _e2898));
                                                                    if (_e2930 != _e2930) {
                                                                        phi_26714_ = true;
                                                                    } else {
                                                                        phi_26714_ = (0f >= _e2930);
                                                                    }
                                                                    let _e2934 = phi_26714_;
                                                                    let _e2938 = ((_e2920 / sqrt(_e2898)) + (sqrt(select(_e2921, 0f, _e2925)) * sqrt(select(_e2930, 0f, _e2934))));
                                                                    let _e2940 = (1f - (_e2938 * _e2938));
                                                                    if (_e2940 != _e2940) {
                                                                        phi_26729_ = true;
                                                                    } else {
                                                                        phi_26729_ = (0f >= _e2940);
                                                                    }
                                                                    let _e2944 = phi_26729_;
                                                                    let _e2946 = sqrt(select(_e2940, 0f, _e2944));
                                                                    let _e2947 = (_e2734 * 0.00000037450704f);
                                                                    let _e2948 = sqrt(_e2889);
                                                                    let _e2949 = (_e2882 / _e2948);
                                                                    let _e2950 = (_e2883 / _e2948);
                                                                    let _e2951 = (_e2884 / _e2948);
                                                                    let _e2953 = select(-1f, 1f, (_e2951 >= 0f));
                                                                    let _e2955 = (-1f / (_e2953 + _e2951));
                                                                    let _e2957 = ((_e2949 * _e2950) * _e2955);
                                                                    let _e2970 = (_e2946 * cos(_e2947));
                                                                    let _e2975 = (_e2946 * sin(_e2947));
                                                                    let _e2988 = -(((((1f + (((_e2953 * _e2949) * _e2949) * _e2955)) * _e2970) + (_e2957 * _e2975)) + (_e2949 * _e2938)));
                                                                    let _e2989 = -(((((_e2953 * _e2957) * _e2970) + ((_e2953 + ((_e2950 * _e2950) * _e2955)) * _e2975)) + (_e2950 * _e2938)));
                                                                    let _e2990 = -(((((-(_e2953) * _e2949) * _e2970) + (-(_e2950) * _e2975)) + (_e2951 * _e2938)));
                                                                    phi_16959_ = type_71(vec3<f32>((_e2737.member.x + (_e2988 * _e2737.member.w)), (_e2737.member.y + (_e2989 * _e2737.member.w)), (_e2737.member.z + (_e2990 * _e2737.member.w))), vec3<f32>(_e2988, _e2989, _e2990), (0.15915494f / _e2907));
                                                                }
                                                                let _e3058 = phi_16959_;
                                                                phi_17145_ = _e3058;
                                                            } else {
                                                                let _e2741 = sqrt(_e2718);
                                                                let _e2742 = (1f - _e2741);
                                                                let _e2743 = ((_e2734 * 0.000000059604645f) * _e2741);
                                                                let _e2765 = (_e2741 - _e2743);
                                                                let _e2769 = (((_e2737.member.x * _e2742) + (_e2737.member_1.x * _e2743)) + (_e2737.member_2.x * _e2765));
                                                                let _e2770 = (((_e2737.member.y * _e2742) + (_e2737.member_1.y * _e2743)) + (_e2737.member_2.y * _e2765));
                                                                let _e2771 = (((_e2737.member.z * _e2742) + (_e2737.member_1.z * _e2743)) + (_e2737.member_2.z * _e2765));
                                                                let _e2773 = (_e2737.member_1.x - _e2737.member.x);
                                                                let _e2774 = (_e2737.member_1.y - _e2737.member.y);
                                                                let _e2775 = (_e2737.member_1.z - _e2737.member.z);
                                                                let _e2776 = (_e2737.member_2.x - _e2737.member.x);
                                                                let _e2777 = (_e2737.member_2.y - _e2737.member.y);
                                                                let _e2778 = (_e2737.member_2.z - _e2737.member.z);
                                                                let _e2781 = ((_e2774 * _e2778) - (_e2777 * _e2775));
                                                                let _e2784 = ((_e2775 * _e2776) - (_e2778 * _e2773));
                                                                let _e2787 = ((_e2773 * _e2777) - (_e2776 * _e2774));
                                                                let _e2793 = sqrt((((_e2781 * _e2781) + (_e2784 * _e2784)) + (_e2787 * _e2787)));
                                                                let _e2794 = (1f / _e2793);
                                                                let _e2795 = (_e2781 * _e2794);
                                                                let _e2796 = (_e2784 * _e2794);
                                                                let _e2797 = (_e2787 * _e2794);
                                                                if _e2740 {
                                                                    let _e2830 = (_e2737.member.x - _e1528);
                                                                    let _e2831 = (_e2737.member.y - _e1529);
                                                                    let _e2832 = (_e2737.member.z - _e1530);
                                                                    let _e2837 = (((_e2830 * _e2830) + (_e2831 * _e2831)) + (_e2832 * _e2832));
                                                                    let _e2838 = (_e2737.member.w * _e2737.member.w);
                                                                    if (_e2837 <= (_e2838 * 1.000001f)) {
                                                                        let _e2852 = (_e2769 - _e1528);
                                                                        let _e2853 = (_e2770 - _e1529);
                                                                        let _e2854 = (_e2771 - _e1530);
                                                                        let _e2859 = (((_e2852 * _e2852) + (_e2853 * _e2853)) + (_e2854 * _e2854));
                                                                        phi_26612_ = (_e2859 / (((abs((((_e2795 * _e2852) + (_e2796 * _e2853)) + (_e2797 * _e2854))) / sqrt(_e2859)) * 12.566371f) * _e2838));
                                                                    } else {
                                                                        let _e2841 = (_e2838 / _e2837);
                                                                        if (_e2841 != _e2841) {
                                                                            phi_26631_ = true;
                                                                        } else {
                                                                            phi_26631_ = (1f <= _e2841);
                                                                        }
                                                                        let _e2845 = phi_26631_;
                                                                        let _e2846 = select(_e2841, 1f, _e2845);
                                                                        phi_26612_ = (0.15915494f / (_e2846 / (1f + sqrt((1f - _e2846)))));
                                                                    }
                                                                    let _e2872 = phi_26612_;
                                                                    phi_17143_ = _e2872;
                                                                } else {
                                                                    let _e2799 = (_e2769 - _e1528);
                                                                    let _e2800 = (_e2770 - _e1529);
                                                                    let _e2801 = (_e2771 - _e1530);
                                                                    let _e2806 = (((_e2799 * _e2799) + (_e2800 * _e2800)) + (_e2801 * _e2801));
                                                                    let _e2814 = (abs((((_e2795 * _e2799) + (_e2796 * _e2800)) + (_e2797 * _e2801))) / sqrt(_e2806));
                                                                    if (_e2814 <= 0f) {
                                                                        phi_17142_ = 0f;
                                                                    } else {
                                                                        if (_e2737.member_6.x == 0u) {
                                                                            phi_17139_ = (_e2793 * 0.5f);
                                                                        } else {
                                                                            phi_17139_ = ((12.566371f * _e2737.member.w) * _e2737.member.w);
                                                                        }
                                                                        let _e2823 = phi_17139_;
                                                                        phi_17142_ = (_e2806 / (_e2814 * _e2823));
                                                                    }
                                                                    let _e2827 = phi_17142_;
                                                                    phi_17143_ = _e2827;
                                                                }
                                                                let _e2874 = phi_17143_;
                                                                phi_17145_ = type_71(vec3<f32>(_e2769, _e2770, _e2771), vec3<f32>(_e2795, _e2796, _e2797), _e2874);
                                                            }
                                                            let _e3060 = phi_17145_;
                                                            let _e3065 = (_e3060.member.x - _e1528);
                                                            let _e3067 = (_e3060.member.y - _e1529);
                                                            let _e3069 = (_e3060.member.z - _e1530);
                                                            let _e3075 = sqrt((((_e3065 * _e3065) + (_e3067 * _e3067)) + (_e3069 * _e3069)));
                                                            let _e3076 = (_e3065 / _e3075);
                                                            let _e3077 = (_e3067 / _e3075);
                                                            let _e3078 = (_e3069 / _e3075);
                                                            if (_e3060.member_2 > 0f) {
                                                                let _e3090 = global_4.member[_e2737.member_6.y];
                                                                if (_e3090.member == 4u) {
                                                                    if (_e3090.member_1 == 1u) {
                                                                        phi_17223_ = true;
                                                                    } else {
                                                                        phi_17223_ = ((((-(_e3076) * _e3060.member_1.x) + (-(_e3077) * _e3060.member_1.y)) + (-(_e3078) * _e3060.member_1.z)) > 0f);
                                                                    }
                                                                    let _e3105 = phi_17223_;
                                                                    if _e3105 {
                                                                        phi_17232_ = vec3<f32>(_e3090.member_4.x, _e3090.member_4.y, _e3090.member_4.z);
                                                                    } else {
                                                                        phi_17232_ = vec3<f32>();
                                                                    }
                                                                    let _e3112 = phi_17232_;
                                                                    phi_17234_ = _e3112;
                                                                    phi_17235_ = select(true, false, _e3105);
                                                                } else {
                                                                    phi_17234_ = vec3<f32>();
                                                                    phi_17235_ = true;
                                                                }
                                                                let _e3115 = phi_17234_;
                                                                let _e3117 = phi_17235_;
                                                                phi_17240_ = ((_e2689 - _e2691) * _e3060.member_2);
                                                                phi_17241_ = select(_e3115, vec3<f32>(0f, 0f, 0f), vec3(_e3117));
                                                            } else {
                                                                phi_17240_ = 0f;
                                                                phi_17241_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3121 = phi_17240_;
                                                            let _e3123 = phi_17241_;
                                                            phi_34619_ = _e2731;
                                                            phi_17242_ = _e3121;
                                                            phi_17243_ = vec3<f32>(_e3076, _e3077, _e3078);
                                                            phi_17244_ = _e3123;
                                                            phi_17245_ = ((_e3075 * 0.9999f) - _e1521);
                                                        } else {
                                                            phi_34619_ = _e2410;
                                                            phi_17242_ = 0f;
                                                            phi_17243_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17244_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_17245_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e3125 = phi_34619_;
                                                        let _e3127 = phi_17242_;
                                                        let _e3129 = phi_17243_;
                                                        let _e3131 = phi_17244_;
                                                        let _e3133 = phi_17245_;
                                                        phi_34618_ = _e3125;
                                                        phi_17246_ = _e3127;
                                                        phi_17247_ = _e3129;
                                                        phi_17248_ = _e3131;
                                                        phi_17249_ = _e3133;
                                                    }
                                                    let _e3135 = phi_34618_;
                                                    let _e3137 = phi_17246_;
                                                    let _e3139 = phi_17247_;
                                                    let _e3141 = phi_17248_;
                                                    let _e3143 = phi_17249_;
                                                    phi_34610_ = _e3135;
                                                    phi_17608_ = _e3137;
                                                    phi_17609_ = _e3139;
                                                    phi_17610_ = _e3141;
                                                    phi_17611_ = _e3143;
                                                } else {
                                                    if ((_e358 & 2u) == 0u) {
                                                        phi_34611_ = _e2410;
                                                        phi_17605_ = 0f;
                                                        phi_17606_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_17607_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2428 = ((_e2410 * 747796405u) + 2891336453u);
                                                        let _e2436 = (((_e2428 >> bitcast<u32>((((_e2428 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2428) * 277803737u);
                                                        let _e2439 = ((_e2436 >> bitcast<u32>(22i)) ^ _e2436);
                                                        let _e2443 = (f32((_e2439 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2445 = ((_e2439 * 747796405u) + 2891336453u);
                                                        let _e2453 = (((_e2445 >> bitcast<u32>((((_e2445 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2445) * 277803737u);
                                                        let _e2456 = ((_e2453 >> bitcast<u32>(22i)) ^ _e2453);
                                                        let _e2460 = (f32((_e2456 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2463 = global_1.member.member_3[1u];
                                                        let _e2466 = global_1.member.member_3[2u];
                                                        let _e2469 = global_1.member.member_2[3u];
                                                        phi_17270_ = 0u;
                                                        phi_17273_ = _e2466;
                                                        loop {
                                                            let _e2471 = phi_17270_;
                                                            let _e2473 = phi_17273_;
                                                            local_78 = _e2471;
                                                            local_81 = _e2471;
                                                            local_82 = _e2471;
                                                            let _e2475 = ((_e2471 + 1u) < _e2473);
                                                            if _e2475 {
                                                                let _e2477 = ((_e2471 + _e2473) / 2u);
                                                                let _e2480 = global_6.member[(_e2469 + _e2477)];
                                                                let _e2481 = (_e2480 <= _e2443);
                                                                phi_17271_ = select(_e2471, _e2477, _e2481);
                                                                phi_17274_ = select(_e2477, _e2473, _e2481);
                                                            } else {
                                                                phi_17271_ = u32();
                                                                phi_17274_ = u32();
                                                            }
                                                            let _e2485 = phi_17271_;
                                                            let _e2487 = phi_17274_;
                                                            phi_17270_ = _e2485;
                                                            phi_17273_ = _e2487;
                                                            if !(_e2475) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2491 = global_1.member.member_3[0u];
                                                        let _e2492 = (_e2463 + 1u);
                                                        let _e2494 = local_78;
                                                        let _e2496 = (_e2491 + (_e2494 * _e2492));
                                                        phi_17311_ = 0u;
                                                        phi_17314_ = _e2463;
                                                        loop {
                                                            let _e2498 = phi_17311_;
                                                            let _e2500 = phi_17314_;
                                                            local_79 = _e2498;
                                                            local_80 = _e2498;
                                                            let _e2502 = ((_e2498 + 1u) < _e2500);
                                                            if _e2502 {
                                                                let _e2504 = ((_e2498 + _e2500) / 2u);
                                                                let _e2507 = global_6.member[(_e2496 + _e2504)];
                                                                let _e2508 = (_e2507 <= _e2460);
                                                                phi_17312_ = select(_e2498, _e2504, _e2508);
                                                                phi_17315_ = select(_e2504, _e2500, _e2508);
                                                            } else {
                                                                phi_17312_ = u32();
                                                                phi_17315_ = u32();
                                                            }
                                                            let _e2512 = phi_17312_;
                                                            let _e2514 = phi_17315_;
                                                            phi_17311_ = _e2512;
                                                            phi_17314_ = _e2514;
                                                            if !(_e2502) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2517 = local_79;
                                                        let _e2520 = local_80;
                                                        let _e2521 = (_e2496 + _e2520);
                                                        let _e2523 = global_6.member[_e2521];
                                                        let _e2526 = global_6.member[(_e2521 + 1u)];
                                                        if (_e2526 > _e2523) {
                                                            let _e2530 = ((_e2460 - _e2523) / (_e2526 - _e2523));
                                                            let _e2532 = select(_e2530, 0f, (_e2530 < 0f));
                                                            phi_17391_ = select(_e2532, 0.99999f, (_e2532 > 0.99999f));
                                                        } else {
                                                            phi_17391_ = 0.5f;
                                                        }
                                                        let _e2536 = phi_17391_;
                                                        let _e2538 = f32(_e2463);
                                                        let _e2541 = local_81;
                                                        let _e2544 = local_82;
                                                        let _e2545 = (_e2469 + _e2544);
                                                        let _e2547 = global_6.member[_e2545];
                                                        let _e2550 = global_6.member[(_e2545 + 1u)];
                                                        if (_e2550 > _e2547) {
                                                            let _e2554 = ((_e2443 - _e2547) / (_e2550 - _e2547));
                                                            let _e2556 = select(_e2554, 0f, (_e2554 < 0f));
                                                            phi_17440_ = select(_e2556, 0.99999f, (_e2556 > 0.99999f));
                                                        } else {
                                                            phi_17440_ = 0.5f;
                                                        }
                                                        let _e2560 = phi_17440_;
                                                        let _e2562 = f32(_e2466);
                                                        let _e2565 = ((((f32(_e2517) + _e2536) / _e2538) - 0.5f) * 6.2831855f);
                                                        let _e2566 = (((f32(_e2541) + _e2560) / _e2562) * 3.1415927f);
                                                        let _e2567 = sin(_e2566);
                                                        let _e2569 = (_e2567 * cos(_e2565));
                                                        let _e2570 = cos(_e2566);
                                                        let _e2572 = (_e2567 * sin(_e2565));
                                                        let _e2575 = ((atan2(_e2572, _e2569) * 0.15915494f) + 0.5f);
                                                        let _e2579 = select(_e2570, -1f, (_e2570 < -1f));
                                                        let _e2582 = acos(select(_e2579, 1f, (_e2579 > 1f)));
                                                        let _e2584 = ((_e2575 - floor(_e2575)) * _e2538);
                                                        let _e2589 = select(select(u32(_e2584), 0u, (_e2584 < 0f)), 4294967295u, (_e2584 > 4294967000f));
                                                        let _e2590 = (_e2463 - 1u);
                                                        let _e2593 = ((_e2582 * 0.31830987f) * _e2562);
                                                        let _e2598 = select(select(u32(_e2593), 0u, (_e2593 < 0f)), 4294967295u, (_e2593 > 4294967000f));
                                                        let _e2599 = (_e2466 - 1u);
                                                        let _e2601 = select(_e2598, _e2599, (_e2599 < _e2598));
                                                        let _e2602 = sin(_e2582);
                                                        if (_e2602 <= 0f) {
                                                            phi_17596_ = 0f;
                                                        } else {
                                                            let _e2606 = (_e2469 + _e2601);
                                                            let _e2609 = global_6.member[(_e2606 + 1u)];
                                                            let _e2611 = global_6.member[_e2606];
                                                            let _e2614 = ((_e2491 + (_e2601 * _e2492)) + select(_e2589, _e2590, (_e2590 < _e2589)));
                                                            let _e2617 = global_6.member[(_e2614 + 1u)];
                                                            let _e2619 = global_6.member[_e2614];
                                                            phi_17596_ = ((((_e2609 - _e2611) * _e2562) * ((_e2617 - _e2619) * _e2538)) / (19.73921f * _e2602));
                                                        }
                                                        let _e2626 = phi_17596_;
                                                        let _e2627 = (_e2626 > 0f);
                                                        if _e2627 {
                                                            phi_17603_ = vec3<f32>(_e2569, _e2570, _e2572);
                                                        } else {
                                                            phi_17603_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2630 = phi_17603_;
                                                        phi_34611_ = _e2456;
                                                        phi_17605_ = select(0f, _e2626, _e2627);
                                                        phi_17606_ = _e2630;
                                                        phi_17607_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2627));
                                                    }
                                                    let _e2635 = phi_34611_;
                                                    let _e2637 = phi_17605_;
                                                    let _e2639 = phi_17606_;
                                                    let _e2641 = phi_17607_;
                                                    phi_34610_ = _e2635;
                                                    phi_17608_ = _e2637;
                                                    phi_17609_ = _e2639;
                                                    phi_17610_ = _e2641;
                                                    phi_17611_ = 1000000000000000000000000000000f;
                                                }
                                                let _e3145 = phi_34610_;
                                                let _e3147 = phi_17608_;
                                                let _e3149 = phi_17609_;
                                                let _e3151 = phi_17610_;
                                                let _e3153 = phi_17611_;
                                                phi_34609_ = _e3145;
                                                phi_17612_ = _e3147;
                                                phi_17613_ = _e3149;
                                                phi_17614_ = _e3151;
                                                phi_17615_ = _e3153;
                                            }
                                            let _e4395 = phi_34609_;
                                            let _e4397 = phi_17612_;
                                            let _e4399 = phi_17613_;
                                            let _e4401 = phi_17614_;
                                            let _e4403 = phi_17615_;
                                            phi_34608_ = _e4395;
                                            phi_17616_ = _e4397;
                                            phi_17617_ = _e4399;
                                            phi_17618_ = _e4401;
                                            phi_17619_ = _e4403;
                                        }
                                        let _e4472 = phi_34608_;
                                        let _e4474 = phi_17616_;
                                        let _e4476 = phi_17617_;
                                        let _e4478 = phi_17618_;
                                        let _e4480 = phi_17619_;
                                        if ((((_e4478.x * _e4478.x) + (_e4478.y * _e4478.y)) + (_e4478.z * _e4478.z)) > 0f) {
                                            let _e4497 = (((_e4476.x * _e1503.x) + (_e4476.y * _e1503.y)) + (_e4476.z * _e1503.z));
                                            if (_e4497 != _e4497) {
                                                phi_28196_ = true;
                                            } else {
                                                phi_28196_ = (0f >= _e4497);
                                            }
                                            let _e4501 = phi_28196_;
                                            let _e4503 = (select(_e4497, 0f, _e4501) * 0.31830987f);
                                            if (_e2395 == 0u) {
                                                if (_e4497 <= 0f) {
                                                    phi_28993_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_28993_ = (_e2389 * (_e4497 * 0.31830987f));
                                                }
                                                let _e4979 = phi_28993_;
                                                phi_18015_ = type_55(_e4979, _e4503);
                                                phi_18016_ = type_55();
                                                phi_18017_ = true;
                                            } else {
                                                let _e4509 = (((_e1475 * _e1551) + (_e1476 * _e1552)) + (_e1477 * _e1554));
                                                let _e4514 = (((_e1475 * _e1547) + (_e1476 * _e1557)) + (_e1477 * _e1558));
                                                let _e4519 = (((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z));
                                                let _e4524 = (((_e4476.x * _e1551) + (_e4476.y * _e1552)) + (_e4476.z * _e1554));
                                                let _e4529 = (((_e4476.x * _e1547) + (_e4476.y * _e1557)) + (_e4476.z * _e1558));
                                                if (_e4519 <= 0f) {
                                                    phi_17979_ = type_55();
                                                    phi_17980_ = true;
                                                } else {
                                                    let _e4531 = (_e4497 <= 0f);
                                                    if _e4531 {
                                                        phi_17978_ = type_55();
                                                    } else {
                                                        let _e4532 = (_e4509 + _e4524);
                                                        let _e4533 = (_e4514 + _e4529);
                                                        let _e4534 = (_e4519 + _e4497);
                                                        let _e4541 = (1f / sqrt((((_e4532 * _e4532) + (_e4533 * _e4533)) + (_e4534 * _e4534))));
                                                        let _e4544 = (_e4534 * _e4541);
                                                        if (_e4544 <= 0f) {
                                                            phi_28306_ = 0f;
                                                        } else {
                                                            let _e4546 = (_e2393 * _e2393);
                                                            let _e4550 = ((((_e4546 - 1f) * _e4544) * _e4544) + 1f);
                                                            phi_28306_ = (_e4546 / ((3.1415927f * _e4550) * _e4550));
                                                        }
                                                        let _e4555 = phi_28306_;
                                                        let _e4556 = (_e4519 * _e4519);
                                                        if (_e4556 >= 1f) {
                                                            phi_28334_ = 0f;
                                                        } else {
                                                            phi_28334_ = ((sqrt((1f + (((_e2393 * _e2393) * ((_e4509 * _e4509) + (_e4514 * _e4514))) / _e4556))) - 1f) * 0.5f);
                                                        }
                                                        let _e4569 = phi_28334_;
                                                        let _e4570 = (1f + _e4569);
                                                        let _e4571 = (_e4497 * _e4497);
                                                        if (_e4571 >= 1f) {
                                                            phi_28362_ = 0f;
                                                        } else {
                                                            phi_28362_ = ((sqrt((1f + (((_e2393 * _e2393) * ((_e4524 * _e4524) + (_e4529 * _e4529))) / _e4571))) - 1f) * 0.5f);
                                                        }
                                                        let _e4584 = phi_28362_;
                                                        let _e4591 = (((_e4509 * (_e4532 * _e4541)) + (_e4514 * (_e4533 * _e4541))) + (_e4519 * _e4544));
                                                        if (_e2391 == 0u) {
                                                            let _e4886 = (1f - _e4591);
                                                            if (_e4886 != _e4886) {
                                                                phi_28865_ = true;
                                                            } else {
                                                                phi_28865_ = (0f >= _e4886);
                                                            }
                                                            let _e4890 = phi_28865_;
                                                            let _e4892 = pow(select(_e4886, 0f, _e4890), 5f);
                                                            let _e4893 = (1f - _e4892);
                                                            phi_17966_ = vec3<f32>(((_e2389.x * _e4893) + _e4892), ((_e2389.y * _e4893) + _e4892), ((_e2389.z * _e4893) + _e4892));
                                                        } else {
                                                            if (_e2391 == 1u) {
                                                                let _e4859 = (1f / _e2385);
                                                                let _e4863 = ((_e4859 * _e4859) * (1f - (_e4591 * _e4591)));
                                                                if (_e4863 >= 1f) {
                                                                    phi_28826_ = 1f;
                                                                } else {
                                                                    let _e4866 = sqrt((1f - _e4863));
                                                                    if (_e4591 != _e4591) {
                                                                        phi_28837_ = true;
                                                                    } else {
                                                                        phi_28837_ = (_e4866 <= _e4591);
                                                                    }
                                                                    let _e4870 = phi_28837_;
                                                                    let _e4875 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                                    phi_28826_ = (_e4875 + ((1f - _e4875) * pow((1f - select(_e4591, _e4866, _e4870)), 5f)));
                                                                }
                                                                let _e4882 = phi_28826_;
                                                                phi_17965_ = vec3(_e4882);
                                                            } else {
                                                                let _e4595 = select(_e4591, 0f, (_e4591 < 0f));
                                                                let _e4597 = select(_e4595, 1f, (_e4595 > 1f));
                                                                let _e4599 = (1f - (_e4597 * _e4597));
                                                                let _e4612 = ((_e2389.x * _e2389.x) - (_e2387.x * _e2387.x));
                                                                let _e4613 = ((_e2389.y * _e2389.y) - (_e2387.y * _e2387.y));
                                                                let _e4614 = ((_e2389.z * _e2389.z) - (_e2387.z * _e2387.z));
                                                                let _e4616 = (_e2389.x * (_e2387.x + _e2387.x));
                                                                let _e4618 = (_e2389.y * (_e2387.y + _e2387.y));
                                                                let _e4620 = (_e2389.z * (_e2387.z + _e2387.z));
                                                                let _e4627 = ((_e4612 * _e4612) + (_e4616 * _e4616));
                                                                let _e4628 = ((_e4613 * _e4613) + (_e4618 * _e4618));
                                                                let _e4629 = ((_e4614 * _e4614) + (_e4620 * _e4620));
                                                                let _e4634 = ((_e4599 * _e4612) / _e4627);
                                                                let _e4635 = ((_e4599 * _e4613) / _e4628);
                                                                let _e4636 = ((_e4599 * _e4614) / _e4629);
                                                                let _e4637 = (1f - _e4634);
                                                                let _e4638 = (1f - _e4635);
                                                                let _e4639 = (1f - _e4636);
                                                                let _e4641 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4634, _e4635, _e4636));
                                                                let _e4642 = (_e4599 * _e4616);
                                                                let _e4643 = (_e4599 * _e4618);
                                                                let _e4644 = (_e4599 * _e4620);
                                                                let _e4649 = (vec3<f32>(_e4642, _e4643, _e4644) / vec3<f32>(_e4627, _e4628, _e4629));
                                                                let _e4662 = ((_e4641.x * _e4641.x) + (_e4649.x * _e4649.x));
                                                                let _e4663 = ((_e4641.y * _e4641.y) + (_e4649.y * _e4649.y));
                                                                let _e4664 = ((_e4641.z * _e4641.z) + (_e4649.z * _e4649.z));
                                                                if (_e4662 != _e4662) {
                                                                    phi_28496_ = true;
                                                                } else {
                                                                    phi_28496_ = (0f >= _e4662);
                                                                }
                                                                let _e4668 = phi_28496_;
                                                                let _e4670 = sqrt(select(_e4662, 0f, _e4668));
                                                                if (_e4663 != _e4663) {
                                                                    phi_28511_ = true;
                                                                } else {
                                                                    phi_28511_ = (0f >= _e4663);
                                                                }
                                                                let _e4674 = phi_28511_;
                                                                let _e4676 = sqrt(select(_e4663, 0f, _e4674));
                                                                if (_e4664 != _e4664) {
                                                                    phi_28526_ = true;
                                                                } else {
                                                                    phi_28526_ = (0f >= _e4664);
                                                                }
                                                                let _e4680 = phi_28526_;
                                                                let _e4682 = sqrt(select(_e4664, 0f, _e4680));
                                                                let _e4686 = ((_e4670 + _e4637) * 0.5f);
                                                                let _e4687 = ((_e4676 + _e4638) * 0.5f);
                                                                let _e4688 = ((_e4682 + _e4639) * 0.5f);
                                                                if (_e4686 != _e4686) {
                                                                    phi_28553_ = true;
                                                                } else {
                                                                    phi_28553_ = (0f >= _e4686);
                                                                }
                                                                let _e4692 = phi_28553_;
                                                                let _e4694 = sqrt(select(_e4686, 0f, _e4692));
                                                                if (_e4687 != _e4687) {
                                                                    phi_28568_ = true;
                                                                } else {
                                                                    phi_28568_ = (0f >= _e4687);
                                                                }
                                                                let _e4698 = phi_28568_;
                                                                let _e4700 = sqrt(select(_e4687, 0f, _e4698));
                                                                if (_e4688 != _e4688) {
                                                                    phi_28583_ = true;
                                                                } else {
                                                                    phi_28583_ = (0f >= _e4688);
                                                                }
                                                                let _e4704 = phi_28583_;
                                                                let _e4706 = sqrt(select(_e4688, 0f, _e4704));
                                                                let _e4707 = vec3<f32>(_e4694, _e4700, _e4706);
                                                                let _e4711 = ((_e4670 - _e4637) * 0.5f);
                                                                let _e4712 = ((_e4676 - _e4638) * 0.5f);
                                                                let _e4713 = ((_e4682 - _e4639) * 0.5f);
                                                                if (_e4711 != _e4711) {
                                                                    phi_28610_ = true;
                                                                } else {
                                                                    phi_28610_ = (0f >= _e4711);
                                                                }
                                                                let _e4717 = phi_28610_;
                                                                let _e4719 = sqrt(select(_e4711, 0f, _e4717));
                                                                if (_e4712 != _e4712) {
                                                                    phi_28625_ = true;
                                                                } else {
                                                                    phi_28625_ = (0f >= _e4712);
                                                                }
                                                                let _e4723 = phi_28625_;
                                                                let _e4725 = sqrt(select(_e4712, 0f, _e4723));
                                                                if (_e4713 != _e4713) {
                                                                    phi_28640_ = true;
                                                                } else {
                                                                    phi_28640_ = (0f >= _e4713);
                                                                }
                                                                let _e4729 = phi_28640_;
                                                                let _e4731 = sqrt(select(_e4713, 0f, _e4729));
                                                                if ((_e4642 / _e4627) < 0f) {
                                                                    phi_17842_ = -(_e4719);
                                                                } else {
                                                                    phi_17842_ = _e4719;
                                                                }
                                                                let _e4735 = phi_17842_;
                                                                if ((_e4643 / _e4628) < 0f) {
                                                                    phi_17850_ = -(_e4725);
                                                                } else {
                                                                    phi_17850_ = _e4725;
                                                                }
                                                                let _e4739 = phi_17850_;
                                                                if ((_e4644 / _e4629) < 0f) {
                                                                    phi_17858_ = -(_e4731);
                                                                } else {
                                                                    phi_17858_ = _e4731;
                                                                }
                                                                let _e4743 = phi_17858_;
                                                                let _e4747 = (_e2387.x * _e4597);
                                                                let _e4748 = (_e2387.y * _e4597);
                                                                let _e4749 = (_e2387.z * _e4597);
                                                                let _e4759 = vec3<f32>(((_e2389.x * _e4694) - (_e2387.x * _e4735)), ((_e2389.y * _e4700) - (_e2387.y * _e4739)), ((_e2389.z * _e4706) - (_e2387.z * _e4743)));
                                                                let _e4766 = ((_e2389.x * _e4735) + (_e2387.x * _e4694));
                                                                let _e4767 = ((_e2389.y * _e4739) + (_e2387.y * _e4700));
                                                                let _e4768 = ((_e2389.z * _e4743) + (_e2387.z * _e4706));
                                                                let _e4770 = vec3<f32>((_e2389.x * _e4597), (_e2389.y * _e4597), (_e2389.z * _e4597));
                                                                let _e4771 = (_e4770 - _e4707);
                                                                let _e4772 = (_e4747 - _e4735);
                                                                let _e4773 = (_e4748 - _e4739);
                                                                let _e4774 = (_e4749 - _e4743);
                                                                let _e4787 = (_e4770 + _e4707);
                                                                let _e4788 = (_e4747 + _e4735);
                                                                let _e4789 = (_e4748 + _e4739);
                                                                let _e4790 = (_e4749 + _e4743);
                                                                let _e4800 = ((_e4787.x * _e4787.x) + (_e4788 * _e4788));
                                                                let _e4801 = ((_e4787.y * _e4787.y) + (_e4789 * _e4789));
                                                                let _e4802 = ((_e4787.z * _e4787.z) + (_e4790 * _e4790));
                                                                let _e4812 = vec3(_e4597);
                                                                let _e4813 = (_e4812 - _e4759);
                                                                let _e4814 = -(vec3<f32>(_e4766, _e4767, _e4768));
                                                                let _e4830 = (_e4812 + _e4759);
                                                                let _e4840 = ((_e4830.x * _e4830.x) + (_e4766 * _e4766));
                                                                let _e4841 = ((_e4830.y * _e4830.y) + (_e4767 * _e4767));
                                                                let _e4842 = ((_e4830.z * _e4830.z) + (_e4768 * _e4768));
                                                                phi_17965_ = vec3<f32>((((((_e4771.x * _e4771.x) + (_e4772 * _e4772)) / select(0.000000000000000000000000000001f, _e4800, (_e4800 > 0.000000000000000000000000000001f))) + (((_e4813.x * _e4813.x) + (_e4814.x * _e4814.x)) / select(0.000000000000000000000000000001f, _e4840, (_e4840 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4771.y * _e4771.y) + (_e4773 * _e4773)) / select(0.000000000000000000000000000001f, _e4801, (_e4801 > 0.000000000000000000000000000001f))) + (((_e4813.y * _e4813.y) + (_e4814.y * _e4814.y)) / select(0.000000000000000000000000000001f, _e4841, (_e4841 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4771.z * _e4771.z) + (_e4774 * _e4774)) / select(0.000000000000000000000000000001f, _e4802, (_e4802 > 0.000000000000000000000000000001f))) + (((_e4813.z * _e4813.z) + (_e4814.z * _e4814.z)) / select(0.000000000000000000000000000001f, _e4842, (_e4842 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e4885 = phi_17965_;
                                                            phi_17966_ = _e4885;
                                                        }
                                                        let _e4905 = phi_17966_;
                                                        let _e4907 = (4f * _e4519);
                                                        phi_17978_ = type_55((_e4905 * (((1f / (_e4570 + _e4584)) * _e4555) / _e4907)), ((_e4555 / _e4570) / _e4907));
                                                    }
                                                    let _e4914 = phi_17978_;
                                                    phi_17979_ = _e4914;
                                                    phi_17980_ = _e4531;
                                                }
                                                let _e4916 = phi_17979_;
                                                let _e4918 = phi_17980_;
                                                if _e4918 {
                                                    phi_17985_ = type_55(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_17985_ = _e4916;
                                                }
                                                let _e4920 = phi_17985_;
                                                let _e4923 = (_e2395 == 1u);
                                                if _e4923 {
                                                    phi_18013_ = _e4920;
                                                    phi_18014_ = type_55();
                                                } else {
                                                    if (_e4519 != _e4519) {
                                                        phi_28902_ = true;
                                                    } else {
                                                        phi_28902_ = (0f >= _e4519);
                                                    }
                                                    let _e4927 = phi_28902_;
                                                    let _e4928 = select(_e4519, 0f, _e4927);
                                                    let _e4929 = (1f / _e2385);
                                                    let _e4933 = ((_e4929 * _e4929) * (1f - (_e4928 * _e4928)));
                                                    if (_e4933 >= 1f) {
                                                        phi_28921_ = 1f;
                                                    } else {
                                                        let _e4936 = sqrt((1f - _e4933));
                                                        if (_e4928 != _e4928) {
                                                            phi_28932_ = true;
                                                        } else {
                                                            phi_28932_ = (_e4936 <= _e4928);
                                                        }
                                                        let _e4940 = phi_28932_;
                                                        let _e4945 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                        phi_28921_ = (_e4945 + ((1f - _e4945) * pow((1f - select(_e4928, _e4936, _e4940)), 5f)));
                                                    }
                                                    let _e4952 = phi_28921_;
                                                    if (_e4497 <= 0f) {
                                                        phi_28971_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_28971_ = (_e2383 * (_e4497 * 0.31830987f));
                                                    }
                                                    let _e4957 = phi_28971_;
                                                    let _e4958 = (1f - _e4952);
                                                    phi_18013_ = type_55();
                                                    phi_18014_ = type_55((_e4920.member + vec3<f32>((_e4957.x * _e4958), (_e4957.y * _e4958), (_e4957.z * _e4958))), ((_e4952 * _e4920.member_1) + (_e4958 * _e4503)));
                                                }
                                                let _e4972 = phi_18013_;
                                                let _e4974 = phi_18014_;
                                                phi_18015_ = _e4972;
                                                phi_18016_ = _e4974;
                                                phi_18017_ = _e4923;
                                            }
                                            let _e4982 = phi_18015_;
                                            let _e4984 = phi_18016_;
                                            let _e4986 = phi_18017_;
                                            if _e4986 {
                                                phi_18021_ = _e4982;
                                            } else {
                                                phi_18021_ = _e4984;
                                            }
                                            let _e4988 = phi_18021_;
                                            if (_e4474 > 0f) {
                                                if (_e4474 >= 300000000000000000000000000000000000000f) {
                                                    phi_29010_ = 1f;
                                                } else {
                                                    let _e4993 = (_e4474 * _e4474);
                                                    let _e4995 = (_e4993 + (_e4988.member_1 * _e4988.member_1));
                                                    if (_e4995 == 0f) {
                                                        phi_29007_ = 0f;
                                                    } else {
                                                        phi_29007_ = (_e4993 / _e4995);
                                                    }
                                                    let _e4999 = phi_29007_;
                                                    phi_29010_ = _e4999;
                                                }
                                                let _e5001 = phi_29010_;
                                                phi_18030_ = (_e5001 / _e4474);
                                            } else {
                                                phi_18030_ = 1f;
                                            }
                                            let _e5004 = phi_18030_;
                                            if ((((_e4988.member.x * _e4988.member.x) + (_e4988.member.y * _e4988.member.y)) + (_e4988.member.z * _e4988.member.z)) > 0f) {
                                                let _e5016 = (vec3<f32>(1f, 1f, 1f) / _e4476);
                                                local_67 = array<u32, 100>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                phi_18064_ = _e364;
                                                phi_18067_ = _e371;
                                                phi_18069_ = _e368;
                                                phi_18073_ = type_63(_e4480, 4294967295u, vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                loop {
                                                    let _e5018 = phi_18064_;
                                                    let _e5020 = phi_18067_;
                                                    let _e5022 = phi_18069_;
                                                    let _e5024 = phi_18073_;
                                                    local_106 = _e5024;
                                                    if (_e5018 < _e5022) {
                                                        let _e5193 = global_2.member[_e5018];
                                                        if (_e5193.member_6.x == 0u) {
                                                            let _e5299 = (_e5193.member_1.x - _e5193.member.x);
                                                            let _e5300 = (_e5193.member_1.y - _e5193.member.y);
                                                            let _e5301 = (_e5193.member_1.z - _e5193.member.z);
                                                            let _e5306 = (_e5193.member_2.x - _e5193.member.x);
                                                            let _e5307 = (_e5193.member_2.y - _e5193.member.y);
                                                            let _e5308 = (_e5193.member_2.z - _e5193.member.z);
                                                            let _e5311 = ((_e4476.y * _e5308) - (_e5307 * _e4476.z));
                                                            let _e5314 = ((_e4476.z * _e5306) - (_e5308 * _e4476.x));
                                                            let _e5317 = ((_e4476.x * _e5307) - (_e5306 * _e4476.y));
                                                            let _e5322 = (((_e5299 * _e5311) + (_e5300 * _e5314)) + (_e5301 * _e5317));
                                                            if (abs(_e5322) < 0.000000000001f) {
                                                                phi_18240_ = vec4<f32>();
                                                                phi_18241_ = true;
                                                            } else {
                                                                let _e5325 = (1f / _e5322);
                                                                let _e5326 = (_e1528 - _e5193.member.x);
                                                                let _e5327 = (_e1529 - _e5193.member.y);
                                                                let _e5328 = (_e1530 - _e5193.member.z);
                                                                let _e5334 = ((((_e5326 * _e5311) + (_e5327 * _e5314)) + (_e5328 * _e5317)) * _e5325);
                                                                if (_e5334 < 0f) {
                                                                    phi_18230_ = vec4<f32>();
                                                                    phi_18231_ = true;
                                                                    phi_18232_ = false;
                                                                } else {
                                                                    let _e5336 = (_e5334 > 1f);
                                                                    if _e5336 {
                                                                        phi_18228_ = vec4<f32>();
                                                                        phi_18229_ = false;
                                                                    } else {
                                                                        let _e5339 = ((_e5327 * _e5301) - (_e5300 * _e5328));
                                                                        let _e5342 = ((_e5328 * _e5299) - (_e5301 * _e5326));
                                                                        let _e5345 = ((_e5326 * _e5300) - (_e5299 * _e5327));
                                                                        let _e5351 = ((((_e4476.x * _e5339) + (_e4476.y * _e5342)) + (_e4476.z * _e5345)) * _e5325);
                                                                        if (_e5351 < 0f) {
                                                                            phi_18218_ = vec4<f32>();
                                                                            phi_18219_ = true;
                                                                            phi_18220_ = false;
                                                                        } else {
                                                                            let _e5354 = ((_e5334 + _e5351) > 1f);
                                                                            if _e5354 {
                                                                                phi_18216_ = vec4<f32>();
                                                                                phi_18217_ = false;
                                                                            } else {
                                                                                let _e5360 = ((((_e5306 * _e5339) + (_e5307 * _e5342)) + (_e5308 * _e5345)) * _e5325);
                                                                                if (_e5360 <= 0f) {
                                                                                    phi_18211_ = vec4<f32>();
                                                                                    phi_18212_ = true;
                                                                                } else {
                                                                                    let _e5362 = (_e5360 >= _e5024.member);
                                                                                    if _e5362 {
                                                                                        phi_18210_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_18210_ = vec4<f32>(_e5360, _e5334, _e5351, 1f);
                                                                                    }
                                                                                    let _e5365 = phi_18210_;
                                                                                    phi_18211_ = _e5365;
                                                                                    phi_18212_ = _e5362;
                                                                                }
                                                                                let _e5367 = phi_18211_;
                                                                                let _e5369 = phi_18212_;
                                                                                phi_18216_ = _e5367;
                                                                                phi_18217_ = _e5369;
                                                                            }
                                                                            let _e5371 = phi_18216_;
                                                                            let _e5373 = phi_18217_;
                                                                            phi_18218_ = _e5371;
                                                                            phi_18219_ = _e5354;
                                                                            phi_18220_ = _e5373;
                                                                        }
                                                                        let _e5375 = phi_18218_;
                                                                        let _e5377 = phi_18219_;
                                                                        let _e5379 = phi_18220_;
                                                                        phi_18228_ = _e5375;
                                                                        phi_18229_ = select(_e5379, true, _e5377);
                                                                    }
                                                                    let _e5382 = phi_18228_;
                                                                    let _e5384 = phi_18229_;
                                                                    phi_18230_ = _e5382;
                                                                    phi_18231_ = _e5336;
                                                                    phi_18232_ = _e5384;
                                                                }
                                                                let _e5386 = phi_18230_;
                                                                let _e5388 = phi_18231_;
                                                                let _e5390 = phi_18232_;
                                                                phi_18240_ = _e5386;
                                                                phi_18241_ = select(_e5390, true, _e5388);
                                                            }
                                                            let _e5393 = phi_18240_;
                                                            let _e5395 = phi_18241_;
                                                            phi_18387_ = _e5393;
                                                            phi_18388_ = _e5395;
                                                        } else {
                                                            if (_e5193.member_6.x == 1u) {
                                                                let _e5221 = (_e1528 - _e5193.member.x);
                                                                let _e5222 = (_e1529 - _e5193.member.y);
                                                                let _e5223 = (_e1530 - _e5193.member.z);
                                                                let _e5228 = (((_e5221 * _e4476.x) + (_e5222 * _e4476.y)) + (_e5223 * _e4476.z));
                                                                if (_e5228 > 0f) {
                                                                    phi_18281_ = select(true, false, ((((_e5221 * _e5221) + (_e5222 * _e5222)) + (_e5223 * _e5223)) > (_e5193.member.w * _e5193.member.w)));
                                                                } else {
                                                                    phi_18281_ = true;
                                                                }
                                                                let _e5241 = phi_18281_;
                                                                if _e5241 {
                                                                    let _e5245 = (_e5221 - (_e4476.x * _e5228));
                                                                    let _e5246 = (_e5222 - (_e4476.y * _e5228));
                                                                    let _e5247 = (_e5223 - (_e4476.z * _e5228));
                                                                    let _e5256 = ((_e5193.member.w * _e5193.member.w) - (((_e5245 * _e5245) + (_e5246 * _e5246)) + (_e5247 * _e5247)));
                                                                    if (_e5256 < 0f) {
                                                                        phi_18340_ = vec4<f32>();
                                                                        phi_18341_ = true;
                                                                    } else {
                                                                        let _e5258 = sqrt(_e5256);
                                                                        let _e5259 = -(_e5228);
                                                                        let _e5260 = (_e5259 - _e5258);
                                                                        if (_e5260 <= 0f) {
                                                                            phi_18318_ = true;
                                                                        } else {
                                                                            phi_18318_ = (_e5260 >= _e5024.member);
                                                                        }
                                                                        let _e5264 = phi_18318_;
                                                                        if _e5264 {
                                                                            phi_18324_ = (_e5259 + _e5258);
                                                                        } else {
                                                                            phi_18324_ = _e5260;
                                                                        }
                                                                        let _e5267 = phi_18324_;
                                                                        if (_e5267 <= 0f) {
                                                                            phi_18335_ = vec4<f32>();
                                                                            phi_18336_ = true;
                                                                        } else {
                                                                            let _e5269 = (_e5267 >= _e5024.member);
                                                                            if _e5269 {
                                                                                phi_18334_ = vec4<f32>();
                                                                            } else {
                                                                                phi_18334_ = vec4<f32>(_e5267, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5272 = phi_18334_;
                                                                            phi_18335_ = _e5272;
                                                                            phi_18336_ = _e5269;
                                                                        }
                                                                        let _e5274 = phi_18335_;
                                                                        let _e5276 = phi_18336_;
                                                                        phi_18340_ = _e5274;
                                                                        phi_18341_ = _e5276;
                                                                    }
                                                                    let _e5278 = phi_18340_;
                                                                    let _e5280 = phi_18341_;
                                                                    phi_18345_ = _e5278;
                                                                    phi_18346_ = _e5280;
                                                                } else {
                                                                    phi_18345_ = vec4<f32>();
                                                                    phi_18346_ = true;
                                                                }
                                                                let _e5282 = phi_18345_;
                                                                let _e5284 = phi_18346_;
                                                                phi_18377_ = _e5282;
                                                                phi_18378_ = _e5284;
                                                            } else {
                                                                if (abs(_e4476.y) < 0.000000000001f) {
                                                                    phi_18375_ = vec4<f32>();
                                                                    phi_18376_ = true;
                                                                } else {
                                                                    let _e5203 = ((_e5193.member.x - _e1529) / _e4476.y);
                                                                    if (_e5203 <= 0f) {
                                                                        phi_18370_ = vec4<f32>();
                                                                        phi_18371_ = true;
                                                                    } else {
                                                                        let _e5205 = (_e5203 >= _e5024.member);
                                                                        if _e5205 {
                                                                            phi_18369_ = vec4<f32>();
                                                                        } else {
                                                                            phi_18369_ = vec4<f32>(_e5203, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5208 = phi_18369_;
                                                                        phi_18370_ = _e5208;
                                                                        phi_18371_ = _e5205;
                                                                    }
                                                                    let _e5210 = phi_18370_;
                                                                    let _e5212 = phi_18371_;
                                                                    phi_18375_ = _e5210;
                                                                    phi_18376_ = _e5212;
                                                                }
                                                                let _e5214 = phi_18375_;
                                                                let _e5216 = phi_18376_;
                                                                phi_18377_ = _e5214;
                                                                phi_18378_ = _e5216;
                                                            }
                                                            let _e5286 = phi_18377_;
                                                            let _e5288 = phi_18378_;
                                                            phi_18387_ = _e5286;
                                                            phi_18388_ = select(true, false, select(true, false, _e5288));
                                                        }
                                                        let _e5397 = phi_18387_;
                                                        let _e5399 = phi_18388_;
                                                        let _e5402 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5397, vec4(select(true, false, _e5399)));
                                                        let _e5404 = (_e5402.w > 0f);
                                                        if _e5404 {
                                                            let _e5410 = type_63(_e5402.x, _e5024.member_1, _e5024.member_2, _e5024.member_3);
                                                            let _e5415 = type_63(_e5410.member, _e5018, _e5410.member_2, _e5410.member_3);
                                                            phi_18409_ = _e5415;
                                                            phi_18411_ = _e5415;
                                                        } else {
                                                            phi_18409_ = type_63();
                                                            phi_18411_ = _e5024;
                                                        }
                                                        let _e5417 = phi_18409_;
                                                        let _e5419 = phi_18411_;
                                                        let _e5420 = select(true, false, _e5404);
                                                        if _e5420 {
                                                            phi_18417_ = (_e5018 + 1u);
                                                        } else {
                                                            phi_18417_ = u32();
                                                        }
                                                        let _e5423 = phi_18417_;
                                                        phi_18065_ = _e5423;
                                                        phi_18068_ = _e5020;
                                                        phi_18070_ = _e5022;
                                                        phi_18074_ = _e5419;
                                                        phi_18490_ = _e5417;
                                                        phi_18491_ = _e5420;
                                                        phi_18492_ = false;
                                                    } else {
                                                        let _e5026 = (_e5020 == 0u);
                                                        if _e5026 {
                                                            phi_18486_ = u32();
                                                            phi_18487_ = u32();
                                                            phi_18488_ = u32();
                                                        } else {
                                                            let _e5027 = (_e5020 - 1u);
                                                            let _e5028 = (_e5027 < 100u);
                                                            if _e5028 {
                                                            } else {
                                                                phi_34167_ = true;
                                                                phi_20306_ = bool();
                                                                phi_20305_ = type_63();
                                                                break;
                                                            }
                                                            let _e5030 = local_67[_e5027];
                                                            let _e5032 = global_3.member[_e5030];
                                                            let _e5043 = ((_e5032.member.x - _e1528) * _e5016.x);
                                                            let _e5045 = ((_e5032.member.y - _e1529) * _e5016.y);
                                                            let _e5047 = ((_e5032.member.z - _e1530) * _e5016.z);
                                                            let _e5054 = ((_e5032.member_2.x - _e1528) * _e5016.x);
                                                            let _e5055 = ((_e5032.member_2.y - _e1529) * _e5016.y);
                                                            let _e5056 = ((_e5032.member_2.z - _e1530) * _e5016.z);
                                                            if (_e5043 == _e5043) {
                                                                let _e5058 = (_e5054 == _e5054);
                                                                if _e5058 {
                                                                    let _e5059 = (_e5043 != _e5043);
                                                                    if _e5059 {
                                                                        phi_29107_ = true;
                                                                    } else {
                                                                        phi_29107_ = (_e5054 <= _e5043);
                                                                    }
                                                                    let _e5062 = phi_29107_;
                                                                    let _e5063 = select(_e5043, _e5054, _e5062);
                                                                    if _e5059 {
                                                                        phi_29137_ = true;
                                                                    } else {
                                                                        phi_29137_ = (_e5054 >= _e5043);
                                                                    }
                                                                    let _e5068 = phi_29137_;
                                                                    let _e5069 = select(_e5043, _e5054, _e5068);
                                                                    phi_29084_ = type_52(select(-300000000000000000000000000000000000000f, _e5063, (_e5063 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e5069, (_e5069 <= 300000000000000000000000000000000000000f)));
                                                                } else {
                                                                    phi_29084_ = type_52();
                                                                }
                                                                let _e5074 = phi_29084_;
                                                                phi_29087_ = _e5074;
                                                                phi_29088_ = select(true, false, _e5058);
                                                            } else {
                                                                phi_29087_ = type_52();
                                                                phi_29088_ = true;
                                                            }
                                                            let _e5077 = phi_29087_;
                                                            let _e5079 = phi_29088_;
                                                            if _e5079 {
                                                                phi_29093_ = type_52(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                            } else {
                                                                phi_29093_ = _e5077;
                                                            }
                                                            let _e5081 = phi_29093_;
                                                            if (_e5045 == _e5045) {
                                                                let _e5085 = (_e5055 == _e5055);
                                                                if _e5085 {
                                                                    let _e5086 = (_e5045 != _e5045);
                                                                    if _e5086 {
                                                                        phi_29194_ = true;
                                                                    } else {
                                                                        phi_29194_ = (_e5055 <= _e5045);
                                                                    }
                                                                    let _e5089 = phi_29194_;
                                                                    let _e5090 = select(_e5045, _e5055, _e5089);
                                                                    if (_e5081.member != _e5081.member) {
                                                                        phi_29209_ = true;
                                                                    } else {
                                                                        phi_29209_ = (_e5090 >= _e5081.member);
                                                                    }
                                                                    let _e5094 = phi_29209_;
                                                                    if _e5086 {
                                                                        phi_29224_ = true;
                                                                    } else {
                                                                        phi_29224_ = (_e5055 >= _e5045);
                                                                    }
                                                                    let _e5098 = phi_29224_;
                                                                    let _e5099 = select(_e5045, _e5055, _e5098);
                                                                    if (_e5081.member_1 != _e5081.member_1) {
                                                                        phi_29239_ = true;
                                                                    } else {
                                                                        phi_29239_ = (_e5099 <= _e5081.member_1);
                                                                    }
                                                                    let _e5103 = phi_29239_;
                                                                    phi_29171_ = type_52(select(_e5081.member, _e5090, _e5094), select(_e5081.member_1, _e5099, _e5103));
                                                                } else {
                                                                    phi_29171_ = type_52();
                                                                }
                                                                let _e5107 = phi_29171_;
                                                                phi_29174_ = _e5107;
                                                                phi_29175_ = select(true, false, _e5085);
                                                            } else {
                                                                phi_29174_ = type_52();
                                                                phi_29175_ = true;
                                                            }
                                                            let _e5110 = phi_29174_;
                                                            let _e5112 = phi_29175_;
                                                            if _e5112 {
                                                                phi_29180_ = _e5081;
                                                            } else {
                                                                phi_29180_ = _e5110;
                                                            }
                                                            let _e5114 = phi_29180_;
                                                            if (_e5047 == _e5047) {
                                                                let _e5118 = (_e5056 == _e5056);
                                                                if _e5118 {
                                                                    let _e5119 = (_e5047 != _e5047);
                                                                    if _e5119 {
                                                                        phi_29281_ = true;
                                                                    } else {
                                                                        phi_29281_ = (_e5056 <= _e5047);
                                                                    }
                                                                    let _e5122 = phi_29281_;
                                                                    let _e5123 = select(_e5047, _e5056, _e5122);
                                                                    if (_e5114.member != _e5114.member) {
                                                                        phi_29296_ = true;
                                                                    } else {
                                                                        phi_29296_ = (_e5123 >= _e5114.member);
                                                                    }
                                                                    let _e5127 = phi_29296_;
                                                                    if _e5119 {
                                                                        phi_29311_ = true;
                                                                    } else {
                                                                        phi_29311_ = (_e5056 >= _e5047);
                                                                    }
                                                                    let _e5131 = phi_29311_;
                                                                    let _e5132 = select(_e5047, _e5056, _e5131);
                                                                    if (_e5114.member_1 != _e5114.member_1) {
                                                                        phi_29326_ = true;
                                                                    } else {
                                                                        phi_29326_ = (_e5132 <= _e5114.member_1);
                                                                    }
                                                                    let _e5136 = phi_29326_;
                                                                    phi_29258_ = type_52(select(_e5114.member, _e5123, _e5127), select(_e5114.member_1, _e5132, _e5136));
                                                                } else {
                                                                    phi_29258_ = type_52();
                                                                }
                                                                let _e5140 = phi_29258_;
                                                                phi_29261_ = _e5140;
                                                                phi_29262_ = select(true, false, _e5118);
                                                            } else {
                                                                phi_29261_ = type_52();
                                                                phi_29262_ = true;
                                                            }
                                                            let _e5143 = phi_29261_;
                                                            let _e5145 = phi_29262_;
                                                            if _e5145 {
                                                                phi_29267_ = _e5114;
                                                            } else {
                                                                phi_29267_ = _e5143;
                                                            }
                                                            let _e5147 = phi_29267_;
                                                            if (_e5147.member_1 < 0f) {
                                                                phi_29063_ = bool();
                                                                phi_29064_ = true;
                                                            } else {
                                                                let _e5151 = (_e5147.member > _e5147.member_1);
                                                                if _e5151 {
                                                                    phi_29060_ = bool();
                                                                } else {
                                                                    phi_29060_ = (_e5147.member > _e5024.member);
                                                                }
                                                                let _e5154 = phi_29060_;
                                                                phi_29063_ = _e5154;
                                                                phi_29064_ = _e5151;
                                                            }
                                                            let _e5156 = phi_29063_;
                                                            let _e5158 = phi_29064_;
                                                            if !(select(_e5156, true, _e5158)) {
                                                                if (_e5032.member_3 > 0u) {
                                                                    phi_18480_ = _e5032.member_1;
                                                                    phi_18481_ = _e5027;
                                                                    phi_18482_ = (_e5032.member_1 + _e5032.member_3);
                                                                } else {
                                                                    let _e5163 = (_e5020 + 1u);
                                                                    let _e5164 = (_e5163 <= 100u);
                                                                    if _e5164 {
                                                                        if _e5028 {
                                                                        } else {
                                                                            phi_34167_ = true;
                                                                            phi_20306_ = bool();
                                                                            phi_20305_ = type_63();
                                                                            break;
                                                                        }
                                                                        local_67[_e5027] = _e5032.member_1;
                                                                        if (_e5020 < 100u) {
                                                                        } else {
                                                                            phi_34167_ = true;
                                                                            phi_20306_ = bool();
                                                                            phi_20305_ = type_63();
                                                                            break;
                                                                        }
                                                                        local_67[_e5020] = (_e5030 + 1u);
                                                                    }
                                                                    phi_18480_ = _e5018;
                                                                    phi_18481_ = select(_e5027, _e5163, _e5164);
                                                                    phi_18482_ = _e5022;
                                                                }
                                                                let _e5173 = phi_18480_;
                                                                let _e5175 = phi_18481_;
                                                                let _e5177 = phi_18482_;
                                                                phi_18483_ = _e5173;
                                                                phi_18484_ = _e5175;
                                                                phi_18485_ = _e5177;
                                                            } else {
                                                                phi_18483_ = _e5018;
                                                                phi_18484_ = _e5027;
                                                                phi_18485_ = _e5022;
                                                            }
                                                            let _e5179 = phi_18483_;
                                                            let _e5181 = phi_18484_;
                                                            let _e5183 = phi_18485_;
                                                            phi_18486_ = _e5179;
                                                            phi_18487_ = _e5181;
                                                            phi_18488_ = _e5183;
                                                        }
                                                        let _e5185 = phi_18486_;
                                                        let _e5187 = phi_18487_;
                                                        let _e5189 = phi_18488_;
                                                        phi_18065_ = _e5185;
                                                        phi_18068_ = _e5187;
                                                        phi_18070_ = _e5189;
                                                        phi_18074_ = _e5024;
                                                        phi_18490_ = type_63();
                                                        phi_18491_ = select(true, false, _e5026);
                                                        phi_18492_ = _e5026;
                                                    }
                                                    let _e5425 = phi_18065_;
                                                    let _e5427 = phi_18068_;
                                                    let _e5429 = phi_18070_;
                                                    let _e5431 = phi_18074_;
                                                    let _e5433 = phi_18490_;
                                                    let _e5435 = phi_18491_;
                                                    let _e5437 = phi_18492_;
                                                    phi_18064_ = _e5425;
                                                    phi_18067_ = _e5427;
                                                    phi_18069_ = _e5429;
                                                    phi_18073_ = _e5431;
                                                    phi_34167_ = _e2408;
                                                    phi_20306_ = _e5437;
                                                    phi_20305_ = _e5433;
                                                    if !(_e5435) {
                                                        break;
                                                    }
                                                }
                                                let _e5440 = phi_34167_;
                                                let _e5442 = phi_20306_;
                                                let _e5444 = phi_20305_;
                                                phi_34566_ = _e4472;
                                                phi_34552_ = _e5440;
                                                if _e5440 {
                                                    break;
                                                }
                                                if _e5442 {
                                                    let _e7932 = local_106;
                                                    phi_18664_ = _e7932;
                                                } else {
                                                    phi_18664_ = type_63();
                                                }
                                                let _e5446 = phi_18664_;
                                                if select(true, false, _e5442) {
                                                    phi_18669_ = _e5444;
                                                } else {
                                                    phi_18669_ = _e5446;
                                                }
                                                let _e5449 = phi_18669_;
                                                if (_e5449.member_1 != 4294967295u) {
                                                    phi_18723_ = _e2412;
                                                    phi_18724_ = _e2414;
                                                } else {
                                                    if (_e2416 == (_e2406 + 2u)) {
                                                        phi_18721_ = _e2412;
                                                        phi_18722_ = type_39(vec4<f32>(_e1528, _e1529, _e1530, -1f), vec4<f32>(_e4476.x, _e4476.y, _e4476.z, 0f), vec4<f32>(((_e1398.x * _e4988.member.x) * _e5004), ((_e1398.y * _e4988.member.y) * _e5004), ((_e1398.z * _e4988.member.z) * _e5004), 0f));
                                                    } else {
                                                        phi_18721_ = vec3<f32>((_e2412.x + (((_e1398.x * _e4988.member.x) * _e4478.x) * _e5004)), (_e2412.y + (((_e1398.y * _e4988.member.y) * _e4478.y) * _e5004)), (_e2412.z + (((_e1398.z * _e4988.member.z) * _e4478.z) * _e5004)));
                                                        phi_18722_ = _e2414;
                                                    }
                                                    let _e5487 = phi_18721_;
                                                    let _e5489 = phi_18722_;
                                                    phi_18723_ = _e5487;
                                                    phi_18724_ = _e5489;
                                                }
                                                let _e5491 = phi_18723_;
                                                let _e5493 = phi_18724_;
                                                phi_34555_ = _e5440;
                                                phi_18725_ = _e5491;
                                                phi_18726_ = _e5493;
                                                phi_18727_ = (_e2418 + 1u);
                                            } else {
                                                phi_34555_ = _e2408;
                                                phi_18725_ = _e2412;
                                                phi_18726_ = _e2414;
                                                phi_18727_ = _e2418;
                                            }
                                            let _e5495 = phi_34555_;
                                            let _e5497 = phi_18725_;
                                            let _e5499 = phi_18726_;
                                            let _e5501 = phi_18727_;
                                            phi_34554_ = _e5495;
                                            phi_18728_ = _e5497;
                                            phi_18729_ = _e5499;
                                            phi_18730_ = _e5501;
                                        } else {
                                            phi_34554_ = _e2408;
                                            phi_18728_ = _e2412;
                                            phi_18729_ = _e2414;
                                            phi_18730_ = _e2418;
                                        }
                                        let _e5503 = phi_34554_;
                                        let _e5505 = phi_18728_;
                                        let _e5507 = phi_18729_;
                                        let _e5509 = phi_18730_;
                                        phi_34797_ = _e4472;
                                        phi_34553_ = _e5503;
                                        phi_14575_ = _e5505;
                                        phi_14578_ = _e5507;
                                        phi_14580_ = (_e2416 + 1u);
                                        phi_14582_ = _e5509;
                                    } else {
                                        phi_34797_ = _e2410;
                                        phi_34553_ = _e2408;
                                        phi_14575_ = vec3<f32>();
                                        phi_14578_ = type_39();
                                        phi_14580_ = u32();
                                        phi_14582_ = u32();
                                    }
                                    let _e5512 = phi_34797_;
                                    let _e5514 = phi_34553_;
                                    let _e5516 = phi_14575_;
                                    let _e5518 = phi_14578_;
                                    let _e5520 = phi_14580_;
                                    let _e5522 = phi_14582_;
                                    phi_34247_ = _e5514;
                                    phi_32314_ = _e5512;
                                    phi_14574_ = _e5516;
                                    phi_14577_ = _e5518;
                                    phi_14579_ = _e5520;
                                    phi_14581_ = _e5522;
                                    phi_34566_ = _e5512;
                                    phi_34552_ = _e5514;
                                    if !(_e2420) {
                                        break;
                                    }
                                }
                                let _e5525 = phi_34566_;
                                let _e5527 = phi_34552_;
                                if _e5527 {
                                    break;
                                }
                                if (_e339 >= _e355) {
                                    let _e6531 = local_100;
                                    let _e6534 = local_101;
                                    let _e6537 = local_102;
                                    let _e6540 = local_103;
                                    let _e6546 = local_104;
                                    phi_19281_ = type_68(type_35(_e312.member, _e312.member_1, vec4<f32>(_e1398.x, _e1398.y, _e1398.z, f32(_e339)), vec4<f32>(_e6534.x, _e6537.y, _e6540.z, f32(_e6531)), vec4<u32>(_e5525, (_e2403 & 4294967294u), 0u, 0u)), _e1400, _e6546);
                                    phi_19282_ = type_68();
                                    phi_19283_ = true;
                                } else {
                                    let _e5530 = ((_e5525 * 747796405u) + 2891336453u);
                                    let _e5538 = (((_e5530 >> bitcast<u32>((((_e5530 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5530) * 277803737u);
                                    let _e5541 = ((_e5538 >> bitcast<u32>(22i)) ^ _e5538);
                                    let _e5544 = f32((_e5541 >> bitcast<u32>(8i)));
                                    let _e5547 = ((_e5541 * 747796405u) + 2891336453u);
                                    let _e5555 = (((_e5547 >> bitcast<u32>((((_e5547 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5547) * 277803737u);
                                    let _e5558 = ((_e5555 >> bitcast<u32>(22i)) ^ _e5555);
                                    let _e5561 = f32((_e5558 >> bitcast<u32>(8i)));
                                    let _e5563 = ((_e5558 * 747796405u) + 2891336453u);
                                    let _e5571 = (((_e5563 >> bitcast<u32>((((_e5563 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5563) * 277803737u);
                                    let _e5574 = ((_e5571 >> bitcast<u32>(22i)) ^ _e5571);
                                    let _e5579 = (_e2395 == 0u);
                                    if _e5579 {
                                        phi_29436_ = vec3<f32>();
                                        phi_29437_ = true;
                                    } else {
                                        if (_e2395 == 2u) {
                                            let _e5585 = (((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z));
                                            if (_e5585 != _e5585) {
                                                phi_29477_ = true;
                                            } else {
                                                phi_29477_ = (0f >= _e5585);
                                            }
                                            let _e5589 = phi_29477_;
                                            let _e5590 = select(_e5585, 0f, _e5589);
                                            let _e5591 = (1f / _e2385);
                                            let _e5595 = ((_e5591 * _e5591) * (1f - (_e5590 * _e5590)));
                                            if (_e5595 >= 1f) {
                                                phi_29496_ = 1f;
                                            } else {
                                                let _e5598 = sqrt((1f - _e5595));
                                                if (_e5590 != _e5590) {
                                                    phi_29507_ = true;
                                                } else {
                                                    phi_29507_ = (_e5598 <= _e5590);
                                                }
                                                let _e5602 = phi_29507_;
                                                let _e5607 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                phi_29496_ = (_e5607 + ((1f - _e5607) * pow((1f - select(_e5590, _e5598, _e5602)), 5f)));
                                            }
                                            let _e5614 = phi_29496_;
                                            phi_29427_ = select(true, false, ((f32((_e5574 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5614));
                                        } else {
                                            phi_29427_ = true;
                                        }
                                        let _e5618 = phi_29427_;
                                        if _e5618 {
                                            let _e5623 = (((_e1475 * _e1551) + (_e1476 * _e1552)) + (_e1477 * _e1554));
                                            let _e5628 = (((_e1475 * _e1547) + (_e1476 * _e1557)) + (_e1477 * _e1558));
                                            let _e5633 = (((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z));
                                            if (_e5633 <= 0f) {
                                                phi_29661_ = vec3<f32>();
                                                phi_29662_ = true;
                                            } else {
                                                let _e5636 = (_e2393 * _e5623);
                                                let _e5637 = (_e2393 * _e5628);
                                                let _e5644 = (1f / sqrt((((_e5636 * _e5636) + (_e5637 * _e5637)) + (_e5633 * _e5633))));
                                                let _e5645 = (_e5636 * _e5644);
                                                let _e5646 = (_e5637 * _e5644);
                                                let _e5647 = (_e5633 * _e5644);
                                                let _e5650 = ((_e5645 * _e5645) + (_e5646 * _e5646));
                                                if (_e5650 > 0f) {
                                                    let _e5653 = sqrt(_e5650);
                                                    phi_29563_ = vec3<f32>((-(_e5646) / _e5653), (_e5645 / _e5653), 0f);
                                                } else {
                                                    phi_29563_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e5658 = phi_29563_;
                                                let _e5671 = sqrt((_e5544 * 0.000000059604645f));
                                                let _e5672 = (_e5561 * 0.00000037450704f);
                                                let _e5674 = (_e5671 * cos(_e5672));
                                                let _e5676 = (0.5f * (1f + _e5647));
                                                let _e5679 = (1f - (_e5674 * _e5674));
                                                if (_e5679 != _e5679) {
                                                    phi_29719_ = true;
                                                } else {
                                                    phi_29719_ = (0f >= _e5679);
                                                }
                                                let _e5683 = phi_29719_;
                                                let _e5690 = (((1f - _e5676) * sqrt(select(_e5679, 0f, _e5683))) + ((_e5676 * _e5671) * sin(_e5672)));
                                                let _e5701 = (_e5679 - (_e5690 * _e5690));
                                                if (_e5701 != _e5701) {
                                                    phi_29734_ = true;
                                                } else {
                                                    phi_29734_ = (0f >= _e5701);
                                                }
                                                let _e5705 = phi_29734_;
                                                let _e5707 = sqrt(select(_e5701, 0f, _e5705));
                                                let _e5713 = (((_e5658.z * _e5674) + (((_e5645 * _e5658.y) - (_e5658.x * _e5646)) * _e5690)) + (_e5647 * _e5707));
                                                let _e5714 = (_e2393 * (((_e5658.x * _e5674) + (((_e5646 * _e5658.z) - (_e5658.y * _e5647)) * _e5690)) + (_e5645 * _e5707)));
                                                let _e5715 = (_e2393 * (((_e5658.y * _e5674) + (((_e5647 * _e5658.x) - (_e5658.z * _e5645)) * _e5690)) + (_e5646 * _e5707)));
                                                if (_e5713 != _e5713) {
                                                    phi_29749_ = true;
                                                } else {
                                                    phi_29749_ = (0f >= _e5713);
                                                }
                                                let _e5719 = phi_29749_;
                                                let _e5720 = select(_e5713, 0f, _e5719);
                                                let _e5727 = (1f / sqrt((((_e5714 * _e5714) + (_e5715 * _e5715)) + (_e5720 * _e5720))));
                                                let _e5728 = (_e5714 * _e5727);
                                                let _e5729 = (_e5715 * _e5727);
                                                let _e5730 = (_e5720 * _e5727);
                                                let _e5736 = (2f * (((_e5623 * _e5728) + (_e5628 * _e5729)) + (_e5633 * _e5730)));
                                                let _e5739 = (_e5730 * _e5736);
                                                let _e5742 = (vec3<f32>((_e5728 * _e5736), (_e5729 * _e5736), _e5739) - vec3<f32>(_e5623, _e5628, _e5633));
                                                let _e5743 = ((_e5739 - _e5633) <= 0f);
                                                if _e5743 {
                                                    phi_29658_ = vec3<f32>();
                                                } else {
                                                    phi_29658_ = vec3<f32>((((_e1551 * _e5742.x) + (_e1547 * _e5742.y)) + (_e1503.x * _e5742.z)), (((_e1552 * _e5742.x) + (_e1557 * _e5742.y)) + (_e1503.y * _e5742.z)), (((_e1554 * _e5742.x) + (_e1558 * _e5742.y)) + (_e1503.z * _e5742.z)));
                                                }
                                                let _e5764 = phi_29658_;
                                                phi_29661_ = _e5764;
                                                phi_29662_ = _e5743;
                                            }
                                            let _e5766 = phi_29661_;
                                            let _e5768 = phi_29662_;
                                            phi_29432_ = select(_e5766, vec3<f32>(0f, 0f, 0f), vec3(_e5768));
                                        } else {
                                            phi_29432_ = vec3<f32>();
                                        }
                                        let _e5772 = phi_29432_;
                                        phi_29436_ = _e5772;
                                        phi_29437_ = select(true, false, _e5618);
                                    }
                                    let _e5775 = phi_29436_;
                                    let _e5777 = phi_29437_;
                                    if _e5777 {
                                        let _e5778 = (_e5544 * 0.00000037450704f);
                                        let _e5780 = ((_e5561 * 0.00000011920929f) - 1f);
                                        let _e5782 = (1f - (_e5780 * _e5780));
                                        if (_e5782 != _e5782) {
                                            phi_29849_ = true;
                                        } else {
                                            phi_29849_ = (0f >= _e5782);
                                        }
                                        let _e5786 = phi_29849_;
                                        let _e5788 = sqrt(select(_e5782, 0f, _e5786));
                                        let _e5793 = (_e1503.x + (_e5788 * cos(_e5778)));
                                        let _e5794 = (_e1503.y + (_e5788 * sin(_e5778)));
                                        let _e5795 = (_e1503.z + _e5780);
                                        let _e5801 = sqrt((((_e5793 * _e5793) + (_e5794 * _e5794)) + (_e5795 * _e5795)));
                                        if (_e5801 < 0.000001f) {
                                            phi_29823_ = _e1503;
                                        } else {
                                            let _e5803 = (1f / _e5801);
                                            phi_29823_ = vec3<f32>((_e5793 * _e5803), (_e5794 * _e5803), (_e5795 * _e5803));
                                        }
                                        let _e5809 = phi_29823_;
                                        phi_29444_ = _e5809;
                                    } else {
                                        phi_29444_ = _e5775;
                                    }
                                    let _e5811 = phi_29444_;
                                    let _e5819 = (((_e5811.x * _e1503.x) + (_e5811.y * _e1503.y)) + (_e5811.z * _e1503.z));
                                    if (_e5819 != _e5819) {
                                        phi_29864_ = true;
                                    } else {
                                        phi_29864_ = (0f >= _e5819);
                                    }
                                    let _e5823 = phi_29864_;
                                    let _e5825 = (select(_e5819, 0f, _e5823) * 0.31830987f);
                                    if _e5579 {
                                        if (_e5819 <= 0f) {
                                            phi_30661_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_30661_ = (_e2389 * (_e5819 * 0.31830987f));
                                        }
                                        let _e6300 = phi_30661_;
                                        phi_19122_ = type_55(_e6300, _e5825);
                                        phi_19123_ = type_55();
                                        phi_19124_ = true;
                                    } else {
                                        let _e5830 = (((_e1475 * _e1551) + (_e1476 * _e1552)) + (_e1477 * _e1554));
                                        let _e5835 = (((_e1475 * _e1547) + (_e1476 * _e1557)) + (_e1477 * _e1558));
                                        let _e5840 = (((_e1475 * _e1503.x) + (_e1476 * _e1503.y)) + (_e1477 * _e1503.z));
                                        let _e5845 = (((_e5811.x * _e1551) + (_e5811.y * _e1552)) + (_e5811.z * _e1554));
                                        let _e5850 = (((_e5811.x * _e1547) + (_e5811.y * _e1557)) + (_e5811.z * _e1558));
                                        if (_e5840 <= 0f) {
                                            phi_19086_ = type_55();
                                            phi_19087_ = true;
                                        } else {
                                            let _e5852 = (_e5819 <= 0f);
                                            if _e5852 {
                                                phi_19085_ = type_55();
                                            } else {
                                                let _e5853 = (_e5830 + _e5845);
                                                let _e5854 = (_e5835 + _e5850);
                                                let _e5855 = (_e5840 + _e5819);
                                                let _e5862 = (1f / sqrt((((_e5853 * _e5853) + (_e5854 * _e5854)) + (_e5855 * _e5855))));
                                                let _e5865 = (_e5855 * _e5862);
                                                if (_e5865 <= 0f) {
                                                    phi_29974_ = 0f;
                                                } else {
                                                    let _e5867 = (_e2393 * _e2393);
                                                    let _e5871 = ((((_e5867 - 1f) * _e5865) * _e5865) + 1f);
                                                    phi_29974_ = (_e5867 / ((3.1415927f * _e5871) * _e5871));
                                                }
                                                let _e5876 = phi_29974_;
                                                let _e5877 = (_e5840 * _e5840);
                                                if (_e5877 >= 1f) {
                                                    phi_30002_ = 0f;
                                                } else {
                                                    phi_30002_ = ((sqrt((1f + (((_e2393 * _e2393) * ((_e5830 * _e5830) + (_e5835 * _e5835))) / _e5877))) - 1f) * 0.5f);
                                                }
                                                let _e5890 = phi_30002_;
                                                let _e5891 = (1f + _e5890);
                                                let _e5892 = (_e5819 * _e5819);
                                                if (_e5892 >= 1f) {
                                                    phi_30030_ = 0f;
                                                } else {
                                                    phi_30030_ = ((sqrt((1f + (((_e2393 * _e2393) * ((_e5845 * _e5845) + (_e5850 * _e5850))) / _e5892))) - 1f) * 0.5f);
                                                }
                                                let _e5905 = phi_30030_;
                                                let _e5912 = (((_e5830 * (_e5853 * _e5862)) + (_e5835 * (_e5854 * _e5862))) + (_e5840 * _e5865));
                                                if (_e2391 == 0u) {
                                                    let _e6207 = (1f - _e5912);
                                                    if (_e6207 != _e6207) {
                                                        phi_30533_ = true;
                                                    } else {
                                                        phi_30533_ = (0f >= _e6207);
                                                    }
                                                    let _e6211 = phi_30533_;
                                                    let _e6213 = pow(select(_e6207, 0f, _e6211), 5f);
                                                    let _e6214 = (1f - _e6213);
                                                    phi_19073_ = vec3<f32>(((_e2389.x * _e6214) + _e6213), ((_e2389.y * _e6214) + _e6213), ((_e2389.z * _e6214) + _e6213));
                                                } else {
                                                    if (_e2391 == 1u) {
                                                        let _e6180 = (1f / _e2385);
                                                        let _e6184 = ((_e6180 * _e6180) * (1f - (_e5912 * _e5912)));
                                                        if (_e6184 >= 1f) {
                                                            phi_30494_ = 1f;
                                                        } else {
                                                            let _e6187 = sqrt((1f - _e6184));
                                                            if (_e5912 != _e5912) {
                                                                phi_30505_ = true;
                                                            } else {
                                                                phi_30505_ = (_e6187 <= _e5912);
                                                            }
                                                            let _e6191 = phi_30505_;
                                                            let _e6196 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                            phi_30494_ = (_e6196 + ((1f - _e6196) * pow((1f - select(_e5912, _e6187, _e6191)), 5f)));
                                                        }
                                                        let _e6203 = phi_30494_;
                                                        phi_19072_ = vec3(_e6203);
                                                    } else {
                                                        let _e5916 = select(_e5912, 0f, (_e5912 < 0f));
                                                        let _e5918 = select(_e5916, 1f, (_e5916 > 1f));
                                                        let _e5920 = (1f - (_e5918 * _e5918));
                                                        let _e5933 = ((_e2389.x * _e2389.x) - (_e2387.x * _e2387.x));
                                                        let _e5934 = ((_e2389.y * _e2389.y) - (_e2387.y * _e2387.y));
                                                        let _e5935 = ((_e2389.z * _e2389.z) - (_e2387.z * _e2387.z));
                                                        let _e5937 = (_e2389.x * (_e2387.x + _e2387.x));
                                                        let _e5939 = (_e2389.y * (_e2387.y + _e2387.y));
                                                        let _e5941 = (_e2389.z * (_e2387.z + _e2387.z));
                                                        let _e5948 = ((_e5933 * _e5933) + (_e5937 * _e5937));
                                                        let _e5949 = ((_e5934 * _e5934) + (_e5939 * _e5939));
                                                        let _e5950 = ((_e5935 * _e5935) + (_e5941 * _e5941));
                                                        let _e5955 = ((_e5920 * _e5933) / _e5948);
                                                        let _e5956 = ((_e5920 * _e5934) / _e5949);
                                                        let _e5957 = ((_e5920 * _e5935) / _e5950);
                                                        let _e5958 = (1f - _e5955);
                                                        let _e5959 = (1f - _e5956);
                                                        let _e5960 = (1f - _e5957);
                                                        let _e5962 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5955, _e5956, _e5957));
                                                        let _e5963 = (_e5920 * _e5937);
                                                        let _e5964 = (_e5920 * _e5939);
                                                        let _e5965 = (_e5920 * _e5941);
                                                        let _e5970 = (vec3<f32>(_e5963, _e5964, _e5965) / vec3<f32>(_e5948, _e5949, _e5950));
                                                        let _e5983 = ((_e5962.x * _e5962.x) + (_e5970.x * _e5970.x));
                                                        let _e5984 = ((_e5962.y * _e5962.y) + (_e5970.y * _e5970.y));
                                                        let _e5985 = ((_e5962.z * _e5962.z) + (_e5970.z * _e5970.z));
                                                        if (_e5983 != _e5983) {
                                                            phi_30164_ = true;
                                                        } else {
                                                            phi_30164_ = (0f >= _e5983);
                                                        }
                                                        let _e5989 = phi_30164_;
                                                        let _e5991 = sqrt(select(_e5983, 0f, _e5989));
                                                        if (_e5984 != _e5984) {
                                                            phi_30179_ = true;
                                                        } else {
                                                            phi_30179_ = (0f >= _e5984);
                                                        }
                                                        let _e5995 = phi_30179_;
                                                        let _e5997 = sqrt(select(_e5984, 0f, _e5995));
                                                        if (_e5985 != _e5985) {
                                                            phi_30194_ = true;
                                                        } else {
                                                            phi_30194_ = (0f >= _e5985);
                                                        }
                                                        let _e6001 = phi_30194_;
                                                        let _e6003 = sqrt(select(_e5985, 0f, _e6001));
                                                        let _e6007 = ((_e5991 + _e5958) * 0.5f);
                                                        let _e6008 = ((_e5997 + _e5959) * 0.5f);
                                                        let _e6009 = ((_e6003 + _e5960) * 0.5f);
                                                        if (_e6007 != _e6007) {
                                                            phi_30221_ = true;
                                                        } else {
                                                            phi_30221_ = (0f >= _e6007);
                                                        }
                                                        let _e6013 = phi_30221_;
                                                        let _e6015 = sqrt(select(_e6007, 0f, _e6013));
                                                        if (_e6008 != _e6008) {
                                                            phi_30236_ = true;
                                                        } else {
                                                            phi_30236_ = (0f >= _e6008);
                                                        }
                                                        let _e6019 = phi_30236_;
                                                        let _e6021 = sqrt(select(_e6008, 0f, _e6019));
                                                        if (_e6009 != _e6009) {
                                                            phi_30251_ = true;
                                                        } else {
                                                            phi_30251_ = (0f >= _e6009);
                                                        }
                                                        let _e6025 = phi_30251_;
                                                        let _e6027 = sqrt(select(_e6009, 0f, _e6025));
                                                        let _e6028 = vec3<f32>(_e6015, _e6021, _e6027);
                                                        let _e6032 = ((_e5991 - _e5958) * 0.5f);
                                                        let _e6033 = ((_e5997 - _e5959) * 0.5f);
                                                        let _e6034 = ((_e6003 - _e5960) * 0.5f);
                                                        if (_e6032 != _e6032) {
                                                            phi_30278_ = true;
                                                        } else {
                                                            phi_30278_ = (0f >= _e6032);
                                                        }
                                                        let _e6038 = phi_30278_;
                                                        let _e6040 = sqrt(select(_e6032, 0f, _e6038));
                                                        if (_e6033 != _e6033) {
                                                            phi_30293_ = true;
                                                        } else {
                                                            phi_30293_ = (0f >= _e6033);
                                                        }
                                                        let _e6044 = phi_30293_;
                                                        let _e6046 = sqrt(select(_e6033, 0f, _e6044));
                                                        if (_e6034 != _e6034) {
                                                            phi_30308_ = true;
                                                        } else {
                                                            phi_30308_ = (0f >= _e6034);
                                                        }
                                                        let _e6050 = phi_30308_;
                                                        let _e6052 = sqrt(select(_e6034, 0f, _e6050));
                                                        if ((_e5963 / _e5948) < 0f) {
                                                            phi_18949_ = -(_e6040);
                                                        } else {
                                                            phi_18949_ = _e6040;
                                                        }
                                                        let _e6056 = phi_18949_;
                                                        if ((_e5964 / _e5949) < 0f) {
                                                            phi_18957_ = -(_e6046);
                                                        } else {
                                                            phi_18957_ = _e6046;
                                                        }
                                                        let _e6060 = phi_18957_;
                                                        if ((_e5965 / _e5950) < 0f) {
                                                            phi_18965_ = -(_e6052);
                                                        } else {
                                                            phi_18965_ = _e6052;
                                                        }
                                                        let _e6064 = phi_18965_;
                                                        let _e6068 = (_e2387.x * _e5918);
                                                        let _e6069 = (_e2387.y * _e5918);
                                                        let _e6070 = (_e2387.z * _e5918);
                                                        let _e6080 = vec3<f32>(((_e2389.x * _e6015) - (_e2387.x * _e6056)), ((_e2389.y * _e6021) - (_e2387.y * _e6060)), ((_e2389.z * _e6027) - (_e2387.z * _e6064)));
                                                        let _e6087 = ((_e2389.x * _e6056) + (_e2387.x * _e6015));
                                                        let _e6088 = ((_e2389.y * _e6060) + (_e2387.y * _e6021));
                                                        let _e6089 = ((_e2389.z * _e6064) + (_e2387.z * _e6027));
                                                        let _e6091 = vec3<f32>((_e2389.x * _e5918), (_e2389.y * _e5918), (_e2389.z * _e5918));
                                                        let _e6092 = (_e6091 - _e6028);
                                                        let _e6093 = (_e6068 - _e6056);
                                                        let _e6094 = (_e6069 - _e6060);
                                                        let _e6095 = (_e6070 - _e6064);
                                                        let _e6108 = (_e6091 + _e6028);
                                                        let _e6109 = (_e6068 + _e6056);
                                                        let _e6110 = (_e6069 + _e6060);
                                                        let _e6111 = (_e6070 + _e6064);
                                                        let _e6121 = ((_e6108.x * _e6108.x) + (_e6109 * _e6109));
                                                        let _e6122 = ((_e6108.y * _e6108.y) + (_e6110 * _e6110));
                                                        let _e6123 = ((_e6108.z * _e6108.z) + (_e6111 * _e6111));
                                                        let _e6133 = vec3(_e5918);
                                                        let _e6134 = (_e6133 - _e6080);
                                                        let _e6135 = -(vec3<f32>(_e6087, _e6088, _e6089));
                                                        let _e6151 = (_e6133 + _e6080);
                                                        let _e6161 = ((_e6151.x * _e6151.x) + (_e6087 * _e6087));
                                                        let _e6162 = ((_e6151.y * _e6151.y) + (_e6088 * _e6088));
                                                        let _e6163 = ((_e6151.z * _e6151.z) + (_e6089 * _e6089));
                                                        phi_19072_ = vec3<f32>((((((_e6092.x * _e6092.x) + (_e6093 * _e6093)) / select(0.000000000000000000000000000001f, _e6121, (_e6121 > 0.000000000000000000000000000001f))) + (((_e6134.x * _e6134.x) + (_e6135.x * _e6135.x)) / select(0.000000000000000000000000000001f, _e6161, (_e6161 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6092.y * _e6092.y) + (_e6094 * _e6094)) / select(0.000000000000000000000000000001f, _e6122, (_e6122 > 0.000000000000000000000000000001f))) + (((_e6134.y * _e6134.y) + (_e6135.y * _e6135.y)) / select(0.000000000000000000000000000001f, _e6162, (_e6162 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e6092.z * _e6092.z) + (_e6095 * _e6095)) / select(0.000000000000000000000000000001f, _e6123, (_e6123 > 0.000000000000000000000000000001f))) + (((_e6134.z * _e6134.z) + (_e6135.z * _e6135.z)) / select(0.000000000000000000000000000001f, _e6163, (_e6163 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6206 = phi_19072_;
                                                    phi_19073_ = _e6206;
                                                }
                                                let _e6226 = phi_19073_;
                                                let _e6228 = (4f * _e5840);
                                                phi_19085_ = type_55((_e6226 * (((1f / (_e5891 + _e5905)) * _e5876) / _e6228)), ((_e5876 / _e5891) / _e6228));
                                            }
                                            let _e6235 = phi_19085_;
                                            phi_19086_ = _e6235;
                                            phi_19087_ = _e5852;
                                        }
                                        let _e6237 = phi_19086_;
                                        let _e6239 = phi_19087_;
                                        if _e6239 {
                                            phi_19092_ = type_55(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19092_ = _e6237;
                                        }
                                        let _e6241 = phi_19092_;
                                        let _e6244 = (_e2395 == 1u);
                                        if _e6244 {
                                            phi_19120_ = _e6241;
                                            phi_19121_ = type_55();
                                        } else {
                                            if (_e5840 != _e5840) {
                                                phi_30570_ = true;
                                            } else {
                                                phi_30570_ = (0f >= _e5840);
                                            }
                                            let _e6248 = phi_30570_;
                                            let _e6249 = select(_e5840, 0f, _e6248);
                                            let _e6250 = (1f / _e2385);
                                            let _e6254 = ((_e6250 * _e6250) * (1f - (_e6249 * _e6249)));
                                            if (_e6254 >= 1f) {
                                                phi_30589_ = 1f;
                                            } else {
                                                let _e6257 = sqrt((1f - _e6254));
                                                if (_e6249 != _e6249) {
                                                    phi_30600_ = true;
                                                } else {
                                                    phi_30600_ = (_e6257 <= _e6249);
                                                }
                                                let _e6261 = phi_30600_;
                                                let _e6266 = pow(((1f - _e2385) / (1f + _e2385)), 2f);
                                                phi_30589_ = (_e6266 + ((1f - _e6266) * pow((1f - select(_e6249, _e6257, _e6261)), 5f)));
                                            }
                                            let _e6273 = phi_30589_;
                                            if (_e5819 <= 0f) {
                                                phi_30639_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_30639_ = (_e2383 * (_e5819 * 0.31830987f));
                                            }
                                            let _e6278 = phi_30639_;
                                            let _e6279 = (1f - _e6273);
                                            phi_19120_ = type_55();
                                            phi_19121_ = type_55((_e6241.member + vec3<f32>((_e6278.x * _e6279), (_e6278.y * _e6279), (_e6278.z * _e6279))), ((_e6273 * _e6241.member_1) + (_e6279 * _e5825)));
                                        }
                                        let _e6293 = phi_19120_;
                                        let _e6295 = phi_19121_;
                                        phi_19122_ = _e6293;
                                        phi_19123_ = _e6295;
                                        phi_19124_ = _e6244;
                                    }
                                    let _e6303 = phi_19122_;
                                    let _e6305 = phi_19123_;
                                    let _e6307 = phi_19124_;
                                    if _e6307 {
                                        phi_19128_ = _e6303;
                                    } else {
                                        phi_19128_ = _e6305;
                                    }
                                    let _e6309 = phi_19128_;
                                    if ((((_e5811.x * _e5811.x) + (_e5811.y * _e5811.y)) + (_e5811.z * _e5811.z)) == 0f) {
                                        phi_36059_ = _e5574;
                                        phi_19267_ = type_68();
                                        phi_19268_ = type_68();
                                        phi_19269_ = true;
                                        phi_19270_ = false;
                                    } else {
                                        let _e6318 = (_e6309.member_1 <= 0f);
                                        if _e6318 {
                                            phi_36060_ = _e5574;
                                            phi_19264_ = type_68();
                                            phi_19265_ = type_68();
                                            phi_19266_ = false;
                                        } else {
                                            let _e6320 = (_e6309.member.x / _e6309.member_1);
                                            let _e6322 = (_e6309.member.y / _e6309.member_1);
                                            let _e6324 = (_e6309.member.z / _e6309.member_1);
                                            let _e6333 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6320 * _e6320) + (_e6322 * _e6322)) + (_e6324 * _e6324))) <= _e6333) {
                                                phi_36053_ = _e5574;
                                                phi_19215_ = _e1398;
                                                phi_19216_ = false;
                                            } else {
                                                let _e6337 = global_1.member.member_7[1u];
                                                if (_e6337 == 1u) {
                                                    let _e6342 = global_1.member.member_4[3u];
                                                    if (f32(_e339) >= _e6342) {
                                                        let _e6345 = select(_e6322, _e6320, (_e6320 > _e6322));
                                                        let _e6347 = select(_e6324, _e6345, (_e6345 > _e6324));
                                                        if (_e6347 != _e6347) {
                                                            phi_30686_ = true;
                                                        } else {
                                                            phi_30686_ = (1f <= _e6347);
                                                        }
                                                        let _e6351 = phi_30686_;
                                                        let _e6352 = select(_e6347, 1f, _e6351);
                                                        let _e6354 = ((_e5574 * 747796405u) + 2891336453u);
                                                        let _e6362 = (((_e6354 >> bitcast<u32>((((_e6354 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6354) * 277803737u);
                                                        let _e6365 = ((_e6362 >> bitcast<u32>(22i)) ^ _e6362);
                                                        let _e6370 = ((f32((_e6365 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6352);
                                                        if _e6370 {
                                                            phi_19204_ = vec3<f32>();
                                                        } else {
                                                            phi_19204_ = vec3<f32>((_e1398.x / _e6352), (_e1398.y / _e6352), (_e1398.z / _e6352));
                                                        }
                                                        let _e6379 = phi_19204_;
                                                        phi_36056_ = _e6365;
                                                        phi_19206_ = _e6379;
                                                        phi_19207_ = select(true, false, _e6370);
                                                    } else {
                                                        phi_36056_ = _e5574;
                                                        phi_19206_ = _e1398;
                                                        phi_19207_ = true;
                                                    }
                                                    let _e6382 = phi_36056_;
                                                    let _e6384 = phi_19206_;
                                                    let _e6386 = phi_19207_;
                                                    phi_36055_ = _e6382;
                                                    phi_19208_ = _e6384;
                                                    phi_19209_ = _e6386;
                                                } else {
                                                    phi_36055_ = _e5574;
                                                    phi_19208_ = _e1398;
                                                    phi_19209_ = true;
                                                }
                                                let _e6388 = phi_36055_;
                                                let _e6390 = phi_19208_;
                                                let _e6392 = phi_19209_;
                                                phi_36053_ = _e6388;
                                                phi_19215_ = select(_e1398, _e6390, vec3(_e6392));
                                                phi_19216_ = select(false, true, _e6392);
                                            }
                                            let _e6397 = phi_36053_;
                                            let _e6399 = phi_19215_;
                                            let _e6401 = phi_19216_;
                                            if _e6401 {
                                                if ((((_e5811.x * _e1490.x) + (_e5811.y * _e1490.y)) + (_e5811.z * _e1490.z)) < 0f) {
                                                    phi_19245_ = -(_e1490);
                                                } else {
                                                    phi_19245_ = _e1490;
                                                }
                                                let _e6440 = phi_19245_;
                                                let _e6457 = local_90;
                                                let _e6460 = local_91;
                                                let _e6463 = local_92;
                                                let _e6466 = local_93;
                                                let _e6472 = local_94;
                                                phi_19261_ = type_68(type_35(vec4<f32>((_e1405 + (_e6440.x * _e1521)), (_e1406 + (_e6440.y * _e1521)), (_e1407 + (_e6440.z * _e1521)), _e312.member.w), vec4<f32>(_e5811.x, _e5811.y, _e5811.z, _e6309.member_1), vec4<f32>((_e6399.x * _e6320), (_e6399.y * _e6322), (_e6399.z * _e6324), f32((_e339 + 1u))), vec4<f32>(_e6460.x, _e6463.y, _e6466.z, f32(_e6457)), vec4<u32>(_e6397, _e2403, 0u, 0u)), _e1400, _e6472);
                                                phi_19262_ = type_68();
                                            } else {
                                                let _e6409 = local_85;
                                                let _e6412 = local_86;
                                                let _e6415 = local_87;
                                                let _e6418 = local_88;
                                                let _e6424 = local_89;
                                                phi_19261_ = type_68();
                                                phi_19262_ = type_68(type_35(_e312.member, _e312.member_1, vec4<f32>(_e6399.x, _e6399.y, _e6399.z, f32(_e339)), vec4<f32>(_e6412.x, _e6415.y, _e6418.z, f32(_e6409)), vec4<u32>(_e6397, (_e2403 & 4294967294u), 0u, 0u)), _e1400, _e6424);
                                            }
                                            let _e6475 = phi_19261_;
                                            let _e6477 = phi_19262_;
                                            phi_36060_ = _e6397;
                                            phi_19264_ = _e6475;
                                            phi_19265_ = _e6477;
                                            phi_19266_ = select(true, false, _e6401);
                                        }
                                        let _e6480 = phi_36060_;
                                        let _e6482 = phi_19264_;
                                        let _e6484 = phi_19265_;
                                        let _e6486 = phi_19266_;
                                        phi_36059_ = _e6480;
                                        phi_19267_ = _e6482;
                                        phi_19268_ = _e6484;
                                        phi_19269_ = _e6318;
                                        phi_19270_ = _e6486;
                                    }
                                    let _e6488 = phi_36059_;
                                    let _e6490 = phi_19267_;
                                    let _e6492 = phi_19268_;
                                    let _e6494 = phi_19269_;
                                    let _e6496 = phi_19270_;
                                    if _e6494 {
                                        let _e6504 = local_95;
                                        let _e6507 = local_96;
                                        let _e6510 = local_97;
                                        let _e6513 = local_98;
                                        let _e6519 = local_99;
                                        phi_19276_ = type_68(type_35(_e312.member, _e312.member_1, vec4<f32>(_e1398.x, _e1398.y, _e1398.z, f32(_e339)), vec4<f32>(_e6507.x, _e6510.y, _e6513.z, f32(_e6504)), vec4<u32>(_e6488, (_e2403 & 4294967294u), 0u, 0u)), _e1400, _e6519);
                                    } else {
                                        phi_19276_ = _e6492;
                                    }
                                    let _e6522 = phi_19276_;
                                    phi_19281_ = _e6522;
                                    phi_19282_ = _e6490;
                                    phi_19283_ = select(_e6496, true, _e6494);
                                }
                                let _e6549 = phi_19281_;
                                let _e6551 = phi_19282_;
                                let _e6553 = phi_19283_;
                                phi_19287_ = _e6549;
                                phi_19288_ = _e6551;
                                phi_19289_ = _e6553;
                            } else {
                                phi_19287_ = _e2399;
                                phi_19288_ = type_68();
                                phi_19289_ = true;
                            }
                            let _e6555 = phi_19287_;
                            let _e6557 = phi_19288_;
                            let _e6559 = phi_19289_;
                            phi_19293_ = _e6555;
                            phi_19294_ = _e6557;
                            phi_19295_ = _e6559;
                        }
                        let _e6771 = phi_19293_;
                        let _e6773 = phi_19294_;
                        let _e6775 = phi_19295_;
                        phi_19299_ = _e6771;
                        phi_19300_ = _e6773;
                        phi_19301_ = _e6775;
                    }
                    let _e7283 = phi_19299_;
                    let _e7285 = phi_19300_;
                    let _e7287 = phi_19301_;
                    if _e7287 {
                        phi_19305_ = _e7283;
                    } else {
                        phi_19305_ = _e7285;
                    }
                    let _e7289 = phi_19305_;
                    phi_19309_ = vec4<f32>(0f, 0f, 0f, 0f);
                    phi_19310_ = _e7289.member_2;
                    phi_19311_ = _e7289.member_1;
                    phi_19312_ = _e7289.member;
                }
                let _e7339 = phi_19309_;
                let _e7341 = phi_19310_;
                let _e7343 = phi_19311_;
                let _e7345 = phi_19312_;
                if _e309 {
                } else {
                    break;
                }
                global_7.member[_e304.member_1] = _e7345;
                let _e7346 = (2u * _e304.member_1);
                if (_e7346 < _e284) {
                } else {
                    break;
                }
                global_8.member[_e7346] = _e7343;
                let _e7350 = (_e7346 + 1u);
                if (_e7350 < _e284) {
                } else {
                    break;
                }
                global_8.member[_e7350] = _e7341;
                if (_e7339.w > 0f) {
                    if (_e304.member_1 < arrayLength((&global_9.member))) {
                    } else {
                        break;
                    }
                    let _e7361 = global_9.member[_e304.member_1][0u];
                    global_9.member[_e304.member_1][0u] = (_e7361 + _e7339.x);
                    let _e7367 = global_9.member[_e304.member_1][1u];
                    global_9.member[_e304.member_1][1u] = (_e7367 + _e7339.y);
                    let _e7373 = global_9.member[_e304.member_1][2u];
                    global_9.member[_e304.member_1][2u] = (_e7373 + _e7339.z);
                    let _e7378 = global_9.member[_e304.member_1][3u];
                    global_9.member[_e304.member_1][3u] = (_e7378 + _e7339.w);
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
