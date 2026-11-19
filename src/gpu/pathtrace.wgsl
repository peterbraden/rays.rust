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

struct type_63 {
    member: vec4<f32>,
    member_1: vec4<f32>,
}

struct type_65 {
    member: array<type_63>,
}

struct type_74 {
    member: type_34,
    member_1: type_41,
    member_2: type_41,
}

struct type_77 {
    member: vec3<f32>,
    member_1: vec3<f32>,
    member_2: f32,
}

struct type_79 {
    member: u32,
    member_1: vec4<f32>,
}

struct type_84 {
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
var<storage, read_write> global_8: type_84;
@group(0) @binding(8) 
var<storage, read_write> global_9: type_43;
@group(0) @binding(1) 
var<storage, read_write> global_10: type_36;
@group(0) @binding(2) 
var<storage, read_write> global_11: type_84;
@group(0) @binding(3) 
var<storage, read_write> global_12: type_36;
@group(0) @binding(1) 
var<storage> global_13: type_65;
@group(0) @binding(2) 
var<storage, read_write> global_14: type_36;
@group(0) @binding(3) 
var<storage, read_write> global_15: type_84;
@group(0) @binding(1) 
var<storage> global_16: type_36;
@group(0) @binding(2) 
var<storage, read_write> global_17: type_13;

fn function_() {
    var phi_514_: type_48;
    var phi_34003_: bool;
    var phi_534_: vec3<f32>;
    var phi_537_: u32;
    var phi_22168_: f32;
    var phi_22180_: type_56;
    var phi_22191_: type_57;
    var phi_22192_: type_57;
    var phi_22196_: type_57;
    var phi_22197_: type_57;
    var phi_22198_: bool;
    var phi_22202_: type_57;
    var phi_22089_: type_57;
    var phi_22240_: bool;
    var phi_22320_: f32;
    var phi_22332_: type_56;
    var phi_22343_: type_57;
    var phi_22344_: type_57;
    var phi_22348_: type_57;
    var phi_22349_: type_57;
    var phi_22350_: bool;
    var phi_22354_: type_57;
    var phi_22288_: f32;
    var phi_22392_: bool;
    var phi_22117_: f32;
    var phi_22119_: f32;
    var phi_22128_: type_57;
    var phi_22132_: type_57;
    var phi_22136_: type_57;
    var phi_22407_: bool;
    var phi_9537_: vec3<f32>;
    var phi_9540_: vec3<f32>;
    var phi_9542_: vec3<f32>;
    var phi_9544_: vec3<f32>;
    var phi_9546_: u32;
    var phi_9630_: u32;
    var phi_9645_: type_56;
    var phi_9646_: type_56;
    var phi_9647_: type_56;
    var phi_22607_: bool;
    var phi_22734_: bool;
    var phi_22749_: bool;
    var phi_9975_: vec3<f32>;
    var phi_10048_: vec3<f32>;
    var phi_10049_: vec3<f32>;
    var phi_10050_: bool;
    var phi_10055_: vec3<f32>;
    var phi_10056_: bool;
    var phi_22873_: bool;
    var phi_22888_: bool;
    var phi_10347_: vec3<f32>;
    var phi_9538_: vec3<f32>;
    var phi_9541_: vec3<f32>;
    var phi_9543_: vec3<f32>;
    var phi_9545_: vec3<f32>;
    var phi_9547_: u32;
    var phi_33986_: bool;
    var local: vec3<f32>;
    var local_1: vec3<f32>;
    var local_2: vec3<f32>;
    var local_3: vec3<f32>;
    var local_4: vec3<f32>;
    var local_5: vec3<f32>;
    var local_6: vec3<f32>;
    var local_7: vec3<f32>;
    var phi_34470_: bool;
    var phi_10404_: type_60;
    var phi_34469_: bool;
    var phi_10406_: type_60;
    var phi_10407_: bool;
    var phi_10412_: type_60;
    var phi_913_: vec3<f32>;
    var phi_915_: vec3<f32>;
    var phi_916_: bool;
    var phi_34078_: bool;
    var phi_926_: vec3<f32>;
    var phi_929_: i32;
    var phi_931_: u32;
    var phi_23157_: bool;
    var phi_1353_: u32;
    var phi_1383_: u32;
    var phi_1354_: u32;
    var phi_1384_: bool;
    var local_8: u32;
    var local_9: u32;
    var local_10: u32;
    var phi_1402_: u32;
    var phi_2707_: type_55;
    var phi_2715_: type_52;
    var phi_2717_: type_52;
    var phi_2718_: bool;
    var phi_2719_: type_52;
    var phi_2720_: bool;
    var phi_2721_: bool;
    var phi_23268_: f32;
    var phi_23280_: type_56;
    var phi_23291_: type_57;
    var phi_23292_: type_57;
    var phi_23296_: type_57;
    var phi_23297_: type_57;
    var phi_23298_: bool;
    var phi_23302_: type_57;
    var phi_23189_: type_57;
    var phi_23340_: bool;
    var phi_23420_: f32;
    var phi_23432_: type_56;
    var phi_23443_: type_57;
    var phi_23444_: type_57;
    var phi_23448_: type_57;
    var phi_23449_: type_57;
    var phi_23450_: bool;
    var phi_23454_: type_57;
    var phi_23388_: f32;
    var phi_23492_: bool;
    var phi_23217_: f32;
    var phi_23219_: f32;
    var phi_23228_: type_57;
    var phi_23232_: type_57;
    var phi_23236_: type_57;
    var phi_2739_: vec3<f32>;
    var phi_2742_: vec3<f32>;
    var phi_2744_: vec3<f32>;
    var phi_2746_: vec3<f32>;
    var phi_2748_: u32;
    var phi_2832_: u32;
    var phi_2847_: type_56;
    var phi_2848_: type_56;
    var phi_2849_: type_56;
    var phi_23691_: bool;
    var phi_23818_: bool;
    var phi_23833_: bool;
    var phi_3177_: vec3<f32>;
    var phi_23955_: f32;
    var phi_23967_: type_56;
    var phi_23978_: type_57;
    var phi_23979_: type_57;
    var phi_23983_: type_57;
    var phi_23984_: type_57;
    var phi_23985_: bool;
    var phi_23989_: type_57;
    var phi_23923_: f32;
    var phi_24065_: f32;
    var phi_24077_: type_56;
    var phi_24088_: type_57;
    var phi_24089_: type_57;
    var phi_24093_: type_57;
    var phi_24094_: type_57;
    var phi_24095_: bool;
    var phi_24099_: type_57;
    var phi_24033_: f32;
    var phi_23860_: vec3<f32>;
    var phi_23861_: u32;
    var phi_24231_: bool;
    var phi_23894_: vec3<f32>;
    var phi_23895_: u32;
    var local_11: vec3<f32>;
    var phi_23902_: type_58;
    var phi_23906_: type_58;
    var phi_3236_: vec3<f32>;
    var phi_3237_: vec3<f32>;
    var phi_2740_: vec3<f32>;
    var phi_2743_: vec3<f32>;
    var phi_2745_: vec3<f32>;
    var phi_2747_: vec3<f32>;
    var phi_2749_: u32;
    var phi_34046_: bool;
    var local_12: vec3<f32>;
    var local_13: vec3<f32>;
    var local_14: vec3<f32>;
    var phi_34140_: bool;
    var phi_3257_: type_52;
    var phi_34139_: bool;
    var phi_3258_: type_52;
    var phi_34138_: bool;
    var phi_3281_: vec3<f32>;
    var phi_3282_: i32;
    var phi_3283_: u32;
    var phi_34137_: bool;
    var phi_927_: vec3<f32>;
    var phi_930_: i32;
    var phi_932_: u32;
    var phi_3284_: bool;
    var phi_34136_: bool;
    var phi_3618_: vec3<f32>;
    var phi_3621_: u32;
    var phi_24359_: f32;
    var phi_24374_: f32;
    var phi_24389_: f32;
    var phi_3692_: i32;
    var phi_3722_: i32;
    var phi_3730_: vec3<f32>;
    var phi_3733_: i32;
    var phi_3756_: vec3<f32>;
    var phi_3759_: i32;
    var phi_3757_: vec3<f32>;
    var phi_3760_: i32;
    var phi_3731_: vec3<f32>;
    var phi_3734_: i32;
    var phi_3958_: vec3<f32>;
    var phi_3959_: vec3<f32>;
    var phi_3960_: vec3<f32>;
    var phi_3619_: vec3<f32>;
    var phi_3622_: u32;
    var phi_34141_: bool;
    var local_15: vec3<f32>;
    var phi_34189_: bool;
    var phi_3971_: f32;
    var phi_3995_: f32;
    var phi_3997_: u32;
    var phi_4000_: f32;
    var phi_4103_: f32;
    var phi_4104_: f32;
    var phi_4120_: f32;
    var phi_3998_: u32;
    var phi_4001_: f32;
    var phi_4123_: u32;
    var phi_4126_: f32;
    var phi_4234_: f32;
    var phi_4124_: u32;
    var phi_4127_: f32;
    var local_16: f32;
    var local_17: vec3<f32>;
    var local_18: vec3<f32>;
    var local_19: vec3<f32>;
    var phi_24505_: f32;
    var phi_24517_: type_56;
    var phi_24528_: type_57;
    var phi_24529_: type_57;
    var phi_24533_: type_57;
    var phi_24534_: type_57;
    var phi_24535_: bool;
    var phi_24539_: type_57;
    var phi_24473_: f32;
    var phi_4635_: bool;
    var phi_24665_: f32;
    var phi_24677_: type_56;
    var phi_24688_: type_57;
    var phi_24689_: type_57;
    var phi_24693_: type_57;
    var phi_24694_: type_57;
    var phi_24695_: bool;
    var phi_24699_: type_57;
    var phi_24586_: type_57;
    var phi_24737_: bool;
    var phi_24817_: f32;
    var phi_24829_: type_56;
    var phi_24840_: type_57;
    var phi_24841_: type_57;
    var phi_24845_: type_57;
    var phi_24846_: type_57;
    var phi_24847_: bool;
    var phi_24851_: type_57;
    var phi_24785_: f32;
    var phi_24889_: bool;
    var phi_24614_: f32;
    var phi_24616_: f32;
    var phi_24625_: type_57;
    var phi_24629_: type_57;
    var phi_24633_: type_57;
    var phi_4649_: vec3<f32>;
    var phi_4652_: u32;
    var phi_4736_: u32;
    var phi_4751_: type_56;
    var phi_4752_: type_56;
    var phi_4753_: type_56;
    var phi_25088_: bool;
    var phi_4650_: vec3<f32>;
    var phi_4653_: u32;
    var phi_34168_: bool;
    var local_20: vec3<f32>;
    var local_21: vec3<f32>;
    var local_22: vec3<f32>;
    var phi_34235_: bool;
    var phi_4778_: vec3<f32>;
    var phi_34234_: bool;
    var phi_4780_: vec3<f32>;
    var phi_4782_: bool;
    var phi_34230_: bool;
    var phi_4793_: vec3<f32>;
    var phi_4794_: bool;
    var phi_34229_: bool;
    var phi_4796_: vec3<f32>;
    var phi_4797_: bool;
    var phi_4798_: bool;
    var phi_34228_: bool;
    var phi_4802_: vec3<f32>;
    var phi_4803_: vec3<f32>;
    var phi_4804_: bool;
    var phi_4805_: vec3<f32>;
    var phi_4806_: bool;
    var phi_5126_: vec3<f32>;
    var phi_5129_: u32;
    var phi_25159_: f32;
    var phi_25174_: f32;
    var phi_25189_: f32;
    var phi_5200_: i32;
    var phi_5230_: i32;
    var phi_5238_: vec3<f32>;
    var phi_5241_: i32;
    var phi_5264_: vec3<f32>;
    var phi_5267_: i32;
    var phi_5338_: vec3<f32>;
    var phi_5341_: u32;
    var phi_5461_: vec3<f32>;
    var phi_5339_: vec3<f32>;
    var phi_5342_: u32;
    var phi_5265_: vec3<f32>;
    var phi_5268_: i32;
    var phi_5239_: vec3<f32>;
    var phi_5242_: i32;
    var phi_5466_: vec3<f32>;
    var phi_5467_: vec3<f32>;
    var phi_5468_: vec3<f32>;
    var phi_5127_: vec3<f32>;
    var phi_5130_: u32;
    var phi_34223_: bool;
    var phi_34265_: bool;
    var phi_5488_: vec3<f32>;
    var phi_5701_: u32;
    var phi_5702_: bool;
    var phi_5705_: u32;
    var phi_5706_: bool;
    var phi_5707_: bool;
    var phi_5717_: f32;
    var phi_5720_: f32;
    var phi_5722_: u32;
    var phi_5724_: vec3<f32>;
    var phi_5726_: vec3<f32>;
    var phi_5728_: f32;
    var phi_5730_: f32;
    var phi_5768_: u32;
    var phi_5783_: vec3<f32>;
    var phi_5803_: vec3<f32>;
    var phi_25204_: vec3<f32>;
    var phi_5804_: u32;
    var phi_5805_: vec3<f32>;
    var phi_5806_: u32;
    var phi_5807_: vec3<f32>;
    var phi_5808_: u32;
    var phi_5810_: vec3<f32>;
    var phi_5945_: u32;
    var phi_5948_: f32;
    var phi_5950_: f32;
    var phi_5952_: f32;
    var phi_5954_: vec3<f32>;
    var phi_25259_: f32;
    var phi_25265_: f32;
    var phi_25293_: f32;
    var phi_25299_: f32;
    var phi_25327_: f32;
    var phi_25333_: f32;
    var phi_25361_: f32;
    var phi_25367_: f32;
    var phi_25395_: f32;
    var phi_25401_: f32;
    var phi_25429_: f32;
    var phi_25435_: f32;
    var phi_25463_: f32;
    var phi_25469_: f32;
    var phi_25497_: f32;
    var phi_25503_: f32;
    var phi_5946_: u32;
    var phi_5949_: f32;
    var phi_5951_: f32;
    var phi_5953_: f32;
    var phi_5955_: vec3<f32>;
    var local_23: f32;
    var local_24: f32;
    var local_25: f32;
    var phi_6371_: f32;
    var phi_6423_: vec3<f32>;
    var phi_6424_: f32;
    var phi_6425_: vec3<f32>;
    var phi_6426_: f32;
    var phi_6427_: vec3<f32>;
    var phi_6428_: f32;
    var phi_6430_: vec3<f32>;
    var phi_6431_: f32;
    var phi_6432_: f32;
    var phi_6433_: vec3<f32>;
    var phi_6434_: vec3<f32>;
    var phi_6435_: f32;
    var phi_6436_: f32;
    var phi_6437_: vec3<f32>;
    var phi_6438_: vec3<f32>;
    var phi_6439_: f32;
    var phi_6440_: f32;
    var phi_6441_: vec3<f32>;
    var phi_6442_: vec3<f32>;
    var phi_6443_: f32;
    var phi_6444_: f32;
    var phi_6445_: vec3<f32>;
    var phi_6446_: vec3<f32>;
    var phi_6448_: f32;
    var phi_6449_: f32;
    var phi_6450_: vec3<f32>;
    var phi_6451_: vec3<f32>;
    var phi_6452_: f32;
    var phi_6453_: f32;
    var phi_6454_: f32;
    var phi_6455_: f32;
    var phi_6456_: vec3<f32>;
    var phi_6457_: vec3<f32>;
    var phi_6458_: f32;
    var phi_6459_: f32;
    var phi_5718_: f32;
    var phi_5721_: f32;
    var phi_5723_: u32;
    var phi_5725_: vec3<f32>;
    var phi_5727_: vec3<f32>;
    var phi_5729_: f32;
    var phi_5731_: f32;
    var local_26: f32;
    var local_27: f32;
    var local_28: f32;
    var phi_25579_: bool;
    var local_29: f32;
    var phi_6491_: vec3<f32>;
    var local_30: f32;
    var local_31: f32;
    var local_32: f32;
    var phi_6570_: f32;
    var phi_6571_: vec3<f32>;
    var phi_6572_: f32;
    var phi_6574_: vec3<f32>;
    var phi_6577_: u32;
    var phi_7025_: f32;
    var phi_7205_: u32;
    var phi_7208_: f32;
    var phi_7210_: f32;
    var phi_7212_: f32;
    var phi_7214_: vec3<f32>;
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
    var phi_7206_: u32;
    var phi_7209_: f32;
    var phi_7211_: f32;
    var phi_7213_: f32;
    var phi_7215_: vec3<f32>;
    var local_33: f32;
    var local_34: f32;
    var local_35: f32;
    var phi_7628_: f32;
    var phi_25958_: bool;
    var phi_25973_: bool;
    var phi_7721_: vec3<f32>;
    var phi_7722_: vec3<f32>;
    var phi_7723_: vec3<f32>;
    var phi_7724_: vec3<f32>;
    var phi_6575_: vec3<f32>;
    var phi_6578_: u32;
    var phi_7743_: vec3<f32>;
    var phi_7746_: u32;
    var phi_7748_: f32;
    var phi_8049_: vec3<f32>;
    var phi_8050_: u32;
    var phi_8051_: f32;
    var phi_8052_: vec3<f32>;
    var phi_8069_: vec3<f32>;
    var phi_8071_: u32;
    var phi_8074_: vec3<f32>;
    var phi_8076_: f32;
    var phi_8078_: f32;
    var phi_8080_: f32;
    var phi_8100_: bool;
    var phi_8116_: bool;
    var phi_8118_: bool;
    var phi_8120_: bool;
    var phi_8121_: bool;
    var phi_8122_: bool;
    var phi_8123_: bool;
    var phi_8124_: bool;
    var phi_8168_: vec3<f32>;
    var phi_25992_: vec3<f32>;
    var phi_8169_: u32;
    var phi_8170_: vec3<f32>;
    var phi_8171_: u32;
    var phi_8172_: vec3<f32>;
    var phi_8307_: u32;
    var phi_8310_: f32;
    var phi_8312_: f32;
    var phi_8314_: f32;
    var phi_8316_: vec3<f32>;
    var phi_26047_: f32;
    var phi_26053_: f32;
    var phi_26081_: f32;
    var phi_26087_: f32;
    var phi_26115_: f32;
    var phi_26121_: f32;
    var phi_26149_: f32;
    var phi_26155_: f32;
    var phi_26183_: f32;
    var phi_26189_: f32;
    var phi_26217_: f32;
    var phi_26223_: f32;
    var phi_26251_: f32;
    var phi_26257_: f32;
    var phi_26285_: f32;
    var phi_26291_: f32;
    var phi_8308_: u32;
    var phi_8311_: f32;
    var phi_8313_: f32;
    var phi_8315_: f32;
    var phi_8317_: vec3<f32>;
    var local_36: f32;
    var local_37: f32;
    var local_38: f32;
    var phi_8730_: f32;
    var phi_8756_: vec3<f32>;
    var phi_8757_: f32;
    var phi_8758_: f32;
    var phi_8759_: vec3<f32>;
    var phi_8760_: f32;
    var phi_8761_: f32;
    var phi_8762_: vec3<f32>;
    var phi_8763_: f32;
    var phi_8764_: f32;
    var phi_8765_: vec3<f32>;
    var phi_8766_: f32;
    var phi_8767_: f32;
    var phi_8769_: vec3<f32>;
    var phi_8770_: f32;
    var phi_8771_: f32;
    var phi_8772_: f32;
    var phi_8072_: u32;
    var phi_8075_: vec3<f32>;
    var phi_8077_: f32;
    var phi_8079_: f32;
    var phi_8081_: f32;
    var local_39: f32;
    var local_40: f32;
    var local_41: f32;
    var phi_9062_: type_59;
    var local_42: f32;
    var phi_9063_: type_59;
    var local_43: f32;
    var local_44: f32;
    var phi_9064_: type_59;
    var phi_9065_: type_59;
    var phi_7744_: vec3<f32>;
    var phi_7747_: u32;
    var phi_7749_: f32;
    var local_45: f32;
    var local_46: f32;
    var local_47: f32;
    var local_48: f32;
    var phi_9094_: vec3<f32>;
    var local_49: vec3<f32>;
    var local_50: vec3<f32>;
    var local_51: vec3<f32>;
    var local_52: vec3<f32>;
    var local_53: vec3<f32>;
    var local_54: vec3<f32>;
    var phi_34473_: bool;
    var phi_9141_: vec3<f32>;
    var phi_34472_: bool;
    var phi_9142_: vec3<f32>;
    var phi_34471_: bool;
    var phi_9143_: vec3<f32>;
    var phi_34467_: bool;
    var phi_10416_: vec3<f32>;
    var phi_34466_: bool;
    var phi_10417_: vec3<f32>;
    var phi_34465_: bool;
    var phi_535_: vec3<f32>;
    var phi_538_: u32;
    var phi_34464_: bool;
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
            let _e285 = global;
            let _e295 = global_1.member.member_7[2u];
            let _e299 = (_e285.x + (_e285.y * 4194240u));
            let _e304 = atomicLoad((&global_8.member[((_e295 * 4u) + 3u)]));
            if (_e299 >= _e304) {
                phi_514_ = type_48(0u, type_48().member_1);
            } else {
                let _e309 = global_1.member.member[0u];
                let _e313 = global_1.member.member[1u];
                let _e318 = global_1.member.member_7[3u];
                let _e325 = atomicLoad((&global_8.member[((12u + (_e295 * ((_e309 * _e313) * _e318))) + _e299)]));
                phi_514_ = type_48(1u, _e325);
            }
            let _e331 = phi_514_;
            if (bitcast<i32>(_e331.member) != 0i) {
                phi_34003_ = false;
                phi_534_ = vec3<f32>(0f, 0f, 0f);
                phi_537_ = 0u;
                loop {
                    let _e337 = phi_34003_;
                    let _e339 = phi_534_;
                    let _e341 = phi_537_;
                    local_55 = _e339;
                    local_56 = _e339;
                    local_57 = _e339;
                    let _e342 = (_e341 < 2u);
                    if _e342 {
                        let _e344 = ((2u * _e331.member_1) + _e341);
                        if (_e344 < arrayLength((&global_9.member))) {
                        } else {
                            phi_34464_ = true;
                            break;
                        }
                        let _e348 = global_9.member[_e344];
                        if ((((_e348.member_2.x * _e348.member_2.x) + (_e348.member_2.y * _e348.member_2.y)) + (_e348.member_2.z * _e348.member_2.z)) == 0f) {
                            phi_34466_ = _e337;
                            phi_10417_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            if (_e348.member.w < 0f) {
                                let _e1303 = global_1.member.member_8[0u];
                                let _e1306 = global_1.member.member_8[3u];
                                if (_e1303 == 4294967295u) {
                                    let _e1308 = (_e1306 == 4294967295u);
                                    if _e1308 {
                                        let _e1311 = global_1.member.member_3[1u];
                                        let _e1314 = global_1.member.member_3[2u];
                                        let _e1317 = ((atan2(_e348.member_1.z, _e348.member_1.x) * 0.15915494f) + 0.5f);
                                        let _e1322 = (((_e1317 - floor(_e1317)) * f32(_e1311)) - 0.5f);
                                        let _e1324 = select(_e348.member_1.y, -1f, (_e348.member_1.y < -1f));
                                        let _e1331 = (((acos(select(_e1324, 1f, (_e1324 > 1f))) * 0.31830987f) * f32(_e1314)) - 0.5f);
                                        let _e1332 = (_e1314 - 1u);
                                        let _e1333 = f32(_e1332);
                                        if (0f <= _e1333) {
                                        } else {
                                            phi_34464_ = true;
                                            break;
                                        }
                                        let _e1336 = select(_e1331, 0f, (_e1331 < 0f));
                                        let _e1338 = select(_e1336, _e1333, (_e1336 > _e1333));
                                        let _e1339 = floor(_e1322);
                                        let _e1340 = floor(_e1338);
                                        let _e1341 = (_e1322 - _e1339);
                                        let _e1342 = (_e1338 - _e1340);
                                        let _e1353 = (_e1311 == 0u);
                                        if _e1353 {
                                            phi_34464_ = true;
                                            break;
                                        }
                                        let _e1354 = (bitcast<u32>((select(0i, select(select(i32(_e1339), i32(-2147483648), (_e1339 < -2147483600f)), 2147483647i, (_e1339 > 2147483500f)), (_e1339 == _e1339)) + bitcast<i32>(_e1311))) % _e1311);
                                        let _e1359 = select(select(u32(_e1340), 0u, (_e1340 < 0f)), 4294967295u, (_e1340 > 4294967000f));
                                        let _e1360 = (_e1359 + 1u);
                                        let _e1365 = global_1.member.member_2[2u];
                                        let _e1366 = (_e1359 * _e1311);
                                        if _e1353 {
                                            phi_34464_ = true;
                                            break;
                                        }
                                        let _e1367 = (_e1354 % _e1311);
                                        let _e1370 = (_e1365 + (3u * (_e1366 + _e1367)));
                                        let _e1372 = global_6.member[_e1370];
                                        let _e1375 = global_6.member[(_e1370 + 1u)];
                                        let _e1378 = global_6.member[(_e1370 + 2u)];
                                        if _e1353 {
                                            phi_34464_ = true;
                                            break;
                                        }
                                        let _e1381 = ((_e1354 + 1u) % _e1311);
                                        let _e1384 = (_e1365 + (3u * (_e1366 + _e1381)));
                                        let _e1386 = global_6.member[_e1384];
                                        let _e1389 = global_6.member[(_e1384 + 1u)];
                                        let _e1392 = global_6.member[(_e1384 + 2u)];
                                        let _e1400 = (vec3<f32>(_e1372, _e1375, _e1378) + vec3<f32>(((_e1386 - _e1372) * _e1341), ((_e1389 - _e1375) * _e1341), ((_e1392 - _e1378) * _e1341)));
                                        let _e1401 = (select(_e1360, _e1332, (_e1332 < _e1360)) * _e1311);
                                        if _e1353 {
                                            phi_34464_ = true;
                                            break;
                                        }
                                        let _e1404 = (_e1365 + (3u * (_e1401 + _e1367)));
                                        let _e1406 = global_6.member[_e1404];
                                        let _e1409 = global_6.member[(_e1404 + 1u)];
                                        let _e1412 = global_6.member[(_e1404 + 2u)];
                                        if _e1353 {
                                            phi_34464_ = true;
                                            break;
                                        }
                                        let _e1416 = (_e1365 + (3u * (_e1401 + _e1381)));
                                        let _e1418 = global_6.member[_e1416];
                                        let _e1421 = global_6.member[(_e1416 + 1u)];
                                        let _e1424 = global_6.member[(_e1416 + 2u)];
                                        let _e1432 = (vec3<f32>(_e1406, _e1409, _e1412) + vec3<f32>(((_e1418 - _e1406) * _e1341), ((_e1421 - _e1409) * _e1341), ((_e1424 - _e1412) * _e1341)));
                                        phi_913_ = (_e1400 + vec3<f32>(((_e1432.x - _e1400.x) * _e1342), ((_e1432.y - _e1400.y) * _e1342), ((_e1432.z - _e1400.z) * _e1342)));
                                    } else {
                                        phi_913_ = vec3<f32>();
                                    }
                                    let _e1448 = phi_913_;
                                    phi_915_ = _e1448;
                                    phi_916_ = select(true, false, _e1308);
                                } else {
                                    phi_915_ = vec3<f32>();
                                    phi_916_ = true;
                                }
                                let _e1451 = phi_915_;
                                let _e1453 = phi_916_;
                                if _e1453 {
                                    let _e1454 = (_e1306 != 4294967295u);
                                    if (_e1303 != 4294967295u) {
                                        phi_34078_ = _e337;
                                        phi_926_ = vec3<f32>(0f, 0f, 0f);
                                        phi_929_ = 0i;
                                        phi_931_ = _e1303;
                                        loop {
                                            let _e1457 = phi_34078_;
                                            let _e1459 = phi_926_;
                                            let _e1461 = phi_929_;
                                            let _e1463 = phi_931_;
                                            local_17 = _e1459;
                                            local_18 = _e1459;
                                            local_19 = _e1459;
                                            local_61 = _e1459;
                                            local_62 = _e1459;
                                            if (_e1463 != 4294967295u) {
                                                let _e1465 = (_e1461 < 2i);
                                                if _e1465 {
                                                    let _e1467 = global_6.member[_e1463];
                                                    let _e1470 = global_6.member[(_e1463 + 1u)];
                                                    let _e1473 = global_6.member[(_e1463 + 2u)];
                                                    let _e1476 = global_6.member[(_e1463 + 3u)];
                                                    let _e1479 = global_6.member[(_e1463 + 4u)];
                                                    let _e1482 = global_6.member[(_e1463 + 5u)];
                                                    let _e1485 = global_6.member[(_e1463 + 6u)];
                                                    let _e1488 = global_6.member[(_e1463 + 7u)];
                                                    let _e1491 = global_6.member[(_e1463 + 8u)];
                                                    let _e1494 = global_6.member[(_e1463 + 9u)];
                                                    let _e1497 = global_6.member[(_e1463 + 10u)];
                                                    let _e1500 = global_6.member[(_e1463 + 11u)];
                                                    let _e1503 = global_6.member[(_e1463 + 12u)];
                                                    let _e1506 = global_6.member[(_e1463 + 13u)];
                                                    let _e1509 = global_6.member[(_e1463 + 14u)];
                                                    let _e1512 = global_6.member[(_e1463 + 15u)];
                                                    let _e1516 = global_6.member[(_e1463 + 16u)];
                                                    let _e1519 = global_6.member[(_e1463 + 17u)];
                                                    let _e1522 = global_6.member[(_e1463 + 18u)];
                                                    let _e1525 = global_6.member[(_e1463 + 19u)];
                                                    let _e1528 = global_6.member[(_e1463 + 20u)];
                                                    let _e1531 = global_6.member[(_e1463 + 30u)];
                                                    let _e1532 = bitcast<u32>(_e1531);
                                                    let _e1535 = global_6.member[(_e1463 + 31u)];
                                                    let _e1536 = bitcast<u32>(_e1535);
                                                    let _e1539 = global_6.member[(_e1463 + 32u)];
                                                    let _e1541 = (_e348.member_1.y < 0f);
                                                    if _e1541 {
                                                        phi_2719_ = type_52();
                                                        phi_2720_ = true;
                                                        phi_2721_ = false;
                                                    } else {
                                                        let _e1542 = (_e1536 == 4294967295u);
                                                        if _e1542 {
                                                            phi_2717_ = type_52();
                                                            phi_2718_ = false;
                                                        } else {
                                                            let _e1556 = (_e348.member.x - _e1467);
                                                            let _e1557 = (_e348.member.y - _e1470);
                                                            let _e1558 = (_e348.member.z - _e1473);
                                                            let _e1566 = ((((((_e348.member.x * _e348.member.x) + (_e348.member.y * _e348.member.y)) + (_e348.member.z * _e348.member.z)) - (2f * (((_e348.member.x * _e1467) + (_e348.member.y * _e1470)) + (_e348.member.z * _e1473)))) + _e1482) / (sqrt((((_e1556 * _e1556) + (_e1557 * _e1557)) + (_e1558 * _e1558))) + _e1476));
                                                            let _e1567 = (_e1463 + 21u);
                                                            let _e1570 = global_6.member[(_e1463 + 28u)];
                                                            if (_e1566 > _e1570) {
                                                                phi_2707_ = type_55(0u, type_55().member_1);
                                                            } else {
                                                                let _e1573 = global_6.member[_e1567];
                                                                if (_e1566 != _e1566) {
                                                                    phi_23157_ = true;
                                                                } else {
                                                                    phi_23157_ = (_e1573 >= _e1566);
                                                                }
                                                                let _e1577 = phi_23157_;
                                                                let _e1578 = select(_e1566, _e1573, _e1577);
                                                                phi_1353_ = 0u;
                                                                loop {
                                                                    let _e1580 = phi_1353_;
                                                                    local_8 = _e1580;
                                                                    local_9 = _e1580;
                                                                    local_10 = _e1580;
                                                                    if (_e1580 < 8u) {
                                                                        let _e1584 = global_6.member[(_e1567 + _e1580)];
                                                                        let _e1585 = (_e1584 <= _e1578);
                                                                        if _e1585 {
                                                                            phi_1383_ = (_e1580 + 1u);
                                                                        } else {
                                                                            phi_1383_ = u32();
                                                                        }
                                                                        let _e1588 = phi_1383_;
                                                                        phi_1354_ = _e1588;
                                                                        phi_1384_ = _e1585;
                                                                    } else {
                                                                        phi_1354_ = u32();
                                                                        phi_1384_ = false;
                                                                    }
                                                                    let _e1590 = phi_1354_;
                                                                    let _e1592 = phi_1384_;
                                                                    phi_1353_ = _e1590;
                                                                    if !(_e1592) {
                                                                        break;
                                                                    }
                                                                }
                                                                let _e1595 = local_8;
                                                                if (_e1595 < 1u) {
                                                                    phi_1402_ = 1u;
                                                                } else {
                                                                    let _e1598 = local_9;
                                                                    let _e1601 = local_10;
                                                                    phi_1402_ = select(_e1601, 7u, (_e1598 > 7u));
                                                                }
                                                                let _e1604 = phi_1402_;
                                                                let _e1605 = (_e1604 - 1u);
                                                                let _e1608 = global_6.member[(_e1567 + _e1605)];
                                                                let _e1611 = global_6.member[(_e1567 + _e1604)];
                                                                let _e1614 = ((_e1578 - _e1608) / (_e1611 - _e1608));
                                                                let _e1615 = select(_e348.member_1.y, 0f, _e1541);
                                                                let _e1621 = (sqrt((asin(select(_e1615, 1f, (_e1615 > 1f))) * 0.63661975f)) * 127f);
                                                                let _e1623 = (atan2(_e348.member_1.z, _e348.member_1.x) * 0.15915494f);
                                                                let _e1626 = ((_e1623 - floor(_e1623)) * 256f);
                                                                let _e1627 = floor(_e1626);
                                                                let _e1633 = (select(select(u32(_e1627), 0u, (_e1627 < 0f)), 4294967295u, (_e1627 > 4294967000f)) % 256u);
                                                                let _e1634 = floor(_e1621);
                                                                let _e1639 = select(select(u32(_e1634), 0u, (_e1634 < 0f)), 4294967295u, (_e1634 > 4294967000f));
                                                                let _e1641 = select(_e1639, 126u, (126u < _e1639));
                                                                let _e1643 = ((_e1633 + 1u) % 256u);
                                                                let _e1645 = (_e1626 - _e1627);
                                                                let _e1647 = (_e1621 - f32(_e1641));
                                                                let _e1649 = (_e1536 + (_e1605 * 294912u));
                                                                let _e1650 = (_e1641 * 256u);
                                                                let _e1652 = (9u * (_e1650 + _e1633));
                                                                let _e1653 = (_e1649 + _e1652);
                                                                let _e1655 = (9u * (_e1650 + _e1643));
                                                                let _e1656 = (_e1649 + _e1655);
                                                                let _e1657 = ((_e1641 + 1u) * 256u);
                                                                let _e1659 = (9u * (_e1657 + _e1633));
                                                                let _e1660 = (_e1649 + _e1659);
                                                                let _e1662 = (9u * (_e1657 + _e1643));
                                                                let _e1663 = (_e1649 + _e1662);
                                                                let _e1664 = (1f - _e1645);
                                                                let _e1665 = (1f - _e1647);
                                                                let _e1666 = (_e1664 * _e1665);
                                                                let _e1667 = (_e1645 * _e1665);
                                                                let _e1668 = (_e1664 * _e1647);
                                                                let _e1669 = (_e1645 * _e1647);
                                                                let _e1671 = global_6.member[_e1653];
                                                                let _e1674 = global_6.member[(_e1653 + 1u)];
                                                                let _e1677 = global_6.member[(_e1653 + 2u)];
                                                                let _e1682 = global_6.member[_e1656];
                                                                let _e1685 = global_6.member[(_e1656 + 1u)];
                                                                let _e1688 = global_6.member[(_e1656 + 2u)];
                                                                let _e1696 = global_6.member[_e1660];
                                                                let _e1699 = global_6.member[(_e1660 + 1u)];
                                                                let _e1702 = global_6.member[(_e1660 + 2u)];
                                                                let _e1710 = global_6.member[_e1663];
                                                                let _e1713 = global_6.member[(_e1663 + 1u)];
                                                                let _e1716 = global_6.member[(_e1663 + 2u)];
                                                                let _e1725 = global_6.member[(_e1653 + 3u)];
                                                                let _e1728 = global_6.member[(_e1653 + 4u)];
                                                                let _e1731 = global_6.member[(_e1653 + 5u)];
                                                                let _e1737 = global_6.member[(_e1656 + 3u)];
                                                                let _e1740 = global_6.member[(_e1656 + 4u)];
                                                                let _e1743 = global_6.member[(_e1656 + 5u)];
                                                                let _e1752 = global_6.member[(_e1660 + 3u)];
                                                                let _e1755 = global_6.member[(_e1660 + 4u)];
                                                                let _e1758 = global_6.member[(_e1660 + 5u)];
                                                                let _e1767 = global_6.member[(_e1663 + 3u)];
                                                                let _e1770 = global_6.member[(_e1663 + 4u)];
                                                                let _e1773 = global_6.member[(_e1663 + 5u)];
                                                                let _e1782 = global_6.member[(_e1653 + 6u)];
                                                                let _e1785 = global_6.member[(_e1653 + 7u)];
                                                                let _e1788 = global_6.member[(_e1653 + 8u)];
                                                                let _e1794 = global_6.member[(_e1656 + 6u)];
                                                                let _e1797 = global_6.member[(_e1656 + 7u)];
                                                                let _e1800 = global_6.member[(_e1656 + 8u)];
                                                                let _e1809 = global_6.member[(_e1660 + 6u)];
                                                                let _e1812 = global_6.member[(_e1660 + 7u)];
                                                                let _e1815 = global_6.member[(_e1660 + 8u)];
                                                                let _e1824 = global_6.member[(_e1663 + 6u)];
                                                                let _e1827 = global_6.member[(_e1663 + 7u)];
                                                                let _e1830 = global_6.member[(_e1663 + 8u)];
                                                                let _e1838 = (_e1536 + (_e1604 * 294912u));
                                                                let _e1839 = (_e1838 + _e1652);
                                                                let _e1840 = (_e1838 + _e1655);
                                                                let _e1841 = (_e1838 + _e1659);
                                                                let _e1842 = (_e1838 + _e1662);
                                                                let _e1844 = global_6.member[_e1839];
                                                                let _e1847 = global_6.member[(_e1839 + 1u)];
                                                                let _e1850 = global_6.member[(_e1839 + 2u)];
                                                                let _e1855 = global_6.member[_e1840];
                                                                let _e1858 = global_6.member[(_e1840 + 1u)];
                                                                let _e1861 = global_6.member[(_e1840 + 2u)];
                                                                let _e1869 = global_6.member[_e1841];
                                                                let _e1872 = global_6.member[(_e1841 + 1u)];
                                                                let _e1875 = global_6.member[(_e1841 + 2u)];
                                                                let _e1883 = global_6.member[_e1842];
                                                                let _e1886 = global_6.member[(_e1842 + 1u)];
                                                                let _e1889 = global_6.member[(_e1842 + 2u)];
                                                                let _e1898 = global_6.member[(_e1839 + 3u)];
                                                                let _e1901 = global_6.member[(_e1839 + 4u)];
                                                                let _e1904 = global_6.member[(_e1839 + 5u)];
                                                                let _e1910 = global_6.member[(_e1840 + 3u)];
                                                                let _e1913 = global_6.member[(_e1840 + 4u)];
                                                                let _e1916 = global_6.member[(_e1840 + 5u)];
                                                                let _e1925 = global_6.member[(_e1841 + 3u)];
                                                                let _e1928 = global_6.member[(_e1841 + 4u)];
                                                                let _e1931 = global_6.member[(_e1841 + 5u)];
                                                                let _e1940 = global_6.member[(_e1842 + 3u)];
                                                                let _e1943 = global_6.member[(_e1842 + 4u)];
                                                                let _e1946 = global_6.member[(_e1842 + 5u)];
                                                                let _e1955 = global_6.member[(_e1839 + 6u)];
                                                                let _e1958 = global_6.member[(_e1839 + 7u)];
                                                                let _e1961 = global_6.member[(_e1839 + 8u)];
                                                                let _e1967 = global_6.member[(_e1840 + 6u)];
                                                                let _e1970 = global_6.member[(_e1840 + 7u)];
                                                                let _e1973 = global_6.member[(_e1840 + 8u)];
                                                                let _e1982 = global_6.member[(_e1841 + 6u)];
                                                                let _e1985 = global_6.member[(_e1841 + 7u)];
                                                                let _e1988 = global_6.member[(_e1841 + 8u)];
                                                                let _e1997 = global_6.member[(_e1842 + 6u)];
                                                                let _e2000 = global_6.member[(_e1842 + 7u)];
                                                                let _e2003 = global_6.member[(_e1842 + 8u)];
                                                                let _e2010 = (1f - _e1614);
                                                                phi_2707_ = type_55(1u, type_52(vec3<f32>(((((((_e1671 * _e1666) + (_e1682 * _e1667)) + (_e1696 * _e1668)) + (_e1710 * _e1669)) * _e2010) + (((((_e1844 * _e1666) + (_e1855 * _e1667)) + (_e1869 * _e1668)) + (_e1883 * _e1669)) * _e1614)), ((((((_e1674 * _e1666) + (_e1685 * _e1667)) + (_e1699 * _e1668)) + (_e1713 * _e1669)) * _e2010) + (((((_e1847 * _e1666) + (_e1858 * _e1667)) + (_e1872 * _e1668)) + (_e1886 * _e1669)) * _e1614)), ((((((_e1677 * _e1666) + (_e1688 * _e1667)) + (_e1702 * _e1668)) + (_e1716 * _e1669)) * _e2010) + (((((_e1850 * _e1666) + (_e1861 * _e1667)) + (_e1875 * _e1668)) + (_e1889 * _e1669)) * _e1614))), vec3<f32>(((((((_e1725 * _e1666) + (_e1737 * _e1667)) + (_e1752 * _e1668)) + (_e1767 * _e1669)) * _e2010) + (((((_e1898 * _e1666) + (_e1910 * _e1667)) + (_e1925 * _e1668)) + (_e1940 * _e1669)) * _e1614)), ((((((_e1728 * _e1666) + (_e1740 * _e1667)) + (_e1755 * _e1668)) + (_e1770 * _e1669)) * _e2010) + (((((_e1901 * _e1666) + (_e1913 * _e1667)) + (_e1928 * _e1668)) + (_e1943 * _e1669)) * _e1614)), ((((((_e1731 * _e1666) + (_e1743 * _e1667)) + (_e1758 * _e1668)) + (_e1773 * _e1669)) * _e2010) + (((((_e1904 * _e1666) + (_e1916 * _e1667)) + (_e1931 * _e1668)) + (_e1946 * _e1669)) * _e1614))), vec3<f32>(((((((_e1782 * _e1666) + (_e1794 * _e1667)) + (_e1809 * _e1668)) + (_e1824 * _e1669)) * _e2010) + (((((_e1955 * _e1666) + (_e1967 * _e1667)) + (_e1982 * _e1668)) + (_e1997 * _e1669)) * _e1614)), ((((((_e1785 * _e1666) + (_e1797 * _e1667)) + (_e1812 * _e1668)) + (_e1827 * _e1669)) * _e2010) + (((((_e1958 * _e1666) + (_e1970 * _e1667)) + (_e1985 * _e1668)) + (_e2000 * _e1669)) * _e1614)), ((((((_e1788 * _e1666) + (_e1800 * _e1667)) + (_e1815 * _e1668)) + (_e1830 * _e1669)) * _e2010) + (((((_e1961 * _e1666) + (_e1973 * _e1667)) + (_e1988 * _e1668)) + (_e2003 * _e1669)) * _e1614)))));
                                                            }
                                                            let _e2047 = phi_2707_;
                                                            let _e2050 = (bitcast<i32>(_e2047.member) != 0i);
                                                            if _e2050 {
                                                                phi_2715_ = _e2047.member_1;
                                                            } else {
                                                                phi_2715_ = type_52();
                                                            }
                                                            let _e2053 = phi_2715_;
                                                            phi_2717_ = _e2053;
                                                            phi_2718_ = select(true, false, _e2050);
                                                        }
                                                        let _e2056 = phi_2717_;
                                                        let _e2058 = phi_2718_;
                                                        phi_2719_ = _e2056;
                                                        phi_2720_ = _e1542;
                                                        phi_2721_ = _e2058;
                                                    }
                                                    let _e2060 = phi_2719_;
                                                    let _e2062 = phi_2720_;
                                                    let _e2064 = phi_2721_;
                                                    if select(_e2064, true, _e2062) {
                                                        let _e2073 = ((((_e348.member.x - _e1467) * _e348.member_1.x) + ((_e348.member.y - _e1470) * _e348.member_1.y)) + ((_e348.member.z - _e1473) * _e348.member_1.z));
                                                        let _e2085 = ((((_e348.member.x * _e348.member.x) + (_e348.member.y * _e348.member.y)) + (_e348.member.z * _e348.member.z)) - (2f * (((_e348.member.x * _e1467) + (_e348.member.y * _e1470)) + (_e348.member.z * _e1473))));
                                                        let _e2086 = (_e2085 + _e1485);
                                                        let _e2087 = (_e2073 * _e2073);
                                                        let _e2088 = (_e2087 - _e2086);
                                                        if (_e2088 < 0f) {
                                                            phi_23296_ = type_57(0u, type_57().member_1);
                                                            phi_23297_ = type_57();
                                                            phi_23298_ = true;
                                                        } else {
                                                            if (_e2073 < 0f) {
                                                                phi_23268_ = -(sqrt(_e2088));
                                                            } else {
                                                                phi_23268_ = sqrt(_e2088);
                                                            }
                                                            let _e2095 = phi_23268_;
                                                            let _e2097 = -((_e2073 + _e2095));
                                                            let _e2098 = (_e2097 == 0f);
                                                            if _e2098 {
                                                                phi_23291_ = type_57(1u, type_56(0f, 0f));
                                                                phi_23292_ = type_57();
                                                            } else {
                                                                let _e2099 = (_e2086 / _e2097);
                                                                if (_e2097 < _e2099) {
                                                                    phi_23280_ = type_56(_e2097, _e2099);
                                                                } else {
                                                                    phi_23280_ = type_56(_e2099, _e2097);
                                                                }
                                                                let _e2104 = phi_23280_;
                                                                phi_23291_ = type_57();
                                                                phi_23292_ = type_57(1u, _e2104);
                                                            }
                                                            let _e2107 = phi_23291_;
                                                            let _e2109 = phi_23292_;
                                                            phi_23296_ = _e2107;
                                                            phi_23297_ = _e2109;
                                                            phi_23298_ = _e2098;
                                                        }
                                                        let _e2114 = phi_23296_;
                                                        let _e2116 = phi_23297_;
                                                        let _e2118 = phi_23298_;
                                                        if _e2118 {
                                                            phi_23302_ = _e2114;
                                                        } else {
                                                            phi_23302_ = _e2116;
                                                        }
                                                        let _e2120 = phi_23302_;
                                                        if (bitcast<i32>(_e2120.member) != 0i) {
                                                            phi_23189_ = type_57(0u, _e2120.member_1);
                                                        } else {
                                                            phi_23189_ = type_57(1u, type_57().member_1);
                                                        }
                                                        let _e2130 = phi_23189_;
                                                        if (bitcast<i32>(_e2130.member) != 0i) {
                                                            phi_23236_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e2130.member_1.member_1 <= 0f) {
                                                                phi_23232_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                if (_e2130.member_1.member != _e2130.member_1.member) {
                                                                    phi_23340_ = true;
                                                                } else {
                                                                    phi_23340_ = (0f >= _e2130.member_1.member);
                                                                }
                                                                let _e2142 = phi_23340_;
                                                                let _e2143 = select(_e2130.member_1.member, 0f, _e2142);
                                                                let _e2144 = (_e2085 + _e1482);
                                                                if (_e2144 > 0f) {
                                                                    let _e2146 = (_e2087 - _e2144);
                                                                    if (_e2146 < 0f) {
                                                                        phi_23448_ = type_57(0u, type_57().member_1);
                                                                        phi_23449_ = type_57();
                                                                        phi_23450_ = true;
                                                                    } else {
                                                                        if (_e2073 < 0f) {
                                                                            phi_23420_ = -(sqrt(_e2146));
                                                                        } else {
                                                                            phi_23420_ = sqrt(_e2146);
                                                                        }
                                                                        let _e2153 = phi_23420_;
                                                                        let _e2155 = -((_e2073 + _e2153));
                                                                        let _e2156 = (_e2155 == 0f);
                                                                        if _e2156 {
                                                                            phi_23443_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23444_ = type_57();
                                                                        } else {
                                                                            let _e2157 = (_e2144 / _e2155);
                                                                            if (_e2155 < _e2157) {
                                                                                phi_23432_ = type_56(_e2155, _e2157);
                                                                            } else {
                                                                                phi_23432_ = type_56(_e2157, _e2155);
                                                                            }
                                                                            let _e2162 = phi_23432_;
                                                                            phi_23443_ = type_57();
                                                                            phi_23444_ = type_57(1u, _e2162);
                                                                        }
                                                                        let _e2165 = phi_23443_;
                                                                        let _e2167 = phi_23444_;
                                                                        phi_23448_ = _e2165;
                                                                        phi_23449_ = _e2167;
                                                                        phi_23450_ = _e2156;
                                                                    }
                                                                    let _e2172 = phi_23448_;
                                                                    let _e2174 = phi_23449_;
                                                                    let _e2176 = phi_23450_;
                                                                    if _e2176 {
                                                                        phi_23454_ = _e2172;
                                                                    } else {
                                                                        phi_23454_ = _e2174;
                                                                    }
                                                                    let _e2178 = phi_23454_;
                                                                    if (bitcast<i32>(_e2178.member) != 0i) {
                                                                        phi_23388_ = select(_e2178.member_1.member_1, _e2178.member_1.member, (_e2178.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23388_ = -1f;
                                                                    }
                                                                    let _e2189 = phi_23388_;
                                                                    if (_e2189 >= 0f) {
                                                                        if (_e2130.member_1.member_1 != _e2130.member_1.member_1) {
                                                                            phi_23492_ = true;
                                                                        } else {
                                                                            phi_23492_ = (_e2189 <= _e2130.member_1.member_1);
                                                                        }
                                                                        let _e2194 = phi_23492_;
                                                                        phi_23217_ = select(_e2130.member_1.member_1, _e2189, _e2194);
                                                                    } else {
                                                                        phi_23217_ = _e2130.member_1.member_1;
                                                                    }
                                                                    let _e2197 = phi_23217_;
                                                                    phi_23219_ = _e2197;
                                                                } else {
                                                                    phi_23219_ = _e2130.member_1.member_1;
                                                                }
                                                                let _e2199 = phi_23219_;
                                                                if (_e2199 <= _e2143) {
                                                                    phi_23228_ = type_57(0u, type_57().member_1);
                                                                } else {
                                                                    phi_23228_ = type_57(1u, type_56(_e2143, _e2199));
                                                                }
                                                                let _e2207 = phi_23228_;
                                                                phi_23232_ = _e2207;
                                                            }
                                                            let _e2212 = phi_23232_;
                                                            phi_23236_ = _e2212;
                                                        }
                                                        let _e2217 = phi_23236_;
                                                        if (bitcast<i32>(_e2217.member) != 0i) {
                                                            phi_2739_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2742_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2744_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2746_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_2748_ = 0u;
                                                            loop {
                                                                let _e2226 = phi_2739_;
                                                                let _e2228 = phi_2742_;
                                                                let _e2230 = phi_2744_;
                                                                let _e2232 = phi_2746_;
                                                                let _e2234 = phi_2748_;
                                                                local_12 = _e2232;
                                                                local_13 = _e2230;
                                                                local_14 = _e2228;
                                                                let _e2235 = (_e2234 < 32u);
                                                                if _e2235 {
                                                                    let _e2236 = -(_e2073);
                                                                    if (_e2217.member_1.member <= _e2217.member_1.member_1) {
                                                                    } else {
                                                                        phi_34046_ = true;
                                                                        break;
                                                                    }
                                                                    let _e2239 = select(_e2236, _e2217.member_1.member, (_e2236 < _e2217.member_1.member));
                                                                    let _e2241 = select(_e2239, _e2217.member_1.member_1, (_e2239 > _e2217.member_1.member_1));
                                                                    let _e2242 = (_e2241 - _e2217.member_1.member);
                                                                    let _e2243 = (_e2217.member_1.member_1 - _e2241);
                                                                    let _e2244 = (_e2217.member_1.member_1 - _e2217.member_1.member);
                                                                    let _e2245 = (0.000000001f * _e2244);
                                                                    if (_e2242 <= _e2245) {
                                                                        let _e2320 = (f32(_e2234) * 0.03125f);
                                                                        let _e2323 = (f32((_e2234 + 1u)) * 0.03125f);
                                                                        let _e2325 = ((_e2320 + _e2323) * 0.5f);
                                                                        phi_2849_ = type_56((_e2217.member_1.member + ((_e2244 * _e2325) * _e2325)), (_e2244 * ((_e2323 * _e2323) - (_e2320 * _e2320))));
                                                                    } else {
                                                                        if (_e2243 <= _e2245) {
                                                                            let _e2303 = (f32((31u - _e2234)) * 0.03125f);
                                                                            let _e2306 = (f32((32u - _e2234)) * 0.03125f);
                                                                            let _e2308 = ((_e2303 + _e2306) * 0.5f);
                                                                            phi_2848_ = type_56((_e2217.member_1.member_1 - ((_e2244 * _e2308) * _e2308)), (_e2244 * ((_e2306 * _e2306) - (_e2303 * _e2303))));
                                                                        } else {
                                                                            let _e2251 = round(((32f * _e2242) / (_e2242 + _e2243)));
                                                                            let _e2256 = select(select(u32(_e2251), 0u, (_e2251 < 0f)), 4294967295u, (_e2251 > 4294967000f));
                                                                            if (_e2256 < 1u) {
                                                                                phi_2832_ = 1u;
                                                                            } else {
                                                                                phi_2832_ = select(_e2256, 31u, (_e2256 > 31u));
                                                                            }
                                                                            let _e2261 = phi_2832_;
                                                                            if (_e2234 < _e2261) {
                                                                                let _e2282 = ((_e2261 - 1u) - _e2234);
                                                                                let _e2284 = f32(_e2261);
                                                                                let _e2285 = (f32(_e2282) / _e2284);
                                                                                let _e2288 = (f32((_e2282 + 1u)) / _e2284);
                                                                                let _e2290 = ((_e2285 + _e2288) * 0.5f);
                                                                                phi_2847_ = type_56((_e2241 - ((_e2242 * _e2290) * _e2290)), (_e2242 * ((_e2288 * _e2288) - (_e2285 * _e2285))));
                                                                            } else {
                                                                                let _e2264 = (_e2234 - _e2261);
                                                                                let _e2266 = f32((32u - _e2261));
                                                                                let _e2267 = (f32(_e2264) / _e2266);
                                                                                let _e2270 = (f32((_e2264 + 1u)) / _e2266);
                                                                                let _e2272 = ((_e2267 + _e2270) * 0.5f);
                                                                                phi_2847_ = type_56((_e2241 + ((_e2243 * _e2272) * _e2272)), (_e2243 * ((_e2270 * _e2270) - (_e2267 * _e2267))));
                                                                            }
                                                                            let _e2300 = phi_2847_;
                                                                            phi_2848_ = _e2300;
                                                                        }
                                                                        let _e2318 = phi_2848_;
                                                                        phi_2849_ = _e2318;
                                                                    }
                                                                    let _e2335 = phi_2849_;
                                                                    let _e2341 = (_e348.member.x + (_e348.member_1.x * _e2335.member));
                                                                    let _e2342 = (_e348.member.y + (_e348.member_1.y * _e2335.member));
                                                                    let _e2343 = (_e348.member.z + (_e348.member_1.z * _e2335.member));
                                                                    let _e2356 = ((((_e2341 * _e2341) + (_e2342 * _e2342)) + (_e2343 * _e2343)) - (2f * (((_e2341 * _e1467) + (_e2342 * _e1470)) + (_e2343 * _e1473))));
                                                                    let _e2357 = (_e2356 + _e1482);
                                                                    let _e2358 = (_e2341 - _e1467);
                                                                    let _e2359 = (_e2342 - _e1470);
                                                                    let _e2360 = (_e2343 - _e1473);
                                                                    let _e2366 = sqrt((((_e2358 * _e2358) + (_e2359 * _e2359)) + (_e2360 * _e2360)));
                                                                    let _e2368 = (_e2357 / (_e2366 + _e1476));
                                                                    let _e2372 = (1f - (abs((_e2368 - 25000f)) * 0.00006666667f));
                                                                    if (_e2372 != _e2372) {
                                                                        phi_23691_ = true;
                                                                    } else {
                                                                        phi_23691_ = (0f >= _e2372);
                                                                    }
                                                                    let _e2376 = phi_23691_;
                                                                    let _e2378 = -(_e2368);
                                                                    let _e2381 = (exp((_e2378 / _e1497)) * _e2335.member_1);
                                                                    let _e2384 = (exp((_e2378 / _e1500)) * _e2335.member_1);
                                                                    let _e2386 = (_e2381 * 0.5f);
                                                                    let _e2387 = (_e2384 * 0.5f);
                                                                    let _e2388 = ((select(_e2372, 0f, _e2376) * _e2335.member_1) * 0.5f);
                                                                    let _e2390 = (_e2226.x + _e2386);
                                                                    let _e2392 = (_e2226.y + _e2387);
                                                                    let _e2394 = (_e2226.z + _e2388);
                                                                    let _e2399 = (_e1516 * 1.1f);
                                                                    let _e2400 = (_e2399 * _e2392);
                                                                    let _e2419 = (_e1516 * _e2384);
                                                                    if (_e1532 == 4294967295u) {
                                                                        phi_3177_ = vec3<f32>(0f, 0f, 0f);
                                                                    } else {
                                                                        let _e2427 = (1f / _e2366);
                                                                        let _e2435 = ((((_e2358 * _e2427) * _e1488) + ((_e2359 * _e2427) * _e1491)) + ((_e2360 * _e2427) * _e1494));
                                                                        if (_e2368 != _e2368) {
                                                                            phi_23818_ = true;
                                                                        } else {
                                                                            phi_23818_ = (0f >= _e2368);
                                                                        }
                                                                        let _e2439 = phi_23818_;
                                                                        let _e2443 = sqrt((select(_e2368, 0f, _e2439) / (_e1479 - _e1476)));
                                                                        if (_e2443 != _e2443) {
                                                                            phi_23833_ = true;
                                                                        } else {
                                                                            phi_23833_ = (1f <= _e2443);
                                                                        }
                                                                        let _e2447 = phi_23833_;
                                                                        let _e2449 = (select(_e2443, 1f, _e2447) * 31f);
                                                                        let _e2451 = select(_e2435, -1f, (_e2435 < -1f));
                                                                        let _e2455 = ((select(_e2451, 1f, (_e2451 > 1f)) + 1f) * 15.5f);
                                                                        let _e2460 = select(select(u32(_e2449), 0u, (_e2449 < 0f)), 4294967295u, (_e2449 > 4294967000f));
                                                                        let _e2462 = select(_e2460, 30u, (30u < _e2460));
                                                                        let _e2467 = select(select(u32(_e2455), 0u, (_e2455 < 0f)), 4294967295u, (_e2455 > 4294967000f));
                                                                        let _e2469 = select(_e2467, 30u, (30u < _e2467));
                                                                        let _e2471 = (_e2449 - f32(_e2462));
                                                                        let _e2473 = (_e2455 - f32(_e2469));
                                                                        let _e2477 = (_e1532 + (3u * ((_e2462 * 32u) + _e2469)));
                                                                        let _e2482 = (_e1532 + (3u * (((_e2462 + 1u) * 32u) + _e2469)));
                                                                        let _e2484 = global_6.member[_e2477];
                                                                        let _e2487 = global_6.member[(_e2477 + 1u)];
                                                                        let _e2490 = global_6.member[(_e2477 + 2u)];
                                                                        let _e2491 = (1f - _e2473);
                                                                        let _e2497 = global_6.member[(_e2477 + 3u)];
                                                                        let _e2500 = global_6.member[(_e2477 + 4u)];
                                                                        let _e2503 = global_6.member[(_e2477 + 5u)];
                                                                        let _e2510 = (1f - _e2471);
                                                                        let _e2515 = global_6.member[_e2482];
                                                                        let _e2518 = global_6.member[(_e2482 + 1u)];
                                                                        let _e2521 = global_6.member[(_e2482 + 2u)];
                                                                        let _e2527 = global_6.member[(_e2482 + 3u)];
                                                                        let _e2530 = global_6.member[(_e2482 + 4u)];
                                                                        let _e2533 = global_6.member[(_e2482 + 5u)];
                                                                        phi_3177_ = vec3<f32>(((((_e2484 * _e2491) + (_e2497 * _e2473)) * _e2510) + (((_e2515 * _e2491) + (_e2527 * _e2473)) * _e2471)), ((((_e2487 * _e2491) + (_e2500 * _e2473)) * _e2510) + (((_e2518 * _e2491) + (_e2530 * _e2473)) * _e2471)), ((((_e2490 * _e2491) + (_e2503 * _e2473)) * _e2510) + (((_e2521 * _e2491) + (_e2533 * _e2473)) * _e2471)));
                                                                    }
                                                                    let _e2548 = phi_3177_;
                                                                    let _e2566 = (((_e2358 * _e1488) + (_e2359 * _e1491)) + (_e2360 * _e1494));
                                                                    let _e2567 = (_e2566 * _e2566);
                                                                    let _e2568 = (_e2567 - _e2357);
                                                                    if (_e2568 < 0f) {
                                                                        phi_23983_ = type_57(0u, type_57().member_1);
                                                                        phi_23984_ = type_57();
                                                                        phi_23985_ = true;
                                                                    } else {
                                                                        if (_e2566 < 0f) {
                                                                            phi_23955_ = -(sqrt(_e2568));
                                                                        } else {
                                                                            phi_23955_ = sqrt(_e2568);
                                                                        }
                                                                        let _e2575 = phi_23955_;
                                                                        let _e2577 = -((_e2566 + _e2575));
                                                                        let _e2578 = (_e2577 == 0f);
                                                                        if _e2578 {
                                                                            phi_23978_ = type_57(1u, type_56(0f, 0f));
                                                                            phi_23979_ = type_57();
                                                                        } else {
                                                                            let _e2579 = (_e2357 / _e2577);
                                                                            if (_e2577 < _e2579) {
                                                                                phi_23967_ = type_56(_e2577, _e2579);
                                                                            } else {
                                                                                phi_23967_ = type_56(_e2579, _e2577);
                                                                            }
                                                                            let _e2584 = phi_23967_;
                                                                            phi_23978_ = type_57();
                                                                            phi_23979_ = type_57(1u, _e2584);
                                                                        }
                                                                        let _e2587 = phi_23978_;
                                                                        let _e2589 = phi_23979_;
                                                                        phi_23983_ = _e2587;
                                                                        phi_23984_ = _e2589;
                                                                        phi_23985_ = _e2578;
                                                                    }
                                                                    let _e2594 = phi_23983_;
                                                                    let _e2596 = phi_23984_;
                                                                    let _e2598 = phi_23985_;
                                                                    if _e2598 {
                                                                        phi_23989_ = _e2594;
                                                                    } else {
                                                                        phi_23989_ = _e2596;
                                                                    }
                                                                    let _e2600 = phi_23989_;
                                                                    if (bitcast<i32>(_e2600.member) != 0i) {
                                                                        phi_23923_ = select(_e2600.member_1.member_1, _e2600.member_1.member, (_e2600.member_1.member >= 0f));
                                                                    } else {
                                                                        phi_23923_ = -1f;
                                                                    }
                                                                    let _e2611 = phi_23923_;
                                                                    if (_e2611 >= 0f) {
                                                                        phi_23906_ = type_58(0u, type_58().member_1);
                                                                    } else {
                                                                        let _e2613 = (_e2356 + _e1485);
                                                                        let _e2614 = (_e2567 - _e2613);
                                                                        if (_e2614 < 0f) {
                                                                            phi_24093_ = type_57(0u, type_57().member_1);
                                                                            phi_24094_ = type_57();
                                                                            phi_24095_ = true;
                                                                        } else {
                                                                            if (_e2566 < 0f) {
                                                                                phi_24065_ = -(sqrt(_e2614));
                                                                            } else {
                                                                                phi_24065_ = sqrt(_e2614);
                                                                            }
                                                                            let _e2621 = phi_24065_;
                                                                            let _e2623 = -((_e2566 + _e2621));
                                                                            let _e2624 = (_e2623 == 0f);
                                                                            if _e2624 {
                                                                                phi_24088_ = type_57(1u, type_56(0f, 0f));
                                                                                phi_24089_ = type_57();
                                                                            } else {
                                                                                let _e2625 = (_e2613 / _e2623);
                                                                                if (_e2623 < _e2625) {
                                                                                    phi_24077_ = type_56(_e2623, _e2625);
                                                                                } else {
                                                                                    phi_24077_ = type_56(_e2625, _e2623);
                                                                                }
                                                                                let _e2630 = phi_24077_;
                                                                                phi_24088_ = type_57();
                                                                                phi_24089_ = type_57(1u, _e2630);
                                                                            }
                                                                            let _e2633 = phi_24088_;
                                                                            let _e2635 = phi_24089_;
                                                                            phi_24093_ = _e2633;
                                                                            phi_24094_ = _e2635;
                                                                            phi_24095_ = _e2624;
                                                                        }
                                                                        let _e2640 = phi_24093_;
                                                                        let _e2642 = phi_24094_;
                                                                        let _e2644 = phi_24095_;
                                                                        if _e2644 {
                                                                            phi_24099_ = _e2640;
                                                                        } else {
                                                                            phi_24099_ = _e2642;
                                                                        }
                                                                        let _e2646 = phi_24099_;
                                                                        if (bitcast<i32>(_e2646.member) != 0i) {
                                                                            phi_24033_ = select(_e2646.member_1.member_1, _e2646.member_1.member, (_e2646.member_1.member >= 0f));
                                                                        } else {
                                                                            phi_24033_ = -1f;
                                                                        }
                                                                        let _e2657 = phi_24033_;
                                                                        if (_e2657 < 0f) {
                                                                            phi_23902_ = type_58(0u, type_58().member_1);
                                                                        } else {
                                                                            phi_23860_ = vec3<f32>(0f, 0f, 0f);
                                                                            phi_23861_ = 0u;
                                                                            loop {
                                                                                let _e2660 = phi_23860_;
                                                                                let _e2662 = phi_23861_;
                                                                                local_11 = _e2660;
                                                                                let _e2663 = (_e2662 < 16u);
                                                                                if _e2663 {
                                                                                    let _e2665 = (f32(_e2662) * 0.0625f);
                                                                                    let _e2666 = (_e2662 + 1u);
                                                                                    let _e2668 = (f32(_e2666) * 0.0625f);
                                                                                    let _e2670 = ((_e2665 + _e2668) * 0.5f);
                                                                                    let _e2672 = ((_e2657 * _e2670) * _e2670);
                                                                                    let _e2676 = (_e2657 * ((_e2668 * _e2668) - (_e2665 * _e2665)));
                                                                                    let _e2681 = (vec3<f32>(_e2341, _e2342, _e2343) + vec3<f32>((_e1488 * _e2672), (_e1491 * _e2672), (_e1494 * _e2672)));
                                                                                    let _e2698 = (_e2681.x - _e1467);
                                                                                    let _e2699 = (_e2681.y - _e1470);
                                                                                    let _e2700 = (_e2681.z - _e1473);
                                                                                    let _e2708 = ((((((_e2681.x * _e2681.x) + (_e2681.y * _e2681.y)) + (_e2681.z * _e2681.z)) - (2f * (((_e2681.x * _e1467) + (_e2681.y * _e1470)) + (_e2681.z * _e1473)))) + _e1482) / (sqrt((((_e2698 * _e2698) + (_e2699 * _e2699)) + (_e2700 * _e2700))) + _e1476));
                                                                                    let _e2712 = (1f - (abs((_e2708 - 25000f)) * 0.00006666667f));
                                                                                    if (_e2712 != _e2712) {
                                                                                        phi_24231_ = true;
                                                                                    } else {
                                                                                        phi_24231_ = (0f >= _e2712);
                                                                                    }
                                                                                    let _e2716 = phi_24231_;
                                                                                    let _e2718 = -(_e2708);
                                                                                    phi_23894_ = (_e2660 + vec3<f32>((exp((_e2718 / _e1497)) * _e2676), (exp((_e2718 / _e1500)) * _e2676), (select(_e2712, 0f, _e2716) * _e2676)));
                                                                                    phi_23895_ = _e2666;
                                                                                } else {
                                                                                    phi_23894_ = vec3<f32>();
                                                                                    phi_23895_ = u32();
                                                                                }
                                                                                let _e2729 = phi_23894_;
                                                                                let _e2731 = phi_23895_;
                                                                                phi_23860_ = _e2729;
                                                                                phi_23861_ = _e2731;
                                                                                if !(_e2663) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e2734 = local_11;
                                                                            phi_23902_ = type_58(1u, _e2734);
                                                                        }
                                                                        let _e2740 = phi_23902_;
                                                                        phi_23906_ = _e2740;
                                                                    }
                                                                    let _e2745 = phi_23906_;
                                                                    if (bitcast<i32>(_e2745.member) != 0i) {
                                                                        let _e2750 = (vec3<f32>(_e2390, _e2392, _e2394) + _e2745.member_1);
                                                                        let _e2756 = (_e2399 * _e2750.y);
                                                                        let _e2768 = exp(-((((_e1506 * _e2750.x) + _e2756) + (_e1522 * _e2750.z))));
                                                                        let _e2770 = exp(-((((_e1509 * _e2750.x) + _e2756) + (_e1525 * _e2750.z))));
                                                                        let _e2772 = exp(-((((_e1512 * _e2750.x) + _e2756) + (_e1528 * _e2750.z))));
                                                                        phi_3236_ = vec3<f32>((_e2230.x + (_e2768 * _e2384)), (_e2230.y + (_e2770 * _e2384)), (_e2230.z + (_e2772 * _e2384)));
                                                                        phi_3237_ = vec3<f32>((_e2232.x + (_e2768 * _e2381)), (_e2232.y + (_e2770 * _e2381)), (_e2232.z + (_e2772 * _e2381)));
                                                                    } else {
                                                                        phi_3236_ = _e2230;
                                                                        phi_3237_ = _e2232;
                                                                    }
                                                                    let _e2794 = phi_3236_;
                                                                    let _e2796 = phi_3237_;
                                                                    phi_2740_ = vec3<f32>((_e2390 + _e2386), (_e2392 + _e2387), (_e2394 + _e2388));
                                                                    phi_2743_ = vec3<f32>((_e2228.x + ((exp(-((((_e1506 * _e2390) + _e2400) + (_e1522 * _e2394)))) * ((_e1506 * _e2381) + _e2419)) * _e2548.x)), (_e2228.y + ((exp(-((((_e1509 * _e2390) + _e2400) + (_e1525 * _e2394)))) * ((_e1509 * _e2381) + _e2419)) * _e2548.y)), (_e2228.z + ((exp(-((((_e1512 * _e2390) + _e2400) + (_e1528 * _e2394)))) * ((_e1512 * _e2381) + _e2419)) * _e2548.z)));
                                                                    phi_2745_ = _e2794;
                                                                    phi_2747_ = _e2796;
                                                                    phi_2749_ = (_e2234 + 1u);
                                                                } else {
                                                                    phi_2740_ = vec3<f32>();
                                                                    phi_2743_ = vec3<f32>();
                                                                    phi_2745_ = vec3<f32>();
                                                                    phi_2747_ = vec3<f32>();
                                                                    phi_2749_ = u32();
                                                                }
                                                                let _e2803 = phi_2740_;
                                                                let _e2805 = phi_2743_;
                                                                let _e2807 = phi_2745_;
                                                                let _e2809 = phi_2747_;
                                                                let _e2811 = phi_2749_;
                                                                phi_2739_ = _e2803;
                                                                phi_2742_ = _e2805;
                                                                phi_2744_ = _e2807;
                                                                phi_2746_ = _e2809;
                                                                phi_2748_ = _e2811;
                                                                phi_34046_ = _e1457;
                                                                if !(_e2235) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2814 = phi_34046_;
                                                            phi_34136_ = _e2814;
                                                            if _e2814 {
                                                                break;
                                                            }
                                                            let _e2816 = local_12;
                                                            let _e2819 = local_13;
                                                            let _e2822 = local_14;
                                                            phi_34140_ = _e2814;
                                                            phi_3257_ = type_52((_e2816 * vec3<f32>(_e1506, _e1509, _e1512)), (_e2819 * _e1516), _e2822);
                                                        } else {
                                                            phi_34140_ = _e1457;
                                                            phi_3257_ = type_52(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
                                                        }
                                                        let _e2825 = phi_34140_;
                                                        let _e2827 = phi_3257_;
                                                        phi_34139_ = _e2825;
                                                        phi_3258_ = _e2827;
                                                    } else {
                                                        phi_34139_ = _e1457;
                                                        phi_3258_ = _e2060;
                                                    }
                                                    let _e2829 = phi_34139_;
                                                    let _e2831 = phi_3258_;
                                                    let _e2839 = (((_e348.member_1.x * _e1488) + (_e348.member_1.y * _e1491)) + (_e348.member_1.z * _e1494));
                                                    let _e2841 = (1f + (_e2839 * _e2839));
                                                    let _e2842 = (0.059683103f * _e2841);
                                                    let _e2849 = (_e1519 * _e1519);
                                                    let _e2860 = ((0.119366206f * ((1f - _e2849) * _e2841)) / ((2f + _e2849) * pow(((1f + _e2849) - ((2f * _e1519) * _e2839)), 1.5f)));
                                                    phi_34138_ = _e2829;
                                                    phi_3281_ = vec3<f32>((_e1459.x + (((((_e2831.member.x * _e2842) + (_e2831.member_1.x * _e2860)) + _e2831.member_2.x) * _e1503) * 0.38317f)), (_e1459.y + (((((_e2831.member.y * _e2842) + (_e2831.member_1.y * _e2860)) + _e2831.member_2.y) * _e1503) * 0.38317f)), (_e1459.z + (((((_e2831.member.z * _e2842) + (_e2831.member_1.z * _e2860)) + _e2831.member_2.z) * _e1503) * 0.38317f)));
                                                    phi_3282_ = (_e1461 + 1i);
                                                    phi_3283_ = bitcast<u32>(_e1539);
                                                } else {
                                                    phi_34138_ = _e1457;
                                                    phi_3281_ = vec3<f32>();
                                                    phi_3282_ = i32();
                                                    phi_3283_ = u32();
                                                }
                                                let _e2891 = phi_34138_;
                                                let _e2893 = phi_3281_;
                                                let _e2895 = phi_3282_;
                                                let _e2897 = phi_3283_;
                                                phi_34137_ = _e2891;
                                                phi_927_ = _e2893;
                                                phi_930_ = _e2895;
                                                phi_932_ = _e2897;
                                                phi_3284_ = _e1465;
                                            } else {
                                                phi_34137_ = _e1457;
                                                phi_927_ = vec3<f32>();
                                                phi_930_ = i32();
                                                phi_932_ = u32();
                                                phi_3284_ = false;
                                            }
                                            let _e2899 = phi_34137_;
                                            let _e2901 = phi_927_;
                                            let _e2903 = phi_930_;
                                            let _e2905 = phi_932_;
                                            let _e2907 = phi_3284_;
                                            phi_34078_ = _e2899;
                                            phi_926_ = _e2901;
                                            phi_929_ = _e2903;
                                            phi_931_ = _e2905;
                                            phi_34136_ = _e2899;
                                            if !(_e2907) {
                                                break;
                                            }
                                        }
                                        let _e2910 = phi_34136_;
                                        phi_34464_ = _e2910;
                                        if _e2910 {
                                            break;
                                        }
                                        if _e1454 {
                                            let _e2913 = global_6.member[(_e1306 + 10u)];
                                            let _e2916 = global_6.member[(_e1306 + 21u)];
                                            let _e2917 = bitcast<u32>(_e2916);
                                            let _e2920 = global_6.member[(_e1306 + 22u)];
                                            let _e2921 = bitcast<u32>(_e2920);
                                            let _e2924 = global_6.member[(_e1306 + 23u)];
                                            let _e2928 = global_6.member[(_e1306 + 24u)];
                                            let _e2932 = global_6.member[(_e1306 + 25u)];
                                            let _e2936 = global_6.member[(_e1306 + 26u)];
                                            let _e2939 = global_6.member[_e1306];
                                            let _e2942 = global_6.member[(_e1306 + 1u)];
                                            let _e2945 = global_6.member[(_e1306 + 2u)];
                                            let _e2948 = global_6.member[(_e1306 + 3u)];
                                            let _e2951 = global_6.member[(_e1306 + 4u)];
                                            let _e2954 = global_6.member[(_e1306 + 5u)];
                                            let _e2957 = global_6.member[(_e1306 + 6u)];
                                            let _e2960 = global_6.member[(_e1306 + 7u)];
                                            let _e2963 = global_6.member[(_e1306 + 8u)];
                                            let _e2968 = (((_e348.member_1.x * _e2948) + (_e348.member_1.y * _e2951)) + (_e348.member_1.z * _e2954));
                                            let _e2970 = select(_e2968, -1f, (_e2968 < -1f));
                                            let _e2973 = asin(select(_e2970, 1f, (_e2970 > 1f)));
                                            let _e2984 = atan2((((_e348.member_1.x * _e2957) + (_e348.member_1.y * _e2960)) + (_e348.member_1.z * _e2963)), (((_e348.member_1.x * _e2939) + (_e348.member_1.y * _e2942)) + (_e348.member_1.z * _e2945)));
                                            if (_e2917 != 4294967295u) {
                                                let _e2987 = global_6.member[_e2917];
                                                let _e2990 = global_6.member[(_e2917 + 1u)];
                                                let _e2991 = bitcast<u32>(_e2990);
                                                let _e2994 = global_6.member[(_e2917 + 4u)];
                                                phi_3618_ = vec3<f32>(0f, 0f, 0f);
                                                phi_3621_ = 0u;
                                                loop {
                                                    let _e3000 = phi_3618_;
                                                    let _e3002 = phi_3621_;
                                                    local_15 = _e3000;
                                                    let _e3003 = (_e3002 < 6u);
                                                    if _e3003 {
                                                        let _e3004 = (_e3002 / 2u);
                                                        if (_e3004 == 0u) {
                                                            phi_24359_ = _e348.member_1.x;
                                                        } else {
                                                            phi_24359_ = select(_e348.member_1.z, _e348.member_1.y, (_e3004 == 1u));
                                                        }
                                                        let _e3012 = phi_24359_;
                                                        let _e3013 = (_e3012 * select(-1f, 1f, ((_e3002 % 2u) == 0u)));
                                                        if (_e3013 > 0f) {
                                                            let _e3017 = ((_e3004 + 1u) % 3u);
                                                            if (_e3017 == 0u) {
                                                                phi_24374_ = _e348.member_1.x;
                                                            } else {
                                                                phi_24374_ = select(_e348.member_1.z, _e348.member_1.y, (_e3017 == 1u));
                                                            }
                                                            let _e3022 = phi_24374_;
                                                            let _e3023 = (_e3022 / _e3013);
                                                            let _e3025 = ((_e3004 + 2u) % 3u);
                                                            if (_e3025 == 0u) {
                                                                phi_24389_ = _e348.member_1.x;
                                                            } else {
                                                                phi_24389_ = select(_e348.member_1.z, _e348.member_1.y, (_e3025 == 1u));
                                                            }
                                                            let _e3030 = phi_24389_;
                                                            let _e3031 = (_e3030 / _e3013);
                                                            let _e3033 = (1f + (_e2987 * 12f));
                                                            if (abs(_e3023) <= _e3033) {
                                                                if (abs(_e3031) <= _e3033) {
                                                                    let _e3039 = f32(_e2991);
                                                                    let _e3041 = floor((((_e3023 + 1f) * 0.5f) * _e3039));
                                                                    let _e3048 = select(0i, select(select(i32(_e3041), i32(-2147483648), (_e3041 < -2147483600f)), 2147483647i, (_e3041 > 2147483500f)), (_e3041 == _e3041));
                                                                    let _e3050 = (bitcast<i32>(_e2991) - 1i);
                                                                    let _e3051 = (0i <= _e3050);
                                                                    if _e3051 {
                                                                    } else {
                                                                        phi_34141_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3048 < 0i) {
                                                                        phi_3692_ = 0i;
                                                                    } else {
                                                                        phi_3692_ = select(_e3048, _e3050, (_e3048 > _e3050));
                                                                    }
                                                                    let _e3056 = phi_3692_;
                                                                    let _e3060 = floor((((_e3031 + 1f) * 0.5f) * _e3039));
                                                                    let _e3067 = select(0i, select(select(i32(_e3060), i32(-2147483648), (_e3060 < -2147483600f)), 2147483647i, (_e3060 > 2147483500f)), (_e3060 == _e3060));
                                                                    if _e3051 {
                                                                    } else {
                                                                        phi_34141_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3067 < 0i) {
                                                                        phi_3722_ = 0i;
                                                                    } else {
                                                                        phi_3722_ = select(_e3067, _e3050, (_e3067 > _e3050));
                                                                    }
                                                                    let _e3072 = phi_3722_;
                                                                    let _e3073 = (_e3056 - 1i);
                                                                    phi_3730_ = _e3000;
                                                                    phi_3733_ = select(0i, _e3073, (0i < _e3073));
                                                                    loop {
                                                                        let _e3077 = phi_3730_;
                                                                        let _e3079 = phi_3733_;
                                                                        let _e3080 = (_e3056 + 1i);
                                                                        local_59 = _e3077;
                                                                        let _e3083 = (_e3079 <= select(_e3080, _e3050, (_e3050 < _e3080)));
                                                                        if _e3083 {
                                                                            let _e3084 = (_e3072 - 1i);
                                                                            phi_3756_ = _e3077;
                                                                            phi_3759_ = select(0i, _e3084, (0i < _e3084));
                                                                            loop {
                                                                                let _e3088 = phi_3756_;
                                                                                let _e3090 = phi_3759_;
                                                                                let _e3091 = (_e3072 + 1i);
                                                                                local_58 = _e3088;
                                                                                let _e3094 = (_e3090 <= select(_e3091, _e3050, (_e3050 < _e3091)));
                                                                                if _e3094 {
                                                                                    let _e3103 = global_6.member[(bitcast<u32>(_e2994) + ((((_e3002 * _e2991) + bitcast<u32>(_e3079)) * _e2991) + bitcast<u32>(_e3090)))];
                                                                                    phi_3757_ = vec3<f32>((_e3088.x + _e3103), vec3<f32>().y, vec3<f32>().z);
                                                                                    phi_3760_ = (_e3090 + 1i);
                                                                                } else {
                                                                                    phi_3757_ = vec3<f32>();
                                                                                    phi_3760_ = i32();
                                                                                }
                                                                                let _e3112 = phi_3757_;
                                                                                let _e3114 = phi_3760_;
                                                                                phi_3756_ = _e3112;
                                                                                phi_3759_ = _e3114;
                                                                                if !(_e3094) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e6949 = local_58;
                                                                            phi_3731_ = _e6949;
                                                                            phi_3734_ = (_e3079 + 1i);
                                                                        } else {
                                                                            phi_3731_ = vec3<f32>();
                                                                            phi_3734_ = i32();
                                                                        }
                                                                        let _e3118 = phi_3731_;
                                                                        let _e3120 = phi_3734_;
                                                                        phi_3730_ = _e3118;
                                                                        phi_3733_ = _e3120;
                                                                        if !(_e3083) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6953 = local_59;
                                                                    phi_3958_ = _e6953;
                                                                } else {
                                                                    phi_3958_ = _e3000;
                                                                }
                                                                let _e3123 = phi_3958_;
                                                                phi_3959_ = _e3123;
                                                            } else {
                                                                phi_3959_ = _e3000;
                                                            }
                                                            let _e3125 = phi_3959_;
                                                            phi_3960_ = _e3125;
                                                        } else {
                                                            phi_3960_ = _e3000;
                                                        }
                                                        let _e3127 = phi_3960_;
                                                        phi_3619_ = _e3127;
                                                        phi_3622_ = (_e3002 + 1u);
                                                    } else {
                                                        phi_3619_ = vec3<f32>();
                                                        phi_3622_ = u32();
                                                    }
                                                    let _e3130 = phi_3619_;
                                                    let _e3132 = phi_3622_;
                                                    phi_3618_ = _e3130;
                                                    phi_3621_ = _e3132;
                                                    phi_34141_ = _e2910;
                                                    if !(_e3003) {
                                                        break;
                                                    }
                                                }
                                                let _e3135 = phi_34141_;
                                                phi_34464_ = _e3135;
                                                if _e3135 {
                                                    break;
                                                }
                                                let _e3137 = local_15;
                                                phi_34189_ = _e3135;
                                                phi_3971_ = (_e3137.x * (1f / ((6.2831855f * _e2987) * _e2987)));
                                            } else {
                                                phi_34189_ = _e2910;
                                                phi_3971_ = 0f;
                                            }
                                            let _e3141 = phi_34189_;
                                            let _e3143 = phi_3971_;
                                            if (_e2921 != 4294967295u) {
                                                let _e3160 = (_e2973 * 1.3333334f);
                                                let _e3167 = global_6.member[_e2921];
                                                phi_3995_ = (_e3143 + ((_e3167 * ((exp((-(abs(_e2973)) / (0.06f + (abs(_e2984) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e2984 * -0.71428573f) * (_e2984 * 0.71428573f)))))) + (1.3f * exp((((_e2984 * _e2984) + (_e3160 * _e3160)) * -12.499999f))))) * 4.73f));
                                            } else {
                                                phi_3995_ = _e3143;
                                            }
                                            let _e3173 = phi_3995_;
                                            phi_3997_ = 0u;
                                            phi_4000_ = _e3173;
                                            loop {
                                                let _e3175 = phi_3997_;
                                                let _e3177 = phi_4000_;
                                                local_60 = _e3177;
                                                let _e3178 = (_e3175 < bitcast<u32>(_e2928));
                                                if _e3178 {
                                                    let _e3180 = (bitcast<u32>(_e2924) + (_e3175 * 18u));
                                                    let _e3183 = global_6.member[(_e3180 + 1u)];
                                                    let _e3186 = global_6.member[(_e3180 + 2u)];
                                                    let _e3189 = global_6.member[(_e3180 + 3u)];
                                                    let _e3197 = global_6.member[(_e3180 + 5u)];
                                                    if ((((_e348.member_1.x * _e3183) + (_e348.member_1.y * _e3186)) + (_e348.member_1.z * _e3189)) >= _e3197) {
                                                        let _e3200 = global_6.member[_e3180];
                                                        let _e3205 = select(select(u32(_e3200), 0u, (_e3200 < 0f)), 4294967295u, (_e3200 > 4294967000f));
                                                        if (_e3205 == 0u) {
                                                            phi_4104_ = 1.8f;
                                                        } else {
                                                            if (_e3205 == 1u) {
                                                                phi_4103_ = 1f;
                                                            } else {
                                                                phi_4103_ = select(0f, 4.7f, (_e3205 == 2u));
                                                            }
                                                            let _e3211 = phi_4103_;
                                                            phi_4104_ = _e3211;
                                                        }
                                                        let _e3213 = phi_4104_;
                                                        let _e3216 = global_6.member[(_e3180 + 6u)];
                                                        phi_4120_ = (_e3177 + (_e3216 * _e3213));
                                                    } else {
                                                        phi_4120_ = _e3177;
                                                    }
                                                    let _e3220 = phi_4120_;
                                                    phi_3998_ = (_e3175 + 1u);
                                                    phi_4001_ = _e3220;
                                                } else {
                                                    phi_3998_ = u32();
                                                    phi_4001_ = f32();
                                                }
                                                let _e3223 = phi_3998_;
                                                let _e3225 = phi_4001_;
                                                phi_3997_ = _e3223;
                                                phi_4000_ = _e3225;
                                                if !(_e3178) {
                                                    break;
                                                }
                                            }
                                            phi_4123_ = 0u;
                                            let _e6973 = local_60;
                                            phi_4126_ = _e6973;
                                            loop {
                                                let _e3228 = phi_4123_;
                                                let _e3230 = phi_4126_;
                                                local_16 = _e3230;
                                                let _e3231 = (_e3228 < bitcast<u32>(_e2936));
                                                if _e3231 {
                                                    let _e3233 = (bitcast<u32>(_e2932) + (_e3228 * 29u));
                                                    let _e3236 = global_6.member[(_e3233 + 1u)];
                                                    let _e3239 = global_6.member[(_e3233 + 2u)];
                                                    let _e3242 = global_6.member[(_e3233 + 3u)];
                                                    let _e3250 = global_6.member[(_e3233 + 5u)];
                                                    if ((((_e348.member_1.x * _e3236) + (_e348.member_1.y * _e3239)) + (_e348.member_1.z * _e3242)) >= _e3250) {
                                                        let _e3253 = global_6.member[_e3233];
                                                        let _e3257 = global_6.member[(_e3233 + 6u)];
                                                        phi_4234_ = (_e3230 + (_e3257 * select(1f, 3.3808f, (_e3253 < 0.5f))));
                                                    } else {
                                                        phi_4234_ = _e3230;
                                                    }
                                                    let _e3262 = phi_4234_;
                                                    phi_4124_ = (_e3228 + 1u);
                                                    phi_4127_ = _e3262;
                                                } else {
                                                    phi_4124_ = u32();
                                                    phi_4127_ = f32();
                                                }
                                                let _e3265 = phi_4124_;
                                                let _e3267 = phi_4127_;
                                                phi_4123_ = _e3265;
                                                phi_4126_ = _e3267;
                                                if !(_e3231) {
                                                    break;
                                                }
                                            }
                                            let _e3271 = global_6.member[(_e1306 + 9u)];
                                            let _e3273 = local_16;
                                            let _e3276 = local_17;
                                            let _e3280 = local_18;
                                            let _e3285 = local_19;
                                            let _e3290 = ((_e3273 * _e3271) <= (_e2913 * (((0.2126f * _e3276.x) + (0.7152f * _e3280.y)) + (0.0722f * _e3285.z))));
                                            if _e3290 {
                                                phi_34230_ = _e3141;
                                                phi_4793_ = vec3<f32>();
                                                phi_4794_ = bool();
                                            } else {
                                                let _e3292 = global_6.member[_e1303];
                                                let _e3295 = global_6.member[(_e1303 + 1u)];
                                                let _e3298 = global_6.member[(_e1303 + 2u)];
                                                let _e3301 = global_6.member[(_e1303 + 3u)];
                                                let _e3304 = global_6.member[(_e1303 + 5u)];
                                                let _e3307 = global_6.member[(_e1303 + 6u)];
                                                let _e3310 = global_6.member[(_e1303 + 10u)];
                                                let _e3313 = global_6.member[(_e1303 + 11u)];
                                                let _e3316 = global_6.member[(_e1303 + 13u)];
                                                let _e3319 = global_6.member[(_e1303 + 14u)];
                                                let _e3322 = global_6.member[(_e1303 + 15u)];
                                                let _e3325 = global_6.member[(_e1303 + 16u)];
                                                let _e3328 = global_6.member[(_e1303 + 18u)];
                                                let _e3331 = global_6.member[(_e1303 + 19u)];
                                                let _e3334 = global_6.member[(_e1303 + 20u)];
                                                let _e3346 = ((((_e348.member.x * _e348.member.x) + (_e348.member.y * _e348.member.y)) + (_e348.member.z * _e348.member.z)) - (2f * (((_e348.member.x * _e3292) + (_e348.member.y * _e3295)) + (_e348.member.z * _e3298))));
                                                let _e3347 = (_e3346 + _e3304);
                                                let _e3348 = (_e3347 > 0f);
                                                if _e3348 {
                                                    let _e3356 = ((((_e348.member.x - _e3292) * _e348.member_1.x) + ((_e348.member.y - _e3295) * _e348.member_1.y)) + ((_e348.member.z - _e3298) * _e348.member_1.z));
                                                    let _e3358 = ((_e3356 * _e3356) - _e3347);
                                                    if (_e3358 < 0f) {
                                                        phi_24533_ = type_57(0u, type_57().member_1);
                                                        phi_24534_ = type_57();
                                                        phi_24535_ = true;
                                                    } else {
                                                        if (_e3356 < 0f) {
                                                            phi_24505_ = -(sqrt(_e3358));
                                                        } else {
                                                            phi_24505_ = sqrt(_e3358);
                                                        }
                                                        let _e3365 = phi_24505_;
                                                        let _e3367 = -((_e3356 + _e3365));
                                                        let _e3368 = (_e3367 == 0f);
                                                        if _e3368 {
                                                            phi_24528_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24529_ = type_57();
                                                        } else {
                                                            let _e3369 = (_e3347 / _e3367);
                                                            if (_e3367 < _e3369) {
                                                                phi_24517_ = type_56(_e3367, _e3369);
                                                            } else {
                                                                phi_24517_ = type_56(_e3369, _e3367);
                                                            }
                                                            let _e3374 = phi_24517_;
                                                            phi_24528_ = type_57();
                                                            phi_24529_ = type_57(1u, _e3374);
                                                        }
                                                        let _e3377 = phi_24528_;
                                                        let _e3379 = phi_24529_;
                                                        phi_24533_ = _e3377;
                                                        phi_24534_ = _e3379;
                                                        phi_24535_ = _e3368;
                                                    }
                                                    let _e3384 = phi_24533_;
                                                    let _e3386 = phi_24534_;
                                                    let _e3388 = phi_24535_;
                                                    if _e3388 {
                                                        phi_24539_ = _e3384;
                                                    } else {
                                                        phi_24539_ = _e3386;
                                                    }
                                                    let _e3390 = phi_24539_;
                                                    if (bitcast<i32>(_e3390.member) != 0i) {
                                                        phi_24473_ = select(_e3390.member_1.member_1, _e3390.member_1.member, (_e3390.member_1.member >= 0f));
                                                    } else {
                                                        phi_24473_ = -1f;
                                                    }
                                                    let _e3401 = phi_24473_;
                                                    phi_4635_ = select(true, false, (_e3401 >= 0f));
                                                } else {
                                                    phi_4635_ = true;
                                                }
                                                let _e3405 = phi_4635_;
                                                if _e3405 {
                                                    let _e3413 = ((((_e348.member.x - _e3292) * _e348.member_1.x) + ((_e348.member.y - _e3295) * _e348.member_1.y)) + ((_e348.member.z - _e3298) * _e348.member_1.z));
                                                    let _e3414 = (_e3346 + _e3307);
                                                    let _e3415 = (_e3413 * _e3413);
                                                    let _e3416 = (_e3415 - _e3414);
                                                    if (_e3416 < 0f) {
                                                        phi_24693_ = type_57(0u, type_57().member_1);
                                                        phi_24694_ = type_57();
                                                        phi_24695_ = true;
                                                    } else {
                                                        if (_e3413 < 0f) {
                                                            phi_24665_ = -(sqrt(_e3416));
                                                        } else {
                                                            phi_24665_ = sqrt(_e3416);
                                                        }
                                                        let _e3423 = phi_24665_;
                                                        let _e3425 = -((_e3413 + _e3423));
                                                        let _e3426 = (_e3425 == 0f);
                                                        if _e3426 {
                                                            phi_24688_ = type_57(1u, type_56(0f, 0f));
                                                            phi_24689_ = type_57();
                                                        } else {
                                                            let _e3427 = (_e3414 / _e3425);
                                                            if (_e3425 < _e3427) {
                                                                phi_24677_ = type_56(_e3425, _e3427);
                                                            } else {
                                                                phi_24677_ = type_56(_e3427, _e3425);
                                                            }
                                                            let _e3432 = phi_24677_;
                                                            phi_24688_ = type_57();
                                                            phi_24689_ = type_57(1u, _e3432);
                                                        }
                                                        let _e3435 = phi_24688_;
                                                        let _e3437 = phi_24689_;
                                                        phi_24693_ = _e3435;
                                                        phi_24694_ = _e3437;
                                                        phi_24695_ = _e3426;
                                                    }
                                                    let _e3442 = phi_24693_;
                                                    let _e3444 = phi_24694_;
                                                    let _e3446 = phi_24695_;
                                                    if _e3446 {
                                                        phi_24699_ = _e3442;
                                                    } else {
                                                        phi_24699_ = _e3444;
                                                    }
                                                    let _e3448 = phi_24699_;
                                                    if (bitcast<i32>(_e3448.member) != 0i) {
                                                        phi_24586_ = type_57(0u, _e3448.member_1);
                                                    } else {
                                                        phi_24586_ = type_57(1u, type_57().member_1);
                                                    }
                                                    let _e3458 = phi_24586_;
                                                    if (bitcast<i32>(_e3458.member) != 0i) {
                                                        phi_24633_ = type_57(0u, type_57().member_1);
                                                    } else {
                                                        if (_e3458.member_1.member_1 <= 0f) {
                                                            phi_24629_ = type_57(0u, type_57().member_1);
                                                        } else {
                                                            if (_e3458.member_1.member != _e3458.member_1.member) {
                                                                phi_24737_ = true;
                                                            } else {
                                                                phi_24737_ = (0f >= _e3458.member_1.member);
                                                            }
                                                            let _e3470 = phi_24737_;
                                                            let _e3471 = select(_e3458.member_1.member, 0f, _e3470);
                                                            if _e3348 {
                                                                let _e3472 = (_e3415 - _e3347);
                                                                if (_e3472 < 0f) {
                                                                    phi_24845_ = type_57(0u, type_57().member_1);
                                                                    phi_24846_ = type_57();
                                                                    phi_24847_ = true;
                                                                } else {
                                                                    if (_e3413 < 0f) {
                                                                        phi_24817_ = -(sqrt(_e3472));
                                                                    } else {
                                                                        phi_24817_ = sqrt(_e3472);
                                                                    }
                                                                    let _e3479 = phi_24817_;
                                                                    let _e3481 = -((_e3413 + _e3479));
                                                                    let _e3482 = (_e3481 == 0f);
                                                                    if _e3482 {
                                                                        phi_24840_ = type_57(1u, type_56(0f, 0f));
                                                                        phi_24841_ = type_57();
                                                                    } else {
                                                                        let _e3483 = (_e3347 / _e3481);
                                                                        if (_e3481 < _e3483) {
                                                                            phi_24829_ = type_56(_e3481, _e3483);
                                                                        } else {
                                                                            phi_24829_ = type_56(_e3483, _e3481);
                                                                        }
                                                                        let _e3488 = phi_24829_;
                                                                        phi_24840_ = type_57();
                                                                        phi_24841_ = type_57(1u, _e3488);
                                                                    }
                                                                    let _e3491 = phi_24840_;
                                                                    let _e3493 = phi_24841_;
                                                                    phi_24845_ = _e3491;
                                                                    phi_24846_ = _e3493;
                                                                    phi_24847_ = _e3482;
                                                                }
                                                                let _e3498 = phi_24845_;
                                                                let _e3500 = phi_24846_;
                                                                let _e3502 = phi_24847_;
                                                                if _e3502 {
                                                                    phi_24851_ = _e3498;
                                                                } else {
                                                                    phi_24851_ = _e3500;
                                                                }
                                                                let _e3504 = phi_24851_;
                                                                if (bitcast<i32>(_e3504.member) != 0i) {
                                                                    phi_24785_ = select(_e3504.member_1.member_1, _e3504.member_1.member, (_e3504.member_1.member >= 0f));
                                                                } else {
                                                                    phi_24785_ = -1f;
                                                                }
                                                                let _e3515 = phi_24785_;
                                                                if (_e3515 >= 0f) {
                                                                    if (_e3458.member_1.member_1 != _e3458.member_1.member_1) {
                                                                        phi_24889_ = true;
                                                                    } else {
                                                                        phi_24889_ = (_e3515 <= _e3458.member_1.member_1);
                                                                    }
                                                                    let _e3520 = phi_24889_;
                                                                    phi_24614_ = select(_e3458.member_1.member_1, _e3515, _e3520);
                                                                } else {
                                                                    phi_24614_ = _e3458.member_1.member_1;
                                                                }
                                                                let _e3523 = phi_24614_;
                                                                phi_24616_ = _e3523;
                                                            } else {
                                                                phi_24616_ = _e3458.member_1.member_1;
                                                            }
                                                            let _e3525 = phi_24616_;
                                                            if (_e3525 <= _e3471) {
                                                                phi_24625_ = type_57(0u, type_57().member_1);
                                                            } else {
                                                                phi_24625_ = type_57(1u, type_56(_e3471, _e3525));
                                                            }
                                                            let _e3533 = phi_24625_;
                                                            phi_24629_ = _e3533;
                                                        }
                                                        let _e3538 = phi_24629_;
                                                        phi_24633_ = _e3538;
                                                    }
                                                    let _e3543 = phi_24633_;
                                                    let _e3546 = (bitcast<i32>(_e3543.member) != 0i);
                                                    if _e3546 {
                                                        phi_4649_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_4652_ = 0u;
                                                        loop {
                                                            let _e3552 = phi_4649_;
                                                            let _e3554 = phi_4652_;
                                                            local_20 = _e3552;
                                                            local_21 = _e3552;
                                                            local_22 = _e3552;
                                                            let _e3555 = (_e3554 < 32u);
                                                            if _e3555 {
                                                                let _e3556 = -(_e3413);
                                                                if (_e3543.member_1.member <= _e3543.member_1.member_1) {
                                                                } else {
                                                                    phi_34168_ = true;
                                                                    break;
                                                                }
                                                                let _e3559 = select(_e3556, _e3543.member_1.member, (_e3556 < _e3543.member_1.member));
                                                                let _e3561 = select(_e3559, _e3543.member_1.member_1, (_e3559 > _e3543.member_1.member_1));
                                                                let _e3562 = (_e3561 - _e3543.member_1.member);
                                                                let _e3563 = (_e3543.member_1.member_1 - _e3561);
                                                                let _e3564 = (_e3543.member_1.member_1 - _e3543.member_1.member);
                                                                let _e3565 = (0.000000001f * _e3564);
                                                                if (_e3562 <= _e3565) {
                                                                    let _e3640 = (f32(_e3554) * 0.03125f);
                                                                    let _e3643 = (f32((_e3554 + 1u)) * 0.03125f);
                                                                    let _e3645 = ((_e3640 + _e3643) * 0.5f);
                                                                    phi_4753_ = type_56((_e3543.member_1.member + ((_e3564 * _e3645) * _e3645)), (_e3564 * ((_e3643 * _e3643) - (_e3640 * _e3640))));
                                                                } else {
                                                                    if (_e3563 <= _e3565) {
                                                                        let _e3623 = (f32((31u - _e3554)) * 0.03125f);
                                                                        let _e3626 = (f32((32u - _e3554)) * 0.03125f);
                                                                        let _e3628 = ((_e3623 + _e3626) * 0.5f);
                                                                        phi_4752_ = type_56((_e3543.member_1.member_1 - ((_e3564 * _e3628) * _e3628)), (_e3564 * ((_e3626 * _e3626) - (_e3623 * _e3623))));
                                                                    } else {
                                                                        let _e3571 = round(((32f * _e3562) / (_e3562 + _e3563)));
                                                                        let _e3576 = select(select(u32(_e3571), 0u, (_e3571 < 0f)), 4294967295u, (_e3571 > 4294967000f));
                                                                        if (_e3576 < 1u) {
                                                                            phi_4736_ = 1u;
                                                                        } else {
                                                                            phi_4736_ = select(_e3576, 31u, (_e3576 > 31u));
                                                                        }
                                                                        let _e3581 = phi_4736_;
                                                                        if (_e3554 < _e3581) {
                                                                            let _e3602 = ((_e3581 - 1u) - _e3554);
                                                                            let _e3604 = f32(_e3581);
                                                                            let _e3605 = (f32(_e3602) / _e3604);
                                                                            let _e3608 = (f32((_e3602 + 1u)) / _e3604);
                                                                            let _e3610 = ((_e3605 + _e3608) * 0.5f);
                                                                            phi_4751_ = type_56((_e3561 - ((_e3562 * _e3610) * _e3610)), (_e3562 * ((_e3608 * _e3608) - (_e3605 * _e3605))));
                                                                        } else {
                                                                            let _e3584 = (_e3554 - _e3581);
                                                                            let _e3586 = f32((32u - _e3581));
                                                                            let _e3587 = (f32(_e3584) / _e3586);
                                                                            let _e3590 = (f32((_e3584 + 1u)) / _e3586);
                                                                            let _e3592 = ((_e3587 + _e3590) * 0.5f);
                                                                            phi_4751_ = type_56((_e3561 + ((_e3563 * _e3592) * _e3592)), (_e3563 * ((_e3590 * _e3590) - (_e3587 * _e3587))));
                                                                        }
                                                                        let _e3620 = phi_4751_;
                                                                        phi_4752_ = _e3620;
                                                                    }
                                                                    let _e3638 = phi_4752_;
                                                                    phi_4753_ = _e3638;
                                                                }
                                                                let _e3655 = phi_4753_;
                                                                let _e3661 = (_e348.member.x + (_e348.member_1.x * _e3655.member));
                                                                let _e3662 = (_e348.member.y + (_e348.member_1.y * _e3655.member));
                                                                let _e3663 = (_e348.member.z + (_e348.member_1.z * _e3655.member));
                                                                let _e3677 = (_e3661 - _e3292);
                                                                let _e3678 = (_e3662 - _e3295);
                                                                let _e3679 = (_e3663 - _e3298);
                                                                let _e3687 = ((((((_e3661 * _e3661) + (_e3662 * _e3662)) + (_e3663 * _e3663)) - (2f * (((_e3661 * _e3292) + (_e3662 * _e3295)) + (_e3663 * _e3298)))) + _e3304) / (sqrt((((_e3677 * _e3677) + (_e3678 * _e3678)) + (_e3679 * _e3679))) + _e3301));
                                                                let _e3691 = (1f - (abs((_e3687 - 25000f)) * 0.00006666667f));
                                                                if (_e3691 != _e3691) {
                                                                    phi_25088_ = true;
                                                                } else {
                                                                    phi_25088_ = (0f >= _e3691);
                                                                }
                                                                let _e3695 = phi_25088_;
                                                                let _e3697 = -(_e3687);
                                                                phi_4650_ = (_e3552 + vec3<f32>((exp((_e3697 / _e3310)) * _e3655.member_1), (exp((_e3697 / _e3313)) * _e3655.member_1), (select(_e3691, 0f, _e3695) * _e3655.member_1)));
                                                                phi_4653_ = (_e3554 + 1u);
                                                            } else {
                                                                phi_4650_ = vec3<f32>();
                                                                phi_4653_ = u32();
                                                            }
                                                            let _e3709 = phi_4650_;
                                                            let _e3711 = phi_4653_;
                                                            phi_4649_ = _e3709;
                                                            phi_4652_ = _e3711;
                                                            phi_34168_ = _e3141;
                                                            if !(_e3555) {
                                                                break;
                                                            }
                                                        }
                                                        let _e3714 = phi_34168_;
                                                        phi_34464_ = _e3714;
                                                        if _e3714 {
                                                            break;
                                                        }
                                                        let _e3716 = local_20;
                                                        let _e3723 = local_21;
                                                        let _e3725 = ((_e3325 * 1.1f) * _e3723.y);
                                                        let _e3730 = local_22;
                                                        phi_34235_ = _e3714;
                                                        phi_4778_ = vec3<f32>(exp(-((((_e3316 * _e3716.x) + _e3725) + (_e3328 * _e3730.z)))), exp(-((((_e3319 * _e3716.x) + _e3725) + (_e3331 * _e3730.z)))), exp(-((((_e3322 * _e3716.x) + _e3725) + (_e3334 * _e3730.z)))));
                                                    } else {
                                                        phi_34235_ = _e3141;
                                                        phi_4778_ = vec3<f32>();
                                                    }
                                                    let _e3746 = phi_34235_;
                                                    let _e3748 = phi_4778_;
                                                    phi_34234_ = _e3746;
                                                    phi_4780_ = _e3748;
                                                    phi_4782_ = select(true, false, _e3546);
                                                } else {
                                                    phi_34234_ = _e3141;
                                                    phi_4780_ = vec3<f32>();
                                                    phi_4782_ = true;
                                                }
                                                let _e3751 = phi_34234_;
                                                let _e3753 = phi_4780_;
                                                let _e3755 = phi_4782_;
                                                let _e3759 = select(_e3753, select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e3405)), vec3(_e3755));
                                                let _e3763 = select(_e3759.y, _e3759.x, (_e3759.x > _e3759.y));
                                                phi_34230_ = _e3751;
                                                phi_4793_ = _e3759;
                                                phi_4794_ = (select(_e3759.z, _e3763, (_e3763 > _e3759.z)) > 0f);
                                            }
                                            let _e3769 = phi_34230_;
                                            let _e3771 = phi_4793_;
                                            let _e3773 = phi_4794_;
                                            phi_34229_ = _e3769;
                                            phi_4796_ = _e3771;
                                            phi_4797_ = _e3773;
                                            phi_4798_ = select(true, false, _e3290);
                                        } else {
                                            phi_34229_ = _e2910;
                                            phi_4796_ = vec3<f32>(1f, 1f, 1f);
                                            phi_4797_ = _e1454;
                                            phi_4798_ = true;
                                        }
                                        let _e3776 = phi_34229_;
                                        let _e3778 = phi_4796_;
                                        let _e3780 = phi_4797_;
                                        let _e3782 = phi_4798_;
                                        phi_34228_ = _e3776;
                                        let _e7044 = local_61;
                                        phi_4802_ = _e7044;
                                        phi_4803_ = _e3778;
                                        phi_4804_ = _e3780;
                                        let _e7049 = local_62;
                                        phi_4805_ = _e7049;
                                        phi_4806_ = _e3782;
                                    } else {
                                        phi_34228_ = _e337;
                                        phi_4802_ = vec3<f32>(0f, 0f, 0f);
                                        phi_4803_ = vec3<f32>(1f, 1f, 1f);
                                        phi_4804_ = _e1454;
                                        phi_4805_ = vec3<f32>();
                                        phi_4806_ = true;
                                    }
                                    let _e3784 = phi_34228_;
                                    let _e3786 = phi_4802_;
                                    let _e3788 = phi_4803_;
                                    let _e3790 = phi_4804_;
                                    let _e3792 = phi_4805_;
                                    let _e3794 = phi_4806_;
                                    if _e3794 {
                                        if _e3790 {
                                            let _e3795 = (_e1306 + 20u);
                                            let _e3798 = global_6.member[(_e1306 + 21u)];
                                            let _e3799 = bitcast<u32>(_e3798);
                                            let _e3802 = global_6.member[(_e1306 + 22u)];
                                            let _e3803 = bitcast<u32>(_e3802);
                                            let _e3806 = global_6.member[(_e1306 + 23u)];
                                            let _e3810 = global_6.member[(_e1306 + 24u)];
                                            let _e3814 = global_6.member[(_e1306 + 25u)];
                                            let _e3818 = global_6.member[(_e1306 + 26u)];
                                            let _e3821 = global_6.member[_e1306];
                                            let _e3824 = global_6.member[(_e1306 + 1u)];
                                            let _e3827 = global_6.member[(_e1306 + 2u)];
                                            let _e3830 = global_6.member[(_e1306 + 3u)];
                                            let _e3833 = global_6.member[(_e1306 + 4u)];
                                            let _e3836 = global_6.member[(_e1306 + 5u)];
                                            let _e3839 = global_6.member[(_e1306 + 6u)];
                                            let _e3842 = global_6.member[(_e1306 + 7u)];
                                            let _e3845 = global_6.member[(_e1306 + 8u)];
                                            let _e3850 = (((_e348.member_1.x * _e3830) + (_e348.member_1.y * _e3833)) + (_e348.member_1.z * _e3836));
                                            let _e3852 = select(_e3850, -1f, (_e3850 < -1f));
                                            let _e3855 = asin(select(_e3852, 1f, (_e3852 > 1f)));
                                            let _e3866 = atan2((((_e348.member_1.x * _e3839) + (_e348.member_1.y * _e3842)) + (_e348.member_1.z * _e3845)), (((_e348.member_1.x * _e3821) + (_e348.member_1.y * _e3824)) + (_e348.member_1.z * _e3827)));
                                            if (_e3799 != 4294967295u) {
                                                let _e3869 = global_6.member[_e3799];
                                                let _e3872 = global_6.member[(_e3799 + 1u)];
                                                let _e3873 = bitcast<u32>(_e3872);
                                                let _e3876 = global_6.member[(_e3799 + 2u)];
                                                let _e3880 = global_6.member[(_e3799 + 3u)];
                                                let _e3882 = (4f * _e3869);
                                                phi_5126_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5129_ = 0u;
                                                loop {
                                                    let _e3887 = phi_5126_;
                                                    let _e3889 = phi_5129_;
                                                    local_66 = _e3887;
                                                    let _e3890 = (_e3889 < 6u);
                                                    if _e3890 {
                                                        let _e3891 = (_e3889 / 2u);
                                                        if (_e3891 == 0u) {
                                                            phi_25159_ = _e348.member_1.x;
                                                        } else {
                                                            phi_25159_ = select(_e348.member_1.z, _e348.member_1.y, (_e3891 == 1u));
                                                        }
                                                        let _e3899 = phi_25159_;
                                                        let _e3900 = (_e3899 * select(-1f, 1f, ((_e3889 % 2u) == 0u)));
                                                        if (_e3900 > 0f) {
                                                            let _e3904 = ((_e3891 + 1u) % 3u);
                                                            if (_e3904 == 0u) {
                                                                phi_25174_ = _e348.member_1.x;
                                                            } else {
                                                                phi_25174_ = select(_e348.member_1.z, _e348.member_1.y, (_e3904 == 1u));
                                                            }
                                                            let _e3909 = phi_25174_;
                                                            let _e3910 = (_e3909 / _e3900);
                                                            let _e3912 = ((_e3891 + 2u) % 3u);
                                                            if (_e3912 == 0u) {
                                                                phi_25189_ = _e348.member_1.x;
                                                            } else {
                                                                phi_25189_ = select(_e348.member_1.z, _e348.member_1.y, (_e3912 == 1u));
                                                            }
                                                            let _e3917 = phi_25189_;
                                                            let _e3918 = (_e3917 / _e3900);
                                                            let _e3920 = (1f + (_e3869 * 12f));
                                                            if (abs(_e3910) <= _e3920) {
                                                                if (abs(_e3918) <= _e3920) {
                                                                    let _e3926 = f32(_e3873);
                                                                    let _e3928 = floor((((_e3910 + 1f) * 0.5f) * _e3926));
                                                                    let _e3935 = select(0i, select(select(i32(_e3928), i32(-2147483648), (_e3928 < -2147483600f)), 2147483647i, (_e3928 > 2147483500f)), (_e3928 == _e3928));
                                                                    let _e3937 = (bitcast<i32>(_e3873) - 1i);
                                                                    let _e3938 = (0i <= _e3937);
                                                                    if _e3938 {
                                                                    } else {
                                                                        phi_34223_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3935 < 0i) {
                                                                        phi_5200_ = 0i;
                                                                    } else {
                                                                        phi_5200_ = select(_e3935, _e3937, (_e3935 > _e3937));
                                                                    }
                                                                    let _e3943 = phi_5200_;
                                                                    let _e3947 = floor((((_e3918 + 1f) * 0.5f) * _e3926));
                                                                    let _e3954 = select(0i, select(select(i32(_e3947), i32(-2147483648), (_e3947 < -2147483600f)), 2147483647i, (_e3947 > 2147483500f)), (_e3947 == _e3947));
                                                                    if _e3938 {
                                                                    } else {
                                                                        phi_34223_ = true;
                                                                        break;
                                                                    }
                                                                    if (_e3954 < 0i) {
                                                                        phi_5230_ = 0i;
                                                                    } else {
                                                                        phi_5230_ = select(_e3954, _e3937, (_e3954 > _e3937));
                                                                    }
                                                                    let _e3959 = phi_5230_;
                                                                    let _e3960 = (_e3943 - 1i);
                                                                    phi_5238_ = _e3887;
                                                                    phi_5241_ = select(0i, _e3960, (0i < _e3960));
                                                                    loop {
                                                                        let _e3964 = phi_5238_;
                                                                        let _e3966 = phi_5241_;
                                                                        let _e3967 = (_e3943 + 1i);
                                                                        local_65 = _e3964;
                                                                        let _e3970 = (_e3966 <= select(_e3967, _e3937, (_e3937 < _e3967)));
                                                                        if _e3970 {
                                                                            let _e3971 = (_e3959 - 1i);
                                                                            phi_5264_ = _e3964;
                                                                            phi_5267_ = select(0i, _e3971, (0i < _e3971));
                                                                            loop {
                                                                                let _e3975 = phi_5264_;
                                                                                let _e3977 = phi_5267_;
                                                                                let _e3978 = (_e3959 + 1i);
                                                                                local_64 = _e3975;
                                                                                let _e3981 = (_e3977 <= select(_e3978, _e3937, (_e3937 < _e3978)));
                                                                                if _e3981 {
                                                                                    let _e3988 = (bitcast<u32>(_e3876) + ((((_e3889 * _e3873) + bitcast<u32>(_e3966)) * _e3873) + bitcast<u32>(_e3977)));
                                                                                    let _e3990 = global_6.member[_e3988];
                                                                                    let _e3994 = global_6.member[(_e3988 + 1u)];
                                                                                    phi_5338_ = _e3975;
                                                                                    phi_5341_ = bitcast<u32>(_e3990);
                                                                                    loop {
                                                                                        let _e3997 = phi_5338_;
                                                                                        let _e3999 = phi_5341_;
                                                                                        local_63 = _e3997;
                                                                                        let _e4000 = (_e3999 < bitcast<u32>(_e3994));
                                                                                        if _e4000 {
                                                                                            let _e4002 = (bitcast<u32>(_e3880) + (6u * _e3999));
                                                                                            let _e4004 = global_6.member[_e4002];
                                                                                            let _e4007 = global_6.member[(_e4002 + 1u)];
                                                                                            let _e4010 = global_6.member[(_e4002 + 2u)];
                                                                                            let _e4013 = global_6.member[(_e4002 + 3u)];
                                                                                            let _e4016 = global_6.member[(_e4002 + 4u)];
                                                                                            let _e4019 = global_6.member[(_e4002 + 5u)];
                                                                                            let _e4020 = (_e348.member_1.x - _e4004);
                                                                                            let _e4021 = (_e348.member_1.y - _e4007);
                                                                                            let _e4022 = (_e348.member_1.z - _e4010);
                                                                                            let _e4027 = (((_e4020 * _e4020) + (_e4021 * _e4021)) + (_e4022 * _e4022));
                                                                                            if (_e4027 < (_e3882 * _e3882)) {
                                                                                                let _e4035 = ((1f / ((6.2831855f * _e3869) * _e3869)) * exp((-(_e4027) / ((2f * _e3869) * _e3869))));
                                                                                                phi_5461_ = vec3<f32>((_e3997.x + (_e4013 * _e4035)), (_e3997.y + (_e4016 * _e4035)), (_e3997.z + (_e4019 * _e4035)));
                                                                                            } else {
                                                                                                phi_5461_ = _e3997;
                                                                                            }
                                                                                            let _e4047 = phi_5461_;
                                                                                            phi_5339_ = _e4047;
                                                                                            phi_5342_ = (_e3999 + 1u);
                                                                                        } else {
                                                                                            phi_5339_ = vec3<f32>();
                                                                                            phi_5342_ = u32();
                                                                                        }
                                                                                        let _e4050 = phi_5339_;
                                                                                        let _e4052 = phi_5342_;
                                                                                        phi_5338_ = _e4050;
                                                                                        phi_5341_ = _e4052;
                                                                                        if !(_e4000) {
                                                                                            break;
                                                                                        }
                                                                                    }
                                                                                    let _e7069 = local_63;
                                                                                    phi_5265_ = _e7069;
                                                                                    phi_5268_ = (_e3977 + 1i);
                                                                                } else {
                                                                                    phi_5265_ = vec3<f32>();
                                                                                    phi_5268_ = i32();
                                                                                }
                                                                                let _e4056 = phi_5265_;
                                                                                let _e4058 = phi_5268_;
                                                                                phi_5264_ = _e4056;
                                                                                phi_5267_ = _e4058;
                                                                                if !(_e3981) {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            let _e7073 = local_64;
                                                                            phi_5239_ = _e7073;
                                                                            phi_5242_ = (_e3966 + 1i);
                                                                        } else {
                                                                            phi_5239_ = vec3<f32>();
                                                                            phi_5242_ = i32();
                                                                        }
                                                                        let _e4062 = phi_5239_;
                                                                        let _e4064 = phi_5242_;
                                                                        phi_5238_ = _e4062;
                                                                        phi_5241_ = _e4064;
                                                                        if !(_e3970) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e7077 = local_65;
                                                                    phi_5466_ = _e7077;
                                                                } else {
                                                                    phi_5466_ = _e3887;
                                                                }
                                                                let _e4067 = phi_5466_;
                                                                phi_5467_ = _e4067;
                                                            } else {
                                                                phi_5467_ = _e3887;
                                                            }
                                                            let _e4069 = phi_5467_;
                                                            phi_5468_ = _e4069;
                                                        } else {
                                                            phi_5468_ = _e3887;
                                                        }
                                                        let _e4071 = phi_5468_;
                                                        phi_5127_ = _e4071;
                                                        phi_5130_ = (_e3889 + 1u);
                                                    } else {
                                                        phi_5127_ = vec3<f32>();
                                                        phi_5130_ = u32();
                                                    }
                                                    let _e4074 = phi_5127_;
                                                    let _e4076 = phi_5130_;
                                                    phi_5126_ = _e4074;
                                                    phi_5129_ = _e4076;
                                                    phi_34223_ = _e3784;
                                                    if !(_e3890) {
                                                        break;
                                                    }
                                                }
                                                let _e4079 = phi_34223_;
                                                phi_34464_ = _e4079;
                                                if _e4079 {
                                                    break;
                                                }
                                                phi_34265_ = _e4079;
                                                let _e7086 = local_66;
                                                phi_5488_ = _e7086;
                                            } else {
                                                phi_34265_ = _e3784;
                                                phi_5488_ = vec3<f32>(0f, 0f, 0f);
                                            }
                                            let _e4081 = phi_34265_;
                                            let _e4083 = phi_5488_;
                                            if (_e3803 != 4294967295u) {
                                                let _e4086 = global_6.member[_e3803];
                                                let _e4089 = global_6.member[(_e3803 + 1u)];
                                                let _e4092 = global_6.member[(_e3803 + 2u)];
                                                let _e4095 = global_6.member[(_e3803 + 3u)];
                                                let _e4098 = global_6.member[(_e3803 + 4u)];
                                                let _e4101 = global_6.member[(_e3803 + 5u)];
                                                let _e4104 = global_6.member[(_e3803 + 6u)];
                                                let _e4107 = global_6.member[(_e3803 + 7u)];
                                                let _e4110 = global_6.member[(_e3803 + 8u)];
                                                let _e4113 = global_6.member[(_e3803 + 9u)];
                                                let _e4116 = global_6.member[(_e3803 + 10u)];
                                                let _e4120 = global_6.member[(_e3803 + 11u)];
                                                let _e4123 = global_6.member[(_e3803 + 12u)];
                                                let _e4126 = global_6.member[(_e3803 + 13u)];
                                                let _e4128 = (_e4086 > 0f);
                                                let _e4129 = (_e4089 > 0f);
                                                if _e4129 {
                                                    let _e4132 = (abs(_e3855) <= 0.8f);
                                                    let _e4133 = select(2u, 0u, _e4128);
                                                    phi_5701_ = _e4133;
                                                    phi_5702_ = _e4132;
                                                    phi_5705_ = _e4133;
                                                    phi_5706_ = _e4132;
                                                    phi_5707_ = select(true, false, _e4132);
                                                } else {
                                                    phi_5701_ = u32();
                                                    phi_5702_ = bool();
                                                    phi_5705_ = select(2u, 0u, _e4128);
                                                    phi_5706_ = false;
                                                    phi_5707_ = true;
                                                }
                                                let _e4136 = phi_5701_;
                                                let _e4138 = phi_5702_;
                                                let _e4140 = phi_5705_;
                                                let _e4142 = phi_5706_;
                                                let _e4144 = phi_5707_;
                                                let _e4146 = select(select(f32(), 0f, _e4129), 0f, _e4144);
                                                phi_5717_ = _e4146;
                                                phi_5720_ = _e4146;
                                                phi_5722_ = select(_e4136, _e4140, _e4144);
                                                phi_5724_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5726_ = vec3<f32>(0f, 0f, 0f);
                                                phi_5728_ = 0f;
                                                phi_5730_ = 1f;
                                                loop {
                                                    let _e4151 = phi_5717_;
                                                    let _e4153 = phi_5720_;
                                                    let _e4155 = phi_5722_;
                                                    let _e4157 = phi_5724_;
                                                    let _e4159 = phi_5726_;
                                                    let _e4161 = phi_5728_;
                                                    let _e4163 = phi_5730_;
                                                    local_26 = _e4163;
                                                    local_27 = _e4163;
                                                    local_28 = _e4163;
                                                    local_29 = _e4163;
                                                    local_30 = _e4161;
                                                    local_31 = _e4153;
                                                    local_32 = _e4151;
                                                    let _e4164 = (_e4155 < select(11u, 2u, _e4144));
                                                    if _e4164 {
                                                        let _e4165 = (_e4155 < 2u);
                                                        if _e4165 {
                                                            let _e4221 = select(23f, 4f, (_e4155 == 0u));
                                                            phi_5808_ = 4u;
                                                            phi_5810_ = vec3<f32>((_e348.member_1.x * _e4221), (_e348.member_1.y * _e4221), (_e348.member_1.z * _e4221));
                                                        } else {
                                                            if (_e4155 == 2u) {
                                                                phi_5806_ = 3u;
                                                                phi_5807_ = vec3<f32>(((_e348.member_1.x * 1.5f) + 5f), ((_e348.member_1.y * 1.5f) + 5f), ((_e348.member_1.z * 1.5f) + 5f));
                                                            } else {
                                                                let _e4167 = (_e4155 >= 7u);
                                                                if _e4167 {
                                                                    phi_5768_ = (_e4155 - 7u);
                                                                } else {
                                                                    phi_5768_ = (_e4155 - 3u);
                                                                }
                                                                let _e4171 = phi_5768_;
                                                                if _e4167 {
                                                                    phi_5783_ = vec3<f32>(((_e348.member_1.x * 11f) + -3f), ((_e348.member_1.y * 11f) + 1f), ((_e348.member_1.z * 11f) + 7f));
                                                                } else {
                                                                    phi_5783_ = vec3<f32>((_e348.member_1.x * 7f), (_e348.member_1.y * 7f), (_e348.member_1.z * 7f));
                                                                }
                                                                let _e4184 = phi_5783_;
                                                                if (_e4171 < 3u) {
                                                                    if (_e4171 == 0u) {
                                                                        phi_25204_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                    } else {
                                                                        phi_25204_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e4171 == 1u)));
                                                                    }
                                                                    let _e4203 = phi_25204_;
                                                                    phi_5804_ = 4u;
                                                                    phi_5805_ = (_e4184 + _e4203);
                                                                } else {
                                                                    if _e4167 {
                                                                        phi_5803_ = (_e4184 + _e4157);
                                                                    } else {
                                                                        phi_5803_ = (_e4184 + vec3<f32>((_e4159.x * 1.1f), (_e4159.y * 1.1f), (_e4159.z * 1.1f)));
                                                                    }
                                                                    let _e4196 = phi_5803_;
                                                                    phi_5804_ = select(7u, 5u, _e4167);
                                                                    phi_5805_ = _e4196;
                                                                }
                                                                let _e4206 = phi_5804_;
                                                                let _e4208 = phi_5805_;
                                                                phi_5806_ = _e4206;
                                                                phi_5807_ = _e4208;
                                                            }
                                                            let _e4217 = phi_5806_;
                                                            let _e4219 = phi_5807_;
                                                            phi_5808_ = _e4217;
                                                            phi_5810_ = _e4219;
                                                        }
                                                        let _e4227 = phi_5808_;
                                                        let _e4229 = phi_5810_;
                                                        let _e4231 = select(vec3<f32>(_e4120, _e4123, _e4126), vec3<f32>(_e4110, _e4113, _e4116), vec3(_e4165));
                                                        let _e4233 = global_6.member[_e3795];
                                                        let _e4234 = bitcast<u32>(_e4233);
                                                        let _e4237 = global_6.member[(_e1306 + 11u)];
                                                        let _e4240 = global_6.member[(_e1306 + 12u)];
                                                        let _e4243 = global_6.member[(_e1306 + 13u)];
                                                        let _e4246 = global_6.member[(_e1306 + 14u)];
                                                        let _e4249 = global_6.member[(_e1306 + 15u)];
                                                        let _e4252 = global_6.member[(_e1306 + 16u)];
                                                        let _e4255 = global_6.member[(_e1306 + 17u)];
                                                        let _e4258 = global_6.member[(_e1306 + 18u)];
                                                        let _e4261 = global_6.member[(_e1306 + 19u)];
                                                        phi_5945_ = 0u;
                                                        phi_5948_ = 0f;
                                                        phi_5950_ = 1f;
                                                        phi_5952_ = 0f;
                                                        phi_5954_ = _e4229;
                                                        loop {
                                                            let _e4263 = phi_5945_;
                                                            let _e4265 = phi_5948_;
                                                            let _e4267 = phi_5950_;
                                                            let _e4269 = phi_5952_;
                                                            let _e4271 = phi_5954_;
                                                            local_23 = _e4265;
                                                            local_24 = _e4269;
                                                            local_25 = _e4265;
                                                            let _e4272 = (_e4263 < _e4227);
                                                            if _e4272 {
                                                                let _e4275 = (_e4271.x + _e4231.x);
                                                                let _e4278 = (_e4271.y + _e4231.y);
                                                                let _e4281 = (_e4271.z + _e4231.z);
                                                                let _e4282 = floor(_e4275);
                                                                let _e4283 = floor(_e4278);
                                                                let _e4284 = floor(_e4281);
                                                                let _e4293 = bitcast<u32>((select(0i, select(select(i32(_e4282), i32(-2147483648), (_e4282 < -2147483600f)), 2147483647i, (_e4282 > 2147483500f)), (_e4282 == _e4282)) & 255i));
                                                                let _e4302 = bitcast<u32>((select(0i, select(select(i32(_e4283), i32(-2147483648), (_e4283 < -2147483600f)), 2147483647i, (_e4283 > 2147483500f)), (_e4283 == _e4283)) & 255i));
                                                                let _e4311 = bitcast<u32>((select(0i, select(select(i32(_e4284), i32(-2147483648), (_e4284 < -2147483600f)), 2147483647i, (_e4284 > 2147483500f)), (_e4284 == _e4284)) & 255i));
                                                                let _e4312 = (_e4275 - _e4282);
                                                                let _e4313 = (_e4278 - _e4283);
                                                                let _e4314 = (_e4281 - _e4284);
                                                                let _e4321 = (((_e4312 * _e4312) * _e4312) * ((_e4312 * ((_e4312 * 6f) - 15f)) + 10f));
                                                                let _e4328 = (((_e4313 * _e4313) * _e4313) * ((_e4313 * ((_e4313 * 6f) - 15f)) + 10f));
                                                                let _e4338 = global_6.member[(_e4234 + _e4293)];
                                                                let _e4344 = (select(select(u32(_e4338), 0u, (_e4338 < 0f)), 4294967295u, (_e4338 > 4294967000f)) + _e4302);
                                                                let _e4347 = global_6.member[(_e4234 + _e4344)];
                                                                let _e4353 = (select(select(u32(_e4347), 0u, (_e4347 < 0f)), 4294967295u, (_e4347 > 4294967000f)) + _e4311);
                                                                let _e4357 = global_6.member[(_e4234 + (_e4344 + 1u))];
                                                                let _e4363 = (select(select(u32(_e4357), 0u, (_e4357 < 0f)), 4294967295u, (_e4357 > 4294967000f)) + _e4311);
                                                                let _e4367 = global_6.member[(_e4234 + (_e4293 + 1u))];
                                                                let _e4373 = (select(select(u32(_e4367), 0u, (_e4367 < 0f)), 4294967295u, (_e4367 > 4294967000f)) + _e4302);
                                                                let _e4376 = global_6.member[(_e4234 + _e4373)];
                                                                let _e4382 = (select(select(u32(_e4376), 0u, (_e4376 < 0f)), 4294967295u, (_e4376 > 4294967000f)) + _e4311);
                                                                let _e4386 = global_6.member[(_e4234 + (_e4373 + 1u))];
                                                                let _e4392 = (select(select(u32(_e4386), 0u, (_e4386 < 0f)), 4294967295u, (_e4386 > 4294967000f)) + _e4311);
                                                                let _e4395 = global_6.member[(_e4234 + _e4353)];
                                                                let _e4401 = (select(select(u32(_e4395), 0u, (_e4395 < 0f)), 4294967295u, (_e4395 > 4294967000f)) % 12u);
                                                                let _e4404 = select(-1f, 1f, ((_e4401 & 1u) == 0u));
                                                                let _e4407 = select(-1f, 1f, ((_e4401 & 2u) == 0u));
                                                                if (_e4401 < 4u) {
                                                                    phi_25265_ = ((_e4404 * _e4312) + (_e4407 * _e4313));
                                                                } else {
                                                                    if (_e4401 < 8u) {
                                                                        phi_25259_ = ((_e4404 * _e4312) + (_e4407 * _e4314));
                                                                    } else {
                                                                        phi_25259_ = ((_e4404 * _e4313) + (_e4407 * _e4314));
                                                                    }
                                                                    let _e4417 = phi_25259_;
                                                                    phi_25265_ = _e4417;
                                                                }
                                                                let _e4422 = phi_25265_;
                                                                let _e4425 = global_6.member[(_e4234 + _e4382)];
                                                                let _e4431 = (_e4312 - 1f);
                                                                let _e4432 = (select(select(u32(_e4425), 0u, (_e4425 < 0f)), 4294967295u, (_e4425 > 4294967000f)) % 12u);
                                                                let _e4435 = select(-1f, 1f, ((_e4432 & 1u) == 0u));
                                                                let _e4438 = select(-1f, 1f, ((_e4432 & 2u) == 0u));
                                                                if (_e4432 < 4u) {
                                                                    phi_25299_ = ((_e4435 * _e4431) + (_e4438 * _e4313));
                                                                } else {
                                                                    if (_e4432 < 8u) {
                                                                        phi_25293_ = ((_e4435 * _e4431) + (_e4438 * _e4314));
                                                                    } else {
                                                                        phi_25293_ = ((_e4435 * _e4313) + (_e4438 * _e4314));
                                                                    }
                                                                    let _e4448 = phi_25293_;
                                                                    phi_25299_ = _e4448;
                                                                }
                                                                let _e4453 = phi_25299_;
                                                                let _e4456 = global_6.member[(_e4234 + _e4363)];
                                                                let _e4462 = (_e4313 - 1f);
                                                                let _e4463 = (select(select(u32(_e4456), 0u, (_e4456 < 0f)), 4294967295u, (_e4456 > 4294967000f)) % 12u);
                                                                let _e4466 = select(-1f, 1f, ((_e4463 & 1u) == 0u));
                                                                let _e4469 = select(-1f, 1f, ((_e4463 & 2u) == 0u));
                                                                if (_e4463 < 4u) {
                                                                    phi_25333_ = ((_e4466 * _e4312) + (_e4469 * _e4462));
                                                                } else {
                                                                    if (_e4463 < 8u) {
                                                                        phi_25327_ = ((_e4466 * _e4312) + (_e4469 * _e4314));
                                                                    } else {
                                                                        phi_25327_ = ((_e4466 * _e4462) + (_e4469 * _e4314));
                                                                    }
                                                                    let _e4479 = phi_25327_;
                                                                    phi_25333_ = _e4479;
                                                                }
                                                                let _e4484 = phi_25333_;
                                                                let _e4487 = global_6.member[(_e4234 + _e4392)];
                                                                let _e4493 = (select(select(u32(_e4487), 0u, (_e4487 < 0f)), 4294967295u, (_e4487 > 4294967000f)) % 12u);
                                                                let _e4496 = select(-1f, 1f, ((_e4493 & 1u) == 0u));
                                                                let _e4499 = select(-1f, 1f, ((_e4493 & 2u) == 0u));
                                                                if (_e4493 < 4u) {
                                                                    phi_25367_ = ((_e4496 * _e4431) + (_e4499 * _e4462));
                                                                } else {
                                                                    if (_e4493 < 8u) {
                                                                        phi_25361_ = ((_e4496 * _e4431) + (_e4499 * _e4314));
                                                                    } else {
                                                                        phi_25361_ = ((_e4496 * _e4462) + (_e4499 * _e4314));
                                                                    }
                                                                    let _e4509 = phi_25361_;
                                                                    phi_25367_ = _e4509;
                                                                }
                                                                let _e4514 = phi_25367_;
                                                                let _e4518 = global_6.member[(_e4234 + (_e4353 + 1u))];
                                                                let _e4524 = (_e4314 - 1f);
                                                                let _e4525 = (select(select(u32(_e4518), 0u, (_e4518 < 0f)), 4294967295u, (_e4518 > 4294967000f)) % 12u);
                                                                let _e4528 = select(-1f, 1f, ((_e4525 & 1u) == 0u));
                                                                let _e4531 = select(-1f, 1f, ((_e4525 & 2u) == 0u));
                                                                if (_e4525 < 4u) {
                                                                    phi_25401_ = ((_e4528 * _e4312) + (_e4531 * _e4313));
                                                                } else {
                                                                    if (_e4525 < 8u) {
                                                                        phi_25395_ = ((_e4528 * _e4312) + (_e4531 * _e4524));
                                                                    } else {
                                                                        phi_25395_ = ((_e4528 * _e4313) + (_e4531 * _e4524));
                                                                    }
                                                                    let _e4541 = phi_25395_;
                                                                    phi_25401_ = _e4541;
                                                                }
                                                                let _e4546 = phi_25401_;
                                                                let _e4550 = global_6.member[(_e4234 + (_e4382 + 1u))];
                                                                let _e4556 = (select(select(u32(_e4550), 0u, (_e4550 < 0f)), 4294967295u, (_e4550 > 4294967000f)) % 12u);
                                                                let _e4559 = select(-1f, 1f, ((_e4556 & 1u) == 0u));
                                                                let _e4562 = select(-1f, 1f, ((_e4556 & 2u) == 0u));
                                                                if (_e4556 < 4u) {
                                                                    phi_25435_ = ((_e4559 * _e4431) + (_e4562 * _e4313));
                                                                } else {
                                                                    if (_e4556 < 8u) {
                                                                        phi_25429_ = ((_e4559 * _e4431) + (_e4562 * _e4524));
                                                                    } else {
                                                                        phi_25429_ = ((_e4559 * _e4313) + (_e4562 * _e4524));
                                                                    }
                                                                    let _e4572 = phi_25429_;
                                                                    phi_25435_ = _e4572;
                                                                }
                                                                let _e4577 = phi_25435_;
                                                                let _e4581 = global_6.member[(_e4234 + (_e4363 + 1u))];
                                                                let _e4587 = (select(select(u32(_e4581), 0u, (_e4581 < 0f)), 4294967295u, (_e4581 > 4294967000f)) % 12u);
                                                                let _e4590 = select(-1f, 1f, ((_e4587 & 1u) == 0u));
                                                                let _e4593 = select(-1f, 1f, ((_e4587 & 2u) == 0u));
                                                                if (_e4587 < 4u) {
                                                                    phi_25469_ = ((_e4590 * _e4312) + (_e4593 * _e4462));
                                                                } else {
                                                                    if (_e4587 < 8u) {
                                                                        phi_25463_ = ((_e4590 * _e4312) + (_e4593 * _e4524));
                                                                    } else {
                                                                        phi_25463_ = ((_e4590 * _e4462) + (_e4593 * _e4524));
                                                                    }
                                                                    let _e4603 = phi_25463_;
                                                                    phi_25469_ = _e4603;
                                                                }
                                                                let _e4608 = phi_25469_;
                                                                let _e4612 = global_6.member[(_e4234 + (_e4392 + 1u))];
                                                                let _e4618 = (select(select(u32(_e4612), 0u, (_e4612 < 0f)), 4294967295u, (_e4612 > 4294967000f)) % 12u);
                                                                let _e4621 = select(-1f, 1f, ((_e4618 & 1u) == 0u));
                                                                let _e4624 = select(-1f, 1f, ((_e4618 & 2u) == 0u));
                                                                if (_e4618 < 4u) {
                                                                    phi_25503_ = ((_e4621 * _e4431) + (_e4624 * _e4462));
                                                                } else {
                                                                    if (_e4618 < 8u) {
                                                                        phi_25497_ = ((_e4621 * _e4431) + (_e4624 * _e4524));
                                                                    } else {
                                                                        phi_25497_ = ((_e4621 * _e4462) + (_e4624 * _e4524));
                                                                    }
                                                                    let _e4634 = phi_25497_;
                                                                    phi_25503_ = _e4634;
                                                                }
                                                                let _e4639 = phi_25503_;
                                                                let _e4642 = (_e4422 + (_e4321 * (_e4453 - _e4422)));
                                                                let _e4648 = (_e4546 + (_e4321 * (_e4577 - _e4546)));
                                                                let _e4654 = (_e4642 + (_e4328 * ((_e4484 + (_e4321 * (_e4514 - _e4484))) - _e4642)));
                                                                phi_5946_ = (_e4263 + 1u);
                                                                phi_5949_ = (_e4265 + _e4267);
                                                                phi_5951_ = (_e4267 * 0.5f);
                                                                phi_5953_ = (_e4269 + ((_e4654 + ((((_e4314 * _e4314) * _e4314) * ((_e4314 * ((_e4314 * 6f) - 15f)) + 10f)) * ((_e4648 + (_e4328 * ((_e4608 + (_e4321 * (_e4639 - _e4608))) - _e4648))) - _e4654))) * _e4267));
                                                                phi_5955_ = vec3<f32>(((((_e4237 * _e4271.x) + (_e4240 * _e4271.y)) + (_e4243 * _e4271.z)) * 2.03f), ((((_e4246 * _e4271.x) + (_e4249 * _e4271.y)) + (_e4252 * _e4271.z)) * 2.03f), ((((_e4255 * _e4271.x) + (_e4258 * _e4271.y)) + (_e4261 * _e4271.z)) * 2.03f));
                                                            } else {
                                                                phi_5946_ = u32();
                                                                phi_5949_ = f32();
                                                                phi_5951_ = f32();
                                                                phi_5953_ = f32();
                                                                phi_5955_ = vec3<f32>();
                                                            }
                                                            let _e4686 = phi_5946_;
                                                            let _e4688 = phi_5949_;
                                                            let _e4690 = phi_5951_;
                                                            let _e4692 = phi_5953_;
                                                            let _e4694 = phi_5955_;
                                                            phi_5945_ = _e4686;
                                                            phi_5948_ = _e4688;
                                                            phi_5950_ = _e4690;
                                                            phi_5952_ = _e4692;
                                                            phi_5954_ = _e4694;
                                                            if !(_e4272) {
                                                                break;
                                                            }
                                                        }
                                                        let _e4697 = local_23;
                                                        if (_e4697 > 0f) {
                                                            let _e4700 = local_24;
                                                            let _e4702 = local_25;
                                                            phi_6371_ = (_e4700 / _e4702);
                                                        } else {
                                                            phi_6371_ = 0f;
                                                        }
                                                        let _e4705 = phi_6371_;
                                                        if (_e4155 == 0u) {
                                                            phi_6454_ = _e4151;
                                                            phi_6455_ = _e4153;
                                                            phi_6456_ = _e4157;
                                                            phi_6457_ = _e4159;
                                                            phi_6458_ = _e4161;
                                                            phi_6459_ = (_e4163 + (1.5f * _e4705));
                                                        } else {
                                                            if (_e4155 == 1u) {
                                                                phi_6448_ = _e4151;
                                                                phi_6449_ = _e4153;
                                                                phi_6450_ = _e4157;
                                                                phi_6451_ = _e4159;
                                                                phi_6452_ = _e4161;
                                                                phi_6453_ = (_e4163 + (0.8f * _e4705));
                                                            } else {
                                                                let _e4708 = (_e4155 == 2u);
                                                                if _e4708 {
                                                                    phi_6443_ = _e4151;
                                                                    phi_6444_ = _e4153;
                                                                    phi_6445_ = _e4157;
                                                                    phi_6446_ = _e4159;
                                                                } else {
                                                                    if (_e4155 == 3u) {
                                                                        phi_6439_ = _e4151;
                                                                        phi_6440_ = _e4153;
                                                                        phi_6441_ = _e4157;
                                                                        phi_6442_ = vec3<f32>(_e4705, _e4159.y, _e4159.z);
                                                                    } else {
                                                                        if (_e4155 == 4u) {
                                                                            phi_6435_ = _e4151;
                                                                            phi_6436_ = _e4153;
                                                                            phi_6437_ = _e4157;
                                                                            phi_6438_ = vec3<f32>(_e4159.x, _e4705, _e4159.z);
                                                                        } else {
                                                                            if (_e4155 == 5u) {
                                                                                phi_6431_ = _e4151;
                                                                                phi_6432_ = _e4153;
                                                                                phi_6433_ = _e4157;
                                                                                phi_6434_ = vec3<f32>(_e4159.x, _e4159.y, _e4705);
                                                                            } else {
                                                                                let _e4712 = (_e4155 == 6u);
                                                                                if _e4712 {
                                                                                    phi_6428_ = _e4151;
                                                                                    phi_6430_ = _e4157;
                                                                                } else {
                                                                                    if (_e4155 == 7u) {
                                                                                        phi_6426_ = _e4151;
                                                                                        phi_6427_ = vec3<f32>(_e4705, _e4157.y, _e4157.z);
                                                                                    } else {
                                                                                        if (_e4155 == 8u) {
                                                                                            phi_6424_ = _e4151;
                                                                                            phi_6425_ = vec3<f32>(_e4157.x, _e4705, _e4157.z);
                                                                                        } else {
                                                                                            let _e4715 = (_e4155 == 9u);
                                                                                            if _e4715 {
                                                                                                phi_6423_ = vec3<f32>(_e4157.x, _e4157.y, _e4705);
                                                                                            } else {
                                                                                                phi_6423_ = _e4157;
                                                                                            }
                                                                                            let _e4721 = phi_6423_;
                                                                                            phi_6424_ = select(_e4705, _e4151, _e4715);
                                                                                            phi_6425_ = _e4721;
                                                                                        }
                                                                                        let _e4728 = phi_6424_;
                                                                                        let _e4730 = phi_6425_;
                                                                                        phi_6426_ = _e4728;
                                                                                        phi_6427_ = _e4730;
                                                                                    }
                                                                                    let _e4736 = phi_6426_;
                                                                                    let _e4738 = phi_6427_;
                                                                                    phi_6428_ = _e4736;
                                                                                    phi_6430_ = _e4738;
                                                                                }
                                                                                let _e4740 = phi_6428_;
                                                                                let _e4742 = phi_6430_;
                                                                                phi_6431_ = _e4740;
                                                                                phi_6432_ = select(_e4153, _e4705, _e4712);
                                                                                phi_6433_ = _e4742;
                                                                                phi_6434_ = _e4159;
                                                                            }
                                                                            let _e4749 = phi_6431_;
                                                                            let _e4751 = phi_6432_;
                                                                            let _e4753 = phi_6433_;
                                                                            let _e4755 = phi_6434_;
                                                                            phi_6435_ = _e4749;
                                                                            phi_6436_ = _e4751;
                                                                            phi_6437_ = _e4753;
                                                                            phi_6438_ = _e4755;
                                                                        }
                                                                        let _e4761 = phi_6435_;
                                                                        let _e4763 = phi_6436_;
                                                                        let _e4765 = phi_6437_;
                                                                        let _e4767 = phi_6438_;
                                                                        phi_6439_ = _e4761;
                                                                        phi_6440_ = _e4763;
                                                                        phi_6441_ = _e4765;
                                                                        phi_6442_ = _e4767;
                                                                    }
                                                                    let _e4773 = phi_6439_;
                                                                    let _e4775 = phi_6440_;
                                                                    let _e4777 = phi_6441_;
                                                                    let _e4779 = phi_6442_;
                                                                    phi_6443_ = _e4773;
                                                                    phi_6444_ = _e4775;
                                                                    phi_6445_ = _e4777;
                                                                    phi_6446_ = _e4779;
                                                                }
                                                                let _e4781 = phi_6443_;
                                                                let _e4783 = phi_6444_;
                                                                let _e4785 = phi_6445_;
                                                                let _e4787 = phi_6446_;
                                                                phi_6448_ = _e4781;
                                                                phi_6449_ = _e4783;
                                                                phi_6450_ = _e4785;
                                                                phi_6451_ = _e4787;
                                                                phi_6452_ = select(_e4161, _e4705, _e4708);
                                                                phi_6453_ = _e4163;
                                                            }
                                                            let _e4792 = phi_6448_;
                                                            let _e4794 = phi_6449_;
                                                            let _e4796 = phi_6450_;
                                                            let _e4798 = phi_6451_;
                                                            let _e4800 = phi_6452_;
                                                            let _e4802 = phi_6453_;
                                                            phi_6454_ = _e4792;
                                                            phi_6455_ = _e4794;
                                                            phi_6456_ = _e4796;
                                                            phi_6457_ = _e4798;
                                                            phi_6458_ = _e4800;
                                                            phi_6459_ = _e4802;
                                                        }
                                                        let _e4806 = phi_6454_;
                                                        let _e4808 = phi_6455_;
                                                        let _e4810 = phi_6456_;
                                                        let _e4812 = phi_6457_;
                                                        let _e4814 = phi_6458_;
                                                        let _e4816 = phi_6459_;
                                                        phi_5718_ = _e4806;
                                                        phi_5721_ = _e4808;
                                                        phi_5723_ = (_e4155 + 1u);
                                                        phi_5725_ = _e4810;
                                                        phi_5727_ = _e4812;
                                                        phi_5729_ = _e4814;
                                                        phi_5731_ = _e4816;
                                                    } else {
                                                        phi_5718_ = f32();
                                                        phi_5721_ = f32();
                                                        phi_5723_ = u32();
                                                        phi_5725_ = vec3<f32>();
                                                        phi_5727_ = vec3<f32>();
                                                        phi_5729_ = f32();
                                                        phi_5731_ = f32();
                                                    }
                                                    let _e4819 = phi_5718_;
                                                    let _e4821 = phi_5721_;
                                                    let _e4823 = phi_5723_;
                                                    let _e4825 = phi_5725_;
                                                    let _e4827 = phi_5727_;
                                                    let _e4829 = phi_5729_;
                                                    let _e4831 = phi_5731_;
                                                    phi_5717_ = _e4819;
                                                    phi_5720_ = _e4821;
                                                    phi_5722_ = _e4823;
                                                    phi_5724_ = _e4825;
                                                    phi_5726_ = _e4827;
                                                    phi_5728_ = _e4829;
                                                    phi_5730_ = _e4831;
                                                    if !(_e4164) {
                                                        break;
                                                    }
                                                }
                                                if _e4128 {
                                                    let _e4846 = (exp((-(abs(_e3855)) / (0.06f + (abs(_e3866) * 0.015915494f)))) * (0.3f + (0.7f * exp(((_e3866 * -0.71428573f) * (_e3866 * 0.71428573f))))));
                                                    let _e4848 = (_e3855 * 1.3333334f);
                                                    let _e4853 = (1.3f * exp((((_e3866 * _e3866) + (_e4848 * _e4848)) * -12.499999f)));
                                                    let _e4864 = local_26;
                                                    let _e4866 = local_27;
                                                    if (_e4864 != _e4866) {
                                                        phi_25579_ = true;
                                                    } else {
                                                        let _e4869 = local_28;
                                                        phi_25579_ = (0.1f >= _e4869);
                                                    }
                                                    let _e4872 = phi_25579_;
                                                    let _e4874 = local_29;
                                                    let _e4877 = (pow(select(_e4874, 0.1f, _e4872), 1.3f) * _e4086);
                                                    phi_6491_ = vec3<f32>((_e4083.x + (((_e4092 * _e4846) + (_e4101 * _e4853)) * _e4877)), (_e4083.y + (((_e4095 * _e4846) + (_e4104 * _e4853)) * _e4877)), (_e4083.z + (((_e4098 * _e4846) + (_e4107 * _e4853)) * _e4877)));
                                                } else {
                                                    phi_6491_ = _e4083;
                                                }
                                                let _e4889 = phi_6491_;
                                                if select(_e4138, _e4142, _e4144) {
                                                    let _e4891 = local_30;
                                                    let _e4897 = ((_e3855 - (0.025f * _e4891)) / (0.045f + (abs(_e3866) * 0.0095492955f)));
                                                    let _e4902 = local_31;
                                                    let _e4903 = (_e4902 * 3.3333333f);
                                                    let _e4905 = select(_e4903, 0f, (_e4903 < 0f));
                                                    let _e4907 = select(_e4905, 1f, (_e4905 > 1f));
                                                    let _e4923 = local_32;
                                                    let _e4925 = ((_e4923 - 0.25f) * 5.0000005f);
                                                    let _e4927 = select(_e4925, 0f, (_e4925 < 0f));
                                                    let _e4929 = select(_e4927, 1f, (_e4927 > 1f));
                                                    phi_6570_ = (_e4089 * (((exp((-(_e4897) * _e4897)) * (0.15f + (3f * ((_e4907 * _e4907) * (3f - (2f * _e4907)))))) * (0.5f + (0.9f * exp(((_e3866 * -0.6666667f) * (_e3866 * 0.6666667f)))))) + (1.2f * (exp(((_e3855 * -3.3333333f) * (_e3855 * 3.3333333f))) * ((_e4929 * _e4929) * (3f - (2f * _e4929)))))));
                                                } else {
                                                    phi_6570_ = 0f;
                                                }
                                                let _e4943 = phi_6570_;
                                                phi_6571_ = _e4889;
                                                phi_6572_ = _e4943;
                                            } else {
                                                phi_6571_ = _e4083;
                                                phi_6572_ = 0f;
                                            }
                                            let _e4945 = phi_6571_;
                                            let _e4947 = phi_6572_;
                                            phi_6574_ = _e4945;
                                            phi_6577_ = 0u;
                                            loop {
                                                let _e4949 = phi_6574_;
                                                let _e4951 = phi_6577_;
                                                local_49 = _e4949;
                                                local_50 = _e4949;
                                                local_51 = _e4949;
                                                let _e4952 = (_e4951 < bitcast<u32>(_e3818));
                                                if _e4952 {
                                                    let _e4954 = (bitcast<u32>(_e3814) + (_e4951 * 29u));
                                                    let _e4956 = global_6.member[_e4954];
                                                    let _e4957 = (_e4956 > 0.5f);
                                                    let _e4960 = global_6.member[(_e4954 + 1u)];
                                                    let _e4963 = global_6.member[(_e4954 + 2u)];
                                                    let _e4966 = global_6.member[(_e4954 + 3u)];
                                                    let _e4969 = global_6.member[(_e4954 + 4u)];
                                                    let _e4972 = global_6.member[(_e4954 + 5u)];
                                                    let _e4975 = global_6.member[(_e4954 + 6u)];
                                                    let _e4980 = (((_e348.member_1.x * _e4960) + (_e348.member_1.y * _e4963)) + (_e348.member_1.z * _e4966));
                                                    let _e4981 = (_e4980 < _e4972);
                                                    if _e4981 {
                                                        phi_7723_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_7724_ = vec3<f32>();
                                                    } else {
                                                        let _e4984 = global_6.member[(_e4954 + 7u)];
                                                        let _e4987 = global_6.member[(_e4954 + 8u)];
                                                        let _e4990 = global_6.member[(_e4954 + 9u)];
                                                        let _e4993 = global_6.member[(_e4954 + 10u)];
                                                        let _e4996 = global_6.member[(_e4954 + 11u)];
                                                        let _e4999 = global_6.member[(_e4954 + 12u)];
                                                        let _e5002 = global_6.member[(_e4954 + 13u)];
                                                        let _e5005 = global_6.member[(_e4954 + 14u)];
                                                        let _e5008 = global_6.member[(_e4954 + 15u)];
                                                        let _e5011 = global_6.member[(_e4954 + 16u)];
                                                        let _e5014 = global_6.member[(_e4954 + 17u)];
                                                        let _e5017 = global_6.member[(_e4954 + 18u)];
                                                        let _e5020 = global_6.member[(_e4954 + 19u)];
                                                        let _e5023 = global_6.member[(_e4954 + 20u)];
                                                        let _e5026 = global_6.member[(_e4954 + 21u)];
                                                        let _e5029 = global_6.member[(_e4954 + 22u)];
                                                        let _e5032 = global_6.member[(_e4954 + 23u)];
                                                        let _e5035 = global_6.member[(_e4954 + 24u)];
                                                        let _e5038 = global_6.member[(_e4954 + 25u)];
                                                        let _e5041 = global_6.member[(_e4954 + 26u)];
                                                        let _e5044 = global_6.member[(_e4954 + 27u)];
                                                        let _e5047 = global_6.member[(_e4954 + 28u)];
                                                        let _e5054 = (((_e348.member_1.x / _e4980) - _e4960) / _e4969);
                                                        let _e5055 = (((_e348.member_1.y / _e4980) - _e4963) / _e4969);
                                                        let _e5056 = (((_e348.member_1.z / _e4980) - _e4966) / _e4969);
                                                        if _e4957 {
                                                            let _e5661 = (((_e5054 * _e5011) + (_e5055 * _e5014)) + (_e5056 * _e5017));
                                                            let _e5667 = ((((_e5054 * _e5020) + (_e5055 * _e5023)) + (_e5056 * _e5026)) / _e5029);
                                                            phi_7721_ = (vec3<f32>(1.1633643f, 0.97722596f, 0.74455315f) * (_e4975 * exp((-2.5f * pow((sqrt(((_e5661 * _e5661) + (_e5667 * _e5667))) * 3.3333333f), 0.7f)))));
                                                            phi_7722_ = vec3<f32>();
                                                        } else {
                                                            let _e5061 = (((_e4984 * _e4960) + (_e4987 * _e4963)) + (_e4990 * _e4966));
                                                            if (abs(_e5061) < 0.12f) {
                                                                phi_7025_ = select(0.12f, -0.12f, (_e5061 < 0f));
                                                            } else {
                                                                phi_7025_ = _e5061;
                                                            }
                                                            let _e5067 = phi_7025_;
                                                            let _e5073 = ((((_e4984 * _e5054) + (_e4987 * _e5055)) + (_e4990 * _e5056)) / _e5067);
                                                            let _e5077 = (_e5054 - (_e4960 * _e5073));
                                                            let _e5078 = (_e5055 - (_e4963 * _e5073));
                                                            let _e5079 = (_e5056 - (_e4966 * _e5073));
                                                            let _e5084 = (((_e5077 * _e4993) + (_e5078 * _e4996)) + (_e5079 * _e4999));
                                                            let _e5089 = (((_e5077 * _e5002) + (_e5078 * _e5005)) + (_e5079 * _e5008));
                                                            let _e5093 = sqrt(((_e5084 * _e5084) + (_e5089 * _e5089)));
                                                            let _e5105 = abs(_e5067);
                                                            let _e5107 = (pow((0.5f + (0.5f * cos(((_e5032 * (atan2(_e5089, _e5084) - (log((_e5093 + 0.03f)) / _e5035))) + _e5038)))), 3f) * sqrt(_e5105));
                                                            let _e5112 = global_6.member[_e3795];
                                                            let _e5113 = bitcast<u32>(_e5112);
                                                            let _e5116 = global_6.member[(_e1306 + 11u)];
                                                            let _e5119 = global_6.member[(_e1306 + 12u)];
                                                            let _e5122 = global_6.member[(_e1306 + 13u)];
                                                            let _e5125 = global_6.member[(_e1306 + 14u)];
                                                            let _e5128 = global_6.member[(_e1306 + 15u)];
                                                            let _e5131 = global_6.member[(_e1306 + 16u)];
                                                            let _e5134 = global_6.member[(_e1306 + 17u)];
                                                            let _e5137 = global_6.member[(_e1306 + 18u)];
                                                            let _e5140 = global_6.member[(_e1306 + 19u)];
                                                            phi_7205_ = 0u;
                                                            phi_7208_ = 0f;
                                                            phi_7210_ = 1f;
                                                            phi_7212_ = 0f;
                                                            phi_7214_ = vec3<f32>((_e5084 * 9f), (_e5089 * 9f), 0f);
                                                            loop {
                                                                let _e5142 = phi_7205_;
                                                                let _e5144 = phi_7208_;
                                                                let _e5146 = phi_7210_;
                                                                let _e5148 = phi_7212_;
                                                                let _e5150 = phi_7214_;
                                                                local_33 = _e5144;
                                                                local_34 = _e5148;
                                                                local_35 = _e5144;
                                                                let _e5151 = (_e5142 < 4u);
                                                                if _e5151 {
                                                                    let _e5153 = (_e5150.x + _e5041);
                                                                    let _e5155 = (_e5150.y + _e5044);
                                                                    let _e5157 = (_e5150.z + _e5047);
                                                                    let _e5158 = floor(_e5153);
                                                                    let _e5159 = floor(_e5155);
                                                                    let _e5160 = floor(_e5157);
                                                                    let _e5169 = bitcast<u32>((select(0i, select(select(i32(_e5158), i32(-2147483648), (_e5158 < -2147483600f)), 2147483647i, (_e5158 > 2147483500f)), (_e5158 == _e5158)) & 255i));
                                                                    let _e5178 = bitcast<u32>((select(0i, select(select(i32(_e5159), i32(-2147483648), (_e5159 < -2147483600f)), 2147483647i, (_e5159 > 2147483500f)), (_e5159 == _e5159)) & 255i));
                                                                    let _e5187 = bitcast<u32>((select(0i, select(select(i32(_e5160), i32(-2147483648), (_e5160 < -2147483600f)), 2147483647i, (_e5160 > 2147483500f)), (_e5160 == _e5160)) & 255i));
                                                                    let _e5188 = (_e5153 - _e5158);
                                                                    let _e5189 = (_e5155 - _e5159);
                                                                    let _e5190 = (_e5157 - _e5160);
                                                                    let _e5197 = (((_e5188 * _e5188) * _e5188) * ((_e5188 * ((_e5188 * 6f) - 15f)) + 10f));
                                                                    let _e5204 = (((_e5189 * _e5189) * _e5189) * ((_e5189 * ((_e5189 * 6f) - 15f)) + 10f));
                                                                    let _e5214 = global_6.member[(_e5113 + _e5169)];
                                                                    let _e5220 = (select(select(u32(_e5214), 0u, (_e5214 < 0f)), 4294967295u, (_e5214 > 4294967000f)) + _e5178);
                                                                    let _e5223 = global_6.member[(_e5113 + _e5220)];
                                                                    let _e5229 = (select(select(u32(_e5223), 0u, (_e5223 < 0f)), 4294967295u, (_e5223 > 4294967000f)) + _e5187);
                                                                    let _e5233 = global_6.member[(_e5113 + (_e5220 + 1u))];
                                                                    let _e5239 = (select(select(u32(_e5233), 0u, (_e5233 < 0f)), 4294967295u, (_e5233 > 4294967000f)) + _e5187);
                                                                    let _e5243 = global_6.member[(_e5113 + (_e5169 + 1u))];
                                                                    let _e5249 = (select(select(u32(_e5243), 0u, (_e5243 < 0f)), 4294967295u, (_e5243 > 4294967000f)) + _e5178);
                                                                    let _e5252 = global_6.member[(_e5113 + _e5249)];
                                                                    let _e5258 = (select(select(u32(_e5252), 0u, (_e5252 < 0f)), 4294967295u, (_e5252 > 4294967000f)) + _e5187);
                                                                    let _e5262 = global_6.member[(_e5113 + (_e5249 + 1u))];
                                                                    let _e5268 = (select(select(u32(_e5262), 0u, (_e5262 < 0f)), 4294967295u, (_e5262 > 4294967000f)) + _e5187);
                                                                    let _e5271 = global_6.member[(_e5113 + _e5229)];
                                                                    let _e5277 = (select(select(u32(_e5271), 0u, (_e5271 < 0f)), 4294967295u, (_e5271 > 4294967000f)) % 12u);
                                                                    let _e5280 = select(-1f, 1f, ((_e5277 & 1u) == 0u));
                                                                    let _e5283 = select(-1f, 1f, ((_e5277 & 2u) == 0u));
                                                                    if (_e5277 < 4u) {
                                                                        phi_25674_ = ((_e5280 * _e5188) + (_e5283 * _e5189));
                                                                    } else {
                                                                        if (_e5277 < 8u) {
                                                                            phi_25668_ = ((_e5280 * _e5188) + (_e5283 * _e5190));
                                                                        } else {
                                                                            phi_25668_ = ((_e5280 * _e5189) + (_e5283 * _e5190));
                                                                        }
                                                                        let _e5293 = phi_25668_;
                                                                        phi_25674_ = _e5293;
                                                                    }
                                                                    let _e5298 = phi_25674_;
                                                                    let _e5301 = global_6.member[(_e5113 + _e5258)];
                                                                    let _e5307 = (_e5188 - 1f);
                                                                    let _e5308 = (select(select(u32(_e5301), 0u, (_e5301 < 0f)), 4294967295u, (_e5301 > 4294967000f)) % 12u);
                                                                    let _e5311 = select(-1f, 1f, ((_e5308 & 1u) == 0u));
                                                                    let _e5314 = select(-1f, 1f, ((_e5308 & 2u) == 0u));
                                                                    if (_e5308 < 4u) {
                                                                        phi_25708_ = ((_e5311 * _e5307) + (_e5314 * _e5189));
                                                                    } else {
                                                                        if (_e5308 < 8u) {
                                                                            phi_25702_ = ((_e5311 * _e5307) + (_e5314 * _e5190));
                                                                        } else {
                                                                            phi_25702_ = ((_e5311 * _e5189) + (_e5314 * _e5190));
                                                                        }
                                                                        let _e5324 = phi_25702_;
                                                                        phi_25708_ = _e5324;
                                                                    }
                                                                    let _e5329 = phi_25708_;
                                                                    let _e5332 = global_6.member[(_e5113 + _e5239)];
                                                                    let _e5338 = (_e5189 - 1f);
                                                                    let _e5339 = (select(select(u32(_e5332), 0u, (_e5332 < 0f)), 4294967295u, (_e5332 > 4294967000f)) % 12u);
                                                                    let _e5342 = select(-1f, 1f, ((_e5339 & 1u) == 0u));
                                                                    let _e5345 = select(-1f, 1f, ((_e5339 & 2u) == 0u));
                                                                    if (_e5339 < 4u) {
                                                                        phi_25742_ = ((_e5342 * _e5188) + (_e5345 * _e5338));
                                                                    } else {
                                                                        if (_e5339 < 8u) {
                                                                            phi_25736_ = ((_e5342 * _e5188) + (_e5345 * _e5190));
                                                                        } else {
                                                                            phi_25736_ = ((_e5342 * _e5338) + (_e5345 * _e5190));
                                                                        }
                                                                        let _e5355 = phi_25736_;
                                                                        phi_25742_ = _e5355;
                                                                    }
                                                                    let _e5360 = phi_25742_;
                                                                    let _e5363 = global_6.member[(_e5113 + _e5268)];
                                                                    let _e5369 = (select(select(u32(_e5363), 0u, (_e5363 < 0f)), 4294967295u, (_e5363 > 4294967000f)) % 12u);
                                                                    let _e5372 = select(-1f, 1f, ((_e5369 & 1u) == 0u));
                                                                    let _e5375 = select(-1f, 1f, ((_e5369 & 2u) == 0u));
                                                                    if (_e5369 < 4u) {
                                                                        phi_25776_ = ((_e5372 * _e5307) + (_e5375 * _e5338));
                                                                    } else {
                                                                        if (_e5369 < 8u) {
                                                                            phi_25770_ = ((_e5372 * _e5307) + (_e5375 * _e5190));
                                                                        } else {
                                                                            phi_25770_ = ((_e5372 * _e5338) + (_e5375 * _e5190));
                                                                        }
                                                                        let _e5385 = phi_25770_;
                                                                        phi_25776_ = _e5385;
                                                                    }
                                                                    let _e5390 = phi_25776_;
                                                                    let _e5394 = global_6.member[(_e5113 + (_e5229 + 1u))];
                                                                    let _e5400 = (_e5190 - 1f);
                                                                    let _e5401 = (select(select(u32(_e5394), 0u, (_e5394 < 0f)), 4294967295u, (_e5394 > 4294967000f)) % 12u);
                                                                    let _e5404 = select(-1f, 1f, ((_e5401 & 1u) == 0u));
                                                                    let _e5407 = select(-1f, 1f, ((_e5401 & 2u) == 0u));
                                                                    if (_e5401 < 4u) {
                                                                        phi_25810_ = ((_e5404 * _e5188) + (_e5407 * _e5189));
                                                                    } else {
                                                                        if (_e5401 < 8u) {
                                                                            phi_25804_ = ((_e5404 * _e5188) + (_e5407 * _e5400));
                                                                        } else {
                                                                            phi_25804_ = ((_e5404 * _e5189) + (_e5407 * _e5400));
                                                                        }
                                                                        let _e5417 = phi_25804_;
                                                                        phi_25810_ = _e5417;
                                                                    }
                                                                    let _e5422 = phi_25810_;
                                                                    let _e5426 = global_6.member[(_e5113 + (_e5258 + 1u))];
                                                                    let _e5432 = (select(select(u32(_e5426), 0u, (_e5426 < 0f)), 4294967295u, (_e5426 > 4294967000f)) % 12u);
                                                                    let _e5435 = select(-1f, 1f, ((_e5432 & 1u) == 0u));
                                                                    let _e5438 = select(-1f, 1f, ((_e5432 & 2u) == 0u));
                                                                    if (_e5432 < 4u) {
                                                                        phi_25844_ = ((_e5435 * _e5307) + (_e5438 * _e5189));
                                                                    } else {
                                                                        if (_e5432 < 8u) {
                                                                            phi_25838_ = ((_e5435 * _e5307) + (_e5438 * _e5400));
                                                                        } else {
                                                                            phi_25838_ = ((_e5435 * _e5189) + (_e5438 * _e5400));
                                                                        }
                                                                        let _e5448 = phi_25838_;
                                                                        phi_25844_ = _e5448;
                                                                    }
                                                                    let _e5453 = phi_25844_;
                                                                    let _e5457 = global_6.member[(_e5113 + (_e5239 + 1u))];
                                                                    let _e5463 = (select(select(u32(_e5457), 0u, (_e5457 < 0f)), 4294967295u, (_e5457 > 4294967000f)) % 12u);
                                                                    let _e5466 = select(-1f, 1f, ((_e5463 & 1u) == 0u));
                                                                    let _e5469 = select(-1f, 1f, ((_e5463 & 2u) == 0u));
                                                                    if (_e5463 < 4u) {
                                                                        phi_25878_ = ((_e5466 * _e5188) + (_e5469 * _e5338));
                                                                    } else {
                                                                        if (_e5463 < 8u) {
                                                                            phi_25872_ = ((_e5466 * _e5188) + (_e5469 * _e5400));
                                                                        } else {
                                                                            phi_25872_ = ((_e5466 * _e5338) + (_e5469 * _e5400));
                                                                        }
                                                                        let _e5479 = phi_25872_;
                                                                        phi_25878_ = _e5479;
                                                                    }
                                                                    let _e5484 = phi_25878_;
                                                                    let _e5488 = global_6.member[(_e5113 + (_e5268 + 1u))];
                                                                    let _e5494 = (select(select(u32(_e5488), 0u, (_e5488 < 0f)), 4294967295u, (_e5488 > 4294967000f)) % 12u);
                                                                    let _e5497 = select(-1f, 1f, ((_e5494 & 1u) == 0u));
                                                                    let _e5500 = select(-1f, 1f, ((_e5494 & 2u) == 0u));
                                                                    if (_e5494 < 4u) {
                                                                        phi_25912_ = ((_e5497 * _e5307) + (_e5500 * _e5338));
                                                                    } else {
                                                                        if (_e5494 < 8u) {
                                                                            phi_25906_ = ((_e5497 * _e5307) + (_e5500 * _e5400));
                                                                        } else {
                                                                            phi_25906_ = ((_e5497 * _e5338) + (_e5500 * _e5400));
                                                                        }
                                                                        let _e5510 = phi_25906_;
                                                                        phi_25912_ = _e5510;
                                                                    }
                                                                    let _e5515 = phi_25912_;
                                                                    let _e5518 = (_e5298 + (_e5197 * (_e5329 - _e5298)));
                                                                    let _e5524 = (_e5422 + (_e5197 * (_e5453 - _e5422)));
                                                                    let _e5530 = (_e5518 + (_e5204 * ((_e5360 + (_e5197 * (_e5390 - _e5360))) - _e5518)));
                                                                    phi_7206_ = (_e5142 + 1u);
                                                                    phi_7209_ = (_e5144 + _e5146);
                                                                    phi_7211_ = (_e5146 * 0.5f);
                                                                    phi_7213_ = (_e5148 + ((_e5530 + ((((_e5190 * _e5190) * _e5190) * ((_e5190 * ((_e5190 * 6f) - 15f)) + 10f)) * ((_e5524 + (_e5204 * ((_e5484 + (_e5197 * (_e5515 - _e5484))) - _e5524))) - _e5530))) * _e5146));
                                                                    phi_7215_ = vec3<f32>(((((_e5116 * _e5150.x) + (_e5119 * _e5150.y)) + (_e5122 * _e5150.z)) * 2.03f), ((((_e5125 * _e5150.x) + (_e5128 * _e5150.y)) + (_e5131 * _e5150.z)) * 2.03f), ((((_e5134 * _e5150.x) + (_e5137 * _e5150.y)) + (_e5140 * _e5150.z)) * 2.03f));
                                                                } else {
                                                                    phi_7206_ = u32();
                                                                    phi_7209_ = f32();
                                                                    phi_7211_ = f32();
                                                                    phi_7213_ = f32();
                                                                    phi_7215_ = vec3<f32>();
                                                                }
                                                                let _e5562 = phi_7206_;
                                                                let _e5564 = phi_7209_;
                                                                let _e5566 = phi_7211_;
                                                                let _e5568 = phi_7213_;
                                                                let _e5570 = phi_7215_;
                                                                phi_7205_ = _e5562;
                                                                phi_7208_ = _e5564;
                                                                phi_7210_ = _e5566;
                                                                phi_7212_ = _e5568;
                                                                phi_7214_ = _e5570;
                                                                if !(_e5151) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e5573 = local_33;
                                                            if (_e5573 > 0f) {
                                                                let _e5576 = local_34;
                                                                let _e5578 = local_35;
                                                                phi_7628_ = (_e5576 / _e5578);
                                                            } else {
                                                                phi_7628_ = 0f;
                                                            }
                                                            let _e5581 = phi_7628_;
                                                            let _e5583 = (1f + (1.2f * _e5581));
                                                            if (_e5583 != _e5583) {
                                                                phi_25958_ = true;
                                                            } else {
                                                                phi_25958_ = (0f >= _e5583);
                                                            }
                                                            let _e5587 = phi_25958_;
                                                            let _e5592 = ((_e5093 - 1.15f) * -2.8571432f);
                                                            let _e5594 = select(_e5592, 0f, (_e5592 < 0f));
                                                            let _e5596 = select(_e5594, 1f, (_e5594 > 1f));
                                                            let _e5602 = (1f / _e5105);
                                                            if (_e5602 != _e5602) {
                                                                phi_25973_ = true;
                                                            } else {
                                                                phi_25973_ = (4f <= _e5602);
                                                            }
                                                            let _e5606 = phi_25973_;
                                                            let _e5609 = (((exp((_e5093 * -3.5714285f)) * ((_e5596 * _e5596) * (3f - (2f * _e5596)))) * select(_e5602, 4f, _e5606)) * 0.12f);
                                                            let _e5615 = sqrt((((_e5054 * _e5054) + (_e5055 * _e5055)) + (_e5056 * _e5056)));
                                                            let _e5622 = (((_e5054 * _e5020) + (_e5055 * _e5023)) + (_e5056 * _e5026));
                                                            let _e5623 = (1f - _e5105);
                                                            let _e5639 = ((_e5609 * (0.6f + (0.4f * (1f - _e5107)))) + exp((_e5615 * -16.666668f)));
                                                            let _e5645 = (((_e5609 * _e5107) * 1.8f) * select(_e5583, 0f, _e5587));
                                                            let _e5652 = (_e4975 * (1f - ((((0.75f * _e5623) * _e5623) * exp(((_e5622 * -28.571428f) * (_e5622 * 28.571428f)))) * exp((_e5615 * -1.4285715f)))));
                                                            phi_7721_ = vec3<f32>();
                                                            phi_7722_ = vec3<f32>((((1.1633643f * _e5639) + (0.86689025f * _e5645)) * _e5652), (((0.97722596f * _e5639) + (1.0155001f * _e5645)) * _e5652), (((0.74455315f * _e5639) + (1.2384146f * _e5645)) * _e5652));
                                                        }
                                                        let _e5679 = phi_7721_;
                                                        let _e5681 = phi_7722_;
                                                        phi_7723_ = _e5679;
                                                        phi_7724_ = _e5681;
                                                    }
                                                    let _e5683 = phi_7723_;
                                                    let _e5685 = phi_7724_;
                                                    phi_6575_ = (_e4949 + select(_e5685, _e5683, vec3(select(_e4957, true, _e4981))));
                                                    phi_6578_ = (_e4951 + 1u);
                                                } else {
                                                    phi_6575_ = vec3<f32>();
                                                    phi_6578_ = u32();
                                                }
                                                let _e5692 = phi_6575_;
                                                let _e5694 = phi_6578_;
                                                phi_6574_ = _e5692;
                                                phi_6577_ = _e5694;
                                                if !(_e4952) {
                                                    break;
                                                }
                                            }
                                            phi_34464_ = _e4081;
                                            if _e4081 {
                                                break;
                                            }
                                            phi_7743_ = vec3<f32>(0f, 0f, 0f);
                                            phi_7746_ = 0u;
                                            phi_7748_ = _e4947;
                                            loop {
                                                let _e5697 = phi_7743_;
                                                let _e5699 = phi_7746_;
                                                let _e5701 = phi_7748_;
                                                local_45 = _e5701;
                                                local_46 = _e5701;
                                                local_47 = _e5701;
                                                local_48 = _e5701;
                                                local_52 = _e5697;
                                                local_53 = _e5697;
                                                local_54 = _e5697;
                                                let _e5702 = (_e5699 < bitcast<u32>(_e3810));
                                                if _e5702 {
                                                    let _e5704 = (bitcast<u32>(_e3806) + (_e5699 * 18u));
                                                    let _e5706 = global_6.member[_e5704];
                                                    let _e5711 = select(select(u32(_e5706), 0u, (_e5706 < 0f)), 4294967295u, (_e5706 > 4294967000f));
                                                    let _e5714 = global_6.member[(_e5704 + 1u)];
                                                    let _e5717 = global_6.member[(_e5704 + 2u)];
                                                    let _e5720 = global_6.member[(_e5704 + 3u)];
                                                    let _e5723 = global_6.member[(_e5704 + 4u)];
                                                    let _e5726 = global_6.member[(_e5704 + 5u)];
                                                    let _e5729 = global_6.member[(_e5704 + 6u)];
                                                    let _e5732 = global_6.member[(_e5704 + 7u)];
                                                    let _e5735 = global_6.member[(_e5704 + 8u)];
                                                    let _e5738 = global_6.member[(_e5704 + 9u)];
                                                    let _e5741 = global_6.member[(_e5704 + 10u)];
                                                    let _e5744 = global_6.member[(_e5704 + 11u)];
                                                    let _e5747 = global_6.member[(_e5704 + 12u)];
                                                    let _e5750 = global_6.member[(_e5704 + 13u)];
                                                    let _e5753 = global_6.member[(_e5704 + 14u)];
                                                    let _e5756 = global_6.member[(_e5704 + 15u)];
                                                    let _e5759 = global_6.member[(_e5704 + 16u)];
                                                    let _e5762 = global_6.member[(_e5704 + 17u)];
                                                    let _e5767 = (((_e348.member_1.x * _e5714) + (_e348.member_1.y * _e5717)) + (_e348.member_1.z * _e5720));
                                                    if (_e5767 < _e5726) {
                                                        phi_9065_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                    } else {
                                                        let _e5772 = ((_e348.member_1.x / _e5767) - _e5714);
                                                        let _e5773 = ((_e348.member_1.y / _e5767) - _e5717);
                                                        let _e5774 = ((_e348.member_1.z / _e5767) - _e5720);
                                                        let _e5781 = ((((_e5772 * _e5735) + (_e5773 * _e5738)) + (_e5774 * _e5741)) / (_e5723 * _e5753));
                                                        let _e5787 = ((((_e5772 * _e5744) + (_e5773 * _e5747)) + (_e5774 * _e5750)) / _e5723);
                                                        let _e5791 = sqrt(((_e5781 * _e5781) + (_e5787 * _e5787)));
                                                        let _e5794 = vec3<f32>((_e5781 * 2.2f), (_e5787 * 2.2f), 0f);
                                                        let _e5795 = (_e5711 == 0u);
                                                        if _e5795 {
                                                            phi_8050_ = 6u;
                                                            phi_8051_ = 1.2f;
                                                            phi_8052_ = _e5794;
                                                        } else {
                                                            let _e5796 = (_e5711 == 1u);
                                                            if _e5796 {
                                                                phi_8049_ = vec3<f32>((_e5781 * 1.54f), (_e5787 * 1.54f), 0f);
                                                            } else {
                                                                phi_8049_ = _e5794;
                                                            }
                                                            let _e5801 = phi_8049_;
                                                            phi_8050_ = select(6u, 5u, _e5796);
                                                            phi_8051_ = select(1f, 0.8f, _e5796);
                                                            phi_8052_ = _e5801;
                                                        }
                                                        let _e5805 = phi_8050_;
                                                        let _e5807 = phi_8051_;
                                                        let _e5809 = phi_8052_;
                                                        if _e5795 {
                                                            phi_8069_ = vec3<f32>(((_e5781 * 3.7400002f) + -7f), ((_e5787 * 3.7400002f) + 2f), 5f);
                                                        } else {
                                                            phi_8069_ = vec3<f32>((_e5781 * 3.3000002f), (_e5787 * 3.3000002f), 0f);
                                                        }
                                                        let _e5819 = phi_8069_;
                                                        phi_8071_ = 0u;
                                                        phi_8074_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_8076_ = 0f;
                                                        phi_8078_ = 0f;
                                                        phi_8080_ = 0f;
                                                        loop {
                                                            let _e5822 = phi_8071_;
                                                            let _e5824 = phi_8074_;
                                                            let _e5826 = phi_8076_;
                                                            let _e5828 = phi_8078_;
                                                            let _e5830 = phi_8080_;
                                                            local_39 = _e5830;
                                                            local_40 = _e5828;
                                                            local_41 = _e5826;
                                                            local_42 = _e5828;
                                                            local_43 = _e5828;
                                                            local_44 = _e5826;
                                                            let _e5831 = (_e5822 < 6u);
                                                            if _e5831 {
                                                                let _e5832 = (_e5822 == 0u);
                                                                if _e5832 {
                                                                    phi_8122_ = true;
                                                                    phi_8123_ = false;
                                                                    phi_8124_ = false;
                                                                } else {
                                                                    if (_e5822 <= 4u) {
                                                                        phi_8100_ = (_e5711 == 2u);
                                                                    } else {
                                                                        phi_8100_ = true;
                                                                    }
                                                                    let _e5836 = phi_8100_;
                                                                    if _e5836 {
                                                                        let _e5837 = (_e5822 == 5u);
                                                                        if _e5837 {
                                                                            if _e5795 {
                                                                                phi_8116_ = false;
                                                                            } else {
                                                                                phi_8116_ = (_e5711 == 2u);
                                                                            }
                                                                            let _e5840 = phi_8116_;
                                                                            phi_8118_ = _e5840;
                                                                        } else {
                                                                            phi_8118_ = false;
                                                                        }
                                                                        let _e5842 = phi_8118_;
                                                                        phi_8120_ = select(false, _e5795, _e5837);
                                                                        phi_8121_ = _e5842;
                                                                    } else {
                                                                        phi_8120_ = false;
                                                                        phi_8121_ = false;
                                                                    }
                                                                    let _e5845 = phi_8120_;
                                                                    let _e5847 = phi_8121_;
                                                                    phi_8122_ = select(true, false, _e5836);
                                                                    phi_8123_ = _e5845;
                                                                    phi_8124_ = _e5847;
                                                                }
                                                                let _e5850 = phi_8122_;
                                                                let _e5852 = phi_8123_;
                                                                let _e5854 = phi_8124_;
                                                                if select(select(_e5854, false, _e5850), true, select(_e5852, true, _e5850)) {
                                                                    if _e5832 {
                                                                        phi_8171_ = 3u;
                                                                        phi_8172_ = vec3<f32>(((_e5781 * 1.7600001f) + 3f), ((_e5787 * 1.7600001f) + 3f), 3f);
                                                                    } else {
                                                                        if (_e5822 < 4u) {
                                                                            let _e5871 = (_e5822 - 1u);
                                                                            if (_e5871 == 0u) {
                                                                                phi_25992_ = vec3<f32>(1.7f, 9.2f, 3.1f);
                                                                            } else {
                                                                                phi_25992_ = select(vec3<f32>(4.6f, 7.1f, 0.9f), vec3<f32>(8.3f, 2.8f, 5.4f), vec3((_e5871 == 1u)));
                                                                            }
                                                                            let _e5877 = phi_25992_;
                                                                            phi_8169_ = 4u;
                                                                            phi_8170_ = (_e5809 + _e5877);
                                                                        } else {
                                                                            let _e5859 = (_e5822 == 4u);
                                                                            if _e5859 {
                                                                                phi_8168_ = (_e5809 + vec3<f32>((_e5824.x * _e5807), (_e5824.y * _e5807), (_e5824.z * _e5807)));
                                                                            } else {
                                                                                phi_8168_ = _e5819;
                                                                            }
                                                                            let _e5869 = phi_8168_;
                                                                            phi_8169_ = select(select(4u, 5u, _e5795), _e5805, _e5859);
                                                                            phi_8170_ = _e5869;
                                                                        }
                                                                        let _e5880 = phi_8169_;
                                                                        let _e5882 = phi_8170_;
                                                                        phi_8171_ = _e5880;
                                                                        phi_8172_ = _e5882;
                                                                    }
                                                                    let _e5889 = phi_8171_;
                                                                    let _e5891 = phi_8172_;
                                                                    let _e5893 = global_6.member[_e3795];
                                                                    let _e5894 = bitcast<u32>(_e5893);
                                                                    let _e5897 = global_6.member[(_e1306 + 11u)];
                                                                    let _e5900 = global_6.member[(_e1306 + 12u)];
                                                                    let _e5903 = global_6.member[(_e1306 + 13u)];
                                                                    let _e5906 = global_6.member[(_e1306 + 14u)];
                                                                    let _e5909 = global_6.member[(_e1306 + 15u)];
                                                                    let _e5912 = global_6.member[(_e1306 + 16u)];
                                                                    let _e5915 = global_6.member[(_e1306 + 17u)];
                                                                    let _e5918 = global_6.member[(_e1306 + 18u)];
                                                                    let _e5921 = global_6.member[(_e1306 + 19u)];
                                                                    phi_8307_ = 0u;
                                                                    phi_8310_ = 0f;
                                                                    phi_8312_ = 1f;
                                                                    phi_8314_ = 0f;
                                                                    phi_8316_ = _e5891;
                                                                    loop {
                                                                        let _e5923 = phi_8307_;
                                                                        let _e5925 = phi_8310_;
                                                                        let _e5927 = phi_8312_;
                                                                        let _e5929 = phi_8314_;
                                                                        let _e5931 = phi_8316_;
                                                                        local_36 = _e5925;
                                                                        local_37 = _e5929;
                                                                        local_38 = _e5925;
                                                                        let _e5932 = (_e5923 < _e5889);
                                                                        if _e5932 {
                                                                            let _e5934 = (_e5931.x + _e5756);
                                                                            let _e5936 = (_e5931.y + _e5759);
                                                                            let _e5938 = (_e5931.z + _e5762);
                                                                            let _e5939 = floor(_e5934);
                                                                            let _e5940 = floor(_e5936);
                                                                            let _e5941 = floor(_e5938);
                                                                            let _e5950 = bitcast<u32>((select(0i, select(select(i32(_e5939), i32(-2147483648), (_e5939 < -2147483600f)), 2147483647i, (_e5939 > 2147483500f)), (_e5939 == _e5939)) & 255i));
                                                                            let _e5959 = bitcast<u32>((select(0i, select(select(i32(_e5940), i32(-2147483648), (_e5940 < -2147483600f)), 2147483647i, (_e5940 > 2147483500f)), (_e5940 == _e5940)) & 255i));
                                                                            let _e5968 = bitcast<u32>((select(0i, select(select(i32(_e5941), i32(-2147483648), (_e5941 < -2147483600f)), 2147483647i, (_e5941 > 2147483500f)), (_e5941 == _e5941)) & 255i));
                                                                            let _e5969 = (_e5934 - _e5939);
                                                                            let _e5970 = (_e5936 - _e5940);
                                                                            let _e5971 = (_e5938 - _e5941);
                                                                            let _e5978 = (((_e5969 * _e5969) * _e5969) * ((_e5969 * ((_e5969 * 6f) - 15f)) + 10f));
                                                                            let _e5985 = (((_e5970 * _e5970) * _e5970) * ((_e5970 * ((_e5970 * 6f) - 15f)) + 10f));
                                                                            let _e5995 = global_6.member[(_e5894 + _e5950)];
                                                                            let _e6001 = (select(select(u32(_e5995), 0u, (_e5995 < 0f)), 4294967295u, (_e5995 > 4294967000f)) + _e5959);
                                                                            let _e6004 = global_6.member[(_e5894 + _e6001)];
                                                                            let _e6010 = (select(select(u32(_e6004), 0u, (_e6004 < 0f)), 4294967295u, (_e6004 > 4294967000f)) + _e5968);
                                                                            let _e6014 = global_6.member[(_e5894 + (_e6001 + 1u))];
                                                                            let _e6020 = (select(select(u32(_e6014), 0u, (_e6014 < 0f)), 4294967295u, (_e6014 > 4294967000f)) + _e5968);
                                                                            let _e6024 = global_6.member[(_e5894 + (_e5950 + 1u))];
                                                                            let _e6030 = (select(select(u32(_e6024), 0u, (_e6024 < 0f)), 4294967295u, (_e6024 > 4294967000f)) + _e5959);
                                                                            let _e6033 = global_6.member[(_e5894 + _e6030)];
                                                                            let _e6039 = (select(select(u32(_e6033), 0u, (_e6033 < 0f)), 4294967295u, (_e6033 > 4294967000f)) + _e5968);
                                                                            let _e6043 = global_6.member[(_e5894 + (_e6030 + 1u))];
                                                                            let _e6049 = (select(select(u32(_e6043), 0u, (_e6043 < 0f)), 4294967295u, (_e6043 > 4294967000f)) + _e5968);
                                                                            let _e6052 = global_6.member[(_e5894 + _e6010)];
                                                                            let _e6058 = (select(select(u32(_e6052), 0u, (_e6052 < 0f)), 4294967295u, (_e6052 > 4294967000f)) % 12u);
                                                                            let _e6061 = select(-1f, 1f, ((_e6058 & 1u) == 0u));
                                                                            let _e6064 = select(-1f, 1f, ((_e6058 & 2u) == 0u));
                                                                            if (_e6058 < 4u) {
                                                                                phi_26053_ = ((_e6061 * _e5969) + (_e6064 * _e5970));
                                                                            } else {
                                                                                if (_e6058 < 8u) {
                                                                                    phi_26047_ = ((_e6061 * _e5969) + (_e6064 * _e5971));
                                                                                } else {
                                                                                    phi_26047_ = ((_e6061 * _e5970) + (_e6064 * _e5971));
                                                                                }
                                                                                let _e6074 = phi_26047_;
                                                                                phi_26053_ = _e6074;
                                                                            }
                                                                            let _e6079 = phi_26053_;
                                                                            let _e6082 = global_6.member[(_e5894 + _e6039)];
                                                                            let _e6088 = (_e5969 - 1f);
                                                                            let _e6089 = (select(select(u32(_e6082), 0u, (_e6082 < 0f)), 4294967295u, (_e6082 > 4294967000f)) % 12u);
                                                                            let _e6092 = select(-1f, 1f, ((_e6089 & 1u) == 0u));
                                                                            let _e6095 = select(-1f, 1f, ((_e6089 & 2u) == 0u));
                                                                            if (_e6089 < 4u) {
                                                                                phi_26087_ = ((_e6092 * _e6088) + (_e6095 * _e5970));
                                                                            } else {
                                                                                if (_e6089 < 8u) {
                                                                                    phi_26081_ = ((_e6092 * _e6088) + (_e6095 * _e5971));
                                                                                } else {
                                                                                    phi_26081_ = ((_e6092 * _e5970) + (_e6095 * _e5971));
                                                                                }
                                                                                let _e6105 = phi_26081_;
                                                                                phi_26087_ = _e6105;
                                                                            }
                                                                            let _e6110 = phi_26087_;
                                                                            let _e6113 = global_6.member[(_e5894 + _e6020)];
                                                                            let _e6119 = (_e5970 - 1f);
                                                                            let _e6120 = (select(select(u32(_e6113), 0u, (_e6113 < 0f)), 4294967295u, (_e6113 > 4294967000f)) % 12u);
                                                                            let _e6123 = select(-1f, 1f, ((_e6120 & 1u) == 0u));
                                                                            let _e6126 = select(-1f, 1f, ((_e6120 & 2u) == 0u));
                                                                            if (_e6120 < 4u) {
                                                                                phi_26121_ = ((_e6123 * _e5969) + (_e6126 * _e6119));
                                                                            } else {
                                                                                if (_e6120 < 8u) {
                                                                                    phi_26115_ = ((_e6123 * _e5969) + (_e6126 * _e5971));
                                                                                } else {
                                                                                    phi_26115_ = ((_e6123 * _e6119) + (_e6126 * _e5971));
                                                                                }
                                                                                let _e6136 = phi_26115_;
                                                                                phi_26121_ = _e6136;
                                                                            }
                                                                            let _e6141 = phi_26121_;
                                                                            let _e6144 = global_6.member[(_e5894 + _e6049)];
                                                                            let _e6150 = (select(select(u32(_e6144), 0u, (_e6144 < 0f)), 4294967295u, (_e6144 > 4294967000f)) % 12u);
                                                                            let _e6153 = select(-1f, 1f, ((_e6150 & 1u) == 0u));
                                                                            let _e6156 = select(-1f, 1f, ((_e6150 & 2u) == 0u));
                                                                            if (_e6150 < 4u) {
                                                                                phi_26155_ = ((_e6153 * _e6088) + (_e6156 * _e6119));
                                                                            } else {
                                                                                if (_e6150 < 8u) {
                                                                                    phi_26149_ = ((_e6153 * _e6088) + (_e6156 * _e5971));
                                                                                } else {
                                                                                    phi_26149_ = ((_e6153 * _e6119) + (_e6156 * _e5971));
                                                                                }
                                                                                let _e6166 = phi_26149_;
                                                                                phi_26155_ = _e6166;
                                                                            }
                                                                            let _e6171 = phi_26155_;
                                                                            let _e6175 = global_6.member[(_e5894 + (_e6010 + 1u))];
                                                                            let _e6181 = (_e5971 - 1f);
                                                                            let _e6182 = (select(select(u32(_e6175), 0u, (_e6175 < 0f)), 4294967295u, (_e6175 > 4294967000f)) % 12u);
                                                                            let _e6185 = select(-1f, 1f, ((_e6182 & 1u) == 0u));
                                                                            let _e6188 = select(-1f, 1f, ((_e6182 & 2u) == 0u));
                                                                            if (_e6182 < 4u) {
                                                                                phi_26189_ = ((_e6185 * _e5969) + (_e6188 * _e5970));
                                                                            } else {
                                                                                if (_e6182 < 8u) {
                                                                                    phi_26183_ = ((_e6185 * _e5969) + (_e6188 * _e6181));
                                                                                } else {
                                                                                    phi_26183_ = ((_e6185 * _e5970) + (_e6188 * _e6181));
                                                                                }
                                                                                let _e6198 = phi_26183_;
                                                                                phi_26189_ = _e6198;
                                                                            }
                                                                            let _e6203 = phi_26189_;
                                                                            let _e6207 = global_6.member[(_e5894 + (_e6039 + 1u))];
                                                                            let _e6213 = (select(select(u32(_e6207), 0u, (_e6207 < 0f)), 4294967295u, (_e6207 > 4294967000f)) % 12u);
                                                                            let _e6216 = select(-1f, 1f, ((_e6213 & 1u) == 0u));
                                                                            let _e6219 = select(-1f, 1f, ((_e6213 & 2u) == 0u));
                                                                            if (_e6213 < 4u) {
                                                                                phi_26223_ = ((_e6216 * _e6088) + (_e6219 * _e5970));
                                                                            } else {
                                                                                if (_e6213 < 8u) {
                                                                                    phi_26217_ = ((_e6216 * _e6088) + (_e6219 * _e6181));
                                                                                } else {
                                                                                    phi_26217_ = ((_e6216 * _e5970) + (_e6219 * _e6181));
                                                                                }
                                                                                let _e6229 = phi_26217_;
                                                                                phi_26223_ = _e6229;
                                                                            }
                                                                            let _e6234 = phi_26223_;
                                                                            let _e6238 = global_6.member[(_e5894 + (_e6020 + 1u))];
                                                                            let _e6244 = (select(select(u32(_e6238), 0u, (_e6238 < 0f)), 4294967295u, (_e6238 > 4294967000f)) % 12u);
                                                                            let _e6247 = select(-1f, 1f, ((_e6244 & 1u) == 0u));
                                                                            let _e6250 = select(-1f, 1f, ((_e6244 & 2u) == 0u));
                                                                            if (_e6244 < 4u) {
                                                                                phi_26257_ = ((_e6247 * _e5969) + (_e6250 * _e6119));
                                                                            } else {
                                                                                if (_e6244 < 8u) {
                                                                                    phi_26251_ = ((_e6247 * _e5969) + (_e6250 * _e6181));
                                                                                } else {
                                                                                    phi_26251_ = ((_e6247 * _e6119) + (_e6250 * _e6181));
                                                                                }
                                                                                let _e6260 = phi_26251_;
                                                                                phi_26257_ = _e6260;
                                                                            }
                                                                            let _e6265 = phi_26257_;
                                                                            let _e6269 = global_6.member[(_e5894 + (_e6049 + 1u))];
                                                                            let _e6275 = (select(select(u32(_e6269), 0u, (_e6269 < 0f)), 4294967295u, (_e6269 > 4294967000f)) % 12u);
                                                                            let _e6278 = select(-1f, 1f, ((_e6275 & 1u) == 0u));
                                                                            let _e6281 = select(-1f, 1f, ((_e6275 & 2u) == 0u));
                                                                            if (_e6275 < 4u) {
                                                                                phi_26291_ = ((_e6278 * _e6088) + (_e6281 * _e6119));
                                                                            } else {
                                                                                if (_e6275 < 8u) {
                                                                                    phi_26285_ = ((_e6278 * _e6088) + (_e6281 * _e6181));
                                                                                } else {
                                                                                    phi_26285_ = ((_e6278 * _e6119) + (_e6281 * _e6181));
                                                                                }
                                                                                let _e6291 = phi_26285_;
                                                                                phi_26291_ = _e6291;
                                                                            }
                                                                            let _e6296 = phi_26291_;
                                                                            let _e6299 = (_e6079 + (_e5978 * (_e6110 - _e6079)));
                                                                            let _e6305 = (_e6203 + (_e5978 * (_e6234 - _e6203)));
                                                                            let _e6311 = (_e6299 + (_e5985 * ((_e6141 + (_e5978 * (_e6171 - _e6141))) - _e6299)));
                                                                            phi_8308_ = (_e5923 + 1u);
                                                                            phi_8311_ = (_e5925 + _e5927);
                                                                            phi_8313_ = (_e5927 * 0.5f);
                                                                            phi_8315_ = (_e5929 + ((_e6311 + ((((_e5971 * _e5971) * _e5971) * ((_e5971 * ((_e5971 * 6f) - 15f)) + 10f)) * ((_e6305 + (_e5985 * ((_e6265 + (_e5978 * (_e6296 - _e6265))) - _e6305))) - _e6311))) * _e5927));
                                                                            phi_8317_ = vec3<f32>(((((_e5897 * _e5931.x) + (_e5900 * _e5931.y)) + (_e5903 * _e5931.z)) * 2.03f), ((((_e5906 * _e5931.x) + (_e5909 * _e5931.y)) + (_e5912 * _e5931.z)) * 2.03f), ((((_e5915 * _e5931.x) + (_e5918 * _e5931.y)) + (_e5921 * _e5931.z)) * 2.03f));
                                                                        } else {
                                                                            phi_8308_ = u32();
                                                                            phi_8311_ = f32();
                                                                            phi_8313_ = f32();
                                                                            phi_8315_ = f32();
                                                                            phi_8317_ = vec3<f32>();
                                                                        }
                                                                        let _e6343 = phi_8308_;
                                                                        let _e6345 = phi_8311_;
                                                                        let _e6347 = phi_8313_;
                                                                        let _e6349 = phi_8315_;
                                                                        let _e6351 = phi_8317_;
                                                                        phi_8307_ = _e6343;
                                                                        phi_8310_ = _e6345;
                                                                        phi_8312_ = _e6347;
                                                                        phi_8314_ = _e6349;
                                                                        phi_8316_ = _e6351;
                                                                        if !(_e5932) {
                                                                            break;
                                                                        }
                                                                    }
                                                                    let _e6354 = local_36;
                                                                    if (_e6354 > 0f) {
                                                                        let _e6357 = local_37;
                                                                        let _e6359 = local_38;
                                                                        phi_8730_ = (_e6357 / _e6359);
                                                                    } else {
                                                                        phi_8730_ = 0f;
                                                                    }
                                                                    let _e6362 = phi_8730_;
                                                                    if _e5832 {
                                                                        phi_8765_ = _e5824;
                                                                        phi_8766_ = _e5826;
                                                                        phi_8767_ = _e5828;
                                                                    } else {
                                                                        if (_e5822 == 1u) {
                                                                            phi_8762_ = vec3<f32>(_e6362, _e5824.y, _e5824.z);
                                                                            phi_8763_ = _e5826;
                                                                            phi_8764_ = _e5828;
                                                                        } else {
                                                                            if (_e5822 == 2u) {
                                                                                phi_8759_ = vec3<f32>(_e5824.x, _e6362, _e5824.z);
                                                                                phi_8760_ = _e5826;
                                                                                phi_8761_ = _e5828;
                                                                            } else {
                                                                                if (_e5822 == 3u) {
                                                                                    phi_8756_ = vec3<f32>(_e5824.x, _e5824.y, _e6362);
                                                                                    phi_8757_ = _e5826;
                                                                                    phi_8758_ = _e5828;
                                                                                } else {
                                                                                    let _e6366 = (_e5822 == 4u);
                                                                                    phi_8756_ = _e5824;
                                                                                    phi_8757_ = select(_e6362, _e5826, _e6366);
                                                                                    phi_8758_ = select(_e5828, _e6362, _e6366);
                                                                                }
                                                                                let _e6374 = phi_8756_;
                                                                                let _e6376 = phi_8757_;
                                                                                let _e6378 = phi_8758_;
                                                                                phi_8759_ = _e6374;
                                                                                phi_8760_ = _e6376;
                                                                                phi_8761_ = _e6378;
                                                                            }
                                                                            let _e6384 = phi_8759_;
                                                                            let _e6386 = phi_8760_;
                                                                            let _e6388 = phi_8761_;
                                                                            phi_8762_ = _e6384;
                                                                            phi_8763_ = _e6386;
                                                                            phi_8764_ = _e6388;
                                                                        }
                                                                        let _e6394 = phi_8762_;
                                                                        let _e6396 = phi_8763_;
                                                                        let _e6398 = phi_8764_;
                                                                        phi_8765_ = _e6394;
                                                                        phi_8766_ = _e6396;
                                                                        phi_8767_ = _e6398;
                                                                    }
                                                                    let _e6400 = phi_8765_;
                                                                    let _e6402 = phi_8766_;
                                                                    let _e6404 = phi_8767_;
                                                                    phi_8769_ = _e6400;
                                                                    phi_8770_ = _e6402;
                                                                    phi_8771_ = _e6404;
                                                                    phi_8772_ = select(_e5830, _e6362, _e5832);
                                                                } else {
                                                                    phi_8769_ = _e5824;
                                                                    phi_8770_ = _e5826;
                                                                    phi_8771_ = _e5828;
                                                                    phi_8772_ = _e5830;
                                                                }
                                                                let _e6407 = phi_8769_;
                                                                let _e6409 = phi_8770_;
                                                                let _e6411 = phi_8771_;
                                                                let _e6413 = phi_8772_;
                                                                phi_8072_ = (_e5822 + 1u);
                                                                phi_8075_ = _e6407;
                                                                phi_8077_ = _e6409;
                                                                phi_8079_ = _e6411;
                                                                phi_8081_ = _e6413;
                                                            } else {
                                                                phi_8072_ = u32();
                                                                phi_8075_ = vec3<f32>();
                                                                phi_8077_ = f32();
                                                                phi_8079_ = f32();
                                                                phi_8081_ = f32();
                                                            }
                                                            let _e6416 = phi_8072_;
                                                            let _e6418 = phi_8075_;
                                                            let _e6420 = phi_8077_;
                                                            let _e6422 = phi_8079_;
                                                            let _e6424 = phi_8081_;
                                                            phi_8071_ = _e6416;
                                                            phi_8074_ = _e6418;
                                                            phi_8076_ = _e6420;
                                                            phi_8078_ = _e6422;
                                                            phi_8080_ = _e6424;
                                                            if !(_e5831) {
                                                                break;
                                                            }
                                                        }
                                                        let _e6427 = local_39;
                                                        let _e6430 = (_e5791 * (1f + (0.45f * _e6427)));
                                                        if _e5795 {
                                                            let _e6533 = local_43;
                                                            let _e6535 = ((_e6533 - -0.25f) * 1.4285715f);
                                                            let _e6537 = select(_e6535, 0f, (_e6535 < 0f));
                                                            let _e6539 = select(_e6537, 1f, (_e6537 > 1f));
                                                            let _e6543 = ((_e6539 * _e6539) * (3f - (2f * _e6539)));
                                                            let _e6545 = ((_e6430 - 1.05f) * -1.1764706f);
                                                            let _e6547 = select(_e6545, 0f, (_e6545 < 0f));
                                                            let _e6549 = select(_e6547, 1f, (_e6547 > 1f));
                                                            let _e6553 = ((_e6549 * _e6549) * (3f - (2f * _e6549)));
                                                            let _e6554 = (_e6543 * _e6553);
                                                            let _e6556 = ((_e6430 - 0.65f) * -1.5384616f);
                                                            let _e6558 = select(_e6556, 0f, (_e6556 < 0f));
                                                            let _e6560 = select(_e6558, 1f, (_e6558 > 1f));
                                                            let _e6565 = (((_e6560 * _e6560) * (3f - (2f * _e6560))) * _e6543);
                                                            let _e6567 = local_44;
                                                            let _e6569 = ((_e6567 - 0.1f) * 2.857143f);
                                                            let _e6571 = select(_e6569, 0f, (_e6569 < 0f));
                                                            let _e6573 = select(_e6571, 1f, (_e6571 > 1f));
                                                            let _e6578 = (((_e6573 * _e6573) * (3f - (2f * _e6573))) * _e6553);
                                                            let _e6583 = ((0.8f * _e6565) * _e6565);
                                                            let _e6592 = (_e5729 * exp((-2f * _e6578)));
                                                            phi_9064_ = type_59(vec3<f32>((((3.1815925f * _e6554) + (0.14634217f * _e6583)) * _e6592), (((0.38179109f * _e6554) + (1.2439084f * _e6583)) * _e6592), (((0.69995034f * _e6554) + (1.0975662f * _e6583)) * _e6592)), (_e5732 * _e6578));
                                                        } else {
                                                            if (_e5711 == 1u) {
                                                                let _e6503 = local_42;
                                                                let _e6505 = ((_e6503 - -0.3f) * 1.25f);
                                                                let _e6507 = select(_e6505, 0f, (_e6505 < 0f));
                                                                let _e6509 = select(_e6507, 1f, (_e6507 > 1f));
                                                                let _e6515 = ((_e6430 - 1f) * -1f);
                                                                let _e6517 = select(_e6515, 0f, (_e6515 < 0f));
                                                                let _e6519 = select(_e6517, 1f, (_e6517 > 1f));
                                                                let _e6525 = (((_e6509 * _e6509) * (3f - (2f * _e6509))) * pow(((_e6519 * _e6519) * (3f - (2f * _e6519))), 1.5f));
                                                                phi_9063_ = type_59((vec3<f32>(0.6481841f, 1.0185751f, 1.8519546f) * (_e5729 * _e6525)), (_e5732 * _e6525));
                                                            } else {
                                                                if (_e5711 == 2u) {
                                                                    let _e6459 = local_41;
                                                                    let _e6462 = (_e5791 * (1f + (0.15f * _e6459)));
                                                                    let _e6463 = (_e6462 - 0.65f);
                                                                    let _e6467 = exp(((_e6463 * -5.5555553f) * (_e6463 * 5.5555553f)));
                                                                    let _e6469 = ((_e6462 - 0.75f) * -1.8181818f);
                                                                    let _e6471 = select(_e6469, 0f, (_e6469 < 0f));
                                                                    let _e6473 = select(_e6471, 1f, (_e6471 > 1f));
                                                                    let _e6477 = ((_e6473 * _e6473) * (3f - (2f * _e6473)));
                                                                    let _e6482 = (exp(((_e5791 * -25f) * (_e5791 * 25f))) * 3f);
                                                                    phi_9062_ = type_59(vec3<f32>(((((3.1815925f * _e6467) + (_e6477 * 0.10243952f)) + _e6482) * _e5729), ((((0.38179109f * _e6467) + (_e6477 * 0.8707359f)) + _e6482) * _e5729), ((((0.69995034f * _e6467) + (_e6477 * 0.76829636f)) + _e6482) * _e5729)), 0f);
                                                                } else {
                                                                    let _e6434 = local_40;
                                                                    let _e6436 = ((_e6434 - -0.2f) * 1.6666666f);
                                                                    let _e6438 = select(_e6436, 0f, (_e6436 < 0f));
                                                                    let _e6440 = select(_e6438, 1f, (_e6438 > 1f));
                                                                    let _e6446 = ((_e6430 - 1f) * -1.4285715f);
                                                                    let _e6448 = select(_e6446, 0f, (_e6446 < 0f));
                                                                    let _e6450 = select(_e6448, 1f, (_e6448 > 1f));
                                                                    phi_9062_ = type_59(vec3<f32>(0f, 0f, 0f), (_e5732 * (((_e6440 * _e6440) * (3f - (2f * _e6440))) * ((_e6450 * _e6450) * (3f - (2f * _e6450))))));
                                                                }
                                                                let _e6501 = phi_9062_;
                                                                phi_9063_ = _e6501;
                                                            }
                                                            let _e6531 = phi_9063_;
                                                            phi_9064_ = _e6531;
                                                        }
                                                        let _e6600 = phi_9064_;
                                                        phi_9065_ = _e6600;
                                                    }
                                                    let _e6602 = phi_9065_;
                                                    phi_7744_ = (_e5697 + _e6602.member);
                                                    phi_7747_ = (_e5699 + 1u);
                                                    phi_7749_ = (_e5701 + _e6602.member_1);
                                                } else {
                                                    phi_7744_ = vec3<f32>();
                                                    phi_7747_ = u32();
                                                    phi_7749_ = f32();
                                                }
                                                let _e6609 = phi_7744_;
                                                let _e6611 = phi_7747_;
                                                let _e6613 = phi_7749_;
                                                phi_7743_ = _e6609;
                                                phi_7746_ = _e6611;
                                                phi_7748_ = _e6613;
                                                if !(_e5702) {
                                                    break;
                                                }
                                            }
                                            phi_34464_ = _e4081;
                                            if _e4081 {
                                                break;
                                            }
                                            let _e6616 = local_45;
                                            if (_e6616 <= 0f) {
                                                phi_9094_ = vec3<f32>(1f, 1f, 1f);
                                            } else {
                                                let _e6619 = local_46;
                                                let _e6622 = local_47;
                                                let _e6627 = local_48;
                                                phi_9094_ = vec3<f32>(exp((_e6622 * -0.81f)), exp(-(_e6619)), exp((_e6627 * -1.25f)));
                                            }
                                            let _e6632 = phi_9094_;
                                            let _e6634 = local_49;
                                            let _e6639 = local_50;
                                            let _e6644 = local_51;
                                            let _e6649 = local_52;
                                            let _e6653 = local_53;
                                            let _e6657 = local_54;
                                            let _e6662 = global_6.member[(_e1306 + 9u)];
                                            phi_34473_ = _e4081;
                                            phi_9141_ = vec3<f32>((_e3786.x + ((((_e6634.x * _e6632.x) + _e6649.x) * _e6662) * _e3788.x)), (_e3786.y + ((((_e6639.y * _e6632.y) + _e6653.y) * _e6662) * _e3788.y)), (_e3786.z + ((((_e6644.z * _e6632.z) + _e6657.z) * _e6662) * _e3788.z)));
                                        } else {
                                            phi_34473_ = _e3784;
                                            phi_9141_ = _e3786;
                                        }
                                        let _e6680 = phi_34473_;
                                        let _e6682 = phi_9141_;
                                        phi_34472_ = _e6680;
                                        phi_9142_ = _e6682;
                                    } else {
                                        phi_34472_ = _e3784;
                                        phi_9142_ = _e3792;
                                    }
                                    let _e6684 = phi_34472_;
                                    let _e6686 = phi_9142_;
                                    phi_34471_ = _e6684;
                                    phi_9143_ = _e6686;
                                } else {
                                    phi_34471_ = _e337;
                                    phi_9143_ = _e1451;
                                }
                                let _e6688 = phi_34471_;
                                let _e6690 = phi_9143_;
                                phi_34467_ = _e6688;
                                phi_10416_ = (vec3<f32>(_e348.member_2.x, _e348.member_2.y, _e348.member_2.z) * _e6690);
                            } else {
                                let _e364 = global_1.member.member_8[1u];
                                let _e374 = global_6.member[_e364];
                                let _e377 = global_6.member[(_e364 + 1u)];
                                let _e380 = global_6.member[(_e364 + 2u)];
                                let _e383 = global_6.member[(_e364 + 3u)];
                                let _e386 = global_6.member[(_e364 + 4u)];
                                let _e389 = global_6.member[(_e364 + 5u)];
                                let _e392 = global_6.member[(_e364 + 6u)];
                                let _e395 = global_6.member[(_e364 + 7u)];
                                let _e398 = global_6.member[(_e364 + 8u)];
                                let _e401 = global_6.member[(_e364 + 9u)];
                                let _e404 = global_6.member[(_e364 + 10u)];
                                let _e407 = global_6.member[(_e364 + 11u)];
                                let _e410 = global_6.member[(_e364 + 12u)];
                                let _e413 = global_6.member[(_e364 + 13u)];
                                let _e416 = global_6.member[(_e364 + 14u)];
                                let _e419 = global_6.member[(_e364 + 15u)];
                                let _e423 = global_6.member[(_e364 + 16u)];
                                let _e426 = global_6.member[(_e364 + 17u)];
                                let _e429 = global_6.member[(_e364 + 18u)];
                                let _e432 = global_6.member[(_e364 + 19u)];
                                let _e435 = global_6.member[(_e364 + 20u)];
                                let _e438 = global_6.member[(_e364 + 29u)];
                                let _e439 = bitcast<u32>(_e438);
                                let _e442 = global_6.member[(_e364 + 30u)];
                                let _e443 = bitcast<u32>(_e442);
                                let _e451 = ((((_e348.member.x - _e374) * _e348.member_1.x) + ((_e348.member.y - _e377) * _e348.member_1.y)) + ((_e348.member.z - _e380) * _e348.member_1.z));
                                let _e463 = ((((_e348.member.x * _e348.member.x) + (_e348.member.y * _e348.member.y)) + (_e348.member.z * _e348.member.z)) - (2f * (((_e348.member.x * _e374) + (_e348.member.y * _e377)) + (_e348.member.z * _e380))));
                                let _e464 = (_e463 + _e392);
                                let _e465 = (_e451 * _e451);
                                let _e466 = (_e465 - _e464);
                                if (_e466 < 0f) {
                                    phi_22196_ = type_57(0u, type_57().member_1);
                                    phi_22197_ = type_57();
                                    phi_22198_ = true;
                                } else {
                                    if (_e451 < 0f) {
                                        phi_22168_ = -(sqrt(_e466));
                                    } else {
                                        phi_22168_ = sqrt(_e466);
                                    }
                                    let _e473 = phi_22168_;
                                    let _e475 = -((_e451 + _e473));
                                    let _e476 = (_e475 == 0f);
                                    if _e476 {
                                        phi_22191_ = type_57(1u, type_56(0f, 0f));
                                        phi_22192_ = type_57();
                                    } else {
                                        let _e477 = (_e464 / _e475);
                                        if (_e475 < _e477) {
                                            phi_22180_ = type_56(_e475, _e477);
                                        } else {
                                            phi_22180_ = type_56(_e477, _e475);
                                        }
                                        let _e482 = phi_22180_;
                                        phi_22191_ = type_57();
                                        phi_22192_ = type_57(1u, _e482);
                                    }
                                    let _e485 = phi_22191_;
                                    let _e487 = phi_22192_;
                                    phi_22196_ = _e485;
                                    phi_22197_ = _e487;
                                    phi_22198_ = _e476;
                                }
                                let _e492 = phi_22196_;
                                let _e494 = phi_22197_;
                                let _e496 = phi_22198_;
                                if _e496 {
                                    phi_22202_ = _e492;
                                } else {
                                    phi_22202_ = _e494;
                                }
                                let _e498 = phi_22202_;
                                if (bitcast<i32>(_e498.member) != 0i) {
                                    phi_22089_ = type_57(0u, _e498.member_1);
                                } else {
                                    phi_22089_ = type_57(1u, type_57().member_1);
                                }
                                let _e508 = phi_22089_;
                                if (bitcast<i32>(_e508.member) != 0i) {
                                    phi_22136_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e508.member_1.member_1 <= 0f) {
                                        phi_22132_ = type_57(0u, type_57().member_1);
                                    } else {
                                        if (_e508.member_1.member != _e508.member_1.member) {
                                            phi_22240_ = true;
                                        } else {
                                            phi_22240_ = (0f >= _e508.member_1.member);
                                        }
                                        let _e520 = phi_22240_;
                                        let _e521 = select(_e508.member_1.member, 0f, _e520);
                                        let _e522 = (_e463 + _e389);
                                        if (_e522 > 0f) {
                                            let _e524 = (_e465 - _e522);
                                            if (_e524 < 0f) {
                                                phi_22348_ = type_57(0u, type_57().member_1);
                                                phi_22349_ = type_57();
                                                phi_22350_ = true;
                                            } else {
                                                if (_e451 < 0f) {
                                                    phi_22320_ = -(sqrt(_e524));
                                                } else {
                                                    phi_22320_ = sqrt(_e524);
                                                }
                                                let _e531 = phi_22320_;
                                                let _e533 = -((_e451 + _e531));
                                                let _e534 = (_e533 == 0f);
                                                if _e534 {
                                                    phi_22343_ = type_57(1u, type_56(0f, 0f));
                                                    phi_22344_ = type_57();
                                                } else {
                                                    let _e535 = (_e522 / _e533);
                                                    if (_e533 < _e535) {
                                                        phi_22332_ = type_56(_e533, _e535);
                                                    } else {
                                                        phi_22332_ = type_56(_e535, _e533);
                                                    }
                                                    let _e540 = phi_22332_;
                                                    phi_22343_ = type_57();
                                                    phi_22344_ = type_57(1u, _e540);
                                                }
                                                let _e543 = phi_22343_;
                                                let _e545 = phi_22344_;
                                                phi_22348_ = _e543;
                                                phi_22349_ = _e545;
                                                phi_22350_ = _e534;
                                            }
                                            let _e550 = phi_22348_;
                                            let _e552 = phi_22349_;
                                            let _e554 = phi_22350_;
                                            if _e554 {
                                                phi_22354_ = _e550;
                                            } else {
                                                phi_22354_ = _e552;
                                            }
                                            let _e556 = phi_22354_;
                                            if (bitcast<i32>(_e556.member) != 0i) {
                                                phi_22288_ = select(_e556.member_1.member_1, _e556.member_1.member, (_e556.member_1.member >= 0f));
                                            } else {
                                                phi_22288_ = -1f;
                                            }
                                            let _e567 = phi_22288_;
                                            if (_e567 >= 0f) {
                                                if (_e508.member_1.member_1 != _e508.member_1.member_1) {
                                                    phi_22392_ = true;
                                                } else {
                                                    phi_22392_ = (_e567 <= _e508.member_1.member_1);
                                                }
                                                let _e572 = phi_22392_;
                                                phi_22117_ = select(_e508.member_1.member_1, _e567, _e572);
                                            } else {
                                                phi_22117_ = _e508.member_1.member_1;
                                            }
                                            let _e575 = phi_22117_;
                                            phi_22119_ = _e575;
                                        } else {
                                            phi_22119_ = _e508.member_1.member_1;
                                        }
                                        let _e577 = phi_22119_;
                                        if (_e577 <= _e521) {
                                            phi_22128_ = type_57(0u, type_57().member_1);
                                        } else {
                                            phi_22128_ = type_57(1u, type_56(_e521, _e577));
                                        }
                                        let _e585 = phi_22128_;
                                        phi_22132_ = _e585;
                                    }
                                    let _e590 = phi_22132_;
                                    phi_22136_ = _e590;
                                }
                                let _e595 = phi_22136_;
                                if (bitcast<i32>(_e595.member) != 0i) {
                                    let _e601 = (_e595.member_1.member < _e348.member.w);
                                    if _e601 {
                                        if (_e595.member_1.member_1 != _e595.member_1.member_1) {
                                            phi_22407_ = true;
                                        } else {
                                            phi_22407_ = (_e348.member.w <= _e595.member_1.member_1);
                                        }
                                        let _e607 = phi_22407_;
                                        let _e608 = select(_e595.member_1.member_1, _e348.member.w, _e607);
                                        phi_9537_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9540_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9542_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9544_ = vec3<f32>(0f, 0f, 0f);
                                        phi_9546_ = 0u;
                                        loop {
                                            let _e610 = phi_9537_;
                                            let _e612 = phi_9540_;
                                            let _e614 = phi_9542_;
                                            let _e616 = phi_9544_;
                                            let _e618 = phi_9546_;
                                            local = _e614;
                                            local_1 = _e612;
                                            local_2 = _e610;
                                            local_3 = _e610;
                                            local_4 = _e610;
                                            local_5 = _e616;
                                            local_6 = _e616;
                                            local_7 = _e616;
                                            let _e619 = (_e618 < 32u);
                                            if _e619 {
                                                let _e620 = -(_e451);
                                                if (_e595.member_1.member <= _e608) {
                                                } else {
                                                    phi_33986_ = true;
                                                    break;
                                                }
                                                let _e623 = select(_e620, _e595.member_1.member, (_e620 < _e595.member_1.member));
                                                let _e625 = select(_e623, _e608, (_e623 > _e608));
                                                let _e626 = (_e625 - _e595.member_1.member);
                                                let _e627 = (_e608 - _e625);
                                                let _e628 = (_e608 - _e595.member_1.member);
                                                let _e629 = (0.000000001f * _e628);
                                                if (_e626 <= _e629) {
                                                    let _e704 = (f32(_e618) * 0.03125f);
                                                    let _e707 = (f32((_e618 + 1u)) * 0.03125f);
                                                    let _e709 = ((_e704 + _e707) * 0.5f);
                                                    phi_9647_ = type_56((_e595.member_1.member + ((_e628 * _e709) * _e709)), (_e628 * ((_e707 * _e707) - (_e704 * _e704))));
                                                } else {
                                                    if (_e627 <= _e629) {
                                                        let _e687 = (f32((31u - _e618)) * 0.03125f);
                                                        let _e690 = (f32((32u - _e618)) * 0.03125f);
                                                        let _e692 = ((_e687 + _e690) * 0.5f);
                                                        phi_9646_ = type_56((_e608 - ((_e628 * _e692) * _e692)), (_e628 * ((_e690 * _e690) - (_e687 * _e687))));
                                                    } else {
                                                        let _e635 = round(((32f * _e626) / (_e626 + _e627)));
                                                        let _e640 = select(select(u32(_e635), 0u, (_e635 < 0f)), 4294967295u, (_e635 > 4294967000f));
                                                        if (_e640 < 1u) {
                                                            phi_9630_ = 1u;
                                                        } else {
                                                            phi_9630_ = select(_e640, 31u, (_e640 > 31u));
                                                        }
                                                        let _e645 = phi_9630_;
                                                        if (_e618 < _e645) {
                                                            let _e666 = ((_e645 - 1u) - _e618);
                                                            let _e668 = f32(_e645);
                                                            let _e669 = (f32(_e666) / _e668);
                                                            let _e672 = (f32((_e666 + 1u)) / _e668);
                                                            let _e674 = ((_e669 + _e672) * 0.5f);
                                                            phi_9645_ = type_56((_e625 - ((_e626 * _e674) * _e674)), (_e626 * ((_e672 * _e672) - (_e669 * _e669))));
                                                        } else {
                                                            let _e648 = (_e618 - _e645);
                                                            let _e650 = f32((32u - _e645));
                                                            let _e651 = (f32(_e648) / _e650);
                                                            let _e654 = (f32((_e648 + 1u)) / _e650);
                                                            let _e656 = ((_e651 + _e654) * 0.5f);
                                                            phi_9645_ = type_56((_e625 + ((_e627 * _e656) * _e656)), (_e627 * ((_e654 * _e654) - (_e651 * _e651))));
                                                        }
                                                        let _e684 = phi_9645_;
                                                        phi_9646_ = _e684;
                                                    }
                                                    let _e702 = phi_9646_;
                                                    phi_9647_ = _e702;
                                                }
                                                let _e719 = phi_9647_;
                                                let _e725 = (_e348.member.x + (_e348.member_1.x * _e719.member));
                                                let _e726 = (_e348.member.y + (_e348.member_1.y * _e719.member));
                                                let _e727 = (_e348.member.z + (_e348.member_1.z * _e719.member));
                                                let _e740 = ((((_e725 * _e725) + (_e726 * _e726)) + (_e727 * _e727)) - (2f * (((_e725 * _e374) + (_e726 * _e377)) + (_e727 * _e380))));
                                                let _e742 = (_e725 - _e374);
                                                let _e743 = (_e726 - _e377);
                                                let _e744 = (_e727 - _e380);
                                                let _e750 = sqrt((((_e742 * _e742) + (_e743 * _e743)) + (_e744 * _e744)));
                                                let _e752 = ((_e740 + _e389) / (_e750 + _e383));
                                                let _e756 = (1f - (abs((_e752 - 25000f)) * 0.00006666667f));
                                                if (_e756 != _e756) {
                                                    phi_22607_ = true;
                                                } else {
                                                    phi_22607_ = (0f >= _e756);
                                                }
                                                let _e760 = phi_22607_;
                                                let _e762 = -(_e752);
                                                let _e765 = (exp((_e762 / _e404)) * _e719.member_1);
                                                let _e768 = (exp((_e762 / _e407)) * _e719.member_1);
                                                let _e770 = (_e765 * 0.5f);
                                                let _e771 = (_e768 * 0.5f);
                                                let _e772 = ((select(_e756, 0f, _e760) * _e719.member_1) * 0.5f);
                                                let _e774 = (_e616.x + _e770);
                                                let _e776 = (_e616.y + _e771);
                                                let _e778 = (_e616.z + _e772);
                                                let _e783 = (_e423 * 1.1f);
                                                let _e784 = (_e783 * _e776);
                                                let _e803 = (_e423 * _e768);
                                                if (_e443 == 4294967295u) {
                                                    phi_9975_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e811 = (1f / _e750);
                                                    let _e819 = ((((_e742 * _e811) * _e395) + ((_e743 * _e811) * _e398)) + ((_e744 * _e811) * _e401));
                                                    if (_e752 != _e752) {
                                                        phi_22734_ = true;
                                                    } else {
                                                        phi_22734_ = (0f >= _e752);
                                                    }
                                                    let _e823 = phi_22734_;
                                                    let _e827 = sqrt((select(_e752, 0f, _e823) / (_e386 - _e383)));
                                                    if (_e827 != _e827) {
                                                        phi_22749_ = true;
                                                    } else {
                                                        phi_22749_ = (1f <= _e827);
                                                    }
                                                    let _e831 = phi_22749_;
                                                    let _e833 = (select(_e827, 1f, _e831) * 31f);
                                                    let _e835 = select(_e819, -1f, (_e819 < -1f));
                                                    let _e839 = ((select(_e835, 1f, (_e835 > 1f)) + 1f) * 15.5f);
                                                    let _e844 = select(select(u32(_e833), 0u, (_e833 < 0f)), 4294967295u, (_e833 > 4294967000f));
                                                    let _e846 = select(_e844, 30u, (30u < _e844));
                                                    let _e851 = select(select(u32(_e839), 0u, (_e839 < 0f)), 4294967295u, (_e839 > 4294967000f));
                                                    let _e853 = select(_e851, 30u, (30u < _e851));
                                                    let _e855 = (_e833 - f32(_e846));
                                                    let _e857 = (_e839 - f32(_e853));
                                                    let _e861 = (_e443 + (3u * ((_e846 * 32u) + _e853)));
                                                    let _e866 = (_e443 + (3u * (((_e846 + 1u) * 32u) + _e853)));
                                                    let _e868 = global_6.member[_e861];
                                                    let _e871 = global_6.member[(_e861 + 1u)];
                                                    let _e874 = global_6.member[(_e861 + 2u)];
                                                    let _e875 = (1f - _e857);
                                                    let _e881 = global_6.member[(_e861 + 3u)];
                                                    let _e884 = global_6.member[(_e861 + 4u)];
                                                    let _e887 = global_6.member[(_e861 + 5u)];
                                                    let _e894 = (1f - _e855);
                                                    let _e899 = global_6.member[_e866];
                                                    let _e902 = global_6.member[(_e866 + 1u)];
                                                    let _e905 = global_6.member[(_e866 + 2u)];
                                                    let _e911 = global_6.member[(_e866 + 3u)];
                                                    let _e914 = global_6.member[(_e866 + 4u)];
                                                    let _e917 = global_6.member[(_e866 + 5u)];
                                                    phi_9975_ = vec3<f32>(((((_e868 * _e875) + (_e881 * _e857)) * _e894) + (((_e899 * _e875) + (_e911 * _e857)) * _e855)), ((((_e871 * _e875) + (_e884 * _e857)) * _e894) + (((_e902 * _e875) + (_e914 * _e857)) * _e855)), ((((_e874 * _e875) + (_e887 * _e857)) * _e894) + (((_e905 * _e875) + (_e917 * _e857)) * _e855)));
                                                }
                                                let _e932 = phi_9975_;
                                                let _e946 = (_e740 + _e392);
                                                if (_e946 > 0f) {
                                                    let _e952 = (((_e742 * _e395) + (_e743 * _e398)) + (_e744 * _e401));
                                                    let _e954 = ((_e952 * _e952) - _e946);
                                                    if (_e952 >= 0f) {
                                                        phi_10049_ = vec3<f32>();
                                                        phi_10050_ = true;
                                                    } else {
                                                        let _e956 = (_e954 <= 0f);
                                                        if _e956 {
                                                            phi_10048_ = vec3<f32>();
                                                        } else {
                                                            let _e959 = (-(_e952) - sqrt(_e954));
                                                            phi_10048_ = vec3<f32>((_e725 + (_e395 * _e959)), (_e726 + (_e398 * _e959)), (_e727 + (_e401 * _e959)));
                                                        }
                                                        let _e968 = phi_10048_;
                                                        phi_10049_ = _e968;
                                                        phi_10050_ = _e956;
                                                    }
                                                    let _e970 = phi_10049_;
                                                    let _e972 = phi_10050_;
                                                    phi_10055_ = _e970;
                                                    phi_10056_ = select(true, false, _e972);
                                                } else {
                                                    phi_10055_ = vec3<f32>(_e725, _e726, _e727);
                                                    phi_10056_ = true;
                                                }
                                                let _e975 = phi_10055_;
                                                let _e977 = phi_10056_;
                                                if _e977 {
                                                    let _e979 = (_e975.x - _e374);
                                                    let _e981 = (_e975.y - _e377);
                                                    let _e983 = (_e975.z - _e380);
                                                    let _e989 = sqrt((((_e979 * _e979) + (_e981 * _e981)) + (_e983 * _e983)));
                                                    let _e990 = (1f / _e989);
                                                    let _e1008 = ((((((_e975.x * _e975.x) + (_e975.y * _e975.y)) + (_e975.z * _e975.z)) - (2f * (((_e975.x * _e374) + (_e975.y * _e377)) + (_e975.z * _e380)))) + _e389) / (_e989 + _e383));
                                                    if (_e1008 != _e1008) {
                                                        phi_22873_ = true;
                                                    } else {
                                                        phi_22873_ = (0f >= _e1008);
                                                    }
                                                    let _e1012 = phi_22873_;
                                                    let _e1016 = sqrt((select(_e1008, 0f, _e1012) / (_e386 - _e383)));
                                                    if (_e1016 != _e1016) {
                                                        phi_22888_ = true;
                                                    } else {
                                                        phi_22888_ = (1f <= _e1016);
                                                    }
                                                    let _e1020 = phi_22888_;
                                                    let _e1022 = (select(_e1016, 1f, _e1020) * 63f);
                                                    let _e1027 = ((((_e979 * _e990) * _e395) + ((_e981 * _e990) * _e398)) + ((_e983 * _e990) * _e401));
                                                    let _e1029 = select(_e1027, -1f, (_e1027 < -1f));
                                                    let _e1033 = ((select(_e1029, 1f, (_e1029 > 1f)) + 1f) * 127.5f);
                                                    let _e1038 = select(select(u32(_e1022), 0u, (_e1022 < 0f)), 4294967295u, (_e1022 > 4294967000f));
                                                    let _e1040 = select(_e1038, 62u, (62u < _e1038));
                                                    let _e1045 = select(select(u32(_e1033), 0u, (_e1033 < 0f)), 4294967295u, (_e1033 > 4294967000f));
                                                    let _e1047 = select(_e1045, 254u, (254u < _e1045));
                                                    let _e1049 = (_e1022 - f32(_e1040));
                                                    let _e1051 = (_e1033 - f32(_e1047));
                                                    let _e1055 = (_e439 + (3u * ((_e1040 * 256u) + _e1047)));
                                                    let _e1060 = (_e439 + (3u * (((_e1040 + 1u) * 256u) + _e1047)));
                                                    let _e1062 = global_6.member[_e1055];
                                                    let _e1065 = global_6.member[(_e1055 + 1u)];
                                                    let _e1068 = global_6.member[(_e1055 + 2u)];
                                                    let _e1069 = (1f - _e1051);
                                                    let _e1075 = global_6.member[(_e1055 + 3u)];
                                                    let _e1078 = global_6.member[(_e1055 + 4u)];
                                                    let _e1081 = global_6.member[(_e1055 + 5u)];
                                                    let _e1088 = (1f - _e1049);
                                                    let _e1093 = global_6.member[_e1060];
                                                    let _e1096 = global_6.member[(_e1060 + 1u)];
                                                    let _e1099 = global_6.member[(_e1060 + 2u)];
                                                    let _e1105 = global_6.member[(_e1060 + 3u)];
                                                    let _e1108 = global_6.member[(_e1060 + 4u)];
                                                    let _e1111 = global_6.member[(_e1060 + 5u)];
                                                    phi_10347_ = vec3<f32>(((((_e1062 * _e1069) + (_e1075 * _e1051)) * _e1088) + (((_e1093 * _e1069) + (_e1105 * _e1051)) * _e1049)), ((((_e1065 * _e1069) + (_e1078 * _e1051)) * _e1088) + (((_e1096 * _e1069) + (_e1108 * _e1051)) * _e1049)), ((((_e1068 * _e1069) + (_e1081 * _e1051)) * _e1088) + (((_e1099 * _e1069) + (_e1111 * _e1051)) * _e1049)));
                                                } else {
                                                    phi_10347_ = vec3<f32>(0f, 0f, 0f);
                                                }
                                                let _e1126 = phi_10347_;
                                                let _e1127 = (vec3<f32>(_e774, _e776, _e778) + _e1126);
                                                let _e1133 = (_e783 * _e1127.y);
                                                let _e1145 = exp(-((((_e413 * _e1127.x) + _e1133) + (_e429 * _e1127.z))));
                                                let _e1147 = exp(-((((_e416 * _e1127.x) + _e1133) + (_e432 * _e1127.z))));
                                                let _e1149 = exp(-((((_e419 * _e1127.x) + _e1133) + (_e435 * _e1127.z))));
                                                phi_9538_ = vec3<f32>((_e610.x + ((exp(-((((_e413 * _e774) + _e784) + (_e429 * _e778)))) * ((_e413 * _e765) + _e803)) * _e932.x)), (_e610.y + ((exp(-((((_e416 * _e774) + _e784) + (_e432 * _e778)))) * ((_e416 * _e765) + _e803)) * _e932.y)), (_e610.z + ((exp(-((((_e419 * _e774) + _e784) + (_e435 * _e778)))) * ((_e419 * _e765) + _e803)) * _e932.z)));
                                                phi_9541_ = vec3<f32>((_e612.x + (_e1145 * _e768)), (_e612.y + (_e1147 * _e768)), (_e612.z + (_e1149 * _e768)));
                                                phi_9543_ = vec3<f32>((_e614.x + (_e1145 * _e765)), (_e614.y + (_e1147 * _e765)), (_e614.z + (_e1149 * _e765)));
                                                phi_9545_ = vec3<f32>((_e774 + _e770), (_e776 + _e771), (_e778 + _e772));
                                                phi_9547_ = (_e618 + 1u);
                                            } else {
                                                phi_9538_ = vec3<f32>();
                                                phi_9541_ = vec3<f32>();
                                                phi_9543_ = vec3<f32>();
                                                phi_9545_ = vec3<f32>();
                                                phi_9547_ = u32();
                                            }
                                            let _e1176 = phi_9538_;
                                            let _e1178 = phi_9541_;
                                            let _e1180 = phi_9543_;
                                            let _e1182 = phi_9545_;
                                            let _e1184 = phi_9547_;
                                            phi_9537_ = _e1176;
                                            phi_9540_ = _e1178;
                                            phi_9542_ = _e1180;
                                            phi_9544_ = _e1182;
                                            phi_9546_ = _e1184;
                                            phi_33986_ = _e337;
                                            if !(_e619) {
                                                break;
                                            }
                                        }
                                        let _e1187 = phi_33986_;
                                        phi_34464_ = _e1187;
                                        if _e1187 {
                                            break;
                                        }
                                        let _e1189 = local;
                                        let _e1190 = (_e1189 * vec3<f32>(_e413, _e416, _e419));
                                        let _e1192 = local_1;
                                        let _e1193 = (_e1192 * _e423);
                                        let _e1198 = (((_e348.member_1.x * _e395) + (_e348.member_1.y * _e398)) + (_e348.member_1.z * _e401));
                                        let _e1200 = (1f + (_e1198 * _e1198));
                                        let _e1201 = (0.059683103f * _e1200);
                                        let _e1208 = (_e426 * _e426);
                                        let _e1219 = ((0.119366206f * ((1f - _e1208) * _e1200)) / ((2f + _e1208) * pow(((1f + _e1208) - ((2f * _e426) * _e1198)), 1.5f)));
                                        let _e1230 = local_2;
                                        let _e1234 = local_3;
                                        let _e1238 = local_4;
                                        let _e1247 = local_5;
                                        let _e1254 = local_6;
                                        let _e1256 = ((_e423 * 1.1f) * _e1254.y);
                                        let _e1261 = local_7;
                                        phi_34470_ = _e1187;
                                        phi_10404_ = type_60(vec3<f32>(exp(-((((_e413 * _e1247.x) + _e1256) + (_e429 * _e1261.z)))), exp(-((((_e416 * _e1247.x) + _e1256) + (_e432 * _e1261.z)))), exp(-((((_e419 * _e1247.x) + _e1256) + (_e435 * _e1261.z))))), (vec3<f32>(((((_e1190.x * _e1201) + (_e1193.x * _e1219)) + _e1230.x) * _e410), ((((_e1190.y * _e1201) + (_e1193.y * _e1219)) + _e1234.y) * _e410), ((((_e1190.z * _e1201) + (_e1193.z * _e1219)) + _e1238.z) * _e410)) * 0.38317f));
                                    } else {
                                        phi_34470_ = _e337;
                                        phi_10404_ = type_60();
                                    }
                                    let _e1278 = phi_34470_;
                                    let _e1280 = phi_10404_;
                                    phi_34469_ = _e1278;
                                    phi_10406_ = _e1280;
                                    phi_10407_ = select(true, false, _e601);
                                } else {
                                    phi_34469_ = _e337;
                                    phi_10406_ = type_60();
                                    phi_10407_ = true;
                                }
                                let _e1283 = phi_34469_;
                                let _e1285 = phi_10406_;
                                let _e1287 = phi_10407_;
                                if _e1287 {
                                    phi_10412_ = type_60(vec3<f32>(1f, 1f, 1f), vec3<f32>(0f, 0f, 0f));
                                } else {
                                    phi_10412_ = _e1285;
                                }
                                let _e1289 = phi_10412_;
                                phi_34467_ = _e1283;
                                phi_10416_ = (vec3<f32>(_e348.member_2.x, _e348.member_2.y, _e348.member_2.z) * _e1289.member_1);
                            }
                            let _e6694 = phi_34467_;
                            let _e6696 = phi_10416_;
                            phi_34466_ = _e6694;
                            phi_10417_ = _e6696;
                        }
                        let _e6698 = phi_34466_;
                        let _e6700 = phi_10417_;
                        phi_34465_ = _e6698;
                        phi_535_ = (_e339 + _e6700);
                        phi_538_ = (_e341 + 1u);
                    } else {
                        phi_34465_ = _e337;
                        phi_535_ = vec3<f32>();
                        phi_538_ = u32();
                    }
                    let _e6704 = phi_34465_;
                    let _e6706 = phi_535_;
                    let _e6708 = phi_538_;
                    phi_34003_ = _e6704;
                    phi_534_ = _e6706;
                    phi_537_ = _e6708;
                    phi_34464_ = _e6704;
                    if !(_e342) {
                        break;
                    }
                }
                let _e6711 = phi_34464_;
                if _e6711 {
                    break;
                }
                let _e6713 = local_55;
                let _e6717 = local_56;
                let _e6722 = local_57;
                if ((((_e6713.x * _e6713.x) + (_e6717.y * _e6717.y)) + (_e6722.z * _e6722.z)) > 0f) {
                    if (_e331.member_1 < arrayLength((&global_7.member))) {
                    } else {
                        break;
                    }
                    let _e6732 = global_7.member[_e331.member_1].member_3[0u];
                    global_7.member[_e331.member_1].member_3[0u] = (_e6732 + _e6713.x);
                    let _e6738 = global_7.member[_e331.member_1].member_3[1u];
                    global_7.member[_e331.member_1].member_3[1u] = (_e6738 + _e6717.y);
                    let _e6744 = global_7.member[_e331.member_1].member_3[2u];
                    global_7.member[_e331.member_1].member_3[2u] = (_e6744 + _e6722.z);
                    let _e6750 = global_7.member[_e331.member_1].member_3[3u];
                    global_7.member[_e331.member_1].member_3[3u] = _e6750;
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
            let _e285 = global;
            let _e293 = (_e285.x + (_e285.y * 4194240u));
            let _e296 = atomicLoad((&global_11.member[11u]));
            if (_e293 >= _e296) {
            } else {
                let _e301 = global_1.member.member[0u];
                let _e305 = global_1.member.member[1u];
                let _e310 = global_1.member.member_7[3u];
                let _e317 = atomicLoad((&global_11.member[((12u + (2u * ((_e301 * _e305) * _e310))) + _e293)]));
                if (_e317 < arrayLength((&global_10.member))) {
                } else {
                    break;
                }
                let _e321 = global_10.member[_e317];
                if (_e293 < arrayLength((&global_12.member))) {
                } else {
                    break;
                }
                global_12.member[_e293] = type_34(_e321.member, _e321.member_1, _e321.member_2, _e321.member_3, vec4<u32>(_e321.member_4.x, _e321.member_4.y, _e321.member_4.z, _e317), _e321.member_5);
            }
            break;
        }
    }
    return;
}

fn function_2() {
    var phi_11273_: u32;
    var phi_26388_: vec3<f32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e285 = global;
            let _e293 = (_e285.x + (_e285.y * 4194240u));
            let _e297 = global_1.member.member[0u];
            let _e301 = global_1.member.member[1u];
            let _e302 = (_e297 * _e301);
            let _e306 = global_1.member.member_7[3u];
            let _e307 = (_e302 * _e306);
            if (_e293 >= _e307) {
            } else {
                if (_e293 < arrayLength((&global_13.member))) {
                } else {
                    break;
                }
                let _e312 = global_13.member[_e293];
                if (_e302 == 0u) {
                    break;
                }
                let _e318 = (_e312.member_1.w >= 0.5f);
                if _e318 {
                    phi_11273_ = (select(select(u32(_e312.member_1.w), 0u, (_e312.member_1.w < 0f)), 4294967295u, (_e312.member_1.w > 4294967000f)) - 1u);
                } else {
                    phi_11273_ = 0u;
                }
                let _e326 = phi_11273_;
                let _e330 = global_1.member.member_7[0u];
                if _e318 {
                    phi_26388_ = vec3<f32>(_e312.member_1.x, _e312.member_1.y, _e312.member_1.z);
                } else {
                    phi_26388_ = vec3<f32>(0f, 0f, 0f);
                }
                let _e336 = phi_26388_;
                let _e344 = ((((_e326 * 2654435769u) ^ _e330) * 747796405u) + 2891336453u);
                let _e352 = (((_e344 >> bitcast<u32>((((_e344 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e344) * 277803737u);
                let _e358 = ((((_e293 % _e302) ^ ((_e352 >> bitcast<u32>(22i)) ^ _e352)) * 747796405u) + 2891336453u);
                let _e366 = (((_e358 >> bitcast<u32>((((_e358 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e358) * 277803737u);
                let _e370 = select(8u, 3u, _e318);
                if (_e293 < arrayLength((&global_14.member))) {
                } else {
                    break;
                }
                global_14.member[_e293] = type_34(_e312.member, vec4<f32>(_e336.x, _e336.y, _e336.z, 0f), vec4<f32>(1f, 1f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<u32>(((_e366 >> bitcast<u32>(22i)) ^ _e366), _e370, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f));
                if ((_e370 & 1u) == 0u) {
                } else {
                    let _e381 = global_1.member.member_7[2u];
                    let _e382 = (1u - _e381);
                    let _e383 = (_e382 * 4u);
                    let _e387 = atomicAdd((&global_15.member[(_e383 + 3u)]), 1u);
                    atomicStore((&global_15.member[((12u + (_e382 * _e307)) + _e387)]), _e293);
                    if ((_e387 % 64u) == 0u) {
                        let _e395 = (_e387 / 64u);
                        if (_e395 < 65535u) {
                            let _e399 = atomicAdd((&global_15.member[_e383]), 1u);
                        }
                        if ((_e395 % 65535u) == 0u) {
                            let _e405 = atomicAdd((&global_15.member[(_e383 + 1u)]), 1u);
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
    var local_67: array<u32, 64>;
    var local_68: array<f32, 64>;
    var phi_11464_: type_48;
    var phi_26548_: f32;
    var phi_26560_: type_56;
    var phi_26571_: type_57;
    var phi_26572_: type_57;
    var phi_26576_: type_57;
    var phi_26577_: type_57;
    var phi_26578_: bool;
    var phi_26582_: type_57;
    var phi_26469_: type_57;
    var phi_26620_: bool;
    var phi_26700_: f32;
    var phi_26712_: type_56;
    var phi_26723_: type_57;
    var phi_26724_: type_57;
    var phi_26728_: type_57;
    var phi_26729_: type_57;
    var phi_26730_: bool;
    var phi_26734_: type_57;
    var phi_26668_: f32;
    var phi_26772_: bool;
    var phi_26497_: f32;
    var phi_26499_: f32;
    var phi_26508_: type_57;
    var phi_26512_: type_57;
    var phi_26516_: type_57;
    var phi_26787_: bool;
    var phi_12993_: vec3<f32>;
    var phi_12996_: u32;
    var phi_13080_: u32;
    var phi_13095_: type_56;
    var phi_13096_: type_56;
    var phi_13097_: type_56;
    var phi_26986_: bool;
    var phi_12994_: vec3<f32>;
    var phi_12997_: u32;
    var phi_34475_: bool;
    var local_69: vec3<f32>;
    var local_70: vec3<f32>;
    var local_71: vec3<f32>;
    var phi_36615_: bool;
    var phi_13122_: vec3<f32>;
    var phi_36614_: bool;
    var phi_13124_: vec3<f32>;
    var phi_13125_: bool;
    var phi_36612_: bool;
    var phi_13132_: vec3<f32>;
    var phi_13133_: type_41;
    var phi_36611_: bool;
    var phi_13134_: vec3<f32>;
    var phi_13135_: type_41;
    var phi_13414_: type_60;
    var phi_13381_: vec3<f32>;
    var phi_13382_: vec3<f32>;
    var phi_13415_: type_60;
    var phi_13538_: i32;
    var phi_13541_: u32;
    var phi_13595_: bool;
    var phi_13601_: bool;
    var phi_13608_: u32;
    var phi_13610_: i32;
    var phi_13611_: u32;
    var phi_13539_: i32;
    var phi_13542_: u32;
    var phi_13613_: bool;
    var local_72: u32;
    var phi_14167_: vec3<f32>;
    var phi_14181_: vec3<f32>;
    var phi_27053_: bool;
    var phi_14214_: f32;
    var phi_14322_: vec3<f32>;
    var phi_14328_: f32;
    var phi_27126_: bool;
    var phi_27115_: f32;
    var phi_14409_: vec3<f32>;
    var phi_14410_: vec3<f32>;
    var phi_14411_: vec3<f32>;
    var phi_34609_: u32;
    var phi_14413_: vec3<f32>;
    var phi_14414_: vec3<f32>;
    var phi_14415_: vec3<f32>;
    var phi_14416_: bool;
    var phi_14431_: vec3<f32>;
    var phi_14432_: vec3<f32>;
    var phi_34607_: u32;
    var phi_14434_: vec3<f32>;
    var phi_14435_: vec3<f32>;
    var phi_14436_: vec3<f32>;
    var phi_34606_: u32;
    var phi_14437_: vec3<f32>;
    var phi_14438_: vec3<f32>;
    var phi_14439_: vec3<f32>;
    var phi_14440_: bool;
    var phi_34605_: u32;
    var phi_14441_: vec3<f32>;
    var phi_14442_: vec3<f32>;
    var phi_14443_: vec3<f32>;
    var phi_14444_: bool;
    var phi_14445_: bool;
    var phi_27199_: bool;
    var phi_27361_: bool;
    var phi_27376_: bool;
    var phi_27391_: bool;
    var phi_27418_: bool;
    var phi_27433_: bool;
    var phi_27448_: bool;
    var phi_27475_: bool;
    var phi_27490_: bool;
    var phi_27505_: bool;
    var phi_14624_: f32;
    var phi_14632_: f32;
    var phi_14640_: f32;
    var phi_27702_: bool;
    var phi_27691_: f32;
    var phi_14747_: vec3<f32>;
    var phi_27730_: bool;
    var phi_14748_: vec3<f32>;
    var phi_14749_: vec3<f32>;
    var phi_14750_: vec3<f32>;
    var phi_34993_: vec3<f32>;
    var phi_34916_: vec3<f32>;
    var phi_34837_: u32;
    var phi_34755_: f32;
    var phi_14752_: vec3<f32>;
    var phi_14753_: vec3<f32>;
    var phi_14754_: bool;
    var phi_27745_: bool;
    var phi_27796_: bool;
    var phi_27785_: f32;
    var phi_34964_: vec3<f32>;
    var phi_14287_: vec3<f32>;
    var phi_35316_: vec3<f32>;
    var phi_35110_: f32;
    var phi_34963_: vec3<f32>;
    var phi_34651_: u32;
    var phi_14289_: vec3<f32>;
    var phi_14290_: bool;
    var phi_35262_: vec3<f32>;
    var phi_35070_: f32;
    var phi_34991_: vec3<f32>;
    var phi_34914_: vec3<f32>;
    var phi_34835_: u32;
    var phi_34753_: f32;
    var phi_34676_: u32;
    var phi_34602_: u32;
    var phi_14762_: vec3<f32>;
    var phi_14763_: vec3<f32>;
    var phi_14764_: vec3<f32>;
    var phi_14765_: bool;
    var phi_27862_: bool;
    var phi_14834_: vec3<f32>;
    var phi_34659_: u32;
    var phi_14836_: vec3<f32>;
    var phi_14837_: bool;
    var phi_34658_: u32;
    var phi_14838_: vec3<f32>;
    var phi_14839_: bool;
    var phi_34656_: u32;
    var phi_14845_: vec3<f32>;
    var phi_14846_: bool;
    var phi_14880_: type_74;
    var phi_34661_: u32;
    var phi_14882_: vec3<f32>;
    var phi_14883_: type_74;
    var phi_14884_: bool;
    var phi_14890_: type_74;
    var phi_34665_: u32;
    var phi_14891_: type_74;
    var phi_35259_: vec3<f32>;
    var phi_35067_: f32;
    var phi_34988_: vec3<f32>;
    var phi_34911_: vec3<f32>;
    var phi_34832_: u32;
    var phi_34750_: f32;
    var phi_34673_: u32;
    var phi_34664_: u32;
    var phi_14893_: type_74;
    var phi_14894_: bool;
    var phi_36588_: bool;
    var phi_34663_: u32;
    var phi_14905_: vec3<f32>;
    var phi_14908_: type_41;
    var phi_14910_: u32;
    var phi_14912_: u32;
    var phi_18076_: u32;
    var phi_18079_: u32;
    var phi_18077_: u32;
    var phi_18080_: u32;
    var local_73: u32;
    var phi_18117_: u32;
    var phi_18120_: u32;
    var phi_18118_: u32;
    var phi_18121_: u32;
    var local_74: u32;
    var local_75: u32;
    var phi_18197_: f32;
    var local_76: u32;
    var local_77: u32;
    var phi_18246_: f32;
    var phi_18402_: f32;
    var phi_18409_: vec3<f32>;
    var phi_37008_: u32;
    var phi_18411_: f32;
    var phi_18412_: vec3<f32>;
    var phi_18413_: vec3<f32>;
    var phi_17074_: u32;
    var phi_17077_: u32;
    var phi_17075_: u32;
    var phi_17078_: u32;
    var local_78: u32;
    var local_79: u32;
    var phi_17847_: f32;
    var phi_17850_: f32;
    var phi_28135_: bool;
    var phi_28116_: f32;
    var phi_17851_: f32;
    var phi_28150_: bool;
    var phi_28173_: bool;
    var phi_28188_: bool;
    var phi_17360_: f32;
    var phi_28203_: bool;
    var phi_28218_: bool;
    var phi_28233_: bool;
    var phi_28298_: bool;
    var phi_28386_: bool;
    var phi_28367_: f32;
    var phi_17425_: type_77;
    var phi_17853_: type_77;
    var phi_18029_: bool;
    var phi_18038_: vec3<f32>;
    var phi_18040_: vec3<f32>;
    var phi_18041_: bool;
    var phi_18046_: f32;
    var phi_18047_: vec3<f32>;
    var phi_37016_: u32;
    var phi_18048_: f32;
    var phi_18049_: vec3<f32>;
    var phi_18050_: vec3<f32>;
    var phi_18051_: f32;
    var phi_37015_: u32;
    var phi_18052_: f32;
    var phi_18053_: vec3<f32>;
    var phi_18054_: vec3<f32>;
    var phi_18055_: f32;
    var phi_37007_: u32;
    var phi_18414_: f32;
    var phi_18415_: vec3<f32>;
    var phi_18416_: vec3<f32>;
    var phi_18417_: f32;
    var phi_28401_: bool;
    var phi_28511_: f32;
    var phi_28539_: f32;
    var phi_28567_: f32;
    var phi_28701_: bool;
    var phi_28716_: bool;
    var phi_28731_: bool;
    var phi_28758_: bool;
    var phi_28773_: bool;
    var phi_28788_: bool;
    var phi_28815_: bool;
    var phi_28830_: bool;
    var phi_28845_: bool;
    var phi_15246_: f32;
    var phi_15254_: f32;
    var phi_15262_: f32;
    var phi_29042_: bool;
    var phi_29031_: f32;
    var phi_15369_: vec3<f32>;
    var phi_29070_: bool;
    var phi_15370_: vec3<f32>;
    var phi_15382_: type_59;
    var phi_15383_: type_59;
    var phi_15384_: bool;
    var phi_15389_: type_59;
    var phi_29107_: bool;
    var phi_29137_: bool;
    var phi_29126_: f32;
    var phi_29176_: vec3<f32>;
    var phi_15417_: type_59;
    var phi_15418_: type_59;
    var phi_29198_: vec3<f32>;
    var phi_15419_: type_59;
    var phi_15420_: type_59;
    var phi_15421_: bool;
    var phi_15425_: type_59;
    var phi_29230_: bool;
    var phi_29301_: bool;
    var phi_16245_: f32;
    var phi_29316_: bool;
    var phi_29331_: bool;
    var phi_16718_: vec3<f32>;
    var phi_16719_: vec3<f32>;
    var phi_16720_: bool;
    var phi_16725_: vec3<f32>;
    var phi_16726_: bool;
    var phi_29455_: bool;
    var phi_29470_: bool;
    var phi_17017_: vec3<f32>;
    var phi_17019_: vec3<f32>;
    var phi_17046_: vec3<f32>;
    var phi_17047_: vec3<f32>;
    var phi_15863_: vec3<f32>;
    var phi_15864_: vec3<f32>;
    var phi_15865_: bool;
    var phi_15870_: vec3<f32>;
    var phi_15871_: bool;
    var phi_29632_: bool;
    var phi_29647_: bool;
    var phi_16162_: vec3<f32>;
    var phi_16164_: vec3<f32>;
    var phi_37050_: u32;
    var phi_17048_: f32;
    var phi_17049_: vec3<f32>;
    var phi_17050_: vec3<f32>;
    var phi_37049_: u32;
    var phi_17051_: f32;
    var phi_17052_: vec3<f32>;
    var phi_17053_: vec3<f32>;
    var phi_37006_: u32;
    var phi_18418_: f32;
    var phi_18419_: vec3<f32>;
    var phi_18420_: vec3<f32>;
    var phi_18421_: f32;
    var phi_15008_: f32;
    var phi_15009_: f32;
    var phi_37005_: u32;
    var phi_18422_: f32;
    var phi_18423_: vec3<f32>;
    var phi_18424_: vec3<f32>;
    var phi_18425_: f32;
    var phi_29700_: bool;
    var phi_29810_: f32;
    var phi_29838_: f32;
    var phi_29866_: f32;
    var phi_30000_: bool;
    var phi_30015_: bool;
    var phi_30030_: bool;
    var phi_30057_: bool;
    var phi_30072_: bool;
    var phi_30087_: bool;
    var phi_30114_: bool;
    var phi_30129_: bool;
    var phi_30144_: bool;
    var phi_18648_: f32;
    var phi_18656_: f32;
    var phi_18664_: f32;
    var phi_30341_: bool;
    var phi_30330_: f32;
    var phi_18771_: vec3<f32>;
    var phi_30369_: bool;
    var phi_18772_: vec3<f32>;
    var phi_18784_: type_59;
    var phi_18785_: type_59;
    var phi_18786_: bool;
    var phi_18791_: type_59;
    var phi_30406_: bool;
    var phi_30436_: bool;
    var phi_30425_: f32;
    var phi_30475_: vec3<f32>;
    var phi_18819_: type_59;
    var phi_18820_: type_59;
    var phi_30497_: vec3<f32>;
    var phi_18821_: type_59;
    var phi_18822_: type_59;
    var phi_18823_: bool;
    var phi_18827_: type_59;
    var phi_30511_: f32;
    var phi_30514_: f32;
    var phi_18836_: f32;
    var phi_18869_: u32;
    var phi_18872_: u32;
    var phi_18874_: u32;
    var phi_18876_: u32;
    var phi_18878_: u32;
    var phi_18880_: vec4<f32>;
    var phi_18882_: f32;
    var phi_19269_: u32;
    var phi_19271_: u32;
    var phi_19272_: u32;
    var phi_19274_: bool;
    var phi_19275_: bool;
    var phi_19276_: u32;
    var phi_19277_: u32;
    var phi_19278_: u32;
    var phi_19279_: bool;
    var phi_19280_: bool;
    var phi_19281_: bool;
    var phi_30612_: bool;
    var phi_30642_: bool;
    var phi_30589_: type_56;
    var phi_30592_: type_56;
    var phi_30593_: bool;
    var phi_30598_: type_56;
    var phi_30699_: bool;
    var phi_30714_: bool;
    var phi_30729_: bool;
    var phi_30744_: bool;
    var phi_30676_: type_56;
    var phi_30679_: type_56;
    var phi_30680_: bool;
    var phi_30685_: type_56;
    var phi_30786_: bool;
    var phi_30801_: bool;
    var phi_30816_: bool;
    var phi_30831_: bool;
    var phi_30763_: type_56;
    var phi_30766_: type_56;
    var phi_30767_: bool;
    var phi_30772_: type_56;
    var phi_30567_: bool;
    var phi_30570_: bool;
    var phi_19310_: f32;
    var phi_30933_: bool;
    var phi_30963_: bool;
    var phi_30910_: type_56;
    var phi_30913_: type_56;
    var phi_30914_: bool;
    var phi_30919_: type_56;
    var phi_31020_: bool;
    var phi_31035_: bool;
    var phi_31050_: bool;
    var phi_31065_: bool;
    var phi_30997_: type_56;
    var phi_31000_: type_56;
    var phi_31001_: bool;
    var phi_31006_: type_56;
    var phi_31107_: bool;
    var phi_31122_: bool;
    var phi_31137_: bool;
    var phi_31152_: bool;
    var phi_31084_: type_56;
    var phi_31087_: type_56;
    var phi_31088_: bool;
    var phi_31093_: type_56;
    var phi_30888_: bool;
    var phi_30891_: bool;
    var phi_19319_: f32;
    var phi_19330_: u32;
    var phi_19341_: u32;
    var phi_19380_: u32;
    var phi_19381_: u32;
    var phi_19392_: u32;
    var phi_19393_: u32;
    var phi_19394_: u32;
    var phi_19395_: u32;
    var phi_19397_: u32;
    var phi_19398_: u32;
    var phi_19400_: u32;
    var phi_19401_: u32;
    var phi_19402_: bool;
    var phi_19403_: bool;
    var phi_19195_: vec4<f32>;
    var phi_19196_: vec4<f32>;
    var phi_19197_: bool;
    var phi_19201_: vec4<f32>;
    var phi_19202_: bool;
    var phi_19107_: bool;
    var phi_19144_: bool;
    var phi_19150_: f32;
    var phi_19160_: vec4<f32>;
    var phi_19161_: vec4<f32>;
    var phi_19162_: bool;
    var phi_19166_: vec4<f32>;
    var phi_19167_: bool;
    var phi_19171_: vec4<f32>;
    var phi_19172_: bool;
    var phi_19203_: vec4<f32>;
    var phi_19204_: bool;
    var phi_19037_: vec4<f32>;
    var phi_19038_: vec4<f32>;
    var phi_19039_: bool;
    var phi_19043_: vec4<f32>;
    var phi_19044_: bool;
    var phi_19045_: vec4<f32>;
    var phi_19046_: bool;
    var phi_19047_: bool;
    var phi_19055_: vec4<f32>;
    var phi_19056_: bool;
    var phi_19057_: vec4<f32>;
    var phi_19058_: bool;
    var phi_19059_: bool;
    var phi_19067_: vec4<f32>;
    var phi_19068_: bool;
    var phi_19213_: vec4<f32>;
    var phi_19214_: bool;
    var phi_19235_: f32;
    var phi_19241_: u32;
    var phi_18870_: u32;
    var phi_18873_: u32;
    var phi_18875_: u32;
    var phi_18877_: u32;
    var phi_18879_: u32;
    var phi_18881_: vec4<f32>;
    var phi_18883_: f32;
    var phi_19414_: vec4<f32>;
    var phi_19415_: bool;
    var phi_19416_: bool;
    var phi_36516_: bool;
    var phi_22053_: bool;
    var phi_22052_: vec4<f32>;
    var local_80: u32;
    var local_81: vec4<f32>;
    var phi_19421_: type_79;
    var local_82: u32;
    var phi_19427_: type_79;
    var phi_19479_: vec3<f32>;
    var phi_19480_: type_41;
    var phi_19481_: vec3<f32>;
    var phi_19482_: type_41;
    var phi_36958_: bool;
    var phi_19483_: vec3<f32>;
    var phi_19484_: type_41;
    var phi_19485_: u32;
    var phi_36957_: bool;
    var phi_19486_: vec3<f32>;
    var phi_19487_: type_41;
    var phi_19488_: u32;
    var phi_37243_: u32;
    var phi_36956_: bool;
    var phi_14906_: vec3<f32>;
    var phi_14909_: type_41;
    var phi_14911_: u32;
    var phi_14913_: u32;
    var phi_36963_: u32;
    var phi_36955_: bool;
    var phi_31303_: bool;
    var phi_31333_: bool;
    var phi_31322_: f32;
    var phi_31253_: bool;
    var phi_31389_: vec3<f32>;
    var phi_31545_: bool;
    var phi_31560_: bool;
    var phi_31575_: bool;
    var phi_31484_: vec3<f32>;
    var phi_31487_: vec3<f32>;
    var phi_31488_: bool;
    var phi_31258_: vec3<f32>;
    var phi_31262_: vec3<f32>;
    var phi_31263_: bool;
    var phi_31675_: bool;
    var phi_31649_: vec3<f32>;
    var phi_31270_: vec3<f32>;
    var phi_31690_: bool;
    var phi_31800_: f32;
    var phi_31828_: f32;
    var phi_31856_: f32;
    var phi_31990_: bool;
    var phi_32005_: bool;
    var phi_32020_: bool;
    var phi_32047_: bool;
    var phi_32062_: bool;
    var phi_32077_: bool;
    var phi_32104_: bool;
    var phi_32119_: bool;
    var phi_32134_: bool;
    var phi_19707_: f32;
    var phi_19715_: f32;
    var phi_19723_: f32;
    var phi_32331_: bool;
    var phi_32320_: f32;
    var phi_19830_: vec3<f32>;
    var phi_32359_: bool;
    var phi_19831_: vec3<f32>;
    var phi_19843_: type_59;
    var phi_19844_: type_59;
    var phi_19845_: bool;
    var phi_19850_: type_59;
    var phi_32396_: bool;
    var phi_32426_: bool;
    var phi_32415_: f32;
    var phi_32465_: vec3<f32>;
    var phi_19878_: type_59;
    var phi_19879_: type_59;
    var phi_32487_: vec3<f32>;
    var phi_19880_: type_59;
    var phi_19881_: type_59;
    var phi_19882_: bool;
    var phi_19886_: type_59;
    var phi_32512_: bool;
    var phi_19962_: vec3<f32>;
    var phi_38797_: u32;
    var phi_19964_: vec3<f32>;
    var phi_19965_: bool;
    var phi_38796_: u32;
    var phi_19966_: vec3<f32>;
    var phi_19967_: bool;
    var phi_38794_: u32;
    var phi_19973_: vec3<f32>;
    var phi_19974_: bool;
    var local_83: u32;
    var local_84: vec3<f32>;
    var local_85: vec3<f32>;
    var local_86: vec3<f32>;
    var local_87: type_41;
    var phi_20003_: vec3<f32>;
    var local_88: u32;
    var local_89: vec3<f32>;
    var local_90: vec3<f32>;
    var local_91: vec3<f32>;
    var local_92: type_41;
    var phi_20019_: type_74;
    var phi_20020_: type_74;
    var phi_38801_: u32;
    var phi_20022_: type_74;
    var phi_20023_: type_74;
    var phi_20024_: bool;
    var phi_38800_: u32;
    var phi_20025_: type_74;
    var phi_20026_: type_74;
    var phi_20027_: bool;
    var phi_20028_: bool;
    var local_93: u32;
    var local_94: vec3<f32>;
    var local_95: vec3<f32>;
    var local_96: vec3<f32>;
    var local_97: type_41;
    var phi_20034_: type_74;
    var local_98: u32;
    var local_99: vec3<f32>;
    var local_100: vec3<f32>;
    var local_101: vec3<f32>;
    var local_102: type_41;
    var phi_20039_: type_74;
    var phi_20040_: type_74;
    var phi_20041_: bool;
    var phi_20045_: type_74;
    var phi_20046_: type_74;
    var phi_20047_: bool;
    var phi_20051_: type_74;
    var phi_20052_: type_74;
    var phi_20053_: bool;
    var phi_13672_: bool;
    var phi_13681_: vec3<f32>;
    var phi_13683_: vec3<f32>;
    var phi_13684_: bool;
    var phi_14102_: f32;
    var phi_14105_: f32;
    var phi_32762_: bool;
    var phi_32743_: f32;
    var phi_14106_: f32;
    var phi_32780_: f32;
    var phi_32783_: f32;
    var phi_14112_: f32;
    var phi_14113_: f32;
    var phi_14114_: f32;
    var phi_14115_: f32;
    var phi_20054_: type_74;
    var phi_20055_: type_74;
    var phi_20056_: bool;
    var phi_11728_: f32;
    var phi_32830_: f32;
    var phi_32833_: f32;
    var phi_11730_: f32;
    var phi_11731_: f32;
    var phi_11732_: f32;
    var phi_32859_: bool;
    var phi_11786_: f32;
    var phi_32877_: f32;
    var phi_32880_: f32;
    var phi_11788_: f32;
    var phi_11789_: f32;
    var phi_32891_: bool;
    var phi_32906_: bool;
    var phi_12258_: vec3<f32>;
    var phi_12259_: vec3<f32>;
    var phi_12260_: bool;
    var phi_12265_: vec3<f32>;
    var phi_12266_: bool;
    var phi_33030_: bool;
    var phi_33045_: bool;
    var phi_12557_: vec3<f32>;
    var phi_12559_: vec3<f32>;
    var phi_12586_: vec3<f32>;
    var phi_12600_: vec3<f32>;
    var phi_20060_: type_74;
    var phi_20061_: type_74;
    var phi_20062_: bool;
    var phi_20066_: type_74;

    switch bitcast<i32>(0u) {
        default: {
            let _e287 = global;
            let _e297 = arrayLength((&global_9.member));
            let _e301 = global_1.member.member_7[2u];
            let _e305 = (_e287.x + (_e287.y * 4194240u));
            let _e310 = atomicLoad((&global_8.member[((_e301 * 4u) + 3u)]));
            if (_e305 >= _e310) {
                phi_11464_ = type_48(0u, type_48().member_1);
            } else {
                let _e315 = global_1.member.member[0u];
                let _e319 = global_1.member.member[1u];
                let _e324 = global_1.member.member_7[3u];
                let _e331 = atomicLoad((&global_8.member[((12u + (_e301 * ((_e315 * _e319) * _e324))) + _e305)]));
                phi_11464_ = type_48(1u, _e331);
            }
            let _e337 = phi_11464_;
            if (bitcast<i32>(_e337.member) != 0i) {
                let _e342 = (_e337.member_1 < arrayLength((&global_7.member)));
                if _e342 {
                } else {
                    break;
                }
                let _e345 = global_7.member[_e337.member_1];
                let _e359 = vec3<f32>(_e345.member_1.x, _e345.member_1.y, _e345.member_1.z);
                let _e364 = vec3<f32>(_e345.member_2.x, _e345.member_2.y, _e345.member_2.z);
                let _e370 = select(select(u32(_e345.member_2.w), 0u, (_e345.member_2.w < 0f)), 4294967295u, (_e345.member_2.w > 4294967000f));
                let _e374 = vec3<f32>(_e345.member_3.x, _e345.member_3.y, _e345.member_3.z);
                let _e384 = ((_e345.member_4.y & 2u) != 0u);
                let _e386 = ((_e345.member_4.y & 32u) != 0u);
                let _e389 = global_1.member.member[3u];
                let _e392 = global_1.member.member_3[3u];
                let _e395 = global_1.member.member_1[2u];
                let _e398 = (select(select(u32(_e345.member_3.w), 0u, (_e345.member_3.w < 0f)), 4294967295u, (_e345.member_3.w > 4294967000f)) + 1u);
                if (_e345.member_4.z == 4294967295u) {
                    if _e384 {
                        phi_11732_ = 1f;
                    } else {
                        if _e386 {
                            phi_11731_ = 1f;
                        } else {
                            if ((_e392 & 2u) == 0u) {
                                phi_11730_ = 1f;
                            } else {
                                let _e6609 = global_1.member.member_3[1u];
                                let _e6612 = global_1.member.member_3[2u];
                                let _e6615 = ((atan2(_e345.member_1.z, _e345.member_1.x) * 0.15915494f) + 0.5f);
                                let _e6619 = select(_e345.member_1.y, -1f, (_e345.member_1.y < -1f));
                                let _e6622 = acos(select(_e6619, 1f, (_e6619 > 1f)));
                                let _e6624 = f32(_e6609);
                                let _e6625 = ((_e6615 - floor(_e6615)) * _e6624);
                                let _e6630 = select(select(u32(_e6625), 0u, (_e6625 < 0f)), 4294967295u, (_e6625 > 4294967000f));
                                let _e6631 = (_e6609 - 1u);
                                let _e6634 = f32(_e6612);
                                let _e6635 = ((_e6622 * 0.31830987f) * _e6634);
                                let _e6640 = select(select(u32(_e6635), 0u, (_e6635 < 0f)), 4294967295u, (_e6635 > 4294967000f));
                                let _e6641 = (_e6612 - 1u);
                                let _e6643 = select(_e6640, _e6641, (_e6641 < _e6640));
                                let _e6644 = sin(_e6622);
                                if (_e6644 <= 0f) {
                                    phi_11728_ = 0f;
                                } else {
                                    let _e6648 = global_1.member.member_2[3u];
                                    let _e6651 = global_1.member.member_3[0u];
                                    let _e6655 = (_e6648 + _e6643);
                                    let _e6658 = global_6.member[(_e6655 + 1u)];
                                    let _e6660 = global_6.member[_e6655];
                                    let _e6663 = ((_e6651 + (_e6643 * (_e6609 + 1u))) + select(_e6630, _e6631, (_e6631 < _e6630)));
                                    let _e6666 = global_6.member[(_e6663 + 1u)];
                                    let _e6668 = global_6.member[_e6663];
                                    phi_11728_ = ((((_e6658 - _e6660) * _e6634) * ((_e6666 - _e6668) * _e6624)) / (19.73921f * _e6644));
                                }
                                let _e6675 = phi_11728_;
                                if (_e345.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32833_ = 1f;
                                } else {
                                    let _e6677 = (_e345.member_1.w * _e345.member_1.w);
                                    let _e6679 = (_e6677 + (_e6675 * _e6675));
                                    if (_e6679 == 0f) {
                                        phi_32830_ = 0f;
                                    } else {
                                        phi_32830_ = (_e6677 / _e6679);
                                    }
                                    let _e6683 = phi_32830_;
                                    phi_32833_ = _e6683;
                                }
                                let _e6685 = phi_32833_;
                                phi_11730_ = _e6685;
                            }
                            let _e6687 = phi_11730_;
                            phi_11731_ = _e6687;
                        }
                        let _e6689 = phi_11731_;
                        phi_11732_ = _e6689;
                    }
                    let _e6691 = phi_11732_;
                    if ((_e392 & 4u) == 0u) {
                        phi_12600_ = _e374;
                    } else {
                        if _e384 {
                            phi_11789_ = 1f;
                        } else {
                            if _e386 {
                                phi_11788_ = 0f;
                            } else {
                                let _e6702 = global_1.member.member_5;
                                let _e6706 = (_e345.member_1.x - _e6702.x);
                                let _e6707 = (_e345.member_1.y - _e6702.y);
                                let _e6708 = (_e345.member_1.z - _e6702.z);
                                let _e6715 = (sqrt((((_e6706 * _e6706) + (_e6707 * _e6707)) + (_e6708 * _e6708))) * 0.5f);
                                if (_e6715 != _e6715) {
                                    phi_32859_ = true;
                                } else {
                                    phi_32859_ = (1f <= _e6715);
                                }
                                let _e6719 = phi_32859_;
                                let _e6725 = global_1.member.member_6[3u];
                                if ((2f * asin(select(_e6715, 1f, _e6719))) >= _e6725) {
                                    phi_11786_ = 0f;
                                } else {
                                    let _e6728 = sin((_e6725 * 0.5f));
                                    phi_11786_ = (1f / ((12.566371f * _e6728) * _e6728));
                                }
                                let _e6733 = phi_11786_;
                                if (_e345.member_1.w >= 300000000000000000000000000000000000000f) {
                                    phi_32880_ = 1f;
                                } else {
                                    let _e6735 = (_e345.member_1.w * _e345.member_1.w);
                                    let _e6737 = (_e6735 + (_e6733 * _e6733));
                                    if (_e6737 == 0f) {
                                        phi_32877_ = 0f;
                                    } else {
                                        phi_32877_ = (_e6735 / _e6737);
                                    }
                                    let _e6741 = phi_32877_;
                                    phi_32880_ = _e6741;
                                }
                                let _e6743 = phi_32880_;
                                phi_11788_ = _e6743;
                            }
                            let _e6745 = phi_11788_;
                            phi_11789_ = _e6745;
                        }
                        let _e6747 = phi_11789_;
                        let _e6749 = global_1.member.member_5;
                        let _e6753 = (_e345.member_1.x - _e6749.x);
                        let _e6754 = (_e345.member_1.y - _e6749.y);
                        let _e6755 = (_e345.member_1.z - _e6749.z);
                        let _e6762 = (sqrt((((_e6753 * _e6753) + (_e6754 * _e6754)) + (_e6755 * _e6755))) * 0.5f);
                        if (_e6762 != _e6762) {
                            phi_32891_ = true;
                        } else {
                            phi_32891_ = (1f <= _e6762);
                        }
                        let _e6766 = phi_32891_;
                        let _e6769 = (2f * asin(select(_e6762, 1f, _e6766)));
                        let _e6773 = global_1.member.member_6[3u];
                        if (_e6769 >= _e6773) {
                            phi_12586_ = vec3<f32>(0f, 0f, 0f);
                        } else {
                            let _e6777 = (1f - pow((_e6769 / _e6773), 2f));
                            if (_e6777 != _e6777) {
                                phi_32906_ = true;
                            } else {
                                phi_32906_ = (0f >= _e6777);
                            }
                            let _e6781 = phi_32906_;
                            let _e6784 = global_1.member.member_6;
                            let _e6790 = global_1.member.member_8[2u];
                            if (_e6790 == 4294967295u) {
                                phi_12559_ = vec3<f32>(1f, 1f, 1f);
                            } else {
                                let _e6793 = global_6.member[_e6790];
                                let _e6796 = global_6.member[(_e6790 + 1u)];
                                let _e6799 = global_6.member[(_e6790 + 2u)];
                                let _e6802 = global_6.member[(_e6790 + 3u)];
                                let _e6805 = global_6.member[(_e6790 + 4u)];
                                let _e6808 = global_6.member[(_e6790 + 5u)];
                                let _e6811 = global_6.member[(_e6790 + 6u)];
                                let _e6814 = global_6.member[(_e6790 + 7u)];
                                let _e6817 = global_6.member[(_e6790 + 8u)];
                                let _e6820 = global_6.member[(_e6790 + 9u)];
                                let _e6823 = global_6.member[(_e6790 + 13u)];
                                let _e6826 = global_6.member[(_e6790 + 14u)];
                                let _e6829 = global_6.member[(_e6790 + 15u)];
                                let _e6832 = global_6.member[(_e6790 + 16u)];
                                let _e6835 = global_6.member[(_e6790 + 18u)];
                                let _e6838 = global_6.member[(_e6790 + 19u)];
                                let _e6841 = global_6.member[(_e6790 + 20u)];
                                let _e6844 = global_6.member[(_e6790 + 29u)];
                                let _e6845 = bitcast<u32>(_e6844);
                                let _e6858 = (((((_e345.member.x * _e345.member.x) + (_e345.member.y * _e345.member.y)) + (_e345.member.z * _e345.member.z)) - (2f * (((_e345.member.x * _e6793) + (_e345.member.y * _e6796)) + (_e345.member.z * _e6799)))) + _e6811);
                                if (_e6858 > 0f) {
                                    let _e6867 = ((((_e345.member.x - _e6793) * _e6814) + ((_e345.member.y - _e6796) * _e6817)) + ((_e345.member.z - _e6799) * _e6820));
                                    let _e6869 = ((_e6867 * _e6867) - _e6858);
                                    if (_e6867 >= 0f) {
                                        phi_12259_ = vec3<f32>();
                                        phi_12260_ = true;
                                    } else {
                                        let _e6871 = (_e6869 <= 0f);
                                        if _e6871 {
                                            phi_12258_ = vec3<f32>();
                                        } else {
                                            let _e6874 = (-(_e6867) - sqrt(_e6869));
                                            phi_12258_ = vec3<f32>((_e345.member.x + (_e6814 * _e6874)), (_e345.member.y + (_e6817 * _e6874)), (_e345.member.z + (_e6820 * _e6874)));
                                        }
                                        let _e6883 = phi_12258_;
                                        phi_12259_ = _e6883;
                                        phi_12260_ = _e6871;
                                    }
                                    let _e6885 = phi_12259_;
                                    let _e6887 = phi_12260_;
                                    phi_12265_ = _e6885;
                                    phi_12266_ = select(true, false, _e6887);
                                } else {
                                    phi_12265_ = vec3<f32>(_e345.member.x, _e345.member.y, _e345.member.z);
                                    phi_12266_ = true;
                                }
                                let _e6890 = phi_12265_;
                                let _e6892 = phi_12266_;
                                if _e6892 {
                                    let _e6894 = (_e6890.x - _e6793);
                                    let _e6896 = (_e6890.y - _e6796);
                                    let _e6898 = (_e6890.z - _e6799);
                                    let _e6904 = sqrt((((_e6894 * _e6894) + (_e6896 * _e6896)) + (_e6898 * _e6898)));
                                    let _e6905 = (1f / _e6904);
                                    let _e6923 = ((((((_e6890.x * _e6890.x) + (_e6890.y * _e6890.y)) + (_e6890.z * _e6890.z)) - (2f * (((_e6890.x * _e6793) + (_e6890.y * _e6796)) + (_e6890.z * _e6799)))) + _e6808) / (_e6904 + _e6802));
                                    if (_e6923 != _e6923) {
                                        phi_33030_ = true;
                                    } else {
                                        phi_33030_ = (0f >= _e6923);
                                    }
                                    let _e6927 = phi_33030_;
                                    let _e6931 = sqrt((select(_e6923, 0f, _e6927) / (_e6805 - _e6802)));
                                    if (_e6931 != _e6931) {
                                        phi_33045_ = true;
                                    } else {
                                        phi_33045_ = (1f <= _e6931);
                                    }
                                    let _e6935 = phi_33045_;
                                    let _e6937 = (select(_e6931, 1f, _e6935) * 63f);
                                    let _e6942 = ((((_e6894 * _e6905) * _e6814) + ((_e6896 * _e6905) * _e6817)) + ((_e6898 * _e6905) * _e6820));
                                    let _e6944 = select(_e6942, -1f, (_e6942 < -1f));
                                    let _e6948 = ((select(_e6944, 1f, (_e6944 > 1f)) + 1f) * 127.5f);
                                    let _e6953 = select(select(u32(_e6937), 0u, (_e6937 < 0f)), 4294967295u, (_e6937 > 4294967000f));
                                    let _e6955 = select(_e6953, 62u, (62u < _e6953));
                                    let _e6960 = select(select(u32(_e6948), 0u, (_e6948 < 0f)), 4294967295u, (_e6948 > 4294967000f));
                                    let _e6962 = select(_e6960, 254u, (254u < _e6960));
                                    let _e6964 = (_e6937 - f32(_e6955));
                                    let _e6966 = (_e6948 - f32(_e6962));
                                    let _e6970 = (_e6845 + (3u * ((_e6955 * 256u) + _e6962)));
                                    let _e6975 = (_e6845 + (3u * (((_e6955 + 1u) * 256u) + _e6962)));
                                    let _e6977 = global_6.member[_e6970];
                                    let _e6980 = global_6.member[(_e6970 + 1u)];
                                    let _e6983 = global_6.member[(_e6970 + 2u)];
                                    let _e6984 = (1f - _e6966);
                                    let _e6990 = global_6.member[(_e6970 + 3u)];
                                    let _e6993 = global_6.member[(_e6970 + 4u)];
                                    let _e6996 = global_6.member[(_e6970 + 5u)];
                                    let _e7003 = (1f - _e6964);
                                    let _e7008 = global_6.member[_e6975];
                                    let _e7011 = global_6.member[(_e6975 + 1u)];
                                    let _e7014 = global_6.member[(_e6975 + 2u)];
                                    let _e7020 = global_6.member[(_e6975 + 3u)];
                                    let _e7023 = global_6.member[(_e6975 + 4u)];
                                    let _e7026 = global_6.member[(_e6975 + 5u)];
                                    phi_12557_ = vec3<f32>(((((_e6977 * _e6984) + (_e6990 * _e6966)) * _e7003) + (((_e7008 * _e6984) + (_e7020 * _e6966)) * _e6964)), ((((_e6980 * _e6984) + (_e6993 * _e6966)) * _e7003) + (((_e7011 * _e6984) + (_e7023 * _e6966)) * _e6964)), ((((_e6983 * _e6984) + (_e6996 * _e6966)) * _e7003) + (((_e7014 * _e6984) + (_e7026 * _e6966)) * _e6964)));
                                } else {
                                    phi_12557_ = vec3<f32>(0f, 0f, 0f);
                                }
                                let _e7041 = phi_12557_;
                                let _e7048 = ((_e6832 * 1.1f) * _e7041.y);
                                phi_12559_ = vec3<f32>(exp(-((((_e6823 * _e7041.x) + _e7048) + (_e6835 * _e7041.z)))), exp(-((((_e6826 * _e7041.x) + _e7048) + (_e6838 * _e7041.z)))), exp(-((((_e6829 * _e7041.x) + _e7048) + (_e6841 * _e7041.z)))));
                            }
                            let _e7067 = phi_12559_;
                            let _e7079 = sin((_e6773 * 0.5f));
                            let _e7082 = (((1f - (0.6f * (1f - sqrt(select(_e6777, 0f, _e6781))))) * 1.25f) / ((12.566371f * _e7079) * _e7079));
                            phi_12586_ = vec3<f32>(((_e6784.x * _e7067.x) * _e7082), ((_e6784.y * _e7067.y) * _e7082), ((_e6784.z * _e7067.z) * _e7082));
                        }
                        let _e7088 = phi_12586_;
                        phi_12600_ = vec3<f32>((_e345.member_3.x + ((_e345.member_2.x * _e7088.x) * _e6747)), (_e345.member_3.y + ((_e345.member_2.y * _e7088.y) * _e6747)), (_e345.member_3.z + ((_e345.member_2.z * _e7088.z) * _e6747)));
                    }
                    let _e7103 = phi_12600_;
                    phi_20060_ = type_74(type_34(_e345.member, _e345.member_1, vec4<f32>(_e345.member_2.x, _e345.member_2.y, _e345.member_2.z, f32(_e370)), vec4<f32>(_e7103.x, _e7103.y, _e7103.z, f32(_e398)), vec4<u32>(_e345.member_4.x, (_e345.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), type_41(vec4<f32>(_e345.member.x, _e345.member.y, _e345.member.z, -1f), vec4<f32>(_e345.member_1.x, _e345.member_1.y, _e345.member_1.z, 0f), vec4<f32>((_e345.member_2.x * _e6691), (_e345.member_2.y * _e6691), (_e345.member_2.z * _e6691), 0f)), type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                    phi_20061_ = type_74();
                    phi_20062_ = true;
                } else {
                    if ((_e392 & 8u) == 0u) {
                        phi_36611_ = false;
                        phi_13134_ = _e364;
                        phi_13135_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                    } else {
                        if (_e345.member_5.x > 100f) {
                            let _e409 = global_1.member.member_8[1u];
                            let _e411 = global_6.member[_e409];
                            let _e414 = global_6.member[(_e409 + 1u)];
                            let _e417 = global_6.member[(_e409 + 2u)];
                            let _e420 = global_6.member[(_e409 + 3u)];
                            let _e423 = global_6.member[(_e409 + 5u)];
                            let _e426 = global_6.member[(_e409 + 6u)];
                            let _e429 = global_6.member[(_e409 + 10u)];
                            let _e432 = global_6.member[(_e409 + 11u)];
                            let _e435 = global_6.member[(_e409 + 13u)];
                            let _e438 = global_6.member[(_e409 + 14u)];
                            let _e441 = global_6.member[(_e409 + 15u)];
                            let _e444 = global_6.member[(_e409 + 16u)];
                            let _e447 = global_6.member[(_e409 + 18u)];
                            let _e450 = global_6.member[(_e409 + 19u)];
                            let _e453 = global_6.member[(_e409 + 20u)];
                            let _e461 = ((((_e345.member.x - _e411) * _e345.member_1.x) + ((_e345.member.y - _e414) * _e345.member_1.y)) + ((_e345.member.z - _e417) * _e345.member_1.z));
                            let _e473 = ((((_e345.member.x * _e345.member.x) + (_e345.member.y * _e345.member.y)) + (_e345.member.z * _e345.member.z)) - (2f * (((_e345.member.x * _e411) + (_e345.member.y * _e414)) + (_e345.member.z * _e417))));
                            let _e474 = (_e473 + _e426);
                            let _e475 = (_e461 * _e461);
                            let _e476 = (_e475 - _e474);
                            if (_e476 < 0f) {
                                phi_26576_ = type_57(0u, type_57().member_1);
                                phi_26577_ = type_57();
                                phi_26578_ = true;
                            } else {
                                if (_e461 < 0f) {
                                    phi_26548_ = -(sqrt(_e476));
                                } else {
                                    phi_26548_ = sqrt(_e476);
                                }
                                let _e483 = phi_26548_;
                                let _e485 = -((_e461 + _e483));
                                let _e486 = (_e485 == 0f);
                                if _e486 {
                                    phi_26571_ = type_57(1u, type_56(0f, 0f));
                                    phi_26572_ = type_57();
                                } else {
                                    let _e487 = (_e474 / _e485);
                                    if (_e485 < _e487) {
                                        phi_26560_ = type_56(_e485, _e487);
                                    } else {
                                        phi_26560_ = type_56(_e487, _e485);
                                    }
                                    let _e492 = phi_26560_;
                                    phi_26571_ = type_57();
                                    phi_26572_ = type_57(1u, _e492);
                                }
                                let _e495 = phi_26571_;
                                let _e497 = phi_26572_;
                                phi_26576_ = _e495;
                                phi_26577_ = _e497;
                                phi_26578_ = _e486;
                            }
                            let _e502 = phi_26576_;
                            let _e504 = phi_26577_;
                            let _e506 = phi_26578_;
                            if _e506 {
                                phi_26582_ = _e502;
                            } else {
                                phi_26582_ = _e504;
                            }
                            let _e508 = phi_26582_;
                            if (bitcast<i32>(_e508.member) != 0i) {
                                phi_26469_ = type_57(0u, _e508.member_1);
                            } else {
                                phi_26469_ = type_57(1u, type_57().member_1);
                            }
                            let _e518 = phi_26469_;
                            if (bitcast<i32>(_e518.member) != 0i) {
                                phi_26516_ = type_57(0u, type_57().member_1);
                            } else {
                                if (_e518.member_1.member_1 <= 0f) {
                                    phi_26512_ = type_57(0u, type_57().member_1);
                                } else {
                                    if (_e518.member_1.member != _e518.member_1.member) {
                                        phi_26620_ = true;
                                    } else {
                                        phi_26620_ = (0f >= _e518.member_1.member);
                                    }
                                    let _e530 = phi_26620_;
                                    let _e531 = select(_e518.member_1.member, 0f, _e530);
                                    let _e532 = (_e473 + _e423);
                                    if (_e532 > 0f) {
                                        let _e534 = (_e475 - _e532);
                                        if (_e534 < 0f) {
                                            phi_26728_ = type_57(0u, type_57().member_1);
                                            phi_26729_ = type_57();
                                            phi_26730_ = true;
                                        } else {
                                            if (_e461 < 0f) {
                                                phi_26700_ = -(sqrt(_e534));
                                            } else {
                                                phi_26700_ = sqrt(_e534);
                                            }
                                            let _e541 = phi_26700_;
                                            let _e543 = -((_e461 + _e541));
                                            let _e544 = (_e543 == 0f);
                                            if _e544 {
                                                phi_26723_ = type_57(1u, type_56(0f, 0f));
                                                phi_26724_ = type_57();
                                            } else {
                                                let _e545 = (_e532 / _e543);
                                                if (_e543 < _e545) {
                                                    phi_26712_ = type_56(_e543, _e545);
                                                } else {
                                                    phi_26712_ = type_56(_e545, _e543);
                                                }
                                                let _e550 = phi_26712_;
                                                phi_26723_ = type_57();
                                                phi_26724_ = type_57(1u, _e550);
                                            }
                                            let _e553 = phi_26723_;
                                            let _e555 = phi_26724_;
                                            phi_26728_ = _e553;
                                            phi_26729_ = _e555;
                                            phi_26730_ = _e544;
                                        }
                                        let _e560 = phi_26728_;
                                        let _e562 = phi_26729_;
                                        let _e564 = phi_26730_;
                                        if _e564 {
                                            phi_26734_ = _e560;
                                        } else {
                                            phi_26734_ = _e562;
                                        }
                                        let _e566 = phi_26734_;
                                        if (bitcast<i32>(_e566.member) != 0i) {
                                            phi_26668_ = select(_e566.member_1.member_1, _e566.member_1.member, (_e566.member_1.member >= 0f));
                                        } else {
                                            phi_26668_ = -1f;
                                        }
                                        let _e577 = phi_26668_;
                                        if (_e577 >= 0f) {
                                            if (_e518.member_1.member_1 != _e518.member_1.member_1) {
                                                phi_26772_ = true;
                                            } else {
                                                phi_26772_ = (_e577 <= _e518.member_1.member_1);
                                            }
                                            let _e582 = phi_26772_;
                                            phi_26497_ = select(_e518.member_1.member_1, _e577, _e582);
                                        } else {
                                            phi_26497_ = _e518.member_1.member_1;
                                        }
                                        let _e585 = phi_26497_;
                                        phi_26499_ = _e585;
                                    } else {
                                        phi_26499_ = _e518.member_1.member_1;
                                    }
                                    let _e587 = phi_26499_;
                                    if (_e587 <= _e531) {
                                        phi_26508_ = type_57(0u, type_57().member_1);
                                    } else {
                                        phi_26508_ = type_57(1u, type_56(_e531, _e587));
                                    }
                                    let _e595 = phi_26508_;
                                    phi_26512_ = _e595;
                                }
                                let _e600 = phi_26512_;
                                phi_26516_ = _e600;
                            }
                            let _e605 = phi_26516_;
                            if (bitcast<i32>(_e605.member) != 0i) {
                                let _e611 = (_e605.member_1.member < _e345.member_5.x);
                                if _e611 {
                                    if (_e605.member_1.member_1 != _e605.member_1.member_1) {
                                        phi_26787_ = true;
                                    } else {
                                        phi_26787_ = (_e345.member_5.x <= _e605.member_1.member_1);
                                    }
                                    let _e617 = phi_26787_;
                                    let _e618 = select(_e605.member_1.member_1, _e345.member_5.x, _e617);
                                    phi_12993_ = vec3<f32>(0f, 0f, 0f);
                                    phi_12996_ = 0u;
                                    loop {
                                        let _e620 = phi_12993_;
                                        let _e622 = phi_12996_;
                                        local_69 = _e620;
                                        local_70 = _e620;
                                        local_71 = _e620;
                                        let _e623 = (_e622 < 32u);
                                        if _e623 {
                                            let _e624 = -(_e461);
                                            if (_e605.member_1.member <= _e618) {
                                            } else {
                                                phi_34475_ = true;
                                                break;
                                            }
                                            let _e627 = select(_e624, _e605.member_1.member, (_e624 < _e605.member_1.member));
                                            let _e629 = select(_e627, _e618, (_e627 > _e618));
                                            let _e630 = (_e629 - _e605.member_1.member);
                                            let _e631 = (_e618 - _e629);
                                            let _e632 = (_e618 - _e605.member_1.member);
                                            let _e633 = (0.000000001f * _e632);
                                            if (_e630 <= _e633) {
                                                let _e708 = (f32(_e622) * 0.03125f);
                                                let _e711 = (f32((_e622 + 1u)) * 0.03125f);
                                                let _e713 = ((_e708 + _e711) * 0.5f);
                                                phi_13097_ = type_56((_e605.member_1.member + ((_e632 * _e713) * _e713)), (_e632 * ((_e711 * _e711) - (_e708 * _e708))));
                                            } else {
                                                if (_e631 <= _e633) {
                                                    let _e691 = (f32((31u - _e622)) * 0.03125f);
                                                    let _e694 = (f32((32u - _e622)) * 0.03125f);
                                                    let _e696 = ((_e691 + _e694) * 0.5f);
                                                    phi_13096_ = type_56((_e618 - ((_e632 * _e696) * _e696)), (_e632 * ((_e694 * _e694) - (_e691 * _e691))));
                                                } else {
                                                    let _e639 = round(((32f * _e630) / (_e630 + _e631)));
                                                    let _e644 = select(select(u32(_e639), 0u, (_e639 < 0f)), 4294967295u, (_e639 > 4294967000f));
                                                    if (_e644 < 1u) {
                                                        phi_13080_ = 1u;
                                                    } else {
                                                        phi_13080_ = select(_e644, 31u, (_e644 > 31u));
                                                    }
                                                    let _e649 = phi_13080_;
                                                    if (_e622 < _e649) {
                                                        let _e670 = ((_e649 - 1u) - _e622);
                                                        let _e672 = f32(_e649);
                                                        let _e673 = (f32(_e670) / _e672);
                                                        let _e676 = (f32((_e670 + 1u)) / _e672);
                                                        let _e678 = ((_e673 + _e676) * 0.5f);
                                                        phi_13095_ = type_56((_e629 - ((_e630 * _e678) * _e678)), (_e630 * ((_e676 * _e676) - (_e673 * _e673))));
                                                    } else {
                                                        let _e652 = (_e622 - _e649);
                                                        let _e654 = f32((32u - _e649));
                                                        let _e655 = (f32(_e652) / _e654);
                                                        let _e658 = (f32((_e652 + 1u)) / _e654);
                                                        let _e660 = ((_e655 + _e658) * 0.5f);
                                                        phi_13095_ = type_56((_e629 + ((_e631 * _e660) * _e660)), (_e631 * ((_e658 * _e658) - (_e655 * _e655))));
                                                    }
                                                    let _e688 = phi_13095_;
                                                    phi_13096_ = _e688;
                                                }
                                                let _e706 = phi_13096_;
                                                phi_13097_ = _e706;
                                            }
                                            let _e723 = phi_13097_;
                                            let _e729 = (_e345.member.x + (_e345.member_1.x * _e723.member));
                                            let _e730 = (_e345.member.y + (_e345.member_1.y * _e723.member));
                                            let _e731 = (_e345.member.z + (_e345.member_1.z * _e723.member));
                                            let _e745 = (_e729 - _e411);
                                            let _e746 = (_e730 - _e414);
                                            let _e747 = (_e731 - _e417);
                                            let _e755 = ((((((_e729 * _e729) + (_e730 * _e730)) + (_e731 * _e731)) - (2f * (((_e729 * _e411) + (_e730 * _e414)) + (_e731 * _e417)))) + _e423) / (sqrt((((_e745 * _e745) + (_e746 * _e746)) + (_e747 * _e747))) + _e420));
                                            let _e759 = (1f - (abs((_e755 - 25000f)) * 0.00006666667f));
                                            if (_e759 != _e759) {
                                                phi_26986_ = true;
                                            } else {
                                                phi_26986_ = (0f >= _e759);
                                            }
                                            let _e763 = phi_26986_;
                                            let _e765 = -(_e755);
                                            phi_12994_ = (_e620 + vec3<f32>((exp((_e765 / _e429)) * _e723.member_1), (exp((_e765 / _e432)) * _e723.member_1), (select(_e759, 0f, _e763) * _e723.member_1)));
                                            phi_12997_ = (_e622 + 1u);
                                        } else {
                                            phi_12994_ = vec3<f32>();
                                            phi_12997_ = u32();
                                        }
                                        let _e777 = phi_12994_;
                                        let _e779 = phi_12997_;
                                        phi_12993_ = _e777;
                                        phi_12996_ = _e779;
                                        phi_34475_ = false;
                                        if !(_e623) {
                                            break;
                                        }
                                    }
                                    let _e782 = phi_34475_;
                                    if _e782 {
                                        break;
                                    }
                                    let _e784 = local_69;
                                    let _e791 = local_70;
                                    let _e793 = ((_e444 * 1.1f) * _e791.y);
                                    let _e798 = local_71;
                                    phi_36615_ = _e782;
                                    phi_13122_ = vec3<f32>(exp(-((((_e435 * _e784.x) + _e793) + (_e447 * _e798.z)))), exp(-((((_e438 * _e784.x) + _e793) + (_e450 * _e798.z)))), exp(-((((_e441 * _e784.x) + _e793) + (_e453 * _e798.z)))));
                                } else {
                                    phi_36615_ = false;
                                    phi_13122_ = vec3<f32>();
                                }
                                let _e814 = phi_36615_;
                                let _e816 = phi_13122_;
                                phi_36614_ = _e814;
                                phi_13124_ = _e816;
                                phi_13125_ = select(true, false, _e611);
                            } else {
                                phi_36614_ = false;
                                phi_13124_ = vec3<f32>();
                                phi_13125_ = true;
                            }
                            let _e819 = phi_36614_;
                            let _e821 = phi_13124_;
                            let _e823 = phi_13125_;
                            phi_36612_ = _e819;
                            phi_13132_ = (_e364 * select(_e821, vec3<f32>(1f, 1f, 1f), vec3(_e823)));
                            phi_13133_ = type_41(vec4<f32>(_e345.member.x, _e345.member.y, _e345.member.z, _e345.member_5.x), vec4<f32>(_e345.member_1.x, _e345.member_1.y, _e345.member_1.z, 0f), vec4<f32>(_e345.member_2.x, _e345.member_2.y, _e345.member_2.z, 0f));
                        } else {
                            phi_36612_ = false;
                            phi_13132_ = _e364;
                            phi_13133_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                        }
                        let _e828 = phi_36612_;
                        let _e830 = phi_13132_;
                        let _e832 = phi_13133_;
                        phi_36611_ = _e828;
                        phi_13134_ = _e830;
                        phi_13135_ = _e832;
                    }
                    let _e834 = phi_36611_;
                    let _e836 = phi_13134_;
                    let _e838 = phi_13135_;
                    let _e842 = (_e345.member.x + (_e345.member_1.x * _e345.member_5.x));
                    let _e843 = (_e345.member.y + (_e345.member_1.y * _e345.member_5.x));
                    let _e844 = (_e345.member.z + (_e345.member_1.z * _e345.member_5.x));
                    let _e847 = global_1.member.member_1[0u];
                    let _e850 = global_1.member.member_1[1u];
                    let _e851 = (_e847 + _e850);
                    let _e852 = (3u * _e345.member_4.z);
                    let _e854 = global_2.member[_e852];
                    let _e857 = global_2.member[(_e852 + 1u)];
                    let _e863 = select(select(u32(_e857.w), 0u, (_e857.w < 0f)), 4294967295u, (_e857.w > 4294967000f));
                    let _e864 = (3u * _e851);
                    let _e866 = (_e864 + (5u * _e345.member_4.z));
                    let _e868 = global_2.member[_e866];
                    let _e871 = global_2.member[(_e866 + 1u)];
                    let _e874 = global_2.member[(_e866 + 2u)];
                    let _e886 = global_2.member[(_e866 + 3u)];
                    let _e889 = global_2.member[(_e866 + 4u)];
                    let _e896 = (_e863 == 0u);
                    if _e896 {
                        let _e919 = (_e868.x - _e854.x);
                        let _e920 = (_e868.y - _e854.y);
                        let _e921 = (_e868.z - _e854.z);
                        let _e922 = (_e871.x - _e854.x);
                        let _e923 = (_e871.y - _e854.y);
                        let _e924 = (_e871.z - _e854.z);
                        let _e927 = ((_e920 * _e924) - (_e923 * _e921));
                        let _e930 = ((_e921 * _e922) - (_e924 * _e919));
                        let _e933 = ((_e919 * _e923) - (_e922 * _e920));
                        let _e940 = (1f / sqrt((((_e927 * _e927) + (_e930 * _e930)) + (_e933 * _e933))));
                        let _e944 = vec3<f32>((_e927 * _e940), (_e930 * _e940), (_e933 * _e940));
                        if (select(select(u32(_e874.w), 0u, (_e874.w < 0f)), 4294967295u, (_e874.w > 4294967000f)) == 1u) {
                            let _e949 = ((1f - _e345.member_5.y) - _e345.member_5.z);
                            let _e968 = (((_e874.x * _e949) + (_e886.x * _e345.member_5.y)) + (_e889.x * _e345.member_5.z));
                            let _e969 = (((_e874.y * _e949) + (_e886.y * _e345.member_5.y)) + (_e889.y * _e345.member_5.z));
                            let _e970 = (((_e874.z * _e949) + (_e886.z * _e345.member_5.y)) + (_e889.z * _e345.member_5.z));
                            let _e975 = (((_e968 * _e968) + (_e969 * _e969)) + (_e970 * _e970));
                            if (_e975 > 0f) {
                                let _e978 = (1f / sqrt(_e975));
                                phi_13381_ = vec3<f32>((_e968 * _e978), (_e969 * _e978), (_e970 * _e978));
                            } else {
                                phi_13381_ = _e944;
                            }
                            let _e984 = phi_13381_;
                            phi_13382_ = _e984;
                        } else {
                            phi_13382_ = _e944;
                        }
                        let _e986 = phi_13382_;
                        phi_13415_ = type_60(_e944, _e986);
                    } else {
                        if (_e863 == 1u) {
                            let _e912 = vec3<f32>(((_e842 - _e854.x) / _e854.w), ((_e843 - _e854.y) / _e854.w), ((_e844 - _e854.z) / _e854.w));
                            phi_13414_ = type_60(_e912, _e912);
                        } else {
                            let _e900 = select(vec3<f32>(0f, 1f, 0f), vec3<f32>(-0f, -1f, -0f), vec3((_e345.member_1.y > 0f)));
                            phi_13414_ = type_60(_e900, _e900);
                        }
                        let _e915 = phi_13414_;
                        phi_13415_ = _e915;
                    }
                    let _e989 = phi_13415_;
                    let _e1003 = select(select(u32(_e871.w), 0u, (_e871.w < 0f)), 4294967295u, (_e871.w > 4294967000f));
                    phi_13538_ = 0i;
                    phi_13541_ = select(select(u32(_e868.w), 0u, (_e868.w < 0f)), 4294967295u, (_e868.w > 4294967000f));
                    loop {
                        let _e1005 = phi_13538_;
                        let _e1007 = phi_13541_;
                        local_72 = _e1007;
                        if (_e1005 < 4i) {
                            let _e1010 = global_4.member[_e1007];
                            let _e1012 = (_e1010.member != 6u);
                            if _e1012 {
                                phi_13610_ = i32();
                                phi_13611_ = u32();
                            } else {
                                let _e1016 = (abs(_e842) / _e1010.member_6.z);
                                let _e1027 = (abs(_e844) / _e1010.member_6.w);
                                if ((select(select(u32(_e1016), 0u, (_e1016 < 0f)), 4294967295u, (_e1016 > 4294967000f)) % 2u) == 0u) {
                                    phi_13595_ = (_e842 > 0f);
                                } else {
                                    phi_13595_ = (_e842 <= 0f);
                                }
                                let _e1038 = phi_13595_;
                                if ((select(select(u32(_e1027), 0u, (_e1027 < 0f)), 4294967295u, (_e1027 > 4294967000f)) % 2u) == 0u) {
                                    phi_13601_ = (_e844 > 0f);
                                } else {
                                    phi_13601_ = (_e844 <= 0f);
                                }
                                let _e1042 = phi_13601_;
                                if (_e1038 != _e1042) {
                                    phi_13608_ = _e1010.member_1;
                                } else {
                                    phi_13608_ = _e1010.member_2;
                                }
                                let _e1047 = phi_13608_;
                                phi_13610_ = (_e1005 + 1i);
                                phi_13611_ = _e1047;
                            }
                            let _e1050 = phi_13610_;
                            let _e1052 = phi_13611_;
                            phi_13539_ = _e1050;
                            phi_13542_ = _e1052;
                            phi_13613_ = select(true, false, _e1012);
                        } else {
                            phi_13539_ = i32();
                            phi_13542_ = u32();
                            phi_13613_ = false;
                        }
                        let _e1055 = phi_13539_;
                        let _e1057 = phi_13542_;
                        let _e1059 = phi_13613_;
                        phi_13538_ = _e1055;
                        phi_13541_ = _e1057;
                        if !(_e1059) {
                            break;
                        }
                    }
                    let _e1062 = local_72;
                    let _e1064 = global_4.member[_e1062];
                    let _e1066 = (_e1064.member == 4u);
                    if _e1066 {
                        if _e1066 {
                            if (_e1064.member_1 == 1u) {
                                phi_13672_ = true;
                            } else {
                                phi_13672_ = ((((-(_e345.member_1.x) * _e989.member_1.x) + (-(_e345.member_1.y) * _e989.member_1.y)) + (-(_e345.member_1.z) * _e989.member_1.z)) > 0f);
                            }
                            let _e6421 = phi_13672_;
                            if _e6421 {
                                phi_13681_ = vec3<f32>(_e1064.member_4.x, _e1064.member_4.y, _e1064.member_4.z);
                            } else {
                                phi_13681_ = vec3<f32>();
                            }
                            let _e6428 = phi_13681_;
                            phi_13683_ = _e6428;
                            phi_13684_ = select(true, false, _e6421);
                        } else {
                            phi_13683_ = vec3<f32>();
                            phi_13684_ = true;
                        }
                        let _e6431 = phi_13683_;
                        let _e6433 = phi_13684_;
                        let _e6435 = select(_e6431, vec3<f32>(0f, 0f, 0f), vec3(_e6433));
                        if _e384 {
                            phi_14115_ = 1f;
                        } else {
                            if ((_e392 & 1u) == 0u) {
                                phi_14114_ = 1f;
                            } else {
                                if (_e1003 > 0u) {
                                    let _e6442 = global_1.member.member_2[1u];
                                    let _e6443 = (_e6442 + (_e1003 - 1u));
                                    let _e6446 = global_6.member[(_e6443 + 1u)];
                                    let _e6448 = global_6.member[_e6443];
                                    if (_e863 == 1u) {
                                        let _e6509 = (_e854.x - _e345.member.x);
                                        let _e6510 = (_e854.y - _e345.member.y);
                                        let _e6511 = (_e854.z - _e345.member.z);
                                        let _e6516 = (((_e6509 * _e6509) + (_e6510 * _e6510)) + (_e6511 * _e6511));
                                        let _e6517 = (_e854.w * _e854.w);
                                        if (_e6516 <= (_e6517 * 1.000001f)) {
                                            let _e6531 = (_e842 - _e345.member.x);
                                            let _e6532 = (_e843 - _e345.member.y);
                                            let _e6533 = (_e844 - _e345.member.z);
                                            let _e6538 = (((_e6531 * _e6531) + (_e6532 * _e6532)) + (_e6533 * _e6533));
                                            phi_32743_ = (_e6538 / (((abs((((_e989.member.x * _e6531) + (_e989.member.y * _e6532)) + (_e989.member.z * _e6533))) / sqrt(_e6538)) * 12.566371f) * _e6517));
                                        } else {
                                            let _e6520 = (_e6517 / _e6516);
                                            if (_e6520 != _e6520) {
                                                phi_32762_ = true;
                                            } else {
                                                phi_32762_ = (1f <= _e6520);
                                            }
                                            let _e6524 = phi_32762_;
                                            let _e6525 = select(_e6520, 1f, _e6524);
                                            phi_32743_ = (0.15915494f / (_e6525 / (1f + sqrt((1f - _e6525)))));
                                        }
                                        let _e6554 = phi_32743_;
                                        phi_14106_ = _e6554;
                                    } else {
                                        let _e6451 = (_e842 - _e345.member.x);
                                        let _e6452 = (_e843 - _e345.member.y);
                                        let _e6453 = (_e844 - _e345.member.z);
                                        let _e6458 = (((_e6451 * _e6451) + (_e6452 * _e6452)) + (_e6453 * _e6453));
                                        let _e6469 = (abs((((_e989.member.x * _e6451) + (_e989.member.y * _e6452)) + (_e989.member.z * _e6453))) / sqrt(_e6458));
                                        if (_e6469 <= 0f) {
                                            phi_14105_ = 0f;
                                        } else {
                                            if _e896 {
                                                let _e6477 = (_e868.x - _e854.x);
                                                let _e6478 = (_e868.y - _e854.y);
                                                let _e6479 = (_e868.z - _e854.z);
                                                let _e6480 = (_e871.x - _e854.x);
                                                let _e6481 = (_e871.y - _e854.y);
                                                let _e6482 = (_e871.z - _e854.z);
                                                let _e6485 = ((_e6478 * _e6482) - (_e6481 * _e6479));
                                                let _e6488 = ((_e6479 * _e6480) - (_e6482 * _e6477));
                                                let _e6491 = ((_e6477 * _e6481) - (_e6480 * _e6478));
                                                phi_14102_ = (sqrt((((_e6485 * _e6485) + (_e6488 * _e6488)) + (_e6491 * _e6491))) * 0.5f);
                                            } else {
                                                phi_14102_ = ((12.566371f * _e854.w) * _e854.w);
                                            }
                                            let _e6500 = phi_14102_;
                                            phi_14105_ = (_e6458 / (_e6469 * _e6500));
                                        }
                                        let _e6504 = phi_14105_;
                                        phi_14106_ = _e6504;
                                    }
                                    let _e6556 = phi_14106_;
                                    let _e6557 = ((_e6446 - _e6448) * _e6556);
                                    if _e386 {
                                        phi_14112_ = 0f;
                                    } else {
                                        if (_e345.member_1.w >= 300000000000000000000000000000000000000f) {
                                            phi_32783_ = 1f;
                                        } else {
                                            let _e6559 = (_e345.member_1.w * _e345.member_1.w);
                                            let _e6561 = (_e6559 + (_e6557 * _e6557));
                                            if (_e6561 == 0f) {
                                                phi_32780_ = 0f;
                                            } else {
                                                phi_32780_ = (_e6559 / _e6561);
                                            }
                                            let _e6565 = phi_32780_;
                                            phi_32783_ = _e6565;
                                        }
                                        let _e6567 = phi_32783_;
                                        phi_14112_ = _e6567;
                                    }
                                    let _e6569 = phi_14112_;
                                    phi_14113_ = _e6569;
                                } else {
                                    phi_14113_ = 1f;
                                }
                                let _e6571 = phi_14113_;
                                phi_14114_ = _e6571;
                            }
                            let _e6573 = phi_14114_;
                            phi_14115_ = _e6573;
                        }
                        let _e6575 = phi_14115_;
                        phi_20054_ = type_74(type_34(_e345.member, _e345.member_1, vec4<f32>(_e836.x, _e836.y, _e836.z, f32(_e370)), vec4<f32>((_e345.member_3.x + ((_e836.x * _e6435.x) * _e6575)), (_e345.member_3.y + ((_e836.y * _e6435.y) * _e6575)), (_e345.member_3.z + ((_e836.z * _e6435.z) * _e6575)), f32(_e398)), vec4<u32>(_e345.member_4.x, (_e345.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                        phi_20055_ = type_74();
                        phi_20056_ = true;
                    } else {
                        if (_e1064.member == 7u) {
                            let _e6379 = type_34(_e345.member, _e345.member_1, vec4<f32>(_e836.x, _e836.y, _e836.z, f32(_e370)), vec4<f32>(_e345.member_3.x, _e345.member_3.y, _e345.member_3.z, f32(_e398)), vec4<u32>(_e345.member_4.x, ((_e345.member_4.y | 16u) & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f));
                            let _e6391 = type_34(_e6379.member, _e6379.member_1, _e6379.member_2, _e6379.member_3, vec4<u32>(_e6379.member_4.x, _e6379.member_4.y, _e345.member_4.z, _e6379.member_4.w), _e6379.member_5);
                            phi_20051_ = type_74(type_34(_e6391.member, _e6391.member_1, _e6391.member_2, _e6391.member_3, _e6391.member_4, _e345.member_5), _e838, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                            phi_20052_ = type_74();
                            phi_20053_ = true;
                        } else {
                            let _e1068 = -(_e345.member_1.x);
                            let _e1069 = -(_e345.member_1.y);
                            let _e1070 = -(_e345.member_1.z);
                            if ((((_e989.member.x * _e1068) + (_e989.member.y * _e1069)) + (_e989.member.z * _e1070)) < 0f) {
                                phi_14167_ = -(_e989.member);
                            } else {
                                phi_14167_ = _e989.member;
                            }
                            let _e1082 = phi_14167_;
                            if ((((_e989.member_1.x * _e1068) + (_e989.member_1.y * _e1069)) + (_e989.member_1.z * _e1070)) < 0f) {
                                phi_14181_ = -(_e989.member_1);
                            } else {
                                phi_14181_ = _e989.member_1;
                            }
                            let _e1094 = phi_14181_;
                            let _e1097 = global_1.member.member_4[2u];
                            let _e1098 = abs(_e842);
                            let _e1099 = abs(_e843);
                            let _e1100 = abs(_e844);
                            let _e1102 = select(_e1099, _e1098, (_e1098 > _e1099));
                            let _e1106 = (0.00002f * (1f + select(_e1100, _e1102, (_e1102 > _e1100))));
                            if (_e1097 != _e1097) {
                                phi_27053_ = true;
                            } else {
                                phi_27053_ = (_e1106 >= _e1097);
                            }
                            let _e1110 = phi_27053_;
                            let _e1112 = (select(_e1097, _e1106, _e1110) * 2f);
                            let _e1119 = (_e842 + (_e1082.x * _e1112));
                            let _e1120 = (_e843 + (_e1082.y * _e1112));
                            let _e1121 = (_e844 + (_e1082.z * _e1112));
                            let _e1122 = vec3<f32>(_e1119, _e1120, _e1121);
                            if ((_e345.member_4.y & 4u) == 0u) {
                                phi_14214_ = 0f;
                            } else {
                                let _e1127 = global_1.member.member_4[0u];
                                phi_14214_ = _e1127;
                            }
                            let _e1129 = phi_14214_;
                            let _e1132 = select(-1f, 1f, (_e1094.z >= 0f));
                            let _e1134 = (-1f / (_e1132 + _e1094.z));
                            let _e1138 = ((_e1094.x * _e1094.y) * _e1134);
                            let _e1142 = (1f + (((_e1132 * _e1094.x) * _e1094.x) * _e1134));
                            let _e1143 = (_e1132 * _e1138);
                            let _e1145 = (-(_e1132) * _e1094.x);
                            let _e1148 = (_e1132 + ((_e1094.y * _e1094.y) * _e1134));
                            let _e1149 = -(_e1094.y);
                            if (_e1064.member == 0u) {
                                phi_35259_ = vec3<f32>(0f, 0f, 0f);
                                phi_35067_ = 1.5f;
                                phi_34988_ = vec3<f32>(0f, 0f, 0f);
                                phi_34911_ = vec3<f32>(_e1064.member_4.x, _e1064.member_4.y, _e1064.member_4.z);
                                phi_34832_ = 0u;
                                phi_34750_ = 1f;
                                phi_34673_ = 0u;
                                phi_34664_ = _e345.member_4.x;
                                phi_14893_ = type_74();
                                phi_14894_ = true;
                            } else {
                                if (_e1064.member == 1u) {
                                    let _e1698 = (_e1064.member_6.y * _e1064.member_6.y);
                                    if (_e1698 != _e1698) {
                                        phi_27745_ = true;
                                    } else {
                                        phi_27745_ = (_e1129 >= _e1698);
                                    }
                                    let _e1702 = phi_27745_;
                                    let _e1703 = select(_e1698, _e1129, _e1702);
                                    let _e1704 = (_e1703 >= 0.001f);
                                    if _e1704 {
                                        phi_35316_ = vec3<f32>(_e1064.member_4.x, _e1064.member_4.y, _e1064.member_4.z);
                                        phi_35110_ = _e1064.member_6.x;
                                        phi_34963_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34651_ = _e345.member_4.x;
                                        phi_14289_ = vec3<f32>();
                                        phi_14290_ = true;
                                    } else {
                                        let _e1706 = ((_e345.member_4.x * 747796405u) + 2891336453u);
                                        let _e1714 = (((_e1706 >> bitcast<u32>((((_e1706 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1706) * 277803737u);
                                        let _e1717 = ((_e1714 >> bitcast<u32>(22i)) ^ _e1714);
                                        let _e1727 = abs((((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z)));
                                        let _e1730 = (1f / _e1064.member_6.x);
                                        let _e1734 = ((_e1730 * _e1730) * (1f - (_e1727 * _e1727)));
                                        if (_e1734 >= 1f) {
                                            phi_27785_ = 1f;
                                        } else {
                                            let _e1737 = sqrt((1f - _e1734));
                                            if (_e1727 != _e1727) {
                                                phi_27796_ = true;
                                            } else {
                                                phi_27796_ = (_e1737 <= _e1727);
                                            }
                                            let _e1741 = phi_27796_;
                                            let _e1746 = pow(((1f - _e1064.member_6.x) / (1f + _e1064.member_6.x)), 2f);
                                            phi_27785_ = (_e1746 + ((1f - _e1746) * pow((1f - select(_e1727, _e1737, _e1741)), 5f)));
                                        }
                                        let _e1753 = phi_27785_;
                                        let _e1754 = ((f32((_e1717 >> bitcast<u32>(8i))) * 0.000000059604645f) < _e1753);
                                        if _e1754 {
                                            let _e1767 = (((_e1094.x * _e345.member_1.x) + (_e1094.y * _e345.member_1.y)) + (_e1094.z * _e345.member_1.z));
                                            phi_34964_ = vec3<f32>(0f, 0f, 0f);
                                            phi_14287_ = (_e359 - vec3<f32>(((_e1094.x * 2f) * _e1767), ((_e1094.y * 2f) * _e1767), ((_e1094.z * 2f) * _e1767)));
                                        } else {
                                            phi_34964_ = vec3<f32>(_e1064.member_4.x, _e1064.member_4.y, _e1064.member_4.z);
                                            phi_14287_ = vec3<f32>();
                                        }
                                        let _e1774 = phi_34964_;
                                        let _e1776 = phi_14287_;
                                        phi_35316_ = vec3<f32>(0f, 0f, 0f);
                                        phi_35110_ = 1.5f;
                                        phi_34963_ = _e1774;
                                        phi_34651_ = _e1717;
                                        phi_14289_ = _e1776;
                                        phi_14290_ = select(true, false, _e1754);
                                    }
                                    let _e1786 = phi_35316_;
                                    let _e1788 = phi_35110_;
                                    let _e1790 = phi_34963_;
                                    let _e1792 = phi_34651_;
                                    let _e1794 = phi_14289_;
                                    let _e1796 = phi_14290_;
                                    phi_35262_ = _e1786;
                                    phi_35070_ = _e1788;
                                    phi_34991_ = vec3<f32>(0f, 0f, 0f);
                                    phi_34914_ = _e1790;
                                    phi_34835_ = select(0u, 1u, _e1704);
                                    phi_34753_ = select(1f, _e1703, _e1704);
                                    phi_34676_ = select(0u, 2u, _e1704);
                                    phi_34602_ = _e1792;
                                    phi_14762_ = _e1122;
                                    phi_14763_ = _e1794;
                                    phi_14764_ = vec3<f32>(1f, 1f, 1f);
                                    phi_14765_ = _e1796;
                                } else {
                                    if (_e1064.member == 3u) {
                                        phi_34605_ = _e345.member_4.x;
                                        phi_14441_ = vec3<f32>();
                                        phi_14442_ = vec3<f32>();
                                        phi_14443_ = vec3<f32>();
                                        phi_14444_ = true;
                                        phi_14445_ = false;
                                    } else {
                                        let _e1153 = (_e1064.member == 5u);
                                        if _e1153 {
                                            phi_34606_ = _e345.member_4.x;
                                            phi_14437_ = vec3<f32>();
                                            phi_14438_ = vec3<f32>();
                                            phi_14439_ = vec3<f32>();
                                            phi_14440_ = false;
                                        } else {
                                            let _e1154 = (_e1064.member == 2u);
                                            if _e1154 {
                                                let _e1159 = (((_e345.member_1.x * _e989.member_1.x) + (_e345.member_1.y * _e989.member_1.y)) + (_e345.member_1.z * _e989.member_1.z));
                                                let _e1160 = (_e1159 > 0f);
                                                if _e1160 {
                                                    phi_14322_ = -(_e989.member_1);
                                                } else {
                                                    phi_14322_ = _e989.member_1;
                                                }
                                                let _e1163 = phi_14322_;
                                                if _e1160 {
                                                    phi_14328_ = _e1064.member_6.x;
                                                } else {
                                                    phi_14328_ = (1f / _e1064.member_6.x);
                                                }
                                                let _e1168 = phi_14328_;
                                                let _e1169 = abs(_e1159);
                                                let _e1170 = (_e1168 * _e1168);
                                                let _e1173 = (_e1170 * (1f - (_e1169 * _e1169)));
                                                if (_e1173 >= 1f) {
                                                    phi_27115_ = 1f;
                                                } else {
                                                    let _e1176 = sqrt((1f - _e1173));
                                                    if (_e1169 != _e1169) {
                                                        phi_27126_ = true;
                                                    } else {
                                                        phi_27126_ = (_e1176 <= _e1169);
                                                    }
                                                    let _e1180 = phi_27126_;
                                                    let _e1185 = pow(((1f - _e1064.member_6.x) / (1f + _e1064.member_6.x)), 2f);
                                                    phi_27115_ = (_e1185 + ((1f - _e1185) * pow((1f - select(_e1169, _e1176, _e1180)), 5f)));
                                                }
                                                let _e1192 = phi_27115_;
                                                let _e1200 = (((_e345.member_1.x * _e1163.x) + (_e345.member_1.y * _e1163.y)) + (_e345.member_1.z * _e1163.z));
                                                let _e1204 = (1f - (_e1170 * (1f - (_e1200 * _e1200))));
                                                if (_e1204 > 0f) {
                                                    let _e1207 = ((_e345.member_4.x * 747796405u) + 2891336453u);
                                                    let _e1215 = (((_e1207 >> bitcast<u32>((((_e1207 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1207) * 277803737u);
                                                    let _e1218 = ((_e1215 >> bitcast<u32>(22i)) ^ _e1215);
                                                    let _e1223 = ((f32((_e1218 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1192);
                                                    if _e1223 {
                                                        let _e1233 = sqrt(_e1204);
                                                        let _e1237 = (((_e345.member_1.x - (_e1163.x * _e1200)) * _e1168) - (_e1163.x * _e1233));
                                                        let _e1238 = (((_e345.member_1.y - (_e1163.y * _e1200)) * _e1168) - (_e1163.y * _e1233));
                                                        let _e1239 = (((_e345.member_1.z - (_e1163.z * _e1200)) * _e1168) - (_e1163.z * _e1233));
                                                        let _e1246 = (1f / sqrt((((_e1237 * _e1237) + (_e1238 * _e1238)) + (_e1239 * _e1239))));
                                                        phi_14409_ = vec3<f32>((_e842 - (_e1163.x * _e1112)), (_e843 - (_e1163.y * _e1112)), (_e844 - (_e1163.z * _e1112)));
                                                        phi_14410_ = vec3<f32>((_e1237 * _e1246), (_e1238 * _e1246), (_e1239 * _e1246));
                                                        phi_14411_ = vec3<f32>(_e1064.member_4.x, _e1064.member_4.y, _e1064.member_4.z);
                                                    } else {
                                                        phi_14409_ = vec3<f32>();
                                                        phi_14410_ = vec3<f32>();
                                                        phi_14411_ = vec3<f32>();
                                                    }
                                                    let _e1264 = phi_14409_;
                                                    let _e1266 = phi_14410_;
                                                    let _e1268 = phi_14411_;
                                                    phi_34609_ = _e1218;
                                                    phi_14413_ = _e1264;
                                                    phi_14414_ = _e1266;
                                                    phi_14415_ = _e1268;
                                                    phi_14416_ = select(true, false, _e1223);
                                                } else {
                                                    phi_34609_ = _e345.member_4.x;
                                                    phi_14413_ = vec3<f32>();
                                                    phi_14414_ = vec3<f32>();
                                                    phi_14415_ = vec3<f32>();
                                                    phi_14416_ = true;
                                                }
                                                let _e1271 = phi_34609_;
                                                let _e1273 = phi_14413_;
                                                let _e1275 = phi_14414_;
                                                let _e1277 = phi_14415_;
                                                let _e1279 = phi_14416_;
                                                if _e1279 {
                                                    let _e1287 = (((_e989.member_1.x * _e345.member_1.x) + (_e989.member_1.y * _e345.member_1.y)) + (_e989.member_1.z * _e345.member_1.z));
                                                    phi_14431_ = vec3<f32>((_e842 + (_e1163.x * _e1112)), (_e843 + (_e1163.y * _e1112)), (_e844 + (_e1163.z * _e1112)));
                                                    phi_14432_ = (_e359 - vec3<f32>(((_e989.member_1.x * 2f) * _e1287), ((_e989.member_1.y * 2f) * _e1287), ((_e989.member_1.z * 2f) * _e1287)));
                                                } else {
                                                    phi_14431_ = _e1273;
                                                    phi_14432_ = _e1275;
                                                }
                                                let _e1301 = phi_14431_;
                                                let _e1303 = phi_14432_;
                                                phi_34607_ = _e1271;
                                                phi_14434_ = _e1301;
                                                phi_14435_ = _e1303;
                                                phi_14436_ = select(_e1277, vec3<f32>(1f, 1f, 1f), vec3(_e1279));
                                            } else {
                                                phi_34607_ = _e345.member_4.x;
                                                phi_14434_ = vec3<f32>();
                                                phi_14435_ = vec3<f32>();
                                                phi_14436_ = vec3<f32>();
                                            }
                                            let _e1307 = phi_34607_;
                                            let _e1309 = phi_14434_;
                                            let _e1311 = phi_14435_;
                                            let _e1313 = phi_14436_;
                                            phi_34606_ = _e1307;
                                            phi_14437_ = _e1309;
                                            phi_14438_ = _e1311;
                                            phi_14439_ = _e1313;
                                            phi_14440_ = _e1154;
                                        }
                                        let _e1315 = phi_34606_;
                                        let _e1317 = phi_14437_;
                                        let _e1319 = phi_14438_;
                                        let _e1321 = phi_14439_;
                                        let _e1323 = phi_14440_;
                                        phi_34605_ = _e1315;
                                        phi_14441_ = _e1317;
                                        phi_14442_ = _e1319;
                                        phi_14443_ = _e1321;
                                        phi_14444_ = _e1153;
                                        phi_14445_ = _e1323;
                                    }
                                    let _e1325 = phi_34605_;
                                    let _e1327 = phi_14441_;
                                    let _e1329 = phi_14442_;
                                    let _e1331 = phi_14443_;
                                    let _e1333 = phi_14444_;
                                    let _e1335 = phi_14445_;
                                    if _e1333 {
                                        let _e1338 = (_e1064.member_6.y * _e1064.member_6.y);
                                        if (_e1338 != _e1338) {
                                            phi_27199_ = true;
                                        } else {
                                            phi_27199_ = (_e1129 >= _e1338);
                                        }
                                        let _e1342 = phi_27199_;
                                        let _e1343 = select(_e1338, _e1129, _e1342);
                                        let _e1355 = select(2u, 0u, (_e1064.member == 5u));
                                        let _e1356 = (_e1343 < 0.001f);
                                        if _e1356 {
                                            let _e1364 = (((_e1094.x * _e345.member_1.x) + (_e1094.y * _e345.member_1.y)) + (_e1094.z * _e345.member_1.z));
                                            let _e1375 = abs((((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z)));
                                            if (_e1355 == 0u) {
                                                let _e1657 = (1f - _e1375);
                                                if (_e1657 != _e1657) {
                                                    phi_27730_ = true;
                                                } else {
                                                    phi_27730_ = (0f >= _e1657);
                                                }
                                                let _e1661 = phi_27730_;
                                                let _e1663 = pow(select(_e1657, 0f, _e1661), 5f);
                                                let _e1664 = (1f - _e1663);
                                                phi_14748_ = vec3<f32>(((_e1064.member_4.x * _e1664) + _e1663), ((_e1064.member_4.y * _e1664) + _e1663), ((_e1064.member_4.z * _e1664) + _e1663));
                                            } else {
                                                if (_e1355 == 1u) {
                                                    let _e1639 = (0.44444448f * (1f - (_e1375 * _e1375)));
                                                    if (_e1639 >= 1f) {
                                                        phi_27691_ = 1f;
                                                    } else {
                                                        let _e1642 = sqrt((1f - _e1639));
                                                        if (_e1375 != _e1375) {
                                                            phi_27702_ = true;
                                                        } else {
                                                            phi_27702_ = (_e1642 <= _e1375);
                                                        }
                                                        let _e1646 = phi_27702_;
                                                        phi_27691_ = (0.040000003f + (0.96f * pow((1f - select(_e1375, _e1642, _e1646)), 5f)));
                                                    }
                                                    let _e1653 = phi_27691_;
                                                    phi_14747_ = vec3(_e1653);
                                                } else {
                                                    let _e1379 = select(_e1375, 0f, (_e1375 < 0f));
                                                    let _e1381 = select(_e1379, 1f, (_e1379 > 1f));
                                                    let _e1383 = (1f - (_e1381 * _e1381));
                                                    let _e1390 = ((_e1064.member_4.x * _e1064.member_4.x) - (_e1064.member_5.x * _e1064.member_5.x));
                                                    let _e1391 = ((_e1064.member_4.y * _e1064.member_4.y) - (_e1064.member_5.y * _e1064.member_5.y));
                                                    let _e1392 = ((_e1064.member_4.z * _e1064.member_4.z) - (_e1064.member_5.z * _e1064.member_5.z));
                                                    let _e1394 = (_e1064.member_4.x * (_e1064.member_5.x + _e1064.member_5.x));
                                                    let _e1396 = (_e1064.member_4.y * (_e1064.member_5.y + _e1064.member_5.y));
                                                    let _e1398 = (_e1064.member_4.z * (_e1064.member_5.z + _e1064.member_5.z));
                                                    let _e1405 = ((_e1390 * _e1390) + (_e1394 * _e1394));
                                                    let _e1406 = ((_e1391 * _e1391) + (_e1396 * _e1396));
                                                    let _e1407 = ((_e1392 * _e1392) + (_e1398 * _e1398));
                                                    let _e1412 = ((_e1383 * _e1390) / _e1405);
                                                    let _e1413 = ((_e1383 * _e1391) / _e1406);
                                                    let _e1414 = ((_e1383 * _e1392) / _e1407);
                                                    let _e1415 = (1f - _e1412);
                                                    let _e1416 = (1f - _e1413);
                                                    let _e1417 = (1f - _e1414);
                                                    let _e1419 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e1412, _e1413, _e1414));
                                                    let _e1420 = (_e1383 * _e1394);
                                                    let _e1421 = (_e1383 * _e1396);
                                                    let _e1422 = (_e1383 * _e1398);
                                                    let _e1427 = (vec3<f32>(_e1420, _e1421, _e1422) / vec3<f32>(_e1405, _e1406, _e1407));
                                                    let _e1440 = ((_e1419.x * _e1419.x) + (_e1427.x * _e1427.x));
                                                    let _e1441 = ((_e1419.y * _e1419.y) + (_e1427.y * _e1427.y));
                                                    let _e1442 = ((_e1419.z * _e1419.z) + (_e1427.z * _e1427.z));
                                                    if (_e1440 != _e1440) {
                                                        phi_27361_ = true;
                                                    } else {
                                                        phi_27361_ = (0f >= _e1440);
                                                    }
                                                    let _e1446 = phi_27361_;
                                                    let _e1448 = sqrt(select(_e1440, 0f, _e1446));
                                                    if (_e1441 != _e1441) {
                                                        phi_27376_ = true;
                                                    } else {
                                                        phi_27376_ = (0f >= _e1441);
                                                    }
                                                    let _e1452 = phi_27376_;
                                                    let _e1454 = sqrt(select(_e1441, 0f, _e1452));
                                                    if (_e1442 != _e1442) {
                                                        phi_27391_ = true;
                                                    } else {
                                                        phi_27391_ = (0f >= _e1442);
                                                    }
                                                    let _e1458 = phi_27391_;
                                                    let _e1460 = sqrt(select(_e1442, 0f, _e1458));
                                                    let _e1464 = ((_e1448 + _e1415) * 0.5f);
                                                    let _e1465 = ((_e1454 + _e1416) * 0.5f);
                                                    let _e1466 = ((_e1460 + _e1417) * 0.5f);
                                                    if (_e1464 != _e1464) {
                                                        phi_27418_ = true;
                                                    } else {
                                                        phi_27418_ = (0f >= _e1464);
                                                    }
                                                    let _e1470 = phi_27418_;
                                                    let _e1472 = sqrt(select(_e1464, 0f, _e1470));
                                                    if (_e1465 != _e1465) {
                                                        phi_27433_ = true;
                                                    } else {
                                                        phi_27433_ = (0f >= _e1465);
                                                    }
                                                    let _e1476 = phi_27433_;
                                                    let _e1478 = sqrt(select(_e1465, 0f, _e1476));
                                                    if (_e1466 != _e1466) {
                                                        phi_27448_ = true;
                                                    } else {
                                                        phi_27448_ = (0f >= _e1466);
                                                    }
                                                    let _e1482 = phi_27448_;
                                                    let _e1484 = sqrt(select(_e1466, 0f, _e1482));
                                                    let _e1485 = vec3<f32>(_e1472, _e1478, _e1484);
                                                    let _e1489 = ((_e1448 - _e1415) * 0.5f);
                                                    let _e1490 = ((_e1454 - _e1416) * 0.5f);
                                                    let _e1491 = ((_e1460 - _e1417) * 0.5f);
                                                    if (_e1489 != _e1489) {
                                                        phi_27475_ = true;
                                                    } else {
                                                        phi_27475_ = (0f >= _e1489);
                                                    }
                                                    let _e1495 = phi_27475_;
                                                    let _e1497 = sqrt(select(_e1489, 0f, _e1495));
                                                    if (_e1490 != _e1490) {
                                                        phi_27490_ = true;
                                                    } else {
                                                        phi_27490_ = (0f >= _e1490);
                                                    }
                                                    let _e1501 = phi_27490_;
                                                    let _e1503 = sqrt(select(_e1490, 0f, _e1501));
                                                    if (_e1491 != _e1491) {
                                                        phi_27505_ = true;
                                                    } else {
                                                        phi_27505_ = (0f >= _e1491);
                                                    }
                                                    let _e1507 = phi_27505_;
                                                    let _e1509 = sqrt(select(_e1491, 0f, _e1507));
                                                    if ((_e1420 / _e1405) < 0f) {
                                                        phi_14624_ = -(_e1497);
                                                    } else {
                                                        phi_14624_ = _e1497;
                                                    }
                                                    let _e1513 = phi_14624_;
                                                    if ((_e1421 / _e1406) < 0f) {
                                                        phi_14632_ = -(_e1503);
                                                    } else {
                                                        phi_14632_ = _e1503;
                                                    }
                                                    let _e1517 = phi_14632_;
                                                    if ((_e1422 / _e1407) < 0f) {
                                                        phi_14640_ = -(_e1509);
                                                    } else {
                                                        phi_14640_ = _e1509;
                                                    }
                                                    let _e1521 = phi_14640_;
                                                    let _e1525 = (_e1064.member_5.x * _e1381);
                                                    let _e1526 = (_e1064.member_5.y * _e1381);
                                                    let _e1527 = (_e1064.member_5.z * _e1381);
                                                    let _e1537 = vec3<f32>(((_e1064.member_4.x * _e1472) - (_e1064.member_5.x * _e1513)), ((_e1064.member_4.y * _e1478) - (_e1064.member_5.y * _e1517)), ((_e1064.member_4.z * _e1484) - (_e1064.member_5.z * _e1521)));
                                                    let _e1544 = ((_e1064.member_4.x * _e1513) + (_e1064.member_5.x * _e1472));
                                                    let _e1545 = ((_e1064.member_4.y * _e1517) + (_e1064.member_5.y * _e1478));
                                                    let _e1546 = ((_e1064.member_4.z * _e1521) + (_e1064.member_5.z * _e1484));
                                                    let _e1548 = vec3<f32>((_e1064.member_4.x * _e1381), (_e1064.member_4.y * _e1381), (_e1064.member_4.z * _e1381));
                                                    let _e1549 = (_e1548 - _e1485);
                                                    let _e1550 = (_e1525 - _e1513);
                                                    let _e1551 = (_e1526 - _e1517);
                                                    let _e1552 = (_e1527 - _e1521);
                                                    let _e1565 = (_e1548 + _e1485);
                                                    let _e1566 = (_e1525 + _e1513);
                                                    let _e1567 = (_e1526 + _e1517);
                                                    let _e1568 = (_e1527 + _e1521);
                                                    let _e1578 = ((_e1565.x * _e1565.x) + (_e1566 * _e1566));
                                                    let _e1579 = ((_e1565.y * _e1565.y) + (_e1567 * _e1567));
                                                    let _e1580 = ((_e1565.z * _e1565.z) + (_e1568 * _e1568));
                                                    let _e1590 = vec3(_e1381);
                                                    let _e1591 = (_e1590 - _e1537);
                                                    let _e1592 = -(vec3<f32>(_e1544, _e1545, _e1546));
                                                    let _e1608 = (_e1590 + _e1537);
                                                    let _e1618 = ((_e1608.x * _e1608.x) + (_e1544 * _e1544));
                                                    let _e1619 = ((_e1608.y * _e1608.y) + (_e1545 * _e1545));
                                                    let _e1620 = ((_e1608.z * _e1608.z) + (_e1546 * _e1546));
                                                    phi_14747_ = vec3<f32>((((((_e1549.x * _e1549.x) + (_e1550 * _e1550)) / select(0.000000000000000000000000000001f, _e1578, (_e1578 > 0.000000000000000000000000000001f))) + (((_e1591.x * _e1591.x) + (_e1592.x * _e1592.x)) / select(0.000000000000000000000000000001f, _e1618, (_e1618 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1549.y * _e1549.y) + (_e1551 * _e1551)) / select(0.000000000000000000000000000001f, _e1579, (_e1579 > 0.000000000000000000000000000001f))) + (((_e1591.y * _e1591.y) + (_e1592.y * _e1592.y)) / select(0.000000000000000000000000000001f, _e1619, (_e1619 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e1549.z * _e1549.z) + (_e1552 * _e1552)) / select(0.000000000000000000000000000001f, _e1580, (_e1580 > 0.000000000000000000000000000001f))) + (((_e1591.z * _e1591.z) + (_e1592.z * _e1592.z)) / select(0.000000000000000000000000000001f, _e1620, (_e1620 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                }
                                                let _e1656 = phi_14747_;
                                                phi_14748_ = _e1656;
                                            }
                                            let _e1673 = phi_14748_;
                                            phi_14749_ = (_e359 - vec3<f32>(((_e1094.x * 2f) * _e1364), ((_e1094.y * 2f) * _e1364), ((_e1094.z * 2f) * _e1364)));
                                            phi_14750_ = _e1673;
                                        } else {
                                            phi_14749_ = vec3<f32>();
                                            phi_14750_ = vec3<f32>();
                                        }
                                        let _e1675 = phi_14749_;
                                        let _e1677 = phi_14750_;
                                        phi_34993_ = vec3<f32>(_e1064.member_5.x, _e1064.member_5.y, _e1064.member_5.z);
                                        phi_34916_ = vec3<f32>(_e1064.member_4.x, _e1064.member_4.y, _e1064.member_4.z);
                                        phi_34837_ = _e1355;
                                        phi_34755_ = _e1343;
                                        phi_14752_ = _e1675;
                                        phi_14753_ = _e1677;
                                        phi_14754_ = _e1356;
                                    } else {
                                        phi_34993_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34916_ = vec3<f32>(0f, 0f, 0f);
                                        phi_34837_ = 0u;
                                        phi_34755_ = 1f;
                                        phi_14752_ = _e1329;
                                        phi_14753_ = _e1331;
                                        phi_14754_ = _e1335;
                                    }
                                    let _e1679 = phi_34993_;
                                    let _e1681 = phi_34916_;
                                    let _e1683 = phi_34837_;
                                    let _e1685 = phi_34755_;
                                    let _e1687 = phi_14752_;
                                    let _e1689 = phi_14753_;
                                    let _e1691 = phi_14754_;
                                    phi_35262_ = vec3<f32>(0f, 0f, 0f);
                                    phi_35070_ = 1.5f;
                                    phi_34991_ = _e1679;
                                    phi_34914_ = _e1681;
                                    phi_34835_ = _e1683;
                                    phi_34753_ = _e1685;
                                    phi_34676_ = select(0u, 1u, _e1333);
                                    phi_34602_ = _e1325;
                                    phi_14762_ = select(_e1327, _e1122, vec3(_e1333));
                                    phi_14763_ = _e1687;
                                    phi_14764_ = _e1689;
                                    phi_14765_ = select(true, false, _e1691);
                                }
                                let _e1801 = phi_35262_;
                                let _e1803 = phi_35070_;
                                let _e1805 = phi_34991_;
                                let _e1807 = phi_34914_;
                                let _e1809 = phi_34835_;
                                let _e1811 = phi_34753_;
                                let _e1813 = phi_34676_;
                                let _e1815 = phi_34602_;
                                let _e1817 = phi_14762_;
                                let _e1819 = phi_14763_;
                                let _e1821 = phi_14764_;
                                let _e1823 = phi_14765_;
                                let _e1824 = select(true, false, _e1823);
                                if _e1824 {
                                    if (_e370 >= _e389) {
                                        phi_34661_ = _e1815;
                                        phi_14882_ = _e836;
                                        phi_14883_ = type_74();
                                        phi_14884_ = true;
                                    } else {
                                        let _e1837 = global_1.member.member_4[1u];
                                        if (sqrt((((_e1821.x * _e1821.x) + (_e1821.y * _e1821.y)) + (_e1821.z * _e1821.z))) <= _e1837) {
                                            phi_34656_ = _e1815;
                                            phi_14845_ = _e836;
                                            phi_14846_ = false;
                                        } else {
                                            let _e1841 = global_1.member.member_7[1u];
                                            if (_e1841 == 1u) {
                                                let _e1846 = global_1.member.member_4[3u];
                                                if (f32(_e370) >= _e1846) {
                                                    let _e1849 = select(_e1821.y, _e1821.x, (_e1821.x > _e1821.y));
                                                    let _e1851 = select(_e1821.z, _e1849, (_e1849 > _e1821.z));
                                                    if (_e1851 != _e1851) {
                                                        phi_27862_ = true;
                                                    } else {
                                                        phi_27862_ = (1f <= _e1851);
                                                    }
                                                    let _e1855 = phi_27862_;
                                                    let _e1856 = select(_e1851, 1f, _e1855);
                                                    let _e1858 = ((_e1815 * 747796405u) + 2891336453u);
                                                    let _e1866 = (((_e1858 >> bitcast<u32>((((_e1858 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e1858) * 277803737u);
                                                    let _e1869 = ((_e1866 >> bitcast<u32>(22i)) ^ _e1866);
                                                    let _e1874 = ((f32((_e1869 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e1856);
                                                    if _e1874 {
                                                        phi_14834_ = vec3<f32>();
                                                    } else {
                                                        phi_14834_ = vec3<f32>((_e836.x / _e1856), (_e836.y / _e1856), (_e836.z / _e1856));
                                                    }
                                                    let _e1883 = phi_14834_;
                                                    phi_34659_ = _e1869;
                                                    phi_14836_ = _e1883;
                                                    phi_14837_ = select(true, false, _e1874);
                                                } else {
                                                    phi_34659_ = _e1815;
                                                    phi_14836_ = _e836;
                                                    phi_14837_ = true;
                                                }
                                                let _e1886 = phi_34659_;
                                                let _e1888 = phi_14836_;
                                                let _e1890 = phi_14837_;
                                                phi_34658_ = _e1886;
                                                phi_14838_ = _e1888;
                                                phi_14839_ = _e1890;
                                            } else {
                                                phi_34658_ = _e1815;
                                                phi_14838_ = _e836;
                                                phi_14839_ = true;
                                            }
                                            let _e1892 = phi_34658_;
                                            let _e1894 = phi_14838_;
                                            let _e1896 = phi_14839_;
                                            phi_34656_ = _e1892;
                                            phi_14845_ = select(_e836, _e1894, vec3(_e1896));
                                            phi_14846_ = select(false, true, _e1896);
                                        }
                                        let _e1901 = phi_34656_;
                                        let _e1903 = phi_14845_;
                                        let _e1905 = phi_14846_;
                                        if _e1905 {
                                            let _e1906 = (_e1903 * _e1821);
                                            let _e1917 = (_e1819 * (1f / sqrt((((_e1819.x * _e1819.x) + (_e1819.y * _e1819.y)) + (_e1819.z * _e1819.z)))));
                                            phi_14880_ = type_74(type_34(vec4<f32>(_e1817.x, _e1817.y, _e1817.z, _e345.member.w), vec4<f32>(_e1917.x, _e1917.y, _e1917.z, _e345.member_1.w), vec4<f32>(_e1906.x, _e1906.y, _e1906.z, f32((_e370 + 1u))), vec4<f32>(_e345.member_3.x, _e345.member_3.y, _e345.member_3.z, f32(_e398)), vec4<u32>(_e1901, ((_e345.member_4.y | 2u) & 4294967263u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                        } else {
                                            phi_14880_ = type_74();
                                        }
                                        let _e1941 = phi_14880_;
                                        phi_34661_ = _e1901;
                                        phi_14882_ = _e1903;
                                        phi_14883_ = _e1941;
                                        phi_14884_ = select(true, false, _e1905);
                                    }
                                    let _e1944 = phi_34661_;
                                    let _e1946 = phi_14882_;
                                    let _e1948 = phi_14883_;
                                    let _e1950 = phi_14884_;
                                    if _e1950 {
                                        phi_14890_ = type_74(type_34(_e345.member, _e345.member_1, vec4<f32>(_e1946.x, _e1946.y, _e1946.z, f32(_e370)), vec4<f32>(_e345.member_3.x, _e345.member_3.y, _e345.member_3.z, f32(_e398)), vec4<u32>(_e1944, (_e345.member_4.y & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f)));
                                    } else {
                                        phi_14890_ = _e1948;
                                    }
                                    let _e1963 = phi_14890_;
                                    phi_34665_ = _e1944;
                                    phi_14891_ = _e1963;
                                } else {
                                    phi_34665_ = _e1815;
                                    phi_14891_ = type_74();
                                }
                                let _e1965 = phi_34665_;
                                let _e1967 = phi_14891_;
                                phi_35259_ = _e1801;
                                phi_35067_ = _e1803;
                                phi_34988_ = _e1805;
                                phi_34911_ = _e1807;
                                phi_34832_ = _e1809;
                                phi_34750_ = _e1811;
                                phi_34673_ = _e1813;
                                phi_34664_ = _e1965;
                                phi_14893_ = _e1967;
                                phi_14894_ = select(true, false, _e1824);
                            }
                            let _e1975 = phi_35259_;
                            let _e1977 = phi_35067_;
                            let _e1979 = phi_34988_;
                            let _e1981 = phi_34911_;
                            let _e1983 = phi_34832_;
                            let _e1985 = phi_34750_;
                            let _e1987 = phi_34673_;
                            let _e1989 = phi_34664_;
                            let _e1991 = phi_14893_;
                            let _e1993 = phi_14894_;
                            if _e1993 {
                                let _e1995 = ((_e345.member_4.y | 4u) & 4294967261u);
                                let _e1998 = global_1.member.member_1[3u];
                                phi_36588_ = _e834;
                                phi_34663_ = _e1989;
                                phi_14905_ = _e374;
                                phi_14908_ = type_41(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
                                phi_14910_ = 0u;
                                phi_14912_ = _e398;
                                loop {
                                    let _e2000 = phi_36588_;
                                    let _e2002 = phi_34663_;
                                    let _e2004 = phi_14905_;
                                    let _e2006 = phi_14908_;
                                    let _e2008 = phi_14910_;
                                    let _e2010 = phi_14912_;
                                    local_83 = _e2010;
                                    local_84 = _e2004;
                                    local_85 = _e2004;
                                    local_86 = _e2004;
                                    local_87 = _e2006;
                                    local_88 = _e2010;
                                    local_89 = _e2004;
                                    local_90 = _e2004;
                                    local_91 = _e2004;
                                    local_92 = _e2006;
                                    local_93 = _e2010;
                                    local_94 = _e2004;
                                    local_95 = _e2004;
                                    local_96 = _e2004;
                                    local_97 = _e2006;
                                    local_98 = _e2010;
                                    local_99 = _e2004;
                                    local_100 = _e2004;
                                    local_101 = _e2004;
                                    local_102 = _e2006;
                                    let _e2012 = (_e2008 < (_e1998 + 3u));
                                    if _e2012 {
                                        if (_e2008 < _e1998) {
                                            let _e4009 = global_5.member[_e2008];
                                            let _e4014 = (_e4009.member.x - _e1119);
                                            let _e4015 = (_e4009.member.y - _e1120);
                                            let _e4016 = (_e4009.member.z - _e1121);
                                            let _e4021 = (((_e4014 * _e4014) + (_e4015 * _e4015)) + (_e4016 * _e4016));
                                            let _e4022 = sqrt(_e4021);
                                            let _e4023 = (_e4014 / _e4022);
                                            let _e4024 = (_e4015 / _e4022);
                                            let _e4025 = (_e4016 / _e4022);
                                            if (_e4009.member.w < 0.5f) {
                                                phi_15009_ = 1f;
                                            } else {
                                                let _e4049 = (((_e4009.member_2.x * -(_e4023)) + (_e4009.member_2.y * -(_e4024))) + (_e4009.member_2.z * -(_e4025)));
                                                if (_e4009.member_3.x == _e4009.member_2.w) {
                                                    phi_15008_ = select(1f, 0f, (_e4049 < _e4009.member_3.x));
                                                } else {
                                                    let _e4053 = ((_e4049 - _e4009.member_3.x) / (_e4009.member_2.w - _e4009.member_3.x));
                                                    let _e4055 = select(_e4053, 0f, (_e4053 < 0f));
                                                    let _e4057 = select(_e4055, 1f, (_e4055 > 1f));
                                                    phi_15008_ = ((_e4057 * _e4057) * (3f - (2f * _e4057)));
                                                }
                                                let _e4065 = phi_15008_;
                                                phi_15009_ = _e4065;
                                            }
                                            let _e4067 = phi_15009_;
                                            phi_37005_ = _e2002;
                                            phi_18422_ = 0f;
                                            phi_18423_ = vec3<f32>(_e4023, _e4024, _e4025);
                                            phi_18424_ = vec3<f32>(((_e4009.member_1.x * _e4067) / _e4021), ((_e4009.member_1.y * _e4067) / _e4021), ((_e4009.member_1.z * _e4067) / _e4021));
                                            phi_18425_ = _e4022;
                                        } else {
                                            if (_e2008 == _e1998) {
                                                if ((_e392 & 4u) == 0u) {
                                                    phi_37049_ = _e2002;
                                                    phi_17051_ = 0f;
                                                    phi_17052_ = vec3<f32>(0f, 0f, 0f);
                                                    phi_17053_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    let _e2761 = global_1.member.member_5;
                                                    let _e2769 = global_1.member.member_6[3u];
                                                    let _e2771 = sin((_e2769 * 0.5f));
                                                    let _e2773 = ((12.566371f * _e2771) * _e2771);
                                                    let _e2778 = (((_e2761.x * _e1094.x) + (_e2761.y * _e1094.y)) + (_e2761.z * _e1094.z));
                                                    if (_e2778 != _e2778) {
                                                        phi_28401_ = true;
                                                    } else {
                                                        phi_28401_ = (0f >= _e2778);
                                                    }
                                                    let _e2782 = phi_28401_;
                                                    let _e2784 = (select(_e2778, 0f, _e2782) * 0.31830987f);
                                                    if (_e1987 == 0u) {
                                                        if (_e2778 <= 0f) {
                                                            phi_29198_ = vec3<f32>(0f, 0f, 0f);
                                                        } else {
                                                            phi_29198_ = (_e1981 * (_e2778 * 0.31830987f));
                                                        }
                                                        let _e3260 = phi_29198_;
                                                        phi_15419_ = type_59(_e3260, _e2784);
                                                        phi_15420_ = type_59();
                                                        phi_15421_ = true;
                                                    } else {
                                                        let _e2790 = (((_e1068 * _e1142) + (_e1069 * _e1143)) + (_e1070 * _e1145));
                                                        let _e2795 = (((_e1068 * _e1138) + (_e1069 * _e1148)) + (_e1070 * _e1149));
                                                        let _e2800 = (((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z));
                                                        let _e2805 = (((_e2761.x * _e1142) + (_e2761.y * _e1143)) + (_e2761.z * _e1145));
                                                        let _e2810 = (((_e2761.x * _e1138) + (_e2761.y * _e1148)) + (_e2761.z * _e1149));
                                                        if (_e2800 <= 0f) {
                                                            phi_15383_ = type_59();
                                                            phi_15384_ = true;
                                                        } else {
                                                            let _e2812 = (_e2778 <= 0f);
                                                            if _e2812 {
                                                                phi_15382_ = type_59();
                                                            } else {
                                                                let _e2813 = (_e2790 + _e2805);
                                                                let _e2814 = (_e2795 + _e2810);
                                                                let _e2815 = (_e2800 + _e2778);
                                                                let _e2822 = (1f / sqrt((((_e2813 * _e2813) + (_e2814 * _e2814)) + (_e2815 * _e2815))));
                                                                let _e2825 = (_e2815 * _e2822);
                                                                if (_e2825 <= 0f) {
                                                                    phi_28511_ = 0f;
                                                                } else {
                                                                    let _e2827 = (_e1985 * _e1985);
                                                                    let _e2831 = ((((_e2827 - 1f) * _e2825) * _e2825) + 1f);
                                                                    phi_28511_ = (_e2827 / ((3.1415927f * _e2831) * _e2831));
                                                                }
                                                                let _e2836 = phi_28511_;
                                                                let _e2837 = (_e2800 * _e2800);
                                                                if (_e2837 >= 1f) {
                                                                    phi_28539_ = 0f;
                                                                } else {
                                                                    phi_28539_ = ((sqrt((1f + (((_e1985 * _e1985) * ((_e2790 * _e2790) + (_e2795 * _e2795))) / _e2837))) - 1f) * 0.5f);
                                                                }
                                                                let _e2850 = phi_28539_;
                                                                let _e2851 = (1f + _e2850);
                                                                let _e2852 = (_e2778 * _e2778);
                                                                if (_e2852 >= 1f) {
                                                                    phi_28567_ = 0f;
                                                                } else {
                                                                    phi_28567_ = ((sqrt((1f + (((_e1985 * _e1985) * ((_e2805 * _e2805) + (_e2810 * _e2810))) / _e2852))) - 1f) * 0.5f);
                                                                }
                                                                let _e2865 = phi_28567_;
                                                                let _e2872 = (((_e2790 * (_e2813 * _e2822)) + (_e2795 * (_e2814 * _e2822))) + (_e2800 * _e2825));
                                                                if (_e1983 == 0u) {
                                                                    let _e3167 = (1f - _e2872);
                                                                    if (_e3167 != _e3167) {
                                                                        phi_29070_ = true;
                                                                    } else {
                                                                        phi_29070_ = (0f >= _e3167);
                                                                    }
                                                                    let _e3171 = phi_29070_;
                                                                    let _e3173 = pow(select(_e3167, 0f, _e3171), 5f);
                                                                    let _e3174 = (1f - _e3173);
                                                                    phi_15370_ = vec3<f32>(((_e1981.x * _e3174) + _e3173), ((_e1981.y * _e3174) + _e3173), ((_e1981.z * _e3174) + _e3173));
                                                                } else {
                                                                    if (_e1983 == 1u) {
                                                                        let _e3140 = (1f / _e1977);
                                                                        let _e3144 = ((_e3140 * _e3140) * (1f - (_e2872 * _e2872)));
                                                                        if (_e3144 >= 1f) {
                                                                            phi_29031_ = 1f;
                                                                        } else {
                                                                            let _e3147 = sqrt((1f - _e3144));
                                                                            if (_e2872 != _e2872) {
                                                                                phi_29042_ = true;
                                                                            } else {
                                                                                phi_29042_ = (_e3147 <= _e2872);
                                                                            }
                                                                            let _e3151 = phi_29042_;
                                                                            let _e3156 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                                            phi_29031_ = (_e3156 + ((1f - _e3156) * pow((1f - select(_e2872, _e3147, _e3151)), 5f)));
                                                                        }
                                                                        let _e3163 = phi_29031_;
                                                                        phi_15369_ = vec3(_e3163);
                                                                    } else {
                                                                        let _e2876 = select(_e2872, 0f, (_e2872 < 0f));
                                                                        let _e2878 = select(_e2876, 1f, (_e2876 > 1f));
                                                                        let _e2880 = (1f - (_e2878 * _e2878));
                                                                        let _e2893 = ((_e1981.x * _e1981.x) - (_e1979.x * _e1979.x));
                                                                        let _e2894 = ((_e1981.y * _e1981.y) - (_e1979.y * _e1979.y));
                                                                        let _e2895 = ((_e1981.z * _e1981.z) - (_e1979.z * _e1979.z));
                                                                        let _e2897 = (_e1981.x * (_e1979.x + _e1979.x));
                                                                        let _e2899 = (_e1981.y * (_e1979.y + _e1979.y));
                                                                        let _e2901 = (_e1981.z * (_e1979.z + _e1979.z));
                                                                        let _e2908 = ((_e2893 * _e2893) + (_e2897 * _e2897));
                                                                        let _e2909 = ((_e2894 * _e2894) + (_e2899 * _e2899));
                                                                        let _e2910 = ((_e2895 * _e2895) + (_e2901 * _e2901));
                                                                        let _e2915 = ((_e2880 * _e2893) / _e2908);
                                                                        let _e2916 = ((_e2880 * _e2894) / _e2909);
                                                                        let _e2917 = ((_e2880 * _e2895) / _e2910);
                                                                        let _e2918 = (1f - _e2915);
                                                                        let _e2919 = (1f - _e2916);
                                                                        let _e2920 = (1f - _e2917);
                                                                        let _e2922 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e2915, _e2916, _e2917));
                                                                        let _e2923 = (_e2880 * _e2897);
                                                                        let _e2924 = (_e2880 * _e2899);
                                                                        let _e2925 = (_e2880 * _e2901);
                                                                        let _e2930 = (vec3<f32>(_e2923, _e2924, _e2925) / vec3<f32>(_e2908, _e2909, _e2910));
                                                                        let _e2943 = ((_e2922.x * _e2922.x) + (_e2930.x * _e2930.x));
                                                                        let _e2944 = ((_e2922.y * _e2922.y) + (_e2930.y * _e2930.y));
                                                                        let _e2945 = ((_e2922.z * _e2922.z) + (_e2930.z * _e2930.z));
                                                                        if (_e2943 != _e2943) {
                                                                            phi_28701_ = true;
                                                                        } else {
                                                                            phi_28701_ = (0f >= _e2943);
                                                                        }
                                                                        let _e2949 = phi_28701_;
                                                                        let _e2951 = sqrt(select(_e2943, 0f, _e2949));
                                                                        if (_e2944 != _e2944) {
                                                                            phi_28716_ = true;
                                                                        } else {
                                                                            phi_28716_ = (0f >= _e2944);
                                                                        }
                                                                        let _e2955 = phi_28716_;
                                                                        let _e2957 = sqrt(select(_e2944, 0f, _e2955));
                                                                        if (_e2945 != _e2945) {
                                                                            phi_28731_ = true;
                                                                        } else {
                                                                            phi_28731_ = (0f >= _e2945);
                                                                        }
                                                                        let _e2961 = phi_28731_;
                                                                        let _e2963 = sqrt(select(_e2945, 0f, _e2961));
                                                                        let _e2967 = ((_e2951 + _e2918) * 0.5f);
                                                                        let _e2968 = ((_e2957 + _e2919) * 0.5f);
                                                                        let _e2969 = ((_e2963 + _e2920) * 0.5f);
                                                                        if (_e2967 != _e2967) {
                                                                            phi_28758_ = true;
                                                                        } else {
                                                                            phi_28758_ = (0f >= _e2967);
                                                                        }
                                                                        let _e2973 = phi_28758_;
                                                                        let _e2975 = sqrt(select(_e2967, 0f, _e2973));
                                                                        if (_e2968 != _e2968) {
                                                                            phi_28773_ = true;
                                                                        } else {
                                                                            phi_28773_ = (0f >= _e2968);
                                                                        }
                                                                        let _e2979 = phi_28773_;
                                                                        let _e2981 = sqrt(select(_e2968, 0f, _e2979));
                                                                        if (_e2969 != _e2969) {
                                                                            phi_28788_ = true;
                                                                        } else {
                                                                            phi_28788_ = (0f >= _e2969);
                                                                        }
                                                                        let _e2985 = phi_28788_;
                                                                        let _e2987 = sqrt(select(_e2969, 0f, _e2985));
                                                                        let _e2988 = vec3<f32>(_e2975, _e2981, _e2987);
                                                                        let _e2992 = ((_e2951 - _e2918) * 0.5f);
                                                                        let _e2993 = ((_e2957 - _e2919) * 0.5f);
                                                                        let _e2994 = ((_e2963 - _e2920) * 0.5f);
                                                                        if (_e2992 != _e2992) {
                                                                            phi_28815_ = true;
                                                                        } else {
                                                                            phi_28815_ = (0f >= _e2992);
                                                                        }
                                                                        let _e2998 = phi_28815_;
                                                                        let _e3000 = sqrt(select(_e2992, 0f, _e2998));
                                                                        if (_e2993 != _e2993) {
                                                                            phi_28830_ = true;
                                                                        } else {
                                                                            phi_28830_ = (0f >= _e2993);
                                                                        }
                                                                        let _e3004 = phi_28830_;
                                                                        let _e3006 = sqrt(select(_e2993, 0f, _e3004));
                                                                        if (_e2994 != _e2994) {
                                                                            phi_28845_ = true;
                                                                        } else {
                                                                            phi_28845_ = (0f >= _e2994);
                                                                        }
                                                                        let _e3010 = phi_28845_;
                                                                        let _e3012 = sqrt(select(_e2994, 0f, _e3010));
                                                                        if ((_e2923 / _e2908) < 0f) {
                                                                            phi_15246_ = -(_e3000);
                                                                        } else {
                                                                            phi_15246_ = _e3000;
                                                                        }
                                                                        let _e3016 = phi_15246_;
                                                                        if ((_e2924 / _e2909) < 0f) {
                                                                            phi_15254_ = -(_e3006);
                                                                        } else {
                                                                            phi_15254_ = _e3006;
                                                                        }
                                                                        let _e3020 = phi_15254_;
                                                                        if ((_e2925 / _e2910) < 0f) {
                                                                            phi_15262_ = -(_e3012);
                                                                        } else {
                                                                            phi_15262_ = _e3012;
                                                                        }
                                                                        let _e3024 = phi_15262_;
                                                                        let _e3028 = (_e1979.x * _e2878);
                                                                        let _e3029 = (_e1979.y * _e2878);
                                                                        let _e3030 = (_e1979.z * _e2878);
                                                                        let _e3040 = vec3<f32>(((_e1981.x * _e2975) - (_e1979.x * _e3016)), ((_e1981.y * _e2981) - (_e1979.y * _e3020)), ((_e1981.z * _e2987) - (_e1979.z * _e3024)));
                                                                        let _e3047 = ((_e1981.x * _e3016) + (_e1979.x * _e2975));
                                                                        let _e3048 = ((_e1981.y * _e3020) + (_e1979.y * _e2981));
                                                                        let _e3049 = ((_e1981.z * _e3024) + (_e1979.z * _e2987));
                                                                        let _e3051 = vec3<f32>((_e1981.x * _e2878), (_e1981.y * _e2878), (_e1981.z * _e2878));
                                                                        let _e3052 = (_e3051 - _e2988);
                                                                        let _e3053 = (_e3028 - _e3016);
                                                                        let _e3054 = (_e3029 - _e3020);
                                                                        let _e3055 = (_e3030 - _e3024);
                                                                        let _e3068 = (_e3051 + _e2988);
                                                                        let _e3069 = (_e3028 + _e3016);
                                                                        let _e3070 = (_e3029 + _e3020);
                                                                        let _e3071 = (_e3030 + _e3024);
                                                                        let _e3081 = ((_e3068.x * _e3068.x) + (_e3069 * _e3069));
                                                                        let _e3082 = ((_e3068.y * _e3068.y) + (_e3070 * _e3070));
                                                                        let _e3083 = ((_e3068.z * _e3068.z) + (_e3071 * _e3071));
                                                                        let _e3093 = vec3(_e2878);
                                                                        let _e3094 = (_e3093 - _e3040);
                                                                        let _e3095 = -(vec3<f32>(_e3047, _e3048, _e3049));
                                                                        let _e3111 = (_e3093 + _e3040);
                                                                        let _e3121 = ((_e3111.x * _e3111.x) + (_e3047 * _e3047));
                                                                        let _e3122 = ((_e3111.y * _e3111.y) + (_e3048 * _e3048));
                                                                        let _e3123 = ((_e3111.z * _e3111.z) + (_e3049 * _e3049));
                                                                        phi_15369_ = vec3<f32>((((((_e3052.x * _e3052.x) + (_e3053 * _e3053)) / select(0.000000000000000000000000000001f, _e3081, (_e3081 > 0.000000000000000000000000000001f))) + (((_e3094.x * _e3094.x) + (_e3095.x * _e3095.x)) / select(0.000000000000000000000000000001f, _e3121, (_e3121 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3052.y * _e3052.y) + (_e3054 * _e3054)) / select(0.000000000000000000000000000001f, _e3082, (_e3082 > 0.000000000000000000000000000001f))) + (((_e3094.y * _e3094.y) + (_e3095.y * _e3095.y)) / select(0.000000000000000000000000000001f, _e3122, (_e3122 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e3052.z * _e3052.z) + (_e3055 * _e3055)) / select(0.000000000000000000000000000001f, _e3083, (_e3083 > 0.000000000000000000000000000001f))) + (((_e3094.z * _e3094.z) + (_e3095.z * _e3095.z)) / select(0.000000000000000000000000000001f, _e3123, (_e3123 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                                    }
                                                                    let _e3166 = phi_15369_;
                                                                    phi_15370_ = _e3166;
                                                                }
                                                                let _e3186 = phi_15370_;
                                                                let _e3188 = (4f * _e2800);
                                                                phi_15382_ = type_59((_e3186 * (((1f / (_e2851 + _e2865)) * _e2836) / _e3188)), ((_e2836 / _e2851) / _e3188));
                                                            }
                                                            let _e3195 = phi_15382_;
                                                            phi_15383_ = _e3195;
                                                            phi_15384_ = _e2812;
                                                        }
                                                        let _e3197 = phi_15383_;
                                                        let _e3199 = phi_15384_;
                                                        if _e3199 {
                                                            phi_15389_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                        } else {
                                                            phi_15389_ = _e3197;
                                                        }
                                                        let _e3201 = phi_15389_;
                                                        let _e3204 = (_e1987 == 1u);
                                                        if _e3204 {
                                                            phi_15417_ = _e3201;
                                                            phi_15418_ = type_59();
                                                        } else {
                                                            if (_e2800 != _e2800) {
                                                                phi_29107_ = true;
                                                            } else {
                                                                phi_29107_ = (0f >= _e2800);
                                                            }
                                                            let _e3208 = phi_29107_;
                                                            let _e3209 = select(_e2800, 0f, _e3208);
                                                            let _e3210 = (1f / _e1977);
                                                            let _e3214 = ((_e3210 * _e3210) * (1f - (_e3209 * _e3209)));
                                                            if (_e3214 >= 1f) {
                                                                phi_29126_ = 1f;
                                                            } else {
                                                                let _e3217 = sqrt((1f - _e3214));
                                                                if (_e3209 != _e3209) {
                                                                    phi_29137_ = true;
                                                                } else {
                                                                    phi_29137_ = (_e3217 <= _e3209);
                                                                }
                                                                let _e3221 = phi_29137_;
                                                                let _e3226 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                                phi_29126_ = (_e3226 + ((1f - _e3226) * pow((1f - select(_e3209, _e3217, _e3221)), 5f)));
                                                            }
                                                            let _e3233 = phi_29126_;
                                                            if (_e2778 <= 0f) {
                                                                phi_29176_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                phi_29176_ = (_e1975 * (_e2778 * 0.31830987f));
                                                            }
                                                            let _e3238 = phi_29176_;
                                                            let _e3239 = (1f - _e3233);
                                                            phi_15417_ = type_59();
                                                            phi_15418_ = type_59((_e3201.member + vec3<f32>((_e3238.x * _e3239), (_e3238.y * _e3239), (_e3238.z * _e3239))), ((_e3233 * _e3201.member_1) + (_e3239 * _e2784)));
                                                        }
                                                        let _e3253 = phi_15417_;
                                                        let _e3255 = phi_15418_;
                                                        phi_15419_ = _e3253;
                                                        phi_15420_ = _e3255;
                                                        phi_15421_ = _e3204;
                                                    }
                                                    let _e3263 = phi_15419_;
                                                    let _e3265 = phi_15420_;
                                                    let _e3267 = phi_15421_;
                                                    if _e3267 {
                                                        phi_15425_ = _e3263;
                                                    } else {
                                                        phi_15425_ = _e3265;
                                                    }
                                                    let _e3269 = phi_15425_;
                                                    if ((_e3269.member_1 * _e2773) < 0.001f) {
                                                        let _e3696 = global_1.member.member_6;
                                                        let _e3702 = global_1.member.member_8[2u];
                                                        if (_e3702 == 4294967295u) {
                                                            phi_16164_ = vec3<f32>(1f, 1f, 1f);
                                                        } else {
                                                            let _e3705 = global_6.member[_e3702];
                                                            let _e3708 = global_6.member[(_e3702 + 1u)];
                                                            let _e3711 = global_6.member[(_e3702 + 2u)];
                                                            let _e3714 = global_6.member[(_e3702 + 3u)];
                                                            let _e3717 = global_6.member[(_e3702 + 4u)];
                                                            let _e3720 = global_6.member[(_e3702 + 5u)];
                                                            let _e3723 = global_6.member[(_e3702 + 6u)];
                                                            let _e3726 = global_6.member[(_e3702 + 7u)];
                                                            let _e3729 = global_6.member[(_e3702 + 8u)];
                                                            let _e3732 = global_6.member[(_e3702 + 9u)];
                                                            let _e3735 = global_6.member[(_e3702 + 13u)];
                                                            let _e3738 = global_6.member[(_e3702 + 14u)];
                                                            let _e3741 = global_6.member[(_e3702 + 15u)];
                                                            let _e3744 = global_6.member[(_e3702 + 16u)];
                                                            let _e3747 = global_6.member[(_e3702 + 18u)];
                                                            let _e3750 = global_6.member[(_e3702 + 19u)];
                                                            let _e3753 = global_6.member[(_e3702 + 20u)];
                                                            let _e3756 = global_6.member[(_e3702 + 29u)];
                                                            let _e3757 = bitcast<u32>(_e3756);
                                                            let _e3770 = (((((_e1119 * _e1119) + (_e1120 * _e1120)) + (_e1121 * _e1121)) - (2f * (((_e1119 * _e3705) + (_e1120 * _e3708)) + (_e1121 * _e3711)))) + _e3723);
                                                            if (_e3770 > 0f) {
                                                                let _e3779 = ((((_e1119 - _e3705) * _e3726) + ((_e1120 - _e3708) * _e3729)) + ((_e1121 - _e3711) * _e3732));
                                                                let _e3781 = ((_e3779 * _e3779) - _e3770);
                                                                if (_e3779 >= 0f) {
                                                                    phi_15864_ = vec3<f32>();
                                                                    phi_15865_ = true;
                                                                } else {
                                                                    let _e3783 = (_e3781 <= 0f);
                                                                    if _e3783 {
                                                                        phi_15863_ = vec3<f32>();
                                                                    } else {
                                                                        let _e3786 = (-(_e3779) - sqrt(_e3781));
                                                                        phi_15863_ = vec3<f32>((_e1119 + (_e3726 * _e3786)), (_e1120 + (_e3729 * _e3786)), (_e1121 + (_e3732 * _e3786)));
                                                                    }
                                                                    let _e3795 = phi_15863_;
                                                                    phi_15864_ = _e3795;
                                                                    phi_15865_ = _e3783;
                                                                }
                                                                let _e3797 = phi_15864_;
                                                                let _e3799 = phi_15865_;
                                                                phi_15870_ = _e3797;
                                                                phi_15871_ = select(true, false, _e3799);
                                                            } else {
                                                                phi_15870_ = _e1122;
                                                                phi_15871_ = true;
                                                            }
                                                            let _e3802 = phi_15870_;
                                                            let _e3804 = phi_15871_;
                                                            if _e3804 {
                                                                let _e3806 = (_e3802.x - _e3705);
                                                                let _e3808 = (_e3802.y - _e3708);
                                                                let _e3810 = (_e3802.z - _e3711);
                                                                let _e3816 = sqrt((((_e3806 * _e3806) + (_e3808 * _e3808)) + (_e3810 * _e3810)));
                                                                let _e3817 = (1f / _e3816);
                                                                let _e3835 = ((((((_e3802.x * _e3802.x) + (_e3802.y * _e3802.y)) + (_e3802.z * _e3802.z)) - (2f * (((_e3802.x * _e3705) + (_e3802.y * _e3708)) + (_e3802.z * _e3711)))) + _e3720) / (_e3816 + _e3714));
                                                                if (_e3835 != _e3835) {
                                                                    phi_29632_ = true;
                                                                } else {
                                                                    phi_29632_ = (0f >= _e3835);
                                                                }
                                                                let _e3839 = phi_29632_;
                                                                let _e3843 = sqrt((select(_e3835, 0f, _e3839) / (_e3717 - _e3714)));
                                                                if (_e3843 != _e3843) {
                                                                    phi_29647_ = true;
                                                                } else {
                                                                    phi_29647_ = (1f <= _e3843);
                                                                }
                                                                let _e3847 = phi_29647_;
                                                                let _e3849 = (select(_e3843, 1f, _e3847) * 63f);
                                                                let _e3854 = ((((_e3806 * _e3817) * _e3726) + ((_e3808 * _e3817) * _e3729)) + ((_e3810 * _e3817) * _e3732));
                                                                let _e3856 = select(_e3854, -1f, (_e3854 < -1f));
                                                                let _e3860 = ((select(_e3856, 1f, (_e3856 > 1f)) + 1f) * 127.5f);
                                                                let _e3865 = select(select(u32(_e3849), 0u, (_e3849 < 0f)), 4294967295u, (_e3849 > 4294967000f));
                                                                let _e3867 = select(_e3865, 62u, (62u < _e3865));
                                                                let _e3872 = select(select(u32(_e3860), 0u, (_e3860 < 0f)), 4294967295u, (_e3860 > 4294967000f));
                                                                let _e3874 = select(_e3872, 254u, (254u < _e3872));
                                                                let _e3876 = (_e3849 - f32(_e3867));
                                                                let _e3878 = (_e3860 - f32(_e3874));
                                                                let _e3882 = (_e3757 + (3u * ((_e3867 * 256u) + _e3874)));
                                                                let _e3887 = (_e3757 + (3u * (((_e3867 + 1u) * 256u) + _e3874)));
                                                                let _e3889 = global_6.member[_e3882];
                                                                let _e3892 = global_6.member[(_e3882 + 1u)];
                                                                let _e3895 = global_6.member[(_e3882 + 2u)];
                                                                let _e3896 = (1f - _e3878);
                                                                let _e3902 = global_6.member[(_e3882 + 3u)];
                                                                let _e3905 = global_6.member[(_e3882 + 4u)];
                                                                let _e3908 = global_6.member[(_e3882 + 5u)];
                                                                let _e3915 = (1f - _e3876);
                                                                let _e3920 = global_6.member[_e3887];
                                                                let _e3923 = global_6.member[(_e3887 + 1u)];
                                                                let _e3926 = global_6.member[(_e3887 + 2u)];
                                                                let _e3932 = global_6.member[(_e3887 + 3u)];
                                                                let _e3935 = global_6.member[(_e3887 + 4u)];
                                                                let _e3938 = global_6.member[(_e3887 + 5u)];
                                                                phi_16162_ = vec3<f32>(((((_e3889 * _e3896) + (_e3902 * _e3878)) * _e3915) + (((_e3920 * _e3896) + (_e3932 * _e3878)) * _e3876)), ((((_e3892 * _e3896) + (_e3905 * _e3878)) * _e3915) + (((_e3923 * _e3896) + (_e3935 * _e3878)) * _e3876)), ((((_e3895 * _e3896) + (_e3908 * _e3878)) * _e3915) + (((_e3926 * _e3896) + (_e3938 * _e3878)) * _e3876)));
                                                            } else {
                                                                phi_16162_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e3953 = phi_16162_;
                                                            let _e3960 = ((_e3744 * 1.1f) * _e3953.y);
                                                            phi_16164_ = vec3<f32>(exp(-((((_e3735 * _e3953.x) + _e3960) + (_e3747 * _e3953.z)))), exp(-((((_e3738 * _e3953.x) + _e3960) + (_e3750 * _e3953.z)))), exp(-((((_e3741 * _e3953.x) + _e3960) + (_e3753 * _e3953.z)))));
                                                        }
                                                        let _e3979 = phi_16164_;
                                                        phi_37050_ = _e2002;
                                                        phi_17048_ = 0f;
                                                        phi_17049_ = vec3<f32>(_e2761.x, _e2761.y, _e2761.z);
                                                        phi_17050_ = (vec3<f32>(_e3696.x, _e3696.y, _e3696.z) * _e3979);
                                                    } else {
                                                        let _e3274 = ((_e2002 * 747796405u) + 2891336453u);
                                                        let _e3282 = (((_e3274 >> bitcast<u32>((((_e3274 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3274) * 277803737u);
                                                        let _e3285 = ((_e3282 >> bitcast<u32>(22i)) ^ _e3282);
                                                        let _e3291 = (((f32((_e3285 >> bitcast<u32>(8i))) * 0.000000059604645f) * _e2773) * 0.15915494f);
                                                        let _e3292 = (1f - _e3291);
                                                        let _e3294 = (_e3291 * (2f - _e3291));
                                                        if (_e3294 != _e3294) {
                                                            phi_29230_ = true;
                                                        } else {
                                                            phi_29230_ = (0f >= _e3294);
                                                        }
                                                        let _e3298 = phi_29230_;
                                                        let _e3300 = sqrt(select(_e3294, 0f, _e3298));
                                                        let _e3302 = ((_e3285 * 747796405u) + 2891336453u);
                                                        let _e3310 = (((_e3302 >> bitcast<u32>((((_e3302 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e3302) * 277803737u);
                                                        let _e3313 = ((_e3310 >> bitcast<u32>(22i)) ^ _e3310);
                                                        let _e3317 = (f32((_e3313 >> bitcast<u32>(8i))) * 0.00000037450704f);
                                                        let _e3319 = select(-1f, 1f, (_e2761.z >= 0f));
                                                        let _e3321 = (-1f / (_e3319 + _e2761.z));
                                                        let _e3323 = ((_e2761.x * _e2761.y) * _e3321);
                                                        let _e3336 = (_e3300 * cos(_e3317));
                                                        let _e3341 = (_e3300 * sin(_e3317));
                                                        let _e3351 = ((((1f + (((_e3319 * _e2761.x) * _e2761.x) * _e3321)) * _e3336) + (_e3323 * _e3341)) + (_e2761.x * _e3292));
                                                        let _e3352 = ((((_e3319 * _e3323) * _e3336) + ((_e3319 + ((_e2761.y * _e2761.y) * _e3321)) * _e3341)) + (_e2761.y * _e3292));
                                                        let _e3353 = ((((-(_e3319) * _e2761.x) * _e3336) + (-(_e2761.y) * _e3341)) + (_e2761.z * _e3292));
                                                        let _e3355 = (_e3351 - _e2761.x);
                                                        let _e3356 = (_e3352 - _e2761.y);
                                                        let _e3357 = (_e3353 - _e2761.z);
                                                        let _e3364 = (sqrt((((_e3355 * _e3355) + (_e3356 * _e3356)) + (_e3357 * _e3357))) * 0.5f);
                                                        let _e3365 = (_e3364 != _e3364);
                                                        if _e3365 {
                                                            phi_29301_ = true;
                                                        } else {
                                                            phi_29301_ = (1f <= _e3364);
                                                        }
                                                        let _e3368 = phi_29301_;
                                                        if ((2f * asin(select(_e3364, 1f, _e3368))) >= _e2769) {
                                                            phi_16245_ = 0f;
                                                        } else {
                                                            phi_16245_ = (1f / _e2773);
                                                        }
                                                        let _e3375 = phi_16245_;
                                                        if (_e3375 > 0f) {
                                                            if _e3365 {
                                                                phi_29316_ = true;
                                                            } else {
                                                                phi_29316_ = (1f <= _e3364);
                                                            }
                                                            let _e3379 = phi_29316_;
                                                            let _e3382 = (2f * asin(select(_e3364, 1f, _e3379)));
                                                            if (_e3382 >= _e2769) {
                                                                phi_17046_ = vec3<f32>(0f, 0f, 0f);
                                                            } else {
                                                                let _e3386 = (1f - pow((_e3382 / _e2769), 2f));
                                                                if (_e3386 != _e3386) {
                                                                    phi_29331_ = true;
                                                                } else {
                                                                    phi_29331_ = (0f >= _e3386);
                                                                }
                                                                let _e3390 = phi_29331_;
                                                                let _e3393 = global_1.member.member_6;
                                                                let _e3399 = global_1.member.member_8[2u];
                                                                if (_e3399 == 4294967295u) {
                                                                    phi_17019_ = vec3<f32>(1f, 1f, 1f);
                                                                } else {
                                                                    let _e3402 = global_6.member[_e3399];
                                                                    let _e3405 = global_6.member[(_e3399 + 1u)];
                                                                    let _e3408 = global_6.member[(_e3399 + 2u)];
                                                                    let _e3411 = global_6.member[(_e3399 + 3u)];
                                                                    let _e3414 = global_6.member[(_e3399 + 4u)];
                                                                    let _e3417 = global_6.member[(_e3399 + 5u)];
                                                                    let _e3420 = global_6.member[(_e3399 + 6u)];
                                                                    let _e3423 = global_6.member[(_e3399 + 7u)];
                                                                    let _e3426 = global_6.member[(_e3399 + 8u)];
                                                                    let _e3429 = global_6.member[(_e3399 + 9u)];
                                                                    let _e3432 = global_6.member[(_e3399 + 13u)];
                                                                    let _e3435 = global_6.member[(_e3399 + 14u)];
                                                                    let _e3438 = global_6.member[(_e3399 + 15u)];
                                                                    let _e3441 = global_6.member[(_e3399 + 16u)];
                                                                    let _e3444 = global_6.member[(_e3399 + 18u)];
                                                                    let _e3447 = global_6.member[(_e3399 + 19u)];
                                                                    let _e3450 = global_6.member[(_e3399 + 20u)];
                                                                    let _e3453 = global_6.member[(_e3399 + 29u)];
                                                                    let _e3454 = bitcast<u32>(_e3453);
                                                                    let _e3467 = (((((_e1119 * _e1119) + (_e1120 * _e1120)) + (_e1121 * _e1121)) - (2f * (((_e1119 * _e3402) + (_e1120 * _e3405)) + (_e1121 * _e3408)))) + _e3420);
                                                                    if (_e3467 > 0f) {
                                                                        let _e3476 = ((((_e1119 - _e3402) * _e3423) + ((_e1120 - _e3405) * _e3426)) + ((_e1121 - _e3408) * _e3429));
                                                                        let _e3478 = ((_e3476 * _e3476) - _e3467);
                                                                        if (_e3476 >= 0f) {
                                                                            phi_16719_ = vec3<f32>();
                                                                            phi_16720_ = true;
                                                                        } else {
                                                                            let _e3480 = (_e3478 <= 0f);
                                                                            if _e3480 {
                                                                                phi_16718_ = vec3<f32>();
                                                                            } else {
                                                                                let _e3483 = (-(_e3476) - sqrt(_e3478));
                                                                                phi_16718_ = vec3<f32>((_e1119 + (_e3423 * _e3483)), (_e1120 + (_e3426 * _e3483)), (_e1121 + (_e3429 * _e3483)));
                                                                            }
                                                                            let _e3492 = phi_16718_;
                                                                            phi_16719_ = _e3492;
                                                                            phi_16720_ = _e3480;
                                                                        }
                                                                        let _e3494 = phi_16719_;
                                                                        let _e3496 = phi_16720_;
                                                                        phi_16725_ = _e3494;
                                                                        phi_16726_ = select(true, false, _e3496);
                                                                    } else {
                                                                        phi_16725_ = _e1122;
                                                                        phi_16726_ = true;
                                                                    }
                                                                    let _e3499 = phi_16725_;
                                                                    let _e3501 = phi_16726_;
                                                                    if _e3501 {
                                                                        let _e3503 = (_e3499.x - _e3402);
                                                                        let _e3505 = (_e3499.y - _e3405);
                                                                        let _e3507 = (_e3499.z - _e3408);
                                                                        let _e3513 = sqrt((((_e3503 * _e3503) + (_e3505 * _e3505)) + (_e3507 * _e3507)));
                                                                        let _e3514 = (1f / _e3513);
                                                                        let _e3532 = ((((((_e3499.x * _e3499.x) + (_e3499.y * _e3499.y)) + (_e3499.z * _e3499.z)) - (2f * (((_e3499.x * _e3402) + (_e3499.y * _e3405)) + (_e3499.z * _e3408)))) + _e3417) / (_e3513 + _e3411));
                                                                        if (_e3532 != _e3532) {
                                                                            phi_29455_ = true;
                                                                        } else {
                                                                            phi_29455_ = (0f >= _e3532);
                                                                        }
                                                                        let _e3536 = phi_29455_;
                                                                        let _e3540 = sqrt((select(_e3532, 0f, _e3536) / (_e3414 - _e3411)));
                                                                        if (_e3540 != _e3540) {
                                                                            phi_29470_ = true;
                                                                        } else {
                                                                            phi_29470_ = (1f <= _e3540);
                                                                        }
                                                                        let _e3544 = phi_29470_;
                                                                        let _e3546 = (select(_e3540, 1f, _e3544) * 63f);
                                                                        let _e3551 = ((((_e3503 * _e3514) * _e3423) + ((_e3505 * _e3514) * _e3426)) + ((_e3507 * _e3514) * _e3429));
                                                                        let _e3553 = select(_e3551, -1f, (_e3551 < -1f));
                                                                        let _e3557 = ((select(_e3553, 1f, (_e3553 > 1f)) + 1f) * 127.5f);
                                                                        let _e3562 = select(select(u32(_e3546), 0u, (_e3546 < 0f)), 4294967295u, (_e3546 > 4294967000f));
                                                                        let _e3564 = select(_e3562, 62u, (62u < _e3562));
                                                                        let _e3569 = select(select(u32(_e3557), 0u, (_e3557 < 0f)), 4294967295u, (_e3557 > 4294967000f));
                                                                        let _e3571 = select(_e3569, 254u, (254u < _e3569));
                                                                        let _e3573 = (_e3546 - f32(_e3564));
                                                                        let _e3575 = (_e3557 - f32(_e3571));
                                                                        let _e3579 = (_e3454 + (3u * ((_e3564 * 256u) + _e3571)));
                                                                        let _e3584 = (_e3454 + (3u * (((_e3564 + 1u) * 256u) + _e3571)));
                                                                        let _e3586 = global_6.member[_e3579];
                                                                        let _e3589 = global_6.member[(_e3579 + 1u)];
                                                                        let _e3592 = global_6.member[(_e3579 + 2u)];
                                                                        let _e3593 = (1f - _e3575);
                                                                        let _e3599 = global_6.member[(_e3579 + 3u)];
                                                                        let _e3602 = global_6.member[(_e3579 + 4u)];
                                                                        let _e3605 = global_6.member[(_e3579 + 5u)];
                                                                        let _e3612 = (1f - _e3573);
                                                                        let _e3617 = global_6.member[_e3584];
                                                                        let _e3620 = global_6.member[(_e3584 + 1u)];
                                                                        let _e3623 = global_6.member[(_e3584 + 2u)];
                                                                        let _e3629 = global_6.member[(_e3584 + 3u)];
                                                                        let _e3632 = global_6.member[(_e3584 + 4u)];
                                                                        let _e3635 = global_6.member[(_e3584 + 5u)];
                                                                        phi_17017_ = vec3<f32>(((((_e3586 * _e3593) + (_e3599 * _e3575)) * _e3612) + (((_e3617 * _e3593) + (_e3629 * _e3575)) * _e3573)), ((((_e3589 * _e3593) + (_e3602 * _e3575)) * _e3612) + (((_e3620 * _e3593) + (_e3632 * _e3575)) * _e3573)), ((((_e3592 * _e3593) + (_e3605 * _e3575)) * _e3612) + (((_e3623 * _e3593) + (_e3635 * _e3575)) * _e3573)));
                                                                    } else {
                                                                        phi_17017_ = vec3<f32>(0f, 0f, 0f);
                                                                    }
                                                                    let _e3650 = phi_17017_;
                                                                    let _e3657 = ((_e3441 * 1.1f) * _e3650.y);
                                                                    phi_17019_ = vec3<f32>(exp(-((((_e3432 * _e3650.x) + _e3657) + (_e3444 * _e3650.z)))), exp(-((((_e3435 * _e3650.x) + _e3657) + (_e3447 * _e3650.z)))), exp(-((((_e3438 * _e3650.x) + _e3657) + (_e3450 * _e3650.z)))));
                                                                }
                                                                let _e3676 = phi_17019_;
                                                                let _e3687 = (((1f - (0.6f * (1f - sqrt(select(_e3386, 0f, _e3390))))) * 1.25f) / _e2773);
                                                                phi_17046_ = vec3<f32>(((_e3393.x * _e3676.x) * _e3687), ((_e3393.y * _e3676.y) * _e3687), ((_e3393.z * _e3676.z) * _e3687));
                                                            }
                                                            let _e3693 = phi_17046_;
                                                            phi_17047_ = _e3693;
                                                        } else {
                                                            phi_17047_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e3695 = phi_17047_;
                                                        phi_37050_ = _e3313;
                                                        phi_17048_ = _e3375;
                                                        phi_17049_ = vec3<f32>(_e3351, _e3352, _e3353);
                                                        phi_17050_ = _e3695;
                                                    }
                                                    let _e3983 = phi_37050_;
                                                    let _e3985 = phi_17048_;
                                                    let _e3987 = phi_17049_;
                                                    let _e3989 = phi_17050_;
                                                    phi_37049_ = _e3983;
                                                    phi_17051_ = _e3985;
                                                    phi_17052_ = _e3987;
                                                    phi_17053_ = _e3989;
                                                }
                                                let _e3991 = phi_37049_;
                                                let _e3993 = phi_17051_;
                                                let _e3995 = phi_17052_;
                                                let _e3997 = phi_17053_;
                                                phi_37006_ = _e3991;
                                                phi_18418_ = _e3993;
                                                phi_18419_ = _e3995;
                                                phi_18420_ = _e3997;
                                                phi_18421_ = 1000000000000000000000000000000f;
                                            } else {
                                                if (_e2008 == (_e1998 + 1u)) {
                                                    if ((_e392 & 1u) == 0u) {
                                                        phi_37015_ = _e2002;
                                                        phi_18052_ = 0f;
                                                        phi_18053_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_18054_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_18055_ = 1000000000000000000000000000000f;
                                                    } else {
                                                        if (_e395 > 0u) {
                                                            let _e2239 = global_1.member.member_2[1u];
                                                            let _e2241 = ((_e2002 * 747796405u) + 2891336453u);
                                                            let _e2249 = (((_e2241 >> bitcast<u32>((((_e2241 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2241) * 277803737u);
                                                            let _e2252 = ((_e2249 >> bitcast<u32>(22i)) ^ _e2249);
                                                            phi_17074_ = 0u;
                                                            phi_17077_ = _e395;
                                                            loop {
                                                                let _e2258 = phi_17074_;
                                                                let _e2260 = phi_17077_;
                                                                local_78 = _e2258;
                                                                local_79 = _e2258;
                                                                let _e2262 = ((_e2258 + 1u) < _e2260);
                                                                if _e2262 {
                                                                    let _e2264 = ((_e2258 + _e2260) / 2u);
                                                                    let _e2267 = global_6.member[(_e2239 + _e2264)];
                                                                    let _e2268 = (_e2267 <= (f32((_e2252 >> bitcast<u32>(8i))) * 0.000000059604645f));
                                                                    phi_17075_ = select(_e2258, _e2264, _e2268);
                                                                    phi_17078_ = select(_e2264, _e2260, _e2268);
                                                                } else {
                                                                    phi_17075_ = u32();
                                                                    phi_17078_ = u32();
                                                                }
                                                                let _e2272 = phi_17075_;
                                                                let _e2274 = phi_17078_;
                                                                phi_17074_ = _e2272;
                                                                phi_17077_ = _e2274;
                                                                if !(_e2262) {
                                                                    break;
                                                                }
                                                            }
                                                            let _e2277 = local_78;
                                                            let _e2278 = (_e2239 + _e2277);
                                                            let _e2281 = global_6.member[(_e2278 + 1u)];
                                                            let _e2283 = global_6.member[_e2278];
                                                            let _e2287 = global_1.member.member_2[0u];
                                                            let _e2289 = local_79;
                                                            let _e2292 = global_6.member[(_e2287 + _e2289)];
                                                            let _e2293 = bitcast<u32>(_e2292);
                                                            let _e2295 = ((_e2252 * 747796405u) + 2891336453u);
                                                            let _e2303 = (((_e2295 >> bitcast<u32>((((_e2295 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2295) * 277803737u);
                                                            let _e2306 = ((_e2303 >> bitcast<u32>(22i)) ^ _e2303);
                                                            let _e2309 = f32((_e2306 >> bitcast<u32>(8i)));
                                                            let _e2310 = (_e2309 * 0.000000059604645f);
                                                            let _e2312 = ((_e2306 * 747796405u) + 2891336453u);
                                                            let _e2320 = (((_e2312 >> bitcast<u32>((((_e2312 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2312) * 277803737u);
                                                            let _e2323 = ((_e2320 >> bitcast<u32>(22i)) ^ _e2320);
                                                            let _e2326 = f32((_e2323 >> bitcast<u32>(8i)));
                                                            let _e2328 = (3u * _e2293);
                                                            let _e2330 = global_2.member[_e2328];
                                                            let _e2333 = global_2.member[(_e2328 + 1u)];
                                                            let _e2339 = select(select(u32(_e2333.w), 0u, (_e2333.w < 0f)), 4294967295u, (_e2333.w > 4294967000f));
                                                            let _e2341 = (_e864 + (5u * _e2293));
                                                            let _e2343 = global_2.member[_e2341];
                                                            let _e2346 = global_2.member[(_e2341 + 1u)];
                                                            let _e2353 = (_e2339 == 1u);
                                                            if _e2353 {
                                                                let _e2482 = (_e2330.x - _e1119);
                                                                let _e2483 = (_e2330.y - _e1120);
                                                                let _e2484 = (_e2330.z - _e1121);
                                                                let _e2489 = (((_e2482 * _e2482) + (_e2483 * _e2483)) + (_e2484 * _e2484));
                                                                let _e2490 = (_e2330.w * _e2330.w);
                                                                let _e2492 = (_e2489 <= (_e2490 * 1.000001f));
                                                                if _e2492 {
                                                                    let _e2601 = (_e2309 * 0.00000037450704f);
                                                                    let _e2603 = ((_e2326 * 0.00000011920929f) - 1f);
                                                                    let _e2605 = (1f - (_e2603 * _e2603));
                                                                    if (_e2605 != _e2605) {
                                                                        phi_28298_ = true;
                                                                    } else {
                                                                        phi_28298_ = (0f >= _e2605);
                                                                    }
                                                                    let _e2609 = phi_28298_;
                                                                    let _e2611 = sqrt(select(_e2605, 0f, _e2609));
                                                                    let _e2613 = (_e2611 * cos(_e2601));
                                                                    let _e2615 = (_e2611 * sin(_e2601));
                                                                    let _e2620 = (_e2330.x + (_e2613 * _e2330.w));
                                                                    let _e2621 = (_e2330.y + (_e2615 * _e2330.w));
                                                                    let _e2622 = (_e2330.z + (_e2603 * _e2330.w));
                                                                    if _e2492 {
                                                                        let _e2635 = (_e2620 - _e1119);
                                                                        let _e2636 = (_e2621 - _e1120);
                                                                        let _e2637 = (_e2622 - _e1121);
                                                                        let _e2642 = (((_e2635 * _e2635) + (_e2636 * _e2636)) + (_e2637 * _e2637));
                                                                        phi_28367_ = (_e2642 / (((abs((((_e2613 * _e2635) + (_e2615 * _e2636)) + (_e2603 * _e2637))) / sqrt(_e2642)) * 12.566371f) * _e2490));
                                                                    } else {
                                                                        let _e2624 = (_e2490 / _e2489);
                                                                        if (_e2624 != _e2624) {
                                                                            phi_28386_ = true;
                                                                        } else {
                                                                            phi_28386_ = (1f <= _e2624);
                                                                        }
                                                                        let _e2628 = phi_28386_;
                                                                        let _e2629 = select(_e2624, 1f, _e2628);
                                                                        phi_28367_ = (0.15915494f / (_e2629 / (1f + sqrt((1f - _e2629)))));
                                                                    }
                                                                    let _e2655 = phi_28367_;
                                                                    phi_17425_ = type_77(vec3<f32>(_e2620, _e2621, _e2622), vec3<f32>(_e2613, _e2615, _e2603), _e2655);
                                                                } else {
                                                                    let _e2493 = (_e2490 / _e2489);
                                                                    let _e2494 = (_e2493 != _e2493);
                                                                    if _e2494 {
                                                                        phi_28150_ = true;
                                                                    } else {
                                                                        phi_28150_ = (1f <= _e2493);
                                                                    }
                                                                    let _e2497 = phi_28150_;
                                                                    let _e2498 = select(_e2493, 1f, _e2497);
                                                                    if _e2494 {
                                                                        phi_28173_ = true;
                                                                    } else {
                                                                        phi_28173_ = (1f <= _e2493);
                                                                    }
                                                                    let _e2502 = phi_28173_;
                                                                    let _e2503 = select(_e2493, 1f, _e2502);
                                                                    let _e2507 = (_e2503 / (1f + sqrt((1f - _e2503))));
                                                                    if (_e2498 < 0.00068523f) {
                                                                        phi_17360_ = (_e2498 * _e2310);
                                                                    } else {
                                                                        let _e2510 = (1f - (_e2310 * _e2507));
                                                                        let _e2512 = (1f - (_e2510 * _e2510));
                                                                        if (_e2512 != _e2512) {
                                                                            phi_28188_ = true;
                                                                        } else {
                                                                            phi_28188_ = (0f >= _e2512);
                                                                        }
                                                                        let _e2516 = phi_28188_;
                                                                        phi_17360_ = select(_e2512, 0f, _e2516);
                                                                    }
                                                                    let _e2520 = phi_17360_;
                                                                    let _e2521 = (1f - _e2520);
                                                                    if (_e2521 != _e2521) {
                                                                        phi_28203_ = true;
                                                                    } else {
                                                                        phi_28203_ = (0f >= _e2521);
                                                                    }
                                                                    let _e2525 = phi_28203_;
                                                                    let _e2530 = (1f - (_e2520 / _e2498));
                                                                    if (_e2530 != _e2530) {
                                                                        phi_28218_ = true;
                                                                    } else {
                                                                        phi_28218_ = (0f >= _e2530);
                                                                    }
                                                                    let _e2534 = phi_28218_;
                                                                    let _e2538 = ((_e2520 / sqrt(_e2498)) + (sqrt(select(_e2521, 0f, _e2525)) * sqrt(select(_e2530, 0f, _e2534))));
                                                                    let _e2540 = (1f - (_e2538 * _e2538));
                                                                    if (_e2540 != _e2540) {
                                                                        phi_28233_ = true;
                                                                    } else {
                                                                        phi_28233_ = (0f >= _e2540);
                                                                    }
                                                                    let _e2544 = phi_28233_;
                                                                    let _e2546 = sqrt(select(_e2540, 0f, _e2544));
                                                                    let _e2547 = (_e2326 * 0.00000037450704f);
                                                                    let _e2548 = sqrt(_e2489);
                                                                    let _e2549 = (_e2482 / _e2548);
                                                                    let _e2550 = (_e2483 / _e2548);
                                                                    let _e2551 = (_e2484 / _e2548);
                                                                    let _e2553 = select(-1f, 1f, (_e2551 >= 0f));
                                                                    let _e2555 = (-1f / (_e2553 + _e2551));
                                                                    let _e2557 = ((_e2549 * _e2550) * _e2555);
                                                                    let _e2570 = (_e2546 * cos(_e2547));
                                                                    let _e2575 = (_e2546 * sin(_e2547));
                                                                    let _e2588 = -(((((1f + (((_e2553 * _e2549) * _e2549) * _e2555)) * _e2570) + (_e2557 * _e2575)) + (_e2549 * _e2538)));
                                                                    let _e2589 = -(((((_e2553 * _e2557) * _e2570) + ((_e2553 + ((_e2550 * _e2550) * _e2555)) * _e2575)) + (_e2550 * _e2538)));
                                                                    let _e2590 = -(((((-(_e2553) * _e2549) * _e2570) + (-(_e2550) * _e2575)) + (_e2551 * _e2538)));
                                                                    phi_17425_ = type_77(vec3<f32>((_e2330.x + (_e2588 * _e2330.w)), (_e2330.y + (_e2589 * _e2330.w)), (_e2330.z + (_e2590 * _e2330.w))), vec3<f32>(_e2588, _e2589, _e2590), (0.15915494f / _e2507));
                                                                }
                                                                let _e2658 = phi_17425_;
                                                                phi_17853_ = _e2658;
                                                            } else {
                                                                let _e2354 = sqrt(_e2310);
                                                                let _e2355 = (1f - _e2354);
                                                                let _e2356 = ((_e2326 * 0.000000059604645f) * _e2354);
                                                                let _e2369 = (_e2354 - _e2356);
                                                                let _e2373 = (((_e2330.x * _e2355) + (_e2343.x * _e2356)) + (_e2346.x * _e2369));
                                                                let _e2374 = (((_e2330.y * _e2355) + (_e2343.y * _e2356)) + (_e2346.y * _e2369));
                                                                let _e2375 = (((_e2330.z * _e2355) + (_e2343.z * _e2356)) + (_e2346.z * _e2369));
                                                                let _e2377 = (_e2343.x - _e2330.x);
                                                                let _e2378 = (_e2343.y - _e2330.y);
                                                                let _e2379 = (_e2343.z - _e2330.z);
                                                                let _e2380 = (_e2346.x - _e2330.x);
                                                                let _e2381 = (_e2346.y - _e2330.y);
                                                                let _e2382 = (_e2346.z - _e2330.z);
                                                                let _e2385 = ((_e2378 * _e2382) - (_e2381 * _e2379));
                                                                let _e2388 = ((_e2379 * _e2380) - (_e2382 * _e2377));
                                                                let _e2391 = ((_e2377 * _e2381) - (_e2380 * _e2378));
                                                                let _e2397 = sqrt((((_e2385 * _e2385) + (_e2388 * _e2388)) + (_e2391 * _e2391)));
                                                                let _e2398 = (1f / _e2397);
                                                                let _e2399 = (_e2385 * _e2398);
                                                                let _e2400 = (_e2388 * _e2398);
                                                                let _e2401 = (_e2391 * _e2398);
                                                                if _e2353 {
                                                                    let _e2432 = (_e2330.x - _e1119);
                                                                    let _e2433 = (_e2330.y - _e1120);
                                                                    let _e2434 = (_e2330.z - _e1121);
                                                                    let _e2439 = (((_e2432 * _e2432) + (_e2433 * _e2433)) + (_e2434 * _e2434));
                                                                    let _e2440 = (_e2330.w * _e2330.w);
                                                                    if (_e2439 <= (_e2440 * 1.000001f)) {
                                                                        let _e2454 = (_e2373 - _e1119);
                                                                        let _e2455 = (_e2374 - _e1120);
                                                                        let _e2456 = (_e2375 - _e1121);
                                                                        let _e2461 = (((_e2454 * _e2454) + (_e2455 * _e2455)) + (_e2456 * _e2456));
                                                                        phi_28116_ = (_e2461 / (((abs((((_e2399 * _e2454) + (_e2400 * _e2455)) + (_e2401 * _e2456))) / sqrt(_e2461)) * 12.566371f) * _e2440));
                                                                    } else {
                                                                        let _e2443 = (_e2440 / _e2439);
                                                                        if (_e2443 != _e2443) {
                                                                            phi_28135_ = true;
                                                                        } else {
                                                                            phi_28135_ = (1f <= _e2443);
                                                                        }
                                                                        let _e2447 = phi_28135_;
                                                                        let _e2448 = select(_e2443, 1f, _e2447);
                                                                        phi_28116_ = (0.15915494f / (_e2448 / (1f + sqrt((1f - _e2448)))));
                                                                    }
                                                                    let _e2474 = phi_28116_;
                                                                    phi_17851_ = _e2474;
                                                                } else {
                                                                    let _e2403 = (_e2373 - _e1119);
                                                                    let _e2404 = (_e2374 - _e1120);
                                                                    let _e2405 = (_e2375 - _e1121);
                                                                    let _e2410 = (((_e2403 * _e2403) + (_e2404 * _e2404)) + (_e2405 * _e2405));
                                                                    let _e2418 = (abs((((_e2399 * _e2403) + (_e2400 * _e2404)) + (_e2401 * _e2405))) / sqrt(_e2410));
                                                                    if (_e2418 <= 0f) {
                                                                        phi_17850_ = 0f;
                                                                    } else {
                                                                        if (_e2339 == 0u) {
                                                                            phi_17847_ = (_e2397 * 0.5f);
                                                                        } else {
                                                                            phi_17847_ = ((12.566371f * _e2330.w) * _e2330.w);
                                                                        }
                                                                        let _e2426 = phi_17847_;
                                                                        phi_17850_ = (_e2410 / (_e2418 * _e2426));
                                                                    }
                                                                    let _e2430 = phi_17850_;
                                                                    phi_17851_ = _e2430;
                                                                }
                                                                let _e2476 = phi_17851_;
                                                                phi_17853_ = type_77(vec3<f32>(_e2373, _e2374, _e2375), vec3<f32>(_e2399, _e2400, _e2401), _e2476);
                                                            }
                                                            let _e2660 = phi_17853_;
                                                            let _e2665 = (_e2660.member.x - _e1119);
                                                            let _e2667 = (_e2660.member.y - _e1120);
                                                            let _e2669 = (_e2660.member.z - _e1121);
                                                            let _e2675 = sqrt((((_e2665 * _e2665) + (_e2667 * _e2667)) + (_e2669 * _e2669)));
                                                            let _e2676 = (_e2665 / _e2675);
                                                            let _e2677 = (_e2667 / _e2675);
                                                            let _e2678 = (_e2669 / _e2675);
                                                            if (_e2660.member_2 > 0f) {
                                                                let _e2694 = global_4.member[select(select(u32(_e2343.w), 0u, (_e2343.w < 0f)), 4294967295u, (_e2343.w > 4294967000f))];
                                                                if (_e2694.member == 4u) {
                                                                    if (_e2694.member_1 == 1u) {
                                                                        phi_18029_ = true;
                                                                    } else {
                                                                        phi_18029_ = ((((-(_e2676) * _e2660.member_1.x) + (-(_e2677) * _e2660.member_1.y)) + (-(_e2678) * _e2660.member_1.z)) > 0f);
                                                                    }
                                                                    let _e2709 = phi_18029_;
                                                                    if _e2709 {
                                                                        phi_18038_ = vec3<f32>(_e2694.member_4.x, _e2694.member_4.y, _e2694.member_4.z);
                                                                    } else {
                                                                        phi_18038_ = vec3<f32>();
                                                                    }
                                                                    let _e2716 = phi_18038_;
                                                                    phi_18040_ = _e2716;
                                                                    phi_18041_ = select(true, false, _e2709);
                                                                } else {
                                                                    phi_18040_ = vec3<f32>();
                                                                    phi_18041_ = true;
                                                                }
                                                                let _e2719 = phi_18040_;
                                                                let _e2721 = phi_18041_;
                                                                phi_18046_ = ((_e2281 - _e2283) * _e2660.member_2);
                                                                phi_18047_ = select(_e2719, vec3<f32>(0f, 0f, 0f), vec3(_e2721));
                                                            } else {
                                                                phi_18046_ = 0f;
                                                                phi_18047_ = vec3<f32>(0f, 0f, 0f);
                                                            }
                                                            let _e2725 = phi_18046_;
                                                            let _e2727 = phi_18047_;
                                                            phi_37016_ = _e2323;
                                                            phi_18048_ = _e2725;
                                                            phi_18049_ = vec3<f32>(_e2676, _e2677, _e2678);
                                                            phi_18050_ = _e2727;
                                                            phi_18051_ = ((_e2675 * 0.9999f) - _e1112);
                                                        } else {
                                                            phi_37016_ = _e2002;
                                                            phi_18048_ = 0f;
                                                            phi_18049_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_18050_ = vec3<f32>(0f, 0f, 0f);
                                                            phi_18051_ = 1000000000000000000000000000000f;
                                                        }
                                                        let _e2729 = phi_37016_;
                                                        let _e2731 = phi_18048_;
                                                        let _e2733 = phi_18049_;
                                                        let _e2735 = phi_18050_;
                                                        let _e2737 = phi_18051_;
                                                        phi_37015_ = _e2729;
                                                        phi_18052_ = _e2731;
                                                        phi_18053_ = _e2733;
                                                        phi_18054_ = _e2735;
                                                        phi_18055_ = _e2737;
                                                    }
                                                    let _e2739 = phi_37015_;
                                                    let _e2741 = phi_18052_;
                                                    let _e2743 = phi_18053_;
                                                    let _e2745 = phi_18054_;
                                                    let _e2747 = phi_18055_;
                                                    phi_37007_ = _e2739;
                                                    phi_18414_ = _e2741;
                                                    phi_18415_ = _e2743;
                                                    phi_18416_ = _e2745;
                                                    phi_18417_ = _e2747;
                                                } else {
                                                    if ((_e392 & 2u) == 0u) {
                                                        phi_37008_ = _e2002;
                                                        phi_18411_ = 0f;
                                                        phi_18412_ = vec3<f32>(0f, 0f, 0f);
                                                        phi_18413_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        let _e2020 = ((_e2002 * 747796405u) + 2891336453u);
                                                        let _e2028 = (((_e2020 >> bitcast<u32>((((_e2020 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2020) * 277803737u);
                                                        let _e2031 = ((_e2028 >> bitcast<u32>(22i)) ^ _e2028);
                                                        let _e2035 = (f32((_e2031 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2037 = ((_e2031 * 747796405u) + 2891336453u);
                                                        let _e2045 = (((_e2037 >> bitcast<u32>((((_e2037 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e2037) * 277803737u);
                                                        let _e2048 = ((_e2045 >> bitcast<u32>(22i)) ^ _e2045);
                                                        let _e2052 = (f32((_e2048 >> bitcast<u32>(8i))) * 0.000000059604645f);
                                                        let _e2055 = global_1.member.member_3[1u];
                                                        let _e2058 = global_1.member.member_3[2u];
                                                        let _e2061 = global_1.member.member_2[3u];
                                                        phi_18076_ = 0u;
                                                        phi_18079_ = _e2058;
                                                        loop {
                                                            let _e2063 = phi_18076_;
                                                            let _e2065 = phi_18079_;
                                                            local_73 = _e2063;
                                                            local_76 = _e2063;
                                                            local_77 = _e2063;
                                                            let _e2067 = ((_e2063 + 1u) < _e2065);
                                                            if _e2067 {
                                                                let _e2069 = ((_e2063 + _e2065) / 2u);
                                                                let _e2072 = global_6.member[(_e2061 + _e2069)];
                                                                let _e2073 = (_e2072 <= _e2035);
                                                                phi_18077_ = select(_e2063, _e2069, _e2073);
                                                                phi_18080_ = select(_e2069, _e2065, _e2073);
                                                            } else {
                                                                phi_18077_ = u32();
                                                                phi_18080_ = u32();
                                                            }
                                                            let _e2077 = phi_18077_;
                                                            let _e2079 = phi_18080_;
                                                            phi_18076_ = _e2077;
                                                            phi_18079_ = _e2079;
                                                            if !(_e2067) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2083 = global_1.member.member_3[0u];
                                                        let _e2084 = (_e2055 + 1u);
                                                        let _e2086 = local_73;
                                                        let _e2088 = (_e2083 + (_e2086 * _e2084));
                                                        phi_18117_ = 0u;
                                                        phi_18120_ = _e2055;
                                                        loop {
                                                            let _e2090 = phi_18117_;
                                                            let _e2092 = phi_18120_;
                                                            local_74 = _e2090;
                                                            local_75 = _e2090;
                                                            let _e2094 = ((_e2090 + 1u) < _e2092);
                                                            if _e2094 {
                                                                let _e2096 = ((_e2090 + _e2092) / 2u);
                                                                let _e2099 = global_6.member[(_e2088 + _e2096)];
                                                                let _e2100 = (_e2099 <= _e2052);
                                                                phi_18118_ = select(_e2090, _e2096, _e2100);
                                                                phi_18121_ = select(_e2096, _e2092, _e2100);
                                                            } else {
                                                                phi_18118_ = u32();
                                                                phi_18121_ = u32();
                                                            }
                                                            let _e2104 = phi_18118_;
                                                            let _e2106 = phi_18121_;
                                                            phi_18117_ = _e2104;
                                                            phi_18120_ = _e2106;
                                                            if !(_e2094) {
                                                                break;
                                                            }
                                                        }
                                                        let _e2109 = local_74;
                                                        let _e2112 = local_75;
                                                        let _e2113 = (_e2088 + _e2112);
                                                        let _e2115 = global_6.member[_e2113];
                                                        let _e2118 = global_6.member[(_e2113 + 1u)];
                                                        if (_e2118 > _e2115) {
                                                            let _e2122 = ((_e2052 - _e2115) / (_e2118 - _e2115));
                                                            let _e2124 = select(_e2122, 0f, (_e2122 < 0f));
                                                            phi_18197_ = select(_e2124, 0.99999f, (_e2124 > 0.99999f));
                                                        } else {
                                                            phi_18197_ = 0.5f;
                                                        }
                                                        let _e2128 = phi_18197_;
                                                        let _e2130 = f32(_e2055);
                                                        let _e2133 = local_76;
                                                        let _e2136 = local_77;
                                                        let _e2137 = (_e2061 + _e2136);
                                                        let _e2139 = global_6.member[_e2137];
                                                        let _e2142 = global_6.member[(_e2137 + 1u)];
                                                        if (_e2142 > _e2139) {
                                                            let _e2146 = ((_e2035 - _e2139) / (_e2142 - _e2139));
                                                            let _e2148 = select(_e2146, 0f, (_e2146 < 0f));
                                                            phi_18246_ = select(_e2148, 0.99999f, (_e2148 > 0.99999f));
                                                        } else {
                                                            phi_18246_ = 0.5f;
                                                        }
                                                        let _e2152 = phi_18246_;
                                                        let _e2154 = f32(_e2058);
                                                        let _e2157 = ((((f32(_e2109) + _e2128) / _e2130) - 0.5f) * 6.2831855f);
                                                        let _e2158 = (((f32(_e2133) + _e2152) / _e2154) * 3.1415927f);
                                                        let _e2159 = sin(_e2158);
                                                        let _e2161 = (_e2159 * cos(_e2157));
                                                        let _e2162 = cos(_e2158);
                                                        let _e2164 = (_e2159 * sin(_e2157));
                                                        let _e2167 = ((atan2(_e2164, _e2161) * 0.15915494f) + 0.5f);
                                                        let _e2171 = select(_e2162, -1f, (_e2162 < -1f));
                                                        let _e2174 = acos(select(_e2171, 1f, (_e2171 > 1f)));
                                                        let _e2176 = ((_e2167 - floor(_e2167)) * _e2130);
                                                        let _e2181 = select(select(u32(_e2176), 0u, (_e2176 < 0f)), 4294967295u, (_e2176 > 4294967000f));
                                                        let _e2182 = (_e2055 - 1u);
                                                        let _e2185 = ((_e2174 * 0.31830987f) * _e2154);
                                                        let _e2190 = select(select(u32(_e2185), 0u, (_e2185 < 0f)), 4294967295u, (_e2185 > 4294967000f));
                                                        let _e2191 = (_e2058 - 1u);
                                                        let _e2193 = select(_e2190, _e2191, (_e2191 < _e2190));
                                                        let _e2194 = sin(_e2174);
                                                        if (_e2194 <= 0f) {
                                                            phi_18402_ = 0f;
                                                        } else {
                                                            let _e2198 = (_e2061 + _e2193);
                                                            let _e2201 = global_6.member[(_e2198 + 1u)];
                                                            let _e2203 = global_6.member[_e2198];
                                                            let _e2206 = ((_e2083 + (_e2193 * _e2084)) + select(_e2181, _e2182, (_e2182 < _e2181)));
                                                            let _e2209 = global_6.member[(_e2206 + 1u)];
                                                            let _e2211 = global_6.member[_e2206];
                                                            phi_18402_ = ((((_e2201 - _e2203) * _e2154) * ((_e2209 - _e2211) * _e2130)) / (19.73921f * _e2194));
                                                        }
                                                        let _e2218 = phi_18402_;
                                                        let _e2219 = (_e2218 > 0f);
                                                        if _e2219 {
                                                            phi_18409_ = vec3<f32>(_e2161, _e2162, _e2164);
                                                        } else {
                                                            phi_18409_ = vec3<f32>(0f, 0f, 0f);
                                                        }
                                                        let _e2222 = phi_18409_;
                                                        phi_37008_ = _e2048;
                                                        phi_18411_ = select(0f, _e2218, _e2219);
                                                        phi_18412_ = _e2222;
                                                        phi_18413_ = select(vec3<f32>(0f, 0f, 0f), vec3<f32>(1f, 1f, 1f), vec3(_e2219));
                                                    }
                                                    let _e2227 = phi_37008_;
                                                    let _e2229 = phi_18411_;
                                                    let _e2231 = phi_18412_;
                                                    let _e2233 = phi_18413_;
                                                    phi_37007_ = _e2227;
                                                    phi_18414_ = _e2229;
                                                    phi_18415_ = _e2231;
                                                    phi_18416_ = _e2233;
                                                    phi_18417_ = 1000000000000000000000000000000f;
                                                }
                                                let _e2749 = phi_37007_;
                                                let _e2751 = phi_18414_;
                                                let _e2753 = phi_18415_;
                                                let _e2755 = phi_18416_;
                                                let _e2757 = phi_18417_;
                                                phi_37006_ = _e2749;
                                                phi_18418_ = _e2751;
                                                phi_18419_ = _e2753;
                                                phi_18420_ = _e2755;
                                                phi_18421_ = _e2757;
                                            }
                                            let _e3999 = phi_37006_;
                                            let _e4001 = phi_18418_;
                                            let _e4003 = phi_18419_;
                                            let _e4005 = phi_18420_;
                                            let _e4007 = phi_18421_;
                                            phi_37005_ = _e3999;
                                            phi_18422_ = _e4001;
                                            phi_18423_ = _e4003;
                                            phi_18424_ = _e4005;
                                            phi_18425_ = _e4007;
                                        }
                                        let _e4076 = phi_37005_;
                                        let _e4078 = phi_18422_;
                                        let _e4080 = phi_18423_;
                                        let _e4082 = phi_18424_;
                                        let _e4084 = phi_18425_;
                                        if ((((_e4082.x * _e4082.x) + (_e4082.y * _e4082.y)) + (_e4082.z * _e4082.z)) > 0f) {
                                            let _e4101 = (((_e4080.x * _e1094.x) + (_e4080.y * _e1094.y)) + (_e4080.z * _e1094.z));
                                            if (_e4101 != _e4101) {
                                                phi_29700_ = true;
                                            } else {
                                                phi_29700_ = (0f >= _e4101);
                                            }
                                            let _e4105 = phi_29700_;
                                            let _e4107 = (select(_e4101, 0f, _e4105) * 0.31830987f);
                                            if (_e1987 == 0u) {
                                                if (_e4101 <= 0f) {
                                                    phi_30497_ = vec3<f32>(0f, 0f, 0f);
                                                } else {
                                                    phi_30497_ = (_e1981 * (_e4101 * 0.31830987f));
                                                }
                                                let _e4583 = phi_30497_;
                                                phi_18821_ = type_59(_e4583, _e4107);
                                                phi_18822_ = type_59();
                                                phi_18823_ = true;
                                            } else {
                                                let _e4113 = (((_e1068 * _e1142) + (_e1069 * _e1143)) + (_e1070 * _e1145));
                                                let _e4118 = (((_e1068 * _e1138) + (_e1069 * _e1148)) + (_e1070 * _e1149));
                                                let _e4123 = (((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z));
                                                let _e4128 = (((_e4080.x * _e1142) + (_e4080.y * _e1143)) + (_e4080.z * _e1145));
                                                let _e4133 = (((_e4080.x * _e1138) + (_e4080.y * _e1148)) + (_e4080.z * _e1149));
                                                if (_e4123 <= 0f) {
                                                    phi_18785_ = type_59();
                                                    phi_18786_ = true;
                                                } else {
                                                    let _e4135 = (_e4101 <= 0f);
                                                    if _e4135 {
                                                        phi_18784_ = type_59();
                                                    } else {
                                                        let _e4136 = (_e4113 + _e4128);
                                                        let _e4137 = (_e4118 + _e4133);
                                                        let _e4138 = (_e4123 + _e4101);
                                                        let _e4145 = (1f / sqrt((((_e4136 * _e4136) + (_e4137 * _e4137)) + (_e4138 * _e4138))));
                                                        let _e4148 = (_e4138 * _e4145);
                                                        if (_e4148 <= 0f) {
                                                            phi_29810_ = 0f;
                                                        } else {
                                                            let _e4150 = (_e1985 * _e1985);
                                                            let _e4154 = ((((_e4150 - 1f) * _e4148) * _e4148) + 1f);
                                                            phi_29810_ = (_e4150 / ((3.1415927f * _e4154) * _e4154));
                                                        }
                                                        let _e4159 = phi_29810_;
                                                        let _e4160 = (_e4123 * _e4123);
                                                        if (_e4160 >= 1f) {
                                                            phi_29838_ = 0f;
                                                        } else {
                                                            phi_29838_ = ((sqrt((1f + (((_e1985 * _e1985) * ((_e4113 * _e4113) + (_e4118 * _e4118))) / _e4160))) - 1f) * 0.5f);
                                                        }
                                                        let _e4173 = phi_29838_;
                                                        let _e4174 = (1f + _e4173);
                                                        let _e4175 = (_e4101 * _e4101);
                                                        if (_e4175 >= 1f) {
                                                            phi_29866_ = 0f;
                                                        } else {
                                                            phi_29866_ = ((sqrt((1f + (((_e1985 * _e1985) * ((_e4128 * _e4128) + (_e4133 * _e4133))) / _e4175))) - 1f) * 0.5f);
                                                        }
                                                        let _e4188 = phi_29866_;
                                                        let _e4195 = (((_e4113 * (_e4136 * _e4145)) + (_e4118 * (_e4137 * _e4145))) + (_e4123 * _e4148));
                                                        if (_e1983 == 0u) {
                                                            let _e4490 = (1f - _e4195);
                                                            if (_e4490 != _e4490) {
                                                                phi_30369_ = true;
                                                            } else {
                                                                phi_30369_ = (0f >= _e4490);
                                                            }
                                                            let _e4494 = phi_30369_;
                                                            let _e4496 = pow(select(_e4490, 0f, _e4494), 5f);
                                                            let _e4497 = (1f - _e4496);
                                                            phi_18772_ = vec3<f32>(((_e1981.x * _e4497) + _e4496), ((_e1981.y * _e4497) + _e4496), ((_e1981.z * _e4497) + _e4496));
                                                        } else {
                                                            if (_e1983 == 1u) {
                                                                let _e4463 = (1f / _e1977);
                                                                let _e4467 = ((_e4463 * _e4463) * (1f - (_e4195 * _e4195)));
                                                                if (_e4467 >= 1f) {
                                                                    phi_30330_ = 1f;
                                                                } else {
                                                                    let _e4470 = sqrt((1f - _e4467));
                                                                    if (_e4195 != _e4195) {
                                                                        phi_30341_ = true;
                                                                    } else {
                                                                        phi_30341_ = (_e4470 <= _e4195);
                                                                    }
                                                                    let _e4474 = phi_30341_;
                                                                    let _e4479 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                                    phi_30330_ = (_e4479 + ((1f - _e4479) * pow((1f - select(_e4195, _e4470, _e4474)), 5f)));
                                                                }
                                                                let _e4486 = phi_30330_;
                                                                phi_18771_ = vec3(_e4486);
                                                            } else {
                                                                let _e4199 = select(_e4195, 0f, (_e4195 < 0f));
                                                                let _e4201 = select(_e4199, 1f, (_e4199 > 1f));
                                                                let _e4203 = (1f - (_e4201 * _e4201));
                                                                let _e4216 = ((_e1981.x * _e1981.x) - (_e1979.x * _e1979.x));
                                                                let _e4217 = ((_e1981.y * _e1981.y) - (_e1979.y * _e1979.y));
                                                                let _e4218 = ((_e1981.z * _e1981.z) - (_e1979.z * _e1979.z));
                                                                let _e4220 = (_e1981.x * (_e1979.x + _e1979.x));
                                                                let _e4222 = (_e1981.y * (_e1979.y + _e1979.y));
                                                                let _e4224 = (_e1981.z * (_e1979.z + _e1979.z));
                                                                let _e4231 = ((_e4216 * _e4216) + (_e4220 * _e4220));
                                                                let _e4232 = ((_e4217 * _e4217) + (_e4222 * _e4222));
                                                                let _e4233 = ((_e4218 * _e4218) + (_e4224 * _e4224));
                                                                let _e4238 = ((_e4203 * _e4216) / _e4231);
                                                                let _e4239 = ((_e4203 * _e4217) / _e4232);
                                                                let _e4240 = ((_e4203 * _e4218) / _e4233);
                                                                let _e4241 = (1f - _e4238);
                                                                let _e4242 = (1f - _e4239);
                                                                let _e4243 = (1f - _e4240);
                                                                let _e4245 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e4238, _e4239, _e4240));
                                                                let _e4246 = (_e4203 * _e4220);
                                                                let _e4247 = (_e4203 * _e4222);
                                                                let _e4248 = (_e4203 * _e4224);
                                                                let _e4253 = (vec3<f32>(_e4246, _e4247, _e4248) / vec3<f32>(_e4231, _e4232, _e4233));
                                                                let _e4266 = ((_e4245.x * _e4245.x) + (_e4253.x * _e4253.x));
                                                                let _e4267 = ((_e4245.y * _e4245.y) + (_e4253.y * _e4253.y));
                                                                let _e4268 = ((_e4245.z * _e4245.z) + (_e4253.z * _e4253.z));
                                                                if (_e4266 != _e4266) {
                                                                    phi_30000_ = true;
                                                                } else {
                                                                    phi_30000_ = (0f >= _e4266);
                                                                }
                                                                let _e4272 = phi_30000_;
                                                                let _e4274 = sqrt(select(_e4266, 0f, _e4272));
                                                                if (_e4267 != _e4267) {
                                                                    phi_30015_ = true;
                                                                } else {
                                                                    phi_30015_ = (0f >= _e4267);
                                                                }
                                                                let _e4278 = phi_30015_;
                                                                let _e4280 = sqrt(select(_e4267, 0f, _e4278));
                                                                if (_e4268 != _e4268) {
                                                                    phi_30030_ = true;
                                                                } else {
                                                                    phi_30030_ = (0f >= _e4268);
                                                                }
                                                                let _e4284 = phi_30030_;
                                                                let _e4286 = sqrt(select(_e4268, 0f, _e4284));
                                                                let _e4290 = ((_e4274 + _e4241) * 0.5f);
                                                                let _e4291 = ((_e4280 + _e4242) * 0.5f);
                                                                let _e4292 = ((_e4286 + _e4243) * 0.5f);
                                                                if (_e4290 != _e4290) {
                                                                    phi_30057_ = true;
                                                                } else {
                                                                    phi_30057_ = (0f >= _e4290);
                                                                }
                                                                let _e4296 = phi_30057_;
                                                                let _e4298 = sqrt(select(_e4290, 0f, _e4296));
                                                                if (_e4291 != _e4291) {
                                                                    phi_30072_ = true;
                                                                } else {
                                                                    phi_30072_ = (0f >= _e4291);
                                                                }
                                                                let _e4302 = phi_30072_;
                                                                let _e4304 = sqrt(select(_e4291, 0f, _e4302));
                                                                if (_e4292 != _e4292) {
                                                                    phi_30087_ = true;
                                                                } else {
                                                                    phi_30087_ = (0f >= _e4292);
                                                                }
                                                                let _e4308 = phi_30087_;
                                                                let _e4310 = sqrt(select(_e4292, 0f, _e4308));
                                                                let _e4311 = vec3<f32>(_e4298, _e4304, _e4310);
                                                                let _e4315 = ((_e4274 - _e4241) * 0.5f);
                                                                let _e4316 = ((_e4280 - _e4242) * 0.5f);
                                                                let _e4317 = ((_e4286 - _e4243) * 0.5f);
                                                                if (_e4315 != _e4315) {
                                                                    phi_30114_ = true;
                                                                } else {
                                                                    phi_30114_ = (0f >= _e4315);
                                                                }
                                                                let _e4321 = phi_30114_;
                                                                let _e4323 = sqrt(select(_e4315, 0f, _e4321));
                                                                if (_e4316 != _e4316) {
                                                                    phi_30129_ = true;
                                                                } else {
                                                                    phi_30129_ = (0f >= _e4316);
                                                                }
                                                                let _e4327 = phi_30129_;
                                                                let _e4329 = sqrt(select(_e4316, 0f, _e4327));
                                                                if (_e4317 != _e4317) {
                                                                    phi_30144_ = true;
                                                                } else {
                                                                    phi_30144_ = (0f >= _e4317);
                                                                }
                                                                let _e4333 = phi_30144_;
                                                                let _e4335 = sqrt(select(_e4317, 0f, _e4333));
                                                                if ((_e4246 / _e4231) < 0f) {
                                                                    phi_18648_ = -(_e4323);
                                                                } else {
                                                                    phi_18648_ = _e4323;
                                                                }
                                                                let _e4339 = phi_18648_;
                                                                if ((_e4247 / _e4232) < 0f) {
                                                                    phi_18656_ = -(_e4329);
                                                                } else {
                                                                    phi_18656_ = _e4329;
                                                                }
                                                                let _e4343 = phi_18656_;
                                                                if ((_e4248 / _e4233) < 0f) {
                                                                    phi_18664_ = -(_e4335);
                                                                } else {
                                                                    phi_18664_ = _e4335;
                                                                }
                                                                let _e4347 = phi_18664_;
                                                                let _e4351 = (_e1979.x * _e4201);
                                                                let _e4352 = (_e1979.y * _e4201);
                                                                let _e4353 = (_e1979.z * _e4201);
                                                                let _e4363 = vec3<f32>(((_e1981.x * _e4298) - (_e1979.x * _e4339)), ((_e1981.y * _e4304) - (_e1979.y * _e4343)), ((_e1981.z * _e4310) - (_e1979.z * _e4347)));
                                                                let _e4370 = ((_e1981.x * _e4339) + (_e1979.x * _e4298));
                                                                let _e4371 = ((_e1981.y * _e4343) + (_e1979.y * _e4304));
                                                                let _e4372 = ((_e1981.z * _e4347) + (_e1979.z * _e4310));
                                                                let _e4374 = vec3<f32>((_e1981.x * _e4201), (_e1981.y * _e4201), (_e1981.z * _e4201));
                                                                let _e4375 = (_e4374 - _e4311);
                                                                let _e4376 = (_e4351 - _e4339);
                                                                let _e4377 = (_e4352 - _e4343);
                                                                let _e4378 = (_e4353 - _e4347);
                                                                let _e4391 = (_e4374 + _e4311);
                                                                let _e4392 = (_e4351 + _e4339);
                                                                let _e4393 = (_e4352 + _e4343);
                                                                let _e4394 = (_e4353 + _e4347);
                                                                let _e4404 = ((_e4391.x * _e4391.x) + (_e4392 * _e4392));
                                                                let _e4405 = ((_e4391.y * _e4391.y) + (_e4393 * _e4393));
                                                                let _e4406 = ((_e4391.z * _e4391.z) + (_e4394 * _e4394));
                                                                let _e4416 = vec3(_e4201);
                                                                let _e4417 = (_e4416 - _e4363);
                                                                let _e4418 = -(vec3<f32>(_e4370, _e4371, _e4372));
                                                                let _e4434 = (_e4416 + _e4363);
                                                                let _e4444 = ((_e4434.x * _e4434.x) + (_e4370 * _e4370));
                                                                let _e4445 = ((_e4434.y * _e4434.y) + (_e4371 * _e4371));
                                                                let _e4446 = ((_e4434.z * _e4434.z) + (_e4372 * _e4372));
                                                                phi_18771_ = vec3<f32>((((((_e4375.x * _e4375.x) + (_e4376 * _e4376)) / select(0.000000000000000000000000000001f, _e4404, (_e4404 > 0.000000000000000000000000000001f))) + (((_e4417.x * _e4417.x) + (_e4418.x * _e4418.x)) / select(0.000000000000000000000000000001f, _e4444, (_e4444 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4375.y * _e4375.y) + (_e4377 * _e4377)) / select(0.000000000000000000000000000001f, _e4405, (_e4405 > 0.000000000000000000000000000001f))) + (((_e4417.y * _e4417.y) + (_e4418.y * _e4418.y)) / select(0.000000000000000000000000000001f, _e4445, (_e4445 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e4375.z * _e4375.z) + (_e4378 * _e4378)) / select(0.000000000000000000000000000001f, _e4406, (_e4406 > 0.000000000000000000000000000001f))) + (((_e4417.z * _e4417.z) + (_e4418.z * _e4418.z)) / select(0.000000000000000000000000000001f, _e4446, (_e4446 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                            }
                                                            let _e4489 = phi_18771_;
                                                            phi_18772_ = _e4489;
                                                        }
                                                        let _e4509 = phi_18772_;
                                                        let _e4511 = (4f * _e4123);
                                                        phi_18784_ = type_59((_e4509 * (((1f / (_e4174 + _e4188)) * _e4159) / _e4511)), ((_e4159 / _e4174) / _e4511));
                                                    }
                                                    let _e4518 = phi_18784_;
                                                    phi_18785_ = _e4518;
                                                    phi_18786_ = _e4135;
                                                }
                                                let _e4520 = phi_18785_;
                                                let _e4522 = phi_18786_;
                                                if _e4522 {
                                                    phi_18791_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                                } else {
                                                    phi_18791_ = _e4520;
                                                }
                                                let _e4524 = phi_18791_;
                                                let _e4527 = (_e1987 == 1u);
                                                if _e4527 {
                                                    phi_18819_ = _e4524;
                                                    phi_18820_ = type_59();
                                                } else {
                                                    if (_e4123 != _e4123) {
                                                        phi_30406_ = true;
                                                    } else {
                                                        phi_30406_ = (0f >= _e4123);
                                                    }
                                                    let _e4531 = phi_30406_;
                                                    let _e4532 = select(_e4123, 0f, _e4531);
                                                    let _e4533 = (1f / _e1977);
                                                    let _e4537 = ((_e4533 * _e4533) * (1f - (_e4532 * _e4532)));
                                                    if (_e4537 >= 1f) {
                                                        phi_30425_ = 1f;
                                                    } else {
                                                        let _e4540 = sqrt((1f - _e4537));
                                                        if (_e4532 != _e4532) {
                                                            phi_30436_ = true;
                                                        } else {
                                                            phi_30436_ = (_e4540 <= _e4532);
                                                        }
                                                        let _e4544 = phi_30436_;
                                                        let _e4549 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                        phi_30425_ = (_e4549 + ((1f - _e4549) * pow((1f - select(_e4532, _e4540, _e4544)), 5f)));
                                                    }
                                                    let _e4556 = phi_30425_;
                                                    if (_e4101 <= 0f) {
                                                        phi_30475_ = vec3<f32>(0f, 0f, 0f);
                                                    } else {
                                                        phi_30475_ = (_e1975 * (_e4101 * 0.31830987f));
                                                    }
                                                    let _e4561 = phi_30475_;
                                                    let _e4562 = (1f - _e4556);
                                                    phi_18819_ = type_59();
                                                    phi_18820_ = type_59((_e4524.member + vec3<f32>((_e4561.x * _e4562), (_e4561.y * _e4562), (_e4561.z * _e4562))), ((_e4556 * _e4524.member_1) + (_e4562 * _e4107)));
                                                }
                                                let _e4576 = phi_18819_;
                                                let _e4578 = phi_18820_;
                                                phi_18821_ = _e4576;
                                                phi_18822_ = _e4578;
                                                phi_18823_ = _e4527;
                                            }
                                            let _e4586 = phi_18821_;
                                            let _e4588 = phi_18822_;
                                            let _e4590 = phi_18823_;
                                            if _e4590 {
                                                phi_18827_ = _e4586;
                                            } else {
                                                phi_18827_ = _e4588;
                                            }
                                            let _e4592 = phi_18827_;
                                            if (_e4078 > 0f) {
                                                if (_e4078 >= 300000000000000000000000000000000000000f) {
                                                    phi_30514_ = 1f;
                                                } else {
                                                    let _e4597 = (_e4078 * _e4078);
                                                    let _e4599 = (_e4597 + (_e4592.member_1 * _e4592.member_1));
                                                    if (_e4599 == 0f) {
                                                        phi_30511_ = 0f;
                                                    } else {
                                                        phi_30511_ = (_e4597 / _e4599);
                                                    }
                                                    let _e4603 = phi_30511_;
                                                    phi_30514_ = _e4603;
                                                }
                                                let _e4605 = phi_30514_;
                                                phi_18836_ = (_e4605 / _e4078);
                                            } else {
                                                phi_18836_ = 1f;
                                            }
                                            let _e4608 = phi_18836_;
                                            if ((((_e4592.member.x * _e4592.member.x) + (_e4592.member.y * _e4592.member.y)) + (_e4592.member.z * _e4592.member.z)) > 0f) {
                                                let _e4619 = (vec3<f32>(1f, 1f, 1f) / _e4080);
                                                local_67 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                                                local_68 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                                                phi_18869_ = 4294967295u;
                                                phi_18872_ = select(4294967295u, 0u, (_e847 > 0u));
                                                phi_18874_ = 0u;
                                                phi_18876_ = _e851;
                                                phi_18878_ = _e847;
                                                phi_18880_ = vec4<f32>(0f, 0f, 0f, 0f);
                                                phi_18882_ = _e4084;
                                                loop {
                                                    let _e4623 = phi_18869_;
                                                    let _e4625 = phi_18872_;
                                                    let _e4627 = phi_18874_;
                                                    let _e4629 = phi_18876_;
                                                    let _e4631 = phi_18878_;
                                                    let _e4633 = phi_18880_;
                                                    let _e4635 = phi_18882_;
                                                    local_80 = _e4623;
                                                    local_81 = _e4633;
                                                    local_82 = _e4631;
                                                    if (_e4631 < _e4629) {
                                                        let _e4998 = (3u * _e4631);
                                                        let _e5000 = global_2.member[_e4998];
                                                        let _e5003 = global_2.member[(_e4998 + 1u)];
                                                        let _e5009 = select(select(u32(_e5003.w), 0u, (_e5003.w < 0f)), 4294967295u, (_e5003.w > 4294967000f));
                                                        if (_e5009 == 0u) {
                                                            let _e5103 = global_2.member[(_e4998 + 2u)];
                                                            let _e5112 = ((_e4080.y * _e5103.z) - (_e5103.y * _e4080.z));
                                                            let _e5115 = ((_e4080.z * _e5103.x) - (_e5103.z * _e4080.x));
                                                            let _e5118 = ((_e4080.x * _e5103.y) - (_e5103.x * _e4080.y));
                                                            let _e5123 = (((_e5003.x * _e5112) + (_e5003.y * _e5115)) + (_e5003.z * _e5118));
                                                            if (abs(_e5123) < 0.000000000001f) {
                                                                phi_19067_ = vec4<f32>();
                                                                phi_19068_ = true;
                                                            } else {
                                                                let _e5126 = (1f / _e5123);
                                                                let _e5130 = (_e1119 - _e5000.x);
                                                                let _e5131 = (_e1120 - _e5000.y);
                                                                let _e5132 = (_e1121 - _e5000.z);
                                                                let _e5138 = ((((_e5130 * _e5112) + (_e5131 * _e5115)) + (_e5132 * _e5118)) * _e5126);
                                                                if (_e5138 < 0f) {
                                                                    phi_19057_ = vec4<f32>();
                                                                    phi_19058_ = true;
                                                                    phi_19059_ = false;
                                                                } else {
                                                                    let _e5140 = (_e5138 > 1f);
                                                                    if _e5140 {
                                                                        phi_19055_ = vec4<f32>();
                                                                        phi_19056_ = false;
                                                                    } else {
                                                                        let _e5143 = ((_e5131 * _e5003.z) - (_e5003.y * _e5132));
                                                                        let _e5146 = ((_e5132 * _e5003.x) - (_e5003.z * _e5130));
                                                                        let _e5149 = ((_e5130 * _e5003.y) - (_e5003.x * _e5131));
                                                                        let _e5155 = ((((_e4080.x * _e5143) + (_e4080.y * _e5146)) + (_e4080.z * _e5149)) * _e5126);
                                                                        if (_e5155 < 0f) {
                                                                            phi_19045_ = vec4<f32>();
                                                                            phi_19046_ = true;
                                                                            phi_19047_ = false;
                                                                        } else {
                                                                            let _e5158 = ((_e5138 + _e5155) > 1f);
                                                                            if _e5158 {
                                                                                phi_19043_ = vec4<f32>();
                                                                                phi_19044_ = false;
                                                                            } else {
                                                                                let _e5164 = ((((_e5103.x * _e5143) + (_e5103.y * _e5146)) + (_e5103.z * _e5149)) * _e5126);
                                                                                if (_e5164 <= 0f) {
                                                                                    phi_19038_ = vec4<f32>();
                                                                                    phi_19039_ = true;
                                                                                } else {
                                                                                    let _e5166 = (_e5164 >= _e4635);
                                                                                    if _e5166 {
                                                                                        phi_19037_ = vec4<f32>();
                                                                                    } else {
                                                                                        phi_19037_ = vec4<f32>(_e5164, _e5138, _e5155, 1f);
                                                                                    }
                                                                                    let _e5169 = phi_19037_;
                                                                                    phi_19038_ = _e5169;
                                                                                    phi_19039_ = _e5166;
                                                                                }
                                                                                let _e5171 = phi_19038_;
                                                                                let _e5173 = phi_19039_;
                                                                                phi_19043_ = _e5171;
                                                                                phi_19044_ = _e5173;
                                                                            }
                                                                            let _e5175 = phi_19043_;
                                                                            let _e5177 = phi_19044_;
                                                                            phi_19045_ = _e5175;
                                                                            phi_19046_ = _e5158;
                                                                            phi_19047_ = _e5177;
                                                                        }
                                                                        let _e5179 = phi_19045_;
                                                                        let _e5181 = phi_19046_;
                                                                        let _e5183 = phi_19047_;
                                                                        phi_19055_ = _e5179;
                                                                        phi_19056_ = select(_e5183, true, _e5181);
                                                                    }
                                                                    let _e5186 = phi_19055_;
                                                                    let _e5188 = phi_19056_;
                                                                    phi_19057_ = _e5186;
                                                                    phi_19058_ = _e5140;
                                                                    phi_19059_ = _e5188;
                                                                }
                                                                let _e5190 = phi_19057_;
                                                                let _e5192 = phi_19058_;
                                                                let _e5194 = phi_19059_;
                                                                phi_19067_ = _e5190;
                                                                phi_19068_ = select(_e5194, true, _e5192);
                                                            }
                                                            let _e5197 = phi_19067_;
                                                            let _e5199 = phi_19068_;
                                                            phi_19213_ = _e5197;
                                                            phi_19214_ = _e5199;
                                                        } else {
                                                            if (_e5009 == 1u) {
                                                                let _e5033 = (_e1119 - _e5000.x);
                                                                let _e5034 = (_e1120 - _e5000.y);
                                                                let _e5035 = (_e1121 - _e5000.z);
                                                                let _e5040 = (((_e5033 * _e4080.x) + (_e5034 * _e4080.y)) + (_e5035 * _e4080.z));
                                                                if (_e5040 > 0f) {
                                                                    phi_19107_ = select(true, false, ((((_e5033 * _e5033) + (_e5034 * _e5034)) + (_e5035 * _e5035)) > (_e5000.w * _e5000.w)));
                                                                } else {
                                                                    phi_19107_ = true;
                                                                }
                                                                let _e5052 = phi_19107_;
                                                                if _e5052 {
                                                                    let _e5056 = (_e5033 - (_e4080.x * _e5040));
                                                                    let _e5057 = (_e5034 - (_e4080.y * _e5040));
                                                                    let _e5058 = (_e5035 - (_e4080.z * _e5040));
                                                                    let _e5066 = ((_e5000.w * _e5000.w) - (((_e5056 * _e5056) + (_e5057 * _e5057)) + (_e5058 * _e5058)));
                                                                    if (_e5066 < 0f) {
                                                                        phi_19166_ = vec4<f32>();
                                                                        phi_19167_ = true;
                                                                    } else {
                                                                        let _e5068 = sqrt(_e5066);
                                                                        let _e5069 = -(_e5040);
                                                                        let _e5070 = (_e5069 - _e5068);
                                                                        if (_e5070 <= 0f) {
                                                                            phi_19144_ = true;
                                                                        } else {
                                                                            phi_19144_ = (_e5070 >= _e4635);
                                                                        }
                                                                        let _e5074 = phi_19144_;
                                                                        if _e5074 {
                                                                            phi_19150_ = (_e5069 + _e5068);
                                                                        } else {
                                                                            phi_19150_ = _e5070;
                                                                        }
                                                                        let _e5077 = phi_19150_;
                                                                        if (_e5077 <= 0f) {
                                                                            phi_19161_ = vec4<f32>();
                                                                            phi_19162_ = true;
                                                                        } else {
                                                                            let _e5079 = (_e5077 >= _e4635);
                                                                            if _e5079 {
                                                                                phi_19160_ = vec4<f32>();
                                                                            } else {
                                                                                phi_19160_ = vec4<f32>(_e5077, 0f, 0f, 1f);
                                                                            }
                                                                            let _e5082 = phi_19160_;
                                                                            phi_19161_ = _e5082;
                                                                            phi_19162_ = _e5079;
                                                                        }
                                                                        let _e5084 = phi_19161_;
                                                                        let _e5086 = phi_19162_;
                                                                        phi_19166_ = _e5084;
                                                                        phi_19167_ = _e5086;
                                                                    }
                                                                    let _e5088 = phi_19166_;
                                                                    let _e5090 = phi_19167_;
                                                                    phi_19171_ = _e5088;
                                                                    phi_19172_ = _e5090;
                                                                } else {
                                                                    phi_19171_ = vec4<f32>();
                                                                    phi_19172_ = true;
                                                                }
                                                                let _e5092 = phi_19171_;
                                                                let _e5094 = phi_19172_;
                                                                phi_19203_ = _e5092;
                                                                phi_19204_ = _e5094;
                                                            } else {
                                                                if (abs(_e4080.y) < 0.000000000001f) {
                                                                    phi_19201_ = vec4<f32>();
                                                                    phi_19202_ = true;
                                                                } else {
                                                                    let _e5016 = ((_e5000.x - _e1120) / _e4080.y);
                                                                    if (_e5016 <= 0f) {
                                                                        phi_19196_ = vec4<f32>();
                                                                        phi_19197_ = true;
                                                                    } else {
                                                                        let _e5018 = (_e5016 >= _e4635);
                                                                        if _e5018 {
                                                                            phi_19195_ = vec4<f32>();
                                                                        } else {
                                                                            phi_19195_ = vec4<f32>(_e5016, 0f, 0f, 1f);
                                                                        }
                                                                        let _e5021 = phi_19195_;
                                                                        phi_19196_ = _e5021;
                                                                        phi_19197_ = _e5018;
                                                                    }
                                                                    let _e5023 = phi_19196_;
                                                                    let _e5025 = phi_19197_;
                                                                    phi_19201_ = _e5023;
                                                                    phi_19202_ = _e5025;
                                                                }
                                                                let _e5027 = phi_19201_;
                                                                let _e5029 = phi_19202_;
                                                                phi_19203_ = _e5027;
                                                                phi_19204_ = _e5029;
                                                            }
                                                            let _e5096 = phi_19203_;
                                                            let _e5098 = phi_19204_;
                                                            phi_19213_ = _e5096;
                                                            phi_19214_ = select(true, false, select(true, false, _e5098));
                                                        }
                                                        let _e5201 = phi_19213_;
                                                        let _e5203 = phi_19214_;
                                                        let _e5206 = select(vec4<f32>(0f, 0f, 0f, 0f), _e5201, vec4(select(true, false, _e5203)));
                                                        let _e5208 = (_e5206.w > 0f);
                                                        if _e5208 {
                                                            phi_19235_ = _e5206.x;
                                                        } else {
                                                            phi_19235_ = _e4635;
                                                        }
                                                        let _e5211 = phi_19235_;
                                                        let _e5215 = select(true, false, _e5208);
                                                        if _e5215 {
                                                            phi_19241_ = (_e4631 + 1u);
                                                        } else {
                                                            phi_19241_ = u32();
                                                        }
                                                        let _e5218 = phi_19241_;
                                                        phi_18870_ = select(_e4623, _e4631, _e5208);
                                                        phi_18873_ = _e4625;
                                                        phi_18875_ = _e4627;
                                                        phi_18877_ = _e4629;
                                                        phi_18879_ = _e5218;
                                                        phi_18881_ = select(_e4633, _e5206, vec4(_e5208));
                                                        phi_18883_ = _e5211;
                                                        phi_19414_ = _e5206;
                                                        phi_19415_ = _e5215;
                                                        phi_19416_ = false;
                                                    } else {
                                                        if (_e4625 == 4294967295u) {
                                                            let _e4638 = (_e4627 == 0u);
                                                            if _e4638 {
                                                                phi_19271_ = u32();
                                                                phi_19272_ = u32();
                                                                phi_19274_ = false;
                                                                phi_19275_ = false;
                                                            } else {
                                                                let _e4639 = (_e4627 - 1u);
                                                                let _e4640 = (_e4639 < 64u);
                                                                if _e4640 {
                                                                } else {
                                                                    phi_36516_ = true;
                                                                    phi_22053_ = bool();
                                                                    phi_22052_ = vec4<f32>();
                                                                    break;
                                                                }
                                                                let _e4642 = local_68[_e4639];
                                                                let _e4643 = (_e4642 >= _e4635);
                                                                if _e4643 {
                                                                    phi_19269_ = u32();
                                                                } else {
                                                                    if _e4640 {
                                                                    } else {
                                                                        phi_36516_ = true;
                                                                        phi_22053_ = bool();
                                                                        phi_22052_ = vec4<f32>();
                                                                        break;
                                                                    }
                                                                    let _e4645 = local_67[_e4639];
                                                                    phi_19269_ = _e4645;
                                                                }
                                                                let _e4647 = phi_19269_;
                                                                phi_19271_ = _e4639;
                                                                phi_19272_ = _e4647;
                                                                phi_19274_ = select(true, false, _e4643);
                                                                phi_19275_ = _e4643;
                                                            }
                                                            let _e4650 = phi_19271_;
                                                            let _e4652 = phi_19272_;
                                                            let _e4654 = phi_19274_;
                                                            let _e4656 = phi_19275_;
                                                            phi_19276_ = _e4650;
                                                            phi_19277_ = _e4652;
                                                            phi_19278_ = _e4650;
                                                            phi_19279_ = _e4654;
                                                            phi_19280_ = _e4656;
                                                            phi_19281_ = _e4638;
                                                        } else {
                                                            phi_19276_ = u32();
                                                            phi_19277_ = _e4625;
                                                            phi_19278_ = _e4627;
                                                            phi_19279_ = true;
                                                            phi_19280_ = false;
                                                            phi_19281_ = false;
                                                        }
                                                        let _e4658 = phi_19276_;
                                                        let _e4660 = phi_19277_;
                                                        let _e4662 = phi_19278_;
                                                        let _e4664 = phi_19279_;
                                                        let _e4666 = phi_19280_;
                                                        let _e4668 = phi_19281_;
                                                        if _e4664 {
                                                            let _e4670 = ((_e4660 & 2147483648u) == 0u);
                                                            if _e4670 {
                                                                let _e4678 = global_3.member[_e4660];
                                                                if (_e4678.member_1 == 4294967295u) {
                                                                    phi_19310_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4690 = ((_e4678.member.x - _e1119) * _e4619.x);
                                                                    let _e4692 = ((_e4678.member.y - _e1120) * _e4619.y);
                                                                    let _e4694 = ((_e4678.member.z - _e1121) * _e4619.z);
                                                                    let _e4701 = ((_e4678.member_2.x - _e1119) * _e4619.x);
                                                                    let _e4702 = ((_e4678.member_2.y - _e1120) * _e4619.y);
                                                                    let _e4703 = ((_e4678.member_2.z - _e1121) * _e4619.z);
                                                                    if (_e4690 == _e4690) {
                                                                        let _e4705 = (_e4701 == _e4701);
                                                                        if _e4705 {
                                                                            let _e4706 = (_e4690 != _e4690);
                                                                            if _e4706 {
                                                                                phi_30612_ = true;
                                                                            } else {
                                                                                phi_30612_ = (_e4701 <= _e4690);
                                                                            }
                                                                            let _e4709 = phi_30612_;
                                                                            let _e4710 = select(_e4690, _e4701, _e4709);
                                                                            if _e4706 {
                                                                                phi_30642_ = true;
                                                                            } else {
                                                                                phi_30642_ = (_e4701 >= _e4690);
                                                                            }
                                                                            let _e4715 = phi_30642_;
                                                                            let _e4716 = select(_e4690, _e4701, _e4715);
                                                                            phi_30589_ = type_56(select(-300000000000000000000000000000000000000f, _e4710, (_e4710 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4716, (_e4716 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30589_ = type_56();
                                                                        }
                                                                        let _e4721 = phi_30589_;
                                                                        phi_30592_ = _e4721;
                                                                        phi_30593_ = select(true, false, _e4705);
                                                                    } else {
                                                                        phi_30592_ = type_56();
                                                                        phi_30593_ = true;
                                                                    }
                                                                    let _e4724 = phi_30592_;
                                                                    let _e4726 = phi_30593_;
                                                                    if _e4726 {
                                                                        phi_30598_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30598_ = _e4724;
                                                                    }
                                                                    let _e4728 = phi_30598_;
                                                                    if (_e4692 == _e4692) {
                                                                        let _e4732 = (_e4702 == _e4702);
                                                                        if _e4732 {
                                                                            let _e4733 = (_e4692 != _e4692);
                                                                            if _e4733 {
                                                                                phi_30699_ = true;
                                                                            } else {
                                                                                phi_30699_ = (_e4702 <= _e4692);
                                                                            }
                                                                            let _e4736 = phi_30699_;
                                                                            let _e4737 = select(_e4692, _e4702, _e4736);
                                                                            if (_e4728.member != _e4728.member) {
                                                                                phi_30714_ = true;
                                                                            } else {
                                                                                phi_30714_ = (_e4737 >= _e4728.member);
                                                                            }
                                                                            let _e4741 = phi_30714_;
                                                                            if _e4733 {
                                                                                phi_30729_ = true;
                                                                            } else {
                                                                                phi_30729_ = (_e4702 >= _e4692);
                                                                            }
                                                                            let _e4745 = phi_30729_;
                                                                            let _e4746 = select(_e4692, _e4702, _e4745);
                                                                            if (_e4728.member_1 != _e4728.member_1) {
                                                                                phi_30744_ = true;
                                                                            } else {
                                                                                phi_30744_ = (_e4746 <= _e4728.member_1);
                                                                            }
                                                                            let _e4750 = phi_30744_;
                                                                            phi_30676_ = type_56(select(_e4728.member, _e4737, _e4741), select(_e4728.member_1, _e4746, _e4750));
                                                                        } else {
                                                                            phi_30676_ = type_56();
                                                                        }
                                                                        let _e4754 = phi_30676_;
                                                                        phi_30679_ = _e4754;
                                                                        phi_30680_ = select(true, false, _e4732);
                                                                    } else {
                                                                        phi_30679_ = type_56();
                                                                        phi_30680_ = true;
                                                                    }
                                                                    let _e4757 = phi_30679_;
                                                                    let _e4759 = phi_30680_;
                                                                    if _e4759 {
                                                                        phi_30685_ = _e4728;
                                                                    } else {
                                                                        phi_30685_ = _e4757;
                                                                    }
                                                                    let _e4761 = phi_30685_;
                                                                    if (_e4694 == _e4694) {
                                                                        let _e4765 = (_e4703 == _e4703);
                                                                        if _e4765 {
                                                                            let _e4766 = (_e4694 != _e4694);
                                                                            if _e4766 {
                                                                                phi_30786_ = true;
                                                                            } else {
                                                                                phi_30786_ = (_e4703 <= _e4694);
                                                                            }
                                                                            let _e4769 = phi_30786_;
                                                                            let _e4770 = select(_e4694, _e4703, _e4769);
                                                                            if (_e4761.member != _e4761.member) {
                                                                                phi_30801_ = true;
                                                                            } else {
                                                                                phi_30801_ = (_e4770 >= _e4761.member);
                                                                            }
                                                                            let _e4774 = phi_30801_;
                                                                            if _e4766 {
                                                                                phi_30816_ = true;
                                                                            } else {
                                                                                phi_30816_ = (_e4703 >= _e4694);
                                                                            }
                                                                            let _e4778 = phi_30816_;
                                                                            let _e4779 = select(_e4694, _e4703, _e4778);
                                                                            if (_e4761.member_1 != _e4761.member_1) {
                                                                                phi_30831_ = true;
                                                                            } else {
                                                                                phi_30831_ = (_e4779 <= _e4761.member_1);
                                                                            }
                                                                            let _e4783 = phi_30831_;
                                                                            phi_30763_ = type_56(select(_e4761.member, _e4770, _e4774), select(_e4761.member_1, _e4779, _e4783));
                                                                        } else {
                                                                            phi_30763_ = type_56();
                                                                        }
                                                                        let _e4787 = phi_30763_;
                                                                        phi_30766_ = _e4787;
                                                                        phi_30767_ = select(true, false, _e4765);
                                                                    } else {
                                                                        phi_30766_ = type_56();
                                                                        phi_30767_ = true;
                                                                    }
                                                                    let _e4790 = phi_30766_;
                                                                    let _e4792 = phi_30767_;
                                                                    if _e4792 {
                                                                        phi_30772_ = _e4761;
                                                                    } else {
                                                                        phi_30772_ = _e4790;
                                                                    }
                                                                    let _e4794 = phi_30772_;
                                                                    if (_e4794.member_1 < 0f) {
                                                                        phi_30570_ = true;
                                                                    } else {
                                                                        if (_e4794.member > _e4794.member_1) {
                                                                            phi_30567_ = true;
                                                                        } else {
                                                                            phi_30567_ = (_e4794.member > _e4635);
                                                                        }
                                                                        let _e4801 = phi_30567_;
                                                                        phi_30570_ = _e4801;
                                                                    }
                                                                    let _e4803 = phi_30570_;
                                                                    phi_19310_ = select(_e4794.member, 300000000000000000000000000000000000000f, _e4803);
                                                                }
                                                                let _e4806 = phi_19310_;
                                                                if (_e4678.member_5 == 4294967295u) {
                                                                    phi_19319_ = 300000000000000000000000000000000000000f;
                                                                } else {
                                                                    let _e4818 = ((_e4678.member_4.x - _e1119) * _e4619.x);
                                                                    let _e4820 = ((_e4678.member_4.y - _e1120) * _e4619.y);
                                                                    let _e4822 = ((_e4678.member_4.z - _e1121) * _e4619.z);
                                                                    let _e4829 = ((_e4678.member_6.x - _e1119) * _e4619.x);
                                                                    let _e4830 = ((_e4678.member_6.y - _e1120) * _e4619.y);
                                                                    let _e4831 = ((_e4678.member_6.z - _e1121) * _e4619.z);
                                                                    if (_e4818 == _e4818) {
                                                                        let _e4833 = (_e4829 == _e4829);
                                                                        if _e4833 {
                                                                            let _e4834 = (_e4818 != _e4818);
                                                                            if _e4834 {
                                                                                phi_30933_ = true;
                                                                            } else {
                                                                                phi_30933_ = (_e4829 <= _e4818);
                                                                            }
                                                                            let _e4837 = phi_30933_;
                                                                            let _e4838 = select(_e4818, _e4829, _e4837);
                                                                            if _e4834 {
                                                                                phi_30963_ = true;
                                                                            } else {
                                                                                phi_30963_ = (_e4829 >= _e4818);
                                                                            }
                                                                            let _e4843 = phi_30963_;
                                                                            let _e4844 = select(_e4818, _e4829, _e4843);
                                                                            phi_30910_ = type_56(select(-300000000000000000000000000000000000000f, _e4838, (_e4838 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e4844, (_e4844 <= 300000000000000000000000000000000000000f)));
                                                                        } else {
                                                                            phi_30910_ = type_56();
                                                                        }
                                                                        let _e4849 = phi_30910_;
                                                                        phi_30913_ = _e4849;
                                                                        phi_30914_ = select(true, false, _e4833);
                                                                    } else {
                                                                        phi_30913_ = type_56();
                                                                        phi_30914_ = true;
                                                                    }
                                                                    let _e4852 = phi_30913_;
                                                                    let _e4854 = phi_30914_;
                                                                    if _e4854 {
                                                                        phi_30919_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                                                    } else {
                                                                        phi_30919_ = _e4852;
                                                                    }
                                                                    let _e4856 = phi_30919_;
                                                                    if (_e4820 == _e4820) {
                                                                        let _e4860 = (_e4830 == _e4830);
                                                                        if _e4860 {
                                                                            let _e4861 = (_e4820 != _e4820);
                                                                            if _e4861 {
                                                                                phi_31020_ = true;
                                                                            } else {
                                                                                phi_31020_ = (_e4830 <= _e4820);
                                                                            }
                                                                            let _e4864 = phi_31020_;
                                                                            let _e4865 = select(_e4820, _e4830, _e4864);
                                                                            if (_e4856.member != _e4856.member) {
                                                                                phi_31035_ = true;
                                                                            } else {
                                                                                phi_31035_ = (_e4865 >= _e4856.member);
                                                                            }
                                                                            let _e4869 = phi_31035_;
                                                                            if _e4861 {
                                                                                phi_31050_ = true;
                                                                            } else {
                                                                                phi_31050_ = (_e4830 >= _e4820);
                                                                            }
                                                                            let _e4873 = phi_31050_;
                                                                            let _e4874 = select(_e4820, _e4830, _e4873);
                                                                            if (_e4856.member_1 != _e4856.member_1) {
                                                                                phi_31065_ = true;
                                                                            } else {
                                                                                phi_31065_ = (_e4874 <= _e4856.member_1);
                                                                            }
                                                                            let _e4878 = phi_31065_;
                                                                            phi_30997_ = type_56(select(_e4856.member, _e4865, _e4869), select(_e4856.member_1, _e4874, _e4878));
                                                                        } else {
                                                                            phi_30997_ = type_56();
                                                                        }
                                                                        let _e4882 = phi_30997_;
                                                                        phi_31000_ = _e4882;
                                                                        phi_31001_ = select(true, false, _e4860);
                                                                    } else {
                                                                        phi_31000_ = type_56();
                                                                        phi_31001_ = true;
                                                                    }
                                                                    let _e4885 = phi_31000_;
                                                                    let _e4887 = phi_31001_;
                                                                    if _e4887 {
                                                                        phi_31006_ = _e4856;
                                                                    } else {
                                                                        phi_31006_ = _e4885;
                                                                    }
                                                                    let _e4889 = phi_31006_;
                                                                    if (_e4822 == _e4822) {
                                                                        let _e4893 = (_e4831 == _e4831);
                                                                        if _e4893 {
                                                                            let _e4894 = (_e4822 != _e4822);
                                                                            if _e4894 {
                                                                                phi_31107_ = true;
                                                                            } else {
                                                                                phi_31107_ = (_e4831 <= _e4822);
                                                                            }
                                                                            let _e4897 = phi_31107_;
                                                                            let _e4898 = select(_e4822, _e4831, _e4897);
                                                                            if (_e4889.member != _e4889.member) {
                                                                                phi_31122_ = true;
                                                                            } else {
                                                                                phi_31122_ = (_e4898 >= _e4889.member);
                                                                            }
                                                                            let _e4902 = phi_31122_;
                                                                            if _e4894 {
                                                                                phi_31137_ = true;
                                                                            } else {
                                                                                phi_31137_ = (_e4831 >= _e4822);
                                                                            }
                                                                            let _e4906 = phi_31137_;
                                                                            let _e4907 = select(_e4822, _e4831, _e4906);
                                                                            if (_e4889.member_1 != _e4889.member_1) {
                                                                                phi_31152_ = true;
                                                                            } else {
                                                                                phi_31152_ = (_e4907 <= _e4889.member_1);
                                                                            }
                                                                            let _e4911 = phi_31152_;
                                                                            phi_31084_ = type_56(select(_e4889.member, _e4898, _e4902), select(_e4889.member_1, _e4907, _e4911));
                                                                        } else {
                                                                            phi_31084_ = type_56();
                                                                        }
                                                                        let _e4915 = phi_31084_;
                                                                        phi_31087_ = _e4915;
                                                                        phi_31088_ = select(true, false, _e4893);
                                                                    } else {
                                                                        phi_31087_ = type_56();
                                                                        phi_31088_ = true;
                                                                    }
                                                                    let _e4918 = phi_31087_;
                                                                    let _e4920 = phi_31088_;
                                                                    if _e4920 {
                                                                        phi_31093_ = _e4889;
                                                                    } else {
                                                                        phi_31093_ = _e4918;
                                                                    }
                                                                    let _e4922 = phi_31093_;
                                                                    if (_e4922.member_1 < 0f) {
                                                                        phi_30891_ = true;
                                                                    } else {
                                                                        if (_e4922.member > _e4922.member_1) {
                                                                            phi_30888_ = true;
                                                                        } else {
                                                                            phi_30888_ = (_e4922.member > _e4635);
                                                                        }
                                                                        let _e4929 = phi_30888_;
                                                                        phi_30891_ = _e4929;
                                                                    }
                                                                    let _e4931 = phi_30891_;
                                                                    phi_19319_ = select(_e4922.member, 300000000000000000000000000000000000000f, _e4931);
                                                                }
                                                                let _e4934 = phi_19319_;
                                                                if (_e4678.member_3 > 0u) {
                                                                    phi_19330_ = ((2147483648u | ((_e4678.member_3 - 1u) << bitcast<u32>(28u))) | _e4678.member_1);
                                                                } else {
                                                                    phi_19330_ = _e4678.member_1;
                                                                }
                                                                let _e4943 = phi_19330_;
                                                                if (_e4678.member_7 > 0u) {
                                                                    phi_19341_ = ((2147483648u | ((_e4678.member_7 - 1u) << bitcast<u32>(28u))) | _e4678.member_5);
                                                                } else {
                                                                    phi_19341_ = _e4678.member_5;
                                                                }
                                                                let _e4952 = phi_19341_;
                                                                let _e4953 = (_e4806 <= _e4934);
                                                                let _e4957 = select(_e4806, _e4934, _e4953);
                                                                if (_e4957 < 300000000000000000000000000000000000000f) {
                                                                    let _e4959 = (_e4662 < 64u);
                                                                    if _e4959 {
                                                                        if _e4959 {
                                                                        } else {
                                                                            phi_36516_ = true;
                                                                            phi_22053_ = bool();
                                                                            phi_22052_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_67[_e4662] = select(_e4943, _e4952, _e4953);
                                                                        if _e4959 {
                                                                        } else {
                                                                            phi_36516_ = true;
                                                                            phi_22053_ = bool();
                                                                            phi_22052_ = vec4<f32>();
                                                                            break;
                                                                        }
                                                                        local_68[_e4662] = _e4957;
                                                                        phi_19380_ = (_e4662 + 1u);
                                                                    } else {
                                                                        phi_19380_ = _e4662;
                                                                    }
                                                                    let _e4964 = phi_19380_;
                                                                    phi_19381_ = _e4964;
                                                                } else {
                                                                    phi_19381_ = _e4662;
                                                                }
                                                                let _e4966 = phi_19381_;
                                                                phi_19392_ = select(4294967295u, select(_e4952, _e4943, _e4953), (select(_e4934, _e4806, _e4953) < 300000000000000000000000000000000000000f));
                                                                phi_19393_ = _e4966;
                                                                phi_19394_ = u32();
                                                                phi_19395_ = u32();
                                                            } else {
                                                                let _e4671 = (_e4660 & 268435455u);
                                                                phi_19392_ = u32();
                                                                phi_19393_ = u32();
                                                                phi_19394_ = ((_e4671 + ((_e4660 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                                                phi_19395_ = _e4671;
                                                            }
                                                            let _e4970 = phi_19392_;
                                                            let _e4972 = phi_19393_;
                                                            let _e4974 = phi_19394_;
                                                            let _e4976 = phi_19395_;
                                                            phi_19397_ = _e4970;
                                                            phi_19398_ = _e4972;
                                                            phi_19400_ = _e4974;
                                                            phi_19401_ = _e4976;
                                                            phi_19402_ = select(true, false, _e4670);
                                                            phi_19403_ = _e4670;
                                                        } else {
                                                            phi_19397_ = u32();
                                                            phi_19398_ = u32();
                                                            phi_19400_ = _e4629;
                                                            phi_19401_ = _e4631;
                                                            phi_19402_ = _e4666;
                                                            phi_19403_ = false;
                                                        }
                                                        let _e4979 = phi_19397_;
                                                        let _e4981 = phi_19398_;
                                                        let _e4983 = phi_19400_;
                                                        let _e4985 = phi_19401_;
                                                        let _e4987 = phi_19402_;
                                                        let _e4989 = phi_19403_;
                                                        phi_18870_ = _e4623;
                                                        phi_18873_ = select(_e4979, 4294967295u, _e4987);
                                                        phi_18875_ = select(_e4981, select(_e4658, _e4662, _e4664), _e4987);
                                                        phi_18877_ = select(_e4629, _e4983, _e4987);
                                                        phi_18879_ = select(_e4631, _e4985, _e4987);
                                                        phi_18881_ = _e4633;
                                                        phi_18883_ = _e4635;
                                                        phi_19414_ = vec4<f32>();
                                                        phi_19415_ = select(_e4989, true, _e4987);
                                                        phi_19416_ = select(select(_e4668, false, _e4664), false, _e4987);
                                                    }
                                                    let _e5220 = phi_18870_;
                                                    let _e5222 = phi_18873_;
                                                    let _e5224 = phi_18875_;
                                                    let _e5226 = phi_18877_;
                                                    let _e5228 = phi_18879_;
                                                    let _e5230 = phi_18881_;
                                                    let _e5232 = phi_18883_;
                                                    let _e5234 = phi_19414_;
                                                    let _e5236 = phi_19415_;
                                                    let _e5238 = phi_19416_;
                                                    phi_18869_ = _e5220;
                                                    phi_18872_ = _e5222;
                                                    phi_18874_ = _e5224;
                                                    phi_18876_ = _e5226;
                                                    phi_18878_ = _e5228;
                                                    phi_18880_ = _e5230;
                                                    phi_18882_ = _e5232;
                                                    phi_36516_ = _e2000;
                                                    phi_22053_ = _e5238;
                                                    phi_22052_ = _e5234;
                                                    if !(_e5236) {
                                                        break;
                                                    }
                                                }
                                                let _e5241 = phi_36516_;
                                                let _e5243 = phi_22053_;
                                                let _e5245 = phi_22052_;
                                                phi_36963_ = _e4076;
                                                phi_36955_ = _e5241;
                                                if _e5241 {
                                                    break;
                                                }
                                                if _e5243 {
                                                    let _e5247 = local_80;
                                                    let _e5249 = local_81;
                                                    phi_19421_ = type_79(_e5247, _e5249);
                                                } else {
                                                    phi_19421_ = type_79();
                                                }
                                                let _e5252 = phi_19421_;
                                                if select(true, false, _e5243) {
                                                    let _e5255 = local_82;
                                                    phi_19427_ = type_79(_e5255, _e5245);
                                                } else {
                                                    phi_19427_ = _e5252;
                                                }
                                                let _e5258 = phi_19427_;
                                                if (_e5258.member != 4294967295u) {
                                                    phi_19481_ = _e2004;
                                                    phi_19482_ = _e2006;
                                                } else {
                                                    if (_e2008 == (_e1998 + 2u)) {
                                                        phi_19479_ = _e2004;
                                                        phi_19480_ = type_41(vec4<f32>(_e1119, _e1120, _e1121, -1f), vec4<f32>(_e4080.x, _e4080.y, _e4080.z, 0f), vec4<f32>(((_e836.x * _e4592.member.x) * _e4608), ((_e836.y * _e4592.member.y) * _e4608), ((_e836.z * _e4592.member.z) * _e4608), 0f));
                                                    } else {
                                                        phi_19479_ = vec3<f32>((_e2004.x + (((_e836.x * _e4592.member.x) * _e4082.x) * _e4608)), (_e2004.y + (((_e836.y * _e4592.member.y) * _e4082.y) * _e4608)), (_e2004.z + (((_e836.z * _e4592.member.z) * _e4082.z) * _e4608)));
                                                        phi_19480_ = _e2006;
                                                    }
                                                    let _e5296 = phi_19479_;
                                                    let _e5298 = phi_19480_;
                                                    phi_19481_ = _e5296;
                                                    phi_19482_ = _e5298;
                                                }
                                                let _e5300 = phi_19481_;
                                                let _e5302 = phi_19482_;
                                                phi_36958_ = _e5241;
                                                phi_19483_ = _e5300;
                                                phi_19484_ = _e5302;
                                                phi_19485_ = (_e2010 + 1u);
                                            } else {
                                                phi_36958_ = _e2000;
                                                phi_19483_ = _e2004;
                                                phi_19484_ = _e2006;
                                                phi_19485_ = _e2010;
                                            }
                                            let _e5304 = phi_36958_;
                                            let _e5306 = phi_19483_;
                                            let _e5308 = phi_19484_;
                                            let _e5310 = phi_19485_;
                                            phi_36957_ = _e5304;
                                            phi_19486_ = _e5306;
                                            phi_19487_ = _e5308;
                                            phi_19488_ = _e5310;
                                        } else {
                                            phi_36957_ = _e2000;
                                            phi_19486_ = _e2004;
                                            phi_19487_ = _e2006;
                                            phi_19488_ = _e2010;
                                        }
                                        let _e5312 = phi_36957_;
                                        let _e5314 = phi_19486_;
                                        let _e5316 = phi_19487_;
                                        let _e5318 = phi_19488_;
                                        phi_37243_ = _e4076;
                                        phi_36956_ = _e5312;
                                        phi_14906_ = _e5314;
                                        phi_14909_ = _e5316;
                                        phi_14911_ = (_e2008 + 1u);
                                        phi_14913_ = _e5318;
                                    } else {
                                        phi_37243_ = _e2002;
                                        phi_36956_ = _e2000;
                                        phi_14906_ = vec3<f32>();
                                        phi_14909_ = type_41();
                                        phi_14911_ = u32();
                                        phi_14913_ = u32();
                                    }
                                    let _e5321 = phi_37243_;
                                    let _e5323 = phi_36956_;
                                    let _e5325 = phi_14906_;
                                    let _e5327 = phi_14909_;
                                    let _e5329 = phi_14911_;
                                    let _e5331 = phi_14913_;
                                    phi_36588_ = _e5323;
                                    phi_34663_ = _e5321;
                                    phi_14905_ = _e5325;
                                    phi_14908_ = _e5327;
                                    phi_14910_ = _e5329;
                                    phi_14912_ = _e5331;
                                    phi_36963_ = _e5321;
                                    phi_36955_ = _e5323;
                                    if !(_e2012) {
                                        break;
                                    }
                                }
                                let _e5334 = phi_36963_;
                                let _e5336 = phi_36955_;
                                if _e5336 {
                                    break;
                                }
                                if (_e370 >= _e389) {
                                    let _e6340 = local_98;
                                    let _e6343 = local_99;
                                    let _e6346 = local_100;
                                    let _e6349 = local_101;
                                    let _e6355 = local_102;
                                    phi_20039_ = type_74(type_34(_e345.member, _e345.member_1, vec4<f32>(_e836.x, _e836.y, _e836.z, f32(_e370)), vec4<f32>(_e6343.x, _e6346.y, _e6349.z, f32(_e6340)), vec4<u32>(_e5334, (_e1995 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, _e6355);
                                    phi_20040_ = type_74();
                                    phi_20041_ = true;
                                } else {
                                    let _e5339 = ((_e5334 * 747796405u) + 2891336453u);
                                    let _e5347 = (((_e5339 >> bitcast<u32>((((_e5339 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5339) * 277803737u);
                                    let _e5350 = ((_e5347 >> bitcast<u32>(22i)) ^ _e5347);
                                    let _e5353 = f32((_e5350 >> bitcast<u32>(8i)));
                                    let _e5356 = ((_e5350 * 747796405u) + 2891336453u);
                                    let _e5364 = (((_e5356 >> bitcast<u32>((((_e5356 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5356) * 277803737u);
                                    let _e5367 = ((_e5364 >> bitcast<u32>(22i)) ^ _e5364);
                                    let _e5370 = f32((_e5367 >> bitcast<u32>(8i)));
                                    let _e5372 = ((_e5367 * 747796405u) + 2891336453u);
                                    let _e5380 = (((_e5372 >> bitcast<u32>((((_e5372 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e5372) * 277803737u);
                                    let _e5383 = ((_e5380 >> bitcast<u32>(22i)) ^ _e5380);
                                    let _e5388 = (_e1987 == 0u);
                                    if _e5388 {
                                        phi_31262_ = vec3<f32>();
                                        phi_31263_ = true;
                                    } else {
                                        if (_e1987 == 2u) {
                                            let _e5394 = (((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z));
                                            if (_e5394 != _e5394) {
                                                phi_31303_ = true;
                                            } else {
                                                phi_31303_ = (0f >= _e5394);
                                            }
                                            let _e5398 = phi_31303_;
                                            let _e5399 = select(_e5394, 0f, _e5398);
                                            let _e5400 = (1f / _e1977);
                                            let _e5404 = ((_e5400 * _e5400) * (1f - (_e5399 * _e5399)));
                                            if (_e5404 >= 1f) {
                                                phi_31322_ = 1f;
                                            } else {
                                                let _e5407 = sqrt((1f - _e5404));
                                                if (_e5399 != _e5399) {
                                                    phi_31333_ = true;
                                                } else {
                                                    phi_31333_ = (_e5407 <= _e5399);
                                                }
                                                let _e5411 = phi_31333_;
                                                let _e5416 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                phi_31322_ = (_e5416 + ((1f - _e5416) * pow((1f - select(_e5399, _e5407, _e5411)), 5f)));
                                            }
                                            let _e5423 = phi_31322_;
                                            phi_31253_ = select(true, false, ((f32((_e5383 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e5423));
                                        } else {
                                            phi_31253_ = true;
                                        }
                                        let _e5427 = phi_31253_;
                                        if _e5427 {
                                            let _e5432 = (((_e1068 * _e1142) + (_e1069 * _e1143)) + (_e1070 * _e1145));
                                            let _e5437 = (((_e1068 * _e1138) + (_e1069 * _e1148)) + (_e1070 * _e1149));
                                            let _e5442 = (((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z));
                                            if (_e5442 <= 0f) {
                                                phi_31487_ = vec3<f32>();
                                                phi_31488_ = true;
                                            } else {
                                                let _e5445 = (_e1985 * _e5432);
                                                let _e5446 = (_e1985 * _e5437);
                                                let _e5453 = (1f / sqrt((((_e5445 * _e5445) + (_e5446 * _e5446)) + (_e5442 * _e5442))));
                                                let _e5454 = (_e5445 * _e5453);
                                                let _e5455 = (_e5446 * _e5453);
                                                let _e5456 = (_e5442 * _e5453);
                                                let _e5459 = ((_e5454 * _e5454) + (_e5455 * _e5455));
                                                if (_e5459 > 0f) {
                                                    let _e5462 = sqrt(_e5459);
                                                    phi_31389_ = vec3<f32>((-(_e5455) / _e5462), (_e5454 / _e5462), 0f);
                                                } else {
                                                    phi_31389_ = vec3<f32>(1f, 0f, 0f);
                                                }
                                                let _e5467 = phi_31389_;
                                                let _e5480 = sqrt((_e5353 * 0.000000059604645f));
                                                let _e5481 = (_e5370 * 0.00000037450704f);
                                                let _e5483 = (_e5480 * cos(_e5481));
                                                let _e5485 = (0.5f * (1f + _e5456));
                                                let _e5488 = (1f - (_e5483 * _e5483));
                                                if (_e5488 != _e5488) {
                                                    phi_31545_ = true;
                                                } else {
                                                    phi_31545_ = (0f >= _e5488);
                                                }
                                                let _e5492 = phi_31545_;
                                                let _e5499 = (((1f - _e5485) * sqrt(select(_e5488, 0f, _e5492))) + ((_e5485 * _e5480) * sin(_e5481)));
                                                let _e5510 = (_e5488 - (_e5499 * _e5499));
                                                if (_e5510 != _e5510) {
                                                    phi_31560_ = true;
                                                } else {
                                                    phi_31560_ = (0f >= _e5510);
                                                }
                                                let _e5514 = phi_31560_;
                                                let _e5516 = sqrt(select(_e5510, 0f, _e5514));
                                                let _e5522 = (((_e5467.z * _e5483) + (((_e5454 * _e5467.y) - (_e5467.x * _e5455)) * _e5499)) + (_e5456 * _e5516));
                                                let _e5523 = (_e1985 * (((_e5467.x * _e5483) + (((_e5455 * _e5467.z) - (_e5467.y * _e5456)) * _e5499)) + (_e5454 * _e5516)));
                                                let _e5524 = (_e1985 * (((_e5467.y * _e5483) + (((_e5456 * _e5467.x) - (_e5467.z * _e5454)) * _e5499)) + (_e5455 * _e5516)));
                                                if (_e5522 != _e5522) {
                                                    phi_31575_ = true;
                                                } else {
                                                    phi_31575_ = (0f >= _e5522);
                                                }
                                                let _e5528 = phi_31575_;
                                                let _e5529 = select(_e5522, 0f, _e5528);
                                                let _e5536 = (1f / sqrt((((_e5523 * _e5523) + (_e5524 * _e5524)) + (_e5529 * _e5529))));
                                                let _e5537 = (_e5523 * _e5536);
                                                let _e5538 = (_e5524 * _e5536);
                                                let _e5539 = (_e5529 * _e5536);
                                                let _e5545 = (2f * (((_e5432 * _e5537) + (_e5437 * _e5538)) + (_e5442 * _e5539)));
                                                let _e5548 = (_e5539 * _e5545);
                                                let _e5551 = (vec3<f32>((_e5537 * _e5545), (_e5538 * _e5545), _e5548) - vec3<f32>(_e5432, _e5437, _e5442));
                                                let _e5552 = ((_e5548 - _e5442) <= 0f);
                                                if _e5552 {
                                                    phi_31484_ = vec3<f32>();
                                                } else {
                                                    phi_31484_ = vec3<f32>((((_e1142 * _e5551.x) + (_e1138 * _e5551.y)) + (_e1094.x * _e5551.z)), (((_e1143 * _e5551.x) + (_e1148 * _e5551.y)) + (_e1094.y * _e5551.z)), (((_e1145 * _e5551.x) + (_e1149 * _e5551.y)) + (_e1094.z * _e5551.z)));
                                                }
                                                let _e5573 = phi_31484_;
                                                phi_31487_ = _e5573;
                                                phi_31488_ = _e5552;
                                            }
                                            let _e5575 = phi_31487_;
                                            let _e5577 = phi_31488_;
                                            phi_31258_ = select(_e5575, vec3<f32>(0f, 0f, 0f), vec3(_e5577));
                                        } else {
                                            phi_31258_ = vec3<f32>();
                                        }
                                        let _e5581 = phi_31258_;
                                        phi_31262_ = _e5581;
                                        phi_31263_ = select(true, false, _e5427);
                                    }
                                    let _e5584 = phi_31262_;
                                    let _e5586 = phi_31263_;
                                    if _e5586 {
                                        let _e5587 = (_e5353 * 0.00000037450704f);
                                        let _e5589 = ((_e5370 * 0.00000011920929f) - 1f);
                                        let _e5591 = (1f - (_e5589 * _e5589));
                                        if (_e5591 != _e5591) {
                                            phi_31675_ = true;
                                        } else {
                                            phi_31675_ = (0f >= _e5591);
                                        }
                                        let _e5595 = phi_31675_;
                                        let _e5597 = sqrt(select(_e5591, 0f, _e5595));
                                        let _e5602 = (_e1094.x + (_e5597 * cos(_e5587)));
                                        let _e5603 = (_e1094.y + (_e5597 * sin(_e5587)));
                                        let _e5604 = (_e1094.z + _e5589);
                                        let _e5610 = sqrt((((_e5602 * _e5602) + (_e5603 * _e5603)) + (_e5604 * _e5604)));
                                        if (_e5610 < 0.000001f) {
                                            phi_31649_ = _e1094;
                                        } else {
                                            let _e5612 = (1f / _e5610);
                                            phi_31649_ = vec3<f32>((_e5602 * _e5612), (_e5603 * _e5612), (_e5604 * _e5612));
                                        }
                                        let _e5618 = phi_31649_;
                                        phi_31270_ = _e5618;
                                    } else {
                                        phi_31270_ = _e5584;
                                    }
                                    let _e5620 = phi_31270_;
                                    let _e5628 = (((_e5620.x * _e1094.x) + (_e5620.y * _e1094.y)) + (_e5620.z * _e1094.z));
                                    if (_e5628 != _e5628) {
                                        phi_31690_ = true;
                                    } else {
                                        phi_31690_ = (0f >= _e5628);
                                    }
                                    let _e5632 = phi_31690_;
                                    let _e5634 = (select(_e5628, 0f, _e5632) * 0.31830987f);
                                    if _e5388 {
                                        if (_e5628 <= 0f) {
                                            phi_32487_ = vec3<f32>(0f, 0f, 0f);
                                        } else {
                                            phi_32487_ = (_e1981 * (_e5628 * 0.31830987f));
                                        }
                                        let _e6109 = phi_32487_;
                                        phi_19880_ = type_59(_e6109, _e5634);
                                        phi_19881_ = type_59();
                                        phi_19882_ = true;
                                    } else {
                                        let _e5639 = (((_e1068 * _e1142) + (_e1069 * _e1143)) + (_e1070 * _e1145));
                                        let _e5644 = (((_e1068 * _e1138) + (_e1069 * _e1148)) + (_e1070 * _e1149));
                                        let _e5649 = (((_e1068 * _e1094.x) + (_e1069 * _e1094.y)) + (_e1070 * _e1094.z));
                                        let _e5654 = (((_e5620.x * _e1142) + (_e5620.y * _e1143)) + (_e5620.z * _e1145));
                                        let _e5659 = (((_e5620.x * _e1138) + (_e5620.y * _e1148)) + (_e5620.z * _e1149));
                                        if (_e5649 <= 0f) {
                                            phi_19844_ = type_59();
                                            phi_19845_ = true;
                                        } else {
                                            let _e5661 = (_e5628 <= 0f);
                                            if _e5661 {
                                                phi_19843_ = type_59();
                                            } else {
                                                let _e5662 = (_e5639 + _e5654);
                                                let _e5663 = (_e5644 + _e5659);
                                                let _e5664 = (_e5649 + _e5628);
                                                let _e5671 = (1f / sqrt((((_e5662 * _e5662) + (_e5663 * _e5663)) + (_e5664 * _e5664))));
                                                let _e5674 = (_e5664 * _e5671);
                                                if (_e5674 <= 0f) {
                                                    phi_31800_ = 0f;
                                                } else {
                                                    let _e5676 = (_e1985 * _e1985);
                                                    let _e5680 = ((((_e5676 - 1f) * _e5674) * _e5674) + 1f);
                                                    phi_31800_ = (_e5676 / ((3.1415927f * _e5680) * _e5680));
                                                }
                                                let _e5685 = phi_31800_;
                                                let _e5686 = (_e5649 * _e5649);
                                                if (_e5686 >= 1f) {
                                                    phi_31828_ = 0f;
                                                } else {
                                                    phi_31828_ = ((sqrt((1f + (((_e1985 * _e1985) * ((_e5639 * _e5639) + (_e5644 * _e5644))) / _e5686))) - 1f) * 0.5f);
                                                }
                                                let _e5699 = phi_31828_;
                                                let _e5700 = (1f + _e5699);
                                                let _e5701 = (_e5628 * _e5628);
                                                if (_e5701 >= 1f) {
                                                    phi_31856_ = 0f;
                                                } else {
                                                    phi_31856_ = ((sqrt((1f + (((_e1985 * _e1985) * ((_e5654 * _e5654) + (_e5659 * _e5659))) / _e5701))) - 1f) * 0.5f);
                                                }
                                                let _e5714 = phi_31856_;
                                                let _e5721 = (((_e5639 * (_e5662 * _e5671)) + (_e5644 * (_e5663 * _e5671))) + (_e5649 * _e5674));
                                                if (_e1983 == 0u) {
                                                    let _e6016 = (1f - _e5721);
                                                    if (_e6016 != _e6016) {
                                                        phi_32359_ = true;
                                                    } else {
                                                        phi_32359_ = (0f >= _e6016);
                                                    }
                                                    let _e6020 = phi_32359_;
                                                    let _e6022 = pow(select(_e6016, 0f, _e6020), 5f);
                                                    let _e6023 = (1f - _e6022);
                                                    phi_19831_ = vec3<f32>(((_e1981.x * _e6023) + _e6022), ((_e1981.y * _e6023) + _e6022), ((_e1981.z * _e6023) + _e6022));
                                                } else {
                                                    if (_e1983 == 1u) {
                                                        let _e5989 = (1f / _e1977);
                                                        let _e5993 = ((_e5989 * _e5989) * (1f - (_e5721 * _e5721)));
                                                        if (_e5993 >= 1f) {
                                                            phi_32320_ = 1f;
                                                        } else {
                                                            let _e5996 = sqrt((1f - _e5993));
                                                            if (_e5721 != _e5721) {
                                                                phi_32331_ = true;
                                                            } else {
                                                                phi_32331_ = (_e5996 <= _e5721);
                                                            }
                                                            let _e6000 = phi_32331_;
                                                            let _e6005 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                            phi_32320_ = (_e6005 + ((1f - _e6005) * pow((1f - select(_e5721, _e5996, _e6000)), 5f)));
                                                        }
                                                        let _e6012 = phi_32320_;
                                                        phi_19830_ = vec3(_e6012);
                                                    } else {
                                                        let _e5725 = select(_e5721, 0f, (_e5721 < 0f));
                                                        let _e5727 = select(_e5725, 1f, (_e5725 > 1f));
                                                        let _e5729 = (1f - (_e5727 * _e5727));
                                                        let _e5742 = ((_e1981.x * _e1981.x) - (_e1979.x * _e1979.x));
                                                        let _e5743 = ((_e1981.y * _e1981.y) - (_e1979.y * _e1979.y));
                                                        let _e5744 = ((_e1981.z * _e1981.z) - (_e1979.z * _e1979.z));
                                                        let _e5746 = (_e1981.x * (_e1979.x + _e1979.x));
                                                        let _e5748 = (_e1981.y * (_e1979.y + _e1979.y));
                                                        let _e5750 = (_e1981.z * (_e1979.z + _e1979.z));
                                                        let _e5757 = ((_e5742 * _e5742) + (_e5746 * _e5746));
                                                        let _e5758 = ((_e5743 * _e5743) + (_e5748 * _e5748));
                                                        let _e5759 = ((_e5744 * _e5744) + (_e5750 * _e5750));
                                                        let _e5764 = ((_e5729 * _e5742) / _e5757);
                                                        let _e5765 = ((_e5729 * _e5743) / _e5758);
                                                        let _e5766 = ((_e5729 * _e5744) / _e5759);
                                                        let _e5767 = (1f - _e5764);
                                                        let _e5768 = (1f - _e5765);
                                                        let _e5769 = (1f - _e5766);
                                                        let _e5771 = (vec3<f32>(1f, 1f, 1f) - vec3<f32>(_e5764, _e5765, _e5766));
                                                        let _e5772 = (_e5729 * _e5746);
                                                        let _e5773 = (_e5729 * _e5748);
                                                        let _e5774 = (_e5729 * _e5750);
                                                        let _e5779 = (vec3<f32>(_e5772, _e5773, _e5774) / vec3<f32>(_e5757, _e5758, _e5759));
                                                        let _e5792 = ((_e5771.x * _e5771.x) + (_e5779.x * _e5779.x));
                                                        let _e5793 = ((_e5771.y * _e5771.y) + (_e5779.y * _e5779.y));
                                                        let _e5794 = ((_e5771.z * _e5771.z) + (_e5779.z * _e5779.z));
                                                        if (_e5792 != _e5792) {
                                                            phi_31990_ = true;
                                                        } else {
                                                            phi_31990_ = (0f >= _e5792);
                                                        }
                                                        let _e5798 = phi_31990_;
                                                        let _e5800 = sqrt(select(_e5792, 0f, _e5798));
                                                        if (_e5793 != _e5793) {
                                                            phi_32005_ = true;
                                                        } else {
                                                            phi_32005_ = (0f >= _e5793);
                                                        }
                                                        let _e5804 = phi_32005_;
                                                        let _e5806 = sqrt(select(_e5793, 0f, _e5804));
                                                        if (_e5794 != _e5794) {
                                                            phi_32020_ = true;
                                                        } else {
                                                            phi_32020_ = (0f >= _e5794);
                                                        }
                                                        let _e5810 = phi_32020_;
                                                        let _e5812 = sqrt(select(_e5794, 0f, _e5810));
                                                        let _e5816 = ((_e5800 + _e5767) * 0.5f);
                                                        let _e5817 = ((_e5806 + _e5768) * 0.5f);
                                                        let _e5818 = ((_e5812 + _e5769) * 0.5f);
                                                        if (_e5816 != _e5816) {
                                                            phi_32047_ = true;
                                                        } else {
                                                            phi_32047_ = (0f >= _e5816);
                                                        }
                                                        let _e5822 = phi_32047_;
                                                        let _e5824 = sqrt(select(_e5816, 0f, _e5822));
                                                        if (_e5817 != _e5817) {
                                                            phi_32062_ = true;
                                                        } else {
                                                            phi_32062_ = (0f >= _e5817);
                                                        }
                                                        let _e5828 = phi_32062_;
                                                        let _e5830 = sqrt(select(_e5817, 0f, _e5828));
                                                        if (_e5818 != _e5818) {
                                                            phi_32077_ = true;
                                                        } else {
                                                            phi_32077_ = (0f >= _e5818);
                                                        }
                                                        let _e5834 = phi_32077_;
                                                        let _e5836 = sqrt(select(_e5818, 0f, _e5834));
                                                        let _e5837 = vec3<f32>(_e5824, _e5830, _e5836);
                                                        let _e5841 = ((_e5800 - _e5767) * 0.5f);
                                                        let _e5842 = ((_e5806 - _e5768) * 0.5f);
                                                        let _e5843 = ((_e5812 - _e5769) * 0.5f);
                                                        if (_e5841 != _e5841) {
                                                            phi_32104_ = true;
                                                        } else {
                                                            phi_32104_ = (0f >= _e5841);
                                                        }
                                                        let _e5847 = phi_32104_;
                                                        let _e5849 = sqrt(select(_e5841, 0f, _e5847));
                                                        if (_e5842 != _e5842) {
                                                            phi_32119_ = true;
                                                        } else {
                                                            phi_32119_ = (0f >= _e5842);
                                                        }
                                                        let _e5853 = phi_32119_;
                                                        let _e5855 = sqrt(select(_e5842, 0f, _e5853));
                                                        if (_e5843 != _e5843) {
                                                            phi_32134_ = true;
                                                        } else {
                                                            phi_32134_ = (0f >= _e5843);
                                                        }
                                                        let _e5859 = phi_32134_;
                                                        let _e5861 = sqrt(select(_e5843, 0f, _e5859));
                                                        if ((_e5772 / _e5757) < 0f) {
                                                            phi_19707_ = -(_e5849);
                                                        } else {
                                                            phi_19707_ = _e5849;
                                                        }
                                                        let _e5865 = phi_19707_;
                                                        if ((_e5773 / _e5758) < 0f) {
                                                            phi_19715_ = -(_e5855);
                                                        } else {
                                                            phi_19715_ = _e5855;
                                                        }
                                                        let _e5869 = phi_19715_;
                                                        if ((_e5774 / _e5759) < 0f) {
                                                            phi_19723_ = -(_e5861);
                                                        } else {
                                                            phi_19723_ = _e5861;
                                                        }
                                                        let _e5873 = phi_19723_;
                                                        let _e5877 = (_e1979.x * _e5727);
                                                        let _e5878 = (_e1979.y * _e5727);
                                                        let _e5879 = (_e1979.z * _e5727);
                                                        let _e5889 = vec3<f32>(((_e1981.x * _e5824) - (_e1979.x * _e5865)), ((_e1981.y * _e5830) - (_e1979.y * _e5869)), ((_e1981.z * _e5836) - (_e1979.z * _e5873)));
                                                        let _e5896 = ((_e1981.x * _e5865) + (_e1979.x * _e5824));
                                                        let _e5897 = ((_e1981.y * _e5869) + (_e1979.y * _e5830));
                                                        let _e5898 = ((_e1981.z * _e5873) + (_e1979.z * _e5836));
                                                        let _e5900 = vec3<f32>((_e1981.x * _e5727), (_e1981.y * _e5727), (_e1981.z * _e5727));
                                                        let _e5901 = (_e5900 - _e5837);
                                                        let _e5902 = (_e5877 - _e5865);
                                                        let _e5903 = (_e5878 - _e5869);
                                                        let _e5904 = (_e5879 - _e5873);
                                                        let _e5917 = (_e5900 + _e5837);
                                                        let _e5918 = (_e5877 + _e5865);
                                                        let _e5919 = (_e5878 + _e5869);
                                                        let _e5920 = (_e5879 + _e5873);
                                                        let _e5930 = ((_e5917.x * _e5917.x) + (_e5918 * _e5918));
                                                        let _e5931 = ((_e5917.y * _e5917.y) + (_e5919 * _e5919));
                                                        let _e5932 = ((_e5917.z * _e5917.z) + (_e5920 * _e5920));
                                                        let _e5942 = vec3(_e5727);
                                                        let _e5943 = (_e5942 - _e5889);
                                                        let _e5944 = -(vec3<f32>(_e5896, _e5897, _e5898));
                                                        let _e5960 = (_e5942 + _e5889);
                                                        let _e5970 = ((_e5960.x * _e5960.x) + (_e5896 * _e5896));
                                                        let _e5971 = ((_e5960.y * _e5960.y) + (_e5897 * _e5897));
                                                        let _e5972 = ((_e5960.z * _e5960.z) + (_e5898 * _e5898));
                                                        phi_19830_ = vec3<f32>((((((_e5901.x * _e5901.x) + (_e5902 * _e5902)) / select(0.000000000000000000000000000001f, _e5930, (_e5930 > 0.000000000000000000000000000001f))) + (((_e5943.x * _e5943.x) + (_e5944.x * _e5944.x)) / select(0.000000000000000000000000000001f, _e5970, (_e5970 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5901.y * _e5901.y) + (_e5903 * _e5903)) / select(0.000000000000000000000000000001f, _e5931, (_e5931 > 0.000000000000000000000000000001f))) + (((_e5943.y * _e5943.y) + (_e5944.y * _e5944.y)) / select(0.000000000000000000000000000001f, _e5971, (_e5971 > 0.000000000000000000000000000001f)))) * 0.5f), (((((_e5901.z * _e5901.z) + (_e5904 * _e5904)) / select(0.000000000000000000000000000001f, _e5932, (_e5932 > 0.000000000000000000000000000001f))) + (((_e5943.z * _e5943.z) + (_e5944.z * _e5944.z)) / select(0.000000000000000000000000000001f, _e5972, (_e5972 > 0.000000000000000000000000000001f)))) * 0.5f));
                                                    }
                                                    let _e6015 = phi_19830_;
                                                    phi_19831_ = _e6015;
                                                }
                                                let _e6035 = phi_19831_;
                                                let _e6037 = (4f * _e5649);
                                                phi_19843_ = type_59((_e6035 * (((1f / (_e5700 + _e5714)) * _e5685) / _e6037)), ((_e5685 / _e5700) / _e6037));
                                            }
                                            let _e6044 = phi_19843_;
                                            phi_19844_ = _e6044;
                                            phi_19845_ = _e5661;
                                        }
                                        let _e6046 = phi_19844_;
                                        let _e6048 = phi_19845_;
                                        if _e6048 {
                                            phi_19850_ = type_59(vec3<f32>(0f, 0f, 0f), 0f);
                                        } else {
                                            phi_19850_ = _e6046;
                                        }
                                        let _e6050 = phi_19850_;
                                        let _e6053 = (_e1987 == 1u);
                                        if _e6053 {
                                            phi_19878_ = _e6050;
                                            phi_19879_ = type_59();
                                        } else {
                                            if (_e5649 != _e5649) {
                                                phi_32396_ = true;
                                            } else {
                                                phi_32396_ = (0f >= _e5649);
                                            }
                                            let _e6057 = phi_32396_;
                                            let _e6058 = select(_e5649, 0f, _e6057);
                                            let _e6059 = (1f / _e1977);
                                            let _e6063 = ((_e6059 * _e6059) * (1f - (_e6058 * _e6058)));
                                            if (_e6063 >= 1f) {
                                                phi_32415_ = 1f;
                                            } else {
                                                let _e6066 = sqrt((1f - _e6063));
                                                if (_e6058 != _e6058) {
                                                    phi_32426_ = true;
                                                } else {
                                                    phi_32426_ = (_e6066 <= _e6058);
                                                }
                                                let _e6070 = phi_32426_;
                                                let _e6075 = pow(((1f - _e1977) / (1f + _e1977)), 2f);
                                                phi_32415_ = (_e6075 + ((1f - _e6075) * pow((1f - select(_e6058, _e6066, _e6070)), 5f)));
                                            }
                                            let _e6082 = phi_32415_;
                                            if (_e5628 <= 0f) {
                                                phi_32465_ = vec3<f32>(0f, 0f, 0f);
                                            } else {
                                                phi_32465_ = (_e1975 * (_e5628 * 0.31830987f));
                                            }
                                            let _e6087 = phi_32465_;
                                            let _e6088 = (1f - _e6082);
                                            phi_19878_ = type_59();
                                            phi_19879_ = type_59((_e6050.member + vec3<f32>((_e6087.x * _e6088), (_e6087.y * _e6088), (_e6087.z * _e6088))), ((_e6082 * _e6050.member_1) + (_e6088 * _e5634)));
                                        }
                                        let _e6102 = phi_19878_;
                                        let _e6104 = phi_19879_;
                                        phi_19880_ = _e6102;
                                        phi_19881_ = _e6104;
                                        phi_19882_ = _e6053;
                                    }
                                    let _e6112 = phi_19880_;
                                    let _e6114 = phi_19881_;
                                    let _e6116 = phi_19882_;
                                    if _e6116 {
                                        phi_19886_ = _e6112;
                                    } else {
                                        phi_19886_ = _e6114;
                                    }
                                    let _e6118 = phi_19886_;
                                    if ((((_e5620.x * _e5620.x) + (_e5620.y * _e5620.y)) + (_e5620.z * _e5620.z)) == 0f) {
                                        phi_38800_ = _e5383;
                                        phi_20025_ = type_74();
                                        phi_20026_ = type_74();
                                        phi_20027_ = true;
                                        phi_20028_ = false;
                                    } else {
                                        let _e6127 = (_e6118.member_1 <= 0f);
                                        if _e6127 {
                                            phi_38801_ = _e5383;
                                            phi_20022_ = type_74();
                                            phi_20023_ = type_74();
                                            phi_20024_ = false;
                                        } else {
                                            let _e6129 = (_e6118.member.x / _e6118.member_1);
                                            let _e6131 = (_e6118.member.y / _e6118.member_1);
                                            let _e6133 = (_e6118.member.z / _e6118.member_1);
                                            let _e6142 = global_1.member.member_4[1u];
                                            if (sqrt((((_e6129 * _e6129) + (_e6131 * _e6131)) + (_e6133 * _e6133))) <= _e6142) {
                                                phi_38794_ = _e5383;
                                                phi_19973_ = _e836;
                                                phi_19974_ = false;
                                            } else {
                                                let _e6146 = global_1.member.member_7[1u];
                                                if (_e6146 == 1u) {
                                                    let _e6151 = global_1.member.member_4[3u];
                                                    if (f32(_e370) >= _e6151) {
                                                        let _e6154 = select(_e6131, _e6129, (_e6129 > _e6131));
                                                        let _e6156 = select(_e6133, _e6154, (_e6154 > _e6133));
                                                        if (_e6156 != _e6156) {
                                                            phi_32512_ = true;
                                                        } else {
                                                            phi_32512_ = (1f <= _e6156);
                                                        }
                                                        let _e6160 = phi_32512_;
                                                        let _e6161 = select(_e6156, 1f, _e6160);
                                                        let _e6163 = ((_e5383 * 747796405u) + 2891336453u);
                                                        let _e6171 = (((_e6163 >> bitcast<u32>((((_e6163 >> bitcast<u32>(28i)) + 4u) & 31u))) ^ _e6163) * 277803737u);
                                                        let _e6174 = ((_e6171 >> bitcast<u32>(22i)) ^ _e6171);
                                                        let _e6179 = ((f32((_e6174 >> bitcast<u32>(8i))) * 0.000000059604645f) >= _e6161);
                                                        if _e6179 {
                                                            phi_19962_ = vec3<f32>();
                                                        } else {
                                                            phi_19962_ = vec3<f32>((_e836.x / _e6161), (_e836.y / _e6161), (_e836.z / _e6161));
                                                        }
                                                        let _e6188 = phi_19962_;
                                                        phi_38797_ = _e6174;
                                                        phi_19964_ = _e6188;
                                                        phi_19965_ = select(true, false, _e6179);
                                                    } else {
                                                        phi_38797_ = _e5383;
                                                        phi_19964_ = _e836;
                                                        phi_19965_ = true;
                                                    }
                                                    let _e6191 = phi_38797_;
                                                    let _e6193 = phi_19964_;
                                                    let _e6195 = phi_19965_;
                                                    phi_38796_ = _e6191;
                                                    phi_19966_ = _e6193;
                                                    phi_19967_ = _e6195;
                                                } else {
                                                    phi_38796_ = _e5383;
                                                    phi_19966_ = _e836;
                                                    phi_19967_ = true;
                                                }
                                                let _e6197 = phi_38796_;
                                                let _e6199 = phi_19966_;
                                                let _e6201 = phi_19967_;
                                                phi_38794_ = _e6197;
                                                phi_19973_ = select(_e836, _e6199, vec3(_e6201));
                                                phi_19974_ = select(false, true, _e6201);
                                            }
                                            let _e6206 = phi_38794_;
                                            let _e6208 = phi_19973_;
                                            let _e6210 = phi_19974_;
                                            if _e6210 {
                                                if ((((_e5620.x * _e1082.x) + (_e5620.y * _e1082.y)) + (_e5620.z * _e1082.z)) < 0f) {
                                                    phi_20003_ = -(_e1082);
                                                } else {
                                                    phi_20003_ = _e1082;
                                                }
                                                let _e6249 = phi_20003_;
                                                let _e6266 = local_88;
                                                let _e6269 = local_89;
                                                let _e6272 = local_90;
                                                let _e6275 = local_91;
                                                let _e6281 = local_92;
                                                phi_20019_ = type_74(type_34(vec4<f32>((_e842 + (_e6249.x * _e1112)), (_e843 + (_e6249.y * _e1112)), (_e844 + (_e6249.z * _e1112)), _e345.member.w), vec4<f32>(_e5620.x, _e5620.y, _e5620.z, _e6118.member_1), vec4<f32>((_e6208.x * _e6129), (_e6208.y * _e6131), (_e6208.z * _e6133), f32((_e370 + 1u))), vec4<f32>(_e6269.x, _e6272.y, _e6275.z, f32(_e6266)), vec4<u32>(_e6206, _e1995, 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, _e6281);
                                                phi_20020_ = type_74();
                                            } else {
                                                let _e6218 = local_83;
                                                let _e6221 = local_84;
                                                let _e6224 = local_85;
                                                let _e6227 = local_86;
                                                let _e6233 = local_87;
                                                phi_20019_ = type_74();
                                                phi_20020_ = type_74(type_34(_e345.member, _e345.member_1, vec4<f32>(_e6208.x, _e6208.y, _e6208.z, f32(_e370)), vec4<f32>(_e6221.x, _e6224.y, _e6227.z, f32(_e6218)), vec4<u32>(_e6206, (_e1995 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, _e6233);
                                            }
                                            let _e6284 = phi_20019_;
                                            let _e6286 = phi_20020_;
                                            phi_38801_ = _e6206;
                                            phi_20022_ = _e6284;
                                            phi_20023_ = _e6286;
                                            phi_20024_ = select(true, false, _e6210);
                                        }
                                        let _e6289 = phi_38801_;
                                        let _e6291 = phi_20022_;
                                        let _e6293 = phi_20023_;
                                        let _e6295 = phi_20024_;
                                        phi_38800_ = _e6289;
                                        phi_20025_ = _e6291;
                                        phi_20026_ = _e6293;
                                        phi_20027_ = _e6127;
                                        phi_20028_ = _e6295;
                                    }
                                    let _e6297 = phi_38800_;
                                    let _e6299 = phi_20025_;
                                    let _e6301 = phi_20026_;
                                    let _e6303 = phi_20027_;
                                    let _e6305 = phi_20028_;
                                    if _e6303 {
                                        let _e6313 = local_93;
                                        let _e6316 = local_94;
                                        let _e6319 = local_95;
                                        let _e6322 = local_96;
                                        let _e6328 = local_97;
                                        phi_20034_ = type_74(type_34(_e345.member, _e345.member_1, vec4<f32>(_e836.x, _e836.y, _e836.z, f32(_e370)), vec4<f32>(_e6316.x, _e6319.y, _e6322.z, f32(_e6313)), vec4<u32>(_e6297, (_e1995 & 4294967294u), 0u, 0u), vec4<f32>(0f, 0f, 0f, 0f)), _e838, _e6328);
                                    } else {
                                        phi_20034_ = _e6301;
                                    }
                                    let _e6331 = phi_20034_;
                                    phi_20039_ = _e6331;
                                    phi_20040_ = _e6299;
                                    phi_20041_ = select(_e6305, true, _e6303);
                                }
                                let _e6358 = phi_20039_;
                                let _e6360 = phi_20040_;
                                let _e6362 = phi_20041_;
                                phi_20045_ = _e6358;
                                phi_20046_ = _e6360;
                                phi_20047_ = _e6362;
                            } else {
                                phi_20045_ = _e1991;
                                phi_20046_ = type_74();
                                phi_20047_ = true;
                            }
                            let _e6364 = phi_20045_;
                            let _e6366 = phi_20046_;
                            let _e6368 = phi_20047_;
                            phi_20051_ = _e6364;
                            phi_20052_ = _e6366;
                            phi_20053_ = _e6368;
                        }
                        let _e6401 = phi_20051_;
                        let _e6403 = phi_20052_;
                        let _e6405 = phi_20053_;
                        phi_20054_ = _e6401;
                        phi_20055_ = _e6403;
                        phi_20056_ = _e6405;
                    }
                    let _e6600 = phi_20054_;
                    let _e6602 = phi_20055_;
                    let _e6604 = phi_20056_;
                    phi_20060_ = _e6600;
                    phi_20061_ = _e6602;
                    phi_20062_ = _e6604;
                }
                let _e7116 = phi_20060_;
                let _e7118 = phi_20061_;
                let _e7120 = phi_20062_;
                if _e7120 {
                    phi_20066_ = _e7116;
                } else {
                    phi_20066_ = _e7118;
                }
                let _e7122 = phi_20066_;
                if _e342 {
                } else {
                    break;
                }
                global_7.member[_e337.member_1] = _e7122.member;
                let _e7126 = (2u * _e337.member_1);
                if (_e7126 < _e297) {
                } else {
                    break;
                }
                global_9.member[_e7126] = _e7122.member_1;
                let _e7130 = (_e7126 + 1u);
                if (_e7130 < _e297) {
                } else {
                    break;
                }
                global_9.member[_e7130] = _e7122.member_2;
                if ((_e7122.member.member_4.y & 1u) == 0u) {
                } else {
                    let _e7138 = (1u - _e301);
                    let _e7139 = (_e7138 * 4u);
                    let _e7143 = atomicAdd((&global_8.member[(_e7139 + 3u)]), 1u);
                    let _e7147 = global_1.member.member[0u];
                    let _e7151 = global_1.member.member[1u];
                    let _e7156 = global_1.member.member_7[3u];
                    atomicStore((&global_8.member[((12u + (_e7138 * ((_e7147 * _e7151) * _e7156))) + _e7143)]), _e337.member_1);
                    if ((_e7143 % 64u) == 0u) {
                        let _e7165 = (_e7143 / 64u);
                        if (_e7165 < 65535u) {
                            let _e7169 = atomicAdd((&global_8.member[_e7139]), 1u);
                        }
                        if ((_e7165 % 65535u) == 0u) {
                            let _e7175 = atomicAdd((&global_8.member[(_e7139 + 1u)]), 1u);
                        }
                    }
                }
                if ((_e7122.member.member_4.y & 16u) == 0u) {
                } else {
                    let _e7180 = atomicAdd((&global_8.member[11u]), 1u);
                    let _e7184 = global_1.member.member[0u];
                    let _e7188 = global_1.member.member[1u];
                    let _e7193 = global_1.member.member_7[3u];
                    atomicStore((&global_8.member[((12u + (2u * ((_e7184 * _e7188) * _e7193))) + _e7180)]), _e337.member_1);
                    if ((_e7180 % 64u) == 0u) {
                        let _e7202 = (_e7180 / 64u);
                        if (_e7202 < 65535u) {
                            let _e7206 = atomicAdd((&global_8.member[8u]), 1u);
                        }
                        if ((_e7202 % 65535u) == 0u) {
                            let _e7211 = atomicAdd((&global_8.member[9u]), 1u);
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
    switch bitcast<i32>(0u) {
        default: {
            let _e285 = global;
            let _e293 = (_e285.x + (_e285.y * 4194240u));
            let _e296 = atomicLoad((&global_11.member[11u]));
            if (_e293 >= _e296) {
            } else {
                if (_e293 < arrayLength((&global_12.member))) {
                } else {
                    break;
                }
                let _e301 = global_12.member[_e293];
                if (_e301.member_4.w < arrayLength((&global_10.member))) {
                } else {
                    break;
                }
                global_10.member[_e301.member_4.w] = type_34(_e301.member, _e301.member_1, _e301.member_2, _e301.member_3, vec4<u32>(_e301.member_4.x, _e301.member_4.y, _e301.member_4.z, 0u), _e301.member_5);
                if ((_e301.member_4.y & 1u) == 0u) {
                } else {
                    let _e326 = global_1.member.member_7[2u];
                    let _e327 = (1u - _e326);
                    let _e328 = (_e327 * 4u);
                    let _e332 = atomicAdd((&global_11.member[(_e328 + 3u)]), 1u);
                    let _e336 = global_1.member.member[0u];
                    let _e340 = global_1.member.member[1u];
                    let _e345 = global_1.member.member_7[3u];
                    atomicStore((&global_11.member[((12u + (_e327 * ((_e336 * _e340) * _e345))) + _e332)]), _e301.member_4.w);
                    if ((_e332 % 64u) == 0u) {
                        let _e354 = (_e332 / 64u);
                        if (_e354 < 65535u) {
                            let _e358 = atomicAdd((&global_11.member[_e328]), 1u);
                        }
                        if ((_e354 % 65535u) == 0u) {
                            let _e364 = atomicAdd((&global_11.member[(_e328 + 1u)]), 1u);
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

fn function_5() {
    var local_103: array<u32, 64>;
    var local_104: array<f32, 64>;
    var phi_21243_: type_48;
    var phi_21296_: u32;
    var phi_21299_: u32;
    var phi_21301_: u32;
    var phi_21303_: u32;
    var phi_21305_: u32;
    var phi_21307_: vec4<f32>;
    var phi_21309_: f32;
    var phi_21679_: u32;
    var phi_21681_: u32;
    var phi_21682_: u32;
    var phi_21684_: bool;
    var phi_21685_: bool;
    var phi_21686_: u32;
    var phi_21687_: u32;
    var phi_21688_: u32;
    var phi_21689_: bool;
    var phi_21690_: bool;
    var phi_21691_: bool;
    var phi_33214_: bool;
    var phi_33244_: bool;
    var phi_33191_: type_56;
    var phi_33194_: type_56;
    var phi_33195_: bool;
    var phi_33200_: type_56;
    var phi_33301_: bool;
    var phi_33316_: bool;
    var phi_33331_: bool;
    var phi_33346_: bool;
    var phi_33278_: type_56;
    var phi_33281_: type_56;
    var phi_33282_: bool;
    var phi_33287_: type_56;
    var phi_33388_: bool;
    var phi_33403_: bool;
    var phi_33418_: bool;
    var phi_33433_: bool;
    var phi_33365_: type_56;
    var phi_33368_: type_56;
    var phi_33369_: bool;
    var phi_33374_: type_56;
    var phi_33169_: bool;
    var phi_33172_: bool;
    var phi_21720_: f32;
    var phi_33535_: bool;
    var phi_33565_: bool;
    var phi_33512_: type_56;
    var phi_33515_: type_56;
    var phi_33516_: bool;
    var phi_33521_: type_56;
    var phi_33622_: bool;
    var phi_33637_: bool;
    var phi_33652_: bool;
    var phi_33667_: bool;
    var phi_33599_: type_56;
    var phi_33602_: type_56;
    var phi_33603_: bool;
    var phi_33608_: type_56;
    var phi_33709_: bool;
    var phi_33724_: bool;
    var phi_33739_: bool;
    var phi_33754_: bool;
    var phi_33686_: type_56;
    var phi_33689_: type_56;
    var phi_33690_: bool;
    var phi_33695_: type_56;
    var phi_33490_: bool;
    var phi_33493_: bool;
    var phi_21729_: f32;
    var phi_21740_: u32;
    var phi_21751_: u32;
    var phi_21790_: u32;
    var phi_21791_: u32;
    var phi_21802_: u32;
    var phi_21803_: u32;
    var phi_21804_: u32;
    var phi_21805_: u32;
    var phi_21807_: u32;
    var phi_21808_: u32;
    var phi_21810_: u32;
    var phi_21811_: u32;
    var phi_21812_: bool;
    var phi_21813_: bool;
    var phi_21605_: vec4<f32>;
    var phi_21606_: vec4<f32>;
    var phi_21607_: bool;
    var phi_21611_: vec4<f32>;
    var phi_21612_: bool;
    var phi_21522_: bool;
    var phi_21556_: bool;
    var phi_21562_: f32;
    var phi_21572_: vec4<f32>;
    var phi_21573_: vec4<f32>;
    var phi_21574_: bool;
    var phi_21578_: vec4<f32>;
    var phi_21579_: bool;
    var phi_21583_: vec4<f32>;
    var phi_21584_: bool;
    var phi_21613_: vec4<f32>;
    var phi_21614_: bool;
    var phi_21455_: vec4<f32>;
    var phi_21456_: vec4<f32>;
    var phi_21457_: bool;
    var phi_21461_: vec4<f32>;
    var phi_21462_: bool;
    var phi_21463_: vec4<f32>;
    var phi_21464_: bool;
    var phi_21465_: bool;
    var phi_21473_: vec4<f32>;
    var phi_21474_: bool;
    var phi_21475_: vec4<f32>;
    var phi_21476_: bool;
    var phi_21477_: bool;
    var phi_21485_: vec4<f32>;
    var phi_21486_: bool;
    var phi_21623_: vec4<f32>;
    var phi_21624_: bool;
    var phi_21645_: f32;
    var phi_21297_: u32;
    var phi_21300_: u32;
    var phi_21302_: u32;
    var phi_21304_: u32;
    var phi_21306_: u32;
    var phi_21308_: vec4<f32>;
    var phi_21310_: f32;
    var phi_21824_: vec4<f32>;
    var phi_21825_: bool;
    var phi_21826_: bool;
    var phi_38856_: bool;
    var phi_22063_: bool;
    var phi_22062_: vec4<f32>;
    var local_105: u32;
    var local_106: vec4<f32>;
    var phi_21831_: type_79;
    var local_107: u32;
    var phi_21837_: type_79;

    switch bitcast<i32>(0u) {
        default: {
            let _e287 = global;
            let _e296 = global_1.member.member_7[2u];
            let _e300 = (_e287.x + (_e287.y * 4194240u));
            let _e305 = atomicLoad((&global_8.member[((_e296 * 4u) + 3u)]));
            if (_e300 >= _e305) {
                phi_21243_ = type_48(0u, type_48().member_1);
            } else {
                let _e310 = global_1.member.member[0u];
                let _e314 = global_1.member.member[1u];
                let _e319 = global_1.member.member_7[3u];
                let _e326 = atomicLoad((&global_8.member[((12u + (_e296 * ((_e310 * _e314) * _e319))) + _e300)]));
                phi_21243_ = type_48(1u, _e326);
            }
            let _e332 = phi_21243_;
            if (bitcast<i32>(_e332.member) != 0i) {
                let _e337 = (_e332.member_1 < arrayLength((&global_7.member)));
                if _e337 {
                } else {
                    break;
                }
                let _e343 = global_7.member[_e332.member_1].member;
                let _e350 = global_7.member[_e332.member_1].member_1;
                let _e356 = global_1.member.member_1[0u];
                let _e359 = global_1.member.member_1[1u];
                let _e362 = (vec3<f32>(1f, 1f, 1f) / vec3<f32>(_e350.x, _e350.y, _e350.z));
                local_103 = array<u32, 64>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                local_104 = array<f32, 64>(0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f, 0f);
                phi_21296_ = 4294967295u;
                phi_21299_ = select(4294967295u, 0u, (_e356 > 0u));
                phi_21301_ = 0u;
                phi_21303_ = (_e356 + _e359);
                phi_21305_ = _e356;
                phi_21307_ = vec4<f32>(0f, 0f, 0f, 0f);
                phi_21309_ = 300000000000000000000000000000000000000f;
                loop {
                    let _e366 = phi_21296_;
                    let _e368 = phi_21299_;
                    let _e370 = phi_21301_;
                    let _e372 = phi_21303_;
                    let _e374 = phi_21305_;
                    let _e376 = phi_21307_;
                    let _e378 = phi_21309_;
                    local_105 = _e366;
                    local_106 = _e376;
                    local_107 = _e374;
                    if (_e374 < _e372) {
                        let _e741 = (3u * _e374);
                        let _e743 = global_2.member[_e741];
                        let _e746 = global_2.member[(_e741 + 1u)];
                        let _e752 = select(select(u32(_e746.w), 0u, (_e746.w < 0f)), 4294967295u, (_e746.w > 4294967000f));
                        if (_e752 == 0u) {
                            let _e846 = global_2.member[(_e741 + 2u)];
                            let _e855 = ((_e350.y * _e846.z) - (_e846.y * _e350.z));
                            let _e858 = ((_e350.z * _e846.x) - (_e846.z * _e350.x));
                            let _e861 = ((_e350.x * _e846.y) - (_e846.x * _e350.y));
                            let _e866 = (((_e746.x * _e855) + (_e746.y * _e858)) + (_e746.z * _e861));
                            if (abs(_e866) < 0.000000000001f) {
                                phi_21485_ = vec4<f32>();
                                phi_21486_ = true;
                            } else {
                                let _e869 = (1f / _e866);
                                let _e873 = (_e343.x - _e743.x);
                                let _e874 = (_e343.y - _e743.y);
                                let _e875 = (_e343.z - _e743.z);
                                let _e881 = ((((_e873 * _e855) + (_e874 * _e858)) + (_e875 * _e861)) * _e869);
                                if (_e881 < 0f) {
                                    phi_21475_ = vec4<f32>();
                                    phi_21476_ = true;
                                    phi_21477_ = false;
                                } else {
                                    let _e883 = (_e881 > 1f);
                                    if _e883 {
                                        phi_21473_ = vec4<f32>();
                                        phi_21474_ = false;
                                    } else {
                                        let _e886 = ((_e874 * _e746.z) - (_e746.y * _e875));
                                        let _e889 = ((_e875 * _e746.x) - (_e746.z * _e873));
                                        let _e892 = ((_e873 * _e746.y) - (_e746.x * _e874));
                                        let _e898 = ((((_e350.x * _e886) + (_e350.y * _e889)) + (_e350.z * _e892)) * _e869);
                                        if (_e898 < 0f) {
                                            phi_21463_ = vec4<f32>();
                                            phi_21464_ = true;
                                            phi_21465_ = false;
                                        } else {
                                            let _e901 = ((_e881 + _e898) > 1f);
                                            if _e901 {
                                                phi_21461_ = vec4<f32>();
                                                phi_21462_ = false;
                                            } else {
                                                let _e907 = ((((_e846.x * _e886) + (_e846.y * _e889)) + (_e846.z * _e892)) * _e869);
                                                if (_e907 <= 0f) {
                                                    phi_21456_ = vec4<f32>();
                                                    phi_21457_ = true;
                                                } else {
                                                    let _e909 = (_e907 >= _e378);
                                                    if _e909 {
                                                        phi_21455_ = vec4<f32>();
                                                    } else {
                                                        phi_21455_ = vec4<f32>(_e907, _e881, _e898, 1f);
                                                    }
                                                    let _e912 = phi_21455_;
                                                    phi_21456_ = _e912;
                                                    phi_21457_ = _e909;
                                                }
                                                let _e914 = phi_21456_;
                                                let _e916 = phi_21457_;
                                                phi_21461_ = _e914;
                                                phi_21462_ = _e916;
                                            }
                                            let _e918 = phi_21461_;
                                            let _e920 = phi_21462_;
                                            phi_21463_ = _e918;
                                            phi_21464_ = _e901;
                                            phi_21465_ = _e920;
                                        }
                                        let _e922 = phi_21463_;
                                        let _e924 = phi_21464_;
                                        let _e926 = phi_21465_;
                                        phi_21473_ = _e922;
                                        phi_21474_ = select(_e926, true, _e924);
                                    }
                                    let _e929 = phi_21473_;
                                    let _e931 = phi_21474_;
                                    phi_21475_ = _e929;
                                    phi_21476_ = _e883;
                                    phi_21477_ = _e931;
                                }
                                let _e933 = phi_21475_;
                                let _e935 = phi_21476_;
                                let _e937 = phi_21477_;
                                phi_21485_ = _e933;
                                phi_21486_ = select(_e937, true, _e935);
                            }
                            let _e940 = phi_21485_;
                            let _e942 = phi_21486_;
                            phi_21623_ = _e940;
                            phi_21624_ = _e942;
                        } else {
                            if (_e752 == 1u) {
                                let _e776 = (_e343.x - _e743.x);
                                let _e777 = (_e343.y - _e743.y);
                                let _e778 = (_e343.z - _e743.z);
                                let _e783 = (((_e776 * _e350.x) + (_e777 * _e350.y)) + (_e778 * _e350.z));
                                if (_e783 > 0f) {
                                    phi_21522_ = select(true, false, ((((_e776 * _e776) + (_e777 * _e777)) + (_e778 * _e778)) > (_e743.w * _e743.w)));
                                } else {
                                    phi_21522_ = true;
                                }
                                let _e795 = phi_21522_;
                                if _e795 {
                                    let _e799 = (_e776 - (_e350.x * _e783));
                                    let _e800 = (_e777 - (_e350.y * _e783));
                                    let _e801 = (_e778 - (_e350.z * _e783));
                                    let _e809 = ((_e743.w * _e743.w) - (((_e799 * _e799) + (_e800 * _e800)) + (_e801 * _e801)));
                                    if (_e809 < 0f) {
                                        phi_21578_ = vec4<f32>();
                                        phi_21579_ = true;
                                    } else {
                                        let _e811 = sqrt(_e809);
                                        let _e812 = -(_e783);
                                        let _e813 = (_e812 - _e811);
                                        if (_e813 <= 0f) {
                                            phi_21556_ = true;
                                        } else {
                                            phi_21556_ = (_e813 >= _e378);
                                        }
                                        let _e817 = phi_21556_;
                                        if _e817 {
                                            phi_21562_ = (_e812 + _e811);
                                        } else {
                                            phi_21562_ = _e813;
                                        }
                                        let _e820 = phi_21562_;
                                        if (_e820 <= 0f) {
                                            phi_21573_ = vec4<f32>();
                                            phi_21574_ = true;
                                        } else {
                                            let _e822 = (_e820 >= _e378);
                                            if _e822 {
                                                phi_21572_ = vec4<f32>();
                                            } else {
                                                phi_21572_ = vec4<f32>(_e820, 0f, 0f, 1f);
                                            }
                                            let _e825 = phi_21572_;
                                            phi_21573_ = _e825;
                                            phi_21574_ = _e822;
                                        }
                                        let _e827 = phi_21573_;
                                        let _e829 = phi_21574_;
                                        phi_21578_ = _e827;
                                        phi_21579_ = _e829;
                                    }
                                    let _e831 = phi_21578_;
                                    let _e833 = phi_21579_;
                                    phi_21583_ = _e831;
                                    phi_21584_ = _e833;
                                } else {
                                    phi_21583_ = vec4<f32>();
                                    phi_21584_ = true;
                                }
                                let _e835 = phi_21583_;
                                let _e837 = phi_21584_;
                                phi_21613_ = _e835;
                                phi_21614_ = _e837;
                            } else {
                                if (abs(_e350.y) < 0.000000000001f) {
                                    phi_21611_ = vec4<f32>();
                                    phi_21612_ = true;
                                } else {
                                    let _e759 = ((_e743.x - _e343.y) / _e350.y);
                                    if (_e759 <= 0f) {
                                        phi_21606_ = vec4<f32>();
                                        phi_21607_ = true;
                                    } else {
                                        let _e761 = (_e759 >= _e378);
                                        if _e761 {
                                            phi_21605_ = vec4<f32>();
                                        } else {
                                            phi_21605_ = vec4<f32>(_e759, 0f, 0f, 1f);
                                        }
                                        let _e764 = phi_21605_;
                                        phi_21606_ = _e764;
                                        phi_21607_ = _e761;
                                    }
                                    let _e766 = phi_21606_;
                                    let _e768 = phi_21607_;
                                    phi_21611_ = _e766;
                                    phi_21612_ = _e768;
                                }
                                let _e770 = phi_21611_;
                                let _e772 = phi_21612_;
                                phi_21613_ = _e770;
                                phi_21614_ = _e772;
                            }
                            let _e839 = phi_21613_;
                            let _e841 = phi_21614_;
                            phi_21623_ = _e839;
                            phi_21624_ = select(true, false, select(true, false, _e841));
                        }
                        let _e944 = phi_21623_;
                        let _e946 = phi_21624_;
                        let _e949 = select(vec4<f32>(0f, 0f, 0f, 0f), _e944, vec4(select(true, false, _e946)));
                        let _e951 = (_e949.w > 0f);
                        if _e951 {
                            phi_21645_ = _e949.x;
                        } else {
                            phi_21645_ = _e378;
                        }
                        let _e954 = phi_21645_;
                        phi_21297_ = select(_e366, _e374, _e951);
                        phi_21300_ = _e368;
                        phi_21302_ = _e370;
                        phi_21304_ = _e372;
                        phi_21306_ = (_e374 + 1u);
                        phi_21308_ = select(_e376, _e949, vec4(_e951));
                        phi_21310_ = _e954;
                        phi_21824_ = _e949;
                        phi_21825_ = true;
                        phi_21826_ = false;
                    } else {
                        if (_e368 == 4294967295u) {
                            let _e381 = (_e370 == 0u);
                            if _e381 {
                                phi_21681_ = u32();
                                phi_21682_ = u32();
                                phi_21684_ = false;
                                phi_21685_ = false;
                            } else {
                                let _e382 = (_e370 - 1u);
                                let _e383 = (_e382 < 64u);
                                if _e383 {
                                } else {
                                    phi_38856_ = true;
                                    phi_22063_ = bool();
                                    phi_22062_ = vec4<f32>();
                                    break;
                                }
                                let _e385 = local_104[_e382];
                                let _e386 = (_e385 >= _e378);
                                if _e386 {
                                    phi_21679_ = u32();
                                } else {
                                    if _e383 {
                                    } else {
                                        phi_38856_ = true;
                                        phi_22063_ = bool();
                                        phi_22062_ = vec4<f32>();
                                        break;
                                    }
                                    let _e388 = local_103[_e382];
                                    phi_21679_ = _e388;
                                }
                                let _e390 = phi_21679_;
                                phi_21681_ = _e382;
                                phi_21682_ = _e390;
                                phi_21684_ = select(true, false, _e386);
                                phi_21685_ = _e386;
                            }
                            let _e393 = phi_21681_;
                            let _e395 = phi_21682_;
                            let _e397 = phi_21684_;
                            let _e399 = phi_21685_;
                            phi_21686_ = _e393;
                            phi_21687_ = _e395;
                            phi_21688_ = _e393;
                            phi_21689_ = _e397;
                            phi_21690_ = _e399;
                            phi_21691_ = _e381;
                        } else {
                            phi_21686_ = u32();
                            phi_21687_ = _e368;
                            phi_21688_ = _e370;
                            phi_21689_ = true;
                            phi_21690_ = false;
                            phi_21691_ = false;
                        }
                        let _e401 = phi_21686_;
                        let _e403 = phi_21687_;
                        let _e405 = phi_21688_;
                        let _e407 = phi_21689_;
                        let _e409 = phi_21690_;
                        let _e411 = phi_21691_;
                        if _e407 {
                            let _e413 = ((_e403 & 2147483648u) == 0u);
                            if _e413 {
                                let _e421 = global_3.member[_e403];
                                if (_e421.member_1 == 4294967295u) {
                                    phi_21720_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e433 = ((_e421.member.x - _e343.x) * _e362.x);
                                    let _e435 = ((_e421.member.y - _e343.y) * _e362.y);
                                    let _e437 = ((_e421.member.z - _e343.z) * _e362.z);
                                    let _e444 = ((_e421.member_2.x - _e343.x) * _e362.x);
                                    let _e445 = ((_e421.member_2.y - _e343.y) * _e362.y);
                                    let _e446 = ((_e421.member_2.z - _e343.z) * _e362.z);
                                    if (_e433 == _e433) {
                                        let _e448 = (_e444 == _e444);
                                        if _e448 {
                                            let _e449 = (_e433 != _e433);
                                            if _e449 {
                                                phi_33214_ = true;
                                            } else {
                                                phi_33214_ = (_e444 <= _e433);
                                            }
                                            let _e452 = phi_33214_;
                                            let _e453 = select(_e433, _e444, _e452);
                                            if _e449 {
                                                phi_33244_ = true;
                                            } else {
                                                phi_33244_ = (_e444 >= _e433);
                                            }
                                            let _e458 = phi_33244_;
                                            let _e459 = select(_e433, _e444, _e458);
                                            phi_33191_ = type_56(select(-300000000000000000000000000000000000000f, _e453, (_e453 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e459, (_e459 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_33191_ = type_56();
                                        }
                                        let _e464 = phi_33191_;
                                        phi_33194_ = _e464;
                                        phi_33195_ = select(true, false, _e448);
                                    } else {
                                        phi_33194_ = type_56();
                                        phi_33195_ = true;
                                    }
                                    let _e467 = phi_33194_;
                                    let _e469 = phi_33195_;
                                    if _e469 {
                                        phi_33200_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_33200_ = _e467;
                                    }
                                    let _e471 = phi_33200_;
                                    if (_e435 == _e435) {
                                        let _e475 = (_e445 == _e445);
                                        if _e475 {
                                            let _e476 = (_e435 != _e435);
                                            if _e476 {
                                                phi_33301_ = true;
                                            } else {
                                                phi_33301_ = (_e445 <= _e435);
                                            }
                                            let _e479 = phi_33301_;
                                            let _e480 = select(_e435, _e445, _e479);
                                            if (_e471.member != _e471.member) {
                                                phi_33316_ = true;
                                            } else {
                                                phi_33316_ = (_e480 >= _e471.member);
                                            }
                                            let _e484 = phi_33316_;
                                            if _e476 {
                                                phi_33331_ = true;
                                            } else {
                                                phi_33331_ = (_e445 >= _e435);
                                            }
                                            let _e488 = phi_33331_;
                                            let _e489 = select(_e435, _e445, _e488);
                                            if (_e471.member_1 != _e471.member_1) {
                                                phi_33346_ = true;
                                            } else {
                                                phi_33346_ = (_e489 <= _e471.member_1);
                                            }
                                            let _e493 = phi_33346_;
                                            phi_33278_ = type_56(select(_e471.member, _e480, _e484), select(_e471.member_1, _e489, _e493));
                                        } else {
                                            phi_33278_ = type_56();
                                        }
                                        let _e497 = phi_33278_;
                                        phi_33281_ = _e497;
                                        phi_33282_ = select(true, false, _e475);
                                    } else {
                                        phi_33281_ = type_56();
                                        phi_33282_ = true;
                                    }
                                    let _e500 = phi_33281_;
                                    let _e502 = phi_33282_;
                                    if _e502 {
                                        phi_33287_ = _e471;
                                    } else {
                                        phi_33287_ = _e500;
                                    }
                                    let _e504 = phi_33287_;
                                    if (_e437 == _e437) {
                                        let _e508 = (_e446 == _e446);
                                        if _e508 {
                                            let _e509 = (_e437 != _e437);
                                            if _e509 {
                                                phi_33388_ = true;
                                            } else {
                                                phi_33388_ = (_e446 <= _e437);
                                            }
                                            let _e512 = phi_33388_;
                                            let _e513 = select(_e437, _e446, _e512);
                                            if (_e504.member != _e504.member) {
                                                phi_33403_ = true;
                                            } else {
                                                phi_33403_ = (_e513 >= _e504.member);
                                            }
                                            let _e517 = phi_33403_;
                                            if _e509 {
                                                phi_33418_ = true;
                                            } else {
                                                phi_33418_ = (_e446 >= _e437);
                                            }
                                            let _e521 = phi_33418_;
                                            let _e522 = select(_e437, _e446, _e521);
                                            if (_e504.member_1 != _e504.member_1) {
                                                phi_33433_ = true;
                                            } else {
                                                phi_33433_ = (_e522 <= _e504.member_1);
                                            }
                                            let _e526 = phi_33433_;
                                            phi_33365_ = type_56(select(_e504.member, _e513, _e517), select(_e504.member_1, _e522, _e526));
                                        } else {
                                            phi_33365_ = type_56();
                                        }
                                        let _e530 = phi_33365_;
                                        phi_33368_ = _e530;
                                        phi_33369_ = select(true, false, _e508);
                                    } else {
                                        phi_33368_ = type_56();
                                        phi_33369_ = true;
                                    }
                                    let _e533 = phi_33368_;
                                    let _e535 = phi_33369_;
                                    if _e535 {
                                        phi_33374_ = _e504;
                                    } else {
                                        phi_33374_ = _e533;
                                    }
                                    let _e537 = phi_33374_;
                                    if (_e537.member_1 < 0f) {
                                        phi_33172_ = true;
                                    } else {
                                        if (_e537.member > _e537.member_1) {
                                            phi_33169_ = true;
                                        } else {
                                            phi_33169_ = (_e537.member > _e378);
                                        }
                                        let _e544 = phi_33169_;
                                        phi_33172_ = _e544;
                                    }
                                    let _e546 = phi_33172_;
                                    phi_21720_ = select(_e537.member, 300000000000000000000000000000000000000f, _e546);
                                }
                                let _e549 = phi_21720_;
                                if (_e421.member_5 == 4294967295u) {
                                    phi_21729_ = 300000000000000000000000000000000000000f;
                                } else {
                                    let _e561 = ((_e421.member_4.x - _e343.x) * _e362.x);
                                    let _e563 = ((_e421.member_4.y - _e343.y) * _e362.y);
                                    let _e565 = ((_e421.member_4.z - _e343.z) * _e362.z);
                                    let _e572 = ((_e421.member_6.x - _e343.x) * _e362.x);
                                    let _e573 = ((_e421.member_6.y - _e343.y) * _e362.y);
                                    let _e574 = ((_e421.member_6.z - _e343.z) * _e362.z);
                                    if (_e561 == _e561) {
                                        let _e576 = (_e572 == _e572);
                                        if _e576 {
                                            let _e577 = (_e561 != _e561);
                                            if _e577 {
                                                phi_33535_ = true;
                                            } else {
                                                phi_33535_ = (_e572 <= _e561);
                                            }
                                            let _e580 = phi_33535_;
                                            let _e581 = select(_e561, _e572, _e580);
                                            if _e577 {
                                                phi_33565_ = true;
                                            } else {
                                                phi_33565_ = (_e572 >= _e561);
                                            }
                                            let _e586 = phi_33565_;
                                            let _e587 = select(_e561, _e572, _e586);
                                            phi_33512_ = type_56(select(-300000000000000000000000000000000000000f, _e581, (_e581 >= -300000000000000000000000000000000000000f)), select(300000000000000000000000000000000000000f, _e587, (_e587 <= 300000000000000000000000000000000000000f)));
                                        } else {
                                            phi_33512_ = type_56();
                                        }
                                        let _e592 = phi_33512_;
                                        phi_33515_ = _e592;
                                        phi_33516_ = select(true, false, _e576);
                                    } else {
                                        phi_33515_ = type_56();
                                        phi_33516_ = true;
                                    }
                                    let _e595 = phi_33515_;
                                    let _e597 = phi_33516_;
                                    if _e597 {
                                        phi_33521_ = type_56(-300000000000000000000000000000000000000f, 300000000000000000000000000000000000000f);
                                    } else {
                                        phi_33521_ = _e595;
                                    }
                                    let _e599 = phi_33521_;
                                    if (_e563 == _e563) {
                                        let _e603 = (_e573 == _e573);
                                        if _e603 {
                                            let _e604 = (_e563 != _e563);
                                            if _e604 {
                                                phi_33622_ = true;
                                            } else {
                                                phi_33622_ = (_e573 <= _e563);
                                            }
                                            let _e607 = phi_33622_;
                                            let _e608 = select(_e563, _e573, _e607);
                                            if (_e599.member != _e599.member) {
                                                phi_33637_ = true;
                                            } else {
                                                phi_33637_ = (_e608 >= _e599.member);
                                            }
                                            let _e612 = phi_33637_;
                                            if _e604 {
                                                phi_33652_ = true;
                                            } else {
                                                phi_33652_ = (_e573 >= _e563);
                                            }
                                            let _e616 = phi_33652_;
                                            let _e617 = select(_e563, _e573, _e616);
                                            if (_e599.member_1 != _e599.member_1) {
                                                phi_33667_ = true;
                                            } else {
                                                phi_33667_ = (_e617 <= _e599.member_1);
                                            }
                                            let _e621 = phi_33667_;
                                            phi_33599_ = type_56(select(_e599.member, _e608, _e612), select(_e599.member_1, _e617, _e621));
                                        } else {
                                            phi_33599_ = type_56();
                                        }
                                        let _e625 = phi_33599_;
                                        phi_33602_ = _e625;
                                        phi_33603_ = select(true, false, _e603);
                                    } else {
                                        phi_33602_ = type_56();
                                        phi_33603_ = true;
                                    }
                                    let _e628 = phi_33602_;
                                    let _e630 = phi_33603_;
                                    if _e630 {
                                        phi_33608_ = _e599;
                                    } else {
                                        phi_33608_ = _e628;
                                    }
                                    let _e632 = phi_33608_;
                                    if (_e565 == _e565) {
                                        let _e636 = (_e574 == _e574);
                                        if _e636 {
                                            let _e637 = (_e565 != _e565);
                                            if _e637 {
                                                phi_33709_ = true;
                                            } else {
                                                phi_33709_ = (_e574 <= _e565);
                                            }
                                            let _e640 = phi_33709_;
                                            let _e641 = select(_e565, _e574, _e640);
                                            if (_e632.member != _e632.member) {
                                                phi_33724_ = true;
                                            } else {
                                                phi_33724_ = (_e641 >= _e632.member);
                                            }
                                            let _e645 = phi_33724_;
                                            if _e637 {
                                                phi_33739_ = true;
                                            } else {
                                                phi_33739_ = (_e574 >= _e565);
                                            }
                                            let _e649 = phi_33739_;
                                            let _e650 = select(_e565, _e574, _e649);
                                            if (_e632.member_1 != _e632.member_1) {
                                                phi_33754_ = true;
                                            } else {
                                                phi_33754_ = (_e650 <= _e632.member_1);
                                            }
                                            let _e654 = phi_33754_;
                                            phi_33686_ = type_56(select(_e632.member, _e641, _e645), select(_e632.member_1, _e650, _e654));
                                        } else {
                                            phi_33686_ = type_56();
                                        }
                                        let _e658 = phi_33686_;
                                        phi_33689_ = _e658;
                                        phi_33690_ = select(true, false, _e636);
                                    } else {
                                        phi_33689_ = type_56();
                                        phi_33690_ = true;
                                    }
                                    let _e661 = phi_33689_;
                                    let _e663 = phi_33690_;
                                    if _e663 {
                                        phi_33695_ = _e632;
                                    } else {
                                        phi_33695_ = _e661;
                                    }
                                    let _e665 = phi_33695_;
                                    if (_e665.member_1 < 0f) {
                                        phi_33493_ = true;
                                    } else {
                                        if (_e665.member > _e665.member_1) {
                                            phi_33490_ = true;
                                        } else {
                                            phi_33490_ = (_e665.member > _e378);
                                        }
                                        let _e672 = phi_33490_;
                                        phi_33493_ = _e672;
                                    }
                                    let _e674 = phi_33493_;
                                    phi_21729_ = select(_e665.member, 300000000000000000000000000000000000000f, _e674);
                                }
                                let _e677 = phi_21729_;
                                if (_e421.member_3 > 0u) {
                                    phi_21740_ = ((2147483648u | ((_e421.member_3 - 1u) << bitcast<u32>(28u))) | _e421.member_1);
                                } else {
                                    phi_21740_ = _e421.member_1;
                                }
                                let _e686 = phi_21740_;
                                if (_e421.member_7 > 0u) {
                                    phi_21751_ = ((2147483648u | ((_e421.member_7 - 1u) << bitcast<u32>(28u))) | _e421.member_5);
                                } else {
                                    phi_21751_ = _e421.member_5;
                                }
                                let _e695 = phi_21751_;
                                let _e696 = (_e549 <= _e677);
                                let _e700 = select(_e549, _e677, _e696);
                                if (_e700 < 300000000000000000000000000000000000000f) {
                                    let _e702 = (_e405 < 64u);
                                    if _e702 {
                                        if _e702 {
                                        } else {
                                            phi_38856_ = true;
                                            phi_22063_ = bool();
                                            phi_22062_ = vec4<f32>();
                                            break;
                                        }
                                        local_103[_e405] = select(_e686, _e695, _e696);
                                        if _e702 {
                                        } else {
                                            phi_38856_ = true;
                                            phi_22063_ = bool();
                                            phi_22062_ = vec4<f32>();
                                            break;
                                        }
                                        local_104[_e405] = _e700;
                                        phi_21790_ = (_e405 + 1u);
                                    } else {
                                        phi_21790_ = _e405;
                                    }
                                    let _e707 = phi_21790_;
                                    phi_21791_ = _e707;
                                } else {
                                    phi_21791_ = _e405;
                                }
                                let _e709 = phi_21791_;
                                phi_21802_ = select(4294967295u, select(_e695, _e686, _e696), (select(_e677, _e549, _e696) < 300000000000000000000000000000000000000f));
                                phi_21803_ = _e709;
                                phi_21804_ = u32();
                                phi_21805_ = u32();
                            } else {
                                let _e414 = (_e403 & 268435455u);
                                phi_21802_ = u32();
                                phi_21803_ = u32();
                                phi_21804_ = ((_e414 + ((_e403 & 2147483647u) >> bitcast<u32>(28u))) + 1u);
                                phi_21805_ = _e414;
                            }
                            let _e713 = phi_21802_;
                            let _e715 = phi_21803_;
                            let _e717 = phi_21804_;
                            let _e719 = phi_21805_;
                            phi_21807_ = _e713;
                            phi_21808_ = _e715;
                            phi_21810_ = _e717;
                            phi_21811_ = _e719;
                            phi_21812_ = select(true, false, _e413);
                            phi_21813_ = _e413;
                        } else {
                            phi_21807_ = u32();
                            phi_21808_ = u32();
                            phi_21810_ = _e372;
                            phi_21811_ = _e374;
                            phi_21812_ = _e409;
                            phi_21813_ = false;
                        }
                        let _e722 = phi_21807_;
                        let _e724 = phi_21808_;
                        let _e726 = phi_21810_;
                        let _e728 = phi_21811_;
                        let _e730 = phi_21812_;
                        let _e732 = phi_21813_;
                        phi_21297_ = _e366;
                        phi_21300_ = select(_e722, 4294967295u, _e730);
                        phi_21302_ = select(_e724, select(_e401, _e405, _e407), _e730);
                        phi_21304_ = select(_e372, _e726, _e730);
                        phi_21306_ = select(_e374, _e728, _e730);
                        phi_21308_ = _e376;
                        phi_21310_ = _e378;
                        phi_21824_ = vec4<f32>();
                        phi_21825_ = select(_e732, true, _e730);
                        phi_21826_ = select(select(_e411, false, _e407), false, _e730);
                    }
                    let _e960 = phi_21297_;
                    let _e962 = phi_21300_;
                    let _e964 = phi_21302_;
                    let _e966 = phi_21304_;
                    let _e968 = phi_21306_;
                    let _e970 = phi_21308_;
                    let _e972 = phi_21310_;
                    let _e974 = phi_21824_;
                    let _e976 = phi_21825_;
                    let _e978 = phi_21826_;
                    phi_21296_ = _e960;
                    phi_21299_ = _e962;
                    phi_21301_ = _e964;
                    phi_21303_ = _e966;
                    phi_21305_ = _e968;
                    phi_21307_ = _e970;
                    phi_21309_ = _e972;
                    phi_38856_ = false;
                    phi_22063_ = _e978;
                    phi_22062_ = _e974;
                    if !(_e976) {
                        break;
                    }
                }
                let _e981 = phi_38856_;
                let _e983 = phi_22063_;
                let _e985 = phi_22062_;
                if _e981 {
                    break;
                }
                if _e983 {
                    let _e987 = local_105;
                    let _e989 = local_106;
                    phi_21831_ = type_79(_e987, _e989);
                } else {
                    phi_21831_ = type_79();
                }
                let _e992 = phi_21831_;
                if select(true, false, _e983) {
                    let _e995 = local_107;
                    phi_21837_ = type_79(_e995, _e985);
                } else {
                    phi_21837_ = _e992;
                }
                let _e998 = phi_21837_;
                let _e1001 = global_7.member[_e332.member_1];
                let _e1013 = type_34(_e1001.member, _e1001.member_1, _e1001.member_2, _e1001.member_3, vec4<u32>(_e1001.member_4.x, _e1001.member_4.y, _e998.member, _e1001.member_4.w), _e1001.member_5);
                if _e337 {
                } else {
                    break;
                }
                global_7.member[_e332.member_1] = type_34(_e1013.member, _e1013.member_1, _e1013.member_2, _e1013.member_3, _e1013.member_4, _e998.member_1);
            }
            break;
        }
    }
    return;
}

fn function_6() {
    var phi_21871_: type_48;
    var phi_21885_: u32;
    var phi_21938_: bool;
    var phi_21939_: bool;
    var phi_21940_: bool;
    var phi_21956_: vec4<f32>;
    var phi_21886_: u32;
    var phi_38978_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e285 = global;
            let _e289 = arrayLength((&global_17.member));
            let _e293 = (_e285.x + (_e285.y * 4194240u));
            let _e297 = global_1.member.member[0u];
            let _e301 = global_1.member.member[1u];
            let _e302 = (_e297 * _e301);
            if (_e293 >= _e302) {
                phi_21871_ = type_48(0u, type_48().member_1);
            } else {
                phi_21871_ = type_48(1u, _e293);
            }
            let _e309 = phi_21871_;
            if (bitcast<i32>(_e309.member) != 0i) {
                phi_21885_ = 0u;
                loop {
                    let _e315 = phi_21885_;
                    let _e319 = global_1.member.member_7[3u];
                    let _e320 = (_e315 < _e319);
                    if _e320 {
                        let _e322 = ((_e315 * _e302) + _e309.member_1);
                        if (_e322 < arrayLength((&global_16.member))) {
                        } else {
                            phi_38978_ = true;
                            break;
                        }
                        let _e328 = global_16.member[_e322].member_4[1u];
                        if ((_e328 & 8u) == 0u) {
                            let _e334 = global_16.member[_e322].member_3;
                            let _e342 = global_16.member[_e322].member[3u];
                            let _e343 = (_e334.x * _e342);
                            let _e344 = (_e334.y * _e342);
                            let _e345 = (_e334.z * _e342);
                            if (_e343 == _e343) {
                                let _e348 = (_e344 == _e344);
                                if _e348 {
                                    phi_21938_ = select(true, false, (_e345 == _e345));
                                } else {
                                    phi_21938_ = false;
                                }
                                let _e352 = phi_21938_;
                                phi_21939_ = select(true, false, _e348);
                                phi_21940_ = _e352;
                            } else {
                                phi_21939_ = true;
                                phi_21940_ = false;
                            }
                            let _e355 = phi_21939_;
                            let _e357 = phi_21940_;
                            let _e360 = select(vec3<f32>(_e343, _e344, _e345), vec3<f32>(0f, 0f, 0f), vec3(select(_e357, true, _e355)));
                            let _e365 = global_16.member[_e322].member_3[3u];
                            phi_21956_ = vec4<f32>(_e360.x, _e360.y, _e360.z, _e365);
                        } else {
                            phi_21956_ = vec4<f32>(0f, 0f, 0f, 0f);
                        }
                        let _e371 = phi_21956_;
                        if (_e371.w > 0f) {
                            let _e377 = (2u * _e309.member_1);
                            if (_e377 < _e289) {
                            } else {
                                phi_38978_ = true;
                                break;
                            }
                            let _e382 = global_17.member[_e377][0u];
                            global_17.member[_e377][0u] = (_e382 + _e371.x);
                            let _e387 = global_17.member[_e377][1u];
                            global_17.member[_e377][1u] = (_e387 + _e371.y);
                            let _e392 = global_17.member[_e377][2u];
                            global_17.member[_e377][2u] = (_e392 + _e371.z);
                            let _e397 = global_17.member[_e377][3u];
                            global_17.member[_e377][3u] = (_e397 + _e371.w);
                            let _e399 = (_e377 + 1u);
                            if (_e399 < _e289) {
                            } else {
                                phi_38978_ = true;
                                break;
                            }
                            let _e407 = global_17.member[_e399][0u];
                            global_17.member[_e399][0u] = (_e407 + (_e371.x * _e371.x));
                            let _e412 = global_17.member[_e399][1u];
                            global_17.member[_e399][1u] = (_e412 + (_e371.y * _e371.y));
                            let _e417 = global_17.member[_e399][2u];
                            global_17.member[_e399][2u] = (_e417 + (_e371.z * _e371.z));
                            let _e422 = global_17.member[_e399][3u];
                            global_17.member[_e399][3u] = (_e422 + 1f);
                        }
                        phi_21886_ = (_e315 + 1u);
                    } else {
                        phi_21886_ = u32();
                    }
                    let _e426 = phi_21886_;
                    phi_21885_ = _e426;
                    phi_38978_ = false;
                    if !(_e320) {
                        break;
                    }
                }
                let _e429 = phi_38978_;
                if _e429 {
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
fn start_paths(@builtin(global_invocation_id) param_2: vec3<u32>) {
    global = param_2;
    function_2();
}

@compute @workgroup_size(64, 1, 1) 
fn bounce_paths(@builtin(global_invocation_id) param_3: vec3<u32>) {
    global = param_3;
    function_3();
}

@compute @workgroup_size(64, 1, 1) 
fn scatter_host(@builtin(global_invocation_id) param_4: vec3<u32>) {
    global = param_4;
    function_4();
}

@compute @workgroup_size(64, 1, 1) 
fn intersect_paths(@builtin(global_invocation_id) param_5: vec3<u32>) {
    global = param_5;
    function_5();
}

@compute @workgroup_size(64, 1, 1) 
fn add_paths(@builtin(global_invocation_id) param_6: vec3<u32>) {
    global = param_6;
    function_6();
}
